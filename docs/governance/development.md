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
Commit and PR requests follow the [shared contribution rules](../../rules/contributions.md):
an explicit PR request includes scoped commits and its topic-branch push;
ordinary fixes do not authorize either. Merge and direct integration pushes
retain separate authority. The maintainer-operated one-shot release boundary in
[AGENTS.md](../../AGENTS.md#delivery-and-release) remains in force here.

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
full delivery validation follows the adopted baseline; release effects retain
their separate authority. Relevant issue
work follows the shared maintenance rules.

Use targeted checks while implementing. Before delivering completed code as ready,
run the documented full suite under the adopted baseline's standing validation
authority. Inspection-only tasks do not authorize this suite. Documentation-only
work needs links, consistency and diff checks; release commands retain separate
explicit authority.
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
Provision the reviewed jq/yq, ripgrep with PCRE2, cloc and common IC executables
explicitly with `make install-tools` before validation. The reviewed
`make/tools.mk` snapshot owns setup, offline verification and LOC recipes.
Make selects `.tools/host/bin`, `.tools/ic/bin` and `.tools/rust/bin`;
the offline checkers never download missing tools. The shared aggregate runs
host, IC and Cargo setup/check stages in order, stopping on the first failure,
including under parallel Make. Blob extends `LOCAL_TOOL_INSTALL_TARGETS` and
`LOCAL_TOOL_CHECK_TARGETS` with the existing `install-testkit` and `testkit-check`
owner targets after the common set. Primitive Testkit targets do only their
named operation; CI uses the same complete aggregate. `make install-testkit`
provisions the lock-selected CLI/server; `make testkit-check` authenticates it
offline. Tool installation may compile and use the
network; it stays explicit and separate from ordinary validation.
`make test-native-host` is the focused CLI/example/PocketIC installation and
Metrics restoration-read check used by
the native host CI matrix, separate from the complete gate.
Testkit/PocketIC own client/server compatibility and managed server lifetime.
Blob has no independent crate-version/server-version equality gate. The
snapshot-owned `ci/ic-tools.tsv` selects five non-PocketIC executables through the
canonical shared installer; `make ic-tools-check` authenticates that bundle.
Testkit owns its PocketIC selection, authenticated setup and offline check under
`.tools/testkit-server`. The thin consumer adapter selects the CLI version from
Cargo.lock and delegates installation to the shared Cargo installer. PocketIC
Make callers obtain the admitted absolute server path before test dispatch and
pass it as `POCKET_IC_BIN` when no explicit binary was selected. Environment and
Make `POCKET_IC_BIN` overrides remain caller-owned byte admission; the harness
receives them unchanged, including empty values that it refuses. Tests never
silently replace a caller selection or download a missing server.
The test harness retains its separate owned server per instance and drop order.
`make msrv-check` explicitly selects Rust 1.88, checks the public libraries
separately, all native workspace targets/features and the supported Wasm libraries.
CI prepares that compiler and locked cache in its own minimum-version lane;
the development compiler still owns formatting and Clippy.
Opt-in browser/SDK targets check the exact prepared Node/npm selections and
manifest/lock declarations with `make browser-tools-check`; setup and version
ownership stay in the private browser build root under
[the dependency guide](../dependencies.md#browser-certificate-evidence).
Its offline `probe-check` verifies retained Caffeine run artifacts; it never runs
new network probes. The [probe ledger](../evidence/caffeine-probes/README.md) governs
continuous evidence recording and separates source/local/live observations.

`make documentation-links-check` selects root Markdown and all Markdown under
docs, audits and rules; it verifies supported local targets, not anchors or remote
URLs. `make shared-tooling-tests` exercises formatter prerequisite, portable digest,
evidence archive, IC installer, local
lockfile, Cargo metadata, host/Cargo-tool installation and common Make/LOC refusals
with offline substitutes. The declaration
gate opts in to shared Cargo package/dependency inheritance checks.
The upstream exporter/governance integration fixture runs in Shared Tooling;
it is omitted from Blob's selected files and gate. Actual snapshot verification,
consumer adapter checks and native adoption qualification remain here. Reusable
fixture dependencies are explicit companions checked by the canonical exporter.
Its focused `make tooling-evidence-check` checks retained-root selection,
including `formatting.*` failure logs, and
shared archive behavior without builds or network effects. The native tooling
workflow separately uploads/downloads a synthetic checksum-bound archive by its
exact returned ID; passing local checks do not qualify that transport. Ordinary
failure collection preserves scoped tool candidates and fixtures in one archive
with 30-day retention. Original inputs and failed partial archives remain if
collection fails; collection does not clear the original job failure.
`make hooks-check` supplies the consumer's real formatter inputs to the shared
adoption checker using Cargo's complete member roster and each member's Rust
sources, including packages not yet present in HEAD. It retains the additional
mutating-formatter rollback and Make-mode refusal cases.
`make release-commands-check` copies the
reviewed Makefile, explicit parse-time inputs and the direct-delivery admission
adapter into a private fixture,
then uses a substitute runner. `make release-check` includes that shared routing
check and retains the repository's metadata, publication and recovery fixtures.
The reviewed `make/release.mk` owns standard entrypoint recipes and conflicting
goals; Blob attaches direct-delivery admission to all four entrypoints. The
reviewed `make/rust-format.mk` owns root-workspace formatting and its prerequisite
check. Its `scripts/ci/run-formatting.sh` companion reports one success line or
a failing status and complete retained log. All isolated Make/hook/release fixtures
include these inputs; product
validation, metadata exports, tool setup and publication remain local.
Both Make includes carry `make/execution.mk` and its existing behavioral probe.
The probe rejects unsupported Make modes before recipes, including ignored
failures and non-executing modes. Every isolated fixture includes those inputs;
the release-command checker binds its tooling root to the disposable checkout.
The [snapshot adoption record](../evidence/shared-tooling-0181.md) qualifies the
current direct-delivery adapter and changed shared helpers. The
[fixture ownership map](../evidence/release-fixture-ownership.md) records
the shared runner matrix and the retained Blob adapter obligations.

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
After clean source/candidate admission, release preflight runs locked `make deps`,
the consumer's existing `make install-tools` and offline `make tools-check`,
then rechecks source identity/cleanliness and runs `make release-tools-check`
for the selected ShellCheck
executable, reviewed cargo-sort version and rustfmt availability before entering
full validation. Both formatting targets delegate formatter prerequisites to
the shared offline checker; setup and CI retain the same consumer pin.
Missing or unusable tools refuse with setup guidance before release-file mutation;
the prerequisite check never installs or compiles. Setup is confined to preflight;
post-validation preparation and saved-release reconciliation do not replay it.
Focused PocketIC/standalone qualifications run offline Testkit admission before
their first build; native-host qualification checks the complete prepared toolset.
These ordered recipes preserve that sequencing with parallel Make.

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
