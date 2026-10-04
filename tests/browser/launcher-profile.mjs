// Opt-in actual launcher/CDP/SDK measurements; deliberately stop before certificate intent.
import { chromium } from 'playwright';
import { Ed25519KeyIdentity } from '@icp-sdk/core/identity';
import { createServer } from 'node:net';
import { mkdir, open, writeFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { resolve, join } from 'node:path';
import assert from 'node:assert/strict';
import { attachBrowserMemory, sampleBrowserMemory } from './memory.mjs';
import { launchPublicationBrowser, LauncherRefusal } from '../../clients/browser/launcher.mjs';

const directory = resolve(process.argv[2]);
await mkdir(directory, { mode: 0o700 });
const reservation = createServer();
await new Promise(resolve => reservation.listen(0, '127.0.0.1', resolve));
const port = reservation.address().port, origin = `http://127.0.0.1:${port}`;
await new Promise(resolve => reservation.close(resolve));
const signer = Ed25519KeyIdentity.generate(new Uint8Array(32).fill(42));
const binding = { key: 'aaaaa-aa:rrkah-fqaaa-aaaaa-aaaaq-cai:1', service: 'aaaaa-aa',
  tenant: 'rrkah-fqaaa-aaaaa-aaaaq-cai', uploader: signer.getPrincipal().toText(),
  project: 'handoff-profile', bucket: 'test', operation: '1',
  root: 'sha256:' + '0'.repeat(64), permission: [68, 73, 68, 76] };
const samples = [], requests = [], blocked = [];
let session, context, memoryMonitor, active;
const engine = { async launchPersistentContext(...args) {
  context = await chromium.launchPersistentContext(...args);
  context.on('request', request => requests.push({ method: request.method(), url: request.url() }));
  await context.route('**/*', async route => {
    const target = new URL(route.request().url());
    if (target.origin === origin && ['/', '/host.js', '/worker.js'].includes(target.pathname) &&
      route.request().method() === 'GET') await route.continue();
    else { blocked.push(route.request().url()); await route.abort(); }
  });
  return {
    close: () => context.close(),
    async newPage() {
      const page = await context.newPage(); session = await context.newCDPSession(page);
      return {
        goto: (...args) => page.goto(...args),
        async evaluate(fn, argument) {
          if (active && typeof argument?.base64 === 'string' && Number.isSafeInteger(argument.offset)) {
            const bytes = Buffer.from(argument.base64, 'base64').length;
            active.cdp_frames++; active.cdp_bytes += bytes;
            active.max_frame_bytes = Math.max(active.max_frame_bytes, bytes);
          }
          if (active && argument?.action === 'upload') {
            active.page_assembled = await session.send('Runtime.getHeapUsage');
            active.worker_before_upload = await memoryMonitor.workerCommand('Runtime.getHeapUsage');
            const start = performance.now(), result = await page.evaluate(fn, argument);
            active.worker_wait_ms = performance.now() - start;
            active.worker_after_upload = await memoryMonitor.workerCommand('Runtime.getHeapUsage');
            return result;
          }
          return page.evaluate(fn, argument);
        },
      };
    },
  };
} };

let bridge;
try {
  bridge = await launchPublicationBrowser(engine, {
    profile: join(directory, 'profile'), assetPort: port, nativeSession: null,
    hostBundle: resolve('.tmp/browser/publication-host.js'),
    workerBundle: resolve('.tmp/browser/publication-worker.js'),
    bootstrap: { schema: 1, operation: 'bootstrap', signer: { kind: 'ed25519', json: JSON.stringify(signer.toJSON()) },
      configuration: { host: origin, rootKey: [1], service: binding.service, tenant: binding.tenant,
        uploader: binding.uploader, project: binding.project, bucket: binding.bucket,
        origin: 'https://substitute.invalid', maxBodyBytes: 32 * 1024 * 1024,
        maxRequests: 256, maxRequestBytes: 2 * 1024 * 1024, maxTotalRequestBytes: 64 * 1024 * 1024,
        maxJobs: 16, timeoutSeconds: 120 }, journal: { database: 'handoff-profile-v1', mode: 'create', maxSlots: 1 } },
  });
  memoryMonitor = await attachBrowserMemory(context, origin);
  let id = 0;
  for (const bytes of [1024 * 1024, 8 * 1024 * 1024, 32 * 1024 * 1024]) {
    const body = join(directory, `body-${bytes}.bin`), hash = createHash('sha256');
    const file = await open(body, 'wx', 0o600), frame = Buffer.alloc(65536, 43);
    try {
      for (let offset = 0; offset < bytes; offset += frame.length) {
        await file.writeFile(frame); hash.update(frame);
      }
      await file.sync();
    } finally { await file.close(); }
    active = { bytes, body_sha256: hash.digest('hex'), cdp_frames: 0, cdp_bytes: 0, max_frame_bytes: 0,
      page_before: await session.send('Runtime.getHeapUsage') };
    const row = active;
    // Keep an in-progress observation if a later sample or assertion fails.
    samples.push(row);
    const sample = await sampleBrowserMemory(memoryMonitor);
    row.memory = sample.result;
    const start = performance.now();
    try {
      const result = await bridge.execute({ id: ++id, index: 0, action: 'upload', transfer: {
        binding, body, bytes: String(bytes), body_sha256: row.body_sha256,
        manifest_json: '{}', preparation: {},
      } });
      row.elapsed_ms = performance.now() - start;
      assert.equal(result.state, 'failed'); assert.equal(result.error, 'root');
      row.outcome = result;
    } finally {
      await sample.stop(); active = undefined;
    }
    assert.equal(row.memory.error, null);
    const inspected = await bridge.execute({ id: ++id, index: 0, action: 'inspect', binding });
    assert.deepEqual(inspected.journal, { present: false });
    assert.equal(row.cdp_bytes, bytes); assert(row.max_frame_bytes <= 65536);
    await session.send('HeapProfiler.collectGarbage');
    await memoryMonitor.workerCommand('HeapProfiler.collectGarbage');
    row.page_after_gc = await session.send('Runtime.getHeapUsage');
    row.worker_after_gc = await memoryMonitor.workerCommand('Runtime.getHeapUsage');
    row.browser_after_gc = await memoryMonitor.browserRSS();
    await writeFile(join(directory, `sample-${bytes}.json`), JSON.stringify(row, null, 2) + '\n', { flag: 'wx', mode: 0o600 });
    console.log(JSON.stringify({ bytes, elapsed_ms: row.elapsed_ms, cdp_frames: row.cdp_frames,
      worker_wait_ms: row.worker_wait_ms, page_assembled: row.page_assembled }));
  }
  assert.deepEqual(blocked, []);
  assert(requests.every(request => request.method === 'GET' && new URL(request.url).origin === origin));
  await writeFile(join(directory, 'summary.json'), JSON.stringify({ schema: 1, samples, requests, blocked,
    chromium: await session.send('Browser.getVersion'), node: process.version, provider_requests: 0, paid_cycles: '0',
    limitations: ['All peaks are sampled lower bounds; worker replies can wait behind CPU work',
      'Summed per-process RSS can double-count shared pages; allocated memory is not live bytes',
      'Instrumented timing; root refusal before intent, not successful transfer', 'No populated journal or million-object qualification'] }, null, 2) + '\n', { flag: 'wx', mode: 0o600 });
} catch (error) {
  await writeFile(join(directory, 'failure.json'), JSON.stringify({ schema: 1, samples, requests, blocked,
    error: error instanceof LauncherRefusal ? error.code : 'fixture', diagnostic: String(error) }, null, 2) + '\n',
  { flag: 'wx', mode: 0o600 }).catch(() => {});
  throw error;
} finally {
  try { await memoryMonitor?.close(); }
  finally { await bridge?.close(); }
}
