# Current status

Date: 2026-09-26

## Released baseline

The maintainer pushed 0.1.12. Local main, origin/main and annotated tag v0.1.12
point to `806407e`, from source `e125fb803d28b8215e7ec0ed294043f925eb82f0`.
Cargo and the release receipt are 0.1.12; `make release-tag-check` passes and
source was clean before the review below. Registry publication was not queried.
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

## Current follow-up — provider contract review

The maintainer requested the provider review as the next batch. The completed
[contract decision](../provider-review.md#contract-decision-after-0112) and
[refresh record](../evidence/caffeine-contract-refresh.json) distinguish current
source facts, anonymous deployment observations and unresolved server semantics.
The investigation is now accompanied by an unpublished funding fixture and
targeted PocketIC evidence, recorded in the 0.1.13 changelog draft. Cargo and the
release receipt remain 0.1.12; no release operation has run.
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

Provider questions in the review were narrowed by the new primary source; none
was sent. The targeted funding experiment passes. Next incorporate its separated
transport/refund observations into the funding contract, preserving uncertainty
after callback loss, and qualify the selected current wire contract. Remaining
provider gaps are:

1. Apply documented self-account/linked-payer patterns to the exact installation
   and existing Toko obligations. Verify the current blob deletion-list Candid
   with the deployed gateway; do not infer namespace authority from defaults.
2. Upload operation identity, retry charging, lost-reply completion lookup,
   incomplete-object behavior and numeric evidence-retention bounds.
3. Exact top-up/ledger credit and refund reconciliation after lost replies.
4. Object-specific deletion/final billing proof and authority surviving restore.

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
