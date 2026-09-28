# Service contract — B1 draft

The maintainer has started the 0.2 service phase after 0.1.19. The
[delivery plan](roadmap.md) owns its milestones and current consumer decisions;
the constraints and evidence requirements below still apply. Candidate service
configuration validation composes existing local models without installing state,
freezing a stable schema or claiming that the B1 provider/recovery gates are closed.
The transient project-to-uploader admission model in the delivery plan additionally
owns exact permissions alongside the existing upload catalog. Root-only exposure
checks authenticated context supplied by the host; one-shot exposure, passive
lookup and revocation preserve uncertain reservations. The maintainer selected
direct browser-to-Caffeine upload after evaluating the cost of mandatory byte
verification. The shared owner binds a bounded manifest to the admitted root and
declared size; the service accepts no file chunks and has no raw-digest verdict.
A matching manifest does not establish actual length, provider completion or
billing. Before a real certificate escapes, the host still must qualify pre-charge
size/tree enforcement, namespace/replay rules, durable intent and recovery.

The shared owner validates raw metadata count/bytes before bounded header parsing.
Names must be nonempty ASCII HTTP tokens and unique ignoring ASCII case. Values
reject controls, line separators and surrounding whitespace, without rewriting.
Exactly one `Content-Length` with that spelling is required, containing unsigned
decimal digits without padding or leading zeroes and equal to the admitted size.
The configuration must fit that header for its largest permitted object. These
canonical casing/value rules are service choices stricter than generic HTTP;
the independent Caffeine hash primitives continue to implement the pinned format.
Reordered equivalent headers remain exact retries; changed or malformed metadata
cannot replace a bound manifest or alter accounting. This is not MIME/serving
policy or evidence of actual provider byte length.

Operator-managed enrollment gates fresh authority; suspension preserves cleanup
and accounting and reactivation invalidates old permissions. Global/tenant lifetime
manifest capacity is reserved at admission and never refunded with byte quota.
The unpublished PocketIC probe supplies actual IC caller/time, bounds Candid
work and measures a 10 MiB declaration through admission, manifest preparation,
retry and local exposure. Stop/start preserves the owner; unsupported upgrades
reject atomically, including skipped outgoing hooks. It has no stable schema,
provider effect or certificate response. Completion remains a separate trusted
host input and is not exposed as a browser command. See the
[upload-path decision](roadmap.md#upload-path-evaluation--2026-09-27).

This is an unresolved contract checklist, not a frozen service API or provider
suitability verdict. Freeze the decisions and evidence before B2 implementation,
subject to the explicit bounded exception below.
The bootstrap package name does not decide the final package split.

Client preparation now uses the additive `CaffeineManifestBuilder`: the shared
hash engine computes each leaf once and retains only an explicitly bounded list.
The local `prepare_upload` example reuses `validate_upload_metadata` from the
service, then requires exact length and successful EOF. Its computed digest/root
and leaves establish local consistency only. The client must preserve the source
bytes/metadata; no raw-digest admission requirement, provider certificate authority,
persisted operation schema or completion evidence is introduced.

The [consumer download direction](roadmap.md#consumer-download-verification)
keeps bulk verification outside the service canister. The additive
`CaffeineRootVerifier` and local stdin example check byte consistency against a
fixed root, length and original metadata; they do not authenticate a download
descriptor or establish successful provider completion. The example optionally
publishes the exact checked bytes to a caller-controlled local file after clean
EOF, without overwriting an existing destination. This is not production/browser
delivery or a crash-durable transaction.
The admission owner now retains original metadata and exposes a bounded borrowed
`content_descriptor` view through its existing tenant authority. Its lifecycle
field distinguishes prepared/uncertain/cancelled/confirmed/retired states; the
view grants no reference, serving permission or provider guarantee. The private
probe delegates the same query to this owner, without certifying its response.
`retained_content_descriptor` additionally requires confirmed completion and an
exact live reference for the supplied object incarnation. It performs one passive
owner read; a copied result is not a retain receipt or permission to publish after
release. Consumer publication/release exclusion remains a workflow obligation.
Production descriptor publication and browser delivery remain outstanding.

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
The local test scope also includes an unpublished PocketIC authority probe over
sample transient objects. Actual IC caller/service context reaches the shared
library policies and catalog; caller isolation, release replay and gateway
revocation have [partial evidence](evidence/core-primitives.md#pocketic-authority-probe-after-018).
This fixture does not select a production API or establish provider/persistence guarantees.
After 0.1.14, the maintainer approved a connected local upload/deletion journey.
The fixture uses the current source's certificate/liveness/deletion method shapes
with shared catalog/policy checks. Tenant-only certificate admission, conservative
protection of unknown/pending roots and gateway-only liveness are local proposed
semantics. In this separate integrity fixture, bytes match the manifest and raw digest before
certificate exposure. The 0.2 continuation extends this fixture to nonempty files
up to 10 MiB in ten chunks, with eight headers and 1 KiB of framed header input.
Admission and journal validation share that fixture envelope. It retains bounded
manifest/hash state across messages, discards checked bytes, and does not select
a production upload architecture or restore format. Tenant progress and exact
chunk retries preserve the verified prefix; a final raw-digest mismatch is
terminal for that declaration. Real IC callback rollback is exercised; gateway certificate
validation, provider storage, completion and billing cessation remain unqualified.
The same local journey reads individual chunks from a driver-controlled source
canister. It requires the bound tenant's live confirmed reference and current
gateway authority before and after the call, and checks bytes against the
admitted manifest before disclosure. Revocation/successful gateway sync invalidate
pending reads; re-addition cannot validate an old reply. This does not specify a
production HTTP transport, range protocol, read-session recovery or provider SLA.
The authority fixture restores synchronously into an inspection-only fence.
PocketIC proves ordinary stop/start continuity, same-release upgrades, atomic
rollback of invalid incoming journals, and retention of uncertainty, root history,
billing and held callbacks. An operator-armed read callback trap rolls back attempted slot cleanup;
that slot stays blocked through stop/start and elapsed time. No reset/unfence
endpoint is provided. The local source has a fixture-only ic-memory journal for
bindings, retained bytes/read state and bounded lifetime call history. Intents
precede dispatch. Synchronous restoration always fences operational endpoints;
only the original driver can inspect the retained journal. Unresolved work rejects
ordinary upgrades, while skipping the outgoing hook still restores behind the
fence. Missing journals reject restoration. These hooks do not qualify snapshot
loads/reinstall or supply independent authority surviving an old backup.
The authority fixture additionally writes a bounded inspection archive in the
same IC message as every mutation. It covers sample confirmed objects, sample
uploads and the connected journey, including immutable identities, release
receipts, accounting, manifests/progress and exact pending read intent. Only its
explicit operator may inspect it. The archive now retains private exact-release
streaming-hash checkpoints. A test-only probe can reconstruct and advance a copy;
its result never updates live progress, catalog state or certificate eligibility.
Hash-state bytes can contain a short plaintext buffer and are omitted from public
inspection views. Their checksum detects damage, not forgery or rollback.
Incoming archives bind the workspace release/dependency selection through the
Cargo.lock digest, including when all content is terminal. Disposable catalogs
validate all records through shared transitions and exact comparison before the
complete frozen archive becomes the inspection owner. Pending read/sync identities
remain evidence; restoration never mints callable tokens from them. Busy active
instances reject ordinary upgrades; forced restoration retains uncertainty under
the fence. All operational endpoints, including gateway liveness/deletion queries,
reject after restoration. No unfence/reset path is provided. Older archives cannot
authorize renewed admission; full snapshot loads remain outside this evidence.
An operator-only status query projects the current active or frozen owner into
separate catalog phase/receipt counts and logical, physical and billing bytes.
Shared pure diagnosis composes existing billing blockers with recovery/provider
qualification and funding-journal uncertainty; outstanding read/sync/upload/delete
work remains visible separately. Validated diagnostic billing limits may now be
installed for an exact balance scope/revision. Shared threshold assessment reports
shortfalls without reserve arithmetic when spendable funds are unknown, preserving
an explicit spendability blocker. Funding activity remains unobserved. Its
separately configured local balance-read scope now binds service, namespace, source
and account to each persisted intent. The shared decoder validates independent
reply bytes; status retains history and only exposes a current total within the
fixture's dispatch-based age bound and current configuration revision. Restoration
always invalidates current use. No query refreshes, credits or funds an account.
Gross canister cycles never stand in for spendable funds.
Status is neither an archive-integrity audit nor operational admission authority.
Queries cannot sync, fund, replay work or clear fences, and older/missing stable
evidence cannot replace live status before an actual restore.
An unpublished local operator client now consumes authority/funding status through
ic-testkit's PocketIC transport. Target server/instance/canister/caller and expected
namespace/peer are explicit; the caller is simulated, not production authentication.
Only the fixed query is callable. Structured blocker checks never grant effects,
and failed reads cannot trigger an update fallback. Real subprocess tests cover
permission/binding failures, method mode and unchanged journals after restoration.
The separate local refresh tool queries an admission preview or explicitly requests
a controlled balance read. The fixture atomically checks caller, fence, full scope,
revision and next attempt before intent/dispatch. Preview cannot reserve admission;
repeating a consumed request cannot dispatch again. Post-status has a separate JSON
outcome and never triggers action replay. Malformed or missing acknowledgements
remain uncertain. These fixture commands establish no production account authority.
The local sync command shares that client flow. Required fixture edit revisions
invalidate previews on every revocation, while the existing registry owns callback
correlation and membership. Exhausted revisions block sync but never revocation.
Source/revision/sequence status is inspection data, not token or restore authority.
Its controlled local source canister additionally exercises gateway sync across
real awaits, including reentrant revocation and replacement. No provider binding
or deployed Cashier transport is implemented by these test-only calls.

The transient gateway model now correlates one pending sync to its exact local
attempt and immutable service/namespace/Cashier scope. Operator edits invalidate
older syncs; malformed responses preserve current state. Pure callback policy
checks current membership against the trusted object and execution context.
These local rules do not establish endpoint authentication, provider revocation,
durable freshness or a restore-safe sequence allocator. Operators can explicitly
start a later sync that re-adds a member; no permanent denylist is implied.
`StableGatewayRegistry` now persists these same model transitions in one explicit
host-granted memory. One bounded v1 record retains the service/operator/Cashier/
namespace binding, processing and membership limits, ordered members, last sequence
and pending identity together. Installation accepts at most 1024 distinct members;
the complete record is bounded at 64 KiB. Each mutation validates before writing
the whole bounded membership record, without accumulating lifetime sync rows.
Malformed lists preserve pending state; operator adds/removals invalidate earlier
attempts even if membership does not change. Explicit cancellation applies only to
the exact read-only sync. Sequence exhaustion never prevents an operator removal.
Restoration validates the retained record and installed bounds without repair,
preserves pending identity and permanently fences mutation. Host release/installation
checks remain separate. Native and local PocketIC evidence covers persistence and
rollback; this store does not authenticate provider replies, assign read-session
generations or qualify membership as callback authority. A host must bind live
transport to the original opaque attempt; handles in the probe are bounded,
labelled test controls and do not survive restore as operational authority.
`workflow::gateways` constructs the canonical Cashier query before reserving its
durable sync, then retains request and token together for completion or cancellation.
Authority and restore fencing precede request/source/token checks and bounded Candid
decoding. Only a fully valid reply commits membership and clears pending state;
failures retain both unchanged. The storage probe supplies encoded local replies to
this shared workflow, including write-trap rollback and restore evidence. A host
must retain authenticated operator and source context across transport and bound
buffering separately. Supplied bytes/scope do not authenticate a provider fetch;
the advertised query has no automatic replicated-call fallback here.
`workflow::gateways::transport::query_sync` accepts an already persisted attempt
and invokes the host's `CashierQueryTransport` only after checking current authority,
fence and exact pending identity on polling. `GatewayRegistryAccess` releases each
synchronous borrow before transport. The original context/request remain captured;
source matching and current-owner completion checks precede any membership write.
Transport failures retain pending state for explicit cancellation or operator
invalidation, without an automatic retry. Each invocation performs one read-only
query; this is not a paid-effect dispatch permit or a persistent transport-attempt
counter. The host must authenticate responses and independently bound buffering.
PocketIC exercises this handler with a differently named local update substitute
and canonical empty arguments. Delayed replies cannot overwrite revocation, a newer
pending attempt or completed replacement; callback write traps preserve pending
state after the source has answered. Restored dispatch sends nothing. These tests
establish scheduling/rollback behavior, not replicated support for Cashier's query.
`ReplicatedGatewayQuery` now explicitly selects replicated IC execution for the
canonical query method. It checks its configured service against the actual running
canister and its Cashier against the original request, refuses ordinary query
execution and other query kinds, and sends once with a positive timeout of at most
300 seconds. It attaches no cycles and installs no automatic fallback or retry;
ordinary platform fees still apply. The CDK initially buffers under platform limits;
the host's smaller byte budget is checked before the owned buffer reaches decoding,
without copying it again. PocketIC exercises the canonical query-only export and
tests zero attachment, scope/execution refusal, rejection, size limits, callback
rollback and fenced restoration. The [current review](evidence/caffeine-gateway-transport.json)
retains the unchanged public provider baseline. Local replicated execution is now
established; deployment ownership, actual Cashier replicated execution and provider
semantics remain qualification work. Off-chain query clients acquire no implicit
permission to switch transport modes.
Local account-balance reply decoding likewise binds successful reports to a
supplied requested account and rejects unusable amounts. It does not establish
transport identity, account ownership, observation freshness or payment outcomes.
The post-0.1.18 [installation proposal](provider-review.md#integration-decision-after-0118)
separates service owner, tenant project and payer and inventories the remaining
deployment inputs. Bounded local relationship decoding checks both expected
principals and retains signed provider figures without interpreting allowance.
No compatible relationship reported is not self-payment evidence. This extends
local inspection only; it freezes no production binding, schema or transport.
The library also owns the explicit method/argument encoding for balance,
relationship and gateway-list queries. The controlled balance fixture sends the
maintained account record to its local substitute endpoint and uses the shared
reply decoder; IC interruption and restore checks remain local-model evidence.
No real Cashier call, provider namespace or production workflow follows from
constructing an encoded request.
Balance/relationship decoding can retain the original request's method, target
and account expectations; supplied response-source context is still not transport
authentication. The balance fixture carries that request across its await. A
driver-only local relationship query shares the bounded raw-response source and
rejects held, busy, rejected or fenced observations. It cannot activate a payer
or remove an obligation, and adds no production account workflow.
Gateway reply application likewise checks the original method/target before the
registry's scope, pending token and bounded decoding. The local reentrant sync
fixture retains that request across its await but still calls its scheduling
endpoint; a separate driver-only query exercises the provider's empty-argument
shape. Scripted modes and restored sources cannot answer that passive query.
Audit request encoding requires an explicit account and positive page limit;
present cursor accounts must match. Reply inspection checks the original method
and trusted source and enforces both requested and local reported-count bounds.
Account/filter correctness is not independently verifiable from the opaque CSV
envelope. There is no automatic pagination, production audit call or payment
reconciliation; cursor order, retention and row semantics remain unqualified.
The controlled-source workflow now supplies explicit IC target/caller and exact
attempt correlation, with one pending read and sixteen lifetime attempts. This
tests local orchestration, not deployed Cashier authority or certified freshness.

Pure funding policy now also assesses admission of a new intent from supplied
recovery/activity observations. This is part of the local policy exception;
it neither establishes those observations nor persists or executes an intent.
An additive incomplete-evidence assessment retains unknown spendability, recovery
and activity alongside known blockers. Reserve arithmetic never replaces missing
funds with zero. The local funding-preview query binds sender/peer/id/amount and
checks identity reuse/capacity without consuming an intent. Its raw transfer
experiment remains separate from operator admission; no funding action is exposed
through the preview CLI. Gross cycles and transport acceptance supply neither
authoritative spendability nor provider credit.
An exact local funding lookup now exposes absent, pending or retained transport
evidence after checking caller, service/peer and all original request inputs.
Its CLI only queries: missing evidence or a failed read cannot trigger payment.
It reads the active owner and preserves restore fences; it does not reconcile a
lost Cashier reply or establish that an absent operation had no external effect.
The local fixture now requires an installed attachment allocation and positive
reserve. Its bounded journal derives accounting from original intents and exact
terminal observations; admission persists the full reservation before dispatch.
The library's `FundingAllocation` model now owns this amount reconstruction for
bounded sequential journals. Each original full offer must fit before applying its
own return; unknown transport may appear only last. The fixture retains binding,
identity, revision and persistence checks. This arithmetic model supplies neither
omitted history nor a reconstruction rule for overlapping transfers.
Known refunds and unsent attachments release only their allocation. Accepted and
unresolved attachments remain charged, including through a permanent restore
fence. Preview requests bind the budget revision, which changes even after a full
refund. This is an attachment envelope, not production spendability: execution
fees and other liabilities are outside it, and incoming cycles cannot replenish it.
An additional local liquidity guard now samples the platform balance and exact
call-cost bound after intent persistence. It preserves separately installed
positive operating slack and explicit other liabilities. A refused dispatch is
recorded as unsent with zero acceptance and no callback refund; its identity stays
consumed. Callback failure controls apply only to real callbacks, preserving
unsent refusals through restore. Preview amount limits match update admission.
Query observations can be cached or become stale without any journal
revision change, so the update rechecks them. These local resource inputs do not
establish complete production accounting, independent recovery or provider credit.
Shared transfer values now validate exact unbounded-call refund arithmetic and
distinguish proven enqueue failure from missing evidence. Pure reconciliation
policy diagnoses no transfer, accepted cycles requiring credit evidence, or an
unknown transfer with the full attachment unresolved. Its result neither clears
account-wide activity nor permits a retry. Service/provider/account/operation
bindings and authoritative credit reconciliation remain workflow obligations.
Shared activity diagnosis now examines all uncredited attempts, so a later refund
or unsent call cannot hide older credit requirements. The funding fixture exposes
that diagnosis in a driver-only query bound to its actual canister and retained
local peer. Incoming acceptance receipts remain separate from outgoing attempts;
neither establishes provider credit. Active experiments report recovery as unknown;
restored instances report an enforced permanent fence. Restoration checks the actual
service and Cargo.lock release binding, bounded unique operation/receipt identities,
exact refund arithmetic and retained shared reconciliation before installing the
inspection owner. The sender rejects all new/reused payment intents; the receiver
rejects before cycle acceptance. Late callbacks cannot complete restored intents.
Unknown transfers and uncredited acceptance remain inspectable through repeated
upgrades. Missing/corrupt/foreign journals and failed hooks reject atomically.
The changed fixture schema is reinstall-only, with no older reader or reset/unfence.
Whole-canister snapshots can bypass hooks and remain unqualified. Missing billing
configuration, balance and spendable reservations remain unknown. Queries preserve
journals and make no calls. Before restore, experimental admission still allows
distinct completed transport cases; this is not a production funding workflow.
Local Cashier audit decoding now covers its verified Candid response envelope,
with bounded opaque CSV and reported pagination fields. No row schema, request
transport, automatic pagination, complete-history proof or credit matching is
implemented; provider errors and unusable pages cannot settle funding.

The maintainer subsequently requested the persistence contract and lifecycle
model, and explicitly directed a fresh design review of Canic's decisions.
That local scope includes the transient confirmed-object lifecycle model below.
It does not close provider qualification or authorize persisted workflows/effects.

The 0.1.11 continuation also authorizes local upload admission/reservation work.
`UploadCatalog` owns its confirmed catalog and operation history together, using
the same root claims and aggregate byte bounds. Exact tenant-scoped operations
reserve lifetime history, concurrent slots and bytes before possible exposure.
Cancellation is allowed only before exposure; unknown outcomes retain capacity.
Confirmation consumes an independently authenticated exact fact and transfers
the reservation without allocating a second object. Cancelled/settled history
and root claims are retained. This transient model has no certificates, provider
transport, timeout, serialization, restore or automatic uncertain-effect retry.
Byte liabilities do not bound monetary costs. Pending-upload gateway liveness,
provider reconciliation and durable admission remain integration work.
Local read policy now includes tenant-scoped active-upload pages and aggregate
reserved/confirmed usage. Scope-bound cursors convey no authority or snapshot;
scan/result budgets come from configuration. Gateway observations cover pending,
cancelled and confirmed root history with current membership and namespace checks.
These typed observations deliberately define no provider liveness/deletion boolean.
Gateway batches use temporary input-bounded maps and at most one shared history
scan; duplicates do not multiply scans, and no read cache persists between calls.
Native tests and fixed PocketIC caller/cancellation/revocation fixtures cover this
read boundary; pending-root protection in the actual protocol remains unqualified.

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

The local content-identity exception now includes bounded streaming computation
of the reviewed Caffeine client's nonempty content tree and raw digest. Explicit
metadata is hashed with the client's normalization and ordering; no MIME or header
inference occurs. This is local hashing, not an upload tree/certificate, chunk proof,
HTTP-header validator, completion observation or persisted checkpoint. Independent
client vectors provide algorithm evidence only. Empty provider objects remain
unqualified rather than inheriting the client's failing empty-tree branch.
The same local algorithm now validates bounded ordered chunk manifests against
an expected root and verifies exact bytes at any chunk index. These are immutable
local identities, not provider wire manifests or certificates. Construction
checks consistency only; verified bytes, trusted length, tenant binding, provider
availability and durable resume progress remain separate requirements. In
particular, a root without trusted length metadata cannot alone authenticate the
declared final-chunk length.
The local byte-verification scope also includes bounded in-memory coverage of
unique chunk positions in one manifest. This tracks successful observations only;
it neither owns destination bytes nor serializes resume state. All-chunks-verified
is separate from successful writes, durable completion and provider availability.
An ordered variant composes exact manifest leaf checks with raw-content hashing:
bad chunks reject before advancing the hash, and finalization checks the full
declared length and separately supplied raw digest. Its successful identity pair
still makes no destination-write, persistence or provider-completion claim.
Missing-chunk enumeration is also local: positive scan/result limits bound each
page, and byte ranges follow the immutable manifest's declared length. Scans use
current coverage, confer no reservation/authority and are not durable checkpoints
or evidence that the provider supports any HTTP range protocol.
The unpublished PocketIC fixture exercises these local byte algorithms in Wasm
using compiled independent vectors and measures ordered-append instructions.
It exposes fixed test cases only; no production read API, input-size qualification,
provider transport or durable state is established by this experiment.

The [independent deployment review](provider-review.md#independent-deployment-support)
now identifies DFINITY's explicit Rust onboarding guide and payment-account
linking example. Caffeine is the sole provider target. Exact installation
bindings, deployed interoperability and recovery/economic behavior still
need qualification; lack of general Rust integration guidance is no longer a
blocker. The example does not replace tenant policy, host memory ownership or
restore fencing. Target the current package's `vec blob` deletion list, without
the older example's text fallback. Executed callback-refund and same-release
upgrade experiments are platform evidence with fixture-owned ic-memory journals,
not authorization for production provider effects or frozen service schemas.

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
exists as a product package; the test canister and host harness are unpublished
fixtures. Final product names and package split remain open. Both adapters belong here,
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

`reference_receipt` exposes the original exact operation result through the same
service/tenant/namespace/object and payload checks used by mutation replay. It
never applies a request or consumes a receipt, including when the result is
absent. Suspension, history exhaustion and settlement preserve inspection. A
recorded lifecycle failure remains a failure, and historical retain success is
not current liveness. Unknown roots and conflicting identities are errors, never
absent receipts. The local saved-intent tool demonstrates this query path without
dispatch; its mutable filesystem artifact supplies neither fresh allocation nor
surviving operational recovery authority.

For fresh uploads, `admission_capacity` observes the tighter tenant/global
headroom in each independent dimension: lifetime operation/root slots, concurrent
reservations, lifetime manifest leaves and bytes. Byte headroom is the minimum of
tenant logical, global physical and global continuing-liability headroom, including
unconfirmed reservations. Enrollment and per-object size/metadata limits accompany
the observation. Only the exact enrolled tenant can inspect this scope; suspended
tenants retain visibility without regaining admission authority. Shared contention
affects the returned headroom, but other tenants' identities and records stay private.

The read uses maintained totals without scanning history or changing state. All
dimensions must fit together; the counts are not a promised number of uploads or
a reservation. Cancellation can restore byte/concurrent capacity without restoring
lifetime slots. Logical release and physical deletion cannot erase continuing
billing. Combine this view with tenant content discovery and exact reference
capacity for reuse; absence still does not prove a root is globally unclaimed.
This adds a transient model observation and bounded private fixture query, not
production persistence, authenticated publisher transport or a provider guarantee.

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

Most boundaries below remain proposed v1 responsibilities rather than installed
records or a frozen wire format. Keep them independent of Canic's memory IDs and store layout.
The maintainer selected `ic-memory` as the allocation owner aligned with Canic
and IcyDB. Future core stores use its re-exported stable collections, and the
integrating host owns bootstrap, policy, grants and bucket profile. This dependency decision
does not freeze blob schemas/keys/IDs or close the provider and recovery gates.

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

`admission_capacity` and `reference_capacity` use maintained counters and indexed
identity reads, without loading reference/receipt histories or manifests. Heap and
stable owners share headroom arithmetic. Admission observes the tighter global/tenant
operation, active-upload, leaf and byte dimensions. Cancellation/settlement never
refund lifetime history; physical deletion alone cannot refund continuing billing.
Reference headroom reserves one release receipt per active reference and two slots
per additional retain/release pair. Unknown, foreign and unconfirmed roots expose no
reference capacity. Retired confirmed objects have zero fresh retains. Suspension
and restoration allow inspection, not fresh admission or removal of the fence.

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

`workflow::reads::download::describe` provides a separate direct-client descriptor
path without allocating a session or fetching any body. It requires active tenant
authority, an unfenced owner and exactly the requested confirmed live reference.
Its host-supplied `CaffeineDownloadScope` binds an explicit owner/project mapping
to the installed local namespace. Owner must equal the service, independent of
payment account and tenant. Project text is bounded at 256 UTF-8 bytes before
copying, with no empty value, controls or surrounding whitespace; this is a local
representation bound, not a provider naming guarantee or a default assignment.

The original root, length and hash headers accompany an encoded relative Caffeine
`/v1/blob/` request target. Request fields are individually percent-encoded; no
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

`dto::download::{DownloadRequest,DownloadResponse,DownloadFailure}` now owns the
maintained service boundary. `workflow::reads::download::handle` checks actual
service/caller plus every nonzero identity and delegates to the same operational
descriptor workflow. The unpublished probe exports the canonical update
`blob_download_descriptor`; its private descriptor endpoint/DTO is removed.
The response echoes the full request, owner/project, declared size and original
headers. It returns no URL, origin, credentials or raw-digest assertion. Adapters
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

The private storage probe exercises interrupted writes from admission through
completion, references and settlement, then same-release upgrade in every phase.
Changed-operator restoration fails without changing the old instance. Completion,
deletion and settlement APIs consume facts already authenticated by the trusted
host; the probe supplies explicit operator-only substitutes. No real certificate,
provider call or evidence qualification follows. Other durable provider-call
journals, deployed verified-read delivery, operational recovery and resource qualification
remain required.

`ops::service::funding::StableFundingJournal` owns a separate attachment allocation
through two exclusively owned, host-granted memories: exact intent history and one
configuration/accounting row. It binds service, operator, Cashier candidate, payment
account, namespace, allocation, positive reserve and lifetime history limit. Hosts
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
hold must agree. Earlier accepted amounts, external unknowns and the actual fence
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
reconciliation policy to that transfer alone. Reported success never clears required
credit evidence; missing transport retains the full offered amount as potentially
spent. The v1 record schema is replaced directly, without migration or dual
readers; cross-release transitions remain reinstall-only. Production provider
authentication, independent credit reconciliation, account-wide
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
