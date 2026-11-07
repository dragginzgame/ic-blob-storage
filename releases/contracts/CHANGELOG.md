# Contracts release notes

Packages: `ic-blob-storage-contracts`.

Independent releases start after coordinated 0.65.0; [shared history](../history.md) retains earlier entries once.

## [0.73.1] - Unreleased

### 🔧 Changed

- Adopt Platform 0.24 and Wire 0.17.8 while retaining the same canonical
  Principal/Candid identities, packet bytes, decoder limits and acceptance.
  Refresh native qualification with Host 0.75 and Shared Tooling 0.31.5.

## [0.73.0] - 2026-11-05

### 💥 Breaking

- Remove the unused host Wire-schema implementation from the external Caffeine
  certificate reply. Keep its existing Rust path and required Candid
  `method`/`blob_hash` record; the owned certificate-assessment endpoint retains
  its Wire schema. No second DTO, compatibility re-export or protocol conversion
  is introduced. [#117](https://github.com/dragginzgame/ic-blob-storage/issues/117).

## [0.72.0] - 2026-11-04

### 💥 Breaking

- Adopt published IC Wire 0.17.1 for public borrowed values, limits and errors.
  Keep metadata derivation opt-in under the existing host schema feature; codec
  builds exclude the derive. Align consumer Wire selections to the same minor.
  Method identities, canonical frames, budgets and retained retry bytes are unchanged.

## [0.71.1] - 2026-11-04

### 🔧 Changed

- Adopt published Wire/Derive 0.16.2, Platform 0.22 and canonical Types 0.18.1.
  Keep passive Contracts free of Candid/Agent/execution; full-service Caffeine
  codecs retain their fields, quotas, method identities and exact Wire bytes.

## [0.71.0] - 2026-11-04

### 💥 Breaking

- Make external Caffeine Candid APIs an explicit `caffeine-candid` capability. Default/passive contracts import canonical Types directly and exclude the facade/Candid; gate complete account DTOs and codecs together, conditionally derive Candid on the passive certificate reply, and require the feature for full endpoint mounting. Full-service schema generation selects it; provider JSON remains independent. Preserve endpoint identities, full-width decimal fields, provider Candid bytes and decoder quotas.
  [#115](https://github.com/dragginzgame/ic-blob-storage/issues/115).

- Remove standalone telemetry DTOs, method identity and standalone-only schema/client generator modes. The shared service catalog and application Wire contracts remain unchanged; Canic owns operational reporting.

## [0.70.0] - 2026-11-04

### 💥 Breaking

- Adopt Metrics 0.29's instrumentation boundary and Platform 0.21. Contracts owns
  report decoding and rejects inconsistent numeric quality without constructing
  recorders. Remove the public recorder-to-report conversion implementations;
  hosts project their recorded fields explicitly. Report fields, Wire identities,
  canonical bytes and decoder limits remain unchanged. Align consumers on the
  new public dependency family.
  [Metrics #130 — Restrict Metrics to instrumentation: privatize leaking arithmetic and recording internals](https://github.com/dragginzgame/ic-observability/issues/130).

### 🔧 Changed

- Adopt Shared Tooling 0.29.3's test layout: extract inline contract and schema
  example tests into owner-local `tests.rs` modules. Preserve private access,
  test identities, fixtures and all production behavior.

- Add explicit standalone metadata/client outputs to the host-only Wire schema
  generator. Operator tools can encode and inspect the existing `blob_metrics`
  query with canonical DTOs and budgets while using their configured signer.
  Shared service generation, endpoint bytes and deployed code remain unchanged.

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
