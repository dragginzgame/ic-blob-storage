# Current status — 2026-10-24

## Released baseline and pending cleanup

Released HEAD is `dcbb881` (0.43.4). Blob #100/#101 are closed; their published
schema and descriptor APIs are adopted in Canic's working tree. The
[previous handoff](archive/2026-10-24-pre-cleanup.md) retains original validation
and historical pending-release descriptions.

Pending **0.44.0** consolidates method names/IDs/budgets in the endpoint catalog,
uses canonical tenant enrollment/failure DTOs inside borrowed Wire projections,
and removes forwarding-only capacity/discovery/manifest workflow layers. The
removed public `wire::tenant::{Enrollment, Failure}` projections require consumers
to use `dto::tenant::{TenantEnrollment, TenantFailure}`. Endpoint identities,
protocol bytes and persisted records remain unchanged; normal cross-release
retirement/reinstall policy still applies.

Tenant scope checks now share a pure ordered authority predicate. Synchronous
service access observes continuity and performs work within one host borrow.
Private probe journals share bounded persistence plumbing with original record
codecs, keys and quotas. Production feature checks reject test-support access.
Provider/acceptance documentation describes the released trust contract and
retains superseded investigations as historical evidence.

The second cleanup pass isolates `test-standalone` builds, rejects private fixture
packages in deployment graphs, and applies catalog budgets to generic codec helpers.
It retires the unused public Candid configuration/operator-amount input API in favor
of Contracts validation and typed positive amounts. Provider balance decoding remains.
Certificate roots resolve once before exact final exposure checks; test-support
completion returns a marker and callers inspect observations explicitly. Remaining
history/operator orchestration and account checks now share the workflow owner.
Decoder configuration and CLI method names reuse existing canonical definitions.

## Upstream package and shared-tooling adoption

The incoming lock selects Platform 0.6.13, Wire 0.6.12, Host/Build/Testkit 0.66.3,
Memory/Storage 0.50.13 and Metrics 0.15.7. Shared Tooling remains 0.26.8.
This batch preserves incoming dependency selections, adds the existing Memory
edge for optional private probe persistence, and selects native-only Clap 4.6.7
without color/suggestions for the CLI grammar.
Manifests, lockfile and snapshot remain authoritative; no sibling overrides exist.

## Current checks and next work

The third cleanup pass replaces hand-written flag machinery and syntax help with
Clap across Blob, probe and package-verification commands. Domain identity/trust
and budget checks remain in command handlers; Blob JSON version and sanitized
argument refusals remain supported. Reference-status parsing/observation now lives
in its existing workflow, and stale current storage/funding descriptions are removed.
The proposed account split was not applied: operator inspection checks all four
owners' retained-state consistency before and after await. Its small presentation
conversion does not justify a second observation API or weakening those checks.

Third-pass before-images: `/tmp/blob-third-before/`; logs: `/tmp/blob-third-*.log`.
Focused CLI tests pass (114 Blob, 7 probe, 1 native-file); the reference-status
rejection/liveness tests and account authority/restore test pass (3 Core tests).
Native HTTP fixtures required loopback permission; their earlier sandbox failures
were retained and the focused CLI suite rerun successfully. Strict CLI/Core Clippy,
Rust 1.91 CLI all-target/all-feature checks, Core Wasm compilation, manifest/Rust
formatting and dependency/deployment boundaries pass. Only Clap's dependency
closure changed relative to this pass's incoming Platform 0.6.13 graph.

Earlier second-pass evidence under `/tmp/blob-next-*.log` qualified Platform 0.6.12:
699 native tests, 246 PocketIC cases, 137 frozen frames/27 methods, byte-identical
schema JSON, and workspace/standalone MSRV checks. Those runtime results are not
new qualification of Platform 0.6.13. Fifteen separately selected cases were ignored.

Full release CI, installed/optimized-Wasm qualification, native macOS, current-feed
security audit and deployed-provider qualification were not rerun in this pass.
No staging, commit, push, release, publication, sibling edit or paid effect occurred.

## Ownership and outstanding acceptance

Blob owns service authority, storage, references, quotas, funding/GC journals and
recovery. Canic owns framework adapters, lifecycle and installed composition.
The latest read-only Canic review selects Blob 0.43.4 in all three locks and
reports dedicated/embedded and accepted-payment/recovery tests. Its #613 still
needs schema generation in normal builds, release hash propagation and frontend
delivery; #622 needs delivery, and #494 retains positive independent-credit
receipt qualification. These are not missing ordinary Blob funding APIs.

Blob #4/#5/#6/#31/#34 retain actual application/provider acceptance: publication,
serving, multi-user uploads, payer linkage/activity, deletion and billing cessation.
[Funding qualification](../funding-consumer-qualification.md) and
[service gaps](../service-gaps.md) distinguish these from local fixtures.

[Blob #102](https://github.com/dragginzgame/ic-blob-storage/issues/102) tracks the
native Agent crypto-bigint advisory; [Platform #50](https://github.com/dragginzgame/ic-platform/issues/50)
owns the supplier correction. The normal standalone graph excludes it. The
[dependency review](../dependencies.md#open-native-agent-advisory) records the
incoming graph and retained RustSec revision; no exemption or correction is claimed.
