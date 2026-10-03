// Actual maintained Node/Chromium/profile boundary; no IC/provider request.
import { chromium } from 'playwright';
import { Ed25519KeyIdentity } from '@icp-sdk/core/identity';
import { createServer } from 'node:net';
import { mkdir, writeFile, lstat, symlink } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
import { resolve, join } from 'node:path';
import assert from 'node:assert/strict';
import { launchPublicationBrowser, LauncherRefusal } from '../../clients/browser/launcher.mjs';
const report = resolve(process.argv[2]);
await mkdir(report, { mode: 0o700 });
const reservation = createServer();
await new Promise(resolve => reservation.listen(0, '127.0.0.1', resolve));
const port = reservation.address().port;
const signer = Ed25519KeyIdentity.generate(new Uint8Array(32).fill(42));
const cases = [], requests = [], unexpected = [];
const origin = `http://127.0.0.1:${port}`;
const assetOrigins = new Set([origin]);
const engine = { async launchPersistentContext(...args) {
  const context = await chromium.launchPersistentContext(...args);
  context.on('request', request => {
    requests.push({ method: request.method(), path: new URL(request.url()).pathname });
    if (!assetOrigins.has(new URL(request.url()).origin)) unexpected.push(new URL(request.url()).origin);
  });
  return context;
} };
const options = mode => ({ profile: join(report, 'profile'), assetPort: port,
  hostBundle: resolve('.tmp/browser/publication-host.js'), workerBundle: resolve('.tmp/browser/publication-worker.js'),
  bootstrap: { schema: 1, operation: 'bootstrap', signer: { kind: 'ed25519', json: JSON.stringify(signer.toJSON()) },
    configuration: { host: origin, rootKey: [1], service: 'aaaaa-aa', tenant: 'rrkah-fqaaa-aaaaa-aaaaq-cai',
      uploader: signer.getPrincipal().toText(), project: 'bootstrap-fixture', bucket: 'test', origin: 'https://substitute.invalid',
      maxBodyBytes: 1024, maxRequests: 2, maxRequestBytes: 65536, maxTotalRequestBytes: 131072, maxJobs: 32, timeoutSeconds: 30 },
    journal: { database: 'native-browser-v1', mode, maxSlots: 2 } } });
const binding = { key: 'aaaaa-aa:rrkah-fqaaa-aaaaa-aaaaq-cai:1', service: 'aaaaa-aa',
  tenant: 'rrkah-fqaaa-aaaaa-aaaaq-cai', uploader: signer.getPrincipal().toText(),
  project: 'bootstrap-fixture', bucket: 'test', operation: '1', root: 'sha256:' + '1'.repeat(64), permission: [68, 73, 68, 76] };
const body = join(report, 'body.bin'); await writeFile(body, 'abc', { flag: 'wx', mode: 0o600 });
const transfer = { binding, body, bytes: '3', body_sha256: createHash('sha256').update('abc').digest('hex'),
  manifest_json: '{"tree_type":"DSBMTWH"}', preparation: {} };
const refusal = code => error => error instanceof LauncherRefusal && error.code === code;
let bridge;
try {
  await assert.rejects(launchPublicationBrowser(engine, options('create')), refusal('launch'));
  await assert.rejects(lstat(join(report, 'profile')), { code: 'ENOENT' }); cases.push('occupied_port_refuses_before_profile');
  await new Promise(resolve => reservation.close(resolve));
  bridge = await launchPublicationBrowser(engine, options('create'));
  const inspection = await bridge.execute({ id: 1, index: 0, action: 'inspect', binding });
  assert.equal(inspection.state, 'inspected'); assert.deepEqual(inspection.journal, { present: false });
  cases.push('explicit_profile_and_origin_start_without_effects');
  const pending = bridge.execute({ id: 2, index: 0, action: 'upload', transfer });
  await assert.rejects(bridge.execute({ id: 3, index: 0, action: 'inspect', binding }), refusal('busy'));
  transfer.body_sha256 = '0'.repeat(64); // The bridge already owns the original request.
  const result = await pending; assert.equal(result.error, 'root'); assert.equal(result.id, 2);
  transfer.body_sha256 = createHash('sha256').update('abc').digest('hex'); cases.push('owned_request_and_single_job');
  const changed = structuredClone(transfer); changed.body_sha256 = '0'.repeat(64);
  await assert.rejects(bridge.execute({ id: 3, index: 0, action: 'upload', transfer: changed }), refusal('body-digest'));
  changed.body_sha256 = transfer.body_sha256; changed.bytes = '2';
  await assert.rejects(bridge.execute({ id: 3, index: 0, action: 'upload', transfer: changed }), refusal('body-size'));
  changed.bytes = '1025';
  await assert.rejects(bridge.execute({ id: 3, index: 0, action: 'upload', transfer: changed }), refusal('body-size'));
  changed.bytes = '03';
  await assert.rejects(bridge.execute({ id: 3, index: 0, action: 'upload', transfer: changed }), refusal('body-size'));
  cases.push('changed_truncated_oversized_and_noncanonical_body_refuse');
  changed.bytes = '3'; changed.body = join(report, 'link.bin'); await symlink(body, changed.body);
  await assert.rejects(bridge.execute({ id: 3, index: 0, action: 'upload', transfer: changed }), refusal('file'));
  changed.body = report;
  await assert.rejects(bridge.execute({ id: 3, index: 0, action: 'upload', transfer: changed }), refusal('body-size'));
  cases.push('symlinks_and_directories_are_not_bodies');
  changed.body = join(report, 'pipe');
  await promisify(execFile)('mkfifo', [changed.body]);
  await assert.rejects(bridge.execute({ id: 3, index: 0, action: 'upload', transfer: changed }), refusal('body-size'));
  cases.push('fifo_without_writer_refuses_without_blocking');
  for (const hints of [{ filename: null }, { filename: 'β'.repeat(2049) }, { inferred_type: 'text/plain' }]) {
    changed.body = body; changed.preparation = hints;
    await assert.rejects(bridge.execute({ id: 3, index: 0, action: 'upload', transfer: changed }),
      refusal(Object.hasOwn(hints, 'inferred_type') ? 'configuration' : 'metadata-hint'));
  }
  cases.push('metadata_hints_have_exact_fields_and_utf8_bounds');
  assert.deepEqual((await bridge.execute({ id: 3, index: 0, action: 'inspect', binding })).journal, { present: false });
  await bridge.close(); await bridge.close();
  await assert.rejects(bridge.execute({ id: 4, index: 0, action: 'inspect', binding }), refusal('closed'));
  await assert.rejects(launchPublicationBrowser(engine, options('create')), refusal('launch'));
  cases.push('close_preserves_profile_and_refuses_replacement');
  const foreignPort = createServer();
  await new Promise(resolve => foreignPort.listen(0, '127.0.0.1', resolve));
  const changedOrigin = options('open'); changedOrigin.assetPort = foreignPort.address().port;
  assetOrigins.add(`http://127.0.0.1:${changedOrigin.assetPort}`);
  await new Promise(resolve => foreignPort.close(resolve));
  await assert.rejects(launchPublicationBrowser(engine, changedOrigin), refusal('launch'));
  const missing = options('open'); missing.profile = join(report, 'missing-profile');
  await assert.rejects(launchPublicationBrowser(engine, missing), refusal('launch'));
  await assert.rejects(lstat(missing.profile), { code: 'ENOENT' });
  const linked = options('open'); linked.profile = join(report, 'linked-profile');
  await symlink(join(report, 'profile'), linked.profile);
  await assert.rejects(launchPublicationBrowser(engine, linked), refusal('profile'));
  cases.push('changed_origin_missing_and_symlink_profiles_refuse_without_replacement');
  bridge = await launchPublicationBrowser(engine, options('open'));
  assert.deepEqual((await bridge.execute({ id: 1, index: 0, action: 'inspect', binding })).journal, { present: false });
  await bridge.close(); cases.push('same_origin_and_profile_reopen_original_journal');
  const deadline = options('open'); deadline.bootstrap.configuration.timeoutSeconds = 1;
  bridge = await launchPublicationBrowser(engine, deadline);
  await new Promise(resolve => setTimeout(resolve, 1500));
  await assert.rejects(bridge.execute({ id: 1, index: 0, action: 'inspect', binding }), refusal('closed'));
  await bridge.close(); assert((await lstat(join(report, 'profile'))).isDirectory()); cases.push('deadline_closes_browser_without_erasing_profile');
  assert.deepEqual(unexpected, []); assert(requests.every(request => request.method === 'GET'));
  await writeFile(join(report, 'summary.json'), JSON.stringify({ schema: 1, cases, requests,
    assetOrigin: origin, unexpected, liveProviderRequests: 0, paidEffects: 0, automaticRetries: 0 }, null, 2), { flag: 'wx', mode: 0o600 });
  console.log(`PASS ${cases.length} native browser/profile/body boundary cases`);
} finally { await bridge?.close(); reservation.close(); }
