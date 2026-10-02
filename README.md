# 🗃️ ic-blob-storage

Blob storage for Internet Computer canisters, with a shared Rust service core
and a standalone canister. **Caffeine is the storage
provider.** The service owns tenant access, upload permissions, references,
quotas and accounting.

**Status: a working local prototype, with production integration still in progress.**
Standalone and storage lifecycle tests run in PocketIC. A complete live Caffeine
upload and download journey is still being qualified.

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
| Recovery inspection | Keep records available after restore while blocking unsafe mutations |

The design sends file bytes directly from the uploader to Caffeine. Hashing and
whole-file verification can run off-canister. The canister coordinates permissions
and durable metadata; admission does not require uploading the file body to it.

## 🚧 Where we are today

| Area | Current state |
| --- | --- |
| Shared Rust core | Implemented, with native and local IC evidence |
| Standalone canister | Shared handlers; restricted certificate issuance for an explicitly trusted uploader and one object up to 1 KiB |
| Lifecycle | Synchronous installation, inspection-only restoration and local rollback tests |
| Native tooling | Offline installation checks, account-link inputs and verified snapshots; signed setup/recovery, tenant downloads and verifier completion tested locally |
| Application integration | Consumer frameworks own their wrappers, asset transactions and integration tests |
| Browser integration | Reusable upload composition binds Caffeine's SDK to certificate intent, serial transfer and bounded request journaling; locally tested |
| Live service acceptance | Still open: complete consumer flow, provider guarantees and operational recovery |

The public library has no downstream framework dependency. Consumer frameworks
wrap its shared workflows and own integration testing in their repositories.
The [accepted standalone trial contract](docs/standalone-trial.md) records the
1 KiB limit, 100T-cycle planning budget and inputs needed before live effects.
Provider spending limits and old-backup recovery remain outside its guarantees.

Local tests use controlled provider substitutes where stated. A verifier's
attestation records observed content availability; it does not promise future
retention. Releasing a reference, deleting provider bytes and ending billing are
separate events. Restored owners currently allow inspection only.

## 🚀 Get started locally

Install rustup and Cargo, then run from the repository root:

```sh
make deps
make test-native
```

The [toolchain file](rust-toolchain.toml) pins development Rust **1.99.0**, rustfmt,
Clippy and the Wasm target. The library's minimum supported Rust version is
**1.88.0**; the local PocketIC harness needs **1.89.0** for file locking.
`make deps` fetches locked Rust dependencies; validation then uses offline Cargo
and this repository's `target/` directory.

Choose the local canister path you want to exercise:

| Path | Command | Extra setup |
| --- | --- | --- |
| Standalone | `make test-standalone` | PocketIC server |
| Browser certificate flow | `make test-browser` | PocketIC, browser packages, Node and Chromium |

Follow [dependency setup](docs/dependencies.md) to provision those tools.
The test targets use local canisters and do not deploy a live service.

## 🧩 Repository layout

The standalone host and external consumers use the same service workflows and tenant rules.

| Location | Responsibility |
| --- | --- |
| [Rust core](crates/ic-blob-storage) | Content, policy, durable state and shared workflows; no downstream framework dependency |
| [Standalone host](canisters/standalone/README.md) | Explicit endpoints, installation, memory and lifecycle |
| [Native CLI](crates/ic-blob-storage-cli) | Signed inspection, tenant reference submission and verifier tooling |
| [Browser client](clients/browser/README.md) | Certificate transport and durable intent boundary; reuses Caffeine's upload SDK |
| [PocketIC harness](tests/pocketic) | Actual local canister, lifecycle and inter-canister tests |

The host owns one `ic-memory` runtime and its allocation policy. Linking a library registers no endpoints or lifecycle hooks.
See [memory composition](docs/dependencies.md#memory-composition)
for the integration details.

## 🛠️ Tools and examples

| I want to… | Start here |
| --- | --- |
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
