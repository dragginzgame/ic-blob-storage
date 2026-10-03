// Test-only signer/bootstrap; actual dedicated-worker behavior uses maintained clients.
import { Ed25519KeyIdentity } from '@icp-sdk/core/identity';
import { createIndexedDBIntentStore } from '../../clients/browser/intents.js';
import { createPublicationWorker, servePublicationWorker } from '../../clients/browser/worker.js';
let booted = false;
self.onmessage = async ({ data, ports }) => {
  if (booted || ports.length !== 1) return;
  booted = true;
  const port = ports[0], config = data.configuration;
  try {
    const identity = Ed25519KeyIdentity.generate(new Uint8Array(32).fill(42));
    const intents = await createIndexedDBIntentStore({ database: 'standalone-trial-v1',
      maxSlots: config.journalSlots, mode: data.mode });
    const worker = createPublicationWorker({ host: config.url, identity,
      rootKey: new Uint8Array(config.rootKey), service: config.service, tenant: config.tenant,
      uploader: identity.getPrincipal().toText(), project: config.project, bucket: config.bucket,
      intents, origin: config.gateway, maxBodyBytes: config.maxBodyBytes,
      maxRequests: 2, maxRequestBytes: 65536, maxTotalRequestBytes: 131072,
      maxJobs: 32, timeoutSeconds: 120, certificateFetch: (url, init) => {
        if (new URL(url).pathname.endsWith('/call')) port.postMessage({ event: 'certificate-call' });
        return fetch(url, init);
      } });
    servePublicationWorker(worker, port);
    port.postMessage({ event: 'ready' });
  } catch { port.postMessage({ event: 'bootstrap-failed' }); }
};
