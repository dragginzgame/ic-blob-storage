# Current status

Date: 2026-09-26

## Released baseline

The maintainer reports 0.1.4 live. Local Git release is `3aef138`, from source
`f0cabb5`; Cargo and the receipt are 0.1.4. Registry publication was not independently
queried. Release/publication retain build artifacts; only explicit `make clean`
removes them. See [release guidance](../releasing.md).

The released core provides distinct content/provider identities, incremental
raw-byte verification, numeric billing/configuration validation, bounded transient
gateway membership and pure funding/admission/readiness policy. Whole-balance
conversion rejects malformed components even when the total is valid.
Provider bindings, persisted workflows, clients, endpoints and adapters remain
unimplemented. These native primitives do not qualify the complete service.

## Current implementation batch

The maintainer requested continued Canic replication after 0.1.4.
`ProviderRootBatch` now parses bounded binary-root batches while preserving input
order, duplicates and typed errors at each malformed position. Empty batches are
accepted, matching Canic's input shape. Raw entry and combined byte budgets are
checked before result allocation; invalid roots count at their actual byte length.
Decoder allocation limits remain separate. No root is declared live or dead here.

This is the input-parsing portion of BLOB-03, within the local hash-validation
scope. Authenticated liveness lookup, persisted references and provider callbacks remain
unimplemented. Canic's `api/blob_storage/lifecycle.rs` still matches the captured
source hash. That parser slice added no provider calls or dependencies. No sibling
changes or version operations were made. Native behavior tests, Clippy, Wasm, docs
and formatting validate this local scope; no full release gate or PocketIC ran.

The maintainer then requested the persistence contract/lifecycle model and
explicitly directed a fresh review of Canic's choices. The
[independent lifecycle design](../service-contract.md#lifecycle-design-under-independent-review)
records concrete local invariants and proposed persisted boundaries without
claiming provider suitability or freezing deployment budgets. A transient
`BlobLifecycle` now tracks confirmed-object references with bounded retained
release IDs. Final release, physical deletion and economic settlement advance
separately; live-object deletion, reused IDs and new retains after deletion queues
reject. Zero bytes do not imply settled obligations. Native transition tests,
Clippy, Wasm, docs and formatting pass; no persistence or endpoint authentication
or provider calls are implied. Continue evaluating each Canic choice on its merits.

The local lifecycle now carries an immutable service/tenant/provider-namespace/
object/incarnation binding. Reference mutation and supplied confirmations reject
every scope mismatch before replay handling. A pure direct-tenant policy checks
actual service and authenticated actor against trusted object ownership; matching
request IDs never grant access. Native tests cover every mismatch through live,
pending, deleted and settled phases. Endpoint authentication, delegation, trusted
namespace resolution, ID allocation/restore safety and exact provider operation
evidence are still unimplemented. The unbound draft lifecycle signatures were
replaced before release; no compatibility wrappers were introduced.

`ReferenceRequests` now owns the local lifecycle plus bounded exact-request
receipts. Replays return original typed results, including failures, without
reapplying a mutation. Changed actors/payloads reject conflicting IDs. Each live
reference reserves one release-receipt slot, so receipt pressure cannot block
cleanup of already admitted references. Tests cover capacity, original-failure
replay after state changes, no reactivation and access checks before replay.
All targeted native/lint/Wasm/docs checks pass. This is transient bookkeeping;
persistent receipts, provider-effect retries and restore recovery remain pending.

[Core evidence](../evidence/core-primitives.md) and
[the capability inventory](../canic-capabilities.json) record the partial coverage.
The new batch has [separate hashes](../evidence/root-batch.sha256); prior inventories
remain historical. Changes are collected in the undated
[0.1.5 changelog draft](../../CHANGELOG.md); Cargo and the release receipt remain
0.1.4 until the maintainer's release flow.

## Provider evidence and next work

Toko indexed commit `6519b72d2a420564dabaf700fc55f7b8603d9fd3` supplies defaults
`https://blob.caffeine.ai` and Cashier `72ch2-fiaaa-aaaar-qbsvq-cai`. Retained
[deployment evidence](../evidence/caffeine-deployment-observation.json) records
successful anonymous metadata, gateway-list and pricing queries. Both gateway
method names are advertised; the newer top-up wrapper is Candid-compatible with
the deployed signature. No update, payment or private account lookup ran.

The [provider baseline](../provider-baseline.json) records client 1.1.2 and
backend 1.1.1, verified on 2026-09-25. Server revision, account binding and
retry/retention/deletion/billing guarantees remain unresolved in the
[provider review](../provider-review.md). Qualify these guarantees and freeze the
consumer, owners, resource bounds and restore contract before provider transports
or persisted workflows, as required by the [service contract](../service-contract.md).
Recheck upstream before that implementation. Caffeine remains unqualified.

The 2026-09-26 [recovery review](../provider-review.md#recovery-findings--2026-09-26)
rechecked unchanged official GitHub main and npm 1.1.2. Isolated probes confirm
that the client returns a hash/100% without requiring a complete response and
that an empty funding result discards a structured Cashier error. The root-only
deletion callback cannot itself distinguish reused-root incarnations. Recorded
source hashes, controls and reproduction details are in
[recovery evidence](../evidence/caffeine-recovery-review.json).

The maintainer then explicitly requested resolving those findings. Local Caffeine
reply decoders now preserve typed top-up failures, reject unusable funding
responses under byte/work/type bounds, and expose upload completion only as a
reported status. Provider DTOs have one private owner; neither decoder performs
effects or establishes completion/credit. `serde_json` 1.0.151 became a direct
dependency at its existing lockfile version. Anonymous refreshes confirm the
Cashier Candid hash and Mops backend 1.1.1 remain unchanged on 2026-09-26.

`RootClaims` now keeps a bounded lifetime association from each root to its
original complete object binding across the whole service. It rejects reuse
after settlement and preserves lookup/replay at capacity. The native delayed-
confirmation test rejects correlation to a newer deletion-pending incarnation.
This conservative local policy denies fresh objects with a previously claimed
root, including identical content under another tenant. It is not yet persisted
or exposed through an endpoint. Tests, strict Clippy, Wasm, docs and formatting
pass; [implementation evidence](../evidence/core-primitives.md#provider-response-and-root-correlation-fixes)
records the local scope. No paid operation or full release gate ran.

The three local parsing/reassociation paths are addressed, but the full journey
remains gated. Next obtain authoritative upload completion/reconciliation and
funding retention/settlement semantics; qualify exclusive provider namespaces,
then freeze bounds and durable intent/root-claim restore handling before adding
transports. Do not clear these gates merely because local rejection tests pass.

The maintainer confirmed Canic's 0.110 human acceptance for work here without
changing Canic's handoff. Local primitive work does not close the remaining
service gates. Library publication is separately enabled. Agents must not create
commits; sibling repositories remain read-only. All Canic blob capabilities,
including operator workflows and both deployment journeys, must work here before
removal there; see [parity](../canic-parity.md) and [acceptance](../acceptance-plan.md).
