// Actual IndexedDB persistence and adversarial checks, without an IC/provider call.
import { chromium } from 'playwright';
import { createServer } from 'node:http';
import { readFile, mkdtemp } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import assert from 'node:assert/strict';

const profile = await mkdtemp(fileURLToPath(new URL('../../.tmp/browser-store-profile-', import.meta.url)));
const bundle = await readFile(new URL('../../.tmp/browser/store.js', import.meta.url));
const server = createServer((req, res) => {
  if (req.url === '/store.js') {
    res.writeHead(200, { 'content-type': 'text/javascript' }); res.end(bundle);
  } else {
    res.writeHead(200, { 'content-type': 'text/html' });
    res.end('<!doctype html><script type="module" src="/store.js"></script>');
  }
});
await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
const origin = `http://127.0.0.1:${server.address().port}`;
let context;
const deadline = setTimeout(() => { console.error('Store evidence exceeded 60 seconds'); process.exit(1); }, 60_000);
async function launch() {
  context = await chromium.launchPersistentContext(profile, { headless: true });
  await context.route('**/*', route => new URL(route.request().url()).origin === origin ?
    route.continue() : route.abort());
}
async function page() {
  const result = await context.newPage();
  await result.goto(origin); await result.waitForFunction(() => !!window.createIntentStore);
  await result.evaluate(origin => {
    window.binding = operation => ({ key: `aaaaa-aa:aaaaa-aa:${operation}`, service: 'aaaaa-aa',
      tenant: 'aaaaa-aa', uploader: 'rrkah-fqaaa-aaaaa-aaaaq-cai', operation,
      root: `sha256:${'1'.repeat(64)}`, project: operation === '3' ? 'é'.repeat(128) : 'trial-project',
      bucket: operation === '3' ? 'β'.repeat(128) : 'trial-bucket',
      permission: [68, 73, 68, 76], icOrigin: origin, icRootKey: [1] });
    window.refusal = async fn => { try { await fn(); return 'accepted'; } catch (error) { return error.code; } };
    window.options = { database: 'maintained-trial-intents-v1', maxSlots: 3 };
    window.scope = { origin, maxRequests: 3, maxRequestBytes: 8, maxTotalRequestBytes: 4 };
    window.owner = '11111111-1111-4111-8111-111111111111';
    window.request = path => ({ url: `${origin}/${path}`, method: 'PUT', headers: [],
      bodyBytes: 2, bodySha256: '2'.repeat(64) });
    window.id = '3'.repeat(64);
  }, origin);
  return result;
}
try {
  await launch();
  const a = await page(), b = await page();
  assert.equal(await a.evaluate(() => refusal(() => createIntentStore({ ...options, mode: 'open' }))), 'store-missing');
  assert.equal(await a.evaluate(async () => (await indexedDB.databases()).some(db => db.name === options.database)), false);
  assert.equal(await a.evaluate(() => refusal(() => createIntentStore({ ...options, maxSlots: 65, mode: 'create' }))), 'configuration');
  await a.evaluate(async () => { window.store = await createIntentStore({ ...options, mode: 'create' }); });
  await b.evaluate(async () => { window.store = await createIntentStore({ ...options, mode: 'open' }); });
  assert.equal(await b.evaluate(() => refusal(() => createIntentStore({ ...options, mode: 'create' }))), 'store-exists');
  assert.equal(await b.evaluate(() => refusal(() => createIntentStore({ ...options, maxSlots: 2, mode: 'open' }))), 'store-configuration');
  await a.evaluate(async () => {
    const input = binding('1'), saving = store.save(input);
    input.permission[0] = 255; input.root = `sha256:${'9'.repeat(64)}`;
    const row = await saving; row.binding.permission[0] = 255;
  });
  assert.equal(await b.evaluate(async () => (await store.inspect(binding('1'))).binding.permission[0]), 68);
  assert.equal(await b.evaluate(() => refusal(() => store.inspect({ ...binding('1'), permission: [1] }))), 'intent-binding');
  for (const field of ['project', 'bucket']) {
    assert.equal(await b.evaluate(field => refusal(() => store.save({ ...binding('1'), [field]: 'different' })), field), 'intent-binding');
    assert.equal(await b.evaluate(field => refusal(() => store.claim({ ...binding('1'), [field]: 'different' }, [1], id)), field), 'intent-binding');
  }
  for (const [field, value] of [['project', 'β'], ['project', '\u0085'], ['bucket', '\ufeffbucket'], ['bucket', '\ud800'],
    ['project', 'é'.repeat(129)], ['bucket', 'β'.repeat(129)]]) {
    assert.equal(await a.evaluate(([field, value]) => refusal(() => store.save({ ...binding('1'), [field]: value })),
      [field, value]), 'namespace');
  }
  const claims = await Promise.all([a, b].map(p => p.evaluate(() => refusal(() => store.claim(binding('1'), [1, 2], id)))));
  assert.deepEqual(claims.sort(), ['accepted', 'dispatch-blocked']);
  await b.evaluate(async () => { await store.cancel(binding('1')); await store.observe(binding('1'), id); });
  const cancelled = await a.evaluate(() => store.inspect(binding('1')));
  assert.equal(cancelled.cancelled, true); assert.equal(cancelled.phase, 'observed');
  assert.equal(await a.evaluate(() => refusal(() => store.claimGateway(binding('1'), scope, owner, 0, request('a')))), 'gateway-blocked');
  await a.evaluate(async () => {
    await store.save(binding('2')); await store.claim(binding('2'), [1], id); await store.observe(binding('2'), id);
  });
  const beforeGateway = await b.evaluate(() => store.inspect(binding('2')));
  assert.equal(await a.evaluate(() => refusal(() => store.claimGateway(binding('2'), scope, owner, 0,
    { ...request('first'), bodyBytes: 9 }))), 'request');
  assert.equal(await a.evaluate(() => refusal(() => store.claimGateway(binding('2'), scope, owner, 0,
    { ...request('first'), url: 'https://invalid.example/first' }))), 'request');
  assert.deepEqual(await b.evaluate(() => store.inspect(binding('2'))), beforeGateway);
  await a.evaluate(() => store.claimGateway(binding('2'), scope, owner, 0, request('first')));
  assert.equal(await b.evaluate(() => refusal(() => store.claimGateway(binding('2'), scope, owner, 1, request('second')))), 'gateway-uncertain');
  await a.evaluate(async () => {
    await store.observeGateway(binding('2'), scope, owner, 0, request('first'), 200);
  });
  assert.equal(await b.evaluate(() => refusal(() => store.claimGateway(binding('2'), scope, owner, 1, request('first')))), 'gateway-repeat');
  assert.equal(await b.evaluate(() => refusal(() => store.observeGateway(binding('2'), scope, owner, 0, request('other'), 200))), 'gateway-observation');
  await a.evaluate(async () => {
    await store.claimGateway(binding('2'), scope, owner, 1, request('second'));
    await store.observeGateway(binding('2'), scope, owner, 1, request('second'), 503);
  });
  assert.equal(await b.evaluate(() => refusal(() => store.claimGateway(binding('2'), scope, owner, 2,
    { ...request('third'), bodyBytes: 0 }))), 'gateway-uncertain');
  assert.equal(await b.evaluate(() => refusal(() => store.claimGateway(binding('2'), scope, owner, 2, request('third')))), 'gateway-budget');
  await a.evaluate(async () => { await store.save(binding('3')); await store.cancel(binding('3')); });
  assert.equal(await b.evaluate(() => refusal(() => store.claim(binding('3'), [1], id))), 'dispatch-blocked');
  assert.equal(await b.evaluate(() => refusal(() => store.save(binding('4')))), 'capacity');
  const retained = await b.evaluate(() => store.inspect(binding('2')));
  const extended = await b.evaluate(() => store.inspect(binding('3')));
  await context.close(); // Entire browser process exits; reopen the same profile and origin.
  await launch();
  const c = await page();
  await c.evaluate(async () => { window.store = await createIntentStore({ ...options, mode: 'open' }); });
  assert.deepEqual(await c.evaluate(() => store.inspect(binding('2'))), retained);
  for (const field of ['project', 'bucket']) {
    assert.equal(await c.evaluate(field => refusal(() => store.inspect({ ...binding('2'), [field]: 'different' })), field), 'intent-binding');
    assert.equal(await c.evaluate(field => refusal(() => store.claimGateway({ ...binding('2'), [field]: 'different' }, scope,
      owner, 2, request('drift'))), field), 'intent-binding');
  }
  assert.deepEqual(await c.evaluate(() => store.inspect(binding('1'))), cancelled);
  assert.deepEqual(await c.evaluate(() => store.inspect(binding('3'))), extended);
  assert.equal(await c.evaluate(() => refusal(() => store.save(binding('4')))), 'capacity');
  assert.equal(await c.evaluate(() => refusal(() => store.claim(binding('2'), [1], id))), 'dispatch-blocked');
  assert.equal(await c.evaluate(() => refusal(() => store.claimGateway(binding('2'), scope,
    '22222222-2222-4222-8222-222222222222', 2, request('third')))), 'gateway-session');
  // Deliberate origin-owner tampering is a substitute, not an authentic storage fault.
  await c.evaluate(() => new Promise((resolve, reject) => {
    const opening = indexedDB.open(options.database, 1);
    opening.onsuccess = () => {
      const db = opening.result, tx = db.transaction('intents', 'readwrite');
      const records = tx.objectStore('intents'), get = records.get(binding('2').key);
      get.onsuccess = () => {
        const row = get.result; row.gateway.requests[0].phase = 'uncertain'; delete row.gateway.requests[0].status;
        records.put(row);
      };
      tx.oncomplete = () => { db.close(); resolve(); }; tx.onabort = () => { db.close(); reject(tx.error); };
    };
    opening.onerror = () => reject(opening.error);
  }));
  assert.equal(await c.evaluate(() => refusal(() => store.inspect(binding('2')))), 'intent-corrupt');
  assert.equal(await c.evaluate(() => refusal(() => store.save(binding('2')))), 'intent-corrupt');
  await c.evaluate(() => store.close());
  assert.equal(await c.evaluate(() => refusal(() => store.inspect(binding('1')))), 'store-closed');
  console.log(JSON.stringify({ outcome: 'passed', browser: context.browser().version(), profile,
    facts: ['missing-store refusal', 'immutable capacity', 'argument snapshots', 'cross-tab claim',
      'permanent cancellation', 'bounded gateway history', 'browser restart persistence',
      'immutable project/bucket', 'UTF-8 and header byte bounds', 'corrupt-history refusal'],
    limits: ['graceful restart only', 'no eviction/rollback/power-loss guarantee', 'no provider requests'] }));
} finally {
  clearTimeout(deadline); await context?.close(); await new Promise(resolve => server.close(resolve));
}
