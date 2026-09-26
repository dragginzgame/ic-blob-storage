# Service contract — B1 draft

This is an unresolved contract checklist, not a frozen service API or provider
suitability verdict. Freeze the decisions and evidence before B2 implementation,
subject to the explicit bounded exception below.
The bootstrap package name does not decide the final package split.

On 2026-09-25 the maintainer explicitly approved implementing content identities,
hash parsing and pure funding/readiness policy with native tests before B1
closes. This exception includes local implementation and its evidence only;
provider bindings/effects, persisted workflows and Canic removal remain gated.
The maintainer subsequently removed B1 ownership/readiness as a library
publication gate; crates.io publication follows the separate release workflow.
[Core evidence](evidence/core-primitives.md) records the resulting
scope. It does not settle tenant authority, provider identity, configuration
persistence, recovery reconciliation or overall service readiness.

The continuing local port includes billing input/configuration validation and
transient gateway-list validation and membership operations. These values add no provider interface,
persisted workflow or callback authority; the remaining implementation gates
below still apply.

The transient gateway model now correlates one pending sync to its exact local
attempt and immutable service/namespace/Cashier scope. Operator edits invalidate
older syncs; malformed responses preserve current state. Pure callback policy
checks current membership against the trusted object and execution context.
These local rules do not establish endpoint authentication, provider revocation,
durable freshness or a restore-safe sequence allocator. Operators can explicitly
start a later sync that re-adds a member; no permanent denylist is implied.
Bounded local Candid decoding now applies gateway-list replies through that
registry, checking correlation before parsing and preserving state on failure.
It accepts supplied bytes and trusted context; no provider fetch is implemented.
Local account-balance reply decoding likewise binds successful reports to a
supplied requested account and rejects unusable amounts. It does not establish
transport identity, account ownership, observation freshness or payment outcomes.

Pure funding policy now also assesses admission of a new intent from supplied
recovery/activity observations. This is part of the local policy exception;
it neither establishes those observations nor persists or executes an intent.

The maintainer subsequently requested the persistence contract and lifecycle
model, and explicitly directed a fresh design review of Canic's decisions.
That local scope includes the transient confirmed-object lifecycle model below.
It does not close provider qualification or authorize persisted workflows/effects.

On 2026-09-26 the maintainer explicitly requested resolving the upload-completion,
funding-error and stale-deletion findings. That scope now includes bounded local
Caffeine reply decoding with one private wire owner, and a transient immutable
root-claim model. It does not authorize paid tests, deployed adapters or a claim
that source-level response checks establish the missing server guarantees.

The [Canic parity review](canic-parity.md) records the captured
Canic source inventory, preserved behavior, required safety corrections and
removal obligations. The [acceptance plan](acceptance-plan.md) supplies
concrete proposed cases A01–A12. These are B1 working inputs, not a frozen
contract or executed qualification. The repository is now
`dragginzgame/ic-blob-storage`; accountable maintainers and registry ownership
remain to be assigned.

The maintainer selected the latest official Caffeine integration as the target.
The [provider baseline](provider-baseline.json) pins the verified latest npm
client and current official backend source; the [provider review](provider-review.md)
records verification and differences from Canic's snapshots. Canic supplies
extraction history, not authority for the provider contract. Refresh the exact
upstream baseline before implementation and qualification. Caffeine is not yet
qualified; deployed-contract and recovery/economic evidence still must close
before provider bindings and effects are implemented.

The [independent deployment review](provider-review.md#independent-deployment-support)
also requires a supported Caffeine onboarding/namespace arrangement for this
service. Public integration packages and Toko's source defaults do not establish
that arrangement. The current candidate remains Caffeine; its platform guidance
alone is not a decision to change providers or drop standalone deployment.

## Design inputs and assumptions

The maintainer requested a fresh review on 2026-09-26, including choices made
here. Canic supplies the capability/removal checklist; Toko supplies concrete
consumer scenarios; current authoritative provider contracts define provider
behavior. Neither application's implementation automatically specifies this
service. Refresh dated source pins before implementing an external boundary.

Toko's [asset operations](https://github.com/dragginzgame/toko/blob/6519b72d2a420564dabaf700fc55f7b8603d9fd3/fleets/toko/project/instance/src/ops/asset.rs)
check root liveness during creation without an explicit root-uniqueness check in
that path. Deletion checks token and generator references across awaits. This
supports reviewing repeated-content and reference-ownership scenarios; it does
not establish a final sharing requirement or atomic cross-canister exclusion.

Resolve these assumptions before freezing the production contract:

- Lifetime root non-reuse is a conservative response to ambiguous deletion
  callbacks, not an established consumer requirement. Review repeated uploads,
  multiple assets using one root and delete/re-upload against provider-supported
  operation identity. Retain the current safety restriction until a replacement
  proves delayed callbacks cannot affect a newer object.
- Cross-canister reference acquisition/release needs explicit coordination.
  Sequential reference reads around awaits do not establish deletion exclusion.
- Reserve capacity before provider effects. Bound tenant object metadata as well
  as bytes, retaining capacity for release receipts. Zero-byte and settled history
  must not bypass limits or erase physical/billing obligations.
- Qualify lifetime history/churn limits and catalog-wide scans against workload
  and instruction budgets before choosing production indexes or retirement bounds.
  Billing-byte totals are not monetary accounting.
- Establish project/browser/service authority explicitly. Direct download URLs
  do not establish confidentiality; qualify serving, MIME handling, integrity,
  empty objects and chunk limits against actual consumer scenarios.

Use current ICP guidance on [inter-canister calls](https://docs.internetcomputer.org/guides/security/inter-canister-calls/),
[idempotency](https://docs.internetcomputer.org/guides/canister-calls/idempotency/)
and [storage bounds](https://docs.internetcomputer.org/guides/security/data-storage/):
persist intent before effects, reconcile uncertain outcomes, revalidate across
awaits and prevent one tenant exhausting shared storage. Recovery requirements
apply within the frozen release; pre-1.0 cross-release transitions remain reinstall-only.

## Acceptance target

The proposed package roles are core, passive service protocol, upload/read
client, standalone canister, thin Canic adapter and operator CLI. Only the core
exists; final names and package split remain open. Both adapters belong here,
and only the managed adapter may depend on Canic. One internal provider module
owns request/callback definitions; clients use the service protocol.

The maintainer explicitly requires all Canic blob functionality to be ready
here before Canic removal. The [parity contract](canic-parity.md) and
[capability inventory](canic-capabilities.json) include lifecycle, gateway,
billing, status, operator commands and diagnostics. Operator replacement is
part of this extraction, with A11/A12 acceptance alongside the service journey.
Preserve capabilities using the current provider contract and required safety
corrections; do not preserve superseded APIs or unsafe behavior as aliases.

Name one concrete application and accountable consumer owner. Its journey is:
an authorized tenant uploads a bounded object, resumes after interruption,
reads and verifies bytes, and releases the reference through confirmed provider
deletion and billing cessation. Exercise it against both standalone and
Canic-managed deployments with the same blob API and tenant rules.

Classify each behavior as existing behavior preserved, a safety correction
required for extraction, or a new capability deferred. B2 is bounded by this
journey and necessary corrections; optional ambitions do not gate extraction.
Shared cross-tenant deduplication, generic provider plugins, cross-release
migration, multi-Fleet indexing and new confidentiality guarantees are deferred.

## Lifecycle design under independent review

Canic is a capability inventory and source of counterexamples, not the target
data model. The maintainer explicitly requested reassessing its choices. Existing
capabilities remain required; preserving them does not require copying root-keyed
ownership, automatic liveness on registration, transient funding locks or deletion
of all state after a gateway callback. Each design choice needs its own rationale
and rejection/recovery evidence.

The local [lifecycle model](../crates/ic-blob-storage/src/model/lifecycle/mod.rs)
now implements the confirmed-object part of this design. It is transient and
starts only after independently verified completion. It now carries an immutable
service/tenant/namespace/object/incarnation binding and checks it on every mutation,
before even idempotent replay. Anonymous and management service/tenant principals
reject; the opaque namespace ID still needs a trusted provider configuration.
IDs do not prove fresh allocation or survive old backups by themselves.

The pure direct-tenant access rule requires the actual service instance to match
that trusted binding and the authenticated actor to equal its tenant principal.
It grants no special controller or digest-based access. Execution context and
expected binding must come from trusted state/platform inputs, not request claims.
Delegated actors and per-user permissions need their own reviewed rule; the
current rule does not implement them. Upload admission, sessions, persistence,
endpoint authentication and provider evidence validation remain outside this model.

The local `ReferenceRequests` value now owns a lifecycle and its exact-request
receipts. IDs are scoped to that object incarnation; each admitted ID binds the
authenticated actor, operation kind and complete reference arguments. Exact
replay returns the original typed success or lifecycle failure without applying
the operation again. Scope errors, ID conflicts and receipt-capacity rejection
leave both lifecycle and receipts unchanged. These rejected admissions consume
no ID; recorded lifecycle failures require a fresh ID for re-evaluation.

Each active reference reserves one future release receipt. Admission enforces
`retained receipts + active references <= receipt limit` after staging the
operation, so receipt pressure cannot consume the capacity needed for final
release. Exact retries still work at capacity. No receipt eviction or timeout
exists. The workflow must authenticate before receipt access as well as mutation;
an original success response does not imply the object is still live today.
This is transient local bookkeeping: it does not survive restart, reconcile a
paid effect or permit clearing history through reconstruction. Durable atomic
storage, request allocation and global limits remain open.

| Local decision | Reason and consequence |
| --- | --- |
| Distinct reference identities; count object bytes once while any reference remains | Releasing one consumer use must not delete another; reference counts are charged separately from bytes |
| Retain released reference identities within an explicit lifetime slot bound | Duplicate release is harmless; a released ID cannot be reused. Released slots are not silently recycled, so sustained churn eventually rejects new references until a separately proven retirement path exists |
| Final release queues deletion; new retains then reject | A new reference must not race an already exposed deletion request; cancellation would require a provider guarantee not currently established |
| Physical deletion and billing settlement are separate transitions | Tenant quota release cannot erase global storage or economic liabilities |
| Reject deletion confirmation while references remain | A valid gateway caller still must not delete a live object |
| Settlement is explicit even for zero bytes | Request fees, minimum charges and uncertain effects are not represented by byte counts; zero counters never prove retirement safe |

Local consumer liveness reads use full reference keys after direct-tenant
authorization. Unknown/released references are inactive even if a sibling keeps
the object live. Batches are bounded and preserve input order/duplicates; a wrong
binding rejects the complete batch. These pure reads change no accounting and
are not gateway root-liveness/deletion responses. The eventual workflow must
authenticate callers, resolve current state and fence unreconciled restoration.

The local model also composes confirmed lifecycles, immutable root claims and
reference journals in a bounded transient catalog. Limits cover global and
per-tenant lifetime object slots, per-object metadata, tenant logical bytes across
namespaces, and separate global physical/billing-byte totals. Object counts retain zero-byte
obligations. Exact registration replay never revives released references; no
settlement path deletes history or frees lifetime slots. Derived counters avoid
an independently mutable accounting ledger.

Tenant catalog reads now resolve ownership before inspecting supplied reference
bindings. Bounded cross-object batches preserve order/duplicates and reject
unknown or foreign roots identically, with no partial statuses. Exact request
receipt queries authenticate independently, returning the original result without
reapplying it or consuming capacity. An absent receipt only describes the supplied
local journal; it never proves an uncertain provider effect did not execute or
releases a restore fence. These remain pure reads, not network query guarantees.

Local pending-deletion enumeration bounds scanned entries and returned results,
with service/namespace cursors and current-gateway checks on every page. Pages
are not snapshots; each sweep restarts to catch newly pending earlier roots.
Root observations preserve unknown/malformed/foreign-namespace states rather
than equating them to deletion permission. These are internal model/policy views,
not a new provider wire contract. This extends the transient local exception;
production still needs atomic durable reservations/claims before upload effects,
authenticated endpoints, monetary accounting, provider evidence and restore fencing.

The maintained transition order is below. Every confirmation presupposes exact
authority and operation/incarnation correlation; these methods do not verify that
evidence. If financial evidence arrives before deletion evidence, the future
workflow must retain it without prematurely settling this lifecycle.

| Transition | Tenant logical bytes | Global physical bytes | Unsettled liability |
| --- | --- | --- | --- |
| Confirmed upload with first reference -> Live | Retained | Retained | Retained |
| Release a non-final reference -> Live | Retained | Retained | Retained |
| Final reference release -> DeletionPending | Released | Retained | Retained |
| Exact physical deletion evidence -> ProviderDeleted | Released | Released | Retained |
| Exact final billing evidence -> Settled | Released | Released | Cleared for this object only |

Settled does not erase reference receipts or authorize reset. Account balances,
fees and obligations outside this object still require retirement evidence.
The byte projections in the model are inputs to future aggregate accounting,
not a complete monetary ledger or proof that Caffeine supplies these facts.

The [provider recovery review](provider-review.md#recovery-findings--2026-09-26)
identifies a concrete adapter constraint: Caffeine's reviewed deletion callback
carries roots, not local incarnations. Resolving a root to the current object
and supplying that object's binding would defeat stale-confirmation protection.
The provider association must establish which deletion the callback confirms,
including when a repeated root's newer incarnation is already deletion-pending.
The local `RootClaims` model now enforces one lifetime object binding per root
across the entire service, including all tenants and namespaces. Claims survive
local deletion/settlement and cannot be removed or reassigned; a full lifetime
bound rejects new claims but preserves old lookup and exact claim replay. This
deliberately rejects re-uploading the same root as a new object, including another
tenant's identical content. It introduces no automatic content sharing.

Production adoption still requires an exclusive provider namespace with no
unaccounted previous root usage, claims durably reserved with upload intent
before certificate/effect exposure, and fenced recovery of the complete claim
history. Constructing a new empty map is never a safe restore. A root lookup
supplies the original binding, not gateway authorization or deletion proof;
authenticated callbacks must still pass the namespace and lifecycle checks.
Numeric lifetime budgets and retirement remain B1 decisions. Root non-reuse
does not establish billing cessation or fix loss of history after an old backup.

### Candidate persisted boundaries

These are proposed schema responsibilities for v1, not installed records or a
frozen wire format. Keep them independent of Canic's memory IDs and store layout.

- Object identity: an allocated object incarnation bound to service, tenant and
  provider namespace, with provider root, content digest and declared length as
  data. A root or digest is never the ownership key. Initial design has no
  automatic deduplication; explicitly retaining the same tenant-owned object is
  distinct from merging independently uploaded objects. Provider-side identity
  collisions/sharing must be resolved before creating separate object records.
- References: bind each reference ID to its object and authorized actor or
  consumer use. Persist the request identity/payload binding and release result;
  same request with different arguments rejects. The local model binds references
  to their owning object and the pure policy covers direct tenant callers; neither
  implements per-reference delegated authority. Exact local receipts now bind
  reference-operation payloads; persistent receipts and provider-effect identities
  still require their own implementation.
- Upload/effect intents: persist namespace, incarnation, exact operation ID,
  input fingerprint, reservation and unresolved outcome before any effect.
  Registration and upload certificates never alone establish completed storage.
- Accounting: tenant logical bytes/references, global physical bytes/objects,
  sessions, receipts and monetary liabilities have separate bounds. One ops
  transaction applies model changes, receipts and counter deltas together;
  orchestration must not await between admission and durable reservation.
- Recovery state: restore synchronously into a fence before scheduling work.
  In-place same-release restart preserves unresolved intents. Older-backup restore
  additionally needs surviving identity/accounting authority and concurrent-instance
  exclusion; a restored local counter cannot release the fence. No automatic
  timeout or reinstall may erase uncertain effects or continuing costs.

Installation must explicitly supply positive limits for object/chunk size,
tenant/global bytes and object/reference counts, outstanding sessions/effects,
receipt storage and liability capacity. The lifetime reference bound counts
released IDs too. Reject inconsistent profiles before admission, with no inferred
production defaults from Canic. Numeric deployment budgets and supported evidence/
restore horizons remain open until a concrete consumer and provider guarantees
are selected; the model's native fixtures are not production limits.

Toko's inspected asset helper delegates root liveness and deletion to Canic; it
provides a consumer behavior example, not authority for this design. Its exact
source is already recorded in [deployment evidence](evidence/caffeine-deployment-observation.json).
The next persistence step needs the unresolved provider identity/evidence and
restore decisions below; native transition tests cannot establish those guarantees.

## Decisions and evidence required

Freeze maximum object/chunk sizes, tenant/global byte and count limits,
concurrency, session and receipt bounds, and supported interruption/restore
horizons. Assign the acceptance cases to accountable owners.

| Contract | Owner role to assign | Decision and acceptance evidence |
| --- | --- | --- |
| Scope and publication | Service maintainer and consumer owner | Name application, maintainers, package split, registry/repository ownership and release plan; bind preserved behavior to source |
| Deployments and adapters | Service maintainer; Canic owners qualify generic integration | Both adapters live here, share handlers and tenant rules; core builds without Canic; retain Candid/artifact provenance and both journey results |
| Tenant authority | Service maintainer | Exact tenant/actor bindings and denial cases; a digest or Canic controller status grants no tenant authority |
| Identities and restore | Service maintainer | Recovery identity, non-reuse authority surviving older backups, stale-instance fencing and reconciliation before effects or admission based on stale accounting |
| Provider suitability | Service maintainer | For every paid/destructive operation: exact retry identity, authoritative completion evidence, retention horizon, typed uncertain outcome, bounded reconciliation, separate deletion and billing-cessation evidence |
| Accounting | Service maintainer | Deduplication choice, logical/physical quota basis, reservation/release timing, race-safe counters and ownership of costs until deletion and billing cessation |
| Existing obligations | Each affected installation operator; Canic owner inventories allocations | External objects, uploads, uncertain paid effects, balances and billing inventory; no-obligation evidence or completed owned decommission/disposition before reset |
| Canic removal and generic coverage | Canic runtime/facade, host/CLI and testing owners | Complete removal inventory and replacement evidence for surviving generic fixture coverage; changes occur only under separate Canic work |

Caffeine is the selected integration target but is not yet qualified. Missing exact
retry binding, authoritative completion evidence, adequate evidence retention,
a safe uncertain-result disposition, or required deletion/billing-cessation
proof disqualifies the provider for the required contract. Provider evidence
must identify exact source/deployed interfaces and observed behavior.
PocketIC substitutes prove behavior under a model, not actual provider support.

Expired receipts or evidence never authorize repeating an uncertain paid
effect. Restoring sequence 40 after paid sequence 41 completed must not permit
reuse of 41 or erase its liability. Define fence entry and release, concurrent
instance exclusion and supported restore horizons. If sufficient evidence
cannot survive a restore, reject that path before effects and define a narrower
provable same-release backup/restore boundary.

Deduplication may be absent or tenant-local; shared cross-tenant deduplication
requires a concrete need and explicit privacy, charging and deletion ownership.
Releasing tenant quota never removes still-stored bytes from global capacity
or costs. Upload/release races must preserve references and counters; uncertain
effects retain conservative reservations or equivalent bounded liability.

Source allocation removal and installation retirement are separate. Before
reset, preserve the records needed to reconcile effects and settle obligations.
Unresolved paid effects stay fenced. Residual balances require controlled
return or explicitly reviewed terminal disposition; continuing liabilities
require preserved evidence, authority and funded ownership outside erased
state. No migration engine or old-state reader is implied.

## Exit evidence

B1 closes decisions, owner assignments, removal/obligation inventories and
actual-provider suitability evidence. It assigns exact service acceptance
cases to B2/B3 rather than claiming those tests already pass. Tests must cover
isolation, capacity exhaustion, corrupt bytes, interruption, lost responses,
older-backup restore, expired receipts, upload/release races and delayed
deletion/billing. Every guarantee needs a named owner and a test or evidence
reference; all such assignments remain open at bootstrap.
