# Shared Tooling 0.3 and selected Testkit for pending 0.22.1

Reviewed on released Blob 0.22.0, HEAD `9063e3e`, validated source
`bebd55aa01bcc711b44f36f4765133540ec5b89a`. Package versions and the finalized
receipt remain 0.22.0. The maintainer requests the new Testkit and Shared
Tooling; incoming Cargo edits already select Testkit 0.30.0 and are preserved.

## Initial Testkit 0.30 phase

Canonical export refreshes the existing 96-file selection from committed Shared
0.3.0 `88a73139a0f083344c41a6f6f4b5c3a8aca7dc1d`. The isolated source checkout
excludes sibling working-tree edits. Published, un-yanked Testkit 0.30.0 matches
owner `6ac161b8ed689012bf8b0ce926946f94ea9f407d`; all 41 packaged Rust/original
manifest files match owner source and the archive. Its lock/registry checksum is
`d1f639412105b6f5c909ac7342c94a22c07ccbeb5285112fc123e78fcb3b24d8`.
Memory 0.34.1, Metrics 0.4.0, all four Host crates 0.11.0 and PocketIC 16.1.0
remain selected in this first phase. Then-dirty upcoming Testkit work is excluded.

Shared aggregates now run the complete host, IC and Cargo sets in order, then
Blob's existing Testkit owner targets. Duplicate aggregate prerequisites are
replaced with the ordered local target lists. Testkit's thin lock-selection
adapter, authenticated server, explicit caller overrides and managed lifetime
remain owned by the existing tools. No independent server pin or new wrapper
is introduced. Shared's selected evidence helper calls the complete host check;
retired optional flags have no remaining active Blob callers.

Testkit bounds retained Cargo-build diagnostics to a marked 1 MiB prefix per
stream while observers receive complete output; tool/workspace probes require
complete output within 64 KiB per stream and metadata stdout within 16 MiB.
No elapsed-time build limit is introduced. Blob uses its existing harness APIs,
not Testkit's optional Wasm-cache builder; bounded cache behavior is upstream
source evidence, rather than a new Blob build-path claim.
[Shared #98](https://github.com/dragginzgame/shared-tooling/issues/98) and
[Testkit #49](https://github.com/dragginzgame/ic-testkit/issues/49) own upstream
acceptance; [Blob #39](https://github.com/dragginzgame/ic-blob-storage/issues/39)
owns selected-tool and IC adoption.

## Initial qualification

Raw inputs and logs remain under
`target/review-validation/testkit030-shared030-0221/`: entry Cargo/receipt files,
canonical refresh, selected metadata, source/archive comparison, setup,
retained-tool hashes and consumer command fixtures.
The initial extended consumer fixture omitted its boundary-check substitute;
its failed log and isolated fixture remain retained. The corrected actual-root
Make fixture passes parallel complete setup/check ordering and every common/
Testkit failure boundary, including refusal before native builds. Substitutes
establish routing and failure behavior, not a deployed provider's behavior.

## Incoming Testkit 0.31 / direct Host 0.12 phase

During the full run, incoming Cargo edits select published Testkit 0.31.0 and
direct Host artifacts/fs 0.12.0. Preserve them. The first `make ci` passes tooling,
release/recovery, hooks, formatting, compiler/lint, evidence/docs and 643 native
cases, then refuses before PocketIC because the newly selected 0.31 CLI is
unprepared. Its log remains separate; this partial, changed-input run is not
relabelled as final-graph qualification.

Testkit 0.31.0 matches owner `f1ae9e6d3b0f3f20ec1e1f1b49c8b1dea3155e0a`,
checksum `c05b04dd09011a4d15f02756104a7a7102b6d4fbf765274957fd1056c784c807`.
Both direct Host 0.12.0 packages match owner
`1ba4591868a83b367d56bae9e3c213418c66a192`: artifacts checksum
`d27c6b9ecc60250e8e5330a4fe1b4e96e7b3c0ff2c90b1c39b1605aa0b73cf14`, fs
`11a502fe465dce26671a981c58caa11df1d5d6aae4ff4ccf443a0ce73c76cde7`.
All 80 Rust/original-manifest files match owner source and authenticated archives.
Their Rust sources are identical to Testkit 0.30 / Host 0.11 respectively.
Testkit still selects Host 0.11 privately; no dependency override or compatibility
wrapper is added. No public Blob type identity crosses these private owners.

Explicit complete setup prepares the lock-selected 0.31 CLI. The new graph,
source comparisons, selected-tool checks and affected compiler/test/package
logs are retained separately in
`target/review-validation/testkit031-shared030-0221/`. Reuse the unchanged Shared,
consumer/release/hook fixture results; rerun affected compiler/lint/floor and
native/PocketIC/package stages. The maintainer confirms that this graph is settled.

Complete setup, normal and restricted-PATH offline admission, and explicit offline
setup reuse pass for Testkit 0.31. The first restricted-PATH attempt in the 0.30
packet omitted ShellCheck; its retained failure is corrected by selecting the
existing executable explicitly. No checker installs or falls back to another
CLI/server. All 22,983 captured older tool files remain unchanged, including
0.30's CLI binary and receipts; prior bundles, servers and evidence remain.

The final affected gate passes with
`make ci CI_TARGETS='shared-tooling-check tools-check dependency-pins-check documentation-links-check deps contracts-boundary-check fmt-check check clippy probe-check docs-check test wasm-check package'`.
This completes the documented suite in phases: unchanged Shared/consumer fixtures,
ShellCheck, release/recovery and hook results come from the first run; their
relevant source, pin, Make and substitute fixture inputs are unchanged. Compiler,
selected-tool, native/PocketIC, documentation and package stages are rerun for
this graph. No single earlier `make ci` is relabelled as a final-graph pass.

Complete Rust 1.88 native/Wasm workspace checks pass. Final tests include 643
native cases and the complete PocketIC target's 376 passing cases (including
native harness tests), with actual rebuilt standalone/storage groups at 73/123.
Explicit opt-in cases remain ignored. Both normalized extracted library packages
pass tests/examples/doctests in their fresh verification workspace. All 14,696
captured compiler inputs, selected Cargo files and tooling source identities
remain unchanged after validation; the finalized 0.22.0 receipt is unchanged.

At readback, released Blob 0.22.0
[CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38044340976)
passes Linux, MSRV and Apple Silicon macOS; Intel macOS is queued. Testkit 0.31
[CI](https://github.com/dragginzgame/ic-testkit/actions/runs/38045250725) passes
all Linux jobs, while native macOS jobs are queued. Host 0.12
[CI](https://github.com/dragginzgame/ic-host-tooling/actions/runs/38045025680)
passes Linux, MSRV and both macOS hosts. These are exact-source upstream/released
results, not hosted qualification of this uncommitted Blob adoption.

[Blob adoption feedback](https://github.com/dragginzgame/ic-blob-storage/issues/39#issuecomment-6096739994) and
[Shared consumer feedback](https://github.com/dragginzgame/shared-tooling/issues/98#issuecomment-6096744453) record this local qualification while
retaining committed native acceptance with the existing owners.

Pending 0.22.1 is compatible: these dependencies are private developer tooling;
public Blob APIs, durable schemas and Memory identity are unchanged. No sibling
edit, agent commit/push/release, registry publication, provider probe or paid
or deployed effect occurs. Native macOS acceptance requires matching-source CI;
earlier source/graph qualifications retain their original bindings.
