// Rust coordinates authenticated native phases; this driver owns only the browser.
import { chromium } from 'playwright';
import { readFile, writeFile } from 'node:fs/promises';
import { createInterface } from 'node:readline';
import { once } from 'node:events';
import assert from 'node:assert/strict';
import { loopbackTLS, loopbackH2 } from './tls.mjs';
import { standaloneGateway } from './standalone-gateway.mjs';
const config = JSON.parse(await readFile(process.argv[2], 'utf8'));
const bundle = await readFile(new URL('../../.tmp/browser/standalone.js', import.meta.url));
const workerBundle = await readFile(new URL('../../.tmp/browser/publication-worker.js', import.meta.url));
const sizes = [1024, 2048];
const control = createInterface({ input: process.stdin, crlfDelay: Infinity });
const next = async () => JSON.parse((await once(control, 'line'))[0]);
const send = value => process.stdout.write(`${JSON.stringify(value)}\n`);
const { handle, state } = standaloneGateway(config, bundle, sizes, workerBundle);
const tls = await loopbackTLS(), gateway = await loopbackH2(tls, handle);
await writeFile(config.providerRootCertificate, tls.root, { flag: 'wx' });
const unexpected = [], journals = [], originals = [], workerReports = [];
let context, page;
async function open() {
  context = await chromium.launchPersistentContext(config.profile, tls.launch);
  context.on('request', request => {
    if (![gateway.origin, new URL(config.url).origin].includes(new URL(request.url()).origin))
      unexpected.push(request.url());
  });
  page = await context.newPage();
  await page.goto(gateway.origin); await page.waitForFunction(() => !!window.trial);
}
const deadline = setTimeout(() => { console.error('Serial browser trial exceeded 180 seconds'); process.exit(1); }, 180_000);
try {
  await open();
  send({ gateway: gateway.origin,
    files: await page.evaluate(async sizes => Promise.all(sizes.map(size => trial.plan(size))), sizes) });
  for (const index of [0, 1]) {
    const grant = await next();
    if (grant.finish) { assert(config.corruptRead && index === 1); break; }
    assert.equal(grant.index, index);
    const options = { ...config, ...grant, gateway: gateway.origin,
      maxBodyBytes: 2048, journalSlots: 2 };
    originals.push(options);
    await page.evaluate(options => trial.setup(options, options.index === 0 ? 'create' : 'open'), options);
    const outcome = await page.evaluate(async () => {
      try { return { uploaded: true, result: await trial.upload() }; }
      catch (error) { return { uploaded: false, failure: { name: error.name, message: error.message } }; }
    });
    const lost = config.lostFinalReply && index === 0;
    assert.equal(outcome.uploaded, !lost);
    assert.equal(await page.evaluate(() => trial.calls()), 1);
    if (config.worker && config.corruptRead) await page.evaluate(() => trial.cancel());
    const row = await page.evaluate(() => trial.inspect());
    assert.equal(row.phase, 'observed'); assert.equal(row.gateway.requests.length, 2);
    assert.equal(row.gateway.requests[1].phase, lost ? 'uncertain' : 'responded');
    if (config.worker && config.corruptRead) assert.equal(row.cancelled, true);
    journals.push(row);
    // Whole-browser restart reopens the exact original strict IndexedDB history.
    workerReports.push(...await page.evaluate(() => trial.workerReports()));
    await context.close(); await open();
    await page.evaluate(options => trial.setup(options, 'open'), options);
    const recovered = await page.evaluate(() => trial.recover());
    assert.deepEqual(recovered.row, row); assert(recovered.certificateBytes > 0);
    assert.equal(await page.evaluate(async () => {
      try { await trial.upload(); return 'sent'; } catch (error) { return error.code; }
    }), 'upload-claimed');
    assert.equal(await page.evaluate(() => trial.calls()), 0);
    send({ index, gateway: gateway.origin, uploaded: outcome.uploaded,
      finalRequestPhase: row.gateway.requests[1].phase, browserRestarted: true });
    if (index === 1) assert.deepEqual(await next(), { finish: true });
  }
  assert.equal(state.failure, undefined); assert.deepEqual(unexpected, []);
  assert.equal(state.puts.length, config.corruptRead ? 2 : 4);
  assert.equal(state.gets.length, config.corruptRead ? 1 : 4);
  // File two never replaces file one's original uncertain/observed journal.
  for (const [index, options] of originals.entries()) {
    await page.evaluate(options => trial.setup(options, 'open'), options);
    assert.deepEqual(await page.evaluate(() => trial.inspect()), journals[index]);
  }
  assert.equal(await page.evaluate(() => trial.calls()), 0);
  workerReports.push(...await page.evaluate(() => trial.workerReports()));
  for (const report of workerReports) {
    assert.equal(report.service_completion_checked, false); assert.equal(report.retry_authorized, false);
    for (const sensitive of ['envelope', 'permission', 'authorization', 'requestId'])
      assert(!JSON.stringify(report).includes(sensitive));
  }
  send({ outcome: 'passed', browser: context.browser().version(),
    provider: 'local HTTPS HTTP/2 substitute', puts: state.puts, gets: state.gets,
    arrivals: state.arrivals, journals, browserRestarted: true,
    workerReports,
    uploadRetries: 0, liveProviderRequests: 0, paidEffects: 0 });
} finally {
  clearTimeout(deadline); control.close(); process.stdin.pause();
  await context?.close(); await gateway.close(); await tls.close();
}
