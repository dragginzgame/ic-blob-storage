# PocketIC and minimum compiler repair — pending 0.18.5

## Inputs and ownership

Review base is Blob `afa8844cb0afb615e187982a13d1e9d7a64d3755`, compiled package
version 0.18.4. The maintainer's existing lock edit selects Testkit 0.25.1;
it is preserved. The initial graph selected Host 0.8.1. During validation the maintainer committed
Host 0.8.2/Testkit 0.25.1 as `9ab187a1ab784da14e1caf4aa0df9eea232d1635`.
The final graph selects Host 0.8.2, Memory 0.31.6, private
Metrics 0.2.14 and PocketIC client 16.1.0. No public service, durable or wire
contract, version or release receipt changes.

Original `make pocketic-alignment-check` exits 2 because its shared equality
helper rejects locked client 16.1.0 against server pin 16.0.0. The original log
is `.tmp/issues0185-03/original-alignment.log`; failed metadata evidence remains
at `/tmp/pocketic-alignment.CFlyWi`. Removal of this downstream equality policy
is not a claim that every server is compatible. Testkit/PocketIC retain startup,
protocol admission and lifetime ownership; no replacement consumer parser is added.
The explicit managed-server harness and its buffered final-output helper remain.

The three 16.1.0 archive hashes in the now consumer-owned `ci/ic-tools.tsv`
match [official release metadata](https://github.com/dfinity/pocketic/releases/tag/16.1.0).
All other IC tool rows are unchanged. Shared Tooling owns the installer,
exact artifact/executable admission and bundle receipts. The previous bundle
is retained. This is the documented matrix exception, not an edit to vendored pins.
[Blob #36](https://github.com/dragginzgame/ic-blob-storage/issues/36) owns native
consumer acceptance; [Testkit #34](https://github.com/dragginzgame/ic-testkit/issues/34)
and [Shared #76](https://github.com/dragginzgame/shared-tooling/issues/76) retain
their canonical compatibility/default selection follow-ups.

Shared Tooling adopts 94 files at committed
`1872ed2c20f6c70689bb2249050b1d673c60bfa0` through a clean isolated clone.
Dirty sibling source is excluded. The selection removes the consumer matrix,
the unused alignment/binary helpers and their fixture; adds the 14 task catalog
companions; and preserves installer/archive fixture dependencies. Task adoption
does not enable any recurring job. The owner real-Git tracking fixture is not
selected: consumer release tests are simulation-only.

## Minimum compiler contract

`make msrv-check` explicitly invokes Cargo/rustc 1.88.0, with auto-install disabled.
It checks the public contracts and service libraries separately, the full native
member roster with all targets/features, the contracts Wasm path separately and
the supported workspace Wasm libraries. The independent CI lane explicitly
prepares this compiler/target and the locked cache before offline checks.
Development formatting and linting retain the selected 1.99 toolchain.
[Blob #35](https://github.com/dragginzgame/ic-blob-storage/issues/35) tracks this gate.

## Qualification

Raw attempts, official asset metadata, original snapshot and source inventory
are retained in `.tmp/issues0185-03/`. All 842 final build/selection input hashes
remain unchanged across final validation. The initial validation and the failed
attempt to bind it to the original 0.8.1 lock remain separate; the lock advanced
externally. Final checks bind to committed `9ab187a` plus this working tree:

- Explicit offline dependency preparation and complete Linux IC bundle
  installation/check pass; the actual server is PocketIC 16.1.0. Earlier tool
  sets remain retained. The three new digests match official release metadata.
- `make test-native-host` passes 241 cases: 114 CLI/probe, 103 contracts,
  21 examples, two real PocketIC installation/Candid carrier and one real
  PocketIC Metrics restoration case. It also checks the runtime-free normal
  graph and builds the actual standalone/storage probe Wasm inputs.
- `make msrv-check` passes on Cargo 1.88.0 (`873a06493`) and rustc 1.88.0
  (`6b00bc388`): separate public native libraries, all native workspace
  targets/features, separate contracts Wasm and all supported workspace Wasm
  libraries. Final checks use the unchanged Host 0.8.2 lock.
- Shared 94-file integrity, IC binary receipts, dependency declarations,
  host/IC installer fixtures, snapshot distribution and simulation-only release
  runner fixtures pass. ShellCheck for changed scripts, actionlint, local
  documentation links and diff whitespace checks pass.

`final-native-host.log`, `final-msrv.log`, `qualified-inputs.json`, installer/
fixture logs and the official release JSON preserve the exact inputs/results.
This source has no matching hosted native or MSRV run yet; native macOS and
the newly configured CI lane remain pending. The first GitHub `--log` read for
released CI returned empty output, retained as unavailable detailed-log evidence;
closure relies on the exact-source workflow/job/step readback and published
package identities, not an invented log transcript.
The earlier Host 0.7.1/Testkit 0.24 evidence retains its original source/graph.
No full CI/release gate, agent commit/push, release/publication, sibling edit,
workflow dispatch or live/paid provider effect is part of this repair.

## Issue disposition

[#27](https://github.com/dragginzgame/ic-blob-storage/issues/27) and
[#28](https://github.com/dragginzgame/ic-blob-storage/issues/28) are closed:
released 0.18.4 CI `37776751855` now passes Linux, Intel macOS and Apple Silicon.
Both published crates are non-yanked; checksum-verified official archives record
VCS `ab15c39d208608d2b43a555fc417dd85a6759942`. Contracts' normal dependencies
exclude service/CDK/Memory and the service selects contracts 0.18.4. This resolves
the previous registry/native acceptance gaps; downstream adapter adoption stays
with Canic #444. Released CI does not qualify the new incoming graph.

#35/#36 remain open for committed minimum/native acceptance. #29's archive
round trip remains open for matching hosted evidence. The new
[completion-verifier recipe](../completion-verifier.md) supplies #33's operating
reference using existing commands; two-user/deployed-provider acceptance remains
open. #31 needs application-authenticated delegation and a separately reviewed
contract change; #32/#34 need authoritative provider callback/funding evidence.
#4/#20 retain downstream application/managed acceptance, and #5/#6 retain provider
serving, deletion, billing and lifetime evidence. No substitute evidence closes
those external obligations and no sibling implementation is changed here.

## Removed tooling

Removed scripts: `check-pocketic-alignment.sh`, `check-pocketic-binary.sh` and
`test-pocketic-checks.sh`. Their shell functions were `usage` (alignment helper)
and `expect_failure` (fixture). Removed Make target: `pocketic-alignment-check`.
No Rust function, method or type is removed. Authentication of the managed
bundle remains with `ic-tools-check`; the removed standalone checker had no
remaining consumer beyond the removed gate/fixture.
