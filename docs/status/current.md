# Current status — 2026-10-26

Released baseline: `c4fe55dd` (0.47.1). Pending notes target 0.48.0 for the approved
browser peer hard cut to ICP SDK 6.1.1, alongside CLI recovery cleanup and Rust
dependency adoption. Public Rust APIs, endpoint bytes, saved formats and application
retry policy remain unchanged. Blob package versions remain unchanged.
Prior completed work and evidence are retained in the 0.47.1 Git history and
[earlier handoff history](history.md).

## Current changes

- Align minimums with the incoming lockfile: Host/Build/Testkit 0.68.4,
  Platform 0.6.17, Wire 0.7.12, Memory/Storage 0.51.2 and Metrics 0.16.2.
  All direct registry dependencies were checked against published non-yanked
  crates.io releases on 2026-10-25. Also align libc 0.2.191, Serde JSON 1.0.154,
  SHA-2 0.11.1 and Tokio 1.54.1. PocketIC requires exact thiserror 2.0.18, so
  2.0.21 cannot be selected in this graph.
- Connect Wire's required bulk-advice handler to stderr; generated source stays
  on stdout, with no suppressions or changes to endpoint schemas. Browser tests
  select Playwright 1.64.1 and the approved ICP SDK 6.1.1 peer. An explicit npm
  override selects SDK 6.1.1 for Caffeine 1.1.2 too; `npm ls` verifies one identity.
  Caffeine still declares `^5.3.0`, so this is a consumer-qualified integration,
  not upstream support. The pinned Caffeine source/patch remains unchanged.
- Browser Platform adoption is tracked by [Blob #103](https://github.com/dragginzgame/ic-blob-storage/issues/103)
  with supply/SDK alignment at [Platform #57](https://github.com/dragginzgame/ic-platform/issues/57).
  Public npm readback still returns E404 on 2026-10-26; the local candidate now
  selects SDK 6.1.1, resolving the earlier SDK-5 mismatch. Retain the direct peer
  until published supply and consumer acceptance cover that graph. Sibling
  checkouts remain read-only.
- Browser prerequisites and bundle generation now share peer admission, reject
  duplicate/aliased locked SDK packages and require Blob/Caffeine to resolve the
  same installed Agent. The existing hosted Wire-schema job runs that prerequisite.
  The post-migration direct-import ban remains part of #103's acceptance.
- Setup transport derives its method, service and namespace from the admitted
  request. Unvalidated CLI inputs remain at request admission; query and mutation
  dispatch no longer receive a second source of operation metadata.
- Publication recovery accepts only admission and preparation steps. Permission
  inspection, manifest inspection and revocation cannot name a preparation claim.
- Transfer validation opens the original directory once for batch bindings,
  both signed-attempt validations and the final summary. Initial setup recovery
  also shares its handle between binding and admission checks. Individual packet
  reads remain bounded, confined and no-follow; no path is silently canonicalized.
- The directory-replacement regression verifies original bindings, both nested
  attempt packet paths and the summary remain on the held directory. This is
  directory identity, not a snapshot or a promise against in-place file mutation.
- Exact signed envelopes, actor/service/namespace bindings and original request
  bytes remain mandatory. Historical claims never authorize resubmission.
- Live session frames and retained request frames share one byte ceiling;
  recovery uses the same timeout maximum as CLI admission. Values are unchanged.
- Browser selection uses the existing bounded no-follow reader instead of a
  separate symlink check followed by an ordinary open. This protects the final
  component; parent-directory custody and in-place mutation remain caller-owned.
- Session intents share their unchanged 16 KiB ceiling between writing and
  recovery. Saved formats and failure classifications remain unchanged.

Shared's generic checksum/archive test roots and dispatches are retired through
the canonical exporter at the unchanged 0.26.14 snapshot. Local evidence and
recovery checks, product gates and native-host CI remain selected.
Focused snapshot, local evidence/caller and ShellCheck checks pass on Linux;
native macOS qualification remains with the unchanged hosted gates.

## Retained boundaries

- The wider asynchronous preparation workflow still makes fresh service
  observations and separately admits retained claims; this change does not
  introduce a filesystem lock or authorize replay of an uncertain update.
- Public upload-source errors and private fault-injection probe adapters remain
  as reviewed in 0.47.1. No consumer hard cut or catalog override framework.
- Independent heap models, frozen Candid rejection fixtures and distinct cursor
  formats remain useful coverage or supported contracts. No sibling edits.

## Validation

Platform 0.6.17 adoption passes workspace checks/strict Clippy, 197 focused
contract/client tests, native/Wasm Rust 1.91 checks, a standalone release Wasm
build and the 27-method/137-frame generated-client suite. Dependency pins,
Platform/deployment boundaries, formatting and documentation links pass.
Published 0.6.17 library/macro source trees are byte-identical to 0.6.16; feature
configuration differs only in coordinated package/macro version references.
The unchanged incoming lock also selects Tooling/Testkit 0.68.5,
Memory/Storage 0.51.3 and Wire 0.7.15; those selections are preserved. Logs:
`/tmp/blob-platform0617-*.log`. No installed runtime, macOS, live-provider or
full-CI rerun is claimed for this packaging-only Rust update. The browser
package remains unavailable from public npm; #103 is not completed.

The #103 prerequisite follow-up passes real browser admission and bundle builds;
isolated nested, aliased and unlocked installed SDK copies are rejected by the
production checker. Evidence: `/tmp/blob-platform103-rejection/` and
`/tmp/blob-platform103-build.log`. Formatting, documentation links and changed
shell syntax/lint checks pass. Runtime suites were not rerun for these build-only
checks; prior SDK integration evidence below retains its scope.

Earlier dependency refresh: workspace all-target/all-feature checks and strict
Clippy, 709 native tests, native/Wasm Rust 1.91 checks, fresh standalone release
Wasm and CLI builds pass. Generated TypeScript passes the 27-method/137-frame
checks with Wire's diagnostic handler connected. Dependency pins, Platform and
deployment boundaries, formatting and documentation links pass.

SDK 6.1.1 with Playwright 1.64.1 passes all four installed Chromium/PocketIC
standalone tests: upload/download/release, lost transfer replies, corrupt provider
downloads and independent users. Browser control, IndexedDB storage/restart and
publication frozen-input/budget/mutation tests also pass. These use local provider
substitutes; no deployed Caffeine, billing, macOS or full-CI qualification is claimed.
The npm graph has one SDK 6.1.1 identity, including the Caffeine override. npm's
incremental install retained an invalid nested SDK 5 entry; regenerating the lock
from exact direct pins in `/tmp/blob-sdk6-resolve` and running `npm ci` resolved it.
The incoming Cargo.lock remained unchanged. Logs: `/tmp/blob-deps-refresh0472-*.log`;
publication evidence: `/tmp/blob-sdk611-publication`. The log prefix predates the
approved browser peer cut and the pending changelog move to 0.48.0.

The browser-reader/intent-limit follow-up passes eight native session tests,
including valid/dangling symlink rejection, missing-file and oversize failures,
and unchanged ordinary-file admission. Strict all-target/all-feature CLI Clippy,
formatting and documentation links pass; logs: `/tmp/blob-audit0472-*.log`.
Installed tests were not rerun for this follow-up; the preceding graph adoption
evidence below does not qualify the new filesystem-reader change on macOS.

The earlier 0.68.3/0.7.10 graph passes workspace all-target/all-feature checks and strict
Clippy, all 709 native tests, and Rust 1.91 native/Wasm checks. Generated
TypeScript passes 27-method/137-frozen-frame qualification, including malformed,
identity, direction and limit refusals. A fresh standalone release Wasm and CLI
pass four installed ingress tests and seven publication-session tests using
Testkit 0.68.3. Testkit setup reused the admitted executable after a bounded wait
for another consumer's installation; no lock was removed or bypassed.
Dependency pins, Platform boundaries, deployment exclusions, formatting and
documentation links pass. The incoming lockfile remained byte-for-byte unchanged
during this adoption. Logs: `/tmp/blob-deps0472-*.log`. This is Linux/PocketIC
qualification, not native macOS, full CI or live-provider evidence.

Before dependency adoption, CLI all-target/all-feature checks and strict Clippy
passed. All 124 native CLI tests
and seven installed preparation/recovery tests pass, including lost replies,
tampered journals and no resubmission. Rust 1.91 checks, a fresh standalone
release Wasm build, formatting and documentation links passed. Those runs used
the earlier dependency graph; full CI and native macOS were not rerun.
The shared-limit follow-up passes eight native session tests and all seven
installed publication-session tests, covering transfer handoff, control loss,
tampering, partial sources and idle shutdown. Strict CLI Clippy, formatting and
documentation links pass; logs: `/tmp/blob-session0472-*.log`.
Earlier cleanup evidence: `/tmp/blob-cleanup0472-*.log`. 0.47.1 qualification remains in
`/tmp/blob-cleanup047-*.log`; do not treat it as qualification of new changes.
No full CI, native macOS, live-provider or production-capacity qualification is
claimed. No staging, commit, push, release or deployment was performed.
