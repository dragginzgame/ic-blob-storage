# Current status — 2026-10-27

Released baseline: `6798a827a3dc6187a788d9c9e333d51882160acc`, Blob **0.51.0**.
The pending batch is **0.52.0**, following explicit approval to retire journaled
on-canister chunk reads. Package versions remain 0.51.0 until maintainer release
preparation. Nothing staged, committed, pushed, published or deployed by this task.

## Accepted simplification batch

All six accepted changes are implemented using published Storage APIs.
Funding accounting remains a map because it indexes credit receipts.
The [qualification/migration record](../evidence/simplification0520.md) owns the
size table, exact source/tool/artifact identities, tradeoffs and evidence limits.

- Certificate resolution/commit reuses freshly validated permission records,
  retaining final actor, activation, expiry, fence and host-evidence checks.
- One manifest collection stores independently addressable headers and leaves.
  Descriptors skip leaf decoding; replay/inspection/restore validate both parts.
- GC polls a derived pending-candidate index within the upload owner. Lifecycle
  and original identities remain authoritative; acknowledgment is not physical
  deletion or billing cessation. Restore cross-checks membership without repair.
- Durable read-session owners, chunk transport/tickets, read budgets/status and
  gateway read-generation invalidation are retired. Direct client downloads,
  descriptors, body verification and independent transient source probes remain.
  Logical-memory IO measurement remains, independently of retired sessions.
- Configuration and gateway singleton maps now use governed cells through
  Memory/Storage 0.51.12. Non-initializing load preserves failed-restore behavior;
  the immutable configuration’s existing public accessors are unchanged.
- Workspace release builds select ThinLTO and one codegen unit, retaining level 3
  and symbols. Standalone is built separately from probes, preserving production
  feature exclusions. Downstream Canic retains its own root build profile.

Installation now uses fifteen canonical grants and format
`ic-blob-storage/installation:singleton-cells`. Configuration,
local-status and install Wire identities use `0052`; other owned frames and the
required external Candid methods retain their contracts. CLI status keeps the
single current **schema 1**. No compatibility reader or higher schema version was
introduced. New proposals omit retired read fields; old runs retain original tools.

Cross-release transitions remain retirement/reinstall. Preserve original uncertain
operations, provider objects, balances and continuing bills under the
[retirement runbook](../retiring-installations.md). No stale-backup activation,
automatic reset or loss of obligations is authorized. Same-release interruption,
receipt recovery and platform-history reconciliation remain supported/fenced.
The captured pre-cut populated image refuses the new artifact without changing
stable bytes; its original owner remains usable after rejected upgrade.

## Cell ownership

Published Storage 0.51.12 supplies the read-only loader requested by
[Persistence #136](https://github.com/dragginzgame/ic-persistence/issues/136).
Blob uses it directly; no local header parser, initializing fallback or caller
default is introduced. The upstream issue remains open for its separate macOS
tooling acceptance under Persistence #135; Blob adoption is locally complete.
Real pre-cell upgrade refusal retains all bytes and the
original object, configuration and accounting observations. Schema remains 1.

Read-only Canic review confirms no production read-session implementation; three
testing-internal configuration fixtures still name its retired input. Canic must
adopt the new configuration/status/installation contract and qualify its wrapper.
No sibling checkout was edited. Provider/application acceptance and the native
Agent [advisory #102](https://github.com/dragginzgame/ic-blob-storage/issues/102)
remain separate; this cleanup does not establish deployed Caffeine/Canic readiness.

## Graph and evidence

Require published Host/Build/Testkit **0.69.2**, already selected by the incoming
lock; no further lock changes. Native CLI/process and managed-server APIs need
no caller changes. Testkit and Build CLIs are explicitly prepared/admitted.
Memory/Storage **0.51.12**, Platform 0.7.0, Wire 0.7.18 and Metrics 0.16.3 remain.
Browser Platform 0.1.1 / SDK Core 6.1.1 / Caffeine 1.1.2 remain unchanged.

Raw evidence: `/tmp/blob-simplify052-20261027.wmcqnujw/` retains compiler inputs,
initial/final locks/patches, raw artifacts, commands, measurements and failed/retried
attempts. The cell
step is retained in `/tmp/blob-cell052-20261027.qOTAMW0S/`; standalone `after.wasm` SHA-256:
`a54edcafe0673fd381e596c0a59dfa81903355ff6a75cd5e06bda37615e68646`.
It is 2,585,430 raw / 659,413 gzip bytes: 19.0% / 5.9% below the captured baseline.
No endpoint IC-instruction, allocation or peak-heap gain is claimed.

335 current core tests, sixteen focused cell artifact/PocketIC cases, preceding
contract/golden/CLI tests, installed certificate/descriptor/operator/
recovery/GC/rollback/pre-cut cases, strict Clippy, native/Wasm Rust 1.91 checks,
deployment/dependency exclusions and generated TypeScript qualification pass.
Tooling 0.69.2: CLI 126, offline operator 25, subprocess 1 and installed lifecycle/
CLI 6 tests pass, with strict caller Clippy and native Rust 1.91 checks. Its
standalone deployment tree is byte-identical; evidence is retained at
`/tmp/blob-tooling0692-20261027.XFd9u5hk/`. Full CI, native macOS, production sizing,
deployed provider and downstream Canic qualification remain unrun.

## Preserved Shared Tooling adoption

Shared **0.26.25**, committed source
`6b6bbd9c9e5798f0ecf6105637392a7a55cd450e`, remains the reviewed 130-file snapshot.
The root formatter covers all thirteen members, preserves dependency roles and
shows per-pass timing; unchanged-input reuse excludes staged hooks. Gitlinks stay
opaque. Its prior hook rollback, fixture/release-command and forced-formatting
proofs remain in `/tmp/blob-shared02625-20261027.WNse8z53/` and
`/tmp/blob-shared02623-20261027.0DfInVjK/`. This batch preserves those pending changes;
no mutable sibling tooling or generic producer matrix was copied or replayed.
