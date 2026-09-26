# Current status

Date: 2026-09-26

## Released baseline

The maintainer reports 0.1.9 pushed. Local release/tag is `2cacb1e`, from source
`832b36455e2fe50a859e1c209b44bdd345d4dca6`; Cargo and the receipt are 0.1.9.
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

0.1.8 adds bounded streaming raw/Caffeine hashing and root-bound chunk manifests
with independent current-client vectors. These establish local byte consistency,
not provider presence, upload completion or durable resume progress. Testkit
provides the full PocketIC re-export for host testing.

0.1.9 adds real PocketIC caller/controller checks and inter-canister sync races.
Local fixtures prove release replay, revocation, stale reply rejection and explicit
retry after malformed/failed replies. A controlled source holds operator authority
only to force reentrant test schedules; this is not a production provider contract.
The host harness owns testkit; the published core and its native tests exclude it.

Provider transports, persisted workflows, clients, production endpoints and both deployment
adapters remain unimplemented. No end-to-end service capability is qualified.
[Core evidence](../evidence/core-primitives.md) and
[capability inventory](../canic-capabilities.json) record partial native and test-fixture coverage.

Released source inventories remain historical: 0.1.5 inventories match source
`6bd0d45` (Cargo 0.1.4); 0.1.6 gateway-registry/balance-replies match `d3ca8c4`
(Cargo 0.1.5); 0.1.7 reference-liveness/catalog match `600315e` (Cargo 0.1.6).
The 0.1.8 caffeine-hashing inventory matches source `20ec33d` (Cargo 0.1.7),
verified against Git after release. Do not rotate released inventories for version
bumps or subsequent implementation. The 0.1.9 PocketIC authority inventory
likewise matches source `832b364` (Cargo 0.1.8), verified against Git after release.

## Current follow-up

The maintainer selected 0.1.10; related changes are grouped in its undated draft.
Cargo remains 0.1.9. Continue reassessing choices against
[consumer/provider evidence](../service-contract.md#design-inputs-and-assumptions).

The new CaffeineChunkVerifier owns a validated immutable manifest and one bit per
chunk. It credits each successfully verified position once, reports exact verified
bytes/chunks, and checks duplicate deliveries again. Invalid indices, lengths or
corrupt bytes leave coverage unchanged. A fresh verifier begins at zero; there is
no imported bitmap, serialized checkpoint, content buffer or growing retry history.

Independent client vectors exercise reverse-order delivery with a deliberately
missing middle position, identical chunk hashes at distinct positions, bitmap-byte
boundaries, partial final chunks and corrupted-gap recovery. Unit tests check
rejections before/after success, large indices and repeated valid verification.
The additive API retains the existing stateless manifest contract. All-chunks-
verified means past checks in this instance; it does not establish retained bytes,
successful writes, raw whole-file digest, tenant authority or provider completion.
See [coverage evidence](../evidence/core-primitives.md#transient-chunk-coverage-after-019).

The companion CaffeineOrderedChunkVerifier now checks exact leaves before feeding
an ordered raw-content verifier. Corrupt chunks leave the prefix/hash intact for
retry; skipped/replayed chunks reject. Finalization consumes the verifier and
requires complete length plus the expected raw digest before returning both
identities. Independent vectors exercise rejection/retry at each position through
multi-chunk and metadata-bearing files; unit cases reject incomplete content and
a conflicting raw digest despite valid manifest bytes. Neither variant owns
destination writes, provider transport or persisted read recovery.

Bounded missing-chunk pages now provide exact local index/offset/length ranges.
Independent scan/result limits bound work even across verified prefixes; empty
filtered pages can continue. Calls observe current coverage, skip positions
verified between pages and never mark or reserve selected chunks. Scan completion
does not imply byte verification, and starting a new instance requires a new sweep.
Tests cover budget combinations across client vectors, partial final chunks,
intervening verification, repeated reads and invalid/end positions. These ranges
do not define HTTP requests or assume provider support for range reads.

The local PocketIC probe now executes pinned content vectors inside Wasm: full
1 MiB, one-byte final leaf and ECMAScript metadata cases. It checks reverse-order
coverage, duplicate accounting, empty-page continuation, corrupt ordered retries,
replay rejection and typed wrong-digest/truncated finalization. Fixed enum inputs
select compiled fixture data; this is not a production read endpoint. Measured
peak valid ordered append was 162,109,843 instructions against a local one-billion
regression budget, excluding input generation, manifest setup and Candid handling.
All PocketIC authority/content/sync cases pass using the explicit local server.

Targeted verifier unit/client-vector tests, workspace Clippy, Wasm, rustdoc,
formatting and offline package verification pass. The unpublished probe/harness
reuse existing serde/JSON packages without changing registry versions or core
dependencies. No full CI, provider effect, version mutation or sibling edit ran. Released inventories remain
historical; the new source inventory records this local addition separately.

The next major milestone remains durable upload admission and interruption
recovery, followed by shared service handlers and both adapters. Resolve the
provider gates below before implementing paid retries or persisted workflows;
local verification coverage does not settle those guarantees.

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
