# Content and billing primitives — native evidence

The original core candidate is recorded below; subsequent local ports have
separate source hashes in their sections.

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
