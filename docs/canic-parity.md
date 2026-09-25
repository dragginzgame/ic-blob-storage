# Canic functionality replacement and removal gate

The maintainer requires Canic's blob functionality to be ready here before
removing it from Canic. This includes operator workflows and diagnostics as
well as storage and billing. The first native core primitives are implemented;
no end-to-end service capability is qualified here yet. Installed dependencies,
plans and source inventories do not establish parity.

[The capability inventory](canic-capabilities.json) maps every public method in
Canic's blob API and every emitted blob endpoint at commit
`10d00c6d9494a45b30e66f84d1bd886c8acdd45c`. It also identifies the CLI, diagnostic,
lifecycle and declaration surfaces, with concrete source-test references.
Some references are assertion helpers inside larger tests. They identify
behavior to reproduce, not checks run or passed in this repository.
Source file hashes are retained in [the source inventory](canic-source-inventory.tsv).

## Required replacements

BLOB-01 has native parsing/canonicalization and incremental raw-byte verification
evidence. BLOB-08, BLOB-11 and
BLOB-12 have partial numeric validation and pure-policy evidence, recorded in
[core evidence](evidence/core-primitives.md) and the capability inventory.
Provider bindings, configuration persistence, actual funding, status workflows
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
