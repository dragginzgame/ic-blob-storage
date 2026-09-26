# Current status

Date: 2026-09-26

## Released baseline

The maintainer reports 0.1.7 pushed. Local release/tag is `83ad5d7`, from source
`600315eeb669de337bab15d49ef8b43d78173623`; Cargo and the receipt are 0.1.7.
`make release-tag-check` passes. Registry publication was not independently queried.
Release/publication preserve artifacts; cleanup requires an explicit request.
See [release guidance](../releasing.md).

The library provides content/provider identities, raw-byte verification, bounded
root batches and Caffeine reply decoding, billing/configuration validation and
pure funding/readiness policy. Scoped gateway synchronization rejects stale
replies and preserves revocation. Balance replies check account/amounts without
turning provider failures into zero balances or clearing recovery fences.

0.1.7 adds transient catalog ownership of confirmed-object lifecycles, immutable
root claims and exact reference receipts. Global/per-tenant object, byte and
metadata bounds retain zero-byte/settled history. Logical release, physical
deletion and billing cessation free separate capacities. Tenant reads validate
ownership and full bindings before disclosure; exact receipt reads never repeat
mutations. Gateway pages bound scanning/results and recheck authority per page.
These local models do not implement upload reservations or persistence.

Provider transports, persisted workflows, clients, endpoints and both deployment
adapters remain unimplemented. No end-to-end service capability is qualified.
[Core evidence](../evidence/core-primitives.md) and
[capability inventory](../canic-capabilities.json) record partial native coverage.

Released source inventories remain historical: 0.1.5 inventories match source
`6bd0d45` (Cargo 0.1.4); 0.1.6 gateway-registry/balance-replies match `d3ca8c4`
(Cargo 0.1.5); 0.1.7 reference-liveness/catalog match `600315e` (Cargo 0.1.6).
The 0.1.7 inventories were verified against Git after release; do not rotate them
for the version bump or subsequent implementation.

## Current follow-up

The maintainer requested continued progress and the next milestone. Changes go
in Unreleased; Cargo remains 0.1.7. Continue reassessing Canic and local choices
against current consumer/provider evidence, as recorded in
[design inputs](../service-contract.md#design-inputs-and-assumptions).

The new `CaffeineContentHasher` streams raw SHA-256 and the reviewed client's
metadata-dependent root in one pass. Appends can split anywhere; provider leaves
remain 1 MiB. Fixed hash states/frontier replace whole-file/tree buffering.
Explicit object, append, header-count and raw metadata-byte budgets bound work;
invalid offsets/lengths/budgets leave stream state unchanged. Completion verifies
exact byte length; comparison distinguishes raw corruption from root mismatch.

Independent vectors generated from the pinned unmodified client cover chunk
edges, uneven tree heights through 18 leaves, metadata order, ECMAScript trimming
and UTF-16 sorting. Tests also cover rejection/continuation after completed leaves,
truncation, corruption and metadata changes. Empty provider objects explicitly
remain unqualified; raw empty-content hashing still works separately. This does
not validate HTTP headers, generate upload proofs/certificates, persist resumable
checkpoints or establish upload completion.

The follow-up now also validates bounded chunk manifests against an expected root,
sharing tree, length and metadata checks with the streaming hasher. Distinct
chunk-hash identities retain their order; each leaf can be verified independently
at its exact index/length, with at most 1 MiB hashed per call. Independent client
leaf vectors prove reverse-order reads, repeat verification, rejection of changed
metadata/leaves/order, invalid indices/lengths and retry after corruption. A valid
manifest does not imply verified bytes or provider presence; declarations still
need trusted length/tenant binding. No resume bitmap or persisted checkpoint is
created. See
[hashing evidence](../evidence/core-primitives.md#streaming-caffeine-identities-after-017).

Official upstream main and npm latest/integrity were refreshed and remain at the
reviewed client baseline. Tests, strict Clippy, Wasm, rustdoc and formatting pass.
No full release gate, provider effect, version mutation or sibling edit ran.
Unrelated worktree files remain untouched.

The maintainer subsequently requested consolidating PocketIC through testkit.
Native dev dependencies now use published `ic-testkit` 0.10.0, whose full
`ic_testkit::pocket_ic` export supplies PocketIC 16.0.0. The direct PocketIC
dependency was removed; no existing canister-test imports required migration.
The lockfile adds testkit's host utilities without changing existing resolved
versions. Production/Wasm graphs exclude both crates. Server provisioning stays
explicit and unchanged; see [dependencies](../dependencies.md). The maintainer's
other dependency version-requirement edits are preserved.
Native all-target compilation, tests, strict Clippy and Wasm checks pass with the
locked graph. No server was started; this validates dependency integration, not
canister/service behavior.

The next major milestone is durable upload admission and interruption recovery,
followed by shared service handlers and both adapters. The local hash primitive
removes one byte-integrity gap; it does not remove the provider gates below.
Do not implement paid retries or persisted workflows by assuming those guarantees.

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
