# Current status

Date: 2026-09-26

## Released baseline

The maintainer reports 0.1.8 pushed. Local release/tag is `572a777`, from source
`20ec33d1dc82c7ee47db83b11559a2b9f68bab1e`; Cargo and the receipt are 0.1.8.
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

Provider transports, persisted workflows, clients, production endpoints and both deployment
adapters remain unimplemented. No end-to-end service capability is qualified.
[Core evidence](../evidence/core-primitives.md) and
[capability inventory](../canic-capabilities.json) record partial native and test-fixture coverage.

Released source inventories remain historical: 0.1.5 inventories match source
`6bd0d45` (Cargo 0.1.4); 0.1.6 gateway-registry/balance-replies match `d3ca8c4`
(Cargo 0.1.5); 0.1.7 reference-liveness/catalog match `600315e` (Cargo 0.1.6).
The 0.1.8 caffeine-hashing inventory matches source `20ec33d` (Cargo 0.1.7),
verified against Git after release. Do not rotate released inventories for version
bumps or subsequent implementation.

## Current follow-up

The maintainer named 0.1.9 as the next target. Changes are grouped in its undated
changelog draft; Cargo remains 0.1.8. Continue reassessing Canic and local
choices against current consumer/provider evidence, as recorded in
[design inputs](../service-contract.md#design-inputs-and-assumptions).

An unpublished PocketIC authority probe now installs a real Wasm canister and
uses actual IC caller/service context with the shared library policies/catalog.
It proves tenant isolation from other tenants, anonymous callers and the actual
controller; forged service/tenant fields cannot authorize access. Owner release
and exact replay preserve the other tenant's usage. Pending deletion reads require
current gateway membership, and explicit operator revocation denies the next call.

A second local canister now returns controlled gateway lists across real awaits.
Reentrant overlap rejects before a second source call; revocation invalidates an
old reply; a completed newer sync cannot be overwritten by the earlier response.
Malformed/oversized/empty replies and transport rejection preserve membership,
abandon only the exact read-only attempt and require explicit retry. The source
has an explicit operator role solely to force deterministic test interleavings;
this does not define provider authority in the product. Shared passive fixture
types live in an unpublished protocol package, not the service API.

The probe owns only sample transient confirmed-object facts. It is not a production
adapter, production provider protocol, persistence implementation or upload journey.
No service capability is fully qualified. See
[fixture evidence](../evidence/core-primitives.md#pocketic-authority-probe-after-018).

The unpublished host harness owns native ic-testkit 0.10.0 and uses its full
PocketIC re-export. The core's normal and dev dependency graphs exclude the
simulator. Make targets test-native and test-pocketic run core tests and the local
IC fixtures respectively; test runs both sequentially. The explicitly provisioned
PocketIC 16.0.0 server is managed and cleaned up even on test failure. There is no
implicit server download. Local sandbox execution required loopback permission;
the approved local run passed without external provider calls.

Targeted compilation, strict Clippy, native tests, PocketIC, Wasm, formatting and
offline package verification pass. Release-helper fixtures also pass; no full
CI/release gate or version mutation ran.
The changelog, acceptance plan and capability inventory record this partial scope.

The next major milestone remains durable upload admission and interruption
recovery, followed by shared service handlers and both adapters. Resolve the
provider gates below before implementing paid retries or persisted workflows;
the fixture makes no new assumption about those guarantees.

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
