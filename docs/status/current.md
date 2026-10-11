# Current status

Date: 2026-10-11

## Released source and current batch

Released HEAD is `37cb8b9724de9bc65778d2b41e0c9db5a8d95de6` (0.22.6),
validated source `f998762cde261e1253058f2c99d36d72d67af43b`. All twelve package
versions and the finalized receipt remain 0.22.6. The actual released lock selects
Memory 0.35.7, private Metrics 0.6.1, Testkit 0.33.3 and all four Host 0.13.0;
PocketIC remains 16.1.0 and the Rust floor remains 1.88. The previous agent packet
used an earlier graph; do not relabel [its evidence](../evidence/shared03110226.md).

Pending compatible 0.22.7 now adopts committed Shared 0.5.0 at
`a306180ce534227fcb98b882fba9a526d5d747e0`, adding the two required store
companions for 101 files with unchanged pins. Host/IC bundles use the canonical
per-user store while `.tools/host/bin` and `.tools/ic/bin` remain local exact
selections. Explicit setup is required; old physical local bundles are retained
but no longer admitted as active selections. Cargo slots remain local under
`.tools/rust`; Testkit retains PocketIC ownership. No independent Blob store,
manual payload relocation or mutable sibling PATH is introduced.

Blob's collector now delegates shared-reference admission to the canonical
evidence selector, retaining this checkout's physical bundles and full local
Rust evidence while excluding unrelated store entries and preserving unknown
links unfollowed. The fixture uses an isolated store. The `.tools` tree was
absent before this turn's setup; prior local preservation cannot be measured.
The first setup reused host tools but correctly refused a held IC store lock;
its log remains, and no lock was reclaimed. Once it cleared, ordered complete
setup and offline tool admission passed. Store/collector and premature-exit
fixtures pass on Bash 5.2 and genuine Bash 3.2; actual hooks and formatting pass,
including the Bash 3.2 hook. A disposable second checkout reuses the same real
host/IC payloads with zero download calls and no local copies. See
[the current record](../evidence/shared0500227.md).

[Preceding 0.4 installation/guidance evidence](../evidence/shared0400227.md)
remains bound to its original sources and attempts. Contributor-only adoption
remains a compatible Blob patch. Incoming lock edits select all four Host
0.13.1, Metrics 0.6.2 and Testkit 0.33.4; preserve them without attributing old
compiler results to the new graph. No full CI/release or product compiler suite
was run for these scoped adoptions. The maintainer staged the batch during
qualification; preserve that index. Final collector and evidence updates may
remain unstaged.

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
- [#58](https://github.com/dragginzgame/ic-blob-storage/issues/58) owns the current
  consumer temporary-path fix; dirty 0.22.7 has no hosted acceptance.
- [#59](https://github.com/dragginzgame/ic-blob-storage/issues/59) owns Shared
  installation/PATH/storage adoption. The snapshot now records 0.5.0 and the
  implemented shared host/IC store. Local focused qualification passes; matching
  hosted Linux and both macOS acceptance remain.
- Canic owns managed wrapper/composition under
  [#444](https://github.com/dragginzgame/canic/issues/444); guarded funding evidence
  acquisition remains Canic #494 / Blob #34. Remaining Blob #4/#5/#6/#31/#32/#33/#34
  need concrete application/provider, operational or browser acceptance. Do not
  invent a deployment, signer or paid probe, add downstream framework tests here,
  or substitute local observations for deployed provider guarantees.

[Previous review handoff](review-before-0226.md) and
[history through 0.22.4](history-through-0224.md) retain earlier source-bound results
and limits. Historical handoffs are evidence, not current command authority.
