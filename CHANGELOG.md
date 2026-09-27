# Changelog

## [Unreleased]

## [0.2.3]

### Added

- `StableUploads` persists tenant enrollment, root claims, exact upload permissions,
  manifests, confirmed lifecycles, individual references/receipts and maintained
  quota totals through host-granted `ic-memory`. Synchronous IC updates commit
  related writes together. Heap and stable owners share admission, reference,
  cleanup capacity and settlement rules. Completion preserves charged bytes;
  logical release, physical deletion and billing cessation remain separate.
  Reference mutations touch individual rows without copying complete histories;
  bounded manifest records use variable-size pages.
- `StableTenantEnrollments` preserves activation generations and suspension;
  `StableRootClaims` maintains immutable root/object bindings and a reverse index.
  Both are incorporated into the durable upload owner. Fresh installation rejects
  allocated memory. Reopening validates configuration, records, indexes and totals
  without repair, then fences mutations. Retained local state alone cannot authorize
  operational recovery.
- Durable indexed content discovery preserves independent upload/object identities
  and original metadata. Reference-qualified descriptors require the consumer's
  exact live reference. Tenant and operator history/cleanup scans enforce separate
  scan/result bounds and scope-bound cursors; empty filtered pages advance, and
  continuing billing remains visible after physical deletion.
- Durable admission/reference capacity queries use maintained counters and shared
  model arithmetic, preserving lifetime history and reserved cleanup receipts.
  Bounded operator root observations retain exact identities, pending/retired states,
  duplicate positions and malformed inputs. Suspended/restored reads remain
  available without granting provider callback, deletion, retry or mutation authority.
- `reference_receipt` inspects an exact tenant reference operation without
  applying it. Reads share mutation authority/payload checks and preserve original
  success or typed failure through suspension, full history and settlement.
  Historical success is distinct from current reference liveness.
- `blob-fixture-reference` saves bounded, explicit reference intents in a locked,
  identity-keyed local journal, then inspects their exact receipt on a selected
  local probe. Writes sync the file and directory before acknowledgment; exact
  retries recover the same record and changed payloads conflict. Full journals
  preserve recovery, and interrupted files are never automatically discarded.
  Native and executable/PocketIC cases cover writer termination, lost local
  acknowledgment, missing receipts, failures, release, settlement and stop/start.
  The tool sends no mutations, allocates no identities and grants no restore
  authority; local filesystem persistence is not production service recovery.
- An unpublished storage probe exercises the shared durable owner through actual
  IC callers. PocketIC cases cover partial-write rollback from admission through
  settlement, caller isolation, bounded reads, capacity and same-release upgrades
  into the mutation fence. Native cases additionally cover heap/stable agreement,
  corrupt or missing records, full-width identities and retained history.

Real Caffeine integration, durable provider-call intents, read sessions and
operational recovery remain unfinished. Provider facts in the probe are labelled
substitutes; this release does not qualify the service or Canic retirement.

## [0.2.2] - 2026-09-27

### Added

- `blob-fixture-inventory` connects prepared inventories to local admission,
  content-discovery and reference-capacity queries. It checks manifest/root
  consistency and recomputes totals before querying, preserves per-asset reference
  demand, and separates invisible, unfinished, live and retired content. Typed
  failures produce no partial report. Actual executable/PocketIC journeys cover
  isolation and unchanged service state. Queries are sequential observations,
  never upload permission, reservations or a resumable operation journal.
- Tenant-scoped `admission_capacity` reports remaining lifetime objects, concurrent
  uploads, manifest leaves and byte headroom from the shared admission accounting,
  plus enrollment and per-object metadata/size limits. It includes reservations
  and continuing billing, preserves lifetime history after cancellation/settlement,
  and remains inspectable during suspension. Native and bounded PocketIC queries
  cover scope isolation and lifecycle accounting; observations reserve nothing.
- `prepare_upload --inventory ... --snapshot PARENT_DIRECTORY` saves the exact
  hashed bytes and completed inventory in a fresh private directory. Duplicate
  roots share one saved body; later source replacement cannot alter that copy.
  Ordinary failures remove only the current attempt, and repeated runs preserve
  prior snapshots. Files are synced before success, but this is not a crash-durable
  transaction or operation journal. Saved files still need verification before
  later effects; no provider calls, library dependencies or allocator changes.
- `prepare_upload --inventory` performs a bounded offline multi-file dry run.
  It validates all declarations and aggregate work limits before reading sources,
  preserves separate asset mappings while grouping identical roots, and reports
  source versus distinct-blob byte/leaf totals. It rejects duplicate asset IDs,
  unsafe paths, symlinks and nonregular or wrong-length sources. No report is
  emitted until every file succeeds; source failures identify the asset and path.
  These are local content statistics, not live capacity, reservations or uploads.
- `CaffeineManifestBuilder` prepares a bounded ordered chunk manifest and both
  content identities in one streaming pass through the shared hasher. It reserves
  leaf capacity before accepting bytes and preserves state on rejected appends.
  The local `prepare_upload` example reuses the service metadata validator and
  emits original metadata, computed root/digest and leaves only after clean EOF.
  Independent vectors and a 10 MiB PocketIC admission/descriptor journey cover
  client preparation without relaying file bytes through the canister. No provider
  call, raw-digest admission requirement or allocator change is introduced.
- `retained_content_descriptor` checks confirmed completion and an exact live
  consumer reference in one owner read. Released/unknown references and mismatched
  object incarnations disclose no descriptor, even if another reference keeps the
  blob live or an old successful retain receipt replays. Native and PocketIC cases
  cover caller isolation, suspension, stop/start and cleanup. The observation adds
  no receipt or reservation; trusted publication and release coordination remain
  consumer workflow requirements.
- Tenant-scoped `content_descriptor` views recover the first validated metadata,
  exact object identity and current lifecycle. Admission retains one bounded
  metadata set per lifetime permission; reordered retries, suspension, cancellation
  and settlement preserve it. Actual IC caller, client-verification and capacity
  tests cover the private query adapter. This is not certified browser publication.
- `CaffeineRootVerifier` binds a trusted root, exact length and original hash
  metadata before processing a bounded stream. Clients can verify downloads
  without a supplied raw digest, leaf list or canister byte relay, reusing the
  existing Caffeine hashing implementation. Prefixes remain unverified until
  finalization. A local `verify_download` example requires successful EOF and
  rejects corruption, truncation, excess bytes and late read errors. This is a
  verification primitive, not a production HTTP/browser client or provider proof.
- The local `verify_download` example accepts an optional output path. It stages
  bounded bytes privately, verifies clean EOF and the root, syncs the file, then
  publishes without replacing an existing destination. Failures remove normal
  staging residue; symlinks and racing output creators cannot be overwritten.
  A 10 MiB independent-vector CLI check covers saved bytes and rejection paths.
  This is local file publication, not authenticated descriptors, browser delivery
  or a crash-durable transaction. Only the example adds an already-locked tempfile
  development dependency; library dependencies and the allocator are unchanged.

### Changed

- Resource checks now include maximum metadata retention and report
  `.tmp/descriptor-resources.json`. With retained headers, the 704-object workload
  uses 64 KiB more allocated Wasm memory; retain/release peaks measure 5.43M/5.51M
  instructions. These local costs include allocation/memory-access effects and
  do not establish production limits. The default allocator remains unchanged.
- The private readback fixture uses Candid's bulk byte decoder with unchanged
  wire types, reply limits and manifest verification. A 1 MiB read falls from
  about 234M to 116M measured service instructions; allocated Wasm memory is
  about 1.1 MiB higher but stays flat across repeated reads. The default Rust
  allocator is unchanged. Operator-only, bounded diagnostics and
  `make test-read-resources` cover full chunks, malformed replies, held-slot
  denial and recovery. These fixture costs do not qualify production delivery.
- Reference requests validate a private single-reference transition and cleanup
  receipt capacity before publication, removing the full lifecycle-history copy
  on each mutation. Public APIs, retained failure receipts and exact retry
  semantics are unchanged. A new PocketIC workload checks 256 simultaneously
  live references, all 511 receipts, stop/start and cleanup at capacity;
  `make test-admission-resources` also writes `.tmp/reference-history.json`.
- The unpublished admission probe uses operation-specific Candid inputs through
  the same shared handlers. It no longer sends or decodes the entire command
  enum for every mutation. Byte, work, type-header and skipped-value limits stay
  enforced, with actual-caller rejection, retry and capacity-cleanup coverage.
  Library APIs and production endpoint contracts are unchanged.
- Resource reports separate runtime entry, Candid header/value decoding and
  workflow counters. The 704-object workload's final admission drops from
  5.43M to 3.11M measured instructions, and peak retain from 9.20M to 5.43M.
  These local observations include IC memory-access charges, exclude reply and
  persistence/provider costs, and do not establish a production capacity limit.

## [0.2.1] - 2026-09-27

### Changed

- Admission keeps private global/tenant reservation and manifest-leaf totals
  alongside successful transitions, removing scans of retained upload history.
  Root ownership uses a bounded object-identity index instead of a reverse scan.
  Confirmed-object usage now maintains every global/tenant counter through one
  mutation path, including receipts for rejected lifecycle operations. Root-only
  exposure indexes the original operation and rechecks its authority; rejected
  admissions cannot create a lookup entry.
  Exact retries, cancelled history and separate physical/billing charges remain
  unchanged; failed admission consumes no index or quota capacity.

### Added

- Read-only reference capacity reports separate unused reference identities,
  unreserved receipts and cleanup reservations, including the number of fresh
  distinct retains that fit. The shared owner authorizes disclosure by tenant.
- A 704-object PocketIC workload retains manifests for 288 MiB of synthetic media
  through four reference generations, exhaustion, retries, stop/start and cleanup.
  Completion/deletion/billing controls are explicitly operator-only substitutes.
  `make test-admission-resources` also emits `.tmp/release-history.json`, separating
  pre-workflow and workflow instructions from allocated Wasm memory.
- Tenant-authorized content discovery returns the original upload operation and
  current reservation/lifecycle through the retained root index. Foreign and
  unknown roots disclose no object; suspended tenants retain inspection access.
  Native overlapping-release tests cover exact receipt recovery, stale reads,
  history exhaustion with cleanup and separate physical/billing settlement.
  PocketIC checks real caller isolation and passive discovery across stop/start.
- A two-tenant PocketIC workload filling 256 lifetime operation slots, checking
  cleanup, isolation, exact retries, stop/start and instruction/memory budgets.
  `make test-admission-resources` now writes both single-upload and history reports.
  Measurements retain the before/after tradeoff: fewer explicit scans do not
  establish uniformly lower total call costs, and this workload uses 64 KiB more
  allocated Wasm memory with the indexes.
- Independent accounting checks across interleaved tenant transitions, rejected
  mutations, zero-byte uploads, confirmation, deletion, settlement and retries.
  A 64-object, three-tenant audit covers all confirmed usage fields and totals above
  `u64`; an actual IC test exercises the final permission at 256-operation capacity
  through failed authority checks, stop/start and one-shot exposure.

## [0.2.0] - 2026-09-27

### Added

- Canonical service upload metadata: exactly one `Content-Length` must match the
  reservation; malformed names/values, duplicate names ignoring case and ambiguous
  length spellings reject before manifest mutation. Configuration must accommodate
  the largest object's length header. Native and actual IC checks preserve quota
  and prior manifests on rejection and accept equivalent reordered retries.
- Bounded manifest authorization for direct browser-to-Caffeine uploads, with
  exact uploader, activation and deadline checks. Manifest binding is distinct
  from provider completion; exposed uncertainty keeps its reservation. No file
  chunks pass through service admission. The 10 MiB local admission, preparation,
  retry and exposure sequence measures about 4M instructions, excluding provider
  transport and persistence.
- Bounded decoding, pre-conversion manifest checks and operator-only resource
  observations in the local admission probe. Focused PocketIC checks enforce
  instruction/memory budgets and emit a report through
  `make test-admission-resources`. The metadata-only boundary accepts at most
  16 KiB per command and separately bounds decoder work, skipping and type headers.
- Explicit global and per-tenant budgets for retained manifest leaves. Admission
  reserves capacity from declared size before catalog mutation; retries consume
  nothing extra, and cancellation or settlement cannot refund retained history.
- A local IC adapter exercising the shared owner with actual caller/time bindings.
  Covers role isolation, 10 MiB manifests, exact retries, suspension/expiry,
  stop/start continuity and atomic rejection of unsupported upgrades. It emits
  no certificates or provider effects; durable service recovery remains open.
- Shared service manifest budgets derived from the admitted object size, with
  explicit metadata limits and portable chunk counts. Empty uploads reject before
  reservation. Independent Caffeine vectors and the PocketIC journey now cover
  the 10 MiB media boundary, oversized rejection, exact chunk retries and retained
  recovery history; fixture admission and journal validation share one envelope.
- Bounded operator-managed tenant enrollment in the shared admission model.
  Suspension blocks fresh uploads, exposure and reference retains while preserving
  exact receipts, cleanup and liabilities. Reactivation invalidates old uploader
  permissions; stale enrollment updates conflict. Reference operations now verify
  project/service/namespace context at this boundary. The 0.2 plan specifies the
  remaining consumer registration/release transaction and outbox contract.
- Project-authorized upload admission over the shared catalog, binding an exact
  operation to an uploader and issuance deadline. Root-only certificate lookup
  resolves retained permissions; changed retries, wrong callers and repeated
  exposure reject. Passive lookup cannot renew authority. Revocation cancels only
  unexposed reservations; escaped uploads and failed consumer registration retain
  their accounting and references. This is a transient service model, without
  certificate transport, durable state or provider retry guarantees.
- Service configuration candidate validation for explicit service, operator and
  payer identities, Wasm-compatible metadata counts and consistent object,
  tenant, upload and gateway limits. Reference budgets reserve enough minimum
  receipt history for retain/release. Construction grants no installed authority
  or provider effects and defines no stable schema.
- A finite 0.2 delivery plan and current Toko development source review, separating
  consumer upload limits, browser authorization and asset-registration failures
  from the completed 0.1 foundation. Current status now summarizes milestones
  instead of repeating historical implementation notes.

### Changed

- Hard-cut the mandatory service byte-append workflow, streaming verification
  state and raw-digest verdicts. `UploadRequest` no longer requires a raw digest;
  service configuration names retained manifest capacity explicitly. Independent
  content/read verification primitives remain available, with digest binding owned
  by their integrity fixtures. Provider size, replay and completion guarantees
  remain prerequisites for live issuance, not claims established by a manifest.

## [0.1.19] - 2026-09-27

### Added

- Account-scoped Cashier audit query encoding with an explicit positive page
  bound, event filter and opaque cursor. Replies use the original method/target
  and the stricter requested/decoder count limit. Independent Candid vectors
  cover every filter and full-width cursor values; no automatic pagination or
  payment reconciliation is inferred from audit text.
- Bounded Caffeine payment-relationship reply inspection with explicit expected
  storage owner and payer. Preserves signed limits, period spend and bandwidth
  counters without inventing spending authority; absent reports, binding errors
  and all advertised provider failures remain distinct. Independent Candid
  fixtures cover decoder bounds and optional-field semantics.
- Library-owned Cashier balance, payment-relationship and gateway-list query
  encoding with explicit targets and account bindings. Replies can now be handled
  against the original request, rejecting wrong methods or sources before parsing.
  Gateway application also requires the exact registry scope and pending token.
  The local balance and gateway workflows retain their request across the await;
  independent Candid vectors and IC revocation/recovery checks cover both paths.
- Query-only relationship and gateway inspection in the controlled local source. PocketIC
  covers exact signed amounts, missing/error reports, owner/payer mismatches,
  denied callers, unchanged journals and restoration fences. Gateway queries
  refuse scripted effect modes; consumed or revoked sync tokens cannot reapply
  their replies. This adds no deployed transport, account-link action or paid operation.
- Source-backed installation proposal separating Toko tenants, storage owners
  and Cashier payers, with existing-obligation disposition and concrete gates
  for shared production handlers. Current package/interface metadata rechecked;
  no live installation, account or provider effects are selected by the proposal.

## [0.1.18] - 2026-09-27

### Added

- Bounded Caffeine ledger-deposit notification response decoder, based on the
  refreshed deployed Cashier Candid. It preserves reported credit, arbitrary-width
  block indices and all advertised errors, rejecting invalid amounts and malformed
  or over-budget replies. Independent Candid fixtures cover boundaries and unknown
  variants. Reports do not establish operation correlation or settle uncertain
  payments; no ledger transfer, notification or automatic retry is introduced.
- Shared bounded attachment accounting for sequential funding journals. Original
  full offers must preserve the installed reserve before refunds are applied;
  accepted and unknown transfers retain their allocation. Exact refunds and
  proven unsent offers stay separate, with typed capacity/sequencing/overflow
  failures. The funding fixture now uses this library model instead of owning
  another accounting loop; native and IC recovery checks cover the integration.
- Query-only exact funding lookup and unpublished `blob-fixture-funding-lookup`
  CLI. Full original requests must match retained intents; absent, pending and
  observed states remain distinct from denied/conflicting/failed lookups. IC and
  subprocess tests recover discarded ingress results without retransmission and
  preserve callback uncertainty and old-backup fences. This inspects the local
  experiment, not Cashier credit or permission to repeat a payment.

### Fixed

- Local funding now preserves all four advertised Cashier error categories in
  its journal, operator status and CLI JSON, including the reported unauthorized
  principal. Previously only internal errors survived classification. Actual IC
  tests preserve refunds and uncredited acceptance independently of each error,
  reject wrong-route ledger reports and unknown errors, and retain uncertainty
  after callback traps and fenced restore. Unpublished fixture schema remains v1
  with reinstall required across releases; published library APIs are unchanged.

## [0.1.17] - 2026-09-26

### Added

- Unpublished local `blob-fixture-refresh` command with passive dry-run and an
  explicit balance refresh. Requests bind service, namespace, source, account,
  configuration revision and next attempt; stale or consumed requests cannot
  dispatch another read. This is a PocketIC tool, not a production provider client.
- Separate JSON action and post-status outcomes: a failed diagnosis cannot replay
  the action or erase an acknowledged completion. Actual subprocess/IC tests cover
  stale previews, pending reads, rejected callers, failed observations, fenced
  restores, malformed acknowledgements and query-only dry-run enforcement.
- Local `blob-fixture-sync` command with passive preview and exact service,
  namespace, source, edit revision and next-sequence admission. Revocations,
  including absent-member revocations, invalidate old previews; consumed requests
  cannot dispatch again. Balance refresh and sync share action/status reporting.
  PocketIC covers held replies, reentrant replacement, old archives and live
  callback upgrades without restoring authority or repeating a completed action.
- Additive funding admission assessment preserves missing recovery, spendability
  and activity alongside known blockers. Reserve arithmetic requires known funds
  and retains the complete request; existing admission API behavior is unchanged.
- Passive `blob-fixture-funding-preview` command binds sender, peer, operation ID
  and amount. It reports identity reuse, journal capacity, unknown spendability,
  unverified credit and restore fences without consuming an intent or transferring
  cycles. PocketIC covers refunds, callback traps, exhausted history, added gross
  cycles and update-only method rejection. No operator funding action is exposed.
- Explicit installed attachment budgets for the local funding fixture. Original
  intents reserve the full offer atomically; exact refunds and proven enqueue
  failures release only the corresponding allocation. Accepted or unresolved
  attachments remain charged through restore. Preview revisions change even after
  full refunds. Gross cycle top-ups and incoming receipts cannot replenish this
  budget; execution fees, provider credit and production spendability remain separate.
- Additive liquidity policy checks the full attachment against platform liquid
  cycles, call costs, positive operating slack and explicit other liabilities.
  The local funding fixture rechecks after intent persistence and records liquidity
  refusals as unsent operations, without fabricated refunds or reusable identities.
  Passive previews expose cost/liquidity observations but cannot authorize dispatch;
  PocketIC covers fee-only rejection, operating holds and changed funds with an
  unchanged allocation revision. Production credit and recovery remain unqualified.

### Fixed

- Local funding refusals no longer execute callback trap controls when no call
  was sent. Their consumed identities and released attachment allocations survive
  restore; genuine callback traps still retain the full uncertain attachment.
  PocketIC covers refusal-history exhaustion and zero/full-refund callbacks.
- Funding previews now report the same maximum attachment bound enforced by
  update admission, independently of resource and provider blockers.

## [0.1.16] - 2026-09-26

### Added

- Exact-release Caffeine verification checkpoints bind manifests, content digests,
  lengths and streaming hash state. Reconstruction validates counters, leaf
  boundaries, canonical buffering and an accidental-damage checksum. Protected
  checkpoints remain private; they grant no freshness or provider authority.
- Shared operator diagnosis preserves recovery/provider blockers, unknown or
  uncertain funding and outstanding work. Complete funding-history assessment
  keeps transport acceptance separate from provider credit; later refunds cannot
  clear older obligations.
- Shared balance-threshold assessment reports exact shortfalls without inventing
  spendable funds. Operator diagnosis explicitly blocks unknown spendability.
  The existing complete reserve API retains its signatures and behavior.
- Unpublished `blob-fixture-status` client queries explicitly selected local
  PocketIC instances. JSON preserves unknown values, exact decimal amounts,
  separate catalog charges and recovery fences. Blocker checks and typed read
  failures have distinct exit codes; queries cannot fall back to updates.
- Scoped local balance observations persist exact service/namespace/source/account
  intents and bounded response history. The shared Caffeine decoder distinguishes
  zero, malformed amounts, mismatched accounts and provider failures. Revision
  changes, dispatch-based expiry and restoration prevent stale use. Validated
  diagnostic billing limits remain bound to their original configuration revision.

### Changed

- Authority, controlled-source and funding fixtures restore into permanent
  inspection-only fences. Validation retains catalog identities, receipts,
  reservations, private verification state, byte charges and pending operations.
  Old journals and late callbacks cannot resume uploads or payments. Restored
  funding receivers reject cycles before acceptance.
- PocketIC coverage now includes held gateway/balance replies, older in-flight
  journals, callback/upgrade rollback, repeated restores, lifetime capacity and
  separate physical/billing release. Actual CLI subprocesses prove passive reads,
  caller/target checks, update-only method rejection and configured diagnosis with
  unknown spendability. Fixture schemas are hard cuts with reinstall across releases.

Production provider integration, both service adapters, operational recovery and
whole-canister snapshot safety remain unqualified. These local fixtures and
read-only diagnostics do not establish Canic removal readiness.

## [0.1.15] - 2026-09-26

### Added

- A connected local PocketIC upload/deletion journey uses shared tenant policy,
  upload reservations and lifecycle accounting with current Caffeine method shapes.
  It covers certificate admission before possible exposure, interrupted uploads,
  exact request replay, cross-tenant rejection, logical release, real inter-canister
  deletion callbacks and separate billing cessation. Mixed invalid deletion batches
  roll back on the IC; revoked gateways cannot apply delayed confirmations.
  Upload completion and final billing evidence remain explicit local substitutes.
- The local journey now binds certificate admission to actual content verified
  against its reserved Caffeine manifest and raw digest. Corrupt/truncated bytes,
  digest replacement, unauthorized verification and pre-exposure completion are
  rejected without releasing reservations. Explicit metadata and up to six 1 MiB
  chunks are verified across separate messages with tenant-only progress. Exact
  chunk retries do not advance hashing twice; rejected chunks preserve the prefix,
  and a final raw-digest mismatch cannot be reset by replaying admission. Bounds
  are local fixture limits, not production upload limits or restart guarantees.
- Local readback now fetches one chunk through a real inter-canister call and
  verifies its exact length/hash against the admitted manifest before returning
  bytes. Tenant/reference/gateway checks run before and after the await. Held
  replies are rejected after release or gateway revocation/re-addition; corrupt,
  truncated, oversized and malformed replies return no bytes. One bounded read
  slot prevents overlap, with exact callback cleanup and no automatic retry.
  The controlled source is a provider substitute, not a Caffeine HTTP adapter.
- The transient authority fixture rejects unsupported upgrades in both lifecycle
  hooks, preserving obligations instead of discarding heap journals. PocketIC
  covers stop/start continuity, rejected upgrades (including skipped outgoing
  hooks), held callbacks and retained billing/root history. An actual read-callback
  trap rolls back slot cleanup and leaves further reads blocked through stop/start
  and elapsed time. Upload journal restoration and old-snapshot safety remain open.
- The local gateway source persists a bounded journal through host-owned ic-memory:
  bindings, one retained leaf, read state and lifetime call intents/results. It
  restores synchronously into a permanent inspection-only fence. PocketIC covers
  maximum retained data, exhausted history, missing/older stable journals, skipped
  outgoing hooks and unresolved callbacks. No restored counter authorizes new
  effects; this is fixture evidence, not provider or production recovery.
- Raw content digests can be parsed from exact 32-byte boundary inputs, with
  typed length errors and no implied content verification or tenant authority.
- The authority fixture atomically archives all three catalogs through ic-memory,
  retaining cancelled/settled roots, reservations, release receipts, byte liabilities,
  original manifests and observed verification progress. Exact read intent is
  recorded before dispatch; callback traps and rejected deletion batches roll back
  archive writes with live state. Operator-only inspection reads stable memory.
  The archive cannot resume hashing or authority; unsupported upgrades still reject.
- Gateway registries expose read-only allocated/pending sync sequence observations,
  without exposing reusable tokens or granting reconstruction authority.

## [0.1.14] - 2026-09-26

### Added

- Shared funding-transfer accounting distinguishes exact unbounded-call refunds,
  proven enqueue failures and unknown outcomes. Pure reconciliation policy keeps
  accepted cycles separate from provider credit and retains the full attachment
  when transfer evidence is missing. Invalid refunds are rejected without clamping.
- The local funding fixture now uses the shared model and policy. PocketIC covers
  a real insufficient-cycles enqueue failure with no callback refund, retained
  history across upgrades and rejection of reused identities. Fixture starting
  balances are explicit; live Caffeine credit reconciliation remains unqualified.
- Bounded decoding of Cashier audit-download responses using the refreshed deployed
  Candid interface. Opaque CSV, reported counts and optional cursors are preserved;
  provider errors stay distinct from empty pages. Independent wire fixtures cover
  resource bounds and malformed pagination. Audit rows do not yet establish credit
  or authorize retries.

## [0.1.13] - 2026-09-26

### Added

- Local PocketIC funding experiments capture exact call refunds separately from
  Caffeine reply decoding, covering zero/partial/full acceptance, provider errors,
  malformed replies and rejects. Callback traps retain unresolved intent and
  block another payment. Host-owned ic-memory journals restore synchronously
  across same-release upgrades, preserving payment identities, receipts and
  lifetime limits. Receiver traps roll back acceptance and receipts with a full
  refund; failed upgrades preserve journals and unresolved-payment blocking.
  The bounded unpublished fixture uses ic-testkit's PocketIC
  export; it does not qualify live Cashier credit or recovery from old backups.

## [0.1.12] - 2026-09-26

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
