# Changelog

## [Unreleased]

## [0.1.12]

### Changed

- Replaced the direct `ic-stable-structures` dependency with `ic-memory` 0.14.3,
  matching Canic and IcyDB. The crate re-exports `ic_memory`, including its exact
  stable-structures substrate. Memory bootstrap, allocation grants and bucket
  configuration remain owned by the integrating host; no blob stores are declared.
- Tenant upload usage reads now scan only that tenant's ordered operation range,
  avoiding full service history scans while retaining all reservation and liability
  accounting. No cached counters or additional upload index are introduced.

### Added

- Bounded tenant pages for unsettled confirmed objects, including physically
  deleted and zero-byte objects whose billing obligations remain unresolved.
  Tenant-owned root indexing prevents other tenants from consuming scan budgets
  or appearing in cursor metadata; usage reads share the same index.
- Native pagination, accounting and upload-transfer checks plus a PocketIC
  release/deletion/billing observation journey with real caller isolation.
  Provider confirmation facts remain explicitly substituted by the fixture.
- Native memory composition checks for passive library linking and host-owned
  handles shared through the re-export, with isolated cells and preserved host
  configuration. These do not implement or qualify blob persistence or recovery.
- `scripts/dev/cloc.sh`, copied from Canic and adapted to this repository's crate
  names, plus `make cloc`. Reports Rust runtime/test file LOC and test function
  counts under `crates/`; requires optional developer tools `cloc` and `jq`.

## [0.1.11] - 2026-09-26

### Added

- Transient upload admission sharing one owner's catalog, root history and
  tenant/global capacity. Reservations bind exact request IDs, service, tenant,
  namespace, object incarnation, first reference, raw digest, root and length.
  Concurrent upload limits are separate from retained lifetime history.
- Exact retries return current state without another allocation. Cancellation
  frees byte capacity only before exposure; possibly exposed uploads retain
  reservations until independently confirmed. Confirmation transfers capacity
  into the catalog without double counting or resurrecting released references.
- Native coverage for competing tenants, exhausted bounds, conflicting requests,
  cancellation, uncertain outcomes, completion replay, zero-byte uploads and
  wide byte totals. Logical release, physical deletion and billing cessation
  continue to free separate capacities. No provider effect or persistence is added.
- Tenant-authorized usage and bounded active-upload pages include pending
  reservations. Pages recheck caller/cursor scope, retain continuation through
  terminal history and observe intervening cancellation or completion.
- Gateway root observations distinguish reservations, possible exposure,
  cancellation and confirmed lifecycle state. Namespace isolation and current
  membership apply before disclosure; no observation grants deletion permission.
  Batch reads resolve distinct roots together and scan operation history at most
  once, stopping when all pending roots are found. Duplicates reuse observations;
  confirmed/unknown/malformed-only batches skip history scanning. Temporary maps
  stay bounded by input length and do not add a persistent index.
- PocketIC upload fixtures verify actual tenant/controller isolation, cancellation
  replay, retained uncertain capacity and immediate gateway revocation. Native
  read tests cover page budgets, cross-scope cursors and changing state between pages.

## [0.1.10] - 2026-09-26

### Added

- Bounded in-memory chunk verification progress over an immutable Caffeine
  manifest. Out-of-order reads credit each position once; duplicates cannot
  inflate verified bytes or hide a missing chunk. Every supplied chunk is checked,
  including retries after prior success, and rejected input leaves coverage intact.
  The tracker retains one bit per chunk and no content or retry history; it does
  not claim destination durability, persisted resume or provider completion.
- Independent client-vector coverage for missing-position recovery, repeated
  content, reverse-order delivery and exact partial-chunk byte accounting.
- Ordered manifest-and-raw-digest verification: a chunk must pass its exact leaf
  check before entering the whole-file hash, so corrupt chunks can be retried
  without losing the verified prefix. Finalization requires full length and the
  expected raw digest; skipped/replayed chunks reject without advancing state.
- Bounded missing-chunk pages with independent scan/result limits and exact local
  byte ranges. Empty filtered pages retain continuation, and later pages skip
  chunks verified between calls. Enumeration neither reserves reads nor grants
  completion; tests cover end positions, oversized indices and partial final chunks.
- Actual PocketIC Wasm execution of full-chunk, partial-final-chunk and Unicode
  metadata vectors, including corruption recovery, replay denial, wrong-digest
  and truncated-read rejection. The local fixture measures ordered-append
  instructions against an explicit regression budget; no provider is contacted.

## [0.1.9] - 2026-09-26

### Added

- A test-only Wasm authority probe and unpublished PocketIC harness using
  `ic-testkit`. Real IC caller/controller checks cover tenant reads and release,
  forged bindings, exact release replay and gateway revocation between calls.
  Sample object facts are local substitutes; no provider or persistence is qualified.
- Real inter-canister gateway-sync tests with a controlled local source: overlapping
  attempts reject before effects, revocation survives an old reply, and a completed
  newer sync cannot be overwritten by the earlier response. Malformed, oversized,
  empty-list and rejected replies preserve membership and require explicit retry.
  Shared unpublished fixture types keep host/canister controls in one place.
- `make test-pocketic` builds the fixture and runs it against the explicitly
  provisioned server under managed startup/cleanup. `make test-native` retains
  the core-only suite; `make test` runs both sequentially. Checks/lints now cover
  all workspace packages, with build artifacts retained.

### Changed

- Moved the testkit dev dependency into the actual PocketIC host harness. The
  published core package and its native-only tests no longer pull the simulator
  stack into their dependency graph.

## [0.1.8] - 2026-09-26

### Added

- Bounded streaming Caffeine content hashing and verification: raw SHA-256 and
  the provider's metadata-dependent root in one pass, with 1 MiB provider chunks
  independent of append boundaries. A fixed hash frontier avoids buffering whole
  files or trees; explicit content, append and metadata budgets bound processing.
- Independent client vectors covering exact chunk edges, uneven multi-level trees
  and ECMAScript metadata normalization/order. Tests reject corrupt/truncated bytes,
  metadata mismatch and invalid appends without advancing hash state. Empty provider
  objects remain explicitly unqualified; matching roots do not prove upload completion.
- Bounded Caffeine chunk manifests checked against an expected root, with distinct
  chunk-hash identities and exact per-index byte verification. Individual chunks
  can be checked out of order or retried without changing state. Independent client
  leaf vectors cover reordered/tampered manifests, wrong positions, short/oversized
  chunks and corruption; a valid manifest is not proof of storage or whole-file completion.

### Changed

- Replaced the direct native PocketIC dev dependency with `ic-testkit` 0.10.0.
  Tests use its full `ic_testkit::pocket_ic` re-export and shared harness helpers;
  the locked PocketIC client/server remains 16.0.0. Testkit stays outside the
  production/Wasm dependency graph.

## [0.1.7] - 2026-09-26

Bounded local catalog, tenant reference reads and exact request-result lookup.
Persistence, provider execution and canister adapters remain pending.

### Added

- Tenant-checked reference liveness reads, individually or in bounded batches.
  Released and unknown references report inactive even while another reference
  keeps the object live. Batches preserve order/duplicates and reject unauthorized,
  oversized or mismatched-scope requests without returning partial results.
  Native tests keep physical and billing obligations intact after logical release.
- A bounded transient catalog joining confirmed objects, immutable root claims
  and exact reference receipts. Added lifetime object/reference/receipt limits,
  per-tenant logical quota, separate physical and billing-byte caps, and derived
  service/tenant usage counters. Rejected admissions are atomic; exact registration
  retries cannot reactivate released objects or erase settled history.
- Pending-deletion pagination with independent scan/result budgets and scoped
  forward cursors. Added current-gateway checks on every page and ordered root
  observations that keep unknown, malformed and foreign-namespace inputs explicit.
  Native multi-object tests cover capacity recovery, zero-byte liabilities,
  receipt exhaustion, cross-tenant denial, revocation and late replay. This remains
  local bookkeeping; durable upload reservations and provider execution are pending.
- Per-tenant lifetime object limits across namespaces, including zero-byte and
  settled entries, so byte-free history cannot consume all shared object slots.
  Exact registration replay remains valid at capacity.
- Bounded tenant reference reads across catalog objects and namespaces, preserving
  order and duplicates. Unknown and foreign roots share one rejection; mixed
  unauthorized batches return no partial results or object-binding details.
- Read-only lookup of exact reference-request receipts, including recorded failures
  and results retained after settlement or at capacity. Queries never reapply a
  request or allocate a receipt; original outcomes remain distinct from current
  reference liveness. Shared lookup rules keep mutation replay checks consistent.

### Changed

- Refreshed Canic, Toko and upstream provider evidence and design assumptions.
  Recorded root reuse, reference coordination, history churn and scan costs as
  decisions requiring consumer/provider evidence before production implementation.

## [0.1.6] - 2026-09-26

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
