# Current status — 2026-10-28

## Released baseline and pending work

Blob **0.52.0** is tagged at `be64a7e371c9d1b120162182008348fb46c3f389`;
its receipt names validated source `74c07658df6ceb8fbab54d9dfca1ddd9a931bb8e`.
The pending compatible **0.52.1** batch implements #105/#106/#107 and the prior
Canic/GC documentation corrections. Package versions remain 0.52.0. Nothing was
committed, pushed, published or deployed by the agent; maintainer-staged work is preserved.
Existing incoming Cargo.lock edits are preserved byte-for-byte.

## Implemented issue fixes

- **#105:** reproduced authorized history recovery returning Binding while sixteen
  denied updates arrive during its await. A released object's 1024 physical and
  liability bytes remain outstanding in that fixture. The installed workflow now
  accepts uninterrupted platform observations from the same live owner, while
  still obtaining complete independent IC history and consuming a sealed proof
  in the exact callback. An Arc-backed transient identity is invalidated by every
  version gap, explicit fence or reopened heap owner. Later observations cannot
  revive it; stored schemas, anchors, journals and obligations are unchanged.
  Direct `prove_current_instance` callers retain the strict single-step guard.
  Unobserved application executions/arbitrary platform jumps still refuse safely;
  composed hosts must observe their relevant executions. No ingress filter,
  compatibility reader, automatic retry, backup activation or effect replay was added.
- **#106:** one Linux hosted job runs core and client library tests separately
  through the existing nonempty runner. The actual extracted job body passes on
  two tiny real libraries; deliberately failing either test exits the job with 101.
  Existing contracts/native-host matrices are retained. The job is not yet hosted.
- **#107:** the published 0.52.0 client archive really lacks license text. The initial
  package LICENSE symlink passed Cargo but violated the shared no-symlink rule and
  blocked the maintainer's commit. It is replaced with a regular MIT copy; packaging
  checks regular-file status and equality against root LICENSE. SPDX stays MIT.
  The published 0.52.0 archive remains unchanged; this is a pending package fix.
  Hook/archive proof: `/tmp/blob-license-hook-fix.KoY5miUj/`; real index unchanged.

The [Canic checklist](../dependencies.md#canic-integration) and GC guide now reflect
retired read fields, canonical grants, regenerated clients and pending GC indexing.
All three issues remain open until their fixes are verified on default main.

## Current qualification

Evidence: `/tmp/blob-issues105-107.zRpGdxat/` retains failed attempts and final checks.
Pre-fix artifact: `/tmp/blob-cell052-20261027.qOTAMW0S/after.wasm` (before version bump).
Final source/graph: 339 core and 4 client tests, strict affected-package Clippy,
5 installed lifecycle cases, real snapshot-rollback refusal and exact deployment
exports pass. Four new native observation tests cover continuous execution,
missed versions, explicit fences and reopened owners without changing stable bytes.
Rust 1.91 native/core/client and standalone Wasm checks pass. The real prepared
client archive has matching MIT metadata/text; its local filename still says 0.52.0.
Workflow extraction and deliberate failure checks use actual Cargo in a disposable
workspace, not mocked success. actionlint, formatting, dependency/deployment
boundaries and documentation links pass; no full CI or new macOS run is claimed.

Testkit 0.69.3 setup was initially refused based on its earlier unpublished state.
Public registry/checksum verification resolved this; canonical setup/admission
succeeded without sibling source or an alternate installation mechanism.

## Released contracts and Canic

The [0.52 record](../evidence/simplification0520.md) owns the released simplifications,
exact measurements and proofs for manifests, GC indexing, retired sessions and cells.
Funding accounting remains a multirow receipt-index map. Installation uses fifteen
canonical grants and `ic-blob-storage/installation:singleton-cells`; owned schemas
and CLI status remain 1. Descriptor/client downloads remain supported.
Cross-release transitions require [retirement/reinstall](../retiring-installations.md),
preserving original uncertain operations, provider objects, balances and billing.
Same-release interruption/recovery and exact receipts remain required.

Canic **0.112.0** main/tag is `5c236ab130c0d63892ef281b45bdc3fa353e012f`,
selecting Blob/Contracts **0.51.0**. Its complete release and family publication
passed; retained dedicated/embedded and funding-profile tests pass. Read-only
source inspection still finds two internal constructors using ServiceReadInput.
Canic needs 0.52 adoption and its own composition qualification; no copied grant
roster or production chunk-read transport was found.
[Canic CI 40659441082](https://github.com/dragginzgame/canic/actions/runs/40659441082)
was running at the last inspection. #444/#622/#494 record delivered source and
remaining hosted/independent-credit acceptance. No synchronous CI wait was performed.
Live users/provider uploads/serving, payer linkage, deletion and billing acceptance
remain separate under Blob #4/#5/#6/#31/#34. Acknowledged funding does not require
independent credit after every success; do not fabricate stronger evidence.

## Graph and retained evidence

Minima remain Tooling 0.69.2, Memory/Storage 0.51.12, Platform 0.7.0. The preserved
lock selects Tooling 0.69.3, Memory/Storage 0.51.13, Platform 0.7.1, Wire 0.7.18,
Metrics 0.16.3. Browser Platform 0.1.1/Core 6.1.1/Caffeine 1.1.2 are unchanged.
[Platform #50](https://github.com/dragginzgame/ic-platform/issues/50) and
[Persistence #136](https://github.com/dragginzgame/ic-persistence/issues/136)
retain their supplying acceptance scopes; no closure occurred in this batch.
Older source-bound captures remain in `/tmp/blob-simplify052-20261027.wmcqnujw/`,
`/tmp/blob-cell052-20261027.qOTAMW0S/`, `/tmp/blob-tooling0692-20261027.XFd9u5hk/`
and `/tmp/blob-retired-read-tests-20261027.t7LYiMA5/`.
Released Blob 0.52 hosted runs 40563993222 and 40563993188 passed Linux, both macOS
architectures, MSRV/generated-client and example lanes. #103 is closed as adopted.
Those runs do not qualify this dirty patch or establish live-provider readiness.

Shared **0.27.3**, revision `cc81a58a5b2dd287c577cbffd2c576c178b6785c`: 131 payloads.
Source comparison, offline integrity, formatting/pins and hook/release/Testkit callers pass.
Evidence: `/tmp/blob-shared0273.Q4qkZ1zY/`. Index/lock unchanged; re-stage refreshed files and new RustSource.pm.
