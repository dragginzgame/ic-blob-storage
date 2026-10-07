<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Releasing

The [common release contract](releases.md) owns the standard commands, exact
Git effects and interruption recovery. This repository supplies the validation
gate, Cargo metadata and source-bound receipt. Registry publication is a separate
maintainer action; a library release does not qualify the storage service.

## Setup and preview

Follow [dependency setup](dependencies.md), including pinned cargo-sort 2.1.4,
rustfmt, ShellCheck, Perl with JSON::PP/Digest::SHA, Git, GNU Make and Bash 3.2 or
newer. Provision the reviewed repository-local executables explicitly; for
user-local formatter and shell tools:

```bash
export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"
make install-tools
make tools-check
make release-tools-check
make install-hooks
make release-plan VERSION=patch
```

Hook installation is local to this clone and refuses to replace existing hook
obligations. `release-plan` previews arithmetic and selected branch/remote without
Git effects. Execution checks saved unfinished intent before choosing another
version. Maintain one numbered undated changelog entry under
[the shared rules](../rules/changelogs.md); preserve released history.
Release preflight runs `release-tools-check` before full validation: the selected
ShellCheck executable must run, cargo-sort must match the reviewed version and
rustfmt must report availability through the shared offline prerequisite helper.
Missing or incorrect tools report explicit installation commands; the check
does not install, compile, mutate release files or replace the full gate.
`SHELLCHECK=/absolute/path/to/shellcheck` selects an existing executable.

The released baseline is 0.16.0, including the public ic-memory 0.30
type-identity hard cut. Keep package metadata and the release receipt at 0.16.0
during ordinary development. The single undated entry in
[the changelog](../CHANGELOG.md) records the complete pending batch;
[the current handoff](status/current.md) records its compatibility and selected
dependency graph. Choose the patch or minor target for that complete batch under
the pre-1.0 compatibility rule, rather than repeating the previous minor release.
The current pending batch targets 0.17.0 because the public Memory re-export now
selects 0.31. Host compositions must align that type identity. Use the minor
target when the maintainer releases the complete batch.

Changelog preflight captures a candidate without writing, using the exact saved
`RELEASE_PREVIOUS`, `RELEASE_VERSION` and `RELEASE_DATE`. Finalization uses those
same inputs even after package metadata changes; failed process output is never
written. Historical duplicate release identities still refuse independently.
The shared owner can create a missing heading or finalize empty notes; maintain
meaningful notes under the changelog rules, rather than making presentation a
release gate. This adapter selects `allow_finalized=0`: prepared-state recovery
verifies the frozen receipt/payload instead of re-finalizing or changing dates.

Consumer admission requires successful working-tree, HEAD, parent, version and
receipt-source reads before comparing their output. Matching output from a failed
command is refused before proceeding. These checks preserve existing metadata
and build inputs; they do not replace full validation or exact tag verification.

Commit the implementation and notes before releasing. Agents never create those
commits or invoke the one-shot commands below. The explicit metadata/check
adapters are available for inspection or separately authorized preparation;
do not bypass the runner's saved intent or reuse validation from another source.

## Maintainer release and recovery

From committed clean source on the selected branch:

```bash
make release-patch
# Or select release-minor / release-major for the complete batch's compatibility.
```

All three invoke the same pinned runner. Default inputs are `RELEASE_REMOTE=origin`
and `RELEASE_BRANCH=main`; an explicit override is saved with the exact push URL.
It preflights, validates, prepares Cargo/version/notes/receipt, stages only
Cargo.toml, Cargo.lock, CHANGELOG.md and docs/release.json, creates the release
commit and annotated tag, then atomically pushes exactly that branch and tag.
`--no-follow-tags` prevents configuration from adding other annotated tags.
The runner rechecks the saved destination after validation and before push,
and dispatches to the captured URL with an explicit option terminator.
Publication, deployment and cleanup are separate.
The shared runner, validation logger and formatting hook qualify actual GNU
Make recipe execution and failure propagation before their guarded effects.
Inherited ignore-errors, dry-run, question, touch or version-only modes refuse;
remove those modes before retrying. The isolated admission recipe loads no
consumer Makefile and preserves legitimate release variables and parallel settings.

The complete gate is `make release-verify` (also `make ci` / `make validate`). It
verifies the shared snapshot, local executables and dependency declarations,
checks local documentation links and shared digest/installer fixtures,
fetches the selected lock with `make deps`, then
runs shell/helper checks, hook regressions, manifest/Rust formatting, compilation,
Clippy, retained-probe checks, docs, native/PocketIC tests, Wasm and packaging.
Prerequisite or fetch failure stops before compilation or version mutation. Offline
checks never select newer dependencies. The metadata transaction updates only
workspace versions and local version-qualified references in Cargo.lock, then
checks Cargo metadata with `--offline --locked`; external selections stay fixed.
Prepared metadata must pass formatting before its receipt is written or staged.

The release gate uses the reviewed validation logger around the same `ci` target.
Failed attempts retain unique raw logs under Git's
`release-state/validation-failures`; retries preserve earlier logs. If that
destination is unavailable, the logger preserves and reports its temporary logs.

If preflight or validation fails, fix and commit the source, then rerun the normal
target for fresh gates. Uncommitted preparation remains bound to its saved source
and increment. After the release commit exists, a normal command reconciles that
exact release even when newer fixes are committed on its descendant history:

```bash
make release-patch
# Explicit selection of retained intent, if needed:
make release-resume VERSION=0.14.12
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
older receipt. Recovery pushes the selected release commit; a verified remote
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

Use `make publish-dry-run` or `make publish` explicitly for crates.io. They verify
the clean completed release, receipt, exact parent and annotated tag before
calling Cargo. Publication holds the same lock as release preparation and
recovery. Publishing neither deploys nor qualifies the service. Older
completed releases need no runner plan for this separate publication check.

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
