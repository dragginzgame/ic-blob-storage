// Local test identity, fault injection and controls; transport lives in clients/browser.
import { Cbor } from '@icp-sdk/core/agent';
import { Ed25519KeyIdentity } from '@icp-sdk/core/identity';
import { StorageClient } from '@caffeineai/object-storage';
import { createCertificateClient } from '../../clients/browser/certificate.js';
import { createGatewayTransport } from '../../clients/browser/gateway.js';
import * as intents from './intent.js';
import { admitAndPrepare, finishConsumer } from './admission.js';

const unhex = value => Uint8Array.from(value.match(/../g), b => parseInt(b, 16));
let client, storage, prepared, lastAttempt, abortUpload, config, binding, lastProof, gatewayFetch;
let gatewayHeld = false, releaseGateway;
let gatewayCalls = 0;
let calls = 0, reads = 0, abortClaim = false, responseHeld = false, releaseResponse;
let corruptInspection;
async function setup(cfg) {
  config = cfg;
  // Upstream preparation needs no agent, service authorization or network call.
  prepared ??= await prepare();
  binding = { key: `${cfg.service}:${cfg.tenant}:${cfg.operation}`, service: cfg.service,
    tenant: cfg.tenant, uploader: cfg.uploader, operation: cfg.operation,
    permission: cfg.permission, root: cfg.root };
  client = await createCertificateClient({ host: cfg.url,
    identity: Ed25519KeyIdentity.generate(new Uint8Array(32).fill(42)),
    rootKey: new Uint8Array(cfg.rootKey), binding,
    intents: { ...intents,
      inspect: async binding => {
        const row = await intents.inspect(binding);
        return corruptInspection ? corruptInspection(row) : row;
      },
      claim: (binding, bytes, id) => intents.claim(binding, bytes, id, abortClaim) },
    fetch: async (url, init) => {
      const isCall = new URL(url).pathname.endsWith('/call');
      if (isCall) calls += 1;
      else reads += 1;
      const response = await fetch(url, init);
      if (isCall && cfg.holdResponse) {
        responseHeld = true;
        await new Promise(resolve => { releaseResponse = resolve; });
      }
      return response;
    } });
  abortUpload = new AbortController();
  gatewayFetch = await createGatewayTransport({ certificate: client, origin: cfg.gateway,
    maxRequests: 2, maxRequestBytes: 1024 * 1024,
    intents: { ...intents,
      claimGateway: async (...args) => {
        // Cancel after the hook's read, before the atomic claim rechecks authority.
        if (cfg.cancelAtGatewayClaim) await client.cancel();
        return intents.claimGateway(...args, !!cfg.gatewayWriteAbort);
      },
      observeGateway: (...args) => intents.observeGateway(...args, !!cfg.gatewayObserveAbort) },
    fetch: async (url, init) => {
      const row = await client.inspect();
      // Inspect the committed transaction at the actual transport boundary.
      if (row.gateway?.requests.at(-1)?.phase !== 'uncertain') throw new Error('missing gateway intent');
      gatewayCalls += 1;
      const response = await fetch(url, init);
      if (cfg.holdGateway && new URL(url).pathname === '/v1/blob-tree/') {
        gatewayHeld = true;
        await new Promise(resolve => { releaseGateway = resolve; });
      }
      return response;
    },
  });
  storage = new StorageClient('fixture-bucket', cfg.gateway, cfg.service, 'fixture-project', client.certificateAgent, {
    retry: false, concurrency: 1, signal: abortUpload.signal,
    fetch: async (url, init) => {
      const response = await gatewayFetch(url, init);
      if (cfg.abortAfterTree && new URL(url).pathname === '/v1/blob-tree/') abortUpload.abort();
      return response;
    },
  });
}
async function prepare() {
  const bytes = new Uint8Array(10).fill(config.content);
  const preparing = StorageClient.prepareFile(bytes, 'image/png');
  bytes.fill(255); // The package patch must snapshot bytes before its first await.
  const result = await preparing;
  if (!Object.isFrozen(result) || result.hash !== config.root) throw new Error('preparation binding');
  return result;
}
async function remember(result) {
  const { certificate, ...row } = await result;
  lastProof = { raw: certificate, requestId: unhex(row.requestId) };
  return row;
}
window.fixture = { setup, admitAndPrepare, finishConsumer, plan: async cfg => {
    config = cfg;
    prepared = await prepare();
    return prepared;
  }, issue: async () => {
    // Each explicit test invocation obtains a preparation. The durable claim, not
    // this ephemeral handle, prevents a new upload after uncertainty/cancellation.
    const plan = prepared;
    prepared = await prepare();
    lastAttempt = plan;
    const result = await storage.uploadPrepared(plan);
    if (result.hash !== config.root) throw new Error('upstream root differs from Rust declaration');
    return client.inspect();
  }, recover: () => remember(client.recover()),
  inspect: () => client.inspect(), cancel: () => client.cancel(), calls: () => calls, reads: () => reads,
  preparation: () => prepared, gatewayCalls: () => gatewayCalls, abortUpload: () => abortUpload.abort(),
  repeatHandle: () => storage.uploadPrepared(lastAttempt),
  rejectClonedHandle: () => storage.uploadPrepared({ ...prepared }),
  held: () => responseHeld, release: () => releaseResponse?.(),
  gatewayHeld: () => gatewayHeld, releaseGateway: () => releaseGateway?.(),
  gatewayProbe: (kind = 'ordinary') => gatewayFetch(
    kind === 'origin' ? 'https://invalid.example/probe' : `${config.gateway}/probe`, {
      method: 'PUT', body: kind === 'size' ? new Uint8Array(1024 * 1024 + 1) : new Uint8Array([1]),
    }),
  abortClaim: value => { abortClaim = value; },
  rejectProof: async kind => {
    const { raw, requestId } = lastProof;
    if (kind === 'request') return client.observeCertificate(raw, new Uint8Array(32));
    if (kind === 'size') return client.observeCertificate(new Uint8Array(128 * 1024 + 1), requestId);
    const changed = Cbor.decode(raw);
    changed.signature[0] ^= 1;
    return client.observeCertificate(Cbor.encode(changed), requestId);
  },
  rejectRetained: async kind => {
    corruptInspection = row => {
      if (kind === 'binding') row.binding.permission[0] ^= 1;
      if (kind === 'trust') row.binding.icRootKey[0] ^= 1;
      if (kind === 'phase') row.phase = 'saved';
      if (kind === 'envelope') {
        const envelope = Cbor.decode(new Uint8Array(row.envelope));
        envelope.content.method_name = 'unrelated';
        row.envelope = Array.from(Cbor.encode(envelope));
      }
      return row;
    };
    try { return await client.recover(); } finally { corruptInspection = undefined; }
  },
  rejectSetup: async kind => {
    const changed = structuredClone(binding);
    if (kind === 'identity') changed.uploader = config.tenant;
    if (kind === 'operation') changed.operation = '340282366920938463463374607431768211456';
    return createCertificateClient({ host: config.url,
      identity: Ed25519KeyIdentity.generate(new Uint8Array(32).fill(42)),
      rootKey: new Uint8Array(config.rootKey), binding: changed, intents });
  },
  save: intents.save, observe: intents.observe };
