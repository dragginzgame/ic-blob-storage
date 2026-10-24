# Current status — 2026-10-24

## Released baseline and pending adoption

Released baseline is `cca4571` (0.44.0); pending adoption source is `64aeb49`. The architectural cleanup, native Clap
adoption and Testkit fixture repair are in that source. Its committed handoff
retains previous qualification; those results are not new evidence for this graph.

Pending **0.44.1** adopts published IC Tooling 0.67.0 across Build, Host artifacts,
FS, HTTP, Process, Tools and Testkit. The archive verifier now passes explicit
`CargoFeatureSelection::default()`; archive packaging, metadata and tests preserve
its former default-feature policy. No Blob public protocol, persistence or runtime
contract changes are introduced. Cargo package versions remain 0.44.0.

Shared Tooling is pinned to `4c0f198e56d4c6d27b12a5c2d515e0e01070151e`, verified
as the latest pushed HEAD during adoption. That commit's `VERSION` is **0.26.10**,
although its title/changelog call the setup-timing change 0.26.11. The snapshot
retains the canonical committed annotation; the dirty sibling VERSION was not used.
The exporter ran from an isolated clean clone, preserving the sibling checkout.

The adopted snapshot includes Cargo setup phase timing, offline aggregate tool
reports, literal formatting catalogs and common external-IC dependency inventory.
Blob enables `--external-ic` alongside its stricter local policy and includes the
inventory in frozen CI inputs. Deployment graph exclusions remain independent.
The inventory is a reviewed list, not proof of complete external-package discovery.
Optional hosted Cargo-tool caching was not enabled by this adoption.

## Dependency graph and qualification

The reviewed graph selects Platform 0.6.14, Memory/Storage 0.50.16, Metrics 0.15.8,
Wire 0.6.12 and Tooling 0.67.0. Manifest minimums now match these published releases.
The incoming lockfile, including crypto-common/generic-array selections, is preserved
byte-for-byte. No source overrides or unpublished sibling packages are used.
Registry checks outside this workspace confirmed Wire 0.6.13 is not yet published.
Shared Tooling and IC Tooling remote HEADs remain at the prior selected revisions.

Published Memory/Storage source trees are identical to 0.50.15; Metrics source is
identical to 0.15.7. Platform's source changes add signature-verification evidence
that Blob does not enable; used runtime and codec source paths are unchanged.
No caller rewrite, protocol change or persisted-format migration is needed.

Current before-images: `/tmp/blob-upstream-before/`; evidence:
`/tmp/blob-upstream-*.log`. Strict Clippy passes for the affected Core, Contracts,
Client, CLI, storage probe and PocketIC targets/features. Memory composition,
installation, stored-record and Wire-boundary tests pass, as do native signed-update
and HTTP failure-handling tests. Fresh release standalone Wasm and a rebuilt CLI
pass both installed endpoint-surface and installation/service-binding tests.
Final installed proof is `/tmp/blob-upstream-installed-fresh.log`; the earlier
installed run preceded the explicit CLI rebuild and is not final graph proof.
Rust 1.91 checks pass for affected public libraries, CLI and storage probe, across
all their targets/features. Snapshot, pins, declaration/deployment boundaries,
formatting and documentation links pass.

Previous Tooling-specific qualification remains in `/tmp/blob-tooling-*.log`:
real archive replacement/drift regressions, Shared consumer fixtures, hooks,
substituted release routing and offline tool admission passed. No Tooling or
Shared implementation changed in this follow-up; those checks were not repeated.
The common external-IC inventory also has retained negative alias/optional/dev
admission evidence. Setup uses the published Testkit and Build editor CLIs.

The current lock SHA-256 is
`ed4f3c88b52050dbed1177100bd859c6731d4bf7a083e83e690c203f1e71d26d`.
Full release CI, full PocketIC suites, optimizer qualification, native macOS,
current-feed security audit and deployed provider checks are not claimed.
No staging, commit, push, release, publication, sibling edit or paid effect occurred.

## Ownership and outstanding acceptance

Blob owns service authority, storage, references, quotas, funding/GC journals and
recovery. Canic owns framework adapters, lifecycle and installed composition.
The last read-only Canic review selected Blob 0.43.4 and reported dedicated/embedded
and accepted-payment/recovery tests; adoption was not refreshed in this task.
Canic #613 tracks build/schema delivery, #622 application delivery, and #494
independent credit qualification. They do not indicate missing ordinary Blob funding APIs.

Blob #4/#5/#6/#31/#34 retain actual application/provider acceptance: publication,
serving, multi-user uploads, payer linkage/activity, deletion and billing cessation.
[Funding qualification](../funding-consumer-qualification.md) and
[service gaps](../service-gaps.md) distinguish these from local fixtures.

[Blob #102](https://github.com/dragginzgame/ic-blob-storage/issues/102) tracks the
native Agent crypto-bigint advisory; [Platform #50](https://github.com/dragginzgame/ic-platform/issues/50)
owns the supplier correction. Normal standalone deployment excludes that graph.
No current-feed security audit, exemption or correction is claimed by this adoption.
