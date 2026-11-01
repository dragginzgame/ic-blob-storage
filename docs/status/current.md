# Current status — 2026-11-01

## Delivered baseline

Blob **0.62.0** is delivered on main/tag `v0.62.0` at
`13510b0654577eea8e8e527dfe4544d469af330c`; the release receipt is
[docs/release.json](../release.json). Official registry readback confirms all three 0.62.0 library packages are present
and not yanked; this pass did not repeat their release archive provenance checks.
Its [hosted release/native validation](https://github.com/dragginzgame/ic-blob-storage/actions/runs/41288994685)
was still running at this pass's readback; do not report it as successful yet.
No release, publication, deployment or paid effect was executed in this pass.

The released hard cut removed the parallel admission service, historical Candid
projection framework/old-image drivers, unused ledger-notification decoder and
single-source read traits. The admission probe uses the durable owner. Pure
catalog/lifecycle reference models remain for independent authority/recovery
scenarios. Exact retries, retained obligations and same-release fences remain.
Its earlier cleanup evidence is `/tmp/blob-cleanup-1101/`.

## Optional native verified downloads — 0.62.1

The maintainer authorized a small optional helper after reviewing the removed
mandatory completion-verifier flow. The published client now has a non-default
`native-download` feature, available to registry consumers after the pending
0.62.1 publication. No sibling was edited; no package version, Git ref, release,
publication, deployment or provider probe was changed by this implementation.

`native::Verification` binds the caller's trusted original root, exact length,
metadata, optional raw SHA-256 and explicit hashing budgets before I/O. Its
`read_response` consumes one body under caller-owned transport/deadline/private
sink custody; the CLI now uses this shared verification loop with its existing
HTTP evidence, error projection and Run directory publication unchanged.
`native::download` adds a single bounded HTTP request and fresh private directory:
no redirect, automatic retry, URL user-info or decompression; loopback HTTP needs
explicit opt-in and bypasses proxies. Only successful HTTP EOF, length/root and
optional checksum checks permit Host FS to sync and create-only publish body.bin.
Failure/cancellation retains body.part and the claimed directory. Host publication
errors preserve before/after-publication evidence; failed runs are never silently
reused. Transport buffers and synchronous hashing/filesystem work retain the
limits described by the API; async deadlines are not hard OS preemption.

No verifier identity, attestation, receipt schema, background worker or publication
policy was added. For IC Crates, use Some(expected Cargo archive SHA-256) and trusted
retained manifest metadata; do not compare Cargo's raw checksum to the provider
root or treat local upload verification as a provider read. The helper makes no IC
call and needs no tenant signer, so the native publisher need not impersonate its
registry canister. The application retains its exact operation intent and the
controller-signed catalog transaction; the registry retains current permission/
live-reference checks. This is trusted-publisher verification, not independent
canister-enforced attestation or a guarantee of future storage/availability.

Native HTTP/filesystem dependencies are optional and target-specific. Default
client graphs and all-feature Wasm clients exclude them. The retained local
boundary checker now rejects native dependencies in both selections and in the
standalone deployment. Fixtures exercise attempted dependency leaks. No new
package/version was resolved: existing upstream selections and unrelated pending
adoption edits are preserved; the lock adds only direct client/CLI dependency edges.

Evidence: `/tmp/blob-native-helper-1101/`. Strict client/CLI Clippy, ten client tests
(including successful/mismatched digests, corruption, truncation, status/encoding/
range refusals, timeout, cancellation, private staging, occupied destination and
no-overwrite), the compiled README example and all 122 native CLI tests pass.
Rust 1.91 all-target/all-feature native checking and all-feature Wasm client
checking pass, as does standalone Wasm checking. Direct/deployment/default-client
boundaries, their refusal fixtures, dependency pins, rustdoc with warnings denied,
formatting and scoped ShellCheck pass. The package roster includes the native
module, tests and README. One installed native CLI case passes unconfirmed and
restored-refusal behavior before HTTP, using a freshly rebuilt CLI and unchanged
previously qualified standalone/gateway artifacts. No full release gate, actual
registry consumer installation, fresh native macOS or deployed-provider run was
performed. Publishing/registry adoption remains the maintainer's next delivery step.

## Preserved compatible dependency adoption — 0.62.1

Official crates.io index rows and archive SHA-256 values were verified for
Memory/Storage **0.60.5** and Metrics **0.24.3**. Incoming lock edits were preserved byte-for-byte. The precise Metrics update
confirmed the existing 0.24.3 selection without further resolution changes;
matching declared minimums now include Platform **0.15.3** and Wire **0.13.2**.
Own Cargo versions remain 0.62.0. There are no sibling paths, patches, compatibility
readers, schema changes or data migrations.

Memory 0.60.5 separates pure growth planning from mutation and consolidates
fixture diagnostics. Review confirms physical reservation still precedes manager
mutation, with unchanged public APIs, codecs, allocation identities and bucket
admission. Storage production source is identical to 0.60.4. Metrics production
source/feature definitions are unchanged from 0.24.1; optional Wire metadata now
requires the already selected 0.13.2 pairing. Producer fixtures/CI are not consumer
runtime qualification.

Build/Host Foundation **0.73.3**, Host Tools/Testkit **0.75.2**, Platform **0.15.3**
and Wire **0.13.2** were verified as latest published and already resolved.
Browser Platform remains published **0.1.1**. Shared Tooling stays at reviewed
**0.27.22** (`f4f431627290e18f0ccc1e5f8525d16b197cf7c3`); sibling 0.27.23 is still
marked Unreleased, with Shared-local version preparation changes, and was not
silently adopted as a released consumer snapshot.

Evidence and previous handoff: `/tmp/blob-upstream-1101/`. Strict Core/standalone/
CLI all-target/all-feature Clippy passes. Core **295** and memory-composition **2**
native tests pass, as do Rust **1.91** standalone Wasm checking and fresh release
standalone/storage/gateway Wasm builds. A fresh CLI binary and the unchanged
lock-selected Testkit 0.75.2/PocketIC 16.1.0 admission also pass.

Installed checks pass for growth-refusal rollback, all five standalone lifecycle
cases, both metrics cases and gateway synchronization/restoration. Exact case
counts and commands are retained in `installed-*.log` and `installed-results.json`.
Formatting, direct/deployment dependency boundaries, pins and documentation links
pass. Inputs stayed fixed. No full release gate, fresh native macOS/browser run
or deployed-provider qualification is claimed for this patch graph. Hosted 0.62.0
Core/client, Wire/generated-client and MSRV checks passed at readback; its Linux
and both macOS tooling jobs were still running, which is not a failure or proof
of success for those jobs.

## Next: Canic 0.62 adoption and bounded provider acceptance

Read-only Canic main is now **0.114.0** (`e75378595`), with pending local CI repairs.
Its root and independent Blob consumer graphs have adopted Blob **0.61.0**, with
Platform 0.15.3, Wire 0.13.2 and compatible Memory/Storage 0.60 requirements.
Canic's own handoff distinguishes its 0.114 source delivery from package
publication; do not infer published Canic 0.114 packages from its Git tag.

Next, its owner adopts published Blob **0.62** across the root and both independent
consumer graphs, qualifies dedicated/embedded mounting, lifecycle and generated
clients, then supplies a finite acceptance-service configuration and network/role
binding. Existing configuration remains local-only. No sibling was edited or built.
The maintainer knows the provider is Caffeine; do not ask them to invent another
provider or infer a deployed service from historical trial principals.

The owner handoff remains [Canic #622](https://github.com/dragginzgame/canic/issues/622#issuecomment-6546806175).
Use [the acceptance plan](../acceptance-plan.md#before-the-acceptance-deployment).
A bounded provider-only trial may precede a finished frontend; it does not qualify
application login, serving or catalog publication. Deployment and paid effects
retain their separate authorization/budget boundaries.

GitHub readback confirms no open Blob PRs and four open acceptance issues:
[#5 — Consumer publication and serving](https://github.com/dragginzgame/ic-blob-storage/issues/5),
[#31 — Application multi-user uploads](https://github.com/dragginzgame/ic-blob-storage/issues/31),
[#6 — Reference lifecycle and deletion/billing](https://github.com/dragginzgame/ic-blob-storage/issues/6),
[#34 — Project-payer linkage and funding evidence](https://github.com/dragginzgame/ic-blob-storage/issues/34).
Dependency checks alone close none of these. Provider intent and historical results
remain in [the probe ledger](../evidence/caffeine-probes/README.md); no new provider
probe was run. The prior detailed source-bound handoff is preserved at
`/tmp/blob-upstream-1101/previous-handoff.md` with its original evidence paths.
