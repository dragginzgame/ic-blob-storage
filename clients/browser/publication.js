import { StorageClient } from '@caffeineai/object-storage';
import { createCertificateClient } from './certificate.js';
import { createUploadTransfer } from './transfer.js';
import { validUtf8Text, validGatewayLimits } from './validation.js';

/** Frozen-input refusal, never evidence of service or provider completion. */
export class PublicationRefusal extends Error {
  constructor(code) { super(code); this.name = 'PublicationRefusal'; this.code = code; }
}
const require = (condition, code) => { if (!condition) throw new PublicationRefusal(code); };
const hex = bytes => Array.from(bytes, byte => byte.toString(16).padStart(2, '0')).join('');
const same = (a, b) => JSON.stringify(a) === JSON.stringify(b);

/**
 * Rebuild one frozen native body with Caffeine before any certificate intent.
 * Caller owns full-batch/setup validation, identity/root, persistent store and
 * budgets. This helper owns no IDs, provider format, retry or completion policy.
 */
export async function createPublicationUpload(options) {
  const allowed = ['host', 'identity', 'rootKey', 'binding', 'intents', 'body',
    'bodySha256', 'manifestJSON', 'maxBodyBytes', 'contentType', 'filename', 'cacheControl',
    'origin', 'maxRequests', 'maxRequestBytes', 'maxTotalRequestBytes', 'signal',
    'certificateFetch', 'gatewayFetch'];
  require(options && Object.keys(options).every(key => allowed.includes(key)), 'options');
  const { host, identity, rootKey, intents, contentType, filename, cacheControl, origin,
    maxRequests, maxRequestBytes, maxTotalRequestBytes, signal,
    certificateFetch, gatewayFetch, maxBodyBytes, bodySha256, manifestJSON } = options;
  require(Number.isSafeInteger(maxBodyBytes) && maxBodyBytes > 0 &&
    maxBodyBytes <= 1024 * 1024 * 1024, 'body-limit');
  require(options.body instanceof Uint8Array && options.body.length > 0 &&
    options.body.length <= maxBodyBytes, 'body-size');
  require(typeof bodySha256 === 'string' && /^[0-9a-f]{64}$/.test(bodySha256), 'body-digest');
  require(validUtf8Text(manifestJSON, 256 * 1024, 1), 'manifest-size');
  require(typeof StorageClient.prepareFile === 'function', 'sdk');
  for (const hint of [contentType, filename, cacheControl]) {
    require(hint === undefined || validUtf8Text(hint, 4096), 'metadata-hint');
  }
  // Own the bytes and binding before crypto/SDK awaits; caller mutation cannot
  // substitute another body, principal, root or namespace mid-preparation.
  const body = options.body.slice();
  const binding = structuredClone(options.binding);
  const trustRoot = rootKey instanceof Uint8Array ? rootKey.slice() : rootKey;
  signal?.throwIfAborted();
  require(hex(new Uint8Array(await crypto.subtle.digest('SHA-256', body))) === bodySha256, 'body-digest');
  let original;
  try { original = JSON.parse(manifestJSON); }
  catch { throw new PublicationRefusal('manifest'); }
  const prepared = await StorageClient.prepareFile(body, contentType, filename, cacheControl);
  signal?.throwIfAborted();
  require(prepared.byteLength === body.length && prepared.hash === binding.root, 'root');
  const rebuilt = JSON.parse(prepared.manifestJSON);
  // The SDK owns provider hashing/tree construction. Compare the original exact
  // metadata/ordered leaves and root; do not implement another provider decoder.
  require(original?.tree_type === rebuilt.tree_type && original?.tree?.hash === prepared.hash &&
    same(original.headers, rebuilt.headers) && same(original.chunk_hashes, rebuilt.chunk_hashes), 'manifest');
  // Reject known shortages before constructing the certificate client, which
  // saves intent. SDK chunk sizes own this lower bound; the later opaque tree
  // and certificate envelope still require the gateway's exact request checks.
  require(validGatewayLimits({ maxRequests, maxRequestBytes, maxTotalRequestBytes }) &&
    maxRequests >= 1 + rebuilt.chunk_hashes.length &&
    maxRequestBytes >= prepared.maxChunkBytes &&
    maxTotalRequestBytes >= prepared.byteLength, 'transfer-budget');
  const certificate = await createCertificateClient({ host, identity, rootKey: trustRoot,
    binding, intents, ...(certificateFetch ? { fetch: certificateFetch } : {}) });
  const transfer = await createUploadTransfer({ certificate, intents, origin,
    maxRequests, maxRequestBytes, maxTotalRequestBytes, signal,
    ...(gatewayFetch ? { fetch: gatewayFetch } : {}) });
  let started = false;
  return Object.freeze({
    async upload(onProgress) {
      signal?.throwIfAborted();
      require(!started, 'upload-claimed');
      const row = await certificate.inspect();
      require(!started && row.phase === 'saved' && !row.cancelled && !row.gateway, 'upload-claimed');
      // This local flag is only an execution guard; the existing atomic durable
      // certificate and gateway claims remain the authoritative dispatch journals.
      started = true;
      return transfer.uploadPrepared(prepared, onProgress);
    },
    inspect: () => certificate.inspect(),
    recoverCertificate: () => certificate.recover(),
    cancel: () => certificate.cancel(),
  });
}
