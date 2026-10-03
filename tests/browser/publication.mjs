// Offline frozen-input checks. No certificate/store/provider effect is exercised.
import assert from 'node:assert/strict';
import { mkdir, writeFile, readFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { join } from 'node:path';
import { StorageClient } from '@caffeineai/object-storage';
import { Ed25519KeyIdentity } from '@icp-sdk/core/identity';
import { createPublicationUpload } from '../../clients/browser/publication.js';
const directory = process.argv[2];
assert(directory);
await mkdir(directory, { mode: 0o700 });
const record = (name, value) => writeFile(join(directory, name), JSON.stringify(value,null,2)+'\n', { flag:'wx', mode:0o600 });
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
await record('intent.json', { schema:1, evidence:'offline_sdk_frozen_input_refusals',
  maxBodyBytes:1024, providerRequests:0, certificateRequests:0, intentClaims:0,
  sourceSha256:digest(await readFile(new URL('../../clients/browser/publication.js',import.meta.url))) });
const body = new Uint8Array(1024).fill(42);
const prepared = await StorageClient.prepareFile(body,'image/png');
const identity = Ed25519KeyIdentity.generate(new Uint8Array(32).fill(42));
const service = 'rrkah-fqaaa-aaaaa-aaaaq-cai', uploader = identity.getPrincipal().toText();
let storeCalls = 0, transportCalls = 0, savedBinding;
const intentTrap = new Error('unexpected intent access');
const fail = value => { storeCalls++; savedBinding = structuredClone(value); throw intentTrap; };
const intents = Object.fromEntries(['save','inspect','claim','observe','cancel'].map(name=>[name,fail]));
const transport = () => { transportCalls++; throw new Error('unexpected request'); };
const base = { host:'http://127.0.0.1:1', identity, rootKey:new Uint8Array([1]), intents,
  binding:{key:`${service}:${uploader}:1`,service,tenant:uploader,uploader,operation:'1',root:prepared.hash,
    project:'offline-fixture',bucket:'offline-fixture',permission:[68,73,68,76]},
  body,bodySha256:digest(body),manifestJSON:prepared.manifestJSON,contentType:'image/png',maxBodyBytes:1024,
  origin:'https://substitute.invalid',maxRequests:2,maxRequestBytes:65536,maxTotalRequestBytes:131072,
  certificateFetch:transport,gatewayFetch:transport };
const cases = [];
async function refuses(name, changes, expected) {
  await assert.rejects(createPublicationUpload({...base,...changes}), error => error.code === expected);
  assert.equal(storeCalls,0); assert.equal(transportCalls,0);
  cases.push({name,code:expected,intentAccess:0,transportRequests:0});
}
await refuses('corrupt_body',{body:new Uint8Array(1024)},'body-digest');
await refuses('different_expected_digest',{bodySha256:'0'.repeat(64)},'body-digest');
await refuses('bounded_body',{maxBodyBytes:1023},'body-size');
await refuses('malformed_digest',{bodySha256:'SHA256:00'},'body-digest');
await refuses('empty_body',{body:new Uint8Array()},'body-size');
await refuses('wrong_root',{binding:{...base.binding,root:`sha256:${'0'.repeat(64)}`}},'root');
await refuses('wrong_metadata_hint',{contentType:'text/plain'},'root');
await refuses('oversized_metadata_hint',{filename:'x'.repeat(4097)},'metadata-hint');
await refuses('oversized_utf8_filename',{filename:'é'.repeat(2049)},'metadata-hint');
await refuses('oversized_utf8_content_type',{contentType:'é'.repeat(2049)},'metadata-hint');
await refuses('null_metadata_hint',{filename:null},'metadata-hint');
await refuses('malformed_manifest',{manifestJSON:'{'},'manifest');
await refuses('oversized_manifest',{manifestJSON:'x'.repeat(256*1024+1)},'manifest-size');
await refuses('oversized_utf8_manifest',{manifestJSON:'é'.repeat(128*1024+1)},'manifest-size');
await refuses('empty_manifest',{manifestJSON:''},'manifest-size');
for (const field of ['headers','chunk_hashes','tree_type']) {
  const changed = JSON.parse(prepared.manifestJSON);
  changed[field] = field === 'tree_type' ? 'different' : [];
  await refuses(`changed_${field}`,{manifestJSON:JSON.stringify(changed)},'manifest');
}
const controller = new AbortController(); controller.abort();
await assert.rejects(createPublicationUpload({...base,signal:controller.signal}), error => error.name==='AbortError');
assert.equal(storeCalls,0); assert.equal(transportCalls,0);
cases.push({name:'aborted_before_preparation',code:'AbortError',intentAccess:0,transportRequests:0});
// Mutation during the crypto/SDK await cannot substitute the frozen input. The
// deliberately refusing store is reached only after the original bytes/root match.
const bytes = body.slice(), binding = structuredClone(base.binding), rootKey = base.rootKey.slice();
const pending = createPublicationUpload({...base,body:bytes,binding,rootKey});
bytes.fill(0); binding.root = `sha256:${'0'.repeat(64)}`; rootKey.fill(9);
await assert.rejects(pending, error => error === intentTrap);
assert.equal(storeCalls,1); assert.equal(transportCalls,0);
assert.equal(savedBinding.root, prepared.hash);
assert.deepEqual(savedBinding.icRootKey, [1]);
cases.push({name:'input_snapshots_across_await',reachesOriginalBoundIntent:true,transportRequests:0});
await record('summary.json',{schema:1,complete:true,cases,providerRequests:0,certificateRequests:0,
  caveat:'Deliberately refusing store and transport; actual IC/IndexedDB behavior is qualified separately.'});
console.log('PASS frozen-input refusal and mutation checks');
