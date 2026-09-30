# 🗃️ ic-blob-storage

Blob storage for Internet Computer canisters, with a shared Rust service core,
a standalone canister and a Canic-managed adapter. **Caffeine is the storage
provider.** The service owns tenant access, upload permissions, references,
quotas and accounting.

**Status: a working local prototype, with production integration still in progress.**
Canic composition runs in PocketIC. A complete live Caffeine upload and download
journey is still being qualified.

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
| Standalone canister | Shared handlers and explicit installation configuration; provider certificate issuance remains disabled |
| Canic adapter | **24 of 30 service methods** wired in a controlled managed fixture; their Candid types match standalone |
| Managed lifecycle | Installation, activation, verifier checks, reference-qualified downloads, cleanup accounting and fenced upgrades tested locally |
| Native tooling | Signed inspection, local verification and explicit verifier observation/submission |
| Browser integration | Private certificate/intent client composed with Caffeine's upload SDK in local tests |
| Live service acceptance | Still open: remaining managed endpoints, complete consumer flow, provider guarantees and operational recovery |

Canic's current endpoint macros also need supported query-size and decoding-work
controls before full managed input qualification. See the
[composition contract](docs/service-contract.md#managed-canic-composition) for that gap.

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

The [toolchain file](rust-toolchain.toml) pins Rust, rustfmt, Clippy and the Wasm
target. `make deps` fetches locked Rust dependencies; validation then uses offline
Cargo and this repository's `target/` directory.

Choose the local canister path you want to exercise:

| Path | Command | Extra setup |
| --- | --- | --- |
| Standalone | `make test-standalone` | PocketIC server |
| Canic-managed prototype | `make test-canic-composition` | PocketIC, Canic CLI, ic-wasm and wasm-opt |
| Browser certificate flow | `make test-browser` | PocketIC, browser packages, Node and Chromium |

Follow [dependency setup](docs/dependencies.md) to provision those tools.
The test targets use local canisters and do not deploy a live service.

## 🧩 Repository layout

Both canister adapters call the same service workflows and tenant rules.

| Location | Responsibility |
| --- | --- |
| [Rust core](crates/ic-blob-storage) | Content, policy, durable state and shared workflows; builds without Canic |
| [Standalone host](canisters/standalone/README.md) | Explicit endpoints, installation, memory and lifecycle |
| [Canic composition library](crates/ic-blob-storage-canic) | Opt-in memory declarations, caller guards and synchronous installation/restoration |
| [Managed fixture](canisters/test/canic_probe) | Current Canic endpoint subset and local composition artifact |
| [Native CLI](crates/ic-blob-storage-cli) | Signed service inspection and verifier tooling |
| [Browser client](clients/browser/README.md) | Certificate transport and durable intent boundary; reuses Caffeine's upload SDK |
| [PocketIC harness](tests/pocketic) | Actual local canister, lifecycle and inter-canister tests |

The host owns one `ic-memory` runtime and its allocation policy. In Canic,
that host is Canic. Linking a library registers no endpoints or lifecycle hooks.
See [memory composition](docs/dependencies.md#memory-composition-with-canic-and-icydb)
for the integration details.

## 🛠️ Tools and examples

| I want to… | Start here |
| --- | --- |
| Inspect service state or retained funding | [Operator and verifier guide](docs/operator-guide.md#identity-trust-and-service-status) |
| Find uploads or inspect references | [Upload history](docs/operator-guide.md#upload-history) and [reference inspection](docs/operator-guide.md#reference-receipts-and-current-status) |
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
| `make test-pocketic` | Local IC suites, including standalone and managed fixtures |
| `make test-canic-composition` | Focused managed Canic suite |
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
| [Canic parity](docs/canic-parity.md) | Replacement capabilities and removal obligations |
| [Provider review](docs/provider-review.md) | Reviewed Caffeine interfaces and unresolved guarantees |
| [Probe ledger](docs/evidence/caffeine-probes/README.md) | Tracked investigations, retained artifacts and limitations |
| [Core evidence](docs/evidence/core-primitives.md) | Source-bound local implementation and test results |
| [Release guide](docs/releasing.md) | Maintainer release and registry publication workflow |

Before 1.0, contract changes are hard cuts with no compatibility shims or migration
engine. Breaking changes use minor releases; cross-release installations require
reinstall after obligations are safely retained or discharged. Canic source removal
and retirement of an existing storage installation are separate decisions.

## 📄 License

MIT — see [LICENSE](LICENSE).
