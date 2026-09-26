# Releasing

The command family follows the sibling ic-timers/ic-memory libraries.
The package permits publication to crates.io. Its native content and billing
primitives do not yet qualify the storage service. The commands prepare and
record repository releases independently of registry publication.

## Preview and prepare

Use make release-plan VERSION=patch (or minor, major, x.y.z) for an effect-free
preview. Make patch, make minor and make major validate committed clean source,
then prepare the next version. Make bump-x VERSION=x.y.z selects an exact
version. The first release can retain 0.1.0 using bump-x or release-x; equality
is rejected after any version tag or generated release record exists.

The initial 0.1.0 changelog is undated history. Preparation preserves undated
entries at or below the current package version; any other undated future
version remains a competing draft. A named target must follow empty Unreleased.
Drafting notes does not bump Cargo or create a release receipt: commit the
implementation and draft first, then run `make patch` or the exact-version
preparation command from clean source. Agents can prepare the draft and validate
it; commits remain human-only.

Preparation checks the changelog before running make release-verify. That gate
currently includes shell/helper checks, Rust formatting, native compilation,
strict Clippy, docs, native and local PocketIC tests, Wasm compilation and package verification.
It validates tooling, native primitives and test-fixture composition, not service
qualification. PocketIC requires the explicitly provisioned server and local
loopback access; see [dependency setup](dependencies.md). Install the pinned Rust
toolchain, rustfmt, Clippy, wasm32-unknown-unknown target, ShellCheck, Perl (with
core JSON::PP and Digest::SHA), ripgrep, Bash, flock, Git and Make beforehand.
Library validation uses offline Cargo commands; future dependency changes
must populate the local Cargo cache before release.

`make release-check` tests the release helpers using isolated fixtures and
substituted Git/Cargo/validation commands. Successful runs print a fixture
notice and one summary; simulated version changes are captured, not printed as
real release progress. On failure, the runner identifies the case, shows its
diagnostics and command trace, and retains the fixture/logs under `target/` at
the printed path. Successful runs remove only their own temporary fixtures.
The initial-version and imported-history cases exercise supported release
behavior; their fixture versions are independent of this repository's version.

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
annotated version tag, and an atomic push of main and that tag. Build artifacts
are retained after success, failure and retry. `make clean` is an explicit,
separate cleanup command; release and publication never invoke it.
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
resolve the remote conflict without force and retry release-push. Never rerun the entire one-shot command
merely to recover a failed push.

## Registry publication

Make publish-dry-run checks a registry upload without publishing; make publish
uploads to crates.io. Both require the current release tag at clean HEAD and
the generated release record. Cargo checks registry eligibility and credentials.
Publication needs its own explicit maintainer instruction and registry access;
it is not part of `make release-patch` and does not clean build artifacts.

The maintainer removed the B1 ownership/readiness publication gate. Publishing
the current library does not imply that provider integration, service acceptance
or Canic replacement is complete; those are tracked in the
[service contract](service-contract.md).
