// Actual browser host/worker/IndexedDB boundaries; no IC or provider endpoint.
import { chromium } from 'playwright';
import { createServer } from 'node:http';
import { readFile, mkdir, writeFile } from 'node:fs/promises';
import assert from 'node:assert/strict';
import { resolve } from 'node:path';
const report = resolve(process.argv[2]);
await mkdir(report, { mode: 0o700 });
const bundle = await readFile(new URL('../../.tmp/browser/bootstrap.js', import.meta.url));
const worker = await readFile(new URL('../../.tmp/browser/publication-worker.js', import.meta.url));
const requests = [], unexpected = [], cases = [];
const server = createServer((req, res) => {
  requests.push({ method: req.method, path: req.url });
  res.setHeader('cache-control', 'no-store');
  if (req.url === '/bootstrap.js' || req.url === '/worker-entry.js') {
    res.writeHead(200, { 'content-type': 'text/javascript' });
    res.end(req.url === '/bootstrap.js' ? bundle : worker);
  } else {
    res.writeHead(200, { 'content-type': 'text/html' });
    res.end('<!doctype html><script type="module" src="/bootstrap.js"></script>');
  }
});
await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
const origin = `http://127.0.0.1:${server.address().port}`;
let context;
const deadline = setTimeout(() => { console.error('Bootstrap fixture exceeded 60 seconds'); process.exit(1); }, 60_000);
async function open() {
  context = await chromium.launchPersistentContext(`${report}/profile`, { headless: true });
  context.on('request', request => {
    if (new URL(request.url()).origin !== origin) unexpected.push(request.url());
  });
  await context.route('**/*', route => new URL(route.request().url()).origin === origin ? route.continue() : route.abort());
  const page = await context.newPage();
  await page.goto(origin); await page.waitForFunction(() => !!window.bootstrapFixture);
  return page;
}
try {
  let page = await open();
  const preparation = await page.evaluate(async () => {
    const bytes = new Uint8Array(1024 * 1024 + 479).fill(17);
    bytes.set([0, 128, 255]); bytes.fill(29, 1024 * 1024);
    const results = [];
    for (const hint of ['image/png', undefined]) {
      const expected = await bootstrapFixture.prepare(bytes, hint, 'selected.bin', 'no-store');
      const backing = new Uint8Array(bytes.length + 34).fill(99);
      backing.set(bytes, 17);
      const selected = backing.subarray(17, 17 + bytes.length);
      const pending = bootstrapFixture.prepare(selected, hint, 'selected.bin', 'no-store');
      backing.fill(255);
      const actual = await pending;
      results.push({ mime: hint ?? 'sniffed', expected, actual, frozen: Object.isFrozen(actual),
        caller_bytes: backing.byteLength });
    }
    return results;
  });
  for (const result of preparation) {
    assert.deepEqual(result.actual, result.expected);
    assert.equal(result.actual.byteLength, 1024 * 1024 + 479);
    assert.equal(result.actual.maxChunkBytes, 1024 * 1024);
    assert.equal(result.frozen, true);
    assert.equal(result.caller_bytes, 1024 * 1024 + 479 + 34);
    cases.push({ name: `sdk_owned_selected_view_${result.mime}`, ...result });
  }
  for (const kind of ['ed25519', 'secp256k1']) {
    const result = await page.evaluate(async kind => {
      const f = bootstrapFixture, config = f.input(kind, `bootstrap-${kind}`);
      const host = await f.host({ workerURL: '/worker-entry.js', bootstrap: config });
      const job = f.job(config), first = host.execute(job);
      let busy;
      try { host.execute(f.job(config, 2)); } catch (error) { busy = error.code; }
      job.binding.permission.fill(255); job.binding.root = 'sha256:' + '0'.repeat(64);
      const inspected = await first;
      const missing = await host.execute(f.job(config, 2, 'recover-certificate'));
      host.close();
      return { kind, uploader: config.configuration.uploader, busy, inspected, missing };
    }, kind);
    assert.equal(result.busy, 'host-busy'); assert.equal(result.inspected.state, 'inspected');
    assert.equal(result.inspected.journal.present, false); assert.equal(result.missing.error, 'history-missing');
    assert.equal(result.inspected.retry_authorized, false); assert.equal(result.inspected.service_completion_checked, false);
    cases.push({ name: `selected_${kind}_private_port_snapshot_concurrency`, ...result });
  }
  for (const change of ['signer-kind', 'signer-json', 'key-pair', 'uploader', 'root', 'gateway', 'journal', 'extra']) {
    const result = await page.evaluate(async change => {
      const config = bootstrapFixture.input('ed25519', `refused-${change}`);
      if (change === 'signer-kind') config.signer.kind = 'delegated';
      if (change === 'signer-json') config.signer.json = 'private-do-not-return-this';
      if (change === 'key-pair') {
        const pair = JSON.parse(config.signer.json); pair[1] = '43'.repeat(32); config.signer.json = JSON.stringify(pair);
      }
      if (change === 'uploader') config.configuration.uploader = config.configuration.tenant;
      if (change === 'root') config.configuration.rootKey = new Uint8Array();
      if (change === 'gateway') config.configuration.origin = 'https://user:private-do-not-return-this@substitute.invalid/';
      if (change === 'journal') config.journal.maxSlots = 0;
      if (change === 'extra') config.retry = true;
      // Bypass the host's validation to test the independent worker boundary.
      const worker = new Worker('/worker-entry.js', { type: 'module' }), channel = new MessageChannel();
      const response = new Promise((resolve, reject) => {
        const timer = setTimeout(() => reject(new Error('bootstrap reply deadline')), 10_000);
        channel.port1.onmessage = ({ data }) => { clearTimeout(timer); resolve(data); };
      });
      worker.postMessage({ schema: 1, operation: 'private-port' }, [channel.port2]);
      channel.port1.postMessage(config);
      const reply = await response; worker.terminate(); channel.port1.close();
      const created = (await indexedDB.databases()).some(db => db.name === config.journal.database);
      return { reply, created };
    }, change);
    assert.deepEqual(result.reply, { schema: 1, event: 'bootstrap-failed', error: 'bootstrap',
      service_completion_checked: false, retry_authorized: false });
    assert.equal(result.created, false); assert(!JSON.stringify(result).includes('private-do-not-return-this'));
    cases.push({ name: `worker_refuses_${change}_before_storage`, ...result });
  }
  const snapshot = await page.evaluate(async () => {
    const f = bootstrapFixture, config = f.input('ed25519', 'body-snapshot');
    const host = await f.host({ workerURL: '/worker-entry.js', bootstrap: config });
    const backing = new Uint8Array(4096).fill(42), body = backing.subarray(0, 32);
    const sha = Array.from(new Uint8Array(await crypto.subtle.digest('SHA-256', body)), b => b.toString(16).padStart(2, '0')).join('');
    const job = { ...f.job(config, 1, 'upload'), snapshot: { body, bodySha256: sha, manifestJSON: '{}' } };
    const uploading = host.execute(job); body.fill(0);
    const reply = await uploading;
    const preservedBytes = backing.byteLength; host.close();
    return { reply, preservedBytes };
  });
  assert.equal(snapshot.reply.error, 'root'); assert.equal(snapshot.preservedBytes, 4096);
  cases.push({ name: 'bounded_body_view_owned_without_detaching_caller', ...snapshot });
  const aborts = await page.evaluate(async () => {
    const f = bootstrapFixture, config = f.input('ed25519', 'abort-before-storage');
    const signal = new AbortController(); signal.abort('private-do-not-return-this');
    const refused = async options => {
      try { await f.host(options); return 'accepted'; } catch (error) { return error.code; }
    };
    const aborted = await refused({ workerURL: '/worker-entry.js', bootstrap: config, signal: signal.signal });
    const foreign = await refused({ workerURL: 'https://substitute.invalid/worker-entry.js', bootstrap: config });
    const created = (await indexedDB.databases()).some(db => db.name === config.journal.database);
    return { aborted, foreign, created };
  });
  assert.deepEqual(aborts, { aborted: 'host-closed', foreign: 'worker-origin', created: false });
  cases.push({ name: 'abort_and_foreign_asset_refuse_before_launch', ...aborts });
  const pendingClose = await page.evaluate(async () => {
    const f = bootstrapFixture, config = f.input('ed25519', 'close-pending');
    const host = await f.host({ workerURL: '/worker-entry.js', bootstrap: config });
    const pending = host.execute(f.job(config)); host.close();
    try { await pending; return 'accepted'; } catch (error) { return error.code; }
  });
  assert.equal(pendingClose, 'host-closed');
  cases.push({ name: 'close_rejects_outstanding_job_without_retry', error: pendingClose });
  const stopped = await page.evaluate(async () => {
    const f = bootstrapFixture, config = f.input('ed25519', 'host-deadline', 'create', 1);
    const host = await f.host({ workerURL: '/worker-entry.js', bootstrap: config });
    await new Promise(resolve => setTimeout(resolve, 1200));
    try { await host.execute(f.job(config)); return 'accepted'; } catch (error) { return error.code; }
  });
  assert.equal(stopped, 'host-closed'); cases.push({ name: 'host_deadline_terminates_worker', error: stopped });
  await context.close(); page = await open();
  const reopened = await page.evaluate(async () => {
    const f = bootstrapFixture, config = f.input('ed25519', 'host-deadline', 'open');
    const host = await f.host({ workerURL: '/worker-entry.js', bootstrap: config });
    const result = await host.execute(f.job(config)); host.close(); return result;
  });
  assert.equal(reopened.state, 'inspected'); assert.equal(reopened.journal.present, false);
  cases.push({ name: 'same_profile_reopens_original_journal_after_termination', reply: reopened });
  assert.deepEqual(unexpected, []); assert(requests.every(request => request.method === 'GET'));
  const summary = { schema: 1, evidence: 'actual_chromium_private_port_indexeddb_bootstrap',
    cases, requests, unexpected, profile: `${report}/profile`, ic_requests: 0,
    provider_requests: 0, live_provider_requests: 0, paid_cycles: 0 };
  await writeFile(`${report}/summary.json`, JSON.stringify(summary, null, 2) + '\n', { flag: 'wx' });
  process.stdout.write(JSON.stringify({ passed: cases.length, profile: summary.profile, provider_requests: 0 }) + '\n');
} finally {
  clearTimeout(deadline); await context?.close(); await new Promise(resolve => server.close(resolve));
}
