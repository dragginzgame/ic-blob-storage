# ic-blob-storage

An independent blob-storage service library for Internet Computer canisters.
The core verifies raw-content and Caffeine-tree identities, tracks checked chunks,
and lists missing chunks within explicit work limits. Ordered reads check each
leaf before final raw-digest verification. It also validates billing inputs and
decodes bounded Caffeine replies.

A transient multi-object catalog owns lifecycle, root claims and request receipts,
with tenant quotas, separate physical/billing accounting and
bounded deletion pages. Pure tenant/gateway policy protects local reads and
preserves revocation. A transient upload owner reserves shared tenant/global
capacity before exposure, retains exact operation history and keeps uncertain
uploads accounted. Confirmation transfers the reservation into the same catalog.
Authorized tenant pages list active uploads within explicit work/result limits;
gateway observations include pending roots without granting deletion permission.
Separate bounded tenant pages list unsettled confirmed objects, retaining deleted
and zero-byte objects until billing stops. Tenant indexing keeps other tenants'
objects out of scan budgets and cursor metadata.
Native tests cover rejection/replay; a test-only PocketIC
fixture exercises actual IC caller checks and gateway revocation. A controlled
source canister tests stale sync replies and reentrant membership changes.
The same local harness executes byte-verification vectors inside Wasm and checks
an explicit instruction budget for ordered chunk appends.
Upload fixtures also check real caller isolation, cancellation and retained uncertainty.

Persisted workflows, provider transports, clients and canister
adapters are not implemented yet. Local bookkeeping and decoded provider reports
do not establish a qualified storage service.

The planned service owns tenant authorization, references, quotas, provider
access, billing, retention and deletion. This repository will own standalone
and Canic-managed adapters using the same handlers and blob API. The core
builds without Canic; Canic owns generic deployment and lifecycle.

Memory dependencies align through the re-exported `ic_memory` crate and its
`ic_stable_structures` substrate. The host owns bootstrap and allocation policy;
linking this library declares no stores or memory IDs. See
[memory composition](docs/dependencies.md#memory-composition-with-canic-and-icydb).

## Local development

Rust 1.98.1 and edition 2024 are pinned. Run `make deps` to fetch locked
dependencies; rustup installs the declared Wasm target, rustfmt and Clippy.
See [dependency setup](docs/dependencies.md) for PocketIC provisioning.

| Command | Check |
| --- | --- |
| `make check` | Compilation |
| `make fmt-check` | Formatting |
| `make clippy` | Strict workspace linting |
| `make test-native` | Native core tests and doctests |
| `make test-pocketic` | Build and run the local authority and sync fixtures |
| `make test` | Both suites, sequentially |
| `make cloc` | Rust runtime/test file LOC and test function counts under `crates/` |

Validation uses offline Cargo and this repository's `target/`. These checks
make no provider calls or network deployments. PocketIC installs a test-only
local canister; production service journeys remain unimplemented.
Release/publication preserve build output. Only explicit
`make clean` removes it.

Library publication to crates.io is enabled through the maintainer's
[release workflow](docs/releasing.md); it does not establish service readiness.
See [development governance](docs/governance/development.md) for command and
validation authority.

The optional LOC helper requires `cloc` and `jq`, and also works when invoked
outside the repository. Classification is by file path: inline test LOC remains
in `runtime_loc`, while `inline_fns` reports those test functions separately.
Canister fixtures and the harness outside `crates/` are excluded from this report.

## Documentation

- [Current status](docs/status/current.md): implemented scope, validation and next work.
- [Service contract](docs/service-contract.md): unresolved design decisions and implementation gates.
- [Canic parity](docs/canic-parity.md): required capabilities, source inventory and removal obligations.
- [Acceptance plan](docs/acceptance-plan.md): observable cases needed to qualify the service.
- [Provider review](docs/provider-review.md): pinned Caffeine findings and missing deployment evidence.
- [Core evidence](docs/evidence/core-primitives.md): implemented boundaries and source-bound test results.

Canic removal requires working replacements and separate installation
retirement evidence. No Canic code has been removed by this repository's work.

## License

MIT. See [LICENSE](LICENSE).
