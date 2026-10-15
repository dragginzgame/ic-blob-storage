# Host directory custody, IC Build archives and Shared fixture retirement

Date: 2026-10-15. Base HEAD remains released 0.27.1
`c9aea8b52d471a1b17b158e21a2e71addafdec25`, plus the preserved incoming
Memory/Metrics/Testkit/Shared 0.27.2 draft work. This task captures its complete
starting working tree separately. The combined draft is now 0.28.0 because of
new CLI path admission; own Cargo versions remain 0.27.1.

## Implemented boundaries

The native artifact owner delegates private-directory creation and pathname/
held-descriptor admission to published Host FS 0.19.0. Blob keeps artifact names,
no-replace publication, request/effect ordering and local failures. An occupied
valid claim remains `ExistingRun`; other creation/admission errors remain `File`.
Created-but-unconfirmed directories are retained, never removed or retried.

**CLI contract change:** new run paths and their resolved physical parents must
contain no CR/LF. The Host helper refuses these before creating the run, including
an otherwise normal alias into such a physical parent. Owner-only modes, occupied
claims and substituted/removed directory checks pass. Public library types and
stored formats do not change in this cleanup. Cross-release service disposition
remains governed separately by the retirement runbook.

The opt-in native `blob-package-verify` helper selects two package identities,
the contracts-only verification patch and explicit budgets, then calls
`ic_build::cargo::packages::qualify_package_archives`. The selected published
Build 0.6.1 archive identifies `f5ee4bed73d8a93c5bfc5c3d0d31712f597a8bad`.
Normal CLI and public-library/Wasm dependency trees exclude Build. The native
helper's Host process types match the selected 0.19 family and Rust 1.91 floor.
No sibling source override enters the graph.

IC Build owns Cargo packaging, extracted graph/lock admission, source inventory,
fresh compilation and final integrity. Blob's shell wrapper only selects the
version/archive/evidence roots and propagates status; generic package branches
and their isolated Perl lock-parser matrix are retired. Original command output,
causes and partial attempts remain retained. Both ordinary failure 23 and
signal termination 143 propagate. Each command stream/file has a 64 MiB budget,
with 100,000 source entries and depth 64; build duration has no default deadline.

Shared owns the formatter-prerequisite and digest backend regression matrices.
Their duplicate consumer dispatch and unused selection roots are removed through
the canonical exporter at the same reviewed
`4f3eec42f52d4ec308bb68ab2daa3e5266b3175a` source (0.15.1): 131 payloads.
The producer's portable runner still dispatches both matrices. Blob retains
actual hook rollback, prepared-tool admission, archive checksum/retention,
Make wiring and release/recovery checks. No general product gate is removed
and no gate speed-up is claimed.

## Qualification

Before-images, exporter previews/refresh receipts, dependency graph projections,
raw logs, source/artifact hashes and failed preparation diagnostics remain under
`.tmp/review-validation/cleanup0272/`. That path retains its initial draft name.
Two generic fixture files are snapshot retirements, not deletion of their
canonical Shared implementations. No producer checkout is modified.

Focused Linux results pass:

- 118 native CLI and five probe cases, including occupied claims, control-path
  refusals, private modes and original directory-custody/publication checks.
- Strict all-target/all-feature CLI lint and actual Rust 1.91 native/Wasm
  qualification, including the optional contributor binary.
- Real same-version archive execution with a deliberately warm shared target;
  path/source/version/member admission, valid-output/nonzero-status refusal,
  signal refusal and final archive/source/manifest/lock drift detection.
- Exact coordinated archives of both real public libraries: all-target and
  Rustdoc tests and final graph/input integrity pass. Evidence is retained in
  `target/package/blob-verification.3QWUj5` and the final Make-carried
  `target/package/blob-verification.SR05aa` attempt.
- Actual `make -j2 package` repeats the consumer regression and archive flow.
  A private Cargo wrapper authenticates GNU Make 4.3 pipe jobserver descriptors
  before all 57 Cargo invocations, including those launched through Rust capture.
  It executes the original compiler/tool commands unchanged afterward.
- Fresh CLI/Wasm passes seven installed publication-session cases, three
  lost-reply/observation cases and the ten-scenario Chromium certificate journey.
  Final runtime and selected manifest/lock/snapshot hashes remain unchanged.
- Isolated 0.28.0 metadata rendering advances all twelve member versions and
  contracts requirement while preserving external lock rows and manifest mode/order.
- Local release-adapter and substitute routing fixtures, completion/failure
  retention, hook preservation, real tool/formatter admission, Make integration,
  shell, dependency declarations and snapshot integrity pass.

The first local edit script failed to locate a fixture branch before writing
any source; the corrected edit and selected-root export pass. The initial
unfiltered refresh left the snapshot unchanged. Those attempts are not adoption
proofs; selected-root preview/refresh receipts own the two retirements.

Earlier pending edits and tool catalogs are preserved. Added dependency rows
belong to optional IC Build; existing external versions/checksums remain selected.
Full local CI, native macOS, Canic/application, deployed provider acceptance,
release/publication and paid/reset effects did not run. Blob's index, HEAD and
release versions remain untouched. Consumer delivery/native acceptance stays with
[#69 — Adopt Host custody and IC Build archive qualification](https://github.com/dragginzgame/ic-blob-storage/issues/69).
