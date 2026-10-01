// Browser test driver only. Rust owns the PocketIC instance and installation.
import { chromium } from 'playwright';
import { createServer } from 'node:http';
import { readFile } from 'node:fs/promises';
import assert from 'node:assert/strict';
import { createInterface } from 'node:readline';
import { once } from 'node:events';
import { createHash } from 'node:crypto';
import { refusedCertificate } from './refused-certificate.mjs';

const [major, minor] = process.versions.node.split('.').map(Number);
assert(major > 20 || (major === 20 && minor >= 19), 'Browser evidence requires Node >=20.19.0');
const config = JSON.parse(await readFile(process.argv[2], 'utf8'));
const deadline = setTimeout(() => { console.error('Browser fixture exceeded 60 seconds'); process.exit(1); }, 60_000);
const bundle = await readFile(new URL('../../.tmp/browser/client.js', import.meta.url));
const gateway = [];
const server = createServer((req, res) => {
  if (req.method === 'PUT') {
    const chunks = [];
    req.on('data', data => chunks.push(data));
    req.on('end', () => {
      gateway.push({ url: req.url, body: Buffer.concat(chunks), headers: req.headers });
      res.writeHead(config.gatewayFailure ? 503 : 200, { 'content-type': 'application/json' });
      res.end(config.oversizeGateway ? 'x'.repeat(64 * 1024 + 1) : JSON.stringify({ status: 'blob_complete' }));
    });
    return;
  }
  if (req.url === '/client.js') {
    res.writeHead(200, { 'content-type': 'text/javascript' }); res.end(bundle);
  } else {
    res.writeHead(200, { 'content-type': 'text/html' });
    res.end('<!doctype html><script type="module" src="/client.js"></script>');
  }
});
await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
const origin = `http://127.0.0.1:${server.address().port}`;
const browser = await chromium.launch({ headless: true });
async function exercise() {
  const context = await browser.newContext();
  // Browser traffic is confined to the owned page and PocketIC endpoint.
  await context.route('**/*', route => {
    const url = new URL(route.request().url());
    return [origin, new URL(config.url).origin].includes(url.origin) ? route.continue() : route.abort();
  });
  const load = async page => {
    await page.goto(origin);
    await page.waitForFunction(() => !!window.fixture);
    await page.evaluate(c => fixture.setup(c), { ...config, gateway: origin });
  };
  const a = await context.newPage(), b = await context.newPage();
  await a.goto(origin);
  await a.waitForFunction(() => !!window.fixture);
  const upstreamPlan = await a.evaluate(c => fixture.plan(c), config);
  assert.deepEqual(gateway, []);
  // Rust converts the actual declaration into opaque Candid. The browser signs
  // consumer admission and uploader preparation; Rust checks replies before grant.
  const control = createInterface({ input: process.stdin, crlfDelay: Infinity });
  const commandsReady = once(control, 'line');
  process.stdout.write(`${JSON.stringify(upstreamPlan)}\n`);
  const [commandsLine] = await commandsReady;
  const grant = once(control, 'line');
  const replies = await a.evaluate(({ config, commands }) => fixture.admitAndPrepare(config, commands),
    { config, commands: JSON.parse(commandsLine) });
  assert.deepEqual(gateway, []);
  process.stdout.write(`${JSON.stringify(replies)}\n`);
  const [line] = await grant;
  if (!config.certificateBlocked) { control.close(); process.stdin.pause(); }
  const admitted = JSON.parse(line);
  assert.ok(Array.isArray(admitted.permission));
  Object.assign(config, admitted);
  await a.evaluate(c => fixture.setup(c), { ...config, gateway: origin });
  await load(b);
  const plan = await a.evaluate(() => fixture.preparation());
  assert.equal(plan.hash, config.root);
  assert.equal(plan.byteLength, 10);
  const manifest = JSON.parse(plan.manifestJSON);
  assert.equal(manifest.tree.hash, config.root);
  assert.deepEqual(manifest.headers, ['Content-Length: 10', 'Content-Type: image/png']);
  assert.equal(await a.evaluate(() => fixture.calls()), 0);
  assert.deepEqual(gateway, []);
  if (config.certificateBlocked) {
    await refusedCertificate({ a, b, load, gateway, config, control, browser });
    return;
  }
  for (const [kind, code] of [['origin', 'request'], ['size', 'request-size'], ['ordinary', 'gateway-blocked']]) {
    assert.equal(await a.evaluate(async kind => {
      try { await fixture.gatewayProbe(kind); return 'sent'; } catch (error) { return error.code; }
    }, kind), code);
  }
  assert.deepEqual(gateway, []);
  assert.equal(await a.evaluate(async () => {
    try { await fixture.rejectClonedHandle(); return 'sent'; } catch (error) { return error.name; }
  }), 'TypeError');
  await a.evaluate(() => fixture.abortUpload());
  assert.equal(await a.evaluate(async () => {
    try { await fixture.issue(); return 'sent'; } catch (error) { return error.name; }
  }), 'AbortError');
  assert.equal(await a.evaluate(() => fixture.calls()), 0);
  await load(a);
  for (const kind of ['identity', 'operation']) {
    assert.equal(await a.evaluate(async kind => {
      try { await fixture.rejectSetup(kind); return 'accepted'; } catch (error) { return error.code; }
    }, kind), kind);
  }
  // An actual aborted write transaction must resolve before any HTTP dispatch.
  await a.evaluate(() => fixture.abortClaim(true));
  assert.equal(await a.evaluate(async () => {
    try { await fixture.issue(); return 'sent'; } catch { return 'blocked'; }
  }), 'blocked');
  assert.equal(await a.evaluate(() => fixture.calls()), 0);
  const unsent = await b.evaluate(() => fixture.inspect());
  assert.equal(unsent.phase, 'saved'); assert.equal(unsent.envelope, undefined);
  await a.evaluate(() => fixture.abortClaim(false));
  if (config.holdGateway) {
    await a.evaluate(() => { window.outcome = fixture.issue().then(() => 'observed', () => 'failed'); });
    await a.waitForFunction(() => fixture.gatewayHeld());
    const pending = await b.evaluate(() => fixture.inspect());
    assert.equal(pending.gateway.requests.length, 1);
    assert.equal(pending.gateway.requests[0].phase, 'uncertain');
    // Another tab cannot acquire a new gateway session, even with observed issuance.
    assert.equal(await b.evaluate(async () => {
      try { await fixture.gatewayProbe(); return 'sent'; } catch (error) { return error.code; }
    }), 'gateway-session');
    await b.evaluate(() => fixture.cancel());
    if (config.lateGateway) {
      await a.evaluate(() => fixture.releaseGateway());
      assert.equal(await a.evaluate(() => window.outcome), 'failed');
      const late = await b.evaluate(() => fixture.inspect());
      assert.equal(late.gateway.requests[0].phase, 'responded');
      assert.equal(late.cancelled, true);
    } else {
      await a.close();
    }
  } else if (config.holdResponse) {
    await a.evaluate(() => { window.outcome = fixture.issue().then(() => 'observed', () => 'failed'); });
    await a.waitForFunction(() => fixture.held());
    await b.evaluate(() => fixture.cancel());
    // Destroy the execution while its reply is held; only the IndexedDB intent survives.
    await a.close();
    const pending = await b.evaluate(() => fixture.inspect());
    assert.equal(pending.phase, 'uncertain'); assert.equal(pending.cancelled, true);
  } else {
    const outcomes = await Promise.all([a, b].map(page => page.evaluate(async () => {
      try { await fixture.issue(); return 'observed'; } catch { return 'blocked'; }
    })));
    const failedTransfer = config.gatewayFailure || config.abortAfterTree || config.gatewayWriteAbort ||
      config.gatewayObserveAbort || config.oversizeGateway || config.cancelAtGatewayClaim;
    assert.deepEqual(outcomes.sort(), failedTransfer ? ['blocked', 'blocked'] : ['blocked', 'observed']);
    const counts = await Promise.all([a, b].map(page => page.evaluate(() => fixture.calls())));
    assert.equal(counts.reduce((x, y) => x + y), 1);
  }
  if (!config.holdResponse && !config.holdGateway && !config.abortAfterTree) {
    assert.equal(await a.evaluate(async () => {
      try { await fixture.repeatHandle(); return 'sent'; } catch (error) { return error.name; }
    }), 'TypeError');
    // Exhausted, failed and uncertain transfers cannot send another request even
    // through the still-live hook; the competing tab cannot acquire its session.
    const beforeProbes = await b.evaluate(() => fixture.inspect());
    const refused = await Promise.all([a, b].map(page => page.evaluate(async () => {
      try { await fixture.gatewayProbe(); return 'sent'; } catch (error) { return error.code; }
    })));
    const expected = config.cancelAtGatewayClaim ? ['gateway-blocked', 'gateway-blocked'] :
      config.gatewayWriteAbort ? ['fixture-write-abort', 'fixture-write-abort'] :
      ['gateway-session', config.gatewayFailure ? 'gateway-uncertain' : 'gateway-capacity'];
    assert.deepEqual(refused.sort(), expected.sort());
    assert.deepEqual(await b.evaluate(() => fixture.inspect()), beforeProbes);
  }
  const before = await b.evaluate(() => fixture.inspect());
  await load(b); // Actual document reload, retaining IndexedDB.
  const recovery = await b.evaluate(async () => {
    try { return { row: await fixture.recover() }; }
    catch (error) { return { error: String(error), stack: error?.stack }; }
  });
  assert.ok(recovery?.row, JSON.stringify(recovery));
  const recovered = recovery.row;
  assert.equal(recovered.requestId, before.requestId);
  assert.deepEqual(recovered.envelope, before.envelope);
  assert.equal(recovered.phase, 'observed');
  assert.equal(recovered.cancelled, !!(config.holdResponse || config.holdGateway || config.cancelAtGatewayClaim));
  assert.deepEqual(recovered.gateway, before.gateway);
  assert.equal(await b.evaluate(() => fixture.calls()), 0);
  if (recovered.gateway || recovered.cancelled) {
    assert.equal(await b.evaluate(async () => {
      try { await fixture.gatewayProbe(); return 'sent'; } catch (error) { return error.code; }
    }), recovered.cancelled ? 'gateway-blocked' : 'gateway-session');
    assert.equal(await b.evaluate(() => fixture.gatewayCalls()), 0);
  }
  const proofState = await b.evaluate(() => fixture.inspect());
  const reads = await b.evaluate(() => fixture.reads());
  for (const [kind, code] of [['binding', 'intent-binding'], ['trust', 'intent-binding'],
    ['phase', 'intent-state'], ['envelope', 'envelope']]) {
    assert.equal(await b.evaluate(async kind => {
      try { await fixture.rejectRetained(kind); return 'accepted'; } catch (error) { return error.code; }
    }, kind), code);
    assert.deepEqual(await b.evaluate(() => fixture.inspect()), proofState);
    assert.equal(await b.evaluate(() => fixture.reads()), reads);
  }
  for (const kind of ['request', 'size', 'signature']) {
    assert.equal(await b.evaluate(async kind => {
      try { await fixture.rejectProof(kind); return 'accepted'; } catch { return 'rejected'; }
    }, kind), 'rejected');
    assert.deepEqual(await b.evaluate(() => fixture.inspect()), proofState);
  }
  assert.equal(await b.evaluate(async () => {
    try { await fixture.issue(); return 'sent'; } catch { return 'blocked'; }
  }), 'blocked');
  assert.equal(await b.evaluate(() => fixture.calls()), 0);
  // Changed bindings cannot overwrite uncertainty, and capacity retains tombstones.
  const checks = await b.evaluate(async () => {
    const row = await fixture.inspect();
    const code = async fn => { try { await fn(); return 'unexpected'; } catch (e) { return e.code; } };
    const conflict = await code(() => fixture.save({ ...row.binding, root: 'changed' }));
    const stale = await code(() => fixture.observe(row.binding, 'unrelated'));
    await fixture.save({ ...row.binding, key: 'second' });
    const capacity = await code(() => fixture.save({ ...row.binding, key: 'third' }));
    return { conflict, stale, capacity };
  });
  assert.deepEqual(checks, { conflict: 'conflict', stale: 'observation-binding', capacity: 'capacity' });
  if (config.holdResponse || config.gatewayWriteAbort || config.cancelAtGatewayClaim) {
    assert.deepEqual(gateway, []);
    assert.equal(recovered.gateway, undefined);
  } else {
    assert.equal(gateway.length, config.gatewayFailure || config.abortAfterTree || config.holdGateway ||
      config.gatewayObserveAbort || config.oversizeGateway ? 1 : 2);
    assert.equal(recovered.gateway.requests.length, gateway.length);
    for (const [i, observed] of gateway.entries()) {
      const entry = recovered.gateway.requests[i];
      assert.equal(entry.request.url, `${origin}${observed.url}`);
      assert.equal(entry.request.method, 'PUT');
      assert.equal(entry.request.bodyBytes, observed.body.length);
      assert.equal(entry.request.bodySha256, createHash('sha256').update(observed.body).digest('hex'));
      for (const [name, value] of entry.request.headers) assert.equal(observed.headers[name], value);
      const uncertain = (config.holdGateway && !config.lateGateway) || config.gatewayObserveAbort || config.oversizeGateway;
      assert.equal(entry.phase, uncertain ? 'uncertain' : 'responded');
      assert.equal(entry.status, uncertain ? undefined : config.gatewayFailure ? 503 : 200);
    }
    assert.equal(gateway[0].url, '/v1/blob-tree/');
    const tree = JSON.parse(gateway[0].body);
    assert.deepEqual(tree.blob_tree, manifest);
    assert.equal(tree.blob_tree.tree.hash, config.root);
    assert.equal(tree.owner, config.service);
    assert.equal(tree.project_id, 'fixture-project');
    assert.equal(tree.num_blob_bytes, 10);
    assert.ok(tree.auth.OwnerEgressSignature.length > 0);
    if (gateway.length === 2) {
      const chunk = new URL(gateway[1].url, origin);
      assert.equal(chunk.pathname, '/v1/chunk/');
      assert.equal(chunk.searchParams.get('blob_hash'), config.root);
      assert.equal(chunk.searchParams.get('owner_id'), config.service);
      assert.deepEqual(gateway[1].body, Buffer.alloc(10, config.content));
    }
  }
  const beforeConsumer = await b.evaluate(() => fixture.inspect());
  const gatewayCount = gateway.length;
  const consumer = await b.evaluate(({ config, cancelled }) => fixture.finishConsumer(config, cancelled),
    { config, cancelled: recovered.cancelled });
  assert.deepEqual(await b.evaluate(() => fixture.inspect()), beforeConsumer);
  assert.equal(gateway.length, gatewayCount);
  console.log(JSON.stringify({ browser: browser.version(), outcome: 'passed', cancelled: recovered.cancelled, consumer }));
}
try { await exercise(); } finally {
  clearTimeout(deadline);
  await browser.close();
  await new Promise(resolve => server.close(resolve));
}
