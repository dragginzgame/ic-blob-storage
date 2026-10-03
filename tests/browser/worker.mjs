// Offline worker control/authority checks; deliberately substituted intent backend.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { StorageClient } from '@caffeineai/object-storage';
import { Ed25519KeyIdentity } from '@icp-sdk/core/identity';
import { createPublicationWorker } from '../../clients/browser/worker.js';
import { IntentRefusal } from '../../clients/browser/intents.js';
const directory = process.argv[2]; assert(directory);
await mkdir(directory, { mode: 0o700 });
const record = (name, value) => writeFile(join(directory, name), JSON.stringify(value, null, 2) + '\n', { flag: 'wx', mode: 0o600 });
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
await record('intent.json', { schema: 1, evidence: 'offline_worker_boundary_substituted_store',
  source_sha256: digest(await readFile(new URL('../../clients/browser/worker.js', import.meta.url))),
  provider_requests: 0, certificate_requests: 0, claims: 0 });
const identity = Ed25519KeyIdentity.generate(new Uint8Array(32).fill(42));
const uploader = identity.getPrincipal().toText(), service = 'rrkah-fqaaa-aaaaa-aaaaq-cai';
const body = new Uint8Array(1024).fill(42), prepared = await StorageClient.prepareFile(body, 'image/png');
const binding = { key: `${service}:${uploader}:1`, service, tenant: uploader, uploader,
  operation: '1', root: prepared.hash, project: 'worker-fixture', bucket: 'worker-fixture', permission: [68, 73, 68, 76] };
const request = { schema: 1, id: 1, index: 0, action: 'upload', binding,
  snapshot: { body, bodySha256: digest(body), manifestJSON: prepared.manifestJSON, contentType: 'image/png' } };
let reads = 0, saves = 0, transportCalls = 0, captured;
const secret = 'PRIVATE_SIGNED_ENVELOPE_PROVIDER_HEADER';
const unexpected = () => { throw new Error(secret); };
const intents = { inspect: async value => { reads++; captured = value; return undefined; },
  save: async () => { saves++; throw new Error(secret); },
  claim: unexpected, observe: unexpected, cancel: unexpected,
  claimGateway: unexpected, observeGateway: unexpected };
const options = { host: 'http://127.0.0.1:1', origin: 'https://substitute.invalid',
  identity, rootKey: new Uint8Array([1]), service, tenant: uploader, uploader,
  project: binding.project, bucket: binding.bucket, intents, maxBodyBytes: 1024,
  maxRequests: 2, maxRequestBytes: 65536, maxTotalRequestBytes: 131072, maxJobs: 32,
  timeoutSeconds: 120, certificateFetch: () => { transportCalls++; unexpected(); },
  gatewayFetch: () => { transportCalls++; unexpected(); } };
const cases = [];
async function fails(name, message, expected, settings = {}) {
  reads = 0; saves = 0;
  const result = await createPublicationWorker({ ...options, ...settings }).execute(message);
  assert.equal(result.state, 'failed'); assert.equal(result.error, expected);
  assert.equal(result.retry_authorized, false); assert.equal(result.service_completion_checked, false);
  assert.equal(transportCalls, 0); assert.equal(saves, 0);
  assert(!JSON.stringify(result).includes(secret));
  cases.push({ name, error: result.error, reads, claims: 0, transport_requests: 0 });
}
for (const key of ['service', 'tenant', 'uploader', 'project', 'bucket']) {
  await fails(`foreign_${key}`, { ...request, binding: { ...binding, [key]: 'foreign' } }, 'scope');
  assert.equal(reads, 0);
}
for (const [field, value, error] of [
  ['operation', '01', 'operation'],
  ['operation', (1n << 128n).toString(), 'operation'],
  ['root', 'sha256:' + 'A'.repeat(64), 'root'],
  ['permission', [], 'permission'],
  ['permission', [256], 'permission'],
  ['key', `${service}:${uploader}:2`, 'key'],
]) {
  await fails(`invalid_binding_${field}`, { ...request, binding: { ...binding, [field]: value } }, error);
  assert.equal(reads, 0);
}
await fails('unknown_fields', { ...request, retry: true }, 'message'); assert.equal(reads, 0);
await fails('unknown_action', { ...request, action: 'resume-upload' }, 'message'); assert.equal(reads, 0);
await fails('corrupt_body', { ...request, snapshot: { ...request.snapshot, body: new Uint8Array(1024) } }, 'body-digest');
await fails('changed_manifest', { ...request, snapshot: { ...request.snapshot, manifestJSON: '{}' } }, 'manifest');
for (const row of [{ phase: 'observed' }, { phase: 'saved', cancelled: true }, { phase: 'saved', gateway: {} }])
  await fails('claimed_or_cancelled', request, 'upload-claimed', { intents: { ...intents, inspect: async () => row } });
const lookup = { schema: 1, id: 1, action: 'inspect', index: 0, binding };
await fails('missing_recovery_history', { ...lookup, action: 'recover-certificate' }, 'history-missing');
await fails('missing_cancellation_history', { ...lookup, action: 'cancel' }, 'history-missing');
// A blocked concurrent request must not clear another job's active guard.
let release;
const held = new Promise(resolve => { release = resolve; });
const worker = createPublicationWorker({ ...options, intents: { ...intents, inspect: async () => held } });
const first = worker.execute(lookup);
for (const id of [2, 3]) assert.equal((await worker.execute({ ...lookup, id })).error, 'worker-busy');
release(undefined); assert.equal((await first).journal.present, false);
assert.equal((await worker.execute({ ...lookup, id: 4 })).state, 'inspected');
assert.equal((await worker.execute({ ...lookup, id: 4 })).error, 'message');
cases.push({ name: 'concurrent_guard_and_correlation', claims: 0, transport_requests: 0 });
const bytes = body.slice(), rootKey = options.rootKey.slice(), mutable = structuredClone(request);
mutable.snapshot.body = bytes;
const owned = createPublicationWorker({ ...options, rootKey });
const pending = owned.execute(mutable);
bytes.fill(0); mutable.binding.root = 'sha256:' + '0'.repeat(64); rootKey.fill(9);
const result = await pending;
assert.equal(result.error, 'transport'); assert.equal(saves, 1);
assert.equal(captured.root, binding.root); assert.deepEqual(captured.icRootKey, [1]);
assert(!JSON.stringify(result).includes(secret));
cases.push({ name: 'snapshots_and_redacted_failure', claims: 0, transport_requests: 0 });
const limited = createPublicationWorker({ ...options, maxJobs: 1 });
assert.equal((await limited.execute(lookup)).state, 'inspected');
assert.equal((await limited.execute({ ...lookup, id: 2 })).error, 'job-budget');
const aborted = new AbortController(); aborted.abort();
await fails('aborted_worker', lookup, 'aborted', { signal: aborted.signal }); assert.equal(reads, 0);
const idle = createPublicationWorker({ ...options, timeoutSeconds: 1 });
await new Promise(resolve => setTimeout(resolve, 1100));
reads = 0;
assert.equal((await idle.execute(lookup)).error, 'aborted'); assert.equal(reads, 0);
cases.push({ name: 'idle_deadline_fences_new_jobs', claims: 0, transport_requests: 0 });
for (const changes of [{ uploader: service }, { origin: 'http://127.0.0.1:1' },
  { origin: 'https://user:secret@substitute.invalid' }, { maxJobs: 32770 }]) {
  assert.throws(() => createPublicationWorker({ ...options, ...changes }), error =>
    ['identity', 'origin', 'limits'].includes(error.code));
}
cases.push({ name: 'configuration_roles_origins_and_limits', claims: 0, transport_requests: 0 });
const privateRow = { phase: 'observed', cancelled: false, envelope: [1, 2, 3], requestId: secret,
  gateway: { requests: [{ phase: 'uncertain', request: { headers: [['authorization', secret]] } }] } };
const redacted = await createPublicationWorker({ ...options,
  intents: { ...intents, inspect: async () => privateRow } }).execute(lookup);
assert.deepEqual(redacted.journal, { present: true, certificate_phase: 'observed', cancelled: false,
  gateway_requests: [{ index: 0, phase: 'uncertain' }] });
assert(!JSON.stringify(redacted).includes(secret));
cases.push({ name: 'bounded_journal_projection', claims: 0, transport_requests: 0 });
await fails('exported_error_with_private_code', lookup, 'transport', {
  intents: { ...intents, inspect: async () => { throw new IntentRefusal(secret); } },
});
for (const changed of [{ ...privateRow, phase: secret }, { ...privateRow, cancelled: secret },
  { ...privateRow, gateway: { requests: [{ phase: 'responded', status: secret }] } },
  { ...privateRow, gateway: { requests: new Array(257).fill({ phase: 'uncertain' }) } }]) {
  await fails('corrupt_projection_refused', lookup, 'intent-corrupt', {
    intents: { ...intents, inspect: async () => changed },
  });
}
await record('summary.json', { schema: 1, complete: true, cases, certificate_requests: 0,
  provider_requests: 0, claims: 0, caveat: 'Substituted store for boundary checks; actual Worker/IC/IndexedDB is qualified separately.' });
console.log('PASS worker authority, snapshots, serialization, bounds and redaction');
