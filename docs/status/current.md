# Current status

Date: 2026-10-11

## Released source and current batch

Released Blob is `e8a19df6f172b04af53d23de9727b7b6b1664c91` (0.22.7),
validated source `9d054c42f8ae255324f7993773715853cf5402df`. All twelve package
versions and the finalized receipt remain 0.22.7. Its released lock selects
Memory 0.35.8, private Metrics 0.6.2, Testkit 0.33.5 and Host 0.13.2; PocketIC
remains 16.1.0 and the Rust floor 1.88. Incoming Metrics 0.6.3/Testkit 0.33.6
lock edits were preserved. Earlier compiler results do not qualify this graph.

Pending compatible 0.22.8 adopts committed Shared 0.6.0 at
`75180980a0c4e4252d8a915aaecfaefd92bf9a4e`, including the explicitly selected
Cargo build companion for 102 files with unchanged pins. Host, IC and Cargo tools
use the canonical per-user store with stable checkout command paths. Cargo
identities include the prepared compiler/native environment, source and packaged
lockfile; setup/checks require managed source-cache admission. Testkit keeps
PocketIC ownership. Preserve old bundles, Cargo slots, common sets and failures.

Blob now archives safe local evidence and diagnostics when shared-store admission
fails, exposes the archive for upload and subsequently reports incomplete
collection. Shared owns local Rust and referenced Cargo payload selection.
The local Testkit fixture retains adapter/ordering/server refusal coverage while
canonical Cargo installation cases stay with Shared's fixture.

Actual complete Linux setup/offline checks and a second checkout's offline reuse
of the same four Cargo executables pass. All 8,108 old local Rust regular files
retain identical bytes/modes. Initial setup refused Canic's live Testkit lock;
after it cleared, retry passed without reclamation. Initial ShellCheck's collector
trap warning was fixed, retaining that attempt. Shared fixtures pass on Bash 5.2;
affected fixtures pass on genuine Bash 3.2, including safe partial archives and
premature exits. Shell, actual formatting/hook, snapshot, declarations, links,
workflow and diff checks pass. See [the current record](../evidence/shared0600228.md).
These are focused tooling checks, not full CI/release or new product compilation.

The maintainer's 11:54:02 UTC release validation failed because committed adoption
linked the evidence record before its creation. The retained failure log remains
in Git's validation-failures directory. Current-tree documentation links and
ShellCheck pass; the record is still untracked and must join the final collector
correction and this handoff in the next maintainer source commit before retrying.

The maintainer committed adoption as
`858c68845c4ee571c709f326e200f7770cfe7f37` during qualification. The final collector
correction and current evidence/handoff follow that commit. Preserve the real
index and refs. No agent staging, commit, release/publication, sibling edit or
paid effect occurred. [The preceding 0.5 record](../evidence/shared0500227.md) and
[preceding 0.4 evidence](../evidence/shared0400227.md) keep their original sources
and limitations; their absent-tool-tree observation does not apply to this turn.

The preceding Shared 0.3.12 adoption repaired Blob's local hook fixture too:
resolve its temporary checkout
physically and pass its physical parent to child hooks before changing directories.
With only the shared fix, actual relative-TMPDIR consumer qualification fails;
preserve that initial attempt and its fixtures. Current Bash and genuine Bash
3.2 subsequently pass all three root spellings, success cleanup and injected
baseline-failure retention, without changing the real source or index.

The preceding snapshot/tool admission, declarations/links, shared fixtures,
boundary, shell, release, hook and formatting checks passed at their recorded
inputs. See [that evidence](../evidence/shared03120227.md) for precise reuse of
unchanged release-validated inputs; it does not qualify the new installer or
incoming lock edits. Blob public APIs and durable schemas remain unchanged.
The current developer storage hard cut requires explicit host/IC setup. No service schema
migration is introduced. No agent commit, push,
version transaction, release/publication, sibling edit or paid provider effect.

Further issue review corrects stale operator guidance: tenants approve exact
per-upload grants for multiple browser principals, the verifier stays separate,
and changing immutable installation limits requires retirement/reinstall.
The service contract now links the recorded consumer-owned Canic composition
without qualifying another release graph or deployed application. Scoped review
of publisher recovery/map gating and verifier evidence found no new local code
defect. #4/#6/#31/#33/#56/#57 now describe remaining acceptance rather than
already delivered primitives; their historical comments/evidence remain.
That addition was documentation-only; its links, consistency,
snapshot/declarations and diff checks passed before the incoming lock edits.

## Current checks and next work

- [0.22.5 CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38121988249)
  completed successfully on Linux, Apple Silicon, Intel macOS and Rust 1.88.
  #54/#55 are closed against that exact source and required native stages; this older
  run has no coordinated archive gate and does not qualify 0.22.6's private graph.
- Matching [0.22.6 CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38125908270)
  completed with successful Linux, Apple Silicon and MSRV lanes, but cancelled
  Intel optimizer qualification. Archive qualification/retention steps passed
  on all three hosts. [#56](https://github.com/dragginzgame/ic-blob-storage/issues/56)
  owns retained archive acceptance; [#57](https://github.com/dragginzgame/ic-blob-storage/issues/57)
  still lacks complete Intel graph/optimizer acceptance. Artifact presence alone
  and this older run do not qualify new dirty inputs.
- Released [0.22.7 CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38135713423)
  was queued at inspection; this is not a passed gate or qualification of dirty
  0.22.8 inputs. The committed adoption source `858c688` had no applicable
  hosted run at final inspection; the collector correction remains dirty. [#58](https://github.com/dragginzgame/ic-blob-storage/issues/58)
  retains ownership of matching temporary-path acceptance.
- [#59](https://github.com/dragginzgame/ic-blob-storage/issues/59) owns Shared
  0.6 Cargo/store/partial-evidence adoption. Focused Linux checks pass; matching
  hosted Linux, Intel macOS and Apple Silicon acceptance and artifact readback
  remain. Historical 0.5 local proof remains with its original record.
- Canic owns managed wrapper/composition under
  [#444](https://github.com/dragginzgame/canic/issues/444); guarded funding evidence
  acquisition remains Canic #494 / Blob #34. Remaining Blob #4/#5/#6/#31/#32/#33/#34
  need concrete application/provider, operational or browser acceptance. Do not
  invent a deployment, signer or paid probe, add downstream framework tests here,
  or substitute local observations for deployed provider guarantees.

[Previous review handoff](review-before-0226.md) and
[history through 0.22.4](history-through-0224.md) retain earlier source-bound results
and limits. Historical handoffs are evidence, not current command authority.
