# Client release notes

Packages: `ic-blob-storage-client`, `ic-blob-storage-cli`.

Independent releases start after coordinated 0.65.0; [shared history](../history.md) retains earlier entries once.

## [0.66.0] - Unreleased

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
