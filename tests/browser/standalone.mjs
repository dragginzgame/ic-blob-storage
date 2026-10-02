// Loopback-only rehearsal; Rust owns the real standalone installation and native clients.
import { chromium } from 'playwright';
import { readFile, writeFile } from 'node:fs/promises';
import { createInterface } from 'node:readline';
import { once } from 'node:events';
import { createHash } from 'node:crypto';
import assert from 'node:assert/strict';
import { loopbackTLS, loopbackH2 } from './tls.mjs';
const config = JSON.parse(await readFile(process.argv[2], 'utf8'));
const bundle = await readFile(new URL('../../.tmp/browser/standalone.js', import.meta.url));
const control = createInterface({ input: process.stdin, crlfDelay: Infinity });
const next = async () => JSON.parse((await once(control, 'line'))[0]);
const send = value => process.stdout.write(`${JSON.stringify(value)}\n`);
const puts = [], gets = [], arrivals = [];
let tree, body, failure;
const handle = (req, res) => {
  const url = new URL(req.url, 'http://localhost');
  if (req.method === 'PUT') {
    assert.equal(req.httpVersion, '2.0');
    const chunks = []; let length = 0;
    req.on('data', bytes => {
      length += bytes.length;
      if (length > 65536) req.destroy(new Error('request limit'));
      else chunks.push(bytes);
    });
    req.on('end', () => {
      try {
        const bytes = Buffer.concat(chunks);
        arrivals.push({ path: req.url, bytes: bytes.length, sha256: createHash('sha256').update(bytes).digest('hex') });
        assert(arrivals.length <= 2);
        if (puts.length === 0) {
          assert.equal(url.pathname, '/v1/blob-tree/'); tree = JSON.parse(bytes);
          assert.equal(tree.owner, config.service); assert.equal(tree.project_id, config.project);
          assert.equal(tree.num_blob_bytes, 1024); assert(tree.auth.OwnerEgressSignature.length > 0);
        } else {
          assert.equal(url.pathname, '/v1/chunk/');
          assert.equal(url.searchParams.get('blob_hash'), tree.blob_tree.tree.hash);
          assert.equal(url.searchParams.get('owner_id'), config.service);
          assert.equal(req.headers['x-caffeine-project-id'], config.project);
          assert.deepEqual(bytes, Buffer.alloc(1024, 42)); body = bytes;
        }
        puts.push({ path: req.url, bytes: bytes.length, sha256: createHash('sha256').update(bytes).digest('hex') });
        // Reproduce the original pre-header reset after receiving the whole chunk.
        // Streaming must leave one uncertain claim without a repeated PUT arrival.
        if (config.lostFinalReply && puts.length === 2) {
          req.stream.session.destroy(); return;
        }
        res.writeHead(200, { 'content-type': 'application/json' }); res.end('{"status":"blob_complete"}');
      } catch (error) { failure = String(error); res.writeHead(500); res.end(); }
    });
    return;
  }
  if (url.pathname === '/v1/blob/') {
    try {
      assert.equal(req.httpVersion, '2.0');
      assert.equal(req.method, 'GET'); assert(body); assert(gets.length < (config.refuseFirstRead ? 3 : 2));
      assert.equal(url.searchParams.get('blob_hash'), tree.blob_tree.tree.hash);
      assert.equal(url.searchParams.get('owner_id'), config.service);
      assert.equal(url.searchParams.get('project_id'), config.project);
      gets.push(req.url);
      if (config.refuseFirstRead && gets.length === 1) {
        req.stream.close(7); return; // NGHTTP2_REFUSED_STREAM before response headers.
      }
      const reply = Buffer.from(body);
      if (config.corruptRead) reply[0] ^= 1;
      res.writeHead(200, { 'content-type': 'image/png', 'content-length': reply.length }); res.end(reply);
    } catch (error) { failure = String(error); res.writeHead(500); res.end(); }
    return;
  }
  if (req.url === '/standalone.js') {
    res.writeHead(200, { 'content-type': 'text/javascript' }); res.end(bundle);
  } else { res.writeHead(200, { 'content-type': 'text/html' });
    res.end('<!doctype html><script type="module" src="/standalone.js"></script>'); }
};
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
  send(await page.evaluate(() => trial.plan()));
  const preparation = await next();
  send({ preparation: await page.evaluate(({ config, bytes }) => trial.prepare(config, bytes),
    { config, bytes: preparation.manifest }) });
  const grant = await next();
  const options = { ...config, binding: grant.binding, gateway: uploads.origin };
  await page.evaluate(config => trial.setup(config, 'create'), options);
  assert.equal((await page.evaluate(() => trial.inspect())).phase, 'saved');
  const saved = await page.evaluate(() => trial.inspect());
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
  assert.equal(tree.bucket_name, grant.binding.bucket);
  assert.equal(tree.project_id, grant.binding.project);
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
    try { await trial.issue(); return 'sent'; } catch (error) { return error.code; }
  }), 'dispatch-blocked');
  assert.equal(await page.evaluate(async () => {
    try { await trial.probe(); return 'sent'; } catch (error) { return error.code; }
  }), 'gateway-session');
  assert.equal(await page.evaluate(() => trial.calls()), 0);
  send({ gateway: origin, uploaded: outcome.uploaded, recovered: true, root: grant.binding.root,
    puts, providerCompletion: false, uploadFailure: outcome.failure ?? null,
    finalRequestPhase: uploaded.gateway.requests.at(-1).phase });
  assert.deepEqual(await next(), { finish: true });
  assert.equal(gets.length, config.corruptRead || config.withdrawAfterObservation ? 1 : config.refuseFirstRead ? 3 : 2);
  assert.equal(puts.length, 2); assert.equal(failure, undefined);
  assert.deepEqual(unexpected, []);
  if (config.corruptRead || config.withdrawAfterObservation) {
    const cancelled = await page.evaluate(() => trial.cancel());
    assert.deepEqual(cancelled, { ...uploaded, cancelled: true });
  } else { assert.deepEqual(await page.evaluate(() => trial.inspect()), uploaded); }
  send({ outcome: 'passed', browser: browser.version(), uploadOrigin: uploads.origin,
    downloadOrigin: origin, providerProtocol: 'h2', puts, gets, arrivals,
    uploadOutcome: outcome, journal: await page.evaluate(() => trial.inspect()),
    contentSha256: createHash('sha256').update(body).digest('hex'), provider: 'local substitute' });
} finally {
  clearTimeout(deadline); control.close(); process.stdin.pause();
  await browser.close(); await uploads.close(); await tls.close();
}
