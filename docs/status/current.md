# Current status — 2026-11-14

## Delivery and accepted scope

**0.100.4** is pushed to `origin/0.100` at
[102edcd](https://github.com/dragginzgame/ic-blob-storage/commit/102edcd5d00c5e34dc76bdf66acc2d228962bb6e),
confirmed by `git ls-remote` and a clean matching worktree during post-push review.
This handoff-only follow-up records that delivery and review. The checkpoint adds
stop/upgrade/start/activation custody, reopen verification and Platform 0.33.3.
No tag, registry publication, live deployment or Caffeine call occurred.
Private repo `dragginzgame/ic-blob-storage` (1457703306), branch `0.100`, HTTPS origin.
The [checkpoint handoff](https://github.com/dragginzgame/ic-blob-storage/blob/2051ec2409c5c0f5ac1e2cc7d98efa97eded80c7/docs/status/current.md)
retains earlier validation, artifact identity and release history.
Rust 1.100.0 / edition 2024; Toko Tooling 0.43.1. Three product crates and
`qualification/crates/` share the approved root workspace/lockfile; no Canic dependency.

## Current implementation

The [experimental installed profile](../design/ic-blob-storage-design.md#experimental-installed-profile-in-01002)
uses one durable Service around the policy model and a host-granted Storage lease.
One versioned Wire record permits 64 historical uploads, 64 references and 64 KiB.
Mutations persist before reply; IO/encoding failure traps for IC rollback. Native
callers discard an owner after panic. Reopen validates identity/records and derives
indexes, reference counts and R/L/E; missing/foreign state never becomes genesis.
Permanent root lifetimes and released references survive upgrades. Final release
removes live usage but retains unresolved declared exposure; counters do not bound
physical storage or spending. No migration/import/reset, provider calls, certificates,
timers or asynchronous provider machinery exist. API/format remain experimental.

The host captures caller/time and mounts Wire update/query/manager inspection. The
library gate closes fresh admissions but permits cleanup, inspection and historical
replay. Upgrade requires drain and remains drained on reopen; stale activation cannot
reopen a later attempt. The application owns [catalog publication and lost-reply cleanup](../design/application-boundary.md).
Publication is not private serving. Canic owns its future adapter around Service.

The native CLI requires explicit target, manager PEM, independently trusted root,
expected Wire init configuration, original Wasm and record format. `inspect` verifies
module hashes and replicated configuration without writing stable state. Preparation
signs one request and durably saves immutable intent/packet files. Observation uses
its original request ID without renewal or resubmission. Partial claims stay occupied.
A historical acknowledgment is not current admission, continuity or activation approval.

Dispatch admits the binary's tooling snapshot through a trusted checkout, takes a
shared target lock, rechecks eligibility/expiry and publishes a per-attempt claim
before sending saved bytes. Any existing claim bars redispatch or a replacement ID.
Interruption can strand an unsent request; unknown status never proves no execution.
Host FS owns publication/locking, Platform transport/status. All supported managers
must preserve and use one custody root; alternate roots, controller bypass and
arbitrary Makefiles are outside this trusted arrangement. See [management rules](../design/ic-blob-storage-design.md#interrupted-management-operations).

Stop requires the exact drained attempt, running state, artifact and manager/controller.
Its platform argument cannot enforce Blob's attempt. Upgrade requires the original
stop acknowledgment, matching attempt/claim and stopped state. It binds uncompressed
replacement Wasm ≤1 MiB, fixed Wire arguments and ordinary upgrade options. Original
acknowledgment and replacement hash are both required, including for same-Wasm upgrades.
Candidate compatibility requires operator review. No reinstall/rollback/restart fallback.

Start requires original stop/upgrade acknowledgments and predecessor claims. Its
acknowledgment is followed by running-state, replacement-artifact, configuration,
format and current drained-attempt verification. Current state cannot acknowledge
an unsent request; later activation/drain invalidates that check. Failed verification
preserves actual exposure without automatic compensation.

Explicit activation links the original start and rechecks its acknowledgment and
current drained attempt before dispatch. All predecessor claims must match the
shared custody root. The service rechecks manager/attempt at execution; a delayed
older activation cannot reopen a newer drain. Observation requires original success
and current matching active state; historical success never permits resubmission.
No activation without the stop/upgrade/start chain is implemented.

The populated native lifecycle witness retains reserved, Open and Closing objects,
two uploaders and released references across a distinct Wasm hash. The replacement
adds only labelled custom metadata; this is artifact-binding/state-preservation
qualification, not schema migration or arbitrary-code compatibility.

Next: select further work against the open D0 contract. D1 remains partial: streaming
integrity and catalog publication have separate local witnesses, but no integrated
provider certificate/upload/read journey exists. D0 still gates that work.
Current state cannot replace original-operation completion. No snapshot freshness
proof exists. Supported management refuses destructive replacement; unrestricted controllers can bypass it.
Keep management outside Blob's library and avoid a generic recovery framework.

## Provider boundary

Caffeine calls are authorized **when ready**; do not ask again merely because of
the former local-only boundary. No contact/agreement exists; use available evidence.
The refusal to post the [prepared clarification](../design/caffeine-contract-review.md#prepared-provider-clarification--not-sent)
still applies; provider-call permission does not authorize outreach. The [D0 gate](../design/caffeine-contract-review.md)
remains open for independent Rust enrollment, exclusive deletion scope, byte bounds,
proof recovery/SDK resume and authority-lifetime/charge closure. A live witness needs
an identified fresh target/account, bounded scope and durable custody. Never import
legacy obligations or infer settlement from deletion permission, success or prefunding.
Do not repeat broad unchanged searches or invent local mechanisms as provider evidence.

## Validation and retained push review

0.100.4 passed `make test-management` through the actual native binary, production
Wasm and owned PocketIC HTTP gateway/root, with strict selected native/Wasm Clippy.
Coverage includes custody contention/refusal, stop controller/attempt checks,
original-outcome observation, actual same-Wasm upgrade, predecessor/candidate mismatch,
unsent replacement-ID refusal, invalid-Wasm rejection, guarded start, transitive
predecessor-claim checks and retained state/closed admission after verified reopening.
Explicit activation passed through the real CLI, including missing predecessor
claims, unsent replacement refusal, fresh-process observation and a late activation
rejected by a newer drained attempt. Workspace Rustdoc, formatting, links and tooling
admission passed. No full CI, live IC/provider or macOS execution qualification.
Both management tests passed, including populated state across a distinct artifact,
retained permissions/exposure, non-resurrecting replay, cleanup and fresh admission.
LOC: **2,668 product / 3,655 tests / 396 examples**, **+810 / +1,087 / +0** from the push.

Testkit 0.105.0 / Host FS 0.78.0 / PocketIC 16.1.0 setup remains explicit. Platform
[0.33.3](https://github.com/dragginzgame/ic-platform/blob/9035c3f73a9bd8e866737418daa3b741f14e3fad/releases/platform/CHANGELOG.md#0333---2026-11-14)
adoption aligned the root catalog with the already selected lock; no Blob API migration.
The retained graph uses Runtime/macros/guard 0.22.4, Types 0.20.4 and Agent 0.49.3,
with crypto-common 0.1.6/generic-array 0.14.9. `make test-standalone test-management`
passed at adoption. Wasm retains
one Runtime/Types generation and excludes Agent, HTTP, Testkit, PocketIC, Candid/tooling.

Post-push review of `102edcd` against `2051ec2` used the pinned
[complexity method](https://github.com/dragginzgame/toko-tooling/blob/626da7cd9626cc2e152594862a2d82ea82ef51da/audits/complexity-and-technical-debt.md)
and [pushed overlay](https://github.com/dragginzgame/ic-blob-storage/blob/102edcd5d00c5e34dc76bdf66acc2d228962bb6e/AGENTS.md),
with its common contract, simplicity guidance and IC application addendum at that revision.
**PASS in affected structural scope; no actionable sediment finding.** Source/graph
inspection traced CLI inputs through one prepare/load/dispatch/observe owner and the
installed callers/Make target. Operation-specific admission stays in management;
Service still owns domain/admission state, Host FS durability, Platform protocol/transport.
Axes: five explicit operations, one intent format, fixed bounded predecessor chain,
bound old/new artifacts, original outcome versus current platform/admission, and
unclaimed/claimed dispatch. Partial or existing claims refuse; no fallback mode exists.
The old drain owner and flat test file were replaced, not retained alongside successors.
CLI repetition is argument plumbing; independent record and host checks protect
different trust/execution boundaries. No new ledger, generic workflow or provider path.
Rehearsals: another same-format artifact uses this custody flow with operator review;
Canic composes the unchanged Service through its own host/lifecycle adapter. Neither
requires moving domain policy into native management. No structural blocker found.
Comparable LOC: 1,858/2,568/396 → 2,668/3,655/396 (+810/+1,087/+0).
Growth supplies four native operations and actual installed regressions, including
populated obligations. Product library/host sources and managed tooling are unchanged.
The locked Wasm graph still excludes Agent/HTTP/Candid/Testkit. Existing passing
behavior checks were reused; this review adds source/graph/LOC inspection, not a new
runtime, provider, snapshot, macOS or performance qualification. The handoff-only
follow-up changes delivery/review metadata; it adds no product or tooling behavior.

Legacy is read-only at private `dragginzgame/ic-blob-storage-legacy`, revision
`25bed1352f3d2b9659f0d2583f50fa9af859ea84`. Design §12 records inspected owners/tests
and keep/replace decisions, including original-packet custody, recovery orchestration
and lifecycle regressions. Activation retains authority/binding checks and refusal
without discarding obligations; the trusted-management policy replaces IC-history proofs.
No legacy code, credentials or custody were imported. The sibling is absent; custody
was not reverified. Read its LEGACY.md before separately authorized recovery.
