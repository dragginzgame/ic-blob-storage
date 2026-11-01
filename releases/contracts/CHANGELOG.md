# Contracts release notes

Packages: `ic-blob-storage-contracts`.

Independent releases start after coordinated 0.65.0; [shared history](../history.md) retains earlier entries once.

## [0.66.0] - 2026-11-01

### ⚠️ Breaking

- Adopt Platform 0.17 and Wire 0.14's canonical Types 0.16 identities. Native
  schema consumers must align Metrics 0.25 with Wire 0.14; the old Principal
  and schema Rust identities are no longer interchangeable. Protocol bytes,
  decoder budgets and content/hash identities are unchanged.

### 🔧 Changed

- Release this group independently within the existing workspace. Require explicit
  `RELEASE_GROUP=contracts` for release, recovery and publication; keep other
  groups' package versions and notes unchanged. No runtime or wire contract changes.
