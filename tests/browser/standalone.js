// Local standalone trial harness; maintained clients own all certificate/upload rules.
import { Ed25519KeyIdentity } from '@icp-sdk/core/identity';
import { StorageClient } from '@caffeineai/object-storage';
import { createPublicationUpload } from '../../clients/browser/publication.js';
import { createIndexedDBIntentStore } from '../../clients/browser/intents.js';
let publication;
let calls = 0;
const identity = Ed25519KeyIdentity.generate(new Uint8Array(32).fill(42));
const journal = { database: 'standalone-trial-v1', maxSlots: 1 };
function inputs(config) {
  return { host: config.url, identity, rootKey: new Uint8Array(config.rootKey),
    binding: config.binding, body: new Uint8Array(config.snapshot.body),
    bodySha256: config.snapshot.bodySha256, manifestJSON: config.snapshot.manifestJSON,
    contentType: 'image/png', maxBodyBytes: 1024, origin: config.gateway,
    maxRequests: 2, maxRequestBytes: 65536, maxTotalRequestBytes: 131072,
    certificateFetch: (url, init) => {
      if (new URL(url).pathname.endsWith('/call')) calls++;
      return fetch(url, init);
    } };
}
window.trial = {
  plan: () => StorageClient.prepareFile(new Uint8Array(1024).fill(42), 'image/png'),
  async setup(config, mode) {
    const intents = await createIndexedDBIntentStore({ ...journal, mode });
    const candidate = inputs(config);
    publication = await createPublicationUpload({ ...candidate, intents });
  },
  upload: () => publication.upload(),
  inspect: () => publication.inspect(),
  recover: async () => {
    const { certificate: proof, ...row } = await publication.recoverCertificate();
    return { row, certificateBytes: proof.length };
  },
  cancel: () => publication.cancel(),
  calls: () => calls,
};
