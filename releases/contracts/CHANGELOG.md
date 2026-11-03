# Contracts release notes

Packages: `ic-blob-storage-contracts`.

Independent releases start after coordinated 0.65.0; [shared history](../history.md) retains earlier entries once.

## [0.69.0] - 2026-11-03

### 💥 Breaking

- Adopt published Wire/Derive 0.16 and Platform 0.20 / canonical Types 0.18.
  Rust consumers must align those public identities. Blob's endpoint identities,
  Principal bytes, budgets and retained retry packets remain unchanged; its
  application contracts do not embed the changed Platform Account representation.
- Give standalone reports service-owned `CounterMetrics` and `AggregateMetrics`
  DTOs. Project canonical Metrics values at report time and delegate numeric
  decoding to Metrics. Remove `native-wire` forwarding; frozen report frames and
  JSON fields remain unchanged. Adopt Metrics 0.28's arithmetic/Serde-only boundary;
  report schemas no longer rely on the removed supplier metadata feature.
  [#113](https://github.com/dragginzgame/ic-blob-storage/issues/113)

## [0.68.1] - 2026-11-02

### 🔧 Changed

- Refresh Wire/Derive 0.15.1 and Metrics 0.27.2 within their compatible release
  families; retain application frames, limits and public type identities.
- Adopt Shared Tooling's committed 0.27.28 refresh and source-bound release plans.
  Contracts retains installed reverse-consumer checks; uncertain scope keeps the
  full roster. Binaryen compatibility runs move to Linux hosted qualification.

## [0.68.0] - 2026-11-02

### ⚠️ Breaking

- Adopt Metrics 0.27 for public measurement types. Consumers exchanging those
  Rust values must align Metrics and Platform 0.18/Types 0.17 identities.
  Native schema uses published Wire 0.15. Wire bytes, endpoint identities and
  decoder limits remain unchanged.

## [0.67.0] - 2026-11-02

### ⚠️ Breaking

- Require Rust 1.99.1 or newer to build Contracts. Repository development and
  release checks use the exact 1.99.1 pin; wire bytes and decoder limits are
  unchanged. [#112 — Adopt Rust 1.99.1 and consolidate compiler qualification](https://github.com/dragginzgame/ic-blob-storage/issues/112).

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
