# Engine release notes

Packages: `ic-blob-storage`, `ic-blob-storage-canister`, `blob-admission-probe`, `blob-authority-probe`, `blob-consumer-probe`, `blob-funding-probe`, `blob-gateway-source`, `blob-storage-probe`, `blob-test-protocol`, `ic-blob-storage-pocketic-tests`.

Independent releases start after coordinated 0.65.0; [shared history](../history.md) retains earlier entries once.

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
