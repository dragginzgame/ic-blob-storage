# Current status — 2026-10-24

## Released baseline and pending adoption

Released baseline is `cca4571` (0.44.0). The architectural cleanup, native Clap
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

The incoming Memory/Storage 0.50.15 lock changes are preserved. Platform remains
0.6.13, Wire 0.6.12 and Metrics 0.15.7. Only the seven Tooling package identities
changed against this task's initial graph; no path overrides or sibling dependencies
were added. Manifest/lock selections remain authoritative.

Before-images: `/tmp/blob-tooling-before/`; evidence: `/tmp/blob-tooling-*.log`,
`/tmp/blob-shared-{impact,adoption}.log`. Published archive APIs were inspected
from Cargo's registry source; repository guidance was checked against them.

Strict CLI/PocketIC all-target/all-feature Clippy passes. Native CLI qualification
passes 114 Blob tests, 7 probe tests and 1 native-file test; the PocketIC support
library passes 25 cases. Fresh standalone release Wasm and both focused installed
installation/endpoint-surface checks pass using Testkit 0.67.0. The archive verifier
passes real same-version replacement plus graph and final drift refusal fixtures.

Snapshot integrity, dependency pins, common/local boundaries, deployment exclusions,
formatting, consumer Shared fixtures, hooks and substituted release routing pass.
An isolated real-metadata probe rejects renamed optional and target-specific dev
IC dependencies. Lock-selected Testkit and Build editor setup succeeds; `tools-report`
admits Host, Rust, IC, Testkit and release-editor tools offline. Setup timing remains
on stderr while the adapter's stdout retains only the selected path.

Rust 1.91 CLI/PocketIC all-target/all-feature checks and final documentation links
pass. The final lock SHA-256 is
`bf2acaa4480cbda8056b00a73777e6b5c9805edf06ab117fd130558759f5a888`.
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
