# Publisher custody and Canic acceptance review

Date: 2026-10-14. Released Blob source:
`cedea53e13f04b1a271a5fb9d3220e0d3b062fb1`; release-verify parent:
`6cf57f657da4f52803f55959061dceb848a20ba0`. Task baseline is retained in
`/tmp/blob-integration0234-before`. The incoming lock selects Memory 0.36.4 and
Metrics 0.9.2; these edits are preserved rather than rewritten. Host 0.17.7 and
Testkit 0.38.0 remain selected. Shared 0.11.0 remains unchanged.

## Local implementation

The previously reviewed Host retained-staging extraction is now adopted from
registry source. Native body snapshots/downloads keep their verified writable
handle through publication. Host owns descriptor-relative creation, file/stage
identity checks, syncing and no-replace publication. Blob owns run-directory
custody, verification, retained partial evidence and recovery policy. A failed
publication never authorizes another provider effect or automatic retry.
The root Host fs requirement is raised to the API's first published 0.17.4;
the lock-selected 0.17.7 and all other dependency selections remain unchanged.

Only the filesystem portions of the historical consumer patch were applied.
The delivered Node checker and subsequent typed-input changes are preserved.
The new substitution regression replaces the named partial file while retaining
the verified handle; publication refuses and both evidence files remain intact.
Existing occupied-destination, directory-substitution and failed-body cases stay.
Public library/CLI fields and persisted service contracts are unchanged; the
compatible pending release is 0.23.4.

## Canic and issue review

Canic is read-only at `644d4b13012b395d6211a6907e2e6aafdf391db9` plus active
integration edits. Released 0.111 uses Blob 0.22 / Memory 0.35; working 0.112 uses
the newer family, without a remote 0.112 tag. Independent dedicated/embedded
locks retain Blob 0.23.0 / Memory 0.36.1. Existing managed results retain those
artifact identities. Installation decoding has zero skipping quota; dynamic
initialization binding remains controller-only. Canic #493/#494/#527/#444 and
Blob #31–#34/#4–#6 retain their distinct owning acceptance; the current handoff
links the actionable boundaries. This review adds no downstream fixture or API.

Metrics API issue [#65](https://github.com/dragginzgame/ic-blob-storage/issues/65)
is closed against committed 0.23.3/Metrics 0.9.1, owning native/PocketIC/floor
evidence and the matching successful Rust 1.88 CI job. The incoming 0.9.2 graph
is excluded from that closure. Other tooling issues require matching native-host
acceptance; the reviewed release run remains in progress with a failed Apple
Silicon setup job. No premature closure
of provider/application or native-host requirements occurs.

## Checks and limits

Raw logs, input hashes, installed artifact hashes and reviewed Canic source hashes
are retained in `.tmp/review-validation/integration0234/`. Linux CLI tests and
strict Clippy pass against published Host; the added artifact identity regression
passes separately. A fresh standalone Wasm uses the incoming locked Memory graph.
All 114 unique CLI cases pass across the complete suite and the added focused
artifact case. Twelve installed standalone cases pass: native download refusal,
four indexed preparation recovery cases and seven publication sessions. They
retain partial outputs and original identities without provider redispatch.
After admitting the narrower Host fs requirement, locked CLI checking and final
formatting, dependency-pin, documentation and diff checks pass. Lock/snapshot
hashes and the reviewed Canic inputs remain unchanged; initial and final manifest
hashes are retained separately. This is local/installed substitute evidence.
Full Blob CI, native macOS, current-release Canic execution and actual application/
provider acceptance remain outside this focused local result. No sibling file,
release/version/index/ref, deployment or paid provider effect is changed here.

## Native blocker discovered at batch close

The [Apple Silicon job](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38525659636/job/115725270772)
fails installing Testkit 0.38.0 before Blob tests. Its published packaged lock
selects Host fs 0.17.4; retained/mod.rs passes u32 permissions to Darwin's u16
Mode constructor. Published Host 0.17.7 retains the same call. Blob's application
lock cannot alter this separately frozen exact Cargo tool build. The failed
attempt and tooling archive remain with the hosted job; direct job-log bytes
are retained locally because aggregate logs are unavailable mid-workflow.

[Host #84](https://github.com/dragginzgame/ic-host-tooling/issues/84) already owns
the producer defect. Read-only source `ac5389f726482eecc3ac2cd1080b3921f0c7ef4a`
prepares the correct existing checked native-permissions reuse. The registry
index still ends at 0.17.7 when inspected. Blob's exact failure is added to that
issue; [Testkit #77](https://github.com/dragginzgame/ic-testkit/issues/77) tracks
its required packaged-lock adoption after producer publication. Both direct
consumer and tool graphs need the published repair before macOS qualification.
No sibling patch, cast workaround, cache replacement or platform skip is applied.

## Published opener and native CI setup repair

The following 2026-10-14 issue pass uses the incoming Testkit 0.39.0 / Host 0.17.9 /
Memory 0.36.4 / Metrics 0.9.3 graph. It preserves the prior selections and results
above as historical evidence. Task before-images are retained in
`/tmp/blob-issues0234-before-z62sb1to`; the raw packet is
`.tmp/review-validation/issues0234/`. Its saved incoming lock remains byte-identical
after qualification. Shared 0.11.0 and Blob package versions remain unchanged.

Published Host fs 0.17.9 has VCS source
`6fd078d9b914fff530b1efe521dc08ce18b97cc0` and sparse-index SHA-256
`c98b03923603d9251024e58dae0b531fbf840e6e78cd69b569e2a5fdc41d8b87`.
The root fs minimum is now 0.17.9. Native reads and LocalBody opening delegate to
its regular-file descriptor owner with existing failure mapping, intentional input
symlink following and independent length/content verification. No real-checkout
dependency override or parallel runtime path is used. Adoption remains
[#67](https://github.com/dragginzgame/ic-blob-storage/issues/67) pending delivery.

Testkit 0.39.0's published archive identifies source
`d10763417b3e3a083c9de7f5b0405be97688b21e` and sparse-index SHA-256
`d43f92845b8e5f5f311d4096f52008adc6350cf0025075c46e9c6e4e60332f49`.
Its independently packaged lock uses repaired Host 0.17.8, distinct from this
application's 0.17.9 selection. Explicit `make deps` and `make install-testkit`
pass, followed by offline Testkit/server and release-tool admission. All 115 CLI
cases and strict CLI/PocketIC lint pass. Fresh Wasm/CLI artifacts pass 15 installed
cases: balance/gateway preview and sync, signed status/history, upload verification
and attestation recovery. Input and artifact hashes are retained with the logs.
These are Linux/local-substitute results, not deployed provider or Canic evidence.

Direct logs of the completed 0.23.3 run show that both macOS lanes failed during
old Testkit setup. Linux passed its formatter/release/hook fixtures but failed
because offline `release-tools-check` found no prepared Cargo-edit. The workflow
now invokes the existing explicit `make install-release-tools` before checks on
all native hosts; no installer or offline-check semantics change.
[#56](https://github.com/dragginzgame/ic-blob-storage/issues/56) retains coordinated
archive acceptance. The empty aggregate log response and direct original job logs
are preserved. Full Blob CI, native macOS and hosted archive acceptance are not
established by this pass. No Git, release, sibling or provider effect occurred.
