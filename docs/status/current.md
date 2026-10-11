# Current status

Date: 2026-10-11

## Released source and current batch

Released HEAD is `25d2a66319dd8678bd66355a86d8b068c2b46d32` (0.22.5), with
validated source `4dfabc18fcbad3b97d85db390108b8410eefc425`. All twelve package
versions and the finalized receipt remain 0.22.5. Its committed lock selects
Memory 0.35.6, Metrics 0.5.6, Testkit 0.33.1 and all four Host 0.12.10. Earlier
0.22.5 local qualification used Memory 0.35.5/Host 0.12.9; keep it bound to those
inputs rather than relabelling the historical evidence.

Pending compatible 0.22.6 adopts canonical Shared 0.3.11
`db143ae9ae140eb66826c413360723d23c078f66` with the same 99-file roster and tool
pins. Rust tool probes/installations use the selected consumer context; fatal,
LLVM and missing-file diagnostics remain in retained summaries. Temporary-path
fixes cover symlinked roots and relative TMPDIR. Dirty sibling hook changes are
excluded. No affected dollar-prefixed derive macros were found in Blob.

Preserve incoming direct Host artifacts/fs 0.13.0, private Metrics 0.6.0 and
Testkit 0.33.2. The removed Host lock-progress helper is unused; the changed Metrics
tracked aggregates are unused by the read profiler's existing record_sample
primitive. Host artifacts and Testkit Rust sources match the released selections.
Host fs removes the unused helper. Testkit retains its private Host 0.12.10 graph;
no Host identity crosses a public Blob boundary. Memory remains one 0.35.6 identity.
Public Blob contracts, CLI results and stored schemas stay unchanged, so no Blob
minor hard cut or new migration is needed for this batch.

Explicit locked-cache preparation and Testkit installation/checks pass, retaining
old CLI slots and the authenticated PocketIC 16.1.0 server. Complete delivery
coverage and Rust 1.88 qualification pass on fixed inputs, including 643 native /
376 enabled PocketIC cases, all four Wasm optimizer variants and exact archives;
see [the current evidence](../evidence/shared03110226.md).
Fix #56's missing native archive gate: all three hosted lanes now invoke
`make package` and retain archive/admission/integrity and regression evidence,
excluding compiler caches. The first complete local gate reaches native tests
but socket-based cases refuse with sandbox `Operation not permitted`; retain that
attempt and pass affected tests with local networking permitted. Initial input
capture referenced a helper removed by maintainer cleanup; replace it before
Rust compilation, retain the empty failed capture and pass repeated early tooling
checks. All 1,720 input hashes/modes stay fixed across the corrected retry/floor
checks. Final edits only attest this evidence; compiler/product inputs stay fixed.
No agent commit, push, version transaction, release/publication, sibling edit or
paid provider effect.

## Current checks and next work

- Matching [0.22.5 CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38121988249)
  passes Linux, Apple Silicon and Rust 1.88 at final review; Intel is still running.
  Keep #54/#55/#56 open for their matching native acceptance. The prior 0.22.4
  run was cancelled, and dirty 0.22.6 has no hosted result.
- Completed 0.22.3 hosted evidence owns closure of #52/#53. Earlier cancelled
  revisions and older dependency graphs are not relabelled.
- Canic owns managed wrapper/composition and coherent framework publication under
  [#444](https://github.com/dragginzgame/canic/issues/444); guarded funding evidence
  acquisition remains Canic #494 / Blob #34. Do not add framework dependencies,
  fixtures or implementation here. Remaining #4/#5/#6/#31/#32/#33/#34 require
  concrete application/provider, operational or browser acceptance, not another
  local substitute or an invented deployment/signer/paid probe.
- [#57](https://github.com/dragginzgame/ic-blob-storage/issues/57) tracks the current
  tooling/dependency batch. After fixed
  local qualification, inspect matching hosted acceptance once more and close
  only issues whose stated scope is actually complete.

[Previous review handoff](review-before-0226.md) and
[history through 0.22.4](history-through-0224.md) retain earlier source-bound results,
failed attempts, pending limits and the approved Shared #109 feedback. Historical
handoffs are evidence, not current command authority.
