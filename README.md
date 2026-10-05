<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

<!-- helper-navigation:start -->
<p align="center">
  <a href="https://github.com/dragginzgame/canic"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/canic.svg" width="18" height="18" alt=""> <strong>canic</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/icydb"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/icydb.svg" width="18" height="18" alt=""> <strong>icydb</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-timers"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-timers.svg" width="18" height="18" alt=""> <strong>ic-timers</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-memory"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-memory.svg" width="18" height="18" alt=""> <strong>ic-memory</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-query"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-query.svg" width="18" height="18" alt=""> <strong>ic-query</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-backup"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-backup.svg" width="18" height="18" alt=""> <strong>ic-backup</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-blob-storage"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-blob-storage.svg" width="18" height="18" alt=""> <strong>ic-blob-storage</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-testkit"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-testkit.svg" width="18" height="18" alt=""> <strong>ic-testkit</strong></a>
</p>
<!-- helper-navigation:end -->

`ic-blob-storage` helps Internet Computer apps handle large files such as
images, videos, audio, 3D models and downloads.

Internet Computer apps run in services called *canisters*. Canisters can keep
their own data, but large files are often better handled by a dedicated storage
provider. This project uses Caffeine to hold the file
contents while the app keeps control of upload permissions, file checks, usage
records, storage limits and accounting.

**Project status: working prototype.** Live uploads and downloads have been
demonstrated, but application integration, provider guarantees, deletion,
billing and production recovery are still being qualified. This is not yet a
finished production service.

## What it does

| What you need | What `ic-blob-storage` does |
| --- | --- |
| Control uploads | Decides who may upload files and how much they may store |
| Check files | Confirms that a stored file matches the original |
| Reuse files safely | Tracks every place that still depends on a file |
| Control storage growth | Enforces limits for files, bytes and upload activity |
| Keep records through interruptions | Preserves important records across interruptions and upgrades |
| Track costs and provider activity | Records storage use, payments, refunds and operations that may need checking |

After an upload is approved, the file travels directly from the uploader to
Caffeine. It does not pass through the app's canister. The canister instead
keeps the smaller records needed to control the upload, verify the result and
remember whether the file is still in use.

<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-how-it-works.svg" alt="How an upload moves from permission through direct Caffeine storage and verification to a file reference the app can safely use" width="800">
</p>
<p align="center"><em>File contents go directly to Caffeine while ic-blob-storage manages permission, verification and usage records.</em></p>

## When it may be useful

Consider `ic-blob-storage` when:

- your app handles images, videos, audio, 3D models or other large downloads;
- users, teams or organizations upload their own files;
- the same stored file may be used in several places;
- you need to confirm that uploaded files are complete and unchanged;
- you need limits on file sizes, total storage or upload activity; or
- you need reliable records for recovery after an interruption or upgrade.

<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-decision-guide.svg" alt="Decision guide for whether an app handling large files would benefit from upload control, verification, quotas or reuse tracking" width="800">
</p>
<p align="center"><em>It is most useful when large files need more control than ordinary application storage provides.</em></p>

## What it is not

- It is not a consumer-facing file manager like Dropbox or Google Drive.
- It is not a finished hosted service; a developer must integrate it into an
  application.
- It does not store the large file contents inside the application's canister.
- Releasing a file from the app, deleting the provider's copy and ending billing
  are separate steps. The last two still require further provider qualification.
- A successful file check confirms what was observed at that time; it cannot
  promise that the provider will retain the file forever.

<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-file-lifecycle.svg" alt="File lifecycle showing approval, upload, verification, app references, reference release, provider deletion and billing confirmation as separate events" width="800">
</p>
<p align="center"><em>Releasing the app's reference, deleting the provider copy and ending billing are separate events.</em></p>

---

The remaining sections are for developers and operators who want to integrate,
test or maintain the project.

[Current status](docs/status/current.md) · [Changelog](CHANGELOG.md) ·
[Development plan](docs/roadmap.md) · [Service contract](docs/service-contract.md)

## Technical status

| Area | Current state |
| --- | --- |
| Shared Rust core | Implemented, with native and local IC evidence |
| Standalone canister | Shared handlers; trusted-uploader certificate issuance within configured object sizes, quotas and multi-file capacity |
| Lifecycle | Synchronous fenced restoration; IC-history-proven current-instance recovery and snapshot refusal |
| Native tooling | Installation/account inputs, verified snapshots and signed setup/recovery; tenant downloads and verifier completion pass live |
| Batch publication | Frozen inventories, native session, SDK worker and original source/profile/handoff bindings; callable phase driver and bounded native subprocess helper. Consumer key/history selection and acceptance remain open |
| Application integration | Consumer frameworks own their wrappers, asset transactions and integration tests |
| Browser integration | Caffeine's SDK with certificate intent and bounded persistent journaling; live 1 KiB and ten-chunk 10 MiB transfers pass |
| Live service acceptance | Still open: complete consumer flow, provider guarantees and operational recovery |

The [changelog](CHANGELOG.md) contains the release history. Detailed validation,
open integration work and retained trial obligations are recorded in the
[current status](docs/status/current.md). Earlier live trials remain frozen and
do not establish production readiness for the current release.

The public library has no downstream framework dependency. Consumer frameworks
wrap its shared workflows and own integration testing in their repositories.
Canic adoption is deferred. The [service contract](docs/service-contract.md)
defines wrapper and lifecycle obligations; [GitHub issues](https://github.com/dragginzgame/ic-blob-storage/issues)
track publication, serving and consumer integration work.
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

## Get started locally

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

## Repository layout

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

## Tools and examples

| I want to… | Start here |
| --- | --- |
| Install the native and browser publisher tools | [One selected source checkout](docs/local-tools.md#install-the-native-and-browser-tools), with compiled version and artifact hashes |
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

## Development checks

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

## Further reading

| Document | Use it for |
| --- | --- |
| [Documentation index](docs/README.md) | Choose the right conceptual, integration, operations or evidence guide |
| [Current status](docs/status/current.md) | Latest handoff, validation evidence and next work |
| [Development plan](docs/roadmap.md) | Milestones and consumer integration direction |
| [Service contract](docs/service-contract.md) | Authority, accounting, verifier trust and recovery rules |
| [Acceptance plan](docs/acceptance-plan.md) | What must be demonstrated before service qualification |
| [Service gaps](docs/service-gaps.md) | Remaining consumer, recovery, deletion and billing acceptance |
| [GitHub issues](https://github.com/dragginzgame/ic-blob-storage/issues) | Consumer integration, publication, serving and retirement work |
| [Shared engineering baseline](DRAGGINZGAME.md) | Reviewed common rules; AGENTS.md supplies the local overlay |
| [Provider review](docs/provider-review.md) | Reviewed Caffeine interfaces and unresolved guarantees |
| [Probe ledger](docs/evidence/caffeine-probes/README.md) | Tracked investigations, retained artifacts and limitations |
| [Core evidence](docs/evidence/core-primitives.md) | Source-bound local implementation and test results |
| [Release guide](docs/releasing.md) | Maintainer release and registry publication workflow |

Before 1.0, contract changes are hard cuts with no compatibility shims or migration
engine. Breaking changes use minor releases; cross-release installations require
reinstall after obligations are safely retained or discharged. Source removal
and retirement of an existing storage installation are separate decisions.

## License

MIT — see [LICENSE](LICENSE).
