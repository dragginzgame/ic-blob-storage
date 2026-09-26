# Current status

Date: 2026-09-26

## Released baseline

The maintainer reports 0.1.5 pushed. Local release/tag is `0e08c44`, from source
`6bd0d451469273a46ed17223de82249150f7081e`; Cargo and the receipt are 0.1.5.
`make release-tag-check` passes. Registry publication was not independently
queried. Release/publication preserve artifacts; only explicit `make clean`
removes them. See [release guidance](../releasing.md).

The released library provides content/provider identities, incremental raw-byte
verification, bounded root batches, billing/configuration validation, transient
gateway membership and pure funding/readiness/tenant policy. Its transient
lifecycle separates reference release, physical deletion and billing settlement,
with explicit ownership bindings and bounded exact-request receipts reserving
capacity for release. Immutable service-wide root claims reject reassignment,
including after settlement; this deliberately denies fresh objects with a
previously claimed root, including another tenant's identical content.

Bounded Caffeine reply decoders retain typed Cashier failures and expose chunk
completion only as a report. They do not prove durable upload or credited amounts.
Provider transports, persisted workflows, clients, endpoints and both deployment
adapters remain unimplemented. No end-to-end service capability is qualified.

[Core evidence](../evidence/core-primitives.md) and
[the capability inventory](../canic-capabilities.json) record partial native
coverage. The 0.1.5 root-batch, lifecycle-model and provider-boundaries hash files
are now historical: they match source `6bd0d45`, including its 0.1.4 manifests,
not the version-mutated release commit. All three were checked against that Git
source after release. Do not rotate them to match newer work.

## Current follow-up

The maintainer requested continued work for 0.1.6 after pushing 0.1.5. The next priority
is the provider contract needed for a complete upload/read/release journey.
The earlier local fixes do not close the server-side completion, retry,
namespace and final billing gaps. Continue reviewing Canic's choices independently;
capability parity does not require preserving its internal design.

Continued local work now adds `GatewayRegistry`, composing the existing bounded
membership value with trusted service/namespace/Cashier scope and one pending
sync token. Authorized membership edits invalidate earlier syncs, including
no-op removals. Failed edits/replies leave state intact; cancellation, out-of-order
results and duplicate replies cannot replace newer membership. The attempt
counter never wraps, and exhaustion cannot prevent revocation. A fresh explicit
sync may re-add members; this is not a permanent provider-side denylist.

Pure callback policy checks the running service, object's namespace and current
gateway membership. Native composition proves a revoked gateway remains denied
after its old sync result arrives. This extends the permitted transient gateway
model/policy scope; there are no endpoints, provider calls or persisted records.
Tokens are local correlation only: one authoritative instance and future durable
restore fencing are required. Tests, strict Clippy, Wasm, rustdoc and formatting
pass. Changes are in the undated 0.1.6 draft; Cargo stays 0.1.5. See
[gateway evidence](../evidence/core-primitives.md#gateway-sync-and-revocation-after-015).

Gateway reply ops now composes bounded Candid decoding with this registry.
Scope/stale-attempt checks happen before parsing. Decoder and candidate-validation
failures preserve membership and the exact pending attempt; success replaces
membership and consumes the token. Tests include independent didc bytes, malformed
and over-budget replies, raw/distinct limits, and reply-to-callback revocation
composition. A fresh anonymous metadata read matched the retained Cashier Candid;
there is still no transport or provider effect. No dependencies changed.

Account-balance reply decoding now covers Cashier's `account_balance_get_v1`:
byte/work/type limits, requested-account equality, all four unsigned amount
checks and distinct `AccountNotFound`/`InternalError` reports. It shares the
private balance schema with top-up decoding. Independent Candid fixtures and
native readiness composition prove that failed reads never substitute zero and
valid reports cannot clear a recovery fence. No live account query was made;
source/freshness/attempt binding and payment reconciliation remain external.
See [balance reply evidence](../evidence/core-primitives.md#account-balance-replies-after-015).

A further public-source review found Caffeine's current export guidance says
independently hosted apps must replace its managed file-storage integration.
See [independent deployment support](../provider-review.md#independent-deployment-support).
This does not prove a separately arranged integration is impossible or identify
Toko's actual arrangements. It adds a concrete support/onboarding question;
public packages and reachable endpoints alone cannot settle it. Caffeine remains
the selected candidate, not a qualified provider or a rejected backend.

An asynchronous question asks whether the maintainer has a Caffeine engineering
contact or access to the gateway/Cashier server source. No answer is recorded yet;
no external message was sent. Provider-side work depends on authoritative
answers, not another locally invented retry or completion rule.

## Provider evidence and next work

Toko indexed source `6519b72d2a420564dabaf700fc55f7b8603d9fd3` supplies defaults
`https://blob.caffeine.ai` and Cashier `72ch2-fiaaa-aaaar-qbsvq-cai`. Retained
[deployment evidence](../evidence/caffeine-deployment-observation.json) includes
public metadata, gateway-list and pricing observations; no private account lookup
or payment ran. These are locators, not a selected service account or proof of
independent deployment support.

The [provider baseline](../provider-baseline.json) pins client 1.1.2 and backend
reference 1.1.1. On 2026-09-26 the [recovery review](../provider-review.md#recovery-findings--2026-09-26)
reconfirmed unchanged official GitHub main, npm latest/integrity, Mops highest
version and deployed Cashier Candid hash. Deployed server revision and provider
recovery/economic semantics remain unverified. Refresh pins before implementation.

Resolve these concrete questions before provider transports or persisted workflows:

1. Supported onboarding for this independently deployed Rust canister, with exact
   account/project/bucket ownership and exclusive namespace/callback authority.
2. Authoritative upload completion lookup tied to the original operation, including
   lost replies, incomplete objects and evidence-retention bounds.
3. Exact top-up outcome/accepted-refunded amount reconciliation after a lost reply;
   typed errors and account balances alone do not settle a particular payment.
4. Object-specific deletion and final billing evidence, plus durable intent/root
   history and surviving authority for the selected same-release restore boundary.

The [service contract](../service-contract.md) also needs the concrete consumer,
accountable owners and numeric resource bounds. The local implementation exceptions
remain in force, but generic continuation does not waive the remaining gates.
Paid qualification needs its own explicit authority and bounded resources.

## Ownership and validation

The local 0.1.5 fixes passed native tests, strict Clippy, Wasm, rustdoc and
formatting; its release receipt records `release-verify` on the committed source.
This follow-up checked release provenance and provider documentation, then ran
targeted native/lint/Wasm/docs checks for the gateway registry, policy and
gateway/balance reply ops. It did not change dependencies or run a new full
release gate.

The maintainer confirmed Canic's 0.110 human acceptance for work here without
changing Canic's handoff. Library publication remains separately enabled;
it does not establish service qualification. Agents must not create commits,
change versions or infer publication/deployment authority from continuation.
Sibling repositories remain read-only. All Canic blob capabilities must work
here before removal there, with installation retirement handled separately;
see [parity](../canic-parity.md) and [acceptance](../acceptance-plan.md).
