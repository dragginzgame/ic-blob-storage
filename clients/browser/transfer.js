import { StorageClient } from '@caffeineai/object-storage';
import { createGatewayTransport } from './gateway.js';

/** Local composition refusal; it grants no provider or service authority. */
export class TransferRefusal extends Error {
  constructor(code) { super(code); this.name = 'TransferRefusal'; this.code = code; }
}
const require = (condition, code) => { if (!condition) throw new TransferRefusal(code); };

/**
 * Compose the maintained patched Caffeine SDK with one existing certificate intent.
 * The service owner/root come from that intent; namespace provisioning remains a
 * host obligation. The returned transport is the same guarded request owner used
 * by the SDK, not a second session. SDK success is never service completion.
 */
export async function createUploadTransfer({ certificate, intents, origin, bucket, project,
  maxRequests, maxRequestBytes, maxTotalRequestBytes, signal, fetch }) {
  require(typeof StorageClient.prepareFile === 'function' &&
    typeof StorageClient.prototype.uploadPrepared === 'function', 'sdk');
  for (const value of [bucket, project]) {
    require(typeof value === 'string' && value.length > 0 &&
      value.trim() === value && !/[\u0000-\u001f\u007f]/u.test(value) &&
      new TextEncoder().encode(value).length <= 256, 'namespace');
  }
  // The project is also an HTTP header in the pinned SDK's chunk request.
  try { new Headers({ 'X-Caffeine-Project-ID': project }); }
  catch { throw new TransferRefusal('namespace'); }
  const binding = structuredClone((await certificate.inspect()).binding);
  require(binding && typeof binding.service === 'string' &&
    typeof binding.root === 'string', 'binding');
  const transport = await createGatewayTransport({ certificate, intents, origin,
    maxRequests, maxRequestBytes, maxTotalRequestBytes, fetch });
  const storage = new StorageClient(bucket, new URL(origin).origin, binding.service,
    project, certificate.certificateAgent, {
      retry: false, concurrency: 1, signal, fetch: transport,
    });
  return Object.freeze({
    transport,
    // Caffeine owns preparation, chunking, wire formats and consumption of handles.
    async uploadPrepared(prepared, onProgress) {
      require(prepared?.hash === binding.root, 'root');
      const result = await storage.uploadPrepared(prepared, onProgress);
      require(result.hash === binding.root, 'root');
      return result;
    },
  });
}
