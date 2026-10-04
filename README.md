# 🗃️ ic-blob-storage

Blob storage for Internet Computer canisters, with a shared Rust service core
and a standalone canister. **Caffeine is the storage
provider.** The service owns tenant access, upload permissions, references,
quotas and accounting.

**Status: a working prototype with successful live uploads and downloads.**
The isolated 0.7.0 trial completes 1 KiB and 10 MiB Caffeine uploads, independent
verification, verifier attestation and tenant downloads. Production integration
and operational recovery remain in progress; lifecycle tests run in PocketIC.

[Current status](docs/status/current.md) · [Changelog](CHANGELOG.md) ·
[Development plan](docs/roadmap.md) · [Service contract](docs/service-contract.md)

## ✨ What it does

| Capability | Purpose |
| --- | --- |
| Tenant and uploader permissions | Bind each upload to its tenant, uploader and expiry |
| Content verification | Build manifests and verify Caffeine roots, chunk hashes and complete content |
| References and cleanup | Track which assets remain in use and retain exact mutation receipts |
| Quotas and accounting | Bound objects, bytes, uploads, references and retained history |
| Provider bookkeeping | Preserve gateway state, funding intents, refunds and uncertain outcomes |
| Recovery | Inspect restored records; prove current-instance activation from IC history |

The design sends file bytes directly from the uploader to Caffeine. Hashing and
whole-file verification can run off-canister. The canister coordinates permissions
and durable metadata; admission does not require uploading the file body to it.

## 🚧 Where we are today

| Area | Current state |
| --- | --- |
| Shared Rust core | Implemented, with native and local IC evidence |
| Standalone canister | Shared handlers; trusted-uploader certificate issuance within configured object sizes, quotas and multi-file capacity |
| Lifecycle | Synchronous fenced restoration; IC-history-proven current-instance recovery and snapshot refusal |
| Native tooling | Installation/account inputs, verified snapshots and signed setup/recovery; tenant downloads and verifier completion pass live |
| Batch publication | Frozen inventories, native session, SDK worker and original source/profile/handoff bindings; current development adds a callable phase driver and bounded native subprocess helper. Consumer key/history selection and acceptance remain open |
| Application integration | Consumer frameworks own their wrappers, asset transactions and integration tests |
| Browser integration | Caffeine's SDK with certificate intent and bounded persistent journaling; live 1 KiB and ten-chunk 10 MiB transfers pass |
| Live service acceptance | Still open: complete consumer flow, provider guarantees and operational recovery |

The current library release is **0.13.0**. Configurable certificate sizing shipped
in 0.7.0; indexed batch preparation, frozen-file browser transfer and current-instance
recovery shipped in 0.8.0. Release 0.9.0 adds one-pass batch setup and removes the
standalone DTO forwarding namespace; consumers import the core configuration types.
Release 0.10.0 adds persistent publication sessions, authenticated maps and the
private-port browser worker. Release 0.11.0 adds verifier-phase composition
and selected-signer browser bootstrap; neither constitutes complete consumer acceptance.
Release 0.12.0 consolidates descriptor serving, retires fixture-only reference
journals and identifies the frozen native/stable formats explicitly. Release
0.13.0 adds original session recovery and native/browser selection/transfer
handoffs. Current development follows native phase guidance through a bounded
browser driver with private native pipes and one complete-map/final-report/exit
acceptance boundary. Consumer
key/history selection and full acceptance remain open.
The frozen 0.6.0 live trial retains its original 1 KiB configuration and stopped
upload history. A separate
[0.7.0 trial](docs/evidence/caffeine-probes/deployed/2026-10-02-trial-v070-live-01/summary.json)
qualifies the two sample journeys without resetting those obligations.

The public library has no downstream framework dependency. Consumer frameworks
wrap its shared workflows and own integration testing in their repositories.
Canic adoption is deferred; the [consumer integration backlog](docs/canic-parity.md#integration-feedback)
tracks required wrapper, publication, serving and lifecycle work.
The [issuance contract and retained trial](docs/standalone-trial.md) distinguish
configurable upload limits from the first live owner's 1 KiB configuration.
Exact trial authority, budgets and retained obligations are recorded in the
[current handoff](docs/status/current.md). Provider spending caps remain unqualified.

Local tests use controlled provider substitutes where stated. A verifier's
attestation records observed content availability; it does not promise future
retention. Releasing a reference, deleting provider bytes and ending billing are
separate events. Frozen 0.7.0 restores inspection-only; the current release supports operator-only
current-instance recovery from IC management history. Its twenty-change window must
still reach the immutable installation anchor; older snapshots/backups stay fenced.
See the [recovery guide](docs/operator-guide.md#current-instance-recovery)
for operating limits and the [gap review](docs/service-gaps.md) for consumer and
provider acceptance still required.

## 🚀 Get started locally

Install rustup and Cargo, then run from the repository root:

```sh
make deps
make test-native
```

The [toolchain file](rust-toolchain.toml) pins development Rust **1.99.0**, rustfmt,
Clippy and the Wasm target. The library's minimum supported Rust version is
**1.88.0**, including the local PocketIC harness.
`make deps` fetches locked Rust dependencies; validation then uses offline Cargo
and this repository's `target/` directory.

Choose the local canister path you want to exercise:

| Path | Command | Extra setup |
| --- | --- | --- |
| Standalone | `make test-standalone` | PocketIC server |
| Browser certificate flow | `make test-browser` | PocketIC, browser packages, Node and Chromium |
| Standalone upload/download rehearsal | `make test-browser-standalone` | PocketIC, browser packages, Node and Chromium |

Follow [dependency setup](docs/dependencies.md) to provision those tools.
The test targets use local canisters and do not deploy a live service.

## 🧩 Repository layout

The standalone host and external consumers use the same service workflows and tenant rules.

| Location | Responsibility |
| --- | --- |
| [Rust core](crates/ic-blob-storage) | Content, policy, durable state and shared workflows; no downstream framework dependency |
| [Standalone host](canisters/standalone/README.md) | Explicit endpoints, installation, memory and lifecycle |
| [Native CLI](crates/ic-blob-storage-cli) | Offline snapshots and batch inputs, signed setup/inspection, tenant references, verified downloads and verifier tooling |
| [Browser client](clients/browser/README.md) | Certificate transport and durable intent boundary; reuses Caffeine's upload SDK |
| [PocketIC harness](tests/pocketic) | Actual local canister, lifecycle and inter-canister tests |

The host owns one `ic-memory` runtime and its allocation policy. Linking a library registers no endpoints or lifecycle hooks.
See [memory composition](docs/dependencies.md#memory-composition)
for the integration details.

## 🛠️ Tools and examples

| I want to… | Start here |
| --- | --- |
| Size a consumer installation | [Resource limits and lifetime capacity](docs/operator-guide.md#size-a-consumer-installation) |
| Freeze an upload batch and check live capacity | [Offline inventory](docs/operator-guide.md#freeze-a-publication-inventory-offline), then [signed batch check](docs/operator-guide.md#check-a-frozen-batch-against-live-capacity) |
| Prepare one batch file or recover its setup | [Indexed preparation and original journals](docs/operator-guide.md#prepare-one-indexed-file-with-surviving-setup-intent) |
| Prepare a batch with one verification pass | [Bounded serial setup](docs/operator-guide.md#prepare-a-complete-batch-with-one-verification-pass); active capacity still applies |
| Complete uploads at concurrency one | [Per-file verification and reference checks](docs/operator-guide.md#complete-one-file-before-preparing-the-next); complete headless publisher still open |
| Reuse one validated batch across publication phases | [Persistent native session](docs/operator-guide.md#hold-one-validated-batch-across-publication-phases) with an explicitly selected verifier |
| Offload uploads to a browser worker | [Private-port publication worker](clients/browser/README.md#run-jobs-in-a-browser-worker) and [selected-signer host/bootstrap](clients/browser/README.md#launch-the-maintained-worker-with-a-selected-signer) |
| Launch a browser from a native parent | [Chromium bridge](clients/browser/README.md#launch-chromium-from-a-native-parent), with fixed profile/origin and bounded selected-body loading |
| Confirm every batch file before mapping assets | [Authenticated complete reference map](docs/operator-guide.md#produce-a-complete-confirmed-reference-map); serving and publication remain consumer responsibilities |
| Admit, prepare or cancel an upload | [Signed upload setup](docs/operator-guide.md#admit-and-prepare-an-upload) |
| Hand a verified upload snapshot to the browser client | [Generated binding](clients/browser/README.md) and [offline check](docs/dependencies.md#offline-nativebrowser-handoff) |
| Download a verified file | [Tenant downloads](docs/operator-guide.md#download-a-verified-file) |
| Diagnose a proposed funding intent | [Passive funding assessment](docs/operator-guide.md#passive-funding-assessment) |
| Inspect service state or retained funding | [Operator and verifier guide](docs/operator-guide.md#identity-trust-and-service-status) |
| Observe provider-reported balances or relationships | [Account inspection](docs/operator-guide.md#account-inspection) |
| Sync or revoke gateways, or cancel a pending sync | [Gateway controls](docs/operator-guide.md#gateway-controls) |
| Find uploads or inspect references | [Upload history](docs/operator-guide.md#upload-history) and [reference inspection](docs/operator-guide.md#reference-receipts-and-current-status) |
| Retain or release an exact tenant reference | [Generate inputs](docs/operator-guide.md#generate-reference-inputs-offline), then [submit once](docs/operator-guide.md#submit-a-reference) |
| Use one confirmed blob for two tenant assets | [Share and release references](docs/operator-guide.md#share-a-confirmed-blob-within-a-tenant) |
| Check a file against its saved upload declaration | [Local-file verification](docs/operator-guide.md#verify-a-local-file) |
| Observe provider bytes and submit a verifier statement | [Observation](docs/operator-guide.md#observe-provider-content) and [submission](docs/operator-guide.md#submit-an-attestation) |
| Prepare file manifests, inventories or saved bodies | [Local preparation guide](docs/local-tools.md#prepare-one-file) |
| Inspect or exercise a running PocketIC fixture | [Fixture tools](docs/local-tools.md#inspect-a-running-pocketic-fixture) |
| Integrate the standalone canister | [Host configuration and endpoint contract](canisters/standalone/README.md) |
| Resume a fenced current instance | [Operator recovery and finite IC history](docs/operator-guide.md#current-instance-recovery) |

These guides retain the exact command examples, input formats, limits and recovery
behavior. The native client requires explicit signing identities and installation
scope; controller status does not grant tenant access.

## ✅ Development checks

| Command | What it checks |
| --- | --- |
| `make fmt-check` | Rust formatting |
| `make check` | Native compilation |
| `make clippy` | Strict Rust linting |
| `make test-native` | Native tests and doctests |
| `make test-pocketic` | Local storage and standalone IC suites |
| `make test-admission-resources` | Admission bounds and local instruction-cost reports |
| `make test-read-resources` | Read-slot bounds and local instruction-cost reports |
| `make probe-check` | Offline integrity checks of retained provider evidence |
| `make docs-check` | Rust API documentation |
| `make wasm-check` | Wasm compilation |
| `make ci` | Complete repository validation gate |
| `make cloc` | Rust LOC and test-function counts under `crates/`; requires cloc and jq |

Run focused checks during development. Full validation and release commands follow
[development governance](docs/governance/development.md).
Releases preserve build artifacts; cleanup is a separate `make clean` action.

## 📚 Further reading

| Document | Use it for |
| --- | --- |
| [Current status](docs/status/current.md) | Latest handoff, validation evidence and next work |
| [Development plan](docs/roadmap.md) | Milestones and consumer integration direction |
| [Service contract](docs/service-contract.md) | Authority, accounting, verifier trust and recovery rules |
| [Acceptance plan](docs/acceptance-plan.md) | What must be demonstrated before service qualification |
| [Service gaps](docs/service-gaps.md) | Remaining consumer, recovery, deletion and billing acceptance |
| [Consumer integration backlog](docs/canic-parity.md#integration-feedback) | Open wrapper, publication, serving and retirement actions |
| [Provider review](docs/provider-review.md) | Reviewed Caffeine interfaces and unresolved guarantees |
| [Probe ledger](docs/evidence/caffeine-probes/README.md) | Tracked investigations, retained artifacts and limitations |
| [Core evidence](docs/evidence/core-primitives.md) | Source-bound local implementation and test results |
| [Release guide](docs/releasing.md) | Maintainer release and registry publication workflow |

Before 1.0, contract changes are hard cuts with no compatibility shims or migration
engine. Breaking changes use minor releases; cross-release installations require
reinstall after obligations are safely retained or discharged. Source removal
and retirement of an existing storage installation are separate decisions.

## 📄 License

MIT — see [LICENSE](LICENSE).
