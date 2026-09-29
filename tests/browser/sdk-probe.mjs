// Local SDK fault probe: supplied certificate response and gateway are substitutes.
// No IC behavior, deployed-provider completion, charging or persistence is simulated as fact.
import assert from 'node:assert/strict';
import { mkdirSync, openSync, writeFileSync, fsyncSync, closeSync, readFileSync } from 'node:fs';
import { resolve, join } from 'node:path';
import { createHash } from 'node:crypto';
import { execFile } from 'node:child_process';
import { Cbor } from '@icp-sdk/core/agent';
import { StorageClient } from '@caffeineai/object-storage';
import { createGatewayTransport, GatewayRefusal } from '../../clients/browser/gateway.js';
import { fixtureIntents } from './probe-intents.js';

assert(Number(process.versions.node.split('.')[0]) >= 24, 'Use the provisioned Node 24 tool');
const [directory, verifier] = process.argv.slice(2);
assert(directory && verifier, 'sdk-probe NEW_DIRECTORY VERIFY_DOWNLOAD_BINARY');
const output = resolve(directory);
mkdirSync(output); // No replacement or automatic resume of an earlier run.
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
function record(name, value) {
  const file = openSync(join(output, name), 'wx');
  try { writeFileSync(file, `${JSON.stringify(value, null, 2)}\n`); fsyncSync(file); }
  finally { closeSync(file); }
  const dir = openSync(output, 'r'); try { fsyncSync(dir); } finally { closeSync(dir); }
}
const hashFile = path => digest(readFileSync(path));
record('plan.json', {
  schema: 1, kind: 'local_sdk_substitutes', started: new Date().toISOString(),
  runner: hashFile(process.argv[1]), verifier: hashFile(verifier), node: process.versions.node,
  package: '1.1.2 with maintained repository patch',
  limits: { cases: 8, requestsPerCase: 3, requestBytes: 2 * 1024 * 1024,
    totalRequestBytes: 3 * 1024 * 1024, responseBytes: 65536 },
  origin: 'http://127.0.0.1:1', identity: 'substituted certificate-agent response',
  providerCalls: 0, attachedCycles: '0', cleanup: 'No external objects or account effects',
  substitutes: ['in-memory intent store', 'certificate response', 'all gateway replies'],
});
globalThis.fetch = () => { throw new Error('Unexpected network request'); };
const origin = 'http://127.0.0.1:1';
const cases = [];

async function probe(name, size, mode) {
  const bytes = new Uint8Array(size).fill(17);
  const prepared = await StorageClient.prepareFile(bytes, 'application/octet-stream');
  const manifest = JSON.parse(prepared.manifestJSON);
  const binding = { key: name, root: prepared.hash };
  const intents = fixtureIntents(binding);
  const certificate = { inspect: intents.inspect };
  const stored = [];
  let requests = 0, certificateCalls = 0, uploaded = 0;
  const progress = [];
  const maxTotalRequestBytes = mode === 'budget' ? 512 * 1024 : 3 * 1024 * 1024;
  record(`${name}-input.json`, { size, byte: 17, sha256: digest(bytes), root: prepared.hash,
    manifest, maxTotalRequestBytes, mode, expected: 'local observation only' });
  const fetch = await createGatewayTransport({ certificate, intents, origin,
    maxRequests: 3, maxRequestBytes: 2 * 1024 * 1024, maxTotalRequestBytes,
    fetch: async (url, init) => {
      const row = await intents.inspect();
      const request = row.gateway.requests.at(-1);
      assert.equal(request.phase, 'uncertain');
      assert.equal(request.request.bodySha256, digest(init.body));
      assert.equal(init.redirect, 'error');
      const index = requests++;
      uploaded += init.body.byteLength;
      record(`${name}-request-${index}.json`, { index, url, bytes: init.body.byteLength,
        sha256: digest(init.body), intentBeforeDispatch: true, effect: 'local substitute' });
      const target = new URL(url);
      const tree = target.pathname === '/v1/blob-tree/';
      if (!tree) stored.push(init.body.slice());
      const last = !tree && Number(target.searchParams.get('chunk_index')) === manifest.chunk_hashes.length - 1;
      const status = mode === 'http-failure' && !tree ? 503 : 200;
      const payload = tree ? { status: 'blob_tree_accepted', existing_chunks: manifest.chunk_hashes, chunk_check_errors: 0 }
        : { status: mode === 'non-complete' ? 'fixture_not_complete' : 'blob_complete' };
      const body = JSON.stringify(payload);
      const dropped = mode === 'lost-final' && last;
      record(`${name}-response-${index}.json`, { index, status, body, sha256: digest(body),
        delivered: !dropped, substituteStoredBytes: stored.reduce((n, c) => n + c.length, 0) });
      if (dropped) throw new TypeError('Deliberately discarded local response');
      return new Response(body, { status, headers: { 'content-type': 'application/json' } });
    },
  });
  const agent = { call: async () => {
    certificateCalls++;
    // Valid CBOR container, deliberately no signature/tree. Not a verified IC certificate.
    return { requestId: new Uint8Array(32), response: { body: { certificate: Cbor.encode({}) } } };
  } };
  const storage = new StorageClient('fixture-bucket', origin, 'rrkah-fqaaa-aaaaa-aaaaq-cai',
    'fixture-project', agent, { retry: false, concurrency: 1, fetch });
  let returned = false, refusal = null;
  try { assert.equal((await storage.uploadPrepared(prepared, n => progress.push(n))).hash, prepared.hash); returned = true; }
  catch (error) { refusal = error instanceof GatewayRefusal ? error.code : error.name; }
  const expectedSuccess = ['success', 'non-complete'].includes(mode);
  assert.equal(returned, expectedSuccess);
  assert.equal(certificateCalls, 1);
  const row = await intents.inspect();
  const expectedRequests = ['budget'].includes(mode) ? 1 : mode === 'http-failure' ? 2 : 1 + manifest.chunk_hashes.length;
  assert.equal(requests, expectedRequests);
  assert(uploaded <= maxTotalRequestBytes);
  if (mode === 'budget') assert.equal(refusal, 'request-budget');
  if (mode === 'lost-final') assert.equal(row.gateway.requests.at(-1).phase, 'uncertain');
  if (mode === 'http-failure') assert.equal(row.gateway.requests.at(-1).status, 503);
  // A new preparation cannot make a consumed certificate/transfer safe to repeat.
  await assert.rejects(storage.uploadPrepared(prepared), TypeError);
  const recreated = await createGatewayTransport({ certificate, intents, origin,
    maxRequests: 3, maxRequestBytes: 2 * 1024 * 1024, maxTotalRequestBytes,
    fetch: async () => { throw new Error('Recreated transfer must not dispatch'); } });
  await assert.rejects(recreated(`${origin}/v1/probe/`, { method: 'PUT', body: 'x' }),
    error => error instanceof GatewayRefusal && error.code === 'gateway-session');
  assert.deepEqual(await intents.inspect(), row);
  let verified = null;
  if (['success', 'non-complete', 'lost-final'].includes(mode)) {
    const reconstructed = Buffer.concat(stored.map(value => Buffer.from(value)));
    assert.deepEqual(reconstructed, Buffer.from(bytes));
    const claim = { root: prepared.hash, bytes: size, headers: manifest.headers.map(line => {
      const colon = line.indexOf(':'); return { name: line.slice(0, colon), value: line.slice(colon + 1).trim() };
    }) };
    record(`${name}-claim.json`, claim);
    const checks = [];
    const check = body => new Promise(resolve => {
      const child = execFile(verifier, [join(output, `${name}-claim.json`), String(size)],
        { maxBuffer: 65536, timeout: 10000 }, (error, stdout, stderr) => {
          const result = { status: error ? error.code : 0, signal: error?.signal ?? null,
            killed: error?.killed ?? false, stdout, stderr };
          checks.push(result); resolve(result);
        });
      child.stdin.on('error', () => {}); // Early verifier rejection may close its input.
      child.stdin.end(body);
    });
    const valid = await check(reconstructed);
    assert.equal(valid.status, 0, String(valid.stderr));
    const corrupt = Buffer.from(reconstructed); corrupt[0] ^= 1;
    for (const rejected of [corrupt, reconstructed.subarray(0, reconstructed.length - 1)]) {
      const result = await check(rejected);
      assert.equal(result.killed, false);
      assert.equal(result.signal, null);
      assert(Number.isInteger(result.status) && result.status > 0);
    }
    record(`${name}-verification.json`, checks);
    verified = { original: true, corrupted: false, truncated: false };
  }
  const result = { name, mode, size, requests, uploaded, returned, refusal, progress,
    finalRequestPhase: row.gateway.requests.at(-1).phase, verified,
    retryDispatched: false, providerCompletion: 'not_established', providerCharges: 'not_observed' };
  record(`${name}-result.json`, result); cases.push(result);
}

try {
  await probe('single', 3, 'success');
  await probe('multi', 1024 * 1024 + 1, 'success');
  await probe('not-complete', 3, 'non-complete');
  await probe('lost-final', 1024 * 1024 + 1, 'lost-final');
  await probe('http-failure', 1024 * 1024 + 1, 'http-failure');
  await probe('byte-budget', 1024 * 1024 + 1, 'budget');
  record('summary.json', { outcome: 'passed', cases,
    conclusions: ['SDK uploads chunks despite supplied existing_chunks',
      'SDK return/progress does not require a complete status',
      'Lost final reply preserves uncertainty even when substitute bytes verify',
      'Traffic budget refuses next request before dispatch'],
    limitations: 'Substituted certificate/gateway/store; no IC lifecycle, provider retention, charges or production recovery proved' });
  console.log(JSON.stringify({ outcome: 'passed', directory: output, cases: cases.length }));
} catch (error) {
  record('failure.json', { outcome: 'failed', name: error.name, message: error.message, completedCases: cases.map(c => c.name) });
  throw error;
}
