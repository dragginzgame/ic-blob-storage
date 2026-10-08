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
provider bookkeeping, references, verifier completion, read descriptors and
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
| A01 — authority | Explicit tenant, uploader, operator and verifier bindings; shared handlers and standalone endpoints | Accepted consumer transaction and deployed service identity; controller status grants no tenant access |
| A02 — content | Bounded manifests, streaming blob-root verification and exact verifier receipts; retained 0.7.0 live samples and matching released-tool local media journeys | Current-release consumer/deployment adoption; accepted empty-object/serving limits and deployed retention |
| A03 — interruption | Persisted intents, exact historical lookup, uncertainty retention, local IC rollback and actual IndexedDB tab/restart evidence | Provider lost-response/charging semantics, selected browser persistence environment and production consumer outbox/worker |
| A04 — capacity | Bounded objects, references, receipts, sessions, reservations and liabilities; reserved release capacity | Provider pre-charge bounds and production sizing for the selected consumer |
| A05 — restore | Synchronous same-release reopening preserves history with all owners fenced; operator-only current-instance activation uses bounded independent IC history, with old snapshots and incomplete history refused | Consumer adoption and operating-policy qualification; older-backup activation remains unsupported and retained obligations need independent surviving records |
| A06 — release race | Exact reference mutation/receipt/liveness, tombstones, revocation and separate byte accounting | Consumer release coordination, provider deletion and billing cessation |
| A07 — host integration | Framework-free library, shared installation and explicit host grants; standalone Candid/lifecycle | Consumer-owned wrapper and integration tests using one runtime identity |
| A08 — economics | Maintained Cashier codecs, local transfer/refund journals, conservative liquidity and attachment checks | Production spendability, complete liabilities and authoritative provider/account evidence |
| A09 — retirement | Logical release, physical deletion, billing cessation and installation retirement are distinct | Per-installation inventory and accountable disposition/settlement; no reset authority from source removal |
| A10 — serving | Reference-qualified descriptors and complete native verified file output; local PNG/JPEG/WebP/GLB bytes, MIME, cache/CORS/CSP checks | Consumer certified-registration transaction, actual production origins/access policy and deployed serving/retention |
| A11 — operator | Signed inspection, account/gateway controls, passive funding assessment and verifier tooling | Qualified packaged funding and production operator acceptance |
| A12 — consumer acceptance | Common library contract for independently owned wrappers | Concrete application/maintainer, source-bound wrapper tests and actual deployment evidence |

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
