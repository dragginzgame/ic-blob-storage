// Real browser proof/custody/transport against local IC evidence and HTTP peers.
import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { once } from "node:events";
import { createServer } from "node:http";
import { mkdir, mkdtemp, readFile, rm, open } from "node:fs/promises";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { build } from "esbuild";

const browser = process.env.BROWSER;
assert.ok(browser?.startsWith("/"), "Set BROWSER to an absolute Chromium executable path");
const length = 5 * 1024 * 1024 + 17;
const bytes = Buffer.from(Uint8Array.from({ length }, (_, i) => (i * 31 + 7) % 251));
const fixture = JSON.parse(await readFile(new URL("./fixtures/certificate.json", import.meta.url)));
const bundle = await build({ entryPoints: [fileURLToPath(new URL("../src/custody.js", import.meta.url))],
  bundle: true, format: "esm", platform: "browser", write: false });

async function qualify(scenario) {
  const counts = new Map();
  const uploaded = [];
  let tree, puts = 0, activePuts = 0, preflights = 0, peerFailure;
  let notifyChunk;
  const chunkPending = new Promise(resolve => { notifyChunk = resolve; });
  let pageOrigin;
  async function acceptUpload(request, response) {
    activePuts += 1;
    try {
      assert.equal(activePuts, 1, "PUTs must remain serial until the previous reply completes");
      assert.equal(request.headers.origin, pageOrigin);
      assert.equal(request.headers["x-caffeine-project-id"], "fixture-project");
      const parts = [];
      let size = 0;
      for await (const part of request) {
        size += part.length;
        assert.ok(size <= 1024 * 1024);
        parts.push(part);
      }
      const body = Buffer.concat(parts);
      const url = new URL(request.url, "http://localhost");
      if (url.pathname === "/v1/blob-tree/") {
        assert.equal(puts, 0);
        assert.equal(request.headers["content-type"], "application/json");
        tree = JSON.parse(body);
        assert.equal(tree.blob_tree.tree.hash, "sha256:bc608ea2567f6809b872afc8e9ec44fc0addf3fd58a00e7f7aa7757b35927a03");
        assert.equal(tree.num_blob_bytes, length);
        assert.equal(tree.owner, fixture.issuer);
        assert.equal(tree.project_id, "fixture-project");
        assert.equal(tree.bucket_name, "fixture-bucket");
        assert.deepEqual(tree.auth, { OwnerEgressSignature: [...Buffer.from(fixture.certificate, "base64")] });
      } else {
        assert.equal(url.pathname, "/v1/chunk/");
        assert.equal(request.headers["content-type"], "application/octet-stream");
        assert.ok(tree);
        const index = uploaded.length;
        assert.equal(url.searchParams.get("chunk_index"), String(index));
        assert.equal(url.searchParams.get("blob_hash"), tree.blob_tree.tree.hash);
        assert.equal(url.searchParams.get("chunk_hash"), tree.blob_tree.chunk_hashes[index]);
        assert.equal(url.searchParams.get("owner_id"), fixture.issuer);
        assert.equal(url.searchParams.get("project_id"), "fixture-project");
        assert.equal(url.searchParams.get("bucket_name"), "fixture-bucket");
        assert.deepEqual(body, bytes.subarray(index * 1024 * 1024, (index + 1) * 1024 * 1024));
        uploaded.push(body);
        if (scenario === "cancel-chunk" && index === 0) {
          puts += 1;
          const closed = once(response, "close");
          notifyChunk(); // The browser cancels only once real bytes have arrived.
          await closed; // No reply: the client's cancellation must close the request.
          return;
        }
        if (index === 0) await new Promise(resolve => setTimeout(resolve, 100));
      }
      puts += 1;
      if (scenario === "lost-final-reply" && puts === 7) {
        // All bytes arrived. Deliver a prefix, then break the actual HTTP response.
        response.writeHead(200, { "Content-Length": 16 });
        response.write("partial", () => response.destroy());
      } else {
        response.writeHead(204).end();
      }
    } finally { activePuts -= 1; }
  }
  const peer = createServer((request, response) => {
    if (request.headers.cookie || request.headers.authorization) {
      peerFailure = new Error("ambient credentials reached the peer");
      response.writeHead(400).end();
      return;
    }
    if (request.url !== "/no-cors") response.setHeader("Access-Control-Allow-Origin", pageOrigin);
    if (request.method === "OPTIONS") {
      preflights += 1;
      response.writeHead(204, { "Access-Control-Allow-Methods": "PUT",
        "Access-Control-Allow-Headers": "content-type,x-caffeine-project-id" }).end();
      return;
    }
    if (request.method === "PUT") {
      void acceptUpload(request, response).catch(error => {
        peerFailure = error;
        response.destroy();
      });
      return;
    }
    if (request.method !== "GET") { response.writeHead(405).end(); return; }
    counts.set(request.url, (counts.get(request.url) ?? 0) + 1);
    response.setHeader("Content-Type", "untrusted/metadata");
    response.setHeader("Cache-Control", "no-store");
    if (request.url === "/redirect") {
      response.writeHead(302, { Location: "/redirect-target" }).end();
    } else if (request.url === "/stall") {
      response.writeHead(200);
      response.write(bytes.subarray(0, 1)); // Leave EOF pending until the client cancels.
    } else if (request.url === "/corrupt") {
      const corrupt = Buffer.from(bytes);
      corrupt[1024 * 1024 + 1] ^= 1;
      response.end(corrupt);
    } else if (request.url === "/short") {
      response.end(bytes.subarray(0, bytes.length - 1));
    } else if (request.url === "/extra") {
      response.end(Buffer.concat([bytes, Buffer.from([0])]));
    } else if (request.url === "/partial") {
      response.writeHead(206).end(bytes);
    } else if (request.url === "/uploaded") {
      response.end(Buffer.concat(uploaded));
    } else if (["/ok", "/no-cors"].includes(request.url)) {
      response.end(bytes);
    } else {
      response.writeHead(404).end();
    }
  });
  const sources = new Map([
    ["/", ["text/html", Buffer.from('<!doctype html><title>Blob browser transport qualification</title><script type="module" src="/scenario.js"></script>')]],
    ["/scenario.js", ["text/javascript", await readFile(new URL("./runtime-page.js", import.meta.url))]],
    ["/client.js", ["text/javascript", await readFile(new URL("../src/index.js", import.meta.url))]],
    ["/custody.js", ["text/javascript", bundle.outputFiles[0].contents]],
    ["/proof.json", ["application/json", JSON.stringify(fixture)]],
  ]);
  let report;
  let first;
  const received = new Promise(resolve => { report = resolve; });
  const page = createServer(async (request, response) => {
    if (request.url === "/chunk-pending" && request.method === "GET" && scenario === "cancel-chunk") {
      await chunkPending;
      response.end();
      return;
    }
    if (request.url === "/result" && request.method === "POST") {
      try {
        const chunks = [];
        let size = 0;
        for await (const chunk of request) {
          size += chunk.length;
          if (size > 8192) throw new Error("oversized browser result");
          chunks.push(chunk);
        }
        const value = JSON.parse(Buffer.concat(chunks).toString());
        if (value.stage === "first") first = value;
        else report(value);
        response.end();
      } catch (error) {
        report({ error: String(error) });
        response.writeHead(400).end();
      }
      return;
    }
    const source = sources.get(new URL(request.url, "http://localhost").pathname);
    if (!source || request.method !== "GET") { response.writeHead(404).end(); return; }
    response.writeHead(200, { "Content-Type": source[0], "Cache-Control": "no-store" }).end(source[1]);
  });

  await mkdir(resolve("target"), { recursive: true });
  const scratch = await mkdtemp(resolve("target/browser-runtime-"));
  const log = await open(`${scratch}/chromium.log`, "w");
  let child, closed, timer;
  try {
    peer.listen(0, "127.0.0.1");
    await once(peer, "listening");
    page.listen(0, "127.0.0.1");
    await once(page, "listening");
    pageOrigin = `http://127.0.0.1:${page.address().port}`;
    const peerOrigin = `http://127.0.0.1:${peer.address().port}`;
    child = spawn(browser, ["--headless", "--disable-gpu", "--no-first-run",
      "--no-default-browser-check", "--disable-background-networking", "--disable-sync",
      `--user-data-dir=${scratch}/profile`, `${pageOrigin}/?peer=${encodeURIComponent(peerOrigin)}&scenario=${scenario}`],
      { stdio: ["ignore", log.fd, log.fd] });
    closed = new Promise(resolve => child.once("close", resolve));
    const result = await Promise.race([
      received,
      new Promise((_, reject) => {
        child.once("error", reject);
        child.once("exit", code => reject(new Error(`Browser exited before result: ${code}`)));
        timer = setTimeout(() => reject(new Error("Browser qualification timed out")), 30_000);
      }),
    ]);
    assert.equal(peerFailure, undefined, peerFailure?.stack);
    assert.equal(result.error, undefined, result.error);
    assert.equal(result.stage, "replay");
    assert.equal(result.refused, true);
    assert.equal(result.root, first.root);
    assert.equal(result.verified, scenario !== "cancel-chunk");
    assert.equal(first.outcome, scenario === "success" ? "http-acknowledged" : "uncertain");
    assert.equal(first.root, "sha256:bc608ea2567f6809b872afc8e9ec44fc0addf3fd58a00e7f7aa7757b35927a03");
    assert.deepEqual(first.refused, ["corrupt", "short", "extra", "partial", "redirect", "no-cors", "stall"]);
    assert.equal(first.cancelled, true);
    assert.deepEqual(Object.fromEntries(counts), Object.fromEntries(
      ["ok", ...first.refused, "uploaded"].map(name => [`/${name}`, 1])));
    assert.equal(puts, scenario === "cancel-chunk" ? 2 : 7);
    assert.equal(uploaded.length, scenario === "cancel-chunk" ? 1 : 6);
    assert.equal(activePuts, 0);
    assert.ok(preflights > 0, "actual cross-origin PUT preflight");
    console.log(`Chromium ${scenario} passed: ${puts} PUTs, reload refuses replay, read ${result.verified ? "verified" : "refused incomplete bytes"}.`);
  } finally {
    clearTimeout(timer);
    if (child) {
      child.kill("SIGTERM");
      const kill = setTimeout(() => child.kill("SIGKILL"), 3000);
      await closed;
      clearTimeout(kill);
    }
    for (const server of [page, peer]) { server.closeAllConnections(); server.close(); }
    await log.close();
    console.log(`Browser diagnostics: ${scratch}/chromium.log`);
    // Chromium helpers may finish profile writes just after the parent closes.
    // Bound cleanup retries; an exhausted failure still fails the qualification.
    await rm(`${scratch}/profile`, { recursive: true, force: true, maxRetries: 10, retryDelay: 100 });
  }
}

for (const scenario of ["success", "lost-final-reply", "cancel-chunk"]) await qualify(scenario);
