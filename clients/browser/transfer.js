import { StorageClient } from '@caffeineai/object-storage';
import { createGatewayTransport } from './gateway.js';
import { validNamespace } from './namespace.js';

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
export async function createUploadTransfer(options) {
  const allowed = ['certificate', 'intents', 'origin', 'maxRequests', 'maxRequestBytes',
    'maxTotalRequestBytes', 'signal', 'fetch'];
  require(options && Object.keys(options).every(key => allowed.includes(key)), 'options');
  const { certificate, intents, origin, maxRequests, maxRequestBytes,
    maxTotalRequestBytes, signal, fetch } = options;
  require(typeof StorageClient.prepareFile === 'function' &&
    typeof StorageClient.prototype.uploadPrepared === 'function', 'sdk');
  const binding = structuredClone((await certificate.inspect()).binding);
  require(binding && typeof binding.service === 'string' &&
    typeof binding.root === 'string', 'binding');
  require(validNamespace(binding.project, true) && validNamespace(binding.bucket), 'namespace');
  const transport = await createGatewayTransport({ certificate, intents, origin,
    maxRequests, maxRequestBytes, maxTotalRequestBytes, fetch });
  const storage = new StorageClient(binding.bucket, new URL(origin).origin, binding.service,
    binding.project, certificate.certificateAgent, {
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
