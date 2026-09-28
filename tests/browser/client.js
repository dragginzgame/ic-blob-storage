// Local browser evidence only. This fixture contains no gateway upload operation.
import { HttpAgent, Certificate, Cbor, requestIdOf, lookupResultToBuffer } from '@icp-sdk/core/agent';
import { Ed25519KeyIdentity } from '@icp-sdk/core/identity';
import { Principal } from '@icp-sdk/core/principal';
import { IDL } from '@icp-sdk/core/candid';
import * as intents from './intent.js';

const METHOD = '_immutableObjectStorageCreateCertificate';
const encoder = new TextEncoder();
const hex = value => Array.from(value, b => b.toString(16).padStart(2, '0')).join('');
const unhex = value => Uint8Array.from(value.match(/../g), b => parseInt(b, 16));
const equal = (a, b) => hex(a) === hex(b);
let cfg, agent, binding;
let calls = 0;
let abortClaim = false, lastProof;
let responseHeld, releaseResponse;
async function setup(config) {
  cfg = config;
  const identity = Ed25519KeyIdentity.generate(new Uint8Array(32).fill(42));
  if (identity.getPrincipal().toText() !== cfg.uploader) throw new intents.Refusal('identity');
  binding = { key: `${cfg.service}:${cfg.tenant}:${cfg.operation}`, service: cfg.service,
    tenant: cfg.tenant, uploader: cfg.uploader, operation: cfg.operation,
    permission: cfg.permission, root: cfg.root };
  agent = await HttpAgent.create({ host: cfg.url, identity, rootKey: new Uint8Array(cfg.rootKey),
    shouldFetchRootKey: false, shouldSyncTime: false, retryTimes: 0,
    fetch: guardedFetch });
  await intents.save(binding);
}
async function guardedFetch(url, init) {
  const target = new URL(url);
  if (target.origin !== new URL(cfg.url).origin) throw new intents.Refusal('origin');
  if (target.pathname.endsWith('/call')) {
    if (target.pathname !== `/api/v4/canister/${cfg.service}/call`) throw new intents.Refusal('route');
    const bytes = new Uint8Array(init.body);
    if (bytes.length > 8192) throw new intents.Refusal('envelope-size');
    const envelope = Cbor.decode(bytes);
    const content = envelope.content;
    if (content.request_type !== 'call' || content.method_name !== METHOD ||
        Principal.fromUint8Array(content.sender).toText() !== cfg.uploader ||
        Principal.fromUint8Array(content.canister_id).toText() !== cfg.service ||
        !equal(content.arg, IDL.encode([IDL.Text], [cfg.root]))) throw new intents.Refusal('envelope');
    const requestId = hex(requestIdOf(content));
    // Wait for IndexedDB transaction completion before the first network effect.
    await intents.claim(binding.key, Array.from(bytes), requestId, abortClaim);
    calls += 1;
    const response = await boundedFetch(url, init);
    if (cfg.holdResponse) {
      responseHeld = true;
      await new Promise(resolve => { releaseResponse = resolve; });
    }
    return response;
  }
  return boundedFetch(url, init);
}
async function boundedFetch(url, init) {
  const response = await fetch(url, { ...init, redirect: 'error', signal: AbortSignal.timeout(20_000) });
  const reader = response.body.getReader();
  const chunks = [];
  let length = 0;
  for (;;) {
    const { done, value } = await reader.read();
    if (done) break;
    length += value.length;
    if (length > 256 * 1024) { await reader.cancel(); throw new intents.Refusal('response-size'); }
    chunks.push(value);
  }
  const body = new Uint8Array(length);
  let offset = 0;
  for (const chunk of chunks) { body.set(chunk, offset); offset += chunk.length; }
  return new Response(body, { status: response.status, statusText: response.statusText, headers: response.headers });
}
async function accept(raw, requestId) {
  if (!(raw instanceof Uint8Array) || raw.length > 128 * 1024) throw new intents.Refusal('certificate-size');
  const row = await intents.inspect(binding.key);
  if (!row?.envelope || row.requestId !== hex(requestId)) throw new intents.Refusal('request-id');
  const content = Cbor.decode(new Uint8Array(row.envelope)).content;
  if (hex(requestIdOf(content)) !== row.requestId) throw new intents.Refusal('saved-envelope');
  const certificate = await Certificate.create({ certificate: raw, rootKey: new Uint8Array(cfg.rootKey),
    principal: { canisterId: Principal.fromText(cfg.service) } });
  const path = [encoder.encode('request_status'), requestId];
  const status = lookupResultToBuffer(certificate.lookup_path([...path, encoder.encode('status')]));
  if (!status || new TextDecoder().decode(status) !== 'replied') throw new intents.Refusal('status');
  const reply = lookupResultToBuffer(certificate.lookup_path([...path, encoder.encode('reply')]));
  const [value] = IDL.decode([IDL.Record({ method: IDL.Text, blob_hash: IDL.Text })], reply);
  if (value.method !== 'upload' || value.blob_hash !== cfg.root) throw new intents.Refusal('reply');
  lastProof = { raw, requestId };
  return intents.observe(binding.key, row.requestId);
}
async function issue() {
  const result = await agent.call(Principal.fromText(cfg.service), {
    methodName: METHOD, arg: IDL.encode([IDL.Text], [cfg.root]), callSync: true });
  return accept(result.response.body.certificate, result.requestId);
}
async function recover() {
  const row = await intents.inspect(binding.key);
  if (!row?.requestId) throw new intents.Refusal('missing-request');
  const id = unhex(row.requestId);
  const response = await agent.readState({ canisterId: Principal.fromText(cfg.service) }, {
    paths: [[encoder.encode('request_status'), id]] });
  return accept(response.certificate, id);
}
window.fixture = { setup, issue, recover, inspect: () => intents.inspect(binding.key),
  cancel: () => intents.cancel(binding.key), calls: () => calls,
  held: () => !!responseHeld, release: () => releaseResponse?.(),
  abortClaim: value => { abortClaim = value; },
  rejectProof: async kind => {
    const { raw, requestId } = lastProof;
    if (kind === 'request') return accept(raw, new Uint8Array(32));
    if (kind === 'size') return accept(new Uint8Array(128 * 1024 + 1), requestId);
    const changed = Cbor.decode(raw);
    changed.signature[0] ^= 1;
    return accept(Cbor.encode(changed), requestId);
  },
  // Additional local lifecycle/transaction checks; none dispatch network effects.
  save: intents.save, claim: intents.claim, observe: intents.observe };
