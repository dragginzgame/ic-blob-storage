# Current status — 2026-11-01

## Delivered baseline

**Blob 0.61.0 is released and published.** Default main and annotated tag
`v0.61.0` resolve to **`df66622064584cdfd32b1230ee387adf93829b4d`**.
Contracts, Client and Core registry archives were independently checked against
their index checksums, exact Git source and package paths. The receipt is
[docs/release.json](../release.json). No deployment or paid provider effect occurred.

The complete local release gate passed in an isolated checkout, including native
and PocketIC tests, all four original/O3/Os/Oz Wasm installation/restoration
selections and real package verification. Evidence and publication readback:
`/tmp/blob-release-061-20261031/`. The first attempt stopped on a concurrent
Cargo.lock change; it did not create a release. The fixed committed cohort passed
on retry. The original checkout was fast-forwarded afterward, and every incoming
lockfile change was verified preserved with only the thirteen local package
versions advanced to 0.61.0. Nothing was force-pushed or cleaned.

[Published example installation](https://github.com/dragginzgame/ic-blob-storage/actions/runs/41228721444)
and the [release/native workflow](https://github.com/dragginzgame/ic-blob-storage/actions/runs/41228721315)
are successful for that exact commit. The latter's core/client, Linux, both
native macOS architectures, MSRV and generated-Wire-client jobs passed. This
qualifies delivered 0.61.0, not the pending dependency graph below.

## Pending hard cut — 0.62.0

The maintainer authorized the complexity cleanup as a hard cut. Cargo package
versions remain at released 0.61.0; only the next changelog heading is 0.62.0.
Pending dependency edits are preserved, including incoming Memory/Storage 0.60.4
lock selections. No commit, release, publication, deployment or provider probe
is authorized by this cleanup.

Removed the parallel `test_support::admission::UploadAdmissions` service and
its manifest/enrollment/accounting adapters. The admission canister now grants
exclusive memories through Memory and canonical Blob grants and executes
`StableUploads`; it retains the runtime and uses production certificate gates.
Its explicit provider deletion/settlement inputs remain labelled substitutes.
Removed three single-implementation read-source traits and their fence wrappers;
shared workflows read the same durable owner under one synchronous borrow.
Native regressions assert specific admissions, receipts, headroom, accounting,
activation generations and reopened fences instead of comparing two services.

Removed the unconsumed public ledger-notification decoder. Ordinary top-ups,
exact signed retries, uncertain funding, durable receipts and recovery are intact.
Removed historical Candid DTO projections and old-image clients/drivers, including
`test-hard-cut`; current release/format refusal still compares retained memory
bytes. A frozen valid Candid scope packet exercises current ingress rejection.
External provider response fixtures keep necessary Candid wire types locally.
The CLI's current configuration fixture is JSON, with no historical decoder,
mirror types, Hex dev dependency or explicit Candid test feature.

The remaining pure catalog/lifecycle reference models and authority probe still
have independent caller and recovery scenarios; they were not deleted as unused.
CLI grammar construction and typed command dispatch also remain: replacing them
with another registry/framework would add churn without removing a demonstrated
competing policy owner. Further retirement must first map and migrate unique
coverage. Source-owned durable records, boundaries and retry journals remain.

Focused validation and migration evidence is retained in `/tmp/blob-cleanup-1101/`.
Final strict all-target/all-feature Clippy passes for Core, CLI, admission probe,
protocol and PocketIC support. Core **295**, CLI **122** and private protocol **12**
native tests pass. The installed admission run passed **29** cases and found one
stale probe-specific error assertion; the corrected exact production-refusal case
passes separately. All resource ceilings remain unchanged. Additional installed
checks pass: malformed standalone ingress **4**, external account/provider replies
and native account CLI **4**, consumer codec admission **1**, and repeated
stop/start/upgrade fencing **1**. This is **30 admission + 10 other distinct cases**;
the initial failed run and focused retry are both retained.

Fresh release Wasms were built for admission, storage, consumer and gateway probes,
and separately for the default standalone. Rust **1.91** CLI all-target/all-feature
and standalone Wasm checks pass. Core rustdoc with warnings denied, formatting,
direct/deployment dependency boundaries, pins, documentation links and release
command checks pass. Critical manifest/lock/compiler-pin hashes stayed fixed.
The new private exposure error wraps the production `UploadExposureFailure`
directly; it does not reproduce the old fixture-specific classification.
Published application frames, persisted record formats and same-release retry/
recovery behavior are unchanged. No full release gate, browser run, native macOS
or deployed-provider qualification is claimed. Historical dependency-adoption
qualification below remains evidence for its original source only.

## Preserved dependency update


The maintainer requested the latest Memory/Storage adoption and next steps.
Root minimums are now **Memory/Storage 0.60.3**; they were already selected in the
incoming lock. Official index entries and both published archive checksums were
verified. Production `src/` trees are identical to 0.60.1; the supplying releases
change dependency/CI tooling, including Platform 0.15.3 selection. Stored bytes,
codec bounds, allocation custody and restore fences are unchanged. No migration,
reset, compatibility layer, new schema number or endpoint change was introduced.

The subsequent Build/Host/Testkit request adopts published Foundation **0.73.3**
and Host Tools/Testkit **0.75.2** minimums, matching incoming lock selections.
Testkit requires **Wire 0.13.2**, also already selected in the lock; the direct
Wire minimum stays 0.13.0. Platform **0.15.3**, Types **0.15.1**, Metrics **0.24.1**
and Memory/Storage **0.60.3** are preserved. There is one Platform, Wire and
Metrics version. No broad dependency update or lock regeneration was run.
Shared Tooling remains **0.27.22**, reviewed revision `f4f431627290e18f0ccc1e5f8525d16b197cf7c3`.

## Current Build/Host/Testkit qualification

Evidence: `/tmp/blob-tooling-1101/`, including prior inputs and verified official
release indexes/archives. Foundation 0.73.3 exposes bounded package-command
capture; Blob already uses Build's qualifier and `PackageCommandError` execution/
retention projection, so no local wrapper or import migration was needed. Testkit
0.75.2 preserves codec behavior/budgets, reuses Host's bounded startup port reader
and selects Wire 0.13.2. Published Build release-metadata/CLI implementation
sources match the earlier qualified supplying version; its captured-process and
Wasm-cache code changes have their own producer coverage and consumer checks here.

Strict all-target/all-feature CLI Clippy and selected standalone/journey PocketIC
Clippy pass, along with Rust **1.91** all-feature CLI checking. CLI **122** and
private probe-control **7** tests pass. The real archive-verifier regression
passes same-version changed archives, warm-cache execution and graph/byte/source/
manifest/lock drift refusals (`target/package-fixture.ZnDAyT`). Fresh standalone,
authority and gateway release Wasms and all-feature CLI/verifier builds pass.
Formatting and direct/deployment dependency checks pass.

Canonical Testkit/editor setup initially timed out waiting 60s for an active
shared installer owned by IC Query. The lock was not removed. The supported
300s-wait retry reused the completed cache and passed; exact Testkit 0.75.2 and
Build editor 0.73.3 admission and actual command-forwarding/failure-order fixtures
pass. Both attempts are retained. Server remains PocketIC 16.1.0. All **13**
selected installed cases pass: lifecycle/restoration **5**, malformed ingress
**4**, signed metrics **2**, gateway synchronization/restore fencing **2**.
Input hashes stayed fixed; artifact hashes and logs are retained. No full gate, native macOS or deployed
provider qualification is claimed for this pending graph.

## Preceding Memory/Storage qualification

Evidence: `/tmp/blob-persistence-next/`, including prior files,
release indexes/archives, source comparisons and input hashes. Strict core and
standalone Clippy across targets/features passes, as do **338** core unit tests,
**2** memory-composition tests, Rust **1.91** standalone Wasm checking and fresh
standalone/authority/gateway release Wasm builds. The current lock-selected
Testkit CLI/server was prepared/admitted through canonical setup; no older tool
or artifact generation was removed. All **11** selected installed cases pass:
lifecycle/restoration **5**, malformed ingress **4**, gateway synchronization and
restore fencing **2**. Input hashes stayed fixed. Formatting and direct/deployment
boundaries pass. No full gate, fresh browser run, native macOS or live-provider
qualification is claimed for this pending patch graph.

The dependency-only notes initially targeted 0.61.1. The subsequently authorized
hard cut now targets **0.62.0** as described above; workspace/package versions stay
**0.61.0**. No staging, commit, release, publication or deployment was performed.
The previous detailed handoff is retained at
`/tmp/blob-persistence-next/previous-handoff.md`; historical evidence stays with
its existing owner.

## Next: Canic adoption and bounded Caffeine trial

Canic can adopt the **already published Blob 0.61.0 now**. That release's
compatible requirements already allow Memory/Storage 0.60.3; this local minimum
update does not require another Blob publication before consumer work begins.
The read-only Canic checkout remains at `247ba9d3`, with Blob 0.60, Platform 0.11,
Wire 0.10 and Memory/Storage 0.58 declared in its root/consumer catalogs. Its
Blob dedicated/embedded deployment configs are local-only. No sibling was edited
or built in this pass.

The framework/integration owner should adopt Blob 0.61 across the root and both
independent consumer catalogs/locks, align public/runtime identities, and qualify
the existing dedicated/embedded mounting, synchronous lifecycle and generated
client paths. It then supplies a finite, bounded acceptance-service configuration
and network/role binding. The maintainer knows Caffeine is the provider; do not
ask them to invent an application/fleet or infer a service from historical trial
principals. Caffeine needs no new selection mechanism or receipt API for the
supported acknowledged happy path.

The concrete handoff is on
[Canic #622 — Mounted Blob composition](https://github.com/dragginzgame/canic/issues/622#issuecomment-6546806175).
Once its matching configuration is qualified, deployment of the bounded service
is the next step. A provider-only trial may precede a finished frontend; it does
not qualify application login, CSP or catalog publication. Exact provider intent,
target and approved spending remain separate from deployment preparation.
Use [the acceptance plan](../acceptance-plan.md#before-the-acceptance-deployment).

Four Blob acceptance issues remain open:
[#5 — Consumer publication and serving](https://github.com/dragginzgame/ic-blob-storage/issues/5),
[#31 — Application multi-user uploads](https://github.com/dragginzgame/ic-blob-storage/issues/31),
[#6 — Reference lifecycle and deployed deletion/billing](https://github.com/dragginzgame/ic-blob-storage/issues/6),
[#34 — Project-payer linkage and funding evidence](https://github.com/dragginzgame/ic-blob-storage/issues/34).
IC Crates #1 owns its actual catalog transaction/migration; its last-reviewed
source still uses retired verifier flows. No downstream application acceptance
or issue closure follows from these library dependency updates.

## Preserved evidence and operating boundaries

`/tmp/blob-acceptance-1031.QfEhzI/` records four passing Chromium/CLI/PocketIC cases
and three controls on the preceding patch graph: six 1 KiB uploads, two fixture
signers, verified reads, corruption refusal and lost-reply/withdrawal recovery.
It is a local substitute, not deployed Caffeine or application-session evidence.
The [probe ledger](../evidence/caffeine-probes/README.md) retains intent, failed
preflight receipt, source/manifest comparisons, reports and exact graph scope.
Earlier release/fixture/browser/MSRV proof remains at the paths in the preserved
previous handoff. Optional Observability comment approval remains unresolved;
no new Observability post occurred.

Cross-release service transitions remain retirement/reinstall only. Same-release
recovery and restore fences remain required. Retain provider objects, balances,
uncertain effects, exact retry bytes and continuing billing through
[retirement](../retiring-installations.md). Registration proves no stored bytes;
gateway acknowledgment proves no physical deletion/billing cessation; funding
acknowledgment is no independent account-credit proof. Never clear an allocation
ledger over retained bytes or retry an uncertain paid effect automatically.
