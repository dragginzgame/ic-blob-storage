// Real child pipes; deliberately no browser, IC, keys or provider traffic.
import assert from 'node:assert/strict';
import { test } from 'node:test';
import { readFile, mkdir, writeFile } from 'node:fs/promises';
import { resolve, join } from 'node:path';
import { startPublicationSession, NativeSessionRefusal } from '../../clients/browser/native.mjs';
const report = resolve(process.argv[2]);
await mkdir(report, { mode: 0o700 });
const binary = join(report, 'child.mjs');
await writeFile(binary, `#!${process.execPath}\n${await readFile(new URL('./native-child.mjs', import.meta.url))}`,
  { flag: 'wx', mode: 0o700 });
const options = mode => ({ binary, cwd: report, args: ['publish-session', 'PRIVATE_ARGUMENT'],
  timeoutSeconds: 3, env: { PIPE_CASE: mode } });
const refusal = code => error => error instanceof NativeSessionRefusal && error.code === code &&
  !String(error).includes('PRIVATE_ARGUMENT');

test('explicit command snapshot, split UTF-8 phase, coalesced final and exit', async () => {
  const selected = options('complete'), starting = startPublicationSession(selected);
  selected.args[0] = 'changed'; selected.env.PIPE_CASE = 'hang';
  const session = await starting;
  try {
    assert.equal(session.ready.event, 'ready');
    const event = await session.phase({ phase: 'map' }); assert.equal(event.label, 'β');
    assert.deepEqual(await session.finish(), { report: event.report, exit_code: 0 });
    await assert.rejects(session.phase({ phase: 'map' }), refusal('closed'));
  } finally { await session.close(); }
});
test('one phase, bounded outgoing frame and graceful unfinished result', async () => {
  const session = await startPublicationSession(options('delay'));
  try {
    await assert.rejects(session.phase({ phase: 'status', excess: 'x'.repeat(8192) }), refusal('frame-limit'));
    const waiting = session.phase({ phase: 'status', index: 0 });
    await assert.rejects(session.phase({ phase: 'map' }), refusal('busy'));
    assert.equal((await waiting).step, 0);
    assert.deepEqual(await session.finish(), { report: { error: 'transport' }, exit_code: 3 });
  } finally { await session.close(); }
});
test('unserializable arguments and frames refuse without exposing data or sending a phase', async () => {
  const PRIVATE_ARGUMENT = function PRIVATE_ARGUMENT() {};
  for (const options of [undefined, null]) {
    await assert.rejects(startPublicationSession(options), refusal('configuration'));
  }
  await assert.rejects(startPublicationSession({ ...options('complete'),
    args: ['publish-session', PRIVATE_ARGUMENT] }), refusal('configuration'));
  const session = await startPublicationSession(options('complete'));
  try {
    const circular = { phase: 'map' }; circular.PRIVATE_ARGUMENT = circular;
    for (const frame of [{ phase: 'map', PRIVATE_ARGUMENT },
      { phase: 'map', PRIVATE_ARGUMENT: 1n }, circular]) {
      await assert.rejects(session.phase(frame), refusal('configuration'));
    }
    const event = await session.phase({ phase: 'map' });
    assert.equal(event.step, 0);
    assert.deepEqual(await session.finish(), { report: event.report, exit_code: 0 });
  } finally { await session.close(); }
});
test('abort cancels an idle outstanding phase and owns child shutdown', async () => {
  const session = await startPublicationSession(options('hang')), controller = new AbortController();
  const waiting = session.phase({ phase: 'status', index: 0 }, controller.signal);
  controller.abort(); await assert.rejects(waiting, refusal('closed')); await session.close(); await session.close();
});
test('startup abort and deadline stop before exposing a control peer', async () => {
  const controller = new AbortController();
  const waiting = startPublicationSession({ ...options('startup-hang'), signal: controller.signal });
  controller.abort(); await assert.rejects(waiting, refusal('closed'));
  await assert.rejects(startPublicationSession({ ...options('startup-hang'), timeoutSeconds: 1 }), refusal('deadline'));
  await assert.rejects(startPublicationSession(options('startup-refusal')), refusal('native-failure'));
  await assert.rejects(startPublicationSession({ ...options('complete'), binary: join(report, 'missing') }), refusal('spawn'));
});
test('complete report still requires exit, and argument configuration stays explicit', async () => {
  for (const change of [{ binary: 1 }, { binary: 'blob-storage' }, { args: ['upload'] }, { env: [] }]) {
    await assert.rejects(startPublicationSession({ ...options('complete'), ...change }), refusal('configuration'));
  }
  const session = await startPublicationSession({ ...options('no-exit'), timeoutSeconds: 1 });
  try {
    assert.equal((await session.phase({ phase: 'map' })).report.all_references_live, true);
    await assert.rejects(session.finish(), refusal('deadline'));
  } finally { await session.close(); }
});
test('finish cancellation stops a child after final output but before exit', async () => {
  const session = await startPublicationSession(options('no-exit')), controller = new AbortController();
  try {
    await session.phase({ phase: 'map' });
    const waiting = session.finish(controller.signal); controller.abort();
    await assert.rejects(waiting, refusal('closed'));
  } finally { await session.close(); }
});
for (const [mode, code] of [['partial', 'exit'], ['bad-utf8', 'reply'], ['bad-exit', 'exit'], ['trailing', 'reply']]) {
  test(`refuse ${mode} without trusting successful phase output`, async () => {
    const session = await startPublicationSession(options(mode));
    try {
      const phase = session.phase({ phase: 'map' });
      if (['partial', 'bad-utf8'].includes(mode)) await assert.rejects(phase, refusal(code));
      else { await phase; await assert.rejects(session.finish(), refusal(code)); }
    } finally { await session.close(); }
  });
}
test('unsolicited events and oversized input stop the peer', async () => {
  for (const [mode, code] of [['extra-ready', 'reply'], ['oversized', 'reply-limit']]) {
    let session;
    try {
      session = await startPublicationSession(options(mode));
      await assert.rejects(session.finish(), refusal(code));
    } finally { await session?.close(); }
  }
});
