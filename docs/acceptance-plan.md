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
ic-memory identity/runtime. [Current status](status/current.md) owns the released
baseline and active batch. The earlier
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
| [#4: application media-map transaction](https://github.com/dragginzgame/ic-blob-storage/issues/4) | Use a frozen media inventory and the same exact file/reference identities. Retain the complete map and the application's authenticated asset-registration outcome. Interrupt transfer, host process and frontend mapping publication separately; recover original outcomes before publishing the complete map. | Missing files or unresolved publication prevent advertising the map. A live first reference does not prove provider bytes, and sequential observations are not an atomic publication lease. Retrying the application transaction must not repeat an uncertain upload or create a duplicate asset effect. |
| [#5: serving and integrity](https://github.com/dragginzgame/ic-blob-storage/issues/5) | Use those files from the real application origin with its CSP. Retain final URL, MIME, CORS/cache headers, cold/warm reads, byte length and verified content result. Exercise representative image/model loading and bounded corrupt/truncated/failing responses in a local substitute. | Do not advertise an asset solely because registration succeeded. Public URL accessibility after logical release must not be described as authenticated access or deletion. Local failure injection and deployed successful reads remain separately labelled. |
| [#6: reference lifecycle and provider settlement](https://github.com/dragginzgame/ic-blob-storage/issues/6) | Reuse an object in overlapping application releases. Interrupt retain/release and recover the exact receipts; remove media and exercise supported same-release restoration. Record lifetime capacity, remaining references, provider-object custody and continuing liabilities. Separately retain authenticated deployed deletion and billing-cessation evidence. | A logical release or GC acknowledgment clears neither physical bytes nor billing liability. Exhausted lifetime capacity and permanently claimed retired roots must refuse rather than reset history. Missing/trapping callbacks or lost confirmations require original-object reconciliation; cross-release changes use retirement/reinstall. |
| [#34: payer and funding](https://github.com/dragginzgame/ic-blob-storage/issues/34) | Establish authenticated project-payer/service linkage, limits and expiry; retain provider relationship and activity evidence. Under an explicit budget, inspect two distinct acknowledged payments and their original callback refunds, spent totals and remaining allowance. | An unresolved accepted payment stops further dispatch. A lost host reply requires original-outcome inspection, never resend. Independent credit confirmation requires attributable provider evidence, not a balance delta. Complete only when real linkage, competing activity and spendability are established. |

A concrete #4 consumer still needs its own protocol migration before this run:
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
