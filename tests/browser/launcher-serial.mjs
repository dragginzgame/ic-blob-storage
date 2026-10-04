// Native Rust owns phases; the maintained bridge owns Chromium/body handoff.
import { createRequire as browserPackages } from 'node:module';
const { chromium } = browserPackages(new URL('../../tests/browser/package.json', import.meta.url))('playwright');
import { StorageClient } from '@caffeineai/object-storage';
import { Ed25519KeyIdentity } from '@icp-sdk/core/identity';
import { readFile, writeFile } from 'node:fs/promises';
import { createServer } from 'node:net';
import { resolve, join } from 'node:path';
import { createInterface } from 'node:readline';
import { once } from 'node:events';
import assert from 'node:assert/strict';
import { launchPublicationBrowser } from '../../clients/browser/launcher.mjs';
import { startPublicationSession } from '../../clients/browser/native.mjs';
import { createHash } from 'node:crypto';
import { loopbackTLS, loopbackH2 } from './tls.mjs';
import { standaloneGateway } from './standalone-gateway.mjs';
import { verifyMediaDelivery } from './media-delivery.mjs';
import { attachBrowserMemory, sampleBrowserMemory } from './memory.mjs';

const config = JSON.parse(await readFile(process.argv[2], 'utf8'));
assert(Number.isSafeInteger(config.timeoutSeconds) && config.timeoutSeconds >= 1 && config.timeoutSeconds <= 600);
const reservation = createServer();
await new Promise(resolve => reservation.listen(0, '127.0.0.1', resolve));
const port = reservation.address().port;
await new Promise(resolve => reservation.close(resolve));
config.browserOrigin = `http://127.0.0.1:${port}`;
const bodies = await Promise.all(config.bodies.map(path => readFile(path)));
const requestCounts = bodies.map(body => 1 + Math.ceil(body.length / (1024 * 1024)));
const maxRequests = Math.max(...requestCounts), maxBodyBytes = Math.max(...bodies.map(body => body.length));
const maxRequestBytes = Math.max(65536, Math.min(1024 * 1024, maxBodyBytes));
const { handle, state } = standaloneGateway(config, Buffer.alloc(0), bodies.map(body => body.length), undefined, bodies);
const tls = await loopbackTLS(), gateway = await loopbackH2(tls, handle);
await writeFile(config.providerRootCertificate, tls.root, { flag: 'wx' });
const input = createInterface({ input: process.stdin, crlfDelay: Infinity });
const next = async () => JSON.parse((await once(input, 'line'))[0]);
const send = value => process.stdout.write(`${JSON.stringify(value)}\n`);
const signer = Ed25519KeyIdentity.generate(new Uint8Array(32).fill(42));
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
await writeFile(config.browserSelection, JSON.stringify({
  format: 'ic-blob-storage/browser-selection', session: resolve(config.session), profile: resolve(config.profile),
  project: config.project, bucket: config.bucket, asset_port: port,
  signer_sha256: digest(JSON.stringify({ kind: 'ed25519', json: JSON.stringify(signer.toJSON()) })),
  host_sha256: digest(await readFile('.tmp/browser/publication-host.js')),
  worker_sha256: digest(await readFile('.tmp/browser/publication-worker.js')),
  database: 'standalone-trial-v1', max_slots: 2,
}), { flag: 'wx', mode: 0o600 });
const workerReports = [], journals = [], originals = [], unexpected = [];
let bridge, native, context, memoryOwner, memorySequence = 0,
  certificateCalls = 0, allCertificateCalls = 0, id = 0;
const memoryReports = [];
const engine = { async launchPersistentContext(...args) {
  context = await chromium.launchPersistentContext(...args);
  const owner = { context };
  memoryOwner = owner;
  context.on('request', request => {
    const url = new URL(request.url());
    if (url.origin === new URL(config.url).origin && url.pathname.endsWith('/call')) { certificateCalls++; allCertificateCalls++; }
    if (![config.browserOrigin, gateway.origin, new URL(config.url).origin].includes(url.origin)) unexpected.push(url.origin);
  });
  return process.env.BLOB_BROWSER_MEMORY_PROFILE === '1' ? {
    newPage: (...args) => owner.context.newPage(...args),
    async close() {
      try {
        if (owner.sample) {
          const result = await owner.sample.stop();
          memoryReports.push(result);
          await writeFile(join(config.report, `browser-memory-${++memorySequence}.json`),
            JSON.stringify(result, null, 2) + '\n', { flag: 'wx', mode: 0o600 });
        }
      } finally {
        owner.sample = undefined;
        try { await owner.monitor?.close(); }
        finally { owner.monitor = undefined; await owner.context.close(); }
      }
    },
  } : context;
} };
async function open(mode) {
  certificateCalls = 0; id = 0;
  bridge = await launchPublicationBrowser(engine, {
    profile: resolve(config.profile), assetPort: port, nativeSession: resolve(config.session),
    hostBundle: resolve('.tmp/browser/publication-host.js'), workerBundle: resolve('.tmp/browser/publication-worker.js'),
    bootstrap: { schema: 1, operation: 'bootstrap', signer: { kind: 'ed25519', json: JSON.stringify(signer.toJSON()) },
      configuration: { host: config.url, rootKey: config.rootKey, service: config.service, tenant: config.tenant,
        uploader: signer.getPrincipal().toText(), project: config.project, bucket: config.bucket, origin: gateway.origin,
        maxBodyBytes, maxRequests, maxRequestBytes, maxTotalRequestBytes: maxBodyBytes + 65536, maxJobs: 32, timeoutSeconds: config.timeoutSeconds },
      journal: { database: 'standalone-trial-v1', maxSlots: 2, mode } } }, tls.launch);
  if (process.env.BLOB_BROWSER_MEMORY_PROFILE === '1') {
    memoryOwner.monitor = await attachBrowserMemory(memoryOwner.context, config.browserOrigin);
    memoryOwner.sample = await sampleBrowserMemory(memoryOwner.monitor);
  }
}
async function job(grant, action) {
  const request = { id: ++id, index: grant.index, action,
    ...(action === 'upload' ? { nativePhase: grant.native_phase, action: 'transfer' } : { binding: grant.transfer.binding }) };
  const result = await bridge.execute(request); workerReports.push(result); return result;
}
async function coordinate() {
  const selected = await next();
  assert.equal(selected.native.timeoutSeconds, config.timeoutSeconds);
  let interrupted = false;
  const observed = async value => { send(value); assert.deepEqual(await next(), { accepted: true }); };
  const start = async (args, label) => {
    native = await startPublicationSession({ ...selected.native, args });
    await observed({ ready: native.ready, label });
  };
  await start(selected.native.args, 'session');
  await open('create');
  const phase = async (frame, signal) => {
    if (config.lostFinalReply && frame.phase === 'verify' && frame.index === 0 && !interrupted) {
      interrupted = true;
      await observed({ restart: await native.finish() }); await native.close();
      throw new Error('fixture control loss after browser transfer, before verification');
    }
    const event = await native.phase(frame, signal);
    await observed({ frame, event });
    if (frame.phase === 'transfer') {
      const grant = { index: frame.index, native_phase: event.report.native_phase };
      grant.transfer = JSON.parse(await readFile(resolve(grant.native_phase, 'transfer.json'))).transfer;
      if (!originals.some(original => original.index === grant.index)) originals.push(grant);
    }
    return event;
  };
  let result;
  try { result = await bridge.driveSession({ ...native, phase }); }
  catch (error) {
    assert.equal(error.code, 'native-control');
    if (interrupted) {
      await start(selected.resumedArgs, 'resumed-session');
      await open('open'); result = await bridge.driveSession({ ...native, phase });
    } else { assert(config.corruptRead); result = { state: 'failed', error: error.code }; }
  }
  assert.equal(result.state, config.corruptRead ? 'failed' : 'complete');
  assert.equal(interrupted, !!config.lostFinalReply);
  const final = config.corruptRead ? await native.finish() : result.native_result;
  await observed({ finished: final }); await native.close();
  // Inspect the preserved original claims in a freshly opened worker. Correlation
  // IDs restart per context; none grants permission to dispatch again.
  await bridge.close(); await open('open');
  for (const original of originals) {
    const inspected = await job(original, 'inspect');
    assert.equal(inspected.state, 'inspected');
    const row = inspected.journal;
    assert.equal(row.certificate_phase, 'observed'); assert.equal(row.gateway_requests.length, requestCounts[original.index]);
    assert.equal(row.gateway_requests.at(-1).phase, config.lostFinalReply && original.index === 0 ? 'uncertain' : 'responded');
    journals.push(row);
  }
  assert.equal(allCertificateCalls, config.corruptRead ? 1 : 2);
  workerReports.push(result);
}
const deadline = setTimeout(() => { console.error('Native browser bridge trial exceeded 180 seconds'); process.exit(1); }, 180000);
let media = { decodedMedia: [], publicDelivery: [], opaqueOriginRefused: false, csp: null };
try {
  const files = await Promise.all(bodies.map(async (body, index) => {
    const prepared = await StorageClient.prepareFile(body, config.contentTypes[index], undefined, config.cacheControls[index] ?? undefined);
    return { hash: prepared.hash, byteLength: prepared.byteLength, manifestJSON: prepared.manifestJSON };
  }));
  send({ gateway: gateway.origin, files });
  if (config.coordinated) await coordinate();
  else for (const index of [0, 1]) {
    const grant = await next();
    if (grant.finish) { assert(config.corruptRead && index === 1); break; }
    assert.equal(grant.index, index);
    grant.transfer = JSON.parse(await readFile(resolve(grant.native_phase, 'transfer.json'))).transfer;
    originals.push(grant);
    if (!bridge) await open('create');
    assert.deepEqual(grant.transfer.preparation, { content_type: config.contentTypes[index],
      ...(config.cacheControls[index] === null ? {} : { cache_control: config.cacheControls[index] }) });
    const outcome = await job(grant, 'upload'), lost = config.lostFinalReply && index === 0;
    assert.equal(outcome.state, lost ? 'failed' : 'transfer-observed'); assert.equal(certificateCalls, 1);
    if (config.corruptRead) assert.equal((await job(grant, 'cancel')).state, 'cancelled');
    const inspected = await job(grant, 'inspect'); assert.equal(inspected.state, 'inspected');
    const row = inspected.journal;
    assert.equal(row.certificate_phase, 'observed'); assert.equal(row.gateway_requests.length, requestCounts[index]);
    assert.equal(row.gateway_requests.at(-1).phase, lost ? 'uncertain' : 'responded'); journals.push(row);
    await bridge.close(); await open('open');
    const recovered = await job({ ...grant, native_phase: grant.recovery_phase }, 'upload');
    assert.equal(recovered.state, 'certificate-observed'); assert(recovered.certificate_bytes > 0);
    assert.deepEqual(recovered.journal, row);
    assert.equal((await job(grant, 'upload')).error, 'upload-claimed'); assert.equal(certificateCalls, 0);
    send({ index, gateway: gateway.origin, uploaded: !lost, finalRequestPhase: row.gateway_requests.at(-1).phase, browserRestarted: true });
    if (index === 1) assert.deepEqual(await next(), { finish: true });
  }
  assert.equal(state.failure, undefined); assert.deepEqual(unexpected, []);
  assert.equal(state.puts.length, config.corruptRead ? requestCounts[0] : requestCounts.reduce((sum, count) => sum + count, 0));
  if (config.media && !config.corruptRead) {
    media = await verifyMediaDelivery(context, config, bodies, gateway.origin, state);
    assert.notEqual(files[0].hash, files[1].hash);
    if (bodies[0].length > 1024 * 1024) {
      const chunks = JSON.parse(files[0].manifestJSON).chunk_hashes;
      assert.equal(chunks.length, Math.ceil(bodies[0].length / (1024 * 1024)));
      assert(new Set(chunks).size > 1);
    }
  }
  assert.equal(state.gets.length, config.corruptRead ? 1 : 4 + (config.overlap ? 1 : 0) + media.publicDelivery.length + (media.csp?.images.length ?? 0) + Number(media.opaqueOriginRefused));
  assert.equal(state.failure, undefined); assert.deepEqual(unexpected, []);
  for (const [index, grant] of originals.entries()) assert.deepEqual((await job(grant, 'inspect')).journal, journals[index]);
  // Seal the last context before reporting success, including observer failures.
  if (process.env.BLOB_BROWSER_MEMORY_PROFILE === '1') {
    await bridge.close();
    assert(memoryReports.every(report => report.error === null));
  }
  send({ outcome: 'passed', provider: 'local HTTPS HTTP/2 substitute', browserBridge: true,
    assetOrigin: config.browserOrigin, puts: state.puts, gets: state.gets, arrivals: state.arrivals,
    journals, workerReports, ...media, reads: state.reads,
    browserRestarted: true, sessionTimeoutSeconds: config.timeoutSeconds,
    uploadRetries: 0, liveProviderRequests: 0, paidEffects: 0 });
} finally {
  clearTimeout(deadline); input.close(); process.stdin.pause();
  try {
    // Retain bounded request fingerprints even when a delivery assertion fails.
    await writeFile(join(config.report, 'browser-delivery-trace.json'), JSON.stringify({
      puts: state.puts, gets: state.gets, arrivals: state.arrivals, reads: state.reads,
      failure: state.failure ?? null, journals, workerReports, ...media,
      memoryProfiles: memoryReports.map((report, index) => ({ file: `browser-memory-${index + 1}.json`, error: report.error })),
    }, null, 2), { flag: 'wx', mode: 0o600 });
  } finally {
    await bridge?.close(); await gateway.close(); await tls.close();
    await native?.close();
  }
}
