import { Ed25519KeyIdentity } from '@icp-sdk/core/identity';
import { Secp256k1KeyIdentity } from '@icp-sdk/core/identity/secp256k1';
import { createIndexedDBIntentStore } from './intents.js';
import { createPublicationWorker, servePublicationWorker } from './worker.js';
import { fields, publicationConfiguration, snapshotPublicationJob } from './worker-configuration.js';
import { validJournalConfiguration } from './validation.js';

/** Private host/bootstrap refusal. Never includes signer or transport contents. */
export class BootstrapRefusal extends Error {
  constructor(code) { super(code); this.name = 'BootstrapRefusal'; this.code = code; }
}
const require = (condition, code) => { if (!condition) throw new BootstrapRefusal(code); };

function input(value) {
  fields(value, ['schema', 'operation', 'signer', 'configuration', 'journal']);
  // Own one passive bootstrap before reading its fields or constructing identity.
  value = structuredClone(value);
  require(value.schema === 1 && value.operation === 'bootstrap', 'bootstrap');
  fields(value.signer, ['kind', 'json']);
  require(['ed25519', 'secp256k1'].includes(value.signer.kind) &&
    typeof value.signer.json === 'string' && value.signer.json.length <= 512, 'signer');
  const type = value.signer.kind === 'ed25519' ? Ed25519KeyIdentity : Secp256k1KeyIdentity;
  const parsed = JSON.parse(value.signer.json);
  require(Array.isArray(parsed) && parsed.length === 2 && parsed.every(part =>
    typeof part === 'string' && /^[0-9a-f]+$/.test(part)) && parsed[1].length === 64, 'signer');
  const advertised = type.fromJSON(value.signer.json);
  // The SDK JSON reader accepts a caller-provided public/private pair. Recompute
  // through that SDK before trusting its advertised principal; no custom crypto.
  const identity = type.fromSecretKey(advertised.getKeyPair().secretKey.slice());
  require(identity.getPrincipal().toText() === advertised.getPrincipal().toText(), 'signer');
  fields(value.configuration, ['host', 'rootKey', 'service', 'tenant', 'uploader',
    'project', 'bucket', 'origin', 'maxBodyBytes', 'maxRequests', 'maxRequestBytes',
    'maxTotalRequestBytes', 'maxJobs', 'timeoutSeconds']);
  fields(value.journal, ['database', 'maxSlots', 'mode']);
  require(validJournalConfiguration(value.journal), 'journal');
  // Validate every binding/budget before storage; reuse that checked scope for
  // host jobs. The independently launched worker still validates its boundary.
  const worker = publicationConfiguration({ ...value.configuration, identity, intents: null });
  value.configuration.rootKey = worker.rootKey;
  const options = { ...value.configuration, identity, intents: null };
  return { options, worker, journal: value.journal, payload: value };
}

/** Serve one bootstrap on an already trusted private port, then maintained jobs.
 * The creator owns the Worker/port and process termination. No global listener,
 * identity discovery, HTTP credential route, journal deletion or replay is added.
 */
export function servePublicationBootstrap(port) {
  require(['postMessage', 'addEventListener', 'removeEventListener', 'start', 'close']
    .every(method => typeof port?.[method] === 'function'), 'port');
  let booted = false, closed = false, intents, stopJobs;
  const close = () => {
    closed = true; port.removeEventListener('message', receive);
    port.removeEventListener('messageerror', close);
    stopJobs?.(); intents?.close(); port.close();
  };
  const refuse = () => {
    port.postMessage({ schema: 1, event: 'bootstrap-failed', error: 'bootstrap',
      service_completion_checked: false, retry_authorized: false }); close();
  };
  async function receive({ data }) {
    if (closed) return;
    if (booted) { refuse(); return; }
    booted = true;
    try {
      const prepared = input(data);
      intents = await createIndexedDBIntentStore(prepared.journal);
      if (closed) { intents.close(); return; }
      const worker = createPublicationWorker({ ...prepared.options, intents });
      port.removeEventListener('message', receive);
      stopJobs = servePublicationWorker(worker, port);
      port.postMessage({ schema: 1, event: 'ready', service_completion_checked: false,
        retry_authorized: false });
    } catch { if (!closed) refuse(); }
  }
  port.addEventListener('message', receive); port.addEventListener('messageerror', close); port.start();
  return close;
}

/** Launch the explicitly named, same-origin maintained worker entry. Private
 * SDK identity JSON crosses only the transferred port. Closing/deadline terminates
 * the worker, but never erases its profile or resolves an uncertain effect.
 */
export async function createPublicationWorkerHost(settings) {
  fields(settings, ['workerURL', 'bootstrap'], ['signal']);
  const { workerURL, bootstrap, signal } = settings;
  let configuration;
  try { configuration = input(bootstrap); }
  catch { throw new BootstrapRefusal('bootstrap'); }
  const owned = configuration.payload;
  require(typeof workerURL === 'string' && workerURL.length <= 4096, 'worker-origin');
  let url;
  try { url = new URL(workerURL, globalThis.location.href); }
  catch { throw new BootstrapRefusal('worker-origin'); }
  require(url.origin === globalThis.location.origin && !url.username && !url.password &&
    !url.search && !url.hash, 'worker-origin');
  require(signal === undefined || signal instanceof AbortSignal, 'signal');
  require(!signal?.aborted, 'host-closed');
  let worker;
  try { worker = new Worker(url, { type: 'module' }); }
  catch { throw new BootstrapRefusal('bootstrap'); }
  const channel = new MessageChannel();
  let closed = false, started = false, pending, ready, failed;
  const starting = new Promise((resolve, reject) => { ready = resolve; failed = reject; });
  function close() {
    if (closed) return;
    closed = true; clearTimeout(timer); signal?.removeEventListener('abort', close);
    worker.terminate(); channel.port1.close();
    const error = new BootstrapRefusal('host-closed');
    failed(error); pending?.reject(error); pending = undefined;
  }
  const timer = setTimeout(close, configuration.options.timeoutSeconds * 1000);
  signal?.addEventListener('abort', close, { once: true });
  worker.addEventListener('error', close);
  channel.port1.addEventListener('messageerror', close);
  channel.port1.onmessage = ({ data }) => {
    if (!started && data?.event === 'ready' && data.schema === 1 &&
      data.service_completion_checked === false && data.retry_authorized === false &&
      Object.keys(data).length === 4) { started = true; ready(); return; }
    if (data?.event === 'bootstrap-failed') { close(); return; }
    if (!pending || data?.schema !== 1 || data.id !== pending.job.id ||
      data.index !== pending.job.index || data.action !== pending.job.action ||
      data.service_completion_checked !== false || data.retry_authorized !== false) { close(); return; }
    const resolve = pending.resolve; pending = undefined; resolve(data);
  };
  worker.postMessage({ schema: 1, operation: 'private-port' }, [channel.port2]);
  channel.port1.postMessage(owned);
  try { await starting; } catch { close(); throw new BootstrapRefusal('bootstrap'); }
  return Object.freeze({
    execute(job) {
      require(!closed, 'host-closed'); require(!pending, 'host-busy');
      let original;
      try { original = snapshotPublicationJob(job, configuration.worker.scope, configuration.options.maxBodyBytes); }
      catch { throw new BootstrapRefusal('message'); }
      return new Promise((resolve, reject) => {
        pending = { job: original, resolve, reject };
        const transfer = original.action === 'upload' ? [original.snapshot.body.buffer] : [];
        try { channel.port1.postMessage(original, transfer); } catch { close(); }
      });
    },
    close,
  });
}
