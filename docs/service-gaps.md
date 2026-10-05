<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Service qualification gaps — 2026-10-02

This review stays within this repository. The core and standalone host have no
Canic dependency, endpoint registration or lifecycle ownership on linkage.
Framework wrappers and their composition tests belong to their consumer owners.

The [acceptance plan](acceptance-plan.md) defines the target evidence. This file
records the gaps that remain open; [current status](status/current.md) owns the
active release and implementation handoff.

<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-qualification-layers.svg" alt="Four qualification layers covering native checks, PocketIC evidence, live provider evidence and consumer deployment acceptance" width="800">
</p>

## Open gaps

| Gap | Repository result | Remaining acceptance and owner |
| --- | --- | --- |
| Framework adapter and composition | Existing native memory-composition and lifecycle-binding tests exercise the core boundary. No downstream adapter, fixture or dependency is added. | Canic owner: use one ic-memory identity/runtime, explicit grants and bindings, synchronous restoration, caller guards and exact Candid replies in actual PocketIC composition tests. |
| Complete live upload/download | The [fresh frozen 0.7.0 record](evidence/caffeine-probes/deployed/2026-10-02-trial-v070-live-01/summary.json) completes 1 KiB and 10 MiB uploads, independent verification, accepted attestations and matching tenant downloads. | Consumer owner: qualify distinct real media, complete publication batches, interruption, serving and CSP. These samples do not prove retention or itemized billing. |
| Current-instance restoration | Released 0.8.0 adds operator-only recovery from an independent replicated IC `canister_info` call, checked against the immutable platform installation version. PocketIC qualifies activation with retained obligations and rejection of stale/incomplete history. | Consumer owner: qualify the released contract in its composition and operating policy. Frozen live 0.7.0 does not acquire this endpoint. |
| Older snapshot/backup restoration | Snapshot loads and history that cannot reach installation refuse active recovery. The standalone guard fences management-version gaps, including old heap restoration. | Operator: retain the surviving complete inventory of provider objects, uncertain effects, balances and billing outside any rolled-back installation. Active older-backup recovery remains unsupported. |
| Provider deletion | Final-reference release retains physical bytes. Core deletion acknowledgement and billing settlement remain separate transitions. Reviewed public Caffeine callbacks do not establish an executed live deletion. | Provider/service owner: qualify exact-object authority, deletion request/callback identity, uncertain-result reconciliation and authoritative deployed deletion evidence before decrementing physical accounting. |
| Billing cessation | Logical release or a provider deletion callback cannot clear economic liability. Fresh live samples retain 10,486,784 physical/liability bytes after logical release. | Provider/service owner: obtain exact final object billing evidence, settlement amount/cutoff, and treatment of delayed charges. Current reviewed contracts and zero usage counters supply no such proof. |
| Existing consumers | The [service contract](service-contract.md) defines the current format and lifetime rules; siblings remain read-only. | Toko/Canic owners: remove direct Canic blob and billing APIs, adopt the released shared contract and explicitly disposition old installations and obligations. No compatibility layer or cross-release migration is supplied. |

## Recent local evidence

The [local multi-chunk/reference journeys](evidence/caffeine-probes/README.md#multi-chunk-png-and-overlapping-references--2026-10-04)
verify distinct full/partial PNG chunks and signed overlapping-reference cleanup.
They establish local format/reference/accounting behavior, not consumer asset
transactions or deployed provider acknowledgments, public serving or retirement.

The [direct browser-delivery checks](evidence/caffeine-probes/README.md#direct-browser-url-delivery--2026-10-04)
add local PNG MIME/bytes and selected-origin CORS observations using the canonical
native URLs. The saved public URL still serves after logical reference release
under the substitute, while authenticated descriptors refuse. CORS is a browser
read policy, not confidentiality or provider deletion. Deployed cache/CSP,
retention and consumer access-policy acceptance remain open.

The [local image/CSP checks](evidence/caffeine-probes/README.md#png-image-loading-under-csp--2026-10-04)
exercise ordinary anonymous images, readable canvas pixels and explicit policy
refusal with zero provider requests. They qualify only the authored local policy
and substitute. Consumer image/fetch origins, real assets and deployed CSP/cache/
retention still need acceptance; image decoding does not replace independent
whole-content verification or establish public-access revocation.

The [representative emitted-media record](evidence/caffeine-probes/README.md#representative-emitted-media--2026-10-04)
adds local PNG/JPEG/WebP and an eight-chunk GLB from frozen consumer bytes,
without a consumer build or framework wrapper. Lost replies add no PUT; tail
corruption prevents the next admission. An occupied native map output refuses,
then fresh signed queries recover the map with completion/reference history
intact. This does not qualify an application asset-registration transaction.
The [original-cache metadata checks](evidence/caffeine-probes/README.md#original-cache-metadata--2026-10-04)
now reproduce all four retained original roots through the same SDK preparation.
Local cached completion/recovery and browser headers pass; deployed cache operation
and complete consumer publication remain unqualified. The
[selected-source installation recipe](local-tools.md#install-the-native-and-browser-tools)
now has a fresh-prefix locked CLI/handoff check and an
[isolated-source qualification](evidence/caffeine-probes/README.md#isolated-source-tool-installation--2026-10-04)
with fresh npm dependencies, no Git metadata and matching local media completion/
recovery. A separate [clean-release recipe check](evidence/caffeine-probes/README.md#clean-released-source-tool-installation--2026-10-04)
now qualifies tagged 0.14.3 source, fresh installed tools and matching local media
completion/recovery. Its prefix and retained Cargo/platform tools belong to this
repository; consumer-owned installation, adoption and asset registration remain
open. The [fixture deadline follow-up](evidence/caffeine-probes/README.md#publication-fixture-session-deadline--2026-10-04)
uses one explicit session bound across original/recovery, browser and parent,
without changing production timeouts or provider retry authority.

## Deletion and billing qualification

Deletion qualification must retain the exact original service, namespace, object,
root, incarnation and operation identity before any destructive effect. Record
the request, callback/result, hashes and uncertain outcome; never redispatch from
expired evidence. Exercise delayed/duplicate/mismatched callbacks locally, then
capture the selected deployed provider outcome under a separately recorded intent
and budget. A missing download alone is not a deletion or billing receipt.

Billing qualification needs its own authoritative evidence after deletion, with a
final charge boundary and treatment of obligations already incurred. Check local
logical, physical and liability accounting independently before and after each
transition. Keep both old and new trial owners and their original histories until
retirement obligations are satisfied; do not reset exhausted lifetime capacity.

## Consumer responsibility

The [probe ledger](evidence/caffeine-probes/README.md) distinguishes source review,
local IC execution, substitutes and deployed observations. Missing provider
contracts are recorded gaps, not reasons to infer success or add speculative
callback APIs. Consumers must run their own wrapper qualification after the next
released hard cut; package publication does not establish service qualification.
