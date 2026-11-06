<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Service qualification gaps — 2026-11-06

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

The released 0.42 acknowledgment/GC profile is described in
[current status](status/current.md). The [current application handoff](acceptance-plan.md#next-application-acceptance-run)
records the November 6 read-only Canic review; historical local composition proofs
keep their original dependency graphs.

| Gap | Repository result | Remaining acceptance and owner |
| --- | --- | --- |
| Framework adapter and composition | November 6 source/lock review finds Engine 0.72.2 / Contracts 0.72.0 in all three Canic locks, preceding published Blob 0.73. Its handoff reports dedicated/embedded checks on an earlier lock; the named raw-output directory is unavailable here. Adapter 0.115.0 is delivered; this review neither rebuilt nor requalified its dirty checkout. | Canic #622 owns final-graph composition, five funding profiles and explicitly selected generated-browser-client acceptance. Both consumers are local-only with delegated-token Auth disabled; actual application/network/service binding and signed-in publication remain unselected. #494 owns stronger credit/replay, separately from acknowledged funding. |
| Current standalone mainnet installation | The [fresh 0.73 installation](evidence/caffeine-probes/deployed/2026-11-06-trial-v073-install-01/summary.json) has exact module/configuration/release/controller/fence readback and anonymous refusal. The 0.69/older owners remain untouched. | Fresh payer linkage and both current live uploads/root-verified downloads pass below. Both lifetime object slots are occupied. This bounded development installation is not a fresh application acceptance service, Canic composition, login, production capacity or billing qualification. |
| Current payer/owner/gateway readiness | The [isolated 0.73 provider run](evidence/caffeine-probes/deployed/2026-11-06-trial-v073-provider-01/summary.json) reconciles one 1.001T gross external deposit / 1.0008T credit, exact expiring relationship, separate 1T owner allocation and positive advertised-gateway credit. Two actual verified transfers establish spendability for this sample. | Blob #34 retains original-operation credit/replay and complete competing-activity evidence. No fixed allocation cadence/minimum or necessity of the 5T increase is established. Preserve both zero and positive observations; do not infer readiness from payer/link alone. |
| Complete live upload/download | The [0.73 two-key provider run](evidence/caffeine-probes/deployed/2026-11-06-trial-v073-two-users-01/summary.json) completes both original 1 KiB Chromium uploads and independent root/byte-verified downloads, cross-uploader refusals and two live first references. Actual GET MIME/length/CORS/cache headers are retained; no application login/CSP/catalog claim follows. | #31 still needs two successful signed-in application users; #5 needs actual media/publication/serving/CSP acceptance. One successful test-key transfer does not prove future retention or billing. The bounded installation's two lifetime object slots are occupied; never reset to gain capacity. |
| Current-instance restoration | Released 0.8.0 adds operator-only recovery from an independent replicated IC `canister_info` call, checked against the immutable platform installation version. PocketIC qualifies activation with retained obligations and rejection of stale/incomplete history. | Consumer owner: qualify the released contract in its composition and operating policy. Frozen live 0.7.0 does not acquire this endpoint. |
| Older snapshot/backup restoration | Snapshot loads and history that cannot reach installation refuse active recovery. The standalone guard fences management-version gaps, including old heap restoration. | Operator: retain the surviving complete inventory of provider objects, uncertain effects, balances and billing outside any rolled-back installation. Active older-backup recovery remains unsupported. |
| Provider deletion | Released 0.42 mounts the tree-root GC profile from the shared catalog. The [0.69 live release](evidence/caffeine-probes/deployed/2026-11-03-trial-v069-gc-release-01/summary.json) finds the exact original receipt, preserves first/unknown roots and duplicate positions, and reports the second object `DeletionPending`; its public URL still returns the original bytes. No gateway acknowledgment was impersonated. The [November 5 readback](evidence/caffeine-probes/deployed/2026-11-05-trial-v069-gc-readback-01/summary.json) still finds zero recorded acknowledgment, `DeletionPending`, unchanged liabilities and HEAD 200. | #6: independently attributable real callback/deletion evidence and consumer interruption/recovery remain. Public availability may be cached, and a later 404 would not alone prove physical deletion. Retain physical accounting. |
| Billing cessation | The current 0.69 release reduces logical bytes from 2048 to 1024 while physical/liability bytes stay 2048. Queried owner balance is unchanged and gateway usage returns zero; neither establishes no billing. Historical larger samples keep their original scope. | #6: exact final object billing cutoff/settlement and delayed-charge treatment remain unproved. Logical release, gateway acknowledgment, missing downloads or zero counters cannot clear liability. |
| Existing consumers | The [service contract](service-contract.md) defines the current format and lifetime rules. Earlier Canic adapter publication and Toko Miner's reported registry-only disposable acceptance are recorded in the [historical delivery review](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/canic0229.md); siblings remain read-only. | Application owners: qualify their production installation, tenant/payer bindings and provider activity, and explicitly disposition old installations and obligations. Toko's per-project journey remains separate from Toko Miner. No compatibility layer or cross-release migration is supplied. |

The [next application acceptance run](acceptance-plan.md#next-application-acceptance-run)
turns #31/#5/#34 into explicit inputs, observations and stop conditions without
adding another engine or authorizing live effects.

## Recent local evidence

The released [funding acknowledgment contract](funding-credit.md) permits distinct
bounded payments after matching successful Cashier callbacks. Independent credit
proof is no longer a happy-path prerequisite. Uncertain responses and accepted
provider errors remain blocked. Consumers own guarded dispatch composition and
deployed payer/account/spendability qualification. Acknowledgment is not proof of
credit; local provider substitutes do not qualify production economics.

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
then fresh signed queries recover the map with registration/reference history
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
