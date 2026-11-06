# Releasing

Blob keeps one workspace and lockfile with three independently released groups.
The [Shared release contract](releases.md) owns Git delivery/recovery mechanics;
Blob owns its group policy, validation selection, receipts and registry admission.
Release authority follows [AGENTS.md](../AGENTS.md), including explicit delegated delivery.
Publication neither deploys nor qualifies the storage service.

## Groups and history

| `RELEASE_GROUP` | Published package | Other coordinated members |
| --- | --- | --- |
| `contracts` | `ic-blob-storage-contracts` | None |
| `client` | `ic-blob-storage-client` | Native CLI |
| `engine` | `ic-blob-storage` | Standalone host, six probes, test protocol and PocketIC support |

[ci/cargo-release-groups.json](../ci/cargo-release-groups.json) is authoritative.
Every member has a literal version; dependencies inherit the root catalog. A
release updates only the selected group's versions and its catalog requirements.
For example, an Engine Storage API change leaves Contracts and Client versions
unchanged. A Contracts release updates their inherited Contracts requirement,
without claiming those consumers have themselves been released. Decide whether
consumer republication is required from the changed contract, not supplier age.
Breaking public changes before 1.0 require the affected group's next minor.

Native `publish-map`, `publish-file-status` and `publish-session` require
`BLOB_EXPECTED_HOST_RELEASE` to select the deployed Engine release explicitly.
Make exports the current standalone package selection; direct CLI callers and
native launcher parents must set their approved deployment's version themselves.
The existing launcher `env` option can carry it. No discovery, default to a
Contracts/Client version or relaxed comparison is introduced. Both current and
retained authenticated configuration replies must match the selected release;
changing this pin cannot authorize old session recovery across a reinstall.
Saved intent/control/installation formats and Wire packets are unchanged.
The first independent Client, Contracts and Engine releases are **0.66.0**.
Platform/Wire/Memory public Rust identities required these independent minor
releases; matching numbers do not impose a workspace-wide version rule. The Rust support-floor change is delivered at **0.67.0** in each affected group;
Contracts was released and published first. The root manifests own current
package versions. Each group retains its own ledger, tag and receipt; future
changes do not require synchronized releases.

All members enter this layout at **0.65.0**, without a conversion bump. The
[root changelog](../CHANGELOG.md) indexes group ledgers. Coordinated history is
preserved once in [releases/history.md](../releases/history.md); the existing
[0.65.0 receipt](release.json) and `v0.65.0` tag remain historical evidence.
Group `release.json` files initially contained explicit inherited-baseline markers,
which could not authorize publication. Each 0.66.0 group release replaced only its
own marker with source-bound evidence. Resume or publish
old coordinated releases with their original checkout/tools; no dual reader is
introduced.

## Tools and preview

Prepare the compiler/components from [dependency setup](dependencies.md), then
run `make install-tools` and offline `make tools-check`. Explicit setup includes
the lock-selected Testkit/server and IC Build release editor. After dependency
edits/cache removal use `make deps`; checks never update the graph or install
missing tools. `make release-editor-check` admits the prepared absolute editor,
with an 8 MiB per-document allowance. `make release-tools-check` additionally
admits ShellCheck and the pinned Cargo sort/rustfmt. An existing ShellCheck may
be selected with `SHELLCHECK=/absolute/path`.

Use the [canonical exporter](consuming-snapshots.md) for snapshot adoption, preview
its impact and include every directly invoked helper as an entrypoint. Never
patch shared files or route to mutable sibling tools. The hook, `fmt-check` and
committed metadata checks all use grouped manifest ordering across all members.

Preview without effects:

```sh
make version RELEASE_GROUP=engine
make release-plan RELEASE_GROUP=engine VERSION=minor
```

Missing, empty or unknown group selections refuse at Make and direct adapter
entrypoints before setup or release effects. There is no default group, inferred
selection, all-groups release or automatic dependent release.

Shared 0.30.8 supplies the [frozen batch runner](frozen-releases.md), including
recovery of generated IDs containing underscores. Blob calls that runner directly
from the ordinary shell, keeping the complete supplier-first roster explicit:

```sh
mkdir -p target/tmp
TMPDIR="$PWD/target/tmp" perl scripts/ci/run-frozen-release.pl --root "$PWD" \
  --groups engine --kind patch --publish-target publish
```

Select only the changed groups (for example `contracts,client,engine` when all
three have pending releases). This requests publication too. The first invocation
needs clean committed source and no ambient Cargo target/home override. Do not
invoke it through an outer Make exporting normal-checkout tool paths. The child
Make reconstructs its own source/artifact/tool paths and uses the existing group
hooks. The short, source-independent temporary root keeps PocketIC socket paths
bounded; it contains scratch, never live release source or lockfiles.

The consumer qualification is `bash scripts/release/test-release.sh --frozen`:
real group hooks/metadata/receipts, a local bare remote, compiler/upload substitutes,
deliberately unusable deferred normal lock bytes and explicit publication recovery.
Real locked compilation, installed behavior and archive validation remain required
in the captured checkout before any production delivery. Resume a retained batch
through Shared's exact saved-ID interface; reconcile uncertain registry outcomes
before supplying a publication result. Keep the complete durable Git store.
The normal checkout stays on its original source, preserving later edits; reconcile
it against verified delivery before the next batch. The underlying selected-group
runner and its source-drift refusal still protect the captured checkout. Archive
verification conservatively checks all three published libraries, as required
by the current mapping; archive narrowing is not qualified. Neither snapshot
adoption nor the scoped routing fixture establishes release wall-time savings.

## Maintainer delivery and recovery

From clean committed source and notes, choose the selected group's increment:

```sh
make release-patch RELEASE_GROUP=engine
# Or release-minor / release-major, according to the affected public contract.
```

`ci/release-delivery-policy.txt` admits direct delivery only. Branch and remote
default to main/origin; explicit selections and exact destination URL are saved.
Preflight checks the selected ledger's numbered `Unreleased` candidate before
setup. Authorized one-shot commands prepare the locked sources/tools, then admit
them offline and recheck pins, source identity and cleanliness. Post-validation
preparation and recovery do not repeat installation. Direct preflight is offline
unless preparation authority was supplied by the standard one-shot command.

The full `ci` / `validate` gate remains workspace-wide. Its ordered stage roster
comes from [one policy](../ci/release-validation.json), also consumed by release
plans; Make does not maintain another target list. An unreadable/empty roster
refuses full CI, and command-line target-list overrides cannot narrow it.
Primitive targets do not load the full-stage roster. The gate covers
snapshot/tools/pins, Clippy, dependency boundaries, docs, consumer tooling, shell,
hooks, formatting, builds, native/PocketIC tests, Wasm and all public archives.
`release-verify` uses Blob's [stage policy](../ci/release-validation.json) and
[selector](../scripts/release/validation.pl) with Shared's
[plan runner](release-group-validation.md). Ordinary implementation uses
[focused validation](governance/development.md).

### Validation scope

The selector reviews the entire source delta since the selected group's annotated
release tag. The tag's receipt must already use structured plan evidence and
checkpoint the same [package-scope policy](../ci/release-package-scopes.json).
The first delivery of the package mapping therefore runs the full roster. Missing history,
old receipts, changed manifests/locks, compiler, tooling, recipes, generators,
policy or unclassified files select the full roster. Ambient Cargo configuration
or build/compiler flag overrides also keep the full roster. Renames include both paths.
The selected baseline commit is recorded in the plan's reasons; moving its tag
invalidates that proof. A shallow checkout cannot establish a narrow scope.

| Classified change | Required checks |
| --- | --- |
| Replicated tenant-client implementation only | Common guards and all archives; Client and Engine package roots for Clippy/native tests/docs/Wasm checks; installed standalone/CLI and replicated tenant-client scenarios |
| Other Client/CLI implementation | The same Client and Engine roots plus every installed integration scenario; storage and consumer probes depend on Client |
| Contracts implementation | All thirteen package roots and every installed integration scenario, covering public identities and reverse consumers |
| Engine implementation | Engine roots for the four automatic Cargo stages; common CLI/probe/archive boundaries and both installed partitions remain required |
| Release machinery unchanged in either case | Real selected-group metadata rendering/rollback and preservation of other groups; omit the full publication/recovery fixture matrix |
| Product implementation with unchanged CLI/verifier contract | Omit `package-regression`; keep actual `package` verification |
| Shared or uncertain inputs | Every stage, including the full release-adapter matrix |

Only numbered group notes and the current status handoff are separately admitted
documentation changes. This deliberately conservative mapping does not infer scope
from a selected group alone or an unchanged supplier version. New stages update
the policy and their actual Make recipe; both full CI and release plans consume
that same roster. A missing recipe fails as a required obligation.
The package policy records the reviewed reverse-consumer closure; fixtures compare
it with every Cargo local dependency, including dev/build/optional and target-specific
edges. Every scoped Cargo stage keeps its existing features, targets and flags;
only package roots change. Native test and Rustdoc roots retain their original
full-gate rosters. Package flags are bound into stage reasons, re-derived from the
exact source and configuration before execution, and checkpointed with the policy.
Normal Make targets remain full; `make ci` removes any ambient active-plan setting.

Clippy owns automatic native compilation admission. Its recipe and the optional
`make check` use identical package roots, native target, default profile,
`--all-targets` and `--all-features`; Clippy additionally denies warnings. Neither
owned Rust source nor the CLI build script has a Clippy-specific compilation
branch. The [Clippy Cargo interface](https://doc.rust-lang.org/clippy/usage.html#workspaces)
supports these package selections. CI and release plans therefore omit the
duplicate native `check` stage; `make check` remains available separately.
The standalone package mapping and source-bound historical receipt interpretation
remain intact. Isolated/default-feature graph checks, Wasm, Rustdoc, installed
tests and actual archive verification keep their distinct obligations.

Cargo's locked/offline metadata supplies member names and manifest paths, including
aliases, globbed members and table-form dependency declarations. The bounded
name/path projection is recorded in the checkpointed configuration input and
embedded receipt; verification uses that observation rather than parsing a root
manifest or consulting mutable paths for an old source. Package versions are not
part of this path projection, so authorized version preparation leaves it unchanged.
This inventory query uses `--no-deps`; complete lock/graph admission remains with
the existing release preflight and group renderer, not this projection.

The two installed-test partitions are disjoint and exhaust Cargo's test roster.
One `test-pocketic` stage selects both through a single sequential, fail-fast Make
graph. Their common `test-pocketic-prepare` prerequisite admits Testkit and builds
the probe/standalone Wasm and CLI once. An admitted tenant-client-only plan selects
only `test-pocketic-client-run`; all other plans retain both partitions. The exact
partition list is bound into the stage reason and re-admitted before dispatch.
Individual `test-pocketic-client`/`test-pocketic-engine` targets remain self-contained;
there are no persistent preparation stamps or skip flags. Ordinary CI and direct
`make test-pocketic` run both; release-only candidate reuse below is explicit.

`package-regression` separately qualifies the archive verifier's same-version,
warm-target and refusal behavior. A current annotated baseline with the same stage
roster/configuration can omit it for admitted product-only changes outside the CLI.
CLI/verifier implementation, wrappers, manifests/locks, compiler/tools, policy and
unknown changes retain it. Full CI retains the regression, while hosted native
jobs use the admitted compatibility plan on each host. `make package` always
builds the verifier, checks the Client license and verifies the actual current
package archives; it does not run the regression fixture. Missing/legacy baselines
and uncertain closure keep the regression required. Real archive verification is
never inferred from the regression's result or skipped with it.

Original/O3/Os/Oz qualification is an explicit `make test-wasm-opt` compatibility
check, outside the normal release roster. Linux hosted CI runs it for Engine/shared
changes and uncertainty; manual workspace dispatch forces it. An explicit
`workflow_dispatch` group selection runs `release-verify` on Linux, Apple Silicon
macOS and Intel macOS. Main/PR and manual workspace CI retain the existing full
checks. Group dispatch retains source-bound plans, package reasons, receipts and
stage logs on each host; it does not force an otherwise excluded Engine optimizer
matrix. Generated Wire clients and isolated feature/compiler graphs remain independent
hosted obligations. On manual group dispatch, the plan owns Rust regressions and
native CLI/installation/restoration obligations, including justified exclusions;
the workflow does not append `test-native-host` or separate Contracts/Core/Client
Rust tests. Normal main/PR and workspace dispatch retain those native-host checks;
portable Contracts tests run once in the Linux schema job and Clippy stays in
the native matrix. Routine builds ship
untransformed Cargo Wasm. Their installation, carrier rejection, corruption,
restoration and fencing scenarios stay in the required integration tests. Native
Linux and both macOS lanes retain actual installation/CLI/archive/reopen checks;
they no longer each repeat the Binaryen matrix. The complete release-adapter
fixture matrix similarly runs on Linux when infrastructure changed or is uncertain;
all native hosts retain a real Engine metadata/rollback smoke check.

Each release attempt retains its plan, policy, checkpoints, per-stage logs,
exclusion reasons and timings in `target/blob-release-validation.*`. Only a completed
attempt writes `.git/release-state/validation/<group>-<source>.json`. A fresh attempt
invalidates that selectable record first, preserving prior logs. Missing, failed,
blocked or drifted obligations cannot authorize preparation. The new
`release-verify-stages-v1` receipt embeds source/group/host/compiler, plan/policy,
input hashes, hashed build configuration and executed log identities/timings.
Configuration values are hashed rather than retaining credentials or file contents;
the generated configuration input is checkpointed with the plan, re-observed after
execution and compared again before preparation. Prepared/committed/published
checks re-admit the same selection against its immutable source. Historical
whole-gate receipts retain their original tools; they are not upgraded or reused.
Historical `.tmp/` attempts remain untouched and are still selected by failure
evidence collection. New selector/proof scratch also lives under `target/`.

### Reuse across group releases

Shared Tooling 0.30.0 prepares disposable final candidate source with the canonical
IC Build editor. All pending numbered group versions are compiled as their actual
intended final versions. Later supplier-first group releases can reach identical
candidate content; there is no version stripping, old binary rebinding or whole-
gate cache. Fixed candidate-only notes/empty receipts cannot authorize a release.
Real group rendering, rollback, source/index/selection, tools, compiler, dependency
and boundary checks, formatting, archive verification and registry admission remain
fresh in the actual checkout.

`ci/release-stage-reuse.json` selects eight candidate stages: Clippy, release-adapter
regressions, probe check, Rustdoc, native tests, the combined installed
stage, Wasm check and archive-verifier regressions. Each admission hashes full candidate source, dependency sources,
Rust/native toolchain inputs, explicit environment and host identity. It records
execution or reuse against an immutable original proof and logs; missing or changed
proofs run the stage. Changed source/tool/configuration cannot satisfy an exact
match. Both workspace and admitted group plans can use candidate reuse on
Linux/x86_64. A group plan's exact package roots and installed partitions are
copied from the existing selector into an immutable leaf-scope input. Its bytes
are hashed into the stage inventory and its identity is part of the executed
command. Capture and later receipt verification compare that command with the
original plan. Different package/partition obligations require different proofs;
a Client-only installed proof cannot satisfy an Engine plan requiring both
partitions. Narrow group plans remain narrow at the leaf commands.

An explicit `POCKET_IC_BIN` can participate only when it is the exact absolute
path returned by the canonical managed-server check. The child receives that
explicit selection unchanged, and its environment and admitted payload are part
of the proof. This configuration has its own execution identity; it is not
silently converted into the default selection. Other explicit binaries retain
caller-owned execution.

Unsupported hosts, parallel Make/jobserver scheduling, custom target directories,
ambient Rust/Cargo configuration, relative/empty PATH entries and unsupported
temporary roots still execute the original recipe. Before execution, the log
prints each concrete reason as `Stage reuse unavailable [STAGE]: ...`, naming
configuration keys without disclosing their values. The attempt retains
`stages/STAGE-eligibility.json`, even if the fallback fails. A successful direct
execution also records its reasons in `stages/STAGE.json`. Eligibility means the
configuration can be considered; missing or mismatched proofs still execute.
Native macOS remains unqualified for the candidate observer's native input closure.

For focused changes to this mapping, run
`bash scripts/ci/test-release-stage-reuse.sh --scopes`. It uses real Cargo package
selection and two selected installed scenarios to qualify matching scopes,
partition separation, the explicit managed server, fallback diagnostics and
publication refusal when the actual dependency closure differs.
It is routing/reuse qualification, not another complete service-acceptance gate.
The full fixture and publication-only mode retain their separate purposes.

The installed candidate suites cover final versions. After every admitted
installed stage, a small fresh actual-source sweep checks installation/reopen version
binding, checkpoint corruption and CLI/host release identity. Intermediate mixed-
version packages are still archived and verified fresh. Candidate sources/builds
stay under `target/release-stage-cache/` and `target/release-stage-build/`; their
read-only prepared tools root is explicit. No stage setup or tool-root symlink is
created. A short explicit temp root under the owning checkout's `target/` keeps
PocketIC Unix sockets within the platform path limit; long temp roots fall back
to ordinary execution. Retain this cache's selected original proofs/logs with the
group evidence. Release related groups supplier-first from the same checkout,
retaining `target/` between them; `cargo clean` discards this reuse evidence.

Publication first authenticates an existing registry archive: an exact non-yanked
retry needs no repeated compiler/lint/archive work. New uploads/dry runs keep fresh
dependency/compiler admission and actual archives. Clippy may consume the release's
completed candidate qualification only after tag/source/exact changed-path roster,
original proof/log custody, complete retained candidate inventory and exact actual
primary package/target/feature/dependency closure all agree. This historical lint
qualification is not a claim that a new execution ran on today's host. It avoids
candidate rendering/tool setup again. Missing local evidence or a differing closure
runs ordinary current-source Clippy before archive work/upload. No skip flag or
registry-proof substitution is introduced.

Focused producer and consumer fixtures exercise real Cargo execution, supplier-
first group preparations, fresh receipts, later admissions, publication closure
and source damage refusal. The full real consumer fixture runs only when release
infrastructure is changed/uncertain on Linux, rather than at every group release.
For a publication-only change, `bash scripts/ci/test-release-stage-reuse.sh
--publication-only` qualifies the real wrapper with a Contracts-only Cargo lint
proof, target/configuration fallback and retained-log-loss fallback, using a
substitute registry/upload. The default infrastructure fixture retains all three
group stages and exercises the same wrapper. See
[the Shared protocol](release-validation-reuse.md) for producer contracts.

For [Blob #114](https://github.com/dragginzgame/ic-blob-storage/issues/114), Shared's
[evidence interface](group-validation-adoption.md) now admits the full-stage roster,
observes annotated Git baselines and verifies each actual retained run. Blob no
longer parses runner events or timings. The checkpointed context observes the
host/compiler before execution; Shared binds it with plan, policy, inputs and logs.
Its protected receipt writer publishes only a successful capture.

The self-contained Blob receipt retains that Shared proof plus product selection
and source/configuration bindings. Blob re-admits these after version preparation
and in a fresh publication checkout; original absolute runner paths are never
rewritten or treated as portable execution evidence. Shared's `check` command
re-admits raw logs only at their original physical source location. The three-group
reverse-consumer mapping, installed obligations, receipt custody and publication
admission remain local. [Shared #349](https://github.com/dragginzgame/shared-tooling/issues/349)
owns the common verifier; #114 was closed after consumer delivery; current reuse is a separate follow-up.

Shared's group helper and the published IC Build editor render the selected
literal manifests, root catalog, lock entries and ledger; external lock selections
stay fixed. Blob renders in disposable committed source, checks formatting, then
writes its group receipt. Shared's transaction installs/checks that exact roster
and restores all original files after failure, retaining failed preparation inputs.
This includes the prior receipt. The receipt binds the group, version, validated
source, date, group policy hash, structured validation proof and every other
staged release file's bytes.

The exact roster comes from `make release-files RELEASE_GROUP=engine`: root
manifest and lock, selected member manifests, ledger and receipt. The root
changelog index and other group ledgers/receipts are not release files. Staging
must contain exactly the admitted roster, with no unrelated staged/unstaged work.
The runner creates `Release engine X.Y.Z`, annotated `engine-vX.Y.Z`, then pushes
exactly that branch and tag atomically, with implicit tag following disabled.
It retains configured-destination and exact remote readback guards. No force push.
Make ignore-errors/dry-run/question/touch modes refuse before guarded effects.

Normal targets always validate fresh clean source; they do not finish old intent.
After fixing/committing a gate failure rerun the normal command. To recover one
saved attempt, retain its original group and version:

```sh
make release-resume RELEASE_GROUP=engine VERSION=0.65.1
```

Plans use Git's `release-state/engine-0.65.1.plan` namespace. Source, date,
increment, destination and payload remain bound. Later descendant notes or another
group's release cannot substitute for the selected older receipt: late checks
read `RELEASE_COMMIT`, bind its sole parent to `RELEASE_SOURCE` and verify its
group tag. Matching remote readback reconciles a lost push reply; inconclusive or
conflicting observations refuse. Never rewrite receipts/tags, force-push or erase
the only evidence. Inspect occupied lock ownership before any manual removal.

`release-tag-check` is read-only. `release-check` exercises disposable Git
repositories, real selected metadata/editor/receipt checks and substituted expensive
build/registry effects. It creates no real checkout commit/tag, upload or push.
`release-commands-check` exercises all standard Make routes for each group with a
substitute runner. Failed fixture/preparation paths are printed and retained.
Validation logs remain in unique Git `release-state/validation-failures` entries;
successful retries do not replace failures.

## Retained-state admission

Shared Tooling 0.29.4 checks compact Git-side release/publication/retired-checkout
state before and after release adapter hooks. Blob also applies the canonical
`check-retained-state.pl` helper to direct `release-verify` and publication entry
points. The default limits are 512 MiB aggregate, 32 MiB per file and 30 days for
bulk files of at least 1 MiB; small journals/receipts do not expire. Nested build
or cache trees and symlinks refuse. Ordinary CI scratch stays under `target/`.

Refusal preserves retained evidence and grants no cleanup authority. A refused
initial verification keeps its previous receipt; a failed post-validation check
withdraws the new selectable receipt so it cannot stand in for a successful gate.
Publication still releases only its owned lock. Review active recovery, unique
work and existing provider obligations before any separately authorized cleanup.

## Publication and deployment

Publish from the selected group's clean release commit/tag checkout:

```sh
make publish RELEASE_GROUP=contracts
make publish RELEASE_GROUP=client
make publish RELEASE_GROUP=engine
```

Each command publishes only that group's public package. Publish Contracts first
when a Client/Engine version requires a newly released Contracts version. There
is no requirement to republish an unchanged group. `PUBLISH_PACKAGE` is optional
and may only name the selected group's primary package; foreign selections refuse
before lock/setup. Separate publication requires its completed receipt, exact
parent and annotated tag. A later group release changes shared metadata; switch
to the selected group's tag for publication/readback rather than rewriting receipts.

Publication holds the shared release lock and admits the exact release/tag before
observing registry state. A normal retry skips an existing version only after non-yanked
registry metadata, archive checksum, VCS source and package path match this release.
That verified read-only retry performs no dependency preparation, compilation or
Clippy. New uploads and dry runs prepare locked sources and admit the compiler,
then consume completed exact-primary candidate Clippy or execute current-source
Clippy before Cargo creates/verifies an archive or uploads. The standard wrapper's newly
created `target/publication.XXXXXX/target` is admitted only for that same run's
historical lint check. Arbitrary custom target directories and compiler/configuration
overrides retain fresh lint; exact tag/source, candidate inventory, sealed proof/log
custody and primary closure checks still apply. Cargo owns uploading and index
visibility.
403, failed transport, malformed metadata or foreign bytes/source are fatal and
cannot authorize upload. API reads use Shared's exact metadata helper with jq and
curl 8.4.0+; archive reads also disable implicit curl configuration. Request,
response, status and failures stay in fresh `target/publication.*` evidence.

`publish-dry-run` never uploads and still invokes Cargo verification for an
already-published version. Client/Engine dry-runs require the **Contracts catalog
version**, which may differ from their own version, to be indexed. Use `make
package` for local mixed-version verification before that supplier publication.

`make package-verifier` builds the opt-in native CLI contributor binary. `make
package` uses it to qualify all three exact `.crate` payloads at their independently
observed versions through IC Build, with only Contracts temporarily registry-patched
inside extracted verification. Normal CLI/product Wasm does not enable IC Build.
IC Build owns exact extracted graph admission, frozen external selections, fresh
compilation, all-target/Rustdoc tests and final source/archive/manifest/lock hashes.
Limits are 64 MiB per file/command stream, 100,000 entries, depth64, no implicit
build deadline. Nonzero/signal statuses and partial evidence propagate. Blob's
mixed-version warm-cache regression executes changed current payloads. Verification
never rewrites package manifests for publication or qualifies deployed service use.

Validation builds precede version preparation, so retained binaries may report the
previous version. Rebuild standalone Wasm at the released **Engine** version before
deployment; freeze its bytes and run the maintained installation carrier with
`BLOB_EXPECTED_HOST_RELEASE` independently selected. Default Make attribution comes
from the standalone package's catalog version, never the Client/CLI version.

Complete the [retirement runbook](retiring-installations.md) before hard-cut
reinstallation; preserve provider objects, balances, uncertainty and billing
obligations. Release/publication does not authorize reset or deployment. No release
command cleans consumer artifacts; `make clean` is separate. Linux/cross-build
checks do not establish native macOS or live provider qualification.
