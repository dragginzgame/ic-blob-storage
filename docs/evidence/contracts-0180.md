# Runtime-free contracts — pending 0.18.0

Issue [#27](https://github.com/dragginzgame/ic-blob-storage/issues/27) implements
accepted B1/B2 together. Entry was clean released 0.17.2 at
`e2526159413c79322b9ec89215a01266c9a7f418`. Pending documentation targets the
pre-1.0 public Rust hard cut at 0.18.0; workspace version, local catalog requirement
and receipt remain 0.17.2. This is local implementation, not a committed release.

## Complete boundary and consumers

The [inventory](contracts-0180-inventory.json) traces all **310** direct service
import leaves in the original native CLI/examples, including test imports, and
records old/new module/split-symbol paths and named declarations in **83**
whole-file owners. Module moves carry their methods and subordinate helpers.
Imports in native CLI/examples now target the contracts owner; no effectful native
CLI consumer remains service-dependent. The PocketIC harness, service adapters
and effectful replicated clients intentionally retain their core runtime edge.

| Former service owner | Canonical contracts owner | Disposition |
| --- | --- | --- |
| `dto` | `dto` | Passive Candid shapes, unchanged |
| `model::identity` | `identity` | Digests, Caffeine trees, streaming/ordered verification and algorithm checkpoints |
| `model::lifecycle::binding`, `ReferenceId` | `binding` | Validated object/reference identities |
| Admission/request structs, upload context/permission | `upload::binding`, `reference::binding` | Passive exact operation bindings |
| Immutable service/billing/funding/catalog/upload/gateway limits | `configuration::{service,billing,funding,limits}` | Checked configuration; no accounting or quota state |
| Completion/issuer/download scope and metadata checks | `upload::{completion,issuer,metadata}`, `download::scope` | Explicit checked role/project bindings; caller authentication remains host-owned |
| Pure replies beneath `ops::service` | `upload`, `reference`, `funding`, `download`, `tenant` reply modules | Existing bounded codecs and exact request/receipt correlation |
| Method constants under `ops`/`workflow` | `protocol` | Unchanged method strings |
| Caffeine preparation/download-target/account-link input helpers | `provider::{preparation,download,onboarding}` | Existing offline algorithms/wire input; no dispatch, credentials or retry engine |
| Tenant observations/preconditions, history classifiers | `tenant`, `upload::history` | Passive observations and pure reply prediction/filtering; owners supply authoritative state/capacity |
| Configuration decoding and validation | `configuration` | One bounded decoder/validator; service projects its result into accounting/read owners |

Both libraries and all 12 workspace members inherit one root version/catalog/lock.
The preparation/verification example targets and independent Caffeine hashing and
account-link fixtures move to contracts. Make, browser SDK example build selection,
docs, standalone/probe adapters and tests use that owner. The original provider
fixture bytes remain exact; historical evidence/checksum records remain frozen
with their original source paths.

### Removed owners and deliberate differences

These are **moves**, not discarded behavior. The exact named functions, types,
methods, constants and former files are listed in the inventory. No compatibility
reexports/aliases preserve public former paths. Internal imports of canonical
contracts are ordinary consumption, not a second implementation.

`ops::service::configuration::decode_candidate`, `ConfigurationInputError` and
`ConfigurationScalar` move to contracts. Core `validate_candidate` remains a
service construction projection with `ServiceConfigurationAdapterError`, while
contracts validation returns `ValidatedConfiguration` with private fields and
read-only accessors. `ValidatedInstallationInput` is likewise sealed against
post-validation mutation; neither checked result can be forged by struct literal. Core
`ValidatedServiceInstallation::new` delegates pure checks and retains its record
bound; offline clients instead use `ValidatedInstallationInput::new`. Core
`ValidatedServiceInstallation` and all storage/lifecycle/accounting types remain.
Algorithm-only SHA checkpoints are private contract state; durable service schemas
remain service-owned. Fixed envelope constants/structural predicates have one
contracts owner consumed by service models/ops; opening records and enforcing
live capacities/fences remain service responsibilities. This includes the existing funding attempt,
gateway member, manifest, read-session and allocation-envelope predicates;
accounting transitions, journal codecs and gateway synchronization stay core-owned.

### Compatibility and external owners

Public Rust paths are removed, requiring a coordinated minor release and rebuild.
The service publicly re-exports Memory 0.31 as before; this batch does not change
its identity. Cross-release retirement/reinstallation remains mandatory even with
unchanged wire/data shapes; no old backup is activated or fence cleared here.
Canic [#444](https://github.com/dragginzgame/canic/issues/444) and Toko Miner
[#20](https://github.com/dragginzgame/toko-miner/issues/20) own downstream adoption
and managed/native qualification. No sibling source is edited. Complete publisher
adoption remains Blob [#4](https://github.com/dragginzgame/ic-blob-storage/issues/4).

## Qualification and evidence limits

The [local intent](caffeine-probes/local/2026-10-07-contracts-0180-01/intent.json)
bounds source review, maintained Linux loopback and isolated PocketIC fixtures.
Artifacts and each distinct failed/check attempt are retained in
`.tmp/contracts-0180-01/`; they are not provider observations. Failed extraction
checks and the first Cargo-fix socket refusal remain, followed by repaired checks.
The first release fixture run exposed a Perl delimiter error; its isolated tree
`target/release-tests.B6WWpp` and raw log remain. No live provider request or paid
cycles occur. No build-time or Wasm-size gain is claimed.

- Core native checks pass **345** unit and **38** integration tests.
- Contracts pass **103** unit, **10** independent hash-vector integration and
  **21** preparation/verification example tests, including bounded malformed,
  oversize, scope, receipt, chronology and uncertainty refusals.
- Strict Clippy passes for the affected libraries, CLI, standalone/probe adapters
  and PocketIC harness, all targets/features.
- Actual Make native-host qualification passes **114** CLI/probe, **103** contracts,
  **21** examples, **two** standalone installation/carrier and **one** Metrics
  restoration-attribution case.
- The freshly rebuilt standalone Wasm’s extracted Candid is **byte-for-byte exact**
  against the saved pre-edit 0.17.2 Wasm extraction, including the moved DTO graph.

- Exact extracted Cargo package payloads pass **517** unit/integration/example
  cases and **two** doctests on the final source. Both archives, normalized
  manifests, payload lock and metadata are retained at
  `target/package/blob-verification.EVBS8s`; raw output is
  `.tmp/contracts-0180-01/package-checked-input-final.log`. Payload external
  selections/checksums are a verified subset of the frozen workspace lock.
- Rust **1.88** passes native all-target/all-feature checks for contracts, core,
  CLI, standalone and storage probe, plus Wasm checks for contracts/core and
  standalone, storage, consumer, admission, authority and funding adapters.
- The broader isolated PocketIC run passes **71** publication, lifecycle,
  operator, status/certificate, planning, reads and funding recovery cases.
  After the final canonical funding/gateway predicate and sealed checked-input
  changes, rebuilt standalone/storage/consumer Wasm and CLI pass another **five**
  funding-client, gateway-restoration and installation/carrier cases. The earlier
  native-host run contributes three PocketIC cases; total executed is **79**
  within the recorded budget of 100. The final Candid extraction also matches
  the maintained `canisters/standalone/service.did` exactly, SHA-256
  `1770ce8dac80bec5f167572990866e5312dffd36f773310b98cb08b891c3c2cd`.
- Strict Clippy and warning-free rustdoc pass again after sealing checked inputs.
  Snapshot, pins/inheritance, documentation links, normal dependency boundaries,
  manifest/Rust formatting, shell checks and substitute release-command routing
  pass. Release adapter fixtures pass catalog version advancement/refusal,
  frozen package-selection checks and explicit single-package publication routing.

### Preserved failures and package verification

Ordinary Cargo multi-package verification hits
[Cargo #14396](https://github.com/rust-lang/cargo/issues/14396), the local-registry
checksum error for the unpublished contracts member, on both Cargo 1.99 and 1.91.
Those failed logs/archives remain; optional-lock and Rust 1.88 assembly diagnostics
also remain. This is not a passed standard Cargo verifier. The maintained
`make package` workaround uses standard Cargo to assemble both archives, then
builds/tests their exact extracted payloads in a fresh retained workspace whose
only local patch points to the extracted contracts payload. It rejects any
external identity/checksum absent from the frozen entry lock. Assembly needs
Cargo 1.90 or newer; library/compiler support remains Rust 1.88. See the
[release guide](../releasing.md#publication-and-deployment) for explicit contracts-first,
service-second publication selection; no registry action was tested or executed.

The first standalone doctest attempt found a moved verifier import still naming
the former owner. Its log remains; the corrected import and both extracted-package
doctests pass. Initial missing moved documentation links, intermediate extraction
compile/lint errors and package-workaround diagnostics remain separately retained.
The [summary](caffeine-probes/local/2026-10-07-contracts-0180-01/summary.json)
binds final source inputs, logs, artifacts and failures with hashes. Historical
provider review hashes still bind their original owners even where current
navigation links now point to moved files.

Native macOS requires the matching committed source’s existing matrix; local
Linux evidence does not replace it. #27 remains open for that qualification.
No full CI gate, commit, push, release, registry publication or deployment is run.
No instruction-count, build-time or Wasm-size gain is measured or claimed.
