# Current status — 2026-11-14

## Delivery and accepted scope

**0.100.4** is pushed to `origin/0.100` at
[102edcd](https://github.com/dragginzgame/ic-blob-storage/commit/102edcd5d00c5e34dc76bdf66acc2d228962bb6e),
with handoff/review at [4709158](https://github.com/dragginzgame/ic-blob-storage/commit/4709158fb9e0322e2313bd0ef2a7d20f4e97e281).
The endpoint migration for [issue #1](https://github.com/dragginzgame/ic-blob-storage/issues/1)
and signed application/uploader qualification are **local, uncommitted and unpushed**.
Version remains 0.100.4 with an Unreleased
changelog entry. Issue #1 is closed with [qualification evidence](https://github.com/dragginzgame/ic-blob-storage/issues/1#issuecomment-6880727720)
and explicit pending-delivery status. No tag, publication, live deployment or Caffeine call occurred.
The latest continuation reconciles stale design/README claims and consolidates
implementation history into current rules and a pinned legacy-reuse record.
Its [D0–D4 status table](../design/ic-blob-storage-design.md#9-delivery-and-acceptance)
states remaining evidence; no product behavior or qualification claim changed.
Pending-change review traced the explicit host hooks through the selected Platform
generator and compared the retired Endpoint helpers. No actionable defect found:
caller capture, pre-copy admission, trapping replies and storage ownership remain
intact. Test-only fixtures introduce no second service engine; the Wasm graph
still excludes native/test dependencies. This is a local source/graph review,
not the required post-push review or additional runtime qualification.
Private repo `dragginzgame/ic-blob-storage` (1457703306), branch `0.100`, HTTPS origin.
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

The host now mounts its three ordinary methods directly through Platform's generic
`guard`/`decode_with`/`encode_with` hooks and bounded Wire codecs. Its private codec
module checks ingress size/header before payload copying. Platform still owns the
single executor/reply; Service owns caller policy and typed `Error::Denied` replies.
Names, method IDs, limits, management-as-update, lifecycle and storage are preserved.
Both Wire Endpoint packages are removed; no replacement framework was introduced.
Standalone development remains independent. Canic owns its future Service adapter.
The application owns [catalog publication and lost-reply cleanup](../design/application-boundary.md);
publication does not establish private serving.

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
Platform owns transport/status. Every supported manager must preserve one shared
custody root. Alternate roots, controller bypass and arbitrary Makefiles are outside
this trusted arrangement. See [management rules](../design/ic-blob-storage-design.md#interrupted-management-operations).
Stop requires the exact drained attempt and running artifact/controller. Upgrade
requires the original stop acknowledgment and stopped state, binding uncompressed
replacement Wasm ≤1 MiB and fixed arguments. Same-Wasm upgrades still require the
original outcome. Start and activation require matching predecessor claims and
original acknowledgments; reopening verifies artifact/configuration/format/attempt.
Candidate compatibility needs operator review; no reinstall/rollback/restart fallback
or automatic compensation exists. No snapshot freshness proof exists; unrestricted
controllers can bypass the supported management arrangement.

## Endpoint migration evidence

`make test-endpoints test-standalone test-management` passed: raw-frame qualification,
standalone lifecycle and both native management tests. The frame test checks exact
responses, malformed/truncated/trailing/wrong-direction/wrong-method input, exact and
excess byte limits, denied callers and replicated management. It also passed against
the saved pre-migration Wasm. A separate qualification-only canister forces the
selected encoder to trap after a stable write and verifies rollback, with a successful
write as control. Production Blob has no fault mode or second engine.

The populated standalone witness passed with `BLOB_PREVIOUS_WASM` selecting the old
host and upgrades selecting the replacement. Its state, permissions, references,
accounting and closed admission survived. Saved predecessor SHA-256:
`a3ba5bd978a9330e516e0697cdb7a4882a97e1ba663fc2f6bf507dc019566397`.
The artifact is local build evidence at `target/endpoint-migration/before.wasm`,
not a committed release asset. Default standalone testing still upgrades the current
artifact. Native management also retains its distinct-hash populated-state witness;
that is same-format qualification, not arbitrary schema/code compatibility.

Strict selected native/Wasm Clippy, formatting, Rustdoc and documentation links
passed. The Wasm graph excludes Endpoint, Canic, Agent, HTTP, Testkit and Candid.
No full CI, live IC/provider or macOS
execution qualification is claimed. Current LOC is **2,702 product / 4,248 tests /
396 examples**, **+34 / +593 / +0** from 4709158. The new fixture/tests account for
test growth; production adds only private hooks. No external version changed.
Runtime/macros/guard 0.22.4, Types 0.20.4, Wire 0.22.1, Storage 0.74.1, Platform
0.33.3 (native), Testkit 0.105.0, Host FS 0.78.0 and PocketIC 16.1.0 remain selected.

`make test-signed` passed with strict Clippy against the production Wasm and owned
local HTTP gateway. Application, manager and two uploaders use distinct signing
identities. It checks caller separation, discarded registration reply then original
inspection from a fresh Agent, repeat-registration refusal, overlapping references,
cleanup and non-resurrecting replay. Queries verify signatures; updates verify
certified outcomes against the independent fixture root. Wrong-root inspection
specifically fails certificate verification. Latest formatting/links/diff checks pass.
This adds 289 test LOC, no product code or dependencies. The [application boundary](../design/application-boundary.md#evidence-and-acceptance)
records its legacy comparison and limits: native application signer, no application
canister forwarding/callback race, actual network fault or provider-proof recovery.

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
local test additions alone cannot advance that gate. Documentation links and diff
checks passed after the reconciliation; unchanged runtime suites were not rerun.

The [retained post-push review](https://github.com/dragginzgame/ic-blob-storage/blob/4709158fb9e0322e2313bd0ef2a7d20f4e97e281/docs/status/current.md#validation-and-retained-push-review)
passed for 102edcd against 2051ec2; it does not cover this unpushed migration.
Legacy remains read-only at private `dragginzgame/ic-blob-storage-legacy`, revision
`25bed1352f3d2b9659f0d2583f50fa9af859ea84`. Design §12 records inspected owners/tests
and keep/replace decisions, now including endpoint bounds, codec traps and avoiding
its catalog/manual executor. No legacy code, credentials or custody were imported.
The sibling is absent; custody was not reverified. Read LEGACY.md before any separately
authorized recovery. Legacy obligations must never resume through this fresh checkout.
