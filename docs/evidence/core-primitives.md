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
