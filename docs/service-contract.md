<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Service contract

This defines the maintained library and standalone operating contract. The
[delivery plan](roadmap.md) owns milestones and current consumer decisions.
Candidate validation composes the local models without installing state or
proving provider behavior. Library publication and live service acceptance are
separate.

## Current contract at a glance

- The shared library owns tenant policy, upload records, manifests, references,
  quotas, provider accounting and durable workflow state. A host owns endpoints,
  caller authentication, memory grants and lifecycle integration.
- File bytes travel between the client and Caffeine. The service stores the
  permissions, verification evidence and usage records needed to control them.
- Tenant, uploader, verifier and operator are separate roles. Controller status
  alone grants none of those application authorities.
- Completion requires the configured verifier to fetch and check the complete
  file before submitting its statement. That statement proves an observation,
  not future provider retention.
- Restored state opens fenced. Current-instance recovery requires independent IC
  history; active recovery from an older snapshot remains unsupported.
- Releasing the last application reference, deleting provider bytes and ending
  billing are separate transitions with separate evidence.
- Uncertain paid or provider effects are inspected and reconciled from their
  original records; absence of a reply never authorizes an automatic retry.

<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-authority-boundaries.svg" alt="Authority boundaries between the tenant, uploader, verifier, operator, storage provider and application" width="800">
</p>

## Terms used in this contract

| Term | Meaning here |
| --- | --- |
| Tenant | The application account or organization that owns permissions and references |
| Permission | A bounded authorization for one exact upload request |
| Manifest | The expected file length, content identity and ordered provider chunks |
| Reference | A durable record that an application use still depends on a confirmed file |
| Verifier | The configured identity trusted to fetch and check the complete provider object |
| Fence | A state that permits inspection but blocks mutation until recovery is proven safe |
| Obligation | Provider bytes, uncertain effects, balances or billing that must remain accounted for |

## Configured certificate issuance — accepted 2026-10-02

The maintainer accepted the [first standalone contract](standalone-trial.md), then
explicitly requested relaxing its 1 KiB restriction for Toko Miner integration.
Object sizes, tenant/global byte budgets, lifetime objects/references/receipts,
manifest leaves and concurrent reservations now use the validated installation's
resource configuration. There is no additional one-object or 1 KiB issuance cap.
Admission reserves those resources; preparation validates the exact manifest;
issuance rechecks the retained reservation and permission. Exhaustion still refuses.
The policy requires explicit installed uploader trust, an exact local
service/namespace mapping, a current owner and atomic exposure commit.
The uploader must also hold the exact original tenant permission and
prepared manifest. Controller/operator status grants none of that authority.
No public uploader, automatic paid retry, provider spending-cap/replay guarantee
or operational old-backup activation is supported. Stop/start preserves the owner;
same-release upgrade restoration starts fenced. Explicit current-instance recovery
requires independent continuity evidence before activation. An older management snapshot
restores heap authority too and cannot safely resume operation.

Provider provisioning, account controls, retention, deletion and billing cessation
remain separate observed facts. Trusted participants and configured object limits
do not prove those facts or enforce a maximum external bill. The 100T-cycle total
planning ceiling is independent of actual target selection and paid-effect authority.
This replaces the former provider-qualification gate directly in a minor release:
no alternate mode, manually true flags, compatibility reader or v2 schema.
The public host-evidence field and DTO blocker for the former trial envelope are
removed outright, including Candid and native JSON. This semantic/API change
requires a minor release; existing deployed installations remain frozen.

## Explicit verifier completion trust — accepted 2026-09-29

The maintainer authorized a configured external verifier to attest observed content
availability after independently fetching the whole object under the installed
provider owner/project mapping and checking original root, length and metadata.
This is trusted-role evidence, not an untrusted client's progress, certificate,
local file check or proof of future retention. Provider payment/deletion/billing
semantics remain independent. No whole-file hashing occurs on the storage canister.

`CompletionAuthority` binds actual service, namespace and verifier. Shared
`workflow::uploads::completion` supplies verifier-only manifest inspection,
`attest` and historical receipt inspection. An attestation binds the complete
original permission, raw digest and observation time; time must fall between
admission and acceptance. Only prepared, exposed uploads can transition. Receipt,
confirmation, first reference and maintained accounting commit in one synchronous
IC transaction. The first accepted statement is immutable; exact replay preserves
its acceptance time and never recreates a released reference. A conflicting
statement rejects. Late evidence after revocation/suspension preserves obligations.
Inspection remains available through restore fencing; every attestation update,
including replay, refuses a restored owner. No new stable memory grant is needed.

Standalone requires an immutable `completion_verifier` at installation and exposes
`blob_verification_manifest`, `blob_attest_upload` and `blob_upload_attestation`.
Controller/operator/gateway status grants no implicit verifier authority. The
current v1 host/lifecycle schemas change directly, requiring a minor release and
cross-release reinstall. The existing trusted-host bookkeeping primitive remains
explicitly distinguishable from a verifier receipt; no receipt is invented for it.
Standalone certificate/exposure integration uses the restricted local contract
above; live end-to-end acceptance remains open.
See the [probe ledger](evidence/caffeine-probes/README.md)
for local transaction evidence and the [host contract](../canisters/standalone/README.md)
for endpoint usage.

Native recovery now uses `blob-storage upload-attestation` with an independently
selected expected verifier and a saved exact statement. Shared
`ops::service::uploads::completion::reply` owns the lookup encoding and bounded
inspection/mutation decoding. Scope, full permission, receipt verifier and time
ordering are checked; mutation acknowledgment must match the original digest and
observation time too. Lookup preserves a conflicting accepted statement for
diagnosis. Neither an absent nor a matched historical receipt establishes current
availability or authorizes redispatch; queries never resend statements or write
the journal. Restore fencing remains visible. This recovery reader does not
dispatch an attestation.

`blob_verification_plan` now authenticates that verifier and binds its exact original
permission/declaration to the installed Caffeine owner/project. It requires an
unfenced exposed unfinished upload; revoked/suspended exposed work can still be
reconciled. It does not grant upload, retry or future availability authority.
The native `observe-upload` command authenticates this snapshot, uses the maintained
provider target encoder with an explicitly approved origin, and makes one bounded
GET. Complete root/metadata/length verification precedes durable statement creation.
Fresh no-clobber runs persist intent before effects and retain failures; interrupted
runs cannot resume automatically. Provider reads can cost money and live trials
remain separately authorized. This command does not dispatch the saved statement.

Explicit native `submit-attestation` validates the complete successful observation
and its exact permission, declaration, owner/project, digest, timestamps and hashes.
It exclusively claims the run and persists the statement, signed envelope, request
identity, ingress expiry and trust/artifact hashes before one bounded update. The
saved statement is never regenerated. Shared reply decoding checks a certified
acknowledgment against that exact statement. Pending or uncertain outcomes require
explicit receipt inspection, without polling, refetching or automatic resubmission.
Existing claims refuse even after interruption before dispatch. Artifacts are trusted
local verifier input, not portable authentication or a distributed submission lock;
the canister's immutable receipt remains authoritative. This command introduces no
new installation, public DTO or stable-state schema.

Native saved-request and exact installation decoding share `native::exact_candid`.
It checks total bytes before decoding, rejects skipped fields, extra arguments and
trailing bytes, and suppresses payload diagnostics. Each caller keeps its existing
ceilings: 4 KiB small inputs, 16 KiB installation inputs/replies and 64 KiB manifest
decoding; 100,000 work units, or 2,000,000 for setup; 32 type-table entries for byte
verification/observation permissions and 64 elsewhere. Setup still reads ordinary
permissions with a 4 KiB file bound before its shared manifest decoder. File,
binding and remote-reply refusals stay distinct. Authenticated reply decoders retain
their separate contracts. Local byte verification uses the core permission validator
before opening the body or querying; role checks and its own byte ceiling remain
local. This structural validation grants no completion or retry authority.

## Service configuration and host

`dto::configuration::ServiceConfigurationInput` supplies explicit service, operator,
payer, namespace, resource, billing, funding allocation and read-session candidates. The shared
`ops::service::configuration` boundary bounds encoded bytes and Candid decoding work,
checks the actual host identity and reuses the existing model's invariants without
defaults, clamping or allocation. It also checks every maintained stable owner's
codec/resource envelope. Hosts may validate an already typed candidate.
The host must authenticate installation, bound ingress buffering and obtain its
identity from the platform. `ops::service::stores::ServiceStores` assembles the four
owners synchronously under these validated limits and sixteen distinct host-granted
memories. Fresh installation preflights the entire set before writing; aliased
grants or later write failures must trap for IC rollback. Native memory supplies no
transaction guarantee. Restoration validates every owner before returning the
assembly and leaves all owners inspection-only, with no repair or missing-store
initialization. Hosts still own memory grants, release/installation identity and
publishing the assembly only after success. These APIs neither establish a provider
namespace nor implement a production adapter, reconfiguration or operational
unfencing contract. Linking the library exports no endpoint or lifecycle hook.

`ops::service::installation::requests` builds all seventeen named installation
requests, including configuration, under the supplied host authority. Hosts
compose this passive inventory with their other application declarations before
their own bootstrap. Store-only compositions can use the sixteen requests from
`ops::service::stores::grants::requests`. `grants::open` assembles the current store mapping using that host's
committed-memory lookup; missing grants refuse without creating or repairing a
store. `open_default` checks an existing committed capability before using a
framework-owned default runtime, so absence cannot silently select a bucket policy
or create a second manager. These helpers do not grant exclusive access themselves:
the host must assign the handles to one storage owner. Standalone uses the shared
mapping with its unchanged configuration grant, range and bucket policy.
Framework consumers own their generic lifecycle wrapper and endpoint adapter;
Canic integration remains deferred; [GitHub issues](https://github.com/dragginzgame/ic-blob-storage/issues)
track consumer adoption. This contract owns the required service behavior.

`ops::service::installation::ValidatedServiceInstallation` also validates the
explicit project, verifier, trusted uploader and bounded library release before allocation.
Use `ic_blob_storage::LIBRARY_VERSION` for that compiled dependency identity;
an embedding host's package version and Wasm/module identity are separate facts.
`ServiceInstallation` owns the immutable current configuration record and the
four service owners. Fresh installation preflights all seventeen exclusive grants;
the host must propagate any subsequent trap for IC rollback. Restoration checks
the retained schema, actual service and compiled release before opening owners,
revalidates all installed inputs and returns only after every owner is fenced.
Missing or invalid state is never initialized or repaired. Configuration remains
immutable; getters grant no endpoint, tenant or operational recovery authority.
Standalone uses this owner; current DTOs/Candid and the v1 record now include
required immutable uploader trust without cross-release compatibility.

The [standalone host](../canisters/standalone/README.md) now explicitly owns these
hooks and grants and delegates configuration persistence to the shared owner. Its actual IC
installation validates all inputs before allocation; same-release upgrade restoration
loads the saved service/release-bound configuration without replacement arguments
and fences every owner synchronously. Its tenant, admission, manifest and reference
endpoints delegate to shared workflows, with bounded typed Candid and operator-only
configuration readback. Standalone installation takes the core's passive
`ServiceInstallationInput`:
shared `ServiceConfigurationInput` plus required explicit Caffeine project,
completion verifier and trusted certificate uploader. The
host binds owner to its actual service and project to the installed local namespace,
validates before allocation and retains/revalidates the mapping in its current v1
record. Configuration readback includes the project. This host init/schema hard cut
requires a minor release and cross-release reinstall, without a migration path.
It establishes no provider project assignment. This host exports no provider-fact
substitutes; actual restricted issuance derives only local prerequisites. Management snapshot tests
show that loading an old snapshot bypasses these hooks, restores its heap owner and
can clear a later fence, undo revocation and lose later upload reservations. Snapshot
loading remains unsupported for operation; `fenced = false` proves no independent
freshness. Restricted local eligibility can also return after rollback, so old
snapshot activation is outside the accepted contract. See the historical
[recorded recovery evidence](evidence/caffeine-probes/local/2026-09-30-snapshot-01/summary.json).
Operational recovery and remaining provider/operator integration are still open.

### Standalone ingress decoding

The standalone adapter's `ops::bounded` owns decoding for typed endpoint inputs:
4 KiB for ordinary requests, 16 KiB for installation and 128 KiB for manifest
preparation. It rejects total bytes before decoding, then sets 2,000,000 Candid
decoding units, 1,024 skipped-value units and 64 type-table entries. These are
decoder work counters, not IC instruction or paid-provider spending limits.
Its header-byte ceiling equals the corresponding whole-message ceiling. A valid
header cannot exceed that ceiling independently of the earlier whole-message
check; tests must not relax production bounds to manufacture a separate failure.

The existing standalone PocketIC suite exercises structurally valid extra text
and absent optional-record types, under the byte ceiling. Each payload first
round-trips with only its targeted decoder budget relaxed. Actual query/update,
manifest, certificate and failed-reinstall refusals then preserve owner state;
small extra-argument controls reach maintained handlers and valid initialization.
Certificate assessments carry a fresh timestamp, so unchanged permission and
blockers are checked independently of that timestamp.

A dense manifest with 65,379 empty name/value pairs fits just under 128 KiB and
uses between 1.8 and 2 million decoding units under locked Candid 0.10.37, with
no skipping. It reaches the maintained typed `UploadManifestFailure::Limit`
without stable mutation. Independent work-quota exhaustion has not been
reproduced under current byte/type/skipping limits; this case is not proof for
every Candid subtype or a reason to remove the work ceiling. Any additional case
must demonstrate a valid byte-bounded input, isolate the exhausted budget and
preserve the same trust/state boundaries without production test hooks.

### Library host integration

This repository owns the storage core and standalone canister. Consumer frameworks
own their wrappers, deployment lifecycle and integration tests externally. The
library exports shared DTOs, handlers, immutable installation configuration and
explicit memory requests. Linking it registers no endpoints, bootstraps no memory
and takes no lifecycle ownership. A library release does not qualify a consumer's
wrapper or a deployed service.

The owning canister authenticates the actual caller and service, bounds transport
and decoding work, and delegates to shared workflows. Tenant, operator, uploader
and verifier roles remain explicit; framework activation and controller status
grant none of those roles. Provider request/callback contracts have one core owner,
and clients use the service API rather than defining a provider protocol.

The host declares the installation configuration key and sixteen shared service
requests within one allocation authority, bootstraps one ic-memory runtime, validates
all installation inputs and calls `ServiceInstallation::install` synchronously.
It supplies actual service identity, project, verifier and compiled release.
Publishing a partially constructed owner or initializing through deferred work
is unsupported. Stable-memory failures must propagate for IC transaction rollback.

Restoration calls `ServiceInstallation::open` synchronously before deferred work.
All four owners remain inspection-only and fenced. Missing or corrupt grants,
configuration or history cannot be repaired or reset away. A framework's active
state is not freshness evidence and cannot clear these fences. Cross-release
transitions remain reinstall-only, after installation obligations are preserved
or discharged; same-release interruption recovery remains required.

The standalone adapter provides the maintained endpoint/configuration contract.
Local PocketIC fixtures exercise service state, rollback, authority and lifecycle
with labelled substitutes. Full provider qualification, consumer acceptance and
operational restart remain open. Historical framework implementation observations
are retained in the evidence record; the removed wrapper is not a maintained API.

## Design inputs and assumptions

The maintainer requested a fresh review on 2026-09-26, including choices made
here. Toko supplies concrete
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

The package roles are the shared core, standalone host, native CLI and private
browser client. This repository owns service workflows and provider bookkeeping.
Consumer frameworks own deployment wrappers and integration tests externally.
Operator workflows and diagnostics remain part of service acceptance; removing
framework code here does not waive authority, accounting or retirement requirements.

Name one concrete application and accountable consumer owner. Its journey is:
an authorized tenant uploads a bounded object, resumes after interruption,
reads and verifies bytes, and releases the reference through confirmed provider
deletion and billing cessation. Exercise it against the standalone service and the consumer's own wrapper using
the same shared blob API and tenant rules.

Classify each behavior as existing behavior preserved, a safety correction
required for extraction, or a new capability deferred. B2 is bounded by this
journey and necessary corrections; optional ambitions do not gate extraction.
Shared cross-tenant deduplication, generic provider plugins, multi-Fleet indexing
and new confidentiality guarantees are deferred. Pre-1.0 cross-release migration
is excluded: retire installations with surviving obligations accounted for, then
reinstall against the current contract.

## Recovery scope — maintainer decision, 2026-09-30

Keep one authoritative storage service with its existing local durable journals
and shared handlers. The maintainer selected the simplest architecture until a
clear use case justifies expansion. External write-ahead journals, dedicated
recovery-controller canisters and additional metadata calls are deferred. The
synchronous certificate commit/reply boundary remains the current contract.

Prioritize current-state durability, exact-operation receipt recovery and
state-preserving same-release lifecycle work. The maintainer selects ordinary
current-instance upgrades as the first supported active recovery path on
2026-10-02. The service stores the actual IC installation version immutably in
the current immutable record. Restoration still validates all owners synchronously and
enters their fences before any deferred work.

The immutable installation record uses the frozen layout identity
`ic-blob-storage/installation:platform-anchor-funding-credit-index-renewal`, independently of its exact library
release. The `blob.configuration.v1` grant key identifies the existing allocation
slot, not a compatible record layout. A mismatched identity refuses before opening
the service owners; an undecodable layout traps without initialization or repair.
Same-layout, same-release reopen still validates every owner and enters all fences.
This is a stable-format hard cut: retire existing installations with complete
object/effect/balance/billing disposition before a fresh reinstall. Do not upgrade,
reset or reinterpret the frozen 0.6.0/0.7.0 live owners with this reader.

The installed operator can invoke `blob_resume_current_instance()` without
supplying evidence. Shared workflow obtains one replicated IC `canister_info`
reply, requested history bounded to twenty changes, thirty-second wait and
64 KiB decoded-input bound. It requires coverage through installation, ordered
versions and no subsequent snapshot load, state replacement or unknown change.
Ordinary upgrades and controller changes preserve the state boundary. The proof
is sealed, consumed once in the same callback's executing version and bound to
the actual canister and retained anchor. Operator and installation are rechecked
after the await. Activation changes only owner fences; it never modifies IDs,
reservations, journals, receipts, balances or liabilities, and dispatches no
provider effect. An expired history window is not rotated or replaced by a
local counter. Hosts own their explicit lifecycle/endpoint and heap-version guard.

This path qualifies the maintained host's ordinary same-release reopening of its
existing journals. It cannot attest to arbitrary host-code stable-memory writes
or authorize copying an older backup into those journals during an upgrade.
Such backup replacement remains inspection-only regardless of management history;
consumer wrappers must enforce and qualify this same lifecycle boundary.

The standalone host observes actual platform version before each owner access.
A reversal or gap greater than one fences every owner. Queries conservatively
report that fence. A later update captures its caller before any await; if this
was an active owner, it independently checks IC continuity once before delegation.
Successful continuity preserves that active instance without modifying journals.
Already-fenced upgrade restoration never activates through this preflight and
requires the installed operator's explicit recovery. Callback completion also
retains the version checkpoint on provider-reply refusals. The request-to-callback
version must advance exactly once; intervening execution or management changes
invalidate potentially stale history. Certificate exposure and reply still commit
synchronously in one callback, with no await after exposure. Stop/start preserves
records and uses the same continuity checks. Actual
PocketIC loads restore old heaps without upgrade hooks, yet enter this fence
before certificate eligibility or mutation. A snapshot load in IC history refuses
activation even when the restored heap forgot the later operation. Keep older
snapshots/backups inspection-only and require complete independently surviving
reconciliation before any future active path. A local counter, elapsed time,
operator assertion or missing root is insufficient.
Uncertain effects, balances, provider objects and continuing billing remain retained
obligations. Older-backup activation remains an open requirement. This local
current-instance qualification does not upgrade frozen 0.7.0 installations or
establish provider qualification. The required record/candidate and lifecycle
hard cut joins the next minor release; cross-release transitions are reinstall-only.

Revisit the architecture when a named consumer actually needs active restoration
of an older storage-owner backup after later external effects, and available
provider/consumer evidence cannot close the missing inventory. Evaluate that case
and its costs before selecting extra authority or replication. A restored consumer
reconciles against the current storage owner; it does not roll that owner back.
The earlier source/design review is retained as historical evidence in the
[probe ledger](evidence/caffeine-probes/README.md); it is not the implementation plan.

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

The durable shared receipt boundary (`workflow::references::receipt`,
`dto::reference`) authenticates actual tenant/service and binds the complete
original upload and reference operation. Its `blob_reference_receipt` query uses
explicit `Absent`/`Found` variants; an incompatible payload cannot become absence
through Candid optional coercion. `ReplicatedReferenceClient` sends one bounded
replicated call to the selected service with no attached cycles, checks the exact
returned request and preserves stored inner failures. Ordinary IC fees apply.
Inspection works under suspension and restore fences and after settlement, but
neither absence nor historical success proves current liveness or permits retry
of an uncertain operation. Linking exports no endpoint; adapters own actual
context and ingress limits. The consumer still owns publication and its outbox.

The local saved-intent CLI uses this same endpoint and bounded decoder for both
standalone and durable storage. It retains all independent original identities,
including the first reference, and separates service refusals from absent receipts
and recorded transition failures. Its filesystem journal preserves exact intent
but supplies neither fresh allocation nor surviving restore authority. Transport
remains PocketIC with a simulated caller; no production authentication or dispatch
is provided. The transient fixture's private receipt endpoint is removed.

The shared `workflow::references::status::inspect` exposes tenant-only
`blob_reference_status` in standalone and the durable storage fixture. Its request
contains the complete original upload and exact positive reference ID, without a
mutation action or operation. It echoes that request and returns current local
liveness plus the same owner's restore fence under one synchronous borrow. Missing,
changed or unconfirmed uploads reject; never-retained and released references of a
confirmed upload return non-live. Suspension, settlement and restoration preserve
inspection. This indexed read creates no reference, receipt, quota reservation or
provider effect. A successful retained receipt does not imply a live reference;
live status does not imply active enrollment, serving authority or a cleared fence.
The storage fixture's private boolean endpoint is removed.

Native `blob-storage reference-receipt` and `reference-status` sign these existing
queries as the explicit tenant using saved binary Candid ReferenceCommand and
ReferenceStatusRequest respectively. Shared request encoders apply the maintained
identity/declaration rules; file limits, exact service/namespace and tenant checks
precede signing and transport. Native PEM principals must be actual tenants;
configured operators/controllers cannot impersonate a tenant, and canister tenants
use their existing ReplicatedReferenceClient. Each invocation sends one signed
query with a 30-second deadline, 256 KiB HTTP ceiling and 4 KiB input/reply bounds.
The receipt JSON retains original success/recorded failure/absence, labelling
liveness and fence as not observed because this endpoint carries neither.
Status independently reports current local liveness and restore fence. Refusals
remain typed errors; observations including recorded failures exit successfully.
There is no combined snapshot, provider availability evidence, publication lease,
mutation, local journal write or retry/recovery authority.

`ReplicatedReferenceClient::status` uses the same authenticated single-call transport
as receipt inspection and mutation, with separate exact status-request validation.
It bounds reply bytes and decoding, checks the full echo and preserves the independent
live/fenced flags. Remote refusal never becomes false/absence. No journal, retry,
publication lease or operational recovery authority is created by this read.

Native `submit-reference` uses that same exact command and bounded mutation-reply
decoder. It validates the tenant/service/namespace before signing, atomically
claims a new private directory and synchronizes the canonical command, signed
update and bound dispatch intent before one request. The saved outcome distinguishes
an exact recorded receipt (including an inner transition failure), pending,
typed service refusal and uncertainty. There is no polling, automatic retry,
identity allocation or provider request. Existing/partial claims refuse even if
empty; use the saved command for passive receipt reconciliation. A file copy,
new directory, absent receipt or expired signed ingress supplies no freshness/
retry authority. Consumer transaction/outbox and durable production coordination
remain separate requirements.

Managed signed tenant evidence combines this command and receipt/status queries
with actual signed verifier completion over labelled ten-byte local content.
Acknowledged/dropped/pending reference updates recover exact original results
without resend. A stored unknown-reference failure remains a failure; capacity
and suspension preserve releases. Exact historical replay does not revive dead
references. Last release leaves logical/reserved bytes zero and physical/liability
bytes ten. Same-release restoration fences all owners and mutations while
historical success/failure and dead-reference inspection remain passive. This is
local metadata/client evidence, not a deployed provider deletion/settlement or
production consumer/outbox acceptance run.

Standalone signed submission evidence preserves the production host's actual
boundaries: unknown content, admitted/prepared but unconfirmed content, and
same-release restoration. A typed mutation refusal is retained as refusal.
Dropped/pending acknowledgment remains uncertain/pending if exact receipt
inspection itself refuses; lookup error is neither original outcome nor absence
and cannot authorize resend. Refused retain/release and passive reads preserve
all stable bytes and the full reservation; all four owners stay fenced after
restore. No exposure/completion bypass was added to the standalone artifact.

The existing bounded consumer probe also composes canonical admission/reference/
descriptor clients with the managed service as an actual canister tenant. Its own
durable application record holds asset intent, publication, tombstone and release
outbox; there is no second service journal or recovery controller. One fresh asset
and one reuse asset share ten-byte local fixture content. Callback traps preserve
original retain/release intents. Cleanup at reserved capacity during suspension
retains physical/billing liabilities. A release receipt is recovered after the
service is fenced: mutation rejection makes passive recovery observable. Application
restore preserves its own fence/tombstones/history. Cancellation during a bounded
post-descriptor hold prevents the delayed callback from publishing or admitting
new uses. These are controlled local application/exposure/completion substitutes,
not Toko, production outbox/serving acceptance, or deployed Caffeine guarantees.

Occupied application restoration now also preserves unresolved retain/release
outbox entries after committed service effects, in either upgrade order and through
repeated same-release upgrades. Exact asset intent, payload, cleanup identity,
tombstone and absent acknowledgment remain unchanged. Service receipt/current
reference inspection stays available under explicit tenant identity; a fenced
application cannot record that acknowledgment or resume dispatch/publication.
Restored registration, admission, withdrawal, cleanup, recovery and new-use calls
refuse before effects, preserving both owners' stable bytes. A retained published
view is history, not authority to serve or admit uses through a fence. The original
fresh reference remains live, retaining ten logical/physical/liability bytes;
release of the reuse reference cannot erase the other asset's obligation.
These controlled local restore cases grant no fence-clearing or operational
restart authority and do not qualify old snapshot activation.

The canonical `blob_apply_reference` update delegates to
`workflow::references::apply` with the same `ReferenceCommand` used for receipt
lookup. `ReplicatedReferenceClient::apply` is an explicit single dispatch;
consumers must persist intent before polling it. A rejection, unusable response
or dropped future does not establish non-execution. Preserve the exact operation
and reconcile its receipt; the client allocates no identity and never retries.
Responses distinguish original inner success/failure and exact replay. Fresh
retains require current active enrollment, releases/replays preserve cleanup under
suspension, and restored owners reject every mutation. Native and local IC evidence
cover bound responses, committed-but-unusable replies, reserved cleanup capacity,
rollback and restored inspection. Consumer publication remains a separate transaction.

The canonical `blob_upload_status` query in both the standalone host and storage
fixture delegates to `workflow::uploads::inspect`.
Its replicated client authenticates the service and validates the bounded response
against the full original `ReferenceUpload`. Only the actual tenant can inspect it;
uploader and controller roles do not supply that authority. Suspension and restore
preserve historical inspection. Confirmation records creation of the first
reference, even after its release or settlement. It grants neither current serving
authority nor permission to repeat an uncertain upload; absence does not grant retry.

`workflow::uploads::admission` supplies canonical `blob_admit_upload` and
`blob_upload_admission` inspection through `ReplicatedUploadAdmissionClient`.
Both bind the complete upload, uploader and exclusive expiry; changed permissions
conflict. Fresh reservations require an active tenant and available capacity.
Exact replay returns retained state without renewal, even after expiry or
suspension; restore rejects all admission mutations while preserving inspection.
Adapters supply actual service/caller/time and propagate stable-write traps.
The client sends once, bounds decoding and never retries or attaches cycles;
ordinary IC fees apply. Neither route issues certificates or contacts Caffeine.

The same client exposes `blob_revoke_upload` through the shared synchronous
revocation handler. It checks every original permission field before mutation;
changed uploader or expiry cannot withdraw another permission. Unexposed cancellation
releases reservation bytes while retaining lifetime identities. Possible exposure
remains charged, and confirmed references require their own explicit release.
Neither provider deletion nor billing cessation follows from local revocation.
The response binds the permission, requires `revoked` and reports whether it changed.
Suspension/expiry permit cleanup; restore rejects even exact mutation replay.
Uncertain replies are reconciled using the existing exact permission query.

`workflow::uploads::manifests` supplies canonical `blob_prepare_upload` and
`blob_upload_manifest` inspection. Only the actual admitted uploader may prepare;
the tenant or original uploader may inspect. Both bind every original permission
field, including expiry. Raw leaf/header counts and original metadata bytes are
bounded before conversion; adapters separately bound ingress decoding. The existing
Caffeine root/length and metadata rules apply. Reordered equivalent retries preserve
the first declaration; header-name case is hash-significant and is not normalized.
The query reads one immutable bounded record without rebuilding its tree. Historical
inspection survives expiry, suspension, exposure, cancellation and restore, while
preparation enforces current activation, time, phase and the restore fence.
Explicit Unprepared/Prepared variants and bounded reply decoding reject malformed
data, changed permissions and inconsistent roots. A mutation acknowledgment must
contain a prepared declaration matching the submitted leaves. Save intent before
dispatch; an unusable acknowledgment requires exact inspection and never supplies
authority for repeating an uncertain provider effect. This is declaration evidence,
not verified file bytes, completion, a certificate or publication permission.

`ReplicatedUploadManifestClient` binds actual executing actor, tenant and service.
Preparation requires the admitted uploader; inspection also permits the tenant.
The client validates raw declaration bounds and root consistency before encoding
or dispatch, requires replicated execution and performs one bounded-wait call
without attached cycles or automatic retry. Ordinary IC fees apply. Response
budgets precede decoding but follow the separately platform-bounded CDK buffer.
The host must retain intent before polling; this transport owns no journal or
identity allocation and implements no provider upload or browser delivery policy.

`workflow::uploads::exposure` supplies read-only preparation inspection, guarded
synchronous commit and exact tenant/uploader history. Its host evidence binds the
full original permission and separately reports
explicit local namespace mapping, installed uploader trust, current-owner eligibility
and atomic durable commit. These facts must be established from the installed owner in the
current execution, never accepted from production ingress. Timestamp mismatch blocks
exposure; equal time is not proof of freshness or provenance. All missing host
prerequisites remain visible. Commit rechecks current uploader, bound manifest,
activation, expiry, phase and current restore fence, then writes possible exposure
before any later effect can escape. A preview is not an authorization token.
Historical inspection survives revocation and restore but cannot authorize issuance
or an uncertain retry. A committed exposure rejects repetition even after a lost
reply. The lower-level owner operation remains bookkeeping only. The shared gate
issues no certificates, authenticates no provider evidence itself and adds no state
schema or memory grant. Its IC evidence uses labelled host-fact substitutes.

`workflow::uploads::certificate` connects that gate to the reviewed Caffeine
root-only update reply. `resolve` authenticates the actual uploader and recovers
the complete original permission through bounded index reads. `issue` re-resolves
and rechecks current facts, then commits exposure before constructing the plain
`{ method = "upload"; blob_hash }` response. The adapter must return that record
from a synchronous ingress update, reject/trap failures and propagate write traps;
it must never wrap the provider wire reply in Result, await or issue through a query.
The IC supplies the ingress response certificate. Linking the library exports no
endpoint and neither handler sends a provider request. The local storage fixture
defaults to refusal and requires operator-configured, explicitly simulated facts.
Its response/rollback tests do not prove gateway acceptance, production evidence
acquisition, certificate replay/lifetime, namespace or pre-charge enforcement.

Shared certificate `inspect` returns the original permission, host assessment time
and every missing prerequisite as passive DTOs. It applies the same local authority,
phase and evidence-binding checks without writing exposure. Even an empty blocker
list is only a snapshot; issuance must recheck current facts and permissions.
The shared exposure-blocker conversion also serves the local fixture, replacing its
private duplicate representation.

Native `blob-storage certificate-assessment` uses the uploader's signing identity
and explicit replica trust to query this boundary once. Its saved binary permission
must match the expected service/namespace/uploader before transport; the shared
bounded codec compares every original field in the reply and retains typed
refusals. Duplicate blockers, oversized or malformed replies reject. Full-width
identity and host assessment time survive in JSON. Neither a blocked nor an empty
assessment permits issuance, retry or recovery activation, and the command makes
no update or provider request.

Standalone exports the canonical certificate update and the uploader-only
`blob_upload_certificate_assessment` query, both with bounded root ingress. Host
evidence is constructed internally from the retained permission and execution
clock; callers cannot submit qualification flags. Atomic local durability is
established by synchronous shared commit and trap propagation. Deployed pre-charge
limits, namespace provisioning/enforcement, replay charging and independent recovery
readiness remain unqualified and outside the accepted restricted issuance contract.
The host now derives local prerequisites through the shared installation owner,
issues only for an admitted/prepared reservation within installed quotas and
explicitly installed uploader trust, and commits possible exposure before replying. An empty query does not
reserve issuance. Its current v1 record/readback includes that uploader identity.

Headless Rust tests obtain the actual v4 HTTP ingress certificate using a signing
identity, then verify the IC signature/delegation/time and exact request ID under
the owned PocketIC root key. They bind the saved envelope, uploader, service,
method and root; certified rejection is not a successful upload reply. Read-state
recovery obtains the historical reply without another issuance call or state
change, even after local revocation. Such recovery neither renews permission nor
authorizes gateway use/retry. Production browser integration, consumer cancellation/intent
storage and deployed provider acceptance remain independent requirements.

The reusable `clients/browser` certificate transport accepts caller-supplied
identity, IC trust and atomic durable intent storage. It snapshots original
permission and full-width operation identity, binds stored intent to the IC origin
and root key, and retains explicit project/bucket before issuance. Transfer derives
the SDK namespace from that binding; changed values conflict across setup/reopening.
Native `upload-inputs` requires these original values, the frozen
`ic-blob-storage/upload-inputs:original-preparation` format identity and a complete installation
carrier. Shared validation checks proposed service/namespace/project/trusted uploader
and resource bounds, preserves exact init bytes and checks UTF-8/header bounds.
This does not observe actual installed state or remaining capacity; neither offline
validation nor local storage proves provider provisioning.
The client rechecks the envelope's uploader/service/method/root before
recovery. It issues once after durable claim or reads historical status without
redispatch. Cancellation cannot recall an already claimed request, renew permission
or free service capacity. The private source package contains no test identity,
provider wire implementation or allocator. Its maintained bounded IndexedDB journal
implements the [store contract](../clients/browser/README.md); consumer integration,
environment selection and restore/eviction qualification remain separate.

The private Chromium fixture imports that transport and the maintained IndexedDB store, selecting two slots for full
permission, exact signed envelope/request ID, dispatch phase and cancellation.
Transactions finish before fetch and serialize competing tabs; aborted writes
dispatch nothing. Reload/read-state recovery preserves original identity, while
late verified replies cannot clear cancellation. Response bodies are bounded before
SDK decoding. Its tests reject forged/unrelated proofs, conflicting saves and
capacity exhaustion without erasing retained history. This is local browser
evidence, not an eviction/power-loss/backup guarantee or qualified consumer environment.
The journal independently refuses missing/conflicting stores and malformed history,
retains fixed lifetime capacity and serializes strict commits across tabs. Actual
graceful browser-process restart preserves cancellation and gateway requests; no
replay, reset or automatic replacement is available after missing history.
The certificate guard itself sends no gateway request and performs no automatic
provider retry or publication transition.

The subsequent browser composition now delegates upload hashing/chunking and HTTP
formats to the published Caffeine 1.1.2 client. The certificate guard supplies its
existing HttpAgent constructor hook. A pinned local patch exposes static prepared
manifests and per-client transport controls; the fixture disables retries and
uses only an owned HTTP gateway substitute. Preparation snapshots bytes before
awaiting, yields an immutable manifest view and consumes its original handle once.
These ephemeral handles are not durable operation identities or recovery authority.
Failed/aborted transfers preserve local service exposure, not confirmed completion.
`ops::caffeine::preparation` converts the upstream prepared JSON into the existing
declaration with caller-selected buffering and manifest limits. Metadata and root
checks reuse the existing model; redundant nested tree nodes are not validated.
No file bytes, admission or authority are provided by that decoder. The local
browser fixture now signs admission to the existing consumer canister, which retains
the exact asset/permission and admits as the tenant. The browser separately signs
preparation directly as the uploader. Foreign identities and direct uploader
admission are refused. Opaque Candid uses the existing Rust schemas and validators;
this is local caller-boundary evidence, not Toko authentication or a production API.
The browser then attempts registration through the consumer; unconfirmed exposure
cannot publish even after successful HTTP. Explicit consumer cancellation followed
by tenant withdrawal preserves the exposed reservation and browser request history.
Browser cancellation, consumer tombstones and service withdrawal remain distinct.
The separate browser gateway guard now commits request fingerprints before fetch
through caller-owned storage. Its execution token and bounded request history share
the certificate/cancellation transaction: uncertainty blocks continuation and a
recreated hook cannot restart a transfer. Complete bounded HTTP replies are retained
as history, including after cancellation; they do not establish provider completion.
The local IndexedDB fixture demonstrates claim/observation rollback, cancellation
races, lost replies and reload fencing. Production consumer admission, persistence,
provider reconciliation and deployed qualification remain required; no new provider
wire implementation is introduced.

The private `blob-consumer-probe` exercises consumer transactions separately from
the storage owner: exact bounded intent and reserved cleanup operation before
dispatch, dependency checks, atomic tombstone/publication changes and restore
fencing. Existing content uses explicit retain receipts; fresh uploads inspect
completion and use the first reference without allocating an extra retain receipt.
Publication still obtains an exact live-reference descriptor and rechecks its local
tombstone. Late completion after cancellation remains recoverable for release under
suspension. Local IC tests cover callback traps, cleanup and fenced upgrades.
The fixture persists the complete permission and asset intent before sending
canonical admission from its own canister. Typed refusals remain recorded; unknown
acknowledgments are inspected under the original permission without redispatch.
Cancellation and restore preserve that uncertainty. The consumer now dispatches
revocation only after persisting its tombstone and confirming original admission;
unknown admission must be inspected first. Revocation acknowledgment is independent
of reference release. Lost replies block redispatch and use exact inspection;
an unrevoked observation leaves withdrawal pending.
The same private fixture can run as a separate uploader canister. Two lifetime
manifest-intent slots retain exact request, dispatch state and accepted declaration
or typed refusal inside its existing bounded record. Unknown acknowledgments block
redispatch, including after unprepared inspection; stale/conflicting results cannot
replace accepted history. Uploader cancellation retains a permanent
local tombstone without clearing intent, uncertainty or accepted/refused history.
Dispatch refuses cancelled intents; exact saves cannot reopen them. Inspection and
late acknowledgments may retain historical preparation while preserving the tombstone.
This local cancellation neither exercises tenant authority nor changes service
accounting. The application must separately cancel the tenant asset and withdraw
its permission; exposed late completion still requires reference release.
IC tests hold the uploader acknowledgment across both cancellations and tenant
suspension, checking unexposed cleanup and continuing exposed obligations.
Upgrade validates the current v1 schema synchronously and fences mutation/reconciliation.
The fixture schema is replaced directly;
cross-release transitions remain reinstall-only. Three-canister tests join tenant
admission, uploader preparation recovery, first-reference registration and cleanup.
Exposure/completion retain private fixture controls. Production browser/headless
integration and intent storage, Toko's asset schema, production scale, actual provider
completion and operational recovery remain open.

Each active reference reserves one future release receipt. Admission enforces
`retained receipts + active references <= receipt limit` after staging the
operation, so receipt pressure cannot consume the capacity needed for final
release. Staging now computes one private reference transition without copying
the retained reference map; capacity is checked before its receipt and mutation
are published synchronously. The plan cannot escape the model or survive an
intervening mutation/await. Exact retries still work at capacity. No receipt eviction
or timeout exists. The workflow must authenticate before receipt access as well as mutation;
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
settlement path deletes history or frees lifetime slots. Private global/tenant totals
change through the same owner as object insertion and journal mutation. Each mutation
replaces that entry's contribution, including a receipt recorded for a failed
lifecycle operation; retries and outer rejections do not add charges. Usage reads
do not scan object history. These are transient indexes, with no public counter
mutation, import or recovery authority.

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

The shared admission owner also supports tenant-scoped content discovery by root,
with explicit service context and provider namespace. It returns the original
upload request plus current pending/cancelled/confirmed lifecycle using the retained
index. Unknown and foreign roots both return no object; absence is not global
availability or permission to upload. Operators/uploaders have no implicit tenant
discovery authority. Suspended tenants retain inspection access. This is a passive
transient-model API, with no persistent protocol or provider guarantee.

For overlapping consumer releases, retain a distinct reference to a live object
before publishing the new release. Persist the exact request before dispatch and
recover its receipt after a lost reply. A recorded success is historical, so also
check current reference liveness under the consumer's registration/tombstone
coordination; neither a root lookup nor a receipt prevents a concurrent release.
An abandoned publication must release its acquired reference with its own exact
request. Receipt admission preserves capacity for existing references' cleanup,
but retained identities/receipts are never reclaimed. Final release immediately
queues deletion: any retention horizon must keep a live reference until it ends.
Once queued, content cannot be retained again, and its provider root cannot be
reallocated even after settlement. Reintroducing deleted media remains an M1
provider/identity decision; this lookup does not supply that missing contract.

Reference capacity observations distinguish unused lifetime reference IDs,
unreserved receipt slots and slots reserved for active references' release.
Each fresh distinct retain needs one reference ID and two receipts: its retain
result and its future release. The reported fresh-retain count is the lesser
bound, or zero once deletion queues. Failed admitted lifecycle operations consume
receipts too; exact retries do not. The shared service owner authenticates the
tenant before disclosure, including after suspension. Counts are current history
headroom, not enrollment permission, a reservation or a provider guarantee.

`blob_reference_receipt` exposes the original exact operation result through the same
service/tenant/namespace/object and payload checks used by mutation replay. It
never applies a request or consumes a receipt, including when the result is
absent. Suspension, history exhaustion and settlement preserve inspection. A
recorded lifecycle failure remains a failure, and historical retain success is
not current liveness. Unknown roots and conflicting identities are errors, never
absent receipts. The local saved-intent tool demonstrates this query path without
dispatch; its mutable filesystem artifact supplies neither fresh allocation nor
surviving operational recovery authority.

For fresh uploads, `blob_upload_capacity` observes the tighter tenant/global
headroom in each independent dimension: lifetime operation/root slots, concurrent
reservations, lifetime manifest leaves and bytes. Byte headroom is the minimum of
tenant logical, global physical and global continuing-liability headroom, including
unconfirmed reservations. Enrollment and per-object size/metadata limits accompany
the observation, along with the independent restore fence. Only the exact enrolled
tenant can inspect this scope; suspended
tenants retain visibility without regaining admission authority. Shared contention
affects the returned headroom, but other tenants' identities and records stay private.

The read uses maintained totals without scanning history or changing state. All
dimensions must fit together; the counts are not a promised number of uploads or
a reservation. Cancellation can restore byte/concurrent capacity without restoring
lifetime slots. Logical release and physical deletion cannot erase continuing
billing. Combine this view with tenant content discovery and exact reference
capacity for reuse; absence still does not prove a root is globally unclaimed.
`workflow::uploads::capacity::inspect` serves this boundary in the standalone host,
durable storage fixture and transient admission fixture. All use `TenantScope` and
the same maintained DTOs and conversion. The old private capacity interface is
removed. The local inventory tool uses the shared response and reports a restored
service as blocked despite positive headroom; this is not a provider guarantee or
production publisher authentication.

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

### Current persisted boundaries

The installed owners below use the current v1 records. Their exclusive memory
grants remain independent of a consumer framework's memory IDs and store layout.
The maintainer selected `ic-memory` as the sole allocation owner shared with hosts
and IcyDB. Core stores use its re-exported stable collections, and the
integrating host owns bootstrap, policy, grants and bucket profile. Cross-release
schema transitions are reinstall-only, subject to retirement of existing obligations;
the dependency choice does not close provider and recovery qualification gates.

The pending 0.15.0 contract selects ic-memory 0.27.1 through the library's public
re-export. Its allocation ledger retains current ownership and latest schema
metadata, without per-upgrade/schema audit trails or observation timestamps.
Hosts using those removed APIs or diagnostic DTOs must update their callers and
fixtures. The dependency's earlier logical layouts cannot be reopened, despite
its current envelope still using version 1. There is no dual reader, migration
or automatic reset. Do not clear allocation ID 0 to bypass failed recovery.

Funding intents additionally require the current credit record and accounting
requires `credit_confirmed`; absent fields refuse, with no legacy reader. The
installation format above changes alongside these owners, even while development
artifacts still compile as 0.14.12. Update hosts, passive DTOs/codecs, tools and
fixtures together. Frozen prior Wasms and records retain their original identities.

Before a cross-release reinstall, apply the installation retirement contract and
[operator runbook](retiring-installations.md):
preserve or discharge provider objects, uncertain effects, balances and continuing
billing obligations. Keep the frozen old installation and its evidence with
their original tools until that disposition is complete. Same-release backup,
restoration fences and interruption recovery remain required under the new
contract; ledger simplification grants no authority to reactivate a stale backup.

The first implemented component is `StableTenantEnrollments`. A host supplies one
exclusive granted memory and a validated service configuration. A reserved map
entry binds schema v1, service, operator, namespace and lifetime tenant limit;
individual bounded v1 enrollment records retain activation generation and active
state. Only these enrollment-specific configuration fields are persisted here.
The host must validate the installation/release and every other store before
service startup. No implicit memory grant, lifecycle export or endpoint is added.

Fresh installation requires unallocated memory. Reopening uses load-only stable
collection access, validates all retained enrollments within the configured
count, and fences every mutation. Missing/corrupt storage never becomes a fresh
owner. Binary corruption or storage exhaustion traps; a canister host must
propagate that trap for IC message rollback. The upload owner below incorporates
this store together with pending and confirmed lifecycle bookkeeping.

`StableRootClaims` uses two distinct host-granted memories for immutable root
claims and their reverse object index. Metadata binds service, namespace and the
lifetime root count; bounded v1 records preserve full-width identities. Claim
rules are shared with the heap model. Exact replay at capacity allocates nothing;
there is no deletion/reassignment API. Reopen validates the forward/reverse
bijection without rebuilding missing entries, then fences all mutations. Tenant
lookup hides foreign roots. Enrollment and byte-quota admission belong to the
enclosing workflow, not this index component.

The transient upload model supplies independent accounting comparisons for the
durable owner. It has no persistence or restore contract; production service
handlers use the durable owner. Keeping this reference model does not introduce
an alternate stable schema or a cross-release reader.

`StableUploads` exclusively owns these components plus permission, manifest,
charged-total, confirmed-lifecycle, reference, receipt and root/request index maps in ten distinct
host-granted memories. Admission binds
the exact permission and root while charging global and tenant totals; manifest
preparation retains original validated headers and leaves. Exposure bookkeeping
requires a prepared manifest and current uploader authority. Revocation releases
only unexposed reservation bytes; cancelled identities and leaf capacity remain
retained, and possible exposure remains charged. Each mutation is synchronous:
hosts must propagate traps so all writes roll back in the same IC update.

Completion transfers the existing reservation to a confirmed object and initial
reference without dropping charged bytes. Reference transitions and exact receipts
commit with maintained counts and byte totals. One receipt per live reference
remains reserved for cleanup, and every released identity/result remains retained.
Last release removes logical bytes; authenticated physical deletion removes
physical bytes; final settlement removes liability bytes. These transitions share
the heap model's rules and never scan/copy the full reference history on mutation.

Bounded v1 codecs retain configuration, permissions, totals, manifests and individual
lifecycle/reference/receipt rows. The
manifest codec permits at most 64 KiB and uses variable-size B-tree pages to avoid
maximum-value node allocations for small declarations. Installation rejects a
candidate whose per-object manifest envelope cannot fit. Reopen checks all upload
configuration fields, enrollment/claim/permission/manifest relationships,
confirmed states, reference/receipt counts and recomputed totals without repairing
storage, then fences every mutation. The host
still owns release identity and billing/provider configuration validation.

Root discovery resolves the separately persisted original upload ID, never assuming
it equals the object ID. The root/request index commits during admission and is
validated against every permission on reopen without repair. Tenant-authorized
descriptor queries copy one bounded manifest's original headers; unknown/foreign
roots remain indistinguishable. Reference-qualified descriptors additionally
require confirmed completion and the exact currently live reference, regardless
of another live reference or a historical successful retain receipt.

`scan` bounds inspected operation rows and returned results independently, with
trusted host-selected limits. Tenant scans restrict the storage range to that
tenant; service-wide scans require the configured operator. Cursors bind service,
namespace, scope, filter and last inspected tenant/request ID. Empty filtered
pages still advance. Results include all history, active uploads, pending physical
deletion or outstanding obligations, depending on filter. Physical deletion alone
does not remove continuing billing from the outstanding view. These queries work
while suspended or fenced but grant no effect/recovery authority. Start a new sweep
for insertions or phase changes behind a cursor; no snapshot/completion is implied.

`workflow::uploads::history::inspect` exposes this same scan through the maintained
`blob_upload_history` boundary. Passive DTOs carry full original upload, object,
incarnation and first-reference identities independently, current lifecycle and
the restore fence; responses echo the exact request. The standalone host limits
each call to 64 inspected rows and 32 results. The storage fixture uses the same
handler with single-row limits for cursor/failure evidence; its private scan DTOs
and endpoint are removed. No manifest, file body or provider evidence is returned.

The native `blob-storage upload-history` command signs one service-wide scan as
the explicitly named operator, with required filter and optional saved JSON cursor.
The cursor binds service, namespace, observer scope, filter and last inspected
tenant/upload ID before identity/transport access. The shared bounded request/reply
codec validates the complete request echo, full entry identities, tenant/upload
ordering, root/object uniqueness, selected lifecycle states, scan/result bounds
and forward continuation. Empty filtered pages may continue past unreturned rows;
empty or fenced observations do not become mutation/retry authority. JSON retains
decimal-string identities and scanned counts. No automatic traversal, provider
request, live reference claim or account/freshness reconciliation is performed.

`workflow::uploads::discovery::inspect` exposes indexed tenant discovery as
`blob_lookup_content` in the standalone host and both admission fixtures. Scope
checks precede visibility; operators, uploaders and controllers have no implicit
tenant authority. Responses echo the exact tenant/root request, preserve complete
independent identities via `UploadHistoryEntry` and report the same owner's fence
even for absence. Unknown and foreign roots share absence; unconfirmed, cancelled
and settled uploads retain their local phases. Suspension and restoration preserve
these passive reads. No manifest, file bytes, scan, reservation or provider effect
is involved. Neither absence nor a live observation authorizes allocation, retry,
publication or release of a restore fence. The private discovery endpoint is removed;
fixture descriptor/root-batch views remain separate consumers of their fixture DTOs.

`admission_capacity` and `reference_capacity` use maintained counters and indexed
identity reads, without loading reference/receipt histories or manifests. Heap and
stable owners share headroom arithmetic. Admission observes the tighter global/tenant
operation, active-upload, leaf and byte dimensions. Cancellation/settlement never
refund lifetime history; physical deletion alone cannot refund continuing billing.
Reference headroom reserves one release receipt per active reference and two slots
per additional retain/release pair. Unknown, foreign and unconfirmed roots expose no
reference capacity. Retired confirmed objects have zero fresh retains. Suspension
and restoration allow inspection, not fresh admission or removal of the fence.

The shared `workflow::references::capacity::inspect` boundary exposes these
reference counters as `blob_reference_capacity` in the standalone host and both
admission fixtures. Requests bind `TenantScope` and a fixed-width provider root;
responses echo the request, carry optional headroom and expose the independent
restore fence even for absence. Caller/scope checks precede root visibility.
Unknown, foreign and unconfirmed roots remain indistinguishable. Positive fresh
retains reflect counter/lifecycle limits only, not enrollment, identity freshness,
reference liveness or operational restore authority. All mutation checks remain
in force. Private reference-capacity DTOs, endpoints and duplicate conversion are
removed. The local inventory tool checks exact echoes and preserves a later fence
without treating its earlier capacity observation as continuing authority.

`observe_roots` requires the configured operator plus the actual service and explicit
installed namespace, even for empty input. The host must construct `ProviderRootBatch`
under trusted count/byte limits and separately bound decoding. Each valid position
uses indexed reads; no operation-history scan or manifest load occurs. Results keep
input order, duplicates and typed malformed-root positions. Known roots include the
exact original request and reserved, possibly exposed, cancelled or confirmed phase.
This operator view is distinct from a gateway callback: unknown/cancelled/settled
observations never authorize provider deletion, retry or allocation, and local
completion/settlement observations do not supply independent provider evidence.

`workflow::gateways::callbacks::observe_roots` separately composes the durable
registry and upload owner for scoped gateway observations. Both owners must have
the same complete configuration and match the explicit service/Cashier/namespace.
Actual service and current caller membership are checked even for empty, unknown
or malformed batches. Each known root's stored object binding passes callback
policy; inconsistent indexes reject the entire result. Both owners' restore
fences independently block these operational reads, while operator inspection
remains available. The synchronous handler retains no permit or borrow across
an await and reuses indexed lookups without loading manifests or histories.

Gateway results expose only local phases, unknown roots and malformed positions,
preserving order and duplicates without tenant/request/reference details. Tenant
suspension does not hide continuing obligations. Operator removal or a completed
membership replacement applies on the next call. These phases are not provider
liveness booleans, completion receipts or deletion permission. Provider semantics,
callback correlation, read-session generations and recovery remain separate gates.
The probe endpoint is explicitly a local fixture; no provider callback export,
automatic deletion or paid effect is installed by linking the library.

`workflow::reads::{capture,recheck}` adds current durable read authority. The
opaque host-retained observation binds original caller/service, provider scope,
selected gateway, root, full object lifetime and exact live reference. Both
owners must share configuration and remain unfenced; enrollment must be active.
Indexed reads avoid manifests and reference/receipt histories. Existing passive
descriptor inspection keeps its suspension/restoration behavior.

Gateway membership now owns a separate persisted read invalidation counter.
Successful adds/removals, including no-op edits, and successful complete syncs,
including identical lists, advance it atomically with membership. Failed writes
roll back both. Invalid edits/replies and sync begin/cancel leave it unchanged.
Exhaustion permanently disables new read observations without blocking revocation
or wrapping the counter. Tenant suspension/reactivation uses the independently
retained activation generation. Released reference identities cannot be replaced
by another live reference or a historical retain receipt.

Recheck uses the original context and target after an await, before disclosure.
It is repeatable and read-only: it reserves no session slot, consumes no callback,
authenticates no transport and verifies no bytes. Only the original exclusive
owners may be used; host installation identity and stale-instance exclusion remain
necessary. The internal v1 registry schema retains the counter on same-release
reopen under its fence.
Cross-release transitions remain reinstall-only, with no compatibility branch.

`ops::service::reads::StableReadSessions` owns three explicitly granted memories:
the configuration/high-water counter/global usage, one bounded row per occupied
session, and counters for tenants with occupied sessions. Trusted limits bound
global and per-tenant counts and reply-buffer bytes independently. At most 1024
active rows are supported; this is a representation envelope, not production
resource qualification. Each session reserves the full configured reply budget.
That budget does not account for all decoder copies or total canister memory.
Ordinary mutations touch only the exact row and maintained counters. Completion
removes its row and any empty tenant counter, retaining the global high-water ID;
completed read history does not accumulate. Exhaustion blocks admission without
blocking exact completion. Operator inspection pages are limited to 64 active rows.

`workflow::reads::sessions::begin` checks matching owners, all fences, current
authority and the declared chunk range, then commits the exact intent. No call is
sent. The original authenticated context and opaque ticket must stay with one
host-managed call. `complete` accepts only that exact occupied identity. It checks
all owner fences/bindings, rechecks disclosure authority and removes that returned
call's reservation atomically. Lost authority can therefore return an error after
releasing the slot; the host must refuse byte disclosure. A repeated or stale
callback cannot release a newer slot. No expiry, automatic retry or cancellation
path frees an in-flight, abandoned or interrupted reservation.

Reopen validates bounded retained identities, rows and recomputed global/tenant
totals without repair, then fences both admission and completion. A counter from
the same backup grants no independent freshness. Native tests do not supply IC
transactions; hosts must propagate stable traps. PocketIC verifies that admission
traps send nothing and callback traps preserve occupancy after the remote call
returns, including through upgrade. The fixture replaces its temporary busy flag
with these shared handlers and three additional grants through the same ic-memory
runtime.

`workflow::reads::chunk::read_chunk` now composes admission, one host transport
await and synchronous callback completion/verification. `ReadSessionAccess` borrows
the same owners only within synchronous closures. `ReadChunkTransport` supplies
normalized host-authenticated source and original request correlation, not a
library-selected provider wire or a principal trusted from reply data. The handler
checks source/root/index and decoded size, then the exact admitted leaf length and
domain-separated hash. One bounded immutable manifest record is decoded; its tree
and reference/receipt histories are not rebuilt or copied. Returned bytes are moved
into the verified result. Lost authority rejects before hashing. Ordinary returned
transport/binding/content errors settle their exact reservation; traps roll back
callback writes. A verified leaf does not prove whole-file completion or durability.

The local fixture now calls the existing source's `fixture_chunk` method and checks
encoded size before bounded bulk `vec nat8` decoding. Actual IC call targeting
authenticates its peer. This is explicitly a provider substitute; no deployed
Caffeine chunk interface is inferred. Encoded and decoded buffers can coexist, and
the platform/CDK initially buffers the response before the application size check.
Session budgets therefore do not qualify peak heap, instruction or provider costs.
Production transport, supported recovery and resource sizing remain separate work.

`workflow::reads::download::handle` serves
`dto::download::{DownloadRequest,DownloadResponse,DownloadFailure}` through the
canonical `blob_download_descriptor` update. It checks actual service/caller and
every nonzero identity before the upload owner's operational checks, without
allocating a session or fetching any body. It requires active tenant
authority, an unfenced owner and exactly the requested confirmed live reference.
Its host-supplied `CaffeineDownloadScope` binds an explicit owner/project mapping
to the installed local namespace. Owner must equal the service, independent of
payment account and tenant. Project text is bounded at 256 UTF-8 bytes before
copying, with no empty value, controls or surrounding whitespace; this is a local
representation bound, not a provider naming guarantee or a default assignment.

The response carries the original root, length and hash headers. Clients use the
canonical target builder for the relative Caffeine `/v1/blob/` request. Request
fields are individually percent-encoded. No
bucket or gateway origin is guessed. HTTP origin authority cannot be inferred from
the IC gateway principal list. Hosts must provision and retain the same project
mapping used for uploads, authenticate descriptor delivery, approve the origin and
redirect/credential/CORS/body policy, and coordinate the consumer reference with
publication/release. The fixture exposes an update-only local descriptor using a
labelled project; it is not a production endpoint or certified public release map.
Suspension/release blocks new operational descriptors, not access to saved URLs or
already downloaded bytes. Passive historical descriptors retain their inspection
contract. No provider GET, certificate, account operation or body hashing occurs
in the service descriptor workflow.

Standalone uses its immutable installed
project mapping under the same synchronous borrow as the upload owner; the probe
uses a labelled substitute. The existing retained descriptor moves directly into
the response, which echoes the full request, owner/project, declared size and
original headers. It returns no URL, origin, credentials or raw-digest assertion. Adapters
must bound ingress decoding and supply the installed serving scope; linking the
library exports no endpoint or lifecycle hook.

`ReplicatedDownloadClient` binds tenant, selected storage service and a positive
timeout up to 300 seconds. It checks actual canister identity and replicated
execution, then sends the update once with zero attached cycles. Platform fees
still apply. Actual IC targeting authenticates the response source. Bounded Candid
decoding validates full request equality, owner/project, declared content limits
and the shared upload metadata invariants before returning a descriptor. This
does not limit the CDK's earlier platform-bounded response buffer. Clients using
the standalone decoder must independently authenticate the service response.
No query/method fallback or automatic retry occurs. A refusal, timeout or decoding
failure returns no descriptor and never authorizes publication or a paid effect.
The local test uses a real tenant canister; browser authentication and a certified
public release mapping remain unimplemented. Copies can become stale across
the return await, so consumer reference ownership and release exclusion remain
necessary even after successful authenticated delivery.

Native tenant `download` uses the same descriptor decoder and canonical Caffeine
path, with an independently selected expected project and approved origin. One
replicated descriptor update authenticates the exact tenant/live reference; only
an exact bounded reply permits one provider GET. The maintained streaming verifier
checks declared length, original hash headers and root at complete EOF before
publishing a synced private `body.bin` without replacement. Partial bytes stay
unverified, and run reuse, redirects, retries and content decoding are refused.
This grants no public serving lease, future retention, confidentiality or billing
cessation. Signed local managed completion-to-file evidence uses labelled exposure
and content; standalone exercises its actual unconfirmed/restored refusals.
See [usage](operator-guide.md#download-a-verified-file).

The private storage probe exercises interrupted writes from admission through
completion, references and settlement, then same-release upgrade in every phase.
Changed-operator restoration fails without changing the old instance. Completion,
deletion and settlement APIs consume facts already authenticated by the trusted
host; the probe supplies explicit operator-only substitutes. No real certificate,
provider call or evidence qualification follows. Other durable provider-call
journals, deployed verified-read delivery, operational recovery and resource qualification
remain required.

`ops::service::funding::StableFundingJournal` owns a separate attachment allocation
through two exclusively owned, host-granted memories: exact intent history and
accounting with its bounded receipt-to-operation index. It binds service, operator,
Cashier candidate, payment account, namespace, initial allocation, immutable renewal
ceiling, positive reserve and lifetime history limit. Hosts
must provide a single journal for that allocation and propagate stable-write traps
inside synchronous IC updates. This component neither shares nor refunds upload
byte quotas and does not claim complete service economics.

Preparation reserves the full positive offer under an explicit operation identity.
The v1 intent also retains the `account_top_up_v1` method and exact optional positive
target balance. Shared `CashierTopUpRequest` encoding emits a present request with
a present explicit account; no caller-dependent account default is used. Target
balance absence stays absent and is distinct from the attached offer. Method and
full argument values are retained semantically; the frozen same-release encoder
reconstructs canonical bytes without permitting arbitrary methods or payloads.
The provider has no namespace or operation-ID argument, so those bindings remain
local and cannot establish remote idempotency or lost-response reconciliation.
New IDs must increase; exact replay compares the whole local scope and amount.
Ordering is not proof of freshness after restore. Prepared and uncertain intents
block later reservations. The first attempt marker persists possible dispatch;
every repeated marker rejects. No timeout, expiry, absent reply or lookup result
can authorize another attempt. Terminal observations accept only trusted-host
proof of enqueue failure or an exact unbounded callback refund. They require the
original intent and separately supplied running-service/original-target context;
identical replay is unchanged and conflicting observations reject. The context
must come from trusted transport, never reply fields or caller assertions. Matching
principals checks correlation; it does not authenticate the source by itself.
`mark_attempted` returns the canonical request only after persisting the marker.
Operator `request` inspection returns the same bytes even while fenced, without
granting dispatch authority. Changed target-balance options reject on every exact
intent lookup/mutation rather than silently changing the encoded call.
Unknown transport retains the entire attachment. Returned amounts release only
their own allocation; accepted amounts remain charged and never imply credit.

The [host funding contract](funding-credit.md) defines synchronous credit confirmation
and separate bounded allocation increases. Confirmation does not refund acceptance.
A host-authorized grant belongs to one exact latest credited intent, precedes the
next reservation and cannot exceed that acceptance or the installed cumulative
ceiling. Exact replay leaves every total unchanged; lifetime slots are never
recycled. Available + accepted + reserved/uncertain equals cumulative authorization.
Same-release reopen replays original offers and grants in order and validates all
receipt index bindings/cardinality without initialization or repair. Fresh
confirmation uses the existing owner's index rather than scanning all intent rows.

`workflow::funding::history::inspect` exposes the existing bounded journal traversal
as `blob_funding_history` through both the standalone host and storage fixture.
The shared operator scope binds actual service, namespace, Cashier and payer even
for empty ranges. Descending pages preserve full-width operation IDs, attachment
amounts, optional target balances and local transport phases. Responses echo the
request and retain the independent restore fence. The standalone host returns at
most 32 entries; each scan reads only bounded intent rows plus an index lookahead.
Changed cursor scopes reject, and new activity behind a cursor requires a fresh
sweep. No page proves provider credit, complete account activity or retry safety.
The fixture's private history DTOs, conversion and endpoint are removed.

Ordinary writes touch one intent and maintained totals. Shared model arithmetic
also reconstructs the complete bounded journal on reopen. Missing, orphaned,
changed or inconsistent records reject without repair. Reopened owners permit
operator inspection but fence preparation, attempts and all outcomes, including
late callbacks; they have no unfence/reset/eviction capability. Independent
surviving authority and a complete obligation source remain required for recovery.
The explicit `PreparedCashierTopUp` primitive constructs one owned unbounded IC
call from the canonical request. It captures the running service/original target
and this callback's exact refund before bounded reply decoding or another await.
An unpolled call can be consumed as positively unsent; actual enqueue failures
never sample an ambient refund. Call cost is measured before adding the attachment
so liquidity policy counts the full offer once. The host must persist the attempt
first, establish all activity/holds, then assess current liquidity and dispatch in
the same message. The primitive exports no endpoint or automatic retry. Unbounded
wait preserves attachment conservation but a stalled provider can obstruct upgrades.

The private storage probe composes this transport with the journal against a
labelled local Cashier substitute; its older bookkeeping controls remain explicitly
fixture-only. PocketIC covers exact request bytes, refund/reply independence,
liquidity refusal, receiver traps and callback-write rollback after acceptance.
`record_observation` retains the shared transport's structured result together with
its transport phase and allocation update. It independently checks the call's
original account, offer and optional target; service/Cashier checks still apply.
The bounded v1 intent record now stores normalized balance components, supported
provider errors, decode failure categories or the raw reject code, without wire
buffers or diagnostic strings. Response/phase mismatches reject; exact replay is
unchanged and conflicting observations cannot overwrite history. Recording a
response for already retained matching transport cannot release its return twice.
Transport-only evidence remains explicitly distinguishable from a retained reply.

`workflow::funding::outcome::inspect` now owns `blob_funding_outcome` in both hosts.
The configured operator supplies the complete original scope, operation, positive
offer and exact optional target balance; mismatched retained arguments reject.
The response preserves local phase, typed balance components/provider errors/decode
failures, and the restore fence. Workflow applies the existing reconciliation policy
to retained transfer facts and host credit receipts, independently of reply classification. Missing history
and missing structured replies remain distinct; neither permits another payment.
The fixture's private outcome DTO and endpoint conversion are removed.

Native `blob-storage funding-outcome` uses this existing operator boundary and
the shared validated request encoder and bounded reply decoder. Original operation,
offer and optional target must be supplied exactly, including the distinction
between no target and a present target. JSON separates absence, missing structured
response, exact refund, reported balance/error, conservative reconciliation and
the found journal's restore fence. Absence has no record and therefore no fence
observation; it must not be interpreted as an unfenced owner. Full-width amounts
remain decimal strings. One signed query has a 4 KiB decoded outcome bound and
grants no provider-credit, retry, payment or unfencing authority.

The [host-internal credit workflow](funding-credit.md) authenticates the live
operator and complete original intent before invoking trusted evidence acquisition.
It commits a full positive accepted amount and immutable evidence fingerprint
with confirmed accounting in one synchronous IC update. Exact replay is unchanged;
wrong amounts, receipt conflicts and reuse across local intents refuse. Accepted
cycles remain spent. Confirmation clears only that local uncredited obligation,
never unknown transport, allocation/capacity limits, external activity or fences.
No public ingress credit setter, extra journal or provider credential is added.

`workflow::funding::assessment::inspect` now exposes the existing preparation
policy synchronously through the shared `blob_funding_preparation_assessment` standalone endpoint.
The authenticated operator supplies the complete installed scope and a proposed
positive operation/offer/exact optional target. The host supplies no externally
asserted evidence: qualification is false and recovery, complete account activity
and spendable cycles are unknown. The passive boundary accepts no evidence fields.
It reports all independent missing facts alongside actual journal fencing,
retained/stale identity, lifetime capacity, unresolved allocation and exact local
attachment allowance. Changed retained arguments conflict. It allocates nothing,
reserves nothing and performs no provider call; fresh preparation and dispatch
remain unexposed. A successful report grants no permission and cannot become
cached effect authorization or establish independent identity freshness.

Native `funding-assessment` performs one signed query with exact request echo,
4 KiB bounded Candid decoding, 256 KiB HTTP and thirty-second limits. Full-width
numbers remain decimal strings; optional target absence stays absent. Scope/actor,
changed echo, malformed/oversized/inconsistent reports and untrusted replies refuse.
Local shared-core tests cover occupied funding history and uncertain reservations;
signed PocketIC journeys through both artifacts preserve occupied uploads and all
stable bytes through actual refusals and same-release fenced restore with the
Cashier stopped. This does not establish provider credit/retention or production
funding readiness. Managed query decoder control remains a separate framework gap.

`ReplicatedFundingClient` pins the actual executing canister and full operator
scope for one bounded replicated history/outcome call. It uses maintained method
names and DTOs, attaches no cycles and adds no retry, pagination loop, journal or
lifecycle ownership. Pure bounded reply decoders require independent service
authentication, check exact echoes, descending cursor progress and refund amounts,
and reject reconciliation inconsistent with transport facts. Refusal, absence and
restore fences remain visible; provider reports cannot establish credit. The host
still authenticates its own ingress. Local consumer-fixture evidence covers both
restorations and passive bytes, not a production operator application.

Operator `summary` checks the same explicit `FundingJournalScope` used by history,
including empty journals. It reads maintained accounting and metadata counts without
decoding intent rows, exposing lifetime capacity, the last retained identity and
the independent restore fence. Workflow applies `assess_uncredited_allocation`:
any accepted or reserved/uncertain amount remains unresolved, regardless of later
refunds, unsent attempts or reported balances. This summarizes the complete local
journal only; other installations, linked payers and direct provider-account
activity require independently established completeness. Zero local obligations
neither release the restore fence nor establish payment authority.

Shared `workflow::funding` adds guarded preparation above the low-level journal
bookkeeping API. Inspection authenticates the operator and exact scope/request,
then assesses current local obligations, identity, capacity and allocation against
separately scoped host evidence. Provider qualification, recovery, spendability
and complete external account activity remain independent requirements. Local
clearance cannot fill missing external evidence; external clearance cannot erase
local obligations or release the actual restore fence.
`prepare_new` re-reads these local facts and reserves synchronously without an
await. Blocked requests do not mutate storage, and the update accepts no saved
preview as authority. Host observations are ordinary trusted integration values,
not authenticated proof objects: the host must establish their scope, completeness
and freshness in the same execution, never accept them from ingress. The probe
deliberately reports unknown host evidence and cannot qualify itself through a
request. This handler only prepares an intent; it neither dispatches nor grants
later payment authority. Dispatch still needs current effect and liquidity checks.

`workflow::funding::attempt` inspects and marks the exact existing reservation.
Its host evidence binds every intent field, including the operation, offered
amount and optional target. Other-activity observations exclude only that exact
unattempted intent; available-for-offer funds include its still-unsent attachment
and retain every other liability. The service does not add a reservation back to
an arbitrary balance. Prepared state, the last retained identity and its complete
hold must agree. Earlier uncredited accepted amounts, external unknowns and the actual fence
independently block marking. Full history and exhausted unreserved allocation do
not charge this already-reserved intent again. Uncertain and terminal states
always reject another attempt, including fully refunded or proven-unsent results.
The synchronous update re-reads state rather than accepting an earlier inspection.
It either leaves storage unchanged or persists the first uncertainty marker and
returns the canonical request. This is not dispatch: the host must construct the
call after these writes, assess actual liquid cycles, call cost and complete holds,
then dispatch without an intervening await. Dropping the result does not prove an
unsent effect or confer retry authority.

`workflow::funding::dispatch::dispatch` now owns this composition. The host provides
synchronous `FundingJournalAccess` to the same installed owner, releasing its borrow
before returning. After authenticated exact request lookup and an actual running
service check, the handler invokes the host's observation closure once on polling.
Unknown liquidity holds remain an independent blocker even when first-attempt
evidence is otherwise complete. On admission, marking and all stable writes precede
the platform liquidity observation. The handler either consumes the unpolled call
as positively unsent or executes the canonical unbounded request once. It captures
and records that call's refund and structured outcome against the original intent
before another await. No pending permit or callback identity is supplied by ingress.
Callback write traps retain the full uncertain offer while remote acceptance can
survive; they grant no retry authority. Result settlement refers to local transport
accounting, not independent provider credit. The local probe supplies explicitly
synthetic host observations against a Cashier substitute. Qualified production
host evidence acquisition remains open; this handler creates no automatic payment
endpoint, timer or lifecycle ownership.

Operator `outcome` inspection works while fenced and supplies validated transfer
facts alongside the independent response. The probe workflow applies shared
reconciliation policy to the transfer and retained host receipt. Reported success never clears required
credit evidence; missing transport retains the full offered amount as potentially
spent. The v1 record schema is replaced directly, without migration or dual
readers; cross-release transitions remain reinstall-only. Production provider
authentication, independently verified credit acquisition, account-wide
uncredited activity, spendability and dispatch admission remain separate work before
a production call can use this journal. Local simulated transfers do not qualify
deployed Cashier behavior or authorize a paid trial.

- Object identity: an allocated object incarnation bound to service, tenant and
  provider namespace, with provider root and declared length as data, distinct
  from independently established stored size. A root is never the ownership key.
  The direct-upload service requires no whole-file raw digest. Initial design has no
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
production defaults from a consumer framework. Numeric deployment budgets and supported evidence/
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
| Library and host integration | Service maintainer and consumer owners | Shared core and standalone host live here; consumer wrappers/integration tests live externally and preserve the same authority/workflow contract |
| Tenant authority | Service maintainer | Exact tenant/actor bindings and denial cases; a digest or controller status grants no tenant authority |
| Identities and restore | Service maintainer | Recovery identity, non-reuse authority surviving older backups, stale-instance fencing and reconciliation before effects or admission based on stale accounting |
| Provider suitability | Service maintainer | For every paid/destructive operation: exact retry identity, authoritative completion evidence, retention horizon, typed uncertain outcome, bounded reconciliation, separate deletion and billing-cessation evidence |
| Accounting | Service maintainer | Deduplication choice, logical/physical quota basis, reservation/release timing, race-safe counters and ownership of costs until deletion and billing cessation |
| Existing obligations | Each affected installation operator | External objects, uploads, uncertain paid effects, balances and billing inventory; no-obligation evidence or completed owned decommission/disposition before reset |

Caffeine is the selected integration target; general production qualification
remains open beyond the accepted restricted prototype. Missing exact
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
