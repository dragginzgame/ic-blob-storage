# Client release notes

Packages: `ic-blob-storage-client`, `ic-blob-storage-cli`.

Independent releases start after coordinated 0.65.0; [shared history](../history.md) retains earlier entries once.

## [0.68.0] - Unreleased

### ⚠️ Breaking

- Align the public Contracts dependency with its Metrics 0.27 release family.
  Rebuild callers exchanging Contracts values across the client boundary;
  application frames, limits, dispatch and exact retry policy are unchanged.

### 🔧 Changed

- Select published Types 0.17 and Runtime 0.18 directly for replicated clients.
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
