// Loopback-only rehearsal; Rust owns the real standalone installation and native clients.
import { chromium } from 'playwright';
import { readFile, writeFile } from 'node:fs/promises';
import { createInterface } from 'node:readline';
import { once } from 'node:events';
import { createHash } from 'node:crypto';
import assert from 'node:assert/strict';
import { loopbackTLS, loopbackH2 } from './tls.mjs';
import { standaloneGateway } from './standalone-gateway.mjs';
const config = JSON.parse(await readFile(process.argv[2], 'utf8'));
const bundle = await readFile(new URL('../../.tmp/browser/standalone.js', import.meta.url));
const control = createInterface({ input: process.stdin, crlfDelay: Infinity });
const next = async () => JSON.parse((await once(control, 'line'))[0]);
const send = value => process.stdout.write(`${JSON.stringify(value)}\n`);
const { handle, state } = standaloneGateway(config, bundle, [1024], undefined,
  [Buffer.alloc(1024, config.bodyByte ?? 42)]);
const { puts, gets, arrivals } = state;
const deadline = setTimeout(() => { console.error('Standalone browser trial exceeded 90 seconds'); process.exit(1); }, 90_000);
const tls = await loopbackTLS();
const uploads = await loopbackH2(tls, handle);
await writeFile(config.providerRootCertificate, tls.root, { flag: 'wx' });
if (config.untrustedRootCertificate) {
  const untrusted = await loopbackTLS();
  try { await writeFile(config.untrustedRootCertificate, untrusted.root, { flag: 'wx' }); }
  finally { await untrusted.close(); }
}
const origin = uploads.origin;
const browser = await chromium.launch(tls.launch);
try {
  const context = await browser.newContext();
  // Observe instead of intercepting: routing can rewrite streaming upload bodies.
  const unexpected = [];
  context.on('request', request => { if (![origin, uploads.origin, new URL(config.url).origin].includes(
    new URL(request.url()).origin)) unexpected.push(request.url()); });
  const page = await context.newPage();
  const load = async () => { await page.goto(origin); await page.waitForFunction(() => !!window.trial); };
  await load();
  send(await page.evaluate(byte => trial.plan(1024, byte), config.bodyByte ?? 42));
  const grant = await next();
  const options = { ...config, binding: grant.binding, snapshot: grant.snapshot, gateway: uploads.origin };
  await page.evaluate(config => trial.setup(config, 'create'), options);
  assert.equal((await page.evaluate(() => trial.inspect())).phase, 'saved');
  const saved = await page.evaluate(() => trial.inspect());
  if (config.otherUploaderSeed !== undefined) {
    const changed = { ...options, identitySeed: config.otherUploaderSeed };
    assert.equal(await page.evaluate(async config => {
      try { await trial.setup(config, 'open'); return 'accepted'; }
      catch (error) { return error.code; }
    }, changed), 'identity');
    assert.deepEqual(await page.evaluate(() => trial.inspect()), saved);
    assert.equal(await page.evaluate(() => trial.calls()), 0);
    await page.evaluate(config => trial.setup(config, 'open'), options);
  }
  for (const field of ['project', 'bucket']) {
    const changed = { ...options, binding: { ...options.binding, [field]: 'different' } };
    assert.equal(await page.evaluate(async config => {
      try { await trial.setup(config, 'open'); return 'accepted'; }
      catch (error) { return error.code; }
    }, changed), 'intent-binding');
    assert.deepEqual(await page.evaluate(() => trial.inspect()), saved);
  }
  assert.equal(await page.evaluate(() => trial.calls()), 0);
  const outcome = await page.evaluate(async () => {
    try { return { uploaded: true, result: await trial.upload() }; }
    catch (error) { return { uploaded: false, failure: { name: error.name, message: error.message } }; }
  });
  assert.equal(outcome.uploaded, !config.lostFinalReply);
  if (outcome.uploaded) assert.deepEqual(outcome.result, { hash: grant.binding.root });
  assert.equal(state.tree.bucket_name, grant.binding.bucket);
  assert.equal(state.tree.project_id, grant.binding.project);
  const uploaded = await page.evaluate(() => trial.inspect());
  assert.equal(uploaded.phase, 'observed'); assert.equal(uploaded.cancelled, false);
  assert.equal(await page.evaluate(() => trial.calls()), 1);
  assert.equal(puts.length, 2);
  assert(puts.reduce((n, p) => n + p.bytes, 0) <= 131072);
  for (const [index, entry] of uploaded.gateway.requests.entries()) {
    const uncertain = config.lostFinalReply && index === 1;
    assert.equal(entry.phase, uncertain ? 'uncertain' : 'responded');
    assert.equal(entry.status, uncertain ? undefined : 200);
    assert.equal(entry.request.bodyBytes, puts[index].bytes);
    assert.equal(entry.request.bodySha256, puts[index].sha256);
  }
  await load(); await page.evaluate(config => trial.setup(config, 'open'), options);
  const recovered = await page.evaluate(() => trial.recover());
  assert.deepEqual(recovered.row, uploaded); assert(recovered.certificateBytes > 0);
  assert.equal(await page.evaluate(async () => {
    try { await trial.upload(); return 'sent'; } catch (error) { return error.code; }
  }), 'upload-claimed');
  assert.equal(await page.evaluate(() => trial.calls()), 0);
  send({ gateway: origin, uploaded: outcome.uploaded, recovered: true, root: grant.binding.root,
    puts, providerCompletion: false, uploadFailure: outcome.failure ?? null,
    finalRequestPhase: uploaded.gateway.requests.at(-1).phase });
  assert.deepEqual(await next(), { finish: true });
  assert.equal(gets.length, config.corruptRead || config.withdrawAfterObservation ? 1 : config.refuseFirstRead ? 3 : 2);
  assert.equal(puts.length, 2); assert.equal(state.failure, undefined);
  assert.deepEqual(unexpected, []);
  if (config.corruptRead || config.withdrawAfterObservation) {
    const cancelled = await page.evaluate(() => trial.cancel());
    assert.deepEqual(cancelled, { ...uploaded, cancelled: true });
  } else { assert.deepEqual(await page.evaluate(() => trial.inspect()), uploaded); }
  send({ outcome: 'passed', browser: browser.version(), uploadOrigin: uploads.origin,
    downloadOrigin: origin, providerProtocol: 'h2', puts, gets, arrivals,
    uploadOutcome: outcome, journal: await page.evaluate(() => trial.inspect()),
    contentSha256: createHash('sha256').update(state.body).digest('hex'), provider: 'local substitute' });
} finally {
  clearTimeout(deadline); control.close(); process.stdin.pause();
  await browser.close(); await uploads.close(); await tls.close();
}
