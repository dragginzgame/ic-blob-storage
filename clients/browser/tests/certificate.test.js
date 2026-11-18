import assert from "node:assert/strict";
import { test } from "node:test";
import { readFile } from "node:fs/promises";
import { authenticateCertificate } from "../src/certificate.js";
import { initializeCustody } from "../src/custody.js";
import { callWithMethodFixture } from "@tokostudios/ic-platform/testing";

const fixture = JSON.parse(await readFile(new URL("./fixtures/certificate.json", import.meta.url)));
function input() {
  return { ...fixture, ...Object.fromEntries(["request", "certificate", "rootKey", "badCertificate"]
    .map(key => [key, new Uint8Array(Buffer.from(fixture[key], "base64"))])) };
}

test("custody refuses non-origin gateways before opening IndexedDB", async () => {
  const config = { ...input(), project: "app", bucket: "assets" };
  for (const gateway of ["https://storage.example?", "https://storage.example#"]) {
    await assert.rejects(initializeCustody({ ...config, gateway }), TypeError);
  }
});

test("authenticate historical installed proof against original trusted bindings", async () => {
  const source = input();
  const pending = authenticateCertificate(source);
  source.certificate.fill(0);
  source.rootKey.fill(0);
  source.request.fill(0);
  source.root = `sha256:${"00".repeat(32)}`;
  const proof = await pending;
  assert.equal(proof.root, fixture.root);
  assert.equal(proof.requestId, fixture.requestId);
  assert.deepEqual(proof.bytes, input().certificate);
});

test("wrong signature, trust, request and identity refuse without network access", async () => {
  const source = input();
  const wrongRoot = new Uint8Array(source.rootKey);
  wrongRoot[wrongRoot.length - 1] ^= 1;
  for (const change of [
    { certificate: source.badCertificate }, { rootKey: wrongRoot },
    { request: callWithMethodFixture(source.request, "different_method") },
    { request: source.request.slice(0, 32) }, { root: `sha256:${"00".repeat(32)}` },
    { uploader: source.issuer }, { issuer: source.uploader }, { uploader: "2vxsx-fae" },
    { certificate: new Uint8Array(128 * 1024 + 1) }, { request: new Uint8Array(8193) },
    { rootKey: new Uint8Array(1025) },
  ]) await assert.rejects(authenticateCertificate({ ...source, ...change }), Object.keys(change).join(","));
});
