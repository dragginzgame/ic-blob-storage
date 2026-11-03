# Engine release notes

Packages: `ic-blob-storage`, `ic-blob-storage-canister`, `blob-admission-probe`, `blob-authority-probe`, `blob-consumer-probe`, `blob-funding-probe`, `blob-gateway-source`, `blob-storage-probe`, `blob-test-protocol`, `ic-blob-storage-pocketic-tests`.

Independent releases start after coordinated 0.65.0; [shared history](../history.md) retains earlier entries once.

## [0.69.0] - Unreleased

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
