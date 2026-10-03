import { createCertificateClient, CertificateRefusal } from './certificate.js';
import { createPublicationUpload, PublicationRefusal } from './publication.js';
import { GatewayRefusal } from './gateway.js';
import { TransferRefusal } from './transfer.js';
import { IntentRefusal } from './intents.js';
import { WorkerRefusal, fields, integer, publicationConfiguration, snapshotPublicationJob, actions } from './worker-configuration.js';
export { WorkerRefusal } from './worker-configuration.js';
const require = (condition, code) => { if (!condition) throw new WorkerRefusal(code); };
// Only this finite public vocabulary may cross the private worker boundary.
// Exported error constructors and host-provided transports can carry arbitrary
// code strings; class identity alone does not make their payload safe to return.
const publicCodes = new Set(['message', 'origin', 'principal', 'identity', 'namespace',
  'trust-root', 'limits', 'store', 'scope', 'binding', 'body-size', 'body-digest',
  'body-limit', 'manifest', 'manifest-size', 'metadata-hint', 'root', 'job-budget',
  'worker-busy', 'upload-claimed', 'history-missing', 'port', 'intent-binding', 'operation', 'permission', 'key',
  'dispatch-blocked', 'gateway-blocked', 'gateway-uncertain', 'store-timeout',
  'store-blocked', 'store-missing', 'store-capacity', 'store-closed', 'intent-corrupt']);
function errorCode(error) {
  if ([WorkerRefusal, CertificateRefusal, PublicationRefusal, GatewayRefusal,
    TransferRefusal, IntentRefusal].some(type => error instanceof type) &&
    publicCodes.has(error.code)) return error.code;
  if (error?.name === 'AbortError' || error?.name === 'TimeoutError') return 'aborted';
  // SDK/provider/storage messages can include URLs, signed envelopes or headers.
  return 'transport';
}
function journal(row) {
  if (!row) return { present: false };
  const requests = row.gateway?.requests ?? [];
  require(['saved', 'uncertain', 'observed'].includes(row.phase) &&
    typeof row.cancelled === 'boolean' && Array.isArray(requests) && requests.length <= 256 &&
    requests.every(request => ['uncertain', 'responded'].includes(request.phase) &&
      (request.phase === 'uncertain' ? request.status === undefined :
        integer(request.status, 200, 599))), 'intent-corrupt');
  return { present: true, certificate_phase: row.phase, cancelled: row.cancelled,
    gateway_requests: requests.map((request, index) => ({
      index, phase: request.phase, ...(request.status === undefined ? {} : { status: request.status }),
    })) };
}

/**
 * One explicitly configured browser upload worker, usable in a DedicatedWorker.
 * The caller supplies a trusted signer/store/port and original metadata hints.
 * Existing certificate/gateway journals own dispatch; process IDs and limits are
 * execution guards, not persistent effect identity or recovery authority.
 */
export function createPublicationWorker(options) {
  const { host, origin, scope, rootKey } = publicationConfiguration(options);
  const { identity, intents, maxBodyBytes, maxRequests, maxRequestBytes,
    maxTotalRequestBytes, maxJobs, timeoutSeconds } = options;
  for (const method of ['save', 'inspect', 'claim', 'observe', 'cancel',
    'claimGateway', 'observeGateway']) require(typeof intents?.[method] === 'function', 'store');
  const signal = AbortSignal.any([AbortSignal.timeout(timeoutSeconds * 1000),
    ...(options.signal ? [options.signal] : [])]);
  const transport = callback => (url, init) => callback(url, { ...init,
    signal: AbortSignal.any([signal, ...(init.signal ? [init.signal] : [])]) });
  const certificateFetch = transport(options.certificateFetch ?? globalThis.fetch.bind(globalThis));
  const gatewayFetch = transport(options.gatewayFetch ?? globalThis.fetch.bind(globalThis));
  let active = false, jobs = 0, lastId = 0;
  return Object.freeze({
    async execute(request) {
      let ownsActive = false, current;
      const correlation = { id: integer(request?.id, 1, 1_000_000) ? request.id : null,
        index: integer(request?.index, 0, 4095) ? request.index : null,
        action: actions.includes(request?.action) ? request.action : null };
      const report = value => ({ schema: 1, ...correlation, ...value,
        service_completion_checked: false, retry_authorized: false });
      try {
        require(++jobs <= maxJobs, 'job-budget');
        require(!active, 'worker-busy');
        signal.throwIfAborted();
        require(identity.getPrincipal().toText() === scope.uploader, 'identity');
        current = snapshotPublicationJob(request, scope, maxBodyBytes, lastId); lastId = current.id;
        active = true; ownsActive = true;
        const binding = { ...current.binding, icOrigin: host, icRootKey: Array.from(rootKey) };
        const row = await intents.inspect(binding);
        signal.throwIfAborted();
        if (current.action === 'inspect') return report({ state: 'inspected', journal: journal(row) });
        if (current.action === 'upload') {
          require(!row || (row.phase === 'saved' && !row.cancelled && !row.gateway), 'upload-claimed');
          const upload = await createPublicationUpload({ host, identity, rootKey,
            binding: current.binding, intents, ...current.snapshot, origin, maxBodyBytes,
            maxRequests, maxRequestBytes, maxTotalRequestBytes, signal,
            certificateFetch, gatewayFetch });
          await upload.upload();
          return report({ state: 'transfer-observed', requires_verification: true,
            journal: journal(await upload.inspect()) });
        }
        require(row, 'history-missing');
        const certificate = await createCertificateClient({ host, identity, rootKey,
          binding: current.binding, intents, fetch: certificateFetch });
        if (current.action === 'cancel') return report({ state: 'cancelled',
          journal: journal(await certificate.cancel()) });
        const recovered = await certificate.recover();
        return report({ state: 'certificate-observed', requires_verification: true,
          certificate_bytes: recovered.certificate.length,
          journal: journal(await certificate.inspect()) });
      } catch (error) {
        return report({ state: 'failed', error: errorCode(error), original_journals_must_survive: true });
      } finally { if (ownsActive) active = false; }
    },
  });
}

/** Serve an already trusted private MessagePort; do not bind a public window listener. */
export function servePublicationWorker(worker, port) {
  require(typeof worker?.execute === 'function' && ['postMessage', 'addEventListener',
    'removeEventListener', 'start', 'close'].every(method => typeof port?.[method] === 'function'), 'port');
  const receive = async event => port.postMessage(await worker.execute(event.data));
  port.addEventListener('message', receive); port.start();
  return () => { port.removeEventListener('message', receive); port.close(); };
}
