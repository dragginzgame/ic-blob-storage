# Content and billing primitives — native evidence

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
