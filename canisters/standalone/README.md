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

`blob_upload_history` lists this tenant's operations or, for the operator, all
service operations. Each call scans at most 64 retained rows and returns at most
32 matching identities with current local lifecycle state and the restore fence.
Filters select all history, active uploads, pending deletion or outstanding
obligations. Empty filtered pages may still have a continuation cursor. Cursors
bind the service, namespace, observer scope and filter; start a fresh sweep for
changes behind the cursor. Inspection remains available during suspension and
restore, without granting provider retry, publication or deletion authority.

Tenant enrollment, upload admission/revocation, manifest preparation/inspection and
reference operations use the existing shared workflows and actual caller/service/time.
Preparation carries metadata and hashes; file bodies stay outside the canister.
Reference operations require confirmed content, which this initial endpoint set
cannot establish. No trusted-fact fixture endpoints, certificate issuance, provider
dispatch, funding, deletion or billing-settlement endpoint is exported yet.

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
