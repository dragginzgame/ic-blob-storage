# Shared Tooling 0.1.20 and Host 0.4.3 — pending Blob 0.17.2

Reviewed 2026-10-07 on released base
`7c41e3a90996157312aa40985a9861f4c03ca35e`. The maintainer requested latest
Shared Tooling/Host review and scoped repairs. Entry Cargo.lock already selects
all four Host 0.4.3 packages, testkit 0.21.1, Memory 0.31.1 and private-probe
Metrics 0.2.8. The lock and receipt remain byte-identical. Workspace version
remains 0.17.1. Only the compatible filesystem requirement advances from `0.4`
to `0.4.3`, the minimum containing the newly used API. Earlier records retain
their original Metrics/Host graphs and source bindings.

## Adoption and contract

The prior 77-file snapshot is verified before the canonical clean-private export
of committed Shared Tooling 0.1.20
`3ecc48e579f6cf6e6ab01a6645d8a250fc8c6934`. The resulting 78-file manifest adds
`rules/contributions.md`; no shared implementation is locally patched. Local
contribution/release guidance now recognizes explicit PR requests as authority
for scoped commits and topic-branch pushes. Ordinary repair authority remains
local, and the user-specified maintainer-operated one-shot release boundary in
AGENTS.md remains in force. Owner-only frontend and retention fixtures are not
copied into this consumer. The approved physical layout and 11 members remain.

CLI `Run::json` now serializes directly into Host's typed durable producer instead
of materializing the whole encoded JSON Vec. It retains CreateNew, 0600, exact
pretty bytes without an added newline and the existing `file` error code.
`record_path` centralizes the existing removed/substituted-run admission for byte
and JSON records. Caller admission of trusted ancestors and exclusion of
concurrent directory writers is unchanged. The descriptor API is not used to
change that contract; body.part/body.bin verification/publication is unchanged.
A producer that writes a JSON prefix then fails leaves no published summary or
owned staging file and preserves the original request. Existing names, schemas,
service types, durable journals and lifecycle owners do not change. No named Rust
function/method/type is removed. The complete batch is compatible 0.17.2.

All published Rust src files match their committed publishers: Host 0.4.3
`644d49c096ae05c2e17e1b6aacf14770988c5cf6` (19 artifacts, 18 filesystem,
13 process and six tools files), and testkit 0.21.1
`a98d751fb6991c12a5d0cd8faaddabe1e879199d` (38 files). Cached archives match
the five non-yanked official sparse-index rows and declare Rust 1.88. Dirty
sibling work is excluded. Core native/Wasm normal-edge trees exclude Host,
Testkit and Metrics. CLI normal edges retain artifacts/fs; process/tools remain
harness-owned. These source, registry and execution observations are distinct.

## Focused results

Raw inputs, attempts and outputs remain under `.tmp/latest-tooling-host-0172-01/`.

| Check | Result / raw log |
| --- | --- |
| Prior verification and canonical snapshot refresh | Pass, `entry-snapshot.log`, `refresh.log` |
| Locked offline preparation | Pass, `cache-preparation.log` |
| Published source and registry archive verification | Pass, `published-source-verification.log`, `selected-archives.txt` |
| Focused record publication tests | Three pass, `json-publication-tests.log` |
| Actual `make test-native-host` | 111 CLI, 21 examples, two installation/carrier and one Metrics restoration case pass, `native-host.log` |
| Strict all-target/all-feature CLI/harness Clippy | Pass, `native-clippy.log` |
| Rust 1.88 all-target/all-feature CLI/harness checks | Pass before and after the API-floor edit, `msrv-native.log`, `msrv-manifest-floor.log` |
| Real consumer formatter/hook rollback and shared Make routing | Pass, `hook-commands.log` |
| Snapshot, declaration pins, prepared tools and local links | Pass, `tooling-checks.log`, `final-maintenance.log` |
| Workspace formatting and updated dependency floor | Pass, `final-format-pins.log` |

Rust execution uses `--offline --locked`. Native integration builds fresh
matching standalone/probe Wasms, uses the prepared PocketIC 16 server and runs
actual installation and operator-only restoration. `tested-artifacts.sha256`
binds these artifacts, CLI executables and each executed test binary.
`source-files.sha256` binds the final maintained source, including the new rule
and this record; `SHA256SUMS` binds raw evidence. Entry lock/receipt comparisons,
released changelog history and the single intentional manifest floor edit are
verified. No build/source was modified beneath active validation.

## Released identity and remaining acceptance

Committed 0.17.1 receipt file hashes match HEAD, its sole-parent source is
`cdc5b412f1063a68352d4bdf7814acfb465a0527`, and the remote annotated tag peels
to the released base. Official registry metadata contains non-yanked 0.17.1,
checksum `35113b1fb312a67d79255e48ef64fc58611a79845bd7344e2dffef122fd1693a`,
published 2026-10-07T14:50:15Z with public Memory `^0.31`. Verification uses
committed release files, not the pending working lock. Its
[exact-source consumer CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/37640042157)
passes Linux and ARM macOS; Intel remains running at final inspection.
[#25](https://github.com/dragginzgame/ic-blob-storage/issues/25) and
[#26](https://github.com/dragginzgame/ic-blob-storage/issues/26) remain open for
that matching released acceptance. It selects Host 0.4.2, not this pending graph.

**Host 0.4.3 is not portable-release ready.** Its
[exact-source run](https://github.com/dragginzgame/ic-host-tooling/actions/runs/37639415895)
passes Linux/MSRV and fails both macOS compiler jobs with E0308 at
`crates/ic-host-fs/src/durable/mod.rs:371`. `Mode::from_raw_mode` takes Darwin
`u16`, but `options.permissions` is `u32`. Verified published source contains the
same expression; the library fails even if the descriptor API is unused.
[Host #18](https://github.com/dragginzgame/ic-host-tooling/issues/18) owns a
portable checked conversion and compatible publication with matching native
qualification. No registry-cache patch, local engine copy or lowered check hides
this defect. Await that repair before release readiness.

Shared Tooling's
[0.1.20 run](https://github.com/dragginzgame/shared-tooling/actions/runs/37641211708)
passes Linux/lint and reports both macOS jobs failed with zero steps. Exact
check annotations report repeated runner-acquisition failure; ARM also reports
capacity constraints. Failed/full log reads return empty files, and one commit
check-runs request returns HTTP 500. Individual job/check/annotation reads succeed.
This is an infrastructure gap, not an identified portable assertion defect.
[Shared Tooling #29](https://github.com/dragginzgame/shared-tooling/issues/29)
retains its native installer/artifact-service qualification obligation. A first
Host issue creation returns a GraphQL error; an exact-title read finds no issue,
then the separate retry succeeds as #18. Inspection failures and both attempts
are retained; no workflow is silently rerun.

Open product issues #4/#5/#6/#20 retain their existing downstream/provider
acceptance owners. No new provider observation, paid effect, downstream wrapper,
production instruction claim, full gate, commit, release, deployment or sibling
file edit occurs. Library publication does not qualify the service.
