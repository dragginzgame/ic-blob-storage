// Called only by the installed Rust fixture. Never a live provider runner.
import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import { once } from "node:events";
import { readFile } from "node:fs/promises";
import { createServer } from "node:http";
import { join } from "node:path";
import { parseArgs, promisify } from "node:util";
import { fileURLToPath } from "node:url";
import { prepare, readVerified, TransferError } from "../src/index.js";

const { values } = parseArgs({ options: {
  directory: { type: "string" }, issuer: { type: "string" }, uploader: { type: "string" },
  url: { type: "string" }, replay: { type: "boolean", default: false },
  scenario: { type: "string" },
} });
const { directory, issuer, uploader, url } = values;
assert.ok(directory && issuer && uploader && url && process.env.BLOB_CLI);
assert.ok(["localhost", "127.0.0.1", "[::1]"].includes(new URL(url).hostname));
const content = values.scenario
  ? Uint8Array.from({ length: 5 * 1024 * 1024 + 17 }, (_, i) => (i * 31 + 7) % 251)
  : new TextEncoder().encode("abc");
const contentType = values.scenario ? "application/octet-stream" : "text/plain";
const requestCount = 1 + Math.ceil(content.length / (1024 * 1024));
const certificate = new Uint8Array(await readFile(join(directory, "certificate.cbor")));
const rootSubnet = (await readFile(join(directory, "root-subnet.txt"), "utf8")).trim();
const received = [];
let reads = 0;
const reconcile = values.scenario?.startsWith("reconcile") ?? false;
const cancellation = new AbortController();
// Test-only private pipe: Rust owns signed app calls and Wire encoding. No public
// HTTP control endpoint, JS app schema, or substitute catalog is introduced.
const replies = values.scenario && !values.replay ? process.stdin[Symbol.asyncIterator]() : null;
async function barrier(stage) {
  process.stdout.write(stage);
  const reply = await replies.next();
  assert.equal(reply.done, false);
  assert.equal(reply.value.length, 1);
  return reply.value.toString();
}
if (values.scenario) assert.ok(["success", "corrupt", "cancel-transfer", "cancel-read",
  "lost-final-reply", "interrupted-read", "crash-transfer",
  "reconcile", "reconcile-corrupt", "reconcile-cancelled"].includes(values.scenario));
const server = createServer(async (request, response) => {
  if (request.method === "GET") {
    reads += 1;
    assert.equal(request.url, "/asset");
    assert.equal(received.length, requestCount);
    response.writeHead(200, { "Content-Type": "ignored/response-metadata", "Content-Length": content.length });
    const body = Buffer.concat(received.slice(1).map(item => item.body));
    if (["corrupt", "reconcile-corrupt"].includes(values.scenario)) body[1024 * 1024 + 1] ^= 1;
    response.write(body.subarray(0, 1));
    if (values.scenario === "interrupted-read") {
      assert.equal(await barrier("D"), "D");
      response.destroy(); // Real transport failure after a prefix, not a synthetic verifier error.
      return;
    }
    if (["cancel-read", "reconcile-cancelled"].includes(values.scenario)) {
      assert.equal(await barrier("R"), "C");
      // Deliberately deliver a valid late remainder despite app cancellation.
    }
    response.end(body.subarray(1));
    return;
  }
  assert.equal(request.method, "PUT");
  const parts = [];
  for await (const part of request) parts.push(part);
  received.push({ url: request.url, body: Buffer.concat(parts) });
  if (values.scenario === "crash-transfer" && received.length === 2) {
    await barrier("K"); // Parent kills this process while the first chunk reply is pending.
    assert.fail("terminated transfer must never continue");
  }
  if ((values.scenario === "lost-final-reply" || reconcile) && received.length === requestCount) {
    response.writeHead(200, { "Content-Length": 16 });
    response.write("partial");
    assert.equal(await barrier("L"), "L");
    response.destroy(); // All bytes arrived, but the final HTTP response cannot complete.
    return;
  }
  if (values.scenario === "cancel-transfer" && received.length === 1) {
    assert.equal(await barrier("T"), "C");
    cancellation.abort(new Error("owning application cancelled"));
  }
  response.writeHead(204).end();
});
server.listen(0, "127.0.0.1");
await once(server, "listening");
const gateway = `http://127.0.0.1:${server.address().port}`;
const execute = promisify(execFile);

function options(file, proof, path) {
  return {
    gateway, owner: issuer, project: "local-project", bucket: "local-bucket",
    certificate: { root: file.declaration.root, bytes: proof },
    signal: cancellation.signal,
    claim: async (plan, signal) => {
      assert.equal(plan.owner, issuer);
      assert.equal(plan.gateway, gateway);
      assert.equal(plan.declaration, file.declaration);
      // The real CLI verifies proof/bindings and synchronizes create-new custody.
      // Process success is admitted before the client invokes its first PUT.
      await execute(process.env.BLOB_CLI, ["transfer", "claim", "--url", url,
        "--canister", issuer, "--uploader", uploader, "--root", plan.declaration.root,
        "--identity", join(directory, "uploader.pem"), "--root-key", join(directory, "root.der"),
        "--root-subnet", rootSubnet,
        "--custody", join(directory, "custody"), "--certificate", path,
        "--certificate-sha256", plan.certificateSha256, "--gateway", plan.gateway,
        "--project", plan.project, "--bucket", plan.bucket], { signal, timeout: 10_000, maxBuffer: 8192 });
      return true;
    },
  };
}

try {
  if (!values.replay) {
    const bad = await prepare(content, { contentType });
    const path = join(directory, "bad-certificate.cbor");
    await assert.rejects(bad.upload(options(bad, new Uint8Array(await readFile(path)), path)),
      error => error instanceof TransferError && !error.possibleExposure && error.stage === "claim");
    assert.equal(received.length, 0);
  }
  const file = await prepare(content, { contentType });
  const pending = file.upload(options(file, certificate, join(directory, "certificate.cbor")));
  if (values.replay) {
    await assert.rejects(pending, error => error instanceof TransferError && !error.possibleExposure);
    assert.equal(received.length, 0);
    console.log("replay-refused");
  } else if (values.scenario === "cancel-transfer") {
    await assert.rejects(pending, error => error instanceof TransferError && error.possibleExposure);
    assert.equal(received.length, 1); // No chunk follows the cancelled tree reply.
    assert.equal(await barrier("P"), "C"); // Even a stale publish command cannot revive it.
  } else if (values.scenario === "lost-final-reply" || reconcile) {
    await assert.rejects(pending, error => error instanceof TransferError && error.possibleExposure
      && error.stage === "chunk" && error.chunkIndex === requestCount - 2
      && error.acknowledged === requestCount - 1);
    assert.equal(received.length, requestCount);
    assert.equal(new Set(received.map(item => item.url)).size, requestCount);
    assert.equal(await barrier("F"), "F");
    if (reconcile) {
      assert.equal(await barrier("S"), "S"); // Owning user inspects the original saved draft first.
      const reading = execute(process.execPath, [fileURLToPath(new URL("./installed-read.js", import.meta.url)),
        "--url", `${gateway}/asset`, "--root", file.declaration.root,
        "--bytes", String(file.declaration.bytes), "--content-type", contentType],
      { timeout: 15_000, maxBuffer: 8192 });
      if (values.scenario === "reconcile-corrupt") {
        await assert.rejects(reading, /Read root mismatch/);
        assert.equal(await barrier("F"), "F");
      } else {
        assert.equal((await reading).stdout.trim(), "verified-read");
        assert.equal(await barrier("P"), values.scenario === "reconcile" ? "P" : "C");
      }
      assert.equal(reads, 1);
      assert.equal(received.length, requestCount); // Reconciliation sent only a GET.
    }
  } else {
    assert.equal((await pending).acknowledged, requestCount);
    const tree = JSON.parse(received[0].body);
    assert.equal(received[0].url, "/v1/blob-tree/");
    assert.equal(tree.owner, issuer);
    assert.equal(tree.blob_tree.tree.hash, file.declaration.root);
    assert.deepEqual(new Uint8Array(tree.auth.OwnerEgressSignature), certificate);
    assert.deepEqual(new Uint8Array(Buffer.concat(received.slice(1).map(item => item.body))), content);
    if (values.scenario) assert.equal(await barrier("A"), "A"); // HTTP acknowledgment has not published.
    const reading = readVerified(`${gateway}/asset`, file.declaration);
    if (values.scenario === "corrupt" || values.scenario === "interrupted-read") {
      if (values.scenario === "corrupt") await assert.rejects(reading, /root mismatch/);
      else await assert.rejects(reading);
      assert.equal(await barrier("F"), "F"); // No publication call on failed verification.
    } else {
      const verified = await reading;
      assert.deepEqual(new Uint8Array(await verified.blob.arrayBuffer()), content);
      if (values.scenario) assert.equal(await barrier("P"), values.scenario === "success" ? "P" : "C");
      else console.log("verified-read");
    }
  }
  if (values.scenario && !values.replay) assert.equal(await barrier("X"), "X");
} finally {
  server.closeAllConnections();
  server.close();
  if (values.scenario) process.stdin.destroy();
}
