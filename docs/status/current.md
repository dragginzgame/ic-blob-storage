# Current status

Date: 2026-10-21

## Delivered source and pending work

Blob/Contracts **0.40.0** is released on default main at
`d81d80d1588306b1e2d25dd0a3848ab41c12afff`. Both registry packages are present
and non-yanked; their packaged VCS identities and Rust source match that release.
The public Platform family is 0.6. Hosts must align their types and rebuild;
endpoint identities, Wire frames, decoder budgets and production stable schemas
are unchanged. Cross-release transitions remain safe retirement/reinstall only.
Same-release recovery, exact retries, fences and retained obligations remain required.

Preserve the incoming **Cargo.lock-only** edits: Platform/macros 0.6.2,
Memory/Storage 0.50.5 and Wire/derive 0.5.9. They were present before the backlog
review and remain byte-for-byte unchanged. They are not qualified by the earlier
release-graph results. No dependency upgrade or product-code change was made by
the backlog review. Pending 0.40.1 adds probe evidence regressions; it does not yet
switch the production reader to an unavailable dependency.

## Ownership and delivered tooling

Platform owns IC identities/runtime and required external Candid; Wire owns
application framing/schema; Blob owns tenant policy, durable service workflows,
provider accounting and recovery. Host/Build/Testkit belong to
[IC Tooling](https://github.com/dragginzgame/ic-tooling); Memory/Storage to
[IC Persistence](https://github.com/dragginzgame/ic-persistence).

Shared Tooling **0.26.1**, revision `67d4f25d1a613a27d7dfd7b5a4ae119e0e529fc7`,
has a verified **125-file** snapshot from committed producer source. Only
`wasm-opt` is selected for IC setup/admission. Keep actual consumer routing,
admission, evidence, hooks and release recovery checks; generic matrices remain
with Shared. Production optimizer builds are separate from probes to exclude
`test-support`. Tooling 0.63's Testkit/server and release-editor selections require
explicit `cli` during setup/admission; no application library CLI feature is added.
Current private fixture codecs use Platform with unchanged bounded replies and
one guarded raw transport attempt. Frozen historical projections stay local.

## Backlog disposition and next actions

Reviewed all ten open issues and the remaining PR against delivered source,
registry availability, discussions and actual callers. Closed
[PR #94](https://github.com/dragginzgame/ic-blob-storage/pull/94) as fully superseded
by main after comparing every changed path; its branch/history remain intact.
Closed [#84](https://github.com/dragginzgame/ic-blob-storage/issues/84) for delivered
application DTO/probe decoupling and
[#95](https://github.com/dragginzgame/ic-blob-storage/issues/95) for published
Platform 0.6 installation alignment. No PR merge was needed.

Their remaining consumer qualification is explicitly retained at
[Canic #444](https://github.com/dragginzgame/canic/issues/444#issuecomment-6312734347):
all three graphs, canonical method codecs, dedicated/embedded installation,
production Root binding and same-release recovery, then consumer delivery and
#603 reconciliation. Canic's current working manifest already selects Blob 0.40;
this read-only source observation is not installed acceptance. Its local 0.112
work and remote delivery remain distinct. [IC Crates #1](https://github.com/dragginzgame/ic-crates/issues/1)
retains its older graph and complete worker/provider/catalog transaction.
No sibling checkout, deployment or paid effect was changed.

Eight Blob issues remain. The next local adoption is
[#96](https://github.com/dragginzgame/ic-blob-storage/issues/96), blocked on a
published compatible `ic-host-http` package. Registry and fresh-index checks still
return 404. Committed producer `14a04da07c35abf3c2a0bddcc03c6b0bc51ab95c`
now provides cancellation-safe `read_body_into`; publication is not established.
Seven probe cases pass on the existing reader and an isolated exact-source
candidate; candidate strict CLI Clippy, Rust 1.91 and native/Wasm graph checks pass.
The current checkout adds exact-bound, truncation and timeout-prefix regressions.
Evidence and the unapplied registry-adoption patch remain under
`.tmp/review-validation/host-http-adoption/`. Recheck the published release/API and
lock before applying it; no path override or nonexistent registry requirement is
selected here. The candidate maps new fallible allocation errors to a distinct
`body_allocation` evidence outcome and admits that code in record verification.
Preserve body-before-status ordering, timeout bytes and existing outcome labels;
request policy, hashing, redaction and durable evidence remain local. Keep the
separate verified streaming download path intact. #96 stays open for actual adoption.

Provider GC/funding evidence remains at [#32](https://github.com/dragginzgame/ic-blob-storage/issues/32)
and [#34](https://github.com/dragginzgame/ic-blob-storage/issues/34).
Application/provider publisher, serving, lifetime, multi-user and verifier
acceptance remains at #4/#5/#6/#31/#33. These are genuine unfinished requirements,
not closed by local substitutes. Maintain [the probe ledger](../evidence/caffeine-probes/README.md)
before any provider investigation; existing instructions authorize no paid effects.

## Validation and evidence

Current backlog evidence: `.tmp/review-validation/backlog0400/`. A separate
registry-only locked probe selects Blob/Contracts 0.40.0, Platform 0.6.1,
Memory/Storage 0.50.4 and Wire 0.5.8. The two reported actual installation/restore
call shapes pass warning-denied native Clippy and wasm32 checking; the normal/build
Wasm graph has one Platform 0.6.1. No conversion shim or legacy alias is required.
This compiles the published boundary; it does not execute Canic or qualify macOS.

The fixed Wire 0.5.8 graph passed all **20 local CI targets** in 930 seconds with
all 1,995 tracked file contents unchanged: 680 native cases, complete PocketIC,
original/O3/Os/Oz installation/restoration, Wasm and coordinated archives.
Evidence: `.tmp/review-validation/wire058-gate/` and
`target/blob-wasm-opt.eYy5ny/`. The earlier failure at
`target/blob-wasm-opt.LFJGVq/` records an uncached concurrent lock update and stays
retained. Perform dependency edits before preparation; keep inputs fixed during a gate.

The delivered 0.40 tooling matrix is still running at this review. MSRV and all
three published-example lanes pass; pending tooling jobs are not green evidence.
Current native macOS, installed Canic and live-provider acceptance remain separate.
Earlier Rust 1.91, pinned-image, browser and graph-specific observations remain
with their original sources under `.tmp/review-validation/shared0261-platform/`
and the [release handoff](https://github.com/dragginzgame/ic-blob-storage/blob/d81d80d1588306b1e2d25dd0a3848ab41c12afff/docs/status/current.md).
[Shared #294](https://github.com/dragginzgame/shared-tooling/issues/294) and
[Tooling #223](https://github.com/dragginzgame/ic-tooling/issues/223) retain producer
qualification independently. The [2026-10-20 handoff](handoff-2026-10-20.md),
[2026-10-18 handoff](handoff-2026-10-18.md) and [history](history.md) preserve earlier evidence.
