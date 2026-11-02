# Releasing

Blob keeps one workspace and lockfile with three independently released groups.
The [Shared release contract](releases.md) owns Git delivery/recovery mechanics;
Blob owns its group policy, full validation gate, receipts and registry admission.
Release commands remain maintainer-operated under [AGENTS.md](../AGENTS.md).
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
For example, an Engine Memory API change leaves Contracts and Client versions
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
all thirteen current Cargo package versions are 0.67.0. Contracts was released
and published first. Each group retains its own ledger, tag and receipt; future
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

The full `release-verify` / `ci` / `validate` gate remains workspace-wide. It covers
snapshot/tools/pins, Clippy, dependency boundaries, docs, consumer tooling, shell,
hooks, formatting, builds, native/PocketIC tests, Wasm and all public archives.
Group selection narrows metadata/delivery, not the validation gate. Ordinary
implementation uses [focused validation](governance/development.md).

Shared's group helper and the published IC Build editor render the selected
literal manifests, root catalog, lock entries and ledger; external lock selections
stay fixed. Blob renders in disposable committed source, checks formatting, then
writes its group receipt. Shared's transaction installs/checks that exact roster
and restores all original files after failure, retaining failed preparation inputs.
This includes the prior receipt. The receipt binds the group, version, validated
source, date, group policy hash and every other staged release file's bytes.

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

Publication holds the shared release lock, prepares locked sources and runs the
same Clippy gate before archive creation or registry contact. Cargo owns uploading
and index visibility. A retry skips an existing version only after non-yanked
registry metadata, archive checksum, VCS source and package path match this release.
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
