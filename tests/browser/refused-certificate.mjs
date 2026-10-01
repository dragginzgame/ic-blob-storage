// Actual managed host refusal: no fixture facts turn certificate exposure on.
import assert from 'node:assert/strict';
import { once } from 'node:events';

export async function refusedCertificate({ a, b, load, gateway, config, control, browser }) {
  const outcomes = await Promise.all([a, b].map(page => page.evaluate(async () => {
    try { await fixture.issue(); return 'sent'; } catch { return 'refused'; }
  })));
  assert.deepEqual(outcomes, ['refused', 'refused']);
  const calls = await Promise.all([a, b].map(page => page.evaluate(() => fixture.calls())));
  assert.equal(calls.reduce((x, y) => x + y), 1);
  assert.deepEqual(gateway, []);
  const before = await b.evaluate(() => fixture.inspect());
  assert.equal(before.phase, 'uncertain');
  assert.equal(before.cancelled, false);
  assert.equal(before.gateway, undefined);
  assert.match(before.requestId, /^[0-9a-f]{64}$/);
  assert.ok(before.envelope.length > 0);
  await load(b);
  assert.deepEqual(await b.evaluate(() => fixture.inspect()), before);
  // A certified rejection remains refusal; recovery never produces upload authority.
  assert.equal(await b.evaluate(async () => {
    try { await fixture.recover(); return 'accepted'; } catch (error) { return error.code; }
  }), 'status');
  assert.deepEqual(await b.evaluate(() => fixture.inspect()), before);
  assert.equal(await b.evaluate(async () => {
    try { await fixture.issue(); return 'sent'; } catch { return 'refused'; }
  }), 'refused');
  assert.equal(await b.evaluate(() => fixture.calls()), 0);
  assert.equal(await b.evaluate(async () => {
    try { await fixture.gatewayProbe(); return 'sent'; } catch (error) { return error.code; }
  }), 'gateway-blocked');
  assert.equal(await b.evaluate(() => fixture.gatewayCalls()), 0);
  const cleanup = once(control, 'line');
  process.stdout.write(`${JSON.stringify({ outcome: 'refused', calls: 1, gatewayCalls: 0, intent: before })}\n`);
  const [line] = await cleanup;
  assert.deepEqual(JSON.parse(line), { cleanup: true });
  control.close(); process.stdin.pause();
  const cancelled = await b.evaluate(() => fixture.cancel());
  assert.deepEqual(cancelled, { ...before, cancelled: true });
  const consumer = await b.evaluate(config => fixture.finishConsumer(config, true), config);
  assert.deepEqual(await b.evaluate(() => fixture.inspect()), cancelled);
  assert.deepEqual(gateway, []);
  console.log(JSON.stringify({ browser: browser.version(), outcome: 'passed',
    cancelled: true, calls: 1, gatewayCalls: 0, intent: cancelled, consumer }));
}
