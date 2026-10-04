// Opt-in actual journal scaling; synthetic history, no certificate/provider dispatch.
import { chromium } from 'playwright';
import { createServer } from 'node:http';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve, join } from 'node:path';
import { createHash } from 'node:crypto';
import assert from 'node:assert/strict';

const directory = resolve(process.argv[2]);
await mkdir(directory, { mode: 0o700 });
const record = (name, value) => writeFile(join(directory, name), JSON.stringify(value, null, 2) + '\n',
  { flag: 'wx', mode: 0o600 });
const bundle = await readFile(new URL('../../.tmp/browser/store.js', import.meta.url));
const capacities = [675, 5000, 10000], populated = [], reopened = [], requests = [], blocked = [];
const server = createServer((request, response) => {
  response.setHeader('cache-control', 'no-store');
  if (request.method !== 'GET') { response.writeHead(403); response.end(); return; }
  if (request.url === '/store.js') {
    response.writeHead(200, { 'content-type': 'text/javascript' }); response.end(bundle);
  } else if (request.url === '/') {
    response.writeHead(200, { 'content-type': 'text/html' });
    response.end('<!doctype html><script type="module" src="/store.js"></script>');
  } else { response.writeHead(404); response.end(); }
});
await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
const origin = `http://127.0.0.1:${server.address().port}`, profile = join(directory, 'profile');
let context, page, expired = false;
const deadline = setTimeout(() => { expired = true; void context?.close().catch(() => {}); }, 180_000);
async function launch() {
  assert(!expired, 'fixture deadline');
  context = await chromium.launchPersistentContext(profile, { headless: true, timeout: 30_000 });
  context.on('request', request => requests.push({ method: request.method(), url: request.url() }));
  await context.route('**/*', route => {
    const url = new URL(route.request().url());
    if (url.origin === origin && ['/', '/store.js'].includes(url.pathname) && route.request().method() === 'GET')
      return route.continue();
    blocked.push(route.request().url()); return route.abort();
  });
  page = await context.newPage(); await page.goto(origin);
  await page.waitForFunction(() => typeof createIntentStore === 'function');
  await page.evaluate(origin => {
    globalThis.profileBinding = index => ({ key: `aaaaa-aa:aaaaa-aa:${index}`, service: 'aaaaa-aa',
      tenant: 'aaaaa-aa', uploader: 'rrkah-fqaaa-aaaaa-aaaaq-cai', operation: String(index),
      root: 'sha256:' + index.toString(16).padStart(64, '0'), project: 'journal-profile', bucket: 'test',
      permission: [68, 73, 68, 76], icOrigin: origin, icRootKey: [1] });
    globalThis.profileOptions = capacity => ({ database: `journal-profile-${capacity}`, maxSlots: capacity });
    globalThis.profileScope = { origin, maxRequests: 3, maxRequestBytes: 8, maxTotalRequestBytes: 24 };
    globalThis.profileOwner = '11111111-1111-4111-8111-111111111111';
    globalThis.profileRequest = index => ({ url: `${origin}/opaque/${index}`, method: 'PUT', headers: [],
      bodyBytes: 2, bodySha256: '2'.repeat(64) });
    globalThis.profileId = '3'.repeat(64);
    globalThis.profileRefusal = async operation => {
      try { await operation(); return 'accepted'; } catch (error) { return error.code; }
    };
    globalThis.profileReads = async capacity => {
      const durations = [];
      for (let sample = 0; sample < 30; sample++) {
        const index = 1 + ((sample * 7919) % capacity), start = performance.now();
        const row = await profileStore.inspect(profileBinding(index));
        if (row?.key !== profileBinding(index).key) throw new Error('missing selected row');
        durations.push(performance.now() - start);
      }
      return durations;
    };
  }, origin);
}
try {
  await launch();
  for (const capacity of capacities) {
    const start = performance.now();
    await page.evaluate(async capacity => {
      globalThis.profileStore = await createIntentStore({ ...profileOptions(capacity), mode: 'create' });
    }, capacity);
    const creationMs = performance.now() - start, blocks = [];
    for (let first = 1; first <= capacity; first += 500) {
      const last = Math.min(capacity, first + 499);
      const block = await page.evaluate(async ({ first, last }) => {
        const start = performance.now();
        for (let index = first; index <= last; index++) await profileStore.save(profileBinding(index));
        return { first, last, elapsed_ms: performance.now() - start };
      }, { first, last });
      blocks.push(block);
      await record(`population-${capacity}-${last}.json`, block);
      console.log(JSON.stringify({ capacity, ...block }));
    }
    const history = await page.evaluate(async capacity => {
      await profileStore.claim(profileBinding(1), [1, 2], profileId);
      await profileStore.cancel(profileBinding(Math.floor(capacity / 2)));
      await profileStore.claim(profileBinding(capacity), [3, 4], profileId);
      await profileStore.observe(profileBinding(capacity), profileId);
      await profileStore.claimGateway(profileBinding(capacity), profileScope, profileOwner, 0, profileRequest(0));
      await profileStore.observeGateway(profileBinding(capacity), profileScope, profileOwner, 0, profileRequest(0), 200);
      await profileStore.claimGateway(profileBinding(capacity), profileScope, profileOwner, 1, profileRequest(1));
      return Promise.all([1, Math.floor(capacity / 2), capacity].map(index => profileStore.inspect(profileBinding(index))));
    }, capacity);
    assert.equal(history[0].phase, 'uncertain'); assert.equal(history[1].cancelled, true);
    assert.equal(history[2].gateway.requests[1].phase, 'uncertain');
    const reads = await page.evaluate(capacity => profileReads(capacity), capacity);
    const cdp = await context.newCDPSession(page), heap = await cdp.send('Runtime.getHeapUsage'); await cdp.detach();
    const sample = { capacity, creation_ms: creationMs, population_blocks: blocks,
      population_ms: blocks.reduce((sum, block) => sum + block.elapsed_ms, 0), reads_ms: reads, history, page_heap: heap };
    populated.push(sample); await record(`populated-${capacity}.json`, sample);
    await page.evaluate(() => profileStore.close());
    // Finish this capacity's restart evidence before a larger workload can fail.
    await context.close(); await launch(); // Same original profile and origin, new Chromium process.
    const openingStart = performance.now();
    await page.evaluate(async capacity => {
      globalThis.profileStore = await createIntentStore({ ...profileOptions(capacity), mode: 'open' });
    }, capacity);
    const openingMs = performance.now() - openingStart;
    const result = await page.evaluate(async capacity => ({
      history: await Promise.all([1, Math.floor(capacity / 2), capacity].map(index => profileStore.inspect(profileBinding(index)))),
      reads_ms: await profileReads(capacity),
      overflow: await profileRefusal(() => profileStore.save(profileBinding(capacity + 1))),
      repeat: await profileRefusal(() => profileStore.claim(profileBinding(1), [1, 2], profileId)),
      cancelled: await profileRefusal(() => profileStore.claim(profileBinding(Math.floor(capacity / 2)), [1], profileId)),
      gateway: await profileRefusal(() => profileStore.claimGateway(profileBinding(capacity), profileScope, profileOwner, 2, profileRequest(2))),
      existing: await profileStore.save(profileBinding(capacity)),
    }), capacity);
    const expected = populated.find(sample => sample.capacity === capacity).history;
    assert.deepEqual(result.history, expected); assert.deepEqual(result.existing, expected[2]);
    assert.equal(result.overflow, 'capacity'); assert.equal(result.repeat, 'dispatch-blocked');
    assert.equal(result.cancelled, 'dispatch-blocked'); assert.equal(result.gateway, 'gateway-uncertain');
    const restored = { capacity, opening_ms: openingMs, ...result };
    reopened.push(restored); await record(`reopened-${capacity}.json`, restored);
    await page.evaluate(() => profileStore.close());
    console.log(JSON.stringify({ capacity, opening_ms: openingMs, result: 'preserved' }));
  }
  assert(!expired); assert.deepEqual(blocked, []);
  await record('summary.json', { schema: 1, outcome: 'pass', populated, reopened, requests, blocked,
    chromium: context.browser().version(), node: process.version, profile, origin,
    store_bundle_sha256: createHash('sha256').update(bundle).digest('hex'), live_ic_requests: 0, live_provider_requests: 0, paid_cycles: '0',
    limitations: ['Synthetic compact records and opaque envelopes; no certificate or provider effect',
      'Single instrumented local pass, not timing thresholds or million-row qualification',
      'Graceful process restart, not power loss, eviction, profile rollback or cold OS cache',
      'Page heap excludes browser native storage/RSS; no canister reopen measurement'] });
} catch (error) {
  await record('failure.json', { schema: 1, outcome: 'failed', expired, completed_capacities: populated.map(row => row.capacity),
    reopened_capacities: reopened.map(row => row.capacity), error: typeof error?.code === 'string' ? error.code : 'fixture',
    requests, blocked }).catch(() => {});
  throw error;
} finally {
  clearTimeout(deadline); await context?.close(); await new Promise(resolve => server.close(resolve));
}
