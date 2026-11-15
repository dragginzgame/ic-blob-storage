# Current status — 2026-11-15

## Delivery and pending work

Private repository `dragginzgame/ic-blob-storage`, branch `0.100`.
**0.100.6** is pushed at [decde20](https://github.com/dragginzgame/ic-blob-storage/commit/decde204acb6d7c8362a0003e904beef72ff89b6).
Four product crates now select the 0.100.7 delivery candidate; the separate qualification workspace's two
private packages remain 0.100.5. Publication is disabled; API/persistence remain
experimental. “Push” means a new patch GitHub checkpoint, not registry publication.
The current batch belongs to [0.100.7](../../CHANGELOG.md#01007---2026-11-15).
Delivery checks passed; push is pending and 0.100.8 will open afterward. No deployment
is part of this delivery. Earlier authorized
50T conversion and bounded Caffeine payer funding completed below; do not repeat.

Both workspaces select authenticated Toko Tooling **0.44.3**, the same 49-file
entrypoint closure, Platform **0.33.7** and Wire **0.22.3**. Runtime 0.22.5 and
Types 0.20.4 remain unchanged. Tooling preview/update, setup, both admissions and
locked/offline graphs passed. Default/zero-depth LOC totals agree, depths one/two
expand crates/modules, and reporter JSON matched the previous 0.44.1 output.
Earlier installed signed-call and maintenance tests passed for the Platform update.

## Principal inspection CLI

The crate is now named `ic-blob-storage-cli` and its executable is `blob`, replacing
`ic-blob-storage-admin`. `make blob` builds and installs the development executable
using Cargo's standard installer, normally at `~/.cargo/bin/blob` on PATH.
Repeating it updates the local source installation. `make build-cli` stays build-only;
the unit-test target remains `make test-cli`. Qualification selects the executable
through `BLOB_CLI`. There is one CLI with
no compatibility binary. Its commands include `overview`, `provider-inspect` and
`prepare-enrollment`; see [storage inspection](../operations/storage-inspection.md).
Crate-rename checks passed: CLI build/unit tests/help, strict CLI and qualification lint,
all three installed management/overview tests through the renamed Make wiring,
Rustdoc, formatting, package inventory and documentation links. Existing release
changelog entries retain their historical command names.
The subsequent `blob` executable rename passed `make blob test-cli`, CLI help,
single-binary Cargo metadata, formatting/link checks and the installed overview test
using the newly selected `blob` artifact. Maintenance flows were not rerun for this
binary-name-only change; their preceding crate-rename witnesses remain above.
PATH installation uses `cargo install --path` with locked/offline dependencies and
the existing debug profile, without force-overwriting another package's executable.
Linux installation and repeat replacement passed at `/home/adam/.cargo/bin/blob`;
the command resolves on PATH and runs outside this checkout. No shell configuration
was changed. Native macOS installation has not been exercised in this session.
Blob's application-only overview filters historical grants by uploader, groups
shared roots once and includes all application reference states for those roots.
The library derives accounting from existing records. The appended Wire variant
requires an updated host but changes no persistent record or state version.
Provider inspection reports one account's gauges and balance separately; it is
not a file inventory or an aggregate of a payer's funded owners. Its anonymous
live query to the funded payer returned `NotAuthorized`; no totals were obtained.
Use a provider-authorized existing PEM signer; no named-wallet/key export was added.
If Blob's application authority is a canister, inventory forwarding belongs to
that application's authorized API; this native CLI cannot impersonate it.

Enrollment preparation prints an offline proposal with explicit identities,
deposit address, debit/fee bound, daily limit and expiry. It performs no network
call, signature, payment or owner creation. No generic funding/recovery engine,
Canic dependency, new crate, persistent index or provider-effect path was added.
The tooling-only CLI dev dependency uses the baseline-supported `cfg(any())`
placement so it selects the payload without linking into behavior tests.

The 0.100.7 delivery candidate passed `make tools-check ci` (42 native tests) and
all eight installed cases across standalone, capacity, endpoints, signed calls,
reference application and management/overview targets. Tooling admission, strict
Clippy, Wasm builds, Rustdoc, formatting, inventory and documentation links passed;
`make blob` refreshed the PATH installation to `blob 0.100.7`. The actual CLI passed
an installed 64-upload/64-reference case with 63 distinct roots, denied/empty
views, untrusted-root refusal, unchanged stable memory and closing history after drained
stop/upgrade/start. This is Linux/local-IC evidence, not provider or native macOS qualification.
Product LOC is **3,617**, tests **5,506**, examples **396**: +412 product and +387
test LOC for this CLI slice. Legacy account/history source and regressions were
reviewed at `25bed135`; the runbook records retained lessons and exclusions.

## Provider research and retained effects

The maintainer directs independent research without Caffeine outreach and permits
the local `canic-mainnet` wallet for needed cycles. Caffeine calls are authorized
when concrete bounded operations and durable custody are ready. Do not ask again
merely to cross that boundary. No issue comment, email or other outreach is
permitted; never import or resume legacy objects, accounts or obligations.

[The contract review](../design/caffeine-contract-review.md#independent-qualification--2026-11-15)
pins DFINITY's independent Rust enrollment example at
`ef29e8a6e8063c6fe654cac53a3497cab585fefa`; its separate payer/owner path is the
bootstrap candidate. Its unguarded issuance, reopening and live-row deletion are
excluded. The deletion-list ABI, expiry enforcement and object closure remain open.
[The compact observation](../design/caffeine-observation.json) retains current
Cashier interface, gateway/pricing and account-route observations; raw captures
are under `target/evidence/caffeine/`. Query output is not a certified contract.

The authorized mint produced **50,000,000,009,024 cycles** for **30.60553552 ICP**
including fee. ICP transfer **40446894**, mint block **17206042**, original signed
requests/results remain in private Git-ignored `.operations/mint-50t-20261115/`.
Post-mint balances were 50.415079124210T cycles and 294.03351572 ICP. The configured
signer was used without exporting keys; minting itself paid no Caffeine funds.

The [payer funding witness](../design/caffeine-contract-review.md#fresh-payer-funding-witness--2026-11-15)
spent **10.1B cycles**, acknowledged **9.9B Cashier credit**, and left source balance
**50.404979124210T**. Ledger transfer **17206169**, notification block **17206177**,
exact signed requests/results and readback remain in
`.operations/caffeine-enrollment-01007/`. Preserve this funded account; do not
repeat either payment. The deposit address is empty. Settings already reported
zero overdraft/disabled refill and no relationships, so no setter was called.
No owner, relationship, upload authority or object exists from this rewrite yet.

Next provider operation: select a fresh owner/controller/subnet and qualify payment
binding, owner allocation and gateway credit. Of the
[1.0102T budget](../design/caffeine-contract-review.md#funding-preflight-and-authorized-mint--2026-11-15),
**1.0001T** remains unspent for owner creation/ledger fee. Payer credit is not upload
readiness. D0 stays open for enrollment, callback profile, object scope, enforced
bounds, original-proof recovery/lifetime and charge closure. D1 is partial; D2 has
only the payer-funding witness. Keep declared bytes and unresolved exposure.

## Ownership and lifecycle boundaries

Blob owns one Service, one host-granted versioned 64 KiB store and 64 historical
uploads/references. Reopen validates records and rebuilds accounting; missing or
foreign state never becomes genesis. Encoding/IO failure traps for IC rollback;
native callers discard the owner after panic. Final release retains unresolved
exposure. No provider authority or settlement API exists.

The thin Wire host remains independent of Canic. The reference app owns immutable
users/Blob target and sixteen lifetime drafts; cancellation precedes remote cleanup
and delayed replies cannot undo it. Publication is an application catalog decision,
not provider availability or read verification. See [the application boundary](../design/application-boundary.md).
Canic's optional adapter/catalog at `b36f6a39a67f487def523c847423fdfdb8d60c50` has
five retained installed witnesses naming Blob HEAD `50fe84b`; subsequent changes
are not relabelled as that run. No Canic source changed here. D3 remains partial:
immutable supplier adoption, explicit installation arguments and fleet custody
belong to Canic; its empty lifecycle arguments remain incompatible.

Supported maintenance keeps one custody root and original signed requests, drains
calls, upgrades and activates the exact attempt. Unknown status does not prove
nonexecution. Operator review establishes compatibility; unrestricted controllers
and alternate custody bypass this arrangement. No automatic compensation,
reinstall fallback or snapshot-continuity proof exists. The
[maintenance runbook](../operations/standalone-maintenance.md) owns the procedure.

The pushed 0.100.6 checkpoint passed the full native gate and seven installed cases;
its scoped post-push architecture review passed (`3c26f6c` to `decde20`). The next
push needs its own scoped sediment review. Legacy remains read-only at
`25bed1352f3d2b9659f0d2583f50fa9af859ea84`; no source, credential or recovery custody
was imported and no historical paid obligation was resumed or reverified.
