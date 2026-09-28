// Browser test driver only. Rust owns the PocketIC instance and installation.
import { chromium } from 'playwright';
import { createServer } from 'node:http';
import { readFile } from 'node:fs/promises';
import assert from 'node:assert/strict';

const [major, minor] = process.versions.node.split('.').map(Number);
assert(major > 20 || (major === 20 && minor >= 19), 'Browser evidence requires Node >=20.19.0');
const config = JSON.parse(await readFile(process.argv[2], 'utf8'));
const deadline = setTimeout(() => { console.error('Browser fixture exceeded 60 seconds'); process.exit(1); }, 60_000);
const bundle = await readFile(new URL('../../.tmp/browser/client.js', import.meta.url));
const server = createServer((req, res) => {
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
try {
  const context = await browser.newContext();
  // Browser traffic is confined to the owned page and PocketIC endpoint.
  await context.route('**/*', route => {
    const url = new URL(route.request().url());
    return [origin, new URL(config.url).origin].includes(url.origin) ? route.continue() : route.abort();
  });
  const load = async page => {
    await page.goto(origin);
    await page.waitForFunction(() => !!window.fixture);
    await page.evaluate(c => fixture.setup(c), config);
  };
  const a = await context.newPage(), b = await context.newPage();
  await load(a); await load(b);
  // An actual aborted write transaction must resolve before any HTTP dispatch.
  await a.evaluate(() => fixture.abortClaim(true));
  assert.equal(await a.evaluate(async () => {
    try { await fixture.issue(); return 'sent'; } catch { return 'blocked'; }
  }), 'blocked');
  assert.equal(await a.evaluate(() => fixture.calls()), 0);
  const unsent = await b.evaluate(() => fixture.inspect());
  assert.equal(unsent.phase, 'saved'); assert.equal(unsent.envelope, undefined);
  await a.evaluate(() => fixture.abortClaim(false));
  if (config.holdResponse) {
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
    assert.deepEqual(outcomes.sort(), ['blocked', 'observed']);
    const counts = await Promise.all([a, b].map(page => page.evaluate(() => fixture.calls())));
    assert.equal(counts.reduce((x, y) => x + y), 1);
  }
  const before = await b.evaluate(() => fixture.inspect());
  await load(b); // Actual document reload, retaining IndexedDB.
  const recovered = await b.evaluate(() => fixture.recover());
  assert.equal(recovered.requestId, before.requestId);
  assert.deepEqual(recovered.envelope, before.envelope);
  assert.equal(recovered.phase, 'observed');
  assert.equal(recovered.cancelled, !!config.holdResponse);
  assert.equal(await b.evaluate(() => fixture.calls()), 0);
  const proofState = await b.evaluate(() => fixture.inspect());
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
    const stale = await code(() => fixture.observe(row.key, 'unrelated'));
    await fixture.save({ ...row.binding, key: 'second' });
    const capacity = await code(() => fixture.save({ ...row.binding, key: 'third' }));
    return { conflict, stale, capacity };
  });
  assert.deepEqual(checks, { conflict: 'conflict', stale: 'observation-binding', capacity: 'capacity' });
  console.log(JSON.stringify({ browser: browser.version(), outcome: 'passed', cancelled: recovered.cancelled }));
} finally {
  clearTimeout(deadline);
  await browser.close();
  await new Promise(resolve => server.close(resolve));
}
