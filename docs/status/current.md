# Current status — 2026-11-14

## Delivery and accepted scope

The standalone reference application and dependency adoption are pushed to
`origin/0.100` at [5301b3a](https://github.com/dragginzgame/ic-blob-storage/commit/5301b3a87a04131e21011226b3150525dc2b8d53).
The maintainer clarified that "push" means a new GitHub version checkpoint;
**0.100.5** finalizes these changes and the changelog. Delivery-only review commits
share that checkpoint. No tag, registry publication, live deployment or Caffeine
call occurred. The earlier endpoint migration and
signed qualification remain at [337cdd6](https://github.com/dragginzgame/ic-blob-storage/commit/337cdd6301068093d8faa9a50b62f203a7d00c35).
Private repository `dragginzgame/ic-blob-storage`, branch `0.100`, HTTPS origin.
Four product crates and `qualification/crates/` share the approved root workspace
and lockfile. Standalone development remains independent of Canic.

Rust 1.100.0 / edition 2024; Toko Tooling 0.43.2, Host FS/Artifacts/Process 0.79.0,
Testkit 0.106.0, Platform 0.33.5, Runtime/macros/guard 0.22.5, Types 0.20.4,
Wire 0.22.1, Storage 0.74.1 and PocketIC 16.1.0. Delivery qualified the newer
Platform patches present in the final lock, superseding the earlier adoption-only
checks. Snapshot installation authenticated all 49 unchanged payload files and
rebuilt/admitted the selected Testkit CLI. API and persistence remain experimental.

## Current implementation and operating boundaries

Blob's [installed profile](../design/ic-blob-storage-design.md#experimental-installed-profile-in-01002)
uses one Service around the policy model and a host-granted Storage lease. One
versioned 64 KiB record admits 64 historical uploads and references. Reopen validates
identity/records and derives indexes, reference counts and declared R/L/E; missing
or foreign state never becomes genesis. Mutations persist before reply; encoding/IO
failure traps for IC rollback. Native callers discard the owner after panic.
Permanent root lifetimes and released references survive upgrade. Final release
retains unresolved declared exposure; counters do not bound physical bytes or cost.

The thin host mounts Platform guard/codec hooks with bounded Wire frames; no Wire
Endpoint package, alternate executor or Canic dependency remains. Canic owns its
future adapter. Library, host and admin source are unchanged by this delivery.

The [reference application](../design/reference-application.md) owns one governed
record with immutable users/Blob target and sixteen lifetime drafts. Original input
and reference identity precede calls. Published drafts derive the per-user catalog;
cancellation persists before exact authorization reconciliation and release.
Delayed replies cannot undo cancellation. Unresolved cleanup survives ordinary
upgrade; rejected attempts retain their slots. There is no command passthrough,
automatic retry, second Blob ledger, provider effect or global Verified flag.
Publication accepts the owning user's decision, not independently verified reads.
App maintenance uses trusted stop/drain/upgrade/start; Blob's admin does not manage it.

Blob admission closes on drain while cleanup, inspection and historical replay stay
available; reopen remains drained. The native admin retains immutable original
requests/packets and observes their original IDs without renewal. Unknown status
never proves nonexecution. Every supported manager preserves one shared custody
root. [Management rules](../design/ic-blob-storage-design.md#interrupted-management-operations)
bind explicit target, manager/root/configuration, artifact/format/attempt, predecessor
claims and original stop/upgrade/start acknowledgments before activation. Host FS
owns durability/locking; Platform owns transport/status. Candidate compatibility
needs operator review. Alternate custody roots, arbitrary Makefiles and unrestricted
controllers bypass the supported arrangement. No automatic compensation,
reinstall/rollback fallback or snapshot freshness proof exists.

## Qualification evidence

For 0.100.5, `make test-application` passes (7.38 s), `make test-management`
passes both custody/lifecycle tests (69.72 s), and the built admin reports 0.100.5.
The unchanged endpoint/rollback witness passed at 5301b3a (6.74 s).
The app witness installs both real Wasm modules on separate PocketIC subnets: owner
checks, exact retries, shared references, name replacement, cancellation during
publication, pending authorization, discarded cleanup reply, stopped-host failures,
cleanup after upgrade and full-history cleanup while Blob is drained. Publication
leaves Blob stable bytes unchanged. Historical replies are not current draft state.
Strict workspace/all-target and selected Wasm Clippy, Rustdoc, formatting, full
locked metadata, documentation links and diff checks pass. Unchanged domain suites
were not repeated. Linux/local simulator evidence does not qualify mainnet, macOS,
provider recovery, arbitrary migration, snapshot continuity or pending-call upgrade
refusal. The [application boundary](../design/application-boundary.md#evidence-and-acceptance)
records targeted legacy comparisons; [earlier evidence](https://github.com/dragginzgame/ic-blob-storage/blob/337cdd6301068093d8faa9a50b62f203a7d00c35/docs/status/current.md#endpoint-migration-evidence)
retains the preceding host/CLI and explicit predecessor-upgrade witnesses.

## Post-push architecture review

**PASS within the affected structural scope**, after pushing 5301b3a against 337cdd6.
Method: [complexity review](https://github.com/dragginzgame/toko-tooling/blob/626da7cd9626cc2e152594862a2d82ea82ef51da/audits/complexity-and-technical-debt.md),
common contract, simplicity guidance and IC addendum at that same pinned revision;
overlay: `5301b3a:AGENTS.md`. Inspected committed app protocol/state/entrypoints,
actual installed test, Make wiring, dependency graph and design; reused the exact
focused results above. This is not a full-system, provider or Wasm-capacity audit.

| State/decision | Owner and boundary |
| --- | --- |
| Draft, Published, Cancelled, Cleaned | App State owns durable transitions; cancel never reopens, Published names are unique per owner |
| Immutable user, target, declaration and ID | App protocol carries them; State enforces ownership/replay; endpoints capture ingress caller |
| Authorization, registration, references and R/L/E | Existing Blob Service remains sole authority; app observes through existing endpoints |
| Runtime calls versus durable intent | App endpoints own bounded awaits/counter; State owns saved intent; no timer or recovery engine |
| Fresh versus reopen | Separate lifecycle paths and Storage grants; corruption never creates fresh state |

No actionable duplicated-flow, state-space or ownership debt found. Independent
codec/memory setup is retained because these are separate canister trust/storage
boundaries. The catalog is derived, not a second index. Five Wire endpoints and
init/pre/post-upgrade hooks have concrete callers; no forwarding fixture remains.
Rehearsals by inspection: changing catalog-name policy touches app State/protocol
and its test; raising draft capacity touches that owner's encoding/reopen bounds;
provider integration belongs to Blob/client and remains blocked by D0, not another
application mode. No new abstraction or speculative refactor is justified.

`toko-cloc` reports **3,199 product / 4,561 tests / 396 examples**, versus
**2,702 / 4,248 / 396** at 337cdd6: **+497 / +313 / +0**. The same categorizer payload
is used. Growth is the runnable app and one installed consumer journey, not changed
Blob policy or dependency adapters. These counts are not Wasm size or performance.
Known lifecycle/provider proof gaps above remain explicit; no broader reliability
or settlement guarantee follows from this structural result.

## Provider gate and next work

Caffeine calls are authorized **when ready**; do not ask again. No contact/agreement
exists: use available evidence. The refusal to post the [prepared clarification](../design/caffeine-contract-review.md#prepared-provider-clarification--not-sent)
still applies; provider-call permission does not authorize outreach. [D0](../design/caffeine-contract-review.md)
remains open for independent Rust enrollment, deletion scope, byte bounds, proof
recovery/SDK resume and authority-lifetime/charge closure. A live witness needs an
identified fresh target/account, bounded scope and durable obligation custody.
The local app supplies the selected consumer; next integration needs the supported
contract and actual transfer/read journey. More local scaffolding cannot close D0.
D1 remains partial. Push only when requested; review affected owners after each push.

Legacy is read-only at `25bed1352f3d2b9659f0d2583f50fa9af859ea84` in private
`dragginzgame/ic-blob-storage-legacy`. The sibling is absent; custody was not
reverified. No legacy code, credentials or recovery state was imported. Never resume
legacy obligations here; read its LEGACY.md before separately authorized recovery.
