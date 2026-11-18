// One installation per trusted browser origin/profile. Never evict or auto-reset.
import { providerBinding } from "./binding.js";
import { authenticateCertificate, sha256 } from "./certificate.js";
import { parseCanonicalPrincipal } from "@tokostudios/ic-platform/principal";

const DATABASE = "ic-blob-storage-transfer-v1";
const CAPACITY = 64;

async function configuration(input) {
  const owner = parseCanonicalPrincipal(input.issuer);
  const issuer = owner.toText();
  if (owner.isAnonymous() || issuer === "aaaaa-aa") throw new TypeError("Invalid issuer");
  const { gateway, project, bucket } = providerBinding({
    gateway: input.gateway, owner: issuer, project: input.project, bucket: input.bucket,
  });
  if (!(input.rootKey instanceof Uint8Array) || !(input.rootKey.buffer instanceof ArrayBuffer)
      || input.rootKey.length < 1 || input.rootKey.length > 1024) throw new TypeError("Invalid trust root");
  const rootKey = new Uint8Array(input.rootKey);
  return { version: 1, issuer, gateway, project, bucket,
    trustRootSha256: await sha256(rootKey) };
}

function open(create) {
  return new Promise((resolve, reject) => {
    let fresh = false, stopped = false;
    const request = indexedDB.open(DATABASE, 1);
    request.onblocked = () => { stopped = true; reject(new Error("Custody opening blocked")); };
    request.onupgradeneeded = event => {
      if (!create || stopped || event.oldVersion > 0) { request.transaction.abort(); return; }
      fresh = true;
      request.result.createObjectStore("binding");
      request.result.createObjectStore("claims");
    };
    request.onerror = () => reject(new Error("Custody missing or unavailable; never initialize to recover"));
    request.onsuccess = () => {
      const db = request.result;
      db.onversionchange = () => db.close();
      if (stopped || (create && !fresh) || [...db.objectStoreNames].join(",") !== "binding,claims") {
        db.close(); reject(new Error("Existing or incompatible custody; preserve it"));
      } else resolve(db);
    };
  });
}

function transaction(db, body) {
  return new Promise((resolve, reject) => {
    const tx = db.transaction(["binding", "claims"], "readwrite", { durability: "strict" });
    tx.oncomplete = resolve;
    tx.onabort = () => reject(new Error("Custody refused or uncertain; preserve it, never retry the transfer"));
    tx.onerror = () => {};
    if (tx.durability !== "strict") { tx.abort(); return; }
    body(tx);
  });
}

/** Explicit first-use provisioning for an operator-selected fresh trusted profile.
 * Never call this as recovery from missing/evicted/restored browser storage.
 * This API cannot detect profile rollback or coordinate other devices.
 */
export async function initializeCustody(input) {
  const config = await configuration(input);
  const db = await open(true);
  try { await transaction(db, tx => tx.objectStore("binding").add(config, 0)); }
  finally { db.close(); }
}

/** Authenticate original proof and supply the transfer client's claim callback.
 * Application permission must still be checked by the caller before each upload.
 * No network, signing, issuance, deletion, history compaction or automatic reset.
 */
export async function createTransferClaim(input) {
  // Start both snapshotting operations before either awaits caller-controlled input.
  const [config, proof] = await Promise.all([configuration(input), authenticateCertificate(input)]);
  if (config.issuer !== proof.issuer || config.trustRootSha256 !== proof.trustRootSha256) {
    throw new Error("Proof and custody use different trust bindings");
  }
  const original = new Uint8Array(proof.request), certificate = new Uint8Array(proof.bytes);
  return Object.freeze({ certificate: Object.freeze({ root: proof.root, bytes: new Uint8Array(certificate) }),
    async claim(plan, signal) {
      signal?.throwIfAborted();
      if (plan.owner !== config.issuer || plan.gateway !== config.gateway || plan.project !== config.project
          || plan.bucket !== config.bucket || plan.declaration?.root !== proof.root
          || plan.certificateSha256 !== proof.certificateSha256) throw new Error("Transfer plan differs from authenticated binding");
      const db = await open(false);
      try {
        signal?.throwIfAborted();
        await transaction(db, tx => {
          const binding = tx.objectStore("binding").get(0);
          binding.onsuccess = () => {
            if (JSON.stringify(binding.result) !== JSON.stringify(config)) { tx.abort(); return; }
            const claims = tx.objectStore("claims");
            const count = claims.count();
            count.onsuccess = () => {
              if (count.result >= CAPACITY) { tx.abort(); return; }
              claims.add({ root: proof.root, uploader: proof.uploader, requestId: proof.requestId,
                request: original, certificate, certificateSha256: proof.certificateSha256 }, proof.root);
            };
          };
        });
        // Cancellation cannot erase a claim committed while the caller was waiting.
        signal?.throwIfAborted();
        return true;
      } finally { db.close(); }
    } });
}
