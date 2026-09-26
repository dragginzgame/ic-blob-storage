# Current status

Date: 2026-09-26

## Released baseline

The maintainer reports 0.1.6 pushed. Local release/tag is `0992929`, from source
`d3ca8c472cacca2715543eeb033446eb8ca2bfed`; Cargo and the receipt are 0.1.6.
`make release-tag-check` passes. Registry publication was not independently
queried. Release/publication preserve artifacts; only explicit `make clean`
removes them. See [release guidance](../releasing.md).

The library provides content/provider identities, incremental raw-byte
verification, bounded root batches, billing/configuration validation and pure
funding/readiness/tenant policy. Its transient lifecycle separates reference
release, physical deletion and billing settlement, with explicit ownership
bindings and bounded exact-request receipts reserving capacity for release.
Immutable service-wide root claims reject reassignment even after settlement.

0.1.6 adds scoped gateway sync correlation, revocation-safe membership edits and
pure callback policy. Bounded gateway-list replies apply only to the exact pending
attempt; invalid/stale replies cannot replace membership. Balance replies check
the requested account and every amount, retaining distinct provider errors.
Balance/top-up decoding shares one private schema. No error substitutes zero;
valid reports cannot clear a recovery fence. Chunk completion and balance replies
remain observations, not proof of durable upload or credited/settled amounts.

Provider transports, persisted workflows, clients, endpoints and both deployment
adapters remain unimplemented. No end-to-end service capability is qualified.
[Core evidence](../evidence/core-primitives.md) and
[the capability inventory](../canic-capabilities.json) record partial native coverage.

Source inventories are historical: 0.1.5 root-batch/lifecycle-model/provider-boundaries
match source `6bd0d45` (Cargo 0.1.4); 0.1.6 gateway-registry/balance-replies match
source `d3ca8c4` (Cargo 0.1.5), not their version-mutated release commits. The two
0.1.6 inventories were checked against Git after release. Do not rotate them.

## Current follow-up — 0.1.7 draft

The maintainer requested continued work for 0.1.7. CHANGELOG.md now has an undated
0.1.7 draft after empty Unreleased. Cargo/receipt remain 0.1.6; version preparation,
commits and publication have not run. The new APIs are additive to the released
library. Continue reviewing Canic's choices independently, including our own
local restrictions; parity does not require preserving its internal design.

The draft joins local reference liveness, bounded multi-object catalog ownership
and consumer/gateway reads. Confirmed entries own lifecycle, immutable root claims
and exact reference receipts. Bounds cover global/per-tenant lifetime objects,
references/receipts per object, tenant logical bytes, global physical bytes and
unsettled billing bytes. Zero-byte and settled history still consume object slots.
Usage derives from entries; failed admissions leave claims/state intact. Exact
replay works at capacity and after settlement without reactivating references.

Consumer reads check direct-tenant authority and complete bindings. Cross-object
batches check context and raw count, then ownership of every root before supplied
bindings and result allocation. Unknown/foreign roots reject alike; no partial
statuses are returned. Released/unknown references in owned objects are inactive,
including while sibling references keep those objects live. Ordered duplicates
and multiple owned namespaces are supported within the configured entry budget.

Read-only exact receipt lookup shares its actor/payload/binding check with replay.
It preserves recorded success/failure, changes no state, and works at capacity and
after settlement. A receipt result differs from current liveness. An absent local
receipt is not proof that an uncertain paid effect never ran or that stale state
is safe. Native tests cover scope/conflict denial, mixed batches and unchanged
catalog state across reads, release and settlement.

Gateway pending pages bound scanned objects and returned results; continuations
are scope-bound positions, not authority or snapshots. Membership is rechecked on
every page. Root observations retain explicit unknown/malformed/foreign-namespace
outcomes. Tests cover sparse pages, revocation, separate capacity recovery, totals
above u64, reserved release slots and late replay. The catalog begins with already
confirmed objects; it does not reserve upload capacity before effects or persist
state. See [current evidence](../evidence/core-primitives.md#transient-catalog-after-016).

The [source refresh](../provider-review.md#selected-integration-baseline) found no
provider baseline change. [Design inputs](../service-contract.md#design-inputs-and-assumptions)
separate capabilities, consumer scenarios and provider guarantees. Root non-reuse
remains a conservative local restriction, not a frozen consumer requirement;
repeated content, cross-canister references, history churn, scan costs and serving
authority need evidence before production decisions. No dependency change,
provider effect or sibling edit was made.

`make ci` passes for this draft: release-helper fixtures, shell checks, formatting,
native compilation, strict Clippy, rustdoc, tests, Wasm and package verification.
The effect-free `make release-plan VERSION=0.1.7` resolves the intended patch.
Current catalog/reference-liveness inventories verify; historical inventories
remain source-bound and unchanged. The maintainer must commit the implementation
and draft before running the release flow from clean source.

## Provider evidence and next work

The main path still needs the provider contract for upload/read/release.
Toko indexed source `6519b72d2a420564dabaf700fc55f7b8603d9fd3` supplies defaults
`https://blob.caffeine.ai` and Cashier `72ch2-fiaaa-aaaar-qbsvq-cai`. Retained
[deployment evidence](../evidence/caffeine-deployment-observation.json) includes
public metadata, gateway-list and pricing observations; no private account lookup
or payment ran. Locators are not a selected service account or deployment authority.

The [provider baseline](../provider-baseline.json) pins client 1.1.2 and backend
reference 1.1.1. On 2026-09-26 the [recovery review](../provider-review.md#recovery-findings--2026-09-26)
reconfirmed unchanged official GitHub main, npm latest/integrity, Mops highest
version and deployed Cashier Candid hash. Gateway work subsequently rechecked
that interface. Deployed server revision and recovery/economic semantics remain
unverified. Refresh pins before provider implementation and qualification.

Caffeine's [export guidance](../provider-review.md#independent-deployment-support)
says independently hosted apps must replace its managed file-storage integration.
That does not rule out a separately arranged integration or establish Toko's
arrangements. Caffeine remains the selected candidate, not a qualified provider.
An asynchronous question about a Caffeine engineering contact/private server
source remains unanswered; no external message was sent. Do not replace missing
provider guarantees with locally invented retry or completion rules.

Resolve before provider transports or persisted workflows:

1. Supported independent onboarding and exact account/project/bucket ownership,
   with exclusive namespace/callback authority.
2. Authoritative completion lookup tied to the original upload operation,
   including lost replies, incomplete objects and evidence-retention bounds.
3. Exact top-up outcome and accepted/refunded amounts after a lost reply;
   typed errors/account balances alone do not settle a particular payment.
4. Object-specific deletion/final billing evidence, durable intent/root history
   and surviving authority across the selected same-release restore boundary.

The [service contract](../service-contract.md) also needs the concrete consumer,
accountable owners and numeric resource bounds. Existing local implementation
exceptions remain in force; generic continuation does not waive remaining gates.
Paid qualification needs separate explicit authority and bounded resources.

## Ownership

The maintainer confirmed Canic's 0.110 human acceptance for work here without
changing Canic's handoff. Library publication is separately enabled and does not
establish service qualification. Agents must not commit, change versions or infer
publication/deployment authority from continuation. Sibling repositories remain
read-only. All Canic blob capabilities must work here before removal there, with
installation retirement handled separately; see [parity](../canic-parity.md) and
[acceptance](../acceptance-plan.md).
