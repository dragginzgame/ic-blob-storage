# Current status

Date: 2026-09-26

## Released baseline

The maintainer pushed 0.1.14. Local main, origin/main and annotated tag v0.1.14
point to `3c0e00a`, from source `3f73703cff3914a45b4a60fea05a030ed76550d7`.
Cargo and the release receipt are 0.1.14; the worktree was clean when this release
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
evidence; old-backup fencing and production service persistence remain
unimplemented.

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
Changes remain Unreleased at Cargo 0.1.14. The continued journey now reserves
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
gateway tests, strict Clippy, rustdoc, Wasm and formatting pass. This archive lacks
streaming SHA state and is not a resumable checkpoint; authority upgrades still
reject in both hooks. No product stable schema or recovery authority was added.

Next design a lossless same-release verification checkpoint and reconstruct the
authority's catalogs/receipts and pending operations without resetting uncertainty.
Do not treat archived prefix counters or verdicts as resumable hash state, or
rebuild empty catalogs beside retained obligations. Keep recovery fenced until an
independent authority proves safe identities/accounting. Current source restoration
proves no reactivation from old stable bytes when post_upgrade runs; it does not
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
