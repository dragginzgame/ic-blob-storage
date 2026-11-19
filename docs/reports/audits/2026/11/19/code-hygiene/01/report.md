# Toko 0.45.15 rules and code hygiene review

**Verdict — PASS.** All seven CI stages, four installed management tests and seven installed callback tests passed. The CI diagnostic and workflow fixtures passed 22 and 69 assertions respectively. The optional Chromium catalog test was outside the affected scope.

Reviewed the changes since the [previous pass](../../../18/code-hygiene/01/report.md) against the November 19 working tree on main at `dd1a5ad`. Both workspace selections moved from 0.45.8 to 0.45.15. Comparing each lockfile with its copy from the start of this pass showed only the Toko version and checksum changed. The earlier dependency refresh and ordinary CLI dev-dependency declaration were already present. [Toko code hygiene method](https://github.com/dragginzgame/toko-tooling/blob/a716b501274678a8ebf163ef576bbb549e974acd/audits/code-hygiene.md)

| Finding | Correction and evidence |
| --- | --- |
| Medium · CI failure diagnostics | Blob prints command output during setup, including snapshot admission itself. Its collector could emit bare Bearer credentials, embedded access-key values and private-key lines following an oversized opening delimiter. With synthetic values, the previous collector failed both the console and retained-artifact assertions. The revised collector passes them and preserves the child’s exit status. |
| Low · CLI refusal tests | Some certificate and installation tests accepted any error. They now distinguish decoding failure, a mismatched binding and an invalid installation signature. For a correctly signed but disallowed installation request they also verify the saved packet bytes. |
| Low · Rust layout | Moved late imports into the import block, separated imports from adjacent items and reused the existing deterministic certificate fixture. The ordinary CLI tooling dependency passes strict all-target Clippy and binary tests. |

The layout check covered 84 Rust files across product and qualification. It found no file/directory module collisions or source-path overrides. The six files with target guards cover IC hosts and private canister state. The enrollment probe’s native tests use the same record and argument encoding as its Wasm host. Blob declares its protocol and storage types directly. Deriving serialization or argument parsing adds implementations to those types. The new documentation rule for macro-defined types required no changes there.

Verification used Rust 1.100.0 on Linux x86_64. The selected checks cover snapshot and CLI digest admission, the aggregate CI gate, local workflow and collector fixtures, and installed management and callback behavior. Local logs are under `target/hygiene-04515/`. The diagnostic regressions use synthetic data. Hosted artifact upload and live provider operations were outside this review.
