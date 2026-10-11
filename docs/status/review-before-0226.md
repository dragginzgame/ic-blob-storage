# Current status

Date: 2026-10-11

## Released source and current batch

Released HEAD is `037f91c8ab9ba70675ea76a90a1c7591b2bc554d` (0.22.4),
with validated source `9cd56de4668f4c4b7d950dd1fc6764540088ce4d`. All twelve
package versions and the finalized receipt remain 0.22.4. Its committed graph
selects Memory 0.35.4, Metrics 0.5.5, Testkit 0.33.1, all four Host 0.12.8,
serde_json 1.0.152 and PocketIC 16.1.0. Earlier 0.22.4 adoption evidence selected
older dependencies; retain that distinction. The previous ignored local packets
are no longer present after maintainer cleanup; their durable summaries remain.

The qualified pending 0.22.5 graph preserves incoming Memory 0.35.5, Metrics 0.5.6 and
all four Host 0.12.9 lock edits. Their packaged Rust sources match the released selections;
all other lock selections/edges stay unchanged. Public Blob API and stored schemas
are unchanged. Adopt reviewed Shared 0.3.8 at
`67285b28a98b7c4211ad32de726709d4e87edea4` through the canonical exporter, with
its same 99-file roster and existing native matrix/concurrency policy. Ignore
uncommitted sibling changelog bytes. Sibling repositories remain read-only.

Strengthen coordinated archive qualification: admit both exact unpacked manifests,
names, versions and workspace membership before compilation, retain external
lock selections, and check final archive/source/mode/manifest/lock integrity.
The verifier rejects incomplete exits with retained evidence. Actual Cargo
regressions cover same-version warm-cache execution, false metadata, failed reads
and post-build drift, including paths with spaces and Unicode. Current qualification
is [recorded separately](../evidence/libraries0225.md): full `make ci`, genuine
Bash 3.2 regressions and Rust 1.88 native/Wasm checks pass on fixed inputs.
No agent commit, push, version transaction, release, publication or paid effect.

## Shared 0.3.9 / Host review — 2026-10-11

Reviewed canonical Shared 0.3.9 `1ed2ca523186641e0e0cdf77e3cb10814c22a430`
and Host 0.12.10 `5074de1792f920112c6539629605a6624f573863`; remote main/tag
readbacks match the clean committed sources. Shared adoption remains 0.3.8.
The next refresh changes four selected files: Rust installation/check context,
its fixture, validation diagnostic classification and local setup documentation.
It keeps tool pins and the existing roster. Rust setup now enters the selected
consumer before all probes/installations; fatal/LLVM/missing-file diagnostics
survive retained summaries. The canonical runner fixture remains producer-owned.

Incoming Cargo.lock now selects all four Host 0.12.10 crates. Their 19/18/21/8-file
Rust trees match 0.12.9; the upstream patch adopts Shared tooling and changes no
library API. Preserve these incoming edits. The earlier full gate qualified
0.12.9, so it does not establish this newer graph's readiness. No additional code
removal or Host migration is required by this source review.

Focused canonical Rust installer (substitute Cargo) and validation runner fixtures
pass on current Bash and genuine Bash 3.2.57. The initial Bash 3.2 runner attempt
from a Git-less export fails at its nested clone prerequisite; retain that attempt
and pass the rerun from a clean isolated checkout. Evidence:
`target/review-validation/shared039-host01210-review/`. Inspection only: no snapshot
refresh, dependency resolution, full consumer gate, commit or release effects.
Uncommitted sibling edits appearing during review are excluded.

## Latest Shared review — 0.3.10

Canonical remote main and the clean sibling checkout now agree on Shared 0.3.10
`37af29b2f8fb4532744b600c5f85ce716a8b57cf`. Relative to reviewed 0.3.9, four
selected files change: `docs/local-setup.md`, `rules/git-hooks.md`,
`scripts/ci/check-release-commands.sh` and `scripts/ci/test-host-tools.sh`.
The fixture/checker fixes normalize physical temporary roots and preserve nested
Make probes under relative TMPDIR. Producer-only portable/release/frontend tests
also expand coverage; no tool pins change and no new fixture needs vendoring.

The documented cargo-sort-derives 0.13.0 workaround protects complete macros with
dollar-prefixed derive paths using its existing disable-range markers. Source
inspection of all Rust package directories finds no `$crate` or dollar-prefixed
derive attributes in Blob, so no protective markers or local macro edits are
required. The producer's real hook/downstream compilation fixture qualifies the
scoped workaround, not unprotected rewriting or a replacement tool pin.

Focused canonical host-tool and release-command fixtures pass on current Bash
and genuine Bash 3.2.57 from a clean isolated committed checkout, using a
symlinked temporary parent. The release fixture also exercises relative temporary
directories, nested Make probes and retained refusals with a substitute release
runner; no real release occurs. Evidence remains in
`target/review-validation/shared0310-review/`. Shared adoption here still reads
0.3.8; this inspection performs no refresh, product compilation or full CI.

## Current checks and next work

- Matching [0.22.3 hosted CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38060598267)
  passes Linux, Apple Silicon, Intel and Rust 1.88, including actual original/O3/Os/Oz
  installation/restoration qualification. Closed #52 and #53 against that completed
  source; earlier cancelled 0.22.2 evidence is not relabelled.
- [0.22.4 hosted CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38071662625)
  has successful Linux and MSRV jobs; Apple Silicon is running and Intel is queued
  at final review. #54/#55 retain their matching native acceptance boundaries.
  [#56](https://github.com/dragginzgame/ic-blob-storage/issues/56) tracks the archive
  guards' delivery and native acceptance. Dirty 0.22.5 changes have no hosted result.
- Blob #54/#55/#56 have qualification/readback comments. After explicit maintainer
  approval on 2026-10-11, the retained consumer feedback was
  [posted to Shared #109](https://github.com/dragginzgame/shared-tooling/issues/109#issuecomment-6106452167).
  The earlier automatic approval rejection remains recorded in the evidence.
- Canic owns current managed wrapper/composition and coherent framework publication
  under [#444](https://github.com/dragginzgame/canic/issues/444); guarded funding
  acquisition/dispatch remains Canic #494 / Blob #34. No framework dependency,
  fixture or sibling implementation belongs here. New library availability is
  distinct from managed runtime and application acceptance.
- Remaining #4/#5/#6/#31/#32/#33/#34 require their concrete consumer/provider,
  operational or browser acceptance. Continue with the current library/package
  batch; do not invent a deployment, signer or paid provider probe.

[Historical handoff through 0.22.4](history-through-0224.md) preserves earlier
implementation, source-bound validation, failures and acceptance limits. It is
historical evidence, not the current command authority.
