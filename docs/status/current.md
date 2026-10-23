# Current status

Date: 2026-10-23

## Released contract and pending adoption

Local HEAD is released **0.43.0**, `d23039c`. The tenant methods now use native
Wire with identities `0043/001a` and `0043/001b`, zero-based failure tags,
borrowed 29-byte principals and a 512-byte frame ceiling. Other method identities
stay `0042`. Retired finite Method markers are removed without compatibility
readers. Persisted representations were not changed by the Wire migration.
Cross-release installations still follow retirement/reinstall policy.

Pending **0.43.1** adopts published Host/Build/Testkit **0.65.0** together, keeping
shared Host type identities coherent. Blob already uses native Wire callers and
does not consume Testkit's removed finite Method interface. No application source,
fixture feature, deployment dependency or package release version is changed.
The existing graph selects Wire/derive 0.6.5, Platform 0.6.9, Memory/Storage
0.50.11 and Metrics 0.15.6; this adoption changes only seven Tooling packages.

Shared Tooling **0.26.7**, revision `3e8d7cd3e7e9b7b79a5e800c30daf0be2710dab0`,
is exported through the canonical refresh tool with the existing entrypoints
plus `audits/ci-release-integrity.md`, required by the refreshed audit index.
Snapshot payloads are not locally patched. Explicit Cargo-tool setup now waits
up to 60 seconds for a valid competing owner, then rechecks its inputs and output.
Offline checks remain immediate and no competing process's lock is deleted.
Versions 0.26.5–0.26.6 also improve evidence retention and review guidance.

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

Current adoption evidence is retained in
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
