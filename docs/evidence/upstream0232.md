# Shared 0.10.2 and Host 0.17.2 refresh

Date: 2026-10-13. Pending compatible Blob 0.23.2, over released
`dc169f9bfe6c1e736c189e1ce5ee478cfe81b745` and the already prepared batch.
Earlier [Shared adoption](shared0100232.md) and [Metrics qualification](metrics0900232.md)
retain their original revisions and dependency graphs.

Shared upstream HEAD is `626fc2bd3125195520fb870b92abfec11b01454d` (0.10.2).
A clean committed clone provided the canonical exporter; the sibling's unrelated
dirty file was not copied. Refresh retained all 112 owned payloads and existing
entrypoints. The applicable change normalizes literal Cargo tool cache selectors,
ignoring unrelated pins/comments/order while preserving build and Cargo identities.
Opaque catalogs remain byte-bound and are never executed by key inspection.
Blob still has no hosted Cargo-cache policy of its own. The sibling-dashboard
newline fix is outside Blob's selected payloads; no fleet dashboard was added.
Common catalogs and tool versions/digests remain the snapshot-owned selections,
with no host, IC or Rust exceptions. No installer behavior change requires setup.

Incoming Cargo.lock already selected registry Host artifacts/fs/process/tools
0.17.2. The update preserved it byte-for-byte:
`4cefe4d2bde26b958902d43592f89c93e1cfcee5a5674844eb2751d3e4173040`.
The remote Host tag peels to `e3afa6ddf022309e7ba97f6cefb312c48738db4e`.
Its patch simplifies upstream test fixtures and retains example failure evidence;
no Blob-used API or feature change requires a local shim. Memory remains 0.36.2,
Metrics 0.9.0 and Testkit 0.37.0. Locked fetch passed.

Focused local results are retained in `.tmp/review-validation/upstream0232/`:
119 CLI unit/integration cases and workspace all-target/all-feature Clippy pass.
The canonical Rust-tool fixture, offline tool admission, snapshot integrity,
dependency pins, formatting and shell checks pass. The first two installation
checks refused before runtime because the standalone Wasm was absent; that log
and `/tmp/nonempty-cargo-test.PaZQc6` are retained. Fresh standalone Wasm and CLI
builds pass, followed by both installation tests: canonical Candid/callable-export
agreement and actual install with wrong-service rejection/rollback. Documentation
links and final diff checks pass.

No full CI/release gate, hosted/macOS rerun, sibling edit, package version change,
commit, push, publication or provider operation was performed. Blob #66 still
tracks delivery/native acceptance; the earlier Intel timeout is not a green gate.
