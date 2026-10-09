// Local standalone trial harness; maintained clients own all certificate/upload rules.
import { Ed25519KeyIdentity } from '@icp-sdk/core/identity';
import { StorageClient } from '@caffeineai/object-storage';
import { createPublicationUpload } from '../../clients/browser/publication.js';
import { createIndexedDBIntentStore } from '../../clients/browser/intents.js';
import { workerPublication } from './worker-client.js';
let publication;
const workerReports = [];
let calls = 0;
function inputs(config) {
  const identity = Ed25519KeyIdentity.generate(new Uint8Array(32).fill(config.identitySeed ?? 42));
  return { host: config.url, identity, rootKey: new Uint8Array(config.rootKey),
    binding: config.binding, body: new Uint8Array(config.snapshot.body),
    bodySha256: config.snapshot.bodySha256, manifestJSON: config.snapshot.manifestJSON,
    contentType: 'image/png', maxBodyBytes: config.maxBodyBytes ?? 1024, origin: config.gateway,
    maxRequests: 2, maxRequestBytes: 65536, maxTotalRequestBytes: 131072,
    certificateFetch: (url, init) => {
      if (new URL(url).pathname.endsWith('/call')) calls++;
      return fetch(url, init);
    } };
}
window.trial = {
  async plan(size = 1024, byte = 42) {
    const { hash, byteLength, manifestJSON } = await StorageClient.prepareFile(
      new Uint8Array(size).fill(byte), 'image/png');
    return { hash, byteLength, manifestJSON };
  },
  async setup(config, mode) {
    publication?.close?.();
    if (config.worker) {
      publication = await workerPublication(config, mode, workerReports);
      return;
    }
    const intents = await createIndexedDBIntentStore({
      database: 'standalone-trial-v1', maxSlots: config.journalSlots ?? 1, mode });
    const candidate = inputs(config);
    const upload = await createPublicationUpload({ ...candidate, intents });
    publication = { upload: upload.upload, inspect: upload.inspect, cancel: upload.cancel,
      async recover() {
        const { certificate: proof, ...row } = await upload.recoverCertificate();
        return { row, certificateBytes: proof.length };
      } };
  },
  upload: () => publication.upload(),
  inspect: () => publication.inspect(),
  recover: () => publication.recover(),
  cancel: () => publication.cancel(),
  calls: () => calls,
  workerReports: () => workerReports,
};
