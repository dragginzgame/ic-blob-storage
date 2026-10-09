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
rustfmt, ShellCheck, Perl with JSON::PP/Digest::SHA, Git, curl, GNU Make and Bash 3.2 or
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

The released 0.20.0 developer-tool hard cut requires explicit `make install-tools`
even in an existing checkout: the shared bundle now has five tools and Testkit
owns PocketIC. Old bundles are retained, and validation never converts them or
downloads a server. `make tools-check` includes the lock-selected Testkit CLI and
its authenticated offline server check. See the [adoption record](evidence/pocketic-handoff0200.md).

For the released 0.21.0 hard cut, run `make install-testkit` explicitly after the
lock changes to Testkit 0.27, then `make testkit-check`. The existing authenticated PocketIC
16.1.0 server remains selected; prior CLI installations and receipts are retained.
Memory 0.33 changes the public re-export's type identity, requiring aligned
consumer dependencies and rebuilds. Runtime schemas are unchanged; cross-release
installation retirement/reinstall requirements still apply.

That release changes capacity reply/preflight shapes: rebuild consumers for
the three mandatory byte headrooms and dimension-specific blocker names. The
usable minimum and stored schemas remain unchanged. Shared Tooling's upstream
exporter fixture is omitted from the 92-file snapshot and consumer gate; actual
snapshot verification and native adoption acceptance remain required.

Pending 0.21.1 selects Memory 0.33.1 and private Metrics 0.3.2 with unchanged
runtime sources and dependency edges; focused qualification is recorded in
[the patch record](evidence/libraries0211.md).
The same pending batch adopts Shared Tooling 0.2.5's hook path/read fixes;
formatter and installation qualification is separate in
[the Shared record](evidence/shared0250211.md).
The later incoming Testkit 0.27.1 patch also has unchanged runtime sources.
Run `make install-testkit` explicitly for its lock-selected CLI, then
`make testkit-check`; the authenticated server and old CLI slots remain.
[Its separate record](evidence/testkit0271-0211.md) owns final-graph qualification.
These patches introduce no public contract change.

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

The released identity and validated source are recorded in
[the release receipt](release.json); root Cargo.toml owns the workspace version.
Ordinary development preserves those released identities. The latest finalized
entry in [the changelog](../CHANGELOG.md) describes the released batch; a single
undated entry records subsequent meaningful work when it starts.
[The current handoff](status/current.md) records acceptance status, compatibility
and the selected dependency graph. Choose patch or minor for the complete batch
under the pre-1.0 rule. Changing the public Memory crate line requires a minor
release, and host compositions must align its type identity and rebuild.

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

Deliver ordinary changes through the [shared PR workflow](../rules/contributions.md)
when requested, including its scoped commits and topic-branch push. Release
requires committed clean implementation and notes. Under this repository's
maintainer-operated boundary in [AGENTS.md](../AGENTS.md#delivery-and-release),
agents do not invoke the one-shot commands below. The explicit metadata/check
adapters are available for inspection or separately authorized preparation;
do not bypass the runner's saved intent or reuse validation from another source.

## Maintainer release and recovery

From committed clean source on the selected branch:

```bash
make release-patch # Pending compatible 0.21.1 maintenance.
# Or select release-minor / release-major for the complete batch's compatibility.
```

All three invoke the same pinned runner. This repository explicitly selects
`RELEASE_DELIVERY=direct`; a different command-line or inherited selection refuses
before entering the runner, validation or metadata preparation. The shared PR
helper is included with the baseline, but Blob has not adopted merged-source
receipt adapters. Default destination inputs are `RELEASE_REMOTE=origin`
and `RELEASE_BRANCH=main`; an explicit override is saved with the exact push URL.
It preflights, validates, prepares Cargo/version/notes/receipt, stages only
Cargo.toml, Cargo.lock, CHANGELOG.md and docs/release.json, creates the release
commit and annotated tag, then atomically pushes exactly that branch and tag.
`--no-follow-tags` prevents configuration from adding other annotated tags.
The runner rechecks the saved destination after validation and before push,
and dispatches to the captured URL with an explicit option terminator.
Publication, deployment and cleanup are separate.
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
verifies the shared snapshot, local executables and dependency declarations,
checks local documentation links and shared digest/installer fixtures,
fetches the selected lock with `make deps`, then
runs shell/helper checks, hook regressions, manifest/Rust formatting, compilation,
Clippy, retained-probe checks, docs, native/PocketIC tests, Wasm and packaging.
Prerequisite or fetch failure stops before compilation or version mutation. Offline
checks never select newer dependencies. The metadata transaction updates only
the inherited workspace version, version-qualified local catalog requirements
and local version-qualified references in Cargo.lock, then
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

Publication remains an explicit maintainer action after the clean release,
receipt, exact parent and annotated tag checks:

```sh
make publish
```

The default calls standard Cargo for contracts first, then core, under one release
lock. Cargo owns uploading and waiting for index visibility. A failed contracts
publication stops before core. Retry the same command after investigating any
failure: an exact version already on crates.io is skipped only after its archive
checksum and Git source match the selected release. Failed readback, yanked rows,
malformed responses or a different source refuse instead of authorizing an upload.
Readback inputs and a fresh Cargo target remain under the printed
`target/publication.*` path, preventing stale same-version verification artifacts. No local
publication progress journal or additional polling loop is introduced.
API, archive and dry-run dependency reads identify this application's version
and repository through User-Agent, with implicit curl configuration disabled.
A 403 remains an inconclusive fatal response; it never authorizes uploading.

`PUBLISH_PACKAGE=ic-blob-storage-contracts` or `PUBLISH_PACKAGE=ic-blob-storage`
selects one package when needed; it is optional. `make publish-dry-run` never
uploads and still runs Cargo verification for already-published packages. Before
contracts is published, the default dry-run verifies contracts then refuses core
with dependency guidance: a dry-run cannot make that registry dependency exist.
Use `make package` for paired local payload verification, or select contracts for
its individual dry-run. Publishing neither deploys nor qualifies the service.
Older completed releases need no runner plan for this separate publication check.

### Finish the already-tagged 0.18.3 publication

The released helper's default curl identity can receive a 403 before Cargo runs.
The [User-Agent repair record](evidence/publication-user-agent0184.md) retains
the same-endpoint 403/404 comparison and a prepared clean 0.18.3 checkout with
an external curl identity wrapper. The wrapper changes only read transport;
the released helper still checks its exact source, receipt, tag and registry
archives, and Cargo owns uploads. Use a clean tagged payload for this operation;
do not alter its receipt/tag or publish the dirty repair checkout as 0.18.3.
No upload was performed during preparation.

### Finish the already-tagged 0.18.0 publication

The helper repair belongs to the subsequent compatible batch. Keep the exact
0.18.0 receipt/tag and publish its payload from a clean checkout using its released
helper, which still requires individual selections:

```sh
git worktree add --detach /tmp/ic-blob-storage-publish-0.18.0 v0.18.0
make -C /tmp/ic-blob-storage-publish-0.18.0 publish PUBLISH_PACKAGE=ic-blob-storage-contracts
# Once that exact version is available from crates.io:
make -C /tmp/ic-blob-storage-publish-0.18.0 publish PUBLISH_PACKAGE=ic-blob-storage
```

The reported missing-selector refusal made no registry upload. If a later attempt
has an uncertain reply, inspect the exact crates.io version before resubmitting;
the 0.18.0 helper has no automatic readback. A dirty repair checkout or a new fix
commit is not the tagged payload. Do not bypass the clean-source checks or alter
the old receipt/tag just to publish it.

`make package` assembles both `.crate` archives with Cargo, then runs the full
library tests/examples against their exact extracted, normalized payloads in a
fresh retained workspace with its own Cargo target directory. A real Cargo fixture
checks that a second archive with the same package version executes the updated
contract, rather than reusing stale artifacts with fixed archive timestamps. Its local contracts patch points only at that extracted
payload. Cargo resolves its lock before compilation; the bounded metadata adapter
rejects any external version/source/checksum absent from the frozen root lock.
The root lock is unchanged. This avoids Cargo’s
[local-registry checksum failure](https://github.com/rust-lang/cargo/issues/14396)
without dropping package-content verification. Package assembly needs Cargo 1.90
or newer ([stabilization record](https://blog.rust-lang.org/inside-rust/2025/10/01/this-development-cycle-in-cargo-1.90/));
the supported library/compiler floor remains Rust 1.88.

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
