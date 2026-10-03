// Owned fixture's private port; this is not a production signer/bootstrap policy.
import { createIndexedDBIntentStore } from '../../clients/browser/intents.js';
export async function workerPublication(config, mode, onCall, reports) {
  const browserWorker = new Worker('/publication-worker.js', { type: 'module' });
  const channel = new MessageChannel(), pending = new Map();
  let id = 0, ready, failed;
  const started = new Promise((resolve, reject) => { ready = resolve; failed = reject; });
  channel.port1.onmessage = ({ data }) => {
    if (data.event === 'ready') return ready();
    if (data.event === 'bootstrap-failed') return failed(new Error('worker bootstrap refused'));
    if (data.event === 'certificate-call') return onCall();
    const resolve = pending.get(data.id); pending.delete(data.id);
    if (!resolve) throw new Error('unexpected worker reply');
    reports.push(data); resolve(data);
  };
  browserWorker.postMessage({ mode, configuration: {
    url: config.url, rootKey: config.rootKey, service: config.service, tenant: config.tenant,
    project: config.project, bucket: config.bucket, gateway: config.gateway,
    maxBodyBytes: config.maxBodyBytes, journalSlots: config.journalSlots,
  } }, [channel.port2]);
  await started;
  const intents = await createIndexedDBIntentStore({ database: 'standalone-trial-v1',
    maxSlots: config.journalSlots, mode: 'open' });
  const binding = { ...config.binding, icOrigin: new URL(config.url).origin, icRootKey: config.rootKey };
  const execute = action => new Promise(resolve => {
    const job = { schema: 1, id: ++id, action, index: config.index ?? 0, binding: config.binding };
    if (action === 'upload') job.snapshot = { body: new Uint8Array(config.snapshot.body),
      bodySha256: config.snapshot.bodySha256, manifestJSON: config.snapshot.manifestJSON,
      contentType: 'image/png' };
    pending.set(job.id, resolve); channel.port1.postMessage(job);
  });
  function accepted(result, state) {
    if (result.state === state) return result;
    const error = new Error(result.error); error.name = 'WorkerJobRefusal'; error.code = result.error;
    throw error;
  }
  return {
    async upload() { accepted(await execute('upload'), 'transfer-observed'); return { hash: config.binding.root }; },
    async inspect() {
      const result = accepted(await execute('inspect'), 'inspected');
      if (!result.journal.present) throw new Error('worker history missing');
      return intents.inspect(binding); // Private fixture evidence, never worker RPC output.
    },
    async recover() {
      const result = accepted(await execute('recover-certificate'), 'certificate-observed');
      return { row: await intents.inspect(binding), certificateBytes: result.certificate_bytes };
    },
    async cancel() { accepted(await execute('cancel'), 'cancelled'); return intents.inspect(binding); },
    close() { browserWorker.terminate(); channel.port1.close(); intents.close(); },
  };
}
