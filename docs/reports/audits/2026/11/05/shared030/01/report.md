# Shared Tooling 0.30 adoption and policy audit — 2026-11-05

Reviewed Blob base `1d056dc2772c3bf87a6aa0feb1959d48faa75942` and committed
Shared Tooling **0.30.0**, `07dce12cbe4a798fd4a6d263095518678c4f6b45`.
The canonical exporter selected 166 payload files. The source commit is publicly
readable; this observation is not a claim of an annotated Shared release tag.
Sibling source was read-only. This report describes local, uncommitted changes.

## Boundary and review coverage

Blob owns contracts, replicated/native clients, service state and the standalone
adapter. Storage owns allocations/collections, Platform owns IC identities and
execution/external Candid, Wire owns application encoding, and Tooling owns native
primitives and Testkit. Canic owns its framework adapter and operational reporting.
The review preserved these boundaries, endpoint identities and stored bytes.

Read the shared baseline, four principles and their applicable engineering,
workspace, dependency, pinning, hook, release, changelog, maintenance, RPC-budget
and reporting rules. Reviewed Cargo's complete roster, authored Rust sources,
feature/dependency declarations, generated-client entrypoints, local scripts,
workflow routing, active docs and historical evidence references. This is a
policy/adoption audit, not a fresh proof of every runtime invariant or provider
promise.

| Area | Evidence and disposition |
| --- | --- |
| Governance and compiler | [AGENTS.md](../../../../../../../../AGENTS.md) selects the same committed policy as the snapshot; Rust/Cargo 1.99.1 and edition 2024 remain the owned compiler policy. Actual compiler admission covers all 13 members. |
| Workspace and releases | One virtual workspace, lockfile and inherited catalog; Contracts, Client and Engine groups retain their package names and literal versions. The approved physical layout remains in force. Group metadata rendering/rollback checks pass with the new probe paths. |
| Test ownership | Six probes move from `canisters/test/` to `canisters/tests/`, with no aliases or symlinks. Eight fixture manifests explicitly declare the tests LOC role. Only the native archive verifier path is classified as build tooling within the mixed CLI package. |
| Rust lint and source hygiene | Scanned 836 maintained Rust files: no `allow(...)` attributes, 80 existing `expect` attributes. Configuration/command searches found no suppression of unfulfilled expectations. No singular test owners, path-attribute module workarounds or executable source includes remain. |
| Conditional test code | A substantial corruption helper moved into [its existing test module](../../../../../../../../crates/ic-blob-storage/src/ops/service/roots/tests/mod.rs) with unchanged visibility/callers. Small CLI test entry adapters remain necessary wiring to the same real parser; codec-test imports and the schema-test command helper moved to their existing test modules. Macro bodies remain owned at their source. |
| Dependency roles | Serde CBOR is correctly labelled test/tooling-only. Generic read-cost totals no longer consume Metrics. The root catalog, probe manifest and lock remove that package; the direct boundary refuses it everywhere. No other external package selection changed. |
| Runtime/deployment isolation | Declaration and resolved graph checks cover aliases, dependency kinds, targets and features. Standalone deployment excludes host tools, Testkit, private probes, retired Memory/CDK and schema/provider-JSON tooling. External provider Candid remains behind Platform; application packets remain Wire. |
| Scratch and generated output | Active browser bundles, schema output, reports and local fixture defaults now use `target/`; consumers and workflow artifact paths change together. Explicit temp overrides remain supported. Historical `.tmp` records and evidence-discovery regressions remain valid readers of old custody. |
| Generators and fixtures | Generated TypeScript passes frozen-frame parity. Retained Rust snippets exercise compiler/archive/formatter admission and are legitimate disposable qualification inputs; they do not patch application entrypoints before builds. |
| Recovery and artifacts | No paid/provider operation, reset, source allocation removal or cleanup occurred. Frozen release receipts and historical hash inventories remain unchanged. Existing retention admission still owns release/publication state. Private live-operation custody is not ordinary scratch and was not assessed for deletion eligibility. |
| Documentation | Current dependency/reporting/setup and release descriptions match the new ownership and paths. Historical prose/packets/hash paths stay historical; live Markdown targets were repaired. Canonical LOC guide dependencies were included through the exporter, not patched locally. |

## LOC classification

The new reporter counts **28,586 library/application**, **83,215 tests**,
**734 examples**, **128 build/tooling**, zero benchmarks and zero unclassified
Rust code lines: **112,663 total**. The baseline before role declarations had
112,669 total; reclassification alone preserved that total exactly. The later
six-line net decrease comes from the profiler/test-owner cleanup, not a runtime
size claim.
These are source code lines, not Wasm size or a performance measurement.

All 54 diagnostics were reviewed: 31 unexpanded macro bodies and 23 conditional
attributes. They are disclosed classification limitations, not unknown owners.
No missing-source or legacy-name diagnostic was hidden. Machine evidence is
`target/shared030-before.json`, `target/shared030-after.json` and
`target/shared030-final-cloc.json`.

## Verification

Linux/x86_64 on the pinned Rust/Cargo **1.99.1**:

- Canonical snapshot hashes, grouped sorting/rustfmt, ShellCheck/syntax,
  documentation references, compiler policy, dependency pins and Platform boundary.
- Strict Clippy for the six moved probes, protocol/PocketIC packages and the
  changed core/CLI packages, with all their targets/features and warnings denied.
- All six optimized probe Wasm builds.
- Four private-profiler overflow/reset tests, five root-owner tests and 36 record,
  gateway and CLI tests covering relocated unit-test helpers/imports.
- One installed PocketIC restoration-window case: exact reopening attribution,
  operator authority, immutable observations and repeated same-image recovery.
- Browser bundle build and actual Chromium IndexedDB/restart test, using only
  local fixtures; generated TypeScript checks 27 methods and 137 frozen Rust
  frames including malformed/identity/direction/limit and decimal rejections.
- Real group metadata rendering/rollback, formatter hook preservation, compiler/
  Testkit forwarding, logging, early-completion, evidence custody, archive-path
  and release-plan/partition fixtures. Effects outside these disposable fixtures
  were not executed.

Raw results use `target/shared030-*.log`; failed prerequisites remain there too.
Sandbox loopback and Node subprocess restrictions initially prevented installed/
browser checks; permitted local reruns passed. The release-plan fixture initially
matched the old Metrics step label; its assertion was updated with the workflow.
A missing canonical documentation target was resolved by exporting the source
owner's link closure. No failed attempt establishes successful qualification.

## Limits and upstream disposition

- Canonical Shared helpers still default ordinary scratch to `/tmp`, despite the
  new target ownership rule. [The snapshot](../../../../../../../../.shared-tooling.snapshot)
  remains exact; source-owner correction is tracked in
  [Shared Tooling #356](https://github.com/dragginzgame/shared-tooling/issues/356).
  Do not patch exported helpers or globally override TMPDIR to hide this gap.
- LOC roles/directory conventions are verified by this audit and the reporter;
  they are not claimed to be an additional enforced CI stage. Existing CI still
  owns compiler, formatting, dependency and deployment enforcement.
- Native macOS, the complete release gate, installed production/provider/browser
  acceptance and performance measurements were not rerun. Linux/cross-target
  evidence does not establish those obligations. This audit adds no schedule,
  service, release authority or downstream integration requirement.
