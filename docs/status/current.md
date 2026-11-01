# Current status — 2026-11-01

## Delivered baseline and pending adoption

Git main/tag `v0.62.1` identifies release **0.62.1** at
`a7be19ea8fd7064b97120744a8ff286f0358c6f7`; see
[the release receipt](../release.json). This pass did not repeat registry
publication/provenance or hosted-CI readback. Earlier evidence must not be
reported as qualification of the pending dependency graph.

The pending notes target **0.63.0**, because Platform's executor/guard identity
changes to 0.16 and embedding hosts must align and rebuild together. All thirteen
own package versions stay 0.62.1 for maintainer release preparation. No staging,
commit, push, release, registry publication, deployment, sibling edit or paid
provider effect was performed. Existing incoming lockfile updates, including
Metrics 0.24.4, were preserved.

## Published dependency adoption — 0.63.0

Official crates.io index records and archive SHA-256 values were verified for:

| Capability | Selected release |
| --- | --- |
| Build / Host Artifacts, FS, HTTP and Process | 0.73.4 |
| Host Tools / Testkit | 0.76.0 |
| Memory / Storage | 0.60.6 |
| Platform / macros / executor guard | 0.16.0 |
| Wire / Derive | 0.13.3 |

Canonical Platform Types stays 0.15.1; incoming Metrics stays 0.24.4. Browser
Platform stays 0.1.1 and Shared Tooling stays at reviewed 0.27.22. No sibling path,
patch, compatibility alias, storage migration or versioned reader was added.
Testkit's sibling 0.76.1 notes were marked Unreleased at inspection; registry
0.76.0 was selected instead.

Root minimums and lock now agree, with one Platform/runtime identity. Read-only
source comparison of the actual published archives finds no Rust source changes
in Build, Host Foundation, Memory, Storage or Testkit from their prior selected
releases. Platform 0.16 introduces owned certification/BLS identities and native
macro symbol isolation; its crypto feature upgrades remain opt-in. Native Agent
retains its upstream certificate identity. The headless certificate proof test
now decodes `agent::ExternalCertificate` directly at that boundary, avoiding
unnecessary tree conversion while retaining verification and exact-byte checks.

Wire's new ByteArray representation is not adopted. Existing field types,
endpoint identities, encoded bytes, decoder limits, stored records, exact retry
bytes and lifecycle/accounting fences remain unchanged. Cross-release deployment
still follows [retirement/reinstall](../retiring-installations.md); unchanged
formats do not authorize an in-place cross-release upgrade.

The optional native-download helper delivered in 0.62.1 remains shared by the
client and CLI: trusted original metadata, bounded one-shot body verification,
optional raw SHA-256 and private create-only publication. It adds no verifier
identity, attestation, receipt or background worker. Applications retain their
publication and uncertain-outcome policy. Default and Wasm client graphs exclude
native HTTP/filesystem dependencies. The README no longer calls 0.62.1 pending.
See [the client guide](../../crates/ic-blob-storage-client/README.md) and
[trusted registration](../trusted-registration.md).

Evidence is retained in `/tmp/blob-adopt-1101/`, including original dirty inputs,
registry records/checksums, source comparisons, lock delta, exact commands and
input hashes. The preceding detailed handoff is preserved there as
`previous-handoff.md`; earlier helper evidence remains at
`/tmp/blob-native-helper-1101/` and dependency evidence at
`/tmp/blob-upstream-1101/`. Those earlier runs qualify their own inputs only.

Qualification on fixed source/manifest/lock inputs passes:

- Full locked offline metadata, declaration/pin boundaries and their refusal
  fixtures. Separate native and deployed graph observations confirm singleton
  Platform, Wire and relevant storage/process identities. Deployment and default/
  all-feature Wasm client graphs exclude native transport and tooling.
- Strict all-target/all-feature Clippy for Core, Contracts, Client, CLI,
  standalone and PocketIC support. Core 295, Contracts 131, Client 10 plus its
  compiled README, CLI 122 plus eight artifact/boundary tests, and both memory
  composition cases pass. The frozen Wire contract surface passes.
- Rust 1.91 native all-target/all-feature CLI/PocketIC checks and standalone/
  client Wasm checks. Fresh release standalone/storage/gateway Wasms and native
  CLI builds pass, with exact artifact hashes retained in `artifacts.json`.
- Canonical setup and offline admission of Testkit 0.76.0 and Build 0.73.4
  executables. PocketIC remains 16.1.0. Fourteen selected installed checks pass:
  two signed-ingress certificate/recovery cases, exact endpoint/provider-Candid
  exports, every owned method's malformed/identity/direction refusal, growth
  rollback, five lifecycle/recovery cases, two metrics cases, replicated-client
  refusals and native-download refusals before HTTP.
- The generated TypeScript client passes 27 methods and 137 frozen Rust frames,
  including malformed/direction/identity/limit/decimal checks. Prepared browser
  dependency admission passes; this is codec qualification, not live browser
  application/provider acceptance. Scoped formatting and documentation links pass.

Initial native download tests could not bind loopback under the sandbox; the
affected retry passes with local-network permission. The sandbox also returned
empty npm-version subprocess output; the canonical generated-client target
passes outside it with the unchanged installed Node/npm selection. Initial
failure logs are retained. They do not indicate a product regression.
The documented optional Shared graph helper is not included in this snapshot;
singleton observations use direct locked Cargo trees instead, retained alongside
their commands and required identities. No snapshot-owned implementation was copied.

No full workspace/release gate, native macOS/Windows execution, actual registry
consumer installation of pending source or deployed-provider qualification was
performed. The selected pending version remains documentation only; release and
publication remain maintainer actions.

## Next: Canic adoption and bounded provider acceptance

Last read-only Canic observation was main 0.114.0 (`e75378595`), with Blob 0.61
in its root and two independent consumer graphs. Canic was not reinspected or
built during this dependency adoption. Its owner must align the published Blob,
Platform and Memory identities across all three graphs, then qualify dedicated/
embedded mounting, generated clients, lifecycle and recovery. Do not infer
package publication or a deployed Blob service from a Git tag or local fixtures.

Use [the Canic owner handoff](https://github.com/dragginzgame/canic/issues/622#issuecomment-6546806175)
and [the acceptance plan](../acceptance-plan.md#before-the-acceptance-deployment).
The maintainer knows the provider is Caffeine; retain the original operator/
service selection rather than inferring a current deployment from trial principals.
A bounded provider-only trial may precede a finished frontend, but does not qualify
application login, serving or catalog publication. Deployment and paid effects
retain their separate authorization and budget boundaries.

The last GitHub readback found no open Blob PRs and four open acceptance issues:
[#5 — Consumer publication and serving](https://github.com/dragginzgame/ic-blob-storage/issues/5),
[#31 — Application multi-user uploads](https://github.com/dragginzgame/ic-blob-storage/issues/31),
[#6 — Reference lifecycle and deletion/billing](https://github.com/dragginzgame/ic-blob-storage/issues/6),
[#34 — Project-payer linkage and funding evidence](https://github.com/dragginzgame/ic-blob-storage/issues/34).
This pass did not refresh or close those issues. Dependency validation resolves
none of their deployed-provider acceptance obligations. The
[probe ledger](../evidence/caffeine-probes/README.md) retains intent, observations,
failures and cleanup; no new provider probe was run.
