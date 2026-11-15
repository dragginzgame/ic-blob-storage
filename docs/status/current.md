# Current status — 2026-11-15

## Delivery and next batch

Private repository `dragginzgame/ic-blob-storage`, branch `0.100`.
**0.100.7** is pushed at [29a4920](https://github.com/dragginzgame/ic-blob-storage/commit/29a4920458049572f213a391174250022e66aa2e).
The four product crates and both locks select 0.100.7; the two private qualification
packages remain independently 0.100.5. Publication stays disabled. “Push” means a
new GitHub patch checkpoint, not crates.io publication. No live deployment was
part of this delivery. [0.100.8 is open](../../CHANGELOG.md#01008---unreleased) for
next work; product versions remain at the delivered checkpoint until its next push.

`make tools-check ci` passed with **42 native tests**, strict workspace Clippy,
Rustdoc, formatting and documentation links. All **eight installed cases** passed
across standalone, capacity, endpoints, signed calls, reference application and
management/overview targets. Logs are local under `target/release-01007-*.log`.
Rust 1.100.0, Tooling 0.44.3, Platform 0.33.7, Runtime 0.22.5, Types 0.20.4,
Wire 0.22.3 and Testkit 0.106.0/PocketIC 16.1.0 identify this Linux qualification.
No live-provider closure or native macOS installation claim follows from these tests.

The crate is `ic-blob-storage-cli`, the executable is `blob`. `make blob` builds and
installs through Cargo into its normal bin directory on PATH; repeat it to update.
`make build-cli` remains build-only; `make test-cli` checks CLI behavior.
`blob 0.100.7` is installed at `/home/adam/.cargo/bin/blob` and works outside this
checkout. Cargo owns installation/replacement without forced foreign-binary overwrite.
The snapshot still owns the same 49 files; no moving sibling tooling is executed.

## Current CLI and service boundaries

[Storage inspection](../operations/storage-inspection.md) documents `overview`,
`provider-inspect` and `prepare-enrollment`. Application-only overview groups roots
once across matching uploader grants and includes all application reference states.
It derives counters from existing state; there is no new ledger or persistent
index. Its appended Wire variant needs an updated host; durable format stays 1.
A canister application authority needs its own authorized forwarding path: the
native CLI cannot impersonate it. No reference-app or Canic forwarding API was added.

Provider inspection reports one account's gauges and balance, not individual files
or an aggregate of payer-funded owners. The anonymous live payer query was denied;
no storage totals were obtained. Use a provider-authorized existing PEM signer;
no named-wallet adapter or key export was added. Enrollment preparation only prints
an offline proposal with explicit account/address/fee/budget inputs. It cannot pay,
create an owner, sign authority or settle charges. Provider effects remain gated below.

Blob still owns one Service, a host-granted 64 KiB store and 64 historical
uploads/references. Reopen validates records and rebuilds accounting; missing or
foreign state never becomes genesis. Encoding/IO failure traps for IC rollback;
native callers discard the owner after panic. Final release retains unresolved
exposure. The reference app owns users, target and sixteen lifetime drafts;
cancellation precedes remote cleanup, and publication means its catalog decision.
Supported maintenance retains original signed requests and one custody root,
drains calls, upgrades and explicitly activates the same attempt. Unknown status
is not proof of nonexecution. Unrestricted controllers/alternate custody can bypass
this arrangement; no reinstall fallback or snapshot-continuity proof exists.
See [maintenance](../operations/standalone-maintenance.md) and [application ownership](../design/application-boundary.md).

## Provider continuation for 0.100.8

Next: select a fresh owner/controller/subnet and qualify payment binding, owner
allocation and gateway credit using the already-funded payer. The maintainer
permits Caffeine calls and the local `canic-mainnet` wallet when concrete bounded
operations and durable effect custody are ready; do not ask again merely to cross
that boundary. No Caffeine issue/email outreach is authorized. Never resume legacy
objects, accounts or obligations, and never export the wallet key for convenience.

[The contract review](../design/caffeine-contract-review.md#independent-qualification--2026-11-15)
pins DFINITY's independent Rust example at `ef29e8a6e8063c6fe654cac53a3497cab585fefa`.
Its separate payer/owner path is the bootstrap candidate, excluding its unguarded
issuance, reopening and live-row deletion. The [compact observation](../design/caffeine-observation.json)
retains source identities, public-interface reads and acknowledged funding results.
D0 stays open for enrollment, callback profile, object scope, enforced bounds,
original-proof recovery/lifetime and charge closure. D1 remains partial; D2 has
only the payer-funding witness. Keep declared bytes and unresolved exposure.

Completed mint: **50,000,000,009,024 cycles** for **30.60553552 ICP** including fee;
ICP transfer **40446894**, mint block **17206042**. Post-mint balances were
50.415079124210T cycles and 294.03351572 ICP. Original requests/results remain in
private Git-ignored `.operations/mint-50t-20261115/`. **Do not repeat.**
Completed payer funding: **10.1B debit**, **9.9B Cashier credit**; ledger transfer
**17206169**, notification block **17206177**, source balance afterward
**50.404979124210T**. Preserve `.operations/caffeine-enrollment-01007/`; do not
repeat either payment. Deposit address is empty; zero overdraft/disabled refill
and no relationships were reported without a setter. Of the approved **1.0102T**
budget, **1.0001T remains unspent** for owner creation/ledger fee. No owner,
relationship, certificate or object exists from this rewrite yet. Payer credit
is not owner/gateway capacity. Both custody directories remain excluded from Git.

Canic's optional adapter/catalog at `b36f6a39a67f487def523c847423fdfdb8d60c50` has
five retained installed witnesses naming Blob HEAD `50fe84b`; newer supplier
inputs are not relabelled as that run. D3 remains partial: immutable supplier
adoption, explicit installation inputs and fleet maintenance belong to Canic;
empty lifecycle arguments remain incompatible. No Canic source changed here.
Legacy remains read-only at `25bed1352f3d2b9659f0d2583f50fa9af859ea84`; relevant
account/history source and regressions informed this CLI slice, without importing
source, credentials or recovery custody. No historical paid obligation was resumed.

## Post-push architecture review

Scoped **PASS**, `50fe84b` to pushed `29a4920`, using the committed AGENTS.md overlay
and pinned [complexity method](https://github.com/dragginzgame/toko-tooling/blob/626da7cd9626cc2e152594862a2d82ea82ef51da/audits/complexity-and-technical-debt.md)
(source inspection plus the release evidence above). Reviewed owners are the local
model/Wire projection, CLI inspection/proposal/installation, native maintenance
callers and adopted tooling. This is not a full provider or Canic audit.

Added axes are uploader-filtered inventory, one Cashier-account query and offline
enrollment inputs. Application permission and deduplication stay in LocalModel;
Service/Wire/host carry its result and the CLI renders it. Provider identity/shape
checks stay in the private adapter, transport in Platform, installation in Cargo.
Invalid authority, returned account, reply shape or deposit bound refuses. No
new durable phase, policy owner, fallback, funding engine or duplicate CLI remains.
Existing maintenance custody was mechanically moved and retains its own authority.

Two rehearsals by source inspection: an inventory-field addition touches its
owner/DTO and CLI display; a Cashier gauge change touches the provider adapter and
fixtures. Neither needs host policy or a new store. Keeping those separate adapters
is justified by their different authorities/protocols; no actionable sediment was found.
Product LOC **3,617** (+412), tests **5,506** (+387), examples **396** (unchanged),
against the retained 0.100.6 baseline. Rust tooling LOC remains zero; the same
49-file snapshot updates two managed LOC-reporting scripts. Growth serves the
requested inventory and its rejection/full-history/upgrade evidence, not a new
framework. This delivery-only handoff and next-version header add no runtime modes;
review their actual pushed diff separately before finishing delivery.
