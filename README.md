# ic-blob-storage

An independent blob-storage service library for Internet Computer canisters.
The core currently provides content identities, incremental raw-content
verification, billing/configuration validation, bounded gateway membership and pure billing
policy with native tests. A transient lifecycle model binds references, deletion
and settlement to specific tenants and object incarnations, with bounded request
receipts and a pure access check for direct tenant callers. Bounded Caffeine
reply decoders preserve funding failures and distinguish completion reports from
verified storage; immutable local root claims prevent reassignment. Persisted
workflows, provider transports, clients and canister adapters are not implemented yet.

The planned service owns tenant authorization, references, quotas, provider
access, billing, retention and deletion. This repository will own standalone
and Canic-managed adapters using the same handlers and blob API. The core
builds without Canic; Canic owns generic deployment and lifecycle.

## Local development

Rust 1.98.1 and edition 2024 are pinned. Run `make deps` to fetch locked
dependencies; rustup installs the declared Wasm target, rustfmt and Clippy.
See [dependency setup](docs/dependencies.md) for PocketIC provisioning.

| Command | Check |
| --- | --- |
| `make check` | Compilation |
| `make fmt-check` | Formatting |
| `make clippy` | Strict library linting |
| `make test` | Native tests and doctests |

Validation uses offline Cargo and this repository's `target/`. These checks
make no provider calls or canister deployments; PocketIC service tests are
not implemented yet. Release/publication preserve build output. Only explicit
`make clean` removes it.

Library publication to crates.io is enabled through the maintainer's
[release workflow](docs/releasing.md); it does not establish service readiness.
See [development governance](docs/governance/development.md) for command and
validation authority.

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
