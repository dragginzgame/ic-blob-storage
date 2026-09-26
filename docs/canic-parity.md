# Canic functionality replacement and removal gate

The maintainer requires Canic's blob functionality to be ready here before
removing it from Canic. This includes operator workflows and diagnostics as
well as storage and billing. The first native core primitives are implemented;
no end-to-end service capability is qualified here yet. Installed dependencies,
plans and source inventories do not establish parity.

The maintainer explicitly requires a fresh model review: Canic identifies existing
capabilities, not design decisions to preserve automatically. The
[lifecycle design](service-contract.md#lifecycle-design-under-independent-review)
records the rationale for reference ownership and separate release obligations.

[The capability inventory](canic-capabilities.json) maps every public method in
Canic's blob API and every emitted blob endpoint at commit
`10d00c6d9494a45b30e66f84d1bd886c8acdd45c`. It also identifies the CLI, diagnostic,
lifecycle and declaration surfaces, with concrete source-test references.
Some references are assertion helpers inside larger tests. They identify
behavior to reproduce, not checks run or passed in this repository.
Source file hashes are retained in [the source inventory](canic-source-inventory.tsv).

## Source checkpoint

The inventory captures Canic commit
`10d00c6d9494a45b30e66f84d1bd886c8acdd45c`, whose tracked worktree was clean.
Coordination follows Canic's
[0.111 design](../../canic/docs/design/0.111-standalone-blob-service-extraction/0.111-design.md)
and [tracker](../../canic/docs/design/0.111-standalone-blob-service-extraction/status.md).

The TSV records SHA-256 hashes for tracked, case-insensitive matches to
`blob.storage|immutableObjectStorage|caffeine|cashier` in crates, canisters,
scripts, `.github`, root manifests/Makefile/README and maintained feature,
operation and contract docs, plus the design, tracker and reviewed fixture build
scripts. This is a discovery baseline, not a list of files to delete wholesale.
Refresh it before removal; shared dependencies and generic coverage need review,
and historical measurements/design/audit/changelog evidence stays historical.

Earlier provider protocol evidence lives in Canic's
[gateway inventory](../../canic/docs/contracts/BLOB_STORAGE_INVENTORY.md),
[Cashier inventory](../../canic/docs/contracts/BLOB_STORAGE_CASHIER_INVENTORY.md)
and `crates/canic/tests/fixtures/blob_storage_{gateway,cashier}.did`.
Their Toko checkpoint `9ca150b396a2bde42f2b8977a04a7ca2c6172b56` is historical;
the provider review records a newer Toko source inspection for deployment
locators, without selecting that application as this service's consumer.
Canic informs extraction behavior; the [provider review](provider-review.md)
owns the current integration target and deployment evidence gaps.

## Required replacements

BLOB-01 has native parsing/canonicalization and incremental raw-byte verification
evidence. BLOB-03 has bounded binary-root batch parsing that preserves order,
duplicates and per-entry errors. Local consumer-reference reads now check the
tenant and complete binding, distinguish released references from live siblings,
and bound ordered batch results. Provider-facing gateway liveness responses,
persisted state access and endpoints remain unimplemented.
BLOB-04/05/06 now have partial native lifecycle-transition evidence for confirmed
objects, including bounded reference receipts and separate logical/physical/
economic projections. Authenticated callbacks and persisted counters remain absent.
Lifecycle mutations now check complete object/reference bindings; pure direct-tenant
policy adds partial A01/BLOB-14 evidence. Endpoint actor authentication, delegation
and durable scope enforcement remain outstanding.
Local reference request receipts add exact actor/payload replay checks and reserve
receipt capacity for each active reference's release. These are native bounds and
replay semantics, not durable request recovery or provider retry evidence.
The transient catalog now owns multiple confirmed objects and their journals,
with tenant logical quota, global physical/billing-byte limits and derived usage
counters. Bounded pending-deletion scans and current-gateway read checks add local
BLOB-03/04/05/06 evidence. Root observations preserve unknown/malformed/foreign
namespace statuses; the provider boolean mapping remains unspecified. No persisted
lookup, durable upload reservation or provider operation is implemented here.
BLOB-08, BLOB-11 and
BLOB-12 have partial numeric validation and pure-policy evidence. BLOB-09 and
BLOB-15 now have bounded signed-balance conversion and operator amount parsing
evidence. BLOB-09 validates all four numeric balance components before use,
preserving typed field failures without defining a provider DTO.
The billing input port also extends BLOB-08/11, recorded in
[core evidence](evidence/core-primitives.md) and the capability inventory.
BLOB-07/10 have partial transient gateway-list validation/replacement evidence,
including raw/distinct bounds and unchanged membership after rejected input.
BLOB-07 now also has idempotent individual add/remove and empty-membership
behavior; empty provider sync input remains rejected.
The scoped gateway registry adds one pending local sync identity, invalidation
on operator edits and rejection of stale/cancelled/replayed results. Native
callback-policy composition checks service/namespace and current membership,
including continued denial after revocation and a delayed sync result. This is
partial BLOB-05/07/10/14 coverage, not transport or restart/restore evidence.
Bounded Cashier gateway-list Candid decoding now feeds the same registry: scope
and stale attempts reject before parsing; invalid replies leave membership and
the pending attempt unchanged. This extends local BLOB-10 evidence only.
Account-balance reply decoding adds local BLOB-09/12 coverage for requested-account
checks, structured failures, bounded full-balance conversion and readiness
composition. It performs no query and does not prove source or freshness.
BLOB-08 also has complete local configuration-candidate validation for Cashier
principal, funding thresholds and gateway limits representable on 32-bit Wasm.
BLOB-11 also has pure new-intent admission rejecting recovery fences and
outstanding/uncertain funding activity; durable exclusion is still unimplemented.
BLOB-02 now also has bounded chunk-status decoding and lifetime root claims;
BLOB-05 has native delayed-root-correlation rejection after settlement. BLOB-11
has bounded Candid result decoding that retains structured provider failures.
These add local safeguards, not persisted upload/callback/payment workflows.
Provider transports, configuration persistence, actual funding, status workflows
and the other capabilities remain unimplemented. The boundaries below describe
the complete replacement requirements, not qualification claims.

| Capability | Required behavior here | Necessary correction or boundary |
| --- | --- | --- |
| BLOB-01: identities | Canonical provider root parsing and 32-byte conversion with typed malformed-input rejection | Keep provider identity distinct from a raw-content digest; verify current upstream vectors |
| BLOB-02: registration/certificates | Authorized registration and certificate handling, idempotent replay and unchanged state on rejection | Bind tenant/actor/service/operation; local registration alone cannot prove durable upload |
| BLOB-03: liveness | Batched liveness with defined malformed/order/duplicate behavior, and consumer reference validation | Validate ownership and confirmed state; no tenant authority from a root hash |
| BLOB-04: release | Authorized logical release, retryable deletion work and bounded enumeration | Do not erase references, physical capacity or costs before their own release conditions |
| BLOB-05: deletion callback | Exact gateway authorization, revocation and idempotent confirmation handling | Reject stale/unrelated confirmations; keep billing-stop evidence separate; use current upstream wire types |
| BLOB-06: counters | Stored objects, pending deletion and gateway counts available to authorized diagnostics | Add bounded references, bytes, reservations and liabilities with atomic accounting |
| BLOB-07: gateway administration | Authorized add, replace, revoke and membership inspection | Bind provider namespace; reject malformed/oversized authority sets before mutation |
| BLOB-08: billing configuration | Validated provider identity, reserve/threshold/limit configuration, readback and restart persistence | Invalid values must leave the previous configuration intact; service owns policy |
| BLOB-09: balances | Typed provider balance observations; distinguish absence, malformed values and unavailable observations | Verify the current deployed balance interface; account totals are not transfer-completion evidence |
| BLOB-10: gateway sync | Explicit synchronization, validation/deduplication, state preservation on failure and successful-sync timestamp | Single current provider contract; no sync caused by status reads |
| BLOB-11: funding | Explicit funding, reserve protection, no partial funding when the requested amount cannot be admitted, and observable results | Replace transient-only single-flight with persisted intent, exact identity and uncertain-effect reconciliation |
| BLOB-12: readiness | Configuration, gateways, balance, reserve, blockers and warnings reported without mutation | Include recovery fences; operator status cannot bypass service/provider uncertainty |
| BLOB-13: lifecycle | Same-release restoration of configuration, gateway state, references and pending work | Add supported backup/restore fencing; restoration happens before deferred effects |
| BLOB-14: composition | Explicit endpoint selection, separate actor/operator/provider guards and shared handlers in both adapters | Linking a library exports no application endpoints; a controller is not automatically a tenant |
| BLOB-15: operator commands | Status/check-ready, sync, funding, dry-run previews, structured output/errors and post-action diagnostics | Service-owned operator client; dry runs cause no effects and post-status failure cannot trigger mutation replay |
| BLOB-16: targeting | Exact target/network/identity selection, method existence/mode checks, rejection of missing or ambiguous managed targets | Standalone operation must not require a Fleet; managed target discovery uses generic Canic integration |
| BLOB-17: diagnostics | Passive local capability discovery and explicit selected-target ready/warning/blocked checks with useful next actions | Passive checks make no provider calls; diagnostic paths never sync or fund |
| BLOB-18: declarations | Own service Candid, package/features, state allocation and operational metadata | Canic retains generic Component/lifecycle behavior; remove blob-specific declarations there only in B3 |

“All functionality” means each capability has a working, tested replacement.
It does not require old Rust method names, obsolete provider DTOs, controller-as-
tenant authority, unbounded scans or unsafe callback/funding semantics. The
current [upstream baseline](provider-baseline.json) owns provider compatibility.
Raw Cashier helpers belong to one internal provider owner; they must not become
public arbitrary-provider/payment entrypoints in the new service.

The operator client/CLI is an explicit extraction deliverable owned here. Its
final package name is a B1 decision. It must support the standalone service and
managed deployment using the same service API. Preserve automation behavior:
strict decimal cycle input, typed JSON errors, readiness failure signaling
(Canic currently uses exit 4), effect-free dry runs, and post-action observations.
Do not recreate billing policy in the CLI. A managed integration may discover
targets through generic Canic facilities without adding a blob production
dependency back into Canic.

## Evidence limitations found during the inventory

Canic's `crates/canic-cli/src/blob_storage/tests.rs` contains
`scripted_operator_loop_proves_status_sync_fund_and_recheck_sequence`, which
uses scripted responses. Its
`scripts/ci/blob-storage-cli-proof-lib.sh` installed-CLI proof explicitly
expects a Coordinator-routing rejection and an unused transport. These are
useful bounded tests; neither is a successful live operator journey.
The replacement must exercise its actual operator transport against PocketIC
canisters before that capability is qualified.

The existing PocketIC billing fixture uses a mock Cashier. Its upgrade and
gateway tests do not supply deployed provider retry retention, older-backup
identity authority or final billing settlement. These gaps are tracked in
[the provider review](provider-review.md) and acceptance A03/A05/A08.

## Required evidence before removal

1. Freeze B1: selected provider interfaces/economics/recovery, consumer and
   accountable owners, bounds and installation obligation inventory. Latest
   package selection does not close these decisions.
2. Implement every capability here through the same service handlers. Record
   exact replacement source and executable tests in the capability inventory;
   empty replacement evidence remains an open obligation.
3. Run the bounded upload/resume/verified-read/release journey through both
   standalone and managed adapters. Include operator setup/readiness, authority
   denial, capacity, uncertainty, races and supported restore failures.
4. Retain source-bound native/PocketIC results, exact Wasm/Candid provenance and
   actual-provider evidence separately. Test substitutes must remain labeled.
5. Qualify the operator client against both deployments. Demonstrate correct
   target selection and no mutation from status, diagnostics or dry runs.
6. Have the Canic owners qualify replacement generic lifecycle, guard,
   allocation and endpoint coverage. Refresh the removal inventory and verify
   the independently available service artifacts before B3 depends on them.
7. Handle source removal and affected installation retirement separately.
   No reset may erase the only evidence of uncertain effects or continuing
   balances/billing. No Canic edits, removal, release or publication are
   authorized by this inventory.

The inventory is planning evidence, not an executable release gate. Final
qualification must execute behavior and bind artifacts/results to source;
checking a row, file existence or a manually edited completion flag cannot
establish readiness. No Canic capability may disappear merely because its
replacement is inconvenient or the older implementation's evidence was weak.

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
