# Current status — 2026-11-03

## Contracts, Client and Engine 0.69.0 published

The user explicitly approved direct delivery from this checkout on `main` to
`dragginzgame/ic-blob-storage` via `origin/main`, all three group tags and crates.io
publication. The standard runner completed each group supplier-first; all thirteen
workspace members are **0.69.0**. No alternate checkout or sibling delivery occurred.

| Group | Release commit | Annotated tag |
| --- | --- | --- |
| Contracts | `02622fdd11c0ba8e3b5dc993cb4151f09f53d694` | `contracts-v0.69.0` |
| Client | `46e0ab13dfba63973b670f99cfee5e38cea479df` | `client-v0.69.0` |
| Engine | `d4aded0b01291534a60b50fcc459971fbc11962b` | `engine-v0.69.0` |

All three public crates are indexed, non-yanked, and independently read back.
Registry/archive checksums, VCS release commits and package paths match.
Remote branch/tag objects and peeled commits match local releases. Publication
and readback evidence: `target/release069/`; full publication attempts:
`target/publication.tgy1WF/`, `target/publication.8rG0dw/` and
`target/publication.riDV5k/`. Each `.git/release-state/<group>-0.69.0.plan` is complete.
Publication retries need the selected group's release checkout and original receipt;
a later handoff/source commit is not its release commit. Preserve all evidence.

## Selected suppliers and boundaries

Host Tools **0.80.0**, Metrics **0.28.1**, Platform and independent native PocketIC
client **0.20.0**, Testkit **0.83.0**, Runtime/macros/guard **0.19.0**, Types
**0.18.0**, Wire/Derive **0.16.0**, Storage **0.66.0**. Build **0.74.1**, Host
Foundation **0.74.2**, Pest **2.9.5** and Syn **3.0.9** retain prior selections.
Registry/archive API evidence: `target/upstream-refresh-20261103/`. Metrics 0.28.1
and Host Tools 0.80 have identical Rust sources to their preceding selections;
the compatible Metrics minimum stays 0.28.0. No unpublished source override exists.

Shared Tooling **0.28.2**, committed
`5cf67f81c50c56ac17389ae71a3a87d46e86e8a2`, is exported and verified (154 files).
It changes no exported executable bytes from the previously qualified 9815bbf
snapshot. Readback: `target/shared0282-release/`. Rust **1.99.1** remains both
minimum and pinned development/release compiler. PocketIC server **16.1.0** is
unchanged. Native fixtures use its independent client owner directly; no facade
PocketIC feature or older facade remains. Deployment excludes native tooling,
retired Memory and stable-structures; canonical execution/type/Wire owners are single.

The Engine hard cut consolidates funding progress under fresh `BLFUPR01` and
`ic-blob-storage/installation:singleton-cells:funding-progress` identities.
Retire obligations using original tools before fresh whole backing. There is no
migration, fallback reader or reset. Same-release recovery and exact retries remain
required. Six private operator CLIs are retired; typed scenarios retain coverage.

## Delivered issue work and remaining acceptance

[Blob #113](https://github.com/dragginzgame/ic-blob-storage/issues/113) now has its
implementation on default main: Metrics owns recording/numeric validation;
Contracts owns passive Wire report DTOs; exact prior report bytes are preserved.
CLI's direct Metrics edge is dev-only, and Contracts schema generation excludes
supplier-owned Wire metadata. [Observability #128](https://github.com/dragginzgame/ic-observability/issues/128)
owns the supplying metadata removal; Wire 0.16 resolves
[Wire #44](https://github.com/dragginzgame/ic-wire/issues/44).

[Blob #114](https://github.com/dragginzgame/ic-blob-storage/issues/114) is also on
default main. One CI/release roster and Cargo's member/path observation replace
local inventory parsers. Shared owns roster/baseline/run verification, dispatch
and protected receipt writing. Blob owns product/reverse-consumer selection and
its source-bound self-contained receipt for preparation and fresh-checkout
publication; raw runner paths are never rewritten. New scratch is under `target/`,
and collection retains historical locations. No version projection or test reuse.
The first/unknown/shared/dependency/policy changes stay full; ordinary CI stays full.
See [validation scope](../releasing.md#validation-scope).

Keep #113/#114 open for the latest applicable hosted acceptance, then confirm and
close them without rerunning unchanged local gates. Engine-source hosted run
[41647808416](https://github.com/dragginzgame/ic-blob-storage/actions/runs/41647808416)
was in progress at readback. A later handoff-only commit may supersede that run;
select latest default-branch evidence rather than treating cancelled older runs
as passed. No macOS, hosted optimizer or live-provider result is inferred locally.

## Qualification and custody

Each group's normal release runner passed all **22 stages**, including strict
workspace Clippy, compiler/tool/dependency/deployment admission, real release
metadata/rollback/publication/recovery fixtures, hooks/formatting, native tests,
both installed PocketIC partitions, Wasm and exact mixed-version archive checks.
Each native stage executed **692 passing tests/doctests**. Four standalone opt-in
cases remain ignored; no ignored case is claimed passed.
Full source-bound plans/logs: `target/blob-release-validation.aSBHWU/` (Contracts),
`target/blob-release-validation.5ziODV/` (Client), and
`target/blob-release-validation.cOWR16/` (Engine). Receipts also remain under
`.git/release-state/validation/` and in each committed group `release.json`.
Earlier target/`.tmp` evidence removed during disk recovery was not synthesized or
reused. Historical reports retain their original source/graph, not current proof.

Library publication does not deploy or qualify a live service. Apply the
[retirement contract](../retiring-installations.md) before fresh backing; preserve
provider objects, uncertain effects, balances and billing custody. Canic owns its
adapter, production limits and rollout. Blob #5/#31/#6/#34 and Canic #622/#494 retain
application/provider acceptance. No paid effect, reset, live provider call or sibling
edit occurred; this release does not close those acceptance issues.
