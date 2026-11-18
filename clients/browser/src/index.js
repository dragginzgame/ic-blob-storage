// Experimental Caffeine transfer boundary. No IC calls, storage engine or replay.
import { endpoint, providerBinding } from "./binding.js";

const CHUNK_BYTES = 1024 * 1024;
const MAX_BYTES = 32 * CHUNK_BYTES;
const encoder = new TextEncoder();

/** A stopped transfer. possibleExposure means a PUT was invoked, not accepted. */
export class TransferError extends Error {
  constructor(stage, chunkIndex, possibleExposure, acknowledged, cause) {
    super(`Transfer stopped at ${stage}; preserve custody and do not retry`, { cause });
    this.name = "TransferError";
    this.stage = stage;
    this.chunkIndex = chunkIndex;
    this.possibleExposure = possibleExposure;
    this.acknowledged = acknowledged;
  }
}

function copyBytes(value, maximum) {
  if (!(value instanceof Uint8Array) || !(value.buffer instanceof ArrayBuffer)
      || value.byteLength === 0 || value.byteLength > maximum) {
    throw new TypeError("Expected bounded, nonempty, non-shared Uint8Array bytes");
  }
  return new Uint8Array(value);
}

async function digest(prefix, ...parts) {
  const domain = encoder.encode(prefix);
  const bytes = new Uint8Array(domain.length + parts.reduce((n, part) => n + part.length, 0));
  bytes.set(domain);
  let offset = domain.length;
  for (const part of parts) {
    bytes.set(part, offset);
    offset += part.length;
  }
  return new Uint8Array(await crypto.subtle.digest("SHA-256", bytes));
}

function hashText(bytes) {
  return `sha256:${Array.from(bytes, byte => byte.toString(16).padStart(2, "0")).join("")}`;
}

async function parent(left, right) {
  const hash = await digest("ynode/", left.hash, right?.hash ?? encoder.encode("UNBALANCED"));
  return { hash, json: { hash: hashText(hash), left: left.json, right: right?.json ?? null } };
}

function leaf(hash) {
  return { hash, json: { hash: hashText(hash), left: null, right: null } };
}

// A stalled caller claim must not later resume network dispatch after cancellation.
async function abortable(work, signal) {
  let stop;
  const cancelled = signal.aborted ? Promise.reject(signal.reason) : new Promise((_, reject) => {
    stop = () => reject(signal.reason);
    signal.addEventListener("abort", stop, { once: true });
  });
  try {
    const value = await Promise.race([cancelled, work]);
    signal.throwIfAborted();
    return value;
  } finally { if (stop) signal.removeEventListener("abort", stop); }
}

async function consumeReply(response, signal) {
  if (response.redirected || response.status < 200 || response.status >= 300) {
    await abortable(Promise.resolve(response.body?.cancel()), signal);
    throw new Error("Gateway did not return an unredirected successful HTTP response");
  }
  const reader = response.body?.getReader();
  if (!reader) return;
  let received = 0;
  try {
    for (;;) {
      const { done, value } = await abortable(reader.read(), signal);
      if (done) return;
      received += value.byteLength;
      if (received > 4096) throw new Error("Gateway reply exceeds 4096 bytes");
    }
  } finally {
    try { await abortable(reader.cancel(), signal); }
    finally { reader.releaseLock(); }
  }
}

function admitType(contentType) {
  if (typeof contentType !== "string" || contentType.length > 256
      || !/^[A-Za-z0-9!#$&^_.+-]+\/[A-Za-z0-9!#$&^_.+-]+$/.test(contentType)) {
    throw new TypeError("Invalid explicit content type");
  }
  return contentType;
}

function deadline({ timeoutMs = 120_000, signal } = {}) {
  if (!Number.isSafeInteger(timeoutMs) || timeoutMs < 1 || timeoutMs > 120_000) {
    throw new TypeError("Invalid deadline");
  }
  return AbortSignal.any([AbortSignal.timeout(timeoutMs), ...(signal ? [signal] : [])]);
}

async function treeFor(bytes, headers) {
  const leaves = [];
  for (let offset = 0; offset < bytes.length; offset += CHUNK_BYTES) {
    leaves.push(leaf(await digest("icfs-chunk/", bytes.subarray(offset, offset + CHUNK_BYTES))));
  }
  let level = leaves;
  while (level.length > 1) {
    const next = [];
    for (let index = 0; index < level.length; index += 2) {
      next.push(await parent(level[index], level[index + 1]));
    }
    level = next;
  }
  const metadata = leaf(await digest("icfs-metadata/", encoder.encode(`${headers.join("\n")}\n`)));
  const tree = (await parent(level[0], metadata)).json;
  const chunks = Object.freeze(leaves.map(node => node.json.hash));
  return { tree_type: "DSBMTWH", chunk_hashes: chunks, tree, headers };
}

function readDeclaration(declaration) {
  const { root, bytes: length } = declaration;
  if (typeof root !== "string" || !/^sha256:[0-9a-f]{64}$/.test(root) || !Number.isSafeInteger(length)
      || length < 1 || length > MAX_BYTES || !Array.isArray(declaration.headers)
      || declaration.headers.length !== 2) {
    throw new TypeError("Invalid trusted declaration");
  }
  const headers = Object.freeze([...declaration.headers]);
  if (headers[0] !== `Content-Length: ${length}` || typeof headers[1] !== "string"
      || !headers[1].startsWith("Content-Type: ")) {
    throw new TypeError("Read must use original prepared metadata");
  }
  admitType(headers[1].slice("Content-Type: ".length));
  return Object.freeze({ root, bytes: length, headers });
}

/**
 * One explicitly requested GET, verified against a snapshotted original declaration.
 * No certificate, upload, retry, custody mutation or catalog decision. The caller
 * selects the original object URL and admits any read cost. One deadline spans
 * fetch and verification; custom fetch must honor the same transport rules as upload.
 */
export async function readVerified(url, declaration, options = {}) {
  const target = endpoint(url).href;
  const original = readDeclaration(declaration);
  const signal = deadline(options);
  const transport = options.fetch ?? globalThis.fetch.bind(globalThis);
  if (typeof transport !== "function") throw new TypeError("Invalid transport");
  signal.throwIfAborted();
  const response = await abortable(Promise.resolve(transport(target, {
    method: "GET", signal, redirect: "error", credentials: "omit", cache: "no-store",
  })), signal);
  return verifyRead(response, original, { signal });
}

/**
 * Verify a complete HTTP 200 read against the application's ORIGINAL declaration.
 * Response headers cannot redefine it. Returns an immutable Blob only after EOF,
 * exact length and root verification. This establishes integrity of this read,
 * not future availability or permission to publish an application catalog entry.
 */
export async function verifyRead(response, declaration, options = {}) {
  const { root, bytes: length, headers } = readDeclaration(declaration);
  const contentType = headers[1].slice("Content-Type: ".length);
  const signal = deadline(options);
  signal.throwIfAborted();
  if (response.status !== 200 || response.redirected || !response.body) {
    await abortable(Promise.resolve(response.body?.cancel()), signal);
    throw new Error("Read requires an unredirected complete HTTP 200 response");
  }
  const bytes = new Uint8Array(length);
  const reader = response.body.getReader();
  let offset = 0;
  try {
    for (;;) {
      const { done, value } = await abortable(reader.read(), signal);
      if (done) break;
      if (value.byteLength > length - offset) throw new Error("Read exceeds declared length");
      bytes.set(value, offset);
      offset += value.byteLength;
    }
    if (offset !== length) throw new Error("Read truncated before declared length");
  } finally {
    try { await abortable(reader.cancel(), signal); }
    finally { reader.releaseLock(); }
  }
  if ((await treeFor(bytes, headers)).tree.hash !== root) throw new Error("Read root mismatch");
  signal.throwIfAborted();
  return Object.freeze({ root, blob: new Blob([bytes], { type: contentType }) });
}

/**
 * Snapshot one file before the first await. Returns an immutable declaration and
 * a one-use upload function. Content-Length is derived; MIME is explicit (no sniffing).
 * Bounds are local pilot limits, not provider storage or billing guarantees.
 */
export async function prepare(source, { contentType = "application/octet-stream", maxBytes = MAX_BYTES } = {}) {
  if (!Number.isSafeInteger(maxBytes) || maxBytes < 1 || maxBytes > MAX_BYTES) {
    throw new TypeError("Invalid file bound");
  }
  admitType(contentType);
  const bytes = copyBytes(source, maxBytes);
  const headers = Object.freeze([`Content-Length: ${bytes.length}`, `Content-Type: ${contentType}`]);
  const blobTree = await treeFor(bytes, headers);
  const chunks = blobTree.chunk_hashes;
  const declaration = Object.freeze({ root: blobTree.tree.hash, bytes: bytes.length, chunks, headers });
  let used = false;

  /**
   * Send one claimed attempt. The caller authenticates and root-binds the supplied
   * certificate, and durably admits claim(plan, signal) before returning true.
   * No IC certificate acquisition, automatic retry, durable resume or global lock.
   * Custom fetch implementations must honor signal/redirect/credentials and not retry.
   */
  async function upload(options) {
    if (used) throw new Error("Prepared file already consumed; never retry it");
    const target = providerBinding(options);
    if (options.certificate?.root !== declaration.root || typeof options.claim !== "function") {
      throw new TypeError("Matching authenticated certificate and durable claim callback required");
    }
    const certificate = copyBytes(options.certificate.bytes, 128 * 1024);
    const transport = options.fetch ?? globalThis.fetch.bind(globalThis);
    const claim = options.claim;
    if (typeof transport !== "function") throw new TypeError("Invalid transport");
    const signal = deadline(options);
    // Consume synchronously before any hashing, custody callback or network await.
    used = true;
    let stage = "claim", chunkIndex = null, possibleExposure = false, acknowledged = 0;
    try {
      signal.throwIfAborted();
      const certificateSha256 = hashText(await digest("", certificate));
      const plan = Object.freeze({ ...target, declaration, certificateSha256 });
      signal.throwIfAborted();
      if (await abortable(Promise.resolve(claim(plan, signal)), signal) !== true) {
        throw new Error("Durable transfer claim was not admitted");
      }
      async function put(url, body, contentType) {
        signal.throwIfAborted();
        possibleExposure = true;
        const response = await abortable(Promise.resolve(transport(url, {
          method: "PUT", body, signal, redirect: "error", credentials: "omit", cache: "no-store",
          headers: { "Content-Type": contentType, "X-Caffeine-Project-ID": target.project },
        })), signal);
        await consumeReply(response, signal);
        acknowledged += 1;
      }
      stage = "tree";
      await put(`${target.gateway}/v1/blob-tree/`, JSON.stringify({
        blob_tree: blobTree, bucket_name: target.bucket, num_blob_bytes: bytes.length,
        owner: target.owner, project_id: target.project, headers,
        auth: { OwnerEgressSignature: Array.from(certificate) },
      }), "application/json");
      stage = "chunk";
      for (chunkIndex = 0; chunkIndex < chunks.length; chunkIndex += 1) {
        const query = new URLSearchParams({ owner_id: target.owner, blob_hash: declaration.root,
          chunk_hash: chunks[chunkIndex], chunk_index: String(chunkIndex),
          bucket_name: target.bucket, project_id: target.project });
        // A fresh body copy keeps injected transports from mutating the prepared file.
        const body = bytes.slice(chunkIndex * CHUNK_BYTES, (chunkIndex + 1) * CHUNK_BYTES);
        await put(`${target.gateway}/v1/chunk/?${query}`, body, "application/octet-stream");
      }
      return Object.freeze({ root: declaration.root, acknowledged, outcome: "http-acknowledged" });
    } catch (cause) {
      throw new TransferError(stage, chunkIndex, possibleExposure, acknowledged, cause);
    }
  }
  return Object.freeze({ declaration, upload });
}
