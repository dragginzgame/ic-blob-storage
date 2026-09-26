# Content and billing primitives — native evidence

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
historical [source inventory](../canic-source-inventory.tsv). Other dirty Canic
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
matches the [source inventory](../canic-source-inventory.tsv). Parsing returns
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
