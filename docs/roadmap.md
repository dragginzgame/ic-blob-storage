# Development plan

Released 0.5.0 makes the library and standalone service independent of
consumer frameworks and adopts ic-memory 0.15.0. Downstream wrappers, deployment
integration and their tests belong in consumer repositories. Library publication
is separate from qualified live service operation.

## Starting point

The shared service owns tenant policy, permissions/manifests, references, receipts,
quotas, provider economics and durable local journals. Standalone delegates to
those handlers and restores all owners synchronously into fences. Native tooling
prepares exact inputs and verified snapshots, inspects signed service state,
performs reference/verifier operations once and recovers uncertain acknowledgments.
Browser transfer composes the maintained Caffeine SDK with the certificate intent
and bounded gateway journal. Actual live certificate/upload/download remains open.

## Milestones and completion evidence

| Milestone | Maintained implementation | Remaining evidence |
| --- | --- | --- |
| Provider contract | Caffeine is the sole provider; request/callback definitions have one owner, retained probes separate source/local/live observations | Pre-charge limits, namespace authority, escaped-certificate/replay economics, deletion and billing cessation |
| Durable service | Shared installation and bounded owners, exact replay/history, separate logical/physical/liability accounting and local IC rollback | Qualified operational recovery with complete surviving obligations and freshness authority |
| Standalone prototype | Explicit endpoints and installation, native upload setup/download/verifier/reference tooling and browser preparation | One bounded actual upload and independent verified download under an accepted operating contract |
| Consumer acceptance | Framework-free public library and generic host grants/handlers | Concrete consumer, accountable owner, its wrapper/outbox tests and real asset transaction |
| Retirement | Fences, exact history and obligation-preserving release semantics | Per-installation inventory, owned settlement/disposition and actual billing-cessation evidence |

## Next action

The maintainer accepted the [restricted standalone contract](standalone-trial.md): one 1 KiB
file under explicit trusted roles and a restricted fresh-instance lifecycle.
The maintainer selected a total planning budget of 100T cycles; the proposed
10T service / 1T initial provider / 89T unallocated split is not a price estimate,
enforced provider cap or effect authority. Offline 1 KiB preparation and complete
host-init syntax now pass; the existing 10 MiB rehearsal remains the default.

Finish the existing upload/download trial prerequisites. Select exact isolated
service/tenant/uploader/verifier, Cashier/payer, gateway/project/bucket and reviewable
financial/byte/time limits. The offline checks prepare these inputs but grant no
platform identity, account authority or provider qualification. The current
certificate gate checks installed uploader trust, local namespace, restricted
envelope and durable current ownership. Provider spending/replay guarantees remain
outside the accepted contract; client ceilings are not a provider spending cap.
This semantic/init/API hard cut requires a minor release before the actual trial.

Do not add another journal owner, recovery canister or allocator. Prefer the
single storage owner and its local durable journals. Restored/stale instances stay
fenced until complete independently surviving evidence proves safe obligations,
identity allocation and accounting. Source removal and installation retirement
are separate; cleanup cannot discard uncertain effects or continuing billing.

See [acceptance](acceptance-plan.md), [service contract](service-contract.md),
[trial inputs](operator-guide.md#isolated-uploaddownload-trial-plan) and the
[probe ledger](evidence/caffeine-probes/README.md). All probes retain intent,
requests, results, hashes, failures and cleanup obligations. This plan authorizes
no deployment, funding, provider effect or upstream repository change.

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
| Miner BLOB-001 — managed service | Constrains consumer integration: durable shared handlers and one consumer-owned storage owner, with host-owned memory/lifecycle. Miner tenant, uploader and managed-role bindings still need an accepted application design; no per-shard media owner is implied. |
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
The [consumer acceptance extension](acceptance-plan.md)
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

Fresh-upload capacity now has a shared tenant-scoped `blob_upload_capacity` query
in the standalone host and both admission fixtures. Its independent
lifetime/concurrent/leaf/byte headroom includes pending reservations and continuing
billing; suspension and the durable owner's restore fence remain visible.
It complements existing-object discovery/reference capacity. The unpublished
`blob-fixture-inventory` command now connects validated prepared inventories to
these queries on an explicit existing local probe, reporting fenced capacity as
blocked. Reference headroom now uses the shared `blob_reference_capacity` query
in the same hosts, with exact request echoes and an independent restore fence;
private capacity interfaces and duplicate conversions are removed. Content discovery
now uses shared `blob_lookup_content` in all three hosts, with complete independent
original identities, exact echoes and restore fencing even for absence. The private
discovery endpoint is removed; inventory consumes only shared inspection queries.
It preserves asset reference
demand and separates not-visible, pending, live and retired roots. Sequential
observations reserve nothing and do not prove global absence or fresh admission.
Production authentication, new-object reference sizing and persisted exact
operation identities remain outstanding; no upload or funding occurs here.

Reference recovery now has a shared `blob_reference_receipt` query used by the
local CLI and replicated client. `blob-fixture-reference` journals a bounded exact
intent with independent upload/object/lifetime/first-reference bindings, then
queries standalone or durable storage through the same maintained decoder,
including after release, settlement or fenced restoration. It preserves service
refusals separately from absence and recorded results without allocating a new request
or reference. Caller-supplied IDs still need a surviving allocation authority;
local writes now use an identity-keyed, bounded journal with an exclusive OS lock
and file/directory sync before acknowledgment. Exact retries recover the same
record; changed payloads conflict. This protects cooperating writers of one local
directory, not copies or rollbacks. It is not a registration outbox or
restored-instance fence. Upload/provider intents and actual dispatch remain open.
