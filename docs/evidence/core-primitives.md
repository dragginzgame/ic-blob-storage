<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Content and billing primitives — native evidence

This file retains source-bound history. Sections describing the removed Canic
adapter and its former commands are historical; current library/standalone scope
and validation are recorded in the [current handoff](../status/current.md).

This is an append-only implementation evidence record, not a tutorial or current
API reference. Search by release, feature or date, and follow each section's
source and artifact links against the revision it names. For current behavior,
start with the [service contract](../service-contract.md),
[operator guide](../operator-guide.md) or [documentation index](../README.md).

## Upload restoration simplification — 2026-10-04

The [source/artifact record](upload-restoration-simplification.json) identifies
released 0.14.6 plus the 0.14.7 draft, still compiled with package version 0.14.6.
`StableUploads::open` opens and validates the root/object owner before validating
permissions. Each permission must match its tenant/request key, its root's original
request ID and its root's exact object/tenant. A shared root therefore implies one
tenant and one request key; a second temporary heap set adds no uniqueness fact.
That set is removed. The already decoded admission time also replaces a second
permission read during attested-completion validation.

The [recovery tests](../../crates/ic-blob-storage/src/ops/service/uploads/tests/recovery/mod.rs)
accept distinct roots with shared request IDs across tenants, then reject same-root
corruption both within and across tenants while preserving all ten memories. The
[completion tests](../../crates/ic-blob-storage/src/ops/service/uploads/tests/completion/mod.rs)
retain valid late attestations and reject observations before original admission.
Existing malformed root/request indexes, manifests, references/receipts, totals,
fences and storage assembly remain covered. Scoped native checks, strict core
Clippy, rebuilt Wasm and actual PocketIC same-release restoration checks pass.
An initial compile mistake and the broad-filter missing-fixture run remain recorded;
all five missing-fixture cases pass after provisioning the maintained substitutes.

This removes transient bookkeeping, not a durable index or validation scan. No
wire/storage layout, API, recovery authority, repair path or provider effect changes.
There is no measured instruction/heap saving or million-object qualification.
Older released-media artifacts and all previous records retain their identities.

The original core candidate is recorded below; subsequent local ports have
separate source hashes in their sections.

The billing/gateway inventories describe the 0.1.4 source candidate `f0cabb5`,
whose Cargo version was still 0.1.3. Audit those hashes against that source commit;
release `3aef138` changes version files. They remain historical and are not rotated.

The 0.1.6 gateway-registry and balance-replies inventories match source `d3ca8c4`
(Cargo 0.1.5), checked against Git after release `0992929`. They are historical
too; use that source commit rather than checking them against a newer worktree.

Date: 2026-09-25. The maintainer explicitly approved this bounded implementation
before B1 closure. It implements domain values and pure policy only. It adds no
provider calls, persisted state, serialization, endpoints, lifecycle ownership or Canic
dependency. No sibling repository was changed.

## Implemented contract

- `ContentDigest` computes SHA-256 over exact raw bytes, including empty content.
  `ProviderRootHash` parses Caffeine roots separately; neither grants authority
  or proves a completed upload. Both parse exactly `sha256:` plus 64 ASCII hex
  digits and display lowercase. Binary provider roots require exactly 32 bytes.
  Typed errors distinguish empty input, prefix, text/binary length and invalid
  hex offsets. Parsing does not implement the provider's tree algorithm.
- `ContentVerifier` incrementally checks raw bytes against a fixed expected
  length and digest without buffering the object. Every chunk supplies the exact
  next byte offset; wrong offsets and oversized chunks leave the hash/count
  unchanged. Completion consumes the verifier and rejects incomplete or corrupt
  bytes. Empty content still requires the correct digest. The declared length
  must fit SHA-256's whole-byte message bound (2^61 - 1 bytes); service size/session
  limits remain separately required. The caller supplies a trusted expected
  digest. This transient state is not a persisted upload/checkpoint or an
  authenticated provider completion receipt.
- `FundingLimits` validates positive reserve/minimum/target and minimum <= target.
  Funding takes a positive `NonZeroU128` request and either reports that the
  entire amount fits above reserve or rejects it; no partial top-up is proposed.
  Available cycles must exclude existing commitments. This calculation never
  establishes accounting freshness, retry permission or effect authority.
- Billing readiness reports missing configuration/gateways, unavailable or
  malformed observations, insufficient balance, reserve violations and recovery
  fences. Balance >= minimum needs no top-up; below minimum proposes target minus
  balance. Failed observations never become zero balances. Recovery fencing
  blocks readiness even with sufficient funds. The workflow must establish the
  supplied recovery state; the policy does not reconcile or clear fences.

The raw `abc` digest and provider root reference the pinned client 1.1.2
[observations](caffeine-client-observations.json). Empty raw hashing support does
not claim empty provider-object support. SHA-256 vectors cover empty, `abc` and
the standard 56-byte message spanning SHA-256 padding blocks. Streaming tests
also check the independent million-ASCII-`a` vector without retaining a million
bytes, and verify invariance across chunk splits near SHA-256 block boundaries.

## Executable evidence

| Capability | Source and unit test references | Remaining boundary |
| --- | --- | --- |
| BLOB-01 / A02 | [identity module](../../crates/ic-blob-storage/src/model/identity/mod.rs): `raw_content_matches_sha256_vectors`, `provider_root_roundtrips_every_byte_and_normalizes_hex`, `rejects_malformed_text_and_binary_with_typed_errors`, `pinned_caffeine_root_is_distinct_from_raw_content_digest`; compile-fail doctest rejects digest-to-root conversion | Provider tree computation, chunk/checkpoint verification, endpoint conversion and actual upload/read evidence |
| A02 raw-content verification | [verification module](../../crates/ic-blob-storage/src/model/identity/verification/mod.rs): `chunk_boundaries_do_not_change_verified_digest`, `incremental_hash_matches_independent_million_byte_vector`, `rejected_chunks_leave_hash_and_offset_unchanged`, `truncated_and_same_length_corrupt_content_cannot_finish`, `empty_content_still_requires_the_expected_digest`, `declared_lengths_respect_the_algorithm_bound_without_allocating_content`; executable usage doctest | Provider tree/chunk proofs, trusted manifest/digest provenance, persisted checkpoint/resume and service upload/read workflows |
| BLOB-08 / A04 | [billing model](../../crates/ic-blob-storage/src/model/billing/mod.rs): `limits_reject_each_invalid_boundary`, `limits_accept_equal_thresholds_and_maximum_cycle_values` | Provider/namespace identity, gateway limits, configuration workflow and persistence |
| BLOB-11 / A08 | [billing policy](../../crates/ic-blob-storage/src/policy/billing/mod.rs): `funding_requires_full_amount_and_preserves_exact_reserve`, `funding_conserves_cycles_at_small_and_extreme_boundaries` | Durable intent, reserved liabilities, exact operation identity, uncertain-result handling and provider effects |
| BLOB-12 / A08/A11 | Same policy module: `readiness_uses_minimum_threshold_then_full_target_top_up`, `readiness_preserves_all_simultaneous_blockers`, `observation_failures_never_become_zero_balance_top_ups`, `missing_configuration_and_recovery_fences_fail_closed`, `maximum_target_and_equal_thresholds_do_not_overflow` | Trusted observations, recovery reconciliation, shared service status workflow and real operator transport |

Validated working tree based on maintainer commit
`14d5972`; exact tested sources, dependency lock and toolchain are bound by
[SHA-256 inventory](core-primitives.sha256), checked from the repository root
with `sha256sum -c docs/evidence/core-primitives.sha256`.
This inventory describes the implementation candidate at package version 0.1.0.
Release preparation changes the Cargo manifest/lockfile versions; after that
transaction, audit this historical inventory against the release receipt's
source commit, not its version-mutated release commit. Do not rotate these
historical hashes merely to match a version bump.

With pinned Rust 1.98.1, repository-local `target/`, offline locked dependencies:

- `make test`: native tests, usage example and compile-fail documentation tests pass.
- `make clippy`: all targets/features pass with warnings denied.
- `make wasm-check`: library compiles for `wasm32-unknown-unknown`.
- `make docs-check`: public documentation builds with warnings denied.
- `make fmt-check`: formatting passes.

The subsequent explicit 0.1.1 request authorized `make release-verify`, which
passes on this implementation candidate, including shell/release-helper tests,
native/Wasm checks and real offline package verification. The release-helper
regression tests substitute Git/Cargo/validation commands and create no real
commits or tags. Actual version preparation stops at the uncommitted-source
check; it must revalidate the maintainer's clean source commit before generating
a receipt. Cargo remains 0.1.0 until that transaction.

No PocketIC server, deployment or live provider effect ran.
These results do not qualify A01–A12 journeys, actual provider economics or
backup/restore. The parity inventory remains partial and Canic removal gated.

## Billing input port after 0.1.3

The [billing ops module](../../crates/ic-blob-storage/src/ops/billing/mod.rs)
ports numeric conversions from Canic's `ops/blob_storage/conversion.rs` and
`ops/cashier/conversion.rs`, plus strict decimal funding input from
`canic-cli/src/blob_storage/options.rs`. Configuration validation also follows
`domain/policy/pure/blob_storage/mod.rs` and the validation portion of
`workflow/blob_storage/billing/mod.rs`. Re-inspected at Canic HEAD
`3f825aa223e663a562a7cb1cca72e57b5703e0e9`; these files still match the
historical source inventory. Other dirty Canic
work was neither incorporated nor changed.

Unsigned/signed Candid values must fit `u128`; negative balances never become
zero. Numeric configuration conversion delegates to `FundingLimits` invariants.
Both Candid and operator funding return positive amounts accepted by existing
policy. Decimal input accepts leading zeros and rejects signs, whitespace,
separators, non-ASCII digits, overflow and zero. Errors identify the numeric field
or invalid byte offset without retaining the supplied input. Conversion borrows
big integers; request/decoder limits remain the caller's responsibility.

[Whole-balance conversion](../../crates/ic-blob-storage/src/ops/billing/balance/mod.rs)
also validates total, prepaid, promotional and ledger amounts before exposing
the [bounded values](../../crates/ic-blob-storage/src/model/billing/balance/mod.rs).
Each negative or oversized component rejects with its own typed field, even
when the total is valid. Amounts remain independent: no sum rule, debt-mode
interpretation, provider DTO or completion proof is introduced. Tests cover
every component's rejection, full-width values and Candid-to-readiness composition
where a malformed ledger blocks an otherwise sufficient total. This preserves
Canic's complete numeric validation; its source conversion hash still matches
the captured inventory.

[Configuration conversion](../../crates/ic-blob-storage/src/ops/billing/configuration/mod.rs)
builds a [validated candidate](../../crates/ic-blob-storage/src/model/billing/configuration/mod.rs)
combining Cashier principal, funding limits and gateway bounds. Anonymous and
management Cashier principals reject. Both gateway limits must be positive and
fit 32-bit Wasm indexes even on the host; this ceiling is not a recommended
resource budget. No deployment defaults, account binding, persisted configuration
or provider proof are implied. Tests exercise both bound fields at zero, maximum
and overflow, identity rejection, typed conversion errors, and composition with
gateway normalization and funding/readiness policy.

Unit tests cover each invalid configuration field, signed/unsigned extremes and
operator syntax. [Native composition tests](../../crates/ic-blob-storage/tests/billing_inputs.rs)
pass real Candid encoding/decoding through the conversion API and funding policy;
they prove full-request reserve behavior and rejection of wire-valid oversized
numbers. They do not simulate canisters or provider payments.

`make test`, `make clippy`, `make wasm-check`, `make docs-check` and formatting
pass with the pinned toolchain and offline dependencies. The source batch is
based on release `8d4e228` (0.1.3); verify its exact code/dependencies with
`sha256sum -c docs/evidence/billing-inputs.sha256`. Historical core hashes above
remain unchanged. No provider DTO, persisted workflow, endpoint, operator
transport, dependency update, release or sibling edit is included.

### Funding admission correction

The [admission policy](../../crates/ic-blob-storage/src/policy/billing/admission/mod.rs)
adds a pure decision about preparing a new funding intent. This is a required
safety correction to Canic's transient-only `ops/blob_storage/funding.rs` guard,
not a port of that guard or a completed durable replacement. That source and
`workflow/blob_storage/billing/mod.rs` still match the historical source inventory.

Recovery fencing, outstanding funding activity, missing configuration and reserve
violations reject in that order. In-progress and uncertain effects remain blocked
despite changing requested amounts, available funds or configuration. There is
no expiry-based release. A successful decision preserves the full request and
only proposes intent preparation. Authorization, account/namespace binding,
atomic admission/reservation, durable intent and exact retry identity remain
workflow obligations; supplied observations do not prove any of those facts.

Unit tests cover fence precedence, outstanding effects across changed inputs,
missing configuration, exact reserve boundaries and full-width arithmetic.
The native composition test `diagnostic_top_up_does_not_bypass_funding_admission`
demonstrates that readiness can report a top-up fitting the reserve while admission
rejects it. Native tests, Clippy, Wasm, rustdoc and formatting pass. These tests
evaluate supplied observations; they do not prove interruption recovery,
concurrency exclusion, evidence retention or provider payments. Source hashes
join `billing-inputs.sha256` above.

## Gateway list port after 0.1.3

The [gateway model](../../crates/ic-blob-storage/src/model/gateway/mod.rs) ports
Canic's `CashierConversionOps::normalize_gateway_principals`. The source at
Canic HEAD `3f825aa223e663a562a7cb1cca72e57b5703e0e9` still matches the captured
inventory hash. Empty, anonymous and management entries reject; deduplication
preserves first-occurrence order. Positive limits bound both raw entries and
distinct members, with no deployment defaults. The raw bound is an extraction
correction: duplicate-heavy lists must not bypass processing limits. Decoder
allocation limits remain separate. Ordered-set deduplication avoids quadratic
search through the growing output.

`GatewayList::replace` validates the entire candidate before replacing the
transient value with its original limits. Unit tests cover exact bounds,
duplicate floods, invalid trailing entries, unchanged membership/order/limits
after every rejection class, successful recovery and repeat replacement.

[Gateway membership](../../crates/ic-blob-storage/src/model/gateway/membership/mod.rs)
ports individual add/remove and complete replacement behavior from Canic's
`ops/blob_storage/lifecycle.rs`. Existing members remain idempotent at capacity;
removal frees capacity and may empty the set. Empty provider sync input still
rejects, preserving prior membership. Tests cover repeat/unknown removals,
invalid additions, capacity recovery and failed/successful sync replacement.

Membership is not authority: provider/service binding, trusted synchronization,
stale-response exclusion after revocation, durable state and callback workflows
remain unimplemented. Only local membership changes are implemented here.

Native tests, strict Clippy, Wasm compilation, docs and formatting pass. Verify
the source/dependency hashes with
`sha256sum -c docs/evidence/gateway-list.sha256`. No PocketIC/provider effects,
release, dependency update or sibling mutation ran.

## Binary-root batch parsing after 0.1.4

The [batch parser](../../crates/ic-blob-storage/src/model/identity/batch/mod.rs)
preserves the input ordering, duplicates and individually malformed entries used
by Canic's `BlobStorageApi::blobs_are_live`. The inspected lifecycle API still
matches the source inventory. Parsing returns
one typed result per input; it does not query liveness or authorize a caller.

Explicit raw entry and combined byte limits bound processing and result storage.
Both are checked before allocating results; malformed entries consume their full
byte length. Empty input is accepted. Tests cover mixed valid/invalid/duplicate
roots at exact limits, empty-entry floods, oversized batches and parsing the same
input after budget rejection. Native tests, strict Clippy, Wasm, rustdoc and
formatting pass. No provider or platform behavior is simulated.

The source batch is based on release `3aef138` (0.1.4); verify
`sha256sum -c docs/evidence/root-batch.sha256`. This is partial BLOB-03 input
coverage only. Authenticated state lookup, tenant/reference bindings, callback
semantics and actual service journeys remain outstanding.

## Confirmed-object lifecycle model after 0.1.4

The maintainer requested a fresh model review and work on the persistence
contract/lifecycle model. The [design rationale](../service-contract.md#lifecycle-design-under-independent-review)
and [native model](../../crates/ic-blob-storage/src/model/lifecycle/mod.rs) distinguish
reference release, physical deletion and final billing settlement. Canic's
`ops/blob_storage/lifecycle.rs` and `storage/stable/blob_storage.rs` still match
the captured inventory; their root-keyed records and live-only deletion behavior
were inspected as design inputs, not copied as requirements. Toko's pinned asset
helper was re-read at the revision recorded in deployment evidence and delegates
its liveness/release checks to Canic. No sibling source was changed.

`BlobLifecycle` begins with an already-confirmed upload and one reference. It
retains released IDs within an explicit lifetime slot bound. Tests demonstrate
unchanged state on unknown/repeated release and invalid confirmation, idempotent
active retain at capacity, rejection of released-ID reuse, no retain after the
last release, separate byte projections through settlement, and unresolved
obligations at zero and maximum length. This is native transition evidence toward
BLOB-04/05/06, not authentication, request/payload replay binding or a persistent
upload/release workflow. Confirmation facts remain externally supplied.

Native tests, Clippy, Wasm, docs and formatting pass on this batch based on
`3aef138`; verify `sha256sum -c docs/evidence/lifecycle-model.sha256`.
No PocketIC, provider effect, dependency change or version transaction ran.

The same uncommitted lifecycle batch now includes immutable
[object bindings](../../crates/ic-blob-storage/src/model/lifecycle/binding/mod.rs)
and [direct-tenant policy](../../crates/ic-blob-storage/src/policy/tenant/mod.rs).
The lifecycle's unbound signatures were replaced; every reference mutation and
confirmation checks service, tenant, namespace, object and incarnation before
replay or transition. [Native composition tests](../../crates/ic-blob-storage/tests/lifecycle_binding.rs)
exercise all mismatch dimensions in all four phases, and verify that matching
reference data does not authorize another actor or service. Special-principal
binding validation has adjacent unit coverage. The same targeted checks pass;
hashes join `lifecycle-model.sha256` above.

These checks are partial A01 evidence over supplied values. The workflow must
obtain the expected binding from trusted state and the service/actor from actual
execution context. Neither a user-constructed key nor these native fixtures prove
endpoint authentication, delegated access, namespace resolution, identity reuse
safety, provider confirmation authenticity or durable isolation.

The same batch adds [reference request receipts](../../crates/ic-blob-storage/src/model/lifecycle/requests/mod.rs).
The value owns its lifecycle and compares complete actor/operation/reference data
for an admitted request ID. Exact retries return original successes or failures;
conflicting IDs and capacity/scope rejection leave both values unchanged. Local
lifecycle failures are recorded results, distinct from admission rejection.
There is no timeout/eviction or provider operation in this model.

Tests cover original-success replay after release without reactivation, original
failure replay after later reference creation, changed actors/kinds/arguments,
receipt exhaustion and replay at full capacity after settlement. A receipt is
reserved for each active reference's eventual release, verified even when other
admissions exhaust capacity. Native composition verifies access denial before
replay and unchanged journal state. Targeted tests, Clippy, Wasm, docs and
formatting pass; code/test hashes extend `lifecycle-model.sha256`.
This evidence covers local bounded request semantics, not restart persistence,
provider idempotency or uncertain paid-effect recovery.

## Provider response and root correlation fixes

On 2026-09-26 the maintainer explicitly requested resolving the findings in
the [provider review](../provider-review.md#recovery-findings--2026-09-26).
Anonymous metadata/version queries reconfirmed the retained Cashier Candid
SHA-256 and Mops backend 1.1.1. The requested scope covers the specific local
decoders and claim model, not paid provider execution or blanket B1 acceptance.

The [Caffeine ops owner](../../crates/ic-blob-storage/src/ops/caffeine/mod.rs)
has private passive wire DTOs and bounded response decoders. Native tests check
that upload progress/root fields cannot replace the exact completion indicator;
missing, duplicate and invalid status fields reject. A completion report remains
an observation, with no conversion into an authenticated stored-object proof.
Funding tests retain every advertised provider error, reject negative balance
components, empty/truncated/unknown replies, byte/work/type exhaustion and
excessive skipped reply data. An `Ok` response exposes validated balance amounts,
never an invented credited amount or permission to retry a payment.

Funding fixtures in `crates/ic-blob-storage/tests/fixtures/caffeine-top-up/`
were generated independently with `didc 0.5.4` from the retained deployed Candid,
not the Rust decoder's derived schema. They are synthesized test inputs, not
recorded paid responses. Reproduce each with `didc encode '<value>' --types
'(AccountTopUpResult)' --defs docs/evidence/caffeine-cashier.did`; redirect the
hex output to its fixture. The values are:

| Fixture | Value |
| --- | --- |
| `without-cycles.hex` | `(variant { Err = variant { TopUpWithoutCycles } })` |
| `overflow.hex` | `(variant { Err = variant { AccountBalanceOverflow } })` |
| `internal.hex` | `(variant { Err = variant { InternalError = "fixture diagnostic" } })` |
| `unauthorized.hex` | `(variant { Err = variant { NotAuthorized = principal "2vxsx-fae" } })` |
| `success.hex` | `(variant { Ok = record { balance = record { total = 100 : int; cycles_prepaid = 80 : int; cycles_promo = 10 : int; cycles_ledger = 10 : int; debt_target = variant { Prepaid } }; message = "fixture" } })` |
| `negative-prepaid.hex` | Same success value, with `cycles_prepaid = -1 : int` and `debt_target = variant { Ledger }` |
| `unknown-error.hex` | `(variant { Err = variant { FutureError } })`, encoded without `--types` or `--defs` as an unsupported variant control |

The [root-claim model](../../crates/ic-blob-storage/src/model/lifecycle/roots/mod.rs)
retains an immutable root/object association across tenants, namespaces and
incarnations. Tests reject reassignment, multiple roots for one object and wrong
service; full capacity preserves prior lookup and exact claim replay. Native
lifecycle composition retains claims through settlement and rejects the original
binding against a newer deletion-pending object's confirmation without releasing
its physical bytes or liabilities. No model callback authenticates a real gateway.

`make test`, `make clippy`, `make wasm-check`, `make docs-check` and formatting
pass. `serde_json` uses its existing locked 1.0.151 version as a direct dependency;
no package versions were upgraded. The current uncommitted source inventories
are [provider-boundaries.sha256](provider-boundaries.sha256),
[lifecycle-model.sha256](lifecycle-model.sha256) and [root-batch.sha256](root-batch.sha256).
They include refreshed manifest/lock hashes; prior released inventories stay
historical. Persistence, actual HTTP/IC transports, namespace exclusivity,
provider completion/retention, billing cessation and restart/restore qualification
remain unimplemented or unproved. No full release gate, PocketIC journey or paid
effect ran for this local fix.

## Gateway sync and revocation after 0.1.5

Based on release `0e08c44`, the [scoped registry](../../crates/ic-blob-storage/src/model/gateway/registry/mod.rs)
owns bounded membership and one pending sync. Each new attempt receives a strictly
increasing local identity; scope includes service, provider namespace and Cashier.
Membership edits invalidate an earlier attempt even when the requested member
was absent/already present. Validation failures preserve membership and pending
state; explicit cancellation rejects later arrival of that result. A fresh sync
is a new authorized decision and may re-add a previously removed principal.

Unit tests cover cancellation/new attempt ordering, duplicate replies, exact
service/namespace/Cashier mismatch, invalid complete candidates, failed edits,
idempotent edits invalidating stale results, and sequence exhaustion without
losing the ability to revoke. Sequences are not reused and no wrapping/reset
operation exists. The counter does not claim safety across old backups or clones.

[Pure gateway policy](../../crates/ic-blob-storage/src/policy/gateway/mod.rs)
requires the object's service and namespace to match current registry/context,
then checks current membership. [Native composition](../../crates/ic-blob-storage/tests/gateway_binding.rs)
demonstrates that a stale sync cannot restore callback access after revocation;
tenant, service, Cashier and anonymous principals gain no implicit bypass.
No test simulates IC authentication or claims a callback proves deletion.

[Gateway reply ops](../../crates/ic-blob-storage/src/ops/caffeine/gateway/mod.rs)
decodes the selected `storage_gateway_list_v1` result (`vec principal`) and
applies it through the registry. Scope and pending-token checks precede parsing;
byte, decoding-work, skipping-work and type-table bounds constrain the parser.
Raw/distinct membership bounds apply after decoding, before replacement; they
do not prevent decoder allocations or bound future transport buffering.
Unit tests cover missing, truncated, incompatible and trailing data, each
decoder limit, duplicate normalization/order and complete candidate rejection.
Every failure leaves the full registry, including its pending attempt, unchanged;
a later valid reply may apply, or the caller may explicitly cancel the attempt.
The native callback test now passes actual Candid bytes through this operation.

The [independent fixture](../../crates/ic-blob-storage/tests/fixtures/caffeine-gateway/observed.hex)
was generated with didc 0.5.4 from the retained public
[gateway observation](caffeine-cashier-gateways.did):
`didc encode --types '(vec principal)' < docs/evidence/caffeine-cashier-gateways.did`.
An anonymous metadata read on 2026-09-26 reconfirmed byte-for-byte equality with
the retained [Cashier interface](caffeine-cashier.did), SHA-256
`232b08e4514048d4de48d6d1bf4387f577bfb64c7e2e2ded699a5e52d475d76f`.
This proves local codec compatibility with that schema, not the freshness or
authenticity of supplied replies. No new gateway query or provider effect ran.

`make test`, `make clippy`, `make wasm-check`, `make docs-check` and formatting
pass. Verify this new batch with `sha256sum -c docs/evidence/gateway-registry.sha256`.
Earlier 0.1.5 source inventories remain unchanged. These are native model/policy/codec
results only. The owning workflow must authorize edits, supply trusted response
context, recheck membership after awaits, maintain one authoritative registry,
persist identity/history and fence restoration before exposing actual endpoints.

## Account balance replies after 0.1.5

[Balance reply ops](../../crates/ic-blob-storage/src/ops/caffeine/balance/mod.rs)
decodes the retained Cashier `account_balance_get_v1` schema under explicit
byte, decoding-work, skipping-work and type-table limits. A successful report
must name the requested account and every amount must fit unsigned `u128`.
The reported total is preserved independently; no component sum or spendability
rule is invented. `AccountNotFound` and `InternalError` remain distinct provider
failures, never zero balances. Funding and query decoders now share one private
`AccountCycleBalances` schema; the released funding API is unchanged.

Unit tests cover all failure categories, account mismatch, special requested
principals, every negative/overflowing field and malformed/over-budget replies.
[Native readiness composition](../../crates/ic-blob-storage/tests/balance_readiness.rs)
keeps failed reads out of top-up arithmetic, distinguishes a genuine zero,
recovers diagnosis with later valid input and retains recovery fences.

The synthesized fixtures in `tests/fixtures/caffeine-balance/` within the crate
use didc 0.5.4 and the retained [Cashier interface](caffeine-cashier.did), whose
hash was reconfirmed during the preceding gateway work. They are not live account
observations. Reproduce with `didc encode '<value>' --types
'(AccountBalanceGetResult)' --defs docs/evidence/caffeine-cashier.did`:

| Fixture | Value |
| --- | --- |
| `success.hex` | `(variant { Ok = record { account = principal "rrkah-fqaaa-aaaaa-aaaaq-cai"; account_cycle_balances = record { total = 100 : int; cycles_prepaid = 80 : int; cycles_promo = 10 : int; cycles_ledger = 0 : int; debt_target = variant { Prepaid } } } })` |
| `zero.hex` | Same success value with all amounts zero and `debt_target = variant { Ledger }` |
| `negative-ledger.hex` | Same success value with `cycles_ledger = -1 : int` and `debt_target = variant { Ledger }` |
| `not-found.hex` | `(variant { Err = variant { AccountNotFound } })` |
| `internal.hex` | `(variant { Err = variant { InternalError = "fixture diagnostic" } })` |
| `unknown-error.hex` | `(variant { Err = variant { FutureError } })`, encoded without `--types`/`--defs` |

Native tests (including the existing funding fixtures), strict Clippy, Wasm,
rustdoc and formatting pass. Verify with
`sha256sum -c docs/evidence/balance-replies.sha256`; historical 0.1.5 inventories
remain unchanged. No live account query, provider effect or dependency change
occurred. The eventual workflow must establish source, service/namespace/account
authority, current-attempt correlation and freshness before using observations.
These reports cannot reconcile an uncertain payment or establish billing cessation.

## Consumer reference liveness after 0.1.6

Based on release `0992929`, the [lifecycle model](../../crates/ic-blob-storage/src/model/lifecycle/mod.rs)
can read a fully bound reference without mutation. Unknown/released references
return false, including while another reference keeps the object live.
[Pure liveness policy](../../crates/ic-blob-storage/src/policy/liveness/mod.rs)
checks direct-tenant authority against the trusted object before key details.
Batch reads enforce an explicit raw count bound, validate all bindings before
allocating results, and preserve input order and duplicate positions. Empty
batches still require authority; malformed scope never returns partial statuses.

Unit tests cover unknown/released/live entries, duplicates, empty input, exact
bounds and duplicate floods, all binding dimensions, unauthorized principals
and unchanged model state. [Native lifecycle composition](../../crates/ic-blob-storage/tests/lifecycle_binding.rs)
shows that a known root grants no read authority; reads remain inactive after
final release while physical bytes and billing obligations discharge separately.
Released-reference tombstones and immutable root claims remain intact.

`make test`, strict Clippy, Wasm, rustdoc and formatting pass. Source hashes:
`sha256sum -c docs/evidence/reference-liveness.sha256`. These are supplied-value
model/policy tests, not IC authentication, persisted lookup or provider evidence.
The eventual workflow must supply current trusted state and fence unreconciled
restores. Consumer reference inactivity is not a gateway deletion instruction or
proof that the object no longer exists at the provider.

## Transient catalog after 0.1.6

The [catalog model](../../crates/ic-blob-storage/src/model/catalog/mod.rs) composes
confirmed-object lifecycles, immutable service-wide root claims and exact reference
receipts under one mutable owner. There is no mutable journal escape or history
removal. Limits bound global/per-tenant lifetime objects and per-object metadata, tenant logical
bytes across namespaces, global physical bytes and unresolved billing bytes.
Logical release, physical deletion and final settlement free separate capacities;
settlement never frees lifetime identity slots. Derived counters retain zero-byte
obligation counts and use u128 byte/metadata totals on supported 32/64-bit targets.

[Pending pages](../../crates/ic-blob-storage/src/model/catalog/pending/mod.rs)
have separate scan/result budgets and service/namespace cursors. Empty filtered
pages advance; root order is stable. These are current views, not snapshots:
another sweep is required for objects that become pending behind a cursor.
[Catalog read policy](../../crates/ic-blob-storage/src/policy/catalog/mod.rs)
checks current gateway membership on every page/root batch and exposes tenant
usage only for the supplied authenticated caller. Root observations retain
order, duplicates and explicit unknown/malformed/wrong-namespace outcomes;
no provider deletion boolean is inferred from missing local data.

[Unit tests](../../crates/ic-blob-storage/src/model/catalog/tests/mod.rs) cover
atomic rejected admissions, immutable registration identity, exact retries at
capacity/after settlement, separate capacity recovery, receipt exhaustion with
reserved release capacity, zero-byte liabilities, totals above u64, scoped
pagination and rescanning after concurrent logical changes. Per-tenant object
quota tests retain zero-byte/settled history across namespaces, reject new
registrations atomically at capacity, accept exact retries and leave unused
global slots available to another tenant.
[Native composition](../../crates/ic-blob-storage/tests/catalog_journey.rs)
adds multi-tenant/multi-namespace accounting, reference replay through settlement,
cross-object rejection, revocation between pages and ordered root observations.

The [tenant catalog policy](../../crates/ic-blob-storage/src/policy/catalog/tenant/mod.rs)
adds bounded cross-object reference reads: context, raw count, ownership of every
root and full bindings are checked before allocating statuses. Unknown/foreign
roots share one typed rejection; duplicates and namespace-spanning owned reads
preserve order. Native tests cover every binding dimension, missing/foreign roots,
mixed batches, empty/oversized input and denial before receipt details.

Read-only exact receipt lookup shares its binding/actor/payload predicate with
mutation replay. Journal tests prove reads work at capacity and after settlement;
native composition separates recorded failure/success from current reference
liveness and proves all reads leave the complete catalog unchanged. Missing
receipts stay absent and consume no slot. These tests do not prove network query
authenticity or that missing local evidence makes an uncertain effect safe to retry.

All native tests, strict Clippy, Wasm, rustdoc and formatting pass. Verify with
`sha256sum -c docs/evidence/catalog.sha256`. The existing Unreleased reference-read
inventory was refreshed for shared module exports; released inventories are unchanged.
The named 0.1.7 draft also passes `make ci`, including release-helper fixtures and
offline package verification at the unchanged Cargo version 0.1.6. The release
plan selects 0.1.7 without mutating release files; no commit or publication ran.

This catalog records independently confirmed objects. It is not an upload
admission/reservation workflow: production must reserve durable capacity and root
history before provider authority/effects, not discover a full catalog afterward.
No provider call, new dependency, endpoint, stable schema or restoration was added.
Native confirmation tests supply evidence facts; they do not authenticate provider
deletion/billing reports or prove safe recovery after a restart.

## Streaming Caffeine identities after 0.1.7

The [streaming model](../../crates/ic-blob-storage/src/model/identity/caffeine/mod.rs)
computes raw SHA-256 and the nonempty provider root in one pass. Appends are
rechunked at 1 MiB; a fixed frontier folds domain-separated leaf/node hashes,
padding an uneven right subtree with the client's `UNBALANCED` marker at each
missing level. No content, full chunk or full tree is retained. Metadata lines
are bounded before allocation, trimmed with ECMAScript whitespace, framed and
sorted by UTF-16 code units before UTF-8 hashing. Original duplicate names reject;
upstream object entries cannot represent them. This is not HTTP validation or
header inference. Metadata setup allocates within explicit count/byte budgets;
stream state is fixed-size, independent of content length.

[Independent vectors](../../crates/ic-blob-storage/tests/fixtures/caffeine-hashing/vectors.json)
record source SHA-256, runtime and content recipes. Official `main` and npm
latest/integrity were refreshed on 2026-09-26 and remain at the pinned client
1.1.2 baseline. Generate vectors using the unmodified `YHash`/`BlobHashTree`
classes and isolated VM method in [provider review](../provider-review.md#client-algorithm-and-completion-observations):
create each recorded byte sequence, split at 1 MiB, call `YHash.fromChunk`, then
`BlobHashTree.build` with `Object.fromEntries` of the listed headers. Record the
root, ordered chunk hashes and Node crypto's raw SHA-256. No imports, gateway, certificate substitute
or provider call enters this experiment. Unusual Unicode headers check algorithm
compatibility only. The normalization follows ECMAScript
[whitespace](https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-white-space)
and [string sorting](https://tc39.es/ecma262/multipage/indexed-collections.html#sec-comparearrayelements).

[Native tests](../../crates/ic-blob-storage/tests/caffeine_hashing.rs) stream those
vectors with prime-sized and multi-chunk appends, reversing metadata input order.
They cover exact chunk boundaries and uneven trees through 18 leaves, including
rejected replay after complete leaves followed by correct continuation. Unit tests
cover every budget, algorithm length maximum without large allocation, duplicate
headers, offsets, truncation, corruption and metadata mismatch. All tests, strict
Clippy, Wasm, rustdoc and formatting pass. The source inventory
`docs/evidence/caffeine-hashing.sha256` is now historical: it matches `20ec33d`,
the validated source parent of release 0.1.8, rather than subsequent workspace edits.

The [chunk manifest](../../crates/ic-blob-storage/src/model/identity/caffeine/manifest/mod.rs)
checks an explicitly bounded list of chunk-hash values against one
expected root, using the same length, metadata and tree implementation. It
retains only the ordered hashes and declared length. Construction validates
consistency, not stored content. Individual leaf checks enforce index, exact
full/final length and domain-separated digest, hashing at most 1 MiB per call.
They are read-only: repeat/out-of-order verification never changes completion
state, because no completion bitmap or resume checkpoint is maintained.

The client-generated vectors now include leaf hashes, with previous roots and
raw digests unchanged. Native composition verifies every chunk in reverse order,
rejects corruption after a successful check, then accepts correct retry bytes.
Changed leaf/order/metadata and wrong indices/lengths reject with typed errors.
An independent repeated-content vector preserves identical leaf hashes at their
separate positions; neither manifest construction nor verification deduplicates them.
Unit tests include count/length budgets and show why an otherwise consistent
one-leaf tree does not authenticate declared length by itself. The caller still
needs trusted length and root provenance; future recovery must durably bind both.
Tests, strict Clippy, Wasm, rustdoc and formatting pass for the combined work.

This computes local consistency only. Expected identities need trusted provenance;
matching roots do not prove provider presence, durable upload or caller authority.
Empty provider objects reject as unqualified rather than inheriting the client's
empty-tree bug or inventing a root. No upload tree/proof, persisted checkpoint,
HTTP policy, endpoint or provider effect was added. The raw verifier's empty-byte
support and all released APIs remain unchanged.

## PocketIC authority probe after 0.1.8

On 2026-09-26, the unpublished [test canister](../../canisters/test/authority_probe/src/lib.rs)
and [host harness](../../tests/pocketic/tests/authority.rs) passed `make test-pocketic`.
The harness creates and installs actual Wasm with an explicit controller. Endpoints
capture `msg_caller` and `canister_self`, then use shared library policies/catalog
through test-only workflow and ops modules. No production endpoints are exported
by linking the core. The historical inventory `docs/evidence/pocketic-authority.sha256`
matches `832b364` (Cargo 0.1.8), the validated source parent of release 0.1.9.
It was verified against that Git revision after release; later changes do not rotate it.

Two supplied confirmed objects occupy 100 and 200 bytes in namespace 1. Bounds
are two lifetime objects, one per tenant, 300 physical/liability bytes, 200 logical
bytes per tenant, one reference and two receipt slots per object. Tests observe:

- Owners read their own usage/liveness. Other tenants, anonymous callers, the
  gateway and the actual controller cannot read or release the first reference.
  Forged tenant/service fields and unknown roots do not grant access.
- Owner release and exact replay leave its usage at zero, the other tenant at
  200 and only the released root pending. Denied calls preserve live state.
- Only current gateways read pending deletion. Unauthorized revocation leaves
  access intact; the explicit operator revokes membership and the next read fails.
  Operator authority comes from installation configuration, not controller status.

The [sync integration cases](../../tests/pocketic/tests/gateway_sync.rs) install a
second [controlled source](../../canisters/test/gateway_source/src/lib.rs). For
deterministic scheduling only, that source is also the explicit fixture operator:
it calls back into the probe before returning its captured list. This is not a
Cashier implementation or a product decision to trust providers as operators.
No sleeps, fixed tick counts or simulated platform callbacks determine the race.
Tests prove overlap rejects before another source request, revocation invalidates
an old response, and a completed newer sync survives the stale original callback.
The old gateway remains denied while the replacement can read pending deletion.
Malformed bytes, 4097-byte replies, empty membership and transport rejection each
preserve membership; observed request counts prove no automatic retry. A later
explicit valid sync succeeds after each failure. Probe decoding bounds are 4096
bytes, 100,000 decoding work, 1000 skipping work and 32 type entries; membership
is bounded to one raw/unique principal. These bounds do not limit IC transport buffering.
The private [fixture protocol](../../tests/protocol/src/lib.rs) owns controls and
typed test results. The source uses the CDK's documented
[manual reply mechanism](https://docs.rs/ic-cdk/0.20.3/ic_cdk/attr.update.html)
to return deliberately malformed bytes, without changing production codec code.

The host harness uses `ic-testkit` 0.10.0's full PocketIC re-export and explicitly
starts the provisioned PocketIC 16.0.0 binary. A managed server outlives its instance
and is dropped on success or panic. Sandbox loopback binding was denied; the
approved run with local socket permission passed. No external provider call ran.
Rust was `1.98.1 (48a229cea 2026-09-01)`; SHA-256 of observed artifacts:

- Authority Wasm: `bc5624296880e14ec8d4a9672cc20534ea82127a933563806515d5c76163d895`.
- Source Wasm: `1f980d554be5063445d2edcd270a8be41386880a123367ca60f0b9eaa1f8a9ba`.
- Server: `69e324bdb68d32d878b7a9504b1379f08f8d1921272bacb065b0fabb3d0f3792`.

Targeted `make check`, `make clippy`, `make test-native`, `make wasm-check`,
`make fmt-check`, `make package` and `make release-check` pass. The core normal/dev
graph excludes the simulator; the published archive excludes fixtures, protocol
and harness. `make test`
runs native and PocketIC targets sequentially; server provisioning stays explicit.
No full CI or release preparation ran. Sample objects substitute provider facts;
this partial A01/A06 evidence does not qualify upload, restore, production adapters,
provider callbacks, physical deletion or billing cessation. No persistence exists
in the fixture, and no upgrade/restart recovery is claimed.

## Transient chunk coverage after 0.1.9

The [chunk verifier](../../crates/ic-blob-storage/src/model/identity/caffeine/manifest/verification/mod.rs)
consumes a validated immutable manifest and allocates one bit per chunk, rounded
to bytes. Its manifest budgets bound lifetime memory; retries allocate no receipts
or retained content. Every call verifies exact index/length/hash before crediting
an unseen position, with at most 1 MiB hashed and constant-time bookkeeping.
Queries expose per-position status and exact unique chunk/byte totals. No bitmap
can be imported, and a fresh instance starts at zero.

[Independent client vectors](../../crates/ic-blob-storage/tests/caffeine_hashing.rs)
cover reverse-order reads leaving a middle gap, bitmap-byte boundaries, identical
hashes at different positions and partial final chunks. Repeated good chunks leave
progress unchanged; corrupt gap bytes reject before valid retry finishes coverage.
Adjacent unit tests cover invalid and maximum indices, short/oversized/corrupt
bytes before and after success, duplicate checks and fresh-instance reset.

On 2026-09-26, targeted verifier unit tests, the client-vector integration suite,
workspace Clippy, Wasm, rustdoc, formatting and offline package verification pass.
Source is release `2cacb1e` plus changes bound by
`sha256sum -c docs/evidence/chunk-verification.sha256`. No provider call, core dependency
change, full CI, version mutation or persisted workflow was introduced.
Coverage means successful observations only: it neither retains destination bytes
nor proves writes, durable resume, raw whole-file digest or provider completion.
A rejected duplicate does not erase a prior successful observation, but the new
bytes still return an error. Expected root/length and tenant provenance remain
external; current provider algorithms and serving semantics are not requalified.

The [ordered verifier](../../crates/ic-blob-storage/src/model/identity/caffeine/manifest/verification/ordered/mod.rs)
composes the same immutable manifest with the raw-content verifier. It admits only
the next exact leaf and checks bytes before advancing raw hash state. Each call
hashes at most 1 MiB twice and retains no file bytes or bitmap. Finalization consumes
the instance and returns both identities only after complete length and raw digest
match. Client-vector tests inject corrupt chunks before valid retries at every
position, reject skips/replays, and finalize the independent raw/root pair. Unit
cases reject incomplete input and an inconsistent expected raw digest despite
valid manifest bytes, plus invalid-length/index and post-completion appends.
The same targeted tests, Clippy, Wasm, rustdoc and package checks pass for the
combined addition; this source inventory includes both verification variants.

The [missing-chunk view](../../crates/ic-blob-storage/src/model/identity/caffeine/manifest/verification/missing/mod.rs)
adds independently bounded scans and result lists. Exact ranges come from the
manifest after index validation, including partial final chunks. Empty filtered
pages retain forward progress; a later call skips positions verified in between.
Client vectors exercise scan/result budgets (1/2, 3/1, 4/2); a sparse-gap case
crosses bitmap-byte boundaries and verifies an upcoming position between pages.
Unit cases show that range selection and scan exhaustion do not change coverage,
exact end positions return empty pages, and oversized indices reject before
offset multiplication. The same targeted validation passes. These are local
locations, not HTTP semantics, read reservations or persistent resume cursors.

On 2026-09-26, [PocketIC content cases](../../tests/pocketic/tests/content.rs)
executed the library's release Wasm through the unpublished probe. Fixed enum
requests select compiled independent vectors, with a maximum 1,048,577-byte
object, two leaves, eight headers and 1024 metadata bytes. The fixture generates
one leaf at a time from the recorded recipe; expected digests/roots remain those
of the pinned client. Reverse-order coverage, duplicates, empty-page continuation,
corrupt ordered retries and replay rejection pass. Wrong raw digest and truncated
finalization return their expected typed failures. Non-operator calls are denied.

`make test-pocketic RUST_TEST_NOCAPTURE=1` passes authority, content and sync cases.
Peak valid ordered-append instructions were 162,109,843 for both the full-1-MiB
and 1-MiB-plus-one-byte vectors, and 6352 for the three-byte Unicode metadata case.
The test budget is one billion instructions per valid ordered append, not an exact
count assertion. The measured interval excludes fixture byte generation, manifest
construction, metadata processing, Candid handling, earlier failed attempts and
other coverage checks. It is not a production throughput, ingress or pricing claim.

Observed authority-probe Wasm SHA-256:
`399be3259baf924ab1c8a6cea33afd93aaf541194f36e9f33d1e6e3c115dbc4e`.
The explicit PocketIC 16.0.0 server SHA-256 remains
`69e324bdb68d32d878b7a9504b1379f08f8d1921272bacb065b0fabb3d0f3792`.
The chunk-verification inventory now also binds the fixture, protocol and harness
sources. Workspace Clippy, Wasm, formatting and offline packaging pass; test-only
serde/JSON dependency edges reuse existing versions and leave the core graph
unchanged. No provider, persistence or production adapter was exercised.

## Transient upload admission after 0.1.10

The [admission owner](../../crates/ic-blob-storage/src/model/catalog/admission/mod.rs)
combines pending operations and its private confirmed catalog under the existing
byte/object bounds plus explicit concurrent-upload bounds. One shared root-claim
map prevents admission through another operation, including after cancellation.
An admitted operation permanently consumes a lifetime slot; no rejected request
leaks a claim. Every pending operation reserves its declared bytes in tenant
logical, global physical and global liability totals. Confirmed-object metadata
bounds reserve the first reference's eventual release receipt.

Exact tenant-scoped request IDs bind full object/first-reference identity, raw
digest, provider root and length. `reserve`, `phase`, exposure and cancellation
check the supplied authenticated tenant before looking up/replaying an operation.
Cancellation only releases bytes/concurrent slots while still Reserved. Once
ExposurePossible, neither cancellation, retries nor unrelated deletion/billing
observations free those reservations. There is no timeout/reset API. Completion
consumes a separately authenticated exact fact and transfers capacity into the
same catalog; replay after settlement cannot restore a reference or liability.

[Native tests](../../crates/ic-blob-storage/src/model/catalog/admission/tests/mod.rs)
exercise competing tenants and identical tenant-local IDs, each independent
capacity bound, exact/rejected retries, immutable service/namespace/incarnation/
reference/content inputs, root/object reuse rejection, cancellation, unresolved
exposure, out-of-order completion at capacity and separate logical/physical/
financial release. Zero-byte operations retain slots and totals above u64 remain
exact. Snapshots cover operation history, root claims and confirmed state on
rejection. Tests supply completion/deletion/billing facts as local substitutes.

The [read model](../../crates/ic-blob-storage/src/model/catalog/admission/read/mod.rs)
adds tenant-bounded operation scans with separate scan/result budgets. Terminal
history consumes scan work, empty filtered pages can continue, and cursors bind
service/tenant without granting access or a snapshot. The
[read policy](../../crates/ic-blob-storage/src/policy/catalog/upload/mod.rs) checks
the actual context on each page and includes reservations in tenant usage.
Gateway batches preserve input order, duplicates and malformed entries, report
reserved/uncertain/cancelled/confirmed state, and hide other namespaces. They
recheck current membership even for empty batches. No state maps to a provider
deletion boolean. Batch reads resolve unique roots through existing claim/catalog
lookups, then share one history scan for remaining pending/cancelled roots. The
scan stops as soon as all are found; batches with no such roots skip it entirely.
Temporary maps are bounded by unique valid input count, with no persistent index
or cross-call cache. Duplicates retain their output positions without repeating
history scans. The model's batch view reports actual scanned operation rows,
excluding tree lookups; policy does not disclose that global count to gateways.

[Read integration cases](../../crates/ic-blob-storage/tests/upload_reads.rs)
exercise budget combinations, sparse terminal history, cancellation/completion
between pages, new lower IDs, maximum IDs, tenant ID collisions, scope rejection
and phase/accounting consistency. Duplicate-heavy mixed batches match individual
root observations, visit history at most once, stop early and observe later
confirmation without stale results. Confirmed/unknown/malformed-only and empty
batches inspect no history rows. These views do not mutate operations or usage.
The [PocketIC upload case](../../tests/pocketic/tests/uploads.rs) executes four
fixed local upload facts: reserved/exposed roots for tenant A, confirmed/cancelled
roots for tenant B. Actual callers observe only their own uploads/usage; a real
controller has no tenant override. Foreign cancellation and owner cancellation
after exposure reject. Repeated pre-exposure cancellation frees capacity once,
and revocation immediately blocks the gateway observation. The fixture protocol
and initialization supply no provider transport or persisted state.

The [source inventory](upload-admission.sha256) binds the new implementation,
maintained dependencies, native tests and expanded fixtures at Cargo 0.1.10.
Targeted admission/read tests, `make test-native`, workspace Clippy, Wasm checks,
formatting, rustdoc and offline package verification pass; the archive includes
the new model/policy modules and native integration cases. `make test-pocketic`
passes the new upload case and
existing authority/content/sync cases using the explicit local server. Observed
authority-probe Wasm SHA-256 is
`9f15a6eb3ad3dac2e0797932018a768e6fdaa3379b81be4c06350ba70fc9ab42`;
the retained PocketIC 16.0.0 server is unchanged. No full CI/release gate ran.
The 0.1.10 chunk-verification inventory separately matches released source
`aaa5057`; it remains unchanged.

This is partial A01/A03/A04/A06 evidence, not production upload authority,
persistence, same-release recovery, provider liveness for pending uploads or a
provider protocol. The owner cannot be cloned/imported/serialized; creating it
fresh does not restore an existing installation. Byte liabilities are not a
currency bound. Independent provider qualification, restore fencing and durable
intent-before-effect storage still precede actual certificates or paid uploads.

## Memory dependency alignment after 0.1.11

The [core manifest](../../crates/ic-blob-storage/Cargo.toml) now depends on
`ic-memory` 0.14.3 instead of directly on stable-structures. The
[library export](../../crates/ic-blob-storage/src/lib.rs) exposes the same runtime,
collections and traits through `ic_blob_storage::ic_memory`. The published
package checksum is `b9368f37df84f896d04da5b980e0e038302c4da24892f89e00124c6d7cb953c7`.
Cargo resolved the cached registry package offline; a separate crates.io API
fetch returned HTTP 403, so this is not a fresh registry-latest claim.

Read-only source review found the same 0.14.3 requirement in Canic at
`3f825aa223e663a562a7cb1cca72e57b5703e0e9` and IcyDB at
`b6111f5f7185136b56860ee35287b206d9debafa`. Their published core package manifests
also specify it. An isolated Cargo graph with this local crate, canic-core
`=0.110.42` and icydb-core `=0.261.11` (both default features disabled) resolves:

```text
ic-stable-structures 0.7.2
└── ic-memory 0.14.3
    ├── canic-core 0.110.42
    ├── ic-blob-storage 0.1.11
    └── icydb-core 0.261.11
```

Reproduction inputs and `cargo tree --offline --locked -i ic-memory` / inverse
substrate output are retained under ignored `.tmp/memory-alignment/composition`.
This proves dependency resolution, not compilation or lifecycle qualification of
both frameworks together. Neither framework was added to the core dependency graph.
Canic's memory module owns configured bootstrap; IcyDB resolves IDs from committed
allocation authority. Future blob persistence must compose with that host, not
initialize an independent manager or override its policy/grants/bucket profile.

[Native composition tests](../../crates/ic-blob-storage/tests/memory_composition.rs)
exercise the public re-export: ordinary library use leaves linked store declarations
empty and memory unbootstrapped. Test-only host grants open distinct cells through
shared runtime handle types, retain separate values and preserve the host's bucket
configuration/capability. These keys/IDs are fixtures, not a selected blob schema.
No actual blob persistence, migration, canister restart or backup recovery is tested.

Native tests, workspace Clippy/Wasm, rustdoc, existing PocketIC regressions,
formatting and offline package verification pass. The
[source inventory](memory-alignment.sha256) binds this local batch at Cargo 0.1.11.
Released upload-admission hashes were verified against `3b5ab64` and remain unchanged.
No full CI, release action, provider effect or sibling edit ran.

The LOC helper was copied from Canic's `scripts/dev/cloc.sh` (source SHA-256
`d2198d05bf7f363c6b891f9112ce4ea4be9c11f5cbb54f816f084762ace12dcd`), replacing
its canic-specific directory filter with manifest-bearing `crates/*`. Bash syntax,
ShellCheck and real invocations from the repo and `/tmp` pass. Its path-based
classification counts inline test LOC in runtime files; the separate inline test
function count makes that limitation visible. No fixed LOC/count assertions or
extra CI dependency on cloc/jq were introduced.

## Tenant obligation views after 0.1.11

The [tenant object model](../../crates/ic-blob-storage/src/model/catalog/tenant/mod.rs)
adds bounded current pages of unsettled confirmed objects across a tenant's
namespaces. Live, DeletionPending and ProviderDeleted remain visible, including
zero-byte objects. Only explicit billing settlement removes an object from these
results; root claims, reference receipts and lifetime history remain retained.
Each result carries its complete binding and separate logical, physical and
liability bytes. These bytes are capacity units, not a measured currency amount.

A private per-tenant root index holds exactly one root per confirmed catalog
entry. It changes only on successful insertion and is bounded by the existing
global/per-tenant lifetime object limits. Exact replay and rejected registration
leave it unchanged. Tenant usage shares this index. Upload tenant usage also
selects the tenant's inclusive minimum/maximum request-ID range in the existing
operation map, avoiding foreign history scans without another index or cached
counters. Global usage continues to include every tenant. Pagination inspects at most
its scan budget and returns at most its result budget; foreign rows do not enter
the scan, count or cursor. Settled rows count toward scanning and can produce an
empty page with continuation. Each lookup observes current state, so a fresh sweep
is required for roots inserted behind a cursor. Pages prove no restore safety or
installation retirement condition, and exclude pending upload reservations.

The [tenant policy](../../crates/ic-blob-storage/src/policy/catalog/tenant/mod.rs)
checks actual caller/service context before cursor scope on every page. There
is no requested tenant or controller override. [Native tests](../../crates/ic-blob-storage/tests/tenant_obligations.rs)
cover budget combinations, sparse history, exact bindings, zero bytes, byte
projections, scope rejection, interleaved settlement, minimum/maximum roots,
confirmed index consistency and transfer from pending upload reservations.
Reads leave the catalog and receipts unchanged.

The admission module's mixed-phase unit test checks tenant/global totals for
reserved, exposed, live, deletion-pending, physically deleted, settled and
cancelled operations, including boundary request IDs, multiple namespaces and
neighboring tenants. Targeted admission and read regressions pass after the
tenant-range optimization; workspace Clippy/Wasm, rustdoc and PocketIC were rerun.

The [PocketIC case](../../tests/pocketic/tests/obligations.rs) runs actual caller
isolation and a release/deletion/billing sequence. A real controller receives no
foreign tenant view; an explicitly configured fixture operator can supply local
confirmation facts. Physical deletion empties the deletion queue while the tenant
still observes billing liability; only the separate settlement fact removes that
obligation. These operator facts substitute external evidence and do not model
an authenticated provider callback contract. The fixture remains transient.

Native tests, workspace Clippy/Wasm, rustdoc and PocketIC regressions pass.
The [source inventory](tenant-obligations.sha256) binds the changed model/policy,
native checks and fixture code with maintained dependencies at Cargo 0.1.11.
Authority-probe Wasm SHA-256:
`871b1f4963ce50aeb98ffd089a7f18fc11386ffd878b1d10fa00f7bb6019c9f3`.
This is partial BLOB-06 evidence; no persistence, provider effect, deployment,
version mutation or full CI/release gate ran.

## Funding callback experiment

After 0.1.12, the unpublished [funding probe](../../canisters/test/funding_probe/src/lib.rs)
and [PocketIC cases](../../tests/pocketic/tests/funding.rs) exercise real
unbounded inter-canister cycle transfers. The sender captures the system refund
immediately in the call continuation, before decoding or another await. The
receiver independently records available and accepted cycles. Existing Cashier
reply fixtures feed the production bounded decoder; no second provider schema
is declared by the fixture.

The cases verify zero, partial and full acceptance, typed InternalError,
malformed replies and explicit reject after partial acceptance. Refunds remain
correct across successive calls with different amounts. Decoded success is not
treated as the credited amount: even zero acceptance can accompany the controlled
success reply. Completed identities cannot send again. Driver/peer checks and
amount bounds reject before effects, and journals are private to the driver.
A receiver trap after accepting and journaling cycles rolls back both changes;
the sender observes a full refund and zero transport acceptance. The receiver's
previous receipts remain exact before and after upgrade, and a later independent
payment adds only its own receipt.

A deliberate sender callback trap rolls back the recorded completion while the
receiver's acceptance survives. The sender still observes its original pending
attempt, rejects its repeated identity and blocks a new operation. Both journals
write through host-owned ic-memory before returning from each mutation and
restore synchronously in post_upgrade. Same-release upgrades of both canisters
preserve exact attempts, receipts, driver/peer bindings and lifetime capacity.
Completed IDs remain rejected even with changed parameters; new identities can
proceed after observed completion while capacity remains. A trapped callback's
unresolved intent still blocks another payment after upgrade, and a full journal
remains full. Deliberate post_upgrade traps after synchronous restore on each
canister leave the previous executable and exact journals usable, with new
payments still blocked by unresolved intent; subsequent upgrades succeed.
The fixture owns this bounded experiment schema, not a production
service schema. Old backups, snapshot rollback, enqueue failure and bounded-wait
SYS_UNKNOWN were not exercised. No deployed Caffeine behavior or actual account
credit is established.

Validation: targeted native check, strict Clippy for fixture/protocol/harness,
format check, Wasm build and the funding integration target passed. PocketIC
16.0.0 is accessed through ic-testkit 0.10.0; localhost binding required sandbox
escalation. No additional external dependency or library API change was made.
The [source inventory](funding-callback.sha256) binds this experiment at Cargo
0.1.12. Funding-probe Wasm SHA-256:
`14d68ac2a156b7712aad055734d76e857c30bddc47828b28fcfe0e416998fb3b`.

Targeted reproduction after the fixture build:

```sh
cargo build --offline --locked --release --target wasm32-unknown-unknown -p blob-funding-probe --lib
POCKET_IC_BIN="$PWD/.tmp/tools/pocket-ic-16.0.0/pocket-ic" \
BLOB_FUNDING_PROBE_WASM="$PWD/target/wasm32-unknown-unknown/release/blob_funding_probe.wasm" \
cargo test --offline --locked -p ic-blob-storage-pocketic-tests --test funding
```

## Shared funding reconciliation after 0.1.13

The shared [transfer model](../../crates/ic-blob-storage/src/model/billing/transfer/mod.rs)
and [reconciliation policy](../../crates/ic-blob-storage/src/policy/billing/reconciliation/mod.rs)
now supply the fixture's transport arithmetic and diagnosis. A proven enqueue
failure has no callback refund and zero transfer; missing evidence keeps transfer
unknown. Valid unbounded refunds establish exact transport acceptance, independent
of reply decoding. Positive acceptance requires separate credit reconciliation,
and none of these diagnostics authorizes retry or clears account-wide activity.
Native cases cover full-range amounts, impossible refunds and retained uncertainty.

The local PocketIC fixture verifies these diagnoses against independent receiver
receipts for success, provider error, malformed replies, rejection and traps.
Its new enqueue-failure case requests more than the sender's observed balance,
within the fixture's explicit test-only attachment bound. The CDK cannot send it:
no callback refund is sampled, no receiver receipt appears, and exact unsent
history survives upgrades. A changed request cannot reuse that identity; a new
independent experiment still succeeds. Both fixture canisters now start with
explicit 2-trillion-cycle budgets instead of PocketIC's default allocation.
Previous callback/receiver rollback, failed upgrade and lifetime-limit cases pass.

Validation: targeted native billing tests, strict Clippy for the affected packages,
rustdoc with warnings denied, formatting, fixture Wasm build and the funding
PocketIC target passed. Reproduction uses the commands above. The current
[source inventory](funding-reconciliation.sha256) is at Cargo 0.1.13; the earlier
funding-callback inventory remains historical at `ab75523`. Current Wasm SHA-256:
`40f854068f2c2997838dd7a4d628a80a8653d11c2070833c5ecac2a794b65844`.

This is local transfer/policy evidence. Bounded timeout behavior, old-backup
fencing, production persistence and deployed Caffeine credit remain unqualified.

## Cashier audit response decoding

The [audit decoder](../../crates/ic-blob-storage/src/ops/caffeine/audit/mod.rs)
implements only `payment_account_audit_log_get_v1`'s advertised Candid response.
Anonymous metadata was refreshed on 2026-09-26 with
`icp canister metadata 72ch2-fiaaa-aaaar-qbsvq-cai candid:service --identity anonymous --network https://icp-api.io --root-key mainnet`;
its SHA-256 still matches the retained [interface](caffeine-cashier.did):
`232b08e4514048d4de48d6d1bf4387f577bfb64c7e2e2ded699a5e52d475d76f`.
No account, audit, ledger or paid provider call was made.

Supplied bytes and Candid work/type budgets are bounded before a report is exposed.
Separate limits cap retained UTF-8 CSV bytes and the provider-reported count.
Missing continuation while `has_more` is true rejects without a partial page.
Optional accounts, full-range sequences, terminal flags, CRLF and quoted/non-ASCII
text remain diagnostic data. CSV rows are not parsed or counted; no defaults,
sequence ordering, complete-history claim or retry authority are inferred.

Independent didc 0.5.4 fixtures come from the retained interface rather than Rust
serialization. [Fixture inputs](../../crates/ic-blob-storage/tests/fixtures/caffeine-audit/cases.json)
record the exact generator, source hash and synthetic values, including all three
provider errors. CSV column names are arbitrary test text, not a claimed provider
schema. Native cases additionally cover malformed/truncated/unknown replies,
byte/count/work/type/skip budgets, exact limits and UTF-8 byte accounting.

Targeted `cargo test --offline --locked -p ic-blob-storage --lib ops::caffeine::audit`,
strict library Clippy, rustdoc with warnings denied, formatting and the Wasm target
check pass. All stored fixtures reproduce byte-for-byte through didc.
The [source inventory](cashier-audit.sha256) binds the decoder and independent
fixtures at Cargo 0.1.13. Binding an authenticated transport response to its exact
service/account/filter/operation, interpreting CSV, cursor progression, retention
and deployed credit reconciliation remain unqualified.

## Connected upload and deletion journey after 0.1.14

The [journey test](../../tests/pocketic/tests/journey.rs) drives an initially empty
shared UploadCatalog inside the authority fixture. Current source selection and
explicit local semantic differences are recorded in the
[protocol snapshot](upload-deletion-protocol.json). The four source method names,
argument/reply shapes and query/update modes are used directly by the harness.

Admission binds the actual tenant to an exact immutable request and reserves
logical, physical and liability capacity. Actual content must pass the shared
manifest and ordered verifier against the reserved root, length and raw digest
before a certificate response. This fixture accepts nonempty files up to 6 MiB
in at most six 1 MiB chunks, plus at most eight headers/1 KiB of framed header
input. It discards checked bytes, retaining only bounded manifest and hash state
across messages. The transient fixture model composes the shared ordered verifier
with exact old-chunk checking; no production model or checkpoint format changes.
The independent `abc`, partial-final-chunk and uneven-tree-with-metadata vectors
drive actual installed Wasm. Truncated, extra, corrupt and mismatched-digest bytes reject without
issuing authority or releasing reservations. Only the bound tenant can verify;
exact chunk/re-reservation retries preserve the verified prefix and never feed
the raw hash twice. Tenant-only progress separates checked leaf bytes from the
final raw-digest verdict. Final digest failure is terminal for the declaration;
replaying admission cannot reset it. Changed metadata, excessive header/chunk
inputs and skipped indices reject; canonically equivalent reordered headers
preserve progress. A changed digest cannot replace the reservation, and verification alone cannot confirm
provider storage. Certificate responses mark possible
exposure before returning; repeats cannot issue fresh authority. An interrupted
upload remains protected and charged, cannot be cancelled/released/deleted, and
requires a separate supplied completion fact. Exact replay leaves accounting
unchanged; conflicting parameters, cross-tenant roots and excessive amounts reject.

After completion, logical release leaves physical and billing obligations. A real
second canister delivers binary-root deletion batches. If a later root is still
live, the actual IC rolls back earlier changes in the same callback message.
Duplicate and delayed callbacks cannot reactivate objects or affect a newer root;
revoked gateways cannot apply even empty callbacks. Physical deletion frees only
physical capacity; a separate operator-supplied billing fact releases liability.
Malformed/oversized batches, unauthorized controller/tenant/gateway use, and
pre-exposure cancellation also have observable rejection/accounting checks.
Partial files remain charged until cancellation; cancelling before exposure frees
capacity without allowing the retained identity or prefix to issue authority.
Two tenants can interleave work without resetting or observing each other's
prefixes. Lifetime session/object bounds remain eight global/four per tenant;
global physical/liability capacity is 12 MiB and tenant logical capacity 6 MiB.

The [readback cases](../../tests/pocketic/tests/readback/mod.rs) fetch individual
leaves through real calls to the controlled source canister. Before and after
the await, shared tenant/reference liveness and gateway policy check the complete
admitted binding. Exact manifest length/hash checks precede returning any bytes.
Reads may select leaves out of order and repeat them; they neither advance upload
progress nor release accounting. This verifies individual returned chunks, not
a new whole-file observation, provider durability or a billing/completion receipt.

One service read slot admits at most one pending call. Revocation and successful
gateway synchronization invalidate it without releasing capacity early; only its
exact callback frees the slot. Native tests additionally reject an old callback
against a newer slot. An encoded reply is capped at 1 MiB + 64 bytes before Candid
decoding, with work/skip/type budgets of 10,000,000/64/8. These application bounds
do not cap the platform's earlier reply buffering or prove remote work stopped
after a transport failure. No automatic retry occurs.

The source retains one driver-configured leaf of at most 1 MiB and captures one
reply while held. A bounded chain of up to 128 local raw_rand callbacks yields
across rounds; random bytes are unused. This is fixture scheduling, not a
provider protocol. PocketIC proves no disclosure after release while awaiting a
reply, or after gateway revocation followed by re-addition. Corrupt/truncated,
wrong-file, oversized, malformed and rejected responses return no bytes; a later
explicit valid read still works. Source controls remain driver-only and the source
read endpoint accepts only the installed service. No live HTTP read was performed.

The [lifecycle cases](../../tests/pocketic/tests/recovery/mod.rs) define the narrow
supported boundary of these transient fixtures. Ordinary IC stop/start retains
chunk progress, exposed reservations and accounting; verified prefixes continue
without rehashing duplicate chunks. The authority rejects upgrades in pre_upgrade
and post_upgrade. Actual authority upgrades, including chunked installation with the
outgoing hook skipped, reject with typed canister errors and leave the original
instance usable. Tests retain cancelled root claims, unresolved upload exposure,
billing after physical deletion and a held read that still rejects after release.

An operator-only fault control binds a deliberate callback trap to the next read
of a named admitted root. The trap runs after attempted slot release. Actual IC
rollback preserves the admission from before the source call, while the source's
request observation survives. No further source read occurs; stop/start and a
simulated day passing cannot clear the blocked slot or its accounting. This is
failure-containment evidence, not successful read recovery. No persisted upload
journal, unfence command, snapshot-load protection or surviving recovery
authority was added. Controller reinstall and old-snapshot loads are outside this
supported boundary; lifecycle-hook rejection must not be described as fencing them.

The controlled source now has a same-release journal in its host-owned ic-memory
cell (`fixture.source.journal.v1`, granted memory ID 120). It retains explicit
service/gateway/driver bindings, source configuration, observations, one leaf up
to 1 MiB, pending/ready read flags and at most 64 lifetime outgoing actions. Exact
sync/revoke/delete intent is committed before dispatch; deletion payloads retain
up to eight roots. Callback observations never recycle history and do not establish
provider effects. A callback trap leaves its action unresolved. Journal encoding
is bounded to 1,114,112 bytes and decoding has work/skip/type budgets of
30,000,000/10,000/64. These are local experiment limits and a small full-cell
checkpoint strategy, not a production storage recommendation.

Source post_upgrade synchronously loads/validates the journal and permanently
fences operational endpoints before returning. The original driver may inspect
retained bytes/history; controllers and tenants gain no such authority. There is
no unfence/reset/retry control. Ordinary upgrades reject a pending read or unknown
effect. A forced skip-pre_upgrade restoration still retains and fences them.
PocketIC covers the maximum leaf together with full action history, exhausted
capacity, successful and rejected call results, unresolved callback intent,
missing stable data with failed-upgrade rollback, repeated upgrades, and older
stable bytes whose request counter predates an observed read. None authorizes new
effects. Restoring a journal captured during a held read preserves the pending
marker and denies resume/reconfiguration without releasing it. This stable-memory
injection experiment is not a whole-canister snapshot
load: restoring an old heap can bypass post_upgrade entirely. Upload journals and
independent recovery authority remain unimplemented.

The authority's [inspection archive](../../canisters/test/authority_probe/src/ops/archive/mod.rs)
now records all three fixture owners in a separate host-owned ic-memory cell
(`fixture.authority.archive.v1`, memory ID 120). The encoded archive is capped at
65,536 bytes; Candid work/skip/type budgets are 2,000,000/10,000/64. It retains
the two confirmed samples, four sample uploads and at most eight journey uploads,
including cancelled/settled identities, declared lengths/digests, release request
1's original result and charged logical/physical/liability bytes. Namespace,
incarnation and reference identity remain the fixture's explicit fixed values.
Coverage assertions reject mutations if the fixture grows histories the archive
does not represent. Manifests retain original accepted headers/leaf hashes; each
verification prefix and independent terminal digest verdict is recorded without
retaining file bytes or serializing the streaming hash state.

All state mutations share an archive commit in the same IC message, including
error-valued terminal digest failure. Admission/exposure and exact read intent
are saved before replies or source effects. Read intent includes tenant, root,
index, gateway and token; revocation records invalidation without clearing it.
Gateway scope, current members and last/pending sync sequences are retained too.
The shared registry's additive `sync_view` exposes observations only, never tokens.
PocketIC proves stable rollback alongside live state after a mixed invalid
deletion batch and an actual read callback trap. It also covers full lifetime
history, sample and journey billing, partial verification, settled receipts,
reentrant syncs and inspection by the operator independently of controller status.

Inspection reopens stable memory, with bounded decoding and no empty fallback.
Direct PocketIC injection of old or missing stable data is followed by an existing
stateless update so the IC query cache cannot mask the fault. Old data is observed
as old evidence, missing data rejects inspection, and neither feeds active
admission or changes accounting. This is an atomic inspection archive, not a
lossless operational checkpoint: authority pre/post_upgrade still reject, since
streaming SHA state and supported reconstruction are not implemented. Snapshot
loads and independent restore authority remain outside the demonstrated boundary.

Strict Clippy for the affected fixture/protocol/harness packages, formatting and
both fixture Wasm builds pass. Targeted native raw-identity tests cover the additive
32-byte raw-digest parser and typed length errors. PocketIC journey, content,
authority, uploads, obligations
and gateway-sync targets pass with ic-testkit's exported PocketIC 16.0.0. Run the
journey target with the existing `POCKET_IC_BIN`, `BLOB_AUTHORITY_PROBE_WASM` and
`BLOB_GATEWAY_SOURCE_WASM` environment variables after building the fixtures.
The [source inventory](upload-deletion-journey.sha256) binds the selected sources
at Cargo 0.1.14. Wasm SHA-256:

- authority probe: `6e29a589f95d3b927bc964737cc9d1196863ece8c0f2bd31d77d27fc329b90dd`
- gateway source: `584b5ebd120855239721fb4d03f7b45e7bd068cdaf1668ddf26b0ec2a9e720d4`

This is a transient local composition experiment. File roots and digests
are checked against actual bytes; upload completion and final billing facts
are substitutes. This does not select a production service-mediated upload
architecture or qualify the client's HTTP upload path. Metadata hashing is
checked locally; headers are not interpreted as HTTP behavior. Progress survives
messages and stop/start, not successful upgrades or restore, and is never a
provider receipt.
The refreshed source snapshot records the V4 certificate forwarding and discarded
chunk-completion flag; no gateway verifier or completion lookup was established.
The certificate-shaped
reply is not tested against gateway verification. Provider HTTP behavior, durable
upload recovery, external deletion atomicity and old-backup safety remain open.

## Verification checkpoints after 0.1.15

The [ordered verifier checkpoint](../../crates/ic-blob-storage/src/model/identity/caffeine/manifest/verification/ordered/checkpoint/mod.rs)
adds a 256-byte, exact-library-release record for trusted host storage. It binds
format v1, release, manifest root, expected raw digest/length, accepted byte count
and the pinned SHA-256 state. A SHA-256 checksum detects accidental damage; it
cannot authenticate, establish freshness or prevent deliberate forgery/rollback.
Restoration requires a separately retained, validated manifest and expected digest.
Root equality cannot replace the independent length check. No method creates
service/tenant authority, a provider receipt or a fresh operation identity.

The implementation uses the checked local sha2 0.11.0 `SerializableState` API
(digest 0.11.3, block-buffer 0.12.1 from Cargo.lock). Its 104-byte state comprises
32 chaining bytes, an eight-byte little-endian block counter and a 64-byte eager
buffer encoding. Decode validates canonical buffer state, checked block/byte
arithmetic, the SHA length limit and whole-leaf prefix alignment (or complete
content). Up to 63 plaintext bytes can be buffered: checkpoint Debug is redacted,
and the authority's inspection DTO omits the private checkpoint bytes entirely.
This representation has no cross-release reader or migration path.

The authority fixture stores the private checkpoint beside each pending session's
manifest/progress in the same atomic archive write. Terminal verified/rejected
sessions have no pending hash bytes. `ContentSession::from_record` cross-checks
checkpoint progress and terminal invariants. An operator-only query reconstructs
an isolated copy using the same manifest validation as admission, checks one
supplied chunk and discards the copy. It cannot advance live progress, accounting,
certificate eligibility or external effects. This probe establishes bounded
mathematical reconstruction from stable memory independently of lifecycle recovery.

The authority now restores the full archive synchronously into a permanently
fenced inspection owner. The archive binds the workspace Cargo.lock digest,
covering release/dependency selection even without pending hash state. Disposable
catalogs validate all three catalogs through the shared transitions and compare
every identity, receipt, phase, charge, manifest and checkpoint. Completed history
is rebuilt before charged entries to avoid transient capacity conflicts; the
original journal ordering is retained. These disposable models are discarded,
and no operational catalog/registry is installed. Read/sync identities, invalidation
and fault plans remain evidence rather than reusable tokens. The restored owner
serves inspection from its validated record. All operational endpoints reject,
including provider liveness/deletion queries and callback entry points.

Ordinary upgrades reject active pending reads/syncs. Forced upgrades still validate
and fence the entire archive; repeated upgrades cannot clear that fence. PocketIC
covers partial verification after normal/repeat/skipped-hook upgrades, retained
sample/journey receipts and billing, full lifetime history, old archives, missing
journal rollback, corrupt-checkpoint rollback and controller/operator separation.
Actual trapped read intents remain exact through forced restoration with no source
replay. Native tests additionally reject contradictory records and check retained
sync counter limits.

The current source additionally persists a required `SyncRecord` containing the
exact held gateway-list sequence, captured gateway and driver-release state. Its
bounded management-call schedule is shared with held chunk reads; random bytes
are discarded. PocketIC captures both journals during an actual pending sync,
rejects busy upgrades/overlap and unauthorized release, completes the live call,
then restores the older journals. Authority and source remain fenced with the
pending identities and unresolved outgoing effect intact. A delayed held reply
cannot undo revocation. Exhausting the local hold clears only the read-only
attempt; no automatic retry occurs. This is controlled local scheduling, not
Caffeine latency, retry or retention evidence.

Large-vector cases retain cancelled/settled identity history exceeding the 12 MiB
active byte budget, fill both tenants' lifetime slots and recover exact charges,
receipts, manifests and verification checkpoints. Root reuse and new lifetime
admission still reject before restore. Another case releases physical capacity
while retaining billing liability: new admission fails until settlement, and an
older journal restores with those liabilities and a permanent fence. The journey
and gateway-sync targets, strict affected-package Clippy and Wasm builds pass.
No checksum authenticates external facts or proves freshness. Snapshot loads can
bypass lifecycle hooks; independent recovery authority and production operational
restoration remain unimplemented. Fixture archives hard-cut the previous form;
there is no compatibility reader, public catalog-import API or unfence endpoint.

Native tests reconstruct at SHA block/padding boundaries and at every leaf of
independent client vectors, including metadata and non-aligned final buffers.
They reject format/release/checksum damage, malformed/counter-inconsistent state,
changed declarations and wrong raw digests. PocketIC probes complete one-byte
and metadata-bearing tails from protected stable state, reject corruption and
wrong digests, preserve live authority, and deny controller-only access. Existing
journey, content, authority, uploads, obligations and gateway-sync targets also
pass. The targeted identity unit tests, strict affected-package Clippy, rustdoc
with warnings denied, both fixture Wasm builds and formatting pass.

The [current source inventory](verification-checkpoints.sha256) covers this work
at Cargo 0.1.15. The released upload-deletion inventory remains unchanged and
matches source `481fe67` (Cargo 0.1.14). Pre-balance-follow-up Wasm SHA-256:

- authority probe: `7a517f9daf81134a7b63361926ed726694e01398e2841374b242873cd39c7f66`
- gateway source: `189d9bd02eaac02df4ba13d52789d7db5b13d17b97b8bb7c5447748796838e1e`

## Read-only operator diagnosis after 0.1.15

The additive shared `policy::diagnostics` composes existing billing results with
independently supplied recovery/provider state, funding-journal observations and
pending upload/read/sync/delete work. Missing billing observations or funding
history remain blockers rather than zero balances or clear activity. A stricter
billing recovery fence is retained, with duplicate recovery/gateway blockers
merged. Outstanding work remains visible as warnings independently of global
blockers. The result is not overall service readiness or an effect permit.
Native cases cover unavailable/malformed balances, reserve violations, simultaneous
blockers, uncertain funding despite healthy balances and non-mutating composition.

The unpublished authority fixture adds an explicit `operator_status` query.
It snapshots the current active owner or the validated frozen inspection owner;
older/missing stable evidence does not replace live status before restoration.
It reports three independent catalog summaries (phases, release receipts and
logical/physical/billing-liability bytes), gateway membership and exact pending
read/sync observations. It exposes no hash-state checkpoint bytes. Billing
configuration is absent; provider balance, spendable funding cycles and funding
activity are unobserved. Gross canister cycles are not spendable-accounting proof.
The fixture remains provider-unqualified, and restored instances report the fence.
Status is not a stable-journal integrity check, recovery action or provider call.

Actual PocketIC queries deny tenant/controller-only callers and preserve both
stable journals and the source's effect history. They retain separately released
physical/billing charges and settled receipts, expose held and invalidated reads
while syncs are pending, and preserve those observations under old-journal restore.
Targeted native policy tests, affected-package strict Clippy, warning-free rustdoc,
both Wasm builds and journey/content/authority/uploads/obligations/gateway-sync
PocketIC targets pass. No live provider observation or paid qualification was added.
Sources are bound by the current checkpoint inventory above; the latest Wasm
hashes are in the scoped-balance follow-up below. Released inventories remain unchanged. This adds partial BLOB-06/12/17 and
A04/A05/A11 evidence; configured economic observations,
the final operator client and both production adapters remain follow-up work.

## Funding operator diagnosis after 0.1.15

The funding fixture's driver-only `operator_status` reads its current journal,
bound to the answering canister and retained local peer. It reconstructs shared
`FundingTransfer` facts from each exact original attachment and callback refund or
proven enqueue failure, then applies shared reconciliation. Contradictory transport
facts stay unknown; a stored derived label cannot override the retained facts.
Shared `assess_uncredited_activity` considers the entire outgoing history: a later
no-transfer result cannot erase earlier unknown transfers or unverified credit.
Incoming local acceptance receipts are separately reported, never credited amounts.

Provider qualification/configuration, balance and spendable reservations are absent.
The peer is not a validated storage gateway. The shared operator diagnosis now
accepts missing recovery assessment as an explicit blocker for the active experiment.
The restoration follow-up below now enforces an inspection-only fence; it does not
assert reconciliation. Before restore this remains an experiment that can admit
distinct completed transport cases, not a production funding workflow. No status
result grants a retry.

PocketIC exercises the query alongside zero/partial/full acceptance, provider errors,
malformed/rejected replies, actual sender callback rollback, receiver rollback and
insufficient-cycle enqueue failure. Sender uncertainty persists despite independently
visible receiver acceptance, successful/failed upgrades and elapsed time. Positive
acceptance still requires provider credit after later full refunds or unsent calls.
Lifetime history survives upgrades. Controller/peer/unrelated callers cannot inspect
status; repeated driver queries leave both stable journals and histories unchanged.

Targeted checks passed: shared policy unit tests, fixture conversion unit test,
affected-package strict Clippy, warning-free library rustdoc, three fixture Wasm
builds, the funding PocketIC target and authority operator-status regression cases.
This is partial BLOB-11/12/17 and A08/A11 evidence, not deployed credit reconciliation,
old-backup safety, a production adapter or the final operator client. The current
[source inventory](verification-checkpoints.sha256) includes these additions;
released funding inventories remain unchanged. Funding fixture Wasm SHA-256:
`08fc94562543eb14c2a6b5dd2b809077cc90da82909080a167c6b679e74a7af4`.

## Fenced funding restoration after 0.1.15

The unpublished funding journal now requires actual service, Cargo.lock release
binding and a persisted fence. This hard-cuts the prior fixture schema without a
migration/dual reader. Before restoration, model validation checks principal/release
bindings, lifetime capacities, unique outgoing/incoming identities, positive bounded
attachments, conserved refunds/acceptance, consistent receipts and at most one final
unresolved intent. A callback configured to trap cannot have a retained completion.
Workflow compares retained reconciliation with shared policy before ops commits the
permanent fence and installs the inspection owner synchronously.

All sending and receiver acceptance are denied after restore. Completion requires
an active owner and the exact admitted request, so a late callback cannot turn the
restored journal active or clear uncertainty. Authorized inspection retains intents,
refunds and incoming receipts; active owners still report recovery as unknown and
restored owners report the enforced fence. No old local counter, receipt, provider
balance or transport success grants reactivation. No reset/unfence endpoint exists.

Actual PocketIC cases exercise:

- Older empty sender journals after payment, with surviving receiver receipts,
  reject both reused and new identities through repeat/skip-hook upgrades and restart.
- Older receiver journals reject a real attached-cycle call before acceptance;
  the active sender records the full refund separately from earlier unverified credit.
- Captured real in-flight journals restore as unknown after the live call completes.
  A delayed-success substitute commits acceptance before bounded management-canister
  scheduling calls; it changes no provider protocol or claimed provider behavior.
- Upgrading the sender while its reply is outstanding leaves the intent unresolved
  when the actual late callback arrives. No transfer is repeated.
- Upgrading the receiver after acceptance but before reply retains its receipt;
  the sender observes the exact remaining refund and still needs provider credit.
- Missing, corrupt, foreign-service and wrong-release journals reject atomically;
  failed post-upgrade hooks roll back the attempted fence. Original active owners
  remain usable, with their previous obligations intact.

Existing exact refund, enqueue-failure, callback/receiver-trap, caller-isolation and
lifetime-history cases pass under the maintained inspection-only restore contract.
Native validation/conversion tests, strict affected-package Clippy, warning-free
fixture/protocol rustdoc and three fixture Wasm builds pass. The authority/source
Wasm hashes above are unchanged; funding uses the updated hash above. The current
[inventory](verification-checkpoints.sha256) includes all new source and tests;
released evidence remains historical. This adds partial BLOB-11/12/13 and A05/A08/A11
coverage. It does not qualify whole-canister snapshot loads that bypass hooks,
independent recovery authority, production account credit or either service adapter.

## Read-only fixture client after 0.1.15

`blob-fixture-status` is a host-only, unpublished binary in the existing PocketIC
package. It reuses ic-testkit 0.10.0's exported PocketIC 16.0.0; no dependency or
production provider interface was added. Local Canic CLI/medic source at
`3f825aa223e663a562a7cb1cca72e57b5703e0e9` was inspected read-only. Its metadata-based
query/update selection and method-name discovery are capability inputs, not
permission to invoke updates during inspection.

The client requires a literal loopback socket, explicit instance, canister and
simulated caller, plus expected authority namespace or funding peer. It attaches
without owning the instance. Its only method is the `operator_status` query; no
method override, update fallback or application-level retry exists. The SDK may
poll/busy-retry its read request. Its 10-second request-processing setting does not
bound a stalled HTTP socket, and reply-byte limits apply after SDK reception.
This is a local harness tool, not production authentication or hardened networking.

Candid inspection rejects replies over 65,536 bytes, with decoding/skipping quotas
of 1,000,000/10,000 and type-table length 64. Answering service and namespace/peer
must match the selection. JSON reports label simulator scope and unassessed service
qualification. Full-width counters, bytes and cycles are decimal strings; absent
observations remain null. Catalog byte charges remain separate; funding acceptance
never becomes provider credit. Shared server diagnosis is projected without
recomputing admission policy. `status` succeeds on a valid report even if blocked;
`check` exits 4 when its typed report contains blockers. Invalid arguments exit 2;
transport/query/permission/decode/binding errors exit 3 with stable error tags.

The actual binary runs as subprocesses in `tests/pocketic/tests/operator.rs`:

- Authorized status/check preserve authority, source and funding stable journals,
  and dropping the attached client leaves the parent's instance usable.
- An explicit controller without operator authority is denied. Wrong namespace or
  funding peer, missing methods and invalid instances produce failure signals.
- A tiny real Wasm exports only an update named `operator_status`. Client inspection
  rejects without changing stable memory. An explicit control update then grows
  memory, proving the witness actually has a callable mutation.
- An accepted local transfer remains uncredited and uncertain, with exact operation
  identity/refund/acceptance visible through the binary and after fenced restoration.
- Authority restoration remains visible as an enforced fence; inspection does not
  change retained journals or reactivate operations.

Native cases cover ambiguous/missing/remote inputs, full-width Candid-to-JSON values,
unknowns, truncated/oversized replies and service/peer mismatch. Targeted native tests,
strict protocol/client Clippy, warning-free rustdoc, host-client Wasm checks, all three
fixture Wasm builds and the operator PocketIC target pass. The normal
`make test-pocketic` package selection includes these cases; it was not run as a
full suite for this slice. The current [source inventory](verification-checkpoints.sha256)
includes all client/test sources; released inventories are unchanged. Client-slice
artifacts before the scoped-balance follow-up below:

- authority: `7a517f9daf81134a7b63361926ed726694e01398e2841374b242873cd39c7f66`
- gateway source: `189d9bd02eaac02df4ba13d52789d7db5b13d17b97b8bb7c5447748796838e1e`
- funding: `08fc94562543eb14c2a6b5dd2b809077cc90da82909080a167c6b679e74a7af4`

This supplies partial BLOB-12/15/16/17 and A11 evidence. Production transport,
identity/discovery, configured observations, sync/funding commands and both adapters
remain open; neither fixture success nor an empty blocker list qualifies the service.

## Scoped fixture balance observations after 0.1.15

The authority fixture now explicitly configures a local balance-read scope containing
service, namespace, source canister and requested account. This is separate from
billing limits and funding-account authority. `refresh_balance` admits one pending
read, persists intent before dispatch and attaches no cycles. Sixteen lifetime attempts
retain their exact scope, configuration revision, dispatch time and terminal result;
there is no reset or recycling. Reconfiguring even the same scope invalidates older
replies without freeing an outstanding slot. Callback completion matches the exact
slot/scope and current revision. Only the explicit operator can configure or refresh.

The source's `fixture_balance` method is a deliberate local substitute. It checks the
calling authority and explicit account, returns driver-configured independent fixture
bytes, and can reject or hold a reply across bounded IC scheduling rounds. It permits
one pending reply, 64 lifetime receives and at most 4,097 configured bytes. Its existing
ic-memory journal now retains this state. No production Cashier request/interface was
recreated, and no upstream currency refresh or live provider call occurred in this slice.

The maintained `decode_balance_reply` owns the response contract: 4,096 reply bytes,
100,000 decoding work, 1,000 skipping work and 32 type entries. Independent existing
Candid vectors prove total/prepaid/promotional/ledger conversion, valid zero, negative
components, provider errors and account mismatch. Rejected, malformed and oversized
reads produce distinct retained failures, never synthetic zero. Transport buffering
is governed separately by the IC and is not bounded by this post-receive decoder.

Passive status/CLI projection retains all attempts. A current displayed total requires
the latest configuration and successful latest reply, an unfenced owner, and local age
at most 30 seconds from dispatch. Receipt after a long delay cannot freshen a report;
a clock before dispatch/receipt is unusable. Expiry changes only the view, retaining
history. This local observation-age rule cannot prove provider freshness. Unknown
spendable funds and funding activity remain unknown, billing limits remain absent and
shared operator diagnosis retains its blockers. No report supplies provider credit,
funding permission, a readiness claim or an automatic refresh.

Both fixture records require the added fields under v1: a pre-1.0 hard cut, without
migration or fallback. Incoming authority validation checks all retained scopes,
revisions, bounds, pending-slot placement and outcome consistency. Ordinary busy
upgrades reject; forced and older-journal restores remain permanently fenced. Actual
callbacks can trap after a forced upgrade clears their async heap; the unresolved
intent remains durable and inspection-only. Source restoration likewise prevents
replying/resuming and preserves the captured pending request.

Native journal cases cover exact identity, repeated configuration, clock bounds,
full-width totals, lifetime capacity and invalid records. Actual PocketIC/CLI cases
in `tests/pocketic/tests/operator_balance/mod.rs` cover:

- Passive inspection before/after refresh leaves all journals and source request
  counts unchanged; JSON exposes exact independent components and unknown economics.
- Provider-shaped errors, malformed bytes/amounts, account mismatch and actual reject
  preserve prior history and never become zero; an independent valid zero remains zero.
- Held replies cannot overlap or apply after configuration revision changes. Slow
  replies expire from dispatch; a separately explicit refresh can observe again.
- Captured old pending journals restore fenced on both sides after the newer live
  call completed. Repeat restoration neither sends nor completes an attempt.
- Forced upgrades during live authority/source callbacks preserve pending evidence
  and prevent stale completion. Ordinary busy upgrades reject atomically.
- Explicit controller denial and wrong namespace fail before calls. Lifetime capacity
  rejects without changing history; full status fits the client decoder and restores.

Targeted authority/client unit tests, affected-package strict Clippy, warning-free
rustdoc, host-client Wasm checks, three fixture Wasm builds and operator/journey/
gateway-sync PocketIC targets pass. The current [source inventory](verification-checkpoints.sha256)
includes these additions and the independent balance vectors; released inventories
remain unchanged. Current Wasm SHA-256 for this slice:

- authority: `bbefbf02cc57d55292eebd138e77af05147e6129786363f17c4ec45ddf52bf5d`
- gateway source: `bb089ade0904825b30c2b186801f338ce339ec35581a82ceb837b0dce95511b0`
- funding: `183da9c2626589709487e58d4da045d0368d0666a322264cab0c9058b019a845`

This adds partial BLOB-09/12/13/17 and A05/A08/A11 coverage. Configured billing-policy
composition with unknown spendability, actual provider/account authority, production
freshness/reconciliation and both adapters remain open. No acceptance case is closed.

## Configured diagnostic billing for 0.1.16

Shared `policy::billing::balance::assess_balance` diagnoses configuration, gateway,
recovery and balance thresholds without requiring local spendable funds. It reports
the exact target shortfall independently of reserve feasibility. The released
`assess_readiness` API uses this same threshold logic, then performs reserve arithmetic
only with its explicitly supplied known funds. Released signatures, enum variants,
blocker ordering and behavior are unchanged; this is an additive library API.

The current unpublished operator diagnosis can consume balance-only evidence. It
adds `SpendabilityUnknown` for configured observations even when the minimum is met,
and retains independent provider, funding and recovery blockers. Missing accounting
never becomes zero, a reserve violation, available gross canister cycles or permission
to transfer. Unknown/fenced recovery and uncertain funding cannot be cleared by a
balance report. Diagnosis remains pure and grants no effect authority.

The authority fixture validates numeric limits through `FundingLimits` and retains
service/namespace/source/account scope and the exact observation revision. A required
BillingRecord wraps the optional configuration, preserving the pre-1.0 fixture hard
cut. Invalid/stale/unauthorized updates change no journal. Reconfiguring balance scope
invalidates current use of old limits, including when reinstalling the same scope;
retained limits stay inspectable until explicitly replaced. Restoration validates
numeric and scope/revision consistency before entering the existing permanent fence.

Passive status and actual CLI reports expose retained/current limits and the shared
threshold result with exact decimal amounts. Local spendability remains null. Current
malformed/account-mismatched/oversized replies diagnose malformed balance; missing,
rejected, expired, invalidated or fenced observations diagnose unavailable balance.
No query refreshes, funds or clears uncertainty; limits do not qualify a provider.

Checks passed:

- Existing billing unit tests and public balance-readiness/billing-input integration
  cases retain reserve behavior; new policy tests cover extreme shortfalls and unknown
  spendability together with uncertain funding, unknown recovery and enforced fences.
- Native fixture validation checks exact revision retention and malformed restored limits.
- Actual PocketIC/CLI cases exercise low/sufficient balances, extreme decimal amounts,
  failed/expired observations, stale scopes, denied/invalid limit changes and restoration.
  Check exits remain blocked with unknown spendability; journals/source counts do not
  change on diagnosis. Existing operator, funding and journey targets also pass.
- Strict affected-package Clippy, warning-free rustdoc, three fixture Wasm builds,
  targeted library/client Wasm checks, formatting and source-inventory verification pass.

The [current inventory](verification-checkpoints.sha256) covers these sources. Earlier
artifact hashes above describe their earlier slices; current Wasm SHA-256:

- authority: `231d1cc23f1187ff4d14735be17f67cd77135f35f1b960a7af66776d784d4629`
- gateway source: `5dcef1b08627a204dfeb2ca77ef2f7b1a1e654c33358f387eb7fc18a2eb06f1e`
- funding: `c7d2b3ee9ce58c7561554c6e6aafe5cf56c34ace00133fb05f40b2c68ac0b472`

This adds partial BLOB-08/12/17 and A08/A11 evidence. The 0.1.16 changelog draft is
undated; Cargo/receipt remain 0.1.15. An effect-free exact-version release preview
selects 0.1.16. Full CI/release verification remains the maintainer workflow's next
step after committing clean source; targeted results do not claim that gate passed.
No version preparation, commit, tag, push, publication or live provider operation ran.

## Explicit local operator refresh after 0.1.16

The maintainer released 0.1.16 at `4a3aaa6` from source
`d084112e811866682f8c9374ab46ac9c329624ff`. The earlier checkpoint inventory's
151 source hashes match that source (Cargo 0.1.15); it remains historical.
This batch uses [operator-actions.sha256](operator-actions.sha256), with Cargo
0.1.16. No published library API or production adapter changed.

The separate unpublished `blob-fixture-refresh` binary requires explicit loopback
server, instance, canister, simulated caller, namespace, source, account, revision
and next lifetime attempt. Dry-run calls only `preview_balance_refresh`. Its shared
model admission predicate checks the currently configured binding and revision,
next sequence, pending exclusion and bounded capacity. Workflow authority/fence
checks apply to both preview and refresh. Preview does not reserve an attempt.
Refresh rechecks atomically and persists intent before its controlled-source call.
A completed or failed observation consumes its sequence; repeated requests cannot
start another call. There is no reset, compatibility endpoint or restore reactivation.

Refresh performs one application-level update invocation, then a distinct passive
status query. JSON and exit status preserve acknowledged completion even when that
query fails. Transport/decoding failure remains an uncertain action outcome and does
not infer completion from the later status. Typed failures may have consumed an
attempt; their history stays visible. Neither status errors nor action errors cause
application retries or automatic sequence allocation. The SDK handles instance-busy
responses and polls accepted operations; this is not a claim of one HTTP request.
As with the status client, the SDK polling budget is not a stalled-socket deadline,
and reply byte limits apply after reception. This is a trusted local harness tool,
not a hardened network client. Refresh advances simulator rounds and carries no
provider payment. Caller identity remains simulated.

Checks passed:

- Native model admission tests cover exact scope/revision/next attempt, pending
  exclusion and consumed identities after failed transport. Existing balance model
  and client parsing/bounded-status-decoding tests pass.
- Actual operator subprocess/PocketIC tests cover passive previews, successful
  refresh, repeated request rejection, changed revision after preview, wrong account
  or namespace, denied callers, failed observations, permanent restore fences and
  held source calls. Journals and source-call counts prove no extra dispatch.
- A tiny actual IC Wasm witness grows stable memory for each update and acknowledges
  completion while providing no status method. CLI output retains completion with
  a separate failed post-status, and exactly one growth proves no replay. A malformed
  acknowledgement stays uncertain with one growth. An update-only preview method
  is rejected by dry-run without growth. These witnesses prove transport control
  flow, not balance/provider semantics.
- Strict Clippy for authority/protocol/client targets, warning-free protocol/client
  rustdoc, three fixture Wasm builds, client Wasm check and formatting pass. The full
  existing operator target passed; additional held-call and preview-mode cases passed
  in the focused refresh target. Full CI/release verification was not requested.

Fixture Wasm SHA-256 for this source:

- authority: `b276f6aca2a67f3398a78f895025b44a67b53e8090e4d2dec61fc6f0054e268c`
- gateway source: `ba24dfab3aaebb3a87f1d6699fe13349e9fad824e3964788ca56c6baa65de8d2`
- funding: `64ce70c0f3930a69ee6f05cfa8359423b8062173f1f2242e7338e9551ad20f64`

This adds partial BLOB-15/16/17 and A11 evidence. Production Caffeine transport,
namespace/account authority, gateway sync/funding commands, independent recovery,
managed discovery and both service adapters remain unqualified. No production
calls, paid operations, commits or release actions were performed in this batch.

## Explicit local gateway sync after 0.1.16

The current [operator inventory](operator-actions.sha256) now also covers
`blob-fixture-sync`. Balance refresh and sync share parsing primitives, fixed
query/update transport and the action/post-status result flow; each operation
retains its own typed request and reply. No published library API changed.

The fixture's required v1 SyncControlRecord retains an operator-edit revision.
Every revocation advances it, including removal of an absent member. Exhaustion
permanently blocks new syncs while allowing revocation. Requests bind exact service,
namespace, controlled source, revision and next sequence before allocating a token
and saving intent. Preview uses the same admission checks without mutation. The
existing GatewayRegistry still owns membership, exact callback correlation and
stale-token rejection. The archive/status retain source, edit revision and sync
counters; restoration is permanently fenced. No removed no-argument endpoint or
old archive reader remains. Cross-release fixtures require reinstall.

The controlled-source protocol now carries the explicit request for deliberate
reentrant schedules. Replacement explicitly advances the revision/sequence after
its own revocation; overlap reuses the captured request and is rejected. These are
local fixture controls, not an invented Caffeine request schema. The source canister
is the explicitly installed operator in these scenarios; CLI callers are simulated.

Targeted validation passed:

- Native fixture records and client checks; strict authority/source/protocol/client
  Clippy; warning-free client/protocol rustdoc; fixture Wasm builds and client Wasm check.
- Full gateway-sync, journey/recovery and operator PocketIC targets. Existing balance
  commands retain behavior after sharing the action runner. Actual subprocesses prove
  passive previews, stale/repeated request rejection, denied/misbound callers,
  source failures, no-op revocation and re-addition only through a later explicit sync.
- Held source calls retain revocation. Existing reentrant replacement and old-archive
  tests retain newer membership or fenced evidence. The additional live-callback
  upgrade case passes: after forced restoration, the old callback traps at the IC
  boundary and cannot update membership or clear the frozen pending intent.
- Real Wasm update witnesses preserve completed/uncertain action output despite failed
  post-status with exactly one stable-memory growth. An update-only preview method
  cannot mutate. No application retry or sequence advance follows diagnosis failure.

Current Wasm SHA-256 (earlier balance-only hashes above identify its earlier slice):

- authority: `800e68da5817458291f9bb989d42ce39731449df7a3542350f4f52bb2d13bc7b`
- gateway source: `4c7ec9acaa5c8b235ed5fef44aafe413f1cdbaf7d0159b1c90fbba59eb61f395`
- funding: `3fc392e0b73d3fbe28113bf51d2a2c82090d1c7c27a8f6437b8b2c7f94497287`

This adds partial BLOB-10/15/16/17 and A05/A11 evidence. Production identity,
Cashier transport, sync freshness, independent recovery, funding operator admission
and both adapters remain open. No paid provider call or release action occurred;
full CI/release validation was not requested.

## Passive funding admission preview after 0.1.16

Review found that the raw funding fixture intentionally exercises transfers without
production spendable accounting, reserve configuration or qualified provider bindings.
Its completed callback records transport acceptance, not provider credit. It cannot
be exposed as a reserve-protected operator funding action on that evidence.

The additive library `policy::billing::admission::evidence` module diagnoses optional
recovery, spendability and complete activity observations. All independent blockers
remain visible; known reserve arithmetic runs only with validated limits and known
funds. The exact full request is preserved. Native tests compare complete evidence
with the maintained const admission API and cover missing inputs alongside uncertain
payments and fences. No existing public signature, enum or behavior changed.

The local `preview_funding` query checks driver authority, actual sender/peer, exact
proposed ID and positive amount before projecting used identity and journal-capacity
facts. It observes the full uncredited transfer journal and uses shared reconciliation
and admission policy. Missing recovery remains unknown until an actual restored fence
is enforced; spendability and billing limits remain absent. Provider qualification
is explicitly blocked. The view includes independent reasons and never consumes an
ID, reserves cycles, changes history or calls the receiver.

`blob-fixture-funding-preview` supports only `dry-run` with an explicit local target,
simulated caller, peer, ID and amount. It shares the fixed query transport used by
status, validates every echoed request field, bounds Candid decoding and emits exact
decimal values. Exit 4 reports a valid blocked preview; read/authority/binding failures
use 3 and invalid arguments 2. No method override, accounting override, update fallback
or transfer/retry path exists. The raw fixture `fund` method remains a test control,
not an operator workflow. None of these local observations qualify Caffeine credit.

Targeted checks passed:

- Native existing/new funding admission, funding journal/conversion and CLI decode
  checks; strict library/funding/protocol/client Clippy; warning-free rustdoc; three
  fixture Wasm builds and library/client Wasm checks.
- Complete funding and operator PocketIC targets. Actual subprocess previews remain
  passive before any transfer, after partial acceptance and later full refund, after
  a real callback trap, at journal capacity and after fenced restore. Reusing an ID
  with a different amount remains blocked. Adding gross canister cycles does not
  establish spendability or create a synthetic reserve result.
- Native reply checks reject wrong service/peer/id/amount and oversized/malformed
  responses, retaining full u128 reserve values as decimal strings. Actual PocketIC
  query mode rejects an update-only preview witness without growing stable memory.

The [current operator inventory](operator-actions.sha256) covers this slice. Current
Wasm SHA-256 (earlier hashes above remain evidence of their earlier slices):

- authority: `38b25d9253869b7353596a8e9fd79ada21ea78acbf1e36907af3fab3b0032070`
- gateway source: `9b5cca8c699071a595a2971ba3c08e2031e863f6532447a7b94d647109ca993a`
- funding: `4f810eb9071a4505cecd9265403c4c4b811b768b2170f2c77e4b5ff5524245fa`

Partial BLOB-11/15/17 and A08/A11 evidence only. Atomic spendable reservations,
provider/account qualification, exact credit reconciliation, independent recovery
and both production adapters remain open. No live provider calls, release/version
changes or full CI/release gate ran in this continuation.

## Local attachment budget after 0.1.16

The funding fixture now requires an explicit installed allocation and positive
reserve. Its required v1 budget record is an unpublished hard cut, with reinstall
across releases. Accounting derives from the bounded original transfer journal;
there are no independently mutable balance counters or replenishment operations.
Admission checks the full offer against remaining allocation before appending and
persisting its exact intent. Each intent reserves its full offer until a terminal
observation. Accepted cycles stay charged, exact callback refunds release returned
attachments, and proven enqueue failures have their own unsent category. Missing
callbacks retain the full reservation, including after callback rollback or restore.
Validation also checks that each historical offer fitted before its own refund.

Status and preview expose a separately labelled local attachment budget. Revisions
count admissions and terminal observations, so even a full refund invalidates an
old preview. The query binds the supplied revision and reports local reserve
violations alongside missing production evidence. Incoming receipts and actual
gross cycle top-ups cannot change the installed allocation. Execution fees and
other operating liabilities are outside this envelope: top-level spendability
remains unknown. No provider credit, independent recovery or operator funding
action is inferred. Restored owners remain permanently fenced.

Targeted validation passed:

- Native model checks cover exact reservations/returns, historical over-budget
  offers, invalid reserve, full-width allocation arithmetic and retained unknowns.
  Client checks preserve decimal budget values and bind the echoed revision.
- Complete funding and operator PocketIC targets pass. Actual IC transfers prove
  atomic reserve rejection with unchanged stable journals, partial acceptance,
  full refunds, distinct enqueue failures and no replenishment from incoming or
  added gross cycles. Callback traps and real upgrades retain full reservations;
  old archives and live callbacks retain their existing recovery fences.
- Actual preview subprocesses reject a stale revision after a full refund despite
  unchanged available allocation. Query-only behavior and unknown production
  spendability remain intact. Existing enqueue-failure coverage was extended rather
  than adding a duplicate transfer schedule.
- Strict funding/protocol/client Clippy, warning-free rustdoc, all three fixture
  Wasm builds, client Wasm check, formatting and diff checks pass.

The [current operator inventory](operator-actions.sha256) includes these sources.
Wasm SHA-256 for this slice (earlier hashes remain historical observations):

- authority: `edfe318db30714df456b52f18dd3a757e934f616213e47b596553332ebded24d`
- gateway source: `8df2e383c83cc31e0f75f228c27409c419d513915386650f524d08561975f451`
- funding: `f6c9692ee918dd3c14248d2787f2ca7058374d2fee95856914cba4f855ad4195`

This advances partial BLOB-11/15/17 and A08/A11 evidence. Production spendability
including execution fees and other liabilities, complete update admission, deployed
provider/account credit reconciliation, independent recovery and both adapters
remain open. Cargo/release receipt remain 0.1.16. No live paid effect, release action
or full CI/release gate ran.

## Funding liquidity guard after 0.1.16

The [IC system API](https://docs.internetcomputer.org/references/ic-interface-spec/canister-interface/#cycle-cost-calculation)
provides a call-cost bound including request transmission and maximum response and
callback reservations. Its liquid-balance API accounts for platform restrictions;
memory growth can change the amount available. The pinned ic-cdk 0.20.3
`Call::get_cost` includes attached cycles, so this fixture samples it before adding
the attachment. No pricing constants or gross-balance estimates were introduced.

The additive pure `policy::billing::liquidity` API subtracts known call cost,
positive operating slack and additional liabilities from supplied liquid funds.
Sequential subtraction avoids overflow even when the combined holds exceed u128.
The full positive request either fits or is refused; no smaller payment is proposed.
Unknown costs/liabilities must not be converted to zero to construct this complete
input. Already deducted transfers/platform reservations must not be deducted twice.

The required local budget record now retains explicit operating slack and other
liabilities, with no default/reset/replenishment path. It remains v1 and requires
reinstall across releases. The workflow persists intent first, prepares the exact
controlled `receive` call, samples platform cost/liquidity, and applies shared
policy before awaiting it. Dropping a refused call sends nothing. Its completed
LiquidityBlocked observation retains the identity with zero acceptance and no
callback refund, releasing only the unsent attachment allocation. The earlier
insufficient-cycles test now asserts this earlier refusal; historical CDK enqueue
failure evidence remains historical. Actual CDK failures still use NotEnqueued.

Passive previews show separately labelled liquidity/cost figures and full-width
operating holds. Their cost payload bounds all valid acceptance/reply controls;
update dispatch uses its own actual encoding. A same-query PocketIC observation
remained unchanged after an administrative cycle top-up, including after a tick;
a distinct proposed operation observed the changed funds. Tests do not assume
query freshness. The update rechecks current resources, and neither queries nor
added cycles change installed allocation/revision. No production credit, complete
liability accounting or independent recovery is inferred from these local holds.

Targeted checks passed:

- Native policy boundary/extreme-value arithmetic; native fixture record validation,
  bounded preview payloads and invalid no-transfer observations; client decoding
  retains exact decimal liquidity/cost/hold amounts and bound requests.
- Complete funding and operator PocketIC targets: fee-only refusal even when the
  attachment fits liquid balance, explicit liabilities blocking dispatch, unsent
  identities/refunds, changed liquidity without a budget revision, successful
  controlled transfers, rollback, live callback upgrades and permanent restore fences.
- Strict library/funding/protocol/client Clippy, warning-free rustdoc, three fixture
  Wasm builds, library/client Wasm checks and formatting/diff checks.

The [current operator inventory](operator-actions.sha256) includes these sources.
Current fixture Wasm SHA-256:

- authority: `28f002c59ebdba43c8bdf055f1194a8df928ff1c676946bc6afa5ce01a0b1ace`
- gateway source: `6aa184e0cc40d9aaa26695755da64f5473c6c93f75e0717a469e707ce9a56f98`
- funding: `e71f052e6169c090b9c48df66f7c635692279a0e3e79751d4f5c4486ac19794d`

This advances partial BLOB-11/15/17 and A08/A11. Production operating bounds,
complete funding admission, Caffeine account/credit reconciliation, independent
recovery and both adapters remain open. No paid provider call, release action or
full CI/release gate ran; Cargo and the release receipt remain 0.1.16.

## Funding refusal recovery after 0.1.16

A new PocketIC regression first reproduced a liquidity refusal executing the
fixture's callback trap control without an actual callback. That synchronous trap
rolled back the whole ingress message, including the retained refusal and identity.
The workflow now applies this control only when a callback refund observation
exists. Model validation likewise permits terminal unsent records with the control
set, while rejecting completed records that should have trapped in a real callback.
No callback is fabricated for liquidity refusal or CDK enqueue failure.

Actual PocketIC tests fill the bounded journal with unsent attempts under different
receiver/callback controls. Refusals retain their identities without receipts or
refunds, exhaust lifetime capacity, and preserve records/allocation through fenced
restore. Separate real zero-refund and full-refund callback traps both retain the
full attachment as uncertain and block further payments. Native checks also cover
restoration of both kinds of unsent terminal record.

Preview now exposes AmountLimitExceeded using the model's same maximum attachment
as update admission. Actual CLI/query tests cover the exact limit, one above it and
u128::MAX; oversized updates return typed Limit without changing journals. Client
JSON preserves the maximum as a decimal string alongside other independent blockers.

Targeted native, strict funding/protocol/client Clippy, warning-free rustdoc, three
fixture Wasm builds, client Wasm check and complete funding/operator PocketIC targets
pass. Formatting, diff and the current operator source inventory checks pass.
Current Wasm SHA-256 (earlier slice hashes above remain historical observations):

- authority: `fc52e1dd94bd41e6af790602b29f87b598295d5b2a5e7ef35418ce83bdd0f7db`
- gateway source: `6aa184e0cc40d9aaa26695755da64f5473c6c93f75e0717a469e707ce9a56f98`
- funding: `2e7ab2862340298dc6d89109ff619f3a10357c250d9de44221d4b589214c911f`

This strengthens the existing partial funding/recovery evidence; no production
provider credit or recovery qualification changed. Cargo/release receipt remain
0.1.16. No paid provider call, release action or full CI/release gate ran.

## Ledger notification response decoding after 0.1.17

The [funding review](caffeine-funding-review.json) refreshed public package/source
metadata and anonymously read deployed Cashier Candid. The retained interface
hash is unchanged. The new library decoder uses that advertised
`NotifyCyclesLedgerDepositResult` schema, with private wire types and explicit
byte, decode-work, skip-work and type-table bounds. No notification is sent.

Independent didc 0.5.4 fixtures preserve `credited` separately from balance,
including zero and u128::MAX, and a block index larger than u128::MAX. Invalid
cycle amounts or any negative balance component reject the complete report.
All four provider errors remain distinct; diagnostic messages are discarded.
Malformed, truncated, unknown-variant and over-budget replies reject. A synthetic
additional field decodes only with sufficient skip budget. A valid direct-top-up
success cannot impersonate ledger notification success. Exact source values and
synthetic extensions are recorded in the fixture cases.json.

Checks passed: all targeted Caffeine native tests, strict library/test Clippy,
library wasm32-unknown-unknown check, warning-free library rustdoc and formatting.
The [funding reports inventory](funding-reports.sha256) binds decoder, shared
conversion, independent fixtures and reviewed interface. Cargo/release receipt
remain 0.1.17; the 0.1.18 changelog is an undated draft. Historical inventories
were not refreshed. No full CI/release gate or new PocketIC run was needed for
this pure decoder; there is no platform or transport change.

This is partial BLOB-11 wire evidence. The update reply lacks an echoed account,
ledger principal and caller operation identity. Neither the block index nor
NothingToDeposit establishes correlation to an earlier deposit or direct top-up.
Production reconciliation, retention, safe retry and independent recovery remain
open; no decoder result can settle the existing fixture journal.

## Funding error propagation for 0.1.18

Review found that the local funding experiment mapped only `InternalError` from
its shared decoder; other advertised failures became `InvalidReply`. The fixture
now retains all four categories and the reported `NotAuthorized` principal in a
passive outcome view. The private library decoder remains the only wire-schema
owner. No diagnostic text becomes authority, and no provider error supplies a
refund. The unpublished request/outcome schema is hard-cut within v1; no legacy
reader or migration was added. Cross-release fixtures require reinstall.

Actual PocketIC calls reuse independently encoded Cashier fixtures, controlling
acceptance separately from response bytes. These combinations test local
accounting, not whether the deployed Cashier accepts cycles on each error:

- Partial acceptance with each provider error, full acceptance with overflow and
  full refund with missing-cycles error retain exact callback facts. Later full
  refunds cannot hide earlier credit obligations. Status preserves each category.
- A ledger credit report and an unknown error on the direct-top-up path yield
  InvalidReply while preserving independently observed receiver acceptance.
- Success, missing-cycles error and wrong-route ledger replies followed by callback
  traps retain the whole unresolved offer. Failed upgrades preserve the active
  owner; successful same-release restores remain fenced without repeating payment.
- Actual CLI subprocesses retain typed provider errors, principal text, decimal
  amounts and absent provider credit. Repeated diagnosis leaves journals unchanged;
  successful restore retains exact attempt outcomes and attachment accounting.

Funding native checks, the complete funding/operator PocketIC targets, strict
funding/protocol/client Clippy, warning-free rustdoc, fixture Wasm builds and
formatting/diff checks pass. The preview payload bound covers every new reply
control. A final test-data extraction was rechecked with the affected refund case.
The current [funding reports inventory](funding-reports.sha256) includes this work;
released inventories remain historical. Tested Wasm SHA-256:

- authority: `3f38fb4094226079a38edcd0da383e478cec5411afddbf807d62a342c012ad21`
- gateway source: `3cccb99777af423ab0cd0aabc4c0f7454f38ba40f31fabdd702a3af5d7fd81ff`
- funding: `aa1a780694f8643c0600b14e1a94749783f5a9bc245835cdd373e3fd1008e35e`

This strengthens partial BLOB-11/15/17 evidence. Public correlation research did
not establish a Cashier completion lookup or numeric retention contract. Cycles
Ledger transaction evidence alone cannot prove the separate Cashier account credit.
No paid provider effect, deployment, version mutation or full CI/release gate ran.

## Shared attachment accounting for 0.1.18

The additive library `FundingAllocation` model now reconstructs amount accounting
from complete, bounded, sequential `FundingTransfer` history. Configuration has an
explicit allocation, positive reserve and lifetime slot limit. Every original
full offer must fit before its own return is applied. Unknown transport can only
be last; this algorithm does not claim to reconstruct overlapping calls.

The view conserves the original allocation across available, accepted and unknown
amounts. Historical callback refunds and proven unsent offers are independent
totals, already returned to the available allocation. Neither is added twice.
Native tests cover original over-budget offers followed by full refunds, consumed
no-transfer slots, repeated reconstruction, reserve-only allocations, u128 limits,
exact lifetime totals at the maximum, and overflow without partial output.

The funding fixture's duplicate arithmetic loop was removed. It validates its
own exact requests/transport observations, then delegates amount projection to the
library. Service/peer/release bindings, identities, revisions, persistence,
operating holds and restore fences remain fixture-owned. Existing IC budget and
operator tests pass with this shared implementation, covering incoming-cycle
isolation, full refunds, refusals, callbacks, provider errors, capacity exhaustion,
old journals and live-callback restores. Library/fixture native tests, strict
affected-package Clippy, warning-free library/fixture rustdoc and fixture Wasm
builds also pass. Formatting and the current funding reports inventory pass.

Tested Wasm SHA-256 (earlier slice observations above remain historical):

- authority: `789e78b07cea694ee428f05e377e6e7878197cc1e5f9b0e86a1c1039536567c8`
- gateway source: `3cccb99777af423ab0cd0aabc4c0f7454f38ba40f31fabdd702a3af5d7fd81ff`
- funding: `7d08b74a97043e5ab62437e93f2e27e2c305b47d67ad66276140f755b23bc127`

This advances reusable local BLOB-11 accounting. It cannot establish omitted
history, authentic provider credit, total operating liabilities, safe identity
allocation or a fresh recovery authority. No provider call or full CI/release
gate ran; Cargo and the release receipt remain 0.1.17.

## Exact local funding lookup for 0.1.18

The local funding fixture now queries an exact original request through the active
journal owner. Driver authority and service/peer bindings are checked before
validating every immutable request input. A reused ID with different amount,
acceptance control, reply mode or callback control returns Conflict, exposing no
retained result. Absent, Pending and Observed describe local retained evidence;
absence cannot establish no external effect, and pending includes callback rollback.

The unpublished `blob-fixture-funding-lookup` CLI requires explicit loopback target,
caller, peer and every original request field. It uses only `lookup_funding` query,
with bounded Candid decoding, full-width decimal amounts and exact response binding.
Typed failures are not converted to absence. It shares reconciliation JSON rendering
with status and neither sends a transfer nor clears a fence. No provider schema or
production transport was added; the fixture's stored schema was not changed by this
lookup addition.

Actual PocketIC and subprocess cases passed:

- Discard a completed ingress result, then recover the exact retained provider-error
  observation and refund without a second transfer. Conflicting inputs reject;
  repeated reads leave sender/receiver journals unchanged. Fenced restore retains it.
- Observe a live committed intent as Pending, then query its original completion.
  A real callback trap instead stays Pending through queries and fenced restoration.
- Replace stable bytes with an old empty backup: queries still read the active owner
  until actual upgrade. Restored lookup reports Absent and fenced, while the receiver
  retains its acceptance receipt and the sender rejects another payment.
- Denied/misbound/invalid requests expose no outcome. An actual update-only lookup
  witness is rejected without mutation; a positive update control proves the method
  would mutate if invoked. There is no query-to-update fallback.

Native model/CLI checks cover every immutable input, missing/duplicate arguments,
unknown/mutating modes, malformed/oversized replies and exact u128/u64 JSON. Complete
funding/operator PocketIC targets, strict affected-package Clippy, warning-free
rustdoc, fixture Wasm builds and client Wasm check pass. Source hashes are retained
in funding-reports.sha256. Tested Wasm SHA-256:

- authority: `8efc571ddba3534904b8ad899b26895dcb10c47dd6ac69b2cba99e36c0ccad27`
- gateway source: `7ebd7319b5d7f92a666012668190941d185b759112bd1145673a040ce2428b04`
- funding: `7e48ad603b0c7b075d3fe30382d098fc9a01427005d023feab25d35bbe605874`

This is local ingress-result recovery only. Cashier operation/account correlation,
credit reconciliation and independent recovery remain open. No paid provider call,
release action or full CI/release validation ran. Cargo/receipt remain 0.1.17.

## Payment relationship inspection after 0.1.18

The [installation review](caffeine-installation-review.json) pins the refreshed
Cashier schema and Canic/Toko sources. The additive library decoder for
`payment_account_canister_get_v1` accepts supplied bytes and explicitly expected
paid-canister/payer principals. No transport or production installation is added.

Independent didc 0.5.4 vectors live in
`crates/ic-blob-storage/tests/fixtures/caffeine-relationship/`; `cases.json`
retains exact Candid inputs and the schema hash. Reproduce each `candid` value
with `didc encode --defs docs/evidence/caffeine-cashier.did --types
'(PaymentAccountCanisterGetResult)'` and compare the resulting hex to its `file`.
These are synthetic responses, not queried accounts. All fixtures reproduce
byte-for-byte against the retained schema.

Targeted native tests establish:

- Present linked and explicit self-payer relationships bind both trusted principals;
  mismatched owner/payer and anonymous/management expected principals reject.
- Daily limits and period spend remain exact signed arbitrary-width observations,
  including negative values and spend above the reported limit. Wide natural
  counters, zero, maximum timestamps and optional expiry are not narrowed.
- All four advertised provider failures and no-relationship reports remain distinct.
  Diagnostic text is discarded. Unknown errors, truncated/malformed input and
  byte/work/type/skip exhaustion return typed decoding failures.
- Candid missing or incompatible optional fields can yield no relationship. That
  result deliberately establishes neither self-payment nor absence of obligations.

Checks passed: targeted library Caffeine tests, strict library all-target Clippy,
warning-free library rustdoc, library Wasm check, formatting and diff whitespace.
No platform behavior changed, so no new PocketIC claim is made. Existing 0.1.18
funding-report hashes were verified against historical source `fff08a9`, not
rotated to this worktree. Full CI/release validation, live account reads and
provider effects were not run. Cargo and the release receipt remain 0.1.18.

## Cashier query encoding and local composition after 0.1.18

Anonymous Cashier Candid retrieval again matched SHA-256
`232b08e4514048d4de48d6d1bf4387f577bfb64c7e2e2ded699a5e52d475d76f`.
`CashierQueryRequest` owns target, method, encoded arguments and the original
balance/relationship/gateway query selection. Anonymous/management principals
reject in every role. The relationship payer is a retained response expectation,
not sent as the requested usage owner. No constructor performs transport or
selects an arbitrary/paid method. Workflows still own full operation correlation.

Independent didc 0.5.4 request vectors live in
`crates/ic-blob-storage/tests/fixtures/caffeine-query/cases.json`. For each case,
encode its `candid` input using `--defs docs/evidence/caffeine-cashier.did
--method <method>` and compare with its `file`. All method/argument pairs match
byte-for-byte. Native checks additionally cover independently changed targets,
accounts, owner and payer, including explicit self-payment expectations.

The authority fixture now uses these balance arguments with its explicitly local
`fixture_balance` endpoint. The controlled receiver independently decodes the
account record and retains its service/driver checks, bounded history and hold/
reject controls. No alternate old request shape remains. This is a hard cut of
the unpublished v1 fixture; cross-release reinstall remains required.

The complete operator-balance PocketIC group passes: actual replies preserve
account checks, malformed/error distinctions, dispatch-based age, pending slots,
configuration invalidation, callback interruption, frozen restoration, request
identity consumption and passive status/preview behavior. The source still is a
substitute, not an authenticated Cashier. Paid effects and provider qualification
remain outside these results.

Targeted Caffeine native tests, affected strict all-target Clippy, warning-free
affected rustdoc, fixture Wasm builds, formatting and diff checks pass. Tested
fixture Wasm SHA-256 values:

- authority: `0f597ff2fc83c86dabbcb3cab8eaf19f217bda589c93466045a670ce20e434f1`
- gateway source: `8e9fc5f854ce6e4835a8fbb5d2cc7d45b4303b780434215da35f71786a12cffe`
- funding (test setup): `935d26f3c9c9537b5ec4d184948824d01d0b3f0042a7ffb9fe28af5c63d68518`

No full CI, live account lookup, production transport or paid effect ran.
Cargo/receipt remain 0.1.18; historical source inventories remain unchanged.

## Request-bound replies and relationship queries after 0.1.18

`CashierQueryRequest` now supplies its original bindings to balance/relationship
reply decoding. Wrong decoder method or trusted response-source context rejects
before byte/decoder limits. Native tests use the independent response fixtures
to prove account, owner and payer mismatch handling, distinct absence/errors and
unchanged request selection. The API does not authenticate caller-supplied source
context or establish a response's freshness.

The balance fixture retains the encoded request alongside its response across
the actual await. Its journal still owns service/namespace/revision/attempt checks
and restoration fencing; no current configuration is substituted into decoding.

Gateway application now checks the original query method and target before the
registry's service/namespace/Cashier scope, exact pending token and bounded
decoding. Native checks cover wrong bindings, malformed/oversized/over-limit lists,
success using the independent gateway fixture, and consumed/cancelled/revoked
tokens. Every rejection preserves the entire registry, including pending state.
The reentrant sync fixture retains the selected request across its await, while
still calling its fixture-only scheduling endpoint with revision/sequence data.
It does not send the provider's empty arguments to that scheduling endpoint.

The controlled source exposes the maintained relationship query's name and request
shape over its existing bounded raw-response slot. Its explicit driver restriction
is a substitute policy, not Cashier authorization evidence. Queries reject wrong
owners/callers and held/rejected/busy source state, and never alter journals.
Actual PocketIC checks cover:

- Exact signed amounts beyond u128 and distinct absent/all advertised error reports.
- Wrong returned owner/payer, malformed and oversized responses with no fallback.
- Caller/owner/hold/reject denial and stable journals before/after queries.
- Injected old stable bytes cannot replace active heap observations. Actual
  restoration fences the source and refuses the same formerly valid query.

The separate gateway query accepts the maintained empty argument list. PocketIC
feeds its reply into an exact pending library sync, checks consumed/revoked token
rejection and unchanged fixture journals, and denies wrong callers, all scripted
source modes and restored sources. The existing actual inter-canister sync tests
still cover overlapping calls, revocation, newer membership, held replies and
forced restoration with an outstanding callback.

The PocketIC package adds only a local library dev-dependency; Cargo.lock records
that edge, with no dependency version change. Native request checks, affected
strict all-target Clippy, warning-free rustdoc and fixture Wasm builds pass. The
complete operator and gateway-sync PocketIC targets pass for the final code.
Formatting/diff checks pass.

Tested Wasm SHA-256 values:

- authority: `49c53f73d9a600fe2b37aa2ff46e1539561b99170f0bb4d042df17ba739ef8b5`
- gateway source: `b8f4408966733d3250508148867f049ed03e077729ff7b79e63110388b0c7fdb`
- funding: `399d89b1e342f4e3329e4661a858eecf6bc1fc80bce7745d0106380fb5a89386`

This adds neither a production transport nor provider qualification. No live
account lookup, payment, deployment or full CI/release validation ran. Release
version remains 0.1.18; historical source inventories are unchanged.

## Account-scoped audit requests after 0.1.18

Anonymous Cashier Candid metadata was fetched again and still matches SHA-256
`232b08e4514048d4de48d6d1bf4387f577bfb64c7e2e2ded699a5e52d475d76f`.
The shared query encoder now accepts an explicit audit account, positive page
bound, optional event filter and optional opaque cursor. It always sends the
account and maximum, with no all-account/default-bound mode. A cursor naming a
different account rejects locally; absent cursor accounts and every nat64
sequence remain exact. This is a conservative local request restriction, not
qualification of server cursor semantics.

Request-bound reply decoding rejects wrong method/source before parsing and
uses the smaller of requested and decoder reported-count bounds. It preserves
opaque CSV, absent/terminal pages, provider failures and optional cursor fields.
There is no row-count verification, account/filter authentication, completeness
claim, automatic follow-up or payment reconciliation.

Independent didc 0.5.4 vectors in the existing `caffeine-query/cases.json` cover
the first page, all seven event variants with an account-bearing cursor, and
maximum nat64 values with an absent cursor account. Native tests also cover
invalid target/account, mismatched cursors, request-field independence, both
directions of the count bound and unusable replies. The targeted Caffeine tests,
strict affected all-target Clippy, affected Wasm checks, warning-free rustdoc,
formatting and diff checks pass. No new platform workflow was added or PocketIC
run repeated for this continuation. Earlier tested Wasm hashes remain evidence
of those earlier runs. No live account read, paid effect or release action ran.

## Service configuration candidates for 0.2

`model::service::configuration` composes the existing catalog, upload and billing
limits into one validated candidate. Identity fields are explicit and reject
anonymous/management principals. The namespace remains a local nonzero binding,
not a provisioned gateway project/bucket. Payer validation does not prove a linked
relationship, and an operator identity does not enroll a tenant.

Native tests cover independent logical/physical/liability budgets, tenant/global
object and upload relations, special principals and Wasm32 count boundaries.
Reference receipt validation uses wide arithmetic for `2 * references - 1`;
an actual shared reference journal demonstrates retain and both releases at the
minimum accepted bound. The 10 MiB object input reflects the reviewed Toko media
constant but is a test envelope, not a production default or large-file journey.

Targeted service-model tests, strict library all-target Clippy, Wasm compilation
check, warning-free rustdoc, formatting and diff checks pass. Configuration
construction allocates no service state and neither freezes a persisted schema
nor enforces live request admission. No new platform workflow or provider call
was introduced; PocketIC and full CI/release validation were not rerun. The
[0.2 plan](../roadmap.md) records the remaining implementation and evidence gates.

## Project-authorized upload admission for 0.2

The current Toko development and official Caffeine main commits were rechecked
unchanged. Refetched Mixin.mo and Storage.mo hashes match the retained provider
baseline. Mixin's root-only certificate method returns `method` and `blob_hash`,
without a caller check in that reference method; this is not deployed server
authorization or certificate-replay evidence. Source pins are recorded in the
[consumer/certificate review](toko-0.2-review.json).

`UploadAdmissions` owns one exact project permission per shared catalog operation.
It uses the existing reservation/reference/deletion/accounting transitions rather
than a parallel lifecycle. Immutable arguments include uploader and deadline.
The actual service/project/uploader contexts are supplied by the host; this model
does not authenticate an IC call. Enrollment enforcement is extended below. Possible exposure must be
committed before certificate bytes escape. Content verification and recovery
eligibility remain independent host prerequisites.

Native tests cover role separation, changed request/uploader/deadline rejection,
wrong service/namespace, expiry and backwards time, exact replay, root lookup,
retained cancelled-root claims, object/lifetime bounds and passive exact lookup.
Revocation before exposure cancels reservation but retains identity; afterward it
retains uncertain bytes and permits independently established late completion.
Consumer-registration failure does not change the owned reference. Replayed
completion after release cannot resurrect it; physical and billing release remain
separate. Provider completion/deletion/billing facts are supplied test inputs.

Targeted service native tests, strict library all-target Clippy, Wasm check,
warning-free rustdoc, formatting and diff checks pass. No new canister endpoint,
stable schema or transport was introduced; PocketIC and full CI/release validation
were not run. Certificate lifetime, replay charging, namespace enforcement and
recovery authority remain open; local revocation cannot recall an escaped certificate.

## Tenant enrollment and consumer coordination for 0.2

The same transient upload owner now contains a bounded lifetime enrollment map.
Only the configured operator can update it using the exact observed state. There
is no eviction, removal or reset. Suspension preserves reservations, references and
liabilities; reactivation advances an activation generation and cannot revive old
uploader permissions. Generation exhaustion still permits suspension and rejects
reactivation. These are local authority generations, not surviving restore evidence.

Reference application now checks supplied project/service/namespace context before
the shared journal. Fresh retains require active enrollment; exact receipts and
release remain available while suspended. Native tests cover unauthorized enrollment,
invalid principals, capacity including suspended tenants, stale updates, reactivation,
overflow, old-permission rejection, retained uncertainty and late completion. They
also cover tenant/service/namespace isolation, changed receipt payloads, suspended
retain replay and release at reserved receipt capacity, with separate physical and
billing confirmation. Host-supplied provider facts remain labeled test inputs.

The existing roadmap defines the consumer-side transaction/outbox requirements for
stable registration identities, asset tombstones, exact release retries and bounded
retained history. It is an integration contract, not implemented Toko behavior or
evidence of atomicity across canisters. No new document or consumer implementation
was introduced for this contract; sibling sources remain unchanged.

Targeted service tests, strict library all-target Clippy, Wasm check, warning-free
rustdoc, formatting and diff checks pass. No new endpoint, stable schema, provider
call, version mutation or publication occurred. PocketIC and full CI were not rerun
for these native models. Provider/recovery gates remain open.

## Consumer-sized resource envelope for 0.2

Service configuration now derives the maximum manifest leaf count from its object
size, with separate raw metadata-entry and framed-byte bounds. Unsupported SHA-256
lengths and nonportable leaf/header counts reject before allocation. Empty uploads
reject before consuming a service operation or reservation. Native service tests
cover exact/partial chunk boundaries and invalid envelopes without allocating the
advertised maximum collections. Hosts still must apply the derived manifest limits;
configuration alone cannot verify a request's metadata or bytes.

The pinned published client artifact was fetched and verified against SHA-256
`6eb7f5b424f02476c9096e4684d8e520121c21b2b2f6f29f3c9a25e0c07cad57`.
Its unmodified hashing classes, isolated in Node v18.19.1 with WebCrypto and no
gateway/network access, generated the added `pattern-10485760` and
`pattern-10485761` vectors using the existing documented linear-mod251 generator.
The maintained native vector suite passes both additions. Earlier vector data
and historical source inventories remain unchanged.

The connected PocketIC journey now admits ten 1 MiB leaves and a 10 MiB tenant
logical budget, while keeping its separate 12 MiB global physical/liability bounds.
The new boundary case verifies all ten ingress chunks and exact duplicate retries,
denies certificate exposure before complete verification, and retains separate
physical/billing obligations after logical release. One byte over rejects without
allocation. The existing capacity/restore case now retains a ten-leaf cancelled
manifest across ordinary and skipped-hook upgrades into the inspection-only fence.

The first run caught a stale six-leaf journal bound; admission, reconstruction and
journal validation now share the fixture's content envelope. The rerun passes all
affected journey/recovery cases. Targeted service and independent-vector tests,
strict affected all-target Clippy, release Wasm builds, warning-free rustdoc,
formatting and diff checks pass. Completion/deletion/settlement are controlled
provider substitutes. This is not a production admission journey, operational
restore, throughput result or deployed Caffeine evidence. No full CI, paid effect,
release or sibling mutation ran.

## Direct-upload service admission for 0.2

The maintainer selected direct browser-to-Caffeine upload after the measured
cost evaluation. The shared owner now binds a bounded manifest before local
exposure and accepts no file chunks. The generic upload request contains identity,
root, declared size and reference binding, without a whole-file raw digest.
Manifest consistency does not prove length or provider storage. Hosts still need
qualified pre-charge size/tree enforcement, namespace/replay guarantees and
independently established completion before using this model in production.

Service admission additionally requires canonical `Content-Length` metadata equal
to its reservation, unique ASCII-token names ignoring case, and values without
controls, line separators or surrounding whitespace. Raw count/framed-byte bounds
precede scans and temporary allocations. Configuration rejects a metadata budget
too small for its largest object's required length header. Generic pinned hashing
remains unchanged. Native rejection cases preserve unprepared and already-bound
observations/accounting; a consistent false-length declaration still cannot prove
the actual bytes. PocketIC covers missing/conflicting/duplicate/injected metadata,
unchanged reservations, reordered retries and changed-root rejection before exposure.

The independent `media-1048577` and `media-10485760` vectors use canonical length
and content-type metadata. Their roots were generated with the pinned, unmodified
official `BlobHashTree`/`YHash` classes from the existing independent leaf hashes,
without imports, gateway calls or network. The native hashing suite checks the
roots and leaves against actual content. Source provenance remains in the vector
fixture; these vectors do not qualify deployed provider behavior.

One bounded leaf array is retained per prepared operation; global/tenant manifest
capacity is reserved at admission and survives cancellation and settlement. Exact
retries preserve the original declaration. Invalid manifests allocate no partial
state. Enrollment, uploader, service, deadline and phase checks apply on each call.
Uncertain exposure stays charged through revocation; completion and consumer asset
registration remain distinct, preserving references when consumer registration fails.

The unpublished admission probe exercises this owner with actual IC callers/time,
role separation, stop/start and atomic rejection of unsupported upgrades. Its
metadata-only protocol bounds command bytes, decoder work, skipping, type-table
size and header bytes. It returns no certificate and has no completion command.
The separate integrity/readback fixtures retain independent raw-digest inputs and
checkpoints outside the catalog request. They demonstrate integrity/recovery
primitives, not a second production upload mode.

## Local admission resource measurements

The following measurements are the 0.2.0 baseline; the continuation and retained
history workload are recorded separately below.

`make test-admission-resources` builds release Wasm and writes
`.tmp/admission-resources.json`, including its SHA-256, instruction samples,
encoded request sizes and allocated Wasm memory. PocketIC 16 uses actual IC
counters. Samples exclude diagnostic storage and reply encoding; memory reports
allocated pages, not peak live allocations. Rust 1.98.1, CDK 0.20.3 and Candid
0.10.37 were used. The maintained test allows headroom rather than freezing exact
instruction/page counts: under 5M instructions per call, 10M across the sequence,
and at most 1 MiB allocated growth from the initialized owner.

| Step for a 10 MiB declaration | Instructions through workflow | Request bytes |
| --- | ---: | ---: |
| Admission | 934,195 | 287 |
| Manifest preparation | 1,132,730 | 642 |
| Exact manifest retry | 1,132,084 | 642 |
| Local exposure | 908,929 | 195 |
| Total | 4,107,938 | 1,766 |

Allocated Wasm memory stayed at 1,245,184 bytes. Wasm SHA-256:
`1a7027b3ab7c240505287a2dbdc1a4d447b5f352b7f595e57029ede1203db488`.
The prior full-byte experiment spent 1,713,702,451 instructions just on fresh
chunks, plus 903,016,377 for one retry per chunk; its measurement provenance is
retained in the [upload-path review](toko-0.2-review.json). Those service append
commands and tests have been removed. No live Caffeine request, production
latency/cycle price or durable-service capacity is established by either run.

## Retained admission history after 0.2.0

The same transient owners now maintain private global/tenant reservation totals
and lifetime manifest-leaf totals. Only successful admission increments them;
cancellation or confirmation releases a reservation exactly once, while retained
operation/leaf counts never decrease. Root claims use a private object-identity
index with one key per root, scoped to the checked service and full tenant,
namespace, object and incarnation identity. No new public mutation/import API or
recovery authority follows from these indexes. At this checkpoint, confirmed-object
usage and root-only permission lookup still scanned history; the follow-up below
removes those scans.

`make test-admission-resources` additionally writes `.tmp/admission-history.json`.
Its fixed test installation admits then cancels 128 one-byte operations per tenant,
filling 256 lifetime slots. Synthetic roots exercise reservations only; no file
bytes, manifests or provider effects are supplied. Tenant exhaustion leaves the
other tenant's capacity available. Rejections acquire no operation; exact replay,
cleanup and zero byte charges survive stop/start at capacity. Every admission and
cancellation is below a broad 5M-instruction ceiling; final allocated memory stays
within 2 MiB growth. These are maintained regression bounds, not production limits.

The [before/after record](admission-history.json) identifies both Wasm builds and
all six checkpoints. It compares the 0.2.0 model under the same fixture with the
indexed implementation. Selected total-call observations illustrate the tradeoff:

| Retained operations | Baseline admission instructions | Indexed admission instructions |
| --- | ---: | ---: |
| 1 | 935,120 | 936,813 |
| 128 | 1,443,874 | 1,480,278 |
| 129 (second tenant's first) | 1,482,947 | 1,440,578 |
| 256 | 1,732,502 | 1,802,377 |

Both start at 1,245,184 allocated Wasm bytes. The baseline ends at 1,572,864;
the indexed owner ends at 1,638,400, an extra 64 KiB. Explicit scans are removed,
but the measured total cost is not uniformly lower. Decoder, allocation/layout
and page effects are included without isolating their contributions. Do not infer
constant whole-call cost or a production throughput improvement from this sample.
That build's 10 MiB manifest sequence separately measures 4,113,659 instructions,
1,766 wire bytes and steady 1,245,184 allocated Wasm bytes. Both reports exclude
diagnostic storage/reply encoding, production persistence and provider transport.

Native accounting audits independently reconstruct totals through rejected and
successful transitions, zero bytes, tenant interleaving, cancellation, confirmation,
release, deletion, settlement and exact replay. Existing wide-total and capacity
tests remain effective. Targeted native validation, all 12 admission and 46
integrity/recovery PocketIC cases, upload ownership, affected Clippy/release Wasm
and library rustdoc pass. No full CI, provider effect or release command ran.

## Confirmed usage and indexed exposure

Confirmed catalog usage now keeps private global and tenant totals. One mutation
path replaces the affected entry's contribution after reference, deletion or
settlement changes. It also accounts for a newly recorded lifecycle error receipt
without changing byte charges. Unchanged retries and rejected requests preserve
totals; subtraction precedes addition for wide aggregates. Global usage is a copy
and tenant usage a lookup; neither scans object history. Enumeration keeps its
independent scan/result bounds and tenant index.

The admission owner additionally retains one root-to-tenant/operation mapping per
accepted permission. Root-only exposure uses that mapping and the original
permission, then rechecks uploader, activation, deadline and phase. No rejected
admission can install or overwrite the mapping. Cancelled and settled history
remains bound; there is no counter/index import, reset or new public API.

Native evidence reconstructs every usage field from journals across 64 objects,
three tenants, distinct namespaces, multiple references, rejected-operation
receipts, zero bytes and totals above `u64`. Each transition and exact replay is
checked. Another case separates operation ID from object identity and rejects
object/root conflicts without leaking a permission-index entry. PocketIC fills
256 operation slots and exercises the last valid permission through unprepared,
wrong-caller, cancelled-root and rejected-admission paths, stop/start and one-shot
exposure, preserving the original declaration and quota.

The latest build and reports are retained in the
[same measurement record](admission-history.json). The 256-cancellation workload
still ends at 1,638,400 allocated Wasm bytes; the 10 MiB sequence measures 4,116,047
instructions with unchanged 1,766 request bytes and 1,245,184 allocated Wasm bytes.
Equal allocated pages do not imply that the new index costs no memory. These
workloads do not measure a full confirmed catalog or retained receipts/manifests,
and establish no production throughput or persistence/provider guarantee.

Targeted validation passes 30 service and 20 catalog/admission unit tests, 23
catalog/upload/tenant integration cases, all 13 admission and 46 integrity/recovery
PocketIC cases, and upload ownership. Affected release Wasm, strict all-target
Clippy, warning-free library rustdoc, formatting and diff checks pass. Full CI
and release/provider actions were not run.

### Tenant content discovery and release reuse

The post-0.2.0 shared-owner API now discovers original operations by explicitly
authorized tenant, namespace and root. It returns current reservation/lifecycle
state, preserves retired identities and uses the existing index without a history
scan. It does not reserve a reference or prove present provider availability.
Native tests separate operation from object ID, deny foreign discovery, preserve
suspended inspection and exercise overlapping release references. Lost retain
replies recover exact receipts; stale live reads cannot bypass queued deletion.
Receipt exhaustion preserves cleanup capacity, physical deletion remains separate
from billing cessation, and settled-root reallocation rejects without mutation.

Targeted commands: `cargo test --offline --locked -p ic-blob-storage --lib model::service`
(33 passing), and `cargo test --offline --locked -p ic-blob-storage-pocketic-tests
--test admission -- --test-threads=2` (15 passing), with this repository's target
directory and explicitly selected PocketIC 16.0.0 binary. The release admission
Wasm SHA-256 is `5f9e8d93f1cdbc91a900b02af85faef3bb159a8a0b20eabcce21947029b21804`.
The IC cases verify actual caller isolation, unchanged observations/accounting,
uncertainty after revocation, suspended inspection and cancelled history through
stop/start. They do not simulate provider confirmation through a new control.
Affected strict all-target Clippy, warning-free library rustdoc, release Wasm,
formatting and diff checks pass. Existing resource cases pass their broad budgets;
the earlier exact measurement records remain tied to their original artifacts.
No production endpoint, persistence, restore, provider behavior, large release
capacity or deleted-content reintroduction is qualified by this batch.

### Multi-file release history

The next local workload uses 640 synthetic 256 KiB files and 64 synthetic 2 MiB
files: 704 objects / 288 MiB, with 768 retained manifest leaves. File hashing runs
on the test host; only declarations/manifests enter the canister. Four overlapping
reference generations retain 2,816 reference IDs and 4,928 receipts. The final
reference is releasable at history capacity; physical deletion and billing cessation
then discharge separate byte totals without refunding lifetime admission slots.
Exact retries and stop/start preserve history. Operator-only completion, deletion
and settlement controls are explicit substitutes, not deployed Caffeine evidence.

The [machine-readable report](release-history.json) binds the release Wasm hash
and samples. Allocated Wasm memory grows from 1,245,184 to 4,587,520 bytes, including
allocator headroom, not just live payload. At object 704, admission measures
5,430,827 instructions, of which 5,334,922 precede workflow dispatch. The largest
sampled retain total is 9,197,167; maximum retain workflow work is 495,359 and
maximum release workflow work is 334,171 (independent maxima, not the same calls).
The growth is predominantly before workflow dispatch; decoder/allocation work
needs investigation before production sizing. No allocator root cause is proven.
Counters exclude diagnostic recording and reply encoding; every measured mutation
stays below the broad 10M ceiling. Larger per-object histories and read sessions
remain unmeasured, and no persistence/provider cost follows from these numbers.

The new reference-capacity view reports unused identities, unreserved receipts,
reserved release receipts and fresh distinct retains. Native cases cover odd
receipt budgets, admitted failures, effect-free inspection and zero fresh retains
after deletion queues. The shared owner authenticates tenant disclosure; the IC
workload denies foreign observers and unauthorized substitute facts.

Targeted validation: 15 lifecycle and 33 service native cases; all 16 admission
PocketIC cases using PocketIC 16.0.0; admission release Wasm; strict all-target
Clippy for the core, protocol, admission probe and PocketIC package; warning-free
library rustdoc; formatting and diff checks. The final admission suite took about
271 seconds locally. `make test-admission-resources` now also writes
`.tmp/release-history.json`; its large workload is intentionally several minutes.
No full CI, release, live provider operation or consumer adoption ran.

### Operation-specific admission inputs

After the maintainer pushed 0.2.1 (`85967c82902c4730614ca5bd28ec81fea382f28a`),
the private admission probe replaces its catch-all Candid command envelope with
separate enrollment, admission, preparation, exposure, revocation, reference and
operator-substitute inputs. The in-process dispatcher and service model are
unchanged. No public library API or production transport is added. Every mutation
keeps the same byte/work/type-header/skip limits, including trailing-argument
validation. Rejected decoding rolls back without replacing the last observation.
Diagnostics retain one operator-only sample, now separating decoder entry, header,
value, final input disposal and workflow counters.

The [new reports](admission-inputs.json) retain exact Wasm hashes and tool versions;
the [released report](release-history.json) remains unchanged. The same synthetic
workload, authority checks, exact retries, stop/start, saturated cleanup and
separate deletion/settlement pass through the new boundaries.

| Observation | Released baseline | Narrow inputs |
| --- | ---: | ---: |
| 10 MiB admission/prepare/retry/exposure instructions | 4,116,047¹ | 3,302,052 |
| Same sequence request bytes | 1,766¹ | 1,355 |
| Admission at 704 confirmed objects | 5,430,827 | 3,985,302 |
| Peak retain instructions | 9,197,167 | 6,783,953 |
| Peak release instructions | 9,196,705 | 6,622,482 |
| Final allocated Wasm bytes | 4,587,520 | 4,521,984 |

¹ Earlier accounting-build measurement in [admission-history.json](admission-history.json),
not a remeasurement of the 0.2.1 tag. The 704-object comparison uses the released
multi-file artifact. Compiler/layout and extra diagnostic calls are included;
these comparisons do not isolate a pure decoder CPU speedup.

PocketIC 16's [release metadata](https://github.com/dfinity/pocketic/releases/tag/16.0.0)
pins IC commit `fc21803c3c3a8dd452b3b58b959751c41fecb89c`.
Its [subnet configuration](https://github.com/dfinity/ic/blob/fc21803c3c3a8dd452b3b58b959751c41fecb89c/rs/config/src/subnet_config.rs)
sets 5,000 instructions per OS page; its
[memory tracker](https://github.com/dfinity/ic/blob/fc21803c3c3a8dd452b3b58b959751c41fecb89c/rs/memory_tracker/src/deterministic.rs)
charges the 16 OS pages in a 64 KiB region on first access and again on first write.
That explains the 80,000-instruction charge granularity. Observed phase increments
are consistent with it, but these reports do not count pages or separate all CPU
and memory charges. Moving the first touch between phases can shift apparent
workflow cost without changing the workflow. No allocator root cause is proven.

At object 704, 3,889,357 instructions still precede workflow dispatch; narrower
inputs reduce the overhead but do not eliminate retained-state growth. Production
sizing must include allocation locality, larger per-object histories, read sessions,
reply encoding and durable storage. No provider transport/completion, funded
installation, restore path or production capacity follows from this experiment.

Targeted validation: admission release Wasm, all 16 existing admission PocketIC
cases (207 seconds), a focused cross-caller check for each substitute endpoint and
both reference actions, affected strict all-target Clippy, formatting and diff
checks. Full CI, version mutation, publication and provider effects were not run.

### Reference history without lifecycle copies

Reference requests now compute one private transition instead of cloning the
complete `BlobLifecycle`. Binding, current reference state and lifetime capacity
are checked first; the resulting active count must still leave one receipt per
live reference. Only then are the exact receipt and reference transition published
synchronously. The plan cannot leave the model or cross an await. Lifecycle errors
still produce retained failure receipts when capacity permits, while scope,
request conflicts and capacity rejection mutate neither history. Public API and
error/retry ordering are unchanged; this is local return-path atomicity, not a
durable transaction or a recovery mechanism.

The [measurement record](reference-history.json) contains before/after release
Wasm hashes and counters. Its new single-object workload fills 256 simultaneously
live reference slots and all 511 receipt slots. It checks capacity observations
throughout, rejected fresh/unknown operations, stop/start, exact replay at full
capacity and after settlement, all releases, and separate physical/billing cleanup.
Operator completion/deletion/settlement are explicitly local substitutes.

| Whole-call observation | Before copying removal | After |
| --- | ---: | ---: |
| 256-reference peak retain instructions | 1,085,132 | 1,034,109 |
| 256-reference peak release instructions | 1,248,789 | 1,195,115 |
| 256-reference final allocated Wasm bytes | 1,441,792 | 1,441,792 |
| 704-object peak retain instructions | 6,783,953 | 5,104,779 |
| 704-object peak release instructions | 6,622,482 | 4,867,114 |
| 704-object final allocated Wasm bytes | 4,521,984 | 4,521,984 |

The 704-object prior report is in [admission-inputs.json](admission-inputs.json).
These observations include layout and memory-region charges, so individual phase
maxima are not uniformly lower: the 256-reference maximum release workflow counter
increases from 234,824 to 340,459 despite the lower whole-call peak. Removing the
history copy eliminates a source-level linear allocation/copy operation; it does
not prove a pure CPU speedup or constant whole-message costs. The latest final
admission is 3,825,260 instructions, of which 3,729,316 precede workflow dispatch;
that admission change is not attributed to the reference algorithm. Read-session,
persistence, reply-encoding and provider costs remain outside these measurements.

A separate pre-optimization experiment replaced only the private probe's default
heap allocator with [Talc 5.1.1](https://docs.rs/talc/5.1.1/talc/wasm/index.html),
using its safe `new_wasm_dynamic_allocator()` constructor. The record includes
configuration, package checksum and Wasm hash. All 17 then-existing admission
cases passed, but final admission rose from 3,985,302 to 4,148,196 instructions;
peak retain fell from 6,783,953 to 5,909,854, and final allocated memory rose from
4,521,984 to 4,653,056 bytes. This mixed result does not justify selecting it.
The temporary allocator declaration, feature and dependencies were removed.
The library does not own a global heap allocator; ic-memory's stable allocation
governance and dependency alignment are unchanged.

Targeted validation passes 15 lifecycle, 33 service and 20 catalog unit cases;
28 lifecycle binding/catalog/tenant/upload integration cases; all 18 admission
PocketIC cases (247 seconds); affected strict all-target Clippy, warning-free
library rustdoc, admission release Wasm, formatting and diff checks.
`make test-admission-resources` also writes the
new `.tmp/reference-history.json` report. No full CI, release, provider operation
or production capacity selection ran.

## Single-slot read resources

The 2026-09-27 [matched read experiment](read-resources.json) keeps the default
Rust allocator, as requested by the maintainer. `make test-read-resources` builds
only the authority/source fixtures and runs the readback cases; all per-call
samples go to `.tmp/read-resources.json`. The archived comparison pins both Wasm
hashes, Rust 1.98.1, PocketIC 16.0.0 and the existing Candid/serde_bytes versions.

| Full 1 MiB leaf, 12 successive reads | Vec byte decoder | Bulk blob decoder |
| --- | ---: | ---: |
| Measured service workflow instructions | 234.4–234.6M | 115.9–116.0M |
| Candid decode instructions | 122.1M | 3.63M |
| Manifest verification instructions | 81.1M | 81.1M |
| Allocated Wasm memory after each read | 5,898,240 bytes | 7,012,352 bytes |

`serde_bytes::ByteBuf` selects Candid's checked `vec nat8` bulk-copy path instead
of visiting each byte; conversion into the existing reply vector transfers its
allocation. The wire format, encoded-size check, decoder quotas, hash binding and
pre/post-await authorization remain unchanged. The higher allocated heap is a
measured tradeoff, not evidence of higher live-byte requirements or a memory leak;
this workload shows no growth across repeated reads. No allocator was substituted.

The bounded diagnostic overwrites one volatile observation after each completed
workflow. Only the configured operator can inspect it; restored owners deny it.
It is neither durable evidence nor authority to clear a slot. Real PocketIC calls
cover 3-byte and full-chunk success, corruption, oversized/malformed/wrong-type/
truncated-encoding replies, transport rejection and interleaved busy/foreign
callers. A held full-chunk read keeps its one slot without extra source effects.
Callback traps publish no completed profile and preserve uncertain intent;
release, revocation, stop/start and forced-restore cases retain their guarantees.

Counters use the IC's call-context counter across awaits, including fixture
journal saves and IC memory-access charges. They exclude ingress decoding,
source execution, final response encoding and storing the diagnostic itself.
Allocated memory is sampled after messages, not peak live memory. Reply-size
checks limit application decoding, not the IC's earlier response buffering.
The local source is a substitute, not a deployed Caffeine read transport.

All 48 journey/recovery cases and affected strict all-target Clippy pass. The
optimized artifacts were restored after the comparison and their hashes match
those validated by the complete journey suite. Full CI and provider trials did
not run. This sizes the existing single-slot experiment; it neither selects a
production read proxy nor justifies multiplying its costs into a concurrency
limit. Consumer download/integrity placement and production persistence costs
must be decided before that limit is frozen.

## Consumer root verification

The 2026-09-27 follow-up adds `CaffeineRootVerifier`, binding the expected root,
length and original metadata at construction while reusing the existing
fixed-memory tree/raw hasher. It consumes arbitrary bounded frames without a
leaf manifest or independently trusted raw digest. Only finalization checks the
root; accepted prefixes are not verified or durable bytes. A matching root is
not a provider completion or tenant-authority receipt.

Independent published-client vectors cover metadata, chunk boundaries, uneven
and repeated-leaf trees and 10 MiB content using a fixed 65,537-byte receive frame.
Native cases reject substituted raw-digest identities, corrupt bodies, metadata
changes, truncation and excess input. The local `verify_download` example also
checks successful EOF, including interrupted reads, trailing bytes in a later
frame and connection failure after the expected final byte. It emits no success
on those failures and performs no destination write or network request. Its
unit test is included in ordinary Cargo testing through `test = true`.

Validation passes 20 Caffeine model cases, nine independent-vector integration
cases, the example transport-boundary case, core all-target strict Clippy, Wasm
compilation and warning-free rustdoc. The example verifies the independent
`abc-text` claim/body end to end. Full CI and browser/provider trials did not run.
The [download direction](../operator-guide.md#download-a-verified-file) records
remaining descriptor provenance, original-metadata retention and browser delivery
requirements. No allocator, dependency or production endpoint change is included.

## Retained download descriptors

The 2026-09-27 descriptor follow-up retains the first validated metadata set
alongside each prepared permission's manifest. Root, declared length and leaves
are still checked before publication; header names/case/values/order are copied
only on the first successful preparation. Reordered retries preserve that copy.
Existing per-object header limits and lifetime object counts bound retention;
no new default, public mutation API, allocator or stable schema was introduced.

`UploadAdmissions::content_descriptor` returns a borrowed view after the same
service/namespace/tenant checks as discovery. It contains the original request,
current lifecycle and retained headers, including uncertain/cancelled/settled
history. Reads grant no reference or publication permission and change no
accounting. Unknown, foreign and unprepared roots return no descriptor; suspended
tenants retain inspection. A configured provider locator, authenticated/certified
consumer publication and serving policy are still required outside this view.

The private IC query applies bounded input decoding and delegates to the same
owner. Tests reject unrelated actors, controller status, foreign namespace/service
and oversized arguments; consume an actual descriptor with `CaffeineRootVerifier`;
and preserve metadata through reordered retries, cancellation, stop/start and
separate physical deletion/billing settlement. The owner exposes no mutable header
borrow and copies input strings, so later caller changes cannot rebind it.

[Resource evidence](descriptor-resources.json) pins the final Wasm and records
four maximal eight-header/1 KiB declarations across two tenants, retained after
cancellation at full object history. Repeated retries and suspension/stop/start
keep them readable without growing the 1,245,184-byte allocated heap. The existing
704-object/four-generation workload ends at 4,587,520 bytes, 64 KiB above the prior
reference batch. Peak retain/release costs are 5,425,185/5,505,129 instructions;
final admission measures 3,105,760. This is a sequential-build comparison with IC
memory-access charges and allocator locality, not isolated CPU attribution.
Per-object payload bounds do not qualify arbitrary total-memory configurations.

All 36 service model cases and 20 admission PocketIC cases pass, along with
strict affected all-target Clippy, release admission Wasm and warning-free core
rustdoc. `make test-admission-resources` now also writes the maximal metadata
report. No full CI, provider effect, certified response or production recovery
was exercised. Existing inspection-only and unsupported-upgrade boundaries remain.

The reference-qualified follow-up adds `retained_content_descriptor` using the
same metadata owner and confirmed lifecycle. Native cases cover authority and
every object binding, unconfirmed/cancelled content, unknown/released references,
suspension and separate cleanup states. A consumer cannot borrow another live
reference. Copied observations remain stale after release; no query reserves a
reference or writes a receipt. Existing historical descriptors remain readable.

The private bounded adapter's PocketIC case checks actual caller isolation,
oversized/zero-identity rejection, exact descriptor-to-body verification and two
overlapping references. Stop/start preserves the remaining reference. After its
release, replaying the old successful retain receipt still yields no descriptor;
deletion and settlement do not change that result. All 30 upload model cases and
four discovery/descriptor PocketIC cases pass, alongside strict affected Clippy
and release Wasm compilation. These targeted checks do not refresh the preceding
resource artifact or qualify certified publication, consumer transaction/outbox
coordination, production recovery or provider behavior.

## Local verified file output

The `verify_download` example now accepts an optional destination. The same
64 KiB stream that feeds the verifier writes to a temporary file under a private
staging directory; declared-length limits apply before writes. Clean EOF, final
root verification, flush and file sync precede `persist_noclobber`. The example
does not reopen the input or copy a different file after verification. Unix
staging directory/file permissions exclude group/other access. Only the example
adds the existing tempfile 3.27.0 dependency; public library dependencies and
heap allocation remain unchanged.

Seven native example cases pass: output stays absent through the final EOF/error
read; corrupt/short/excess/late-error input leaves no output or normal staging
residue; short/interrupted writes complete and write/flush errors reject;
existing, racing and dangling-symlink targets are preserved. An actual CLI run
uses the independent `media-10485760` vector, compares the saved file's raw SHA-256
with the published fixture digest, checks verify-only output, rejects a repeated
destination, corrupted body and insufficient byte budget, and emits no success
receipt on rejection. Normal shell-piped input also verifies. Strict core
all-target Clippy and library Wasm compilation pass; no full CI ran.

The client does not treat the byte count alone as completion. An interrupted
process can leave staging residue. Publication can
precede a lost stdout receipt; there is no automatic overwrite, resume or orphan
cleanup. The caller must control the destination directory, and the parent is
not synced. Platform behavior follows
[tempfile's no-clobber persistence contract](https://docs.rs/tempfile/3.27.0/tempfile/struct.NamedTempFile.html#method.persist_noclobber),
not a portable crash-durable transaction or authenticated browser delivery.

## Client manifest preparation

`CaffeineManifestBuilder` collects completed leaves from the existing streaming
hasher through a private observer; raw and leaf hashes are each computed once.
The existing fixed-memory hasher retains no new fields or leaves. Construction
validates length, bounds `ceil(bytes / 1 MiB)` and reserves the leaf vector before
accepting body input. Appends share offset/work/length checks before mutation;
finalization moves the ordered leaves into an immutable manifest without a copy
or second tree pass. The library adds no dependency or allocator change.

Independent vectors compare every leaf, raw digest and provider root under
65,537-byte frames, covering exact boundaries, partial/uneven trees, repeated
leaves, metadata and 10 MiB content. Native cases also cover multiple leaves in
one append, rejected appends after completed leaves, constructor budgets and
incomplete finalization. The local `prepare_upload` example shares the service's
metadata validator, rejecting malformed metadata and resource limits before
reading its input. EOF, truncation, excess and late read errors are tested.

An actual 10 MiB CLI preparation matches the independent `media-10485760` vector;
its generated claim verifies and saves the body through `verify_download`, with
the saved raw digest independently checked. Byte/leaf limits, inconsistent length
metadata and short/excess input reject without a successful JSON result. A
PocketIC case supplies the generated leaves to shared admission, which returns
no retained descriptor before substituted completion. The returned original
metadata then verifies the independently generated body on the client. No body
is sent to the canister, and completion remains an explicit fixture substitute.

Targeted checks pass 23 Caffeine and 39 service model cases, ten independent-vector/
integrity integration cases, nine example cases and five discovery/descriptor
PocketIC cases, strict affected all-target Clippy and release admission Wasm.
No full CI, provider call or browser integration ran. File freezing, resumable
publisher operation identity, authenticated descriptor delivery and deployed
provider guarantees remain outstanding. Earlier resource measurements retain
their original artifact identities and are not claims about this build.

## Offline multi-file inventory

`prepare_upload --inventory` reuses the same single-file preparation path. It
checks the entire input's asset IDs, relative paths, metadata and file/aggregate
byte/leaf budgets before opening body sources. Each regular source is then
checked and hashed sequentially, including duplicate content. The report keeps
every asset mapping while grouping prepared blobs by provider root; metadata
changes can make identical raw bodies distinct blobs. No report is emitted before
every source succeeds. Source failures identify their asset and path.

Seven preparation example cases and strict core all-target Clippy pass. Cases
cover reordered duplicate metadata, distinct metadata roots, all inventory limits,
overflow, input-size/JSON rejection, duplicate assets, traversal, nonregular files,
wrong length and leaf/parent symlinks. An actual CLI run with 704 synthetic asset
entries (2,112 source bytes) produces 352 roots, preserves every mapping, repeats
deterministically, and emits no report after an aggregate-budget failure or a
missing final source. This exercises inventory behavior, not media throughput.

The caller must control the unchanged source tree; checks do not prevent a
concurrent filesystem adversary or freeze source bytes. Totals cover the local
inventory only: they omit existing service history, references and obligations
and establish no tenant authority, fresh-upload eligibility, reservation or
provider price. Core APIs, dependencies and allocator are unchanged. No full CI,
live service query, upload or consumer-repository change ran for this follow-up.

The optional `--snapshot PARENT_DIRECTORY` follow-up saves the exact buffers
passed to the hasher in a fresh private directory. It retains one body per root
and writes the complete inventory last. Duplicate asset mappings remain separate;
later source replacement cannot change the saved copy. Ordinary failures remove
only the current attempt, and repeated runs never overwrite earlier snapshots.

Ten preparation example cases and strict core all-target Clippy pass. Added cases
cover independent-vector saved bytes, source replacement, duplicate mappings,
Unix permissions, repeat isolation, late source failure cleanup and interrupted,
short, failed or unflushed writes. An actual 10 MiB CLI snapshot with two asset
mappings retains one body, matches an independently computed raw SHA-256 digest,
and passes `verify_download` after the original is deleted. A failed repeat emits
no success report and preserves the earlier snapshot without staging residue.

Files are synced before keeping the directory, but directory entries are not:
this is not a portable crash-durable transaction. A crash can leave incomplete
residue or a completed copy without its stdout receipt. Saved local files remain
mutable and require reverification before later effects; neither a snapshot nor
its report supplies authenticated descriptors or resumable operation identity.
Core APIs, dependencies and allocator remain unchanged. No service/provider call,
PocketIC rerun, full CI or historical resource-record refresh ran for this change.

## Tenant admission capacity

`UploadAdmissions::admission_capacity` reads maintained reservation, catalog and
manifest totals in one synchronous observation. It authenticates the exact
enrolled tenant/service/namespace and reports the tighter tenant/global bounds
without scanning history or exposing another tenant's records. Byte headroom
includes pending reservations, physical storage and continuing billing. Lifetime
object/manifest headroom remains consumed through cancellation and settlement.
Suspended tenants can inspect, but cannot use headroom for fresh admission.

Targeted validation passes 35 upload model cases (including five new capacity
cases), three actual IC capacity-query cases, strict all-target Clippy for the
core/protocol/probe/PocketIC packages, release admission Wasm and warning-free core
rustdoc. Tests cover scope/input rejection, passive reads, wide arithmetic, shared
contention, failed/exact retries, separate cleanup obligations, suspension and
stop/start. No full CI or historical resource measurements were rerun. Provider
completion/deletion/settlement in the IC cases remain explicit operator substitutes.
The inventory publisher is not yet connected to this private query; counts reserve
nothing and do not prove root availability, funded capacity or durable recovery.

The subsequent `blob-fixture-inventory` command connects prepared reports to
these local queries. Input validation reuses core manifest/metadata checks and
recomputes totals; it opens no source paths. Bounded sequential queries distinguish
not-visible content, unfinished operations, live reference demand and retired
roots. One fresh reference per asset is explicit; reference sizing for newly
admitted objects is not assessed. Missing content never proves global absence,
and no result reserves resources or authorizes effects.

Seventeen operator unit cases pass, including eleven inventory cases for explicit
targets, input tampering/bounds, wide values, reply bindings, inconsistent/missing
observations and query failures. Two actual executable/PocketIC cases pass through
absence, admission, completion, duplicate reference pressure, settlement and
stop/start; repeated reads preserve state and foreign tenants cannot see original
operations. Completion/cleanup remain operator substitutes. Strict host-tool
all-target Clippy, formatting and diff checks pass. Existing locked dependencies
are reused by the unpublished client only. No full CI, provider call, production
authentication, body reverification or historical resource refresh ran.

## Exact reference receipt inspection

The maintained `blob_reference_receipt` query binds the full original upload and
reference command. Its shared bounded decoder preserves explicit absence, recorded
success or lifecycle failure, and service refusal as separate outcomes.
`blob-fixture-reference` uses it against local standalone and durable storage hosts.
The transient fixture receipt endpoint, DTO, conversion and superseded cases are
removed. The heap model's receipt read remains used by mutation replay.

The local v1 intent journal retains independent full-width upload, object, lifetime
and first-reference IDs. The scoped upload/operation key makes changed original
arguments conflict. Eleven tool unit cases cover bounded inputs/replies, typed
refusals, exact recovery, no-clobber files, locking, capacity and independent IDs.
A separate subprocess case kills the actual lock holder and recovers a write whose
success output was discarded. This is local Linux process-interruption evidence,
not power-loss, production persistence or restored-writer authority.

Eight targeted PocketIC cases pass: one standalone saved-intent refusal/restore
case and seven durable receipt/mutation cases, including two CLI subprocess
journeys. Historical success survives release, settlement and fenced restoration;
querying leaves stable memory unchanged. Standalone unknown and unconfirmed uploads
remain service refusals, never absent receipts. Confirmed provider facts remain
labelled substitutes. Strict affected all-target/all-feature Clippy and release
Wasm builds for standalone, admission and durable storage pass. No full CI or
deployed provider call ran; transport still uses simulated PocketIC identity.

## Shared stable tenant enrollment

`StableTenantEnrollments` starts shared canister storage with a host-granted
`ic-memory` stable map. Bounded v1 records retain enrollment generation and active
state; reserved metadata binds service, operator, namespace and lifetime count.
The stable store and existing heap owner call the same model transition, covering
compare-and-set updates, suspension and generation exhaustion. Each mutation
reads/writes an individual tenant record. Reopening validates the metadata and
all enrollments in one pass before returning an enforced mutation fence.

Native evidence passes 55 service tests, including seven new storage cases and
two record-codec cases. A host tears down and reopens its actual `MemoryRuntime`
over retained `VectorMemory` and sees the original suspended state. Restored
mutation fails. Tests compare stable/heap transition results, preserve bytes on
authority/binding failures, reject stale preconditions, exercise full lifetime capacity and
generation exhaustion, and reject invalid records or unrecognized memory without
seeding replacement data. Maximum-width records round-trip within the 256-byte
encoded bound; malformed encodings fail. Two existing memory-composition cases
also pass, including absence of implicit allocation from library linking.

Strict core all-target Clippy, Wasm target compilation, warning-free rustdoc,
formatting and diff checks pass. Dependencies and allocator are unchanged.
These are native storage and compile checks, not canister install/upgrade,
trap-rollback, snapshot or resource measurements. The admission probe remains
transient and rejects upgrades. The stable enrollment component is not wired to
that owner; upload permissions, manifests, root claims, reference receipts,
accounting and the complete durable transaction remain next. The host must check
installation/release identity independently; reopening grants no restored-instance
or provider authority. No full CI or provider effect ran.

## Shared stable root claims and IC transaction evidence

`StableRootClaims` stores immutable root/object pairs and a reverse object index
in two host-granted `ic-memory` maps. The shared model owns uniqueness/replay and
lifetime-capacity decisions. Normal claims touch individual tree paths; reopening
checks every forward record against the reverse index and validates equal counts.
Missing, conflicting or orphaned entries reject without repair. Full history still
permits exact replay before restoration. Reopened stores always fence mutation.

Five new native record/store cases cover maximum-width identities, heap/stable
agreement, changed scopes/callers, byte-preserving rejection, missing memories and
inconsistent index pairs. All 83 targeted catalog/service/root cases pass.

In the preceding component batch, two PocketIC cases used enrollment and root stores through an
unpublished storage probe. Its memory wrapper deliberately traps during a reverse
index write, after the forward insert. The IC rolls back stable writes and heap
counts; the same root can then bind a different object, and the original object
can bind another root. Full capacity preserves exact replay and rejects conflicts.
The other case retains suspension and a full-width object ID through same-release
upgrade, validates the original claim, rejects all restored mutations and keeps
the fence through stop/start. A mismatched-operator upgrade traps and leaves the
old instance intact. Controller/operator roles do not grant tenant claim authority.

Release storage Wasm, strict affected all-target Clippy, warning-free core rustdoc,
formatting and diff checks pass. Make's fixture target includes the new unpublished
probe. External dependency versions and allocator are unchanged. Those cases did
not persist upload permissions, manifests, references, receipts or byte accounting,
and the probe does not run enrollment/quota admission or call a provider. The
complete upload transaction, snapshot safety, operational recovery and resource
envelope remain open. No full CI or historical resource-pin refresh ran.

## Durable pending upload transaction

The preceding pending-state batch replaced the independent component controls
with an owner then named `StablePendingUploads`. That owner held six host-granted maps:
enrollment, forward/reverse root claims, permissions, maintained totals and
immutable manifests. Shared model checks enforce exact admission, metadata/root
validation, uploader/generation/time rules and global/tenant reservation limits.
Unexposed cancellation releases bytes but retains operation/root/leaf capacity;
exposure followed by revocation retains uncertain bytes in every accounting view.

Seven native owner cases plus three codec cases cover heap/stable agreement,
exact retries, rejection before charging/claiming, suspension/reactivation,
original headers after reordered retries, exposure uncertainty, missing or altered
manifests and counters, configuration mismatch, full-width identifiers and
malformed records. A near-64 KiB declaration survives bounded decoding/reopening;
a candidate exceeding the codec envelope rejects before memory allocation. The
manifest map uses variable-size pages with a separately enforced 64 KiB codec
limit; a one-leaf declaration stays within a 128 KiB native stable-memory ceiling.
That ceiling is a local regression bound, not a production resource qualification.
All 93 targeted catalog/service/root native cases pass.

Four PocketIC cases use actual caller/time, the default allocator and the shared
pending owner. A fixture-only memory wrapper traps at reverse-root, permission
and accounting writes during admission; all earlier writes and cached state roll
back. Failed roots and object/operation identities can be independently reused.
Manifest preparation faults roll back both the declaration and permission flag.
A cancellation fault retains the original permission and byte reservation until
an exact successful retry. Exposure then revocation keeps all uncertain bytes.

The upgrade case retains a maximum-width operation/object identity, exposed and
revoked state, prepared cancellation history, enrollment suspension and exact
totals. A changed operator traps restoration and leaves the previous instance
unchanged. Valid same-release restoration validates all records without repair,
permits authenticated inspection and fences enrollment/admission/preparation/
exposure/revocation through stop/start. No test-only behavior is in production
code. The earlier raw-claim fixture endpoints were removed in this unpublished
batch; the underlying component native tests remain.

Release storage Wasm, strict affected all-target Clippy, warning-free core rustdoc,
formatting and diff checks pass. No dependency version or allocator change, full
CI, resource benchmark refresh, certificate, provider call or paid effect ran.
That pending-only component could not confirm an upload or mutate references. Confirmed lifecycle,
reference receipts, deletion/billing obligations, snapshot freshness and
operational recovery remain open; inspection-only upgrade does not close M2.

## Durable confirmed lifecycles and exact reference receipts

The current `ops::service::uploads::StableUploads` replaces the earlier unpublished
pending-only owner. It adds three host-granted memories for confirmed metadata,
individual reference identities and individual exact receipts. The nine-map owner
has no independent mutable escape. Confirmation commits the first reference,
permission phase and reservation-to-confirmed accounting together. Reference
mutations use the same constant-size transition decisions as `BlobLifecycle`;
both owners reserve one receipt for every active reference's eventual release.
No full object/reference/receipt history is cloned or loaded for a mutation.

Six added owner cases compare stable and heap accounting through settlement, check
cleanup headroom and suspension, preserve a recorded failure after the reference
later becomes live, restore every lifecycle phase, reject missing/orphaned records
without repair, and reject unexposed/changed completion without releasing quota.
Two codec cases cover maximum-width IDs/principals, all reference result variants,
retained counts and malformed/oversized receipt rejection. All 113 targeted
catalog/service/lifecycle native cases pass. Historical retain success is distinct
from current liveness, and settlement never permits reference reactivation.

The storage PocketIC suite now has seven cases. Its three added cases exercise:

- Completion traps at initial-reference, permission and accounting writes. The
  reservation and exposure phase survive with no leaked confirmed reference.
- Reference/receipt/metadata write failures and last-release accounting failure.
  The IC rolls back earlier writes; retries consume exactly one receipt. Reserved
  release capacity survives failed requests and tenant suspension. Deletion and
  settlement accounting traps preserve the previous obligation until exact retry.
- Same-release upgrade in Live, DeletionPending, ProviderDeleted and Settled phases.
  Charged totals, reference liveness and exact receipts survive. Read authority and
  the permanent restored mutation fence remain enforced through stop/start.

The fixture authenticates its configured operator before supplying any upload,
physical deletion or billing cessation fact. These are labelled local substitutes;
neither controller status nor a tenant call grants this test authority. Core
confirmation methods consume facts authenticated/correlated independently by the
host and perform no provider calls. These tests do not qualify deployed Caffeine.

Release storage Wasm, strict affected all-target Clippy, warning-free core rustdoc,
formatting and diff checks pass. Dependencies and the default allocator are
unchanged; no full CI, resource-pin refresh or paid/provider action ran. Durable
reference-qualified descriptors/traversal, provider-call intents, read sessions,
provider economics and operational recovery remain incomplete. All restored
mutation remains fenced; persisted counters do not establish freshness authority.

## Durable indexed discovery and bounded read traversal

The ten-memory `StableUploads` owner now retains root-to-request identity separately
from immutable object identity. The new index commits in admission and must agree
with every permission on reopen; missing, changed and orphaned index entries reject
without repair. Root discovery uses tree lookups and never assumes the upload ID
is the object ID. A maximum-width request with a different object ID is covered.

Descriptor reads copy original headers from one bounded manifest record, not file
bytes or other objects' manifests. Reference-qualified reads require confirmed
completion and the exact live reference in the same synchronous owner observation.
Another live reference or an old retain success cannot substitute. Suspension and
the restored fence preserve tenant inspection; queries confer no provider locator,
certified serving, publication coordination or fresh mutation authority.

Traversal restricts tenant ranges before inspecting rows; cross-tenant scans require
configured operator authority. Scan and result budgets are independent host inputs.
Cursors bind service, namespace, scope, filter and last inspected tenant/request ID.
Filtered empty pages advance; new sweeps are required for changes behind a cursor.
Outstanding results include physically deleted objects while billing is unresolved.
The private fixture fixes both budgets to one instead of accepting caller limits.

Seven new native cases cover independent identity/index corruption, exact-reference
metadata, tenant range isolation, empty pages and phase changes behind a cursor,
independent limits, full-width endpoints, and continuing billing after deletion.
All 120 targeted catalog/service/lifecycle cases pass. The storage PocketIC suite
now passes nine cases: its admission rollback case additionally traps at the new
index write; two added cases check caller/descriptor isolation through upgrade and
cursor rejection, empty-page continuation and fresh cleanup sweeps on the IC.
These reads leave accounting and receipt history unchanged.

Release storage Wasm, strict affected all-target Clippy, warning-free core rustdoc,
formatting and diff checks pass. Default allocator and dependency versions remain
unchanged. No full CI, resource benchmark refresh or provider effect ran. Capacity
planning, provider reconciliation views and intents, durable read sessions and
operational recovery remain incomplete; inspection after upgrade remains fenced.

## Durable capacity and operator root reconciliation

`StableUploads::admission_capacity` reads enrollment and maintained global/tenant
counters. `reference_capacity` resolves the original request by root, then reads
one bounded confirmed record; it does not load reference/receipt history. Both
owners use shared model arithmetic. Lifetime slots and leaves remain consumed
through cancellation/settlement, active references retain release receipt slots,
and byte headroom remains limited by continuing billing after physical deletion.
Suspension/fencing preserve inspection without allowing fresh work.

`observe_roots` checks actual service, explicit namespace and configured operator
before indexed reads, including empty input. Ordered results retain duplicates,
malformed positions, exact request identities and every pending/confirmed phase.
There is no full-history scan or metadata load. This is an operator observation,
not a gateway callback contract, liveness boolean, provider completion proof or
permission to delete/retry. Hosts select trusted batch bounds and decoder limits.
The probe fixes processing limits to eight entries and 256 raw bytes; these tiny
fixture limits do not establish a production resource envelope.

All 122 targeted catalog/service/lifecycle native cases pass. Existing heap/stable
journeys now compare capacity through admission, cancellation, reference replay,
release, physical deletion and settlement. New cases cover scope, suspension,
restored inspection, malformed/duplicate batches and all retained root phases.
Shared global contention is observed without revealing other tenants' identities.

All eleven storage PocketIC cases pass. The two added cases exercise actual caller
and namespace/service checks, cleanup receipt headroom, continuing billing,
count/byte rejection, ordered root results, unchanged accounting and same-release
upgrade into the enforced mutation fence. Provider facts remain labelled operator
substitutes. No call to deployed Caffeine or paid effect ran.

Release storage-probe Wasm, strict affected all-target Clippy and warning-free core
rustdoc pass, alongside formatting and diff checks. No dependency version or
allocator change, full CI or resource benchmark refresh ran. This completes the
current durable planning/read batch; provider intents, callback authority, read
sessions, adapters and operational recovery remain open. Release/version/commit
and publication remain maintainer actions.

## Durable local funding intent and attachment journal

`StableFundingJournal` owns two host-granted memories. Exact intent rows bind the
service, Cashier candidate, account, namespace, operation and offered attachment;
a separate row retains configuration and allocation totals. Shared model arithmetic
reserves the original full offer, distinguishes known refunds from proven unsent
amounts, and retains accepted/uncertain attachments independently of provider credit.
Ordinary mutations touch individual rows instead of reconstructing history.

Preparation and the first attempt marker precede any possible host effect. Pending
or uncertain intents block later reservations. Repeated attempt markers reject,
terminal replay cannot refund twice, and conflicting outcomes preserve history.
Reopen validates the complete bounded journal and recomputed totals without repair,
then fences every mutation, including late outcomes. Increasing operation IDs are
only local ordering; they cannot establish freshness or release the restore fence.

Eight native journal cases cover exact identity/replay, reservation conservation,
shared reconstruction agreement, invalid refunds, lifetime overflow, all restored
phases, scope/authority, missing/orphaned/changed records and maximal record widths.
All 162 targeted billing/policy/catalog/service/lifecycle cases pass, as do the nine
existing funding-probe unit cases affected by shared allocation arithmetic.

All thirteen storage PocketIC cases pass. Two added journeys trap after intent
writes but before accounting writes, verify rollback of both stable and cached
state, reject a second attempt, preserve every phase through same-release upgrade,
and reject late outcomes under the fence. Transport facts are labelled substitutes;
the fixture sends no cycles and performs no provider call. This is not deployed
Cashier evidence, independent credit reconciliation or production call admission.

Release storage-probe Wasm, affected all-target strict Clippy, warning-free core
rustdoc, formatting and diff checks pass. Final checks used the concurrent
ic-testkit 0.10.1 Cargo update, which this work preserved. No allocator or package
version change, full CI, resource benchmark refresh or paid action ran. Cashier
method/payload binding, authenticated callbacks, complete account activity,
spendability/fee gates, other provider intents and operational recovery remain open.

## Durable Cashier request binding and transport-context correlation

On 2026-09-27, an anonymous read of Cashier `72ch2-fiaaa-aaaar-qbsvq-cai`
`candid:service` through `https://icp-api.io` with the mainnet root key returned
SHA-256 `232b08e4514048d4de48d6d1bf4387f577bfb64c7e2e2ded699a5e52d475d76f`,
matching the retained `caffeine-cashier.did`. This verifies the advertised interface,
not deployment behavior, account authority, provider credit or effect guarantees.

`CashierTopUpRequest` owns the fixed method and canonical Candid encoding. The
request/account options are present explicitly; the supplied optional positive
target balance is retained independently of the offered attachment. Two fixtures
under `tests/fixtures/caffeine-top-up/request-*.hex` were generated independently
using `didc encode --method account_top_up_v1 --defs docs/evidence/caffeine-cashier.did`.
They cover absent target and the maximum u128 target with an explicit account.
Local operation/namespace identities have no provider wire fields and supply no
remote deduplication or completion-query contract.

The durable intent records the method and all argument choices; same-release
encoding reconstructs exact bytes. Attempt marking returns that request only after
its write. Operator inspection remains available while fenced. Changed target
options reject as identity conflicts. Transport observations check independently
supplied service/target context before changing allocation; the host must still
authenticate transport and carry the exact original intent across the call.

All 169 targeted core billing/policy/catalog/service/lifecycle/Cashier cases and
fourteen storage PocketIC cases pass. The added IC case preserves canonical bytes
through upgrade, rejects changed target balances and unrelated callers, and keeps
the full offer charged after wrong-source or fenced late outcomes. Affected strict
Clippy, release storage Wasm, warning-free rustdoc, formatting and diff checks pass.
No paid call, account mutation, version change, full CI or resource benchmark ran.
Provider authentication, account-wide credit/activity and spendability remain open.

## Shared host memory grants — 2026-09-30

This continuation is based on released `v0.4.6` / source
`5ee367f2ed10c3ba4f034f6d708775238b2d9687`, with unchanged dependencies and
the default allocator. It adds named grant requests and owner assembly under the
host's existing ic-memory runtime; standalone removes its duplicate mapping.
No Canic dependency, static registration, physical placement, lifecycle export,
provider behavior or operational unfencing is supplied by this helper.

Four adjacent native tests pass in 0.03 seconds. Two host-selected ranges preserve
populated tenant/funding records through reopen into all-owner fences, alongside
an unchanged application memory. An incomplete composed snapshot returns the typed
missing-key failure without writing or creating stores. Unbootstrapped access leaves
backing bytes unchanged; default lookup leaves an absent runtime unconstructed,
including the upstream allocation diagnostic's typed NotBootstrapped result.
Native memory supplies no IC rollback or independent recovery guarantee.

Four existing focused PocketIC cases pass against the rebuilt standalone Wasm:
admission/manifest authority and restore (5.04 s), invalid installation/ingress
rollback (6.16 s), foreign/missing-state refusal (6.41 s), and repeated whole-host
lifecycle restoration (6.56 s). These use real management/lifecycle execution;
they do not establish managed Canic composition or deployed provider guarantees.
Strict core/standalone all-target/all-feature Clippy and warning-free rustdoc pass.

Captured SHA-256 values bind this local candidate, not a future build:

| Artifact | SHA-256 |
| --- | --- |
| `crates/ic-blob-storage/src/ops/service/stores/grants/mod.rs` | `ad956eba5e64429c3dc18223b20ed7bcf1da44b9462137866d6d474c50a9f2ab` |
| `crates/ic-blob-storage/src/ops/service/stores/grants/tests/mod.rs` | `3725f5c1c82872c291538191f89b9458ab3f7125a4f302ea1892f319e2f02dbb` |
| `canisters/standalone/src/ops/memory/mod.rs` | `034dc1ad567287dfa296bacc1a0b6f40e523ae0e4c58abdb234cac9c2b14f326` |
| `target/wasm32-unknown-unknown/release/ic_blob_storage_canister.wasm` | `8aa82dc181084298c7589de0e86daf8462b8d32aef3b70e587d491a56b279aaa` |

No paid/provider trial or new service qualification occurred. The maintainer's
0.4.6 release receipt was verified before this continuation; a redundant agent
full-gate run was interrupted after that release completed and is not a passed
gate for this changed source. Its log remains in `/tmp/ic-blob-storage-046-release-verify.log`.

## Shared immutable installation — 2026-09-30

This continuation has the same released base
`5ee367f2ed10c3ba4f034f6d708775238b2d9687`. The shared core now owns one bounded
immutable installation record plus the four service owners. Whole-candidate
validation runs before allocation. Fresh installation preflights all seventeen
grants; restore checks the host's actual service and compiled release, revalidates
project/verifier/resource inputs and opens every owner synchronously into fences.
The standalone private record/conversion path is removed. Public DTOs, exported
Candid, v1 record fields/codecs, physical placement and allocation policy are
unchanged. No compatibility reader, migration, dependency or allocator is added.

Five adjacent native cases pass in 0.04 seconds: invalid whole candidates;
allocation at any grant rejects before writes; missing configuration and wrong row
counts never initialize; wrong service/release, invalid project or missing owners
never repair; and populated tenant/funding records preserve exact full-width
configuration, project, verifier and all four fences. Native memory does not prove
transaction rollback, independent freshness or recovery authority.

Seven focused actual PocketIC cases pass against the rebuilt standalone artifact:

| Maintained behavior | IC execution |
| --- | --- |
| Admission/manifest authority and fenced restore | 5.21 s |
| Invalid installation and bounded-ingress rollback | 6.39 s |
| Foreign/missing installation memory refusal | 6.39 s |
| Stop/start and repeated upgrades preserve installation/all-owner fences | 6.47 s |
| Configured verifier authority, exposure refusal and restore fence | 5.00 s |
| Invalid project installation rollback and corrupt restore refusal | 7.72 s |
| UTF-8 project byte limit and retained configuration through restore | 3.06 s |

Total IC execution is 40.24 seconds. The unchanged Candid export test, strict
core/standalone all-target/all-feature Clippy and warning-free rustdoc also pass.
Test instances were dropped. No new full CI/release gate, live Caffeine call,
paid effect, active recovery or managed deployment was exercised.

Captured SHA-256 values describe this local candidate; earlier grant-only hashes
above remain their original capture:

| Artifact | SHA-256 |
| --- | --- |
| `crates/ic-blob-storage/src/ops/service/installation/mod.rs` | `6a22abbcefe9530a807f832de9ea8b3818994f4ca65b73a1b4b4678f92e05554` |
| `crates/ic-blob-storage/src/ops/service/installation/configuration/mod.rs` | `26f884f8d7096574121d2f84b7708e886b0be1cdc00ab60ce75007a7d1bad1ff` |
| `crates/ic-blob-storage/src/ops/service/installation/tests/mod.rs` | `0bce6260753c94f77ad0cb3ff91cf0edb3fb387c8638aa9673272292983223c4` |
| `crates/ic-blob-storage/src/model/service/installation/record/mod.rs` | `dba6a6b8c15dd88430ca4ed5ea92f78b481aa5b3ec39ab625e09800b8a2286b2` |
| `canisters/standalone/src/ops/mod.rs` | `b6d4a21ecad1eb1cf927143e5edb6665c686e30f2f296b80c32767851bc6d50d` |
| `canisters/standalone/src/ops/memory/mod.rs` | `642ed42e14e4a6cb4b79373b4b8ded178fa60b7cde4fc82d9ecd0c4ac517abe8` |
| `canisters/standalone/service.did` | `b112c53c777c43190bf35dd3f73c5913d790779ea3ebd271adfac4c04bf003b7` |
| `target/wasm32-unknown-unknown/release/ic_blob_storage_canister.wasm` | `43a38cd34f27c8e1efef38c157017d559878063ea4327503e8faf4b6edecc83b` |

Generic Canic API review separately used published local sources for `canic` and
`canic-core` 0.110.48. `canic::start!` supports synchronous lifecycle participants;
the activation facade supplies `require_active`, and Fleet admission has a caller
guard for the selected projection. These are source observations, not executed
managed composition. Reviewed source hashes are:

| Published source | SHA-256 |
| --- | --- |
| `canic-0.110.48/src/macros/start.rs` | `c254abc321e778c9e6d21b43f9176f4fb1e197e9cd41dd3e7971442846d9f6fd` |
| `canic-core-0.110.48/src/api/fleet_activation.rs` | `38d6394ef06319deb34bc4641e7ed371cf0af8566dfba99f3810e7384fc464c3` |
| `canic-0.110.48/src/fleet_admission/mod.rs` | `c07e101ccf10807f25cb76d584a50be5be33279f2fcc32fb4d729c1759aff7ef` |

A read-only upstream Git query confirms annotated v0.110.48 object
`0707017060fcc086074d46a21435fe9759af109d`, peeled to
`8d37c74c9a4457b9e2bd47ee883f98fd2889d63b`. It queried the v0.110.4* range only,
not all versions. Registry HTTP lookup returned 403 and browser retrievals failed
or returned stale indexed information; latest registry version remains unverified.
No Canic dependency was selected, sibling modified or blob removal inventory
refreshed. Configuration access during the argument-free synchronous participant,
managed endpoint guards and actual combined allocation/lifecycle evidence remain
the adapter's next implementation work.

## Managed Canic composition — 2026-09-30

Base is released 0.4.7 at `f5d050f2335de7e328fc56cedc3ac3325ab77abd`.
The unpublished composition library reuses the released shared installation and
seventeen grants. It declares nothing on linkage. Explicit macros contribute
memory requests and emit named caller guards in the owning artifact; Canic is
that artifact's direct dependency, not a core/library dependency. Synchronous
participants run after Canic framework bootstrap and before deferred application
work, publishing only complete owners. Restore retains all four blob fences.

Published Canic 0.110.48 is pinned. Its supported CLI built the managed fixture
through declaration/runtime validation and artifact finalization. Tools were
Canic CLI 0.110.48, ic-wasm 0.11.1 and wasm-opt 132. The main workspace retains
resolver 3; the unpublished fixture alone uses resolver 2 as required by the
host validator. Its ignored lock is seeded from the root lock and projected
offline. Every selected registry version/checksum remains in the root lock;
every previously locked registry version/checksum is unchanged. Wasm trees show
one ic-memory 0.14.3 / ic-stable-structures 0.7.2 pair and no testkit/PocketIC;
the core and composition library trees contain no Canic. No allocator changes.

The earlier generic source review above described `require_active` as a facade
guard. That function is internal to Canic-core. The executed guard instead uses
public `ComponentRuntimeApi::status` and `operation_status`, which correlate the
runtime and Fleet activation operation identity, and requires both phases Active.
The separately selected Fleet guard calls the public admission predicate.

One managed PocketIC journey passes in **13.53 seconds** IC execution:

- Complete shared installation is observable while Prepared and before deferred
  callbacks; both ingress and a real inter-canister call refuse without changing
  stable memory. Bounded IC progress later observes deferred callbacks.
- Actual platform caller binding reaches shared tenant handlers. Outsider,
  synthetic Root and actual IC controllers receive typed tenant denials without
  stable mutation; the explicit operator may enroll. Fleet admission accepts
  configured members and refuses an outsider separately.
- Replacement upgrade arguments trap and preserve the complete stable memory.
  Canic's same-release upgrade helper sends one Candid unit value; that exact form
  succeeds. Installed configuration, UTF-8 project, verifier, compiled fixture
  release, tenant enrollment and neighboring application bytes survive.
- All four service owners are synchronously fenced after upgrade. Tenant
  inspection remains passive; operator mutation receives the typed fenced failure.

Two adjacent native cases pass: linkage neither constructs nor registers a memory
runtime, and application-argument extraction preserves the exact payload while
rejecting missing, oversized, malformed, trailing or third-argument input.
The latter uses controlled carrier values, not a production managed installation
payload. Strict composition/harness and isolated fixture all-target/all-feature
Clippy, warning-free composition rustdoc, formatting and diff checks pass.
No full CI or release gate ran.

Failures remain separate captures. `/tmp/ic-blob-storage-canic-composition-01.log`
failed because the test assumed deferred work ran immediately upon activation
(SHA-256 `756f470c45b4cfeed4e64b7cf64022df2bb06916bdc3e1bec94af8990b9be1c3`).
Bounded IC progress corrected that expectation.
`/tmp/ic-blob-storage-canic-composition-02.log` exposed the mismatch between
zero Candid arguments and Canic's single unit upgrade argument
(`15ea357eed4aa61b97ad5ccaf04656d8034d467dc398c3eaf0bff36ae7d055ed`).
The adapter now uses only the maintained Canic unit form; no fallback was added.
The passing capture is `/tmp/ic-blob-storage-canic-composition-03.log`
(`418f570713a01545b8c419d0fa53998ae72f439feb17f20dd04be2795c645c69`).

Build attempts retain `/tmp/ic-blob-storage-canic-build.log` and numbered
`-02` through `-07.log`: resolver rejection, missing explicit feature arrays,
forbidden transitive Canic facade dependency, sandbox sccache startup failure,
initial successful build, unavailable CDK size-helper compilation failure and
final successful build respectively. The final capture is
`/tmp/ic-blob-storage-canic-build-07.log`
(`9d4bc2e7a13d221c40063df388a4a72126284871806e771c7575adfe1d0eaf8d`);
final build took 22.90 seconds. Cargo wrapper selection is now explicit rather
than relying on Canic's automatic sccache startup.

| Local source/artifact | SHA-256 |
| --- | --- |
| `crates/ic-blob-storage-canic/src/lib.rs` | `39bed5bbd2d5fc0c28a92fb26281b77f65936e0960c2a66b96cb2ae7be577131` |
| `crates/ic-blob-storage-canic/src/arguments/mod.rs` | `56f1fc6313a56ed68a81fc2a5f00dcf04ea8f96e85910fb09a93b3cef5996782` |
| `crates/ic-blob-storage-canic/src/lifecycle/mod.rs` | `c41c3e53dae1f5876900f119a25631a5183a979c4360bd4642e69a7518d9cc45` |
| `crates/ic-blob-storage-canic/src/memory/mod.rs` | `84774317bdd324291a46a4bc83b750a984d70457e67ce22b04f158abbc7fdbf4` |
| `canisters/test/canic_probe/src/lib.rs` | `1ea35df3cd4ce2fa8b603a6db528df06ec0e32340d402c374ada05e6c44e4808` |
| `canisters/test/canic_probe/canic.toml` | `63b4ee017ee33071f7f7d55b01e42b21cb77a0cdf670fb04ea93906c1e2ccc61` |
| `tests/pocketic/tests/canic_composition.rs` | `d404483a1700e5fd433f861dae0adb2b9d239ca0475a9916b65c815aca45a155` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.did` | `167e76637a7750415f0bc311439c7be6ff618f95e1cc790fb5b5a75d48dfe5cb` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.wasm` | `22648e35c82a0b1fdb8b37681df6b91d87430744ca8ed83649b30131339154c0` |

This is actual IC composition under controlled installation configuration and
Canic's public test helper's synthetic Root/Coordinator authority. Fixed release
identity `01` repeated 32 times is fixture authority only, not finalized production
Fleet release provenance. The fixture exports observation/tenant probes rather
than the complete production blob Candid API. Test instances were dropped.
Production managed input composition, endpoint parity, common consumer/operator
journeys, occupied/in-flight managed recovery and combined IcyDB application remain
open. No provider behavior was investigated, Caffeine request/paid effect performed,
sibling modified, deployment or release made. The provider probe ledger is unchanged.

## Managed installation input and release authority — 2026-09-30

Continuation of the same unreleased composition batch over 0.4.7 base
`f5d050f2335de7e328fc56cedc3ac3325ab77abd`. Existing core, standalone APIs,
memory grants/allocator and current v1 record schema are unchanged.

The managed adapter now accepts one explicit `ManagedInstallationInput` inside
Canic's application bytes: operator, payer, namespace, resource/billing/funding/
read limits, project and completion verifier. The participant checks the 256 KiB
carrier before its own platform copy, bounds application bytes to 16 KiB, decodes
one typed value with work/type limits and reuses shared whole-candidate validation
before opening blob grants. Canic's initial typed CDK decoding happens first;
these bounds do not claim to precede that framework buffer.

Explicit `declare_installation!` operations obtain correlated public Canic status
only after the lifecycle has authenticated its payload and checked the compiled
release binding. Actual service identity comes from the platform; neither service
nor release is selectable in application policy. The shared installation retains
Canic's validated release identity and checks it on restore. The former fixture
package-version binding and baked installation-hook candidate are removed.
No alternate reader, fallback input or migration exists.

The fixture now uses Canic's public Component Group helper with explicit application
bytes. Its operator-only transient carrier observation lets the test reuse the exact
upstream-generated envelope, rather than reconstructing Canic authority. Only a
fresh fixture with no tenant/provider obligations is successfully reinstalled to
observe Prepared behavior. Failed-reinstall evidence instead starts with an enrolled
tenant and never clears it. This carrier query is test machinery, not a production
configuration/recovery endpoint or durable journal.

Two actual IC journeys and one built-interface check pass in **30.13 seconds**
suite runtime. The maintained lifecycle journey covers Prepared ingress and actual
inter-canister denials, deferred work after owner publication, tenant/Fleet authority,
replacement-upgrade rollback and same-artifact restoration with all owners fenced.
The new journey rejects ten installation carriers: absent application input,
oversized application, malformed bytes, excessive carrier size, missing policy
fields, third carrier argument, zero object bound, anonymous verifier, invalid
project and protected release mismatch. Every rejection returns the typed IC
canister-error category and preserves the complete stable memory, retained
configuration/release/project/verifier/fences, neighboring bytes and tenant
enrollment. Host-side decoding of the captured actual carrier also verifies exact
policy preservation and rejects inner extra values/trailing data with typed errors.

Artifact review found that the preceding fixture placed Canic's finish expansion
before application endpoint registration. The prior adjacent Candid hashes record
only generic Canic methods; they do not evidence declaration of the called probes.
The maintained module expands last, with all needed signature types in scope.
The adjacent interface now includes every application method. Official native
candid_parser 0.4.1 parses and type-checks it, then checks the maintained names,
query/update modes and argument/result shapes. Actual typed IC calls separately
exercise those endpoints. This fixes declaration coverage without treating a
successful runtime call as interface evidence.

Two adjacent native library cases, strict affected library/harness and isolated
fixture Clippy, warning-free composition rustdoc, formatting and diff checks pass.
Wasm trees exclude candid_parser, testkit and PocketIC; core/composition trees
remain Canic-free. Safe copying reuses the already locked ic0 1.2.0. The parser
adds native-only packages, with no changes to previously locked registry versions/
checksums; every isolated fixture selection remains bound to the root lock.
No full CI/release gate, deployed Fleet, provider request, paid effect, sibling edit,
version mutation, commit or publication occurred. Local instances were dropped.

Retained passing attempts: `/tmp/ic-blob-storage-canic-input-01.log` (27.30 s,
initial eight refusal cases) and `-02.log` (28.09 s, populated state and ten cases)
precede the interface correction. Final `-03.log` includes the parsed interface
check, SHA-256 `af9a29ad6dbc1954b5eba3ad022bb029b38179b0737fd402cc11cd0afdb35e4c`.
Build attempts `/tmp/ic-blob-storage-canic-input-build-01.log` through `-04.log`
remain separate. The final supported Canic build passed in 17.63 s;
`-04.log` SHA-256 is `d769b936e6fa0e66e68a80d7b046a74e948a7590a1c2b77081b38cf7020d704d`.

| Current local source/artifact | SHA-256 |
| --- | --- |
| `crates/ic-blob-storage-canic/src/lib.rs` | `5d9535a135fc15be2874cc70ac734165dcea06987fe680f7d3504b1f1f42455e` |
| `crates/ic-blob-storage-canic/src/dto/mod.rs` | `8a23b52f48adff43adfca65dae8bf7a69734d67652d51380707b519519d53e2b` |
| `crates/ic-blob-storage-canic/src/arguments/mod.rs` | `bd6cb7209a36fb0fbd00bc6e5d5183b6e721a61d39378472c67ad4ec6ad658c6` |
| `crates/ic-blob-storage-canic/src/lifecycle/mod.rs` | `7b8d39e67410e69a62791bc616d2bd686470fb5618b34d67a87a7ac89179a478` |
| `canisters/test/canic_probe/src/lib.rs` | `effcf475dd71e8cc3950dd7d588e0c3f4612f3a385944837f1d4e879bb6a1d6c` |
| `canisters/test/canic_probe/src/managed/mod.rs` | `841d6ec9cab016f548fec3199b46e3d850e94b8bfa27cd8722e0ceb73f07c537` |
| `tests/pocketic/tests/canic_managed/mod.rs` | `2d0f171615a3e19bec2140c33c24dfa89502bb6cd24601553a6ba8beb80d37d5` |
| `tests/pocketic/tests/canic_managed/installation/mod.rs` | `2dc3e817a9fb40dc2a5b0b72aec42dc491c26baf6bbbe591dfeca17b342307e5` |
| `tests/pocketic/tests/canic_managed/declaration/mod.rs` | `b78854d5838f7c0702319108b690103c8831b9df793557851fa54cf3969abf21` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.did` | `d06633e182a72f5764e8c4cd534c8b6cfa7a95b2b61d89fbab15355f7cd8c54e` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.wasm` | `7fcb907530a25c649666949da80329586e6e802161a2039d5991c149fc973a42` |

Fixed compiled release `01` repeated 32 times and synthetic Root/Coordinator
remain fixture authority. The actual current carrier is evidenced, not every
consumer/topology sizing choice. Full managed blob Candid/API, production Fleet
artifact provenance, common consumer/operator journeys and occupied/in-flight
managed recovery remain open. This is not combined IcyDB or provider qualification.

## Managed blob endpoint subset — 2026-09-30

Local IC composition evidence after released 0.4.8, using published Canic 0.110.48,
its supported CLI and ic-testkit's PocketIC reexport. The core/allocator/registry
versions are unchanged. The controlled artifact supplies six managed blob methods
over shared configuration, admission, manifests and capacity workflows. The
official Candid checker proves their complete method types and modes equal to
standalone's maintained interface; standalone's native export check also passes.
Adapter-local passive error newtypes retain the existing Candid shape, while ops
maps generic Canic Fleet refusals to denial without a core Canic dependency.

The new actual IC journey denies configuration to tenant/uploader/outsider/Root/
anonymous/actual controllers, denies admission to unrelated actors, accepts the
tenant's exact reservation and permits only its named uploader to prepare. An
admitted/prepared ten-byte object with full-width IDs, original manifest, readback
configuration and capacity survives same-release upgrade. All four owners are
fenced; mutation refusal and inspection preserve stable memory. Valid Candid at
exactly 131,072 bytes reaches the shared manifest limit, whereas 131,073 bytes
receives typed ingress rejection and an actual inter-canister transport refusal.
The smaller Canic default cannot silently exclude maintained manifest requests.
The operator-only local forwarder is exclusively a fixture helper.

Final managed suite: three PocketIC journeys and one Candid comparison pass in
37.82 seconds (`/tmp/ic-blob-storage-managed-endpoints-06.log`, SHA-256
`b98fbc6ffb9206c567ce470bedecf85eca4d45b8aac278878c725166fa33356c`).
The existing standalone admission/manifest/stop-start/rejected-upgrade/fenced-
restore journey passes in 5.31 seconds
(`/tmp/ic-blob-storage-managed-endpoints-standalone-01.log`, SHA-256
`0ca561a6a9089aa0a41c3dbfad15f45057ead41909cae552c670c054dd5412a5`).
Six shared installation cases and the standalone native Candid check pass.
Strict affected core/standalone/harness and isolated fixture Clippy, warning-free
core/standalone rustdoc and formatting/diff checks pass. No full CI/release gate
ran for this continuation. Test instances were dropped; build artifacts retained.

Distinct failed attempts remain in `/tmp`: managed build `-build-01.log` exposed
the required Canic guard-error conversion; harness `-01.log` failed a private DTO
import, `-02.log` failed a misplaced assertion and `-03.log` exposed duplicate
endpoint symbols from linking both artifacts for comparison. `-04.log` passed
Candid then could not bind PocketIC inside the sandbox and was interrupted;
`-05.log` passed the initial complete suite. Harness Clippy `-clippy-02.log`
identified long journey functions, subsequently split by authority phase;
isolated `-clippy-fixture-01.log` required a narrow owned-Candid-input lint
expectation. Final supported build is `-build-03.log`; final strict captures are
`-clippy-04.log`, `-clippy-standalone-01.log` and `-clippy-fixture-02.log`.
These are local validation captures, not provider probes or production receipts.

Selected final SHA-256 hashes:

| Source/artifact | SHA-256 |
| --- | --- |
| `canisters/standalone/src/dto/mod.rs` | `684e9377576c6ef05beaf8603a2e03280436b50f5aceebd280d1aa236491c7b2` |
| `canisters/standalone/src/ops/mod.rs` | `88188bd076db7c566b3cc9a5bf578e3034bcceb305a7143e3f654a0565b38145` |
| `canisters/standalone/src/workflow/mod.rs` | `15ef957d637706f9421088bef3f705ff9af2a90e3471520a94fd5495c4233635` |
| `crates/ic-blob-storage/src/dto/configuration/mod.rs` | `197196fcce01794f6f71343ea63ff4e1faf7924856d4b52fa1be5385e8a0a4fc` |
| `crates/ic-blob-storage/src/policy/installation/mod.rs` | `9c5ff8107fa770c8dcca517225ceed13126d564405738cc73119fd8577f29619` |
| `crates/ic-blob-storage/src/workflow/installation/mod.rs` | `f62198aaafcef199670b59da8a9033ad78b32be9941d115e1642df4c176dd297` |
| `crates/ic-blob-storage/src/ops/service/installation/mod.rs` | `3498489fadde519f75a8941a0672327fe0b45d8136570e32031f6c2f3076e48f` |
| `crates/ic-blob-storage/src/ops/service/installation/tests/mod.rs` | `f453ce625efc9dbcad9edb5d30bf5e2d9696d66f26111ef996c434611c3eb3c1` |
| `canisters/test/canic_probe/src/api/mod.rs` | `4165975b93c6484432d920b75aac01d612a7e19bbff3f338551e9e4165390bce` |
| `canisters/test/canic_probe/src/dto/mod.rs` | `785ba05bdbfd8d4717bc9f5f6233e79ded11d3e38380e5421a444688d23bc604` |
| `canisters/test/canic_probe/src/ops/mod.rs` | `9d449af7232caf176f84b71cd5a1dc99aba956f97bc6c4543db05a108ba233be` |
| `canisters/test/canic_probe/src/lib.rs` | `4de0c336215a7e971dc1d4920aefb1ef4daedbb45a77c2dddf106f5ad65f50f3` |
| `canisters/test/canic_probe/src/managed/mod.rs` | `71face9dbb60bcd44621b9d50abeedecdc62c3fb05f84a76fffa92909890382b` |
| `tests/pocketic/tests/canic_managed/endpoints/mod.rs` | `1ff753c4b3f1e29086e8cb98789fd7ade7d787a199338af434d58e399d28ecf6` |
| `tests/pocketic/tests/canic_managed/declaration/mod.rs` | `f66de4c4564f51500e8be0951f7a8861611203b01c891d259e90b30fb287057c` |
| `tests/pocketic/tests/canic_managed/mod.rs` | `c4f08b5181ef883ae4be16822bdb53717b56ac7d96e8677325e7f6edbeb857f9` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.wasm` | `f19ed6adfbf264b50ee52b0b33d88eff8c44a2f8a2a3a43ed9e475bcb5e87bf9` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.did` | `18cd56efb73a9105bb16c981c372d12e9baf355152c436d11b6b54e64cd7589c` |
| `target/wasm32-unknown-unknown/release/ic_blob_storage_canister.wasm` | `e93fe31cde818924ce62105c764b1e13208392d785b809cbbbc190671f340c7a` |

This is an endpoint subset with controlled installation inputs and fixed fixture
release/synthetic Root authority. Remaining managed endpoints, equivalent query-
byte/decoding-work bounds, production Fleet provenance, common consumer/operator
acceptance, in-flight provider recovery and combined IcyDB composition remain open.
No Caffeine behavior was investigated; no live provider effect or probe was sent.

## Managed cleanup, history and passive operator inspection — 2026-09-30

Continuation of the same unreleased batch after 0.4.8, using published Canic
0.110.48 and its supported CLI. The controlled artifact now exposes nineteen
blob methods over shared configuration, tenant, upload/history/discovery, manifest,
reference/capacity and local operator/funding workflows. Official candid_parser
checks all nineteen complete method types and modes against maintained standalone
Candid. One passive generic error wrapper keeps Canic conversion in adapter ops;
fixture-only tenant methods were replaced throughout by the blob tenant API.

The new actual PocketIC journey prepares a ten-byte full-width-ID reservation,
suspends its tenant and cancels the unexposed upload. Reserved/active counters
reach zero while the operation and cancellation history remain. Tenant history,
discovery and unconfirmed reference observations remain available after suspension
and same-release restore. Uploader/operator/outsider/Root cannot revoke or
inspect/mutate tenant references or private history. Local operator status, empty
funding history and absent funding outcome are scoped and passive; tenant/outsider/
Root refusals and wrong namespace preserve stable bytes. After upgrade every owner
is fenced, history remains, and reference/revocation mutations refuse unchanged.
This does not exercise confirmed references or populated/in-flight funding/read
owners and does not establish provider deletion, credit or billing cessation.

The managed suite passes in **47.34 s**, including four IC journeys and the exact
Candid comparison. Expanded revocation/reference denial checks then pass in the
focused cleanup case in **13.63 s**. Logs are
`/tmp/ic-blob-storage-managed-history-01.log` (SHA-256
`55f58b7243a4f08464e3ea2f49f78c707fbe7bc83e7482ce3495aee3314a8304`)
and `-02.log` (`a7e6081f2d260cd69d3c648d9a3e28445f30ded1806ee9ab698b454c48450849`).
Commands use this repository's target, offline/locked Cargo, ic-testkit's
PocketIC 16.0.0 reexport/binary, the supported built fixture and one test thread.
The final focused run changes only the cleanup test, not fixture source/artifacts.
Test instances were dropped; build artifacts retained.

Supported build `-build-01.log` passes in 21.42 s (SHA-256
`978a29c820ae4963a4533ce786c3a62211171f619f503ec2a627390c4acefb2b`).
Strict harness Clippy `-clippy-03.log` and isolated fixture all-target/all-feature
Clippy `-clippy-fixture-01.log` pass. Retained failed `-clippy-01.log` rejected a
long journey function; meaningful restore-phase extraction fixed it, and
`-clippy-02.log` passed before expanded denial checks. Both formatting checks,
diff and capability JSON checks pass. Core/composition dependency trees remain
Canic-free; fixture Wasm excludes parser/testkit/PocketIC. Prior shared/core and
standalone checks above remain applicable; those sources did not change here.
No full CI/release gate, version mutation, commit, publication, deployment,
sibling edit or provider probe/effect occurred.

Read-only source review found no supported Canic decoder hook: query payload
limits are rejected, update raw decoding exposes only its 10,000 skipping quota,
and framework preflight is internal. Reviewed registry sources are
`canic-macros-0.110.48/src/endpoint/{parse,expand,validate}/mod.rs`, with respective
SHA-256 `e591b5749791cdd6bde85f46f0016df6ae333abe6d50c667d82cb8aac5bc04f3`,
`852b10c671b59ce3fedb1a29367fbfcffb5ae40404da4a34fc7af69ec5c91cd1`,
`6a34e2b942caf2603fcb83070ea9f1e7c8910f1366a3f870e1abaa1b798f5750`.
The local read-only Canic checkout reports 0.110.48 at
`e9ea2a2698135f583d56fb322a815af1480a3881`; this is not a removal-inventory refresh
or latest-registry claim. Required upstream controls are in the service contract.
No duplicated dispatch or unbounded-then-bounded re-decoding was introduced.

Selected final source/artifact SHA-256 hashes:

| Source/artifact | SHA-256 |
| --- | --- |
| `canisters/test/canic_probe/src/api/mod.rs` | `78310d4ea6008a5a2ad1aee6ad698ae65034e0a0d4cc89f14f202570e749e5a7` |
| `canisters/test/canic_probe/src/dto/mod.rs` | `220ef440a25adae5fae4b905044c7f7d69a0a6cc66837f42b2bd35a129876b1d` |
| `canisters/test/canic_probe/src/ops/mod.rs` | `9fdf7a6bfd6433f3ca3004a8448198927a1814e6ac5f3000dc7ce82544b4cfb1` |
| `canisters/test/canic_probe/src/lib.rs` | `411b0b19ffec7c9d01de52f757077a8f5245c8420c720a3ec0da2d3894a15352` |
| `canisters/test/canic_probe/src/managed/mod.rs` | `1f85bd20f24a05b55418fb237fc8a3483da9eac731a171aacb4a932ce387eab4` |
| `tests/pocketic/tests/canic_managed/cleanup/mod.rs` | `89e79689b2608ef81097da3bc06e983b5f10186befd1d030957c4dbd9103cdce` |
| `tests/pocketic/tests/canic_managed/declaration/mod.rs` | `c39e9ff54f914c98afd429623720209310366bf0fe8d10da33a2302e134e5ce4` |
| `tests/pocketic/tests/canic_managed/mod.rs` | `65b1e1c32bb42628d3d814f8176c86b0634386910f81b67fb1a731c947e50d12` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.wasm` | `f2560baf088a7d451ce6422f3299645a776b8b6700b750485a00efdc337830c8` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.did` | `8928139331f422295daa295573a70cacb6003265b4cdc362be5114ed5cd37fe4` |

Full managed endpoints, equivalent query-byte/decoding-work bounds, production
Fleet provenance, common consumer/operator acceptance, occupied provider recovery
and combined IcyDB composition remain open. Synthetic Root/Coordinator and fixed
fixture release still qualify only this local subset. The Caffeine ledger remains
unchanged because this batch investigates generic Canic composition only.

## Managed verification, attestation and reference-qualified delivery — 2026-09-30

Unreleased continuation after maintainer release 0.4.9, source base
`5f35bf6971fb0fefee72b9d62a71c4b7baae02ad`. Published Canic 0.110.48 and its
supported CLI build the controlled artifact. Five managed methods now call shared
completion/download workflows: `blob_verification_manifest`,
`blob_verification_plan`, `blob_attest_upload`, `blob_upload_attestation` and
`blob_download_descriptor`. Official candid_parser compares all twenty-four
implemented service methods' complete types/modes against maintained standalone
Candid. Attestation/download updates select Canic's 4 KiB payload bound; the
existing manifest transport-bound evidence still passes. Query/work limits remain
the separately recorded unsupported-framework gate.

Two new actual PocketIC journeys retain full-width independent identities and
installed service/project/verifier bindings. Prepared uploads refuse attestation
and delivery without advancing state. Only the installed verifier obtains plans
or attests; tenant/uploader may inspect historical receipts. Invalid observation
times and changed replay digests refuse. Exact accepted replay returns the same
receipt, including after both references are released, without resurrecting them.
Descriptors require a live tenant reference and retain original metadata. A second
retain exhausts fresh-retain headroom while reserved release capacity remains.
Suspension blocks delivery but permits release. Last release leaves reserved and
logical bytes zero, physical bytes ten and liability bytes ten, with deletion
pending. Same-release upgrade preserves original manifests, attestation/release
receipts, cleanup history and those liabilities; all four owners are fenced and
rejected mutations leave full stable memory unchanged.

**Substitutes:** the ten-byte body is local fixture data. An operator-only
`probe_expose_upload` hook checks the installed service/namespace and exact retained
permission, then selects its uploader for the local exposure primitive. This is
test setup, not certificate issuance, a provider response, recovery authority or
qualification evidence. No Caffeine bytes are fetched and no provider call is
made. The test hook must not be copied into deployment endpoints. Synthetic
Root/Coordinator and fixed compiled release remain controlled fixture authority.

Commands use repository `target/`, offline/locked Cargo and ic-testkit's PocketIC
16.0.0 reexport/binary. Build: `make build-canic-probe`. Runtime:
`cargo test -p ic-blob-storage-pocketic-tests --test canic_composition -- --test-threads=1`
with `POCKET_IC_BIN=$PWD/.tmp/tools/pocket-ic-16.0.0/pocket-ic` and
`BLOB_CANIC_PROBE_WASM=$PWD/.tmp/canic-probe/.icp/local/canisters/storage/storage.wasm`.
All seven cases pass in **89.57 s**; supported build passes in **36.00 s**.
Strict focused harness and isolated fixture all-target/all-feature Clippy pass,
as do root/fixture formatting, diff, capability JSON and read-only changelog
validation. Dependency trees retain Canic-free core/composition libraries and no
parser/testkit/PocketIC dependencies in fixture Wasm. Test instances were dropped;
build artifacts retained. Core/standalone source and contracts are unchanged.

Retained log prefix is `/tmp/ic-blob-storage-managed-completion`. Every failed
attempt remains distinct; macro export/lint corrections did not alter domain
authority or wire contracts.

| Log suffix | Result | SHA-256 |
| --- | --- | --- |
| `-01.log` | Managed suite passes | `bcca6a95debc6a1dd493809bffd96c50551e44831078d8f55c97a0fe9ebe49a0` |
| `-build-01.log` | Failed: export could not resolve qualified failure type | `6a4f87f6a3a42cdb4906a62045730f6a45168aae28c946359803f6cc9e61d559` |
| `-build-02.log` | Supported build passes after explicit root type import | `ecbe6d3747aaa16a5e8940a31326c7a487fbe8471db65d8005e956789351e343` |
| `-clippy-01.log` | Failed: test assumed manifest response had a fence field | `0ed3e93f32a21321e4c939adcaed77641c0d6354c73d3e62b9d7bb2d60f44684` |
| `-clippy-02.log` | Failed: native helper self/owned large arguments | `20e58af5fae10ea361f0b645f57c793ae2cdb270a565488dc467c24fa5e28611` |
| `-clippy-03.log` | Harness passes before final export import | `0c5d42e9aaed28c1cae1a3164a365ea1b34a8def3b664d7fca0b4d27296708e3` |
| `-clippy-04.log` | Final focused harness passes | `3076c329ff17fb4dd699c511374543ef364c1a78bd1bca9ed2a50b4866d39cb1` |
| `-clippy-fixture-01.log` | Failed: ignored isolated lock still pre-release | `aadd21c4a5e6c3073fb39a32f3b359927a77ef02362a46a82c497cd764d5406e` |
| `-clippy-fixture-02.log` | Failed: Candid owns a large statement argument | `ef743341eff99e1084d0d0ed13837b4b808efe6b53d9fced2f2469494ac7e867` |
| `-clippy-fixture-03.log` | Failed: macro cloned function-level lint expectation | `48034befeb4ce959a5fa8aa973d3b006510a49279a8b73938188ae80a271c278` |
| `-clippy-fixture-04.log` | Fixture passes with module-level expectation | `360fd5a576cb90c09fa5831d6c360a66a24fbe007fc43edfd00302779bc0106a` |
| `-clippy-fixture-05.log` | Final isolated fixture passes | `8d9e668089171bde0ab6d47fdfe3e54ee4b6b1488cc89a734a18e891a2229a93` |

Final source/artifact SHA-256 hashes:

| Source/artifact | SHA-256 |
| --- | --- |
| `canisters/test/canic_probe/src/api/mod.rs` | `9e5a8311989310a251bf10e21d2541c69ea2ef4681c965f62a6f4776bd9fb240` |
| `canisters/test/canic_probe/src/dto/mod.rs` | `e54646a12c6e53870609b7f4585b9eb3aa9b4ebe55357cb39206f5d6bcd34efe` |
| `canisters/test/canic_probe/src/lib.rs` | `56593766373ef4323dc96de927d38eb45d8d1069b2004040ee95953f86678850` |
| `canisters/test/canic_probe/src/managed/mod.rs` | `fef9ed4b7b51b2a12d13d93b851c168365c5bc4e9b530127dbcc23afb333a3f6` |
| `canisters/test/canic_probe/src/ops/mod.rs` | `73c753ad9414d46b3a5243deab330bfc138ec3124ab2c48c12d4457821a3872c` |
| `canisters/test/canic_probe/src/ops/fixture/mod.rs` | `659a0641f8c412eb60b35093fd96c69602f17f30d6195185cd0b92c7dc3367c7` |
| `tests/pocketic/tests/canic_managed/completion/mod.rs` | `0be7f25f26a1eb366f2bb0838beb49cf83f12546ca9c62cb8d52e5f408a955f9` |
| `tests/pocketic/tests/canic_managed/declaration/mod.rs` | `6760be34f03a9eb8786ebecc834bdaf53911818c81e4206eab11cd54c2b8f75e` |
| `tests/pocketic/tests/canic_managed/mod.rs` | `4a115674da9cde0cd61fcc0dc31cc744e4bbdfcbc8b7abc02a3a01e99a11ff89` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.wasm` | `4679f9f4a891d589bb82dcdd6460d07e882fd8d55109d5f8af73c742f3a291e0` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.did` | `cf7234dc72ccfb2f57cb1f3df3e89853175979f79640c93b52f24feecf9fcae3` |

Remaining six methods cover certificate assessment/issuance, gateway
sync/cancellation/revocation and account inspection. Production Fleet provenance,
supported decoder controls, common consumer/operator journeys, live provider
completion/economics/deletion, occupied recovery and combined IcyDB composition
remain open. No full CI/release gate, version mutation, commit, tag, push,
publication, deployment or sibling edit occurred. No provider behavior was
investigated; the Caffeine probe ledger is unchanged.

## Managed certificate assessment and operator queries — 2026-09-30

Unreleased continuation after maintainer release 0.4.10, clean source base
`429db4ab516b7da2745cd242e3ac175a5cf13281`. Five managed adapters call existing
shared workflows: certificate assessment, gateway sync/cancellation/revocation
and account inspection. Official candid_parser compares all twenty-nine supported
service methods' complete types/modes to maintained standalone Candid. Four new
updates select 4 KiB bounds. The supported Canic 0.110.48 build passes in
**43.34 s**; the existing query-only source builds in **18.15 s**.

The certificate-assessment journey prepares a full-width-ID ten-byte reservation.
Only its uploader can inspect; tenant/operator/verifier/outsider/Root/controller
calls refuse. Unprepared/revoked/fenced state, malformed and unknown roots retain
their typed failures. Assessment reports PrechargeLimits, ProviderNamespace,
ReplayCharging and Recovery, with the exact original permission and host time;
no exposure, successful provider reply or qualification fact is invented. Stable
bytes remain unchanged during successful observations and refusals.

The operator journey allocates one local source and re-provisions only its fresh
empty managed installation through the original authenticated carrier, changing
the explicit Cashier principal before any tenant/object/obligation exists. It then
enrolls a tenant and prepares ten reserved bytes. Operator/service/namespace/
Cashier/payer checks precede calls. Valid Candid at exactly 4 KiB reaches scope
refusal on the async sync endpoint; 4 KiB plus one rejects before dispatch.
Successful sync, present/absent revocation, malformed/oversized reply retention,
Busy refusal, stale cancellation and exact cancellation preserve unrelated upload,
funding and read owners. Passive account queries report full-width independent
balances, no relationship, AccountNotFound, malformed and oversized failures
without local credit/allocation/readiness changes. A rejected sync remains pending
through same-release upgrade: membership and allocated sequence survive, all four
owners fence, and sync/cancel/revoke/account calls refuse unchanged.

**Evidence class and budget:** actual managed IC behavior over the existing local
query-only Cashier substitute; gateway/response bytes are controlled fixture data.
Five gateway-list and five account queries execute, with zero attached cycles,
30-second bounded waits, 64 KiB gateway and 4 KiB account reply ceilings plus fixed
decoder work/type budgets. The source checks actual service/replicated execution
and zero attachments; it cannot establish deployed Caffeine semantics or fees.
Successful gateway/account queries are passive at the source; source configuration
updates are explicit fixture setup. No payment, provider object or deployed request
occurs. The ledger records bounded intent and outcome before/after the run.

The canonical `_immutableObjectStorageCreateCertificate` adapter remains unwired.
The attempted plain-record `canic_update(public, ...)` fails compilation. Pinned
macro `build_access_plan` injects DefaultFleet even for public endpoints;
`expand` requires a direct Result for every nonempty access plan. Caffeine's
maintained plain-record success/IC rejection contract cannot be changed to Result,
and `internal` would omit the normal application Fleet guard. The attempted method
was completely removed; no alternative contract/bypass remains. Supported plain-
reply rejection is a framework gate, separate from decoder and provider readiness.
Initial root formatting also refused the not-yet-created operator module; the
module was completed before subsequent successful formatting/compilation. No
upstream edit/request or copied framework dispatch was introduced.

All nine managed cases pass in **107.47 s**, including the prior lifecycle,
rollback, cleanup and verifier/delivery cases. Commands use repository `target/`,
offline/locked Cargo, ic-testkit's PocketIC 16.0.0 and one test thread. Build commands
are `make build-canic-probe` and
`cargo build --release --target wasm32-unknown-unknown -p blob-gateway-source --lib`.
The runtime command is
`cargo test -p ic-blob-storage-pocketic-tests --test canic_composition -- --test-threads=1`,
with explicit `POCKET_IC_BIN`, `BLOB_CANIC_PROBE_WASM` and
`BLOB_GATEWAY_SOURCE_WASM` paths to the local artifacts below. Final strict focused
harness and isolated fixture all-target/all-feature Clippy, both formatting checks,
diff, capability JSON and read-only changelog checks pass. No core/standalone
contract/schema, dependency version or allocator changed. Instances were dropped;
build artifacts retained. No full CI/release gate, version mutation, agent commit,
tag, push, publication, deployment or sibling edit occurred.

Retained log prefix: `/tmp/ic-blob-storage-managed-operator`.

| Log suffix | Result | SHA-256 |
| --- | --- | --- |
| `-01.log` | Managed suite passes | `8d4a0b6678ca7b21b06eea1ef7d142b78ee1012640fb0c2fcac643da646b44af` |
| `-build-01.log` | Supported managed build passes | `8eec0cda324d5f9068922cc97163c9dd2304a036e089602b354b94588e7ba4da` |
| `-source-build-01.log` | Existing source build passes | `40f31c73977c43ee98c19fc9d5d0b4d834c341b4ea0e5ed215b4c22994fb63af` |
| `-prepare-01.log` | Isolated lock projection passes | `d225a034651838c96019d1a9a93c240beb450d96ca99f9c08fc1063dbf222b17` |
| `-fixture-clippy-01.log` | Failed: public plain-record endpoint unsupported | `671bce41745443f21dee38a5cbcd954cba9acf655f03fdb175d467be99b19f1f` |
| `-clippy-01.log` | Same unsupported endpoint fails harness compilation | `84090e94f1aa49173f855f5f21b90d070f67a832a4c1dcf9decf5bf9f42ac8a4` |
| `-clippy-02.log` | Harness passes after removing attempted issuer | `a5cd4446343f85e8271024ac4000bd26fbdc4a79394e56dcf8ffa6a881ad3855` |
| `-fixture-clippy-02.log` | Final isolated fixture passes | `0e84c73aec9e02d293a993f1b0bc78790d416f0084af45a7ccdc7c54cda19307` |
| `-clippy-03.log` | Final harness passes including 4 KiB boundary | `2c2d60d318cbf2e0057eac03f75fafde4b6bf8e63b0e71be6e0a66a93bef61cd` |

Final source/artifact SHA-256 hashes:

| Source/artifact | SHA-256 |
| --- | --- |
| `canisters/test/canic_probe/src/api/mod.rs` | `80050815490b4d9d2b4b6cd239f1f80c670d32d39ec4a8f9e65f9b400bccecfb` |
| `canisters/test/canic_probe/src/ops/mod.rs` | `acd09bf188f5e26e2d9870c1437a183f174c7bdbfd8ffc6a73a7aa2fa7ea8d7a` |
| `canisters/test/canic_probe/src/ops/account/mod.rs` | `468ab73ebdc6fc1f580a404fee5c87c9a72090f64d20be1deae4ce43acd22453` |
| `canisters/test/canic_probe/src/ops/certificate/mod.rs` | `6a62295ac18b68a6f17e1d623e7655ea206132748af8f50160f54c6003a0bad7` |
| `canisters/test/canic_probe/src/ops/gateways/mod.rs` | `d8cee1da6c2013811d24f89eb48ed3e7d015ef5d99d41133069983bc9c2bbbb3` |
| `tests/pocketic/tests/canic_managed/certificate/mod.rs` | `cd81ce77147c55ad0044f64a01dd6d1e76adbcb385dcb99628253f93cf495dba` |
| `tests/pocketic/tests/canic_managed/operator/mod.rs` | `6c2e6b255e75133666af92830ba0aa6a11127942934ae819a3a3dd58da8b5d47` |
| `tests/pocketic/tests/canic_managed/declaration/mod.rs` | `8d1adc16795b046b7b5f2f46a2763b9805afa490ae10672925382df250643c91` |
| `tests/pocketic/tests/canic_managed/mod.rs` | `620f60fdeb26ae673f5cf1256f369ba47f424610cd262e1df625236c8b846c3b` |
| `Makefile` | `e6365e04ba39e34bd2c0b4298ac3dfd6fc76875a074cedf03d939a2c80f303e9` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.wasm` | `61138d7e982b98c93ab12fba5b38fb1244f88d15323f8d1ec8bc320bf3e3e0dc` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.did` | `4c0c27182e49ff9385b0f3adf67858116a79c803e6f4ae91051da7f2d2e43622` |
| `target/wasm32-unknown-unknown/release/blob_gateway_source.wasm` | `f01fdb8e3cf9867cccd9689c222a8f2976ecc5b07fe7f8623c4bf5eb0ec94325` |

Pinned macro source hashes: `canic-macros-0.110.48/src/endpoint/expand/access.rs`
is `40df7ef009a5baa79bf12749edc803f03fb3f121d5aab94257f67c00025b7406`;
`expand/mod.rs` is
`852b10c671b59ce3fedb1a29367fbfcffb5ae40404da4a34fc7af69ec5c91cd1`.
These identify the reviewed registry source, not a latest-version assertion.
Common native operator/consumer journeys, supported framework rejection/decoding,
production Fleet provenance, live provider qualification, operational recovery,
combined IcyDB composition and removal/retirement acceptance remain open.

## Managed signed client and local byte verification — 2026-09-30

Evidence class: local managed PocketIC and actual native signed subprocesses.
This continuation retains the preceding unreleased twenty-nine-method artifact;
it changes test/tooling composition, not production endpoints, stable schemas,
registry versions or allocator. The public Canic qualification helper remains the
installation owner and now takes explicit fixture input with canonical Fleet
principal ordering. Shared native subprocess helpers no longer require the
standalone server harness; failed-command diagnostics include the command name.

The controlled signer is explicitly installed as operator and uploader. The test
pins the application-only instance's undelegated subnet key through the owned
PocketIC control API before starting its literal-loopback gateway using a supported
non-owning handle. CLI origin restrictions, proxy bypass and query signatures stay
enabled; no root is fetched by the client and no framework bootstrap is recreated.
This local trust arrangement does not test mainnet NNS delegation or production
Fleet provenance. Exploratory reads of official current PocketIC server source
are unversioned source review, not claims about the installed binary; executed
evidence is bound to the pinned local server and artifacts below.

The maintained `status`, `funding-history`, `upload-history`,
`certificate-assessment` and `verify-upload` commands run against one ten-byte
reservation. Incorrect declared identity refuses before transport; another valid
signer reaches service authorization and is denied. Incorrect installed scope and
tampered root refuse. The original permission preserves full-width IDs/deadline;
changed deadline refuses. Assessment reports the four real blockers without
exposure. Local ten-byte content verifies against the authenticated original
manifest; corrupt, short and long files refuse. Same-release restore retains
inventory and permits historical byte inspection while assessment refuses the
permission fence. All four owners report fenced, and each complete query phase
preserves full stable memory. Provider completion/availability and retry authority
remain expressly absent. There is no native update or provider query in this new
case; fixture setup mutations are local metadata, not provider effects.

Budget/outcome: intent is in the probe ledger. The new journey invokes the CLI
22 times, including pre-transport refusals, below its 32-invocation ceiling;
each uses the maintained 30-second client/256 KiB response limits. Control key
lookup is one local read with a 30-second timeout. Zero provider requests/payments
and zero attached provider cycles. Stop/drop owned progress, gateway, instances
and temporary files; build artifacts are retained. No full CI/release gate,
version mutation, commit, tag, push, publication, deployment or sibling edit.

All commands use this repository's `CARGO_TARGET_DIR`, offline locked Cargo and
explicit `POCKET_IC_BIN`, `BLOB_CANIC_PROBE_WASM`, `BLOB_GATEWAY_SOURCE_WASM`,
`BLOB_STANDALONE_WASM` and `BLOB_CLI_BIN` where required. Test filters below are
complete executed arguments, not a full validation claim.

| Capture under `/tmp/ic-blob-storage-managed-cli` | Command/result | SHA-256 |
| --- | --- | --- |
| `-clippy-01.log` | Managed/standalone harness strict Clippy failed on long test and input ownership; also records a build-directory lock wait after an idle precheck, with no source mutation during validation | `42ce2a008c3c962cca206f8be6a87ffd4509cb71e3f1dda9fadd3e3107f414a8` |
| `-clippy-02.log` | Same strict targeted Clippy passes after helper extraction and borrowed input | `82f5bcef4dffc9a7a797876e598dc419279f297a7c6422bc37ae078a7bcf6b69` |
| `-build-01.log` | `cargo build --offline --locked -p ic-blob-storage-cli --bin blob-storage` passes in 10.77 s | `adea0b0f705977ebcb4de38bbc12da11b6eaa9efa153618047036f00087c5699` |
| `-01.log` | `cargo test … --test canic_composition managed_signed_cli -- --test-threads=1` fails before CLI invocation: noncanonical Fleet principal order | `8311446fcc7c6047cbc3b5e88875af8db05f0669277e30f45fd12fe0fa9886ea` |
| `-02.log` | Same filter fails: instance API path is outside maintained CLI origin contract | `0ae2a74e1246bf3618997f5c898b1b65c4f9b66888ffeca4573051db53b1cd96` |
| `-03.log` | Same filter fails: `PocketIc::url()` returns localhost despite gateway domain; CLI requires literal loopback | `d6e59b8d67676ec55d5bcb5959a1631b9a6cfc7e35e7806c8b6ec751a26d40d1` |
| `-04.log` | Same filter passes in 9.71 s using constructor's returned URL; this precedes added byte checks | `aeefc50e33c78b753a821f5abb2862826e83919d3a7daece8f00dba209865702` |
| `-clippy-03.log` | `cargo clippy --offline --locked -p ic-blob-storage-pocketic-tests --test canic_composition --test standalone --all-features -- -D warnings` passes with final byte checks | `00ddff306936204a3464159a9b89dd48961484111313cf632e3165524a9cebf8` |
| `-05.log` | `cargo test --offline --locked -p ic-blob-storage-pocketic-tests --test canic_composition -- --test-threads=1`: all ten cases pass in 114.87 s | `128720009be32c67e76a0457f53a7bdb85d9eb9dec350f1ed92d0a43153cecb3` |
| `-standalone-01.log` | `cargo test --offline --locked -p ic-blob-storage-pocketic-tests --test standalone standalone_authenticated_cli -- --test-threads=1` passes in 6.24 s | `e592de3c99696db2785edd6ab9f04261ced88e5169998e4ca36708659c2a3ccc` |

| Final source/artifact | SHA-256 |
| --- | --- |
| `tests/pocketic/tests/canic_managed/cli/mod.rs` | `ba39e7e202964433b107c3ff5861e9a4c2e890df1fa92c68c54c59493d81a06b` |
| `tests/pocketic/tests/canic_managed/mod.rs` | `09304d49c666657c5f2a4d69a90a7a6c86edd85d95f2aa8b87ca86043228ec81` |
| `tests/pocketic/tests/authenticated_cli/mod.rs` | `14f7afa4b6ea9586e6539317c47cc9a44642f776c4c5604a720c685462680909` |
| `tests/pocketic/tests/canic_composition.rs` | `82c24ab203ab7d31d3cd18a2b376cc70b6221291dd30d7249876cfcc40143a6f` |
| `Makefile` | `552cdb9330050a8ff794741689d77de7c818e5110a4d9e1c6fc190c9a6bfaccb` |
| `target/debug/blob-storage` | `fb53ccb6b04df6ed5fab35feb483e2f67091da572bbebce84cc975815831d18e` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.wasm` | `61138d7e982b98c93ab12fba5b38fb1244f88d15323f8d1ec8bc320bf3e3e0dc` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.did` | `4c0c27182e49ff9385b0f3adf67858116a79c803e6f4ae91051da7f2d2e43622` |
| `target/wasm32-unknown-unknown/release/ic_blob_storage_canister.wasm` | `426aec9fa34c613b7609e735b6fa34348894d13ea3b63d9ddf49a6bc6fbbe4dd` |
| `.tmp/tools/pocket-ic-16.0.0/pocket-ic` | `69e324bdb68d32d878b7a9504b1379f08f8d1921272bacb065b0fabb3d0f3792` |

Exploratory official `dfinity/ic` master `rs/pocket_ic_server/src/lib.rs` and
`pocket_ic.rs` copies are retained at `/tmp/ic-blob-storage-pocketic-server-review.rs`
(`d3c7f181724ec370e84cd3b7520757869fc15a28fac6cb49a313a0258ca80fbe`)
and `/tmp/ic-blob-storage-pocketic-topology-review.rs`
(`167677b138e58021b36520e893fd35a28484dd7849b82190e1472182060780f0`).
The initial web fetch of the latter failed with cache miss; direct official-source
reads then succeeded. These mutable upstream captures are not a refreshed provider
baseline. Pinned PocketIC Rust client `root_key`, `new_from_existing_instance`,
drop and gateway URL behavior was reviewed read-only in its registry source.

Next evidence is the trusted verifier's retained observation and one-shot signed
submission through the managed adapter. Complete consumer composition, provider
qualification, supported framework plain-reply rejection/decoding, operational
recovery, IcyDB composition and source-removal/retirement acceptance remain open.

Final root/isolated-fixture formatting, diff, capability JSON and read-only
`release-data.pl changelog-check 0.4.11 2026-09-30` pass. All 302 local links/
anchors in maintained edited Markdown, balanced fences and the twenty current
source/artifact/log table hashes validate. Exploratory source captures retain
their separately recorded hashes; historical evidence hashes were not rotated.

## Managed signed verifier observation submission and recovery — 2026-09-30

Evidence class: local managed PocketIC with actual signed native commands and
owned HTTP source/fault transport. Exposure and ten-byte HTTP content are labelled
substitutes, not a Caffeine certificate/object. The fixed test signer is installed
solely as completion verifier; operator, tenant and uploader are distinct. The
public Canic helper, sole runtime/storage owner and undelegated local subnet trust
arrangement remain unchanged. No production code/API/schema, registry graph or
allocator changes. The former fixture-local proxy is removed; existing native
and managed cases use one maintained HTTP fault proxy/source.

Maintained `observe-upload` saves plan/permission/service response and GET intent
before the source receives a request. The source checks the exact owner/project/
full-width namespace/root target, no Authorization header and pre-effect records.
Verified bytes yield the exact original permission and content digest in a saved
statement; observation preserves full service stable memory. Unexposed and foreign
verifier queries refuse before GET. Changed observation project refuses before
creating a dispatch claim, with tampered input/refusal retained separately.

Maintained `submit-attestation` persists its signed request/statement/intent before
one real replica update. The shared proxy checks the exact signed sender, service,
method, argument, expiry and stored bytes before forwarding. Three modes pass the
actual acknowledgment, drop it after replica execution, or replace it with pending
HTTP admission. Their saved outcomes remain accepted, uncertain or pending.
Repeated submission refuses the existing claim and preserves original intent.
Signed `upload-attestation` resolves the exact historical receipt without resend.
The live first reference permits a tenant descriptor after acceptance. Same-release
upgrade fences operational delivery/new observation while preserving signed receipt
inspection and complete stable bytes during those reads/refusals. Historical
metadata establishes neither current availability, future retention nor billing
cessation. No native funding/provider mutation or deployed Caffeine effect.

Budget: per managed verifier journey, one local GET/ten content bytes and one signed
IC metadata update, with at most sixteen CLI invocations and maintained 30-second/
256 KiB client bounds. The executed journey uses eleven invocations. Each fault mode
asserts exactly one forwarded update and no further source request. Provider cycles
attached/payments/deployed provider requests are zero. Local control key lookup
uses a 30-second limit. Setup exposure is a labelled fixture-only metadata cut.
The existing native regression cases additionally retain their pre-existing partial
GET/process interruption and concurrent submission checks; they are local substitutes.

Each initial managed run uses a fresh owned parent beneath `/tmp`; a mode child
is created without replacement and rejects retained/partial children. Optional
`BLOB_MANAGED_VERIFIER_REPORT` retains the reports; normal tests use temporary
directories. Source plans label local evidence and all role/owner/project/budget
bindings before requests. Set 01 captures the first successful three cases (88
files); set 02 captures final raw HTTP response, tamper refusal and recovered/
restored receipts (103 files). Both retain public local trust keys, observation
plans/outcomes, exact signed requests and service replies; no private key or
provider certificate is retained. Their SHA256SUMS manifests bind all files;
they establish capture integrity, not independent provider signatures/freshness.
Owned sockets, proxy, gateway/progress and instances were closed/dropped. Reports
and build artifacts remain; there are no new external objects/cleanup obligations.

All commands use repository `CARGO_TARGET_DIR`, offline locked Cargo and explicit
fixture paths. The managed fixture/CLI are unchanged artifacts from the preceding
batch. No full CI/release gate, version mutation, agent commit, tag, push,
publication, deployment or sibling edits occurred.

| Retained capture | Command/result | SHA-256 |
| --- | --- | --- |
| `/tmp/ic-blob-storage-managed-verifier-clippy-01.log` | Strict affected harness lint fails: module ancestor and assumed descriptor digest field; corrected to maintained DTO and update mode | `4802d69af68fc2aac0f62bf9d9dbbd2851b5d4bf37d5001eb0a503a67f85dd66` |
| `/tmp/ic-blob-storage-managed-verifier-clippy-02.log` | Same lint fails: large attestation passed by value; corrected to borrowed input | `a5df30dd757be948bb84c4b7b1912cb55a3126ed08a3b99e3af29e55ef45cf96` |
| `/tmp/ic-blob-storage-managed-verifier-clippy-03.log` | Targeted managed/storage harness Clippy passes before final capture expansion | `7ca149b78561d0b92aacb63f094fb21e9c965a9076069baf7616f3e7678795c8` |
| `/tmp/ic-blob-storage-managed-verifier-01.log` | `cargo test --offline --locked -p ic-blob-storage-pocketic-tests --test canic_composition managed_native_verifier_submission -- --test-threads=1`: three cases pass in 33.37 s | `5afb1712a601117e78cac243f913a350a38106b2ec4dea3f77fa0ee3dec71a17` |
| `/tmp/ic-blob-storage-managed-verifier-clippy-04.log` | `cargo clippy --offline --locked -p ic-blob-storage-pocketic-tests --test canic_composition --test storage --all-features -- -D warnings` passes with final capture | `5f8d6ab8726dad6ab9ffbf384dbe9108ea12ffe65b5c67afecf0c6dbbfbed8e4` |
| `/tmp/ic-blob-storage-managed-verifier-02.log` | `cargo test --offline --locked -p ic-blob-storage-pocketic-tests --test canic_composition canic_managed::cli -- --test-threads=1`: four affected managed cases pass in 41.86 s | `994c7c7bc9df0c39794203eb9eea4a42fa59e2edbb2f0df5d2a659b687ddc4af` |
| `/tmp/ic-blob-storage-managed-verifier-storage-build-01.log` | `cargo build --offline --locked --release --target wasm32-unknown-unknown -p blob-storage-probe --lib` passes in 8.29 s | `000c5464aa8bc66fa4a4e4701b7d4d350e9ed0dcaa4b9ba3b55eefa4801a6770` |
| `/tmp/ic-blob-storage-managed-verifier-storage-01.log` | `cargo test --offline --locked -p ic-blob-storage-pocketic-tests --test storage storage_observe_cli -- --test-threads=1`: three existing cases pass in 12.63 s | `cb1de78710e697ce8a2473ec7a2eed7a335f8a8446e1b642c3627cbc18f2cc7d` |
| `/tmp/ic-blob-storage-managed-verifier-evidence-01/SHA256SUMS` | Immutable initial 88-file report manifest | `2c2a725eb16305accfa238fa2ad6cb293c5db9fcb33ef0f2ff0879fc9e014270` |
| `/tmp/ic-blob-storage-managed-verifier-evidence-02/SHA256SUMS` | Immutable final 103-file report manifest | `426ecb608f34aa023513528d28ca0d36a6913e4dbc4b2a517270e63b79b9b001` |

| Final source/artifact | SHA-256 |
| --- | --- |
| `tests/pocketic/tests/canic_managed/cli/verifier/mod.rs` | `5eb46398943514238e46d5f68eed1f9bece4ebe002343e148c8a83677bcc8f5f` |
| `tests/pocketic/tests/canic_managed/cli/mod.rs` | `8aa0865ec6b978e18832091389019e61976d89303fd93b6c241b084e4e00b6a2` |
| `tests/pocketic/tests/submission_proxy/mod.rs` | `ab2d5b9c5e110f99e1d704c2f700123c9c9d006f41dae745691fa05b53930faf` |
| `tests/pocketic/tests/observation_provider/mod.rs` | `ea073603804aa43f0e035628553b4f95c9c9de993a3fb2212bfc8ef4b6860ef5` |
| `tests/pocketic/tests/storage_observe_cli/mod.rs` | `5acacf0f7e8ac89de7703b40afcf69af383d4109e55fe38efbe7d65696affa34` |
| `tests/pocketic/tests/canic_composition.rs` | `6e36b5715ae24a62e7e0cec09f662a23f7851213bc43c960afb58ec15e56e9c8` |
| `tests/pocketic/tests/storage.rs` | `f5838658246395aa445e41e8b59e24d6db0e91855baee3f852f861b9cb504208` |
| `target/wasm32-unknown-unknown/release/blob_storage_probe.wasm` | `817dd57e09ee70a6d7c37e8c6af60bb4f25c8a6891d71bf9ee3e0cb3acc09933` |

Next is signed tenant reference receipt/liveness and cleanup beside trusted
completion. Supported framework plain-reply rejection/decoding, production Fleet/
verifier provenance, deployed Caffeine qualification, concrete consumer/outbox,
operational recovery, IcyDB composition and removal/retirement acceptance remain
open. This local metadata completion must not be relabelled provider qualification.

Final root/isolated fixture formatting, diff, capability JSON and read-only draft
changelog checks pass. All 307 maintained local documentation links/anchors and
balanced fences validate. Eighteen current source/artifact/log/manifest table
hashes match; both complete 88/103-file SHA256SUMS manifests validate. Historical
source/artifact/log hashes remain bound to their original captures.

## Managed signed tenant reference submission and cleanup — 2026-09-30

Evidence class: actual local managed PocketIC/native signing with labelled
exposure and owned ten-byte HTTP content substitutes. No Caffeine certificate,
deployed object or paid request. Test tenant seed 43 and installed verifier seed
42 are distinct from operator/uploader. The public Canic helper, memory runtime,
application-only subnet trust and service/stable contract are unchanged.

Native `submit-reference` validates the saved full-width tenant command, claims
a new private directory and synchronizes canonical `request.candid`, exact
`signed-request.cbor` and bound `intent.json` before one IC update. It uses the
maintained request/reply validators. Empty/interrupted/existing claims refuse.
No polling, automatic retry or identity allocation. Exact recorded receipts
carry either success or a stored transition failure; pending, remote refusal and
transport uncertainty remain distinct. The output never equates historical
retain success with current liveness or logical release with provider deletion/
billing cessation. The source input is unchanged; recovery can use the durable
dispatch copy even after the incoming request file is removed.

The managed journey first runs real signed verifier observation/submission over
one persisted plan and local GET. The tenant then records an unknown-reference
release failure, retains a second reference and recovers acknowledged/dropped/
pending replies through signed receipt inspection. The shared proxy now accepts
one explicit sender/service/method/argument binding; it checks intent/signed bytes
before forwarding exactly one real reference update. Repeated invocation refuses
the claim without a second request. Foreign scope/identity and verifier-as-tenant
refuse before dispatch; an empty interrupted claim is preserved.

At full reference headroom, both releases still succeed, including final release
during suspension. Fresh retain refuses inactive enrollment. A deliberately
separate exact historical replay returns the original success without reviving
either reference; it is not the uncertainty-recovery path. Original success and
stored unknown-reference failure remain readable, separately from current dead
status. Last release leaves reserved/logical bytes zero and physical/liability
bytes ten. Same-release restoration fences all four owners; exact success/failure
receipt inspection and dead status survive, while replay/delivery refuse and full
stable memory remains unchanged. No provider deletion/settlement is fabricated.

Per journey, the pre-recorded maximum is ten signed updates, sixty CLI invocations,
one GET and ten content bytes; actual use is eight signed updates and twenty-seven
CLI invocations. Ordinary setup/enrollment/descriptor operations are local
PocketIC effects. CLI deadlines are thirty seconds, HTTP replies at most 256 KiB
and reference inputs/replies at most 4 KiB; the fault proxy bounds requests and
has a ninety-second lifetime. All attached provider cycles/payments/deployed
provider requests are zero. Unknown outcomes never authorize repeating an effect.

Optional `BLOB_MANAGED_REFERENCE_REPORT` captures new per-mode children under
an existing parent; retained/partial children refuse. Set 01 contains 277 files
with a no-overwrite SHA256SUMS manifest: public trust, role/budget plans, exact
commands, signed dispatches/outcomes, original/current/restored observations and
raw labelled HTTP request/response. Each of its twenty-four signed intent records
passes independent request/argument/trust SHA-256 and scope/budget inspection.
ContentDigest display hashes have the maintained `sha256:` prefix. No private
PEM is retained. Capture integrity supplies no provider signature or independent
freshness authority. Sources/proxy/gateway/progress and instances were stopped/
dropped; temporary identities removed, reports/build artifacts retained.

All Cargo commands use offline locked resolution, repository CARGO_TARGET_DIR
and explicit local artifact paths. This is targeted implementation validation,
not full CI/release verification. Registry versions, allocator, service Candid
and stable schemas remain unchanged. No version mutation, agent commit, tag,
push, publication, deployment or sibling edits occurred.

| Retained capture | Command/result | SHA-256 |
| --- | --- | --- |
| `/tmp/ic-blob-storage-managed-reference-cli-clippy-01.log` | CLI all-target strict lint fails: borrowed request-ID preferred; corrected | `9645a031ffbbc73e4f095423c9e7c44aa7eba6796428f10d0ac223c0de449e41` |
| `/tmp/ic-blob-storage-managed-reference-cli-clippy-02.log` | CLI strict lint passes before the named intent record refinement | `cd261f6e4baab0e488401b22de73a8bd8cbdd7cac2e361a1164de75bda2814d0` |
| `/tmp/ic-blob-storage-managed-reference-cli-clippy-03.log` | `cargo clippy --offline --locked -p ic-blob-storage-cli --all-targets -- -D warnings` passes with final source | `676401afa2a182f457d9685e31a7619594bb180cd678df9938a2723dbe2cc096` |
| `/tmp/ic-blob-storage-managed-reference-harness-clippy-01.log` | Affected harness lint fails: unused imports, boolean assertion and small dispatch boundary needing Copy; corrected | `023607f86f7cd357cab5e6b36848042c6e048e1d67a2abd5eda155731543bf1e` |
| `/tmp/ic-blob-storage-managed-reference-harness-clippy-02.log` | `cargo clippy --offline --locked -p ic-blob-storage-pocketic-tests --test canic_composition --test storage --all-features -- -D warnings` passes | `9cd6b49ba2d83f2a7321b9251b5267c28d834513da9c54142cafb468d3dd3739` |
| `/tmp/ic-blob-storage-managed-reference-cli-unit-01.log` | Native CLI unit run fails in existing HTTP download case on sandbox socket denial; twenty-eight other cases pass | `97684dfc8e799543897e9e5228df3d8aa321e842396e6881b1a02daeb211ac3e` |
| `/tmp/ic-blob-storage-managed-reference-cli-unit-02.log` | `cargo test --offline --locked -p ic-blob-storage-cli --bin blob-storage native::`: all twenty-nine cases pass with local socket access, 0.64 s | `fb16e68f7073bf987b424d7e31f50a85d7057e708298153a49a043804266e5e5` |
| `/tmp/ic-blob-storage-managed-reference-cli-build-01.log` | `cargo build --offline --locked -p ic-blob-storage-cli --bin blob-storage` passes in 2.00 s | `8a96b05301b98cbb2c600ab6b27a3cca2064d4aa3b2e027fff7111009349008a` |
| `/tmp/ic-blob-storage-managed-reference-01.log` | `cargo test … --test canic_composition managed_native_tenant_reference -- --test-threads=1`: three cases pass in 33.45 s | `74f5391c70a7a5081e04462296fdd27fcd756da8afead436d2c23603af89a0d5` |
| `/tmp/ic-blob-storage-managed-reference-cli-regression-01.log` | `cargo test … --test canic_composition canic_managed::cli -- --test-threads=1`: seven affected cases pass in 73.13 s | `21a237a3ad7f50bdd657f1896d9c8749d0acbf61d12d8408465fb95a34b9ec65` |
| `/tmp/ic-blob-storage-managed-reference-storage-observe-01.log` | `cargo test … --test storage storage_observe_cli -- --test-threads=1`: three existing cases pass in 11.90 s | `0aff9e3d8f790fd6a29c190b98e129469ae5cfb0cb71a4a56b6c5b1b9e87a08e` |
| `/tmp/ic-blob-storage-managed-reference-storage-receipt-01.log` | `cargo test … --test storage storage_reference_native_cli -- --test-threads=1`: existing signed receipt/status case passes in 11.84 s | `f38b4f0ebe3ba5808c45e902eb63e1ee33815eb49f62bd7d40eeee0b5da72696` |
| `/tmp/ic-blob-storage-managed-reference-evidence-01/SHA256SUMS` | Immutable 277-file report manifest | `087b26e690e5c69fc81b31a807fb48f1ca954c76c65823e2212701b9985414da` |

| Final source/artifact | SHA-256 |
| --- | --- |
| `crates/ic-blob-storage-cli/src/native/submit_reference/mod.rs` | `0ec673dfb89274cea9b2b193248fb6ebde7919a04a2cac5d82b6f8623fe0b83f` |
| `crates/ic-blob-storage-cli/src/native/submit_reference/tests/mod.rs` | `79e5952f31ec5d5471f59820c84e4344d3451628bfbf06b1244933838ed26f50` |
| `crates/ic-blob-storage-cli/src/native/references/mod.rs` | `ea704d978ea7fb8cec8f90565358f945c3c700947bc289a327d630d23da71e9c` |
| `crates/ic-blob-storage-cli/src/native/mod.rs` | `945006a7f4ced5f98a603a9d718f5dfa77def4a83148772a8d336c6a07d70f74` |
| `crates/ic-blob-storage-cli/src/native/arguments/mod.rs` | `d1c0ecc38f34019594cb956e36da63d483ee11141eb4276fcfc6ced97b20d747` |
| `tests/pocketic/tests/canic_managed/cli/references/mod.rs` | `aec8d53cd60177066b6ea69f49968f6b83c8e6919fdc973236a20b898b193ef0` |
| `tests/pocketic/tests/canic_managed/cli/verifier/mod.rs` | `2a801a741e7bba1988d534aa606737dde9f60d1ccfed3db578cb386b7ea872d1` |
| `tests/pocketic/tests/submission_proxy/mod.rs` | `9871318efefb30e3ad10d46e8b831a20893e6a39be3a71179405d2d6b4c23f0d` |
| `tests/pocketic/tests/storage_observe_cli/mod.rs` | `a58bac4200b9bdba9e0f36e54df6cb09a6a5601a88362d577e27d101f34cdad6` |
| `target/debug/blob-storage` | `52ff271acd70f949c0ee6f634585ac3bfc25b2d8ccdd71163f765841c50a4386` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.wasm` | `61138d7e982b98c93ab12fba5b38fb1244f88d15323f8d1ec8bc320bf3e3e0dc` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.did` | `4c0c27182e49ff9385b0f3adf67858116a79c803e6f4ae91051da7f2d2e43622` |

Next exercise native reference submission against the standalone host and define
the concrete consumer transaction/outbox journey. Supported framework plain-reply
rejection/decoding, production Fleet/verifier provenance, deployed Caffeine
qualification, operational recovery, IcyDB composition and removal/retirement
acceptance remain open. Local reference completion/cleanup does not qualify the
provider or authorize operating a restored instance.

Final root formatting, diff, read-only draft changelog and capability JSON checks
pass. All 313 maintained local documentation links/anchors and balanced fences
validate. Twenty-five current source/artifact/log/manifest table hashes and the
complete 277-file SHA256SUMS manifest match. Historical source/artifact/log hashes
remain bound to their original captures.

## Standalone signed reference submission and refused inspection — 2026-10-01

Evidence class: local PocketIC against the actual standalone artifact, genuine
signed native commands and owned loopback fault transport. The application and
NNS supply real replica/query signing; tenant, uploader, operator and controllers
are distinct. Production issuance/exposure stays disabled. No confirmed object
is injected and no successful standalone retain/release is claimed.

Three maintained journeys use acknowledged, dropped and pending acknowledgments
for one unknown-upload reference update. Exact command, signed body and intent
are durable before the shared proxy forwards it once. Acknowledged refusal stays
typed Unknown. When the acknowledgment is lost/pending, receipt inspection itself
refuses Unknown; it cannot resolve the original uncertainty or prove absence.
The original outcome remains byte-exact and a repeated submission cannot resend.
Actual admission/preparation then yields typed Unconfirmed for both retain and
release. Wrong scope, signer and trust refuse. Same-release upgrade fences all
four service owners; release refuses Fenced while historical inspection remains
Unconfirmed. All inspection/refusal phases preserve complete stable memory and
the original 10 MiB reserved/logical/physical/liability accounting.

Budget per journey: at most eight signed updates and forty CLI invocations,
30-second deadlines, 256 KiB transport/4 KiB reference replies and one owned
10 MiB manifest/reservation. Each executed journey records four signed intents.
Provider GETs, deployed requests, attached provider cycles and payments are zero.
Optional BLOB_STANDALONE_REFERENCE_REPORT retains fresh mode directories; normal
runs use temporary directories. The fresh 82-file report contains twelve signed
intents with request/body/trust hashes and raw service replies; temporary PEM
keys are removed. Proxies, gateways, progress and instances were stopped/dropped.
No external object or cleanup obligation was created; build artifacts remain.

| Retained command/report | Outcome | SHA-256 |
| --- | --- | --- |
| `/tmp/ic-blob-storage-standalone-reference-clippy-01.log` | Failed: journey too long and inefficient clones; corrected | `c45d6f490b1bb31e75d71361e56fa0b3be97daa58e676c7626443a5ccddddf22` |
| `/tmp/ic-blob-storage-standalone-reference-clippy-02.log` | Failed: journey still too long; preparation helper extracted | `384b1739850740cc2e82642c850c77a449a44801ae2c3fd3a4f101e587fec6c8` |
| `/tmp/ic-blob-storage-standalone-reference-clippy-03.log` | Failed: journey two lines over lint bound; preparation stage extracted | `1d2b6721a1126c05ce7de9ebc0ffb211f61f4e4d80375aaa06abc7f04815239c` |
| `/tmp/ic-blob-storage-standalone-reference-clippy-04.log` | Strict affected standalone harness Clippy passes | `5aeafbfe96538610960891b8f38d5f96812fda854630e2e67e7c56a5b7a034c3` |
| `/tmp/ic-blob-storage-standalone-reference-wasm-build-01.log` | Offline locked standalone release Wasm build passes, 4.30 s | `e632015134b05312e276e40dca8f85ffcc1552d1299772e070f4ac61795d0bcc` |
| `/tmp/ic-blob-storage-standalone-reference-cli-build-01.log` | Offline locked native CLI build passes, 8.96 s | `9ddaef0a7f45ba4a1b2b6917fed3f2560f9145c8f39ddcc643e46fdbe189031a` |
| `/tmp/ic-blob-storage-standalone-reference-01.log` | Offline locked standalone_reference_native_cli: three cases pass, 35.21 s | `2a7298f4bcd438e54e73c5a7f25121a426314d3c445264fa28da4782a7fa45f1` |
| `/tmp/ic-blob-storage-standalone-reference-evidence-01/SHA256SUMS` | Immutable 82-file report manifest | `c2f585b60c2ef134bdc074770829722fcfbe283e22ee12c9a16116d23c9f88ad` |

| Current source/artifact | SHA-256 |
| --- | --- |
| `tests/pocketic/tests/standalone_reference_native_cli/mod.rs` | `0dbee152313870d2bc16fa18494b321a91652fb03c7bae83e15713921ec8ab77` |
| `tests/pocketic/tests/standalone.rs` | `907c0e8409cf58c3113fcc2ffb19d162c8ec24b34971f1663993ca509ca578e9` |
| `tests/pocketic/tests/submission_proxy/mod.rs` | `9871318efefb30e3ad10d46e8b831a20893e6a39be3a71179405d2d6b4c23f0d` |
| `Cargo.lock` | `c1e76826b4bd79ad28ae67c66939cfd04ed8b7fea1c720956ca94fa76a91f53a` |
| `target/debug/blob-storage` | `08fb49a53bddc96d19a31f68cb653e64c46e4117cfa9ec3416ba3b4c081eaa87` |
| `target/wasm32-unknown-unknown/release/ic_blob_storage_canister.wasm` | `f9c51d78f9b7d4bf45c46cbc4d01daeab7e1894f29281ce9e742eed2228f563e` |

The pre-existing worktree yoke-derive 0.8.3 → 0.8.4 lock change was preserved,
not introduced here. Rebuilt CLI/standalone hashes bind this current input;
earlier evidence keeps its original artifact hashes. This proves maintained
production refusals, not provider upload/completion, deletion or billing cessation.

## Managed application outbox and publication race — 2026-10-01

Evidence class: local application substitute using the existing bounded consumer
probe as an actual canister tenant against managed storage. The public Canic
qualification helper remains installation/runtime owner. The probe calls the
shared replicated admission/reference/descriptor clients; no application workflow,
provider contract or second service journal is copied. Its local durable asset/
outbox record remains bounded to 16 KiB in its own memory 120. Service, tenant,
operator, uploader and verifier are explicit and distinct. The managed input
uses the probe's maintained fixture project; IDs retain full-width independence.

One fresh asset uses its original first reference; a reuse asset acquires a
separate reference. Actual replicated admission precedes manifest preparation.
Fixture-only operator exposure and configured-verifier attestation over known
ten-byte content are labelled metadata substitutes: no GET, provider object,
certificate or production issuance. Operator/root cannot perform tenant mutations.

The first case traps callbacks after committed retain/release effects. Durable
application intents retain unresolved results without pretending those effects
failed. Cancellation preserves the tombstone and release waits for acquisition
reconciliation; exact receipt recovery records the original retain without
publishing or redispatch. Active uses refuse cancellation without changing either
owner. After detach, reserved cleanup remains available at zero fresh-retain
headroom during suspension. Final logical/reserved bytes become zero while ten
physical/liability bytes remain. The service is fenced before recovery of the
pending release; successful passive receipt recovery under that mutation fence
proves no redispatch. Repeated release returns the saved result unchanged.

The second case holds a reuse registration after its descriptor call. Cancellation
and release win before resuming that callback; the late callback returns typed
State, cannot publish, and new use refuses. The other asset/reference stays live
until its own cancellation/release. Both cases upgrade the consumer within the
current release after service restore, retaining exact asset history/tombstones
and its independent fence. Fenced registration/release/recovery refuse with both
stable memories unchanged. They do not resume operational writes.

Budget per case: sixty-four explicit application/service updates, sixty-four
inspection queries, 30-second client deadlines and 4 KiB replies. The existing
hold bounds management raw_rand calls to 128. Declared content is ten bytes;
provider GETs/deployed requests/attached provider cycles/payments are zero.
Optional BLOB_MANAGED_CONSUMER_REPORT uses fresh children; normal runs use temporary
directories. Set 01 retains the initial outbox case (15 files); set 02 retains
both final cases and precise budget labels (29 files). Raw canonical Candid
captures retain intents, admission/completion, pending/recovered assets,
accounting and restored history. No prior capture was overwritten. Owned
instances/temporary resources were dropped; reports and build artifacts remain.

| Retained command/report | Outcome | SHA-256 |
| --- | --- | --- |
| `/tmp/ic-blob-storage-managed-consumer-clippy-01.log` | Failed: constructor too long; completion stage extracted | `a763bdd3217df7b1a965aa835c065fb03ce16155dfb8feef5ca114146cd4ebed` |
| `/tmp/ic-blob-storage-managed-consumer-clippy-02.log` | Initial affected managed harness strict Clippy passes | `d9db2f10eda07bb8c321efdc4af706b1178a944ec4b95145368178d06053fce8` |
| `/tmp/ic-blob-storage-managed-consumer-clippy-03.log` | Managed and standalone harness strict Clippy passes after race addition | `5f7da738179eeea65682d4a97df4133f2ba96819d12fcd7493ebbcf8195011ff` |
| `/tmp/ic-blob-storage-managed-consumer-clippy-04.log` | Final affected managed and standalone harness strict Clippy passes | `5f7da738179eeea65682d4a97df4133f2ba96819d12fcd7493ebbcf8195011ff` |
| `/tmp/ic-blob-storage-managed-consumer-build-01.log` | Offline locked existing consumer release Wasm build passes, 4.45 s | `f96ee95fb70a755efe9b1b91c3faca2d831b293f16e623d7f3ebde0828998b6a` |
| `/tmp/ic-blob-storage-managed-consumer-01.log` | Initial outbox case passes, 12.14 s | `a74fcd37513a6203d6a4b50cdff1ff17884889b088f3e675ef8f50a586ffb605` |
| `/tmp/ic-blob-storage-managed-consumer-02.log` | Offline locked canic_managed::consumer: both cases pass, 20.06 s | `76d9c632431c23951300a6c5d85210c05d91ae13cf396f0834d7546601df5d2d` |
| `/tmp/ic-blob-storage-managed-consumer-declaration-01.log` | Maintained declaration comparison passes: twenty-nine exact method types/modes, 0.02 s | `42b3bfdc287c77a1cf3163351b423cd34d41bb3ee26eef820347e72c21df41b0` |
| `/tmp/ic-blob-storage-managed-consumer-evidence-01/SHA256SUMS` | Immutable initial 15-file report manifest | `39a8e9df7428da74f867fe6e2a0091429c5197d7d2fbc0035b4b69c6129631a4` |
| `/tmp/ic-blob-storage-managed-consumer-evidence-02/SHA256SUMS` | Immutable final 29-file report manifest | `29367dc7d6fac568c4a7186bcd43eae3a7bf45fa32b3839d01ed39c719520da5` |

| Current source/artifact | SHA-256 |
| --- | --- |
| `tests/pocketic/tests/canic_managed/consumer/mod.rs` | `d7c6ba615cae9b8c14c7b23b80fbd36d368bda67870dc3d05f76ad57ebdef68d` |
| `tests/pocketic/tests/canic_managed/mod.rs` | `e592952f9ca705036a32b82289789aba7dd00700d0a4aebd782df87279e95a5e` |
| `canisters/test/consumer_probe/src/ops/mod.rs` | `a3b5345e1613d8b6b399505c80253b88f526154feac3bed29a8eb4634baf1e82` |
| `canisters/test/consumer_probe/src/model/mod.rs` | `b212a8df9bb33f71bf083f22447af2803c4bb2c283b0bbc979ff1914e546d485` |
| `canisters/test/consumer_probe/src/workflow/mod.rs` | `b7ed8fdffa8563b731c9f1e986c65cc2750ec5645b79d71cd4169f18b8ffe5ba` |
| `Makefile` | `36dbf37637f579cf341a067d581c4503d2ef5e5427501c64937aff02ccb94c2f` |
| `target/wasm32-unknown-unknown/release/blob_consumer_probe.wasm` | `0ab231b0dc3127dea9606d1c5e02e40da2ef3993b1c3d104a89c07f6c75a90e9` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.wasm` | `61138d7e982b98c93ab12fba5b38fb1244f88d15323f8d1ec8bc320bf3e3e0dc` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.did` | `4c0c27182e49ff9385b0f3adf67858116a79c803e6f4ae91051da7f2d2e43622` |

The focused Canic target builds the existing consumer artifact alongside its
existing inputs. No production service/probe/CLI contract, schema, dependency
version or allocator changed in these test additions. Targeted checks only;
no full CI/release validation, version mutation, commit, publication or deployment.
Next cover occupied application restoration with an unresolved outbox entry;
current successful recovery occurs before the consumer's own fence. Production
consumer/serving acceptance, Fleet/verifier provenance, supported framework
certificate reply/decoding, deployed provider qualification, operational recovery,
IcyDB composition and source removal/retirement remain open.

Final root formatting/diff, read-only draft changelog and capability JSON checks
pass. All 340 maintained local documentation links/anchors and balanced fences
validate. Thirty-three current source/artifact/log/manifest hashes and all three
complete retained report manifests match; historical captures remain unchanged.

## Managed unresolved outbox restoration — 2026-10-01

Evidence class: local managed PocketIC/application substitute. The existing
consumer probe, configured ten-byte exposure/completion and public Canic fixture
are unchanged. Service, tenant, operator, uploader and verifier stay distinct;
consumer and service keep their own existing durable owners. No production API,
stable schema, allocator, dependency or extra recovery component is added.

Four new cases interrupt callbacks after actual committed retain/release effects,
then restore the application before the service or the service before the
application. Both owners are upgraded again within the same release. Complete
asset intent/payload and independent upload/object/lifetime/reference/cleanup IDs,
tombstones and absent application acknowledgments remain exact. An unresolved
retain has an original successful service receipt and a live reference but no
application retain result or release dispatch. An unresolved release has both
original successful service receipts and a dead reuse reference, while its
application release acknowledgment stays absent. The published first asset and
its original live reference stay independently owned, retaining ten logical,
physical and liability bytes with zero reservation.

Each phase compares exact decoded histories, all four service fences and current
reference fences. Consumer registration, preparation/admission, cancellation,
release, both acknowledgment recovery paths, withdrawal/recovery and new use
refuse Fenced before effects. The existing workflow checks mutation authority
before awaiting clients; acknowledgment itself is a mutation and cannot bypass
the consumer fence. Controller asset inspection/recovery returns Denied. Service
reference mutation returns Fenced even for exact historical replay. Full stable
memory of both owners is unchanged across inspection/refusals and both asset
views remain exact. In application-first restoration, these application refusals
also run while the service is still operational. This gives executable fence/
history evidence alongside the source authority checks, not an outbound network
request counter. Restored published views are retained history, not serving leases.

Passive service receipt/status queries explicitly select the tenant identity in
PocketIC. They do not show the fenced consumer performing reconciliation or
persisting an acknowledgment. No fence is cleared. These cases exercise retained
unresolved outbox work, not an old snapshot or a callback still in flight during
code installation. Operational restart and production consumer serving remain open.

Per case budget: sixty-four explicit application/service updates and sixty-four
inspection queries, thirty-second client deadlines, 4 KiB replies and ten declared
bytes. No hold is used by the new cases; the rerun existing publication-race case
retains its 128-call management hold bound. Provider GETs/deployed requests/attached
provider cycles/payments are zero. The fresh retained parent
/tmp/ic-blob-storage-unresolved-outbox-evidence-01 has 83 files across four new
interruption children plus the existing outbox/cancel-race children. Canonical
Candid snapshots retain original intents, prepared manifest, labelled completion,
unresolved history, restore order, phase histories and typed refusals. The immutable
manifest covers every report file. No previous capture was overwritten. Owned
instances/temporary resources were dropped; reports/build artifacts remain, with
no external object or outstanding external cleanup.

| Retained command/report | Outcome | SHA-256 |
| --- | --- | --- |
| `/tmp/ic-blob-storage-unresolved-outbox-clippy-01.log` | Failed compilation: installation extension trait missing in new module; corrected | `c24989f83d6da0da74427a8e8f73912bb361284a933e4bdb1b42bf813e1fcae5` |
| `/tmp/ic-blob-storage-unresolved-outbox-clippy-02.log` | Offline locked affected canic_composition harness strict Clippy passes | `d6c935d1c6a7e7b4dabadacd1c108b70c781c9116a13fbb2765fb3bab2b7d1dd` |
| `/tmp/ic-blob-storage-unresolved-outbox-01.log` | Offline locked canic_managed::consumer: four new and two existing cases pass, 57.45 s | `a589ae48f5c162906c2aad33c49447e562bd36acbf7aa3b0ba4d1eceabf7aa60` |
| `/tmp/ic-blob-storage-unresolved-outbox-evidence-01/SHA256SUMS` | Immutable complete 83-file report manifest | `40a1d8a0c2ca7476ab995aff3fe5033569c15ab76a99a87334cef148e4509efb` |

| Current source/artifact | SHA-256 |
| --- | --- |
| `tests/pocketic/tests/canic_managed/consumer/interruption/mod.rs` | `8b811b826b71683fb4d29a137b2c390d39701665c8395b829f9df03fce455a59` |
| `tests/pocketic/tests/canic_managed/consumer/mod.rs` | `2da33a638e0234e9561f9cbec16c0e765fe7fcf1e0d5a653db77d7533392f168` |
| `canisters/test/consumer_probe/src/ops/mod.rs` | `a3b5345e1613d8b6b399505c80253b88f526154feac3bed29a8eb4634baf1e82` |
| `canisters/test/consumer_probe/src/model/mod.rs` | `b212a8df9bb33f71bf083f22447af2803c4bb2c283b0bbc979ff1914e546d485` |
| `canisters/test/consumer_probe/src/workflow/mod.rs` | `b7ed8fdffa8563b731c9f1e986c65cc2750ec5645b79d71cd4169f18b8ffe5ba` |
| `target/wasm32-unknown-unknown/release/blob_consumer_probe.wasm` | `0ab231b0dc3127dea9606d1c5e02e40da2ef3993b1c3d104a89c07f6c75a90e9` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.wasm` | `61138d7e982b98c93ab12fba5b38fb1244f88d15323f8d1ec8bc320bf3e3e0dc` |

Source began clean at released 0.4.11, HEAD/origin/main/peeled tag
807b0ae0e8c8d2f43f01bab7a6d285888fe2d20c; its release receipt binds parent
c833c402a9abbc90147f7f48b55188557e673846. Only targeted implementation checks ran;
no new full CI/release gate, artifact rebuild, version mutation, agent commit,
push/publication/deployment or paid provider probe. Historical hashes remain
bound to their original captures. Next address the supported managed certificate
boundary and remaining provider/operational recovery evidence. Production consumer
adoption/serving, Fleet/verifier provenance, IcyDB composition and source removal/
installation retirement are still unqualified.

Final root formatting/diff, read-only draft changelog and capability JSON checks
pass. All 344 maintained local documentation links/anchors, balanced fences,
eleven current source/artifact/log/manifest hashes and the complete 83-file
report manifest validate. Earlier evidence remains bound to its original captures.

## Managed certificate framework review — 2026-10-01

Evidence class: immutable upstream source/cached registry comparison, not deployed
Caffeine or full framework qualification. The ledger records intent and request
budgets before the investigation. Four metadata requests and four source fetches
were used; deadlines were thirty seconds, metadata/source ceilings 2 MiB and
unused archive allowance 4 MiB. No provider/payment/deployment or sibling edit.

The GitHub landing discovery succeeded; the web registry opening was unavailable
and the explicit crates API request returned HTTP 403 with an empty body. Both
failures are retained. Latest registry version is therefore unverified.
`git ls-remote` resolved HEAD/main and peeled v0.110.48 to
`8d37c74c9a4457b9e2bd47ee883f98fd2889d63b`; the annotated tag object is
`0707017060fcc086074d46a21435fe9759af109d`. The immutable Cargo.toml reports
0.110.48. Downloaded access/expansion/parser files match pinned registry files
byte-for-byte. The read-only dirty sibling checkout is at
`02af7277664c1664a58bfaa5e6fde997474c9802`; it was neither altered nor treated
as a clean release. Relevant source:
[public expansion](https://github.com/dragginzgame/canic/blob/8d37c74c9a4457b9e2bd47ee883f98fd2889d63b/crates/canic-macros/src/endpoint/expand/mod.rs),
[access planning](https://github.com/dragginzgame/canic/blob/8d37c74c9a4457b9e2bd47ee883f98fd2889d63b/crates/canic-macros/src/endpoint/expand/access.rs),
[attribute parser](https://github.com/dragginzgame/canic/blob/8d37c74c9a4457b9e2bd47ee883f98fd2889d63b/crates/canic-macros/src/endpoint/parse/mod.rs).

The normal default Fleet guard still requires Result and cannot emit the maintained
plain Caffeine success record with IC rejection on refusal. Queries use default
CDK decoding; update payload limits expose raw byte limits/skipping quota 10,000
but not work/type/header hooks. The prior actual compilation refusal remains
separate retained evidence; unchanged macro bytes did not justify another failed
build. Internal classification, wrapper replies and copied dispatch remain
prohibited. Supported rejection/decoder hooks, provider qualification, production
provenance and operational recovery stay distinct gates. No upstream message sent.

Retained directory: `/tmp/ic-blob-storage-certificate-framework-review-01`. Its
immutable manifest binds eighteen files (headers, stderr, discovery, refs, source
and comparison bindings); manifest SHA256 is
`7a7c921b607ab6a574da72716a02a0c5174f31b35cfbbd2bf466876b567742d9`. Owned request resources closed; no external
cleanup outstanding. Responses/hashes demonstrate inspected source integrity,
not provider guarantees or framework implementation.

| Source capture | SHA256 |
| --- | --- |
| `/tmp/ic-blob-storage-certificate-framework-review-01/access.rs` | `40df7ef009a5baa79bf12749edc803f03fb3f121d5aab94257f67c00025b7406` |
| `/tmp/ic-blob-storage-certificate-framework-review-01/expand.rs` | `852b10c671b59ce3fedb1a29367fbfcffb5ae40404da4a34fc7af69ec5c91cd1` |
| `/tmp/ic-blob-storage-certificate-framework-review-01/parse.rs` | `e591b5749791cdd6bde85f46f0016df6ae333abe6d50c667d82cb8aac5bc04f3` |

## Signed native account inspection and bounded transport — 2026-10-01

Native `inspect-account` signs one existing `blob_inspect_account` read update
for explicit balance/relationship kind and complete installed operator scope. The
service remains the provider request owner. A thirty-second deadline bounds exact
IC request waiting, with 256 KiB HTTP and 4 KiB Candid replies, decoding quota
100,000, skipping zero, 128 types and 4 KiB header bound. No payment, automatic
refresh/redispatch, gateway mutation, journal, new endpoint/schema or dependency.

Actual signed native subprocesses run through standalone and managed adapters over
the existing query-only local Cashier. The standalone fixture owner supplies NNS
root trust; Canic's public qualification helper owns its application-only instance,
and its independent control API supplies the undelegated subnet trust key. Normal
Fleet/bootstrap/caller guards remain active. Fixture setup now uses its configured
operator for readback and enrollment rather than an old fixed test principal.
Each fresh journey's pre-effect plan budgets twenty-four CLI invocations and local
provider queries; fifteen client invocations execute. Raw canonical source/request
Candid, command/result JSON and public roots are retained, with temporary PEM only.

Full-width independent reported balances, arbitrary-width signed relationship
amounts/Nat bandwidth, u64-max period and absent expiry are preserved. Account
absence, no reported relationship and diagnostic unauthorized principal stay
observations, never zero/credit/spendability/retry authority. Invalid/oversized
provider replies, wrong actual/declared signer, namespace/Cashier/payer and damaged
root refuse. Certificate failure after a read update may follow its execution; it
is not proof the provider read was unsent. Both complete service stable memories
stay unchanged through observations/refusals. Same-release upgrade fences balance
and relationship inspection and preserves both service/source stable bytes.

Cached ic-agent 0.49.2 source exposed a retry mismatch: `with_http_client` wraps
its client in RetryLogic, including HTTP 429/503 retries despite TCP retries zero.
The maintained Agent now uses public `with_arc_http_middleware` with the existing
reqwest client (retry never, redirects refused), retaining byte limits and trust
verification. An owned network counter regression asserts one service query or
update under each response. Separate query trust-key reads are not redispatch.
The first test attempt incorrectly counted those reads; its failure is retained.

Validation, all offline/locked and targeted:

- Thirty-five native CLI unit cases pass in 0.65 seconds; CLI build passes in
  2.52 seconds and final CLI all-target strict Clippy in 0.75 seconds.
- New standalone signed account journey passes in 7.03 seconds; managed in
  10.82 seconds. The first managed attempt refused fixture readback with Denied
  because the helper assumed its prior operator; that attempt is retained.
- Existing managed signed client (10.30 seconds), standalone signed client
  (6.46 seconds) and occupied managed operator (16.05 seconds) regressions pass.
  Final harness all-target/all-feature strict Clippy passes in 1.01 seconds.
- Earlier lint and unit failures remain in the fresh validation capture; no
  evidence was overwritten. Production storage Wasm inputs are unchanged, so
  existing released artifacts were reused. No full CI/release gate was run.

Commands select CLI units in `ic-blob-storage-cli`, and `standalone_native_account`
or `managed_native_account` in the corresponding PocketIC test target with
`--test-threads=1`; configured artifact paths are retained in validation inputs.
Local query sources/exposure are substitutes. There were zero deployed Caffeine
requests, attached provider cycles, payments or outstanding external cleanup.
Owned gateway/progress/instances/sockets and temporary identities were closed or
dropped; build artifacts and reports remain. This does not qualify complete
account activity, provider credit, real serving, production Fleet or safe recovery.

The first fresh root `/tmp/ic-blob-storage-native-account-evidence-01` retains
44 standalone files plus validation logs/inputs. Its manifest SHA256 is
`0dde008111c9d03d19ce515cef62185b31c9b37508494372634bf89f85070070`. The second fresh root
`/tmp/ic-blob-storage-native-account-evidence-02` retains 44 managed files,
manifest SHA256 `7bb8d10e7f5e11578669a26f6389c3cbbd3420da68c2ebc16dd51486da79873e`.
No private identity is retained. Successful IC certificate validation concerns
these controlled service observations; report hashes are local integrity only.

| Maintained input/artifact | SHA256 |
| --- | --- |
| `crates/ic-blob-storage-cli/src/native/mod.rs` | `dca5601cb8c762c2601f21eee1a60f0a29a0cd392b14f69bcc9c4fcb5958e41b` |
| `crates/ic-blob-storage-cli/src/native/arguments/mod.rs` | `f385ad2a15d9e4e22f738915f25a85589189a66cfe9a2940eb0dfdf3d7d7e1a7` |
| `crates/ic-blob-storage-cli/src/native/account/mod.rs` | `0010bcb04c383e2cebf9b47b2ac741aa15468cfe106afe7f6c87f120897a659d` |
| `crates/ic-blob-storage-cli/src/native/account/tests/mod.rs` | `91d924ffddef30d53ca79d377cd2f3198d441ad4767b59846702d2a9b5a91c3e` |
| `crates/ic-blob-storage-cli/src/native/tests/http/mod.rs` | `684353a66ef7e4fe5b4933ffbdadf80259a96d846aa013411afca67da4d254ac` |
| `tests/pocketic/tests/account_native_cli/mod.rs` | `be564bf39497e0437ce21a72d20f6d53cc49b5381e474e81e2af396554d73f14` |
| `tests/pocketic/tests/standalone_account/native_cli/mod.rs` | `b9ef723b164b8f7b32faa40211e88b4912b19482e50a97163bd6d7a856ccc2ce` |
| `tests/pocketic/tests/canic_managed/operator/native_cli/mod.rs` | `d9287c820341cdb049b4a7832a9c4e21aa9b0bf1b403f96f06155baf3dc721bb` |
| `tests/pocketic/tests/canic_managed/operator/mod.rs` | `d8c54e3450d45132088fe9137f7b18094a535ab4002187f07ac4974d7f7a76fd` |
| `tests/pocketic/tests/canic_managed/mod.rs` | `c62abfc8adb86dbeebe9d21990eb3933949630b1ed293615e5c824f7700c721e` |
| `tests/pocketic/tests/canic_managed/installation/mod.rs` | `532ec01851af75b479f8783a2e376101ede5dbfad7cb82c75095df8be844211c` |
| `target/debug/blob-storage` | `27a87703073ba177fab8581b9953b3d70f6f4de1ccd26f2471197724b3c17adb` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.wasm` | `61138d7e982b98c93ab12fba5b38fb1244f88d15323f8d1ec8bc320bf3e3e0dc` |
| `target/wasm32-unknown-unknown/release/ic_blob_storage_canister.wasm` | `f9c51d78f9b7d4bf45c46cbc4d01daeab7e1894f29281ce9e742eed2228f563e` |
| `target/wasm32-unknown-unknown/release/blob_gateway_source.wasm` | `f01fdb8e3cf9867cccd9689c222a8f2976ecc5b07fe7f8623c4bf5eb0ec94325` |

## Signed native gateway controls — 2026-10-01

Released baseline 0.4.12 is `468b871077ac2dcf2fcf0d8d7370795887d2332e`; its
receipt binds source `08de8e69c0a653435d294ad6dce8273bd60cfda6`. Continuation
started clean. This batch adds native tooling and actual signed local journeys,
without service API/stable-schema, dependency, allocator or lifecycle changes.
Existing released standalone/Canic/source Wasm artifacts are reused. No version
mutation, commit/tag/push, publication, deployment or paid effect by this agent.

`sync-gateways`, `cancel-gateway-sync` and `revoke-gateway` call existing shared
handlers. The service remains provider-request owner; only sync invokes the
installed Cashier query. Exact cancellation takes an observed positive sequence;
no predicted/local allocation or implicit new decision is introduced. A fresh
private durable directory is claimed before one network submission, retaining
canonical Candid, signed CBOR and scoped hashed intent. Existing/interrupted claims
refuse without dispatch. Typed decoder bounds are 4 KiB, work 100,000, skip zero,
128 types and 4 KiB header; HTTP is 256 KiB and the single request has a thirty-second
deadline. No polling, automatic retry, fallback or operator payment.

Outcomes distinguish acknowledged, pending admission, typed refusal and uncertain
transport/decoding. Scope, positive returned sync sequence and exact revocation
request are validated; cancellation's unit acknowledgment binds the saved signed
request ID. Missing/untrusted acknowledgments may follow committed effects. All
outcomes deny retry/deletion/billing-cessation authority and explicitly report
that historical gateway mutation receipts are unavailable. Current signed status
remains a separate observation and cannot settle which operation changed state.
Partial client storage is retained, never automatically resumed; it is not an
independently surviving freshness authority after restore/copy.

One shared actual signed subprocess journey runs through both adapters. Standalone
uses the owning NNS root; managed composition uses Canic's public qualification
helper with independently pinned undelegated subnet key and non-owning gateway.
The pre-effect plan budgets forty CLI invocations and twenty local provider reads;
each journey uses thirty-nine CLI invocations and six sync queries. The existing
fault proxy verifies the retained canonical/signed intent and signature bindings
before forwarding each one update, then passes, drops or reports pending admission.
No private PEM is retained. Source mode intent is saved before its local selection.

Signed outsider controls and wrong namespace/Cashier/payer refuse; identity mismatch
and zero sequence fail before claiming. Empty interrupted claims remain empty.
Valid sync acknowledges exact sequence; committed dropped revocation leaves
uncertainty while status shows absence. Pending sync admission is not completion;
status independently observes the applied membership. Malformed/oversized provider
replies preserve exact pending identity and old membership; overlapping sync and
stale cancellation refuse. Dropped cancellation leaves an uncertain outcome while
status shows no pending sync, without clearing original uncertainty. Exact
acknowledged cancellation, another invalid sync, pending revocation and absent
acknowledged revocation preserve the same semantics. A final rejected sync retains
its pending identity across same-release upgrade; all three controls refuse the
restore fence. Source query modes remain labelled substitutes.

The standalone prepared upload retains 10 MiB reservation/physical/liability
accounting; managed retains its prepared ten-byte reservation. Every signed status
phase preserves unrelated upload/funding/read sections. Complete service/source
stable memories stay unchanged through authority refusals and through restored
control/status refusals. Neither provider credit, deletion/billing stop, real
Caffeine behavior, production Fleet/consumer acceptance nor safe restart is proved.

Validation, offline/locked and targeted:

- Thirty-nine native units pass in 0.69 seconds, including exact arguments, bounded
  acknowledgment validation, typed errors and independent outcome classifications.
- Standalone signed gateway journey passes in 13.77 seconds; managed in
  14.70 seconds. Both existing standalone sync/restore cases pass in 11.14 seconds.
- CLI build passes in 12.38 seconds; final strict CLI all-target Clippy passes
  in 0.28 seconds and harness all-target/all-feature Clippy in 0.99 seconds.
  The first harness lint refused a long journey helper; that log remains retained.
- No full CI/release gate ran. Source Wasm rebuilding was unnecessary for the
  unchanged service contract; build caches/artifacts are retained.

The fresh capture `/tmp/ic-blob-storage-gateway-controls-evidence-01` retains
204 standalone and 204 managed files, plus validation commands, logs and exact
input/artifact bindings (420 files overall). Empty interrupted claim directories
remain alongside their refused-command captures. SHA256SUMS manifest digest:
`1c996ffc5ee3534b4a94d42f621a2518d8df99093944a6feb8366f769407f022`. No files or failed attempts were overwritten.
Owned proxy/gateway/progress/instances and temporary PEM/input files were closed
or dropped; reports/artifacts remain, with zero deployed Caffeine requests,
payments/provider cycle attachments and no external cleanup obligations.

| Maintained input/artifact | SHA256 |
| --- | --- |
| `crates/ic-blob-storage-cli/src/native/gateway_controls/mod.rs` | `3cd75eed77b32ae79bcaf8c35dc8cf473c410c42fc874a011b9111565df7247e` |
| `crates/ic-blob-storage-cli/src/native/gateway_controls/tests/mod.rs` | `c09e11a1ca237ecf23ec862465eadf6717189c365ed3365e05d69ab86e9a510e` |
| `crates/ic-blob-storage-cli/src/native/mod.rs` | `1e1da146da6bccf7f9348d6d8b94d7c282cdf9559dc1cc6b57c60e2b5c25984f` |
| `crates/ic-blob-storage-cli/src/native/arguments/mod.rs` | `a4f6919791ba85d2949ba8a829306eb8d58252e422ecbe5b49d047849caac762` |
| `tests/pocketic/tests/gateway_native_cli/mod.rs` | `279745f8d8634c8b9828b06134b0ab100e958712b7fa8bb931659174c20d2eda` |
| `tests/pocketic/tests/standalone_gateways/native_cli/mod.rs` | `f1a796e5a91b6ea85b19e21a85dbf9dbfdeb690135e05f9974618de938c6f37c` |
| `tests/pocketic/tests/standalone_gateways/sync/mod.rs` | `da5823aba9a7291d63287864309c6daaa1d9371716202c9b1785efe9ae4a7f67` |
| `tests/pocketic/tests/canic_managed/operator/gateway_cli/mod.rs` | `0a0b0c3306baacd6c3b2d0dd5365508526a036f79e08490cd9452952308c3478` |
| `crates/ic-blob-storage/src/dto/gateway/mod.rs` | `71f12796a908c7fc2c99d5d3b38e42aa010f906ad9e4f82301620a65bc10ece8` |
| `crates/ic-blob-storage/src/dto/gateway/sync/mod.rs` | `a2e21820b04239ba3b8a2666ccaa52f33f6d91ede5e5455c95d8c03945d9f56a` |
| `target/debug/blob-storage` | `7b6b6e7618f32a7b6d28e53d388a378b1713f81cb0cd196728f067d34203a0dd` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.wasm` | `887ff19971281987a9140353fa1625892959778a19ac1240e1fb24d6f497b57b` |
| `target/wasm32-unknown-unknown/release/ic_blob_storage_canister.wasm` | `198768fbd69e49ef0082d7da9a489e0be2c53294b4c6ab189e1273d672f53418` |
| `target/wasm32-unknown-unknown/release/blob_gateway_source.wasm` | `1c4c3fed02b870e459f163a36674b8c0b6375af0f1ba3b7ab6c7c8422c3bdfe2` |
| `Cargo.lock` | `1c6da13ee8d047778680358bbcad6fba19c4f65205bf51b74daadba3f21e0908` |

## Passive funding preparation assessment — 2026-10-01

This extends the existing unreleased gateway batch after maintainer release 0.4.12
(`468b871077ac2dcf2fcf0d8d7370795887d2332e`). No version, commit, tag/push,
publication, dependency/allocator or paid-effect change occurred. The service adds
a compatible passive query; stable configuration/state generations remain v1.

Shared `workflow::funding::assessment::inspect` delegates to the existing preparation
policy with authenticated installed scope and actual local journal facts. It
reports local unresolved allocation, retained/stale identity, lifetime capacity,
attachment allowance and fencing independently of missing provider qualification,
recovery, complete external account activity and spendability. These hosts have
no production acquisition path for that evidence; ingress cannot assert it. No
intent allocation, reservation, provider call or payment occurs, and preparation/
dispatch remain unexposed. The query cannot establish fresh identities or become
cached authority. Shared ops projects maintained funding totals for both status
and assessment without a workflow dependency or whole-history scan.

Native `funding-assessment` makes one signed query with exact scoped request echo,
positive full-width operation/offer/exact optional target, thirty-second deadline,
256 KiB HTTP and 4 KiB Candid limits. Decoder work/types/header/skipping remain
bounded. Changed echo, malformed/oversized or inconsistent blockers and typed
refusals reject; output always denies preparation/dispatch/retry authority and
keeps provider credit/spendability unestablished. No automatic retry or new journal
is added. Existing outcome inspection remains a separate historical capability.

Three core cases exercise full-width requests, four retained accepted transfers at
capacity, changed original arguments, uncertain reservations, stale identity,
authority and fenced reopen with unchanged memories/totals. These transfer facts
are synthetic model bookkeeping, not provider credit. Three native cases exercise
optional target, exact echo, malformed/oversized/inconsistent reports and fenced
full-history diagnosis. All 28 affected funding units pass in 0.05 seconds; all
42 native units pass in 0.64 seconds, including prior mutation/HTTP regressions.

The shared signed subprocess journey runs through rebuilt standalone and
supported managed artifacts beside a prepared 10 MiB and ten-byte upload,
respectively. Cashier is stopped before all queries and stays stopped through
restoration. Each journey retains 21 CLI invocations within its pre-effect budget
of 24, proving full-width and absent-target reports, actual actor/namespace/
Cashier/payer refusals, rejected caller qualification assertions, invalid IDs,
untrusted replies and same-release fenced inspection. Complete service and
Cashier stable bytes stay unchanged through each phase; funding history remains
empty and no synthetic service funding mutation hook is introduced. These are
local platform/substitute observations, not deployed provider qualification.

| Targeted validation | Result |
| --- | --- |
| Shared standalone signed funding assessment | Pass, 6.19 seconds |
| Shared managed signed funding assessment | Pass, 10.06 seconds |
| Existing standalone signed status/trust/restore | Pass, 6.63 seconds |
| Existing managed signed status/trust/restore | Pass, 10.89 seconds |
| Exact managed/standalone Candid types and modes | Pass, 0.02 seconds |
| Native CLI / standalone Wasm / supported managed build | Pass, 10.58 / 23.67 / 52.12 seconds |
| Final strict affected root / isolated managed Clippy | Pass, 11.41 / 6.72 seconds |

Both declarations cover the same thirty implemented blob methods; managed
certificate issuance, its plain-record Fleet rejection and equivalent query
decoder controls remain separate framework gates. The standalone declaration was
regenerated from its current exporter, including maintained documentation. No
full CI/release validation, resource benchmark or upstream refresh ran.

Fresh evidence is `/tmp/ic-blob-storage-funding-assessment-evidence-01`: 44 files per adapter plus
22 validation files, 110 manifested files total. Plans/public roots, command/
result JSON, exact commands, input/source/artifact hashes and every failed/successful
validation log remain. The immutable `SHA256SUMS` hash is
`f21accdda21a2d987624aea8e6515752af65855b919270f1168c179327c300fb`. No PEM is retained.
Initial core/harness compilation and lint mistakes (journal open name, projection
passing/import/layering, test module path/literal formatting) and both unsupported
identity-constructor attempts are retained; the existing raw-key test API resolves
the latter without a dependency or production feature change. Neither failed
constructor attempt started a signed journey or overwrote its evidence.

Owned local gateways/progress/instances and temporary identity files were cleaned
up; capture/build artifacts stay retained. Zero Cashier queries, deployed Caffeine
requests, payments/provider cycle attachments and no external cleanup obligation.
See the [ledger](caffeine-probes/README.md) and
[operator guide](../operator-guide.md#passive-funding-assessment). Qualified funded
workflows need trusted complete production evidence acquisition and safe recovery;
local clearance or balances cannot supply those facts. Consumer/Fleet provenance,
operational restart, IcyDB composition and removal/retirement remain open.

Current exact source/artifact inputs (earlier captures are historical):

| Input | SHA-256 |
| --- | --- |
| `Cargo.toml` | `482ec2708cfd20c9ed87afe8441078788c24b3bc6bc3da9b3bcbfd52f3ef6277` |
| `Cargo.lock` | `1c6da13ee8d047778680358bbcad6fba19c4f65205bf51b74daadba3f21e0908` |
| `crates/ic-blob-storage/src/dto/funding/assessment/mod.rs` | `a18ede5878811e7bfc2a4980f0e8ae9fb43c2c583166434990dbe3e57d3530fc` |
| `crates/ic-blob-storage/src/ops/service/funding/assessment/mod.rs` | `d5cfd637aaa98f83739a7953af24f0dee081d2a8c99a890d363e431ef6c4351b` |
| `crates/ic-blob-storage/src/workflow/funding/assessment/mod.rs` | `820135218cbb1495e715df9be836eaef3966da73bcdebd3fccd67e953c8482e1` |
| `crates/ic-blob-storage/src/workflow/funding/mod.rs` | `e678cc8c601a34bc9c69bb688e80cb9b90290fea6d563ab3de7da8d15026976a` |
| `crates/ic-blob-storage/src/ops/service/operator/mod.rs` | `7c1cba5efb5c542cd3430a0312d92af1c404d94b2726fea0372c354abe91d794` |
| `crates/ic-blob-storage/src/ops/service/funding/tests/assessment/mod.rs` | `bd8bc60fda2790eb27ef42a6619225227bcf5581349233ba7ddc4172578adc9b` |
| `crates/ic-blob-storage-cli/src/native/funding_assessment/mod.rs` | `18a07dc7157e3e16f19deba4ede64d9948dc5ad011355ed58b52e42e888ba05f` |
| `crates/ic-blob-storage-cli/src/native/funding_assessment/tests/mod.rs` | `0d4c93d7f7de1f0888d894172806e13013c32833ee1b494ae4808d8102108afb` |
| `crates/ic-blob-storage-cli/src/native/arguments/mod.rs` | `35b1c8ad9818ff17f972ad93f0c824d731443062225497d9ae9a29be12ded911` |
| `crates/ic-blob-storage-cli/src/native/reply/mod.rs` | `551fa50e8e64f697f14c1083b1986a3861095a441a63fe7b115cbcbae0c95071` |
| `canisters/standalone/src/api/mod.rs` | `c4d769f15f297da4f30eedfa36e179844194a080ff249fad6ac40794a9e5ab70` |
| `canisters/standalone/src/workflow/mod.rs` | `83b6035a84af149aefa920160ac506352adb80eb732d1e0a5f3401c802bfbfd6` |
| `canisters/standalone/service.did` | `88b8cf28e504a873bba5bf691837719ba03431cb332aad8bba275355dcfab882` |
| `canisters/test/canic_probe/src/api/mod.rs` | `3fc2216c5fd52657962c130210d097da0839dc6f95df4d0f325f19d105db47ec` |
| `canisters/test/canic_probe/src/ops/mod.rs` | `16a2fec5c78ad452f9a08613b44033e9085ba20800f180cca858325d1d087e90` |
| `canisters/test/canic_probe/canic.toml` | `63b4ee017ee33071f7f7d55b01e42b21cb77a0cdf670fb04ea93906c1e2ccc61` |
| `tests/pocketic/tests/funding_assessment_cli/mod.rs` | `79a8150093bda4a8fe1b10f6dfe7e917286647026e52a091b3e5336bc486d9aa` |
| `tests/pocketic/tests/standalone_gateways/funding_assessment/mod.rs` | `7e7de9d049a27c294973985b84c2fd636b9ba7d017f3106adb63024027ddc9af` |
| `tests/pocketic/tests/canic_managed/operator/funding_assessment/mod.rs` | `5d2c8d49ee8a13632f615acee26fedc661cc1305ddec9c17635fe9cf52d7c347` |
| `tests/pocketic/tests/canic_managed/declaration/mod.rs` | `8fadbdd6236a137290102fa3e1449905837c24fc5c086711665eb2eb0cb84e70` |
| `target/debug/blob-storage` | `df49a844074113025f594695eca46b6d8af0e4be0a22d6228621ce86a5a8a910` |
| `target/wasm32-unknown-unknown/release/ic_blob_storage_canister.wasm` | `dbaba86076cc85d64271cb31f4fa5c03fd3917b6700b90a49a4fb647fa0554b5` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.wasm` | `4d665991604cb240a28bd226ac1fd71e710fa6097249806306394c2356aeb42a` |
| `.tmp/canic-probe/.icp/local/canisters/storage/storage.did` | `a6607bf95a82b3213827efcae65892fc865cbe01be3e7dd8acc049f23c9aa889` |
| `target/wasm32-unknown-unknown/release/blob_gateway_source.wasm` | `1c4c3fed02b870e459f163a36674b8c0b6375af0f1ba3b7ab6c7c8422c3bdfe2` |

## Verified native tenant downloads — 2026-10-01

The maintainer released 0.4.13 during this continuation at
`842fa41664c74a2ae8c2fe27f88460e508be0edf`, direct-parent source
`4b68d2524505a2ae243e01125d22cd9e306abd83`. This new Unreleased tooling batch
adds no service endpoint/schema, dependency, allocator, journal or lifecycle hook.
Native tenant `download` authenticates one replicated descriptor for the exact
live reference and installed expected project, then fetches one explicitly bounded
provider body. It reuses the canonical core path and verifier HTTP implementation;
the old observation-only module is removed. Original service headers, declared
length and root own verification. Private partial bytes become a synced `body.bin`
only after complete EOF/root verification, without replacement. See
[usage](../operator-guide.md#download-a-verified-file) for commands and limits.

The successful managed journey admits/prepares using the existing fixture exposure
substitute, observes the ten-byte local HTTP source as an independently configured
verifier, submits the exact statement once, then downloads as the actual tenant.
It covers exact scope/trust, existing-output refusal, corrupt/short/long/redirect
responses, last reference release and fenced same-release restoration. Downloads
and refusal phases preserve complete service stable bytes. Fresh descriptor refusal
cannot recall an earlier verified file. Standalone proves actual signed unconfirmed
and restored refusal with no GET, without introducing a completion/evidence hook.
Existing signed verifier acceptance and lost/pending-reply recovery still pass
through the shared artifact/verifier refactor. These are local platform and labelled
exposure/content observations, not deployed Caffeine upload/availability evidence.

| Targeted validation | Result |
| --- | --- |
| Native CLI units, including streaming socket and private output cuts | Pass, 45 cases, 0.62 seconds |
| Signed managed completion-to-file/release/restore | Pass, 12.80 seconds |
| Signed standalone unconfirmed/restored refusal | Pass, 5.15 seconds |
| Existing managed verifier acceptance/lost/pending recovery | Pass, three cases, 12.06 seconds |
| CLI / standalone / supported managed builds | Pass, 3.10 / 20.45 / 53.19 seconds |
| Final strict affected CLI/harness Clippy | Pass, 0.69 seconds |

Fresh `/tmp/ic-blob-storage-tenant-download-evidence-01`, `-02` and `-03`
retain 12, 31 and 156 manifested files. First failed setup redundantly repeated
already-recorded fixture exposure (zero GETs). Second expected the wrong submission
label after accepted completion (one GET). Their failed-attempt records preserve
then-current harness/CLI/Wasm hashes; neither is overwritten or relabelled passing.
Final 03 retains exact commands, seventeen source/artifact bindings, every
compile/lint/test development log, raw descriptor/request/source/body captures,
public trust and summaries. Successful managed uses fourteen CLI invocations/six
GETs; standalone two/zero within pre-effect plans. Existing verifier regressions
use three additional local GETs; their raw captures are temporary, with the test
log retained. Initial compile/lint mistakes, including helper extraction and
fixture receipt-field errors, remain development history rather than final coverage.

| Capture manifest | SHA-256 |
| --- | --- |
| `evidence-01/SHA256SUMS` | `97a3fe1fc9aaad57559d6e0bf922f383b56119073533ae80f1f6a7b281c004d5` |
| `evidence-02/SHA256SUMS` | `706a035570794820f14957aeb6c8fe56569935aa05cca190f2f1778fca277039` |
| `evidence-03/SHA256SUMS` | `ae5c84d948d6cce3f2f616f4395ea0f3d1b7f82e2577e38af92879fe8883ed78` |

Important final source/artifact bindings (all seventeen are in 03's
`validation/inputs.sha256`; earlier dated captures remain historical):

| Input | SHA-256 |
| --- | --- |
| `native/download/mod.rs` | `ff65ce70a660610d0a6e969c47f52492f71e2ddf2b93cd5568107763c5f9de6a` |
| `native/provider_download/mod.rs` | `7bba79dcd5f14108466a7982c0030232e4c134b0256933b4036c2a2b7ba700e2` |
| `native/artifacts/mod.rs` | `045898f91fa19e6a5c0f5abfadf437e74c5bac4ac060f630e1440a6fad391afb` |
| Managed download journey | `30d51023f4c225956bc6296142e2eb4141a64d721ca24762bcffc71a349cb2e5` |
| Standalone download journey | `0476759c4c7af5e40cd7adabfc6104cf5cf4702f408e1885e5c0e72602571ec7` |
| `target/debug/blob-storage` | `4a13a42fe1f94e9a10b1e22e8c968059c3bb2819ae562d8c1b3eeecc6aa32b79` |
| Standalone Wasm | `de8f589db237d49961f6cd1983defe500701fad37aead3c51c71bc10fb6138ee` |
| Supported managed Wasm | `979229dea10ce040871ba62cd4e00bff38db9a30dd77ae09ddd8f1b4b32ebbc5` |

Owned sockets/gateways/progress/instances and temporary PEM were dropped; build
artifacts and evidence remain. Zero deployed Caffeine/Cashier requests, payments
or provider cycle attachments; no external cleanup. No provider/upstream source
refresh, full CI/release validation or resource benchmark ran. Existing issuance
provider/recovery prerequisites and managed plain-record rejection/decoder hooks
remain open, as do production consumer/Fleet/verifier provenance, operational
restart and removal/retirement. Next prioritize the first real upload-to-download
journey, respecting the [ledger](caffeine-probes/README.md)'s target/budget authority.

## Signed native upload setup — 2026-10-01

This continues Unreleased after maintainer release 0.4.13 at
`842fa41664c74a2ae8c2fe27f88460e508be0edf`. Native `admit-upload`,
`prepare-upload`, `revoke-upload`, `upload-permission` and `upload-manifest`
use the existing DTOs, handlers and exact bounded decoders. The same pure
permission/declaration preflight validators are public for native/canister reuse;
all internal consumers use their maintained names, with no alias or fallback.
No new endpoint/schema, dependency, allocator, journal or host qualification fact.

One fresh private mutation run saves canonical request/permission copy, signed
envelope and intent before one update. Pending/refused/uncertain results remain
separate; exact historical query recovery never repeats the mutation or changes
its original artifacts. Tenant admission/cancellation and uploader preparation
remain distinct. Preparation validates original metadata, length, ordered leaves
and root without file transfer, certificate issuance or provider effects.

The same subprocess journey runs through standalone (10 MiB declaration) and
supported managed (ten-byte declaration) endpoints. Wrong roles/PEM, unknown
preparation, dropped admission, pending preparation, corrupt declaration before
claim, changed expiry, exact historical recovery and cancellation pass. Cancellation
releases all reserved/logical/physical/liability bytes while retaining lifetime
operation/manifest history. Restored queries preserve original cancelled records;
mutations refuse and complete stable bytes remain unchanged. No exposure/completion
fixture hook is used. Both production hosts still report the four independent
certificate prerequisites; local setup never overrides their evidence.

| Targeted check | Result |
| --- | --- |
| Native CLI units | Pass, 47 cases, 0.66 seconds |
| Admission reply and manifest client core units | Pass, four cases |
| Standalone signed setup/recovery/cancellation/restore | Pass, 6.66 seconds |
| Managed signed setup/recovery/cancellation/restore | Pass, 7.74 seconds |
| CLI / standalone / supported managed builds | Pass, 2.81 / 19.47 / 43.44 seconds |
| Final strict affected core/CLI/harness Clippy | Pass, 0.76 seconds |

Captures `/tmp/ic-blob-storage-upload-setup-evidence-01` and `-02` retain 92
and 111 manifested files. Successful standalone is in 01; failed first managed
enrollment used a hard-coded operator unlike the configuration and stopped before
native initialization (3.10 seconds). That failure's source/artifact hashes and
log/result remain unchanged. Corrected managed uses fresh 02, which also retains
all development logs, exact commands and 23 final source/artifact bindings in
`validation/inputs.sha256`. Initial missed private-validator consumers and newly
public panic-documentation lint errors are retained, not final coverage. Each
successful host uses 21 CLI invocations within its recorded pre-effect budget;
proxies validate saved signed intent and forward each selected update exactly once.

| Capture manifest | SHA-256 |
| --- | --- |
| `evidence-01/SHA256SUMS` | `6b98b794ce42b0b521dc8971d1bd5d4801d2e00b322d5c80e304574b95b39fc5` |
| `evidence-02/SHA256SUMS` | `62eb1c9fc261d97908010f83459a76d5c434eb3b05fa393f6971502218b27c85` |
| Native CLI | `3ba1e603a134f2ec3dc79ff5887f512fec1ffadd010bd42ff4fdad986c1219c5` |
| Standalone Wasm | `36ad28cbe907f9e58085e73ab9f0cdc36fb29bbd4a08bfcd51aeaa6390e6c2ad` |
| Supported managed Wasm | `ff3831dcb05f22afc84cc1757e9f1fb5c2c34425dc8a6ce42c13f677826834f9` |

A read-only local Canic review at clean HEAD
`70a0bc9a435a7695d8931a7c66c745576e678597` compares access/expansion/parser
files byte-for-byte against pinned registry 0.110.48; all match. Hashes remain in
02's local-review log. Guarded plain-record replies are still compile-rejected;
query decoder work/byte hooks remain absent. This is local source review, not an
upstream-main/registry refresh. No sibling mutation or message occurred.

Owned proxies/gateways/progress/instances and temporary PEM were dropped; capture/
build artifacts retained. Zero provider GETs, deployed Caffeine/Cashier requests,
payments or provider attachments; no external cleanup. Only targeted checks ran,
without full CI/release gates or benchmarks. Existing provider/recovery qualification,
real certificate issuance, authenticated consumer/Fleet/verifier provenance,
operational recovery and removal/retirement remain open. See
[usage](../operator-guide.md#admit-and-prepare-an-upload) and the
[ledger](caffeine-probes/README.md). Earlier dated captures remain historical;
current rebuilt artifacts are bound above, never rotated into earlier records.

## Offline native upload inputs — 2026-10-01

Native `upload-inputs` converts bounded Caffeine `manifestJSON` plus an explicit
original binding into the existing `UploadAdmissionRequest` and
`UploadManifestRequest`. It delegates to the maintained upstream preparation
decoder and shared service validators, with no second hash/tree/chunk/transfer
implementation. Source DTOs, schema generations, authority and service endpoints
remain unchanged. Application-selected identity values remain unauthenticated and
unallocated; this does not check body bytes, current expiry or provider availability.

Four new native tests prove full-width canonical IDs/expiry and distinct roles,
exact maintained Candid output, preserved original metadata, changed root/leaves/
metadata refusal, bounded regular inputs, malformed bindings/options and no-clobber
repeat. All 51 native CLI tests pass in 0.61 seconds and three existing probe-tool
units pass in 0.20 seconds. Strict CLI all-target lint and binary build pass.
Initial compilation/lint failures and the sandbox-denied loopback regression run
are retained; the final authorized local-socket regression passes. No full CI,
release gate, platform lifecycle or provider experiment ran for this conversion.

The actual binary smoke uses the existing independent Caffeine 1.1.2 abc/text
vector, with max u128 namespace/upload and max u64 expiry. Three invocations produce
canonical request files (exit 0), refuse an existing run without changing any output
hash (exit 3), and refuse inconsistent Content-Length before output claim (exit 3).
This is local native/vector evidence, not an executed upstream SDK or deployed
provider qualification. Inputs and outputs remain private local preparation data;
partial runs are not resumed and local hashes do not authenticate mutable files.

Capture `/tmp/ic-blob-storage-upload-inputs-evidence-01` retains 30 manifested files,
including exact input/output JSON/Candid, outcomes, plan, commands, nine development/
validation logs and fourteen final source/artifact bindings. Its `SHA256SUMS` hash is
`698dce085460b310a545a048bcaca8a5e7a378f0c77f20c6c1de20ad6c572946`.
Zero service/provider requests in the smoke, payments or attachments; existing
regressions use only their local HTTP substitutes. Owned regression servers were
dropped; smoke owns no key, socket or external resource. Build and evidence retained.
See [usage](../operator-guide.md#generate-upload-inputs-offline) and
[ledger](caffeine-probes/README.md). Provider/recovery certificate gates and real
consumer upload/download acceptance remain open.

## Verified native upload snapshots — 2026-10-01

The unreleased offline `upload-inputs` now requires a body file and preserves the
exact buffers checked by the maintained Caffeine root verifier. Source length and
regular-file checks precede output claim; one open handle survives source-path
replacement. Copying uses 64 KiB frames, bounds growth and requires complete EOF/
original metadata/root before synced no-clobber `body.bin` publication. Request
files follow successful body verification, summary is last. Corrupt input leaves
private partial/failure evidence without published body or Candid requests. Saved
files remain mutable local data; this establishes no tenant, freshness, expiry,
provider-completion or future-retention authority. Signed `verify-upload` removes
its separate file loop and delegates to the same maintained local-file verifier.

All 54 CLI units and three existing probe-tool units pass in 0.61/0.21 seconds.
New cases cover private corruption residue/no request files, source growth/truncated
EOF and path replacement with the selected original open handle. Existing signed
local-file binding/authorization/corrupt-source cases still pass. CLI and harness
all-target/all-feature strict lint and the native binary build pass; formatting,
diff, read-only draft changelog, capability JSON and maintained doc links validate.
Only targeted checks ran, not a full CI/release gate or a provider experiment.

Both existing PocketIC setup journeys now consume the actual binary's generated
permission/manifest Candid and body snapshots, rather than Rust-created Candid
inputs. The standalone 10 MiB journey passes in 7.19 seconds and pinned managed
ten-byte journey in 8.86 seconds, with 23 CLI invocations per host. Intentional
source edits cannot change the snapshot checked against the signed original
service manifest. Existing lost/pending updates, exact recovery, corrupt declaration,
cancellation and unchanged complete stable state during fenced restore remain
covered. Every certificate assessment still reports the four real blockers. No
host qualification override, exposure or completion is injected. Manifest JSON is
an explicit local upstream-format substitute derived from the Rust fixture; it
does not establish a fresh SDK or deployed provider behavior.

Three additional actual binary smoke cases retain corrupt abc/text-vector source
(exit 3), repaired-source refusal of the same failed directory (exit 3, all partial
hashes unchanged) and successful fresh verified snapshot (exit 0). The raw digest
of successful abc is the independent standard SHA-256 value. No service/provider
request occurs in these smoke commands. Successful host journeys make no provider
requests/GETs/payments/attachments; provider facts remain unknown.

Capture `/tmp/ic-blob-storage-upload-snapshot-evidence-01` retains 230 manifested
files: both host commands/results/intents/snapshots, smoke outcomes/partial bytes,
exact command record, seven validation logs and 22 final source/artifact bindings.
`SHA256SUMS` hashes to
`d4c84d1606e21c8eda231b3aa5bc4f1718a3073da1494b8eeb6c34f2ba8358b6`.
Owned sockets/proxies/progress/instances/temporary PEMs dropped; build/evidence
retained and no external cleanup. Earlier immutable captures retain their original
contract and hashes. See [usage](../operator-guide.md#generate-upload-inputs-offline)
and [ledger](caffeine-probes/README.md). Qualified issuance and the real consumer
upload/download journey remain the next delivery gates.

## Offline reference and download inputs — 2026-10-01

The unreleased `upload-inputs` now emits first-reference download/status Candid
after successful body verification. Offline `reference-inputs` accepts the exact
saved permission plus explicit canonical positive reference/operation IDs and a
retain/release action. Shared core validation and Candid preserve original upload,
tenant, object/lifetime, root and byte binding. It saves permission/command/read
files and hashes before summary in a fresh private directory. It allocates no
identity, authenticates nobody, renews no expiry and grants no liveness or retry
authority. Expired original permissions remain representable for cleanup.

Three new CLI cases cover full-width retain/release output, expired cleanup
binding, exact read requests, wrong schema/invalid permission, canonical IDs,
bounded inputs/options and no-clobber partial/repeat refusal. Existing upload-input
tests also assert first-reference output and no read files after corrupt snapshots.
All 57 CLI and three probe-tool units pass in 0.63/0.22 seconds; native binary
build and final affected all-target/all-feature strict lint pass. The first lint
attempt rejected constant `chunks_exact` in a fixture helper; its retained log
precedes the passing maintained `as_chunks` implementation.

The existing standalone 10 MiB and pinned managed ten-byte setup journeys pass
in 7.61/8.47 seconds, with 25 CLI invocations each. Actual generated first-reference
status/download requests refuse unconfirmed content without a source GET. Their
lost/pending recovery, source-edit verification, cancellation, accounting and
same-release fencing remain intact; all four issuance blockers remain present.
The managed completion-to-download journey passes in 11.92 seconds, 21 total CLI
invocations/six bounded local source GETs. Generated reference inputs replace its
hand-built download input and raw fixture release dispatch. Signed release records
success, exact receipt is retained and status changes from live to inactive;
released download refuses without GET. Same-release restore preserves receipt,
inactive/fenced status and all stable bytes, while the already downloaded file
remains usable. Logical release does not prove deletion or billing cessation.
Existing exposure/content are labelled local substitutes, not deployed evidence.

Three offline binary smoke invocations use a copied historical abc/text permission:
full-width retain inputs (exit 0), refused repeat (exit 3) and noncanonical reference
refusal before claim (exit 2). The smoke retains exact commands/results/exits; unit
coverage separately checks unchanged command bytes on refused repeat. No service
or provider request occurs in these smoke calls. Fresh capture
`/tmp/ic-blob-storage-reference-inputs-evidence-01` retains original commands,
results, Candid, snapshots, HTTP substitute bodies, signed intents, validation logs
and 31 final source/artifact bindings. Its 389 manifested files have `SHA256SUMS`
hash `7fdb41be26da8b3e97e788942bdf5ad08acdbffacd2365b0186cdaab2acb6a31`.
The transient missing-anchor documentation check is retained alongside the final
passing check. Only targeted checks ran, no full CI/release
gate, SDK/upstream refresh or provider experiment. Owned local servers/proxies/
progress/instances/temporary PEMs dropped; build/evidence retained. No deployed
provider request, payment, attachment or external cleanup. Earlier captures remain
immutable. See [usage](../operator-guide.md#generate-reference-inputs-offline) and
[ledger](caffeine-probes/README.md); real issuance and consumer delivery remain open.

## Shared native reference downloads — 2026-10-01

The existing managed native download journey now covers two references to one
confirmed blob within the same tenant. Generated retain inputs feed one real
signed update through the local drop proxy, which verifies saved request/signature/
intent before forwarding. Transport remains uncertain; exact signed-query receipt
finds its historical success and a separate status finds current liveness. A
same-run submit refuses. Original uncertain artifacts remain byte-for-byte intact
through queries, cleanup and same-release restore; no automatic redispatch.

First-reference release refuses its download while the generated second-reference
request still retrieves the identical root-verified ten-byte file. Local logical,
physical and liability bytes remain ten. Generated final release stops both
references from downloading and changes logical bytes to zero; ten physical and
ten liability bytes remain. Signed cleanup establishes neither provider deletion
nor billing cessation. Fenced restore preserves inactive status, original success
receipts, both files and the owner's complete stable bytes. A fresh signed mutation
attempt and both restored download requests refuse; old retain success does not
reactivate the reference. No new production API, journal, allocator or SDK.

The extended journey passes in 14.16 seconds: 37 total CLI invocations, seven
bounded local source GETs and one forwarded retain update, within the planned
forty tenant/two verifier invocation budget. The expected/client body bound is ten
bytes; existing fault replies include zero/nine/eleven bytes, sixty source content
bytes in total and no oversized published file. Targeted managed-harness all-feature
strict lint passes. Production CLI/core/host code is unchanged; prior 57 CLI units
were not repeated. Formatting, diff and read-only changelog checks pass. Only
targeted checks ran, not full CI or a release gate. Existing exposure/content remain
labelled local substitutes, not deployed Caffeine or production-consumer evidence.

Fresh `/tmp/ic-blob-storage-shared-download-evidence-01` retains commands/results,
generated Candid, signed uncertain intents, accounting snapshots, downloaded files,
HTTP substitute bodies, validation logs and final source/artifact bindings.
The capture contains 226 manifested files and 32 source/artifact bindings;
`SHA256SUMS` hashes to
`4f224d1e0cfb75373dad826411919c3dd84372c7540e98d78122267959c055ca`.
Owned temporary keys/proxy/gateway/progress/instances dropped; build/capture retained.
Zero deployed provider requests, payments or attachments and no external cleanup.
Earlier capture manifests remain immutable. See
[usage](../operator-guide.md#share-a-confirmed-blob-within-a-tenant) and
[ledger](caffeine-probes/README.md). Certificate issuance/provider recovery and
real consumer upload/download delivery remain open.

## Native browser certificate binding — 2026-10-01

Verified `upload-inputs` exports the existing browser client's binding directly
from its maintained Rust permission: exact service/tenant/uploader, upload ID as
operation, root/key and opaque Candid bytes. No JS reconstruction of service DTOs,
identity allocation, credential, certificate or new provider format. Encoding
precedes claim; binding follows complete body verification/publication and its
hash is in summary. Corrupt body produces no binding. The private snapshot helper
retains the previous same-buffer EOF/root, failure, sync and no-clobber semantics.

The real hash-checked pinned Caffeine 1.1.2 package with the maintained patch
prepares a 10 MiB ten-chunk file. Native conversion verifies/saves it and the
existing browser source client in Node accepts the generated full-width binding
and permission. The setup-only in-memory store and deliberately non-IC trust
bytes are labelled substitutes: no signature, certificate, IC endpoint, durable
store or browser platform behavior is exercised. All network calls throw.
After changing the original source, SDK preparation from the snapshot and original
metadata reproduces exact root/length/manifest. A repeat refuses with every file
hash unchanged; corrupt source retains only private partial/failure, no usable
body/Candid/browser binding/summary. Three native invocations return 0/3/3; two
SDK preparations and zero network requests. No provider account or paid effect.

All five affected native cases pass in 0.04 seconds; final CLI all-target strict
lint, native binary and SDK bundle build pass. The first run-length lint failure
and both pre-/post-refactor unit/build logs are retained. The new opt-in Make
recipe is inspected without repeating the capture. Only targeted checks ran;
no full CI/release gate, Chromium/PocketIC rerun or upstream SDK refresh. Fresh
`/tmp/ic-blob-storage-browser-handoff-evidence-01` retains actual manifests,
bindings, snapshots/partial bytes, commands/outcomes and validation/source/package
bindings. The known fixture identity lived in memory only; no private key file,
network resource, account/object or external cleanup. Build/evidence are retained.
The capture has 38 manifested files and 26 source/artifact/package bindings;
`SHA256SUMS` hashes to
`657361e996f6cfc78c284d370714d277d33815fb170762f7d855c894f8517a4d`.

Read-only local Canic review now finds clean committed public hooks at
`c36a0edf8518e0f0793dad13e0908ffc3394aeae`; its handoff reports focused generic
qualification passes, not rerun here. Local Cargo still says 0.110.48. The web
metadata lookup was unavailable; latest registry version remains unverified.
This is source/handoff review, not published dependency/downstream qualification.
Our pin and host prerequisites remain unchanged; adoption awaits supported
published hooks. See [usage](../operator-guide.md#generate-upload-inputs-offline),
[client](../../clients/browser/README.md) and [ledger](caffeine-probes/README.md).

## Managed certificate and decoding — 2026-10-01

The maintainer selected local Canic development and persistent feedback reminders.
The managed fixture now uses public rejection/decoder/lifecycle hooks from frozen
Canic `32da629d0214bf791541a9b3c1832dbef13ece29`, through the canonical Canic CLI
build. Both copied workspaces patch the three Canic packages; main manifests,
lockfile and release receipt are unchanged. The independent core has no Canic
production dependency. Old capture `.tmp/local-canic-01` remains immutable;
new intent/sources/locks/artifact/logs are retained in `.tmp/local-canic-02`.
The initial canonical build takes 49.38 seconds. After syncing the library's
documentation title, a 19.92-second rebuild produces byte-identical Wasm and
Candid; exact pre-/post-rebuild hashes bind the executed tests to the final artifact.

The exact `_immutableObjectStorageCreateCertificate` update has a plain record
Candid result matching standalone. It calls the same synchronous root resolution/
issuance workflow and actual actor binding, with no internal dispatch workaround
or qualification override. Actual IC issuance refuses unprepared/blocked/revoked/
restored uploads and wrong actors, retaining stable bytes and local reservation
accounting. There is no observed successful certificate reply or deployed transfer.

All blob queries/updates select five-dimensional decoding limits. Actual malformed,
oversized, excessive-type and excessive-header requests reject before mutation;
the latter two are valid Candid under ordinary decoding. An exact 4 KiB envelope
reaches shared root validation. Existing manifest evidence retains the 128 KiB
transport boundary, smaller semantic limits, ingress and inter-canister overflow
refusal, tenant authority and fenced restore. Initial carrier limits precede
participants; invalid installation and oversized upgrade preserve occupied owners,
neighboring memory and tenant state through whole-transaction rollback.

Six focused cases pass: two certificate cases (16.63 seconds), two lifecycle cases
(21.02 seconds), manifest/authority/restore (8.54 seconds), and exact Candid types
(0.02 seconds). Fixture all-target/all-feature strict lint and the affected native
harness strict lint pass. The initial test compile used the wrong testkit rejection
accessor; its failure is retained, and the corrected assertion uses the structured
rejection response. No error-text assertions or production test overrides were added.
Only targeted checks ran, not full CI/release validation or provider experiments.

The four certificate prerequisites remain false. Canic publication/pin adoption
for normal workspace/release checks is CF-01;
local development continues without a live Fleet deployment. AGENTS.md requires
maintained feedback and delivery reminders. No Canic defect was established by
the downstream checks; provider economics/recovery are separate service work.
Sibling repositories stay read-only; instances drop and artifacts remain, with
no paid effect, deployment, commit/version/tag/publication or external cleanup.
The new capture's `SHA256SUMS` is
`2177a07389e98b0a48405eb27bb6a7e04a54b110ff4c0b6023a232d2bd6cd172`;
the previous local-source manifest still verifies. Current code matches its copied
counterpart, final links/JSON/formatting/diff checks pass, and owned local PocketIC
processes have exited.

## Published Canic adoption — 2026-10-01

The maintainer reports Canic 0.110.49 published while requesting the 0.4.15 notes.
Both direct workspace pins now select registry 0.110.49; metadata resolves `canic`,
`canic-core` and `canic-macros` to that version without local overrides. Cached
registry archives match their lockfile checksums and identify source commit
`75c7f0998fd4531f8a6b81d91b53b71daffca332`. The required public rejection and
endpoint/lifecycle decoding hooks are present. External crates.io/docs.rs page
lookups failed; this establishes the selected cached package identity, not a
claim about the registry's latest version. The maintainer's staged core/macros and
lazy_static updates remain unchanged; our additional lockfile change selects the
Canic facade. Product version and release receipt remain 0.4.14.

Canonical `make build-canic-probe` succeeds in 101.05 seconds using installed
Canic CLI 0.110.48, registry libraries 0.110.49, offline Cargo and this repository's
target. The resulting Wasm hashes to
`11eb4861a467215e530ef86dd07afed784c67caf79be5d4731a8d36fa00cafee`;
its Candid hashes to
`7daefe99d6779f9994b070f13d1a8cbf20d64c1c04f6e4834b261ce2f468839f`.
The same six focused cases pass: certificate authority/refusal and hostile decoding
(16.76 seconds), invalid installation and occupied oversized-upgrade rollback
(21.80 seconds), manifest bounds/authority/fenced restore (8.50 seconds) and all
blob Candid types/modes (0.01 seconds). These retain missing provider prerequisites;
there is no successful certificate exposure or deployed transfer.
Fixture all-target/all-feature strict lint passes in 35.51 seconds; the affected
native harness strict lint passes in 10.20 seconds.

The first harness compiles but PocketIC cannot bind loopback inside the sandbox.
Only its identified stalled test process is terminated; the refusal log is retained
separately. A loopback-enabled repeat passes. Pre-run intent, commands, package/VCS
identities, both resolved locks, metadata, source/artifact hashes, copied Wasm/Candid
and separate attempt/result logs are retained in `.tmp/published-canic-49-01`.
This closes package adoption CF-01.
No full CI/release gate, deployed Fleet, provider request, payment, sibling edit,
upstream message, product version/receipt mutation, commit/tag/publication or
cleanup. Earlier frozen-source evidence remains historical and unchanged.

## Managed browser setup — 2026-10-01

The maintainer released 0.4.15; baseline main is `d988835`. New opt-in
`make test-canic-browser` connects the existing browser client and maintained
patched Caffeine 1.1.2 SDK to the actual managed host, not the earlier successful
certificate fixture. Inputs are registry Canic 0.110.49, SDK 5.4.0, Node 24.21.0,
Chromium 153.0.8010.12 and PocketIC 16.0.0. Production source/dependencies remain
unchanged. Existing service/consumer release artifacts are used; the new Make
recipe is inspected, not rerun as a redundant artifact build. The pinned bundle
build checks upstream source hashes and applies the maintained patch.

Rust independently derives the ten-byte body root and validates the actual SDK
manifest. Rust-owned opaque Candid binds the consumer tenant, uploader, permission,
independent full-width identities and prepared declaration. Actual signed browser
admission/preparation succeed; foreign admission/preparation and direct uploader
tenant admission return typed denial. No second SDK, DTO schema or service workflow
is introduced. The shared bounded Node control owner also serves earlier scenarios.

Two tabs claim exactly one certificate update. The current production host reports
the four pre-charge/namespace/replay/recovery blockers and refuses issuance without
changing service stable bytes. The browser retains the exact uncertain envelope
and request ID through reload and certified rejection inspection, without resend.
Gateway authority refuses and observed gateway requests remain zero. Explicit
browser cancellation and consumer withdrawal release the unexposed reservation:
reserved/global logical/physical/liability bytes change from ten to zero, while
the original cancelled/revoked permission and operation remain. Same-release
upgrade retains this history with the service mutation fence set.

Initial new-harness assumptions fail before certificate grant: the consumer already
reports Reserved after admission, and global physical/liability capacity includes
reservations. Separate first/second failure logs and correction records are retained;
neither attempt issues a certificate or sends gateway traffic. Corrected managed
Chromium passes in 9.25 seconds; all ten earlier browser scenarios pass in 54.56
seconds after the shared helper refactor. Strict lint for the managed/storage harness
targets passes in 11.24 seconds; formatting and both changed JavaScript syntax checks
pass. No full CI/release gate is run. Existing scenarios still use labelled local
certificate/exposure/gateway substitutes and do not qualify deployed Caffeine.

Pre-run intent, source/artifact/package identities, commands, separate logs and
browser request/intent reports remain in `.tmp/managed-browser-01`; previous captures
are unchanged. All traffic is confined to owned local page/PocketIC origins, with
zero deployed provider/payment budget. Local processes/instances exit and build
artifacts remain; no external object, balance or cleanup obligation is created.
There is no successful managed certificate exposure, upload/download transfer,
production browser store/recovery qualification or deployed Fleet evidence.
CF-01 remains closed and no new actionable
Canic finding is established. Siblings remain read-only; no version/release/commit/
tag/publication/deployment or Cargo cleanup occurs. See the
[ledger](caffeine-probes/README.md) and setup.

## Shared browser upload transfer — 2026-10-01

After maintainer release 0.4.16 (`d8e20ce`), the private browser package exports
`createUploadTransfer`. It composes the maintained patched Caffeine 1.1.2 SDK with
the existing certificate client/gateway journal. The original certificate binding
supplies owner/root; explicit project/bucket and traffic bounds, fixed serial
transfer and disabled retries replace duplicated SDK configuration. Wrong roots
and malformed namespace values refuse before certificate dispatch. The exposed
transport is the SDK's same guarded session, not a second provider request owner.
Preparation, wire formats, chunking and handle consumption remain Caffeine-owned.
No production store, public Rust contract or qualification override is added.

Six existing SDK fault cases use the helper and pass, retaining exact emitted
owner/project/bucket, multiple chunks, HTTP/aggregate-budget refusals, lost-final
uncertainty and independent native verification of substitute bytes. SDK success
without complete status remains explicitly unqualified. Raw local request/result
records are retained under
`caffeine-probes/local/2026-10-01-sdk-transfer-01`; certificates, gateway and store
are labelled substitutes. The actual managed Chromium refusal journey passes
in 9.35 seconds, retaining all four blockers and zero gateway requests. All ten
existing Chromium scenarios pass in 53.90 seconds. The first browser regression
fails at the fixture's post-tree abort: manual composition had moved cancellation
before complete response reading. Its separate failure/correction records remain
in `.tmp/upload-gates-01`; the corrected fault occurs after observation commits.
The maintained guard semantics are unchanged. Node syntax and pinned bundle/peer
checks pass; verifier build is targeted, not full CI/release validation.

Fresh bounded anonymous public-source capture observes skills
`4ebf43c518c9fc7b090ed4d27677fea26d6b98f3`, unchanged Mixin/Storage hashes and npm
1.1.2 with retained integrity. The initial transport failure is preserved separately
from the four-GET successful capture; its reused probe binary reports 0.4.15 and is
not relabelled. A pinned April-guide lookup is unavailable. No Mops/deployed server,
account or gateway behavior is refreshed. The [trial-gate review](caffeine-upload-gates.json)
distinguishes these public observations and local controls from unproved limits,
namespace, replay charging and operational recovery. A proposed different economic
operating contract is a review item requiring a minor semantic change, not current
host qualification. No selected trial owner/payer/financial budget is inferred.

Intent, commands, package/source/artifact identities and separate outcomes remain
in `.tmp/upload-gates-01`; maintained raw public/local records are indexed by the
[ledger](caffeine-probes/README.md). Current hosts keep all four prerequisite facts
false. Browser/IC traffic remains local and substitutes create no external object,
payment, balance or cleanup obligation. Instances exit and artifacts remain.
Siblings stay read-only; no deploy/release/commit/tag/publication or Cargo cleanup.
No new actionable Canic finding is established; CF-01 remains closed.

## Offline account-link inputs and public Cashier review — 2026-10-01

The additive native `account-link-inputs` command uses one ops-owned Cashier wire
encoder. Explicit cashier/caller/paid-canister/payer and positive raw daily-limit/
expiry inputs produce complete Candid arguments and a hashed summary in a fresh
private directory. It allocates no identity, signs nothing and dispatches no
request. Units, freshness, caller authority and cap enforcement remain unproved;
the prepared file is not an effect journal or retry token. See the
[operator guide](../operator-guide.md#generate-account-link-inputs-offline).

Two codec tests match independently generated `didc` bytes against the maintained
Cashier declaration, including full-width values and explicit option fields.
Two CLI tests cover valid output, role/sentinel/overflow/noncanonical refusal,
missing/duplicate/unknown options and preserved existing/partial output. Actual
executable output independently decodes against freshly observed Cashier Candid;
repeat refuses with original artifacts preserved. Strict affected all-target lint,
formatting and warning-free core rustdoc pass. Initial lint failures remain in
separate logs. No new platform test or full CI/release gate runs for this offline
codec/tooling addition. Dependencies, production adapter facts and release files
are unchanged.

Separately, two anonymous mainnet native commands observe public Cashier metadata
and `pricelist_v1`; both replies match retained bytes. Their
[deployed public capture](caffeine-probes/deployed/2026-10-01-cashier-preflight-01/summary.json)
does not establish server source/revision, account relationships or enforcement.
Internal HTTP request/retry counts are unknown. A failed pinned-guide lookup is
retained; the historical retained guide's request pricing labels differ from live
factor `M`, without a justified conversion. No provider object/account/payment
effect or cleanup obligation is created. The maintainer confirms no existing trial
canister/account. All four certificate facts remain false; CF-01 remains closed
with no new actionable Canic finding. Siblings are read-only and no version,
commit, publication, deployment or build cleanup occurs.

Pre-run intent, independent fixture inputs, executable examples, command results,
source/binary hashes and separate logs remain in `.tmp/caffeine-trial-preflight-01`.
Prior captures stay immutable; the new public record is verified offline by
`probe-check`. Account selection/provisioning, exact financial terms and the
trial operating/recovery contract remain next work, with separate effect authority.

## ic-memory 0.15 adoption — 2026-10-01

The maintainer requests the published 0.15 dependency and confirms Canic adoption
is being handled separately. Core now requires `ic-memory` 0.15.0; its registry
archive SHA-256 matches lock checksum
`21a7b22c19fd5a35f074f7ecac0abb8544a4c84b0e925967ff5588d40a1d9c39`
and packaged VCS identity is `26306737cf75efb320bb9c4029dcd1d77bc83184`.
The exact stable-structures substrate remains 0.7.2. No other locked package
version changes; Canic still requires a separate 0.14.3 runtime.

Direct runtime growth now returns `Result<u64, RuntimeGrowError>`. Shared-grant
tests, consumer initialization and the managed neighbor fixture assert typed
success. The storage fault wrapper explicitly delegates `Memory::grow`, preserving
the upstream sentinel contract rather than recreating the error conversion.
Source review observes nonconstructing default opens, protected capacity
reservation, numeric allocation summaries and authority-scoped adoption checks.
No ic-memory defect is established by this limited integration review.

| Check | Result / scope |
| --- | --- |
| Core `memory_composition` | Both cases pass: linking does not claim memory; host grants preserve cells and bucket configuration. |
| Core `ops::service::stores` | All eleven cases pass, including incomplete-grant refusal without writes, composed owner/neighbor preservation, scope checks and restore fences. |
| Core `ops::service::installation` | All six cases pass, including occupied/missing state refusal and populated configuration/owner restoration. |
| Canic adapter native units | Both cases pass; absent runtime stays unconstructed and malformed carrier inputs refuse. |
| Release Wasm | Standalone, consumer and storage-probe builds pass with the new direct growth contract and generic trait wrapper. |
| Actual standalone lifecycle | Stop/start and repeated same-release upgrades preserve installation/history/reservations and all owner fences (6.78 seconds). Initial sandbox loopback refusal is retained separately; permitted retry passes. |
| Strict affected lint and formatting | Core, adapter, standalone, consumer and storage-probe all-target/all-feature lint passes; both workspace formatting checks pass. |
| Native managed fixture | Passes after refreshing its ignored lockfile from the root. The initial stale-lock refusal is retained separately. Native compilation does not qualify shared runtime identity. |
| Canonical managed artifact | CLI 0.110.49 refuses before build with `role_contract_multiple_memory_runtimes`: registry Canic/core 0.110.49 uses 0.14.3 while blob uses 0.15.0. No new managed Wasm/lifecycle evidence exists. |

CF-02 tracks the remaining publication,
both-manifest adoption, single-identity resolution and managed build/lifecycle
checks. The guard correctly prevents multiple default runtimes. Registry web
lookups failed; public crates.io API lookup separately failed DNS in the sandbox
and returned HTTP 403 on a permitted retry. Those failures establish no latest
version. Existing package source and exact resolution establish the pinned mismatch.

Commands/results, separate failure/retry logs, dependency trees and source/lock/
Wasm hashes remain in `.tmp/ic-memory-015-01`. Earlier evidence and unrelated dirty
onboarding/browser work stay preserved. Release version stays 0.4.16 and notes
join Unreleased; the public re-exported growth API change requires a minor product
release when requested. No full CI/release gate, live provider observation,
operational recovery qualification, sibling edit, commit, deployment, paid effect
or build cleanup occurs.

## Offline installation check and isolated trial plan — 2026-10-01

Native `installation-check` uses the existing complete installation validator,
without copying either host's init schema or provider wire. A bounded one-value
`ServiceConfigurationInput` decode precedes semantic validation with explicit
planned service/project/verifier/release. It retains original bytes/hash and a
passive summary in fresh private output. Namespace uses a full-width decimal
string; platform/release identity, stable allocation, host carrier generation,
provisioning, dispatch and provider/recovery qualification remain false.

Three targeted CLI cases pass: independent `didc` fixture preservation through
source edits, repeat/partial-output refusal, complete resource/binding/authority
validation and malformed/oversized/extra-value/trailing/ambiguous input refusal.
Affected CLI all-target strict lint passes. Actual executable output independently
decodes against maintained standalone Candid, and exact repeat refuses without
replacing artifacts. The initial fixture parser failure (unquoted reserved
`service` field), empty output and dependent failing tests remain separate from
corrected evidence. This is offline/native validation, without a new platform
case, provider observation or full CI/release gate.

The [trial plan](../operator-guide.md#isolated-uploaddownload-trial-plan) ties exact
roles, artifact/configuration/account inputs, financial/recovery qualification,
authorized provisioning, one maintained SDK transfer, trusted whole-body
completion, tenant download and continuing-obligation closeout together. Actual
service identity is finalized after authorized creation and rechecked before
install; shared configuration bytes alone are not either host's init carrier.
No trial installation/account/financial terms are selected; all four certificate
facts stay false. A weakened economic contract remains a reviewed minor semantic
change, never a local gate override.

The maintainer confirms concurrent dependency work. Its ic-memory 0.15 changes
are preserved; targeted checks above use that graph. A brief lock wait is retained
and subsequent checks use explicit build-lock preflight. Existing Canic
CF-02 remains open for matching adoption
and canonical managed artifact/lifecycle checks; this batch establishes no new
framework defect. Current/previous dependency results retain their own source and
artifact identities. Intent, failures/corrections, commands, exact source/lock/
binary hashes and offline samples remain in `.tmp/isolated-trial-plan-01`; prior
captures remain immutable. No network/account/provider-object/payment effect,
deployment, sibling edit, commit/version/publication or build cleanup occurs.

## ic-memory 0.15 follow-ups — 2026-10-01

The maintainer authorizes explicit managed adoption verification, a PocketIC
growth-refusal case, compact host diagnostics and removal of redundant readiness
checks. The dependency/package identities remain those in the preceding 0.15
adoption evidence; product version stays 0.4.16. Existing offline trial/browser
dirty work and earlier evidence are preserved.

Managed opening now takes an explicit authority, builds the seventeen maintained
requirements and invokes `verify_default_memory_manager_authority` before any
configuration/store handle opens. The lifecycle macro and both install/restore
paths require that authority, with the owning fixture updated in the same hard
cut. No registration, admission replay, policy change or second runtime occurs.
Five native subprocess hosts qualify absent runtime, wrong authority, missing
key, mismatched diagnostic metadata and matching adoption, with unchanged physical
allocation accounting, committed generation and populated neighbor bytes. These
are native host integration cases, not actual Canic lifecycle evidence. The first
fixture used an incorrect registration API and then incorrectly assumed the
process-global registry was thread-local; both failures are retained separately
and corrected by fresh processes without a reset API or production test branches.

Both adapters call one shared operator-authenticated `memory_status` workflow.
Its host-supplied compact summary maps to passive `HostMemoryStatus`, with backing/
virtual extent, assigned bucket capacity/slack, remaining slots and current/ledger/
unknown binding totals. It measures the entire host, not tenant payload, quota,
provider bytes or billing. Authentication/full owner configuration agreement
precedes the measurement callback; failed measurement yields typed `Internal`
instead of zero totals. The native denial/scope/error case and existing shared-store
cases pass. Default store opens now rely directly on nonconstructing key lookup;
the existing absent-runtime test still passes.

The new standalone endpoint uses the maintained bounded decoder and the managed
endpoint uses the maintained Canic limits/transport wrapper. Standalone deployment
Candid is regenerated; its exported-contract guard passes. Actual standalone
PocketIC coverage passes (4.26 seconds): denied controller/tenant/uploader/anonymous
callers, every scope binding, backing and attribution conservation, byte-preserving
query/replicated inspection, admitted reservations and passive fenced restore.
The existing operator admission/cancellation/restoration journey passes too
(4.37 seconds). Querying capacity grants no recovery authority and clears no fence.

The storage fixture wraps actual `DefaultMemoryImpl` backing growth with an
explicit refusal substitute, and declares a separate populated neighbor. A valid
admission writes real owners before a direct runtime backing reservation refuses;
only typed `BackingRefused`, unchanged virtual extent and the upstream `-1` adapter
result reach the intentional trap. Unexpected outcomes return a normal error so a
generic assertion panic cannot qualify refusal. The test observes typed IC
rejection, byte-identical whole stable memory (including ledger/manager/neighbor),
unchanged counters and absent admission. Exact storage-only retry succeeds once,
ordinary replay changes nothing, and same-release restore remains fenced.
The final growth case and both existing admission/manifest write-rollback cases
pass together (14.57 seconds). This is an explicitly forced growth after local
writes, not observed IC memory exhaustion, a BTree allocation benchmark or any
provider paid-effect/retry qualification.

Standalone and storage-probe release Wasm builds, affected all-target/all-feature
strict lint, all three changed native harness targets and warning-free core/host/
adapter rustdoc pass. The first Wasm build had missing fixture type imports; the
first lint rejected a test import placed after statements. Separate initial and
corrected logs remain in `.tmp/ic-memory-followups-01`. No full CI/release gate or
resource-budget benchmark refresh occurs.

Updated managed source compiles natively, including the explicit authority macro
and new query/contract comparison, but canonical managed Wasm still awaits the
existing two-runtime mismatch closure in CF-02.
Adopt matching Canic in both manifests, confirm one runtime identity, then run
canonical build plus managed lifecycle/query/contract checks. This batch establishes
no new Canic defect and never bypasses its multiple-runtime guard.

Logs, command record, source/lock/archive and copied final Wasm/Candid hashes remain
in the fresh `.tmp/ic-memory-followups-01` capture. Notes join Unreleased; the
breaking re-export/lifecycle API batch needs a minor product release when requested.
No provider request or financial effect, operational recovery qualification,
sibling edit, commit/version/publication, deployment or build cleanup occurs.

## Published memory alignment — 2026-10-01

The maintainer directs this release to proceed without waiting for Canic's
unpublished ic-memory adoption. Pin the core to registry `ic-memory =0.14.3`,
matching published Canic/core 0.110.49. Root and managed fixture lock graphs now
contain one memory package identity. The canonical `make build-canic-probe`
passes with the normal multiple-runtime guard enabled; no path overrides,
second runtime or sibling edits are used. The earlier 0.15 adoption, follow-up
and failed release captures remain unchanged as historical evidence.

Remove the unreleased 0.15-only typed-growth adoption, explicit managed-adoption
API and compact host-capacity query, together with their DTOs, consumers and
feature-specific tests. Restore the maintained lifecycle API and committed-runtime
preflight; this is not a second compatibility path. Keep browser upload composition
and offline account/installation checks. The real growth-refusal fixture uses the
upstream `Memory::grow` -1 sentinel, unchanged extent and deliberate IC trap after
admission writes, retaining full stable-image rollback, neighbor, exact retry and
fenced same-release restoration coverage.

Current native checks pass for shared grants, absent managed runtime, standalone
exported Candid and offline installation validation. Release Wasm builds for
standalone, storage, consumer and gateway fixtures pass.
Seven focused managed cases pass: adjacent Candid comparison, synchronous install/
activation/all-owner fenced restore, invalid input/release install rollback,
oversized upgrade rollback, certificate blocker persistence, bounded certificate
decoding and tenant admission/manifest authority through fenced upgrade. Actual
storage backing-growth refusal preserves the complete stable image and neighbor,
then succeeds on exact storage-only retry and restores fenced. Standalone stop/
start and repeated same-release upgrade also pass. Affected all-target/all-feature
strict lint and both formatting checks pass, including the separate managed
workspace. JSON/history checks preserve historical capability/changelog records
and false provider facts; maintained documentation links and draft syntax pass.
Intent, commands, raw logs, source/artifact hashes and graph are retained in
the sealed `.tmp/published-memory-alignment-01`. This is local composition evidence,
not deployed Caffeine, production Fleet or operational recovery qualification.
No full CI/release preparation, commit, version change, deployment, provider/paid
effect or build cleanup occurs. All four certificate host facts remain false.
CF-02 no longer requires a Canic release;
matching 0.15 adoption may be revisited later with a compatible published host.

### Dependency pin repair — 2026-10-02

The reported managed neighbor growth E0308 is reproduced by committed root
manifest/lock selection of ic-memory 0.15.0 in `0afc794`, beside published Canic's
0.14.3; the prior dependency-file alignment is absent from that commit. Restore
the exact `=0.14.3` pin and resolve offline, then project the managed lock using
`make prepare-canic-probe`. Both trees contain only 0.14.3. Canonical managed
build, strict managed all-target/all-feature lint and affected core/adapter/
storage/consumer lint pass. Growth callers and runtime guard remain unchanged;
no extra platform claim is inferred from compilation. The brief initial build
lock wait is retained, and a host process inspection subsequently finds no active
Cargo/Canic/build process. No full CI/release or new PocketIC run occurs.

Fresh intent, original dependency files, graph, commands, logs and hashes are
sealed in `.tmp/memory-pin-repair-01`. Earlier captures remain unchanged.
Both root dependency files must be committed by the maintainer before release
preparation. No sibling edit, commit, version change, cleanup or provider effect
is performed. CF-02 remains closed locally; no Canic publication is required.

## Independent library and memory 0.15 — 2026-10-02

The maintainer explicitly removes all Canic code and tests here; consumer frameworks
own their wrappers and integration qualification externally. Remove the adapter
crate, managed fixture and complete managed test tree, dependencies, build/test
commands and current framework inventories. No opt-in lane, adapter shim or path
override remains. Generic service APIs, grants, tenant rules, standalone endpoints,
native/browser tools and framework-free PocketIC fixtures remain maintained.
Earlier removed-integration captures are history, not current wrapper qualification.
All provider probe records, failed/inconclusive observations and outstanding facts
are retained; no source removal authorizes installation reset or retirement.

A bounded read of the official Cargo index on 2026-10-02 observes ic-memory 0.15.0
as the latest non-yanked release, checksum
`21a7b22c19fd5a35f074f7ecac0abb8544a4c84b0e925967ff5588d40a1d9c39`.
The root selects =0.15.0 and its locked graph contains one runtime identity and
no Canic package. Direct growth callers use typed results; backing wrappers use
the unchanged upstream trait. The allocator and product protocol generation stay
unchanged. Product package/receipt remain 0.4.16; the existing 0.5.0 minor draft
records the public growth API break and complete adapter removal.

Whole-workspace native all-target/all-feature check and strict lint pass. Targeted
shared-store/grant and installation cases and standalone exported-Candid check pass.
Release standalone/storage/consumer/gateway Wasm builds pass. Real PocketIC typed
backing refusal after admission preserves the complete stable image and neighbor,
then succeeds on exact storage-only retry and restores fenced. Standalone stop/start
and repeated same-release upgrade preserve all owner fences. These are actual IC
checks with labelled local substitutes, not deployed Caffeine or consumer wrapper
qualification. No full CI/release gate runs in this batch.

Framework-free Wasm workspace check, warning-free core/standalone rustdoc,
formatting and offline core package verification pass. Maintained documentation
links, draft syntax, the default target
plan and structural dependency/removal checks pass. Historical release notes and
raw provider artifacts are unchanged; two probe-ledger links to removed feedback
documentation are converted to historical text.

Intent and the maintainer's steering, official index, metadata/tree, source/artifact
hashes, commands, successful logs and the initial bounded documentation-edit failure
remain in `.tmp/memory-independent-release-01`. No commit, version/publication,
sibling change, provider call, paid effect, deployment or build cleanup occurs.
All four certificate host facts remain false. Consumers, including Canic, must
implement and test their wrappers in their own repositories; that downstream work
has not been done here and no upstream message is sent.

## Rust minimum and development toolchain — 2026-10-02

The maintainer requests an MSRV audit and development Rust 1.99.0. The prior
workspace minimum 1.98.1 unnecessarily matched its development pin. Audit the
locked Cargo metadata: no selected dependency declares a minimum above 1.88,
with `ic-cdk`, `ic-memory`, `ic-agent`, `ic-testkit`, ICU and time among the
packages declaring that floor. The maintained content codecs use
[`slice::as_chunks`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.as_chunks),
stabilized in 1.88. Set the workspace minimum to 1.88.0 independently of the
development pin 1.99.0; do not change dependency or product package versions.

The initial full-workspace 1.88 check fails in the native harness journal on
[`File::try_lock`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.try_lock)
and `TryLockError`, stabilized in 1.89. Declare 1.89.0 only for that unpublished
host harness, preserving the library's lower consumer requirement. Its first
1.89 check also reveals five newer `Duration::from_hours(24)` test calls;
replace them with the equivalent `Duration::from_secs(24 * 60 * 60)`.
Actual native all-target/all-feature checks pass at 1.88 for all other workspace
members and at 1.89 for the complete workspace. Corresponding Wasm checks pass.
These are source/locked-graph checks on Linux x86_64 and wasm32-unknown-unknown,
not promises about arbitrary consumer resolutions or other target platforms.

Rust 1.99 introduces assertion diagnostics and expands result-use linting here.
Use equivalent empty/nonempty comparisons, explicit types for ambiguous byte/path
collections and a `must_use` annotation. No lint is suppressed. The Cargo fixer
first cannot bind its restricted local locking listener; the approved offline
retry retains its invalid internal type-path and ambiguous slice suggestions.
Repair those suggestions manually, keep readable assertions and retain all
failed attempts. Strict workspace Clippy, rustfmt and Wasm checking pass on 1.99;
minimum-compiler checks are repeated against the final source after lint edits.

Intent, metadata, per-package/dependency floors, exact compiler identities,
commands, logs, failures and final source hashes remain in the fresh
`.tmp/msrv-toolchain-01` capture. The README, dependency guide, handoff and existing
0.5.0 draft distinguish development tooling from supported consumer compilers.
Earlier source-removal captures and dirty work are preserved. No full CI/release
gate, new runtime/provider probe, sibling edit, commit, publication, deployment,
paid effect or build cleanup occurs. Consumer wrapper/integration testing remains
external; all four Caffeine certificate host facts remain false.

## First standalone trial preparation — 2026-10-02

After released 0.5.0, the maintainer authorizes the trial review and selects a
100T-cycle total planning budget. The [proposal](../standalone-trial.md) separates
trusted participants, one fresh owner/1 KiB file, local request limits, unsupported
rollback activation and retained billing obligations from still-unqualified provider
economics. The proposed 10T service / 1T initial provider / 89T unallocated split
is not an estimate, account selection, accepted gate change or dispatched effect.

The existing opt-in SDK/native handoff accepts a canonical body size up to its
unchanged 10 MiB envelope. Both the 1 KiB case and default pass with network calls
refused, including snapshot repreparation after source edits, repeated-output and
corrupt-source refusal. Invalid sizes fail before creating output. Shared validation
accepts a one-object/1 KiB candidate; the maintained standalone DID independently
encodes and decodes its complete host carrier. An initial unsupported decoder flag
fails and is retained before corrected stdin decoding. The local principals and
financial values remain syntax substitutes, not proposed live identities or terms.

Fresh 0.5.0 standalone release Wasm builds and the focused actual PocketIC
certificate-boundary case passes. It proves refusal, authority, state preservation,
stop/start and fenced restore under the maintained four false host facts. There
are no substituted true facts or deployed provider observations in that check.
Public source refresh, two unavailable primary guide lookups and the public
pricing-unit query's valid reply/failed process are separately retained in the
[probe ledger](caffeine-probes/README.md). They qualify no charge cap or namespace.

Intent, original failed attempts, commands, complete inputs, CLI/SDK outputs,
Wasm/DID hashes and local logs remain in `.tmp/standalone-trial-review-01`; the
[retained summary](caffeine-probes/local/2026-10-02-standalone-trial-review-01/summary.json)
identifies each evidence class. Full CI/release validation, deployment, account
changes, payment, upload certificate or provider object traffic do not occur.
Current gate semantics, allocator, v1 schema and consumer-owned wrapper boundary
remain unchanged; any accepted semantic revision requires a minor hard cut.

## Restricted issuance contract — accepted 2026-10-02

The maintainer explicitly accepts one trusted uploader/fresh owner and one nonempty
file of at most 1 KiB, without provider spending-cap/replay or operational old-backup
guarantees. Required immutable uploader trust is validated before allocation,
retained in the current v1 installation and configuration readback, and bound to the
original tenant permission. Shared installation derives local bounds/namespace/
current-owner facts. Standalone returns Caffeine's plain root reply only after the
same synchronous durable exposure commit. No ingress qualification flags, override,
compatibility path, extra journal or allocator are introduced. Current policy/DTO/
CLI/init/schema semantics replace the former contract and require a minor release.

Actual standalone tests pass all 48 cases, including issuance once, original-role
refusals, rejected untrusted tenant-approved uploader, lost/pending native replies,
stop/start and fenced restore with retained obligations. An actual management
snapshot demonstrates lost later exposure/revocation and renewed local eligibility;
active snapshot rollback remains unsupported. Seven shared exposure/IC-certificate
cases pass with labelled host-fact substitutes. The ten-scenario Chromium/SDK suite
passes against actual local IC/IndexedDB and a gateway substitute after removing an
unused branch for the deleted managed fixture. It does not qualify provider behavior
or production browser storage. Targeted store/installation/policy/CLI checks, strict
affected all-target/all-feature lint, formatting and Wasm builds pass. Current full
init independently encodes/decodes with required trust; the offline checker sends no
installation/provider request.

Capture `.tmp/restricted-contract-01` retains exact artifacts, hashes and failed
attempts: initial lint, denied loopback, oversized fixture, checked conversion and
the wider run interrupted by the maintainer's cargo clean. Retained artifact copies
allow the final rerun to use fixed inputs. The [summary](caffeine-probes/local/2026-10-02-restricted-contract-01/summary.json)
separates current local facts from historical provider/source observations. Budget
remains at most 100T cycles for planning; actual targets/persistence/terms/cleanup
remain unselected, with no live deployment, payment or provider request. Full CI,
version mutation, commit, publication and sibling edits do not occur. Consumer
wrappers must adopt the required trust and current blocker contract externally.

## Maintained browser intent journal — 2026-10-02

`clients/browser/intents.js` supplies one bounded IndexedDB owner for the existing
certificate/gateway contract. Explicit create/open refuses existing/missing stores
and changed capacity; missing-store refusal leaves no empty replacement. Strict
transactions serialize point reads, count/capacity checks, cancellation and bounded
history across tabs and resolve only on completion. Arguments are snapshotted before
storage awaits and records validated. Lifetime slots are not refunded; no reset,
eviction, migration or provider retry API is added. The upload fixture now calls
this same owner, with actual transaction abort injection only in its platform wrapper.

Real Chromium verifies competing claims, permanent cancellation, malformed-request
refusal without mutation, uncertain/non-2xx/repeat/budget blocks, retained tombstones
and exact history across graceful browser-process exit/restart. Deliberate structural
history corruption refuses inspection and saves. The ten-scenario local IC/SDK
suite passes with actual aborted claims/observations and cancellation races against
a gateway substitute. The first run retains a failed extra fixture identity beyond
u128::MAX; only test identities change on correction. Logs and source hashes are in
`.tmp/browser-intents-01`; the [summary](caffeine-probes/local/2026-10-02-browser-intents-01/summary.json)
separates actual browser/IC facts from substituted provider observations.

This is a maintained implementation and local persistence evidence, not qualification
of power loss, browser eviction/rollback, application auth or deployed provider
behavior. The actual trial origin/profile/database and live service/account remain
unselected. No Rust build, live deployment/funding, paid effect, new SDK/allocator,
full CI, version mutation, commit or sibling edit is performed in this step.

## Complete local standalone/browser journey — 2026-10-02

The opt-in `make test-browser-standalone` joins actual restricted standalone
installation/certificate facts with SDK preparation/transfer, one-slot IndexedDB,
signed native admission, browser uploader preparation and reload/read-state recovery.
The owned gateway substitute serves the actual received chunk. HTTP success alone
cannot serve a tenant asset. Native full-body verification, a distinct installed
verifier's signed attestation and tenant verified download pass. Exact reference
release removes logical use while retaining 1,024 physical/liability bytes.

A separate fresh-owner corrupt-GET case creates no statement or completion, keeps
tenant download unavailable and preserves exposure/bytes after tenant withdrawal
and browser cancellation. Both cases pass (8.46 seconds), as do strict affected
integration lint, formatting and target builds. The current Wasm/CLI hashes match
prior retained artifacts; no production behavior change is implied by this harness.
Initial compiler/patch mistakes remain in `.tmp/standalone-browser-01`; the
[summary](caffeine-probes/local/2026-10-02-standalone-browser-01/summary.json) labels
actual IC/browser facts, fixed test identities and local provider substitutes.
No live provider/account call, deployment/funding, allocator/dependency/version
mutation, full CI, commit, cleanup or sibling edit occurs. Actual gateway acceptance,
provisioning, retention/billing and the trial environment remain unqualified.

## Configured certificate sizing — 2026-10-02

The maintainer requests removing the 1 KiB integration cap. The extra trial envelope,
host-evidence field and public blocker are removed directly; issuance uses the
existing admission/manifest quotas and preserves uploader trust, exact permission,
one-time exposure and restoration fences. See [the record](configured-upload.json).
Actual standalone PocketIC issues one 10 MiB certificate and a second object of
1 MiB + 1 byte in the same owner. Configured size and capacity exhaustion return
typed admission errors without releasing physical/liability accounting. Caller,
manifest, replay, revocation, stop/start and same-release restore checks pass.
Local rollback/lost-response and signed setup/carrier/Candid checks also pass.
Core and native boundary tests, release Wasm and strict affected-package lint pass.
Wrong test selection and the initial test-length lint failure remain in the capture.
No live provider effect or Miner adoption is qualified. Existing live captures and
the 0.6.0 owner stay frozen; publication requires a minor release. No allocator,
dependency, version, commit, sibling or cleanup change occurs.
