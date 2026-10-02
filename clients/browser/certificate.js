import { HttpAgent, Certificate, Cbor, requestIdOf, lookupResultToBuffer } from '@icp-sdk/core/agent';
import { Principal } from '@icp-sdk/core/principal';
import { IDL } from '@icp-sdk/core/candid';
import { validNamespace } from './namespace.js';

const METHOD = '_immutableObjectStorageCreateCertificate';
const encoder = new TextEncoder();
const hex = bytes => Array.from(bytes, b => b.toString(16).padStart(2, '0')).join('');
const unhex = value => Uint8Array.from(value.match(/../g), b => parseInt(b, 16));
const same = (a, b) => a.length === b.length && a.every((byte, i) => byte === b[i]);

/** Local refusal; transport, authentication and storage errors also propagate unchanged. */
export class CertificateRefusal extends Error {
  constructor(code) { super(code); this.name = 'CertificateRefusal'; this.code = code; }
}
const require = (condition, code) => { if (!condition) throw new CertificateRefusal(code); };

/**
 * One permission's certificate transport. The caller owns identity, trust root and
 * atomic durable intent storage. See README.md for the required store contract.
 * No identity allocation, gateway upload, provider retry or publication is performed.
 */
export async function createCertificateClient({ host, identity, rootKey, binding, intents,
  fetch: transport = globalThis.fetch.bind(globalThis) }) {
  const endpoint = new URL(host);
  require(endpoint.protocol === 'https:' || (endpoint.protocol === 'http:' &&
    ['localhost', '127.0.0.1', '[::1]'].includes(endpoint.hostname)), 'host');
  require(!endpoint.username && !endpoint.password && endpoint.pathname === '/' &&
    !endpoint.search && !endpoint.hash, 'host');
  require(rootKey instanceof Uint8Array && rootKey.length > 0 && rootKey.length <= 1024, 'trust-root');
  const trustRoot = rootKey.slice();
  // Snapshot inputs so caller mutation cannot change an in-flight operation.
  const saved = structuredClone(binding);
  saved.icOrigin = endpoint.origin;
  saved.icRootKey = Array.from(trustRoot);
  require(validNamespace(saved.project, true) && validNamespace(saved.bucket), 'namespace');
  for (const field of ['service', 'tenant', 'uploader']) {
    require(typeof saved[field] === 'string' &&
      Principal.fromText(saved[field]).toText() === saved[field], 'principal');
  }
  require(saved.uploader !== Principal.anonymous().toText() &&
    identity.getPrincipal().toText() === saved.uploader, 'identity');
  require(typeof saved.operation === 'string' && /^(0|[1-9][0-9]{0,38})$/.test(saved.operation) &&
    BigInt(saved.operation) < (1n << 128n), 'operation');
  require(typeof saved.root === 'string' && /^sha256:[0-9a-f]{64}$/.test(saved.root), 'root');
  require(Array.isArray(saved.permission) && saved.permission.length > 0 &&
    saved.permission.length <= 64 * 1024 && saved.permission.every(byte =>
      Number.isInteger(byte) && byte >= 0 && byte <= 255), 'permission');
  require(saved.key === `${saved.service}:${saved.tenant}:${saved.operation}`, 'key');
  for (const method of ['save', 'inspect', 'claim', 'observe', 'cancel']) {
    require(typeof intents[method] === 'function', 'store');
  }
  const arg = new Uint8Array(IDL.encode([IDL.Text], [saved.root]));
  const service = Principal.fromText(saved.service);
  const callPath = `/api/v4/canister/${saved.service}/call`;
  const readPath = `/api/v3/canister/${saved.service}/read_state`;
  const copyBinding = () => structuredClone(saved);
  function bound(row) {
    require(row?.key === saved.key && row.binding &&
      ['key', 'service', 'tenant', 'uploader', 'operation', 'root', 'project', 'bucket', 'icOrigin'].every(field =>
        row.binding[field] === saved[field]) && Array.isArray(row.binding.permission) &&
      same(row.binding.permission, saved.permission) && Array.isArray(row.binding.icRootKey) &&
      same(row.binding.icRootKey, saved.icRootKey), 'intent-binding');
    require(typeof row.cancelled === 'boolean' &&
      ['saved', 'uncertain', 'observed'].includes(row.phase), 'intent-state');
    require(row.phase !== 'saved' || (row.envelope === undefined && row.requestId === undefined), 'intent-state');
    return row;
  }
  function envelopeId(bytes) {
    require(bytes instanceof Uint8Array && bytes.length > 0 && bytes.length <= 8192, 'envelope-size');
    const { content } = Cbor.decode(bytes);
    require(content.request_type === 'call' && content.method_name === METHOD &&
      Principal.fromUint8Array(content.sender).toText() === saved.uploader &&
      Principal.fromUint8Array(content.canister_id).toText() === saved.service &&
      same(new Uint8Array(content.arg), arg), 'envelope');
    return hex(requestIdOf(content));
  }
  function retained(row) {
    bound(row);
    require(['uncertain', 'observed'].includes(row.phase) && Array.isArray(row.envelope) &&
      row.envelope.length <= 8192 && row.envelope.every(byte =>
        Number.isInteger(byte) && byte >= 0 && byte <= 255), 'saved-envelope');
    require(envelopeId(new Uint8Array(row.envelope)) === row.requestId, 'saved-envelope');
    return row;
  }
  async function boundedFetch(url, init) {
    const response = await transport(url, { ...init, redirect: 'error', signal: AbortSignal.timeout(20_000) });
    require(response.body, 'response-body');
    const reader = response.body.getReader();
    const chunks = [];
    let length = 0;
    for (;;) {
      const { done, value } = await reader.read();
      if (done) break;
      length += value.length;
      if (length > 256 * 1024) { await reader.cancel(); throw new CertificateRefusal('response-size'); }
      chunks.push(value);
    }
    const body = new Uint8Array(length);
    let offset = 0;
    for (const chunk of chunks) { body.set(chunk, offset); offset += chunk.length; }
    return new Response(body, { status: response.status, statusText: response.statusText, headers: response.headers });
  }
  async function guardedFetch(url, init) {
    const target = new URL(url);
    require(target.origin === endpoint.origin && !target.search && !target.hash &&
      init.method === 'POST' && [callPath, readPath].includes(target.pathname), 'route');
    if (target.pathname === callPath) {
      const bytes = new Uint8Array(init.body);
      const requestId = envelopeId(bytes);
      const row = retained(await intents.claim(copyBinding(), Array.from(bytes), requestId));
      require(!row.cancelled && row.phase === 'uncertain' && row.requestId === requestId &&
        same(row.envelope, bytes), 'dispatch-binding');
      // claim must resolve only after its atomic durable transaction commits.
    }
    return boundedFetch(url, init);
  }
  bound(await intents.save(copyBinding()));
  // Caffeine accepts a real HttpAgent. Extend its public call boundary, leaving
  // request construction/signing and the provider's certificate extraction upstream.
  class IntentAgent extends HttpAgent {
    async call(...args) {
      const result = await super.call(...args);
      const observation = await observeCertificate(result.response.body?.certificate, result.requestId);
      require(!observation.cancelled, 'dispatch-blocked');
      return result;
    }
  }
  const agent = new IntentAgent({ host: endpoint.origin, identity, rootKey: trustRoot,
    shouldFetchRootKey: false, shouldSyncTime: false, retryTimes: 0, fetch: guardedFetch });

  /** Verify a historical response and persist observation without clearing cancellation. */
  async function observeCertificate(raw, requestId) {
    require(raw instanceof Uint8Array && raw.length <= 128 * 1024, 'certificate-size');
    require(requestId instanceof Uint8Array && requestId.length === 32, 'request-id');
    // Own buffers across crypto/storage awaits.
    const certificateBytes = raw.slice(), id = requestId.slice();
    const row = retained(await intents.inspect(copyBinding()));
    require(row.requestId === hex(id), 'request-id');
    const certificate = await Certificate.create({ certificate: certificateBytes, rootKey: trustRoot,
      principal: { canisterId: service } });
    const path = [encoder.encode('request_status'), id];
    const status = lookupResultToBuffer(certificate.lookup_path([...path, encoder.encode('status')]));
    require(status && new TextDecoder().decode(status) === 'replied', 'status');
    const reply = lookupResultToBuffer(certificate.lookup_path([...path, encoder.encode('reply')]));
    require(reply && reply.length <= 1024, 'reply');
    const [value] = IDL.decode([IDL.Record({ method: IDL.Text, blob_hash: IDL.Text })], reply);
    require(value.method === 'upload' && value.blob_hash === saved.root, 'reply');
    const observed = retained(await intents.observe(copyBinding(), row.requestId));
    require(observed.phase === 'observed' && observed.requestId === row.requestId &&
      same(observed.envelope, row.envelope) && (!row.cancelled || observed.cancelled), 'observation-binding');
    return { ...observed, certificate: certificateBytes };
  }
  return Object.freeze({
    certificateAgent: agent,
    async issue() {
      const row = bound(await intents.inspect(copyBinding()));
      require(!row.cancelled && row.phase === 'saved', 'dispatch-blocked');
      const result = await agent.call(service, { methodName: METHOD, arg, callSync: true });
      return { ...retained(await intents.inspect(copyBinding())), certificate: result.response.body.certificate };
    },
    async recover() {
      const row = retained(await intents.inspect(copyBinding()));
      const id = unhex(row.requestId);
      const response = await agent.readState(service, {
        paths: [[encoder.encode('request_status'), id]] });
      return observeCertificate(response.certificate, id);
    },
    inspect: async () => bound(await intents.inspect(copyBinding())),
    async cancel() {
      const row = bound(await intents.cancel(copyBinding()));
      require(row.cancelled, 'cancellation');
      return row;
    },
    observeCertificate,
  });
}
