# Current status — 2026-11-14

## Delivery and accepted scope

The endpoint migration, signed application/uploader qualification and reconciled
design are pushed to `origin/0.100` at
[7832c4f](https://github.com/dragginzgame/ic-blob-storage/commit/7832c4f1fe631c07dadaee913c7ba1ae624f32f9),
with delivery/review handoff at
[337cdd6](https://github.com/dragginzgame/ic-blob-storage/commit/337cdd6301068093d8faa9a50b62f203a7d00c35).
Version remains **0.100.4** with Unreleased notes; no tag, registry publication,
live deployment or Caffeine call occurred. [Issue #1](https://github.com/dragginzgame/ic-blob-storage/issues/1)
is closed; its earlier local-only validation is now delivered by that commit.
The maintainer selected **reference application first** and requested delivery.
The prepared changes add one product application crate, replacing the temporary
forwarding fixture, plus Toko Tooling 0.43.2, Host FS/Artifacts/Process 0.79.0 and
Testkit 0.106.0. Final delivery also qualifies the current lock's Platform 0.33.4
and Runtime/macros/guard 0.22.5 patches; no unrelated product code changed.
The [D0–D4 table](../design/ic-blob-storage-design.md#9-delivery-and-acceptance)
separates implemented local behavior from missing provider/application evidence.
Private repo `dragginzgame/ic-blob-storage` (1457703306), branch `0.100`, HTTPS origin.
Rust 1.100.0 / edition 2024; Toko Tooling 0.43.2. Four product crates and
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

The host now mounts its three ordinary methods directly through Platform's generic
`guard`/`decode_with`/`encode_with` hooks and bounded Wire codecs. Its private codec
module checks ingress size/header before payload copying. Platform still owns the
single executor/reply; Service owns caller policy and typed `Error::Denied` replies.
Names, method IDs, limits, management-as-update, lifecycle and storage are preserved.
Both Wire Endpoint packages are removed; no replacement framework was introduced.
Standalone development remains independent. Canic owns its future Service adapter.
The application owns [catalog publication and lost-reply cleanup](../design/application-boundary.md);
publication does not establish private serving.

The [standalone reference application](../design/reference-application.md) has
immutable users/Blob target and one governed, bounded durable record: sixteen
lifetime drafts, original authorization/reference identity and local disposition.
The catalog is derived from Published drafts. Cancellation persists before calls;
exact authorization reconciliation precedes release. Delayed replies cannot undo
cancellation; unresolved cleanup survives ordinary upgrade. It uses existing Blob
methods with captured callers and no command passthrough, automatic retries,
provider effects or second Blob ledger. Publication accepts the owning user's
decision, not an independent read-verification proof. API/format remain unfrozen.

Admission closes on drain while cleanup, inspection and historical replay remain
available. Upgrade requires drain and remains drained on reopen; stale activation
cannot reopen a later attempt. The native CLI binds explicit target, manager PEM,
independently trusted root, expected init configuration, original Wasm and record
format. It saves immutable intent/packet files before dispatch and observes the
original request ID without renewal. Partial or existing claims bar redispatch or
replacement IDs; unknown status never proves no execution. A historical outcome
is not current continuity or activation approval.

Dispatch admits tooling through a trusted checkout, locks the target and rechecks
eligibility/expiry before claiming and sending. Host FS owns durability/locking;
Platform owns transport/status. Every supported manager preserves one shared
custody root. [Management rules](../design/ic-blob-storage-design.md#interrupted-management-operations)
require original stop/upgrade/start acknowledgments and matching artifact,
configuration, format and attempt. Candidate compatibility needs operator review;
no automatic compensation, reinstall/rollback fallback or snapshot freshness proof
exists. Alternate roots, arbitrary Makefiles and unrestricted controllers can
bypass this trusted arrangement.

## Qualification evidence

The [pushed evidence](https://github.com/dragginzgame/ic-blob-storage/blob/337cdd6301068093d8faa9a50b62f203a7d00c35/docs/status/current.md#endpoint-migration-evidence)
records passing endpoint, standalone, signed and native management checks, selected
strict Clippy, formatting, Rustdoc and links. It retains the predecessor Wasm hash
and old-to-new upgrade witness, populated same-format state preservation, reply
encoding rollback and original management custody. These are not arbitrary schema
migration, live IC/provider, macOS or snapshot-freshness qualifications. Runtime
tests remain valid for the unchanged production source; they were not all rerun.

`make test-application` passes (7.09 s on the final delivery lock) with both real Wasm modules on separate
PocketIC subnets. It qualifies owner checks, exact retries, shared-object references,
catalog-name collision/replacement, cancellation during a pending publication read,
pending authorization/cancellation, discarded cleanup reply, stopped-host failures,
cleanup after same-format app upgrade and full-history cleanup while Blob is drained.
Publication leaves Blob stable bytes unchanged. Historical replies are not current
draft state. App lifecycle evidence uses stop/drain/upgrade/start; pending-call
upgrade refusal, arbitrary migration and snapshot continuity are not qualified.
Strict workspace/all-target Clippy, host/app Wasm Clippy, Rustdoc, formatting,
documentation links, dependency inventory and diff checks pass. Existing unrelated
runtime suites were not repeated. On the final delivery lock, `make test-management`
passes both native custody and changed-artifact lifecycle tests (70.46 s), and
`make test-endpoints` passes installed frame/rollback checks (6.74 s).
The [application boundary](../design/application-boundary.md#evidence-and-acceptance)
records pinned legacy publication/recovery lessons and their deliberate exclusions.

Current LOC is **3,199 product / 4,561 tests / 396 examples**: **+497 / +313 / +0**
from 337cdd6. Product growth is the actual application protocol, draft owner and
entrypoints; there is no retained forwarding fixture or generic workflow engine.
The adoption adds no product/test source or compatibility layer. Full locked
metadata and affected native graphs pass. The authenticated exporter admits all
49 snapshot files; their payload is unchanged in 0.43.2. `make toko-tooling-update`
rebuilt/admitted the Testkit 0.106.0 CLI and checked the retained PocketIC server.
Types 0.20.4, Wire 0.22.1, Storage 0.74.1 and PocketIC 16.1.0 remain selected.
The earlier dependency-only checks excluded newer Platform patches; delivery
checks now target the actual final lock, including Platform 0.33.4/Runtime 0.22.5.

## Provider gate and next work

Caffeine calls are authorized **when ready**; do not request that permission again.
No contact/agreement exists; use available evidence. The refusal to post the
[prepared clarification](../design/caffeine-contract-review.md#prepared-provider-clarification--not-sent)
still applies: provider-call permission does not authorize outreach. The [D0 gate](../design/caffeine-contract-review.md)
remains open for independent Rust enrollment, exclusive deletion scope, byte bounds,
proof recovery/SDK resume and authority-lifetime/charge closure. A live witness needs
an identified fresh target/account, bounded scope and durable custody. Never import
legacy obligations or infer settlement from deletion permission, success or prefunding.
D1 remains partial: local integrity/catalog witnesses do not establish an integrated
provider certificate/upload/read journey. Do not repeat unchanged broad searches or
invent local mechanisms as provider evidence. Push only when requested, then perform
the required affected-scope architecture review.
The next provider-facing step needs supported contract evidence and a fresh target;
local test additions alone cannot advance that gate. The reference app now supplies
the selected local consumer. Next integration needs the selected SDK/provider
contract and an actual transfer/read journey; avoid adding speculative app modes.

## Post-push architecture review

The [retained review](https://github.com/dragginzgame/ic-blob-storage/blob/337cdd6301068093d8faa9a50b62f203a7d00c35/docs/status/current.md#post-push-architecture-review)
passed for 7832c4f against 4709158 using the pinned complexity method and overlay.
It covers endpoint retirement, private codecs and their installed/signed witnesses;
it does not cover the current unpushed reference application. Blob's existing library,
host and admin sources remain unchanged. After the next requested push, review
the new application state/async/lifecycle boundary and its measured growth without
implying a provider proof.

Legacy remains read-only at private `dragginzgame/ic-blob-storage-legacy`, revision
`25bed1352f3d2b9659f0d2583f50fa9af859ea84`. Design §12 records inspected owners/tests
and keep/replace decisions, now including endpoint bounds, codec traps and avoiding
its catalog/manual executor. No legacy code, credentials or custody were imported.
The sibling is absent; custody was not reverified. Read LEGACY.md before any separately
authorized recovery. Legacy obligations must never resume through this fresh checkout.
