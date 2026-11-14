# Current status — 2026-11-14

## Delivery and accepted scope

**0.100.2** is pushed to `origin/0.100` at
[ba4003d](https://github.com/dragginzgame/ic-blob-storage/commit/ba4003da4970cf60f4d26a4c0071a78bf71eacd5),
confirmed by `git ls-remote` and a matching worktree during its post-push review.
**0.100.3** is the prepared GitHub checkpoint: native authenticated inspection,
immutable drain-request custody, guarded dispatch and Testkit 0.105.0 adoption.
No tag, publication, live deployment or Caffeine call occurred; all packages are
unpublished. Private repo `dragginzgame/ic-blob-storage` (1457703306), default branch
`0.100`, HTTPS origin. The [checkpoint handoff](https://github.com/dragginzgame/ic-blob-storage/blob/ba4003da4970cf60f4d26a4c0071a78bf71eacd5/docs/status/current.md)
retains earlier validation, artifact identity and release history.
Rust 1.100.0 / edition 2024, Toko Tooling 0.43.1. Three product crates and
`qualification/crates/` share the approved root workspace/lockfile; no Canic dependency.

## Current behavior and next work

The [experimental installed profile](../design/ic-blob-storage-design.md#experimental-installed-profile-in-01002)
uses one durable Service around the policy model and a host-granted Storage lease.
One versioned Wire record permits 64 historical uploads, 64 references and 64 KiB.
Mutations persist before reply; IO/encoding failure traps for IC rollback. Native
callers discard an owner after panic. Reopen validates identity/records and derives
indexes, reference counts and R/L/E. Missing/foreign state never becomes genesis.
Permanent root lifetimes and released references survive upgrades. Final release
removes live usage but retains unresolved declared exposure; counters do not bound
physical storage or spending. No migration/import/reset, provider calls, certificates,
timers or asynchronous provider machinery exist. API/format/identities remain experimental.

The thin host captures caller/time and mounts Wire update, query and manager-only
replicated inspection. The library gate closes fresh admissions while permitting
cleanup, inspection and historical replay. Upgrade requires drain and remains drained
on reopen; stale activation cannot reopen a later attempt. The application owns
[catalog publication and lost-reply cleanup](../design/application-boundary.md);
publication does not make public provider storage private.

The native commands require explicit target, manager PEM, independently trusted root,
expected Wire init configuration, reviewed Wasm and record version. `inspect` checks
authenticated module hashes and replicated configuration without writing stable state.
`prepare-drain` signs the explicit next attempt and durably saves immutable intent/
packet files; `observe-drain` authenticates that original request's status without
renewal or resubmission. Partial claims remain occupied; unknown/pruned/pending
outcomes refuse. Acknowledgment is historical, not current admission or activation approval.

`dispatch-drain` admits the binary's tooling snapshot through a trusted checkout,
locks the target under one shared custody root, rechecks host/attempt/expiry, then
publishes an irreversible per-attempt request-ID claim before submitting saved bytes.
Any existing claim bars another dispatch, including a replacement request ID. A
crash can strand an unsent request; preserve custody and observe, never infer no
execution from unknown status. Host FS owns durable publication/locking; Platform
owns transport/status verification. Trusted operators must preserve the root and
use it for every manager; an alternate root/controller or arbitrary Makefile is
outside this arrangement. See [management rules](../design/ic-blob-storage-design.md#interrupted-management-operations).

Next: implement only required stop/upgrade/start custody and original-outcome
handling, then qualify the provider binding before upload authority. No production
upgrade runner, snapshot freshness proof or activation command exists. Supported
management must refuse destructive replacement; unrestricted controllers can bypass
it. Inspection alone is no continuity proof. Do not add a generic recovery engine.

Caffeine calls are authorized **when ready**; do not ask again merely because of
the former local-only boundary. No contact/agreement exists; use available evidence.
The refusal to post the [prepared clarification](../design/caffeine-contract-review.md#prepared-provider-clarification--not-sent)
still applies; provider-call permission does not authorize outreach. The [D0 gate](../design/caffeine-contract-review.md)
remains open for independent Rust enrollment, exclusive deletion scope, byte bounds,
proof recovery/SDK resume and authority-lifetime/charge closure. A live witness needs
an identified fresh target/account, bounded scope and durable custody. Never import
legacy obligations or infer settlement from deletion permission, success or prefunding.
Do not repeat broad unchanged searches. Canic owns its future adapter.

## Latest validation and retained push review

0.100.3: `make test-management` passed against the actual native binary and production
Wasm through an owned PocketIC HTTP gateway/root, with strict selected native/Wasm
Clippy. It covers trust/configuration/artifact refusal, tooling refusal, target-lock
contention, competing dispatch processes, immutable claims, replacement-ID refusal
after a simulated post-claim interruption, and independent original-outcome observation.
No live IC/provider qualification or full CI was run. Native Rustdoc, formatting,
documentation links and snapshot admission passed. Wasm excludes native HTTP/crypto,
Testkit, PocketIC, Candid and tooling; one Runtime/Types generation remains.
Current LOC: **1,858 product / 2,568 tests / 396 examples**, delta **+560 / +525 / +0**
from the push. No service/host policy or durable-format change was needed in this slice.

Testkit [0.105.0](https://github.com/dragginzgame/ic-tooling/blob/9378ecb023aaff406a492d3f9bf18dd58bdb8221/releases/testkit/CHANGELOG.md#01050---2026-11-14)
retired unused pool APIs; fresh-instance tests needed no migration. Required Host
Artifacts/FS/Process 0.78.0 remove command-fds/nix; PocketIC stays 16.1.0. The matching
CLI was rebuilt/admitted by `make install-test-server`; both installed suites passed
at adoption. Setup remains explicit. Retain Runtime 0.22.3, Types 0.20.4 and existing
crypto-common 0.1.7/generic-array 0.14.7 selections; avoid incidental lockfile refreshes.

Post-push review of `ba4003d` against `18e48c1` used the pinned
[complexity method](https://github.com/dragginzgame/toko-tooling/blob/626da7cd9626cc2e152594862a2d82ea82ef51da/audits/complexity-and-technical-debt.md)
and [pushed overlay](https://github.com/dragginzgame/ic-blob-storage/blob/ba4003da4970cf60f4d26a4c0071a78bf71eacd5/AGENTS.md).
**PASS in affected structural scope; no actionable sediment finding.** It traced
Service/record/model/reference owners, Wire exports/hooks, installed callers, Make
and the Wasm graph. Service owns gate/attempt/persistence, LocalModel grants/root/
accounting, ObjectReferences liveness; record decoding validates and derives indexes.
The host owns ingress/memory/hooks. Axes: immutable configuration, one record format,
one monotonic attempt with active/drained admission. Canic can reuse Service; schema
changes belong to record validation/installed tests; provider exposure requires D0.
Retain bounded whole-record writes, permanent replay identities and independent
reopen validation. No speculative provider/funding/retry framework. Pushed LOC was
1,298/2,043/396 (+580/+669/+0 from prior push). This was not production lifecycle,
provider or performance qualification; current local changes have not been pushed.

Legacy remains read-only at private `dragginzgame/ic-blob-storage-legacy`, revision
`25bed1352f3d2b9659f0d2583f50fa9af859ea84`. Design §12 records inspected owners/tests
and keep/replace decisions, including claim-before-dispatch and original-packet
recovery. No legacy code, credentials or custody were imported. The sibling is absent;
custody was not reverified. Read its LEGACY.md before separately authorized recovery.
