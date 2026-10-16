# Canic integration review and native evidence owners — 2026-10-16

Blob base is released 0.30.0 `e9f71822ae361377686846fe7a4bf7c0e9a6822f`, plus
incoming dirty 0.31 preparation. Before-images, GitHub readbacks, selected Canic
source hashes, commands, failures and results are retained under
`.tmp/review-validation/canic-issues0310/`. The incoming lock selects Memory 0.44,
Metrics 0.11.7, Host 0.23.1, Build 0.11 and Testkit/Candid 0.49. Cargo.lock is
unchanged by this work. Own versions remain 0.30.0; prior decoder API work keeps
the numbered notes at 0.31.0.

## Local repairs

[#75](https://github.com/dragginzgame/ic-blob-storage/issues/75)'s consumer
fingerprint API is present in published Build 0.11.0, registry VCS
`dff188fe27136054a91044900ae1cd607669d870`. The CLI build script captures its
entire source directory, itself, package/root manifests and root lock, with
workspace-relative evidence. This replaces manual source lists in both native
and public-source probe records. Existing prefixed/raw digest representations
remain; the value now describes the complete selected source inventory. A source
digest is neither executable attestation nor authority to repeat an effect.
Build is used at CLI build time and by the existing optional archive verifier;
public libraries/contracts and production Wasm remain independent of it.

[#69](https://github.com/dragginzgame/ic-blob-storage/issues/69)'s operational
path policy now belongs to Blob before Host creates a private run directory.
CR/LF in selected paths and canonical existing parents refuses without creating
a claim. Ordinary spaces, occupied evidence, owner permissions and directory
custody retain their prior behavior. Host still performs filesystem admission;
its current duplicate control-path check can retire through its own reviewed
consumer/host qualification. The descriptor-rooted reader proposal and actual
current-graph archive qualification are separate outstanding work.

## Focused Linux results

- 125 CLI/probe/subprocess cases pass; seven artifact cases are included in that
  total. Strict all-target/all-feature CLI Clippy and actual Rust 1.91 compilation
  pass, including the new build dependency and optional verifier.
- Four actual Cargo builds prove source-directory membership invalidation:
  baseline, added input, renamed input and removal. Addition and rename produce
  distinct fingerprints; removal restores the exact baseline digest. The temporary
  source inputs are removed after the experiment and its logs remain retained.
- Seven installed publication-session cases pass with the freshly built CLI.
  The reused standalone Wasm matches its earlier decoder-cleanup receipt exactly;
  this is not a fresh Wasm build. PocketIC 16.1 executable/archive/decompression
  pins are rechecked against published Testkit's selection before execution.
- Source formatting, manifest/dependency inheritance and documentation checks are
  recorded in the raw evidence. These focused checks are not complete CI,
  selected executable-tool installation, native macOS or deployed-provider proof.

## Canic boundary

Read-only Canic HEAD is `600a831e0d8241796b70773ea8c4d8fbd63e422c`, initially clean.
Concurrent owner edits subsequently changed its root lock and Host callers. Final
readback retains the same Blob/Memory selections and unchanged private locks;
the earlier metadata failures retain their exact inspection scope. Its root lock selects Blob/contracts 0.30.0 and Memory 0.44.0.
Both independent Blob fixture locks still select 0.27.1 and 0.41.0, and the
embedded fixture declares Memory 0.41. Both workspaces refuse locked/offline
metadata resolution because their locks need updating. The exact failed commands
and diagnostics are retained; no Canic build or installed failure is claimed.
Canic owns coherent root/private adoption and renewed dedicated/embedded proofs
under [#517](https://github.com/dragginzgame/canic/issues/517) and
[Blob #77](https://github.com/dragginzgame/ic-blob-storage/issues/77).

For eventual Blob 0.31 adoption, its account and gateway limit adapters must use
`CaffeineReplyLimits`, preserving the existing numeric budgets. A two-file patch
passes `git apply --check` at that Canic HEAD and is retained, unapplied, as
`canic0310.patch` at the raw evidence root. It does not belong to current 0.30
adoption. Canic's configuration decoder already uses zero skipping quota.
No sibling source, lock, index or build artifact is changed by this review.

## Delivered issues and PRs

[Blob 0.30.0's matching workflow](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38902794018)
passes Linux, Apple Silicon, Intel macOS and Rust 1.91. Its maintained tooling,
archive, native CLI/install/Metrics and optimized-Wasm checks support closing the
superseded exact-0.22.6 tooling tracker
[#57](https://github.com/dragginzgame/ic-blob-storage/issues/57) and delivered
Testkit phased-HTTP retirement
[#70](https://github.com/dragginzgame/ic-blob-storage/issues/70). Earlier cancelled
runs remain historical evidence, not successful old-graph execution. Local dirty
source has no remote result and cannot close #75 or the outstanding #69 scope.

[PR #73](https://github.com/dragginzgame/ic-blob-storage/pull/73) has all four
checks green but conflicts with current main; its license changes require owner
reconciliation. [PR #78](https://github.com/dragginzgame/ic-blob-storage/pull/78)
separately adopts direct Host HTTP capture, preserving bounds, absolute deadlines
and caller-owned responses. At the final checkpoint Linux, Apple Silicon and
MSRV pass; Intel macOS remains running. Patch review found no additional blocker.
Neither PR was merged or modified, and no CI rerun was requested. Host framing
and retained-record PRs remain open, so #74/#76 do not have registry APIs to adopt.
Framework qualification remains distinct from the application's real users,
verifier, asset transaction and deployed provider GC/funding evidence.
