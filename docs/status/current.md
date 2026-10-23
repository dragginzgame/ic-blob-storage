# Current status

Date: 2026-10-23

## Released contract and pending adoption

Local HEAD is released **0.43.1**, `f9f8c32`. The tenant methods now use native
Wire with identities `0043/001a` and `0043/001b`, zero-based failure tags,
borrowed 29-byte principals and a 512-byte frame ceiling. Other method identities
stay `0042`. Retired finite Method markers are removed without compatibility
readers. Persisted representations were not changed by the Wire migration.
Cross-release installations still follow retirement/reinstall policy.

Pending **0.43.2** adopts published Host/Build/Testkit **0.66.0** together, keeping
shared Host type identities coherent. The integration harness takes client/server
ownership through `into_parts()` and retains client-before-server drop order.
No application source, fixture feature, deployment dependency or package release
version is changed. Incoming unrelated lockfile edits remain preserved.
The graph selects published Wire/derive 0.6.7, Platform 0.6.9, Memory/Storage
0.50.11 and Metrics 0.15.6. Host 0.66.0 remains the latest published family.

Shared Tooling **0.26.8**, revision `0b07c2a3163a5a66328e100b553baf93afd8ed07`,
is exported through the canonical refresh tool with the existing entrypoints
plus `audits/ci-release-integrity.md`, required by the refreshed audit index.
Snapshot payloads are not locally patched. Explicit Cargo-tool setup now waits
up to 60 seconds for a valid competing owner, then rechecks its inputs and output.
Offline checks remain immediate and no competing process's lock is deleted.
Versions 0.26.5–0.26.6 also improve evidence retention and review guidance.
The 0.26.8 committed snapshot fixes admitted-owner lock cleanup races and removes
the global source-cache setup lock. Explicit Cargo setup is rerun under the new
build policy; prior generations remain retained. Its changelog is still labelled
Unreleased; the committed revision above is the reviewed snapshot identity.

## Service ownership and remaining integration

Blob owns the funding journal and acknowledgment policy, root/reference/GC owner,
quotas and liabilities. Matching successful Cashier replies permit another
bounded distinct payment; missing/malformed accepted outcomes remain unresolved.
GC acknowledgment stops repeat listing but retains physical and billing liability.
Unknown roots remain protected. Source rollback fences and original identities
remain authoritative; no balance delta or timeout authorizes an uncertain retry.

Canic owns its adapter, lifecycle and mounted tests. Last read-only review found
local guarded funding composition and refunded/concurrent-call coverage under
[Canic #494](https://github.com/dragginzgame/canic/issues/494). Accepted-payment,
partial-refund, positive-credit and renewal qualification is separate and uses
Tooling's cycle-accepting fixtures. The current Tooling release includes those
fixtures; Blob does not add a second consumer implementation or test suite.
[Canic #622](https://github.com/dragginzgame/canic/issues/622) reports local GC races
qualified and awaits delivery. Recheck current consumer status before claiming
completion; this adoption did not rerun Canic or modify any sibling checkout.

[Blob #32](https://github.com/dragginzgame/ic-blob-storage/issues/32) is closed after
callback delivery. Remaining application/provider work stays in #4/#5/#6/#31/#34:
media-map publication, serving, multi-user acceptance, payer linkage, deletion and
billing. Provider acknowledgments are not independent credit or deletion proof.
[Funding guidance](../funding-consumer-qualification.md) and
[GC contract](../gateway-gc-contract.md) retain the limits and external questions.

## Validation

Previous 0.65 adoption evidence is retained in
`.tmp/review-validation/host065-shared0267/`, including snapshot preview/refresh,
locked metadata, boundaries, Clippy and explicit tool setup/checks.
All pass, along with contracts/golden tests, Rust 1.91 native CLI all-feature
checks, formatting/hooks, 126 snapshot payloads and 1,556 documentation links.
Shared fixture tests pass on the localhost-permitted rerun.
The first shared-fixture attempt failed only when the sandbox refused its local
HTTP listener; retain that log and `/tmp/tool-downloads.ZRexp6` separately from
its localhost-permitted rerun. No lock removal or ignored failure was used.
Prior Wire migration evidence remains in `.tmp/review-validation/wire062/` and
prior funding/GC implementation evidence in `.tmp/review-validation/provider-hardcut042/`.
No full release gate, native macOS, live provider operation or deployment is
claimed by this adoption. No staging, commit, push or release was performed.
Focused native CLI upload-input tests exercise the adopted Host filesystem path;
results are retained in `cli-inputs.log` alongside the adoption checks.

Current 0.66 adoption logs are in `.tmp/review-validation/host066/`.
Locked metadata, declaration/runtime boundaries, strict workspace Clippy,
formatting and documentation checks pass. The harness changes only explicit
ownership transfer; its existing drop order remains authoritative.
Rust 1.91 standalone-test compilation, release standalone Wasm and the installed
tenant ingress regression pass on the new harness. The PocketIC test exercises
server startup, application refusal/success and normal harness teardown. Full CI,
native macOS and live-provider qualification were not rerun.

Latest Shared 0.26.8 / Wire 0.6.7 adoption evidence is retained in
`.tmp/review-validation/shared0268-wire067/`. Locked metadata, snapshot and
runtime/declaration boundaries, strict workspace Clippy, contracts/golden vectors,
342 core tests, Rust 1.91 standalone Wasm and shared fixture tests pass.
Explicit common Rust/Testkit/Build setup completed under the new build policy.
No new runtime code, protocol identities or fixtures were introduced. Full CI,
fresh installed IC execution and native macOS were not rerun for this slice.
