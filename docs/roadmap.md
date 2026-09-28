# 0.2 delivery plan

0.1.19 closes the foundation phase. 0.2 targets a usable Caffeine-backed service,
with durable shared handlers, standalone and Canic adapters, an operator client
and a consumer acceptance journey. It is not complete when another set of codecs
or test fixtures passes. Canic removal and live installation retirement remain
separate, explicitly authorized work.

The released **0.2.1 library** includes the 0.2 direct-upload admission contract,
tenant enrollment, canonical metadata, indexed accounting and content/reference
capacity inspection, with local resource evidence.
The maintainer confirmed its release. This release does not complete
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
| M2 — durable standalone journey | One production owner of configuration, upload/reference/deletion journals and shared handlers; standalone adapter | Actual PocketIC install, upload/admission, verification, interruption, accounting and supported recovery, including the 10 MiB media boundary | In progress: durable upload/reference/settlement and verified read-session handlers with local IC evidence; complete provider-call journals, standalone adapter and operational recovery outstanding |
| M3 — Caffeine and operator integration | Provider transport, verified completion/readback, explicit funding/reconciliation, production CLI and client | Exact source/interface provenance, client tests and bounded explicitly authorized provider trial; no inference of credit or billing cessation | In progress: shared transports with local IC evidence; deployed qualification and production clients remain open |
| M4 — managed parity and acceptance | Thin Canic adapter, same API/tenant rules and real consumer composition | Same journey/operator cases through both deployments; all BLOB-01–18 replacement obligations resolved; removal readiness handoff | Not implemented |

M2 depends on M1's applicable decisions, not just a milestone label. M2 may use
explicitly labeled local provider substitutes for IC failure cuts; M3 must supply
the actual provider evidence. A release is not proof that a milestone passed.

The maintainer directed the next implementation toward canister service storage;
the local filesystem journal remains optional test/operator tooling. Shared stable
components now form one upload/lifecycle owner through host-granted `ic-memory`,
reusing existing model checks. Enrollment, root claims, exact permissions,
manifests, confirmed objects, individual references/receipts and charged totals
commit in synchronous IC updates. Per-record writes avoid serializing entire
histories. PocketIC covers partial-write rollback through settlement, uncertain
bytes and same-release upgrade into an inspection-only fence. Indexed discovery,
exact-reference descriptors and bounded tenant/operator cleanup traversal now use
this owner too. Admission/reference capacity and bounded operator root observations
now use maintained counters/indexes, including during fenced inspection. A separate
durable funding journal now reserves exact local attachments and retains transport
outcomes with incremental accounting and IC rollback/upgrade evidence.
Retained method/account/target choices now bind canonical Cashier top-up
requests, with exact-identity and transport-context checks on outcomes. Bounded
operator history now recovers exact requests without saved client input, newest
first, including under the restore fence; pagination grants no retry authority.
An explicit shared IC transport now captures exact unbounded-call refunds and
bounded replies separately, with local journal/callback rollback and liquidity
evidence against a Cashier substitute. Production provider/account qualification,
complete account activity and spendability gates remain before payment admission.
Structured Cashier outcomes now persist with their transport accounting; exact
operator lookup keeps reported balances separate from conservative reconciliation.
Scoped summaries now expose maintained local funding totals without history scans;
later refunds cannot clear older uncredited acceptance or the restore fence.
Shared funding preparation now combines these local constraints with separately
scoped host observations, retaining every missing-evidence blocker. Updates
re-read current state before synchronous reservation; previews grant no authority.
Shared first-attempt admission now binds host observations to the exact retained
request, recognising its own reservation without hiding older acceptance or
unknown external activity. It marks once; uncertain/terminal attempts cannot retry.
Shared guarded dispatch now composes marking, actual post-write liquidity, one
canonical call and durable callback settlement, with local IC fault evidence.
Production host evidence acquisition and provider qualification remain open.
The durable gateway registry now retains membership and pending sync together,
reusing the existing revocation rules, with bounded records and fenced restoration.
Its shared begin/complete/cancel workflow retains the canonical Cashier request and
applies the bounded reply decoder transactionally, with IC rejection/rollback evidence.
Shared async query orchestration now releases the store borrow across host transport
and rejects delayed replies after revocation or replacement, with local IC evidence.
Explicit replicated transport now exercises the canonical query-only method with
service/Cashier checks and local IC rollback evidence. Deployed Cashier replicated
execution and provider trust remain unqualified.
Shared scoped gateway root observations now join current durable membership,
matching owner configuration and indexed object bindings. Both restore fences
block operational reads; local IC evidence covers revocation and every cleanup
phase without converting unknown/pending roots into deletion permission.
Shared read-authority capture/recheck now binds the original tenant, exact live
reference, root/object and selected gateway. A separately persisted invalidation
counter prevents remove/re-add and same-list sync from reviving old reads; tenant
reactivation also invalidates them. Held-reply IC evidence covers these checks,
reference release, rollback and restoration. Bounded durable session admission
and exact one-shot completion now retain global/per-tenant slot and reply-buffer
budgets, with IC write-rollback and fenced occupancy evidence. The shared async
read workflow now binds returned source/root/index, bounds decoded bytes and checks
the exact admitted manifest leaf without rebuilding the tree. A local IC source
supplies actual chunks for malformed-reply, delayed authority and rollback tests.
Qualified provider transport, resource sizing and operational session recovery
remain outstanding. This optional canister read path does not change the direct
Caffeine-to-client download direction or require bulk service readback on upload.
Deployed transport qualification and effect callbacks remain separate.
Next, establish complete account activity and credit evidence, then complete other provider
intents, read sessions and operational lifecycle. Provider callback semantics and
deployed-provider evidence remain separate qualification work.
The reusable browser certificate source client now accepts consumer-owned identity,
IC trust and atomic intent storage, with Chromium/PocketIC evidence through that
same implementation. This does not complete M3: actual consumer authentication,
production storage/recovery, gateway upload and deployed qualification remain.
The maintainer's [reuse assessment](provider-review.md#browser-reuse-assessment--2026-09-28)
directs further browser work toward Caffeine's published client, using minimal
preparation/transport extensions and narrow local policy. Do not implement another
browser hash tree, chunk uploader or general-purpose journal by default.
Local composition now uses that actual package through its existing agent hook,
with a small pinned preparation/transport patch and supported SDK 5.4.0. Real IC
certificates and local substitute gateway HTTP requests exercise the path; actual
consumer admission and qualified production persistence remain next. The gateway
fetch guard now records bounded request intent and HTTP observations, fencing
recreated hooks after a claimed transfer. Its local IndexedDB evidence does not
establish production storage durability or provider reconciliation.
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

### Toko Miner feedback — 2026-09-27

The maintainer requested review of `../toko-miner/docs/upstream/ic-blob-storage.md`.
It proposes release-published static media, a separate consumer journey from
Toko's browser uploads. It does not approve Miner adoption or replace the Toko
reference journey. Its recorded review used Miner `333fdc68fb5559d1ba7ba3af8a1aa01b3552319c`;
our read-only follow-up found HEAD `9c896f0adfff002ae915b1dd1a27b7206a7d6965`
and the feedback file clean, SHA-256
`9ab5d9708fc9ed5d74664b265756691496d0b0e38b6d16ce6bf5fbeec7da7f74`.
Spot checks of `frontend/src/outpost/scenery-sources.ts`,
`frontend/src/catalog/artwork.ts`, `frontend/public/.ic-assets.json5`,
`scripts/release/bundle.py` and `scripts/deploy/staging.sh` confirm bundled media
URLs, restricted browser origins and frozen release artefact verification.
No consumer build or provider experiment ran.

| Feedback | Effect on the service plan |
| --- | --- |
| Miner BLOB-001 — managed service | Confirms M2/M4: durable shared handlers and one Canic-managed storage owner, with host-owned memory/lifecycle. Miner tenant, uploader and managed-role bindings still need an accepted application design; no per-shard media owner is implied. |
| Miner BLOB-002 — headless publisher | Make noninteractive publication an explicit M3 client requirement alongside browser upload: dry-run inventory/capacity, bounded concurrency, exact-operation lookup/resume and typed results. Reuse this repository's hash/manifest implementation and direct-to-Caffeine byte path. Explicit service/tenant/uploader/payer/namespace bindings cannot come from interactive login, inferred targets or automatic funding. |
| Miner BLOB-003 — browser delivery | Extend A10 qualification to immutable media URLs, integrity/trust, MIME, CORS, cache, errors and large reads under the consumer's CSP. A root URL alone does not establish certified HTTP or byte integrity. Define the verification path before changing consumer origins; measure cold/warm loading before performance claims. |
| Miner BLOB-004 — repeated releases | Resolve tenant-authorized reuse, exact retain/release, bounded retention and deleted-content reintroduction in M1 before freezing persisted/protocol contracts. Current immutable root claims prohibit allocating the same provider root to a new object even after settlement. A retained live reference is supported by the model; a safe delete/re-upload contract remains unresolved. Do not weaken callback safety or treat indefinite retention as free. |
| Miner BLOB-005 — release identity | README, roadmap and status identify the released library baseline and unfinished service milestones. Local wording does not close the consumer's finding or establish adoption. |

The feedback's asset survey reports 702 media files / 283,211,048 bytes, largest
8,362,256 bytes. That is its recorded checkout envelope, not a newly measured or
qualified publish list. Use it to guide multi-file capacity work; derive actual
inputs from a frozen consumed-media inventory. Toko's 500 MiB UI guard and the
256-cancelled-admission fixture are not Miner capacity requirements or evidence.
Measure retained manifests, objects, references, receipts, leaves, read sessions
and liabilities across overlapping releases, including remaining capacity and a
safe exhaustion path. The supported route for reintroducing removed media is an
adoption blocker, not an assumed capability.

The proposed release owner must retain exact operation/reference intent and
publish its complete media map only after every required object is authoritatively
confirmed and retained. Interruptions must resume without duplicate paid writes
or lost references. Old-reference release needs a bounded policy accounting for
cached/open browsers. Keeping the certified application shell and changing its
self-contained-media contract both require Miner-side acceptance before adoption.
The [consumer acceptance extension](acceptance-plan.md#release-published-media-consumer)
records the failure cuts without changing that repository.

Local progress: the shared owner now exposes tenant-authorized root discovery
with the original operation and current lifecycle. Native overlapping-release
cases cover retain/release replay, cleanup at receipt capacity and stale live
observations; the local IC probe checks discovery isolation and stop/start.
This supplies a publisher planning primitive, not a production client or an
approved delete/re-upload route. The [reference recipe](service-contract.md#lifecycle-design-under-independent-review)
keeps publication coordination and bounded retention explicit.

Headless preparation now has a shared `CaffeineManifestBuilder` and local
`prepare_upload` example. One streaming hash pass retains an explicitly bounded
leaf list; the example reuses service metadata validation and requires clean EOF
before emitting the root, raw digest, leaves and original metadata. A 10 MiB
PocketIC case admits the client-generated declaration without relaying body bytes,
then checks the retained descriptor and client verification after substituted
completion. The example's `--inventory` mode now preflights a bounded explicit
file list, checks aggregate byte/leaf work before deduplication, and emits separate
asset mappings and distinct-root totals after every source succeeds. Duplicate
content does not collapse asset identities. Optional snapshot output keeps one
copy of the exact hashed bytes per root plus the completed inventory, independently
of later source changes. The private local copies remain mutable and must be
reverified before use; they are not durable operation/recovery authority.
This is an offline publisher prerequisite: live capacity inspection, identity/operation persistence,
service/provider transport and publication remain unimplemented. The prepared
source must stay immutable until its exact bytes upload; a distinct local root
does not establish that the service can admit it or reuse retired content.

Fresh-upload capacity now has a tenant-scoped `admission_capacity` model view and
bounded private probe query. Its independent lifetime/concurrent/leaf/byte headroom
includes pending reservations and continuing billing; suspension remains visible.
It complements existing-object discovery/reference capacity. The unpublished
`blob-fixture-inventory` command now connects validated prepared inventories to
these queries on an explicit existing local probe. It preserves asset reference
demand and separates not-visible, pending, live and retired roots. Sequential
observations reserve nothing and do not prove global absence or fresh admission.
Production authentication, new-object reference sizing and persisted exact
operation identities remain outstanding; no upload or funding occurs here.

Reference recovery now has a shared `reference_receipt` read used by both passive
inspection and mutation replay. `blob-fixture-reference` can journal a bounded exact
fixture intent without replacement and query it later, including after release or
settlement. It preserves historical typed results without allocating a new request
or reference. Caller-supplied IDs still need a surviving allocation authority;
local writes now use an identity-keyed, bounded journal with an exclusive OS lock
and file/directory sync before acknowledgment. Exact retries recover the same
record; changed payloads conflict. This protects cooperating writers of one local
directory, not copies or rollbacks. It is not a registration outbox or
restored-instance fence. Upload/provider intents and actual dispatch remain open.

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
permission history or refund these slots. The same owner now maintains private
global/tenant leaf totals only after successful admission, avoiding full permission
scans. Upload reservation totals similarly change only with successful transitions;
the root-claim owner maintains one identity-index key per retained root. No public
counter mutation, state import or recovery authority is introduced. This bounds
retained leaf payloads, not total heap, transient decoding, allocator overhead or
instructions. The admission probe now measures the direct-upload manifest path and bounds
decoder bytes/work, type headers and skipped values; production metadata/history
and read-session capacities still need measurement. See the
[resource evidence](evidence/core-primitives.md#local-admission-resource-measurements).

The post-0.2.0 [history workload](evidence/core-primitives.md#retained-admission-history-after-020)
fills 256 cancelled admissions across two tenants and checks retries/cleanup at
capacity. It records per-call instructions and allocated memory, not just model arithmetic.
Removing the three scans does not give a uniform whole-call speedup in this small
workload; the indexes add 64 KiB of allocated memory at its final checkpoint.
The subsequent [accounting/index follow-up](evidence/core-primitives.md#confirmed-usage-and-indexed-exposure)
removes confirmed-usage and root-only permission scans as well. Native audits cover
all usage fields; the actual IC boundary exercises exposure at full history capacity.
The subsequent 704-object release-history workload retains 768 manifest leaves,
2,816 reference identities and 4,928 receipts for 288 MiB of declared media through
four overlapping reference generations. It exercises cleanup after history fills;
the shared owner's read-only capacity view distinguishes new-reference headroom
from reserved release receipts. See the [release-history evidence](evidence/core-primitives.md#multi-file-release-history).
These are synthetic files and local provider substitutes. Production bounds,
read sessions and operational persistence remain
unqualified; the fixture's four-generation limit is not a production default.
The [per-object follow-up](evidence/core-primitives.md#reference-history-without-lifecycle-copies)
also exercises 256 simultaneously live references and all 511 receipts, including
rejections, stop/start, replay and final cleanup at capacity. Reference mutations
now stage one private change rather than copy the object's retained history.

The [decoder follow-up](evidence/core-primitives.md#operation-specific-admission-inputs)
uses separate bounded inputs per mutation through the same handlers. Carry this
boundary design into M2 instead of a catch-all wire command enum. It lowers local
measured cost without changing model authority or choosing a different allocator.
Counters include memory-region access/write charges; individual phase costs can
shift when earlier work touches a region first. Retained-state decoder growth
remains, so production sizing must include allocation locality, reply encoding,
persistence and larger reference/read-session histories.
An isolated Talc 5.1.1 comparison gave mixed costs and higher final memory;
the maintainer confirmed keeping the default heap allocator. No allocator
dependency is retained. The [single-slot read experiment](evidence/core-primitives.md#single-slot-read-resources)
now measures a full 1 MiB leaf: bulk Candid decoding halves measured service
work to about 116M instructions, with a higher but stable allocated heap.
Hash verification alone still costs about 81M. Define the consumer download and
integrity boundary before choosing production read concurrency; this fixture
does not select a service proxy for browser delivery. Production storage locality
and complete message costs also remain open.

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
preparation ends at 1,142,257 instructions. The 0.2.0 direct-upload probe used 4,107,938 instructions across admission,
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

### Consumer download verification

The working direction is direct Caffeine-to-client delivery with verification
before application use. Keep bulk download hashing outside the service canister;
the single-slot Candid fixture remains integrity/recovery evidence, not the
production media path. This direction does not approve Toko/Miner adoption,
change their origins/CSP or qualify the deployed gateway.

The new `CaffeineRootVerifier` reuses the existing streaming tree hasher with a
root fixed at construction. It needs trusted expected length and original hash
metadata, but no leaf list or independently expected raw digest. Arbitrary body
frames fit explicit per-call bounds; only successful finalization verifies the
body. The local `verify_download` example additionally requires successful EOF,
including after the expected last byte, and fails closed on trailing bytes.
Its optional output path stages the same checked bytes in a private directory,
syncs them after verification and publishes without replacing an existing target.
This supplies local file quarantine, with a 64 KiB receive buffer and disk use
bounded by the declared length. The caller controls the destination directory;
interruption can leave staging residue or a published file without a stdout
receipt. It establishes no authenticated descriptor, portable crash-durable
transaction, resumable transfer or browser cache integration.

The publisher/application must preserve an authenticated download descriptor:
exact service/provider owner and project locator, object root, length, original
hash metadata and the consumer's serving policy. Admission now retains one copy
of the original validated header text with each prepared lifetime permission.
`content_descriptor` borrows it under the same service/namespace/tenant checks as
content discovery, carrying the current state even for cancelled or settled
history. The private IC query adapter proves caller isolation and client hash
verification; it is not certified response delivery or a publication permission.
Do not replace the stored text with HTTP response headers or infer a raw digest
from the provider-root string. A production authenticated descriptor endpoint or
certified release mapping, provider locator and serving policy still need
implementation and consumer qualification.

`workflow::reads::download::describe` now supplies the local operational descriptor:
it checks active tenant authority, exact current live reference and the restore
fence, then combines the original declaration with a trusted host-supplied
`CaffeineDownloadScope`. Its storage owner must equal the installed service;
tenant and payment account cannot substitute. The explicit provider project maps
to the local namespace without deriving either from the other. The reviewed
direct-blob target encodes root/owner/project and leaves origin selection separate.
The update-only local fixture delivers that descriptor and feeds the existing
off-canister root verifier, without fetching a body or allocating a read session.
Provider assignment, authenticated production delivery, approved HTTP policy and
consumer publication/reference coordination remain open; this is no public URL
revocation guarantee. Previously returned descriptors/bytes can remain accessible.
The canonical descriptor boundary now lives in the library, and an explicit
replicated client authenticates metadata delivery from the selected storage
canister to the actual tenant canister. Reply bounds, full reference/owner/project
checks and shared metadata validation apply before returning. Actual two-canister
PocketIC evidence replaces simulated caller identity for this route. Browser
delivery, public certified mappings and consumer publication/release coordination
remain separate work; authenticated observations are not leases across awaits.

The shared owner's `retained_content_descriptor` now checks confirmed completion,
the exact object incarnation and the consumer's live reference in one read. A
released reference gets no descriptor even when another release keeps the blob
live; replay of an old successful retain receipt does not bypass this check.
Suspension preserves inspection of existing references. This read creates no
reference or receipt. Copies can become stale, so publication still needs the
consumer transaction/outbox and release exclusion described below. The private
query adapter does not certify that observation or bind a public provider URL.

Metadata retention is bounded by existing per-object entry/byte limits and global/
tenant lifetime object counts; retries retain the first copy and cleanup does not
recycle it. These payload bounds exclude string/vector/allocator overhead and do
not prove that every candidate configuration fits total canister memory. The
[descriptor measurements](evidence/descriptor-resources.json) cover maximal headers
at a small history limit plus the existing 704-object reference workload. Production
storage and total-message costs remain separate qualifications.

For a browser, verification must precede handing bytes to an image/model decoder
or publishing an object URL. Bound total quarantined bytes and concurrent fetches,
cancel on failure/overflow, and discard uncertain partial bodies. Require clean
transport completion and the exact checked destination before claiming success.
Cached bytes and embedded GLB dependencies need the same integrity policy.
MIME safety, CORS, CSP, redirects, compressed representations, cache behavior,
range/resume support and cold/warm loading remain A10 tests. A fixed-memory hash
state does not make a browser's buffered body or decoded media fixed-memory.

Standard browser SRI checks a raw digest, not Caffeine's metadata/tree root.
An authenticated publisher-computed raw digest could support a separate browser
adapter, but no raw-digest admission requirement is reintroduced. The current
SRI specification covers script/link integration rather than an automatic
integrity attribute for every media loader; see the
[source review](provider-review.md#download-client-follow-up--2026-09-27).
The default Rust allocator and ic-memory governance remain unchanged.

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
   checks the object limit and reserves the shared catalog capacity. The durable
   pending owner commits permission, root claims and reservation totals together.
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
root resolution uses a retained index to the original tenant/operation and then
rechecks the full permission. Failed admission never installs a lookup entry.
The separate storage probe now checks durable pending-state transactions and
fenced inspection after upgrade. Neither probe supplies certificate bytes or
provider effects; complete service persistence is still unfinished. Production host
authentication, consumer outbox atomicity, instruction budgets and recovery remain open.
The provider's accepted certificate lifetime/replay and project/bucket enforcement
are unknown; a local deadline cannot establish those guarantees.

The durable exposure path now uses a shared guarded workflow with exact permission
and current host-evidence binding. Independent provider/recovery/durability blockers
precede mutation; previews cannot override later local changes. Local IC evidence
covers write/response rollback, lost committed acknowledgment, rejection of repeated
exposure and retained charged history through restore. The shared certificate
handler now resolves the original permission from the root and commits the gate
before returning the reviewed plain Caffeine update payload. The local endpoint
rejects by default; successful IC response tests use operator-configured substitute
facts. Headless Rust tests now extract and verify the actual HTTP response
certificate, reject forged/unrelated proofs and recover saved request IDs without
reissuance. Historical recovery survives revocation without renewing permission.
An opt-in Chromium fixture also verifies certificates and retains IndexedDB intent
across competing tabs, cancellation and reload. Production browser/consumer storage,
host-fact acquisition and deployed gateway acceptance remain open.

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

The shared `workflow::references::receipt` boundary and
`ReplicatedReferenceClient` now expose the durable owner's exact receipt through
an authenticated replicated IC call. The original upload/object/lifetime and
reference operation are bound in full; explicit absence, historical success,
historical failure and lookup refusal stay distinct. Local two-canister tests
cover inspection through suspension, settlement and fenced upgrade. This supplies
consumer recovery evidence, not the consumer's transaction/outbox, current
liveness or automatic dispatch authority. Canonical mutation delivery now uses
`workflow::references::apply` and the same client's explicit `apply` method with
one exact command and one bounded call. Its response retains the original inner
result and replay status. PocketIC recovers a committed operation after reply
validation fails, proves accounting is unchanged by retry, and exercises cleanup
under suspension/capacity and restore refusal. No client-side intent journal or
automatic retry is implied by those library clients.

The separate `blob-consumer-probe` provides local existing-content and fresh-upload
registration evidence. It retains bounded asset intent and cleanup identities,
atomically checks publication/tombstones/dependencies, and keeps release intent
through lost acknowledgment. Existing content uses an explicit retain receipt;
fresh content uses exact authenticated admission history and the upload's
first reference without another retain. Both obtain a current live-reference
descriptor before publication. Actual IC tests cover cancellation races, callback
traps, late completion under suspension, exact cleanup and fenced upgrade.
Two lifetime asset/outbox slots preserve cleanup at capacity. This labelled
application substitute persists the full permission and asset intent before
canonical admission through its actual tenant canister. Exact inspection binds
uploader/expiry, recovers interrupted acknowledgments and never resends uncertain
admission. The same permission client now delivers revocation, with the consumer's
tombstone and withdrawal intent committed first. Unknown outcomes require exact
inspection; local withdrawal never clears exposed/provider obligations.
Shared uploader preparation and exact manifest inspection now replace the storage
fixture's private preparation contract. The full permission and bounded declaration
bind actual uploader authority; tenant/uploader queries retain the first metadata
and leaves through revocation and restore. Bounded reply validation and IC tests
cover unusable acknowledgments, unchanged retries and atomic write rollback.
The replicated manifest client now binds actor/tenant/service and sends once after
bounded declaration validation. A separate uploader fixture persists exact intent
and retains uncertain acknowledgments, typed refusals and fenced history. Local
three-canister evidence joins admission, preparation recovery and tenant cleanup.
Uploader cancellation now retains tombstones and uncertain history across delayed
acknowledgments and upgrade. Joint IC races keep tenant withdrawal separate and
preserve exposed late-completion/reference-cleanup obligations under suspension.
Production browser/headless integration and intent storage remain open;
exposure/completion retain private fixture controls.
Real application transactions, production sizing, browser delivery and operational
recovery remain open.

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
records or transport. The consumer substitute above supplies local interruption
and registration-versus-delete evidence for first and additional references; complete
application/adaptor journeys and provider qualification remain open.
Propagate each implemented milestone through both adapters and operator behavior.

The existing [acceptance cases A01–A12](acceptance-plan.md) and
[Canic parity inventory](canic-parity.md) remain the completion checklist.
Generic providers, cross-tenant deduplication, cross-release migration, new
confidentiality guarantees and Canic source removal are outside this implementation
batch. Do not change Cargo versions or run release/deployment commands merely
because this roadmap names 0.2.
