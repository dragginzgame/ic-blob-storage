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
