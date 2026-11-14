# Current status — 2026-11-14

## Delivery and accepted scope

**0.100.3** is pushed to `origin/0.100` at
[2051ec2](https://github.com/dragginzgame/ic-blob-storage/commit/2051ec2409c5c0f5ac1e2cc7d98efa97eded80c7),
confirmed by `git ls-remote` and a matching worktree during its post-push review.
**0.100.4** is prepared for the requested push: stop/upgrade/start/activation custody,
reopen verification and explicit Platform 0.33.3 adoption. Delivery and its required
post-push review will be recorded after remote verification. No tag,
publication, live deployment or Caffeine call occurred; all packages remain unpublished.
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

Stop requires the exact current drained attempt, running platform state, expected
artifact and manager controller membership. Observation separates original outcome
from current platform status without calling stopped Blob. Stopped state does not
acknowledge an unsent request. The platform stop argument does not enforce Blob's attempt.
Upgrade requires an acknowledged original stop, matching attempt/dispatch claim and
current stopped state. It binds exact replacement Wasm (uncompressed, at most 1 MiB),
fixed Wire post-upgrade arguments and ordinary upgrade options. A predecessor-ID file
links custody; existing drain/stop formats remain unchanged. Upgrade observation
requires the original acknowledgment and replacement module hash, even for same-Wasm
upgrades. Rejection preserves the claim; no reinstall, rollback or restart fallback.
Operator review establishes candidate state compatibility; a Wasm header does not.

Start requires original stop and upgrade acknowledgments and both predecessor
claims in the shared custody root. Its original acknowledgment must be followed by
running-state, replacement-artifact, configuration, format and current drained-attempt
verification. An unsent request cannot borrow a running-state observation; later
activation or drain invalidates that post-start check. Actual exposure stays recorded
if verification fails, with no automatic compensation.

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

Next: complete the requested push and post-push review. D1 remains partial: streaming
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
This lifecycle increment added 345 test lines and no product code or dependencies.

Testkit 0.105.0 / Host FS 0.78.0 / PocketIC 16.1.0 setup remains explicit. Platform
[0.33.3](https://github.com/dragginzgame/ic-platform/blob/9035c3f73a9bd8e866737418daa3b741f14e3fad/releases/platform/CHANGELOG.md#0333---2026-11-14)
adoption aligned the root catalog with the already selected lock; no Blob API migration.
The retained graph uses Runtime/macros/guard 0.22.4, Types 0.20.4 and Agent 0.49.3,
with crypto-common 0.1.6/generic-array 0.14.9. Earlier older-selection handoff claims
were stale. `make test-standalone test-management` passed at adoption. Wasm retains
one Runtime/Types generation and excludes Agent, HTTP, Testkit, PocketIC, Candid/tooling.

Post-push review of `2051ec2` against `ba4003d` used the pinned
[complexity method](https://github.com/dragginzgame/toko-tooling/blob/626da7cd9626cc2e152594862a2d82ea82ef51da/audits/complexity-and-technical-debt.md)
and [pushed overlay](https://github.com/dragginzgame/ic-blob-storage/blob/2051ec2409c5c0f5ac1e2cc7d98efa97eded80c7/AGENTS.md).
**PASS in affected structural scope; no actionable sediment finding.** Source/graph
inspection traced CLI/Context, custody, installed callers and Make. Service retains
admission policy; Host FS owns durability, Platform protocol/transport. Axes: explicit
target/trust/artifact, one intent format, active-next-attempt admission and unclaimed/
claimed dispatch. Retain independent record checks and irreversible exposure claims.
Rehearsals: stop extends custody; upgrade binds its arguments/artifacts there; Canic
reuses Service. Pushed LOC 1,858/2,568/396 (+560/+525/+0) supplied native management
and installed qualification, with no second Blob ledger/retry engine or native
transport in Wasm. This did not qualify production lifecycle/provider/performance.
Current local work is not pushed or covered by that post-push review.

Legacy is read-only at private `dragginzgame/ic-blob-storage-legacy`, revision
`25bed1352f3d2b9659f0d2583f50fa9af859ea84`. Design §12 records inspected owners/tests
and keep/replace decisions, including original-packet custody, recovery orchestration
and lifecycle regressions. Activation retains authority/binding checks and refusal
without discarding obligations; the trusted-management policy replaces IC-history proofs.
No legacy code, credentials or custody were imported. The sibling is absent; custody
was not reverified. Read its LEGACY.md before separately authorized recovery.
