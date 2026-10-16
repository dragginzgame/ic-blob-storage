# Publication agents and capacity diagnostics

2026-10-16. Starting Blob HEAD: `7366c79d5ef0c6bf39983f80fb666a5589770356`,
with the preceding funding/completion cleanup already dirty. The maintainer
confirmed incoming Build 0.8.0/Memory 0.43.0 selections; Host 0.21.0, Metrics
0.11.3 and Testkit 0.47.0 remain selected. Own versions stay 0.28.0. Before-images,
commands, failed/final logs, artifact hashes and the source-only adoption patch
remain in `.tmp/review-validation/publication-planning0290/`.

Preparation authenticates tenant/uploader roles before effects and retains their
agents within each invocation. Every request still checks its actual signer;
claim ordering, exact journal/root binding, deadlines and no-redispatch behavior
remain. Separate invocations authenticate afresh. Capacity observers share only
the aggregate object/chunk/three-byte-dimension/sequential-slot comparisons;
per-object checks, distinct root counting, recovery and reference planning remain local.

Linux checks pass: 119 CLI, 110 Contracts and 23 fixture-operator cases; strict
affected all-target/all-feature lint; warning-denied Rustdoc; actual Rust 1.91
native/all-feature and Contracts-Wasm checks. The original three sandbox socket
refusals and obsolete lint-expectation failure are retained beside passing reruns.
Fresh production standalone Wasm and CLI pass 17 selected installed publication,
setup and recovery cases. The existing session case now changes tenant/uploader
credential and root files after startup and still completes with its original agents.
Build 0.8 resolves the earlier optional verifier type mismatch; this is compilation
evidence, not actual archive or full release qualification.

Isolated Testkit source `5a9733bf06338ac8e79ea8926b6bf49aa83c746c` adds bounded
TCP acceptance without workers, HTTP policy or new dependencies. Seven socket/HTTP
cases, strict lint, Rustdoc and Rust 1.91 pass. `adopt-accept.patch` removes both
Blob accept loops; its isolated consumer passes the provider response/refusal
matrix, strict CLI lint, Rust 1.91 and three installed lost/pending-reply observer
cases against fresh storage Wasm. [Testkit PR #108](https://github.com/dragginzgame/ic-testkit/pull/108)
delivers the exact producer source above. Merge/registry publication and consumer
activation remain tracked by [#70](https://github.com/dragginzgame/ic-blob-storage/issues/70).
No unpublished dependency override enters the real checkout.

All provider observations are local substitutes. Native macOS, full CI,
archive/release qualification and deployed-provider/Canic acceptance were not run.
No sibling working checkout, real Blob index/ref, version, release or paid effect changed.
