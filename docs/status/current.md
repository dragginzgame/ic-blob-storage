# Current status

Date: 2026-09-26

## Released baseline

The maintainer reports 0.1.10 live. Local release/tag is `27c9b6b`, from source
`aaa50572ee9d787c1786486ad679c919f0ac62f3`; Cargo and the receipt are 0.1.10.
`make release-tag-check` passed after release. Registry publication was not
independently queried. Release/publication preserve artifacts; cleanup requires
an explicit request. See [release guidance](../releasing.md).

The library provides content/provider identities, streaming Caffeine hashing,
root-bound manifests, chunk verification/coverage and bounded missing-chunk pages.
It also provides billing/configuration validation, bounded provider reply codecs,
pure funding/readiness policy and scoped gateway synchronization with stale-reply
rejection. None establishes provider completion or durable recovery.

The transient catalog owns confirmed-object lifecycles, immutable root claims,
exact reference receipts and bounded deletion pages. It separates logical,
physical and liability capacity; zero-byte/settled objects retain lifetime slots.
Tenant/gateway policy checks full bindings and current authority on reads.

PocketIC fixtures exercise actual caller/controller isolation, reference replay,
gateway revocation and inter-canister sync races. Fixed content vectors run in
Wasm with an ordered-append instruction budget. Fixtures substitute provider facts,
have no persistence and do not qualify production endpoints or restore safety.
The unpublished host harness owns testkit; the core package excludes PocketIC.

Released source inventories remain historical: 0.1.5 matches `6bd0d45` (Cargo
0.1.4), 0.1.6 matches `d3ca8c4` (Cargo 0.1.5), 0.1.7 matches `600315e` (Cargo
0.1.6), 0.1.8 matches `20ec33d` (Cargo 0.1.7), and 0.1.9 matches `832b364`
(Cargo 0.1.8). The 0.1.10 chunk-verification inventory was verified against Git
source `aaa5057` (Cargo 0.1.9). Do not rotate these for later source/version changes.

## Current follow-up — 0.1.11

The maintainer approved the proposed upload admission/reservation batch and
provider-gap review. Cargo remains 0.1.10; completed changes have an undated
0.1.11 changelog draft.

`model::catalog::admission::UploadCatalog` now owns an initially empty catalog
and bounded upload operation history together. Tenant-scoped IDs bind exact raw
content digest, provider root, length, first reference and full service/tenant/
namespace/object/incarnation identity. The caller must supply the authenticated
tenant principal; delegated actors are unsupported. Root claims have one owner.
There is no catalog import, mutable escape, serialization or owner clone.

Every admitted operation reserves global/per-tenant lifetime slots, concurrent
upload slots, byte capacities and eventual first-reference/release-receipt
capacity. Exact retries return current state without another allocation. Cancel
only before exposure: byte/concurrent capacity is freed, history/root claims are
retained. `ExposurePossible` deliberately covers all unresolved outcomes and has
no expiry/reset/retry permission. Independently authenticated exact completion
transfers capacity into the catalog without double counting. Completion replay
never reactivates references after release/settlement. Physical deletion and
billing cessation still free separate capacities.

The local read boundary now includes bounded tenant active-upload pages and
aggregate reserved/confirmed usage. Cursors bind service/tenant, never authority;
terminal history consumes scan budget, empty pages can continue, and new earlier
IDs require a new sweep. Gateway root observations distinguish reserved,
possibly exposed, cancelled and confirmed lifecycle states after membership and
namespace checks. These are local observations, not provider deletion permission.
Gateway batches resolve distinct roots together and share at most one bounded
history scan, stopping after the last needed pending/cancelled root. Confirmed,
unknown and malformed inputs need no history scan. Duplicates reuse results;
temporary maps are bounded by batch length and no second persistent index exists.
Native regressions check scan counts, mixed states and fresh reads after transitions.

Native core tests/doctests, strict workspace Clippy, workspace Wasm, formatting,
rustdoc, offline package verification and source inventory checks pass. The expanded PocketIC probe covers
real tenant/controller isolation, cancellation/replay, retained uncertain capacity
and revocation over fixed upload facts. Existing authority/content/sync cases
also pass. See [core evidence](../evidence/core-primitives.md#transient-upload-admission-after-0110).
No provider/persistence/recovery qualification is claimed. No full CI, version
mutation, commit, publication, paid effect or sibling edit ran.

The next integration work needs durable intent/reservation ownership and recovery
fences, provider mapping/protection of pending roots, exact reconciliation, shared
service handlers and both adapters. The current model consumes trusted completion
facts supplied by future ops; a chunk status/hash or client progress cannot supply
that fact. Byte liabilities are not a currency spending cap. Resolve the gates
below before provider transports or persisted workflows.

## Provider evidence and next work

The public-source follow-up rechecked Caffeine GitHub main at
`e5cacdfe5ce55e939edb02980fca800c0c13f421`, backend Storage/Mixin hashes and npm
latest 1.1.2/integrity; all match retained evidence. The current export and storage
cost guidance still supplies no exact upload/deletion/final-charge receipt
contract. This follow-up did not re-query Mops or deployed Cashier. See
[provider review](../provider-review.md#upload-admission-follow-up--2026-09-26).

Toko's retained source `6519b72d2a420564dabaf700fc55f7b8603d9fd3` supplies
`https://blob.caffeine.ai` and Cashier `72ch2-fiaaa-aaaar-qbsvq-cai`.
[Deployment observations](../evidence/caffeine-deployment-observation.json)
identify a reachable candidate, not an owned account or deployment authorization.
Caffeine remains selected but unqualified. Its independent-export guidance says
managed file storage needs replacement; a separately supported arrangement is
not established. The earlier provider-contact/private-source question remains
unanswered; no message was sent externally.

Resolve before provider transports or persisted workflows:

1. Supported independent onboarding and exact account/project/bucket ownership,
   including exclusive namespace/callback authority.
2. Authoritative completion lookup tied to the original operation, including
   lost replies, incomplete objects and numeric evidence-retention bounds.
3. Exact top-up accepted/refunded amounts after a lost reply; balances and typed
   errors do not settle a particular payment.
4. Object-specific deletion/final-billing evidence, durable root/intent history
   and surviving authority across the same-release restore boundary.

The [service contract](../service-contract.md) also needs a concrete consumer,
accountable owners and production resource bounds. Local exceptions remain in
force; this batch does not waive provider or persistence gates. Refresh provider
pins before boundary implementation. Paid qualification needs explicit authority
and bounded resources. No end-to-end service capability is qualified.

## Ownership

The maintainer confirmed Canic's 0.110 acceptance for work here without changing
Canic's handoff. Agents must not commit, change versions or infer deployment/
publication authority from continuation. Siblings remain read-only. All Canic
blob capabilities must work here before removal there, with installation
retirement handled separately; see [parity](../canic-parity.md) and
[acceptance](../acceptance-plan.md). Library publication is separately enabled
and does not establish service qualification.
