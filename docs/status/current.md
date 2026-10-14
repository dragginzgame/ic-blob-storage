# Current status

Date: 2026-10-14

## Released source and pending batch

Blob **0.23.3** is released at `cedea53e13f04b1a271a5fb9d3220e0d3b062fb1`.
Its release-verify source is `6cf57f657da4f52803f55959061dceb848a20ba0`.
The released graph selects Host 0.17.7, Testkit 0.38.0, Memory 0.36.3 and
Metrics 0.9.1. Shared 0.11.0 remains at
`a15ff323449f4b6190f647aec0de43cbcfb314ba`, with 116 reviewed payloads.

The compatible pending **0.23.4** adopts published Host retained staging for
verified native body files. The verified file handle stays owned through
no-replace publication; failed or interrupted bodies remain named evidence.
[Qualification](../evidence/integration0234.md) records this batch.
Incoming Testkit 0.39.0 / Host 0.17.9 / Memory 0.36.4 / Metrics 0.9.3 lock changes
are preserved and identified separately from the released graph. The earlier
integration record retains its original Metrics 0.9.2 selection.

The next cleanup delegates Unix probe record creation to Host retained files,
reads attestation artifacts through the existing no-follow owner and shares one
operator-scope JSON projection across status, account, gateway and funding output.
Named partial evidence, create-new refusal, observation hashes and decimal-string
fields remain. Gateway intent fingerprints include the shared projection owner.
Host 0.17.8 and other incoming lock changes arrived before these focused checks
and are preserved. Its retained-file source uses the corrected native permission
conversion; Testkit 0.38.0's independent packaged lock remains a separate owner.
Linux CLI checks pass: 115 native and five probe tests. The initial native attempt
had three sandbox loopback refusals; the complete socket-enabled retry passed.
Logs and input hashes remain in `.tmp/review-validation/three0234/`. These tests
use local files and loopback substitutes; no public-source capture, deployed
provider request, paid effect or Canic execution was run. Native macOS and the
complete gate remain separate qualification.

Original manifest headers now reuse the Contracts projection in local and
provider byte verification, preserving their order, duplicates and separate
validation. The hook checker drops Shared-owned Make/tree-observation matrices
but retains all Cargo-member inputs and one actual-consumer failed-formatter
rollback/lock check. All 115 CLI tests, strict CLI lint, hook admission, fixture
completion, ShellCheck, formatting and links pass for this local cleanup.
Evidence and before-images are in `.tmp/review-validation/sixth0234/`.

CLI inspection now keeps preparation, one authenticated query and decoding in
the same command branch. Upload verification keeps its original body descriptor
across the query. Fixture dry runs reuse the existing query transport; update
dispatch remains separate. Upload cursors reuse canonical principal syntax and
retain concrete-tenant and complete original-scan binding checks.
All 115 CLI cases and 15 installed PocketIC cases pass, including passive balance/
gateway previews, signed status/history/byte checks and retained attestation
recovery. Strict CLI/PocketIC lint, formatting and documentation checks pass.
Inputs, fresh artifact hashes, logs and before-images remain in
`.tmp/review-validation/seventh0234/`; the incoming Metrics 0.9.3 graph was already
selected at this task's baseline and stayed unchanged. Full CI, native macOS,
Canic execution and deployed provider qualification remain outside these checks.

Published Host fs 0.17.9 now owns native input opening under
[#67](https://github.com/dragginzgame/ic-blob-storage/issues/67). Blob calls that
owner directly with the new minimum requirement; regular-file, symlink, bounded
read and held-body behavior are preserved. Host
[#88](https://github.com/dragginzgame/ic-host-tooling/issues/88) retains its separate
producer macOS test-fixture repair. Earlier frozen qualification and failed
attempts remain in `/tmp/blob-host-opener-_1fkrkee/`; concurrent real Host
validation remains excluded. Current registry qualification passes all 115 CLI
cases, strict CLI/PocketIC lint and 15 selected installed PocketIC cases with
fresh artifacts. Exact Testkit 0.39 setup/admission passes; its independent
packaged tool lock selects repaired Host 0.17.8. Logs, initial/final inputs and
artifact hashes remain in `.tmp/review-validation/issues0234/`.

The [released-source CI run](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38525659636)
is complete: Rust 1.88 passed, all three native lanes failed. Both macOS lanes
failed during Testkit 0.38 setup with its independently packaged Host fs 0.17.4
permissions defect, now repaired in the selected published tool under
[Testkit #77](https://github.com/dragginzgame/ic-testkit/issues/77). Linux passed
the formatter/release/hook fixtures but failed offline release-tool admission:
the workflow omitted Cargo-edit setup. It now explicitly runs the existing
`make install-release-tools` before checks on every native host under
[#56](https://github.com/dragginzgame/ic-blob-storage/issues/56). Direct original
job logs are retained; the aggregate failed-log response was empty. Full Blob
CI, current native macOS and hosted coordinated archive acceptance remain open.
Metrics API adoption [#65](https://github.com/dragginzgame/ic-blob-storage/issues/65)
is closed against delivered source and its owning qualification. Archive, hook,
tool-storage, common-catalog and Candid-export issues retain their required
native acceptance. Do not infer completion from the release receipt alone.

The delivered cleanup evidence retains its original inputs:
[owners](../evidence/cleanup0233.md),
[typed inputs and Shared adoption](../evidence/next-three0233.md),
[release metadata and recovery](../evidence/third-cleanup0233.md),
[client/reply construction](../evidence/fourth-cleanup0233.md), and
[permission views, intents and accounting](../evidence/fifth-cleanup0233.md).
The Node checker and Host staging consumer adoptions are now applied; do not
reapply the old combined patch.

## Canic boundary

The read-only 2026-10-14 review identifies Canic HEAD
`644d4b13012b395d6211a6907e2e6aafdf391db9` plus active 0.112 integration edits.
Released Canic 0.111.0 still selects Blob/contracts 0.22.2 and Memory 0.35;
no remote 0.112 tag was observed. Working source selects the Blob 0.23 /
Memory 0.36 family. Dedicated and embedded fixture locks still select
Blob 0.23.0 / Memory 0.36.1; their existing managed results qualify those
original artifacts, not a fresh application or current released-Blob run.

Source review confirms exact installation decoding remains in place, including
zero skipping quota. Parent-driven dynamic initialization is still controller-only
under [Canic #493](https://github.com/dragginzgame/canic/issues/493).
Guarded funding composition [Canic #494](https://github.com/dragginzgame/canic/issues/494)
still requires authenticated provider credit/payer evidence under
[Blob #34](https://github.com/dragginzgame/ic-blob-storage/issues/34).

Family delivery remains [Canic #527](https://github.com/dragginzgame/canic/issues/527);
application/provider acceptance remains [Canic #444](https://github.com/dragginzgame/canic/issues/444).
Actual two-user/verifier acceptance is Blob #31/#33; media-map transaction #4;
serving #5; provider GC/deletion/billing #32/#6. No downstream wrapper,
fabricated provider receipt or funding bypass belongs here.

[The preceding source-bound handoff](https://github.com/dragginzgame/ic-blob-storage/blob/cedea53e13f04b1a271a5fb9d3220e0d3b062fb1/docs/status/current.md)
retains original Canic decoder-repair, dedicated/embedded composition and
incident evidence. This batch changes no Canic source or lock and runs no Canic build.
