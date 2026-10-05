# Shared Tooling adoption evidence

Date: 2026-10-05. This records a completed source review and local tooling
adoption, not an issue tracker. GitHub issues in their owning repositories track
future work. Historical release notes retain their original meaning.

## Source identities

The consumer starts at release 0.14.9,
`c3a16529d161547987c138b15bf75cac70b54c55`, with validated source
`760568efd3ad42b0fb5d39ebb554aec306fb883f`. Its local annotated tag, Cargo
version and receipt agree; the maintainer reports it pushed. This review does
not establish crates.io publication or service deployment.

The adopted Shared Tooling source is committed revision
`e16c9c99bd800567189c8024eaf4242a5d1c9e29`, selected while the checkout was clean. The
[snapshot manifest](../../.shared-tooling.snapshot) binds its remote, revision,
eleven declared files, SHA-256 hashes and executable modes. The source-side
refresh exports committed Git blobs; this consumer's builds and checks do not
read the sibling checkout. Shared Tooling's own AGENTS.md was not copied.
At delivery the sibling has subsequent uncommitted baseline/verifier/test changes;
those bytes are not part of this snapshot and were not changed by this task.

The earlier 2026-10-04 review used base
`956236a3848c2cfae6ae05f5c77e9c37b01b3366` and dirty upstream AGENTS hashes
`7d9a581429414d05711c4bb647dfc9a26557bfe09e515b821cec501babb6fb1e` and
`f3d549197a3f8bde1140b38a04a10412e20a117ddef54e9410f060cbad8aef87`.
They were working-file observations, not bytes of that committed revision.
Its [retained review](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/governance/shared-tooling-feedback.md)
records funding-encoder propagation, exact media artifact identity and offline
cache evidence. The local feedback queue and
[old integration queue](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md)
are removed from the maintained tree rather than retained as alternate trackers.

## Maintained ownership

[DRAGGINZGAME.md](../../DRAGGINZGAME.md) owns common engineering practice and
links its vendored guides. [AGENTS.md](../../AGENTS.md) is the local overlay for
service ownership, frozen-contract recovery and exact commands. No new exception
to common rules is introduced. Provider probe records still own experimental
requests, effects and limitations; they do not become issue trackers.

The LOC tool discovers Cargo workspace members. Make supplies an explicit
repository root and `CARGO_NET_OFFLINE=true`, leaving the shared script unchanged.
The report classifies Rust files by path; runtime-file LOC can include inline
tests and test-canister code. Test-attribute totals are lexical counts, not a
count of executed test cases. This report makes no runtime or binary-size claim.

## Focused verification

Commands and outputs are retained under `.tmp/shared-tooling-adoption-01`:

- `make shared-tooling-check`: all eleven declared files verify offline.
- `make cloc`: all eleven workspace members occur once, including canister and
  tests/ members. Report totals agree with individual rows; Cargo.lock remains
  unchanged. Discovery/counting performs no compilation or dependency fetching.
- Shell syntax, ShellCheck and documentation-link checks cover the adopted
  scripts and local instruction links. A deliberately altered isolated snapshot
  is rejected; the real snapshot remains unchanged.

These are local Linux observations. macOS support is required by the baseline,
but native macOS builds, operator workflows and platform prerequisites are not
qualified by this run. Full CI, Rust builds, paid/provider calls, deployment,
commits and sibling edits were not performed. Retained effect records and build
artifacts were not cleared.

## Shared Tooling 0.1.0 refresh

Date: 2026-10-05. Consumer base is
`8e3704bfee6edbf68c30d467c2c552951521ebbc`; the only pre-existing dirty file is
Cargo.lock, changing powerfmt 0.2.0 to 0.2.1. Its SHA-256 before and after this
work is `7600d6d2ce134a303d6dc056f4ebfa19739cabe0448e4c4dfd377353d84ce06e`.
That dependency update is preserved, not qualified by tooling validation.

The clean `/home/adam/projects/shared-tooling` checkout is at
`41e1fd0ba41460bd2127cbf98ac8a4b2b2020d3e`, matching the separately observed
GitHub main revision. The upstream changelog labels 0.1.0 on 2026-10-05; this
does not establish a tag or a passing upstream CI run. The refresh preserves the
same eleven declared paths and exports their committed Git blobs and modes.
Five declared files change: the baseline, reviewable-changes guide,
consuming-snapshots guide, snapshot verifier and LOC script.

The source-side refresh initially refuses the differing source URL spelling.
A transient Git config override does not replace the checkout's first remote
URL, so that attempt stops before replacing consumer files. The reviewed manifest
provenance is then changed from `git@github.com:dragginzgame/shared-tooling.git`
to `https://github.com/dragginzgame/shared-tooling`, identifying the same upstream
repository and matching the clean checkout. The existing manifest is refreshed
with its unchanged file set. The original manifest and refusal log remain intact;
no sibling configuration or source is edited.

The adopted verifier avoids empty-array expansion under Bash 3.2 nounset while
retaining duplicate-record rejection. The LOC tool uses one explicit file set
per package for line and test-attribute counts, excludes nested members from a
parent's set and classifies paths relative to each package. Its existing
`count_test_fns` function is modified, not removed or renamed. No function,
method or type is deleted by this adoption. The baseline also adds GitHub
description review and exact cleanup-symbol reporting. Local authority, service
contracts, build ownership, package version and release receipt remain unchanged.

The fresh Linux host lacks cloc, jq and the cloc Perl modules. Prerequisites are
downloaded over HTTPS with exact versions and SHA-256 digests from local Ubuntu
package metadata, verified before extraction under the consumer's temporary
directory. The initial cloc version check refuses a missing Parallel::ForkManager
module; its prerequisite set is then completed. cloc reports 1.98, jq reports
jq-1.7, Bash reports 5.2.21 and ShellCheck reports 0.11.0. Extracted Perl module
and shared-library paths are explicit for the checks; no system installation,
Rust dependency fetch or new workspace dependency selection occurs.

Focused commands and results are retained under `.tmp/shared-tooling-refresh-01`:

- `bash /home/adam/projects/shared-tooling/scripts/distribution/refresh-consumer.sh
  --consumer /home/adam/projects/ic-blob-storage`: exports and verifies eleven
  files at the selected revision.
- `make shared-tooling-check`: all eleven files and modes agree.
- `bash /home/adam/projects/shared-tooling/scripts/ci/test-snapshot-distribution.sh`:
  passes local fixtures for committed-byte export, concurrent source drift,
  dirty-source refusal, duplicate/mismatched/mode refusal and custom manifest
  parent creation/refusal. Fixture Git responses are substitutes, not upstream
  deployment observations.
- `CARGO_NET_OFFLINE=true bash
  /home/adam/projects/shared-tooling/scripts/ci/test-cloc.sh`: passes actual
  disposable Cargo workspace fixtures, nested members and relocation under
  ancestor names containing `tests`, spaces and glob characters.
- `make cloc`, with the extracted tools, PERL5LIB and LD_LIBRARY_PATH selected:
  reports all eleven actual members exactly once. Compared with the pre-refresh
  script on the same files/tools, 1,533 protocol source lines and 1,688 PocketIC
  harness source lines change classification. Total LOC remains 115,370 and
  test attributes remain 1,004; runtime-named LOC is 50,011, test-named LOC is
  65,359 and inline test attributes are 100. These count paths and lexical
  attributes, not production code size or executed tests.
- ShellCheck and `bash -n` cover the three vendored scripts; changelog selection,
  member coverage/report totals, local instruction links and `git diff --check`
  pass. Hash readback confirms Cargo.toml, Cargo.lock and docs/release.json are
  unchanged from task entry.

The retained source tar archive SHA-256 is
`313ea1a8fbd952c65dfc90854e2cc1940884ab4cbaf8bc99502e33c87b7f2293`.
The before/after manifests, reports, package hashes, selected tool environment,
commands and failed refresh/version-check observations stay with this record's
temporary evidence directory. The original adoption record above is not relabeled.
These scoped Linux checks perform no Rust build, full CI, release/version
transaction, commit, publication, provider probe or cleanup of existing evidence.
Native macOS behavior and the unrelated lockfile change remain unqualified.

A read of `https://api.github.com/repos/dragginzgame/ic-blob-storage` returns
`description: null`; the response is retained as `github-repository.json`.
README and the maintained service contract support this proposed description:

> Prototype blob storage for Internet Computer apps: direct Caffeine uploads,
> verification, references, quotas and accounting.

The remote description is not changed: this continuation authorizes local
snapshot adoption, not remote metadata writes or issue submission. This is
supporting adoption evidence, not an alternate issue tracker.
