# Current status — 2026-11-14

## Delivery and local work

**0.100.1** is pushed to `origin/0.100` at
[18e48c1](https://github.com/dragginzgame/ic-blob-storage/commit/18e48c13f6607279ded6efa0209683dfb33c1881).
GitHub confirmed that SHA; the worktree was clean immediately after the push.
**0.100.2** is prepared for the requested GitHub checkpoint: tooling/design corrections,
durable control state, the standalone host and installed qualification. Its push
and post-push review are pending at this source snapshot. No tag,
crates.io publication, live deployment or Caffeine call occurred; all packages
remain `publish = false`. Private repository: `dragginzgame/ic-blob-storage`,
ID 1457703306; default branch `0.100`; origin uses its GitHub HTTPS URL.
Earlier evidence is in the [pushed handoff](https://github.com/dragginzgame/ic-blob-storage/blob/18e48c13f6607279ded6efa0209683dfb33c1881/docs/status/current.md).

Rust 1.100.0 / edition 2024; Toko Tooling 0.43.1. Two product crates under `crates/`
and the real host test under `qualification/crates/` share the approved root workspace
and lockfile. No placeholder package, second engine or Canic dependency was added.

## Implemented local host

The [experimental profile](../design/ic-blob-storage-design.md#experimental-installed-profile-in-01002)
uses `local::persistence::Service` around the existing policy model. One host-granted
Storage lease holds a versioned Wire record, bounded to 64 historical uploads,
64 references and 64 KiB encoded state. Every successful synchronous command
persists before reply. IO/encoding failure must trap for IC rollback; native
callers discard an owner after panic. Reopen validates records and rebuilds indexes,
reference counts and R/L/E counters; missing/foreign state never becomes genesis.
No migration/import/reset path exists. API, format and provider identities remain experimental.

The thin host captures `msg_caller` and IC nanosecond time, exports owned Wire
`blob_update` / `blob_query`, and composes fresh/open lifecycle hooks. It issues
no certificates or network calls. One library gate closes new admissions while
allowing authenticated cleanup, inspection and historical success replay. Upgrade
requires prior drain; reopening stays drained; stale activation cannot reopen a
later maintenance attempt. Application and management roles are explicit principals.
The manager-only `blob_management` update now inspects bound identity, immutable
configuration, record format and maintenance state without a storage write. Its
replicated result is a preflight input, not a module-identity or continuity proof.

Permanent roots, original uploaders/deadlines, immutable reference targets and
released outcomes survive. Final release removes live usage but preserves unresolved
declared exposure. Counters are not physical storage or spending bounds. The
[application boundary](../design/application-boundary.md) still owns catalog publication,
remote cancellation ordering and lost-reply cleanup; publication is not private serving.

Selected released suppliers: Storage 0.74.1, Wire 0.22.1, Endpoint 0.13.1,
Runtime 0.22.3, Types 0.20.3, Serde 1.0.229 and SHA2 0.11.1. Testkit 0.104.1 /
Tokio 1.54.1 supply native qualification; Memory is retired and was not adopted.
The Wasm graph has one Runtime/Types generation and no Testkit, PocketIC, Candid,
HTTP client or tooling dependency. Testkit owns authenticated PocketIC 16.1.0 setup.

## Next work and constraints

The maintainer authorizes the local installed host and **Caffeine calls when ready**;
do not ask again merely to cross the former no-deployment boundary. The explicit
refusal to post the [prepared clarification](../design/caffeine-contract-review.md#prepared-provider-clarification--not-sent)
still applies: provider-call approval does not authorize issue comments or outreach.
No separate provider agreement/contact exists; use available evidence. Do not import
legacy accounts, retained proofs or outstanding obligations into this installation.

Next: qualify a concrete production management path and provider binding before
exposing upload authority. The manager inspection prerequisite is implemented;
the runner must authenticate outcomes and compare admitted identity/configuration
and artifact, then handle interrupted stop/upgrade/start without destructive retry.
The current local fixture tests drain/stop/upgrade/start,
not production rollback refusal or interrupted management-call reconciliation.
An unrestricted controller can bypass hooks or restore an old image; no snapshot
freshness claim is made. Same-schema upgrades do not qualify schema migration.

The [D0 provider gate](../design/caffeine-contract-review.md) remains open: supported
independent Rust enrollment, exclusive deletion scope, byte enforcement, original-proof
recovery/SDK resume and authority-lifetime/charge closure are unqualified. Rabbithole's
Motoko account integration establishes none of those guarantees. Do not repeat broad
searches of unchanged sources or add a local mechanism in place of provider evidence.
A live witness needs an identified fresh target/account, bounded scope and durable
custody of possible effects. Manifest consistency does not authenticate length;
final local release cannot retract held proofs. Never infer settlement from deletion
permission, a successful sample or prefunding. Canic still owns its future adapter.

## Validation

Passing: `make test-local` (16 model/persistence tests); `make test-standalone`
(the production Wasm installed in PocketIC, full 64-slot history, wide IDs, actual
caller capture, overlapping references, cleanup, repeated upgrades, failed upgrade
rollback, malformed frames, stale activation and replicated manager inspection);
strict selected native and Wasm
Clippy; product Rustdoc; formatting, tooling admission, dependency inventory,
server admission, local documentation links and whitespace checks. The initial
relative server path failed Testkit's physical-path rule; it now uses an explicit
absolute path. No full CI, live signed-ingress or provider qualification was run.
Unchanged integrity/reference algorithms retain their earlier 12/7-test evidence;
the offline example retains its four passing catalog/read-boundary tests.

The installed test reopens at capacity and checks stable memory stays within the
host's 16-page bound; this is not an instruction-cost or sustained-use qualification.
Wasm: 419,940 bytes, SHA256
`95d17d0e3ce58aeaa180fc1d2768e756e24c5db04ef62a9168d7479e539fe783`.
Cargo artifact records and the selected Wasm graph are under `target/blob-wasm-*`.
Current `toko-cloc`: **1,298 product / 2,043 tests / 396 examples**, zero tooling or
unclassified Rust. Since the last push: **+580 / +669 / +0**. Growth is the bounded
record/validation, thin host, Wire requests and their native/installed callers.
The inspection increment adds 34 product lines and 123 test lines, no dependency,
new stored record or independent policy owner; it reuses the durable format version.

Toko 0.43.1 archive checksum
`467162f616caafe69c965fe4e96173a0df37256f4767bfefd28114781aad76b3`, source
`b50aeb42a9e23fca22f08bb0c31cd19b728e8591`: authenticated adoption preserved
49 managed files; only baseline prose changed. Scripts/policy references stayed
fixed. This batch adds consumer-owned simulator setup/test commands; no managed
file was hand-edited. Explicit setup is included in `make install-tools`; ordinary
tests perform offline server admission and do not download implicitly.

## Last post-push architecture review — unchanged historical scope

Push: `18e48c13f6607279ded6efa0209683dfb33c1881`; comparison:
`8cb3c40bdd09e36f4f92c410f8773a361f398706`. Used Toko's
[complexity review](https://github.com/dragginzgame/toko-tooling/blob/626da7cd9626cc2e152594862a2d82ea82ef51da/audits/complexity-and-technical-debt.md)
and the [pushed overlay](https://github.com/dragginzgame/ic-blob-storage/blob/18e48c13f6607279ded6efa0209683dfb33c1881/AGENTS.md).
Reviewed pushed owners, tests, example callers, dependencies and design while the
worktree matched the push. **PASS within that local structural scope**, with no
installed-behavior claim or actionable sediment finding. LocalModel owned grants,
root lifetimes and R/L/E; ObjectReferences owned liveness; inspection derived views;
the private catalog owned application decisions. Removed duplicate object byte and
per-reference upload fields. Kept permanent IDs for stale replay, record limits,
three counters to avoid scans and shared integrity helpers. No new package/dependency,
recovery engine or generic retry mechanism was in that push. Source rehearsals kept
ingress in the host and durability with the policy owner, subject to later qualification.
Pushed LOC: **718 / 1,374 / 396**, delta **+24 / +222 / +52**. The current local
changes have not been pushed or received their required post-push review.

Legacy remains read-only at private `dragginzgame/ic-blob-storage-legacy`, ID 1387534636,
revision `25bed1352f3d2b9659f0d2583f50fa9af859ea84`; relevant issuer, root and lifecycle
implementation/regressions informed this work (links and exclusions in design §12).
This batch imported no legacy state or code. The sibling path is absent; custody was not
reverified. Read its `LEGACY.md` before separately authorized recovery. Bootstrap
logs remain under `target/bootstrap-20261114/`; ignored credentials stay untouched.
