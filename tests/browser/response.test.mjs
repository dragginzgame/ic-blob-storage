import assert from 'node:assert/strict';
import test from 'node:test';
import { readResponseBody } from '../../clients/browser/response.js';

test('response collection preserves ordered chunks at the exact byte ceiling', async () => {
  const body = new ReadableStream({ start(controller) {
    controller.enqueue(new Uint8Array([0, 128]));
    controller.enqueue(new Uint8Array());
    controller.enqueue(new Uint8Array([255]));
    controller.close();
  } });
  assert.deepEqual(await readResponseBody(body, 3, new Error('size')), new Uint8Array([0, 128, 255]));
});

test('an oversized response cancels before accepting more bytes and retains the caller refusal', async () => {
  let cancelled = false;
  const refusal = new Error('owner refusal');
  const body = new ReadableStream({ start(controller) {
    controller.enqueue(new Uint8Array(4));
  }, async cancel() { await Promise.resolve(); cancelled = true; } });
  await assert.rejects(readResponseBody(body, 3, refusal), error => error === refusal);
  assert.equal(cancelled, true);
});

test('response stream and cancellation failures propagate without replacement', async () => {
  const failure = new Error('stream failure');
  const broken = new ReadableStream({ start(controller) { controller.error(failure); } });
  await assert.rejects(readResponseBody(broken, 3, new Error('size')), error => error === failure);
  const overflow = new ReadableStream({ start(controller) {
    controller.enqueue(new Uint8Array(4));
  }, cancel() { throw failure; } });
  await assert.rejects(readResponseBody(overflow, 3, new Error('size')), error => error === failure);
});
