# Release metadata and publication recovery cleanup

Date: 2026-10-13. Blob base: `7ad76317fb1ed2695c16ede4aabee655ded84187`.
Task input is the existing dirty 0.23.3 batch retained in
`/tmp/blob-third-cleanup-before`. Earlier Candid, digest, body-verifier, CLI-fixture
and Shared adoption changes are preserved, not recounted as this task's edits.
Incoming Host 0.17.4 and Memory 0.36.3 remain registry selections.

## Ownership and compatibility

The shared metadata preparer is added through the canonical exporter from the
already selected Shared 0.11.0 revision
`a15ff323449f4b6190f647aec0de43cbcfb314ba` (116 payloads). Blob no longer edits
workspace/catalog versions in Perl or exposes a second changelog finalization
operation. Shared owns Cargo's coordinated mutation and original-lock rewriting.
Blob keeps read-only changelog admission, source/index checks, all-four-file
rollback and receipt verification. The outer backup is necessary when shared
preparation succeeds but formatting or receipt creation subsequently fails.

`ci/release-tools.env` selects Cargo-edit 0.13.13. The thin setup/check adapter
uses the existing shared Cargo installer and its admitted executable path;
release preparation cannot fall back after failed admission. Explicit release
preflight prepares this tool; ordinary checks never install it. Existing common
catalogs, package versions and Git release effects are unchanged.

Publication-map commands and sessions share operator-versus-tenant query routing,
per-query deadlines and transport errors. Overall query counts, phase state,
credential validation, journaling and completion checks stay with their callers.

Recovery packets use Host's no-follow regular-file opening and bounded read.
Final symlinks (including dangling ones), FIFO inputs, empty files and excess
bytes refuse. Ordinary CLI input reads retain their existing link-following
behavior. Journal ancestor checks and session missing-file classification remain
local; this is not a claim of immutable contents or whole-path race exclusion.
Supported native hosts remain Linux/macOS; unsupported no-follow hosts refuse.

## Evidence and limits

Packet: `.tmp/review-validation/third-cleanup0233/`. Real Cargo metadata trials
operate only in `/tmp/blob-release-metadata-trial` and
`/tmp/blob-third-metadata-{managed,stale,foreign}`. Managed preparation changes
exactly Cargo.toml, Cargo.lock and CHANGELOG.md across all 12 members; parsed
external lock records are identical. Stale local requirements and an unadmitted
foreign path dependency refuse with exact restoration. These are offline Cargo
executions, distinct from the release suite's substitute Git/Cargo effects.

Focused CLI tests and strict Clippy pass on Host 0.17.4. Seven installed session,
four indexed preparation and two publication-map cases pass with the rebuilt CLI
and retained same-source standalone Wasm. They cover actor selection, original
journal recovery, rejected changed input and no redispatch after uncertain replies.
No provider request or downstream Canic acceptance is inferred.

All 128 release-adapter cases pass. The suite retains its direct-delivery/recovery and
publication-refusal matrix, adds missing Cargo-edit admission and absent/existing
receipt rollback cases, and checks the actual new shared preparer with substitutes.
The first suite passed before the added cases. The next run exposed an uncleared
failure-injection variable in its retry test; that log/fixture remains retained.
ShellCheck initially needed the newly sourced pin file included as an explicit
input; the maintained shell gate now checks both. Other logs identify snapshot,
prepared-tool, formatting, documentation, shell and release-routing checks.
No real commit, version bump, tag, push, publication or full Blob CI occurred.
Native macOS and actual hosted release/publication transport remain unqualified
by these local checks. Previous failures and qualification records are retained.
