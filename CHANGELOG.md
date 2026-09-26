# Changelog

## [Unreleased]

## [0.1.6]

### Added

- Scoped gateway registries that reject stale, cancelled and replayed sync
  responses. Operator membership edits invalidate earlier syncs, including
  revocation of a currently absent gateway; counter exhaustion never blocks
  revocation. Added pure callback checks for current service/namespace membership
  and native revocation/race tests. Persistence and endpoints remain pending.
- Bounded Cashier gateway-list reply decoding connected to the scoped registry.
  Wrong-scope and stale attempts reject before parsing; malformed, over-budget
  and invalid lists preserve membership and pending state. Added an independent
  Candid fixture and native reply-to-callback revocation coverage.
- Bounded Cashier account-balance reply decoding with requested-account checks,
  validation of every amount and distinct provider failures. Independent Candid
  fixtures and native readiness tests distinguish failed reads from a real zero
  balance and preserve recovery fences. Funding and balance replies share one
  private balance schema; live queries remain pending.

### Changed

- Recorded Caffeine's independent-deployment support question and the concrete
  provider evidence still needed for the full service journey. Updated the
  handoff to the verified 0.1.5 release and preserved its source-bound evidence.

## [0.1.5] - 2026-09-26

Local blob lifecycle, ownership bindings and request replay handling.
Persistence, endpoint authentication and provider execution remain pending.

### Added

- Bounded binary-root batch parsing for future liveness requests. Input order,
  duplicates and per-entry errors are preserved; raw entry and byte limits
  reject oversized batches before allocating results. No liveness lookup or
  callback endpoint is implemented by this parser.
- A transient confirmed-object lifecycle model with bounded, idempotent reference
  bookkeeping. Final release, physical deletion and billing settlement advance
  separately; premature confirmations and reference reuse reject without mutation.
  Native transition tests cover accounting and rejection without mutation.
- Explicit service/tenant/namespace/object/incarnation bindings on lifecycle
  references and confirmations, with rejection before mutation or replay handling.
  Added pure direct-tenant access checks and native cross-scope denial tests.
- Bounded local request receipts for reference mutations: exact retries return
  original success/failure, conflicting request-ID reuse rejects, and receipt
  reservations preserve capacity to release every active reference. Receipt
  replay still requires the caller's current access check; durability is pending.
- Bounded Caffeine response decoding that preserves structured funding errors
  and separates upload completion reports from verified storage. Added private
  wire types, independent Candid fixtures and malformed/over-budget reply tests.
- Immutable, bounded root claims that reject reassignment across tenants,
  namespaces and incarnations, retaining original associations after settlement.
  Native composition verifies delayed confirmations cannot delete a newer object.

### Changed

- Reviewed Canic's lifecycle design independently and documented the proposed
  persistence boundaries, separating logical release, physical deletion and
  continuing billing obligations. Provider qualification remains open.
- Added source-bound provider recovery probes and integration requirements for
  upload completion, structured funding errors and delayed root-only deletion
  callbacks. Client/codec substitutes do not qualify the deployed provider.

## [0.1.4] - 2026-09-25

Blob-storage billing validation and gateway primitives extracted from Canic,
plus provider-interface evidence. Persistent workflows, provider execution and
canister adapters remain pending.

### Added

- Safe conversion of billing amounts from Candid `nat`/`int` values into Rust
  cycle amounts, plus strict positive decimal funding input. Typed errors reject
  malformed or out-of-range total, prepaid, promotional and ledger amounts.
- Validated billing-configuration candidates combining Cashier principal,
  funding thresholds and gateway bounds that fit 32-bit Wasm on every host.
- Bounded gateway lists and transient membership: ordered deduplication, separate
  raw/distinct limits, idempotent add/remove and all-or-nothing sync replacement.
- Pure funding-intent admission that rejects recovery fences, outstanding or
  uncertain payments, missing configuration and full-request reserve violations.
- Native boundary, rejection/recovery and Candid composition tests connecting
  validated inputs to gateway limits, funding admission and readiness diagnostics.

### Changed

- Recorded Toko's provider defaults, the deployed Cashier's Candid interface and anonymous
  gateway/pricing observations. Corrected the compatibility review: both gateway
  names are advertised and the newer top-up wrapper is Candid-compatible.

## [0.1.3] - 2026-09-25

Release-test reliability and documentation cleanup; storage-service scope is unchanged.

### Changed

- Release-helper tests now show a clearly labeled fixture notice and one success
  summary, with case diagnostics and retained logs on failure.
- Isolated release-test cases, consolidated overlapping preparation checks, and
  replaced permissive log matching with exact release-effect assertions.
- Corrected Make help to reflect that releases preserve the build cache.
- Consolidated duplicate extraction/provider planning documents and replaced
  accumulated handoff history with current status, preserving source evidence.

## [0.1.2] - 2026-09-25

Release and publication tooling fixes; storage-service scope is unchanged.

### Changed

- Release and publication commands retain build artifacts; cleanup is available
  only through an explicit `make clean`.
- Enabled crates.io publication and removed the B1 ownership/readiness blocker.
  Publication still validates the clean tagged release and its source receipt.
- Added the public repository URL to the crate's published metadata.

## [0.1.1] - 2026-09-25

Content-verification and billing-policy foundations. Storage workflows and
provider integration remain pending.

### Added

- Separate raw SHA-256 content digests and Caffeine provider root identities,
  with canonical hash parsing, typed malformed-input errors and byte conversion.
- Incremental raw-content verification with exact offsets, declared-length and
  digest checks, bounded memory, and unchanged state after rejected chunks.
- Validated funding limits and pure reserve-protected funding decisions that
  never substitute a partial top-up.
- Read-only billing readiness with typed balance failures, blockers, warnings
  and recovery-fence reporting.
- Native boundary/vector tests and source-bound evidence for the first core
  primitives; service workflows and provider qualification remain pending.
- Official Mops registry evidence confirming backend package 1.1.1 and matching
  source hashes for the pinned Caffeine integration.

### Fixed

- Release preparation now preserves undated historical changelog entries while
  rejecting competing future drafts, allowing 0.1.1 after the recorded 0.1.0.

## [0.1.0]

Initial repository scaffold, dependency setup and extraction planning.

### Added

- Independent Rust 2024 library workspace with Rust 1.98.1, an MIT license,
  repository governance and isolated build output.
- Pinned Candid, Serde, SHA-256, typed-error, IC CDK and stable-storage
  dependencies, with a locked dependency-fetch command and offline native/Wasm
  compilation checks.
- Native-only PocketIC 16 integration-test dependency, verified local server
  provisioning and an overridable server path that prevents automatic downloads
  during tests.
- Maintainer patch, minor, major and exact-version release tooling, including
  release previews, validated version preparation, rollback on failure,
  source-bound release records, atomic branch/tag pushes and separate registry
  publication commands.
- Canic extraction and capability inventories covering storage lifecycle,
  gateway administration, billing, operator commands and diagnostics, with
  replacement acceptance requirements before Canic removal.
- Draft service and acceptance contracts for standalone and Canic-managed
  deployments, including tenant isolation, quotas, paid-effect recovery,
  restore fencing, deletion, billing cessation and installation retirement.
- Caffeine integration baseline for client 1.1.2 and backend reference source
  1.1.1, with package-integrity verification, client hashing observations and
  documented provider-interface and qualification gaps.

### Implementation status

- Storage APIs, provider workflows, clients and canister adapters are not yet
  implemented. The dependency and tooling checks qualify the scaffold only.
- Provider qualification and the B1 service contract remain open. Registry
  publication is disabled, and Canic functionality has not been removed.
