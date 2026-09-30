# Changelog

## [Unreleased]

## [0.4.9]

### Added

- Shared operator-only installation readback binds the actual service and caller,
  preserves exact configuration/project/verifier/release and reports restoration
  fencing. Standalone delegates to this handler with unchanged Candid.
- Controlled Canic artifact exposes nineteen managed blob endpoints for
  configuration, tenant enrollment/suspension, upload admission/revocation,
  status/history/discovery, manifests, references/capacity and passive operator
  status/funding history/outcome. All delegate to shared service workflows;
  native Candid comparison checks exact method types and modes against standalone.
- Managed PocketIC journey preserves an admitted, prepared upload and capacity
  through same-release upgrade while refusing mutations. It checks operator,
  tenant, uploader, outsider, Root and controller authority and valid Candid at
  the 128 KiB update boundary, including oversized inter-canister refusal.
- Managed suspension/cleanup journey cancels an unexposed reservation, preserves
  cancellation history and accounting through fenced restore, and checks private
  reference/history boundaries and passive operator inspection.

### Changed

- Managed fixture enrollment now uses the maintained blob tenant endpoints;
  adapter-owned state access and one passive generic failure wrapper preserve
  standalone reply types without duplicating authority or workflow logic.

## [0.4.8] - 2026-09-30

### Added

- Unpublished Canic composition library supplies explicit memory declarations, caller
  guards and synchronous installation/restoration over the shared service owner.
  Linking registers no endpoints, lifecycle or memory. The owning artifact uses
  Canic directly; the core and composition library remain Canic-free.
- Bounded managed application-argument extraction requires the authenticated
  two-argument carrier and explicit typed installation policy, project and verifier.
  Participants bound their own platform copy before decoding, bind the service to
  its actual principal and retain Canic's validated release identity. No defaults
  or replacement configuration are accepted. Failed installations preserve an
  enrolled tenant, all stable owners and neighboring memory.
- PocketIC composition fixture using published Canic 0.110.48 covers Prepared ingress
  and inter-canister refusal, activation, Fleet admission, tenant denial for
  outsider/Root/controller callers, rejected-upgrade rollback and retained
  installation/neighbor memory with all four owners fenced after upgrade.

### Changed

- Local validation builds the managed fixture through Canic's supported CLI.
  Its isolated resolver-2 workspace follows the host validator while the main
  workspace retains resolver 3 and the existing allocator. Its ignored lockfile
  is seeded from the repository lock and projected offline before build/lint.

### Fixed

- Managed Candid export now follows every application endpoint declaration. A
  native official-parser check validates endpoint names, modes and argument shapes,
  covering an omission that actual endpoint calls alone did not detect.

## [0.4.7] - 2026-09-30

### Added

- Shared named service memory requests and grant assembly let embedding hosts use
  their existing ic-memory runtime without duplicating the sixteen-store mapping.
  Default-runtime access requires prior host bootstrap. Composition tests preserve
  populated owners and neighboring application memory under different placements;
  missing grants refuse without initialization or repair.
- Shared immutable installation owner validates service/resource configuration,
  provider project, verifier and compiled release before memory allocation. It
  preflights all seventeen grants, retains one bounded configuration record and
  restores every owner synchronously into inspection-only fences. Tests preserve
  populated accounting and reject missing or invalid retained state without repair.
- Focused PocketIC coverage verifies installation rollback, caller/verifier
  authority, corrupt-state refusal, project byte bounds and repeated fenced
  upgrades against the refactored standalone canister.

### Changed

- Standalone delegates configuration persistence and service construction to the
  shared installation owner, removing its private record/conversion implementation.
  It uses the shared grant mapping while retaining allocation policy, authenticated
  endpoints and synchronous lifecycle. Public DTOs, Candid, stable keys, schemas,
  limits and restore fences remain unchanged.

## [0.4.6] - 2026-09-30

### Added

- Standalone lifecycle regression coverage preserves full installation and retained
  metadata through stop/start and repeated same-release upgrades. All four owners
  remain fenced, and operational refusals preserve stable memory.
- Native `blob-storage reference-receipt` and `reference-status` sign separate
  tenant queries using saved original boundary requests. Historical success,
  recorded failure and absence remain separate from current liveness and restore
  fences, with exact scope validation and no mutation, provider call or retry
  authority. Shared request encoders reuse the maintained reference rules.
- Signed reference inspection tests preserve receipts through release, settlement
  and fenced upgrade while current liveness changes. Production standalone
  unknown/unconfirmed content remains a typed refusal.
- Native `blob-storage funding-outcome` signs one query for the exact original
  funding operation, offer and optional target. JSON preserves absence, unknown
  attachment, exact refund, structured provider response, conservative
  reconciliation and found-record restore fences without claiming provider
  credit or authorizing another payment. A shared validated request encoder and
  bounded reply decoder retain typed refusals and full-width amounts.
- Signed PocketIC coverage joins funding history to exact outcome inspection and
  checks passive uncertain/refund records through fenced upgrade. Standalone
  absence remains distinct from unknown fence information and scope/trust refusals.

### Changed

- Consolidated acceptance and Canic parity summaries around current implementation
  and source-bound evidence. Durable standalone configuration and signed native
  transport are recorded as implemented; provider, operational recovery, managed
  adapter, operator mutation, consumer and retirement qualification remain open.
  Historical capability records remain explicitly dated evidence.

## [0.4.5] - 2026-09-30

### Added

- Native `blob-storage upload-history` signs one operator query over retained
  upload identities and local lifecycle states. Explicit filters, full-width JSON,
  saved scoped cursors, scanned-row counts and restore fences preserve empty
  filtered progress without automatic pagination, provider calls or retry authority.
- Shared bounded upload-history request/reply codec checks complete identity,
  scope, ordering, root/object uniqueness, selected states and forward continuation.
  Refusals stay distinct from empty pages. Signed PocketIC coverage exercises
  cancelled history, real scan/page bounds and saved continuation after upgrade
  without changing stable state.

## [0.4.4] - 2026-09-30

### Added

- Native `blob-storage certificate-assessment` signs one uploader-only query and
  compares the returned permission with saved original intent. JSON retains every
  missing prerequisite and full-width identity; unprepared, revoked and fenced
  refusals remain distinct. No certificate issuance, provider call, reservation
  or retry authority is introduced, including when the blocker list is empty.
- Shared bounded certificate-assessment request/reply codec validates the complete
  permission and rejects malformed, oversized, foreign or duplicate-blocker replies.
  Signed standalone PocketIC coverage exercises exact intent, root trust,
  revocation and upgrade fencing while preserving stable state.

## [0.4.3] - 2026-09-30

### Added

- Standalone `_immutableObjectStorageCreateCertificate` update and uploader-only
  `blob_upload_certificate_assessment` query delegate to shared permission/exposure
  checks. Assessment returns the original permission and missing prerequisites
  without changing state or reserving issuance. Certificate updates refuse while
  provider pre-charge limits, namespace binding, replay charging and recovery
  readiness remain unqualified. PocketIC covers authentication, malformed input,
  unchanged reservations, stop/start, revocation and upgrade fencing.
- Actual PocketIC management snapshot tests demonstrate that rollback can bypass
  upgrade fencing, revive revoked permissions and lose later upload records and
  reservations. Standalone certificate issuance still refuses after rollback;
  labelled fixture facts demonstrate how falsely asserted recovery readiness could
  permit repeated simulated exposure. Retained evidence documents the unsupported
  snapshot activation path; these tests do not implement operational recovery.
- Retained official Caffeine source capture and recovery review records. Reviewed
  backend hashes and npm 1.1.2 integrity remain unchanged despite a newer upstream
  revision. Source/local evidence supplies no new deployed-provider guarantees.

### Changed

- Shared passive certificate-assessment DTOs and exposure-blocker conversion replace
  the fixture's duplicate wire enum and mapping. Issuance rechecks current authority;
  a successful assessment cannot override later permission changes.
- AGENTS.md, service contract, roadmap and handoff retain one authoritative storage
  owner with local durable journals. External journal/controller machinery and extra
  metadata calls are deferred until a concrete use case justifies them. Existing
  certificate and recovery prerequisites remain enforced.

## [0.4.2] - 2026-09-30

### Added

- Native `blob-storage submit-attestation` validates a complete successful provider
  observation and submits its exact statement once. A durable exclusive claim,
  statement copy, signed request, request ID and artifact hashes precede the update;
  incomplete, failed or mismatched observations reject. Existing claims refuse
  resubmission, including after interruption or a lost reply. Bounded acknowledged
  replies use the shared exact-statement decoder; pending and uncertain results
  remain for explicit historical receipt recovery. No provider fetch, automatic
  retry, polling or regenerated observation time occurs.
- Signed PocketIC coverage checks intent before transport, competing native
  submitters, damaged artifacts and receipt recovery after a local proxy drops an
  actual replica acknowledgment. Provider exposure/content remain labelled fixtures;
  this does not qualify standalone exposure or deployed Caffeine behavior.

## [0.4.1] - 2026-09-29

### Added

- Verifier-only `blob_verification_plan` binds the installed Caffeine owner/project
  and original declaration to an exposed unfinished upload. Standalone and the
  durable fixture share the handler; wrong roles, bindings, phases and restored
  owners reject without changing state.
- Native `blob-storage observe-upload` makes one explicitly bounded provider GET,
  checks the complete stream against original root/metadata/length, and durably saves
  the exact attestation statement. Fresh run artifacts retain request intent,
  responses, failures and hashes; existing or interrupted runs cannot be overwritten
  or automatically resumed. Partial, corrupt, oversized, encoded and redirected
  responses reject. No service update or on-canister file hashing occurs. Local HTTP
  and signed PocketIC evidence includes process interruption and explicit fixture
  attestation/recovery; deployed provider qualification and native dispatch remain open.
- Authenticated `blob-storage upload-attestation` compares a saved exact statement
  with immutable service history, reporting matched, conflicting or absent evidence.
  It preserves full-width identities and restore fences without resending the
  statement, fetching provider content or authorizing retries. Signed PocketIC
  coverage recovers a discarded acknowledgment through reference release,
  settlement and restoration while preserving service state and the saved intent.
- Shared bounded attestation reply handling checks the expected service, namespace,
  permission, verifier and receipt chronology. Mutation acknowledgments must match
  the complete saved statement; malformed replies and service refusals never become
  absent receipts. Historical receipt lookup preserves conflicting statements.

## [0.4.0] - 2026-09-29

### Added

- Explicit verifier trust for upload completion: shared handlers authenticate the
  installed verifier and exact exposed upload, retain its first attestation with
  accounting and the first reference, and support immutable receipt inspection.
  Exact replay never reactivates released references; conflicting statements and
  restored mutations reject. Verifier-only manifest inspection supplies the
  original declaration. Attestations establish trusted observations of content
  availability, not future retention, payment credit or billing cessation.
- Authenticated `blob-storage verify-upload` checks a bounded local file against
  the service's exact retained manifest using the existing native streaming
  verifier. Full-width identities and original metadata remain bound; corruption,
  truncation, changed permissions and unprepared uploads reject. It sends no
  gateway request or attestation and does no on-canister file hashing.

### Changed

- **Breaking:** standalone installation now requires
  `completion_verifier`, validated before allocation and on restore. The current
  v1 host and confirmed-lifecycle schemas retain explicit verifier authority and
  evidence without a compatibility reader. Cross-release transitions remain
  reinstall-only; same-release receipt recovery remains available through fences.
- The standalone test target builds its consumer fixture explicitly.

## [0.3.0] - 2026-09-29

### Added

- Offline Caffeine SDK fault probes retain synthetic request/reply evidence for
  single/multiple chunks, ignored resume hints, non-complete status, lost final
  response, HTTP failure and traffic exhaustion. Native Rust verification checks
  original bytes and rejects corruption/truncation; uncertainty blocks replay.
  `make test-sdk-probe` is opt-in and makes no network or paid provider calls.
  Retained local artifacts are hash-checked by `make probe-check`; local evidence
  does not establish deployed completion, retention or billing behavior.
- Persistent Caffeine probe ledger and independent qualification policy. The
  `caffeine-probe` tool records intent before bounded anonymous source requests,
  retains raw responses and hashes in separate immutable runs, and verifies
  artifacts without treating failed/incomplete capture as provider qualification.
  Local HTTP evidence covers response limits, errors, redirects and interrupted
  records. Provider cooperation is no longer a prerequisite; supported behavior
  still requires evidence and explicit limits, with paid trials separately scoped.
- Authenticated `blob-storage funding-history` reads one bounded page through the
  shared service API and funding decoder. Exact scope, descending operation order,
  full-width amounts, refunds, uncertainty and restore fences survive JSON output;
  saved cursors bind the complete original scope before any network request.
  Native boundary tests and signed HTTP subprocess coverage exercise refusals,
  pagination and restoration without payment, automatic traversal or credit claims.
- Native `blob-storage status` authenticates to the shared service endpoint with
  an explicit PEM identity, operator and full installed scope. IC mode pins the
  SDK's IC root; local mode requires a separately trusted root and loopback URL.
  Verified query signatures, bounded transport/decoding, exact scope checks,
  decimal-string counters and separate restore fences preserve observation-only
  semantics. Signed HTTP subprocess evidence covers rejected trust/identity/scope,
  operator refusal and passive restored inspection. This unpublished CLI adds no
  mutations or provider calls; native dependencies stay out of Wasm builds.
- Standalone operator-only `blob_inspect_account` uses a shared handler for one
  bounded replicated Cashier balance or payer-relationship query. Exact installed
  scope and all restore fences are checked before dispatch and after the await;
  replies reuse the maintained provider encoders/decoders. Independent balances,
  signed relationship figures, missing relationships and provider errors remain
  distinct. No attached cycles, retry, funding, account change or cached authority.
  Native and PocketIC evidence covers linked payers, full-width values, caller and
  account mismatches, malformed/oversized replies, restore fencing and unchanged
  local accounting. Deployed Cashier behavior remains unqualified.

### Changed

- **Breaking:** the private browser gateway contract now requires
  `maxTotalRequestBytes` in configuration and retained scope. Both the hook and
  atomic store claims enforce aggregate outbound body bytes; uncertain requests
  stay charged to that budget. Consumers and the IndexedDB fixture use the current
  contract directly, with no old-scope fallback. Chromium/PocketIC regression
  coverage preserves cancellation, reload and competing-tab fences.
- Local reference recovery now uses the shared `blob_reference_receipt` API and
  bounded library decoder against standalone and durable storage hosts. The v1
  fixture journal retains independent full-width upload, object, lifetime and
  first-reference identities; changed original arguments conflict within the same
  operation slot. Service refusals remain distinct from missing receipts and
  recorded failures. Removed the transient fixture's private receipt endpoint,
  DTO, conversion and superseded tests; subprocess coverage follows saved intents
  through release, settlement and fenced restoration. Transport remains local
  PocketIC; no production authentication or mutation dispatch is introduced.

## [0.2.24] - 2026-09-29

### Changed

- **Breaking — requires a minor release:** standalone installation now takes
  `HostInstallationInput { configuration, project }`. The explicit Caffeine project
  is validated before allocation, retained in the current v1 configuration record,
  revalidated during restore and included in operator configuration readback. No
  project is inferred from a tenant, payer or local namespace. The previous host
  init/schema is replaced directly; cross-release transitions remain reinstall-only.

### Added

- Shared tenant-only `blob_reference_status` query in standalone and the durable
  storage fixture, replacing the fixture's private boolean endpoint. Exact upload
  and reference bindings return current local liveness and the restore fence,
  independently of historical retain/release receipts. The replicated reference
  client now validates bounded status replies. Native and PocketIC tests cover
  independent full-width IDs, changed bindings, suspension, release/settlement,
  passive reads and inspection after both service and client restoration.
- Standalone `blob_download_descriptor` update delegates to the existing shared
  exact-live-reference workflow using the installed project mapping. It returns
  original metadata only, with no provider/body call, read-session allocation or
  serving access for inactive tenants, unconfirmed content or restored owners.
  PocketIC coverage includes bounded inputs, project validation/restore corruption,
  an actual canister client and the existing confirmed-reference fixture journey.
  The standalone test target now builds its local consumer fixture too.
- Shared tenant-scoped `blob_lookup_content` query in the standalone host and both
  admission fixtures, replacing the private discovery endpoint. Indexed discovery
  reuses the maintained history conversion, preserving independent full-width
  upload, object, incarnation and first-reference identities through every local
  lifecycle phase. Responses echo the request and report restore fencing even for
  absence. The inventory tool validates those bindings and reports later fences;
  native and PocketIC tests cover isolation, passive reads, suspension, restoration,
  cleanup/settlement and actual inventory subprocesses.

## [0.2.23] - 2026-09-29

### Added

- Shared tenant/root-scoped `blob_reference_capacity` query in the standalone host
  and both admission fixtures. Correlated responses preserve unknown/foreign/
  unconfirmed absence, lifetime reference slots, reserved cleanup receipts and
  restore fencing. Replaced the private fixture DTO and conversions; inventory
  inspection validates exact replies and blocks on a later observed restore fence.
  Native and PocketIC tests cover caller isolation, suspension, cleanup at capacity,
  settlement, restoration, bounded ingress and inventory subprocesses.
- Shared tenant-only `blob_upload_capacity` query in the standalone host and both
  admission fixtures, reusing maintained global/tenant headroom calculations.
  Responses bind the tenant scope and report enrollment, object/metadata limits,
  lifetime history, concurrent slots, byte headroom and the restore fence. Native
  and PocketIC coverage includes full-width values, shared contention, cleanup,
  continuing billing, bounded ingress and passive suspended/restored inspection.
  Replaced the private capacity DTOs and conversions; the inventory tool consumes
  the shared response and reports restored services as blocked despite spare quota.
- Standalone `blob_upload_status` query using the shared exact-upload handler and
  maintained DTOs. Tenant-only history stays available through suspension and
  fenced restoration without granting current reference liveness or upload retry
  authority. PocketIC tests cover independent full-width identities, passive query
  and replicated execution, cancellation, caller isolation, changed original
  arguments and bounded ingress. The generated deployment Candid includes the query.

### Fixed

- Removed unnecessary `Result` wrappers from inventory test reply helpers so strict
  Clippy also passes when library unit-test targets are included.

## [0.2.22] - 2026-09-29

### Added

- Shared operator-only `blob_revoke_gateway` update in the standalone host and
  storage fixture, bound to the installed service, namespace, Cashier and payer.
  Removal invalidates older sync and read observations even for an absent member;
  it preserves occupied read slots, upload obligations and funding accounting.
  Repeated calls are fresh revocation decisions, not automatic receipt replay.
- Native and PocketIC evidence for caller/scope rejection, absent/final-member
  removal, stable-write rollback, delayed sync replies, remove/re-add during reads
  and restored mutation fences. The fixture's private removal command is replaced
  by the shared endpoint, with a separate local fault hook using the same handler.
  No provider credentials, deletion or billing state changes.
- Shared `blob_sync_gateways` and `blob_cancel_gateway_sync` operator updates in
  both hosts. Refresh persists its pending identity before one bounded replicated
  Cashier query; failures retain pending work, and cancellation targets the exact
  observed sequence. Standalone limits are 30 seconds and 64 KiB per reply, with
  independent decoder and membership bounds. No attached cycles or automatic retry.
  The fixture's private cancellation command is removed. Native and PocketIC tests
  cover malformed/oversized/rejected replies, overlap, stale cancellation, delayed
  callbacks, write rollback and fenced restoration. `make test-standalone` now builds
  the local gateway source fixture too; deployed-provider qualification remains open.

### Fixed

- Updated operator gateway-query tests for passive malformed, oversized and empty
  fixture replies. Client validation rejects each without changing pending sync
  state or source journals; caller, scripted-effect and restore refusals remain covered.

## [0.2.21] - 2026-09-29

### Added

- Shared operator-only `blob_funding_history` query in the standalone host and
  storage fixture. Bounded descending pages recover exact local operation IDs,
  attachment amounts, optional target balances and transport phases, with full
  service/namespace/Cashier/payer binding and an explicit restore fence. The host
  returns at most 32 entries; cursors grant no retry, credit or freshness authority.
- Native and PocketIC coverage for full-width attachments/refunds, caller/scope
  rejection, pagination, interrupted writes and retained uncertainty across restore.
  The fixture's private history endpoint, DTOs and conversion are removed; its
  related summary query now uses the shared operator scope. No funding dispatch
  endpoint or deployed-provider guarantee is introduced.
- Shared operator-only `blob_funding_outcome` query in both hosts, replacing the
  fixture's private outcome endpoint and conversion. Exact lookup checks the full
  original scope, operation, offer and optional target balance. Responses preserve
  transport phase, reported balance components, provider errors, decoder categories
  and restore fencing. Shared reconciliation keeps accepted or uncertain attachments
  unresolved independently of reported success. Native and PocketIC tests cover
  altered intents, absent/transport-only observations, delayed callbacks, write
  rollback and retained outcomes after restoration against local substitutes.
- `ReplicatedFundingClient` for single-call history and exact-outcome inspection
  from a pinned canister operator. Bounded reply validation checks scope, original
  amounts, descending cursors and refund/reconciliation consistency while retaining
  absence, service refusals and restore fences. No attached cycles, automatic
  retries, extra journal or lifecycle hooks. Native and PocketIC tests cover
  malformed observations, caller isolation, reply limits and both host restorations.

## [0.2.20] - 2026-09-29

### Added

- Standalone canister host with explicit installation configuration, seventeen
  exclusive ic-memory grants and synchronous assembly of the shared stores.
  A bounded host record retains configuration and package-release/service binding;
  upgrades load it without replacement inputs and leave every owner fenced.
- Tenant, upload admission/revocation, manifest preparation/inspection and reference
  endpoints delegate to existing shared workflows. Operator configuration readback
  grants no controller authority. The maintained Candid exposes typed requests even
  with custom bounded decoding; a schema check ignores explanatory comments.
- Focused standalone build and PocketIC targets. Actual Wasm tests cover the 10 MiB
  manifest path, caller isolation, stop/start, retained state and fenced mutation,
  failed-install rollback, ingress bounds and foreign/missing/release-mismatched
  restore rejection. Provider effects, operational recovery and the Canic adapter
  remain unfinished; this host does not yet provide a complete storage journey.
- Shared operator-only `blob_local_status` query, exported by the standalone host
  and storage fixture. Explicit service/namespace/Cashier/payer binding and matching
  owner configurations precede a synchronous snapshot of maintained upload totals,
  funding allocation/outcomes, bounded gateway membership and read occupancy.
  Separate restore fences remain visible; inspection makes no provider calls or
  claims about provider credit, platform liquidity or readiness. Native and PocketIC
  checks cover caller/scope rejection, full-width accounting, cancellation history,
  refunds and uncertain funding, pending sync and interrupted reads across restore.
- Shared `blob_upload_history` query for tenant-scoped or operator-wide bounded
  discovery of retained operations. Responses preserve independent full-width
  upload/object/incarnation/first-reference identities, current cleanup state and
  the restore fence. Scope-bound cursors advance through empty filtered pages;
  callers cannot choose work limits. The standalone host scans at most 64 rows
  and returns at most 32 entries per call. The fixture's private scan DTOs and
  endpoint are replaced by the shared handler. PocketIC tests cover isolation,
  cancellation, suspension, cursor rejection, fresh sweeps and continuing billing
  after physical deletion; history remains passive through restoration.

## [0.2.19] - 2026-09-29

### Added

- Shared scoped tenant enrollment API and handlers for operator compare-and-set
  updates and tenant/operator inspection. Requests bind service, namespace and
  tenant; responses retain that scope and report the restore fence. Suspension
  preserves lifetime capacity and obligations, while reactivation advances the
  existing permission generation. Lost replies require inspection before choosing
  another command; inspection does not prove historical execution.
- The storage fixture now exports `blob_update_tenant` and `blob_tenant` through
  the shared handlers, replacing its private enrollment endpoints and conversion.
  PocketIC coverage checks caller isolation, malformed/stale preconditions,
  capacity during suspension, enrollment write rollback and fenced restoration.
  Production adapter/lifecycle ownership and provider qualification remain open.
- Replicated tenant client with explicit executing canister and pinned tenant scope,
  bounded waits/replies and no automatic retry or query-to-update fallback. Update
  acknowledgments must match the existing model's exact generation/state transition;
  inspection preserves absence and restore fences without claiming historical execution.
- Local operator-client evidence uses the existing consumer fixture's bounded store
  to retain one command before dispatch. Success, unusable replies and callback
  interruption cannot reopen dispatch; restoration preserves the command for
  inspection. Stale activation state and generation are rejected without changing
  service storage, and rejected commands remain retained across client restoration.
  This fixture is not a production operator journal or identity provider.

## [0.2.18] - 2026-09-29

### Added

- Shared host configuration input and bounded Candid decoder. Explicit service,
  operator, payer, namespace, resource, billing, funding allocation and read-session
  inputs reuse existing model validation before state allocation. The host's actual
  service identity must match. Invalid limits are rejected without defaults or
  clamping, and balances and byte budgets preserve their full numeric width.
- Shared synchronous assembly of upload, funding, gateway and read-session stores
  with explicit host memory grants. All store envelopes are validated before
  allocation; any occupied grant refuses fresh installation, and restoration never
  initializes a missing store. The fixture's separate initialization paths are
  removed. Changed funding/read limits reject without modifying retained bytes;
  restored stores remain inspection-only.
- Targeted coverage for malformed/oversized configuration, role bindings, resource
  relationships, funding/read budgets and occupied/missing memory grants. A combined
  PocketIC journey preserves upload obligations, uncertain funding, pending gateway
  sync and interrupted read occupancy through same-release restoration, with
  mutations refused afterward. Production adapters and deployed Caffeine
  qualification remain open.

## [0.2.17] - 2026-09-28

### Added

- Browser gateway transport guard around Caffeine's existing fetch hook. Caller-owned
  storage commits bounded request fingerprints before dispatch, atomically with
  cancellation checks. A retained execution fence prevents competing tabs or reloads
  from restarting a claimed transfer; failed or uncertain requests cannot be retried.
  HTTP responses record history without confirming provider completion or billing.
- Chromium/PocketIC coverage for gateway journal write aborts, lost responses,
  failed observation writes, oversized replies, request capacity and cancellation
  before claim/after dispatch. Reload and certificate recovery preserve gateway
  history. Tests use a local gateway substitute and fixture IndexedDB store;
  production persistence, reconciliation and deployed qualification remain open.
- Browser admission now exercises signed ingress through the existing consumer
  canister fixture, which persists asset intent and admits as the actual tenant.
  The browser then prepares the manifest directly as the uploader. Unrelated
  identities and direct uploader admission are refused. Candid encoding and reply
  validation reuse the Rust types; no JavaScript service schema is duplicated.
  This is local integration evidence, not a production Toko endpoint or identity flow.
- Browser reference journeys now attempt consumer registration after gateway outcomes
  and explicitly persist consumer cancellation before tenant withdrawal. HTTP success
  cannot publish unconfirmed content; withdrawal preserves exposed reservations and
  browser request history. Shared authorization probes run once across the fault
  scenarios instead of repeating the same checks for every transport interruption.

## [0.2.16] - 2026-09-28

### Added

- Reusable browser certificate client with caller-owned authentication, explicit IC
  trust and an atomic durable-intent store contract. Issuance saves the exact signed
  request before dispatch; recovery verifies its historical reply without reissuing,
  and cancellation remains permanent. Saved permission, network trust, uploader,
  service, method and root are checked before accepting observations.
- Browser upload composition now uses the actual Caffeine 1.1.2 package and its
  supported IC SDK 5.4.0 through the existing agent hook. A pinned, hash-checked
  patch exposes network-free preparation and per-client transport/cancellation/
  retry controls; upstream still owns hashing, chunks, certificate extraction and
  HTTP formats. Preparation snapshots caller bytes and uses immutable, single-use
  handles. The private fixture applies the patch to a generated package copy and
  verifies the pinned source hashes before building.
- Bounded conversion of Caffeine's prepared manifest into the service's existing
  declaration, reusing metadata and root validation without another tree builder.
  Browser evidence now supplies the actual prepared JSON to tenant admission and
  uploader preparation in PocketIC before receiving the permission; tenant-only
  preparation is refused. File bytes remain in the browser.
- Chromium/PocketIC coverage now imports the reusable client and Caffeine package.
  Tests cover competing tabs, lost replies, cancellation, changed saved bindings,
  invalid proofs, single-use handles, failed gateway requests without retry and
  abort before issuance/after the tree. Decoder tests reject oversized, malformed
  and inconsistent manifests. Gateway responses remain local substitutes;
  production consumer authentication/storage, durable gateway-effect coordination
  and deployed Caffeine qualification remain open. The browser package is private.

## [0.2.15] - 2026-09-28

### Added

- Headless certificate evidence through PocketIC's real HTTP v4 ingress API, using
  an actual signing identity and the official IC agent. Tests extract the raw
  certificate, verify its signature/delegation/time and bind the saved request ID,
  uploader, service and expected root. Forged, unrelated, malformed, oversized and
  rejected-response proofs cannot become upload certificates.
- Saved ingress-request recovery obtains the original certified reply without
  reissuing or changing service state. Historical proof survives local revocation
  but does not renew permission or authorize a gateway retry. New cryptographic
  dependencies belong only to the unpublished native test harness; production
  browser integration and deployed Caffeine acceptance remain open.
- Opt-in Chromium certificate/IndexedDB evidence using the pinned current IC JS
  SDK. A bounded two-slot fixture commits the original permission and signed
  envelope before dispatch, serializes competing tabs, and preserves cancellation
  through a held response, tab closure and reload recovery. Aborted writes send
  nothing; stale, forged or oversized proofs cannot change retained intent.
  `make test-browser` runs the local test without downloading dependencies. This
  is browser integration evidence, not a production journal or provider upload.

## [0.2.14] - 2026-09-28

### Added

- Shared guarded upload-exposure workflow with exact permission binding and
  independent host checks for pre-charge limits, provider namespace, replay charging,
  recovery eligibility and durable commit. Missing or stale observations block
  writes; commit rechecks actual uploader, preparation, activation, deadline, phase
  and restore fencing. Preview results cannot authorize a later mutation.
- Exact tenant/uploader exposure-history recovery and local IC evidence for
  permission-write rollback, traps before update completion, and lost committed
  acknowledgments. Repeat exposure is rejected, while revoked uncertain uploads
  remain charged and inspectable through restore. The storage fixture now uses
  the shared gate with explicitly labelled provider-evidence substitutes. Actual
  evidence acquisition and deployed issuance remain open; no provider effect,
  new stable schema or additional memory grant is introduced.
- Shared Caffeine certificate-response handler resolves a root to its original
  permission, authenticates the actual uploader and commits guarded exposure
  before returning the reviewed plain `upload`/`blob_hash` reply. Linking exports
  no endpoint. Local IC tests cover default refusal, malformed/foreign requests,
  evidence mismatch, write/reply rollback, lost acknowledgment and fenced restore.
  Provider acceptance, certificate replay and pre-charge enforcement remain
  unqualified; the test adapter uses explicitly configured substitute evidence.

## [0.2.13] - 2026-09-28

### Added

- Replicated upload-manifest client with explicit actor, tenant and service bindings.
  Preparation validates the bounded declaration before dispatch; inspection recovers
  the original declaration. Calls send once, attach no cycles and never retry
  automatically. Only the actual admitted uploader may prepare; the tenant may
  also inspect. Query execution and changed bindings are refused.
- Durable uploader intent in the local application fixture, exercised by a separate
  uploader canister. Exact intent and dispatch state survive unusable replies and
  callback traps; uncertainty blocks redispatch until inspection finds the accepted
  declaration. Typed refusals remain recorded, and unprepared observations cannot
  clear uncertainty. Bounded history, conflicting intents, stale observations and
  fenced upgrades are covered. A three-canister journey joins tenant admission,
  uploader recovery, first-reference registration and cleanup. Exposure/completion
  remain labelled substitutes; browser integration and provider qualification are open.
- Uploader cancellation now retains a permanent local tombstone, original intent
  and uncertain/accepted/refused preparation history. It blocks redispatch without
  claiming tenant revocation or freeing service capacity. Recovery and delayed
  acknowledgments preserve cancellation; saved intent cannot reopen it. Local IC
  races cover tenant withdrawal during a held uploader reply, unexposed reservation
  cleanup, exposed late completion and reference release under suspension, plus
  cancelled history through fenced upgrade. This extends the same application fixture.

## [0.2.12] - 2026-09-28

### Added

- Shared tenant upload revocation through `blob_revoke_upload` and the replicated
  permission client. The full original permission, including uploader and expiry,
  is checked before mutation. Unexposed cancellation releases reservation bytes;
  possible exposure and confirmed references preserve their storage and billing
  obligations. Replies distinguish first withdrawal from exact replay and require
  positive revocation evidence. Suspension permits cleanup; restore fences it.
- Consumer cancellation now retains a revocation intent before actual IC dispatch.
  Lost or unusable acknowledgments remain pending until exact inspection; typed
  refusals remain recorded. Local IC evidence covers callback and stable-write
  rollback, late completion, caller/permission isolation and uncertain withdrawal
  through upgrade. The storage probe's private revoke endpoint is replaced by the
  shared contract. This withdraws local issuance permission, not a provider object;
  provider qualification remains open.
- Shared uploader manifest preparation and exact declaration inspection through
  `blob_prepare_upload` and `blob_upload_manifest`. Full original permissions and
  raw declaration bounds precede conversion; root/length validation uses the
  existing Caffeine algorithm. Equivalent reordered retries preserve the first
  metadata and leaves. Tenant/uploader recovery remains available after expiry,
  revocation, exposure and fenced restore. Bounded reply decoders reject changed
  permissions and malformed declarations. The storage fixture now uses this
  contract, including write-trap recovery. This establishes declaration consistency,
  not stored bytes, provider completion or a production uploader client.

## [0.2.11] - 2026-09-28

### Added

- Authenticated upload-status inspection through shared DTOs, a durable handler
  and a bounded replicated IC client. Replies bind the complete original upload
  and distinguish reservation, possible exposure, confirmation and cancellation.
  Historical confirmation survives release, settlement and restore; it does not
  establish current reference liveness or authorize retrying an uncertain effect.
- Fresh-upload registration in the local consumer fixture uses the first
  reference created by completion, with no extra retain or receipt. Intent can
  be persisted before admission. Cancellation preserves uncertain
  uploads for reconciliation and exact cleanup, including late completion under
  suspension. Native and PocketIC evidence covers stale observations, conflicting
  identities, interrupted callbacks, unchanged storage during registration and
  fenced upgrades. Provider completion remains labelled test-host work;
  production application integration and operational recovery are still open.
- Canonical tenant upload admission and exact permission inspection now share
  library DTOs, durable handlers and a bounded replicated client. Recovery binds
  uploader and original expiry as well as every upload field; replay cannot renew
  permission or reserve additional capacity. The consumer fixture persists that
  permission before actual IC dispatch, retains typed refusals and reconciles
  uncertain acknowledgments by inspection without resending. The storage probe's
  private admission endpoint is replaced. Native and PocketIC tests cover changed
  permissions, expiry, suspension, reply limits, write/callback rollback and
  upgrade fencing. Manifest preparation, revocation transport and deployed-provider
  qualification remain open; no certificate or paid provider effect is added.

## [0.2.10] - 2026-09-28

### Added

- Shared authenticated reference-receipt inspection with library-owned DTOs and
  an explicit replicated IC client. Exact original upload, object, lifetime,
  reference and operation arguments bind the returned historical success or
  failure. Absence uses an explicit wire variant so malformed optional data
  cannot silently become a missing receipt. Bounded decoding, actual tenant
  identity and service targeting precede disclosure; no mutation fallback,
  automatic retry or attached cycles are introduced.
- The durable storage probe uses the canonical receipt query in place of its
  private lookup. Native and two-canister PocketIC evidence covers independent
  full-width identities, recorded failures, conflicting arguments, reply limits,
  caller isolation, atomic rollback and receipt retention through release,
  suspension, settlement and fenced upgrade. Historical success and absence
  remain distinct from current liveness, publication and retry authority.
- Canonical retain/release updates now use the same exact command and shared
  durable handler as receipt recovery. The explicit replicated client sends once,
  validates the returned operation and preserves recorded inner failures and
  replay status. Fresh retains require active enrollment; cleanup and exact
  replay remain available under suspension and receipt pressure, while restored
  owners reject mutation. The private mutation endpoint/DTO/conversion is removed.
  Local IC evidence recovers a committed mutation after an unusable reply without
  changing accounting on retry, and preserves rollback, isolation and cleanup.
  Production consumer intent persistence, publication transactions and outboxes
  remain application-owned integration work.
- A separate local consumer canister now exercises existing-content registration
  through the shared retain, descriptor and receipt clients. Its bounded stable
  intent reserves cleanup identities before dispatch; atomic publication,
  dependency checks and tombstones prevent cancelled registrations from reviving
  assets. Interrupted retain/publication/release callbacks retain repairable
  evidence, and upgrade preserves unresolved work under an inspection-only fence.
  Native and PocketIC tests cover these races, exact payload conflicts and cleanup
  at lifetime capacity. This is an application substitute, not a Toko adapter,
  production client journal or fresh-upload journey. Make targets include it.

## [0.2.9] - 2026-09-28

### Added

- Bounded durable read sessions with separate global/per-tenant slot and reply
  buffer budgets. Three host-granted memories retain exact chunk/reference/gateway
  intent, active accounting and a never-reused local sequence. Shared admission
  checks current authority and chunk range before committing; exact callback
  completion releases only its own reservation and rechecks disclosure authority.
  Revocation, elapsed time and dropped tickets do not release occupied capacity.
  Restore validates retained rows/counters and remains inspection-only.
- The local read fixture now uses the shared durable journal in place of its
  temporary busy flag. Native and PocketIC evidence covers independent count/byte
  limits, stale callbacks, exhaustion, authority invalidation, atomic admission
  and callback failures, transport rejection and retained occupancy after upgrade.
  Allocator and existing upload/funding/gateway schemas are unchanged.
- Shared asynchronous chunk reads now combine durable admission, one host call,
  current callback authority and exact manifest-leaf verification. Peer/root/index,
  reply budget, length and hash must match before bytes are returned. Verification
  reads the selected leaf from the bounded immutable manifest without rebuilding
  its tree or copying operation histories. The local IC fixture fetches actual
  chunks, bounds decoding and uses bulk byte decoding; malformed/corrupt replies
  settle occupancy without disclosure, while callback traps retain it through
  upgrade. The private probe reuses the existing locked `serde_bytes` dependency.
  Deployed Caffeine transport, resource sizing and operational recovery remain open.
- Operational download descriptors bind the current service owner, explicit
  provider project/local namespace and exact live tenant reference to the stored
  root, length and original hash metadata. Suspended/restored instances refuse
  delivery while passive inspection remains available. The shared workflow emits
  the reviewed Caffeine direct-blob request target with bounded, escaped fields;
  no default project, origin, provider call or canister body transfer is introduced.
  Source/package review confirms the current client download fields; provisioned
  mappings, approved HTTP policy and production authenticated delivery remain open.
- Library-owned descriptor request/response/error DTOs and one shared endpoint
  handler replace the private fixture descriptor schema. An explicit replicated
  client calls `blob_download_descriptor` once from the actual tenant canister,
  bounds Candid decoding and checks the complete original reference, owner,
  project, declared size and hash metadata. No credentials, server-selected URL,
  cycles attachment, method fallback or automatic retry is introduced. Local
  two-canister evidence covers caller identity, query refusal, limits, suspension
  and restored service refusal. Browser delivery and publication/reference
  coordination remain open.

## [0.2.8] - 2026-09-28

### Added

- Explicit replicated IC transport for the canonical Cashier gateway-list query.
  It checks service/Cashier binding and execution mode, sends once with a bounded
  timeout and no attached cycles, and checks reply size before handing the owned
  buffer to the decoder. Ordinary platform costs still apply. PocketIC exercises
  the query-only endpoint, wrong bindings, non-replicated refusal, rejection,
  oversized replies, callback rollback and restore fencing. Current public
  Caffeine source/package/interface observations remain unchanged; a live
  replicated Cashier call and deployed-provider guarantees remain unqualified.
- Shared scoped gateway root observations compose current durable membership,
  matching owner configuration, stored object bindings and both restore fences.
  Bounded indexed reads return local phases without tenant/operation details,
  preserve pending uncertainty and reject revoked callers even for empty batches.
  Native and PocketIC evidence covers independent fences, corrupt indexes, caller
  isolation, membership replacement and all cleanup phases. These observations
  grant no provider deletion, completion, retry or read-session authority.
- Shared read-authority capture/recheck binds the original tenant, exact live
  reference, object lifetime, root and selected gateway. A durable invalidation
  counter prevents gateway removal/re-addition or unchanged-list syncs from
  reviving old reads; tenant suspension/reactivation also invalidates them.
  Membership and its counter commit together, with exhaustion blocking reads
  while preserving revocation. Native and held-reply PocketIC tests cover these
  checks, write rollback and both restore fences. Bounded session admission,
  one-shot completion and production read transport remain separate work.
  The internal v1 gateway record is replaced directly; cross-release installs
  remain reinstall-only, with no compatibility reader or allocator change.

## [0.2.7] - 2026-09-28

### Added

- Durable gateway membership and pending sync state through one explicit
  host-granted memory. The bounded v1 record reuses existing gateway validation
  and revocation rules: operator edits invalidate older replies, invalid lists
  leave pending state intact, and empty membership remains possible by removal.
  Scope, operator and installed limits are checked on access/restoration; restored
  registries remain inspection-only. Native and PocketIC coverage includes full
  record bounds, stale replies, write rollback and retained pending syncs through
  upgrade. Membership alone grants no provider callback or recovery authority.
- Shared durable gateway sync now binds the canonical Cashier query to its pending
  identity and applies encoded replies through the existing bounded decoder.
  Authority, restore fencing and request/source/token checks precede parsing;
  failed replies preserve the pending attempt for explicit cancellation or a valid
  response. Native and PocketIC tests cover decoding refusal, stale correlation,
  atomic reply-write rollback and restoration. The labelled probe uses this same
  workflow; authenticated provider transport remains integration work.
- Shared async gateway query orchestration checks the durable attempt on polling,
  releases the registry borrow across host transport, and revalidates on completion.
  Transport/source failures retain pending state; delayed replies cannot overwrite
  operator edits or a newer sync. PocketIC covers real delayed local calls, callback
  write traps and fenced restoration. Transport is explicitly host-supplied; the
  local scheduling substitute does not qualify Caffeine's deployed query endpoint.

## [0.2.6] - 2026-09-27

### Added

- Shared guarded funding dispatch now owns first-attempt admission, durable
  marking, post-write platform liquidity checks, one canonical unbounded call and
  callback settlement. Host observations are acquired synchronously on polling;
  missing holds refuse without mutation. Liquidity refusal records a positively
  unsent call, while callback failure preserves uncertainty after remote acceptance.
  PocketIC exercises this complete handler against the labelled local Cashier
  substitute, including malformed replies, delayed replies with duplicate refusal,
  and write traps. Production evidence acquisition and deployed-provider
  qualification remain open.
- Shared first-attempt admission binds fresh host observations to the complete
  retained request. It recognises that request's own reservation without charging
  it twice or requiring another history slot, while preserving older acceptance,
  external uncertainty and restore fences. Only an exact unattempted intent can
  receive its first durable marker; uncertain and terminal intents cannot retry.
  Blocked calls preserve storage and reservations. Native and PocketIC coverage
  exercises full history, identity exclusion, missing evidence and restoration.
  Marking sends no call; qualified host evidence and post-write platform liquidity
  checks remain required when composing production dispatch.
- Shared funding preparation checks exact scope and identity, current journal
  obligations, history capacity and the restore fence alongside independently
  supplied host evidence. Unknown provider qualification, recovery, spendability
  or complete account activity blocks reservation; external clearance cannot
  override local liabilities. The update re-reads current state synchronously,
  accepts no saved preview and leaves blocked requests unchanged. Native and
  PocketIC coverage includes changed state, scope/caller isolation, full history
  and fenced restoration. Host evidence acquisition and production dispatch
  remain separate requirements; preparation sends no provider call.

## [0.2.5] - 2026-09-27

### Added

- Scoped funding summaries expose complete local attachment totals, lifetime
  history capacity and the restore fence without scanning intent rows. Shared
  policy keeps earlier accepted or reserved amounts unresolved after later
  refunds or unsent attempts. Native and PocketIC cases cover scope/caller
  isolation, rollback, actual IC acceptance and fenced upgrade inspection.
  A clear local summary does not establish complete provider-account activity,
  verified credit or payment authority.
- Durable funding observations retain structured Cashier replies and exact
  transport accounting together. Operator outcome lookup reports balances,
  errors or missing replies separately from conservative reconciliation needs;
  accepted cycles still require independent credit evidence. Original account,
  attachment and optional target are checked against the shared call before
  recording. Exact replay cannot refund twice or overwrite a different reply.
  Native and PocketIC coverage includes bounded record widths, response/phase
  validation, callback rollback, mismatched identities and fenced upgrade reads.
  The v1 intent schema is replaced directly under the reinstall-only contract.
- Explicit shared Cashier transport sends the canonical top-up request once and
  captures its exact unbounded-call refund before decoding. Call cost excludes
  the attachment; same-message liquidity checks preserve the full offer. Replies,
  rejects and accepted cycles remain distinct. PocketIC connects the durable
  journal to a local Cashier substitute, covering zero/partial/full acceptance,
  malformed/error replies, receiver and callback-write traps, caller isolation,
  blocked retries and fenced upgrade. This supplies local IC transport evidence;
  deployed-provider qualification, account-wide activity and credit reconciliation
  remain required before production payment admission.
- `StableFundingJournal::history` provides bounded operator-only discovery of
  original funding intents and current local outcomes, newest first, without
  requiring a saved request. Service/Cashier/account/namespace checks bind each
  query and continuation cursor. Reads preserve reservations and remain available
  under the restore fence. Native and PocketIC cases cover full-width identities,
  pagination, changing observations, caller isolation, rollback and upgrade;
  discovered requests confer no payment, retry or unfencing authority.

### Changed

- Reduced the 704-object PocketIC release-history test's overhead by queuing
  independent updates as separate IC messages and reading bounded diagnostic
  windows. Every reply, measurement sequence, caller and instruction bound is
  checked; full history, exact retries, restart and cleanup coverage remain.
  Fixture content preparation uses the existing manifest builder to avoid
  duplicate leaf hashing. No service API, allocator or dependency change.

## [0.2.4] - 2026-09-27

### Added

- `StableFundingJournal` persists exact local attachment intents, first-attempt
  markers and terminal transport observations through two host-granted `ic-memory`
  stores. Full offers are reserved before attempts; pending/uncertain intents block
  later reservations. Exact replay cannot repeat an attempt or refund twice, and
  conflicting outcomes reject. Incremental accounting shares the existing funding
  reconstruction rules, retaining accepted and unresolved amounts independently
  of provider credit. Reopen checks complete bounded history and totals without
  repair, then fences every mutation. Native and PocketIC cases cover corrupted
  history, overflow, interrupted writes and every phase through upgrade.
- `CashierTopUpRequest` owns canonical `account_top_up_v1` encoding. Its method,
  explicit account, optional target balance and exact attachment are bound to the
  retained intent. Request inspection
  survives fencing; changed arguments and mismatched transport service/target
  reject without releasing reservations. Independent Candid vectors and PocketIC
  cover request encoding, correlation and upgrade preservation.

### Changed

- Updated the test dependency `ic-testkit` to 0.10.1.
- Made the pre-1.0 policy explicit in `AGENTS.md`: 100% hard cuts, complete
  removal of superseded contracts and no compatibility or migration paths.
  Same-release recovery and obligation-preservation requirements remain in force.

Provider authentication, spendability and operational recovery remain open;
no payment dispatch is enabled.

## [0.2.3] - 2026-09-27

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
