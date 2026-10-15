# Memory 0.41, Metrics 0.11, Testkit 0.46 and Shared 0.15.1

Date: 2026-10-15. Starting source:
`c9aea8b52d471a1b17b158e21a2e71addafdec25` (released 0.27.1).
Incoming dirty manifest/lock selections choose Metrics 0.11.0 and Testkit 0.46.0;
Memory 0.41.0 is already in that released source. Those selections are preserved.
Own package versions remain 0.27.1; the compatible pending notes target 0.27.2.

## Published boundary and Canic

An exact crates.io observation confirms Blob and contracts 0.27.1 are present.
The checksum-verified Blob archive identifies source
`c9aea8b52d471a1b17b158e21a2e71addafdec25` and requires Memory `0.41`.
This corrects the initial assumption that the selected Memory family still
needed a future 0.28 release. The earlier 0.27.1 unchanged-contract note and
Memory 0.40 qualification do not describe this released Memory 0.41 boundary;
published history is preserved rather than silently rewritten. This batch
qualifies and documents that existing boundary instead of introducing another
public type or retained-format change.

The composition regression now passes Blob's actual public installation factory
into the host's directly imported `Vec<MemoryRequest>`, composes its namespace
with a neighbor, bootstraps once and opens the real installation key. Existing
collection reopening, host bucket profile, committed grants and ungranted-key
refusals remain covered. Public Rustdoc names the actual Memory 0.41 family.

Consumers on Blob 0.27.0/Memory 0.40 must select matching Memory 0.41 types.
The upstream numeric-slot ledger uses `ICMN` version 1 rather than the older
`ICMS` logical encoding. Old ledgers require frozen inspection and the existing
[retirement runbook](../retiring-installations.md); no earlier-layout reader,
automatic reset or retained-data deployment is supplied. Same-release recovery
remains required. Canic owns its root/private fixture locks and adapter tests;
[#77 — Align exposed Memory types with 0.41](https://github.com/dragginzgame/ic-blob-storage/issues/77)
tracks that consumer handoff and documentation delivery.

## Selected sources and ownership

- Memory 0.41.0 registry source: `2c4a6871a805214628b35a634fba65e968e7e1ee`.
- Metrics 0.11.0 registry source: `d68886af183688e365338196a17b83f5d0ed050d`.
  Its raised Rust 1.91 floor already matches Blob; private probe wire fields stay
  unchanged. Testkit retains its own private Metrics 0.10.1 dependency.
- Testkit 0.46.0 registry source: `a6082d6a241c885c6bb398f1d8e5aa7936ff8fc0`.
  Direct callers and Testkit now resolve one Host 0.19.0 artifacts/fs/process
  identity. No source override forces those exposed types across minor lines.
- Shared snapshot: reviewed committed `4f3eec42f52d4ec308bb68ab2daa3e5266b3175a`
  (0.15.1), exported from a clean isolated clone into 133 payloads. The actual
  formatting-hook caller delegates selected-pair preparation to Shared while
  retaining its dependency choices, failed-formatter rollback and index/lock
  preservation. Common executable pins are unchanged. New qualification guidance
  does not automatically remove any existing gate; pending Shared projection and
  larger qualification work are excluded.

Host framing/retained-write PRs #128/#132 and Memory's bounded fixture-cell
extension #105 remain unmerged at inspection. Published Memory's unbounded
fixture cell cannot replace Blob's bounded consumer journal. No pending producer
API, sibling dirty work or compatibility path is adopted.

## Focused qualification

Raw logs, before-images, graph metadata, registry archives/readback and input
hashes remain under `.tmp/review-validation/adoption0280/`. The directory keeps
its initial assessment name. Its initial registry invocation used the wrong
argument order and refused before observation; corrected readback and archive
verification are retained separately. The initial isolated 0.28 renderer case
passed but was superseded by the published-archive classification above.

Passing checks on the frozen incoming registry graph:

- Two composition and five store-grant cases; affected core/probe and native
  CLI/PocketIC all-target compilation, strict affected-package lint, and actual
  Rust 1.91 native/Wasm qualification. No new public runtime implementation or
  DTO/schema is introduced by this task.
- 117 CLI and five probe cases. Explicit Testkit 0.46 setup/check admits the
  existing PocketIC 16.1 bytes against its unchanged archive pin.
- Fresh CLI/seven Wasm artifacts: buffered output, two installation/interface
  cases, four same-release lifecycle/recovery cases, operator-attributed Metrics
  restoration reads, seven publication-session cases, three lost-reply/recovery
  cases and the ten-scenario Chromium certificate journey. Artifact and selected
  manifest/lock/snapshot hashes remain unchanged afterward. Artifacts retain
  their source package version 0.27.1; these are not a future 0.27.2 deployment.
- Actual hook preservation and producer selected-pair/refusal fixtures, shell
  gate, formatter/manifest ordering, dependency declarations and snapshot checks.
- An isolated real twelve-member workspace prepares the correct 0.27.2
  candidate, updating its contracts requirement while preserving external lock
  rows, manifest mode and ordering. Normal package versions are unchanged.

Full CI, native macOS, Canic/application and deployed provider acceptance remain
separate. Released 0.27.1 CI was still in progress at inspection and does not
qualify dirty source. No full local CI, release/publication, reset, paid effect,
sibling edit, Blob index change or Git-ref mutation ran.
