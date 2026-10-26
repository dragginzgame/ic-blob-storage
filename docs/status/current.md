# Current status — 2026-10-26

Released baseline: `8d8a455a` (0.49.1). Local main is `6685029b`, after merging
[PR #104 — Use IcyDB-style sections in the latest root changelog](https://github.com/dragginzgame/ic-blob-storage/pull/104).
Package versions remain 0.49.1. The [released handoff](https://github.com/dragginzgame/ic-blob-storage/blob/8d8a455a/docs/status/current.md)
retains the cleanup and Tooling adoption evidence; earlier records remain in
[handoff history](history.md).

## Current adoption

Shared Tooling 0.26.21 is exported from clean committed source
`66c91c4412856baa147482e648fc80428db8b179`. Only the baseline's changelog
presentation guidance and its detailed rule change. Root notes use emoji category
headings; a small entry can use one `📝 Summary` section. Versions, dates, content
and older history remain preserved. PR #104 supplies that heading in the latest
released entry. This governance-only adoption creates no new release draft.

The snapshot and this handoff remain local edits. No product implementation,
endpoint bytes, persisted schemas, retry policy or executable tooling changes
are part of this adoption. Shared's SDK boundary checker and Blob's installed
SDK identity checks remain as delivered in 0.49.1.

## Dependency state

The [root catalog](../../Cargo.toml) retains Host/Build/Testkit minimums 0.68.7
and Rust Platform 0.6.18. The incoming [lock](../../Cargo.lock) selects Tooling
0.68.8, Platform/macros 0.6.19 and openssl-probe 0.2.2. Those edits are preserved
byte-for-byte; their adoption and runtime qualification were not requested or
performed in this pass. Do not reuse the prior graph's evidence for them.
Memory/Storage remain 0.51.3, Wire/derive 0.7.16 and Metrics 0.16.2. Browser
Platform remains published `@tokostudios/ic-platform` 0.1.0 with Core 6.1.1 and
the existing Caffeine override. Application protocols stay on Wire; required
external/provider and persisted Candid contracts retain their existing owners.

## Released integration support

- Hosts can use `workflow::funding::prepare` with
  `ServiceWorkflow::<Host>::ACCESS`: authenticate the exact intent, release the
  borrow for synchronous host evidence, then recheck and reserve. Canic owns its
  adoption and removal of its private journal-access adapter.
- Funding reads reuse validated views without removing record validation,
  accounting obligations or restore fences. Operator scope and bounded provider
  decode mechanics have shared owners while retaining authentication and errors.
- CLI preparation uses typed saved status, narrowed setup operations and shared
  document bounds. Browser request/retained-intent validation shares pure rules;
  canonical headers, exact bytes and no-resubmission boundaries remain separate.
- Native pipe lifecycle state remains independent because startup, pending work,
  stdin closure and exit can overlap. No competing phase machine was added.

## Qualification and next actions

The PR's two-line insertion was reviewed against exact head `c5d00831`. Removing
that heading and its blank line reconstructs the pre-PR changelog byte-for-byte.
There were no review objections or enforced branch requirements. Schema and MSRV
jobs passed before merge; optional native tooling jobs were still running.
The [merge CI run](https://github.com/dragginzgame/ic-blob-storage/actions/runs/40363210822)
was pending at readback; pending is not passing evidence.

Snapshot integrity, documentation links and diff checks pass for this adoption.
No Rust/runtime suites were rerun for the documentation-only changes. Previous
0.68.7 qualification remains in the released handoff and
`/tmp/blob-adopt-tooling-20261026/`: scoped CLI/standalone Clippy, 125 CLI/probe
cases, seven installed sessions, Wasm surface inspection and CLI Rust 1.91.
Earlier cleanup qualification remains at `/tmp/blob-cleanup0491-*`.

Current review, preservation proof and refresh evidence are under
`/tmp/blob-shared02621-pr104/`. Confirm normal merge CI separately and qualify the
incoming dependency graph before claiming its runtime acceptance. Native macOS,
other engines, production capacity and deployed-provider behavior have no new
qualification here. Installed provider substitutes remain local evidence.

[Blob #103 — Browser Platform adoption](https://github.com/dragginzgame/ic-blob-storage/issues/103),
[Canic #637 — Browser Platform adoption](https://github.com/dragginzgame/canic/issues/637)
and [Blob #102 — Native Agent advisory](https://github.com/dragginzgame/ic-blob-storage/issues/102)
retain their separate ownership; no new acceptance claim or issue closure is made.
No source commit, release, publication, deployment or sibling edit occurred for
this adoption. The authorized PR merge is complete; local edits are unstaged.
