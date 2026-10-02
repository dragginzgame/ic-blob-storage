# Service acceptance plan

Qualification follows the [Caffeine probe ledger](evidence/caffeine-probes/README.md).
Public source review, local substitutes and deployed observations are different
evidence classes. Unsupported guarantees require an explicit operating-contract
review. Repeated success is not a universal retry, retention or billing guarantee.
This plan authorizes no deployment or paid trial; full service acceptance is open.

## Current local evidence and open gates — 2026-10-02

The released 0.5.0 library supplies shared durable owners, tenant policy,
provider bookkeeping, references, verifier completion, read descriptors and
operator workflows. Native tools prepare exact inputs and verified snapshots,
send signed requests once and recover original outcomes through inspection.
Browser composition reuses the maintained Caffeine SDK and journal boundary.

Ownership changed on 2026-10-02: this repository owns the library and standalone
host. Downstream frameworks own their wrappers and integration tests externally.
Their dependencies, fixtures and tests are removed here. Older managed test
captures remain historical evidence; they do not qualify a current wrapper.
The public ic-memory growth API is now 0.15.0. See the
[latest implementation evidence](evidence/core-primitives.md#independent-library-and-memory-015--2026-10-02).

The maintainer accepted the [restricted standalone contract](standalone-trial.md)
on 2026-10-02, then requested configurable sizing for Toko Miner. Current issuance
requires explicit uploader trust, matching local namespace, an admitted/prepared
reservation within installed resource quotas, a current owner and
atomic durability. Provider spending caps, replay-charge guarantees and operational
old-backup recovery are outside that contract; their evidence remains unqualified.
The public API/init/schema hard cut requires a minor release. No isolated live
trial service, funded account, provider terms or paid-effect authority is selected.
The 100T-cycle planning ceiling is separate from an enforced maximum bill.
The [trial plan](operator-guide.md#isolated-uploaddownload-trial-plan) prepares
inputs but does not authorize provider effects.

| Case | Maintained behavior and local evidence | Required qualification |
| --- | --- | --- |
| A01 — authority | Explicit tenant, uploader, operator and verifier bindings; shared handlers and standalone endpoints | Accepted consumer transaction and deployed service identity; controller status grants no tenant access |
| A02 — content | Bounded manifests, streaming root verification and exact verifier receipts | Real certificate/upload path, independent whole-body fetch and accepted empty-object/serving limits |
| A03 — interruption | Persisted intents, exact historical lookup, uncertainty retention, local IC rollback and actual IndexedDB tab/restart evidence | Provider lost-response/charging semantics, selected browser persistence environment and production consumer outbox/worker |
| A04 — capacity | Bounded objects, references, receipts, sessions, reservations and liabilities; reserved release capacity | Provider pre-charge bounds and production sizing for the selected consumer |
| A05 — restore | Synchronous same-release reopening preserves history with all owners fenced | Complete surviving obligations and independent freshness authority before operational restart |
| A06 — release race | Exact reference mutation/receipt/liveness, tombstones, revocation and separate byte accounting | Consumer release coordination, provider deletion and billing cessation |
| A07 — host integration | Framework-free library, shared installation and explicit host grants; standalone Candid/lifecycle | Consumer-owned wrapper and integration tests using one runtime identity |
| A08 — economics | Maintained Cashier codecs, local transfer/refund journals, conservative liquidity and attachment checks | Production spendability, complete liabilities and authoritative provider/account evidence |
| A09 — retirement | Logical release, physical deletion, billing cessation and installation retirement are distinct | Per-installation inventory and accountable disposition/settlement; no reset authority from source removal |
| A10 — serving | Reference-qualified descriptors and complete native verified file output | Real upload/download consumer flow, confidentiality, MIME and public-serving acceptance |
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
