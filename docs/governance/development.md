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
remote result. Relevant issue work follows the shared maintenance rules.
Check selection and execution authority follow
[shared validation policy](../../DRAGGINZGAME.md#validation-and-evidence).
The local [release boundary](../../AGENTS.md#delivery-and-release) remains
maintainer-operated; this guide names the local commands.
Primitive Make targets do only their named operation. The complete current
gate is make ci (also make validate and make release-verify).
That gate first runs the offline `make shared-tooling-check`, verifying the
declared files, hashes and executable modes without a sibling checkout. It then
runs offline `make tools-check` and `make dependency-pins-check`, followed by
`make deps` (`cargo fetch --locked`) to populate the selected cache. Fetching may
use the network and never selects new versions. The unchanged workspace Clippy
gate runs next, before dependency boundaries, documentation/shared fixtures,
compilation, tests and packaging. The sequential validation runner stops on lint
failure under parallel Make too. Publication uses that same gate after source/tag
admission and dependency preparation, before registry or archive work. Rust
compilation/tests use `--offline --locked`. Scoped targets remain offline;
run `make deps` before them after dependency changes or cache removal.
The shared logger owns stage dispatch and prepared-input checkpoints. Blob's
`CI_INPUTS` declares the root and all thirteen member manifests, Cargo.lock,
compiler/tool pins, snapshot/Make inputs and browser selectors. Both `ci` and
`release-verify` pass them through `--input`; nested gates in this checkout keep
the same baseline. Changed, deleted or unreadable inputs stop subsequent stages
with restart guidance and retained hashes/logs. Checks do not refetch or restart
automatically and do not claim isolation from edits inside a running command.
Provision the reviewed jq/yq, ripgrep with PCRE2, cloc and common IC executables
explicitly with `make install-tools` before validation. Blob selects only
`wasm-opt` through `IC_TOOLS`; didc, ic-wasm, ICP CLI and
Quill are not prerequisites of its maintained workflows. The complete upstream
pin catalog stays intact. The optimizer runner explicitly admits `wasm-opt` too.
The reviewed
`make/consumer.mk` snapshot is Blob's single entrypoint for common setup,
offline checks, formatting, hooks and releases. Its `make/tools.mk` companion
owns setup, offline verification and LOC recipes; product tool extensions and
validation policy remain local.
Make selects `.tools/host/bin`, `.tools/ic/bin` and `.tools/rust/common/bin`;
the offline checkers never download missing tools. The reviewed snapshot owns the default common versions and digests; Blob has
no independent common-tool catalog. Explicit host, Rust and IC exceptions are
independent selections, and host overrides do not implicitly select Rust pins.
The shared aggregate runs
common host and Cargo setup/checks, then Blob's selected IC and Testkit stages,
stopping on the first failure including under parallel Make. Shared 0.24 retains
Rust/Cargo preflight before common installation; the selected IC installer owns
its platform admission before effects. Common Rust
tools use the canonical Cargo build/store owner, receipt and byte admission;
the complete set is activated atomically through `common`. Explicit
`make install-rust-tools` prepares the current layout without converting or
removing old installations. Offline checks never run setup. Shared 0.6.0 stores
verified host, IC and Cargo payloads per user while preserving checkout command
paths. Cargo identities include the prepared compiler, native environment,
package source and packaged lockfile; managed source cache admission is required
for both setup and offline checks. Tool builds use the standard environment,
independent of application compiler flags and wrappers. Refresh the installers,
build/store companions and evidence selection together, then explicitly rerun
setup and offline checks. Use one store setting for all three operations. Blob
must not add an independent store or bespoke installer, relocate retained
artifacts or select mutable sibling tools. See
[shared executable storage](../local-setup.md#shared-executable-storage).
Cargo-bearing local recipes preserve Make's jobserver descriptors through their
shell adapters. Blob extends `LOCAL_TOOL_INSTALL_TARGETS` and
`LOCAL_TOOL_CHECK_TARGETS` with `install-ic-tools`/`ic-tools-check`,
`install-testkit`/`testkit-check` and the lock-selected IC Build release editor
after the common set. `install-release-editor` explicitly prepares its published
CLI; `release-editor-check` returns the admitted absolute path offline. Primitive Testkit targets do only their
named operation; CI uses the same complete aggregate. `make install-testkit`
provisions the lock-selected CLI/server; `make testkit-check` authenticates it
offline. Both lock-selected Tooling CLIs require explicit `--features cli` during
setup and admission; this feature belongs to tool selections, not the application
library graph. Tool installation may compile and use the
network; it stays explicit and separate from ordinary validation.
`make test-native-host` is the focused CLI/example/PocketIC installation and
Metrics restoration-read check used by
the native host CI matrix, separate from the complete gate.
Each native lane also runs `make package`, including real same-version/refusal
regressions and exact coordinated archive tests. Hosted archive qualification
retains archives, unpacked sources, admitted metadata/hashes and regression
attempts on success or failure, excluding compiler caches. Artifact presence
alone does not turn a failed or cancelled host into a qualified result.
`make test-wasm-opt` reuses the maintained PocketIC installation, admission and
restoration cases for original and Binaryen O3/Os/Oz standalone/storage Wasm,
with explicit `--enable-bulk-memory-opt`. Fixtures drain installation debt with
bounded real PocketIC rounds, keeping platform rate limits enabled.
Each selected standalone artifact also passes canonical Candid type equality
and exact callable function-export names/modes through Host artifact inspection.
It runs in the complete gate and all three native host lanes. Inputs, output
hashes, source diff, lock, optimizer identity and logs stay in fresh
`target/blob-wasm-opt.*` directories on success or failure; hosted lanes upload
them separately. Ordinary Cargo Wasm output remains unoptimized. These local
canister cases do not establish deployed provider or Canic application behavior.
Testkit/PocketIC own client/server compatibility and managed server lifetime.
Blob has no independent crate-version/server-version equality gate. The
snapshot-owned `ci/ic-tools.tsv` retains the complete upstream executable
catalog. Blob selects only Binaryen
through the canonical shared installer; `make ic-tools-check` authenticates that bundle.
Testkit owns its PocketIC selection, authenticated setup and offline check under
`.tools/testkit-server`. The thin consumer adapter delegates the explicit
Cargo.lock selection and installation to the shared Cargo installer. PocketIC
Make callers obtain the admitted absolute server path before test dispatch and
pass it as `POCKET_IC_BIN` when no explicit binary was selected. Environment and
Make `POCKET_IC_BIN` overrides remain caller-owned byte admission; the harness
receives them unchanged, including empty values that it refuses. Tests never
silently replace a caller selection or download a missing server.
The test harness retains its separate owned server per instance and drop order.
`make msrv-check` explicitly selects Rust 1.91, checks the public libraries
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
URLs. `make shared-tooling-tests` exercises consumer fixture completion, evidence
archive, local lockfile, Cargo metadata and actual Testkit/Make forwarding
with offline substitutes. Dependency changes follow the
[focused adoption contract](../cargo-graph-checks.md#focused-dependency-adoption-contract):
admit each actual locked graph and policy before lint and artifact checks,
preserving separate qualification for native hosts and installed IC behavior.
The declaration
gate opts in to shared Cargo package/dependency inheritance checks. The separate
Platform boundary target uses Shared's canonical package-identity checker and
Blob's explicit exception policy; that policy is a guarded CI input. It covers
all direct dependency kinds/targets, not transitive deployment admission.
The upstream exporter/governance integration fixture runs in Shared Tooling;
it is omitted from Blob's selected files and gate. Actual snapshot verification,
consumer adapter checks and native adoption qualification remain here. Reusable
fixture dependencies are explicit companions checked by the canonical exporter.
Shared owns generic formatter-prerequisite, digest backend, host/IC/Cargo
installer, Cargo-version observer and release-runner matrices in producer CI;
Blob retains actual hook rollback, prepared-tool admission, archive checksums
and failure propagation. The exported fixture roots are retired with the
canonical snapshot exporter after checking callers.
Its focused `make tooling-evidence-check` checks retained-root selection,
including `formatting.*` failure logs, and
shared archive behavior without builds or network effects. The native tooling
workflow separately uploads/downloads a synthetic checksum-bound archive by its
exact returned ID; passing local checks do not qualify that transport. Ordinary
failure collection preserves scoped tool candidates and fixtures in one archive
with 30-day retention. Original inputs and failed partial archives remain if
collection fails; collection does not clear the original job failure.
Blob's collector delegates selection and archival to `collect-tool-evidence.sh`,
retaining this checkout's admitted referenced payloads,
prior local bundles and local Testkit evidence. Unknown links remain unfollowed;
unrelated entries in the per-user store are excluded. If store admission fails,
archive safe local selections and the collection diagnostics, upload that archive
when available, then report incomplete collection. Preserve the original failure;
an available partial archive does not establish complete collection.
`make hooks-check` delegates current Rust/Cargo input discovery to the shared
adoption checker, including untracked additions and tracked deletions across
the approved layout. It retains one actual-consumer
mutating-formatter rollback and lock-preservation check. Shared Tooling owns the
generic Make-mode and failed-tree-observation matrices for the identical hook.
`make release-commands-check` copies the
reviewed Makefile, explicit parse-time inputs and the direct-delivery policy
file/checker into a private fixture,
then uses a substitute runner. `make release-check` includes that shared routing
check and retains the repository's metadata, publication and recovery fixtures.
The reviewed `make/consumer.mk` includes the existing component owners once.
Its `make/release.mk` owns standard entrypoint recipes and conflicting
goals and delivery admission; Blob selects the same direct-only policy file for
Make and standalone metadata commands. The
reviewed `make/rust-format.mk` owns root-workspace formatting and its prerequisite
check. Blob selects `cargo sort --workspace --grouped` in both write and check
commands to preserve dependency-role groups. Prepared and selected committed
release metadata checks use the matching `--manifest-order-grouped` policy. Its `scripts/ci/run-formatting.sh` companion reports one success line or
a failing status and complete retained log. All isolated Make/hook/release fixtures
include these inputs; product
validation, metadata exports, tool setup and publication remain local.
The component includes carry `make/execution.mk` and its existing behavioral probe.
The probe rejects unsupported Make modes before recipes, including ignored
failures and non-executing modes. Every isolated fixture includes those inputs;
the release-command checker binds its tooling root to the disposable checkout.
The actual Testkit/Make routing fixture runs in `make shared-tooling-tests`.
Actual hook, release and parallel Testkit fixtures export `make/consumer.mk`
with all parsed companions; none may fall back to the real checkout's adapters.
Release and Testkit fixtures use `prepare-script-fixture.sh` for explicit roots
and their declared companion closure, then install their product-specific
substitutes. Release fixtures live outside the checkout under
`${TMPDIR:-/tmp}/blob-release-tests.*`; failed fixtures remain there and the
tooling evidence collector includes that prefix.
The [snapshot adoption record](../evidence/shared01500270.md) qualifies the
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
Assess this repository's published consumer boundary: contributor-only Shared
Tooling adoption does not inherit the upstream minor bump or require a separate
crate publication. Public/exposed dependency types, runtime behavior, product
CLI/output, supported consumer builds and stored contracts still need review.
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

## Consumer tooling qualification

Generic matrices stay with Shared Tooling; Blob qualifies its actual selection:

| Consumer obligation | Retained proof |
| --- | --- |
| Installed tools and selected catalogs | `make tools-check`, including Testkit admission |
| Dependency declarations and locked versions | `make dependency-pins-check`; IC Build owns renderer regressions, while `scripts/release/test-release.sh` checks actual consumer preparation |
| Make/tool forwarding and server selection | `scripts/ci/test-testkit-commands.sh` |
| Hooks and formatter preservation | `make hooks-check` |
| Release/publication refusals and recovery | `scripts/release/test-release.sh` and `make release-commands-check` |
| Evidence selection, archives and retention | `make tooling-evidence-check` and `scripts/ci/test-evidence-archive.sh` |

Retire exported generic fixture roots through the canonical snapshot exporter at
its reviewed revision. Generic Make routing, LOC, npm-pin and registry-version
matrices remain at Shared; their runtime commands and Blob's actual dependency
and publication checks remain selected. Producer CI does not qualify Blob's native hosts or its
release adapter. The retained consumer checks and all native CI lanes still run.
