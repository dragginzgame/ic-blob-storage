<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Releasing

The [common release contract](releases.md) owns the standard commands, exact
Git effects and interruption recovery. This repository supplies the validation
gate, Cargo metadata and source-bound receipt. Registry publication is a separate
maintainer action; a library release does not qualify the storage service.

## Setup and preview

Prepare the pinned compiler/components and bootstrap tools from
[dependency setup](dependencies.md), then run `make install-tools` and
`make tools-check`. This includes the lock-selected Testkit CLI/server. Run
`make deps` after dependency edits or cache removal. Offline checks never install
missing executables or change dependency selections; older tool/evidence
generations remain retained.

`make/consumer.mk` owns common setup, checks, formatting/hooks and release routing.
Blob supplies product validation, ordered Testkit extensions and direct-delivery
admission. Inspect the [snapshot](../.shared-tooling.snapshot) and review canonical
`--preview` before refresh. Required companions are derived from the selected
committed source; include every directly used helper as an entrypoint. Never patch
vendored bytes or use mutable sibling tools. See
[Shared snapshot adoption](consuming-snapshots.md) and
[the current adoption](status/current.md#upstream-package-and-shared-tooling-adoption).

Prepare ShellCheck separately; `SHELLCHECK=/absolute/path` selects an existing
binary. Release preparation uses the published IC Build editor selected from
Cargo.lock.
`make install-release-editor` prepares it explicitly; `make release-editor-check`
admits its absolute executable offline. Both are included in the complete tool
setup/checks. The adapter selects the editor automatically and supplies an 8 MiB
per-document input allowance; no manual environment export is needed.
`make release-tools-check` admits that editor, ShellCheck and the reviewed
Cargo sort/rustfmt without compilation. Both `fmt` and `fmt-check` use that same offline owner.
`make install-hooks` activates the reviewed hook; hook and formatting qualification
remain independent checks. Commands and package/version effects follow
[development governance](governance/development.md); release targets are
maintainer-operated here.

## Maintainer release and recovery

From committed clean source on the selected branch:

```bash
make release-patch # Select the increment from the complete pending batch.
# For another batch, select patch/minor/major from its complete compatibility.
```

All three invoke the same pinned runner. This repository's
`ci/release-delivery-policy.txt` admits only `RELEASE_DELIVERY=direct`. The shared
checker enforces the same file at Make's standard release-command/metadata-hook
boundary and in standalone metadata commands; unsupported or explicitly empty
selections refuse before effects. The shared PR
helper is included with the baseline, but Blob has not adopted merged-source
receipt adapters. Default destination inputs are `RELEASE_REMOTE=origin`
and `RELEASE_BRANCH=main`; an explicit override is saved with the exact push URL.
It preflights, validates, prepares Cargo/version/notes/receipt, stages only
Cargo.toml, Cargo.lock, CHANGELOG.md and docs/release.json, creates the release
commit and annotated tag, then atomically pushes exactly that branch and tag.
`--no-follow-tags` prevents configuration from adding other annotated tags.
The runner rechecks the saved destination after validation and before push,
and dispatches to the captured URL with an explicit option terminator.
Publication, deployment and cleanup are separate. The publication adapter also prepares
locked dependencies and runs the same Clippy gate after release/tag admission,
before creating publication archives or contacting the registry.
After confirmed delivery or completed resume, the runner refreshes the matching
configured upstream tracking ref from exact remote observation. It preserves
unrelated, newer, divergent, symbolic or concurrently changed refs; a local
tracking failure reports a fetch remedy without repeating delivery.
The shared runner, validation logger and formatting hook qualify actual GNU
Make recipe execution and failure propagation before their guarded effects.
Inherited ignore-errors, dry-run, question, touch or version-only modes refuse;
remove those modes before retrying. The isolated admission recipe loads no
consumer Makefile and preserves legitimate release variables and parallel settings.

The complete gate is `make release-verify` (also `make ci` / `make validate`). It
verifies the shared snapshot, local executables and dependency declarations, then
fetches the selected lock with `make deps` and runs the existing full-workspace
Clippy gate before other validation stages. Failure stops later compilation,
tests and packaging, including under parallel Make. The remaining gate retains
dependency boundaries, documentation links, shared fixtures, shell/helper checks,
hook regressions, formatting, compilation, probe/docs checks, native/PocketIC
tests, Wasm and packaging.
Prerequisite or fetch failure stops before compilation or version mutation. Offline
checks never select newer dependencies. The reviewed `scripts/ci/cargo-release-metadata.sh` owns coordinated Cargo
version/lock/changelog preparation through the selected IC Build editor.
Blob keeps source admission, its four-file rollback and receipt generation.
The helper checks the complete inherited member/lock roster with locked offline
Cargo metadata; external lock selections stay fixed. A later formatter or receipt
failure still restores the original manifest, lock, changelog and prior receipt.
Prepared metadata must pass formatting before its receipt is written or staged.
Numbered `## [X.Y.Z] - Unreleased` notes are accepted at preflight and finalized
with the saved UTC release date during preparation. Duplicate or competing
candidates still refuse; published history and exact-date recovery stay unchanged.

The release gate uses the reviewed validation logger around the same `ci` target.
Failed attempts retain unique raw logs under Git's
`release-state/validation-failures`; retries preserve earlier logs. If that
destination is unavailable, the logger preserves and reports its temporary logs.

If preflight or validation fails, fix and commit the source, then rerun the normal
target for fresh gates. Uncommitted preparation remains bound to its saved source
and increment. A normal direct target can supersede
preparation when a newer clean descendant commit records the local metadata's
disposition, with no staging evidence and the same destination. It archives the
old intent unchanged, reads the current version, applies the requested increment
and repeats preflight/full validation. Explicit resume remains exact-source.
Blob preparation changes only local metadata; failed transactions restore their
inputs and retain their backup. Committed receipts remain in Git, and validation
logs use unique retained attempts. Do not rewrite an old receipt or remove a
backup to trigger restart. See the [shared restart contract](releases.md#source-changes-after-saved-preparation-intent).

After the release commit exists, a normal command reconciles that
exact release even when newer fixes are committed on its descendant history:

```bash
make release-patch
# Explicit selection of retained intent, if needed:
make release-resume VERSION=VERSION_FROM_SAVED_PLAN
```

An unchanged same-kind retry finishes only the saved release. With newer
committed fixes or a different requested increment, the command finishes the old
release first, then reads the actual local version and validates the next release
against current source. For example, an interrupted minor at `0.15.0`, followed
by a committed fix, lets `make release-patch` reconcile `0.15.0` and then validate
`0.15.1`. Explicit resume finishes only the selected release.

Late checks receive `RELEASE_COMMIT` separately from `RELEASE_SOURCE`. They read
Cargo files, notes and the receipt from that exact commit and bind its sole parent
and annotated tag to the saved identity. New draft notes do not invalidate the
older receipt. Both prepared-source checks and these committed-source checks use
Shared metadata validation with `--manifest-order-grouped`; committed checks
export the selected commit rather than sorting or inspecting current draft files.
The hook and independent `fmt-check` use the same grouped policy. These checks
do not enforce deployment dependency exclusions; see
[dependency admission](dependencies.md#dependency-roles-and-deployment-admission).
Recovery pushes the selected release commit; a verified remote
descendant is preserved when only its tag is missing. New fixes need their own
complete validation before their next release push.

A lost push reply does not prove failure. Matching remote branch and tag complete
the saved release without another push; failed readback or conflicting identity
stops recovery. Never force-push, overwrite tags, increment prepared metadata
again or remove the only retained evidence. Plans live in Git's `release-state`
directory. An occupied lock requires inspecting its owner before manual removal.

The former manual staging/commit/push sequence and exact-version one-shot are
retired. Use the standard semantic release target and its saved recovery, rather
than creating a second release workflow. `make release-tag-check` remains a
read-only check of an already completed source/receipt/tag. `make release-check`
uses isolated Git/Cargo substitutes: no real commit, tag, push or publication.
It also invokes `make release-commands-check`, which exercises all entry points
and conflicting selections against a substitute runner in a private Makefile
copy. The repository's exact metadata, receipts and interrupted effects remain
covered by its own fixtures.
Failed local fixtures and preparation inputs remain at the printed paths.

## Publication and deployment

Publication remains an explicit maintainer action after the clean release,
receipt, exact parent and annotated tag checks:

```sh
make publish
```

The default calls standard Cargo for Contracts first, then Client, then Core, under one release
lock. Cargo owns uploading and waiting for index visibility. A failed package
publication stops before later packages. Retry the same command after investigating any
failure: an exact version already on crates.io is skipped only after its archive
checksum and Git source match the selected release. Failed readback, yanked rows,
malformed responses or a different source refuse instead of authorizing an upload.
Readback inputs and a fresh Cargo target remain under the printed
`target/publication.*` path, preventing stale same-version verification artifacts. No local
publication progress journal or additional polling loop is introduced.
API and dry-run dependency reads use the reviewed
[Shared metadata helper](verification-helpers.md#exact-cratesio-version-observation),
requiring jq and curl 8.4.0 or newer for bounded responses. Each observation has
its own new directory retaining request, response, HTTP/transport status and
diagnostics, including failed reads. Shared owns exact metadata validation and
the identifying API User-Agent; Blob refuses yanked versions and authenticates
archive checksum, release source and package path before skipping an upload.
Archive reads identify Blob's version and repository. Both transport owners
disable implicit curl configuration.
A 403 remains an inconclusive fatal response; it never authorizes uploading.

`PUBLISH_PACKAGE=ic-blob-storage-contracts`, `ic-blob-storage-client` or `ic-blob-storage`
selects one package when needed; it is optional. `make publish-dry-run` never
uploads and still runs Cargo verification for already-published packages. Before
contracts is published, the default dry-run verifies Contracts then refuses Client
with dependency guidance: a dry-run cannot make that registry dependency exist.
Use `make package` for coordinated local payload verification, or select contracts for
its individual dry-run. Publishing neither deploys nor qualifies the service.
Older completed releases need no runner plan for this separate publication check.

`make package` builds the opt-in native `blob-package-verify` contributor helper,
then qualifies all three exact `.crate` payloads through IC Build. The helper's
`package-verification` feature is disabled for ordinary CLI builds; public
libraries/contracts and production Wasm have no IC Build dependency. Run
`make package-verifier` to prepare only that executable. An explicit
`BLOB_PACKAGE_VERIFIER` must name an existing absolute executable.

Blob selects the three package identities, contracts-only temporary registry patch,
archive location and fresh retained evidence directory. IC Build owns packaging,
exact extracted graph admission, frozen external selections, isolated compilation,
all-target/Rustdoc tests and final integrity checks. Each source/metadata/archive
file and each command's stdout/stderr has a 64 MiB ceiling, with at most 100,000
source entries and depth 64. Build duration has no implicit deadline.

The real same-version regression and graph/byte/source/manifest/lock refusal
cases remain consumer checks. Ordinary nonzero and signal termination statuses
propagate through the thin wrapper; original command diagnostics and partial
attempts remain under the printed `target/package/blob-verification.*` root.
No package manifest is rewritten and no override enters publication. IC Build's
fresh target prevents fixed archive timestamps from admitting stale same-version
builds, while avoiding Cargo's
[local-registry checksum failure](https://github.com/rust-lang/cargo/issues/14396).
Package assembly needs Cargo 1.90 or newer; Blob's supported compiler floor is
Rust 1.91. Package qualification never publishes or qualifies a deployed service.

Validation builds use the source version before release preparation. Preserved
binaries may report that earlier version after a successful release. Before
standalone deployment, rebuild at the selected released version, freeze its
bytes and run the maintained PocketIC installation carrier case with
`BLOB_EXPECTED_HOST_RELEASE` set independently to that version. Its compiled
readback and reviewed module hash must match; a tag, receipt or filename does
not establish the Wasm's compiled release.

Before a hard-cut reinstall, complete the
[retirement runbook](retiring-installations.md). It preserves exact provider,
reference, uncertainty, balance and billing obligations independently of the
new allocation ledger. Passing release validation does not authorize reset.

No release command cleans consumer build or evidence artifacts. `make clean`
is a separately authorized action. Native macOS qualification must be recorded
for the selected source; Linux fixture passes do not establish it.
