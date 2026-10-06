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

Agent requests such as `check CI`, `check issues` and `check for work` follow the
[user-triggered maintenance rules](../../rules/agent-maintenance.md). Inspect this
repository's `Release and formatting tooling` workflow at the selected source; dirty changes have no
remote result. Session instructions can enable inspections after each completed
batch. Explicit repair requests authorize scoped fixes and focused checks;
full gates and GitHub writes retain their separate authority.

Use targeted checks while implementing. Full CI or release validation requires
an explicit request or an explicitly authorized version/release target.
Primitive Make targets do only their named operation. The complete current
gate is make ci (also make validate and make release-verify).
That gate first runs the offline `make shared-tooling-check`, verifying the
declared files, hashes and executable modes without a sibling checkout. It then
runs offline `make tools-check`, `make dependency-pins-check`,
`make documentation-links-check` and `make shared-tooling-tests`, then
`make deps` (`cargo fetch --locked`) to populate the cache
for the selected lockfile; this step may use the network and does not select new
versions. Fetch failure stops before validation or release-file mutation. Rust
compilation/tests then use `--offline --locked`. Scoped targets remain offline;
run `make deps` before them after dependency changes or cache removal.
Provision the reviewed jq/yq and common IC executables explicitly with
`make install-tools` before validation. Make selects `.tools/host/bin` and
`.tools/ic/bin`; the offline checkers never download missing tools.
`make test-native-host` is the focused CLI/PocketIC installation check used by
the native host CI matrix, separate from the complete gate.
Its offline `probe-check` verifies retained Caffeine run artifacts; it never runs
new network probes. The [probe ledger](../evidence/caffeine-probes/README.md) governs
continuous evidence recording and separates source/local/live observations.

`make documentation-links-check` selects root Markdown and all Markdown under
docs, audits and rules; it verifies supported local targets, not anchors or remote
URLs. `make shared-tooling-tests` exercises portable digest, IC installer, local
lockfile and Cargo metadata refusals with offline substitutes. The declaration
gate opts in to shared Cargo package/dependency inheritance checks.
`make hooks-check` supplies the consumer's real formatter inputs to the shared
adoption checker and retains the additional mutating-formatter rollback case.
`make release-commands-check` copies the
reviewed Makefile and its explicit parse-time inputs into a private fixture,
then uses a substitute runner. `make release-check` includes that shared routing
check and retains the repository's metadata, publication and recovery fixtures.

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
Follow [the shared changelog rules](../../rules/changelogs.md). Keep one numbered,
undated next release at the top, automatically selected from the latest finalized
release and the complete pending batch. Compatible pre-1.0 work uses the next
patch; a breaking public contract requires the next minor. This documentation
choice changes no package metadata or release authority. Presentation must not
gate deployment; preparation checks exact release identity and finalizes its
date. Historical entries, including imported undated releases, remain immutable.

Repository-only work normally joins the next coherent release. An explicit
maintainer version/release request may choose a repository-only release.
Never claim that changing the version establishes service or provider safety.

## Publication and recovery

Release preparation requires committed clean input and full current validation.
It updates only Cargo.toml, Cargo.lock, CHANGELOG.md and docs/release.json.
Failed metadata preparation restores these files and retains its failed inputs.
The receipt binds exact release-file hashes to the validated source commit.
Release staging/commit reject unrelated changes. Push requires a clean selected
branch, the exact release commit with its validated source as sole parent, and
its matching annotated tag. Recovery can select an older release on HEAD's
descendant history; late adapters verify committed files via `RELEASE_COMMIT`.
The [common runner](../releases.md) owns staging, commit/tag and
the exact atomic branch/tag push, with implicit tag following disabled and no
force. One-shot releases default to main/origin; branch and remote are explicit
saved inputs. Before preparation, a failed gate starts fresh through the same
target. Uncommitted preparation stays bound to the saved source and increment.
Once committed, a normal target reconciles saved intent at its original version;
newer fixes or a different requested increment then get fresh gates for the next
version. Explicit resume finishes only its selected release. An uncertain push
requires remote readback. The local
adapter owns workspace metadata and the receipt, not another release state
machine. Formatter prerequisites are prepared before validation; `fmt-check`
and `hooks-check` run independently in the complete gate.
Release preflight invokes `make release-tools-check` for the selected ShellCheck
executable and reviewed cargo-sort version before entering full validation.
Missing or unusable tools refuse with setup guidance before release-file mutation;
the prerequisite check never installs or compiles.

`release-verify` runs that same complete `ci` gate through the reviewed validation
logger. Actual failed commands retain unique raw logs under Git's
`release-state/validation-failures`, and successful retries preserve them. If the
retention destination fails, the logger reports and preserves its temporary logs.

Release and publication preserve build artifacts on success, failure and retry.
Cleanup is a separate explicit `make clean` action. Registry publication is
enabled for crates.io and remains separate from the Git release.
The maintainer removed B1 ownership/readiness as a library-publication gate.
The validation gate proves the implemented native behavior and release tooling;
service qualification still requires its actual PocketIC/provider/recovery
evidence. Publication does not establish those guarantees.
