# Extraction readiness — B1 working evidence

Date: 2026-09-25. Status: planning in progress; B1 is not frozen.

This review starts the work authorized in `ic-blob-storage`. Canic was read
only. In this session the maintainer confirmed the pending 0.110 acceptance
question and directed work to continue in the order judged appropriate.
That acceptance is recorded here without changing Canic's handoff. B1 still
must freeze before B2; publication, deployment, application changes and
installation reset remain outside this work.

## Source checkpoint and limits

Reviewed Canic HEAD: `10d00c6d9494a45b30e66f84d1bd886c8acdd45c`.
Its tracked worktree was clean at the inventory checkpoint. The coordination
authority is its
[0.111 design](../../canic/docs/design/0.111-standalone-blob-service-extraction/0.111-design.md)
and [tracker](../../canic/docs/design/0.111-standalone-blob-service-extraction/status.md).
Both recorded human 0.110 acceptance and B1 completion as implementation gates;
the session confirmation above resolves the acceptance gate for work here.
The GitHub organization and repository are now established:
`dragginzgame/ic-blob-storage`. Accountable people and package publication
ownership are still unassigned; organization ownership does not assign them.

[The source inventory](canic-source-inventory.tsv) records paths and SHA-256
hashes for tracked matches to `blob.storage|immutableObjectStorage|caffeine|cashier`
(case-insensitive) in Canic's crates, canisters, scripts, `.github`, root
manifests/Makefile/README and current feature, operation and contract docs.
It also includes the design, tracker and reviewed fixture build scripts.
This is a reproducible discovery baseline, not a claim that every match is
removable. Shared dependencies, historical measurements and generic coverage
need owner review. Historical design/audit/changelog prose is outside this
search and must remain historical evidence. Refresh the inventory before B3.

The provider inventories describe earlier consumer-side protocol evidence.
They are not refreshed provider implementation or deployed behavior evidence.
The maintainer now requires the latest official Caffeine integration. Use the
[verified upstream baseline](provider-baseline.json) as the integration target;
Canic's source informs removal and behavior classification, not provider types.
The subsequent [provider review](provider-review.md) adds pinned official
integration source and an isolated client hashing check. It finds interface
drift, not a qualified server contract. No live provider calls, paid effects
or service builds were performed.

## Proposed package boundary

These names are a proposal for the contract freeze, not reserved registry names:

| Package | Responsibility and dependency boundary |
| --- | --- |
| `ic-blob-storage` | Existing core scaffold; model, policy, ops and workflows; no Canic dependency or exported canister endpoints |
| `ic-blob-storage-protocol` | Passive service identities, requests, responses and typed errors; canonical service Candid |
| `ic-blob-storage-client` | Checkpointed upload/read, content hashing and byte verification against the service protocol |
| `ic-blob-storage-canister` | Standalone endpoint/lifecycle entrypoint delegating to the core |
| `ic-blob-storage-canic` | Thin managed entrypoint/lifecycle adapter owned here; sole proposed production package depending on Canic |
| Operator client/CLI (name pending) | Service-owned status, sync, funding, dry-run and diagnostic workflows for both deployments; service retains policy authority |

Keep provider request/callback definitions under one service-owned module;
clients use the service contract and must not duplicate provider authority.
Do not add a backend plugin framework. Fix package artifact/Candid provenance
and registry access before publication; `publish=false` remains in force.

The maintainer's full-functionality requirement is expanded in the
[Canic parity contract](canic-parity.md), including operator and diagnostic
capabilities. Canic removal depends on working replacements and their executed
evidence, not just the existence of packages or this inventory.

## Behavior classification

Paths in this section are relative to Canic. Preserving an intent does not
require preserving its API, unsafe semantics or storage layout.

| Class | Source-backed behavior or required correction | Evidence and planned acceptance |
| --- | --- | --- |
| Preserve | Canonical Caffeine root parsing, 32-byte conversion and malformed-input rejection | `crates/canic-core/src/model/blob_storage/hash.rs`, `ops/blob_storage/conversion.rs`; A02 |
| Preserve | Idempotent local registration and pending-delete marking; registered gateways control deletion callbacks | `crates/canic-core/src/ops/blob_storage/lifecycle.rs`, `api/blob_storage/gateway.rs`; A01, A06 |
| Preserve | Explicit Cashier configuration, bounded funding against project reserve, gateway-list validation and read-only readiness | `crates/canic-core/src/workflow/blob_storage/billing/mod.rs`, `domain/policy/pure/blob_storage/mod.rs`, `ops/cashier/conversion.rs`; A04, A08 |
| Correct | A provider root is not proof of a portable raw-byte digest or successful storage. Certificate creation currently registers local liveness before any provider completion observation | `crates/canic-core/src/api/blob_storage/lifecycle.rs`; A02, A03 |
| Correct | Root-keyed records have no tenant/reference/actor binding. A host certificate guard cannot establish the new shared service's tenant rules | `crates/canic-core/src/storage/stable/blob_storage.rs`, `crates/canic/src/macros/endpoints/blob_storage.rs`; A01 |
| Correct | Fixed record-size bounds do not bound total objects, references, sessions, bytes, receipts or outstanding costs | `crates/canic-core/src/storage/stable/blob_storage.rs`; A04, A06 |
| Correct | Funding single-flight is transient and intentionally resets on upgrade. The funding workflow supplies no durable operation identity to its Cashier request | `crates/canic-core/src/ops/blob_storage/funding.rs`, `workflow/blob_storage/billing/mod.rs`; A03, A05, A08 |
| Correct | A gateway confirmation removes live and pending records, including live-only entries. It does not separately record billing cessation or protect another tenant's live reference | `crates/canic-core/src/ops/blob_storage/lifecycle.rs`; A06, A08 |
| Correct | Stable-state upgrade coverage does not prove older-backup identity non-reuse, stale-instance exclusion or reconciliation of missing liabilities | `crates/canic-tests/tests/pic_blob_storage.rs`; A05 |
| Defer | Shared cross-tenant deduplication, generic provider plugins, cross-release migration, multi-Fleet indexing and new confidentiality guarantees | No demonstrated requirement for the bounded journey; separate scope decision required |

Acceptance IDs refer to [the acceptance plan](acceptance-plan.md). Each is a
planned obligation, not an existing passing test. Consumer resumable upload,
verified read and a durable release outbox belong to the required journey;
the inspected Canic API is not an implementation of that entire journey.

## Provider evidence gap matrix

Historical references:
[gateway inventory](../../canic/docs/contracts/BLOB_STORAGE_INVENTORY.md),
[Cashier inventory](../../canic/docs/contracts/BLOB_STORAGE_CASHIER_INVENTORY.md),
and Canic's `crates/canic/tests/fixtures/blob_storage_{gateway,cashier}.did`.
Their earlier accepted Toko source is commit
`9ca150b396a2bde42f2b8977a04a7ca2c6172b56`; that application was not re-inspected
or selected as this service's consumer here.

| Operation | What the reviewed source establishes | Missing B1 evidence |
| --- | --- | --- |
| Upload/create and resume | The application-side certificate callback returns an upload method and root; local registration is idempotent while live | Exact deployed upload interface, request identity, paid-effect timing, completion lookup, retry/restore retention, chunk/hash rules and namespace exclusivity |
| Read and verify | Canic parses a Caffeine root; this is not a byte-hashing implementation | Actual provider read interface, byte/digest vectors, MIME and active-content isolation, serving/access policy and corruption behavior |
| Delete | An authorized gateway reports roots to remove from local state | Provider-side operation identity, authoritative object absence, replay/stale-callback behavior, evidence retention and bounded unresolved disposition |
| Billing cessation | Cashier balance responses expose account totals | Object/operation-specific final charges, independent proof that billing stopped, settlement/return authority and evidence horizon |
| Cashier top-up | `account_top_up_v1` takes optional account/target balance plus attached cycles; the captured request has no operation-ID field | Lost-response reconciliation bound to the exact transfer, authoritative retained completion, a safe non-repeat disposition, and provider-source/deployed confirmation |
| Gateway synchronization | Cashier returns principals; local code validates before replacing the set | Deployed authority/version provenance, namespace binding and revocation behavior for in-flight callbacks |

**Current suitability verdict: not established.** This does not prove that
Caffeine cannot support the contract. It means the inspected evidence cannot
qualify it. Account balance changes alone do not identify one uncertain top-up.
Local receipts, timeouts or test substitutes cannot supply missing provider
guarantees. For each required operation obtain exact source/deployed provenance,
retention bounds and observed evidence; then select a supported contract or
reject the provider before B2. No numeric restore horizon is justified yet.

## Canic removal inventory and coverage transfer

All removal remains Canic-owned B3 work. The TSV expands the following groups
into concrete files; matches in shared files need selective edits, not deletion
of the whole file.

| Surface | Reviewed ownership and B3 disposition |
| --- | --- |
| Runtime/core | Remove blob API, domain, DTO, model, policy, ops, Cashier, view, workflow and stable modules; update each module registration and protocol constants |
| Stable allocations | Remove IDs 55 (roots), 56 (pending deletions), 57 (gateway principals), 58 (billing) from `role_contract/allocation.rs`, plus catalog/state-contract ownership; no old-state reader |
| Runtime inspection | Remove `RuntimeBlobStorageStatusSummary` and blob feature reporting from runtime DTO/API; propagate current runtime Candid/consumer expectations |
| Facade | Remove `blob-storage`/`blob-storage-billing`, endpoint macros, API/protocol re-exports and blob Candid fixtures; retain generic endpoint-generation coverage |
| Host/CLI | Remove `canic-cli/src/blob_storage`, Medic blob diagnostics, command/help/inspect wiring and blob-specific assertions in host package/descriptor/state-manifest tests |
| Fixtures/testing | Retire `blob_storage_probe`, `blob_storage_cashier_mock`, `pic_blob_storage` and dedicated macro tests; transfer generic lifecycle, guards, role allocation and endpoint coverage before removal |
| Build/tooling | Reconcile workspace/features/lockfile, Make targets, installed-CLI proofs, protocol gates, serial test inventory/runner and artifact measurement lists; do not rewrite historical measured evidence |
| Maintained docs | Update feature/operation docs and crate/root READMEs; keep historical protocol provenance distinct from the extracted service's current contract |

`pic_blob_storage.rs` currently covers gateway authorization/lifecycle,
mock-Cashier wrappers, readiness blockers and stable state across upgrades.
The substitute demonstrates the modeled callback/billing contract only.
Map its generic controller guard, synchronous restore and Component allocation
coverage to a generic maintained fixture in Canic; map blob behavior to A01,
A03, A04, A06 and A07 here. Exact replacement test ownership is open until
the Canic runtime/host/testing owners are named and qualify that mapping.

## Installation obligations

No installation inventory or no-obligation evidence was supplied. The local
service scaffold's lack of provider state says nothing about existing Canic
installations. Do not record an empty inventory as proof of no liabilities.

For each affected installation the operator must supply:

- Network, canister/service identity, provider namespace/account, operator and
  authorized reconciliation/settlement principal.
- Exported object roots, pending deletions, uploads and uncertain paid effects,
  linked to provider records; retain record provenance and collection time.
- Cashier/provider balances, charges, billing status and funding responsibility.
- Evidence location surviving any reset, stop-admission fence and reconciliation
  status, deletion proof, independent billing-stop proof and balance disposition.
- Final no-obligation or completed decommission decision; any reviewed residual
  disposition must preserve records, authority and funded ownership elsewhere.

Source removal closes no installation obligation. Unknown outcomes stay fenced.
No reset is authorized by this document.

## Remaining decisions before implementation

1. Assign accountable service, consumer, operator and Canic integration owners.
   Human 0.110 acceptance was confirmed in this session; it does not assign
   these owners or accept B1's unresolved provider contract.
2. Name the consumer and its actual serving needs; freeze maximum object/chunk
   sizes, tenant/global byte and count limits, concurrency, receipt retention
   and supported interruption/restore horizons.
3. Choose deduplication and charging rules. A conservative proposal is no shared
   cross-tenant deduplication, separate logical quotas and physical/liability
   counters, and reservations before effects. Exact transitions remain open.
4. Complete actual-provider evidence, including recovery identity authority
   surviving restore and proof required to release each fence. Restrict restore
   support explicitly if sufficient external evidence is unavailable.
5. Review package/release ownership, removal coverage and each installation's
   obligations; assign the acceptance plan to named owners and freeze B1.

After those gates close, start B2 with protocol/model/pure-policy invariants
and native tests, then persisted intent/accounting and workflows, then the
shared PocketIC journey through both thin adapters. Do not publish a package
or claim provider support from that ordering alone.
