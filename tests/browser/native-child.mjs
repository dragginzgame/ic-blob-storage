// Child-process substitute for pipe framing/lifetime, not IC/provider evidence.
import { createInterface } from 'node:readline';
const mode = process.env.PIPE_CASE;
const send = value => process.stdout.write(JSON.stringify(value) + '\n');
if (mode === 'startup-hang') setInterval(() => {}, 1000);
else if (mode === 'startup-refusal') { send({ error: 'arguments' }); process.exitCode = 2; }
else {
  send({ schema: 1, event: 'ready' });
  if (mode === 'extra-ready') send({ schema: 1, event: 'ready' });
  if (mode === 'oversized') process.stdout.write('x'.repeat(8 * 1024 * 1024 + 1));
  const input = createInterface({ input: process.stdin });
  let step = 0, final = false;
  input.on('line', async text => {
    const frame = JSON.parse(text);
    if (mode === 'hang') return;
    if (mode === 'partial') { process.stdout.write('{'); process.exit(0); }
    if (mode === 'bad-utf8') { process.stdout.write(Buffer.from([255, 10])); return; }
    if (mode === 'delay') await new Promise(resolve => setTimeout(resolve, 80));
    const report = frame.phase === 'map' ? { all_references_live: true } : { file_live: false };
    const event = { schema: 1, event: 'phase', step: step++, next_index: 0, report, next_frame: null };
    // Deliberately split one UTF-8 JSON line across writes; terminal is coalesced.
    const bytes = Buffer.from(JSON.stringify({ ...event, label: 'β' }) + '\n');
    const split = bytes.indexOf(Buffer.from('β')) + 1;
    process.stdout.write(bytes.subarray(0, split));
    process.stdout.write(bytes.subarray(split));
    if (frame.phase === 'map') {
      final = true; send(report); input.close(); process.stdin.destroy();
      if (mode === 'bad-exit') process.exitCode = 7;
      if (mode === 'trailing') send({ extra: true });
      if (mode === 'no-exit') setInterval(() => {}, 1000);
    }
  });
  input.on('close', () => { if (!final) { send({ error: 'transport' }); process.exitCode = 3; } });
}
