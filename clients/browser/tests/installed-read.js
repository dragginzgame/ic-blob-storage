// Fresh read-only client for the installed fixture. No signer, certificate, body
// source, upload handle or custody path is supplied to this process.
import assert from "node:assert/strict";
import { parseArgs } from "node:util";
import { readVerified } from "../src/index.js";

const { values } = parseArgs({ options: {
  url: { type: "string" }, root: { type: "string" }, bytes: { type: "string" },
  "content-type": { type: "string" },
} });
assert.equal(new URL(values.url).hostname, "127.0.0.1");
const declaration = {
  root: values.root, bytes: Number(values.bytes),
  headers: [`Content-Length: ${values.bytes}`, `Content-Type: ${values["content-type"]}`],
};
const result = await readVerified(values.url, declaration, { timeoutMs: 10_000 });
assert.equal(result.root, declaration.root);
assert.equal(result.blob.size, declaration.bytes);
console.log("verified-read");
