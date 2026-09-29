# Current status

Date: 2026-09-29

## Active work — exact reference inspection after 0.2.23

The maintainer confirmed **0.2.23 is pushed** and requested continued work toward
the 0.2 service journey. Cargo and the release receipt are 0.2.23. Local main,
origin/main and the peeled v0.2.23 tag resolve to
`1d15bc80a5e350982ef33f80da06d7af28a8ee28`, from validated source
`4ef51000ad4bae88028d70b9fce811bd6552369f`. The receipt records the
`release-verify` gate; registry publication was not independently queried.
The dated 0.2.23 notes match that release. New completed work is recorded in the
maintainer-requested, undated 0.2.24 changelog draft; Unreleased is empty. Package
version and release receipt remain at 0.2.23. The draft name does not establish
patch release readiness. The required standalone project mapping changes its init
contract and current stable schema. The next release must be a minor release
(0.3.0), not a 0.2 patch, and the draft must be renamed for that release. This does
not close the remaining delivery milestones below. No version mutation is
authorized or performed.
The browser fixture retains Caffeine 1.1.2 and supported SDK 5.4.0.
The discovery and descriptor work below was already dirty and is preserved in this batch.
No dependency changed.
No version mutation, commit, publication, deployment or deployed-provider effect ran here;
transfers below use simulated cycles between local PocketIC canisters.

Follow the [0.2 delivery plan](../roadmap.md). Its goal remains a usable
Caffeine-backed service through shared durable handlers, both adapters and an
operator client. Library publication does not qualify the service or Canic removal.

| Milestone | State | Remaining completion condition |
| --- | --- | --- |
| M1 — contract | In progress | Freeze admission/resource envelope, provider guarantees and operational recovery |
| M2 — durable standalone service | In progress: initial standalone host, durable shared stores and IC evidence | Connect remaining provider/operator handlers and prove operational recovery/full journey |
| M3 — Caffeine and operator integration | In progress: shared transports with local IC evidence | Qualified provider transport, completion/economics and production client |
| M4 — managed parity and acceptance | Not implemented | Same journey through Canic adapter, complete replacement evidence and handoff |

## Current focus — exact current reference status and client

`workflow::references::status::inspect` now exposes tenant-only
`blob_reference_status` in standalone and the durable storage fixture. It reuses
the maintained exact reference lookup and shared original-upload conversion under
one borrow, returning an exact request echo, current local liveness and the owner's
restore fence. No mutation action or receipt operation is accepted. Unknown or
changed uploads and unconfirmed content reject; never-retained and released
references of a confirmed upload are both non-live. Suspension, settlement and
restoration preserve inspection. Historical retain success can coexist with a
non-live current reference; a live result does not prove active enrollment or
serving/retry authority. No reference, receipt, reservation or provider effect is
created, and no history scan occurs.

The storage fixture's private boolean endpoint and conversion are removed; existing
lifecycle/recovery tests consume the shared boundary and check its fence.
`ReplicatedReferenceClient::status` validates the exact lookup and bounded reply
through the same single-call transport as receipt/mutation delivery, with no attached
cycles or automatic retry. It preserves refusals and both independent flags. Local
consumer controls remain labelled fixtures and accept no provider facts on this path.

Three new native boundary/reply cases pass, including independent full-width IDs,
changed arguments, suspension/release/settlement and fenced decoding. Eleven
affected PocketIC cases pass: one standalone and ten storage lifecycle/client cases
(45.52 seconds IC execution). Actual canister-client coverage includes wrong callers,
small reply bounds, query-execution refusal, passive stable memory and inspection
after independent service/client restoration. Receipt/mutation regressions also
pass after sharing transport internals. Confirmed facts remain local substitutes;
standalone still cannot establish provider completion.

Strict affected all-target/all-feature Clippy, release standalone/storage Wasm,
generated Candid comparison, warning-free core/host rustdoc, formatting and diff
checks pass. No full CI/release gate ran. This step changes no stable schema, memory
grant, allocator, dependency or version; the prior host-init minor-release requirement
remains. Qualified provider completion/economics, operational recovery, production
consumer coordination and Canic parity remain open.

## Standalone operational descriptor delivery — current draft batch

The standalone host now exposes update-only `blob_download_descriptor` through
the existing shared download handler. It authenticates actual caller/service,
requires active enrollment and the exact confirmed live reference, and refuses
restored owners. It returns original metadata with the installed owner/project;
there is no provider call, body hashing, read-session allocation or stable write.
Saved descriptors and bytes are not revoked by a later refusal.

Installation now takes `HostInstallationInput { configuration, project }`, with
the shared configuration DTO unchanged. An explicit project is validated before
allocation and retained in the current v1 host record; restore revalidates it before
publishing the owners. Operator configuration readback includes it. Owner is always
the actual service and the project maps to the installed namespace; tenant/payer/
namespace text never supplies a default. This is representation validation, not
provider assignment proof. Future dispatch must use this same provisioned mapping.
The previous host init/schema is replaced directly, with no fallback/migration.
Memory grants, shared store schemas, allocator and dependencies are unchanged.

All 24 standalone PocketIC cases pass across the suite and targeted rerun. The
initial run exposed the local consumer fixture's hardcoded target namespace; it
now uses its operator-selected request namespace, and the full-width standalone
client case passes. Three affected storage descriptor cases pass, retaining
confirmed success, original metadata verification, exact-reference release,
suspension, restore refusal and actual canister-client delivery. New host coverage
includes malformed/oversized ingress, query refusal, invalid project rollback,
the 256-byte UTF-8 boundary and corrupt-project restore rejection. Confirmed
success still uses labelled fixture provider facts: standalone cannot yet establish
completion and exports no trusted-fact hook.

Strict affected all-target/all-feature Clippy, host/storage/gateway-source release
Wasm, generated Candid comparison, warning-free host rustdoc, formatting, diff
checks and the standalone target's command preview pass. That target now builds
its consumer fixture. No full CI/release gate or deployed provider operation ran.
Remaining provider/operator integration, qualified completion, operational recovery
and Canic parity stay open; the version requirement is not service acceptance.

## Shared indexed content discovery — current Unreleased batch

`workflow::uploads::discovery::inspect` now exposes tenant-only
`blob_lookup_content` in the standalone host and both admission fixtures. It uses
the existing root index and history conversion, preserving independent full-width
upload/object/incarnation/first-reference identities and every local lifecycle
phase. Scope checks precede visibility. Unknown and foreign roots share absence;
responses echo the exact request and report the same owner's restore fence even
when no content is visible. Suspension and restore preserve passive inspection.
There is no manifest/history scan, reservation, stable write or provider effect.

The private discovery endpoints and their dedicated owner calls are removed.
Fixture descriptor/root-batch DTOs still serve their distinct maintained views.
Inventory now consumes shared upload-capacity, discovery and reference-capacity
queries only. It validates request echoes even for absence, rejects zero original
identities, renders independent IDs as decimal strings and retains a later fence.
Sequential observations grant no fresh admission, retry or serving authority.

Both native discovery boundary tests and all fifteen inventory unit tests pass.
Twelve targeted PocketIC cases pass: two standalone discovery, six durable reads,
two transient discovery and two inventory subprocess cases (28.38 seconds total
IC execution). They cover actual caller isolation, bounded ingress, passive query
and replicated reads, full-width independent IDs, suspended/restored discovery,
cancelled/uncertain history and confirmed release/deletion/billing settlement.
Confirmed provider facts remain labelled local substitutes. Strict affected
all-target/all-feature Clippy, all three release Wasm builds, generated standalone
Candid comparison, warning-free core/host rustdoc, formatting and diff checks pass.
No full CI/release gate ran. Stable schemas, memory grants, allocator, dependencies and
version were unchanged by discovery. The descriptor work above extends the host
configuration; provider completion, operational recovery and Canic parity remain open.

## Shared upload/reference capacity and exact inspection — included in 0.2.23

Release-check follow-up: inventory test reply helpers now return encoded bytes;
the simulated query closures own the transport `Result`. The earlier targeted
Clippy selection omitted library unit-test targets. Strict package-scoped Clippy
with `--all-targets --all-features` now passes, as do all thirteen inventory unit
tests, formatting and diff checks. This fix changes no runtime behavior.

`workflow::references::capacity::inspect` now provides `blob_reference_capacity`
in the standalone host and both admission fixtures. Requests bind the exact tenant
scope and provider root. The same maintained model counters supply lifetime
references, unreserved receipts, reserved cleanup receipts and fresh-retain
headroom. Responses echo the request and expose the owner's restore fence even
for absent content. Unknown, foreign and unconfirmed roots remain indistinguishable;
retired objects retain history with zero fresh retains. Suspension/restore allow
inspection only. No reservation, receipt, provider effect or record is created.

The private reference-capacity DTO, endpoints and duplicate conversions are removed.
Inventory inspection checks exact echoes and reports a fence observed after its
earlier upload-capacity query. Content discovery was still a private fixture query
at this release; the current batch above replaces it.
Both native boundary tests and all thirteen inventory unit tests pass. Eight
affected PocketIC cases pass: two standalone, two storage planning, two inventory
subprocess cases and both reference/multifile cleanup resource cases. IC execution
totals 142.69 seconds, including 125.45 seconds for the multifile case; this batch
does not claim a performance improvement or refresh the retained resource reports.
Strict affected Clippy, all three release Wasm builds, generated standalone Candid
comparison, warning-free core/host rustdoc, formatting and diff checks pass.
No full CI/release gate or deployed-provider operation ran.
Stable schemas, memory grants, allocator, dependencies and version are unchanged.

`workflow::uploads::capacity::inspect` now provides tenant-only
`blob_upload_capacity` in the standalone host, durable storage fixture and transient
admission fixture. All use the same scope/DTOs/conversion over existing headroom
calculations. A read-only ops access trait handles the two maintained owners; the
heap owner has no restore path, while the durable owner exposes its actual fence.
Missing enrollment rejects; suspension and restoration preserve inspection.
Independent lifetime/concurrent/leaf/byte dimensions reserve nothing and establish
no provider, allocation or retry authority. No histories are scanned.

The private capacity DTOs, endpoint names and duplicate conversions are removed.
Existing tests and the local inventory tool consume the shared contract; inventory
JSON includes the fence and reports `service_fenced` despite spare quota.
Three native boundary tests and twelve inventory unit tests pass. Nine affected
PocketIC cases pass (8.98 seconds total IC execution), including standalone shared
contention, passive replicated reads, scope/ingress rejection, suspended cleanup,
restore fencing, billing through settlement and actual inventory subprocesses.
Release Wasm for all three hosts and generated standalone Candid comparison pass.
Strict affected core/host/integration Clippy, warning-free core/host rustdoc,
formatting and diff checks pass. The removed fixture capacity contract has no
remaining Rust consumers.
No stable schema, memory grant, allocator, dependency or package version changed.
No full CI/release gate or deployed provider operation ran.

The standalone host now exports `blob_upload_status` through the existing
`workflow::uploads::inspect` handler and shared DTOs, matching the storage fixture.
It authenticates actual tenant/service and binds the complete original upload,
with independent full-width operation/object/incarnation/reference identities.
The query reports retained local phase and revocation without requiring uploader
or expiry. It stays passive through suspension and restore; historical confirmation
is not current liveness, retry, publication or operational recovery authority.
The existing response has no fence field; configuration/history retain that
separate observation. No new schema, record, memory grant or provider effect exists.

Both new standalone PocketIC tests pass (4.47 seconds), covering ordinary and
replicated passive inspection, unknown operations, malformed/changed bindings,
caller isolation, cancellation under suspension, ingress bounds and restoration.
Strict host/standalone-test Clippy, release host Wasm and generated Candid comparison
pass. No full CI/release gate ran. Remaining provider/operator handlers, qualified
completion, operational recovery and the Canic adapter remain open.

## Shared gateway revocation, refresh and cancellation — included in 0.2.22

`dto::gateway` and `workflow::gateways::revocation::revoke` now expose
`blob_revoke_gateway` in the standalone host and storage fixture. Full operator
scope and actual service/caller checks precede the existing durable removal.
Absent and final-member removals invalidate earlier sync/read observations while
preserving occupied read slots and upload/funding accounting. Responses echo the
request and report membership change; there is no historical receipt or automatic
retry. A later explicit addition/sync can re-add a member. Provider credentials,
physical deletion, billing and restore fences remain independent.

The fixture's separate removal action/conversion is removed; normal tests use the
shared endpoint and transaction fault controls call the same handler. Two native
tests and ten targeted PocketIC cases pass (27.80 seconds for IC execution), covering
scope/actor rejection, repeated absent removal, rollback, stale sync replies,
remove/re-add during an outstanding read, preserved occupancy/accounting and fenced
restoration. Strict affected Clippy, generated Candid comparison, release
standalone/storage/gateway-source Wasm, warning-free core/host rustdoc, formatting
and diff checks pass. No stable schema, memory grant,
dependency, allocator or version changed. No full CI/release gate ran.

Shared `workflow::gateways::sync` now connects `blob_sync_gateways` and exact
`blob_cancel_gateway_sync` in both hosts. Refresh records the pending identity
before one canonical replicated Cashier query, drops store borrows across the
await and rechecks the original attempt on completion. Standalone fixes a
30-second bounded wait, 64 KiB reply cap and independent decoding/membership bounds.
There are no attached cycles, retries, method fallbacks or automatic cancellation.
Failures retain the pending sequence in local status; exact operator cancellation
preserves membership, read generations and allocated history. Restore fencing stays
in force. These endpoints do not qualify provider semantics or enable paid effects.

The fixture's separate cancellation action is removed. Retained fixture attempts
now resolve by durable sequence so shared refreshes and adversarial fixture calls
cannot confuse a vector position with an operation identity. Two new native tests
pass, as do two standalone and ten affected storage PocketIC cases (40.05 seconds
IC execution). They cover both-host refresh, malformed/empty/oversized/rejected
replies, overlap, stale cancellation, cancellation of a held reply, rollback and
retained pending work across restore. Strict affected Clippy, Candid comparison
and release host/storage/gateway-source Wasm pass. Warning-free core/host rustdoc,
formatting, diff checks and the standalone target's effect-free command preview
also pass. The standalone test target now
builds its required local provider substitute. No dependency, stable schema, memory
grant, allocator or version changed; no full CI/release gate ran.

Next, connect remaining provider/operator handlers and resolve qualified completion
and operational recovery; this local revocation does not complete M2 or M3.

The maintainer's validation exposed an outdated operator gateway-query assertion:
passive malformed, oversized and empty fixture replies now return bytes for client
validation rather than transport refusal. The test now separates those replies
from scripted-effect refusal and checks typed client errors, unchanged pending
sync and unchanged source journals. Caller and restore rejection checks remain.
All 48 operator PocketIC cases pass (70.44 seconds with two test threads). This
corrects test expectations only; no production or fixture behavior changed.

## Shared operator funding inspection and client — included in 0.2.21

`dto::funding` and `workflow::funding::history::inspect` now expose the existing
bounded durable journal through `blob_funding_history` in the standalone host and
storage fixture. The API reuses `OperatorScope`, preserves exact full-width IDs,
offers, optional target balances and transport phases, echoes the request and
reports the restore fence. The standalone host returns at most 32 entries per
query. Descending cursors bind the complete scope and grant no retry, provider
credit, account-completeness or recovery authority. New or changed activity behind
the cursor requires a fresh sweep; no whole-history reconstruction occurs.

The fixture's private history DTOs, conversion and endpoint are removed. Its
separate summary fixture uses the maintained operator scope too. No stable schema,
configuration, allocator or provider effect changes. The standalone host still
exports no funding mutation; actual nonempty histories below use local substitutes.

Five targeted native funding-history tests and six PocketIC cases pass. Native
coverage includes full-width offers, target balances and refunds through Candid
and restoration without writes. The IC cases cover standalone scope/actor/cursor
rejection and replicated passive reads, two-entry pages with sparse IDs, recovery
of canonical requests after restore, retained uncertain attachments, write rollback
and summary scope propagation. PocketIC execution totals 13.55 seconds. Strict
affected Clippy, generated Candid comparison, release host/storage Wasm and
warning-free core/host rustdoc pass. No full CI/release gate ran for this batch.

`dto::funding::outcome` and `workflow::funding::outcome::inspect` now add
`blob_funding_outcome` to both hosts. Exact inspection binds the original scope,
operation, offer and optional target balance; changing a retained intent rejects.
The response preserves phase, optional structured reply and restore fencing, with
named reported balance components, provider errors and individual decoder failure
categories. Workflow reuses shared reconciliation against retained transfer facts;
a reported success or later refund cannot prove provider credit. Absence and
transport-only history remain distinct. The fixture's private outcome DTO,
endpoint and conversion are removed without changing dispatch or stable schemas.

Three native outcome tests pass, extending the existing response-category matrix
through the shared boundary/Candid and checking malformed/changed input, scope,
absence and no writes. The new standalone case and all eleven affected transport
PocketIC cases pass in 50.38 seconds total. They cover actual replicated passive
lookup, local Cashier success/errors/malformed replies, liquidity refusal, delayed
callbacks, receiver/callback traps and fenced restoration. These remain local
substitutes, not deployed-provider evidence. Strict affected Clippy, Candid schema
comparison, release standalone/storage/funding Wasm and warning-free core/host
rustdoc pass. No full CI/release gate ran.

Next, connect remaining operator/provider handlers and resolve qualified completion
and operational recovery. History inspection does not establish service readiness.

`ops::service::funding::client::ReplicatedFundingClient` now consumes both shared
queries with pinned actual canister and complete operator scope. Each call uses a
bounded replicated wait with no attached cycles, automatic retry or query fallback.
Bounded decoding checks exact request echoes, full-width original amounts, strict
descending page/cursor order and consistency between refunds and reconciliation.
Remote refusal stays distinct from absence; fences and missing structured replies
survive unchanged. These reads need no durable command journal or new stable schema.

Six native client/reply tests pass, including the standalone-sized 32-entry page
with full-width amounts and rejection of malformed/cross-scope/contradictory replies.
Both targeted client PocketIC cases pass in 9.98 seconds: actual canister operator
authorization, host ingress isolation, composite-query refusal, small reply budgets,
multi-page discovery, unknown/changed exact intents and inspection after independent
client/service restoration. Service and client stable bytes stay unchanged by reads.
The consumer remains a labelled local substitute; deployed Caffeine behavior and
production operator authentication are not qualified by this evidence. No dependency,
allocator, version, stable schema or provider effect changed; no full CI/release gate ran.
Strict core/consumer/storage-integration Clippy, release storage/consumer Wasm,
warning-free core rustdoc, formatting and diff checks also pass.

## Standalone host and shared operational inspection — included in 0.2.20

`canisters/standalone` now owns actual init/post-upgrade hooks, one ic-memory runtime
and seventeen exclusive memories. Installation validates the complete explicit
candidate before allocation. A bounded v1 `ConfigurationRecord` retains the full
configuration, actual service identity and package release independently of DTOs.
Restoration takes no replacement inputs, rejects missing/foreign/release-mismatched
state and assembles all four owners synchronously under their existing fences.
Package release is not a module hash; the retained record is not freshness authority.

Tenant, admission/revocation, manifest preparation/inspection and reference endpoints
delegate to the shared workflows with platform caller/service/time. Configuration
readback is operator-only. Explicit Candid declarations preserve typed arguments
despite custom ingress decoders; the checked-in interface is tested against generated
schema with explanatory comments ignored. No fixture protocol or provider-fact
endpoint is linked.

The additive shared `dto::operator` / `workflow::operator::inspect` API now backs
`blob_local_status` in the standalone host and storage fixture. It checks the actual
service, explicit namespace/Cashier/payer scope, operator caller and matching full
service configurations across all four owners before returning local accounting.
Maintained upload/funding counters, one bounded gateway row and read occupancy form
one synchronous snapshot; there is no lifetime history traversal or provider call.
Each restore fence remains visible. Allocation is not platform liquidity, transport
acceptance is not provider credit, and the snapshot authorizes no retry or recovery.

The three standalone PocketIC cases pass in 16.35 seconds. Actual Wasm evidence
covers a 10 MiB manifest without body relay, tenant/uploader/controller isolation,
unconfirmed reference refusal, stop/start, failed upgrade arguments, retained
configuration/permission/manifest and refused mutations after restore. Failed
installation/reinstallation and malformed/oversized ingress leave previous stable
bytes intact. Missing memory, foreign installation and changed release identity
reject without repair. ic-memory legitimately commits allocation-ledger metadata
on successful bootstrap; tests compare service observations across restoration and
assert byte-for-byte refusal behavior against the post-restore image.

The standalone Candid schema test, strict host/integration Clippy, warning-free
host rustdoc and release Wasm pass. Formatting and diff checks pass.
`make build-standalone` and `make test-standalone` provide focused commands, and the
normal native/PocketIC gates include the host. No full CI/release gate ran.

Operator inspection adds three passing native cases for full-width accounting,
scope/actor rejection, mismatched owners, independent fences and absence of writes.
Three targeted PocketIC journeys pass in 10.55 seconds: standalone cancellation
frees reserved bytes while retaining operation history; interrupted funding, gateway
sync and read occupancy survive restoration; funding refunds, accepted/unsent
amounts and retained identities remain distinct. Existing storage journeys use
local substitutes and do not qualify deployed Caffeine behavior. The Candid schema
test, strict affected core/host/fixture/integration Clippy, warning-free core/host
rustdoc, release host/storage Wasm, formatting and diff checks pass. No service
stable schema, memory grant or external dependency changed
for this operator query.

The shared `dto::upload::history` / `workflow::uploads::history::inspect` boundary
now exports `blob_upload_history` in both hosts. It reuses the existing indexed
stable scan with explicit tenant/operator authority, scope-bound cursors and
host-owned independent work/result limits. The standalone endpoint scans at most
64 rows and returns at most 32 entries. Replies echo the request and preserve
distinct full-width upload/object/incarnation/first-reference IDs, current cleanup
state and the restore fence. Empty filtered pages advance; changes behind the
cursor require another sweep. Suspended/fenced inspection grants no retry or
provider authority. The fixture's private scan types, conversion and endpoint are
removed; other private fixture protocols remain only for their separate evidence.

Four targeted PocketIC history journeys pass in 13.23 seconds, covering actual
standalone caller/scope/cursor rejection, replicated passive inspection, cancelled
history, suspension and restoration; fixture pagination, changed cursors and
resweeps; and each lifecycle phase through physical deletion and billing cessation.
Provider transitions in that last case are explicitly local substitutes. Fourteen
targeted native read tests, the standalone Candid check, strict affected Clippy,
release host/storage Wasm and warning-free core/host rustdoc pass. No stable schema,
memory grant, dependency or version changed for this additive shared API. No full
CI/release gate ran.

Next, connect the operator/provider-facing handlers to this host and resolve the
qualified completion/reconciliation path. Certificate issuance and paid provider
effects are deliberately absent; an unfenced host is not provider readiness.
Operational restore remains inspection-only. Production clients, complete journey,
resource qualification and the Canic adapter remain open. See the compact
[host contract](../../canisters/standalone/README.md).

## Scoped tenant handlers and explicit operator client — included in 0.2.19

`dto::tenant` and `workflow::tenants` now own explicit service/namespace/tenant
enrollment requests, compare-and-set updates and correlated inspection responses.
The existing stable tenant model remains the transition authority. Only the
configured operator may update; the tenant and operator may inspect. Controllers,
uploaders and unrelated callers gain no authority. Replies report the retained
generation, activation state and restore fence without granting upload permission.

The storage fixture removes its private enrollment endpoints/conversion and now
exports `blob_update_tenant` and `blob_tenant` through the shared workflow. The
storage test driver uses the maintained DTOs and method names. Other independent
fixture protocols remain private to their own tests. No service stable schema, memory grant,
allocator or provider effect changed. Production host lifecycle and endpoint
ownership remain unfinished.

`ReplicatedTenantClient` now pins actual executing canister, service, namespace and
tenant, and sends a single bounded replicated call. It adds no automatic retry,
query fallback, cycle attachment or identity inference. Bounded reply validation
reuses the existing tenant transition model for exact acknowledgment generation
and activation state. Inspection retains absent/fenced observations and cannot
establish a command's historical execution. The host owns authentication and must
persist its command before polling an update future.

The existing consumer fixture retains one lifetime tenant command in its current
bounded record before dispatch, with no separate journal or reset/retry path.
It permits inspection after uncertain replies and restoration. This is a local
operator/observer substitute, not production persistence or Canic integration.
Four native client/reply tests pass, covering pinned scope/caller, full-width
namespace/generation, transition validation, refusal and bounded decoding.
Core/consumer/storage-integration Clippy, six existing consumer model tests and
release storage/consumer Wasm pass. Both client PocketIC cases pass in 10.90 seconds,
covering real canister operator/observer authority, query refusal, success, small
reply budgets, trapped callbacks and restoration of both client and service.
The saved command remains exact and cannot be dispatched again. The consumer
fixture's current bounded record is extended directly; no compatibility path is added.

Targeted PocketIC tests cover caller/scope isolation, zero/stale preconditions,
inspection after a discarded reply, capacity retained by suspended tenants and
rollback of enrollment/suspension/reactivation writes. All three pass in 7.96
seconds. The restore test checks inspection and rejection of no-op updates as well
as state changes. The two existing pending-upload/combined-obligation upgrade
journeys also pass through the maintained endpoints (4.60 seconds). Strict affected
Clippy, release storage/gateway-source Wasm, formatting and diff checks pass.
Final checks use the preserved current Cargo.lock. No full CI/release gate ran.

Closeout adds a stale-command client journey: changes to the active flag within
one generation and suspension/reactivation into the next generation both reject
the original precondition. The service bytes remain unchanged, the client retains
the exact rejected command and neither replay nor replacement dispatch is allowed.
Same-release client restoration preserves that intent and inspection while fencing
updates. The complete six-case tenant PocketIC group passes in 25.27 seconds;
strict storage-integration Clippy, warning-free core rustdoc, formatting and diff
checks pass. The release helper accepts the 0.2.19 draft and its effect-free plan
selects 0.2.18 -> 0.2.19 without modifying version files.

The maintainer subsequently released this batch as 0.2.19; the release receipt
records its full release-verify gate. Production authentication/persistence,
provider completion and operational recovery qualification remain open.

## Explicit configuration and synchronous owner assembly — included in 0.2.18

`dto::configuration` supplies passive Candid inputs for the existing service
configuration model, funding allocation and concurrent read limits.
`ops::service::configuration` bounds raw input and decoding
work, requires the actual host service identity, converts positive scalar/count
limits and delegates cross-resource and billing validation to the existing model.
It checks all four stable owners' resource/codec envelopes before allocating state.
There are no deployment defaults, clamping, state writes or provider effects.
Roles remain explicit and independent; balances, namespace and byte budgets keep
their full width. The host owns installation authentication and ingress buffering.

`ops::service::stores` now assembles uploads, funding, gateways and read sessions
from one validated configuration and sixteen explicit host memory grants. Fresh
install preflights the whole set; restoration requires every memory and validates
all owners synchronously before returning an inspection-only assembly. There is no
repair, missing-store initialization or unfencing. Hosts must propagate traps for
IC rollback; native memory does not provide that transaction guarantee.

The storage probe uses this path and removes its separate component initialization
functions. Candidate validation now precedes host memory bootstrap. Its small
fixture limits, init signature, memory layout and restore fence remain unchanged.
Memory grants, release/installation identity, production endpoint/lifecycle ownership
and provider qualification remain host responsibilities. Neither a production
standalone host nor a Canic adapter is implemented by this step.

Seven configuration tests and three assembly tests pass. They cover bounded Candid,
role bindings, zero/cross-resource limits, separate attachment allocation, read
budgets, stable codec bounds, every occupied/missing grant and changed funding/read
limits without byte mutation. Strict core/storage-fixture/storage-integration Clippy
and release storage/gateway-source Wasm pass. Two targeted PocketIC upgrade tests
pass in 4.82 seconds: pending uploads with stop/start and changed-operator rejection,
plus one combined restore retaining upload obligations, uncertain funding, pending
gateway sync and interrupted read occupancy. All restored mutations remain fenced.
Formatting and diff checks pass. No full CI/release gate ran.

Next, connect explicit host lifecycle/endpoint ownership to this shared assembly,
preserving inspection-only restoration. Production consumer authentication and
persistence, independent provider completion and recovery qualification remain open.

## Signed consumer admission and gateway coordination — included in 0.2.17

Browser admission now uses signed SDK ingress to the existing consumer probe.
That canister retains the full asset/permission intent and calls the service under
its actual tenant identity. The browser then signs preparation directly as the
admitted uploader. Unrelated identities cannot admit or prepare, and the uploader
cannot bypass the consumer by admitting itself. The harness verifies the exact
persisted consumer intent, acknowledged permission and bounded manifest reply
before allowing certificate issuance. After the gateway outcome, the browser signs
a consumer registration attempt; service exposure remains unconfirmed and the asset
stays unpublished even on HTTP success. Cancelled cases explicitly persist the
consumer tombstone and then withdraw through its tenant client. The exact saved
intent, acknowledged withdrawal, browser history and charged reservation survive.
Local browser cancellation alone still does not withdraw tenant permission.

The read-only Toko development refresh still resolves to
`6519b72d2a420564dabaf700fc55f7b8603d9fd3`. Its project boundary verifies delegated
subjects and project roles: asset registration requires Maintainer; certificate
issuance separately checks AssetsManage. These remain application-owned policy,
not a reason to add Toko/Canic authentication dependencies to the service. See the
[pinned review](../evidence/toko-0.2-review.json). No sibling changes occurred.

Rust supplies opaque Candid encoded from maintained DTOs and validates replies;
the private JavaScript fixture adds no service schema or consumer API. SDK update
calls verify IC certificates under the owned emulator key. `make test-browser`
now also builds the existing consumer probe. There is no new production identity
provider, browser preparation journal or Toko endpoint; the fixture's configured
operator authorization is not Toko membership policy.

`clients/browser/gateway.js` wraps Caffeine's existing per-client fetch hook. It
snapshots opaque requests, commits bounded destination/header/body fingerprints
before dispatch and records bounded HTTP responses before returning them upstream.
The caller-owned store serializes gateway claims with certificate intent and
cancellation. An execution token fences competing tabs and recreated hooks;
lost/uncertain/failed requests cannot be replayed or continued automatically.
This adds no provider parser, chunk uploader, hashing algorithm or SDK patch.

The private IndexedDB fixture retains gateway history through cancellation,
certificate recovery, tab closure and reload. Tests inspect committed intent at
the actual fetch boundary and compare fingerprints to received HTTP bytes. New
scenarios cover aborted claim/observation writes, lost and oversized replies, and
late cancellation; local HTTP observations never confirm service completion.
All ten Chromium/PocketIC scenarios pass in 45.53 seconds with signed consumer
admission, uploader preparation and consumer follow-through, including cancellation
between inspection and claim, request-budget exhaustion and attempts to continue
after HTTP failure/uncertainty. Strict storage-test Clippy, release storage/consumer Wasm,
formatting, JavaScript syntax and diff checks pass. No full CI/release gate ran.
Common signed authorization refusal probes run once in the successful journey;
transport fault cases retain their distinct recovery/accounting assertions.

Next, integrate the production consumer's authentication and qualified persistence,
then authoritative provider reconciliation/completion. Do not turn the fixture
store into a production journal or infer gateway retry authority from recovered
certificates, HTTP status or elapsed time. No deployed provider traffic, release,
publication or sibling edits ran here.

## Caffeine package composition — included in 0.2.16

The unmodified published Caffeine 1.1.2 client passed both local browser scenarios
using a real guarded HttpAgent (4.49 seconds). The existing constructor hook is
sufficient: no additional certificate callback or copied provider client is needed.
`clients/browser` only handles exact intent and IC observation; Caffeine owns its
hashing, tree/chunks, certificate extraction and HTTP formats. The browser graph
now uses SDK 5.4.0 within Caffeine's supported range, with no forced SDK 6 override.

The [small pinned patch](../../clients/browser/patches/README.md) adds static
network-free preparation before admission, single-use immutable preparation handles,
and per-client fetch/abort/retry/concurrency controls. Caller byte mutation cannot
change the prepared data. The build checks original package hashes and applies the
patch to a generated copy, preserving installed dependencies and upstream algorithms.
The fixture selects one attempt and one worker and confines HTTP to its owned local
gateway substitute. Service completion remains unconfirmed after browser success.

The browser's actual prepared JSON now feeds `ops::caffeine::preparation`, a bounded
conversion into the existing service declaration. It reuses metadata validation
and root reconstruction, ignores redundant nested tree nodes and adds no hashing
or provider transport. The local driver waits for real tenant admission and
uploader preparation in PocketIC before receiving the exact permission. A tenant
calling preparation as itself is refused. Only metadata/leaves cross this boundary;
file bytes remain in the browser. This is fixture coordination, not a Toko integration.

Four patched Chromium/PocketIC scenarios pass in 13.25 seconds. They cover success,
lost reply/cancellation, failed tree request with no retry, and abort after the tree
with no chunk dispatch. Static preparation, byte snapshots, copied/consumed handle
refusal and pre-issuance abort are included, alongside the earlier intent/crypto
checks. Release storage Wasm, strict storage-test Clippy, formatting, JavaScript
syntax, reproducible patch build, package/patch hash checks and diff checks pass.
Four decoder tests additionally cover accepted upstream metadata and malformed,
oversized or inconsistent declarations. Core library/test Clippy passes as well.
The npm graph confirms a single shared SDK 5.4.0. No full CI ran.

The gateway follow-up above now retains effect intent around this handoff.
Do not infer that a recovered certificate
or freshly prepared handle permits a retry. Production authentication/storage,
provider completion/economics and deployed qualification remain open. No provider
traffic, upstream contact, deployment or package publication occurred here.

## Browser reuse assessment — direction adopted in 0.2.16

The maintainer questioned duplicated browser work and explicitly requested a reuse
assessment. The [assessment](../provider-review.md#browser-reuse-assessment--2026-09-28)
rechecks official main and npm 1.1.2, verifies the downloaded archive's SHA-512 and
inspects published declarations/source. Caffeine already owns hashing, metadata,
chunking, certificate retrieval, gateway requests, retries, progress and direct URLs.
No new general-purpose browser uploader or production IndexedDB store should be
built as the next task. Existing unreleased client work remains intact pending
demonstrated composition; the direction below supersedes its earlier expansion plan.

Reuse the upstream package with the smallest necessary preparation/transport hooks.
Its injected HttpAgent is an existing certificate integration point; first prove
whether that suffices. Preparation is private, however, and gateway fetch/retry/
cancellation are not configurable. Our manifest admission must precede issuance,
so a plain `putFile()` wrapper is not yet sufficient. Published SDK dependency
`^5.3.0` also differs from our 6.1.0 fixture; do not silently override it.

Keep tenant/quota/reference/accounting policy and required Rust validation here.
Keep exact-operation persistence/recovery/cancellation only as a narrow integration
layer. Next, test the actual published package locally, then add only demonstrated
missing hooks, preferably upstream-supported or as a small reviewed pinned patch.
No upstream contact, implementation replacement, dependency change or provider
effect ran during this source/API assessment. Full CI was not run.

## Reusable browser certificate transport — included in 0.2.16

`clients/browser` now owns certificate transport and verification as a private
source package, using the pinned IC SDK peer. Callers supply authentication,
trusted IC root and an atomic durable intent store. The client snapshots full
permission/operation bindings, persists the exact signed envelope before dispatch
and returns verified historical observations without gateway or retry authority.
Explicit cancellation and recovery preserve uncertain history. The documented
store contract requires cross-tab atomicity, bounded capacity and retained tombstones;
the client does not supply a production storage or identity allocator.

Saved intent additionally binds IC origin and trusted root key. Recovery rejects
changed permission/trust/envelope and inconsistent saved state before read-state.
The exact v4 update and SDK v3 read-state routes are allowed; redirects, fallback
routes and duplicate dispatch are refused. The client contains no fixture identity,
fault switch or window global. Chromium now imports this implementation; only the
two-slot IndexedDB store, test identity and fault injection remain in tests/browser.
The bundler checks that the client peer, fixture pin and installed SDK agree.

Both Chromium/PocketIC scenarios pass in 4.26 seconds, covering write abort,
competing tabs, lost response/tab closure, cancellation and reload recovery, plus
the new binding/state/identity/operation refusal checks. Release storage Wasm builds;
JavaScript syntax, SDK/package-lock consistency, evidence JSON and diff checks pass.
No Rust source, canister schema, allocator, memory grant or provider effect changed.
No full CI/release gate ran; the browser target remains opt-in.

The reuse assessment above now governs the next step. Actual consumer authentication,
storage capacity/eviction/restore behavior and authoritative provider evidence remain
open. This source package is not published to npm and does not establish deployed
Caffeine acceptance or complete either service adapter.

## Browser certificate intent, cancellation and recovery — included in 0.2.15

The private `tests/browser` fixture now uses pinned IC JS SDK 6.1.0 and Chromium
against the actual local PocketIC HTTP endpoint. It retains the full original
permission as Candid bytes, full-width operation identity as text, and exact signed
envelope/request ID in IndexedDB. A strict read/write transaction claims dispatch
before fetch; two tabs cannot both send. Transaction abort leaves an unsent intent
and sends nothing. Uncertain/observed/cancelled slots cannot be dispatched again.

The browser verifies signature/delegation/time, exact request ID, method and root
before recording an observation. HTTP bodies are bounded before SDK decoding;
certificate buffers have a separate bound. Forged signatures, unrelated request
IDs and oversized proofs cannot change intent. The gateway path is absent, and
browser network access is restricted to the owned page and local IC endpoint.

Chromium tests cover simultaneous tabs, held reply/tab closure, reload recovery,
permanent local cancellation, conflicting bindings and lifetime capacity. Read-state
recovery preserves the original envelope and request ID without reissuing. A late
verified reply may update historical observation but cannot clear cancellation.
Browser cancellation is not tenant permission withdrawal or quota release.

`make test-browser` is an explicit opt-in outside the ordinary CI/release gate;
it runs no downloads and fails on missing provisioned tooling. The two-slot fixture
is not a production journal, eviction/crash/backup guarantee or Toko integration.
This batch changes no core source, canister schema, allocator or memory grant.
Browser package versions and setup are in [dependencies](../dependencies.md#browser-certificate-evidence).

The opt-in browser test passes both scenarios in 4.99 seconds. The release storage
Wasm build, strict storage-test Clippy, JavaScript syntax, package-lock/pin consistency,
evidence JSON, formatting and diff checks pass. No full CI/release gate ran.

Next, connect consumer-owned durable intent and cancellation to the production
browser integration and obtain authoritative provider evidence for live issuance.
Provider replay/namespace/pre-charge enforcement, completion/economics, both
production adapters and operational recovery remain open.

## Headless signed ingress and certificate recovery — included in 0.2.15

Two new headless cases use a real test signing identity, the official Rust IC
agent and PocketIC's HTTP v4 ingress endpoint. The raw response certificate is
verified under the explicitly owned emulator NNS key, including delegation and
timestamp. Extraction binds the saved signed envelope/request ID, actual uploader,
service, method and root before accepting the exact plain Caffeine reply. HTTP
success and possession of certificate bytes alone do not establish issuance.
Malformed/oversized bytes, a forged signature, wrong trust root, unrelated request
or changed intent fail. No browser or deployed gateway is exercised.

Saving the signed request before dispatch permits read-state recovery of its
original reply without resubmission or service mutation. A new issuance attempt
receives a certified rejection. The historical reply remains certifiable after
local revocation; it neither renews permission nor authorizes gateway dispatch or
retry. Applications must retain cancellation and uncertain-effect state separately.
This is a temporary test artifact, not a production consumer journal, crash-safe
storage guarantee, provider receipt-retention contract or operational restore.

All four targeted certificate PocketIC cases pass (12.14 seconds), including the
existing response/rollback checks. Strict storage-test Clippy and the release
storage Wasm, formatting, diff and evidence-JSON checks pass. The core Wasm
normal/build dependency tree excludes the new agent/HTTP/async/CBOR test packages.
The new agent, HTTP, async and CBOR dependencies are native dev
dependencies of the unpublished harness only; core source, Wasm allocator, stable
schema and memory grants are unchanged. No full CI/release gate ran.

The current focus above adds local Chromium/IndexedDB evidence; production consumer
intent/cancellation and authoritative provider evidence for live issuance remain.
The fixture still substitutes provider/recovery facts. Deployed certificate replay,
namespace and pre-charge enforcement, completion/economics, both production adapters
and operational recovery remain open.

## Guarded Caffeine certificate response — included in 0.2.14

`workflow::uploads::certificate` adds root-only resolution and issuance through
the shared exposure gate. The root resolves through bounded retained indexes to
the full original permission; only the actual uploader may issue. Unknown and
foreign roots are refused, and broken indexes fail closed. Independent full-width
operation/object/reference identities are preserved. No manifest tree or file
bytes are loaded. Issuance rechecks local eligibility and exact host facts before
committing possible exposure and constructing the plain `upload`/`blob_hash` reply.

The host must return that record from the same synchronous ingress update, with
no await or Result wrapper, and propagate traps so the state and reply commit
together. The IC supplies the response certificate; the library signs no bytes
and exports no endpoint. The storage fixture implements the reviewed method,
defaults to refusal and uses operator-configured evidence substitutes for tests.
Query execution, foreign callers, stale/foreign facts and repeated issuance cannot
produce a successful reply. A lost committed reply preserves inspectable exposure;
revocation and same-release upgrade retain the obligation and cannot renew issuance.

The [source refresh](../evidence/caffeine-contract-refresh.json) reconfirms official
main `ee8e3dda39b105f95133256144172a4506e841a8`, npm 1.1.2/integrity and unchanged
Mixin/client hashes. The client obtains certificate bytes from the agent's update
response, separately from the plain canister reply. No Mops/Cashier/account/gateway
call or deployed-provider effect ran. These source observations establish neither
server qualification nor successful browser extraction/gateway acceptance.

`workflow::uploads::exposure` now supplies `inspect_preparation`, `commit` and
historical `inspect`. Full original permission and actual uploader authority bind
exposure; tenant or original uploader may inspect history. Pure policy reports
independent missing pre-charge limits, provider namespace, replay/lifetime/charging,
recovery eligibility and durable-commit facts. Evidence binds the entire permission
and current observation time. The host must establish provenance/freshness itself;
equal timestamps do not prove either. No production ingress accepts these facts.

Commit rechecks local preparation, activation, clock/deadline, phase and the permanent
restore fence before recording possible exposure. A prior preview cannot authorize
mutation after local changes. The result emits no certificate and grants no later
issuance/retry authority. Committed exposure rejects repetition; revocation retains
charged uncertainty, and exact historical inspection survives restore. No new
stable schema, memory grant, dependency, allocator or provider effect is added.
The lower-level owner method retains its bookkeeping contract.

The storage probe's private exposure route delegates to the same workflow using
labelled evidence scenarios, including consumer/read/lifecycle fixtures. All 49
targeted native upload-owner cases and 55 selected storage PocketIC cases pass;
the latter took 108.70 seconds, excluding funding and gateway transport. Three new
native and two new IC cases cover the exact plain reply, root/permission/caller
isolation, default refusal, malformed/oversized input, corrupt indexes, byte-for-byte
write/response-trap rollback and lost committed acknowledgment through restore.
The preceding exposure step additionally passed six filtered native cases (four new).
Strict affected all-target Clippy, release consumer/storage Wasms, warning-free
core/protocol/storage/consumer rustdoc, formatting and diff checks pass.
No full CI/release gate ran. Cargo, dependencies, allocator, stable schema and
memory grants are unchanged.

The current focus above adds headless extraction of the local IC response
certificate; browser integration and authoritative evidence for live issuance remain.
The gate accepts host-established facts; it does not establish them. Both production
adapters, provider completion/economics and operational recovery remain open.
Tests still substitute host facts and send no certificate to a deployed gateway.

## Uploader client, cancellation and recovery — included in 0.2.13

`ReplicatedUploadManifestClient` now binds actual executing actor, tenant and service.
Only the admitted uploader can prepare; tenant/uploader inspection is available.
Preparation checks bounded declaration/root consistency before encoding/dispatch.
Both operations require replicated execution, send once with no attached cycles
and reuse the bounded reply decoder. No automatic retry, allocator, service journal,
provider call or implicit endpoint/lifecycle is introduced.

The private application fixture can run as a separate uploader instance with two
lifetime manifest-intent slots in its existing bounded record. It saves exact
intent before dispatch and retains typed refusals. Unusable acknowledgments or
callback traps leave dispatch uncertain and block redispatch; exact inspection can
acknowledge a prepared declaration but unprepared cannot clear uncertainty. Accepted
original metadata remains retained; stale/foreign results cannot overwrite it.
Uploader cancellation now persists a permanent local tombstone. It prevents any
further preparation dispatch while retaining unsent/uncertain/accepted/refused
history and lifetime capacity. Exact saves cannot reopen it. Recovery and delayed
acknowledgments preserve cancellation. The tenant still owns permission withdrawal
and asset/reference cleanup; uploader cancellation makes no service accounting claim.
Same-release upgrade validates and fences the uploader record synchronously.
The private v1 fixture schema is replaced directly; cross-release is reinstall-only.

The preceding client step passed 29 manifest native and 49 selected storage PocketIC
cases. The cancellation extension passes all six consumer-model cases, 18 consumer
IC cases (35.56 seconds) and seven manifest IC cases (17.41 seconds). New races
hold uploader acknowledgment across both cancellations and tenant withdrawal, then
preserve unexposed cleanup or exposed late completion/reference release under
suspension. Unsent, uncertain and reconciled cancelled history survives fenced
upgrade. Earlier reply-loss, callback/start-write, authority, bounded-history and
registration regressions remain covered. Preparation alone cannot publish.
Strict affected all-target Clippy, release consumer/storage Wasms, warning-free
core/protocol/consumer rustdoc, formatting and diff checks pass.
No full CI/release gate ran; no dependency, allocator or memory grant changed.
The service stable schema is unchanged.

The current focus above adds shared guarded exposure and the local certificate
response; deployed issuance remains subject to provider qualification and host
evidence acquisition.
Production browser/headless integration, intent sizing/storage, provider qualification,
both production adapters and operational recovery remain open. Exposure/completion
continue to use explicitly labelled fixture controls.

## Manifest preparation and recovery — included in 0.2.12

`workflow::uploads::manifests` now supplies canonical `blob_prepare_upload` and
`blob_upload_manifest` with passive shared DTOs and bounded reply decoders.
Preparation authenticates the actual admitted uploader; inspection allows the
tenant or original uploader. Both bind the entire original permission, including
uploader and expiry. Raw declaration budgets precede conversion, and the existing
Caffeine root/length/metadata checks apply. Reordered equivalent retries retain
the original leaves and metadata. Header-name case remains hash-significant.

Inspection reads one immutable bounded record without rebuilding its tree. It
preserves exact historical declarations after expiry, suspension, exposure,
revocation and restore; it does not authorize replay, file publication, certificate
issuance or provider completion. Preparation enforces current activation, phase,
time and fencing. The storage probe replaces its private preparation endpoint and
write-trap route with the shared handler. The test host still drives uploader
ingress; no production uploader client or uploader outbox is claimed.

All 27 manifest, 31 admission and 22 reference filtered native cases pass, including
four new manifest boundary/reply cases (filters overlap). All 44 selected storage
PocketIC cases pass in 83.15 seconds; funding and gateway transport were excluded.
These cover actual uploader authority, unusable acknowledgments, unchanged stable
memory on equivalent retry/refusal, declaration retention through fenced upgrade,
and existing write-trap/consumer/lifecycle/read regressions. Release consumer/storage
Wasms, strict affected all-target Clippy, warning-free core/protocol/consumer
rustdoc, formatting and diff checks pass. No full CI/release gate ran.
No service stable schema, memory grant, dependency or allocator changed.

The current focus above adds replicated uploader dispatch and saved-intent recovery.
Browser delivery, provider qualification and production integration remain open.

## Consumer permission withdrawal — included in 0.2.12

`workflow::uploads::admission::revoke` and the permission client's `revoke` method
now expose canonical `blob_revoke_upload`. They bind the full original permission,
including uploader and expiry, before mutation. The bounded response requires
positive revocation evidence and distinguishes first withdrawal from exact replay.
The storage probe's private revoke endpoint and write-trap route now use this
shared handler. There is no new provider call, identity allocator or service journal.

Consumer cancellation retains the original permission and tombstone. Explicit
withdrawal persists dispatch intent before its actual IC call; admission uncertainty
must first be reconciled. Typed refusals remain recorded, and uncertain withdrawal
blocks redispatch until exact inspection proves revocation. An unrevoked observation
does not acknowledge cleanup. Same-release upgrade preserves these fields under
the consumer fence. The private fixture schema is replaced directly within v1;
cross-release transitions remain reinstall-only.

Unexposed cancellation releases reservation bytes. Exposed uncertainty stays charged;
late completion still creates a first reference requiring explicit release.
Confirmed references, provider deletion and billing cessation remain separate.
Suspension permits cleanup, while restored owners reject all mutation.

Seven targeted revocation native cases, three consumer-model cases and all 42
selected storage PocketIC cases pass (68.34 seconds for the latter). These include
the consumer, admission/write-trap, lifecycle, planning, read and gateway callback
regressions; funding and gateway transport suites were excluded. Strict affected
all-target Clippy, consumer/storage release Wasms, warning-free core/protocol/consumer
rustdoc, formatting and diff checks pass. No full CI/release gate,
dependency change, allocator change or new memory grant was required.

The current focus above adds shared manifest preparation. Exposure and provider
completion retain private test-host controls; production integration remains open.

## Authenticated admission and first-reference recovery — included in 0.2.11

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

The 0.2.11 implementation validation passed 30 targeted admission native cases, three consumer-model
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

The current section above advances revocation transport. Completion facts remain
labelled substitutes; this does not qualify Toko, Canic retirement or the service.

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
