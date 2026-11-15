import assert from "node:assert/strict";
import { test } from "node:test";
import { prepare, verifyRead, readVerified } from "../src/index.js";

const text = value => new TextEncoder().encode(value);

test("multi-chunk read verifies the original root across arbitrary stream boundaries", async () => {
  const bytes = Uint8Array.from({ length: 5 * 1024 * 1024 + 17 }, (_, i) => (i * 31 + 7) % 251);
  const declaration = (await prepare(bytes)).declaration;
  assert.equal(declaration.root, "sha256:bc608ea2567f6809b872afc8e9ec44fc0addf3fd58a00e7f7aa7757b35927a03");
  const streamed = () => new Response(new ReadableStream({ start(controller) {
    for (let offset = 0; offset < bytes.length; offset += 65537) {
      controller.enqueue(bytes.slice(offset, offset + 65537));
    }
    controller.close();
  } }));
  const verified = await verifyRead(streamed(), declaration);
  assert.deepEqual(new Uint8Array(await verified.blob.arrayBuffer()), bytes);
  bytes[1024 * 1024 + 1] ^= 1;
  await assert.rejects(verifyRead(streamed(), declaration), /root mismatch/);
  await assert.rejects(verifyRead(new Response(bytes.subarray(0, bytes.length - 1)), declaration), /truncated/);
});

test("verified read returns immutable bytes using original metadata", async () => {
  const declaration = (await prepare(text("abc"), { contentType: "text/plain" })).declaration;
  const result = await verifyRead(new Response(text("abc"), {
    headers: { "Content-Type": "untrusted/metadata" },
  }), declaration);
  assert.equal(result.root, declaration.root);
  assert.equal(result.blob.type, "text/plain");
  assert.equal(await result.blob.text(), "abc");
  new Uint8Array(await result.blob.arrayBuffer()).fill(0);
  assert.equal(await result.blob.text(), "abc");
});

test("corruption, truncation, trailing bytes and partial HTTP reads refuse", async () => {
  const declaration = (await prepare(text("abc"))).declaration;
  for (const response of [new Response("abd"), new Response("ab"), new Response("abcd"),
    new Response("abc", { status: 206 }), new Response(null, { status: 204 })]) {
    await assert.rejects(verifyRead(response, declaration));
  }
  for (const change of [
    { root: `sha256:${"00".repeat(32)}` }, { bytes: 2 },
    { headers: ["Content-Length: 3", "Content-Type: text/plain"] },
    { bytes: 33 * 1024 * 1024 },
  ]) {
    await assert.rejects(verifyRead(new Response("abc"), { ...declaration, ...change }));
  }
});

test("bytes are not returned until EOF, and declaration mutation cannot change the binding", async () => {
  const prepared = await prepare(text("abc"));
  const declaration = { ...prepared.declaration, headers: [...prepared.declaration.headers] };
  let source;
  const response = new Response(new ReadableStream({ start(controller) { source = controller; } }));
  let complete = false;
  const read = verifyRead(response, declaration).then(value => { complete = true; return value; });
  declaration.root = `sha256:${"00".repeat(32)}`;
  declaration.headers[1] = "Content-Type: text/plain";
  source.enqueue(text("abc"));
  await new Promise(resolve => setImmediate(resolve));
  assert.equal(complete, false);
  source.close();
  assert.equal((await read).root, prepared.declaration.root);
});

test("read cancellation and interrupted streams never expose a verified file", async () => {
  const declaration = (await prepare(text("abc"))).declaration;
  let cancelled = false;
  const controller = new AbortController();
  const response = new Response(new ReadableStream({ cancel() { cancelled = true; } }));
  const reading = verifyRead(response, declaration, { signal: controller.signal });
  controller.abort();
  await assert.rejects(reading);
  assert.equal(cancelled, true);
  let source;
  const interrupted = new Response(new ReadableStream({ start(value) { source = value; } }));
  const failed = verifyRead(interrupted, declaration);
  source.enqueue(text("abc"));
  source.error(new Error("transport failed after prefix"));
  await assert.rejects(failed);
});

test("read-only reconciliation snapshots the original declaration before its single GET", async () => {
  const original = (await prepare(text("abc"))).declaration;
  const declaration = { ...original, headers: [...original.headers] };
  const url = new URL("https://storage.example/original?owner=selected");
  let reply, calls = 0;
  const pending = readVerified(url, declaration, { fetch: (target, options) => {
    calls += 1;
    assert.equal(target, "https://storage.example/original?owner=selected");
    assert.equal(options.method, "GET");
    assert.equal(options.redirect, "error");
    assert.equal(options.credentials, "omit");
    assert.equal(options.cache, "no-store");
    return new Promise(resolve => { reply = resolve; });
  } });
  declaration.root = `sha256:${"00".repeat(32)}`;
  declaration.headers[1] = "Content-Type: text/plain";
  url.pathname = "/changed";
  reply(new Response("abc"));
  const verified = await pending;
  assert.equal(verified.root, original.root);
  assert.equal(await verified.blob.text(), "abc");
  assert.equal(calls, 1);
});

test("invalid read target or declaration refuses before network access", async () => {
  const declaration = (await prepare(text("abc"))).declaration;
  const fetch = () => assert.fail("invalid read must not dispatch");
  for (const url of ["http://storage.example/asset", "https://user@storage.example/asset",
    "https://storage.example/asset#fragment", "file:///asset", `https://storage.example/${"x".repeat(4096)}`]) {
    await assert.rejects(readVerified(url, declaration, { fetch }), TypeError);
  }
  await assert.rejects(readVerified("https://storage.example/asset", { ...declaration, bytes: 0 }, { fetch }), TypeError);
  await assert.rejects(readVerified("https://storage.example/asset", {
    ...declaration, root: { toString: () => declaration.root },
  }, { fetch }), TypeError);
});

test("read-only reconciliation stops on failed or stalled fetch without retry", async () => {
  const declaration = (await prepare(text("abc"))).declaration;
  let calls = 0;
  await assert.rejects(readVerified("https://storage.example/asset", declaration, {
    fetch: async () => { calls += 1; throw new Error("connection lost"); },
  }), /connection lost/);
  assert.equal(calls, 1);
  const stop = new AbortController();
  let signal;
  const pending = readVerified("https://storage.example/asset", declaration, {
    signal: stop.signal, fetch: (_url, options) => {
      calls += 1; signal = options.signal; return new Promise(() => {});
    },
  });
  stop.abort();
  await assert.rejects(pending);
  assert.equal(signal.aborted, true);
  assert.equal(calls, 2);
});
