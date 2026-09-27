# 0.2 delivery plan

0.1.19 closes the foundation phase. 0.2 targets a usable Caffeine-backed service,
with durable shared handlers, standalone and Canic adapters, an operator client
and a consumer acceptance journey. It is not complete when another set of codecs
or test fixtures passes. Canic removal and live installation retirement remain
separate, explicitly authorized work.

The proposed **0.2.0 library cut** packages the breaking direct-upload admission
contract, tenant enrollment, canonical metadata and measured resource envelope.
It is ready for the maintainer's release workflow after committing the batch;
that workflow must still pass full validation. This release does not complete
M1–M4 below. Durable handlers, provider qualification, both production adapters
and consumer acceptance remain work for the 0.2 series.

The maintainer authorized starting 0.2 and directed the consumer review to Toko's
remote `development` branch. Its current reviewed commit is
`6519b72d2a420564dabaf700fc55f7b8603d9fd3`; do not use a local Toko checkout.
The [review record](evidence/toko-0.2-review.json) pins source paths and hashes.

## Starting point

| Area | Available from 0.1 | 0.2 deliverable still required |
| --- | --- | --- |
| Content | Hashes, manifests, chunk/ordered verification | Real upload and verified read path |
| Ownership | Transient catalogs, references, quotas, reservations | Durable service handlers and tenant enrollment |
| Caffeine | Bounded codecs and scoped query encoding | Qualified transport and completion/economic evidence |
| Recovery | Actual IC interruption tests; inspection-only restored fixtures | Defined operational restore/reconciliation with surviving authority |
| Operations | Local status, refresh, sync, preview and lookup tools | Production authentication, targeting and funding workflow |
| Deployment | Independent core library | Standalone and Canic adapters using identical handlers |

## Milestones and completion evidence

| Milestone | Deliverable | Completion evidence | Current state |
| --- | --- | --- | --- |
| M1 — contract | Toko asset journey, uploader/tenant/operator bindings, resource envelope, provider and recovery decisions | Resolved decision list below, concrete acceptance inputs and authoritative provider evidence for promised guarantees | In progress: configuration, enrollment and upload-permission models implemented; consumer coordination contract specified; provider/recovery evidence still open |
| M2 — durable standalone journey | One production owner of configuration, upload/reference/deletion journals and shared handlers; standalone adapter | Actual PocketIC install, upload/admission, verification, interruption, accounting and supported recovery, including the 10 MiB media boundary | Not implemented |
| M3 — Caffeine and operator integration | Provider transport, verified completion/readback, explicit funding/reconciliation, production CLI and client | Exact source/interface provenance, client tests and bounded explicitly authorized provider trial; no inference of credit or billing cessation | Not implemented |
| M4 — managed parity and acceptance | Thin Canic adapter, same API/tenant rules and real consumer composition | Same journey/operator cases through both deployments; all BLOB-01–18 replacement obligations resolved; removal readiness handoff | Not implemented |

M2 depends on M1's applicable decisions, not just a milestone label. M2 may use
explicitly labeled local provider substitutes for IC failure cuts; M3 must supply
the actual provider evidence. A release is not proof that a milestone passed.
The original B1/B2/B3 extraction gates are project gates, not crate version numbers.
Starting 0.2 does not waive the [service contract](service-contract.md).

## Consumer findings that constrain the design

- **Browser upload:** Toko calls the project canister for a certificate, then sends
  tree/chunk HTTP requests directly to Caffeine. A new service owner changes both
  certificate issuer and download owner. The service must not treat the browser
  caller as the tenant project merely because it supplies the project's ID.
- **Separate limits:** the media-library constant is 10 MiB per remote file; the
  main upload session stages up to 500 MiB in aggregate. Neither is the original
  6 MiB/six-chunk fixture envelope. The 0.2 fixture now exercises 10 MiB.
  The 500 MiB bound is explicitly a UI staging
  guard, not a provider limit. The media upload and main upload paths differ;
  do not treat one path's bound as a universal file limit.
- **Two commits:** browser upload precedes project `create_remote_assets`.
  A service reservation and retained operation must own a completed or uncertain
  upload even when project asset registration fails. Retrying asset registration
  must not silently upload/pay again. Thumbnails stay in Toko's database.
- **References:** Toko permits catalog rows without a root-uniqueness check in the
  reviewed create path. Multiple assets can require distinct references to the
  same content. Token/generator usage is a consumer concern; service release must
  be driven by exact references and a durable consumer release outbox. Rechecking
  across awaits is not distributed exclusion.
- **Public serving:** current URLs include root, owner and project. Provider
  serving is not a confidentiality guarantee or independent client verification.
  Existing generic MIME handling needs explicit active-content/serving policy.
- **Economics:** the hub separates passive status from an explicit maintain path
  that may sync and fund. Preserve this distinction. Its retry behavior, default
  project/bucket and billing amounts are observations, not approved service defaults.

## Decisions to finish M1

| Decision | Working direction | Still required before dependent implementation |
| --- | --- | --- |
| Consumer/installation | Toko project assets as the reference journey; new isolated storage owner; existing installations remain separate | Confirm concrete project/operator and provisioned namespace for a live trial; no principal/default is selected by source review |
| Tenant/uploader | Project canister is the tenant; operator enrolls it; browser identity is a distinct uploader; local permission/suspension rules implemented below | Authenticated endpoints and durable enrollment; implement consumer coordination contract; provider certificate replay/namespace guarantees |
| References | Distinct asset references; no cross-tenant sharing by default | Resolve repeated-content reuse and delete/re-upload against provider callback identity; keep existing conservative root claims until safe replacement is proven |
| Size and resource envelope | Explicit per-file, tenant/global retained-leaf, concurrent, lifetime and liability limits | Size production metadata/read-session/history totals and transient decoding work; measure instruction and total memory costs at configured capacities |
| Payer/funding | Explicit payer; self account proposed for the first isolated trial | Verify any linked-payer authority separately; exact payment reconciliation and credit evidence after lost replies |
| Upload and deletion evidence | Intent before certificate exposure; logical release, physical deletion and billing stop remain separate | Authoritative completion lookup, retry charging, retention and object-specific final billing evidence |
| Recovery | Synchronous fencing on restoration; no reuse from an old local counter | Specify surviving identity/accounting authority, supported backup boundary and a proven operational recovery path; permanently fenced fixtures are insufficient |
| Deployment ownership | Both adapters and client live here; Canic supplies generic lifecycle/discovery | Final package/protocol boundaries and production tenant/operator authentication; Canic changes require separate authority |

Each unresolved provider question must end in evidence, an explicitly reviewed
narrower supported contract, or a reported blocker. Do not substitute another local
fixture for a missing server guarantee. No paid trial or outbound provider message
is authorized by this plan.

## First implementation and next action

`model::service::configuration` now validates candidate service/operator/payer
bindings and cross-resource consistency using the existing catalog, upload and
billing models. It rejects Wasm-incompatible counts, impossible object budgets,
inconsistent tenant/global bounds and insufficient minimum reference receipts.
It also derives the manifest leaf budget from the admitted object size and bounds
metadata entries/bytes, rejecting lengths outside SHA-256 or portable chunk counts.
The upload owner uses the derived limits before allocating manifest state;
metadata consistency alone does not prove content verification or provider storage. Empty objects are
excluded at service admission as well as in the existing content verifier.
This is construction-time validation only: it does not install state, select
provider namespaces, enroll tenants, enforce runtime upload sizes or grant effects.
It deliberately freezes no persisted schema or endpoint API.

Manifest retention has explicit global and per-tenant lifetime chunk budgets.
Admission reserves `ceil(content_bytes / 1 MiB)` slots before catalog mutation,
even before a manifest arrives. A prepared operation retains one leaf array;
configuration rejects a payload envelope beyond the Wasm32 address range and
requires the largest object to fit one tenant. Exact retries need no new slots.
Cancellation, deletion and billing settlement do not remove the
permission history or refund these slots. The owner derives usage from its exact
retained permissions, avoiding a second mutable accounting source. This bounds
retained leaf payloads, not total heap, transient decoding, allocator overhead or
instructions. The admission probe now measures the direct-upload manifest path and bounds
decoder bytes/work, type headers and skipped values; production metadata/history
and read-session capacities still need measurement. See the
[resource evidence](evidence/core-primitives.md#local-admission-resource-measurements).

The [upload-path evaluation](#upload-path-evaluation--2026-09-27) recommends direct
browser-to-Caffeine upload as the production target, with bounded manifest
authorization and independently established provider completion. The maintainer approved this hard cut; the model now accepts manifests without
file chunks. Live certificate issuance still requires the provider guarantees.

The connected PocketIC fixture now exercises a full 10 MiB object through ten
separate 1 MiB chunk messages, exact chunk retries, certificate gating and separate
logical/physical/billing release. A 10 MiB-plus-one-byte vector rejects before
reservation; retained ten-leaf manifests also survive fenced fixture restoration.
Manifest and journal validation share one fixture envelope. These checks close
the earlier small-file evidence gap, not production handler, Caffeine completion,
operational restore or throughput qualification. Read-session/global instruction
budgets and production metadata/history capacity still need concrete sizing.

### Upload-path evaluation — 2026-09-27

**Recommendation:** keep bulk file bytes on the browser-to-Caffeine path. The
service should own tenant/uploader authorization, reservations, certificate
issuance, references and economics. Validate a bounded manifest before issuance;
do not require a second complete upload to the service as the default production
contract. The maintainer approved this design and the local model now implements it.
This does not qualify the gateway or authorize live certificate issuance.

The refreshed official source remains
`e5cacdfe5ce55e939edb02980fca800c0c13f421`, npm latest remains 1.1.2 with unchanged
integrity, and Toko development remains
`6519b72d2a420564dabaf700fc55f7b8603d9fd3`. Both reviewed browser clients send
tree/chunks directly to Caffeine after obtaining a canister certificate. Toko's
reviewed media registration submits a provider root and claimed size; it does
not request an independently service-verified whole-file raw digest. These
findings cover the inspected paths, not every future consumer. Sources and the
length experiment are in the [review record](evidence/toko-0.2-review.json).

| Candidate | Work and useful guarantee | Assessment |
| --- | --- | --- |
| Full bytes through service, then gateway | One leaf hash plus a streaming raw hash per fresh chunk; establishes the bytes/length seen locally | Removed from service admission. Does not establish provider storage, billing or retry safety |
| Leaf-only verification through service | Removes the raw-hash pass but still sends every byte through IC ingress | Possible narrower guarantee, but preserves duplicate transfer; no consumer requirement found to justify this as the default |
| Bounded manifest at service; bytes direct to gateway | Hashes metadata and the small leaf tree; service authorizes exact identity and reserves capacity | Selected model, subject to provider enforcement and completion evidence below |
| Off-chain verifier, canister HTTP readback or spot checks | Adds another trust/operations boundary, paid reads or partial evidence | Not selected; partial reads cannot establish full content, and full readback restores bulk work |

Before the hard cut, the optimized local 10 MiB run spent 1,713,702,451 instructions through the ten
fresh appends and another 903,016,377 for one retry of every chunk. Manifest
preparation ends at 1,142,257 instructions. The direct-upload probe now uses 4,107,938 instructions across admission,
preparation, an exact preparation retry and local exposure, with 1,766 encoded
request bytes total. Allocated Wasm memory stays at 1,245,184 bytes. It eliminates
the extra 10 MiB of canister ingress. This measures local model composition;
provider transport, durable storage, diagnostic writes and reply encoding are
excluded. Browser hashing and Caffeine upload/storage/download charges remain.

**Length is the deciding quota issue.** A manifest commits to hashes and declared
metadata; hashing a `Content-Length` value does not prove it describes the bytes.
An isolated run of the pinned official hashing classes built a root for a 1 MiB
zero-filled chunk with `Content-Length: 1`. No gateway call ran, so its acceptance
is unknown. The maintained Rust manifest test separately proves a consistent
root alone does not authenticate length. A final chunk or random sample cannot
prove all other leaves have the required size.

There are two viable quota contracts to qualify:

- **Exact bytes:** the provider enforces agreement among the authorized root,
  canonical length metadata, `num_blob_bytes`, tree shape and actual stored
  chunks. The shared owner now validates canonical metadata against its reservation
  and rejects duplicate/case/whitespace aliases and conflicting lengths. Production
  integration must still obtain authoritative completion/size evidence. Checking size only after a charge is
  too late to prevent a quota overrun.
- **Conservative bytes:** reserve `leaf_count × 1 MiB` as an upper bound, reducing
  it only on authoritative evidence. This needs verified provider enforcement
  of tree membership and the per-leaf byte maximum, plus bounded metadata and
  any additional billed overhead. It can overreserve a small file by almost
  1 MiB and must cover partial uploads. The client's 1 MiB splitter alone is not
  evidence of a server maximum. This option is not safe under current evidence.

Neither option bounds repeated charges or namespace reuse by itself. The
certificate response visibly contains only `method` and `blob_hash`; owner,
project, bucket and `num_blob_bytes` are also supplied in gateway requests.
Their server enforcement and certificate replay rules remain unresolved.
Local issuance deadlines cannot recall an escaped certificate.

The recommended journey is: authenticated project admission and durable
reservation; bounded manifest/root/metadata validation; exact uploader issuance
with durable exposure intent; direct gateway upload; independently obtained
completion for the bound owner/namespace/root/size; then retained reference and
idempotent project asset registration. A browser success report only requests
reconciliation. The official client discards the chunk completion flag, and
Toko's existing-chunk optimization is not a durable completion or billing receipt.
Unknown outcomes remain reserved and cannot authorize a fresh paid operation.
Exact manifest/admission retries preserve local state; this does not qualify
gateway HTTP retries.

Before live issuance, obtain authoritative answers for (1) pre-charge length,
tree and chunk enforcement, (2) owner/project/bucket binding and certificate
replay/charging, and (3) completion/size lookup or independently verifiable
receipts that survive lost replies. The receipt must identify the exact retained
operation through its immutable binding, not browser-supplied success text.
Deletion settlement and recovery inventory remain the existing service gates.

If those guarantees are unavailable, report a provider-contract blocker or seek
an explicitly narrower reviewed service contract. Full preverification is not a
fallback proof of provider completion. The local mandatory append workflow and raw-digest claim have been removed,
with no dual upload mode or compatibility shim. Hash/read-verifier primitives
remain for integrity consumers. The model distinguishes manifest binding,
possible exposure and confirmed completion; none of the first two claims storage.

### Recovery boundary and evidence still needed

Keep three cases separate: stop/start preserves the current owner; state-preserving
same-release upgrade needs atomic durable journals; loading an older backup needs
complete reconciliation from authority outside that backup. A sequence watermark
alone cannot reconstruct missing roots, references, uncertain payments or liabilities.
The production target remains operational same-release recovery; permanently fenced
inspection is only a safe fallback and does not complete M2.

The IC specifies that canister version can advance for ordinary execution and
management operations, and may advance arbitrarily. It is therefore not a
restore-only flag, an exact-next-version handshake or proof of complete accounting.
[System API specification](https://docs.internetcomputer.org/references/ic-interface-spec/canister-interface/#canister-version).
Snapshots include Wasm code and memory as well as stable state; loading one is a
distinct management operation. A post-upgrade hook alone cannot cover that path.
[Management specification](https://docs.internetcomputer.org/references/ic-interface-spec/management-canister/#ic-load_canister_snapshot).

Before implementing operational restoration, choose the surviving source of the
complete obligation inventory and how its freshness is enforced at admission,
callbacks, timers and provider liveness/deletion responses. A restored instance
must not report a missing root as deletable using stale state. No local counter,
elapsed timeout or operator assertion may release the fence. The reviewed sources
do not yet supply that surviving journal or the provider reconciliation contract;
these remain explicit blockers to the dependent production schema and transport.

### Project-to-uploader admission

The current official Mixin's certificate update accepts a root and returns
`{ method = "upload"; blob_hash }`. Its reference implementation does not check
the caller in that method. The service must supply its own authorization; the
response does not echo our tenant, uploader, operation ID, namespace or deadline.
The [review record](evidence/toko-0.2-review.json) pins the refreshed source.

The implemented local rules in `model::service::upload` are:

1. An independently enrolled project authorizes its user and submits the exact
   upload request, uploader principal and exclusive issuance deadline. The service
   admits only the actual project caller and its configured service/namespace,
   checks the object limit and reserves the shared catalog capacity. Permission
   and reservation must eventually be persisted in the same transaction.
2. The root-only certificate request resolves the original retained operation.
   Only its exact uploader may move an unexposed reservation to possible exposure,
   before the deadline and after admission time. First, that uploader prepares a
   manifest under the configured limits. The owner checks bounded leaves and
   metadata against the root and declared length, without accepting file bytes.
   Exact retries retain the declaration. Each call rechecks uploader, activation,
   deadline and unexposed phase. Before real issuance, the host must establish
   provider size enforcement and namespace/replay guarantees, recovery eligibility
   and durable commit. Local manifest consistency is not provider evidence.
3. An exact admission retry returns retained state. Changing uploader, deadline,
   root, declared length or object/reference binding conflicts. A second exposure
   is denied: losing the original ingress/certificate reply is not permission
   to mint a new certificate or retry an uncertain paid upload.
4. The project or uploader may inspect the exact permission and current phase.
   Lookup reports manifest binding separately from exposure/completion.
   Lookup neither renews a deadline nor dispatches work. Project revocation can
   cancel an unexposed reservation but keeps its lifetime identity/root claim.
   After exposure, revocation and expiry cannot release uncertain capacity or
   recall a gateway certificate. Independently proven late completion still applies.
5. Completion retains the initial asset reference even if Toko registration
   fails. Retrying registration must recover that operation rather than upload
   again. Explicit reference release, physical deletion and billing cessation
   still use the shared catalog; replayed completion cannot resurrect release.

Native tests and a thin local PocketIC admission probe now exercise this same
owner, including the independent 10 MiB vector. The probe supplies actual IC
caller/service/time and verifies role separation, suspension/expiry, exact retries
and retained uncertainty across messages. Stop/start preserves its owner;
both upgrade hooks reject so skipping the outgoing hook cannot reset its history.
It provides no durable restore or provider evidence. The earlier PocketIC journey
remains a separate fixture composition. Permissions
have one bounded lifetime slot per catalog operation. A prepared operation retains
one configured leaf array and no streaming file-hash state;
root resolution currently scans that bounded history. No production persistence,
certificate bytes or substitute provider endpoints were added. Production host
authentication, consumer outbox atomicity, instruction budgets and recovery remain open.
The provider's accepted certificate lifetime/replay and project/bucket enforcement
are unknown; a local deadline cannot establish those guarantees.

### Tenant enrollment and suspension

The shared admission owner now enforces operator-managed enrollment. Each tenant
consumes a configured lifetime slot, including while suspended; no removal/reset
path can erase obligations. Operator updates compare the exact observed enrollment
before changing it. A lost reply requires inspection; a stale instruction conflicts
instead of overwriting a newer suspension or activation.

Fresh upload admissions and reference retains require an active tenant. Exposure
also requires the activation generation captured at admission. Reactivation advances
that generation, leaving every older uploader permission unusable. Suspension works
even at generation exhaustion; reactivation then rejects without wrapping. These
local generations do not establish recovery safety after restoring an old snapshot.

Suspension leaves reservations, references and physical/billing obligations intact.
Exact receipt/admission recovery, project revocation, reference release and independently
authenticated provider reconciliation remain available. Fresh reference operations
check the actual service/project/namespace before touching the catalog; the operator
cannot act as a tenant through this boundary. Enrollment observations are restricted
to that tenant or the operator. Production adapters must supply real IC context and
persist enrollment changes together with the applicable shared state.

### Consumer registration and release coordination

This is the required integration contract, **not implemented Toko behavior**. Its
existing registration/liveness checks do not establish these distributed guarantees.
No sibling source is changed. The consumer owns its asset transaction and outbox;
this service owns exact reference receipts and provider obligations.

| Consumer action | Required durable boundary | Retry or interruption behavior |
| --- | --- | --- |
| Prepare upload/asset | Reserve bounded operation/outbox history and bind a stable asset ID to the exact upload, reference and full asset payload before requesting service admission | Same operation and arguments recover progress; changed payload conflicts. Browser progress is not completion evidence |
| Register uploaded asset | Confirm the exact service reference, then atomically record the asset and registration result under its operation ID, checking the consumer operation is still eligible at commit | Lost reply returns the original asset/result. Failed registration leaves the service reference owned; retry registration without re-uploading |
| Add another asset for existing content | Allocate a distinct reference and persist its exact retain request before dispatch; publish the asset only after accepted retain evidence | Recover the exact receipt. If registration is abandoned, retain the pending operation until its resulting reference is explicitly released |
| Remove or abandon asset | Atomically prevent new consumer uses/registration and enqueue the exact release request after checking current token/generator dependencies | Keep the tombstone and outbox through lost replies. Retry the same service/root/object/reference/request ID, never a fresh identity |
| Acknowledge release | Persist the matching successful inner lifecycle result and consumer finalization together | An outer successful call containing a recorded lifecycle error is not a successful release; retain a repairable pending state. Physical deletion and billing cessation remain separate |

No IC transaction spans the project and storage service. Every consumer path that
can attach a token/generator or publish a delayed registration must respect the
same local tombstone/pending-operation check, without an await between its final
check and write. Rechecking liveness around a service call alone is insufficient.
If cancellation races an uncertain upload or retain, preserve the operation until
its exact outcome is reconciled; cancellation cannot fabricate a release receipt.

Reserve outbox space before accepting work and retain bounded completion history
for the supported retry contract. Exhaustion rejects new work while leaving cleanup
capacity available. Do not evict records by age or reuse asset/reference/request IDs
to recover capacity. Consumer and service restore fences must prevent an older
snapshot from replaying registration or release as new work. Generation counters
from that same snapshot cannot supply the independent recovery authority.

The shared owner's local IC composition now has evidence. Next, resolve
provider certificate replay/namespace and completion evidence, the
supported recovery authority and remaining resource envelope before wiring durable
records or transport. The consumer contract needs actual cross-canister interruption
and registration-versus-delete evidence when shared handlers/adapters exist.
Propagate each implemented milestone through both adapters and operator behavior.

The existing [acceptance cases A01–A12](acceptance-plan.md) and
[Canic parity inventory](canic-parity.md) remain the completion checklist.
Generic providers, cross-tenant deduplication, cross-release migration, new
confidentiality guarantees and Canic source removal are outside this implementation
batch. Do not change Cargo versions or run release/deployment commands merely
because this roadmap names 0.2.
