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
newer. Put prepared tools on PATH; for user-local installs:

```bash
export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"
make install-hooks
make release-plan VERSION=patch
```

Hook installation is local to this clone and refuses to replace existing hook
obligations. `release-plan` previews arithmetic and selected branch/remote without
Git effects. Execution checks saved unfinished intent before choosing another
version. Maintain one numbered undated changelog entry under
[the shared rules](../rules/changelogs.md); preserve released history.

The current pending batch is 0.15.0 because the ic-memory 0.27 public re-export
and durable ledger contract, funding receipt records and operator DTOs are breaking. Use the minor increment for this
batch after committing the implementation; a patch increment cannot include
this hard cut. Package metadata stays at 0.14.12 until release preparation.

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
Publication, deployment and cleanup are separate.

The complete gate is `make release-verify` (also `make ci` / `make validate`). It
verifies the shared snapshot, fetches the selected lock with `make deps`, then
runs shell/helper checks, hook regressions, manifest/Rust formatting, compilation,
Clippy, retained-probe checks, docs, native/PocketIC tests, Wasm and packaging.
Snapshot or fetch failure stops before compilation or version mutation. Offline
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
