// Local standalone trial harness; maintained clients own all certificate/upload rules.
import { HttpAgent } from '@icp-sdk/core/agent';
import { Ed25519KeyIdentity } from '@icp-sdk/core/identity';
import { StorageClient } from '@caffeineai/object-storage';
import { createCertificateClient } from '../../clients/browser/certificate.js';
import { createUploadTransfer } from '../../clients/browser/transfer.js';
import { createIndexedDBIntentStore } from '../../clients/browser/intents.js';
let prepared, certificate, transfer, gateway;
let calls = 0;
const identity = Ed25519KeyIdentity.generate(new Uint8Array(32).fill(42));
const journal = { database: 'standalone-trial-v1', maxSlots: 1 };
window.trial = {
  plan: () => prepared = StorageClient.prepareFile(new Uint8Array(1024).fill(42), 'image/png'),
  async prepare(config, bytes) {
    const agent = new HttpAgent({ host: config.url, identity,
      rootKey: new Uint8Array(config.rootKey), shouldFetchRootKey: false,
      shouldSyncTime: false, retryTimes: 0 });
    const result = await agent.update(config.service, { methodName: 'blob_prepare_upload',
      effectiveCanisterId: config.service, arg: new Uint8Array(bytes), callSync: true });
    if (!result.reply || result.reply.length > 4096) throw new Error('bounded preparation reply');
    return Array.from(result.reply);
  },
  async setup(config, mode) {
    gateway = config.gateway;
    const intents = await createIndexedDBIntentStore({ ...journal, mode });
    certificate = await createCertificateClient({ host: config.url, identity,
      rootKey: new Uint8Array(config.rootKey), binding: config.binding, intents,
      fetch: (url, init) => {
        if (new URL(url).pathname.endsWith('/call')) calls++;
        return fetch(url, init);
      } });
    transfer = await createUploadTransfer({ certificate, intents, origin: config.gateway,
      maxRequests: 2, maxRequestBytes: 65536, maxTotalRequestBytes: 131072 });
  },
  async upload() { return transfer.uploadPrepared(await prepared); },
  inspect: () => certificate.inspect(),
  recover: async () => {
    const { certificate: proof, ...row } = await certificate.recover();
    return { row, certificateBytes: proof.length };
  },
  issue: () => certificate.issue(),
  cancel: () => certificate.cancel(),
  calls: () => calls,
  probe: () => transfer.transport(`${gateway}/probe`, { method: 'PUT', body: new Uint8Array([1]) }),
};
