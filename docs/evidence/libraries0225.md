# Shared 0.3.8, selected libraries and coordinated archives — pending 0.22.5

Date: 2026-10-10. Released base is `037f91c8ab9ba70675ea76a90a1c7591b2bc554d`
(0.22.4), validated source `9cd56de4668f4c4b7d950dd1fc6764540088ce4d`.
The maintainer asks to continue; preserve the incoming lock edits and unchanged
released package versions/receipt. Siblings remain read-only. No agent commit,
push, release/publication, deployment, signer creation or paid effect.

## Selected inputs

Export committed Shared 0.3.8 `67285b28a98b7c4211ad32de726709d4e87edea4` from
an isolated clean canonical clone into the same 99-file roster. Exclude dirty
upstream changelog work. This selected refresh changes common documentation/rules,
without tool pins or native-matrix changes. Blob already uses the scoped same-version
extracted archive override; strengthen its existing admission/integrity boundaries.

The released 0.22.4 graph is newer than the earlier 0.3.7 adoption record:
Memory 0.35.4, Metrics 0.5.5, Testkit 0.33.1, all four Host 0.12.8 and serde_json
1.0.152. Preserve incoming Memory 0.35.5 and the subsequent Metrics 0.5.6 lock edit.
`make deps` admits the selected cache without choosing other versions. Structured
comparison against the committed lock admits only these two version/checksum
changes, with all other selections and edges unchanged. Their packaged Rust sources
compare identical to 0.35.4 / 0.5.5; Host 0.12.8 sources match earlier 0.12.7.
Testkit 0.33.1 differs from 0.33.0 and is the already released selection, not a new
update. All four Host 0.12.9 edits arrived before the full gate's input capture:
their packaged Rust trees match 0.12.8 (19/18/21/8 files), and that exact 0.12.9
graph is the one compiled throughout the complete gate and floor check. Final
structured lock comparison admits only six version/checksum changes (Memory,
Metrics and four Host packages), without other package or edge changes.
PocketIC remains 16.1.0. Public API, runtime schemas and Memory minor identity
are unchanged. Source comparisons are retained in the current packet.

Memory's upstream failed-toolchain/MSRV-read repairs do not reproduce in Blob:
its native workflow delegates selection to rustup and its MSRV recipes invoke the
explicit +1.88.0 toolchain, without that unguarded dynamic helper. No duplicate
compiler-selection helper is added.

## Matching native acceptance

[Released 0.22.3 CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38060598267)
succeeds on Linux, both macOS 15 architectures and Rust 1.88. Jobs metadata confirms
that tooling fixtures, native CLI/install/Metrics and all four optimizer variants
ran successfully; the Intel log records original/O3/Os/Oz installation/restoration.
Closed Blob #52/#53 against their exact completed source scopes. Later assertion
adoption remains #55; selector adoption remains #54. The 0.22.4 matching run is
queued at initial review; final readback shows Linux and MSRV successful, Apple
Silicon running and Intel queued. This dirty batch has no hosted result. Earlier
cancelled runs and older private graphs are not relabelled. Prior ignored local
packets were removed by maintainer cleanup; durable summaries remain, and hosted
acceptance is retained afresh in this packet.

## Coordinated archive guards and actual regressions

Before compilation, require exactly one contracts/core package at the expected
version, no registry source for either, the exact unpacked manifest paths and
only those two workspace members. Preserve existing external lock selection
admission, offline locked builds and unique targets. Hash original archive bytes
and all unpacked file bytes/modes; retain original manifests/lock and admitted
fixture manifests/lock. Recheck after native/doc tests. The verifier requires normal
completion even when Bash 3.2 reports a premature exit as status zero. It keeps
original evidence on every failure and never patches a publication manifest.

The existing real Cargo fixture still proves current same-version archives execute
the changed contract despite a warmed older target. Add actual metadata mismatches
(live-source path, registry source, wrong version, duplicate workspace member),
a valid-JSON failed Cargo read
(status 23), and post-build mutations of an archive, unpacked source, original
manifest and original lock. Metadata cases dispatch no tests; integrity cases run
both test phases then fail instead of reporting success. Independent consumers use
spaces and Unicode in their paths. Reuse the existing completion fixture for the
verifier's four premature-exit modes; it stops before real Cargo effects.

Initial `package-fixtures.*` fails because the test expected status 1 while the
correct Perl admission refusal propagates 255. Retain the rejected payload and
attempt; repair the test to require nonzero, the actual admission diagnostic and
no compiler dispatch. Separate final and portable-path attempts pass. No failure
is overwritten. The portable JSON/filesystem path comparison uses UTF-8 bytes,
matching Cargo's decoded metadata with the actual filesystem path.

A controlled actual-Cargo consumer supplies the wrong live-source manifest
observation to the released verifier: it returns zero and dispatches both test
phases. The revised verifier rejects that observation before dispatch. Retain
`old-verifier-probe.log`, its status and independent consumer directory. This is
substituted metadata in a local consumer, not evidence of a deployed wrong build.

## Qualification

Packet: `target/review-validation/memory03550225/`. Complete `make ci` and
`make msrv-check` pass with 1,718 captured file hashes/modes identical before,
after the gate and after the floor check; original Cargo manifest/lock hashes
also pass. This includes 643 native and 376 enabled PocketIC cases, original/O3/
Os/Oz installation/restoration, Wasm and exact extracted package tests. Optimizer
evidence remains in `target/blob-wasm-opt.EbYlPw/`; package evidence remains in
`target/package/blob-verification.BUlARa/`.

Focused current Bash and genuine Bash 3.2.57 real Cargo regressions pass, including
all nine admission/read/drift cases and the same-version warmed target. Completion
checks pass on both shells. Preserve the earlier incorrect status assertion's
failed attempt separately. `inputs-*.json`, `cargo-before.sha256`, `ci.*`, `msrv.*`,
source/lock comparisons and portable fixture logs bind this exact qualification.
Final documentation attestation changes only four prose files and reruns its
affected checks; compiler evidence is reused for unchanged inputs.

[Blob #56](https://github.com/dragginzgame/ic-blob-storage/issues/56) tracks delivery
and matching native acceptance. Local provider substitutes cannot establish
deployed provider or Canic application behavior. No release effects occurred.

Blob #54/#55/#56 now have qualification and matching native readback comments.
Automatic approval review rejected posting the prepared Shared #109 consumer
feedback because it required explicit authorization for that destination. Retain
`issue-comments/shared109-consumer.md` locally. No indirect posting or sibling
file edit was attempted. The maintainer explicitly approved the destination on
2026-10-11; verified its owner and
[posted the exact prepared feedback](https://github.com/dragginzgame/shared-tooling/issues/109#issuecomment-6106452167).
