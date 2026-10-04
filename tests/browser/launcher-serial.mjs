// Native Rust owns phases; the maintained bridge owns Chromium/body handoff.
import { createRequire as browserPackages } from 'node:module';
const { chromium } = browserPackages(new URL('../../tests/browser/package.json', import.meta.url))('playwright');
import { StorageClient } from '@caffeineai/object-storage';
import { Ed25519KeyIdentity } from '@icp-sdk/core/identity';
import { readFile, writeFile } from 'node:fs/promises';
import { createServer } from 'node:net';
import { resolve } from 'node:path';
import { createInterface } from 'node:readline';
import { once } from 'node:events';
import assert from 'node:assert/strict';
import { launchPublicationBrowser } from '../../clients/browser/launcher.mjs';
import { startPublicationSession } from '../../clients/browser/native.mjs';
import { createHash } from 'node:crypto';
import { loopbackTLS, loopbackH2 } from './tls.mjs';
import { standaloneGateway } from './standalone-gateway.mjs';

const config = JSON.parse(await readFile(process.argv[2], 'utf8'));
const reservation = createServer();
await new Promise(resolve => reservation.listen(0, '127.0.0.1', resolve));
const port = reservation.address().port;
await new Promise(resolve => reservation.close(resolve));
config.browserOrigin = `http://127.0.0.1:${port}`;
const { handle, state } = standaloneGateway(config, Buffer.alloc(0), [1024, 2048]);
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
let bridge, native, certificateCalls = 0, allCertificateCalls = 0, id = 0;
const engine = { async launchPersistentContext(...args) {
  const context = await chromium.launchPersistentContext(...args);
  context.on('request', request => {
    const url = new URL(request.url());
    if (url.origin === new URL(config.url).origin && url.pathname.endsWith('/call')) { certificateCalls++; allCertificateCalls++; }
    if (![config.browserOrigin, gateway.origin, new URL(config.url).origin].includes(url.origin)) unexpected.push(url.origin);
  });
  return context;
} };
async function open(mode) {
  certificateCalls = 0; id = 0;
  bridge = await launchPublicationBrowser(engine, {
    profile: resolve(config.profile), assetPort: port, nativeSession: resolve(config.session),
    hostBundle: resolve('.tmp/browser/publication-host.js'), workerBundle: resolve('.tmp/browser/publication-worker.js'),
    bootstrap: { schema: 1, operation: 'bootstrap', signer: { kind: 'ed25519', json: JSON.stringify(signer.toJSON()) },
      configuration: { host: config.url, rootKey: config.rootKey, service: config.service, tenant: config.tenant,
        uploader: signer.getPrincipal().toText(), project: config.project, bucket: config.bucket, origin: gateway.origin,
        maxBodyBytes: 2048, maxRequests: 2, maxRequestBytes: 65536, maxTotalRequestBytes: 131072, maxJobs: 32, timeoutSeconds: 120 },
      journal: { database: 'standalone-trial-v1', maxSlots: 2, mode } } }, tls.launch);
}
async function job(grant, action) {
  const request = { id: ++id, index: grant.index, action,
    ...(action === 'upload' ? { nativePhase: grant.native_phase, action: 'transfer' } : { binding: grant.transfer.binding }) };
  const result = await bridge.execute(request); workerReports.push(result); return result;
}
async function coordinate() {
  const selected = await next();
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
    assert.equal(row.certificate_phase, 'observed'); assert.equal(row.gateway_requests.length, 2);
    assert.equal(row.gateway_requests[1].phase, config.lostFinalReply && original.index === 0 ? 'uncertain' : 'responded');
    journals.push(row);
  }
  assert.equal(allCertificateCalls, config.corruptRead ? 1 : 2);
  workerReports.push(result);
}
const deadline = setTimeout(() => { console.error('Native browser bridge trial exceeded 180 seconds'); process.exit(1); }, 180000);
try {
  const files = await Promise.all([1024, 2048].map(async size => {
    const prepared = await StorageClient.prepareFile(new Uint8Array(size).fill(42), 'image/png');
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
    assert.deepEqual(grant.transfer.preparation, { content_type: 'image/png' });
    const outcome = await job(grant, 'upload'), lost = config.lostFinalReply && index === 0;
    assert.equal(outcome.state, lost ? 'failed' : 'transfer-observed'); assert.equal(certificateCalls, 1);
    if (config.corruptRead) assert.equal((await job(grant, 'cancel')).state, 'cancelled');
    const inspected = await job(grant, 'inspect'); assert.equal(inspected.state, 'inspected');
    const row = inspected.journal;
    assert.equal(row.certificate_phase, 'observed'); assert.equal(row.gateway_requests.length, 2);
    assert.equal(row.gateway_requests[1].phase, lost ? 'uncertain' : 'responded'); journals.push(row);
    await bridge.close(); await open('open');
    const recovered = await job({ ...grant, native_phase: grant.recovery_phase }, 'upload');
    assert.equal(recovered.state, 'certificate-observed'); assert(recovered.certificate_bytes > 0);
    assert.deepEqual(recovered.journal, row);
    assert.equal((await job(grant, 'upload')).error, 'upload-claimed'); assert.equal(certificateCalls, 0);
    send({ index, gateway: gateway.origin, uploaded: !lost, finalRequestPhase: row.gateway_requests[1].phase, browserRestarted: true });
    if (index === 1) assert.deepEqual(await next(), { finish: true });
  }
  assert.equal(state.failure, undefined); assert.deepEqual(unexpected, []);
  assert.equal(state.puts.length, config.corruptRead ? 2 : 4); assert.equal(state.gets.length, config.corruptRead ? 1 : 4);
  for (const [index, grant] of originals.entries()) assert.deepEqual((await job(grant, 'inspect')).journal, journals[index]);
  send({ outcome: 'passed', provider: 'local HTTPS HTTP/2 substitute', browserBridge: true,
    assetOrigin: config.browserOrigin, puts: state.puts, gets: state.gets, arrivals: state.arrivals,
    journals, workerReports, browserRestarted: true, uploadRetries: 0, liveProviderRequests: 0, paidEffects: 0 });
} finally {
  clearTimeout(deadline); input.close(); process.stdin.pause();
  await bridge?.close(); await gateway.close(); await tls.close();
  await native?.close();
}
