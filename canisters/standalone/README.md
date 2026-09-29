# Standalone host

`ic-blob-storage-canister` explicitly owns the canister endpoints, lifecycle and
ic-memory grants. The shared library still exports no endpoints or lifecycle hooks.
This is an initial host, not yet a qualified Caffeine-backed storage service.

Build with `make build-standalone`; run its local lifecycle cases with
`make test-standalone`. The Wasm is
`target/wasm32-unknown-unknown/release/ic_blob_storage_canister.wasm`.
These commands do not deploy or contact Caffeine.

The [Candid contract](service.did) is generated from the endpoint declarations:

```sh
cargo run --offline --locked -p ic-blob-storage-canister --example export_candid > canisters/standalone/service.did
```

Installation takes one explicit `ServiceConfigurationInput`, including the actual
service principal, operator, payer, namespace and all resource/billing bounds.
Management-canister installation authorization remains the platform's responsibility.
There are no deployment defaults, provider namespace provisioning or account changes.
Operator-only `blob_configuration` returns the installed values, package release
and restore fence; an unfenced owner does not imply provider readiness.

Operator-only `blob_local_status` takes the explicit service, namespace, Cashier
and payer scope. It returns one synchronous snapshot of maintained upload byte
totals, funding allocation/outcomes, bounded gateway membership and read occupancy,
with each owner's restore fence. Inspection reads no lifetime history and makes no
provider calls. Available allocation is not platform liquidity, transport acceptance
is not provider credit, and the snapshot does not authorize retry or reconciliation.

Operator-only `blob_revoke_gateway` takes this same scope and a concrete gateway
principal. It removes local membership and invalidates older sync/read observations,
including when the member was already absent. Read reservations and object/funding
obligations remain accounted for. Each call is a fresh revocation decision, so do
not automatically retry a lost acknowledgment; inspect local status first. Future
explicit additions or syncs can re-add a member. This is not provider credential
revocation or deletion, and restored registries reject the update.

Operator-only `blob_sync_gateways` refreshes membership from the installed Cashier
using the same scope. It records a pending sequence before one replicated
`storage_gateway_list_v1` query, with a 30-second bounded wait, 64 KiB reply limit
and bounded decoding. No cycles attach; ordinary IC execution fees apply. Failed
queries or unusable replies retain pending work and block another refresh. Inspect
`blob_local_status`, then use `blob_cancel_gateway_sync` with that exact sequence
to cancel it. Cancellation preserves membership and allocated sequence history;
revocation or cancellation prevents a delayed response from applying. Restoration
fences both updates. This establishes local transport behavior, not deployed
provider qualification, certificate readiness or payment authority.

`blob_upload_history` lists this tenant's operations or, for the operator, all
service operations. Each call scans at most 64 retained rows and returns at most
32 matching identities with current local lifecycle state and the restore fence.
Filters select all history, active uploads, pending deletion or outstanding
obligations. Empty filtered pages may still have a continuation cursor. Cursors
bind the service, namespace, observer scope and filter; start a fresh sweep for
changes behind the cursor. Inspection remains available during suspension and
restore, without granting provider retry, publication or deletion authority.

Tenant-only `blob_upload_capacity` takes `TenantScope` and returns the tighter
global/tenant headroom for lifetime objects and manifest leaves, concurrent uploads
and bytes. Maintained counters include reserved bytes, physical storage and
continuing billing. The response echoes the scope, per-object metadata/byte limits,
current enrollment and the independent restore fence. Missing enrollment rejects;
suspended or restored tenants can still inspect. Counts are independent dimensions,
not reservations or proof that an upload is safe. Cancellation restores only the
applicable byte/concurrent capacity; lifetime history stays consumed.

Tenant-only `blob_reference_capacity` takes the same tenant scope plus a provider
root. It reports remaining lifetime references, unreserved receipt slots, receipts
reserved for releasing live references and the corresponding fresh-retain headroom.
The response echoes the exact request and includes the restore fence even when no
headroom is visible. Unknown, foreign and unconfirmed roots all return absence;
retired confirmed objects report zero fresh retains. Reads remain passive through
suspension and restoration. Positive counts do not bypass enrollment, identity or
restore checks, reserve a reference or prove that a reference is currently live.

Tenant-only `blob_upload_status` looks up the exact original `ReferenceUpload`,
including its separate upload, object, incarnation and first-reference identities,
root and declared byte count. It reports retained local state and revocation;
changed original arguments conflict, and an unknown operation stays unknown.
Both query and replicated calls remain passive during suspension and restoration.
Unlike `blob_upload_admission`, this lookup does not require the uploader and expiry.
It does not report the restore fence: consult configuration or history for that
independent observation. Historical confirmation never proves a live reference,
safe provider retry or publication eligibility. Canister tenants can use the shared
`ReplicatedUploadStatusClient` from an update with an explicit reply bound.

Operator-only `blob_funding_history` uses the same explicit service, namespace,
Cashier and payer scope as local status. It returns at most 32 retained intents
per call, newest operation first, including exact offered amounts, optional target
balances, transport phases and callback refunds. Replies echo the request and
report the restore fence. A cursor is an exclusive operation-ID bound; start a
fresh sweep for new intents or changed phases. This is local history, not evidence
of complete provider-account activity, credit or safe payment retry.

Operator-only `blob_funding_outcome` inspects one exact original intent using its
scope, operation, offered amount and optional target balance. Changed original
arguments reject; an unknown identity returns absence without retry authority.
Retained results include transport phase, any structured response, the restore
fence and shared reconciliation diagnosis. Reported balances, provider errors and
decoder failures stay separate from exact refunds and accepted-cycle obligations.
The query reads local storage and never refreshes provider balances or sends funds.

Canister operators can use `ops::service::funding::client::ReplicatedFundingClient`
with their actual canister identity, complete `OperatorScope` and a bounded timeout.
`history` accepts explicit byte/entry reply limits; `outcome` checks one original
intent. Calls require replicated execution and attach no cycles. The client checks
reply correlation, pagination and refund consistency, preserves restore fences and
owns no journal. Hosts still authenticate their own operator-facing endpoints.

Tenant enrollment, upload admission/revocation, manifest preparation/inspection and
reference operations use the existing shared workflows and actual caller/service/time.
Preparation carries metadata and hashes; file bodies stay outside the canister.
Reference operations require confirmed content, which this initial endpoint set
cannot establish. No trusted-fact fixture endpoints, certificate issuance, funding
mutation, deletion or billing-settlement endpoint is exported yet.

The host allocates seventeen exclusive grants in range 120–136 with sixteen-page
memory-manager buckets: one bounded v1 installation record and sixteen shared-store
memories. DTOs are converted into an owned configuration record. All state writes
are synchronous and traps propagate for IC rollback. Configuration is immutable.
Ingress bounds are 16 KiB for installation, 4 KiB for fixed commands and 128 KiB
for manifests, with bounded Candid decoding and shared semantic limits afterward.
The CDK first copies the separately platform-bounded ingress buffer.

Same-release upgrade takes Candid empty arguments `()`. It loads the saved
configuration, checks service and package-release identity, validates every owner
and leaves mutations fenced. It never repairs missing state or accepts replacement
configuration. The ic-memory runtime commits allocation-ledger metadata during
bootstrap; this is not service reconciliation or freshness authority. Stop/start
preserves the active owner. Package release is not a module hash, and arbitrary
snapshot rollback does not acquire fresh authority. Operational recovery remains
unfinished. Cross-release transitions require reinstall after the separately
defined installation-retirement requirements; controllers can erase state through
the management canister, so these hooks cannot enforce retirement on their behalf.
