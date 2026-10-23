// Opt-in cross-language qualification; uses the prepared browser esbuild owner.
import '../../scripts/ci/check-browser-tools.mjs';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { build } from 'esbuild';
import { pathToFileURL } from 'node:url';
import { resolve } from 'node:path';

const directory = resolve('.tmp/wire-schema');
await build({ entryPoints: [`${directory}/client.ts`, `${directory}/blob-runtime.ts`],
  bundle: true, platform: 'node', format: 'esm', outdir: `${directory}/js`, outExtension: { '.js': '.mjs' } });
const client = await import(pathToFileURL(`${directory}/js/client.mjs`));
const { createCodec } = await import(pathToFileURL(`${directory}/js/blob-runtime.mjs`));
const methods = JSON.parse(await readFile(`${directory}/metadata.json`, 'utf8'));
let frames = 0;
for (const method of methods) {
  const codec = client[method.name];
  assert.equal(codec.methodName, method.name);
  assert.equal(codec.mode, method.mode);
  for (const direction of ['request', 'response']) {
    const encode = direction === 'request' ? codec.encodeRequest : codec.encodeResponse;
    const decode = direction === 'request' ? codec.decodeRequest : codec.decodeResponse;
    const cases = JSON.parse(await readFile(`crates/ic-blob-storage-contracts/tests/wire_golden/vectors/${method.name}-${direction}.json`, 'utf8'));
    for (const value of cases) {
      const bytes = Uint8Array.from(Buffer.from(value.hex, 'hex'));
      assert.deepEqual(encode(decode(bytes)), bytes, `${method.name} ${direction}`);
      assert.throws(() => decode(Uint8Array.from([...bytes, 0])), /wire/);
      assert.throws(() => decode(bytes.subarray(0, bytes.length - 1)), /wire/);
      const wrong = bytes.slice(); wrong[2] ^= 0x80;
      assert.throws(() => decode(wrong), /wire identity/);
      const flipped = bytes.slice(); flipped[10] ^= 1;
      assert.throws(() => decode(flipped), /wire identity/);
      const oversized = new Uint8Array(method[`${direction}Bytes`] + 1);
      oversized.set(bytes);
      assert.throws(() => decode(oversized), /wire limit/);
      frames++;
    }
  }
}
// Custom field adapters must retain their canonical language, including grouping.
const account = methods.find(m => m.name === 'blob_inspect_account');
const codec = createCodec(account);
const report = { ok: { request: {
  scope: { service: new Uint8Array([4]), namespace: 1n, cashier: new Uint8Array([4]), payment_account: new Uint8Array([4]) },
  kind: { tag: 'PaymentRelationship', value: null },
}, observation: { tag: 'Relationship', value: {
  paid_canister: new Uint8Array([4]), payment_account: new Uint8Array([]),
  spending_limit_per_day: '-1_000', current_period_spent: '1_234_567',
  current_period_start: 1n, added_timestamp: 2n, expiration_timestamp: { some: 3n },
  bandwidth_baseline_uploaded: '123_456_789_012_345_678_901_234_567_890',
  bandwidth_baseline_downloaded: '0', bandwidth_baseline_ts_ns: 4n,
} } } };
assert.deepEqual(structuredClone(codec.decodeResponse(codec.encodeResponse(report))), report);
const relationshipCases = JSON.parse(await readFile('crates/ic-blob-storage-contracts/tests/wire_golden/vectors/blob_inspect_account-response.json', 'utf8'));
assert.equal(Buffer.from(codec.encodeResponse(report)).toString('hex'), relationshipCases.at(-1).hex);
const accountBytes = codec.encodeResponse(report);
const offset = Buffer.from(accountBytes).indexOf('-1_000');
assert(offset >= 0);
for (const bad of ['+1_000', '-10000', '-0_000']) {
  const invalid = accountBytes.slice();
  invalid.set(Buffer.from(bad), offset);
  assert.throws(() => codec.decodeResponse(invalid), /wire value/);
}
for (const bad of ['-0', '+1', '1000', '01', '1__000', '-1_00', '', '0\n', '1\r\n']) {
  const invalid = structuredClone(report);
  invalid.ok.observation.value.spending_limit_per_day = bad;
  assert.throws(() => codec.encodeResponse(invalid), /wire value/);
}
const invalidNat = structuredClone(report);
invalidNat.ok.observation.value.bandwidth_baseline_uploaded = '-1';
assert.throws(() => codec.encodeResponse(invalidNat), /wire value/);
console.log(`Blob generated TypeScript: ${methods.length} methods, ${frames} frozen Rust frames; malformed/identity/direction/limit and decimal checks passed`);
