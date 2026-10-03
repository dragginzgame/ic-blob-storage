// Only fixed test identity/evidence remain here; maintained host owns the port.
import { Ed25519KeyIdentity } from '@icp-sdk/core/identity';
import { createIndexedDBIntentStore } from '../../clients/browser/intents.js';
import { createPublicationWorkerHost } from '../../clients/browser/bootstrap.js';
export async function workerPublication(config, mode, reports) {
  const identity = Ed25519KeyIdentity.generate(new Uint8Array(32).fill(42));
  const host = await createPublicationWorkerHost({ workerURL: '/publication-worker.js',
    bootstrap: { schema: 1, operation: 'bootstrap',
      signer: { kind: 'ed25519', json: JSON.stringify(identity.toJSON()) },
      configuration: { host: config.url, rootKey: new Uint8Array(config.rootKey),
        service: config.service, tenant: config.tenant, uploader: identity.getPrincipal().toText(),
        project: config.project, bucket: config.bucket, origin: config.gateway,
        maxBodyBytes: config.maxBodyBytes, maxRequests: 2, maxRequestBytes: 65536,
        maxTotalRequestBytes: 131072, maxJobs: 32, timeoutSeconds: 120 },
      journal: { database: 'standalone-trial-v1', maxSlots: config.journalSlots, mode } } });
  const intents = await createIndexedDBIntentStore({ database: 'standalone-trial-v1',
    maxSlots: config.journalSlots, mode: 'open' });
  const binding = { ...config.binding, icOrigin: new URL(config.url).origin, icRootKey: config.rootKey };
  let id = 0;
  async function execute(action) {
    const job = { schema: 1, id: ++id, action, index: config.index ?? 0, binding: config.binding };
    if (action === 'upload') job.snapshot = { body: new Uint8Array(config.snapshot.body),
      bodySha256: config.snapshot.bodySha256, manifestJSON: config.snapshot.manifestJSON,
      contentType: 'image/png' };
    const result = await host.execute(job); reports.push(result); return result;
  }
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
      return intents.inspect(binding); // Private fixture evidence, never host/worker RPC output.
    },
    async recover() {
      const result = accepted(await execute('recover-certificate'), 'certificate-observed');
      return { row: await intents.inspect(binding), certificateBytes: result.certificate_bytes };
    },
    async cancel() { accepted(await execute('cancel'), 'cancelled'); return intents.inspect(binding); },
    close() { host.close(); intents.close(); },
  };
}
