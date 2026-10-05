# Development plan

Released 0.14.2 supplies the framework-independent library and standalone service,
configured uploads, exact verifier completion and maintained native/browser
publication components. Consumer owners supply framework wrappers, deployment
integration and their composition tests. Library publication
is separate from qualified live service operation.

## Starting point

The shared service owns tenant policy, permissions/manifests, references, receipts,
quotas, provider economics and durable local journals. Standalone delegates to
those handlers and restores all owners synchronously into fences. Native tooling
prepares exact inputs and verified snapshots, inspects signed service state,
performs reference/verifier operations once and recovers uncertain acknowledgments.
Browser transfer composes the maintained Caffeine SDK with the certificate intent
and bounded gateway journal. A separate [0.7.0 live trial](evidence/caffeine-probes/deployed/2026-10-02-trial-v070-live-01/summary.json)
completes 1 KiB and ten-chunk 10 MiB uploads, independent verification, verifier
attestations and tenant downloads. Logical releases retain physical/billing
obligations. Released 0.8.0 qualifies operator-only current-instance upgrade recovery
against independent IC history and fences old heap snapshots; older-backup
activation and complete consumer batch delivery remain open.

## Milestones and completion evidence

| Milestone | Maintained implementation | Remaining evidence |
| --- | --- | --- |
| Provider contract | Caffeine is the sole provider; request/callback definitions have one owner, retained probes separate source/local/live observations | Pre-charge limits, namespace authority, escaped-certificate/replay economics, deletion and billing cessation |
| Durable service | Shared installation and bounded owners, exact replay/history, separate logical/physical/liability accounting; current-instance IC-history recovery and old-snapshot refusal | Consumer adoption of the hard cut; older-backup activation with complete surviving obligations and freshness authority |
| Standalone prototype | Explicit endpoints; live 0.7.0 1 KiB/10 MiB upload, independent verification, attestation, tenant download and logical release; released current-instance recovery with local IC evidence | Distinct consumer media, complete publication batch and older-backup recovery |
| Consumer acceptance | Framework-free public library and generic host grants/handlers | Concrete consumer, accountable owner, its wrapper/outbox tests and real asset transaction |
| Retirement | Fences, exact history and obligation-preserving release semantics | Per-installation inventory, owned settlement/disposition and actual billing-cessation evidence |

## Next action

Continue from [the current handoff](status/current.md), which owns the exact
released baseline, active implementation batch and remaining actions. The
[implementation history](status/history.md) retains trial funding/link/refusal
captures and superseded next-step instructions; it is not a second work queue.

The maintained native session composes original setup, verification and reference
journals. The browser host/worker and callable Chromium bridge bind selected
signer, profile, asset origin and preparation hints. Released 0.13.0 source-session
recovery retains original native phase paths, including through status-only
restarts. Native intent now retains explicit browser selection; the launcher checks
the original native intent/ready/input hashes and binds that session to its profile.
The native transfer phase retains the exact handoff before browser dispatch;
repeated/recovered phases request observation through the existing browser claims.
Released 0.14.0 native guidance and the bounded `driveSession` bridge select/run
phases using those same owners. The bounded native subprocess helper now supplies
private phase transport and checked final exit. Consumers select their binaries,
keys and original history and choose explicit restart; qualify that adoption
without another dispatcher or effect journal.
Then qualify real consumer media, publication transactions, overlapping references
and serving/CSP behavior. Source code and local substitutes do not prove adoption.

The [representative-byte trial](evidence/caffeine-probes/README.md#representative-emitted-media--2026-10-04)
now qualifies selected emitted PNG/JPEG/WebP/GLB bodies locally, including lost
replies, corruption, native map-output recovery and reference cleanup. Its initial
observations retain their explicitly different trial roots. The
[cache-metadata batch](evidence/caffeine-probes/README.md#original-cache-metadata--2026-10-04)
now reproduces all four retained original roots through explicit cache hints,
with local cached completion/recovery and browser header checks. Qualify the
[selected-source tool recipe](local-tools.md#install-the-native-and-browser-tools)
downstream before claiming consumer adoption. Local fresh-prefix installation,
[isolated-source native/browser composition](evidence/caffeine-probes/README.md#isolated-source-tool-installation--2026-10-04)
and predictable transfer-budget refusal now pass. The source copy uses fresh npm
dependencies and builds without Git metadata; exact installed tools complete
cached media and recover lost replies. This is unreleased local source, not a
released downstream installation. The separate [clean-release check](evidence/caffeine-probes/README.md#clean-released-source-tool-installation--2026-10-04)
now qualifies tagged 0.14.3 tools with a fresh prefix/npm directory and matching
local completion/recovery in this repository. Consumer-owned installation and
adoption remain open. Active requests are [publisher #4](https://github.com/dragginzgame/ic-blob-storage/issues/4),
[delivery #5](https://github.com/dragginzgame/ic-blob-storage/issues/5) and
[lifetime #6](https://github.com/dragginzgame/ic-blob-storage/issues/6); local package
reports in Miner are archived, not the active feedback owner.

Keep both frozen live owners and their full provider/billing histories. Their
lifetime capacity is exhausted; source cleanup cannot reset it. Provider deletion,
billing cessation and older-backup activation remain separate qualification gaps.
Use [service gaps](service-gaps.md) and [the probe ledger](evidence/caffeine-probes/README.md)
for their retained evidence and obligations.

The browser lifetime ceiling can be configured to one million, independently of
4,096-file batches. Actual journal restart evidence covers 675 rows, not a million
populated service records. Qualify stable reopen and browser memory/transport costs
against the [consumed inventory](operator-guide.md#larger-inventories-and-a-dedicated-storage-owner)
before promising that scale. Keep the single storage owner and its local journals;
restoration needs independent continuity evidence rather than a local counter.

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

The 2026-10-03 read-only review finds local HEAD
`3354dfc6b9fe791884ec69e8dd344de313b36940`, clean feedback SHA-256
`62b8cede95cfc1dc5bcac8e33eac07c8b20647767ade2aa0875a7eb5dadb1a91`.
Miner's single-build emitted workload is 679 asset identities / 675 distinct
blobs / 221,173,950 bytes / 763 distinct leaves, with 767 leaves before
deduplication. This recorded measurement is more useful than the 816-file source
envelope; this review does not rerun its build or qualify provider deduplication.
Miner's last assessment recognises both live sample journeys and then-Unreleased
inputs/checker work; those tools are now released here.
BLOB-002/003/004 remain open: full frozen publication/recovery, public delivery
and release retention/reintroduction. Its maintained preparation uses registry
0.7.0; our released 0.8.0/0.9.0 tooling is not that adopted package. The immutable
installation anchor's twenty-change management-history horizon also needs an
operating/retirement plan. Canic wrapper adoption remains deferred until its
independent blob code's published removal. No sibling write, build or provider
experiment occurs. The [retained integration review](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback)
records that historical assessment; current work is tracked in GitHub issues.

The 2026-10-02 read-only follow-up finds local HEAD
`dcc4b0131a928092f39be45d06d5397f3045f8ff` and uncommitted feedback SHA-256
`cd187187521a88392b052eed265ee7d4dc16130120e5f892a369123760703c35`.
Its latest recorded survey reports 816 media files/271,176,671 bytes, largest
8,362,256 bytes. This supersedes the earlier survey for sizing guidance only;
it is not a refreshed asset measurement or production consumed-media inventory.
The current source's configured issuance removes the deliberate size/capacity
policy blocker, released in 0.7.0. Headless publication, delivery/trust,
retention/reintroduction and consumer wrapper acceptance remain open.

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

Reference recovery uses the shared `blob_reference_receipt` query and maintained
native exact-request tools. Signed local tests distinguish historical results,
current liveness and restoration fences through release and settlement. Native
create-new claims retain signed packets before one dispatch; interruption or a
lost reply never authorizes resubmission. The fixture-only append journal is
retired, with no effect ownership or new identity allocation introduced.
