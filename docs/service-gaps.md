# Service qualification gaps — 2026-10-02

This review stays within this repository. The core and standalone host have no
Canic dependency, endpoint registration or lifecycle ownership on linkage.
Framework wrappers and their composition tests belong to their consumer owners.

| Gap | Repository result | Remaining acceptance and owner |
| --- | --- | --- |
| Framework adapter and composition | Existing native memory-composition and lifecycle-binding tests exercise the core boundary. No downstream adapter, fixture or dependency is added. | Canic owner: use one ic-memory identity/runtime, explicit grants and bindings, synchronous restoration, caller guards and exact Candid replies in actual PocketIC composition tests. |
| Complete live upload/download | The [fresh frozen 0.7.0 record](evidence/caffeine-probes/deployed/2026-10-02-trial-v070-live-01/summary.json) completes 1 KiB and 10 MiB uploads, independent verification, accepted attestations and matching tenant downloads. | Consumer owner: qualify distinct real media, complete publication batches, interruption, serving and CSP. These samples do not prove retention or itemized billing. |
| Current-instance restoration | Unreleased adds operator-only recovery from an independent replicated IC `canister_info` call, checked against the immutable platform installation version. PocketIC qualifies activation with retained obligations and rejection of stale/incomplete history. | Library/host maintainer: a future minor release and consumer adoption. Frozen live 0.7.0 does not acquire this endpoint. |
| Older snapshot/backup restoration | Snapshot loads and history that cannot reach installation refuse active recovery. The standalone guard fences management-version gaps, including old heap restoration. | Operator: retain the surviving complete inventory of provider objects, uncertain effects, balances and billing outside any rolled-back installation. Active older-backup recovery remains unsupported. |
| Provider deletion | Final-reference release retains physical bytes. Core deletion acknowledgement and billing settlement remain separate transitions. Reviewed public Caffeine callbacks do not establish an executed live deletion. | Provider/service owner: qualify exact-object authority, deletion request/callback identity, uncertain-result reconciliation and authoritative deployed deletion evidence before decrementing physical accounting. |
| Billing cessation | Logical release or a provider deletion callback cannot clear economic liability. Fresh live samples retain 10,486,784 physical/liability bytes after logical release. | Provider/service owner: obtain exact final object billing evidence, settlement amount/cutoff, and treatment of delayed charges. Current reviewed contracts and zero usage counters supply no such proof. |
| Existing consumers | The [integration backlog](canic-parity.md#integration-feedback) records the required hard cut; siblings remain read-only. | Toko/Canic owners: remove direct Canic blob and billing APIs, adopt the released shared contract and explicitly disposition old installations and obligations. No compatibility layer or cross-release migration is supplied. |

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

The [probe ledger](evidence/caffeine-probes/README.md) distinguishes source review,
local IC execution, substitutes and deployed observations. Missing provider
contracts are recorded gaps, not reasons to infer success or add speculative
callback APIs. Consumers must run their own wrapper qualification after the next
released hard cut; package publication does not establish service qualification.
