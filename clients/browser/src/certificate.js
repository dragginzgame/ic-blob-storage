// Historical proof authentication only: no signing, fetching or provider admission.
import { inspectCallIngress } from "@tokostudios/ic-platform/agent";
import { Certificate, lookupResultToBuffer } from "@tokostudios/ic-platform/certification";
import { parseCanonicalPrincipal } from "@tokostudios/ic-platform/principal";
import { IDL } from "@tokostudios/ic-platform/candid";

function snapshot(value, limit) {
  if (!(value instanceof Uint8Array) || !(value.buffer instanceof ArrayBuffer)
      || value.length < 1 || value.length > limit) throw new TypeError("Invalid bounded proof bytes");
  return new Uint8Array(value);
}
const equal = (a, b) => a?.length === b.length && a.every((value, i) => value === b[i]);
export async function sha256(bytes) {
  return `sha256:${Array.from(new Uint8Array(await crypto.subtle.digest("SHA-256", bytes)),
    byte => byte.toString(16).padStart(2, "0")).join("")}`;
}

/** Authenticate the exact original certified reply. Expired ingress/certificates
 * remain historical evidence, never current permission or provider acceptance.
 * The application independently supplies all expected bindings and trust bytes.
 */
export async function authenticateCertificate(input) {
  const { issuer, uploader, root } = input;
  const owner = parseCanonicalPrincipal(issuer);
  const sender = parseCanonicalPrincipal(uploader);
  if (owner.isAnonymous() || sender.isAnonymous() || issuer === "aaaaa-aa"
      || typeof root !== "string" || !/^sha256:[0-9a-f]{64}$/.test(root)) {
    throw new TypeError("Invalid original proof binding");
  }
  // Snapshot every input before cryptographic awaits; no caller buffer is retained.
  const request = snapshot(input.request, 8192);
  const bytes = snapshot(input.certificate, 128 * 1024);
  const rootKey = snapshot(input.rootKey, 1024);
  const original = inspectCallIngress(request);
  if (original.sender.toText() !== uploader || original.canisterId.toText() !== issuer
      || original.methodName !== "_immutableObjectStorageCreateCertificate"
      || !equal(original.arg, IDL.encode([IDL.Text], [root]))) {
    throw new Error("Original request does not match proof bindings");
  }
  const certificate = await Certificate.create({ certificate: bytes, rootKey,
    principal: { canisterId: owner }, disableTimeVerification: true });
  const path = name => ["request_status", original.requestId, name];
  const status = lookupResultToBuffer(certificate.lookup_path(path("status")));
  if (!equal(status, new TextEncoder().encode("replied"))) throw new Error("Original request is not certified replied");
  const reply = lookupResultToBuffer(certificate.lookup_path(path("reply")));
  if (!reply || reply.length > 1024) throw new Error("Invalid bounded certified reply");
  const [value] = IDL.decode([IDL.Record({ method: IDL.Text, blob_hash: IDL.Text })], reply);
  if (value.method !== "upload" || value.blob_hash !== root) throw new Error("Certified reply has a different root or method");
  return Object.freeze({ root, issuer, uploader, bytes, request,
    requestId: Array.from(original.requestId, byte => byte.toString(16).padStart(2, "0")).join(""),
    trustRootSha256: await sha256(rootKey), certificateSha256: await sha256(bytes) });
}
