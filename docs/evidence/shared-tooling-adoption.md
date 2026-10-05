<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Shared Tooling adoption evidence

Date: 2026-10-05. This records a completed source review and local tooling
adoption, not an issue tracker. GitHub issues in their owning repositories track
future work. Historical release notes retain their original meaning.

## Source identities

The consumer starts at release 0.14.9,
`c3a16529d161547987c138b15bf75cac70b54c55`, with validated source
`760568efd3ad42b0fb5d39ebb554aec306fb883f`. Its local annotated tag, Cargo
version and receipt agree; the maintainer reports it pushed. This review does
not establish crates.io publication or service deployment.

The adopted Shared Tooling source is committed revision
`e16c9c99bd800567189c8024eaf4242a5d1c9e29`, selected while the checkout was clean. The
[snapshot manifest](../../.shared-tooling.snapshot) binds its remote, revision,
eleven declared files, SHA-256 hashes and executable modes. The source-side
refresh exports committed Git blobs; this consumer's builds and checks do not
read the sibling checkout. Shared Tooling's own AGENTS.md was not copied.
At delivery the sibling has subsequent uncommitted baseline/verifier/test changes;
those bytes are not part of this snapshot and were not changed by this task.

The earlier 2026-10-04 review used base
`956236a3848c2cfae6ae05f5c77e9c37b01b3366` and dirty upstream AGENTS hashes
`7d9a581429414d05711c4bb647dfc9a26557bfe09e515b821cec501babb6fb1e` and
`f3d549197a3f8bde1140b38a04a10412e20a117ddef54e9410f060cbad8aef87`.
They were working-file observations, not bytes of that committed revision.
Its [retained review](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/governance/shared-tooling-feedback.md)
records funding-encoder propagation, exact media artifact identity and offline
cache evidence. The local feedback queue and
[old integration queue](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md)
are removed from the maintained tree rather than retained as alternate trackers.

## Maintained ownership

[DRAGGINZGAME.md](../../DRAGGINZGAME.md) owns common engineering practice and
links its vendored guides. [AGENTS.md](../../AGENTS.md) is the local overlay for
service ownership, frozen-contract recovery and exact commands. No new exception
to common rules is introduced. Provider probe records still own experimental
requests, effects and limitations; they do not become issue trackers.

The LOC tool discovers Cargo workspace members. Make supplies an explicit
repository root and `CARGO_NET_OFFLINE=true`, leaving the shared script unchanged.
The report classifies Rust files by path; runtime-file LOC can include inline
tests and test-canister code. Test-attribute totals are lexical counts, not a
count of executed test cases. This report makes no runtime or binary-size claim.

## Focused verification

Commands and outputs are retained under `.tmp/shared-tooling-adoption-01`:

- `make shared-tooling-check`: all eleven declared files verify offline.
- `make cloc`: all eleven workspace members occur once, including canister and
  tests/ members. Report totals agree with individual rows; Cargo.lock remains
  unchanged. Discovery/counting performs no compilation or dependency fetching.
- Shell syntax, ShellCheck and documentation-link checks cover the adopted
  scripts and local instruction links. A deliberately altered isolated snapshot
  is rejected; the real snapshot remains unchanged.

These are local Linux observations. macOS support is required by the baseline,
but native macOS builds, operator workflows and platform prerequisites are not
qualified by this run. Full CI, Rust builds, paid/provider calls, deployment,
commits and sibling edits were not performed. Retained effect records and build
artifacts were not cleared.
