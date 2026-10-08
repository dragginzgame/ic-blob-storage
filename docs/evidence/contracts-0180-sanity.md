# Contracts extraction sanity pass — 2026-10-08

Bounded cleanup of [Blob #27](https://github.com/dragginzgame/ic-blob-storage/issues/27),
continuing the uncommitted [0.18.0 extraction](contracts-0180.md) from released HEAD
`e2526159413c79322b9ec89215a01266c9a7f418`. Methods: adopted
[code hygiene](../../audits/code-hygiene.md) and
[authorized module cleanup](../../audits/module-cleanup.md), Shared Tooling 0.1.20
at `3ecc48e579f6cf6e6ab01a6645d8a250fc8c6934`, with the current AGENTS overlay.
Verdict: **PASS for this bounded source/dependency/documentation cleanup**;
this is not a runtime, deployment, release or macOS qualification verdict.

## Findings and disposition

- **LOW — scattered imports:** extraction left root imports below declarations.
  Consolidated them at file top in 18 affected owners and formatted those files.
  Import sets, declarations and function bodies are preserved. Untouched
  pre-existing import-layout cases outside this extraction were left alone.
- **LOW — stale direct dependency:** no service source/example/test uses `sha2`
  after hashing moved to contracts. Removed `sha2.workspace` from the service
  manifest and its corresponding local package lock edge. The root requirement
  and contracts dependency remain; no external package was removed or reselected.
- **LOW — misleading helper docs:** `tenant::parse` checks expected-generation
  representation, leaving scope/authentication to its caller; `upload::history::filter`
  cannot fail; `upload::manifests::{bounds,headers}` check only explicit count/byte
  limits or borrow original headers, respectively. Their docs now describe those
  actual contracts, preserving header order and duplicates and all existing checks.

**No function, method or type is removed or renamed in this pass.** The earlier
public owner moves remain recorded by the [exact inventory](contracts-0180-inventory.json).
Storage, auth, accounting, journals, transport effects and recovery retain their
original owners; no compatibility API, duplicate validator or new test machinery
is introduced. The existing 0.18.0 hard-cut disposition remains unchanged.

Affected import owners:

- `canisters/standalone/src/workflow/mod.rs`
- `canisters/test/consumer_probe/src/ops/manifests/mod.rs`
- `canisters/test/storage_probe/src/workflow/funding/mod.rs`
- `canisters/test/storage_probe/src/workflow/mod.rs`
- `canisters/test/storage_probe/src/workflow/uploads/mod.rs`
- `crates/ic-blob-storage/src/model/catalog/admission/read/mod.rs`
- `crates/ic-blob-storage/src/model/lifecycle/mod.rs`
- `crates/ic-blob-storage/src/model/service/tenant/mod.rs`
- `crates/ic-blob-storage/src/ops/service/installation/mod.rs`
- `crates/ic-blob-storage/src/ops/service/uploads/history/mod.rs`
- `crates/ic-blob-storage/src/ops/service/uploads/read/mod.rs`
- `crates/ic-blob-storage/src/ops/service/uploads/status/mod.rs`
- `crates/ic-blob-storage-cli/src/native/arguments/mod.rs`
- `crates/ic-blob-storage-cli/src/native/installation_check/mod.rs`
- `crates/ic-blob-storage-cli/src/native/reference_inputs/mod.rs`
- `crates/ic-blob-storage-contracts/src/configuration/mod.rs`
- `crates/ic-blob-storage-contracts/src/tenant/mod.rs`
- `tests/protocol/src/consumer/mod.rs`

## Inputs, checks and limits

The incoming lock already differed from the 2026-10-07 qualified graph: Memory
0.31.2, Testkit 0.21.3, serde_spanned 1.1.2 and the selected TOML patches. These are
preserved user-owned selections, not upgrades performed by this cleanup. Other
inputs and prior logs, archives, Wasms, CLI and probe summaries remain unchanged.
The raw directory is `.tmp/contracts-sanity-0180-02/`; `entry-Cargo.lock` and
`entry-Cargo.toml` preserve the exact incoming graph. Only the service's unused
`sha2 0.11.0` lock edge changes during this pass.

- Current core **345** and contracts **103** unit cases pass (**448** total).
- Strict all-target/all-feature Clippy passes for contracts, core, CLI, standalone,
  storage/consumer/admission/authority/funding probes and the protocol package.
- Warning-free library/standalone rustdoc and native/Wasm Rust **1.88** checks pass.
- Snapshot, dependency pins/inheritance, contracts/CLI normal dependency boundary,
  manifest/Rust formatting, local documentation links, immutable evidence checksum
  checks and diff hygiene pass. `qualified-inputs.json` / `check-bindings.json` in
  the raw directory retain exact source and check-log hashes.
- The first locked lint after manifest removal refused a stale local lock edge.
  Its log remains as `clippy-lock-refresh-refusal.log`. A bounded offline Cargo
  check refreshed that edge; the independent lock check confirms every incoming
  external identity/checksum remains exact. Subsequent locked checks pass.
- No new provider, HTTP or PocketIC probe is run. The earlier 79 harness cases and
  package/Candid evidence stay bound to their original source and Memory 0.31.1 /
  Testkit 0.21.2 graph. The new Testkit 0.21.3 package is not cached; native-host /
  PocketIC qualification needs explicit locked-cache preparation before execution.
- Committed-source native macOS qualification remains on #27. No complete gate,
  commit, push, version mutation, release, registry publication or paid effect runs.

The handoff, dependency notes and pending changelog reflect this cleanup without
rewriting the frozen extraction/probe record or released changelog history.
