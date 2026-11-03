# Current status — 2026-11-03

## 0.69.0 delivery requested; direct-push destination needs approval

Prepare Contracts, Client and Engine **0.69.0** in this checkout. All thirteen
package versions remain **0.68.1**; three draft changelogs select 0.69.0.
Source `984a4c81838fa270a1d016d18a90739e7ad3ad89` was committed locally and
historically passed all 22 release stages; it was not pushed, tagged or published.
The user requested the latest Shared adoption followed by delivery. The selected
scope is Contracts, Client, then Engine 0.69.0 from this checkout on local `main`,
using their existing direct-release runner and crates.io publisher. Direct delivery
to `origin/main` still needs explicit approval under Shared's destination rule.
Prepare the coherent supplier/group-CI/#114 source commit locally; do not start
remote mutations or substitute a PR. No alternate checkout, sibling delivery or
live service deployment is selected. Final full gates belong to the normal runner.

## Selected published suppliers

Host Tools **0.80.0**, Metrics **0.28.1**, Platform and independent native PocketIC
client **0.20.0**, Testkit **0.83.0**, Runtime/macros/guard **0.19.0**, Types
**0.18.0**, Wire/Derive **0.16.0**, Storage **0.66.0**. Build **0.74.1**, Host
Foundation **0.74.2**, Pest **2.9.5** and Syn **3.0.9** retain prior selections.
Canonical registry records and checksum-verified published archives are under
`target/upstream-refresh-20261103/`. Metrics 0.28.1 and Host Tools 0.80 have identical
Rust sources to their preceding selections; the Metrics minimum stays 0.28.0.

Shared Tooling **0.28.2**, committed
`5cf67f81c50c56ac17389ae71a3a87d46e86e8a2`, is exported and verified (154 files).
This released revision changes no exported executable bytes from the qualified
9815bbf snapshot. Adoption/readback: `target/shared0282-release/`.
Native fixtures use the independent `ic-platform-pocket-ic` owner directly,
aligned with Testkit 0.83; the facade PocketIC feature stays disabled. The older
native facade is gone. Native/deployment graphs retain single canonical
Types/Runtime/guard/Wire identities; deployment excludes native tooling and
retired Memory/stable-structures. PocketIC server **16.1.0** is unchanged.
Rust **1.99.1** remains both minimum and pinned development/release compiler.

Preserve the Engine funding-progress consolidation and fresh `BLFUPR01` record /
`ic-blob-storage/installation:singleton-cells:funding-progress` format. Retire
obligations with original tools before fresh whole backing; no migration,
fallback reader or reset. Same-release recovery and exact retries remain required.
Six private operator CLIs are retired; typed scenarios retain meaningful coverage.

[Blob #113](https://github.com/dragginzgame/ic-blob-storage/issues/113) is locally
implemented: Metrics owns recording/numeric validation, Contracts owns passive
report DTOs, and previous report bytes remain fixed. CLI's Metrics edge is dev-only;
Contracts schema generation excludes supplier-owned Wire metadata. Keep #113 open
until default-branch delivery. [Observability #128](https://github.com/dragginzgame/ic-observability/issues/128)
owns supplier metadata removal. Published Wire 0.16 resolves
[Wire #44](https://github.com/dragginzgame/ic-wire/issues/44); Storage and Testkit now
share that identity. Storage's formats are unchanged by supplier adoption.

## Group CI and #114

Blob owns product selection and reverse-consumer policy in
`ci/release-package-scopes.json`; `ci/release-validation.json` is the one full-stage
roster. Shared admits roster/baseline/run evidence and executes plans. Cargo observes
member names/paths without version projection. New attempt/selector/proof scratch
uses `target/`; the collector retains both new and historical locations.

Five Cargo stages select source-bound package roots: Contracts includes all groups;
Client includes Client+Engine; Engine includes Engine. Features/targets stay fixed.
Common CLI/probe/archive guards and applicable installed partitions remain required.
Tenant-client-only changes may omit Engine IC; unchanged release machinery may omit
its regression matrix, never real selected-group metadata admission. Normal CI and
individual targets remain full. Group workflow dispatch retains all three native
hosts plus independent Wire/schema, library and native CLI/restoration obligations.
First delivery, unknown/shared/dependency/compiler/feature/generator/policy changes
stay full; no results are reused. See [validation scope](../releasing.md#validation-scope).

For [#114](https://github.com/dragginzgame/ic-blob-storage/issues/114), Shared's
[evidence interface](../group-validation-adoption.md) replaces local event/timing
and Git-baseline parsing. Host/compiler observations are checkpointed before
execution; its protected writer cannot replace success with failed capture.
Blob's self-contained receipt re-admits source/configuration/product obligations
after preparation and in fresh publication checkouts. Raw runner paths are never
rewritten. Original-location `check` detects changed logs. Keep #114 open until
consumer delivery; [Shared #349](https://github.com/dragginzgame/shared-tooling/issues/349)
owns generic parser regressions.

## Qualification and retained evidence

Current focused results belong to `target/upstream-refresh-20261103/`: strict
CLI/PocketIC Clippy, dependency/deployment/compiler guards, standalone Wasm,
five Metrics report-contract tests and five installed surface/carrier/Metrics/
snapshot cases all pass. Actual consumer receipt/fixture checks and all three
real group metadata/rollback checks pass, preserving suppliers and other groups.
Canonical Testkit 0.83 setup/check passed. No full release gate, macOS/hosted
qualification, optimizer comparison or live provider probe is claimed for this graph.

Earlier target and `.tmp` evidence directories are no longer present after disk
recovery. Prior reports (558 native tests, 24 installed cases, seven Wasm builds,
27 methods/137 frames) describe the earlier graph, not newly retained proof.
Historical source/details remain in `git show 984a4c81:docs/status/current.md`.
Do not synthesize missing logs or treat old results as a current release receipt.
Browser inputs are unchanged; prior setup selected Node **24.21.0**, npm **12.2.1**
via `BLOB_BROWSER_NPM=$PWD/.tools/browser-npm/node_modules/.bin/npm`.

Library publication does not deploy or qualify a live service. Apply the
[retirement contract](../retiring-installations.md) before fresh backing; preserve
provider objects, uncertain effects, balances and billing custody. Canic owns its
adapter, production limits and rollout. Blob #5/#31/#6/#34 and Canic #622/#494 retain
application/provider acceptance. No paid effect, reset, sibling edit or live
provider call occurred here; CI adoption does not close those acceptance issues.
