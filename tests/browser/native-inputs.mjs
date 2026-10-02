// Offline real SDK/native/browser binding handoff; no service or gateway request.
import assert from 'node:assert/strict';
import { mkdirSync, readFileSync, writeFileSync, readdirSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { StorageClient } from '@caffeineai/object-storage';
import { Ed25519KeyIdentity } from '@icp-sdk/core/identity';
import { Principal } from '@icp-sdk/core/principal';
import { createCertificateClient } from '../../clients/browser/certificate.js';
import { configuration } from './installation.js';

assert(Number(process.versions.node.split('.')[0]) >= 24, 'Use the provisioned Node 24 tool');
const [directory, binary, byteArgument = '10485760', ...extra] = process.argv.slice(2);
assert(directory && binary && extra.length === 0,
  'native-inputs NEW_DIRECTORY BLOB_STORAGE_BINARY [CONTENT_BYTES]');
const contentBytes = Number(byteArgument);
assert(/^[1-9]\d*$/.test(byteArgument) && Number.isSafeInteger(contentBytes)
  && contentBytes <= 10 * 1024 * 1024, 'CONTENT_BYTES must be between 1 and 10485760');
const output = resolve(directory), cli = resolve(binary);
mkdirSync(output, { mode: 0o700 });
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const record = (name, value) => writeFileSync(join(output, name),
  `${JSON.stringify(value, null, 2)}\n`, { flag: 'wx', mode: 0o600 });
const bytes = new Uint8Array(contentBytes).fill(42);
const body = join(output, 'source.bin');
let networkCalls = 0;
const refuseNetwork = () => { networkCalls++; throw new Error('Unexpected network request'); };
globalThis.fetch = refuseNetwork;
record('plan.json', { schema: 1, evidence: 'offline_sdk_native_browser_handoff',
  started: new Date().toISOString(), runnerSha256: digest(readFileSync(process.argv[1])),
  binarySha256: digest(readFileSync(cli)), node: process.versions.node,
  sdk: 'Caffeine 1.1.2 with maintained repository patch',
  contentBytes: bytes.length, maxNativeInvocations: 4, maxSdkPreparations: 2,
  store: 'in-memory setup-only substitute', maxNetworkRequests: 0,
  cleanup: 'No network, key file, account, provider object or external resource' });
writeFileSync(body, bytes, { flag: 'wx', mode: 0o600 });
const identity = Ed25519KeyIdentity.generate(new Uint8Array(32).fill(42));
const prepared = await StorageClient.prepareFile(bytes, 'application/octet-stream', 'fixture.bin');
assert.equal(prepared.byteLength, bytes.length);
const binding = { schema: 1, project: 'fixture-project', bucket: 'fixture-bucket', service: 'rrkah-fqaaa-aaaaa-aaaaq-cai',
  namespace: ((1n << 128n) - 1n).toString(), tenant: Principal.selfAuthenticating(new Uint8Array([3])).toText(),
  uploader: identity.getPrincipal().toText(), upload: ((1n << 128n) - 2n).toString(),
  object: ((1n << 128n) - 3n).toString(), incarnation: '1', first_reference: '2',
  root: prepared.hash, bytes: bytes.length.toString(), expires_at_ns: ((1n << 64n) - 1n).toString() };
record('binding.json', binding);
writeFileSync(join(output, 'manifest.json'), prepared.manifestJSON, { flag: 'wx', mode: 0o600 });
const input = join(output, 'inputs');
const configurationPath = join(output, 'configuration.candid');
writeFileSync(configurationPath, configuration(binding.service, binding.namespace), { flag: 'wx', mode: 0o600 });
const installationDirectory = join(output, 'installation');
const args = ['upload-inputs', '--installation', join(installationDirectory, 'installation.candid'),
  '--binding', join(output, 'binding.json'), '--manifest',
  join(output, 'manifest.json'), '--body', body, '--max-bytes', String(bytes.length), '--run-dir', input];
function invoke(name, command, expected) {
  record(`${name}-command.json`, [cli, ...command]);
  const result = spawnSync(cli, command, { encoding: 'utf8', timeout: 30_000, maxBuffer: 64 * 1024 });
  record(`${name}-result.json`, { exit: result.status, signal: result.signal,
    error: result.error?.code ?? null, stdout: result.stdout, stderr: result.stderr });
  assert.equal(result.status, expected);
  return JSON.parse(result.stdout);
}
const installation = invoke('installation', ['installation-check', '--configuration', configurationPath,
  '--service', binding.service, '--project', binding.project, '--trusted-uploader', binding.uploader,
  '--verifier', 'rdmx6-jaaaa-aaaaa-aaadq-cai', '--release', 'offline-fixture', '--run-dir', installationDirectory], 0);
const result = invoke('prepared', args, 0);
assert.equal(result.body_verified, true);
assert.equal(result.installation_binding_checked, true);
assert.equal(result.installed_state_observed, false);
assert.equal(result.installation_sha256, digest(readFileSync(join(installationDirectory, 'installation.candid'))));
assert.deepEqual(readFileSync(join(input, 'installation.candid')), readFileSync(join(installationDirectory, 'installation.candid')));
assert.equal(result.service_dispatched, false);
const generated = JSON.parse(readFileSync(join(input, 'certificate-binding.json'), 'utf8'));
assert.equal(result.certificate_binding_sha256, digest(readFileSync(join(input, 'certificate-binding.json'))));
assert.deepEqual(generated.permission, Array.from(readFileSync(join(input, 'permission.candid'))));
assert.equal(generated.operation, binding.upload);
assert.equal(generated.service, binding.service);
assert.equal(generated.tenant, binding.tenant);
assert.equal(generated.uploader, binding.uploader);
assert.equal(generated.root, prepared.hash);
assert.equal(generated.key, `${binding.service}:${binding.tenant}:${binding.upload}`);

// This substitute permits only setup/inspection; it cannot issue or recover.
let saved;
const refuseEffect = async () => { throw new Error('Unexpected effect'); };
const certificate = await createCertificateClient({ host: 'http://127.0.0.1:1', identity,
  rootKey: new Uint8Array([1]), binding: generated, fetch: refuseNetwork,
  intents: { save: async b => (saved = { key: b.key, binding: structuredClone(b), phase: 'saved', cancelled: false }),
    inspect: async () => structuredClone(saved), claim: refuseEffect, observe: refuseEffect, cancel: refuseEffect } });
const row = await certificate.inspect();
assert.equal(row.phase, 'saved');
assert.equal(row.binding.operation, binding.upload);
assert.deepEqual(row.binding.permission, generated.permission);
assert.equal(row.binding.project, binding.project);
assert.equal(row.binding.bucket, binding.bucket);
// The root is deliberately a substitute: no IC signature or request is tested here.
assert.deepEqual(row.binding.icRootKey, [1]);
assert.equal(networkCalls, 0);
const hashes = () => Object.fromEntries(readdirSync(input).sort().map(name =>
  [name, digest(readFileSync(join(input, name)))]));
const original = hashes();
assert.equal(invoke('repeat', args, 3).error, 'new_run_required');
assert.deepEqual(hashes(), original);
bytes[0] ^= 1;
writeFileSync(body, bytes); // Deliberate later source change; the verified snapshot is separate.
const snapshot = readFileSync(join(input, 'body.bin'));
assert.equal(digest(snapshot), original['body.bin']);
const rebuilt = await StorageClient.prepareFile(new Uint8Array(snapshot), 'application/octet-stream', 'fixture.bin');
assert.equal(rebuilt.hash, prepared.hash);
assert.equal(rebuilt.byteLength, prepared.byteLength);
assert.equal(rebuilt.manifestJSON, prepared.manifestJSON);
const corrupt = args.slice();
corrupt[corrupt.indexOf('--run-dir') + 1] = join(output, 'corrupt');
assert.equal(invoke('corrupt', corrupt, 3).error, 'content_mismatch');
for (const file of ['body.bin', 'permission.candid', 'manifest.candid', 'certificate-binding.json', 'summary.json']) {
  assert(!readdirSync(join(output, 'corrupt')).includes(file));
}
assert.deepEqual(hashes(), original);
assert.equal(networkCalls, 0);
record('summary.json', { schema: 1, sdkPreparations: 2, nativeInvocations: 4,
  installationBindingChecked: true, installedStateObserved: false, installation,
  contentBytes: bytes.length, chunks: JSON.parse(prepared.manifestJSON).chunk_hashes.length,
  certificateBindingAccepted: true, originalSourceChanged: true, snapshotRepreparationMatches: true,
  inputHashes: original, networkRequests: networkCalls, certificateIssued: false,
  storeQualified: false, providerQualified: false });
console.log('PASS real SDK preparation, native snapshot/browser binding, repeat and corrupt-source refusal');
