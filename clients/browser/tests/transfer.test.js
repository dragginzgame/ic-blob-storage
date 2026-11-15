import assert from "node:assert/strict";
import { createServer } from "node:http";
import { once } from "node:events";
import { test } from "node:test";
import { prepare, TransferError } from "../src/index.js";

const text = value => new TextEncoder().encode(value);
const pattern = length => Uint8Array.from({ length }, (_, index) => (index * 31 + 7) % 251);
const defaults = prepared => ({
  gateway: "https://storage.example", owner: "owner", project: "project", bucket: "bucket",
  // Synthetic bytes are deliberately not a provider proof. This suite qualifies
  // transport behavior; the caller owns real certificate authentication.
  certificate: { root: prepared.declaration.root, bytes: Uint8Array.of(1, 2, 3) },
  claim: async () => true,
});

test("snapshot and manifest match existing independently retained Rust/SDK vectors", async () => {
  const source = text("abc");
  const pending = prepare(source, { contentType: "text/plain" });
  source.fill(0);
  const file = await pending;
  assert.equal(file.declaration.root, "sha256:0e9afaf413b048e40834d5b0e737d80fbf304af2045c7564d96ad8aebaf74dfd");
  assert.deepEqual(file.declaration.headers, ["Content-Length: 3", "Content-Type: text/plain"]);
  assert.throws(() => file.declaration.chunks.push("changed"), TypeError);
  assert.throws(() => { file.declaration.bytes = 0; }, TypeError);
  const large = await prepare(pattern(5 * 1024 * 1024 + 17));
  assert.equal(large.declaration.root, "sha256:bc608ea2567f6809b872afc8e9ec44fc0addf3fd58a00e7f7aa7757b35927a03");
  assert.equal(large.declaration.chunks.length, 6);
  const corrupted = pattern(5 * 1024 * 1024 + 17);
  corrupted[1024 * 1024 + 1] ^= 1;
  assert.notEqual((await prepare(corrupted)).declaration.root, large.declaration.root);
  assert.notEqual((await prepare(text("abc"))).declaration.root, file.declaration.root);
});

test("invalid or unbounded preparation refuses before network/custody", async () => {
  for (const bytes of [new Uint8Array(), new Uint8Array(new SharedArrayBuffer(3)), "abc"]) {
    await assert.rejects(prepare(bytes), TypeError);
  }
  for (const options of [{ maxBytes: 2 }, { maxBytes: Infinity }, { maxBytes: 33 * 1024 * 1024 },
    { contentType: "text/plain\r\nX-Evil: 1" }, { contentType: "" }]) {
    await assert.rejects(prepare(text("abc"), options), TypeError);
  }
});

test("claim precedes exposure, copies proof, and bars concurrent use", async () => {
  const file = await prepare(text("abc"));
  const options = defaults(file);
  const calls = [];
  let admit;
  const gate = new Promise(resolve => { admit = resolve; });
  let claimed;
  const ready = new Promise(resolve => { claimed = resolve; });
  options.claim = async plan => {
    assert.equal(plan.declaration, file.declaration);
    assert.equal(plan.gateway, options.gateway);
    assert.equal(plan.certificateSha256, "sha256:039058c6f2c0cb492c533b0a4d14ef77cc0f78abccced5287d84a1a2011cfb81");
    assert.throws(() => { plan.owner = "changed"; }, TypeError);
    claimed();
    return gate;
  };
  options.fetch = async (url, request) => {
    calls.push([url, request]);
    assert.equal(request.redirect, "error");
    assert.equal(request.credentials, "omit");
    assert.equal(request.method, "PUT");
    return new Response(null, { status: 204 });
  };
  const sending = file.upload(options);
  options.certificate.bytes.fill(9);
  await ready;
  assert.equal(calls.length, 0);
  await assert.rejects(file.upload(options));
  admit(true);
  const result = await sending;
  assert.equal(result.outcome, "http-acknowledged");
  assert.equal(result.acknowledged, 2);
  const tree = JSON.parse(calls[0][1].body);
  assert.deepEqual(tree.auth.OwnerEgressSignature, [1, 2, 3]);
  assert.equal(tree.num_blob_bytes, 3);
  assert.equal(tree.blob_tree.tree.hash, file.declaration.root);
  assert.deepEqual(tree.headers, file.declaration.headers);
  assert.deepEqual(calls[1][1].body, text("abc"));
  await assert.rejects(file.upload(options));
  assert.equal(calls.length, 2);
});

test("invalid bindings never reach the durable claim", async () => {
  const file = await prepare(text("abc"));
  for (const change of [
    { gateway: "http://storage.example" }, { gateway: "https://user:secret@storage.example" },
    { gateway: "https://storage.example/path" }, { gateway: "https://storage.example?x=1" },
    { project: "project\r\nInjected: value" }, { timeoutMs: 0 },
    { certificate: { root: "different", bytes: Uint8Array.of(1) } },
    { certificate: { root: file.declaration.root, bytes: new Uint8Array(128 * 1024 + 1) } },
  ]) {
    await assert.rejects(file.upload({ ...defaults(file), ...change,
      claim: () => assert.fail("invalid admission reached custody"),
      fetch: () => assert.fail("invalid admission reached network") }), TypeError);
  }
});

for (const failure of ["claim", "tree", "chunk", "large-reply", "abort"]) {
  test(`${failure} stops without automatic replay`, async () => {
    const file = await prepare(pattern(1024 * 1024 + 1));
    const controller = new AbortController();
    let calls = 0;
    const options = { ...defaults(file), signal: controller.signal,
      claim: async () => failure !== "claim",
      fetch: async () => {
        calls += 1;
        if (failure === "tree") return new Response(null, { status: 503 });
        if (failure === "chunk" && calls === 2) throw new Error("reply lost");
        if (failure === "large-reply") return new Response(new Uint8Array(4097));
        if (failure === "abort") controller.abort();
        return new Response(null, { status: 204 });
      } };
    await assert.rejects(file.upload(options), error => {
      assert.ok(error instanceof TransferError);
      assert.equal(error.possibleExposure, failure !== "claim");
      assert.equal(error.acknowledged, failure === "chunk" ? 1 : 0);
      if (failure === "chunk") assert.equal(error.chunkIndex, 0);
      return true;
    });
    const count = calls;
    await assert.rejects(file.upload(options));
    assert.equal(calls, count);
    assert.equal(calls, failure === "claim" ? 0 : failure === "chunk" ? 2 : 1);
  });
}

test("cancellation during a stalled claim cannot later dispatch", async () => {
  const file = await prepare(text("abc"));
  const controller = new AbortController();
  let admit, started;
  const ready = new Promise(resolve => { started = resolve; });
  const gate = new Promise(resolve => { admit = resolve; });
  const sending = file.upload({ ...defaults(file), signal: controller.signal,
    claim: () => { started(); return gate; }, fetch: () => assert.fail("late dispatch") });
  await ready;
  controller.abort();
  await assert.rejects(sending, error => error instanceof TransferError && !error.possibleExposure);
  admit(true);
  await new Promise(resolve => setImmediate(resolve));
  await assert.rejects(file.upload(defaults(file)));
});

test("a pending chunk prevents the next chunk from starting", async () => {
  const file = await prepare(pattern(1024 * 1024 + 1));
  let release, entered;
  const gate = new Promise(resolve => { release = resolve; });
  const ready = new Promise(resolve => { entered = resolve; });
  let calls = 0;
  const pending = file.upload({ ...defaults(file), fetch: async () => {
    calls += 1;
    if (calls === 2) { entered(); await gate; }
    return new Response(null, { status: 204 });
  } });
  await ready;
  await new Promise(resolve => setImmediate(resolve));
  assert.equal(calls, 2);
  release();
  assert.equal((await pending).acknowledged, 3);
  assert.equal(calls, 3);
});

test("deadline stops an unresponsive transport without another PUT", async t => {
  // AbortSignal.timeout does not itself keep Node alive; browsers retain their loop.
  const keepAlive = setTimeout(() => {}, 5000);
  t.after(() => clearTimeout(keepAlive));
  const file = await prepare(text("abc"));
  let calls = 0;
  const options = { ...defaults(file), timeoutMs: 1000, fetch: () => {
    calls += 1;
    return new Promise(() => {});
  } };
  await assert.rejects(file.upload(options), error => error instanceof TransferError
    && error.possibleExposure && error.stage === "tree");
  await assert.rejects(file.upload(options));
  assert.equal(calls, 1);
});

test("real loopback fetch sends one tree then serial immutable chunks", async t => {
  const received = [];
  const server = createServer(async (request, response) => {
    const parts = [];
    for await (const part of request) parts.push(part);
    received.push({ url: request.url, method: request.method, headers: request.headers, body: Buffer.concat(parts) });
    response.writeHead(204).end();
  });
  t.after(() => { server.closeAllConnections(); server.close(); });
  server.listen(0, "127.0.0.1");
  await once(server, "listening");
  const source = pattern(2 * 1024 * 1024 + 17);
  const file = await prepare(source);
  await file.upload({ ...defaults(file), gateway: `http://127.0.0.1:${server.address().port}` });
  assert.equal(received.length, 4);
  const tree = JSON.parse(received[0].body);
  assert.equal(received[0].url, "/v1/blob-tree/");
  assert.equal(tree.blob_tree.tree.hash, file.declaration.root);
  assert.equal(tree.blob_tree.tree_type, "DSBMTWH");
  assert.deepEqual(tree.blob_tree.chunk_hashes, file.declaration.chunks);
  for (let index = 0; index < 3; index += 1) {
    const request = received[index + 1];
    const url = new URL(request.url, "http://localhost");
    assert.equal(url.pathname, "/v1/chunk/");
    assert.equal(url.searchParams.get("chunk_index"), String(index));
    assert.equal(url.searchParams.get("chunk_hash"), file.declaration.chunks[index]);
    assert.equal(url.searchParams.get("blob_hash"), file.declaration.root);
    assert.equal(request.method, "PUT");
    assert.equal(request.headers["x-caffeine-project-id"], "project");
    assert.deepEqual(new Uint8Array(request.body), source.subarray(index * 1024 * 1024, (index + 1) * 1024 * 1024));
  }
});

test("real fetch refuses redirects without reaching the redirected destination", async t => {
  const requests = [];
  const server = createServer((request, response) => {
    requests.push(request.url);
    request.resume();
    response.writeHead(307, { Location: "/redirected-proof" }).end();
  });
  t.after(() => { server.closeAllConnections(); server.close(); });
  server.listen(0, "127.0.0.1");
  await once(server, "listening");
  const file = await prepare(text("abc"));
  await assert.rejects(file.upload({ ...defaults(file), gateway: `http://127.0.0.1:${server.address().port}` }),
    error => error instanceof TransferError && error.possibleExposure && error.stage === "tree");
  assert.deepEqual(requests, ["/v1/blob-tree/"]);
});
