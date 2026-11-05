# Engine release notes

Packages: `ic-blob-storage`, `ic-blob-storage-canister`, `blob-admission-probe`, `blob-authority-probe`, `blob-consumer-probe`, `blob-funding-probe`, `blob-gateway-source`, `blob-storage-probe`, `blob-test-protocol`, `ic-blob-storage-pocketic-tests`.

Independent releases start after coordinated 0.65.0; [shared history](../history.md) retains earlier entries once.

## [0.73.0]

### 💥 Breaking

- Align the pending family with Contracts 0.73's removal of the external
  certificate reply's unused Wire metadata implementation. Public contract
  consumers select the same Contracts cohort; Caffeine Candid bytes, endpoint
  names, application Wire frames and saved records are unchanged.
  [#117](https://github.com/dragginzgame/ic-blob-storage/issues/117).

### 🔧 Changed

- Adopt Testkit 0.87's async-only native client. Await fixture installation,
  calls, callbacks and recovery checks on caller-owned runtimes; keep isolated
  server custody and remove the older native Platform facade from the graph.
- Adopt published Host Tools 0.83 for native deployment-interface inspection;
  its removed certificate codec has no local caller. Preserve the existing
  Platform-owned certificate boundary and installed-test interface contract.
- Compact the evidence diaries and retire superseded per-run reports. Keep
  benchmark conclusions and exact historical citations; preserve provider,
  recovery and maintained fixture provenance under their existing owners.
- Adopt committed Shared Tooling 0.30.4: enforce the Candid source boundary
  through the existing Platform check, with exact external-protocol declarations;
  retain the Bash 3.2 LOC fix and bounded experimental-evidence guidance.
  Correct undated release-candidate
  projection, default formatting/LOC scratch to its consumer, and refresh LOC
  presentation and command help. Classification diagnostics remain in JSON.
- Select the compatible Storage 0.67.1 patch, preserving the supported minimum,
  stored formats and allocation behavior.
- Adopt published Platform 0.23.0's native IC protocol codecs and remove the
  direct CBOR dependency and temporary test exception. Preserve untagged
  certificate bytes, bounded decoding, original request IDs and ingress
  forgery/refusal/recovery checks; application DTOs remain Wire.
  [Platform #143](https://github.com/dragginzgame/ic-platform/issues/143).
- Select the compatible Foundation/Build 0.74.4 patches with Platform 0.23.

### 🧪 Testing

- Align opt-in browser consumer checks with trusted registration: publication
  records retention intent, cancellation clears publication, and withdrawal
  preserves the first reference and provider-byte liabilities.

## [0.72.2] - 2026-11-05

### 🔧 Changed

- Adopt committed Shared Tooling 0.30.0, rename private probe directories to
  `canisters/tests/`, and declare fixture/tooling LOC roles. Categorized counts
  cover all workspace members without changing package identities or wire bytes;
  workspace table labels no longer repeat an aggregate total.
- Keep active browser builds, generated schemas, reports and local fixture
  defaults beneath `target/`; retain historical evidence and explicit overrides.
- Replace the private read profiler's Metrics recorders with checked numeric
  totals, preserving exact-cap and sticky-overflow reports while removing the
  unused instrumentation dependency. Operational reporting remains consumer-owned.

## [0.72.1] - 2026-11-05

### 🩹 Fixed

- Admit retained release lint during publication with its isolated archive build
  directory, keeping exact source/proof/closure checks and fresh lint fallback.
  The wrapper regression covers archive isolation and missing-proof recovery
  ([#116](https://github.com/dragginzgame/ic-blob-storage/issues/116)).

## [0.72.0] - 2026-11-04

### 💥 Breaking

- Adopt published Storage 0.67.0, Wire 0.17.1 and Testkit 0.86.0, aligning the
  public Storage configuration/codec-error and Wire type identities with
  Contracts 0.72. Codec bytes, endpoint identities, resource budgets and stored
  layouts are unchanged; this dependency cut introduces no reader, conversion,
  reset or new authority for existing provider obligations.

### 🔧 Changed

- Adopt committed Shared Tooling 0.29.6 and qualify exact final-candidate stage
  reuse across supplier-first group releases on Linux/x86_64. Retain separate
  source-bound group receipts and fresh actual metadata, mixed-version identity,
  archive and registry admission; unsupported hosts/configuration execute normally.
- Admit completed candidate Clippy for new publication only when retained source,
  proof/log custody and the actual primary's complete closure match. Exact verified
  registry retries finish without repeated compilation. Missing qualification runs
  fresh lint; no archive/upload admission is skipped. Qualify real native execution,
  later group admissions and publication ordering in focused fixtures.
- Run selected installed-test partitions in one sequential, fail-fast Make graph,
  preparing their Wasm/CLI artifacts once. Preserve Client-only release selection
  and independent partition commands without preparation stamps or skip flags.
- Split archive-verifier regression from actual package verification. Omit the
  regression only for admitted product changes with unchanged verifier/CLI inputs;
  keep it for dependency/configuration drift and uncertainty, and always verify
  actual applicable package archives. Bind hosted checks to native selected plans.

## [0.71.1] - 2026-11-04

### 🔧 Changed

- Adopt committed Shared Tooling 0.29.4 and its retained-state guard. Apply the
  canonical guard to direct group verification and publication as well as the
  shared release runner; preserve evidence on refusal and invalidate only a new
  selectable receipt when post-validation admission fails.
- Qualify published Testkit 0.85.1, Platform 0.22, Host Tools 0.82 and Wire 0.16.2.
  Existing managed server custody supplies Testkit's now-required explicit URL;
  keep server 16.1.0, service frames, storage bytes and lifecycle semantics.

- Align hosted CI with selected group plans: omit the broad native-host rerun
  and separate portable Rust regressions on manual group dispatch. Keep isolated
  feature graphs and generated-client qualification. Give portable Contracts
  tests one Linux owner and remove duplicate schema-job Clippy; normal native
  CLI/installation/restoration coverage remains on all three hosts.

## [0.71.0] - 2026-11-04

### 💥 Breaking

- Explicitly select Contracts’ `caffeine-candid` capability for full service/provider integrations. Keep external Caffeine codecs, callback identities and bounded decoding unchanged; passive clients can link Contracts without Candid.
  [#115](https://github.com/dragginzgame/ic-blob-storage/issues/115).

- Remove the standalone `blob_metrics` endpoint, heap recorders, lifecycle resets, response-attribution workflow and instrumentation-only endpoint wrappers. Drop the standalone Metrics dependency and enforce its exclusion from the deployment graph. Preserve service accounting, provider callbacks and their authority/replay/liability tests. Canic owns operational reporting; private probe resource measurements remain qualification-only.

## [0.70.0] - 2026-11-04

### 💥 Breaking

- Remove the feature-selected heap `BlobCatalog`/`UploadCatalog` owners, their
  private accounting, heap read/cursor APIs and unused catalog failures. Use
  `StableUploads` and its indexed reads, exact receipts and explicit installation
  boundary. Authority probes now exercise that production implementation with
  valid manifests over disposable governed backing; their independent archive
  remains fenced inspection evidence. Pure lifecycle rules remain. Retire the
  old model-only tests in favor of maintained durable-owner and actual-caller
  coverage. Production stored bytes, endpoints and provider obligations are
  unchanged; this is a Rust API hard cut with no compatibility owner or migration.

- Adopt Metrics 0.29 and Platform 0.21 with the matching Contracts family. Keep
  instrumentation local to standalone/probe hosts and project passive report
  fields explicitly. Existing record bytes, quotas, retries and provider
  obligations remain intact. Cross-release adoption still requires retirement
  with original tools and fresh backing; no automatic reset or migration.

### 🔧 Changed

- Give the multi-phase installed publication-session scenario a bounded 120-second
  test budget under concurrent load and report unexpected control replies in
  assertions. Production deadlines and the separate idle-expiry check are unchanged.

- Adopt committed Shared Tooling 0.29.3, including Testkit's native client
  boundary, grouped manifest admission, UTF-8 validation evidence and pinned
  revision export that preserves unrelated source edits. Keep issue outcomes in
  their owning group ledger rather than duplicating them in the root index.
  [Shared #351 — Transfer the native PocketIC dependency boundary to Testkit](https://github.com/dragginzgame/shared-tooling/pull/351),
  [Shared #353 — Export a pinned commit from the existing local repository without requiring a clean checkout](https://github.com/dragginzgame/shared-tooling/issues/353).

- Extract inline model, policy, provider and probe unit tests into owner-local
  `tests.rs` files, preserving module identities, private access and fixtures.
  Existing hierarchical tests remain in `tests/`; feature-selected public probe
  support remains a distinct API rather than being hidden as unit tests.

- Adopt published Testkit 0.84.1 and Host Tools 0.81 for native fixtures. Route simulator client,
  DTO/error and lifecycle imports through Testkit and remove the separate
  Platform PocketIC client dependency. Keep the 16.1.0 server, original typed
  failures, scenario authority and recovery policy; deployed code is unaffected.

- Select the compatible Storage 0.66.1 patch with unchanged production sources
  and stored/wire formats, retaining its existing compatible manifest minimum.
- Clarify the released 0.69 retirement and acceptance procedure, including the
  matching consumer dependency family and the separation between local rehearsal
  and explicitly bound live-provider acceptance.

- Document the separate payer, service-owner and gateway credit checks before
  provider exposure. Retain the current real-provider 403 and exact funding/link
  evidence; account funding or registration alone does not prove stored bytes.
  Record the subsequently observed delayed owner allocation and one successful
  second-tenant browser upload/root-verified download without rewriting the
  earlier refusal, retrying its operation or clearing retained liabilities.
  Qualify the subsequent exact disposable-object release/receipt/liveness against
  the deployed standalone: logical quota drops while physical/billing liability
  remains, and the still-available public URL does not establish deletion.

## [0.69.0] - 2026-11-03

### 💥 Breaking

- Align the published Wire 0.16 / Platform 0.20 / Runtime 0.19 / Types 0.18 family,
  with Tools 0.80 and Testkit 0.83. Preserve endpoint bytes and the single executor
  identity. Testkit callers handle typed client failures, preserving uncertainty
  and distinguishing real canister rejections from failed transport.
- Adopt Storage 0.66's public Wire 0.16 codec identity, removing the remaining
  Wire/Derive 0.15 dependency. Storage's ledger/collection bytes and reopening
  contract are unchanged. Adopt Metrics 0.28 with Blob-owned report schemas;
  recording, numeric validation, report bytes and heap lifecycle are unchanged.
- Select native simulator types directly from the independent Platform PocketIC
  0.20 client, matching published Testkit 0.83 without coupling the facade's
  version or feature selection to that client. Keep one simulator/runtime/type
  identity and preserve typed failures and uncertain-submission handling.
- Store funding transport progress and its permitted diagnostic in one bounded
  state, removing the independently persisted phase/response combination. The
  fresh `BLFUPR01` record identity and installation format reject old backing;
  retire existing obligations with their original tools before a fresh whole-
  backing installation. No migration, fallback reader or automatic reset exists.
  Exact retries, uncertain effects, credit receipts, renewal accounting and
  same-release recovery retain their supported behavior.

### 📚 Documentation

- Correct the library composition guidance to align a direct Storage dependency,
  which owns the backing and collection interfaces.

### 🔧 Changed

- Adopt committed Shared Tooling 0.28.2, including refusal of incomplete
  owned-tree inventories and explicit delivery checkout/direct-push authority.
  Keep deferred engineering work in owning issues rather than release requests.
  Preserve local release groups and compiler pin.
- Extend source-bound group validation to package-scoped Clippy, build, native,
  Rustdoc and Wasm checks, including explicit reverse consumers. Add opt-in group
  CI on all three native hosts; keep normal CI full. Unknown/shared inputs and
  the first delivery of this mapping retain the full gate. Required stage or
  input failures cannot authorize narrower validation.
- Simplify the Blob-owned release selection for
  [#114](https://github.com/dragginzgame/ic-blob-storage/issues/114): share one
  declarative CI/release stage roster, observe member paths through Cargo metadata
  instead of parsing the catalog, and keep new validation/proof scratch under
  `target/`. Preserve existing release evidence and collect both old and new
  failure locations. Delegate roster/baseline observation and run evidence parsing
  to Shared's verifier; keep Blob's product selection and self-contained receipt
  for preparation and fresh-checkout publication. Refuse malformed runs without
  replacing successful receipts or rewriting retained log paths.
- Project heap measurements into Contracts-owned report DTOs without changing
  recording, authorization, reset windows or report bytes. Enforce that native
  Contracts schema generation does not enable Metrics-owned Wire metadata.

### 🧪 Testing

- Retire the six private PocketIC operator CLI tools and their argument/report
  contracts. Typed fixture scenarios retain authority, query-only previews,
  exact funding lookup, uncertain acknowledgements and restoration fences.
- Share fixture creation, artifact admission and installation mechanics while
  keeping scenario principals, cycle budgets and initialization bytes explicit.
  Tenant tests use the published typed codecs directly, without offline-document
  round trips. The supported native CLI and service contracts are unchanged.

## [0.68.1] - 2026-11-02

### 🔧 Changed

- Qualify compatible Storage 0.65.3, Metrics 0.27.2, Testkit 0.80.2, Build 0.74.1,
  Host Foundation 0.74.2, Host Tools 0.78.1 and Wire/Derive 0.15.1 selections.
  Keep one public identity per selected release family and exact retry bytes.
- Refresh Shared Tooling 0.27.28 to committed revision `8075b3f`, including the
  private path-only development-edge admission fix. Preserve the existing catalog.
- Use Shared's strict release-plan runner with Blob-owned scope selection and
  source-bound execution receipts. Shared/unknown changes keep every stage;
  unchanged release machinery uses a real selected-group metadata/rollback check.
- Keep every installed Engine scenario and native host proof. Move the repeated
  original/O3/Os/Oz matrix out of routine local releases and the macOS lanes;
  Linux compatibility CI and explicit/manual checks retain the full matrix.

## [0.68.0] - 2026-11-02

### ⚠️ Breaking

- Replace the retired Memory dependency and public re-export with Storage 0.65.
  Hosts rebuild against Storage's backing, named grants and bounded configuration.
  Allocation directories, collections and private storage keys require fresh whole
  backing after safe installation retirement; no old reader, migration or reset
  fallback is provided. Preserve provider objects, uncertainty and billing custody.
- Handle fallible storage codecs and collections explicitly. Malformed installation
  and gateway cells return the existing invalid-record refusal without repair;
  mutation and backing failures still require the host's IC rollback boundary.

### 🔧 Changed

- Adopt Metrics 0.27, Build 0.74, Host Foundation 0.74.1 and Host Tools 0.78,
  with Platform 0.18/Runtime 0.18.1, Types 0.17, Wire 0.15 and Testkit 0.80.
  Public identities require matching consumers; the PocketIC server is unchanged.
- Adopt Shared Tooling 0.27.28's reviewed snapshot and retain the complete release
  gate. Retired Memory and stable-structures owners are refused at deployment.

## [0.67.0] - 2026-11-02

### ⚠️ Breaking

- Require Rust 1.99.1 or newer for Engine and its coordinated canister/test
  packages. Admit the actual compiler in CI/release validation and publication,
  and remove the duplicate old-compiler job while preserving native hosts,
  archives and installed Wasm checks. [#112 — Adopt Rust 1.99.1 and consolidate compiler qualification](https://github.com/dragginzgame/ic-blob-storage/issues/112).

### 🩹 Fixed

- Prepare the locked workspace dependencies before offline metadata and release
  fixtures on every native CI host, including fresh runners without a prewarmed
  Metrics index/cache.
- Keep compiler-fixture assertions mandatory on macOS Bash 3.2 with explicit
  failure exits, including the premature-completion regression checks.
- Retain the fixture-completion harness's failure evidence in native CI archives.

## [0.66.0] - 2026-11-01

### ⚠️ Breaking

- Adopt Memory/Storage 0.62, Platform 0.17 and Wire 0.14. Public memory, codec
  error and Principal Rust identities require matching consumers; execution
  delegates to the single Runtime 0.17.1 owner. Retained bytes, limits and
  accounting are unchanged. Service transitions remain retirement/reinstall-only;
  no migration, reset or recovery bypass is introduced.

### 🔧 Changed

- Release this group independently within the existing workspace. Require explicit
  `RELEASE_GROUP=engine` for release, recovery and publication; keep other
  groups' package versions and notes unchanged. No runtime or wire contract changes.
- Adopt Shared Tooling 0.27.26's selective release preparation and preserve the
  full workspace validation gate. Package verification supports mixed versions.
- Qualify probes and native test callers with Testkit 0.78, Host Tools 0.77 and
  Metrics 0.25. Testkit's matching Wire/Metrics selection removes the old Metrics
  copy; keep the PocketIC server pinned at 16.1.0.
