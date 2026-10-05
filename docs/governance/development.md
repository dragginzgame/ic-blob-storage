<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Development and release governance

This document owns local commands, validation and release procedure within
[the adopted shared baseline](../../DRAGGINZGAME.md).
[The release guide](../releasing.md) describes the executable workflow.

## Authority and repository boundaries

Ordinary implementation and continuation authorize scoped local work, not
version changes, Git publication, deployment or paid provider effects.
An explicit maintainer instruction authorizes the exact named action; no
second confirmation or magic phrase is required when the target is clear.
Agents must never create or amend commits, directly or through helpers.
Consequently commit-producing one-shot release targets are human-operated.

Sibling repositories remain read-only unless explicitly named and authorized.
Preserve unrelated dirty work. Never alter source beneath an active validation
run or compete for the same build directory. This repository's Makefile owns
its local target directory; do not redirect builds into Canic.

## Development and evidence

Use targeted checks while implementing. Full CI or release validation requires
an explicit request or an explicitly authorized version/release target.
Primitive Make targets do only their named operation. The complete current
gate is make ci (also make validate and make release-verify).
That gate first runs the offline `make shared-tooling-check`, verifying the
declared files, hashes and executable modes without a sibling checkout. It then
runs `make deps` (`cargo fetch --locked`) to populate the cache
for the selected lockfile; this step may use the network and does not select new
versions. Fetch failure stops before validation or release-file mutation. Rust
compilation/tests then use `--offline --locked`. Scoped targets remain offline;
run `make deps` before them after dependency changes or cache removal.
Its offline `probe-check` verifies retained Caffeine run artifacts; it never runs
new network probes. The [probe ledger](../evidence/caffeine-probes/README.md) governs
continuous evidence recording and separates source/local/live observations.

Keep implementation, relevant success/rejection/recovery evidence and cleanup
in one coherent batch. Tests assert typed failures or observable behavior;
do not add placeholder tests or exact aggregate-count assertions.
Canister lifecycle/inter-canister tests use PocketIC. Native tests and provider
substitutes do not prove deployed provider guarantees.

Release guards may enforce versions, hashes, schemas, exact identifiers,
executable behavior and required files. Never enforce explanatory prose,
heading inventories or manually rotated development markers. The version
transaction generates the release record after validation succeeds.

## Versions and changelog

Pre-1.0 breaking public API or semantic changes use a minor version even
though removed contracts are hard-cut. Compatible fixes may use patch.
Do not version each implementation slice or bump Cargo during routine coding.

Maintain CHANGELOG.md for meaningful behavior and maintained tooling changes.
Keep the latest release or current draft notes at the top. Do not use Unreleased
or an extra notes queue. If the next major, minor or patch version is undecided,
use one undated heading without a patch number, such as `## [Draft]` or
`## [0.14]`. A chosen target uses its full undated version heading. The release
helper assigns the selected version and date during preparation; it does not
enforce heading position. Changelog presentation and undecided draft versions
must not gate deployment. Historical release notes remain immutable, including
imported undated entries at or below the current package version.

Repository-only work normally joins the next coherent release. An explicit
maintainer version/release request may choose a repository-only release.
Never claim that changing the version establishes service or provider safety.

## Publication and recovery

Release preparation requires committed clean input and full current validation.
It updates only Cargo.toml, Cargo.lock, CHANGELOG.md and docs/release.json.
Failed preparation restores these files; success leaves them for review.
The receipt binds exact release-file hashes to the validated source commit.
Release staging/commit reject unrelated changes. Push requires a clean main
branch, annotated current-version tag at HEAD and the validated source as its
direct parent. Push exactly main and that tag atomically without force.

Release and publication preserve build artifacts on success, failure and retry.
Cleanup is a separate explicit `make clean` action. Registry publication is
enabled for crates.io and remains separate from the Git release.
The maintainer removed B1 ownership/readiness as a library-publication gate.
The validation gate proves the implemented native behavior and release tooling;
service qualification still requires its actual PocketIC/provider/recovery
evidence. Publication does not establish those guarantees.
