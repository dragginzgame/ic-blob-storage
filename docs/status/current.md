# Current status — 2026-11-15

## Delivery and accepted scope

Private repository `dragginzgame/ic-blob-storage`, branch `0.100`, HTTPS origin.
**0.100.6** is pushed at [decde20](https://github.com/dragginzgame/ic-blob-storage/commit/decde204acb6d7c8362a0003e904beef72ff89b6); the remote branch was verified.
The preceding checkpoint and review are retained at
[3c26f6c](https://github.com/dragginzgame/ic-blob-storage/commit/3c26f6c5c9bd0a32e0ad1a4c7d644f056c6c5e46).
“Push” means the next patch GitHub checkpoint; delivery-only review commits share
that checkpoint. No tag, registry publication, deployment or Caffeine call is part
of this delivery. Publication remains disabled; API and persistence are experimental.
Four product crates inherit 0.100.6. The separate qualification workspace retains
its two private package versions at 0.100.5 and consumes the current local products.
Both workspaces own their lockfiles and admit the same installed tooling payload.

Rust 1.100.0 / edition 2024; Toko Tooling 0.43.2; Host FS/Artifacts/Process 0.79.0;
Testkit 0.106.0, PocketIC 16.1.0, Platform 0.33.6, Runtime 0.22.5, Types 0.20.4,
Storage 0.74.1, Wire 0.22.2. Product thiserror is 2.0.21; qualification retains 2.0.18.
Existing supplier selections were preserved; release preparation changed only
owned package versions in both locks. Managed tooling bytes are unchanged.

## Checkpoint contents and evidence

The [changelog](../../CHANGELOG.md#01006---2026-11-15) owns the complete release notes.
The application cancellation fix accepts an exact never-admitted authorization's
expiry refusal as conclusive cleanup; admitted grants still require release.
[The boundary contract](../design/application-boundary.md#application-decisions-across-delayed-replies)
records the retained-ID lookup and monotonic-time assumptions. Installed cases
cover pending callbacks/upgrades, malformed endpoints, immutable arguments,
expired drafts and missing/invalid/foreign state on disposable recipients.
The [maintenance runbook](../operations/standalone-maintenance.md) replaces the
long README command reference with explicit inputs, sequence and interruption rules.

Final 0.100.6 Linux qualification used the actual product Wasm/admin binaries:

- `make tools-check ci`: tooling admission, formatting, strict native Clippy for
  both workspaces, 35 library tests, four example tests, walkthrough and Rustdoc pass.
- `make test-standalone test-capacity test-endpoints test-signed test-application test-management`:
  all seven installed tests pass, with strict Wasm/native Clippy. Suite wall times
  were 7.03 / 10.47 / 6.46 / 9.88 / 7.73 / 60.20 seconds respectively.
- Dependency inventory distinguishes four product and two qualification packages;
  admin reports 0.100.6. Formatting, local documentation links and diff checks pass.

[Capacity results and raw samples](../design/standalone-capacity.md) retain their
November 14 source/artifact identities, not relabelled as release measurements.
The checkpoint also reran full-history inspection, cleanup and same-artifact
upgrade. Both kinds of evidence concern finite history, not sustained-use/D4,
raw instruction counts, mainnet prices or provider charges. Cleanup retains slots.
Linux evidence does not qualify macOS, arbitrary migrations or snapshot continuity.

LOC: **3,205 product / 5,119 tests / 396 examples**; versus 0.100.5,
**+6 product / +558 tests / +0 examples** under the same reporter categories.
Growth is the expiry fix plus capacity/application qualification. The workspace
split adds no Rust LOC, runtime owner or product package.

## Post-push architecture review

Scoped **PASS**, comparing `3c26f6c` to pushed `decde20`, under its `AGENTS.md`
and the pinned [complexity method](https://github.com/dragginzgame/toko-tooling/blob/626da7cd9626cc2e152594862a2d82ea82ef51da/audits/complexity-and-technical-debt.md).
Source inspection covered cancellation/authorization, qualification ownership,
Make callers and documentation, using the checkpoint evidence above. No new
actionable sediment was found; this is not a full-system or provider audit.

Application phases, Blob grant states, maintenance attempts and authority roles
are unchanged. Blob alone determines whether the original ID was admitted; the
application consumes its typed expiry refusal and retains its existing Cleaned
transition. Unknown outcomes remain Cancelled; no second ledger or retry route.
Qualification adds one measured-capacity test and application boundary cases,
not a new product mode. The old package paths are removed; shared tooling still
owns formatting, dependency discovery and simulator installation.

Two source rehearsals preserve ownership: Canic composition consumes Service and
host-granted memory without changing Blob policy; a future provider contract must
revisit admission/closure at Blob before broadening application cleanup. Neither
requires speculative code now. Finite history and retained custody are intentional
constraints pending their existing qualification gates, not unused compatibility
layers. Canic's implementation, live provider behavior and general snapshot recovery
remain outside this review. The delivery follow-up changes only this handoff.

## Current implementation and operating boundaries

Blob owns one Service and a host-granted, versioned 64 KiB Storage record with
64 historical uploads/references. Reopen validates records and rebuilds indexes
and declared reserved/live/unresolved accounting; missing/foreign state never
becomes genesis. Encoding/IO failure traps for IC rollback; native callers discard
the owner after panic. Final release retains unresolved exposure, not a cost bound.

The thin host uses Platform guard/codec hooks and Wire, independently of Canic.
The reference application owns immutable users/Blob target and sixteen lifetime
drafts; Published drafts derive its per-user catalog. Cancellation precedes remote
cleanup and cannot be undone by delayed replies. Publication is a catalog decision,
not independent read verification. There is no provider authority or settlement API.

App maintenance uses trusted stop/drain/upgrade/start; Blob's admin does not manage
it. Blob drain preserves inspection, cleanup and historical replay. Reopen remains
drained until activation of the exact attempt. The native admin retains original
signed requests and observes without renewal; unknown status cannot prove
nonexecution. All supported managers preserve one custody root and predecessor
claims. Host FS owns durability/locking; Platform owns transport/status. Operator
review establishes candidate compatibility. Alternate custody, arbitrary checkout
code and unrestricted controllers bypass the supported arrangement; no automatic
compensation, reinstall/rollback fallback or snapshot-freshness proof exists.

## Provider gate and next work

The user has started Canic integration in its own session. The
[adapter handoff](../design/application-boundary.md#canic-adapter-handoff) maps the
existing Service API to an embedded catalog witness. No Canic source is changed
by this checkpoint; implementation and qualification belong with that owner.
Standalone use remains independent. Do not add another adapter or ledger here.

Caffeine calls are authorized **when ready**; do not ask again. No contact/agreement
exists: use available evidence. The refusal to post the [prepared clarification](../design/caffeine-contract-review.md#prepared-provider-clarification--not-sent)
still applies; provider-call permission does not authorize outreach. D0 remains
open for independent Rust enrollment, deletion scope, byte bounds, proof recovery,
SDK resume and authority-lifetime/charge closure. A fresh live run needs an identified
account/target, bounded scope and durable obligation custody. D1 remains partial;
more local scaffolding cannot supply the missing provider contract. No new provider
research or calls were made during release preparation.

Legacy is read-only at `25bed1352f3d2b9659f0d2583f50fa9af859ea84` in private
`dragginzgame/ic-blob-storage-legacy`; the sibling is absent. Targeted source reviews
are cited in the design/capacity evidence. No credentials, recovery records or
provider obligations were imported. Never resume legacy obligations here; read its
LEGACY.md before separately authorized recovery. Legacy custody was not reverified.
