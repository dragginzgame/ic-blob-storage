# Client release notes

Packages: `ic-blob-storage-client`, `ic-blob-storage-cli`.

Independent releases start after coordinated 0.65.0; [shared history](../history.md) retains earlier entries once.

## [0.73.2] - 2026-11-07

### 🔧 Changed

- Use IC Build 0.77 for native CLI source evidence and optional package
  qualification, preserving CLI behavior and the replicated Client boundary.
- Dispatch native commands directly from the typed Clap invocation; remove the
  intermediate command catalog and repeated execution match. Share the five
  upload commands' input preparation and CLI report projection while retaining
  pre-effect trust checks, exact request journals and internal publication outcomes.
- Share bounded nonempty reads of admitted file descriptors across ordinary,
  frozen-input and recovery commands; retain each caller's opening authority and
  error classifications.

## [0.73.1] - 2026-11-06

### 🔧 Changed

- Adopt Build 0.75 for native CLI source evidence and optional archive
  verification. Preserve command behavior, exact input custody and the existing
  Host process interfaces; the replicated Client gains no Build dependency.

## [0.73.0] - 2026-11-05

### 💥 Breaking

- Align the pending family with Contracts 0.73's removal of the external
  certificate reply's unused Wire metadata implementation. Public contract
  consumers select the same Contracts cohort; Caffeine Candid bytes, endpoint
  names, application Wire frames and saved records are unchanged.
  [#117](https://github.com/dragginzgame/ic-blob-storage/issues/117).

### 🔧 Changed

- Adopt published browser Platform 0.2 for typed saved-call inspection and
  protocol refusal fixtures, replacing generic CBOR access. Preserve original
  request IDs, envelope bytes, trust/binding checks and SDK Core 6.1.1.
- Consolidate CLI grammar and command guidance in Clap. Advertise account,
  history and reference choices through typed value parsers, and use derived
  arguments for the probe and archive verifier instead of string-keyed dispatch.
  Preserve command names, argument spellings, redacted errors, saved requests and
  recovery behavior.
- Consolidate publication preparation's retained-claim verification across
  prepared and uncertain outcomes, preserving exact-envelope checks and the
  refusal to resend an already claimed operation.
- Qualify the native CLI and archive-verifier interfaces with Platform 0.23.0
  and Foundation/Build 0.74.4, preserving transport, command and recovery behavior.

## [0.72.0] - 2026-11-04

### 💥 Breaking

- Adopt Contracts 0.72 and the IC Wire 0.17 type cohort, including Testkit 0.86
  for native qualification. Align direct consumer Wire dependencies; preserve
  replicated/native transport, authentication, exact retry bytes and explicit
  Engine release selection. No saved-document conversion is introduced.

## [0.71.1] - 2026-11-04

### 🔧 Changed

- Adopt Platform 0.22 and Host Tools 0.82 with the matching Runtime 0.19.1,
  Types 0.18.1, Wire 0.16.2 and Build/Host 0.74.3 selections. Qualify native
  signed queries and certificate refusal/recovery while preserving CLI output,
  authentication, retry custody and passive replicated-client boundaries.

## [0.71.0] - 2026-11-04

### 💥 Breaking

- Consume the default passive Contracts/Types path without selecting external Caffeine Candid. Native CLI explicitly selects the provider capability for its existing account-link and service operations.
  [#115](https://github.com/dragginzgame/ic-blob-storage/issues/115).

- Remove the native `metrics` command and its standalone report decoder/output. Keep service accounting, operator inspection and publication-session recovery commands; Canic owns operational reporting.

## [0.70.0] - 2026-11-04

### 💥 Breaking

- Align with Contracts' Metrics 0.29 report boundary and Platform 0.21. Remove
  the CLI's Metrics dependency used solely to build report fixtures. Keep report
  fields, numeric refusals, canonical packets and exact retries unchanged.

### 🔧 Changed

- Qualify published Host Tools 0.81, Build 0.74.2 and the current native family;
  consume Shared Tooling 0.29.3's formatting and release metadata checks. Move
  inline publication-control tests into owner-local `tests.rs` modules without
  changing their identities or behavior.

## [0.69.0] - 2026-11-03

### 💥 Breaking

- Align public Wire 0.16, Platform 0.20 / Runtime 0.19 and Types 0.18 identities with
  Contracts. Existing application packets, method IDs, limits and exact signed
  retries remain unchanged.

### 🔧 Changed

- Define native CLI commands and their shared flag groups once with typed Clap
  arguments. Remove string-based command dispatch, flag-type lookup and
  `ArgMatches` conversion from offline handlers. Command names, flags, limits,
  trust checks, saved documents and exact retry behavior remain unchanged.
- Render standalone metrics from Contracts-owned transport fields. Keep wide
  decimal output and scope/roster checks; Metrics is now only a direct CLI test
  dependency rather than a production import.

## [0.68.1] - 2026-11-02

### 🔧 Changed

- Qualify published Build 0.74.1, Host Foundation 0.74.2 and Host Tools 0.78.1,
  alongside the compatible Wire/Derive 0.15.1 and Metrics 0.27.2 selections.
- Adopt the committed Shared Tooling 0.27.28 refresh and source-bound release
  plans. Qualified tenant-client implementation changes retain installed Client,
  CLI and host checks while omitting unrelated Engine scenarios and unchanged
  publication/recovery fixtures. The first plan-based release keeps the full gate.

## [0.68.0] - 2026-11-02

### ⚠️ Breaking

- Align the public Contracts dependency with its Metrics 0.27 release family.
  Rebuild callers exchanging Contracts values across the client boundary;
  application frames, limits, dispatch and exact retry policy are unchanged.

### 🔧 Changed

- Select published Types 0.17 and Runtime 0.18.1 directly for replicated clients.
  Align public Principal/call identities with Platform 0.18 and preserve Wire bytes
  and dispatch policy while removing the client's direct dependency on the
  Platform facade. Contracts
  retains its external Candid boundary.

- Adopt Build 0.74 and Host Foundation 0.74.1 for native tooling, and npm 12.2.1
  for browser qualification. The provider SDK override stays unchanged.

## [0.67.0] - 2026-11-02

### ⚠️ Breaking

- Require Rust 1.99.1 or newer to build Client and CLI. The repository admits
  the pinned compiler and retains isolated default-feature and Wasm checks in
  its existing CI jobs. [#112 — Adopt Rust 1.99.1 and consolidate compiler qualification](https://github.com/dragginzgame/ic-blob-storage/issues/112).

## [0.66.0] - 2026-11-01

### ⚠️ Breaking

- Align public Principal and transport types with Platform 0.17/Runtime 0.17.1
  and Contracts' Wire 0.14 identity. Native schemas use Metrics 0.25. Build/Host
  Foundation minimums now select the reviewed 0.73.6 release.

- Native `publish-map`, `publish-file-status` and `publish-session` require
  `BLOB_EXPECTED_HOST_RELEASE` to select the deployed Engine release. Fresh
  replies and retained recovery evidence must match it exactly. The CLI no
  longer assumes Engine and Contracts versions coincide; saved and wire formats
  remain unchanged. Make supplies the current standalone Engine selection;
  direct CLI and launcher callers must set it explicitly.

### 🔧 Changed

- Release this group independently within the existing workspace. Require explicit
  `RELEASE_GROUP=client` for release, recovery and publication; keep other
  groups' package versions and notes unchanged. No service or wire contract changes.
- Report the CLI's own package version rather than the independently released
  Contracts version. Installation checks retain an explicit host release.
