# Current status

Date: 2026-09-28

## Active work — 0.2.11 draft after 0.2.10

The maintainer confirmed **0.2.10 is pushed** and requested continued work.
Cargo and the release receipt are 0.2.10. Local main, origin/main and v0.2.10 resolve to
`db0b357d21c72ac43fb789e275eed20aeb271b1a`, from validated source
`71ff38b012f1302c261388a44c00aac7950f5684`. The receipt records the
`release-verify` gate; registry publication was not independently queried.
The dated 0.2.10 notes match that release. The maintainer requested the **0.2.11**
changelog. Completed work is recorded in that undated draft below empty Unreleased;
Cargo and the release receipt remain 0.2.10 until the maintainer's release flow.
No version mutation, commit, publication, deployment or deployed-provider effect ran here;
transfers below use simulated cycles between local PocketIC canisters.

Follow the [0.2 delivery plan](../roadmap.md). Its goal remains a usable
Caffeine-backed service through shared durable handlers, both adapters and an
operator client. Library publication does not qualify the service or Canic removal.

| Milestone | State | Remaining completion condition |
| --- | --- | --- |
| M1 — contract | In progress | Freeze admission/resource envelope, provider guarantees and operational recovery |
| M2 — durable standalone service | In progress: durable upload/reference/settlement bookkeeping and IC evidence | Complete reads/provider journals, shared handlers and operational recovery/journey evidence |
| M3 — Caffeine and operator integration | In progress: shared transports with local IC evidence | Qualified provider transport, completion/economics and production client |
| M4 — managed parity and acceptance | Not implemented | Same journey through Canic adapter, complete replacement evidence and handoff |

## Current focus — authenticated admission and first-reference recovery

`dto::upload`, `workflow::uploads::inspect` and `ReplicatedUploadStatusClient`
now supply exact tenant-only upload history through `blob_upload_status`. The
bounded response echoes the full original upload and distinguishes reservation,
possible exposure, confirmation and cancellation. Confirmation means the first
reference was created historically; it remains after release/deletion/settlement.
Inspection survives suspension and restore but supplies no current liveness,
publication, provider-completion or uncertain-retry authority. The client sends
one replicated call without attached cycles or automatic retry. Linking exports
no endpoint or lifecycle.

`workflow::uploads::admission` now supplies canonical `blob_admit_upload` and
`blob_upload_admission` inspection with full permission DTOs and a bounded
`ReplicatedUploadAdmissionClient`. Original uploader and expiry are checked alongside
every upload field. Fresh admission enforces active enrollment, expiry and capacity;
exact replay preserves permission and accounting without renewal. Inspection remains
available under expiry, suspension and restore. The storage probe's private admission
endpoint is replaced, including its write-trap path. No certificate or provider
effect is introduced.

The unpublished `blob-consumer-probe` supports both existing-content retain
and fresh-upload first-reference registration. It persists exact asset intent,
permission and cleanup identity before sending admission from its actual tenant
canister. Typed refusals remain recorded; uncertain acknowledgments block redispatch
and recover through exact permission inspection. Its optional `prepare` endpoint
can save intent without dispatch. Fresh registration
observes authenticated completion rather than manufacturing another retain/receipt;
both sources obtain the current descriptor before atomically checking the tombstone
and publishing. Cancellation retains possibly exposed uploads, allowing late
completion to be reconciled and the first reference released under suspension.
Stale status callbacks cannot overwrite newer completion. Reference and operation
identities stay reserved across assets, cancellation and cleanup.

The fixture's bounded v1 schema is replaced directly; cross-release transitions
remain reinstall-only. Same-release upgrade validates and fences restored consumer
state while preserving inspection and unresolved work. The two-slot/full-record
fixture is not production sizing. No dependency, allocator, service stable schema
or memory grant changed. Existing-content retain/release behavior remains covered.

Latest validation passes 30 targeted admission native cases, three consumer-model
cases and all 31 selected storage PocketIC cases (53.31 seconds). These include all
12 consumer cases plus admission, lifecycle, planning and read regressions; unrelated
funding/gateway suites were excluded. The IC cases prove fresh registration leaves
service stable memory unchanged, recover admission after an unusable reply or callback
trap without resending, retain typed refusals and cancelled uploads, reject changed
permissions and preserve uncertainty through fenced upgrades. The preceding status
step also passed two upload-status and 22 reference native cases. Release consumer/storage
Wasms, strict affected all-target Clippy, warning-free core/protocol/consumer
rustdoc, formatting and diff checks pass. Full CI/release validation was not run;
earlier batches' broader results remain historical evidence.

Next, add shared manifest preparation and revocation transport, preserving exact
declarations and cancellation intent before effects. Real application transaction
integration, browser delivery policy,
shared adapters, provider qualification, operational recovery and production
sizing remain open. The host still arranges preparation/exposure/revocation and supplies
labelled completion facts. This does not qualify Toko, Canic retirement or the service.

## Canonical reference mutation and receipt recovery — included in 0.2.10

`dto::reference` and `workflow::references::receipt` now expose the durable owner's
original result under complete upload/object/lifetime/reference/operation binding.
Upload, object and operation IDs remain independent, including full-width values.
Lookup refusal, explicit absence, recorded success and recorded lifecycle failure
are distinct. Explicit `Absent`/`Found` wire variants prevent incompatible Candid
optional data silently turning into absence. The bounded decoder checks the entire
returned request before disclosure. Unexpected internal results fail closed.

`ReplicatedReferenceClient` checks configured and actual tenant identity and
replicated execution, then invokes `blob_reference_receipt` once at the selected
service with bounded wait and no attached cycles. Ordinary platform fees apply;
the initial CDK buffer has separate platform bounds. No retry, mutation fallback,
provider call or new journal is introduced. Suspension, release, settlement and
restore preserve historical inspection, while mutation remains fenced on restore.
Historical success is not current reference liveness; absence is not retry authority.

`workflow::references::apply` now exposes the same durable transition through
`blob_apply_reference`. Mutation and inspection share one passive `ReferenceCommand`
and typed `ReferenceFailure`; the unshipped lookup-only type names were replaced
directly. Responses bind the original command and preserve recorded inner failures
and replay status. Active enrollment is required for fresh retains, but suspension
does not block release or exact replay. Restore blocks every mutation, including
replay, while receipt inspection remains available.

The client's explicit `apply` method sends once, checks actual execution identity,
and validates the bounded response. Callers must persist intent before polling
and retain it through uncertain rejection, unusable replies or cancellation.
No identity allocator, consumer outbox, automatic retry or provider effect is added.
PocketIC demonstrates a committed retain whose reply exceeds the client budget,
exact receipt recovery, byte-for-byte unchanged stable memory on replay, cleanup
at capacity/under suspension and retained evidence after fenced upgrade.

The durable storage probe replaces its private receipt lookup and mutation wire
with the shared query/update. Superseded mutation DTOs/conversion are removed;
the private write-trap endpoint delegates to the same handler. Its existing
trap/upgrade evidence now uses the canonical DTO. A real second
tenant canister exercises the replicated client, stored failures, changed arguments,
wrong contexts, reply limits, query refusal and platform rejection. The separate
transient admission fixture and its saved-intent tool retain their own test wire;
they are not the durable service client.

Twenty-two targeted native reference tests and all 63 storage PocketIC cases pass
(103.04 seconds for the latter), including descriptor, quota, gateway and funding
regressions after replacing the shared fixture mutation path. The preceding
receipt-only step passed twelve native and five lifecycle PocketIC cases.
Strict affected all-target Clippy, release storage-probe Wasm, warning-free core
rustdoc, formatting and diff checks pass.
No dependency, allocator, stable schema or memory grant changed. No full CI/release
gate ran for this batch.

The consumer fixture above now exercises existing-content registration/release
using these shared clients. Production consumer asset transactions, tombstones and
outboxes still belong to the application; service calls alone cannot atomically publish assets.
Browser delivery/origin/body policy, provider qualification, operational recovery,
resource sizing and both deployment adapters remain open.

## Authenticated IC descriptor delivery — included in 0.2.9

`workflow::reads::download::describe` now binds current active tenant authority,
an unfenced durable owner and the exact confirmed live reference to the retained
root, length and original hash metadata. Trusted host `CaffeineDownloadScope`
supplies an explicit storage owner/project/local-namespace mapping. Owner must
equal the service; neither payer nor tenant can replace it. Project text is
bounded before copying, with no default or inference from local namespace IDs.
The result contains the reviewed relative `/v1/blob/` target with escaped fields.
It performs no provider call, body transfer/hash, session admission or mutation.

The probe's canonical update `blob_download_descriptor` delivers that view and feeds
the existing off-canister root verifier with its original metadata. Caller,
object/lifetime and reference isolation hold. Suspension, release and restore
reject new operational descriptors while passive historical inspection remains
available. Stable memory stays unchanged across success and rejection. Saved
descriptors/URLs are not revoked by these checks; publication/reference
coordination and provider serving policy remain required.

`dto::download` now owns the passive request/response/error contract and
`workflow::reads::download::handle` owns its shared boundary conversion and
delegation. The private fixture descriptor schema/endpoint is removed. The
response echoes the full request, owner/project, length and original headers;
it carries no provider URL, credentials or raw-digest assertion. Linking exports
no endpoint or lifecycle. Adapters supply actual caller/service and installed
serving scope, with independently bounded ingress decoding.

`ReplicatedDownloadClient` selects tenant/service and a bounded timeout, checks
actual running identity and replicated execution, and sends the canonical update
once with zero attached cycles. Actual IC targeting authenticates the peer;
ordinary call/execution fees remain. Before disclosure, bounded Candid decoding
checks the entire original reference, owner/project, declared size and shared
metadata invariants. The CDK's initial buffer is platform-bounded separately.
No automatic retry or query/method fallback occurs. A real second tenant canister
now exercises this route, including wrong client/project, size limits, query
refusal, typed suspension/restore failures and platform rejection. This is not
a browser adapter or publication lease; copied observations can become stale
across the return await.

The [serving review](../evidence/caffeine-gateway-transport.json) rechecks official
main at `ee8e3dda39b105f95133256144172a4506e841a8`. Its latest commit changes Motoko
guidance; the reviewed storage client source is unchanged. Npm latest/integrity
remains 1.1.2, and Toko development/client remain at the prior pin. Both clients
use direct HTTP root/owner/project locators; this supplies no IC chunk-read wire
for the optional canister verifier. No gateway object request, deployed account
call or paid effect ran. Mops/backend/Cashier were not refreshed in this review.

Twelve targeted native download cases and all five descriptor/scan PocketIC cases
pass (5.81 seconds for the latter). Strict affected all-target Clippy, release
storage-probe Wasm, warning-free core rustdoc, formatting and diff checks pass.
The private test protocol now depends on the library to reuse its canonical DTOs.
No external dependency version, stable allocation/schema, package version or
allocator change was introduced in this descriptor step. The read-session work below
was included in the same 0.2.9 release; these are the step's targeted validation results.

Next, complete consumer publication/reference coordination and browser delivery,
including explicit origin and HTTP body/redirect policy. The host
must retain the provisioned owner/project mapping used for upload; constructing a
scope does not prove ownership. Keep body delivery and verification off-canister
for consumers. Production provider completion/economics, operational recovery,
resource sizing and both deployment adapters remain open.

## Durable verified chunk reads — included in 0.2.9

`ops::service::reads::StableReadSessions` now owns three explicit host-granted
memories for journal metadata, occupied sessions and active-tenant counters.
Trusted global/per-tenant slot and reply-buffer limits apply independently. Each
read reserves its complete configured reply budget, separately from upload quota.
Ordinary updates touch exact rows/counters, not lifetime history. The bounded v1
records retain chunk/reference/root/gateway, original tenant/service/scope and
both authority generations. Completed rows and empty tenant counters are removed;
the monotonic global identity remains. Exhaustion never wraps or blocks completion.

Shared `workflow::reads::sessions::{begin,complete}` checks all owners, current
authority and chunk range before admission, and commits intent before the host's
single separately qualified call. Only the original exact callback can release
its slot. Revocation, tenant reactivation, released references, elapsed time and
dropped tickets do not free capacity. After the actual local call returns, lost
authority rejects disclosure while releasing that exact reservation. Restore
fences retain occupancy and prevent both admission and completion; bounded
operator inspection remains available. No paid-effect or byte-verification
authority is implied by completion.

`workflow::reads::chunk::read_chunk` now composes those handlers with one normalized
host transport. It admits on first poll, releases all owner borrows across the
await, rechecks callback authority, binds authenticated source/root/index, bounds
decoded bytes and verifies exact length and the immutable manifest leaf hash.
The selected leaf comes from one bounded stored manifest; no tree reconstruction
or reference/receipt history copy occurs. Hashing shares the maintained Caffeine
leaf primitive. A returned error releases only its settled reservation; stale
authority rejects before hashing. Dropped pending futures retain occupancy.

The probe replaces its ephemeral busy flag and authority-only endpoint/DTO with
the durable verified workflow. Three grants extend its existing ic-memory runtime;
the allocator and existing upload/funding/gateway record schemas are unchanged.
Its `fixture_read_chunk` endpoint now fetches actual bytes from the existing local
source's `fixture_chunk`. The old list-discard scheduling path is removed. The
probe checks encoded size before bounded bulk byte decoding, reusing the existing
locked `serde_bytes` dependency. This is a labelled substitute, not Caffeine's wire.
The actual IC call target authenticates the peer; normalized peer/root/index fields
are trusted host assertions, not untrusted payload evidence. Application limits do
not bound the platform/CDK's initial buffer or total encoded-plus-decoded heap.
Admission traps at all three writes send nothing and roll back counters/identity;
a callback trap after row removal preserves the entire occupied reservation.
Upgrade retains that interrupted reservation under the fence. Failed transport
settles occupancy, and stale callbacks cannot release later sessions.

That step's validation passed both shared read-workflow native cases, the stored leaf
range/partial-final-chunk case and all 15 manifest/hash regression cases. All 21
targeted gateway/read PocketIC cases pass in 29.07 seconds. The local cases now
exercise real chunks through corrupt, truncated, malformed, wrong-type, oversized
and rejected replies, held authority changes, callback traps and fenced upgrade.
Affected strict all-target Clippy, release storage/source Wasms, warning-free core
rustdoc, formatting and diff checks pass.

The preceding journal step passed five targeted native session/model cases, together with all 30 durable-upload
unit cases (two overlap). All 54 storage PocketIC cases pass in 82.64 seconds,
including existing funding, upload, gateway and descriptor/lifecycle regressions
after the fixture grant change. Affected strict all-target Clippy, release
storage/source/funding Wasms and warning-free core rustdoc pass. No full CI,
resource benchmark, release/version action or deployed-provider effect ran.

The serving review and direct-client descriptor above now extend this work. The
optional canister verifier must not become mandatory bulk readback during uploads.
Production transport qualification, resource sizing, operational recovery and
adapters remain open. A copied local counter must never authorize restoration.

## Durable read authority across awaits — included in 0.2.8

`workflow::reads::{capture,recheck}` now binds the original caller/service, exact
live reference, full object lifetime, root and selected current gateway. Both
durable owners must match configuration/scope and remain unfenced. Active tenant
enrollment and its activation generation are checked independently. Indexed
lookups avoid manifest and reference/receipt history copies; passive descriptor
inspection retains its existing suspension/restoration behavior.

Gateway membership now owns a separate read invalidation counter in the same
bounded v1 record. Successful edits (including no-ops) and complete syncs (including
identical lists) advance it; failures and sync begin/cancel do not. Remove/re-add
and tenant suspension/reactivation therefore cannot revive old read authority.
Exhaustion disables reads permanently while preserving administrative revocation.
The internal schema is replaced directly; same-release restoration retains the
counter and fence. Cross-release transitions remain reinstall-only. No additional
memory grant, compatibility reader, dependency or allocator change was added.

The opaque observation is host-retained, read-only and repeatable, not an ingress
DTO, session reservation, one-shot callback completion or dispatch permit. The
authority-only fixture holds a real local IC reply using the existing labelled
source, discarding its list; it fetches no blob bytes. It proves invalidation after
gateway remove/re-add, identical-list sync, tenant reactivation and exact reference
release despite another live reference. A trapped membership write preserves
authority; wrong callers and restored instances send nothing.

Targeted validation passes 28 durable-upload unit cases, 31 gateway unit cases
(three overlap), and 11 gateway-binding/upload-read integration cases. All 18
gateway PocketIC cases pass in 27.38 seconds; existing descriptor/scan regressions
also pass in 2.04 seconds. Affected strict all-target Clippy, release storage/source
Wasms, warning-free core rustdoc, formatting and diff checks pass.
No full CI, version mutation, release or deployed-provider operation ran.

The session journal above now composes bounded admission, exact completion and
manifest/chunk verification through a host transport. Deployed transport/provider
qualification and operational recovery remain open without weakening restore fences.

## Scoped durable gateway observations — included in 0.2.8

`workflow::gateways::callbacks::observe_roots` now composes the durable registry
and upload owner synchronously. It requires matching complete configuration and
explicit service/Cashier/namespace scope, actual service and current membership,
then checks both restore fences and each known root's stored object binding.
Membership applies even to empty, unknown or malformed batches. There is no
operator impersonation, public unguarded root reader, cached permit or await.
Indexed reads retain order, duplicates and malformed positions without loading
manifests or operation histories. Gateway results expose only local phases;
tenant, request and reference identities remain private.

The labelled probe endpoint proves actual caller isolation, immediate removal
and sync replacement, uncertainty after exposed revocation, cancelled roots and
all confirmed cleanup phases. Suspension retains operational visibility; upgrade
fences gateway observations while existing operator inspection remains available.
Native tests independently fence either owner, reject mismatched configuration
and reject a broken root/request index without returning partial batch data.
These observations grant no provider liveness/deletion mapping, effect completion,
retry, read-session generation or recovery authority.

Latest targeted checks pass: 25 durable-upload unit cases, 30 gateway unit cases
(three overlap), two gateway binding cases, all 14 storage gateway PocketIC cases
in 18.68 seconds, and four existing planning/read PocketIC cases in 3.79 seconds.
Affected strict all-target Clippy, the release storage-probe Wasm, warning-free
core rustdoc, formatting and diff checks pass.
No full CI, provider call, version mutation or release action ran in this batch.
Stable schemas, dependencies and allocator are unchanged.

Read authority above now composes generations, object/reference bindings and
restore fences. Exact callback correlation and session capacity remain open.
Provider effect semantics, deployed Cashier replicated execution, production
funding evidence and operational recovery remain unqualified.

## Explicit replicated gateway transport — included in 0.2.8

`ops::caffeine::query::transport::replicated::ReplicatedGatewayQuery` now implements
the shared host transport for the canonical gateway-list query. Configuration binds
service and Cashier with a positive timeout no greater than 300 seconds. Execution
checks the actual running service, original request target, maintained method and
replicated mode before sending. One bounded-wait call carries canonical arguments
and no attached cycles, with no automatic retry, alternate method or mode fallback.
Ordinary platform call/execution fees still apply. The application byte budget is
checked before passing the CDK-owned response to decoding without a second copy;
the CDK's initial buffer remains limited by the platform, not this smaller budget.

The new fixture endpoint uses this production primitive against the local source's
actual query-only `storage_gateway_list_v1` export. For the configured service,
that source requires replicated execution and zero attachment. Driver inspection
remains available and scripted scheduling modes still cannot execute in the query.
Existing delayed-reply tests retain their separately labelled update substitute;
it provides scheduling evidence rather than a second provider implementation.

The [transport review](../evidence/caffeine-gateway-transport.json) refreshes official
main, npm latest/integrity, Mops highest, backend file hashes and Cashier Candid,
all unchanged. An anonymous public gateway-list query returns the same retained
principal. Official platform guidance supports replicated queries and the pinned
Motoko wrapper uses that route. No live replicated update, private account lookup,
deployment, transfer or paid provider operation ran; the query text is not a
portable certified proof or service qualification.

All 27 targeted gateway native cases pass. All 11 storage gateway PocketIC cases
pass in 13.38 seconds, including query-only replicated success, wrong identity and
ordinary-query refusal, response-size rejection, remote rejection, callback rollback
and fenced restoration. Both existing gateway-source regressions pass in 2.27
seconds. Affected strict all-target Clippy, release storage/source/authority Wasms,
warning-free core rustdoc, formatting and diff checks pass. No full CI or resource
benchmark refresh ran. Stable schemas, dependencies, Cargo versions and release
receipt are unchanged; the current draft is additive to the released library.

Scoped gateway observations above now compose the durable owners; read-session
generations and effect callbacks remain separate. Deployed Cashier replicated
execution, provider trust, production funding evidence and operational recovery
still need qualification. Do not re-treat the
query annotation as a platform incompatibility, or label this local transport proof
as acceptance of the deployed service.

## Durable gateway query orchestration — included in 0.2.7

`ops::service::gateways::StableGatewayRegistry` now persists the existing gateway
model's membership and pending-sync transitions in one explicit host memory.
The v1 record binds service, operator, namespace, Cashier and list limits, retains
member order, and preserves last/pending sequence together. Its envelope is at
most 1024 distinct principals and 64 KiB per record. Mutation rewrites one bounded
record; lifetime syncs do not accumulate history rows. Restore validates and fences
without repair, pending cancellation or counter reset.

The shared model still owns list validation, duplicate normalization, exact sync
correlation and invalidation by operator edits. Invalid replies leave membership
and pending state unchanged. Even no-op adds/removals invalidate prior syncs;
explicit removal can empty membership and remains possible at sequence exhaustion.
The durable owner checks explicit scope and operator before reads or mutations.
Membership is not callback authority or a read-session generation. Real source
authentication, provider qualification and operational recovery remain open.

`workflow::gateways` now authenticates the operator/scope, constructs the canonical
Cashier query, and durably begins its exact pending attempt. The host retains the
opaque request/token pair across transport. Completion checks authority and the
actual restore fence, then reuses the original request's method/source/token checks
and bounded Candid decoder against a private registry copy. Only a valid complete
list writes membership and consumes pending state. Malformed/over-budget replies
leave the attempt pending for explicit cancellation or a valid reply. Operator
edits invalidate earlier attempts; stale replies cannot affect a newer sync.
The unreleased durable raw-list apply path was removed, with its consumers and
redundant test coverage replaced. Existing released transient APIs are unchanged.

`workflow::gateways::transport::query_sync` now checks an existing durable attempt
on polling, calls a host-supplied `CashierQueryTransport` without a registry borrow,
then checks source and applies the reply through current-owner validation. Original
operator/service/request context stays captured across the await. Failed transport
or decoding preserves pending state; no automatic retry/cancellation is installed.
Each invocation sends one read-only query, not a paid-effect permit. Authentication
and pre-buffering limits are host integration obligations, not facts proved by the
returned response struct. The fixture checks reply length before copying IC bytes.

The storage probe grants one additional memory through its existing `ic-memory`
runtime and restores this owner synchronously alongside uploads/funding. Its encoded
reply bytes and bounded ephemeral request handles are labelled local test controls
that exercise the shared workflow, not authenticated provider observations.
Its separately labelled transport endpoint calls `fixture_gateway_query` on the
existing local source, using canonical empty arguments. This deliberately different
update endpoint supports scheduling tests; the provider-shaped query endpoint and
its query-only contract remain unchanged. Source reply/hold behavior is shared
with the earlier scheduling fixture; no source record or memory layout changed.
Cross-release transitions remain reinstall-only; no old fixture/schema reader,
allocator change or implicit production memory registration was added.

All 26 targeted gateway native tests pass, including bounded durable reply refusal,
correlation before decoding, a full-width 1024-member encoded reply/record round-trip,
and rejection of an original host-retained request after restore. New native
evidence checks source mismatch and invalidation after future construction but
before polling, with no transport invocation. All eight storage gateway PocketIC
cases pass in 10.67 seconds. Delayed real local replies cannot overwrite a newer
pending sync or completed replacement. Queries/edits remain usable during awaits;
callback traps preserve pending state after the source answers. Wrong callers,
cancelled/completed attempts and restored owners send nothing. Malformed, oversized,
empty and rejected replies retain the attempt. Both existing gateway-source
regression cases pass in 2.09 seconds after sharing its reply/hold helper.
The other 34 storage cases remain passing evidence from the preceding broader run;
they were not rerun for this gateway-only change.
Affected strict all-target Clippy, release storage/source/authority Wasms, warning-free core
rustdoc, formatting and diff checks pass. Existing released store schemas, Cargo
versions and dependencies are unchanged. No full CI, resource benchmark refresh,
live provider call, publication or release action ran.

Next, qualify production query transport/authentication against the current provider
contract, then compose gateway callback/read-session authority with the durable
owners. The retained Cashier interface advertises a query; never silently substitute
an unqualified update call. Membership alone cannot authorize provider callbacks or
resume a restored service. Production funding evidence remains unresolved; do not
fill it from local membership or copy synthetic fixture facts into adapters.

## Shared guarded funding dispatch — included in 0.2.6

`workflow::funding::inspect_preparation` combines authenticated current journal
facts with separately scoped trusted host observations. Local obligations, the
actual restore fence, stale/retained identity, full lifetime history and allocation
reserve remain independent blockers alongside provider qualification, host recovery,
spendability and complete external account activity. Neither local nor external
clearance can stand in for the other. `prepare_new` re-reads current state and
reserves the exact offer synchronously; it accepts no previous preview, performs
no provider call and leaves blocked requests unchanged.

Host observations are plain integration values, not authenticated proof objects.
The integrating host still must establish their scope, freshness and completeness
in the same execution. They must never come from ingress or a cached preview.
The unpublished IC probe supplies unknown host observations, exposing only the
exact proposed intent at its boundary; operator authority cannot fill those gaps.
Its separately labelled raw transport/bookkeeping controls remain test fixtures.

`workflow::funding::attempt` now separately inspects and marks an exact existing
reservation. Host evidence binds every intent field and explicitly excludes only
that unattempted request from other activity and liability holds. The handler
requires Prepared state, the final retained identity and its exact complete hold.
It does not charge the offer twice or demand another history slot, so the final
slot can still be attempted. Older accepted amounts, unknown host facts and the
actual restore fence independently block marking. Uncertain and terminal attempts
cannot retry, including fully refunded or proven-unsent outcomes. Blocked updates
leave the original reservation and all storage unchanged.

Marking returns the canonical request after persisting uncertainty; it sends no
call. The host must establish current qualified observations, then compose marking,
call construction, post-write platform liquidity/holds checks and dispatch in the
same message. A dropped result or earlier preview confers no retry authority.
The IC probe supplies unknown observations for its preparation and marking
endpoints; those endpoints accept no evidence flags.

`workflow::funding::dispatch::dispatch` now composes guarded marking with the
shared canonical transport and durable callback settlement. Hosts implement
synchronous `FundingJournalAccess`, releasing the borrow before any await. After
authenticated exact request lookup and actual service binding, the handler invokes
the host observation closure on polling. Missing complete liquidity holds block
independently without mutation. After the marker's writes, actual platform liquidity
and call cost determine whether to execute once or consume the unpolled call as
positively unsent. The original intent and execution remain captured for settlement;
refund capture and durable outcome recording precede any further await.

The separately labelled `fixture_guarded_funding_dispatch` endpoint exercises this
shared handler with fixed synthetic host-evidence scenarios and a local Cashier
substitute. It is not a production endpoint or provider qualification. Tests prove
that pre-dispatch write traps send nothing and preserve Prepared, while callback
write traps retain Uncertain/full accounting after remote acceptance survives.
Missing holds leave stable memory unchanged; malformed replies retain independent
acceptance, liquidity refusals settle as unsent, and later distinct intents require
clear local obligations. A delayed real reply also leaves the journal readable
with its full reservation charged, rejects a concurrent duplicate and settles the
original exact request. Repeated and restored dispatches remain blocked.

All 35 targeted native funding cases and nine admission-policy cases pass. All
33 storage PocketIC cases pass in 48.13 seconds, including unchanged-state refusal,
caller/scope isolation, retained/conflicting identities, capacity and restoration.
The additional delayed-callback concurrency case passes separately in 1.93 seconds.
Native coverage also proves allowed reservation and first marking with explicit
test evidence, exact exclusion binding, full-history use without double charging,
older acceptance blocking and changed-state refusal after an earlier preview.
Affected strict all-target Clippy, both release Wasms and warning-free core rustdoc
pass, as do formatting and diff checks. No stable schema, memory grant, allocator,
dependency or package version changed. No full CI, resource benchmark refresh, live provider call or release
action ran; the separate funding fixture suite remains prior evidence.

Next, establish qualified production host evidence acquisition and account-activity
coverage for the shared dispatcher. Synthetic fixture observations must not be
copied into a production adapter. The local call sequence is implemented; it cannot
establish complete external activity or verified provider credit by itself.
Independent credit evidence, provider qualification, other provider intents, read
sessions, both adapters and operational recovery remain open.

## Scoped funding summaries — included in 0.2.5

`StableFundingJournal::summary` now checks explicit service/Cashier/account/namespace
and operator authority, including empty journals, then reads maintained accounting
and metadata without decoding intent rows. It returns local attachment totals,
lifetime intent count/capacity, the last retained ID and the independent restore
fence. History and summary share `FundingJournalScope`; the earlier draft-only
history scope was replaced directly, with no alias or released API removal.

Workflow applies the pure `assess_uncredited_allocation` policy. Earlier accepted
amounts and prepared/uncertain offers cannot be hidden by newer full refunds,
proven unsent attempts or reported balances. The result describes this entire
local journal, not complete external provider-account activity. A clear result
does not release the fence, prove spendability or authorize payment.

All 29 targeted funding and four reconciliation unit cases pass, including
equivalence with complete-history diagnosis at amount boundaries, full-width IDs,
capacity, empty-scope isolation and metadata consistency. All 25 storage PocketIC
cases pass in 40.57 seconds. New cases cover rollback, unchanged memory on queries,
old accepted amounts through later returns, clear-but-fenced restoration and actual
IC acceptance followed by a fully refunded call against the local Cashier substitute.
Affected strict Clippy, both release Wasms, warning-free core rustdoc, formatting
and diff checks pass. No stable schema, memory grant, allocator, dependency or
package version changed in this slice. No full CI, resource benchmark refresh,
live provider call or release action ran.

Next, establish external account-activity completeness and independent credit
evidence before production payment admission. Do not feed local `Clear` into an
account-wide admission gate without that evidence. Provider qualification, other
provider intents, read sessions, adapters and operational recovery remain open.

## Durable Cashier outcomes — included in 0.2.5

`StableFundingJournal::record_observation` now commits the shared transport's
bounded structured response in the same IC transaction as its phase and attachment
accounting. It checks the actual service/Cashier and original account, attachment
and target balance before recording. Local operation/namespace correlation remains
the host's responsibility because the provider wire method has neither field.
The v1 intent record stores normalized balance components, provider error/decode
categories or reject codes; it retains no response buffers or diagnostic strings.
The maximum intent record is now 1 KiB, using the existing two host memories.
No migration, compatibility reader, new schema generation or allocator is added.
Cross-release transitions remain reinstall-only.

The additive operator `outcome` reader preserves exact input identity, local
transport phase, optional structured response and validated transfer facts. The
probe workflow applies shared conservative reconciliation policy before boundary
conversion. Prepared/uncertain offers remain potentially spent; exact accepted amounts
require independent credit evidence even after reported success. Transport-only
observations remain distinct from missing intents and structured replies. Exact
replay cannot refund twice or erase a response; conflicting replies reject.
Response/phase mismatches reject during transitions and restored-record inspection.
Same-release restoration preserves outcomes while fencing every mutation.

All 22 storage PocketIC cases pass. The shared transport now uses this atomic
writer; actual callback-write traps roll back response and accounting while remote
acceptance survives. Wrong callback account/offer/target remains uncertain. Zero,
partial and full acceptance, malformed/error replies and rejection retain their
independent reconciliation status through upgrade; unauthorized queries reject.
All 27 targeted native funding and 14 billing-model cases pass, including bounded
record widths, full diagnostic conversion, immutable replies and phase validation.
Affected strict Clippy, both release Wasms, warning-free core rustdoc, formatting
and diff checks pass. The final storage run passed in 34.81 seconds. The earlier
batch's 18 funding fixture cases remain prior evidence, not a rerun in this slice.
No full CI, resource benchmark refresh, dependency/version change or live call ran.

Next, establish complete account activity and independent credit evidence before
production payment admission. The new per-intent diagnosis is not an account-wide
clearance, spendability proof, provider qualification or restore authority. Other
provider intents, read sessions, both adapters and operational recovery remain open.

## Shared Cashier transport — included in 0.2.5

`ops::caffeine::funding::transport::PreparedCashierTopUp` owns one canonical
unbounded IC call. It captures actual service/original Cashier identity, measures
call cost before adding the attachment, and supplies current platform liquidity
with explicit host holds to the existing full-offer policy. Consuming an unpolled
call proves it unsent. Enqueue failures never sample a callback refund. Actual
callbacks capture their refund before bounded decoding or another await; accepted
attachment arithmetic stays independent of malformed replies, provider errors and
rejection. Unbounded wait preserves exact refunds but a stalled peer can obstruct
upgrades. No endpoint, lifecycle ownership or retry loop is implicitly exported.

The storage probe persists the journal's exact first-attempt marker before using
this transport, then records the original intent and call-correlated outcome.
The existing funding probe supplies a driver-configured local Cashier substitute:
it checks exact canonical bytes and uses actual IC cycle acceptance. The tests
exercise zero/partial/full acceptance, malformed/error replies and rejection,
receiver-trap rollback/full refund, operator and exact-intent isolation, and full
liquidity refusal without dispatch. A sender callback-write trap preserves the
receiver's acceptance and the full uncertain local reservation; retry and restored
dispatch remain blocked. Substitute responses do not qualify deployed Caffeine.

All 21 storage and 18 existing funding/recovery PocketIC cases pass. The expanded
zero/full-acceptance case also passes after the storage run. All 24 targeted native
funding cases, affected all-target strict Clippy, both release Wasms, warning-free
core rustdoc, formatting and diff checks pass. No full CI or resource benchmark
refresh ran. The durable outcome step above now retains structured replies as well
as transport arithmetic. Complete account activity, spendability, independent
credit evidence, provider/account qualification and production payment admission
remain open. Continue keeping restored instances fenced.

## Funding history discovery — included in 0.2.5

`StableFundingJournal::history` lets the configured operator recover exact original
intents and their current local states without a saved request. It traverses the
existing stable index in descending operation-ID order, so any prepared/uncertain
intent appears first on a fresh sweep. Trusted host limits bound returned intent
values, with one index lookahead. Queries independently check service, Cashier,
account and namespace; cursors retain the complete scope and an exclusive upper
operation bound. They are untrusted current positions, not snapshots or freshness
proof. A fresh sweep is needed for new intents or changed states behind a cursor.

The storage probe exposes the same reader through a bounded query with a fixed
two-result host limit. It returns the original attachment and optional target so
the existing canonical request inspector can be used without guessing lost input.
Terminal transport records remain visible; uncertain offers stay reserved.
Restored history stays readable while all funding mutations remain fenced.

Validation passes 13 funding journal unit cases, all 17 storage PocketIC cases,
affected strict all-target Clippy and the release storage-probe Wasm build.
Warning-free core rustdoc, formatting and diff checks pass as well.
Coverage includes full-width/gapped IDs, cursor boundaries, each scope field,
operator isolation on empty ranges, fresh sweeps after activity, returned-row
validation, rolled-back writes and exact discovery/request inspection through
same-release upgrade. This adds a library read API and a test-probe query, not
payment transport or a production operator endpoint. No stable schema, allocator,
dependency or package version changes are involved.

This discovery path supplies operator visibility; it does not qualify account
activity/spendability gates or release the restore fence. The transport step above
now supplies local callback evidence without enabling a production payment endpoint.

## Resource-test runtime — included in 0.2.5

The full 704-object release-history test queues independent objects in small
groups, still using separate IC messages and checking each reply. Population
respects the real two-active-upload tenant limit; later phases queue up to 32
distinct objects. Reported population milestones run alone. The admission probe
now retains a fixed operator-only window of 32 diagnostic samples; callers request
only the newest samples they need. Contiguous sequences, caller identity, decoder
checkpoints and instruction ceilings are checked for every queued update.
The same fabricated content uses the existing manifest builder, eliminating the
extra independent leaf-hash pass and repeated time queries.

All 704 objects, four reference generations, exact cleanup retries, capacity
checks, stop/start and separate deletion/settlement remain. The unchanged local
test took 224.07 seconds; the final optimized run passed in 119.20 seconds,
about 47% less time. These are local host wall times, not service/provider cost
claims. All 31 admission PocketIC cases pass, including diagnostic window rollover,
bounded reads, authority, unchanged observations on rejected decoding and stop/start.
Affected strict Clippy, release admission Wasm, formatting and diff checks pass.
Historical artifact-bound reports remain
unchanged; new reports stay under `.tmp/`. The test optimization changed no
allocator, dependency or public service API. No version mutation, commit or full
CI run is part of this batch.

## Durable local funding intents — released in 0.2.4

`ops::service::funding::StableFundingJournal` uses two host-granted memories for
bounded exact intent history and maintained attachment accounting. It binds the
service/operator/Cashier/account/namespace and an explicit allocation, reserve and
lifetime limit. New externally supplied IDs increase; replay checks the complete
local identity and amount. Prepared/uncertain intents retain the full attachment
and block later reservations. The first attempt marker persists possible dispatch;
repeating it rejects. The intent retains the sole maintained top-up method and
exact optional target balance. Shared encoding always supplies an explicit account
and preserves the target option independently of the attachment. `mark_attempted`
returns canonical call arguments after the marker write; operator request inspection
also works while fenced and grants no dispatch authority. No provider call or payment
endpoint is enabled.

Trusted-host enqueue-failure or exact unbounded-refund observations update one
intent and its totals in a synchronous IC transaction. Identical outcome replay
changes nothing; conflicting evidence and return-total overflow preserve state.
Outcomes additionally check separately supplied actual service/original target
against the retained identity before mutation. The host must authenticate that
transport context; payload assertions cannot supply it.
Accepted amounts remain charged independently of provider credit. Accounting shares
the existing `FundingAllocation` model, which also reconstructs retained history.
Reopen rejects missing/inconsistent rows and changed scope/allocation, then fences
every mutation, including late outcomes. Its own ordered IDs cannot prove freshness.

At the 0.2.4 closeout, the storage probe added two host memories and labelled
bookkeeping controls without sending cycles. Fourteen storage PocketIC cases passed, including
intent/accounting rollback, every funding phase through upgrade and canonical
request preservation with changed-argument/source rejection.
Targeted validation passes 169 core billing/policy/catalog/service/lifecycle/Cashier cases,
nine existing funding-probe unit cases, affected strict Clippy, release storage
Wasm and warning-free rustdoc. No full CI or resource benchmark refresh ran here.
A concurrent Cargo edit updated ic-testkit from 0.10.0 to 0.10.1 and reformatted the
workspace list; it was preserved, and final checks used that state. The allocator
and package version remain unchanged by this work.

Anonymous Cashier metadata was refreshed on 2026-09-27 and matches the retained
interface hash. Independent didc request vectors cover absent and maximal target
balances. The wire method has no operation-ID field; local intent correlation does
not establish remote idempotency or provider credit.

Next, qualify actual Cashier transport and
authenticated outcomes together with account activity, spendability and execution-cost
gates. Other provider journals, read sessions, adapters and operational
restoration remain incomplete. Continue keeping uncertain obligations inspectable
without permitting repeated effects or releasing the restore fence.

## Released foundation

The detailed prior handoff is retained in Git at v0.2.3. Historical results belong
to their original builds; see [core evidence](../evidence/core-primitives.md) and
[the changelog](../../CHANGELOG.md). The maintained implementation includes:

- Streaming Caffeine hashing, bounded manifests/builders and root verification.
  Direct browser/client upload is selected; admission has no mandatory raw digest
  or on-canister file hashing. Provider enforcement remains unqualified.
- A transient shared upload owner with explicit tenant/uploader/service/operator
  and namespace bindings, enrollment generations, exact permissions/retries,
  manifest/metadata validation and retained original headers.
- Indexed content discovery, exact-live-reference descriptors, admission/reference
  headroom and maintained quota totals. Reservation, logical release, physical
  deletion and billing cessation remain separate. Cancelled/settled operations,
  roots, leaves, references and receipts consume lifetime capacity.
- An unpublished IC admission probe delegating to that owner through bounded
  operation-specific inputs. Actual caller/time, stop/start and history-capacity
  evidence exists. Both upgrade hooks reject; operational persistence is absent.
- Independent billing/read/recovery PocketIC fixtures over labelled substitutes.
  Their restored journals stay permanently inspection-only; they do not prove
  operational recovery or deployed Caffeine behavior.
- Local manifest/inventory preparation and saved-body snapshots, verified file
  output, and `blob-fixture-inventory` joining prepared reports to capacity and
  reuse queries. Production transport, authentication and publication remain open.

The default Rust allocator is required; ic-memory remains stable-memory authority.
The released reference path avoids full lifecycle copies. Historical envelopes
include 704 objects / 288 MiB synthetic media, 256 references, maximal metadata and
one read slot. The recorded descriptor build used 4,587,520 allocated Wasm bytes;
final admission was 3.11M instructions and peak retain/release 5.43M/5.51M. Bulk
Candid decoding reduced the fixture's 1 MiB read to about 116M instructions, with
hashing still about 81M. These artifact-bound observations are not production
limits. Do not rotate their hashes or relabel old measurements for a new build.

## Durable upload baseline — released in 0.2.3

The maintainer clarified that the filesystem journal is optional client tooling,
not a requirement to run a local version of the service. Continue on shared stable
canister storage; do not expand local journal tooling as the primary delivery path.

`ops::service::tenant::StableTenantEnrollments` is the first shared stable
component. It accepts a host-granted `ic-memory` memory and writes individual
bounded v1 enrollment records, using the same model transition as the heap owner.
Metadata binds service, operator, namespace and lifetime tenant limit. The host
still owns installation/release checks and all other configuration/state stores.
No endpoint, lifecycle hook, grant or memory ID is exported implicitly.

Fresh installation rejects any allocated memory. Reopening uses load-only access,
checks the binding and every enrollment in one bounded pass, and returns an
inspection-only owner with all mutations fenced. Missing or corrupt state cannot
be initialized away. Records retain suspension, activation generation and lifetime
capacity. Synchronous canister hosts must propagate stable-memory traps for IC
rollback; native tests do not supply that platform transaction guarantee.

`StableRootClaims` adds two host-granted stable maps for immutable root claims and
the reverse object index, sharing the heap model's claim rules. Metadata binds
service, namespace and lifetime root capacity; full-width object identities are
preserved. Exact replay needs no extra capacity. Reopen validates the index
bijection, rejects missing/conflicting/orphaned history without repair, and fences
mutation. A root claim alone does not check enrollment or reserve object bytes.

`ops::service::uploads::StableUploads` exclusively owns enrollment, root claims,
exact permissions, immutable manifests, confirmed lifecycles, individual references
and receipts, maintained global/tenant totals and a root/request index. Ten distinct host-granted
memories commit together in synchronous IC updates. This replaces the earlier
unreleased pending-only owner; no compatibility alias or old schema path remains.
Shared model decisions govern admission, references, cleanup receipt capacity,
physical deletion and settlement. Ordinary reference mutations touch individual
rows instead of loading or copying the complete reference/receipt history.

Unexposed cancellation releases bytes but retains operation/root/leaf history.
Exposed uncertainty remains charged after revocation. Exact completion transfers
the reservation to a confirmed object and first reference without dropping bytes.
Last release removes logical bytes, physical deletion removes physical bytes, and
billing cessation removes liability bytes. References and exact success/failure
receipts remain retained after settlement. Replay never reactivates references;
suspension blocks fresh retains while preserving receipt replay and release.

Indexed tenant discovery preserves independent request/object IDs. Original metadata
and reference-qualified descriptors come from one bounded manifest record; the
latter require confirmed completion and the exact live reference. Unknown/foreign
roots return the same absence. Tenant and operator history/cleanup scans bound
inspected rows and results independently. Cursors bind service, namespace, scope,
filter and last inspected tenant/request ID; empty filtered pages advance. Fresh
sweeps are required for changes behind a cursor. Continuing billing remains in
the outstanding view after physical deletion. Admission/reference headroom uses
maintained counters and shared heap-model arithmetic, preserving release receipt
capacity and lifetime operation/reference/leaf history. Operator-only root batches
use indexes, retaining order, duplicates, malformed positions and every local phase.
Scope/authority checks apply even to empty batches. These observations do not
implement gateway callbacks or grant provider deletion/retry authority. Suspended/restored reads remain
inspection-only and cannot release the mutation fence.

Bounded v1 records preserve full-width identities and first-accepted metadata.
The manifest codec enforces 64 KiB while variable-size tree pages avoid allocating
maximum-value nodes for small declarations. Installation rejects oversized
manifest envelopes before any allocation. Reopen validates upload configuration,
all store relationships, manifests, reference/receipt counts and recomputed totals without repair, then
fences every mutation. Host release identity and billing/provider configuration
checks remain separate. Reopening is inspection, not operational recovery.

The unpublished storage probe uses this single owner. Eleven PocketIC cases cover
admission, preparation, cancellation, completion, reference/receipt and settlement
write traps, then same-release upgrades of pending and every confirmed phase.
Stable and cached state roll back together; missing receipts after a trap do not
consume cleanup headroom. Stop/start preserves state; restored mutations remain
fenced. Changed-operator restoration leaves the prior instance unchanged. Faults
exist only in fixture memory wrappers. Completion/deletion/settlement facts are
operator-only labelled substitutes, not evidence of deployed Caffeine behavior.
The core's confirmation APIs require independent host authentication/correlation.
The expanded admission fault case includes the root/request index write. The two
new read cases check actual caller isolation, exact-reference descriptors through
upgrade, changed cursors, bounded empty pages and fresh cleanup sweeps.
Two planning cases additionally check admission/reference headroom through cleanup
and settlement, caller/scope isolation, raw root-batch bounds and order, uncertain
and cancelled roots, unchanged accounting and fenced inspection after upgrade.

The 0.2.3 implementation validation passed 122 targeted catalog/service/lifecycle cases,
including heap/stable accounting agreement, cleanup headroom, immutable historical
failures, every release phase, missing/orphaned rows, codec widths, near-bound
manifests, small-manifest allocation, root/request consistency, bounded reads,
heap/stable headroom agreement through settlement, shared global contention and
operator root observations across every phase.
The final storage layout is also checked by the eleven PocketIC cases, strict affected all-target Clippy,
release storage-probe Wasm and warning-free core rustdoc. External dependency
versions and the default allocator are unchanged. No full CI or resource
benchmark refresh ran; the small fixture is not production sizing evidence.

The heap owner still supplies verified-read APIs. Complete provider-call intents,
callback authority/correlation, read sessions, provider
economics and actual adapters remain incomplete. Do not imply all service
obligations survive yet. Operational restoration still needs a complete obligation
source and independently surviving authority; the current fence has no unfence API.

## Receipt inspection and local intent journal — released in 0.2.3

`UploadAdmissions::reference_receipt` authenticates and reads an exact reference
operation's original result without mutation. Mutation replay uses the same check
path. Unknown roots and changed identities remain errors; an absent receipt
allocates nothing. Suspension, full history and settlement preserve original
success or typed lifecycle failure. Historical retain success is not current
liveness. The private bounded query also checks the original upload's declared size.

`blob-fixture-reference` saves a bounded exact `ReferenceIntentRecord` in an
identity-keyed local journal, then queries only that receipt. The record binds the
asset label, service, tenant, namespace, upload/object/lifetime, root, bytes,
reference, operation and retain/release action. IDs are explicit full-width decimal
strings, never automatically allocated. The fixture fixes object ID to upload ID
and incarnation to one; both are recorded explicitly.

Saving holds an exclusive OS file lock, syncs a private file, installs it without
replacement and syncs the journal directory before acknowledgment. Exact retries
recover the same file; changed payloads conflict. At most 4,096 entries besides
the permanent `.writer.lock` are permitted, including interrupted residue. Full
journals still allow exact recovery; there is no automatic deletion or eviction.
The existing journal directory must be durable and caller-controlled. The lock
file must never be removed/replaced; process exit releases the OS lock.

This is tested Linux local persistence, not production service recovery. Files
remain mutable, storage errors can leave unacknowledged records, and copies or
rollbacks have no independent freshness authority. No saved file or receipt proves
ID freshness, restored-instance authority or safe provider retry. The tool sends
no mutations. Consumer transaction and outbox coordination remain outstanding.

Latest targeted validation passes all 26 host-tool unit cases (nine reference
cases), a subprocess termination/lost-output recovery case and the three updated
query/executable PocketIC cases. Strict host-tool all-target Clippy passes. The
preceding receipt step passed 37 upload model cases, release admission Wasm,
affected Clippy and warning-free core rustdoc for that preceding step.
Existing locked tempfile and sha2 are used by the unpublished executable; library
dependencies, allocator and package versions are unchanged. No full CI, hardware
power-loss test or resource benchmark refresh ran this batch.

## Next work and open gates

Before publisher dispatch, settle production intent storage, copy/restore fencing
and a surviving allocation authority. The local journal now serializes cooperating
writers and recovers exact writes, but cannot detect a stale copy. Preserve exact operations after
unknown outcomes; absence never creates retry authority. The inventory's one fresh
reference per asset is explicit, but new-object reference sizing is unassessed.
Production descriptor delivery, provider locator/serving policy and consumer
registration/release coordination still need implementation and evidence.

Close M1 provider guarantees before real certificates: pre-charge size/tree
limits, namespace/owner/project/bucket and replay charging, independent completion
and actual size, lost paid-response reconciliation, physical deletion and final
billing cessation. Recovery needs a surviving complete obligation source; an IC
version counter or counter in the same old backup is insufficient. No trial
account, namespace or paid budget is selected. See [provider review](../provider-review.md)
and [service contract](../service-contract.md).

Reuse requires a live reference. Final release queues deletion; retired roots
cannot be reallocated even after settlement. Deleted-content reintroduction is an
open M1 identity/provider decision. Never weaken callback safety or erase charges.

Toko review uses remote `development`; the local checkout is absent/stale. The
[pinned review](../evidence/toko-0.2-review.json) records 10 MiB/file and 500 MiB
staging inputs. [Miner feedback](../roadmap.md#toko-miner-feedback--2026-09-27) adds
proposed headless release media with 702 files / 270.1 MiB; this is not a qualified
publish list or approved adoption. Recheck upstream before depending on new
provider/consumer behavior. No sibling edits or consumer tests ran here.

## Constraints that remain active

Caffeine is the sole provider. Core builds without Canic; both adapters belong
here and use shared handlers. Linking a library exports no endpoints/lifecycle.
Production schema/transport gates remain open: another transient fixture does not
complete M2, and local file storage does not qualify the production outbox.

AGENTS.md explicitly requires 100% hard cuts before 1.0: remove superseded forms
and update consumers/tests/docs together, without compatibility or migration paths.
Pre-1.0 cross-release transitions remain reinstall-only; same-release interruption
recovery is required. Source removal and installation retirement are separate.
Never erase the only provider, balance, uncertain-effect or billing records. All
Canic capabilities must work here before removal there; see [parity](../canic-parity.md)
and [acceptance](../acceptance-plan.md). This work does not accept Canic's closeout.

No controller/digest authority, automatic funding, compatibility shims or silent
reset. Siblings are read-only. Commits remain maintainer-owned; release/publication
need explicit authority and cleanup is never implicit. Follow
[governance](../governance/development.md); use targeted implementation checks.
