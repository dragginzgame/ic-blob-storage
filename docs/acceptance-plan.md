<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Service acceptance plan

Qualification follows the [Caffeine probe ledger](evidence/caffeine-probes/README.md).
Public source review, local substitutes and deployed observations are different
evidence classes. Unsupported guarantees require an explicit operating-contract
review. Repeated success is not a universal retry, retention or billing guarantee.
This plan authorizes no deployment or paid trial; full service acceptance is open.

This document defines the evidence required for acceptance. The
[service qualification gaps](service-gaps.md) record which requirements remain
open, while [current status](status/current.md) identifies the active released
baseline and current work.

## Completed isolated development installation — 2026-11-06

The separately selected Blob **0.73.0** standalone trial at
**`j7h7p-gyaaa-aaaaa-qhxwa-cai`** completed two distinct signing-key browser
uploads and exact independent downloads, with cross-uploader refusal. The
[retained result](evidence/caffeine-probes/deployed/2026-11-06-trial-v073-two-users-01/summary.json)
owns the module/configuration, original operations, preserved capacity refusal,
funding scope and continuing obligations. Both lifetime object slots are now
occupied; both references remain live and physical/liability bytes remain 2048.
No release, deletion, billing settlement or application login was qualified.
Do not select this exhausted trial as a fresh application installation or reset
its history. Its expiring payer link does not establish a billing cutoff.

## Retained isolated development installation — 2026-11-03

Separately authorized development-key use and the retained scoped trial allowance
produced fresh mainnet standalone **`4h7vw-4qaaa-aaaau-ahieq-cai`** on Blob **0.69.0**.
[The completed installation record](evidence/caffeine-probes/deployed/2026-11-03-trial-v069-install-01/summary.json)
binds frozen module/init hashes, exact configuration/release/fence/controller
readback and anonymous refusal. Two trial tenants / two lifetime 1KiB objects are
bounded to 2KiB global retained/liability bytes; these are not production defaults.
The [subsequent provider phase](evidence/caffeine-probes/deployed/2026-11-03-trial-v069-provider-01/summary.json)
reconciles two distinct deposits, exact linkage and tenant authority. It also
retains a real first tree PUT 403: payer credit/linkage did not provide owner or
gateway credit. The original certificate/reference and 1KiB possible liability
remain; this upload was never retried.
The [explicitly approved higher allowance](evidence/caffeine-probes/deployed/2026-11-03-trial-v069-allowance-01/summary.json)
kept its original expiry and initially left owner/gateway credit zero. Later
[scoped queries reconcile a delayed 1T allocation](evidence/caffeine-probes/deployed/2026-11-03-trial-v069-bootstrap-01/summary.json).
The distinct unused second tenant then [completed a real 1KiB browser upload and
root-verified native download](evidence/caffeine-probes/deployed/2026-11-03-trial-v069-transfer-01/summary.json),
with exact original bytes. One refused and one successful test-key user do not
qualify two signed-in application users. A later [exact disposable-object release](evidence/caffeine-probes/deployed/2026-11-03-trial-v069-gc-release-01/summary.json)
finds its original receipt and changes only the second reference/liveness to dead.
Logical bytes fall to 1KiB; physical/billing liabilities remain 2KiB. The original
public URL still returns the exact second body after release; no physical deletion
or billing cessation is inferred. Lifetime object capacity remains exhausted. No
allocation cadence/minimum, application login or deletion/billing guarantee is
proved. Keep old 0.6/0.7 installations untouched.
This is Blob-owned provider work independent of Canic's matching-cohort adoption;
it does not select a production application or replace its acceptance run.

## Current evidence and open gates

The current library supplies shared durable owners, tenant policy,
provider bookkeeping, references, trusted registration, verified reads and
operator workflows. Native tools prepare exact inputs and verified snapshots,
send signed requests once and recover original outcomes through inspection.
Browser composition reuses the maintained Caffeine SDK and journal boundary.

Ownership changed on 2026-10-02: this repository owns the library and standalone
host. Downstream frameworks own their wrappers and integration tests externally.
Their dependencies, fixtures and tests are removed here. Older managed test
captures remain historical evidence; they do not qualify a current wrapper.
The root workspace owns dependency versions; consumers must use one compatible
ic-storage identity/runtime. [Current status](status/current.md) owns the released
baseline and active batch. The published Blob 0.73 family requires matching public
dependency identities and fresh backing after retirement; Canic's recorded Blob
0.72.2 composition tests do not qualify this newer graph. The earlier
[framework-removal evidence](evidence/core-primitives.md#independent-library-and-memory-015--2026-10-02)
retains its original 0.15-era dependency identity.

The maintainer accepted the [restricted standalone contract](standalone-trial.md)
on 2026-10-02, then requested configurable sizing for Toko Miner. Current issuance
requires the exact project-approved uploader permission, matching local namespace, an admitted/prepared
reservation within installed resource quotas, a current owner and
atomic durability. Provider spending caps, replay-charge guarantees and operational
old-backup recovery are outside that contract; their evidence remains unqualified.
Breaking API/init/schema changes require a minor release and reinstall-only
adoption; same-release recovery preserves the frozen contract. The
[retained live 0.7.0 trial](evidence/caffeine-probes/deployed/2026-10-02-trial-v070-live-01/summary.json)
qualifies its own 1 KiB/10 MiB upload, independent verification, attestation and
tenant download samples. Its exhausted lifetime capacity and the separate frozen
0.6.0 owner's obligations remain; neither is reset or upgraded by current source
work. These observations do not qualify a current-release deployment, production
retention or final billing. A planning/spending allowance is separate from an
enforced provider bill cap. This plan and ordinary continuation authorize no
new paid effects; the [trial guide](operator-guide.md#isolated-uploaddownload-trial-plan)
describes explicit target selection and preparation.

| Case | Maintained behavior and local evidence | Required qualification |
| --- | --- | --- |
| A01 — authority | Explicit tenant, uploader, operator and provider-namespace bindings; shared handlers and standalone endpoints | Accepted consumer transaction and deployed service identity; controller status grants no tenant access |
| A02 — content | Bounded manifests, atomic retention registration and first reference, streaming blob-root verification on actual reads; historical live samples keep their original contract | Current-release consumer/deployment adoption and asset transaction; registration does not prove uploaded bytes, availability or deployed retention |
| A03 — interruption | Persisted intents, exact historical lookup, uncertainty retention, local IC rollback and actual IndexedDB tab/restart evidence | Provider lost-response/charging semantics, selected browser persistence environment and production consumer outbox |
| A04 — capacity | Bounded objects, references, receipts, sessions, reservations and liabilities; reserved release capacity | Provider pre-charge bounds and production sizing for the selected consumer |
| A05 — restore | Synchronous same-release reopening preserves history with all owners fenced; operator-only current-instance activation uses bounded independent IC history, with old snapshots and incomplete history refused | Consumer adoption and operating-policy qualification; older-backup activation remains unsupported and retained obligations need independent surviving records |
| A06 — release race | Exact reference mutation/receipt/liveness, tombstones, revocation and separate byte accounting | Consumer release coordination, provider deletion and billing cessation |
| A07 — host integration | Framework-free library, shared installation and explicit host grants; standalone Wire application/lifecycle adapters and required provider Candid | Consumer-owned wrapper and integration tests using one runtime identity |
| A08 — economics | Maintained Cashier codecs, local transfer/refund journals, conservative liquidity and attachment checks | Production spendability, complete liabilities and authoritative provider/account evidence |
| A09 — retirement | Logical release, physical deletion, billing cessation and installation retirement are distinct | Per-installation inventory and accountable disposition/settlement; no reset authority from source removal |
| A10 — serving | Reference-qualified descriptors and complete native verified file output; local PNG/JPEG/WebP/GLB bytes, MIME, cache/CORS/CSP checks | Consumer certified-registration transaction, actual production origins/access policy and deployed serving/retention |
| A11 — operator | Signed inspection, account/gateway controls, passive funding assessment, local body verification and verified downloads | Qualified packaged funding and production operator acceptance |
| A12 — consumer acceptance | Common library contract for independently owned wrappers | Concrete application/maintainer, source-bound wrapper tests and actual deployment evidence |

The released [0.41 trusted-registration contract](trusted-registration.md) removes
the mandatory completion verifier, attestation endpoints and worker. Consumers
qualify original certificate/retry identities, exact live references and actual
verified reads. They do not provision a completion signer or treat registration
as proof of availability. Released 0.42 added the narrower [GC callback](gateway-gc-contract.md) and
[funding acknowledgment](funding-credit.md) profiles. Local implementation does
not wait for provider answers; production payer linkage, physical deletion,
credit guarantees and billing retain their separate acceptance boundaries.

## Evidence and release scope

Native tests qualify pure policy and local state behavior. PocketIC qualifies
actual IC calls, lifecycle and transaction rollback under labelled substitutes.
Provider probes retain exact requests, responses, hashes, failures and limitations.
None of these classes substitutes for the others. Historical removed integration
cases are not part of the current test suite.

Library release validation is independent of downstream frameworks. Service
qualification still requires the cases above and the
[service contract](service-contract.md). Removing framework test infrastructure
here does not waive provider, recovery, accounting or retirement requirements.

## Next application acceptance run

### Before the acceptance deployment

Blob's Contracts **0.73.0**, Client **0.73.1** and Engine **0.74.0** are published.
The delivered family selects Wire 0.17.3, Platform 0.23.2, Runtime 0.19.2, Types
0.18.1, Storage 0.68.0 and native Testkit 0.88.0. Source-bound release gates and
registry/archive/tag readbacks passed; [current status](status/current.md) records
the exact evidence and host limits. Cross-release service transitions still use
[retirement and fresh installation](retiring-installations.md), preserving old
provider objects, uncertainty and billing. The live 0.69 and 0.73 trials above keep
their original artifacts; publication did not upgrade either installation.

The **2026-11-06 read-only Canic review**, at unchanged local/default-branch base
`e6a061999dfae207bc20917ffefa2082e7ffe2e7` with active unrelated work, finds all
three locks selecting Engine 0.72.2 / Contracts 0.72.0, Storage 0.67.1 and Wire
0.17.2. They now contain Platform 0.23.2 and an older 0.22.3 identity; native
Testkit remains 0.86.0. Manifest/lock review alone does not establish the deployed
feature closure or an incompatible type exchange. Blob 0.73's external certificate
reply retains Candid bytes and its Rust path, but removes its unused host Wire
metadata implementation; the managed consumers must qualify their generated
clients against the selected release rather than add a replacement DTO.

Canic's handoff reports passing November 5 dedicated/embedded composition checks
including all five embedded funding profiles. Those results bind an earlier root
lock, not the current locks, and their named `target/upstream-refresh-20261105/`
directory is unavailable in the reviewed checkout. This review cannot independently
revalidate those raw results. The separately ignored
`pic::blob_service::tests::managed_browser_compositions` still needs explicit
qualification on the selected final graph. Delivered adapter 0.115.0, pending
adoption and actual application acceptance remain distinct under
[Canic #622](https://github.com/dragginzgame/canic/issues/622);
[#494](https://github.com/dragginzgame/canic/issues/494) owns stronger credit/replay.

Both checked-in Blob consumers select only local environments and disable
delegated-token Auth. Neither demo/test application selects a Blob role; no actual
application origin/network/Blob-service binding was identified. Those inputs and
application login belong to Canic/the application, not a missing Blob adapter.

Recorded November 2/3 Canic composition and dependency-conflict observations
retain their original graphs in the earlier handoff/evidence. They do not establish
a current defect or qualify the new 0.74 Engine/Storage identities. This review
did not edit or compile Canic, run its private consumers or infer a production
capacity profile from fixture limits.

Deployment becomes the next step after the remaining owner actions:

1. Canic adopts the selected published Engine 0.74 / Contracts 0.73 family in its
   three maintained catalogs/locks, admits one runtime/type/Wire identity and
   qualifies its dedicated and embedded installation/recovery, funding profiles
   and generated clients.
   Its owner then delivers the selected change with applicable CI/publication
   acceptance. Prior composition checks keep their own inputs; this public-identity
   cut needs current evidence, without duplicating the adapter or its fixtures.
2. The application selects that delivered graph and dedicated or embedded
   mounting/lifecycle path. Framework and application changes remain with their
   owners; do not copy an adapter or application transaction into Blob.
3. Canic/the consuming application's owner supplies a bounded acceptance-service
   configuration and its network/role binding, including finite limits, namespace,
   project/payer relationship and operator. The maintainer has confirmed Caffeine
   as provider; that does not select a deployed Blob instance. Resolve its service
   principal from the selected fleet's registry after deployment. Historical trial
   principals, fixture keys and fixture limits are not production defaults.

At that point deploy the selected acceptance installation and record its actual
module hash and configuration before provider traffic. Use a fresh installation
for this cross-release cut; preserve any previous installation's provider objects,
balances, uncertain effects and billing until its retirement contract is satisfied.
Existing local fixture limits are not an approved production capacity profile.
The separate probe intent records the exact provider operations and approved
spending allowance before uploads or top-ups; deployment readiness does not
authorize those paid effects automatically.

Do not wait for #5/#31 live evidence before this deployment: that would make
acceptance circular. Deployment enables their run below. Deletion/billing (#6)
and payer reconciliation (#34) remain separate observations afterward, rather
than a requirement to invent a new receipt API before the supported happy path.
Record unknown provider outcomes as unknown and retain their obligations.

A separately selected provider-only trial can precede an application frontend.
The completed 0.73 run below establishes its own upload/read/funding evidence;
neither occupied trial is a fresh acceptance service. A provider-only run does
not qualify application login, CSP or media-map publication. The full
#31/#5 run below additionally records the real application origin, two signed-in
users and a frozen media inventory. Do not make that later UI selection an
artificial prerequisite for an otherwise prepared provider trial.

[Canic #494 — Guarded funding](https://github.com/dragginzgame/canic/issues/494)
separately retains delivery and deferred independent-credit/receipt acceptance.
Independent credit proof is not required after each matching acknowledged
payment. Missing, malformed or accepted-error outcomes keep their original
uncertainty and cannot be repaired by resending. No Canic source or deployment
configuration was changed from this repo.

### Execute against the recorded installation

The separately selected **November 6 provider-only 0.73 run completed** on a fresh
standalone service: [two distinct signing-key uploaders and exact downloads](evidence/caffeine-probes/deployed/2026-11-06-trial-v073-two-users-01/summary.json),
with actual Caffeine funding/linkage and both cross-uploader refusals. All 2048
original bytes match. Original profiles, one certified capacity refusal and its
explicit serial re-evaluation remain. Both first references and 2 KiB provider/billing
liabilities are retained; no deletion/settlement or change to the 0.69 trial occurred.
This advances #31's provider cell and #5's small binary-delivery cell. It does not
qualify the actual application's signed-in sessions, media transaction or CSP.
The application-owner steps above remain; repeat neither paid effects nor fixture
scaffolding simply to relabel the provider sample as application acceptance.

Use one released graph and the application's real mounted service. Record the
application commit, three Canic lock hashes where applicable, installed Wasm hash,
Blob version, service/tenant/namespace, payer and public actor principals before
execution. Keep private keys, certificates and signed envelopes out of public
issues. Retain sensitive recovery inputs in the operator's controlled evidence
store. Record a separate intent and approved budget under the probe ledger before
any provider effect; the checklist itself grants no deployment or payment authority.

| Owning issue | Smallest useful run and evidence | Stop/refusal and completion condition |
| --- | --- | --- |
| [#31: two application users](https://github.com/dragginzgame/ic-blob-storage/issues/31) | Two distinct signed-in users obtain separate project-authorized permissions and upload distinct files to the same service. Retain original operation, root, uploader and first-reference identities; verify downloaded bytes against each original file. | Wrong user, tenant/service, expired and revoked permissions must refuse before exposure. Recover lost output from the original operation; never issue another certificate to repair uncertainty. Complete only with actual application-session and deployed-provider evidence. |
| [#5: application media-map transaction](https://github.com/dragginzgame/ic-blob-storage/issues/5) (consolidated from #4) | Use a frozen media inventory and the same exact file/reference identities. Retain the complete map and the application's authenticated asset-registration outcome. Interrupt transfer, host process and frontend mapping publication separately; recover original outcomes before publishing the complete map. | Missing files or unresolved publication prevent advertising the map. A live first reference does not prove provider bytes, and sequential observations are not an atomic publication lease. Retrying the application transaction must not repeat an uncertain upload or create a duplicate asset effect. |
| [#5: serving and integrity](https://github.com/dragginzgame/ic-blob-storage/issues/5) | Use those files from the real application origin with its CSP. Retain final URL, MIME, CORS/cache headers, cold/warm reads, byte length and verified content result. Exercise representative image/model loading and bounded corrupt/truncated/failing responses in a local substitute. | Do not advertise an asset solely because registration succeeded. Public URL accessibility after logical release must not be described as authenticated access or deletion. Local failure injection and deployed successful reads remain separately labelled. |
| [#6: reference lifecycle and provider settlement](https://github.com/dragginzgame/ic-blob-storage/issues/6) | Reuse an object in overlapping application releases. Interrupt retain/release and recover the exact receipts; remove media and exercise supported same-release restoration. Record lifetime capacity, remaining references, provider-object custody and continuing liabilities. Separately retain authenticated deployed deletion and billing-cessation evidence. | A logical release or GC acknowledgment clears neither physical bytes nor billing liability. Exhausted lifetime capacity and permanently claimed retired roots must refuse rather than reset history. Missing/trapping callbacks or lost confirmations require original-object reconciliation; cross-release changes use retirement/reinstall. |
| [#34: payer and funding](https://github.com/dragginzgame/ic-blob-storage/issues/34) | Establish authenticated project-payer/service linkage, limits and expiry; retain provider relationship and activity evidence. Under an explicit budget, inspect two distinct acknowledged payments and their original callback refunds, spent totals and remaining allowance. | An unresolved accepted payment stops further dispatch. A lost host reply requires original-outcome inspection, never resend. Independent credit confirmation requires attributable provider evidence, not a balance delta. Complete only when real linkage, competing activity and spendability are established. |

A concrete consumer of #5's publication transaction (formerly #4) still needs its
own protocol migration before this run:
read-only IC Crates review at local base `3c67db3` finds `completion_verifier` in
`RegistryConfiguration`, attestation-based catalog publication, and worker calls
to retired `observe-upload`/`submit-attestation` commands. That is consumer work
under [IC Crates #1](https://github.com/dragginzgame/ic-crates/issues/1), not a
missing Blob completion API. Follow [the trusted-registration migration](trusted-registration.md#consumer-changes):
retain exact permissions and live references, verify actual archive bytes and the
consumer's Cargo checksum, then perform the authenticated catalog transaction.
Neither registration nor a live reference substitutes for that checksum/availability
check. Preserve historical jobs with their original tools; add no dual reader or
automatic cross-release migration. This source review executes no consumer run.

Existing tooling is sufficient for preparation: the operator guide describes
[offline account-link inputs](operator-guide.md#generate-account-link-inputs-offline),
`inspect-account --kind relationship`, `funding-history` and `funding-outcome`.
Application login, asset publication and scheduling remain consumer-owned. Do not
copy them into Blob or introduce a receipt API merely to fill an evidence gap.

Local references are the maintained two-user case in
[standalone browser tests](../tests/pocketic/tests/standalone_browser/multi_user/mod.rs),
[corruption and interruption cases](../tests/pocketic/tests/standalone_browser/mod.rs),
and the [funding qualification recipe](funding-consumer-qualification.md).
They establish expected local behavior, not results for the application run above.
The opt-in `make test-browser-standalone` requires the exact Node/npm selections,
installed browser packages/Chromium and admitted Testkit; it downloads nothing.
The target selects the complete `standalone_browser::` module, including the
lost-final-reply/withdrawal case whose name no longer has the former trial prefix.
A failed prerequisite check is not a failed upload or a passing browser test.
The 2026-10-24 follow-up passed Node/npm admission outside the execution sandbox;
the earlier empty npm version was environment-specific. Do not install replacement
tools or bypass the version check solely on that earlier failure.

Attach the evidence location, exact source/graph, observed refusals, unresolved
operations and cleanup/continuing liabilities to each owning issue. Leave an item
open when its consumer/provider cells are missing; no aggregate percentage or
library publication substitutes for those cells.
