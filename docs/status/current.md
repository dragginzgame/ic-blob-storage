# Current status — 2026-10-26

Source baseline: `122a2603`, following released 0.49.1 and merged
[PR #104 — Changelog sections](https://github.com/dragginzgame/ic-blob-storage/pull/104).
Pending compatible notes target 0.49.2; package versions remain 0.49.1.
The [released handoff](https://github.com/dragginzgame/ic-blob-storage/blob/8d8a455a/docs/status/current.md)
retains earlier cleanup/Tooling evidence and [handoff history](history.md) retains
older implementation records.

## Upstream adoption

The root catalog now requires published Host/Build/Testkit 0.68.9,
Memory/Storage 0.51.5, Platform/macros 0.6.19, Wire/derive 0.7.18 and Metrics 0.16.3.
The incoming lock already selects this graph and is preserved. No features,
workspace boundaries, endpoint identities, persisted schemas or exact retry
bytes change. No caller adaptation is needed. Cross-release service transitions
remain reinstall-only; unchanged schemas do not authorize an in-place upgrade,
and this batch adds no migration or dual reader.

All direct external Cargo requirements were checked against crates.io's canonical
sparse index. Other direct Rust packages are current except thiserror: retain
2.0.18 because PocketIC 16.1.0 requires it exactly. Do not force the incompatible
2.0.21 minimum. The twelve selected suppliers' `src/` trees match their earlier
qualified releases; Platform macros and Wire derive match too. Their reviewed
changes concern dependency minimums, tooling/governance and package metadata.
Source equality does not replace qualification of the selected composition.

The maintained npm selections are already current: published
`@tokostudios/ic-platform` 0.1.0, Caffeine 1.1.2, esbuild 0.28.2 and Playwright
1.64.1. Core 6.1.1 remains transitive through Platform and the explicitly qualified
Caffeine override. The browser manifests, lock and provider patch are unchanged.

Shared Tooling remains 0.26.21 at
`66c91c4412856baa147482e648fc80428db8b179`. Root release notes use its emoji
sections. Tool setup remains explicit; normal releases start fresh while
`release-resume VERSION=X.Y.Z` recovers one saved attempt.

## Integration boundaries

Blob owns tenant authority, service state, provider accounting, retention,
deletion and recovery; frameworks own their adapters and acceptance. Hosts can
use `workflow::funding::prepare` with `ServiceWorkflow::<Host>::ACCESS` to avoid
repeating authenticated preparation/journal-access orchestration. Canic owns its
remaining adoption and delivery. No sibling checkout is changed.

Application protocols remain on Wire. Required external/provider and persisted
Candid use Platform; stable collection codecs use Storage and allocation/backing
authority uses Memory. Do not infer Candid-free deployment or provider credit
from dependency adoption. The [native Agent advisory](https://github.com/dragginzgame/ic-blob-storage/issues/102)
remains unresolved; these releases do not supply its cryptographic correction.

## Qualification

Registry records, incoming files, supplier source comparisons and command logs
are retained under `/tmp/blob-upstream-20261026.CuhRRC/`. The incoming Cargo.lock
is byte-identical; locked source preparation and metadata admission pass.

Passing focused checks on this graph:

- Dependency pins, Platform boundary, runtime-free contracts and default
  standalone Wasm dependency exclusions; formatting, documentation links,
  browser build and browser admission (15 cases).
- Strict Clippy for core/contracts/client libraries, CLI targets, standalone and
  storage-probe Wasm, and the standalone PocketIC test target.
- Core funding (40), public Memory composition (2), CLI binaries (125), Wire
  golden bytes (1), and generated TypeScript (27 methods / 137 frozen frames).
- Rust 1.91 CLI and standalone Wasm checks, plus fresh development-compiler
  native CLI and release standalone Wasm builds.
- Installed standalone publication/session recovery (7) and exact callable
  deployment-surface/provider-contract comparison (1), using those fresh artifacts.

Initial installed attempts stopped at managed tool admission; one retry masked
that failure with a shell export and ran with an empty server path. Those retained
failures are not Blob runtime evidence. Explicit Testkit 0.68.9 setup in the
execution environment followed by separate Make admission resolved selection;
no installer code or receipt was changed by hand. Tool checks remain offline.

The full workspace gate, native macOS and live provider qualification were not
run for this focused adoption.
Earlier evidence at `/tmp/blob-adopt-tooling-20261026/` and
`/tmp/blob-cleanup0491-*` retains its original dependency scope.

The prior Canic read-only review found passing dedicated/embedded Blob 0.49.1,
generated-client, packaged CLI and operator-recovery evidence. Its newer Auth
selection and unfinished 0.112 source batch need Canic-owned closeout. Linux
PocketIC/provider substitutes do not qualify native macOS, production capacity,
deployed Caffeine, billing or independent provider-credit acceptance.

No staging, commit, push, release, publication, deployment or sibling edit occurs
as part of this adoption.
