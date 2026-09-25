# Releasing

The command family follows the sibling ic-timers/ic-memory libraries.
The package is currently an unpublished scaffold with publishing disabled.
The commands are installed for future maintainer use; none has released it.

## Preview and prepare

Use make release-plan VERSION=patch (or minor, major, x.y.z) for an effect-free
preview. Make patch, make minor and make major validate committed clean source,
then prepare the next version. Make bump-x VERSION=x.y.z selects an exact
version. The first release can retain 0.1.0 using bump-x or release-x; equality
is rejected after any version tag or generated release record exists.

Preparation checks the changelog before running make release-verify. That gate
currently includes shell/helper checks, Rust formatting, native compilation,
strict Clippy, docs, native tests, Wasm compilation and package verification.
It is scaffold validation, not service qualification. Install the pinned Rust
toolchain, rustfmt, Clippy, wasm32-unknown-unknown target, ShellCheck, Perl (with
core JSON::PP and Digest::SHA), ripgrep, Bash, flock, Git and Make beforehand.
Library validation uses offline Cargo commands; future dependency changes
must populate the local Cargo cache before release.

After validation, preparation updates Cargo.toml and Cargo.lock, finalizes
CHANGELOG.md and writes docs/release.json with the source commit, release
version/date and release-file hashes. Failures restore the original files.
It never stages, commits or publishes. Review the resulting diff.

## Maintainer one-shot releases

From committed, clean main with the intended origin configured:

- make release-patch
- make release-minor
- make release-major
- make release-x VERSION=x.y.z

Each runs preparation, exact release-file staging, a release commit, an
annotated version tag, an atomic push of main and that tag, and cargo clean.
These are human-operated commands: agents must not create commits even
indirectly. Creating these commands is not permission to run them.

The individual steps are make release-stage, make release-commit and
make release-push. The current branch must be main for pushing; no force push
or unrelated tag push occurs. Remote non-fast-forward/tag conflicts fail
rather than overwrite remote state.

If preparation fails, correct the cause and retry it. If staging or commit
fails, retain the prepared files and resume the individual step; do not bump
again. If commit succeeds but tag creation fails, inspect HEAD and the
generated record, then have the maintainer create the exact annotated tag
before make release-push. If atomic push fails, retain the local commit/tag,
resolve the remote conflict without force and retry release-push; run make
clean after the successful retry. Never rerun the entire one-shot command
merely to recover a failed push.

## Registry publication

Make publish-dry-run checks a registry upload without publishing; make publish
uploads to crates.io. Both require the current release tag at clean HEAD and
the generated release record. Neither bypasses Cargo's publish=false.
Publication needs its own explicit maintainer instruction and registry access.

Before enabling publication, assign the service/package owner, registry and
remote, close the B1 contract, complete service evidence and extend the release
gate to cover its real guarantees. See [development governance](governance/development.md)
and [the service contract](service-contract.md).
