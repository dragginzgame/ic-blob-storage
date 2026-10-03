// Only test identities and boundary fixtures live here; maintained host/entry do work.
import { Ed25519KeyIdentity } from '@icp-sdk/core/identity';
import { Secp256k1KeyIdentity } from '@icp-sdk/core/identity/secp256k1';
import { createPublicationWorkerHost } from '../../clients/browser/bootstrap.js';
window.bootstrapFixture = {
  host: createPublicationWorkerHost,
  input(kind, database, mode = 'create', timeoutSeconds = 120) {
    const type = kind === 'ed25519' ? Ed25519KeyIdentity : Secp256k1KeyIdentity;
    const identity = type.fromSecretKey(new Uint8Array(32).fill(42));
    const uploader = identity.getPrincipal().toText();
    return { schema: 1, operation: 'bootstrap', signer: { kind, json: JSON.stringify(identity.toJSON()) },
      configuration: { host: location.origin, rootKey: new Uint8Array([1]), service: 'aaaaa-aa',
        tenant: 'rrkah-fqaaa-aaaaa-aaaaq-cai', uploader, project: 'bootstrap-fixture',
        bucket: 'test', origin: 'https://substitute.invalid', maxBodyBytes: 1024,
        maxRequests: 2, maxRequestBytes: 65536, maxTotalRequestBytes: 131072,
        maxJobs: 32, timeoutSeconds }, journal: { database, mode, maxSlots: 2 } };
  },
  job(config, id = 1, action = 'inspect') {
    return { schema: 1, id, action, index: 0, binding: {
      key: 'aaaaa-aa:rrkah-fqaaa-aaaaa-aaaaq-cai:1', service: 'aaaaa-aa',
      tenant: config.configuration.tenant, uploader: config.configuration.uploader,
      project: config.configuration.project, bucket: config.configuration.bucket,
      operation: '1', root: 'sha256:' + '1'.repeat(64), permission: [68, 73, 68, 76] } };
  },
};
