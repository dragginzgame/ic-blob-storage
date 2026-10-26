# Current status — 2026-10-26

Released baseline: `d893da19` (0.48.0). Pending notes target 0.49.0 for the
browser dependency hard cut to published Platform, alongside compatible Tooling
adoption. Blob package versions remain 0.48.0. Public Rust APIs, endpoint bytes,
saved formats and application retry policy remain unchanged. Earlier evidence
is retained in the released commit's handoff and [handoff history](history.md).

## Current changes

- Browser product, test and build imports use published `@tokostudios/ic-platform`
  0.1.0. The private browser client has a regular Platform dependency and retains
  the Caffeine peer; the direct SDK Core peer/dev dependency is removed.
- Platform owns canonical Principal parsing, checked Ed25519/secp256k1 JSON
  restoration and explicit Agent construction. Blob retains its 512-byte signer
  budget, refusal mapping, anonymous-uploader policy, IntentAgent, guarded fetch,
  trust/freshness policy, exact retained envelopes and zero SDK retries.
- SDK Core 6.1.1 remains transitive through Platform and external Caffeine. The
  consumer override is exact `6.1.1`, replacing the former direct-dependency
  reference. The lock and installed graph contain one SDK identity. The Caffeine
  source hashes and reviewed patch are unchanged.
- Browser prerequisites and bundle generation reject first-party direct SDK
  dependencies (normal/dev/peer/optional and aliases), static/dynamic/require/
  re-export imports and inline npm-script imports. Product/test/build sources
  are checked; node_modules and the external Caffeine patch retain their owners.
  Duplicate/aliased locked SDKs and installed Agent shadows also refuse.
- Rust Platform/macros now select published 0.6.18 with existing features.
  Host/Build/Testkit minimums select 0.68.6 from the preceding pending batch;
  the lock retains Memory/Storage 0.51.3, Wire/derive 0.7.15 and Metrics 0.16.2.
  No unrelated Cargo package identity changed during Platform adoption.

## Supply and issue ownership

The supplying npm identity is `@tokostudios/ic-platform`, not the earlier
unpublished `@dragginzgame` proposal. Registry archive:
`https://registry.npmjs.org/@tokostudios/ic-platform/-/ic-platform-0.1.0.tgz`.
Its lockfile integrity, verified against public registry metadata, is
`sha512-i7zwyidmwOXOVxFohbJFPUgNQqO4vYa+BCd9wr7n2OU0A7YNMI5SAkbGPQl714mptYDx/+xmKTvOPfRwnMixnw==`.
No sibling path, local archive or package alias supplies the maintained graph.

[Blob #103](https://github.com/dragginzgame/ic-blob-storage/issues/103) owns
consumer acceptance; the migration is implemented and locally qualified but
awaits default-branch integration before closure. [Platform #57](https://github.com/dragginzgame/ic-platform/issues/57)
owns producer supply. [Canic #637](https://github.com/dragginzgame/canic/issues/637)
retains its own Auth/Core and installed acceptance; sibling checkouts are read-only.

## Validation

Workspace all-target/all-feature checks and strict Clippy pass. Rust 1.91
native/Wasm checks, a fresh standalone release Wasm/CLI and generated TypeScript
qualification (27 methods, 137 frozen frames) pass. Dependency pins and
Platform/deployment exclusions pass. Browser prerequisites pass all 15 test
results, including the positive graph and refusal fixtures. Published-package
bundles, offline publication/worker suites, twelve native pipe tests, IndexedDB
persistence/restart, seventeen Chromium signer/bootstrap cases and twenty-two
launcher/profile cases pass.

All four installed Chromium/PocketIC standalone scenarios pass through published
Platform: upload/download/release, lost replies/withdrawal obligations, corrupt
download refusal and independent users. Browser control tests pass too. These
use local provider substitutes, not deployed Caffeine/billing or real login.
Formatting, documentation links, changed shell syntax and ShellCheck pass.
Logs: `/tmp/blob-platform-npm-*.log`; browser reports: `/tmp/blob-platform010-*`.
Node 24.21.0, npm 12.2.0 and Chromium 156.0.8078.4 were used on Linux. Native
macOS, other engines, full CI and production-capacity qualification were not rerun.

The preceding Tooling-only batch's 125 native CLI and seven installed session
results remain at `/tmp/blob-tooling0686-*.log` with their original graph scope.
No staging, commit, push, release, publication or deployment was performed.
