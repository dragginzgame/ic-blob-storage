# Ownership cleanup after Blob 0.23.2

Date: 2026-10-13. Blob base is `7ad76317fb1ed2695c16ede4aabee655ded84187`.
The maintainer authorized local decoder consolidation and scoped upstream work
in IC Host and Shared Tooling. No release or Git publication was delegated.

## Applied Blob change

Native Candid limit construction has one implementation. Exact input decoding
still checks argument exhaustion explicitly; ordinary replies still use Candid's
existing decoder. Each caller retains its byte ceiling, error mapping, work,
skipping, type and optional header budget. There is no new decoder mode or DTO.
119 CLI cases and strict CLI Clippy pass on incoming registry Host 0.17.3.
The original sandboxed test attempt failed on loopback sockets; its log remains
in `.tmp/review-validation/cleanup0233/` beside successful reruns. Earlier tests
used Host 0.17.2; the final-graph logs identify the incoming selection separately.
No full Blob CI/release gate ran.

## Upstream owners and consumer qualification

IC Host now implements `durable::create_retained_at` and `publish_retained_at`.
They preserve caller-named partial evidence, hold directory/file identity, refuse
occupied destinations and reuse Host's existing no-replace publication engine.
Blob keeps content verification, named run custody and recovery policy. Three
new focused tests, strict fs Clippy and full Linux `make ci` pass in
`/tmp/blob-owner-cleanup/host-check`: released Host 0.17.3 plus this primitive
and complete formatter-fixture inputs. This excludes concurrent Host release
adapter and Wasm CLI work. Initial Clippy, symlinked-tool-directory and omitted
fixture-module failures are retained under `/tmp/blob-owner-cleanup/`.

Shared Tooling now owns `scripts/ci/check-node-tools.mjs` and its fixture/documentation.
It checks caller-selected Node/npm and root package declarations without setup,
network access or mutation. Importable Node-only admission supports browser builds.
The full portable suite and ShellCheck pass in `/tmp/blob-owner-cleanup/shared-check`,
a frozen Shared 0.10.2 source plus this checker and suite entry. It excludes other
concurrent Shared work. The actual Blob Node 24.21.0/npm 12.2.0 adapter check passes;
the initial sandbox subprocess readback failure is retained separately.

An isolated Blob consumer at `/tmp/blob-owner-cleanup/blob-consumer` uses the new
Host source through temporary Cargo patches and the new Shared checker. Its 119
CLI cases, Clippy and thin browser adapters pass. This is source qualification,
not registry publication or snapshot adoption. The real checkout has no path
patch, copied unpublished checker or live filesystem behavior change.

## Delivery boundary

The tested consumer patch is retained at
`.tmp/review-validation/cleanup0233/upstream-consumer.patch`, SHA-256
`306fae78271ece49230415ba8770d9bb231c62dafd7d0b5a41d4ee1324f37eb2`.
It removes Blob's manual staging/publication and browser prerequisite logic only
after the upstream owners are delivered. First select a published Host version
containing the API, then use the canonical Shared exporter with
`scripts/ci/check-node-tools.mjs` as an additional entrypoint. Apply the reviewed
consumer patch and rerun focused tests on those delivered identities. Do not
manually edit snapshot ownership or adopt a dirty sibling as a released package.
Native macOS, provider operations and downstream application acceptance remain
outside this local qualification. Prior retained failures are not overwritten.
