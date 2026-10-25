# Current status — 2026-10-25

Released baseline: `f04c12dd` (0.47.0). Pending notes target 0.47.1: internal
cleanup and command help, with unchanged public Rust APIs, endpoint bytes,
saved formats and retry behavior. Package versions remain untouched. The incoming
Cargo.lock update is preserved; this batch does not change dependencies.
Prior completed cleanup and evidence are retained in the 0.47.0 Git history and
[earlier handoff history](history.md).

## Completed local cleanup

- Admitted CLI setup operations carry the matching payload and required mutation
  journal destination together. Inspections cannot be dispatched as mutations;
  CLI parsing still owns unvalidated input and exact role checks precede effects.
- Recovery packet validation opens its directory once and reads all packets
  through that held handle. Recovery roots retain their no-symlink admission;
  input roots keep their separately documented canonicalization behavior.
- Frozen batch summaries share typed recovery-binding fields between writing
  and reading. Other observational JSON fields remain non-authoritative; the
  original packet hashes and body verification are still required.
- Preparation ceilings and CLI upload limits have named owners. Saved-document
  and service-reply limits remain distinct; no limit was increased.
- Command-specific help replaces the global workflow narrative. The operator
  guide retains detailed recovery procedures and original-journal obligations.
- Decode-only canister adapters no longer require Serialize. Capacity errors
  retain the same public classification with mutation-only failures mapped to
  Internal, without maintaining a second exhaustive list of storage variants.

## Investigated and retained

- Public upload-source traits retain UploadStoreError. Narrowing those signatures
  would require a consumer hard cut and extra conversion types for little gain;
  the local capacity translation was simplified instead.
- The storage probe retains its private endpoint emitter and fault-injection
  workflows. The canonical service catalog emits complete production workflow
  calls; sharing it here would need selection/override machinery. Existing probe
  ingress already uses Contracts' method identities, limits and codecs.
- Independent heap models and frozen Candid rejection fixtures remain useful
  test evidence. Distinct cursor formats and exact same-release recovery journals
  are unchanged. No sibling files, public compatibility shims or new framework.

## Validation

Focused all-target/all-feature checks and strict Clippy pass for the CLI, core,
standalone and edited probe adapters. All 124 CLI tests, 32 capacity tests and
seven installed preparation/recovery tests pass. New regressions cover missing
journal destinations, malformed frozen-summary bindings and replacement of an
opened recovery-directory path. Lost-reply recovery still never resubmits.
Rust 1.91 checks for CLI/core and release Wasm builds for the standalone and
three edited probes pass. Formatting, documentation links and dependency/
deployment boundaries pass. Package-verification has no selected unit tests;
its compilation is covered by the all-feature CLI checks.
Evidence: `/tmp/blob-cleanup047-*.log`. The starting dirty tree is captured at
`/tmp/blob-cleanup047-before/`; the incoming lockfile must remain byte-identical.
No full CI, native macOS, live-provider or production-capacity qualification is
claimed. No staging, commit, push, release or deployment was performed.
