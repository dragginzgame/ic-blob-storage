# Current status

Date: 2026-09-26

## Current batch — local operator actions and funding resource guards

The unpublished `blob-fixture-refresh` command now offers passive dry-run and an
explicit refresh with separate post-status diagnostics. Exact service, namespace,
source, account, configuration revision and next attempt are checked before intent
and dispatch. Old previews and consumed requests cannot launch another read. Busy
and restored owners remain blocked. JSON preserves completion or uncertainty even
if the following query fails; there is no application retry or automatic sequence
advance. The existing status client remains query-only.

`blob-fixture-sync` now uses the same action/diagnosis client workflow. Its exact
request includes the controlled source, operator-edit revision and next sync
sequence. The fixture retains a required SyncControlRecord in its v1 archive;
every revocation invalidates previews, even for an already absent member. Revision
exhaustion blocks new syncs while preserving revocation. Status exposes source,
revision and last sequence without granting a reusable token. The published
GatewayRegistry API is unchanged. Reentrant source scenarios now carry explicit
requests, including the deliberately authorized replacement after revocation.

The raw `fund` endpoint remains a controlled transfer experiment, separate from
production operator admission. It now requires an explicit installed attachment
allocation and positive reserve. A full offer is reserved atomically with its
original intent before dispatch. Accounting derives from the bounded journal:
accepted cycles remain charged, exact callback refunds and proven unsent offers
release their allocation separately, and missing callbacks retain the full offer.
There is no replenishment from gross cycle top-ups or incoming receipts. Required
budget state is a v1 fixture hard cut; cross-release fixtures require reinstall.

The query-only `blob-fixture-funding-preview` checks exact service/peer/id/amount
and budget revision, used identities and journal capacity, then composes all
uncredited history with shared admission
evidence policy. Unknown recovery/spendability and absent limits remain blockers;
positive transport acceptance never proves credit. Later refunds cannot hide
older unresolved payments. The preview cannot transfer, reserve, retry or consume
an identity, including after restore. No accounting/provider overrides are accepted.
Budget revision advances on each admission and terminal observation, including full
refunds. Local reserve violations are distinct from production spendability: the
attachment envelope excludes execution fees and other operating liabilities.

The fixture now also requires positive operating slack and explicit other local
liabilities. Shared additive liquidity policy checks the complete offer against
platform liquid cycles minus those holds and the call-cost bound. The workflow
samples its exact encoded call after persisting intent, before dispatch; a refusal
retains the consumed identity with a LiquidityBlocked/no-transfer observation and
no fabricated callback refund. Accepted/uncertain charges and restore fences remain.
Preview cost bounds cover all valid local reply controls. Its liquid figures may
be cached and can change without a budget revision, so updates never reuse them.
These local holds do not establish complete production liabilities or credit.

Admission review found and fixed a same-message rollback bug: the callback trap
control ran even after a liquidity refusal with no callback. Unsent attempts now
commit their consumed identities, and restoration accepts those no-callback records.
Actual zero/full-refund callback traps still retain the entire uncertain offer.
Preview diagnosis also reports the same maximum attachment bound as update admission.
Native, strict Clippy, Wasm/rustdoc and full affected funding/operator PocketIC checks
pass, including refusal-capacity exhaustion and fenced restore. See
[refusal evidence](../evidence/core-primitives.md#funding-refusal-recovery-after-0116).

The additive library `assess_funding_evidence` API preserves independent missing
and known unsafe facts. Reserve arithmetic runs only with known spendability and
validated limits, without reducing the request. Complete observations retain the
existing admission API's behavior. The fixture supplies no artificial positive
funding evidence and always reports blocked admission.

Native admission tests, strict affected-package Clippy, warning-free client/protocol
rustdoc, fixture Wasm builds, client Wasm check and actual operator PocketIC tests
pass, including held reads and an update witness whose post-status method is absent.
See [operator action evidence](../evidence/core-primitives.md#explicit-local-operator-refresh-after-0116).
Changes are in Unreleased. Cargo and the release receipt remain 0.1.16. Published
library additions are compatible; both unpublished action endpoints require exact requests.
Gateway-sync and full journey/recovery targets also pass. The additional actual
live-sync callback upgrade retains a fenced pending intent; old replies cannot
change membership or free it. See [sync evidence](../evidence/core-primitives.md#explicit-local-gateway-sync-after-0116).
Funding admission native tests, strict affected Clippy, warning-free rustdoc,
library/client Wasm checks, fixture builds and complete funding/operator PocketIC
targets pass. Preview tests cover credit uncertainty, later refunds, callback traps,
capacity, restore, added gross cycles and unchanged journals. See
[funding preview evidence](../evidence/core-primitives.md#passive-funding-admission-preview-after-0116).
The attachment-budget native and actual funding/operator PocketIC checks pass,
including atomic rejection, incoming-cycle isolation, full-refund stale previews,
callback rollback and retained reservations through fenced restore. See
[budget evidence](../evidence/core-primitives.md#local-attachment-budget-after-0116).
Native liquidity, fixture and client checks, strict affected Clippy, warning-free
rustdoc, fixture Wasm builds, library/client Wasm checks and complete funding/operator
PocketIC targets pass. Tests cover fee-only refusal, operating liabilities, consumed
unsent identities, added funds without an allocation revision and fenced restoration.
See [liquidity evidence](../evidence/core-primitives.md#funding-liquidity-guard-after-0116).
Full CI/release validation was not run under this continuation.

## Released baseline

The maintainer pushed 0.1.16. Local main, origin/main and annotated tag v0.1.16
point to `4a3aaa6`, from source `d084112e811866682f8c9374ab46ac9c329624ff`.
Cargo and the release receipt are 0.1.16; the worktree was clean when this release
was checked. Registry publication was not queried.
Release/publication preserve artifacts; cleanup requires an explicit request.
See [release guidance](../releasing.md).

The library provides content/provider identities, streaming Caffeine hashing,
root-bound manifests, chunk verification/coverage and bounded missing-chunk pages.
It also provides billing/configuration validation, bounded reply codecs, pure
funding/readiness policy and scoped gateway sync with stale-reply rejection.

Transient catalogs own confirmed-object lifecycles, immutable root claims,
reference receipts and exact upload reservations. Physical deletion and final
billing cessation release different capacities. Cancelled/settled history retains
lifetime slots; uncertain exposed uploads keep reservations. Tenant/gateway
policy enforces full bindings and current authority.

0.1.12 aligns through ic-memory 0.14.3, with its exact stable-structures 0.7.2,
matching the checked Canic/IcyDB graph. Hosts own bootstrap/policy/grants; no blob
stable stores or lifecycle hooks are declared. It adds bounded tenant obligation
pages, tenant-only confirmed and upload usage scans, and Canic's adapted cloc
helper. Native/PocketIC evidence covers current accounting and actual caller
isolation with explicitly substituted provider facts, not persistence or recovery.
See [core evidence](../evidence/core-primitives.md).

Released source inventories remain historical: 0.1.5 matches `6bd0d45` (Cargo
0.1.4), 0.1.6 matches `d3ca8c4` (Cargo 0.1.5), 0.1.7 matches `600315e` (Cargo
0.1.6), 0.1.8 matches `20ec33d` (Cargo 0.1.7), 0.1.9 matches `832b364` (Cargo
0.1.8), 0.1.10 matches `aaa5057` (Cargo 0.1.9), and 0.1.11 matches `3b5ab64`
(Cargo 0.1.10). The 0.1.12 memory-alignment and tenant-obligations inventories
were verified against Git source `e125fb8` (Cargo 0.1.11) after release. Do not
rotate historical inventories for later source/version changes.
The released funding-callback inventory matches source `ab75523` (Cargo 0.1.12).
The 0.1.14 funding-reconciliation and cashier-audit inventories match source
`3f73703` (Cargo 0.1.13); preserve them as historical evidence.
The 0.1.15 upload-deletion journey inventory was verified against source
`481fe67` (Cargo 0.1.14). Preserve it and its protocol selection as historical
evidence. The released 0.1.16 verification-checkpoints inventory was verified
against source `d084112` (Cargo 0.1.15); all 151 entries match. Preserve it unchanged.
Current operator actions use a separate operator-actions inventory.

## Current follow-up — provider contract review

The maintainer requested the provider review as the next batch. The completed
[contract decision](../provider-review.md#contract-decision-after-0112) and
[refresh record](../evidence/caffeine-contract-refresh.json) distinguish current
source facts, anonymous deployment observations and unresolved server semantics.
The investigation and unpublished funding fixture shipped in repository release
0.1.13. The fixture is never a production provider implementation.
The subsequent continuation produced a concrete
[qualification sequence](../acceptance-plan.md#provider-qualification-sequence-after-0112):
required bindings/budgets, proposed byte fixtures, source review before adapter
implementation, failure cuts and retained-obligation disposition. It is a plan,
not executed provider qualification or authorization for effects. The maintainer
confirmed Caffeine is the sole provider target and requested broader internet
research. The earlier alternative-provider question is superseded.

Official Caffeine main remains `e5cacdfe5ce55e939edb02980fca800c0c13f421`;
refetched backend source hashes, npm latest 1.1.2/integrity, Mops highest 1.1.1
and deployed Cashier Candid all match retained evidence. Archive integrity,
Mops file hashes, gateway membership and pricing were not queried again.
Toko development remains `6519b72d2a420564dabaf700fc55f7b8603d9fd3` and local
Canic HEAD remains `3f825aa223e663a562a7cb1cca72e57b5703e0e9`.

Toko's configured project canister supplies certificates and gateway owner IDs;
Canic funds its current canister's Cashier account. A separately deployed storage
service changes these bindings under the observed pattern. Exact ownership,
payment and existing-installation disposition need a supported arrangement.
Toko's project/bucket defaults and existing-chunk optimization establish neither
exclusive namespace provisioning nor retry/charging guarantees.

Broader research found Rabbithole's documented Caffeine integration and public
Motoko/TypeScript source at `baa4d86314822711735cc9860220c109210b1ce9`, plus
Toko's public Canic integration announcement. Rabbithole implements account
bootstrap through self-account top-up, Cashier delegation and the same gateway
owner/project/bucket pattern. Its client also reads blob-tree metadata by root.
See [independent deployment evidence](../provider-review.md#independent-deployment-support).
General Caffeine app-export guidance does not establish that external integration
is unavailable. These consumer implementations supply concrete integration leads;
they do not establish server retry charges, receipt retention or final billing.

Further investigation found DFINITY's official Caffeine example, current main
`ef29e8a6e8063c6fe654cac53a3497cab585fefa` (2026-04-10). It explicitly documents
Rust onboarding, payment-account linking with a daily limit and existing-chunk
resume responses. General independent-Rust support is no longer an open question.
See [implementation findings](../provider-review.md#findings-that-change-the-implementation-plan).
The older example returns deletion candidates as text while the September
Motoko package returns blobs; its demo authorization, root reuse and config
restoration also cannot be adopted unchanged. Its 30-day cycle-storage terms
differ from app-credit help pages and need deployment-specific confirmation.

IC platform documentation additionally confirms bounded-wait SYS_UNKNOWN can
lose attached/refunded cycles. Canic currently uses bounded wait for direct
top-ups. Proposed replacement funding uses durable intent, bounded attachments
and concurrency, unbounded wait and exact callback refund capture. Accepted
transport cycles and Cashier account credit remain separate facts. No production
transport or stable workflow was added; no live paid operation was performed.

The maintainer explicitly requires the latest version. Rechecking npm, Mops,
official main and deployed Cashier Candid found no changes. Current reference
selection is npm 1.1.2 / Mops 1.1.1, including the backend's `vec blob`
deletion list. Do not adopt the April example's text shape or add a dual reader.

The [funding experiment](../evidence/core-primitives.md#funding-callback-experiment)
uses ic-testkit's exported PocketIC and the existing source-backed Cashier reply
fixtures. Exact refunds match independently observed receiver acceptance for
zero/partial/full success, provider error, malformed reply and explicit reject.
After an actual callback trap, receiver acceptance survives while the sender's
completion rolls back; the unresolved attempt prevents another transfer.
The fixture now writes bounded journals through host-owned ic-memory and restores
synchronously. Same-release upgrades of both canisters preserve exact journals,
bindings, identity reuse rejection, unresolved-payment blocking and lifetime
limits. Receiver traps roll back acceptance and receipts with a full refund.
Failed post_upgrade hooks preserve usable journals and unresolved-payment
blocking, followed by successful upgrades. This is local fixture recovery
evidence. That released fixture lacked old-backup fencing; the current follow-up
below adds inspection-only restoration. Production service persistence remains open.

## Next work and gates

The post-0.1.13 follow-up adds shared `FundingTransfer` values and pure funding
reconciliation policy. Known callback refunds, proven enqueue failure and unknown
effects remain distinct; positive transport acceptance always requires separate
provider credit evidence. This is additive local arithmetic/policy, without a
production journal, paid call, retry or frozen service schema. The fixture now
uses the shared implementation and explicit initial cycle budgets. Native tests
cover extreme amounts and invalid/missing evidence. PocketIC additionally proves
an actual insufficient-cycles enqueue failure has no callback refund or receiver
receipt, retains history through upgrade, and cannot reuse its identity.
Targeted native tests, strict Clippy, rustdoc, Wasm and funding PocketIC checks
pass; see [current evidence](../evidence/core-primitives.md#shared-funding-reconciliation-after-0113).
Shared funding accounting and the audit decoder shipped in library release
0.1.14. That release does not qualify the provider or production service.

The next continuation refreshed anonymous Cashier metadata; its Candid hash still
matches `232b08e4514048d4de48d6d1bf4387f577bfb64c7e2e2ded699a5e52d475d76f`.
A bounded audit-response decoder now retains opaque CSV, counts and cursors while
preserving typed provider failures. Independent didc fixtures and targeted native,
Clippy, rustdoc and Wasm checks pass; see [audit evidence](../evidence/core-primitives.md#cashier-audit-response-decoding).
No live account audit was requested. The search did not establish CSV columns,
cursor semantics, operation matching or retention. No audit page clears uncertainty.

Next qualify the selected wire contract and exact provider reconciliation inputs;
do not add ledger decoding or audit-row interpretation from an assumed schema.
Remaining gaps are:

1. Apply documented self-account/linked-payer patterns to the exact installation
   and existing Toko obligations. Verify the current blob deletion-list Candid
   with the deployed gateway; do not infer namespace authority from defaults.
2. Upload operation identity, retry charging, lost-reply completion lookup,
   incomplete-object behavior and numeric evidence-retention bounds.
3. Exact top-up/ledger credit and refund reconciliation after lost replies.
4. Object-specific deletion/final billing proof and authority surviving restore.

The maintainer approved starting a connected local upload/deletion journey after
0.1.14. Official main and refreshed Mixin.mo/Storage.mo hashes are unchanged;
[protocol selection](../evidence/upload-deletion-protocol.json) records the four
current method shapes and local semantic deviations. The authority fixture now
drives an initially empty UploadCatalog from admission and certificate exposure
through completion, release, physical deletion and separate billing cessation.
The gateway-source fixture sends actual inter-canister deletion confirmations.
PocketIC checks interruption, exact/conflicting replay, tenant separation,
same-message rollback of a mixed invalid deletion batch, delayed completion,
root non-reuse and gateway revocation. Targeted fixture Clippy, Wasm and the journey,
authority, uploads, obligations and gateway-sync targets pass. See
[journey evidence](../evidence/core-primitives.md#connected-upload-and-deletion-journey-after-0114).
The connected journey and recovery experiments shipped in 0.1.15. The journey reserves
real declared roots/digests and checks actual content through the shared manifest
and ordered verifier before certificate exposure. The fixture now accepts
nonempty files up to 6 MiB in six chunks, with explicit metadata bounded to eight
headers and 1 KiB of framed input. It retains only manifest/hash state across
messages and discards checked bytes. Tenant-only progress reports the checked
prefix and separate raw-digest verdict. Exact old chunks are checked without
hashing twice, and admission replay cannot reset either progress or final rejection.
Global physical/liability bounds are 12 MiB, tenant logical capacity is 6 MiB;
eight lifetime objects and four per tenant remain the metadata/session bounds.
Corrupt/truncated/oversized input, wrong raw digests, conflicting replay and
cross-tenant verification cannot issue authority or release capacity. Successful
verification still cannot confirm storage before exposure and the separate
gateway-supplied completion fact. A binary raw-digest parser supports the boundary.
Targeted native identity tests, strict Clippy, fixture Wasm and PocketIC journey,
content, authority, uploads, obligations and gateway-sync checks pass.

Refreshed official main is unchanged; current StorageClient.ts SHA-256 is
`a0a3ee3bb74ecca133bc6f68a024821b3319c7ccd4886940c31d179b9f85f557`.
It forwards the V4 update certificate as OwnerEgressSignature, while parallelUpload
discards the chunk completion flag and putFile returns the root after requests.
This confirms the earlier client finding, not gateway verification/recovery.
Completion and final billing remain substitutes; no gateway HTTP upload,
certificate-chain verification, provider atomicity or upload persistence is qualified.
PocketIC now covers a six-chunk metadata-bearing file through deletion/billing,
one-byte final chunks, skipped/corrupt/duplicate chunks, conflicting metadata,
header/chunk budgets, cancellation of partial work and final raw-digest rejection.
The current fixture hard-cuts its one-shot verification endpoint in favor of
chunk append/progress; no released library API changes in this continuation.
The continued local readback journey now fetches individual leaves from a
driver-controlled source canister through actual inter-canister calls. The
bound tenant's live reference and current gateway authority are checked before
and after the await; returned length/hash must match the admitted manifest.
One pending read bounds local concurrency, with exact callback cleanup and no
automatic retry. Revocation and successful gateway synchronization invalidate
old reads without freeing their slot early. PocketIC proves rejection after
release and revocation/re-addition while a reply is held, along with partial
reads, wrong-file/corrupt/truncated data, byte/decoder limits and source rejection.
Native read-slot tests, strict fixture Clippy, both Wasm builds and all targeted
journey/content/authority/uploads/obligations/gateway-sync checks pass.

Read-source bytes and methods remain explicit substitutes, not Caffeine HTTP or
durability evidence. The transient authority fixture rejects upgrades
in both lifecycle hooks because it cannot reconstruct its journals. PocketIC
proves stop/start continuity and rejected-upgrade rollback, including a skipped
outgoing hook. Verified prefixes, exposed reservations, cancelled root history,
continuing billing and held reads remain intact. An operator-armed callback trap
rolls back attempted read-slot cleanup; later reads stay blocked through stop/start
and elapsed time. There is no unsafe slot-reset or unfence endpoint. Strict Clippy,
both Wasm builds and the targeted lifecycle/journey/regression checks pass.

The source now owns a same-release fixture journal through ic-memory, with one
1 MiB leaf, exact bindings, read scheduling state and at most 64 lifetime outgoing
call intents/results (each deletion action contains at most eight roots). Intent
is saved before dispatch; completed history is never recycled. Its 1,114,112-byte
cell has bounded Candid decoding. Restoration is synchronous and always enters a
permanent fence: the original driver can inspect retained data but no operational
endpoint can act. Ordinary upgrades reject pending reads/unresolved effects;
skipping the outgoing hook still loads and fences the journal. Missing journals
reject atomically. PocketIC covers maximum leaf/history, missing/older stable
data, unresolved callback rollback, repeat/skip-hook upgrades and authority checks.
An older journal containing a held read preserves its pending flag without replay
or slot release. Strict fixture Clippy, both Wasm builds and all targeted journey,
content, authority, uploads, obligations and gateway-sync checks pass.
This recovers only the controlled source, not upload authority or provider state.

The authority now atomically persists an inspection archive through ic-memory
(`fixture.authority.archive.v1`, ID 120 in its own host, 65,536-byte cell).
The archive covers all three catalogs: two sample confirmed objects, four sample
upload operations and up to eight journey operations. It retains exact identities,
reservations, phases, original release receipts, logical/physical/liability bytes,
admitted manifests and observed verification prefixes/verdicts. Cancelled and
settled roots remain. It also retains scoped gateway membership/sync sequences,
exact pending read tenant/root/index/gateway/token, invalidation and fault plans.
The read intent and archive commit before dispatch; terminal digest failure is
archived even though append returns an error. Shared gateway registries expose a
read-only sync view without exposing reusable tokens or restore authority.

Only the explicit operator can inspect the archive, reopening it from stable
memory. No archive input alters active state. PocketIC covers all catalogs,
full lifetime root history, final digest rejection, stable rollback of mixed
deletion batches and actual callback traps, old/missing stable data and controller
separation. Direct memory fault injection requires a real stateless update before
querying to invalidate the IC query cache. Targeted PocketIC regressions, native
gateway tests, strict Clippy, rustdoc, Wasm and formatting passed for 0.1.15. That
release's archive lacked streaming SHA state; authority upgrades rejected in both
hooks. No product stable schema or recovery authority was added.

The current follow-up adds a lossless ordered-verifier checkpoint primitive.
Its fixed 256-byte record binds format v1, library release, manifest root, expected
raw digest/length, received bytes and pinned SHA-256 state, with an accidental-damage
checksum. Decode checks block/buffer counters, canonical padding, SHA bounds and
complete-leaf prefix positions. A separately supplied trusted manifest/digest must
match on reconstruction. The checksum cannot authenticate or make old state fresh.
Hash checkpoints can contain up to 63 plaintext bytes and never appear in the
authority inspection DTO or debug output.

The authority's private ContentRecord now requires checkpoint bytes for pending
verification and empty bytes for terminal verdicts. Reconstruction cross-checks
the archived progress and uses the same manifest validator as admission. An
operator-only query probes a disposable reconstructed copy using one supplied
chunk; it neither changes active progress nor issues certificates, retries calls
or alters accounting. That probe alone proves mathematical reconstruction.
The changed fixture
record is a hard cut; no older-record fallback or migration path was added.
Targeted identity tests, strict affected-package Clippy, rustdoc, both Wasm builds
and the journey/content/authority/uploads/obligations/gateway-sync PocketIC targets
pass. See [checkpoint evidence](../evidence/core-primitives.md#verification-checkpoints-after-0115)
and its current inventory. Cargo remains 0.1.15 with these changes in Unreleased.

The continued batch now restores the complete authority archive synchronously
into a permanently fenced inspection owner. It binds Cargo.lock's release and
dependency selection even for terminal-only archives. Disposable shared catalogs
reconstruct sample and journey transitions, then compare every retained identity,
receipt, charge, manifest and checkpoint. Finished history is reconstructed before
active charges to avoid false capacity failures. Those temporary catalogs are
discarded; no empty live catalog or fresh registry replaces retained obligations.
Pending read/sync counters remain exact evidence, without callable tokens.

PocketIC proves successful normal/repeat/skipped-hook upgrades, partial-verifier
continuation on a disposable copy, all-catalog and full-lifetime history retention,
old-archive fencing, missing/corrupt incoming-journal rollback and operator access
independent of controller status after restore. A trapped read intent survives
forced restoration and repeated upgrades without source replay. All operational
endpoints reject while fenced, including provider liveness/deletion queries.
Active instances with pending reads/syncs reject ordinary upgrades. Native record
tests reject contradictory bindings, accounting, receipts and correlation fields.
Targeted native, strict Clippy, Wasm and PocketIC regressions pass. There is no
production schema, public library catalog restore API or unfence/reset endpoint.

The source now durably holds an exact gateway-list reply across bounded IC rounds.
PocketIC captures real pending sync journals on both sides, rejects busy upgrades
and overlapping calls, then restores those journals after the newer call completed.
Both restores remain fenced, retaining the pending identity/reply and unresolved
outgoing effect without replay. A held reply cannot undo revocation; exhausted
local scheduling rejects and clears only the exact read-only attempt, with a
separate explicit sync needed to send another request. The required source SyncRecord
hard-cuts its old fixture journal; older fixture schemas require reinstall.

Large-object recovery cases fill both tenants' lifetime slots with settled,
cancelled, live and exposed operations. Retained lifetime bytes exceed the active
budget while restored charges, manifests and receipts remain exact. Physical
deletion leaves billing capacity charged, blocking new admission until settlement;
restoring older billing evidence cannot reactivate admission. Targeted journey and
gateway-sync PocketIC checks, affected-package Clippy and both Wasm builds pass.
The parity and acceptance summaries now distinguish this executed fixture evidence
from production gaps instead of accumulating older native-only status paragraphs.

Shared `policy::diagnostics` now composes existing billing diagnoses with recovery,
provider qualification, unknown/in-progress/uncertain funding and outstanding work.
It produces blockers/warnings only, never effect authority or overall qualification.
Native tests retain all simultaneous blockers, preserve missing/malformed balances,
merge stricter recovery fences and keep ordinary outstanding work visible separately.

The authority's operator-only `operator_status` query reports each independent
catalog's lifetime phase/receipt counts and logical/physical/liability bytes, current
gateways and exact pending read/sync observations. It snapshots the active owner or
the validated frozen journal, not separately replaced old stable evidence. No
verifier bytes are exposed. Billing is unconfigured and provider balance, spendable
cycles and funding activity remain unobserved; no synthetic zero or clear state is
substituted. The fixture is always provider-unqualified. This is current-owner
diagnosis, not an archive-integrity audit. PocketIC proves controller/tenant denial,
unchanged journals and source history after repeated queries, separate release
obligations, pending/invalidation visibility and post-restore fencing. Native tests,
strict affected-package Clippy, rustdoc, both Wasm builds and the targeted PocketIC
journey/content/authority/uploads/obligations/gateway-sync checks pass. See
[operator evidence](../evidence/core-primitives.md#read-only-operator-diagnosis-after-0115).

The funding fixture now exposes driver-only current journal diagnosis through the
same operator policy. It reconstitutes transfer facts from retained intent/refund
observations, applies shared reconciliation and assesses the whole uncredited
history. Positive acceptance requires credit evidence; missing callback completion
keeps the full attachment unknown. Later full refunds/unsent calls cannot clear older
obligations. Incoming local receipts never become outgoing credit. Provider balance
and spendable cycles remain absent; the peer is not treated as a storage gateway.
Shared diagnosis now accepts unknown recovery rather than forcing a false claim of
reconciliation. The subsequent restore work below now enforces a permanent fence.
Native policy/conversion tests, strict affected-package Clippy, warning-free rustdoc,
three fixture Wasm builds, funding PocketIC and authority status regressions pass.
Actual queries leave both journals unchanged through traps, enqueue failure,
lifetime limits, elapsed time and successful/failed upgrades. See
[funding diagnosis evidence](../evidence/core-primitives.md#funding-operator-diagnosis-after-0115).
Cargo remains 0.1.15 and the coherent batch remains Unreleased.

Funding-fixture restoration now matches the other inspection-only fixtures. Its
required service/release/fence fields hard-cut the old fixture record, retaining v1.
The model checks actual service and Cargo.lock binding, capacity, unique outgoing
and incoming identities, bounded amounts, exact refund/acceptance arithmetic and
at most one final unresolved intent. Workflow checks stored reconciliation through shared
policy, then ops persists the fence before installing the restored owner. Sending,
acceptance and late callback completion cannot run after restore. No pre-upgrade
snapshot is required: mutations already commit atomically and unresolved intents
remain evidence rather than executable continuations. No reset/unfence exists.

PocketIC proves old empty journals cannot reuse already-paid identities, a restored
receiver rejects real attached cycles with a full refund, and a captured in-flight
intent stays unknown after the newer live call completed. A bounded delayed-success
mode uses actual IC rounds to capture that boundary. Upgrade while a reply is still
outstanding leaves the sender fenced when the real callback arrives. Receiver-side
in-flight upgrades preserve already-accepted cycles and the exact remaining refund.
Repeated and skipped-hook upgrades, stop/start and elapsed time preserve inspection; invalid,
missing, foreign-service and wrong-release journals reject atomically. ic-memory
bootstrap metadata can change on upgrade, so journal equality is checked separately
from allocator bytes. Native journal/conversion tests, strict affected-package
Clippy, warning-free fixture/protocol rustdoc, three Wasm builds and all targeted
funding PocketIC cases pass. See
[funding restore evidence](../evidence/core-primitives.md#fenced-funding-restoration-after-0115).

The unpublished `blob-fixture-status` client now attaches to explicitly selected
local PocketIC instances through ic-testkit. It requires canister/caller and expected
namespace/peer bindings, always queries `operator_status`, and reports typed read
failures separately from diagnosis blockers. It preserves unknown economics and
full-width amounts/counters as nulls and decimal strings. Canic's current local
CLI was inspected read-only: its metadata-selected query/update transport is not
adopted for diagnostics. No controller authority, method fallback or failed-read
retry is inferred. PocketIC SDK polling/busy handling remains inside its query path.
Actual CLI subprocess tests cover denied/wrong bindings, missing/update-only methods,
invalid instances, unchanged journals and ownership, accepted-but-uncredited funding
and successful fenced restores. Native parser/decoder tests, strict affected-package
Clippy, rustdoc, targeted Wasm checks/builds and the operator PocketIC target pass.
See [client evidence](../evidence/core-primitives.md#read-only-fixture-client-after-0115).
This advances BLOB-12/15/16/17 and A11; no production client or provider is qualified.

Scoped balance observations now bind service/namespace/source/account, persist
intent before dispatch and retain up to sixteen lifetime attempts with one pending
slot. Configuration revisions prevent an old reply from applying even when the same
scope is reinstalled. The existing Caffeine decoder validates independently encoded
response bytes from the local source. Valid zero, malformed amounts, account mismatch,
transport rejection and structured provider failures remain distinct. No provider
request schema or paid call was added; the source method is explicitly a substitute.

Status and the CLI project retained history without effects. Current display requires
the latest configuration and a local age of at most 30 seconds from dispatch, never
from delayed receipt. Clock reversal, expiry, failure and fenced restore hide the
current total while retaining evidence. Balance-read configuration is not installed
billing limits, spendable accounting or payment credit; shared diagnosis keeps the
existing billing/funding/provider blockers. Authority/source journals require the
new records (v1 hard cut, reinstall across releases). Ordinary busy upgrades reject;
forced restores retain pending intents and permanently fence activity. Actual old
journals and live callback upgrades cannot complete or replay restored attempts.
Native record/client tests, strict affected-package Clippy, rustdoc, Wasm builds and
operator/journey/gateway-sync PocketIC checks pass. See
[balance evidence](../evidence/core-primitives.md#scoped-fixture-balance-observations-after-0115).

The final 0.1.16 slice adds shared balance-threshold assessment independent of local
spendability. The existing complete reserve API delegates to that same threshold
logic without changing its released contract. Operator diagnosis consumes partial
balance evidence and adds SpendabilityUnknown rather than computing with zero.
Fixture billing limits validate through FundingLimits and persist their exact
scope/revision in a required record. Reconfiguration invalidates current use without
silently rebinding old limits. Failed, malformed and expired observations remain
distinct; restored owners retain limits but no usable old balance. Status and CLI
reads do not refresh, fund or clear funding/provider/recovery blockers.
Native billing/diagnosis and fixture record checks, strict affected-package Clippy,
warning-free rustdoc, Wasm builds/checks and operator/funding/journey PocketIC targets
pass. See [configured billing evidence](../evidence/core-primitives.md#configured-diagnostic-billing-for-0116).

Balance refresh and gateway sync have explicit commands and independent post-status
outcomes. Funding now has a passive admission preview and an installed local
attachment envelope with atomic reservations, exact-return accounting and a
post-persistence liquidity guard for call costs and installed operating holds.
Next, compose complete admission evidence at the operator funding update boundary,
including exact provider/account credit reconciliation and independent recovery.
First revisit the maintained Caffeine reconciliation inputs: local resource checks
cannot supply those missing provider facts. The raw transfer experiment remains
a test control. Keep unknown recovery, provider credit and spendability
blocking; do not invent positive observations to enable a local funding command.
Gross canister cycles cannot supply missing accounting evidence. Independent
recovery and production provider/account authority remain separate prerequisites.
Keep transport simulator-only until production
bindings are settled. No admission, automatic funding, provider credit inference
or production schema follows from a diagnostic result.
Production namespace/account arrangements and both adapters remain unqualified.
No live paid calls or production deployment contract are authorized.
Keep recovery fenced until independent
authority proves safe identities/accounting. Fixture restoration proves no
reactivation from old stable bytes when post_upgrade runs; it does not
protect whole-canister snapshot loads, which can restore an old heap without that
hook. Stop/start is not reconstruction. Lifecycle hooks cannot qualify reinstall.
The gateway's actual certificate validation and lost-reply completion contract
remain prerequisites for production transports and durable schemas.

Only after those facts and the service contract are settled should implementation
freeze stable schemas, persist intent/reservations, add recovery fences and wire
shared handlers into standalone/Canic adapters. Host memory allocation does not
supply provider atomicity or safe uncertain-effect retries. Keep uncertain work
reserved/fenced; an old counter or empty balance is insufficient evidence.

The [service contract](../service-contract.md) also needs a concrete consumer,
accountable owners and production resource bounds. Paid qualification requires
explicit authority and bounded resources. Caffeine remains the sole target;
preserve the complete Canic replacement obligation. No end-to-end service
capability is qualified. Source integration evidence and actual provider recovery
guarantees must remain distinct.

## Ownership

The maintainer confirmed Canic's 0.110 acceptance for work here without changing
Canic's handoff. Agents must not commit, change versions or infer deployment/
publication authority from continuation. Siblings remain read-only. All Canic
blob capabilities must work here before removal there, with installation
retirement handled separately; see [parity](../canic-parity.md) and
[acceptance](../acceptance-plan.md). Library publication is separately enabled
and does not establish service qualification.
