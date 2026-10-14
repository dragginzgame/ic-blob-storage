import test from 'node:test';
import assert from 'node:assert/strict';
import { PassThrough } from 'node:stream';
import { controlChannel } from './control.mjs';

test('fragmented and queued control messages survive work between reads', { timeout: 1000 }, async () => {
  const input = new PassThrough(), output = new PassThrough();
  const channel = controlChannel(input, output);
  try {
    const first = channel.next();
    input.write('{"index":');
    input.end('0}\r\n{"finish":true}\n');
    assert.deepEqual(await first, { index: 0 });
    await new Promise(resolve => setImmediate(resolve));
    assert.deepEqual(await channel.next(), { finish: true });
    await assert.rejects(channel.next(), /control channel closed/);
    channel.send({ outcome: 'passed' });
    assert.equal(output.read().toString(), '{"outcome":"passed"}\n');
  } finally { channel.close(); }
});

test('malformed JSON rejects without consuming the following message', { timeout: 1000 }, async () => {
  const input = new PassThrough(), channel = controlChannel(input, new PassThrough());
  try {
    input.end('not-json\n{"finish":true}\n');
    await assert.rejects(channel.next(), SyntaxError);
    assert.deepEqual(await channel.next(), { finish: true });
  } finally { channel.close(); }
});

test('closing a channel releases a pending reader and pauses input', { timeout: 1000 }, async () => {
  const input = new PassThrough(), channel = controlChannel(input, new PassThrough());
  const pending = channel.next();
  channel.close();
  await assert.rejects(pending, /control channel closed/);
  assert(input.isPaused());
});
