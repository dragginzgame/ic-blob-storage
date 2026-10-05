<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Shared Tooling adoption evidence

Date: 2026-10-05. This records a completed source review and local tooling
adoption, not an issue tracker. GitHub issues in their owning repositories track
future work. Historical release notes retain their original meaning.

## Post-release snapshot integrity — 2026-10-05

After release 0.14.10, documentation merge `baee7a1` added consumer banners to
the five declared principle guides, `docs/consuming-snapshots.md` and
`docs/supported-hosts.md`. The offline snapshot check refused their changed
bytes. The pinned manifest itself remained unchanged.

The original guide bytes, exact banner diff and refusal are retained under
`.tmp/restoration-histories-01`. The canonical `refresh-consumer.sh` helper from
clean Shared Tooling `41e1fd0ba41460bd2127cbf98ac8a4b2b2020d3e` restores the
existing eleven-file snapshot. Only the seven guide contents change; manifest,
source revision, file set, executable modes and tooling stay pinned. The offline
guard then verifies all eleven files. Repository-owned documentation keeps its
banners. Keep local branding outside shared snapshot files; do not bless drift by
changing declared hashes. The
[supporting summary](caffeine-probes/local/2026-10-05-restoration-histories-01/summary.json)
binds the refusal, preserved bytes, refresh and success separately from local
restoration measurements.

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

## 0.14.12 release and formatting adoption

The maintainer reports 0.14.11 live. Entry source is its release commit
`b526ca5bcc2e7976281c60880f83a12bd216c12d`, following
`08783a2d537bf25de02da2faabddc6bd2c1eebee`; annotated `v0.14.11` is
`d29bebd79776a81da9ca5f53283164c128de1a52`. This batch creates the compatible
0.14.12 tooling draft without bumping Cargo or changing the released receipt.
No published Rust API, blob CLI, service DTO or persisted layout changes.

The reviewed Shared Tooling source is committed
`f52c0e2476aee094359ed21de91c468540d3969f`, separately observed on remote main
before this batch. Its top changelog labels 0.1.3 without a release date; this is
source identity, not a claim of upstream tag, publication or passing CI. The
sibling has subsequent uncommitted maintenance work. A clean isolated clone
selects the reviewed commit, retains its provenance and exports only committed
bytes through the canonical refresh helper. The consumer explicitly expands its
manifest and finally selects 22 files. The original eleven-file manifest/source
archive remain under `.tmp/shared-tooling-v01412-01`.

The common runner is the single owner of release intent, staging, commit/tag and
push. Consumer Make entry points delegate patch/minor/major and explicit resume
without adding another increment or state machine. The local adapter owns the
four release files, locked workspace version edits, formatting and source-bound
receipt; failed preparation restores originals and retains its inputs. It checks
an exact release index and completed parent/receipt/tag before separate registry
publication, which shares the runner's release lock. The former manual Git
phase/exact-version targets are retired;
operators use the standard release kind and saved recovery. Preview and completed
release inspection remain read-only. Agents never run the real one-shot or
resume commands, including in this batch.

Formatting uses exactly cargo-sort 2.1.4 and the selected rustfmt. Every child
manifest still inherits root dependencies. Twelve manifests are sorted; canonical
metadata comparison preserves all eleven members/default-member sets, package
versions, dependency/feature selections and binary targets. A transaction against
real workspace metadata updates eleven versions in an isolated copy and retains
every external lock byte. The current Cargo.lock and docs/release.json remain
byte-identical to entry. No dependency fetching or Rust build is performed.

`make install-hooks` activates `.githooks` only in this clone, after inspection
found no conflicting effective setting or executable private hooks. Other clones
perform setup explicitly. The consumer regression uses the actual Make formatter,
keeps its original lock and proves selected formatting/idempotence, unrelated and
untracked edit preservation, partial staging refusal, failed formatter isolation,
physical path alias setup and refusal to replace another hook location. The
upstream suite passes separately in its clean shell-only source tree. These tests
reuse existing Git objects or command stubs, never create fixture commits.

Initial observations remain retained rather than relabelled:

- ShellCheck reports cleanup of a temporary list while it is being read. Cleanup
  is moved after traversal and the corrected check passes; the first log remains.
- Running the upstream hook suite directly against this Rust consumer reaches an
  inherited Cargo.lock and fails its shell-only fixture's no-lock assertion. Both
  the first failure and diagnostic trace remain. The unmodified suite passes in
  its own frozen source; consumer CI instead uses its own real-lock fixture.
- Cargo-sort initially interleaves target dependency tables with `[[bin]]` arrays,
  leaving two manifests unstable. Original sort/check attempts and printer output
  remain. Grouping the existing binary tables together, without changing their
  declarations, lets the canonical formatter and repeated checks pass. No second
  dependency sorter is introduced.
- The first metadata comparator treated workspace traversal order as semantic.
  Its raw/canonical reports remain; sorting the unchanged member/default-member
  sets yields identical complete package/dependency/feature/target metadata.
- An isolated-lock command was initially requested before its working directory
  existed and no process started. The directory was created and the actual
  eleven-member transaction then passed; repository metadata was never mutated.
- The first publication-lock fixture exposed an EXIT trap referring to a local
  variable after the function returned. Its log (`adapter-check-04.log`) and
  failed fixture (`target/release-tests.6cKx7m`) remain. Keeping the lock path in
  adapter scope fixes cleanup; all twenty cases then pass in
  `adapter-check-05.log`.

Focused Linux checks pass: common-runner command stubs, twenty local adapter
cases (all release kinds, bootstrap/notes refusals, validation and metadata retry,
exact staging, saved receipt date, separate publication/locking and lost push
replies), upstream and consumer
hook suites, strict shell checks, manifest/Rust formatting, snapshot integrity,
actionlint, metadata/lock preservation and local instruction links/diff checks.
The [tooling workflow](../../.github/workflows/tooling.yml) declares Ubuntu 24.04,
macOS 15 Apple Silicon and macOS 15 Intel with explicit tool preparation and
independent non-mutating gates. Adding that matrix does not establish a native
macOS pass; matching runs remain pending. Full CI, full Rust/MSRV gates, live
publication, provider/service qualification and deployments are not run.

The GitHub description is still empty at the scoped readback; the earlier
proposed description above remains suitable. No GitHub write, issue, upstream
message or sibling edit is made. This evidence records adoption and observations,
not another issue tracker. Consumer build/profile artifacts are preserved.

The [summary](shared-tooling-v01412.json) binds the entry archive, exact snapshot,
source comparisons, tested patch and retained logs. The release receipt keeps its
0.14.11 identity; no new receipt, commit, tag, push or publication is produced.

Removed local functions are `tag_absent`, `lock_release`, `bump`, `stage`,
`commit_release`, `remote_preflight` and `push_release` in
`scripts/release/release.sh` (canonical runner replaces their workflow); the
Perl `next_version` in `scripts/release/release-data.pl` (canonical version helper
owns arithmetic); and `test_staging` / `test_initial_version` in
`scripts/release/test-release.sh` (exact-index and common-runner increment cases
replace superseded manual/initial release fixtures). No Rust functions, methods
or types are removed. Other retained test functions are changed, not renamed.
