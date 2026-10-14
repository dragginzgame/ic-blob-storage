// Fixture transport only; each scenario owns its messages, authority and deadline.
import { createInterface } from 'node:readline';

export function controlChannel(input = process.stdin, output = process.stdout) {
  const reader = createInterface({ input, crlfDelay: Infinity });
  const lines = reader[Symbol.asyncIterator]();
  return {
    async next() {
      const { value, done } = await lines.next();
      if (done) throw new Error('Browser fixture control channel closed');
      return JSON.parse(value);
    },
    send(value) { output.write(`${JSON.stringify(value)}\n`); },
    close() { reader.close(); input.pause(); },
  };
}
