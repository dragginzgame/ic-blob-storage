# ic-blob-storage

An independent blob-storage service library for Internet Computer canisters.
This repository is bootstrapped; storage behavior is not implemented yet.

The service will own application data, tenant authorization, content identities
and references, quotas, provider access, billing, retention and deletion.
Canic will own deployment and lifecycle through its generic Component contract.

Two deployments will use the same service implementation and blob API:

- A standalone service canister that builds and operates without Canic.
- A Canic-managed canister embedding the service through thin adapters, with
  the additional required Canic lifecycle endpoints.

This repository will own both adapters. The core will remain buildable without
Canic; neither adapter may duplicate storage workflows or tenant policy.

## Current layout

- `crates/ic-blob-storage`: dependency-free library scaffold; no exported API.
- [Service contract](docs/service-contract.md): B1 decisions and acceptance map.
- [Current status](docs/status/current.md): scope, gates and next work.
- [Agent instructions](AGENTS.md): repository boundaries and workflow.

Only the core package is scaffolded. Protocol, client and adapter package names
will be fixed during B1. The local `0.1.0` version is an unpublished scaffold,
and Cargo publishing is disabled. No remote repository is configured.

## Local development

Rust 1.98.1 and edition 2024 are pinned for the bootstrap. A lower supported
Rust version has not been qualified.

Run `make check` for compilation, `make fmt-check` for formatting and
`make clippy` for strict library linting. The Makefile
uses this repository's own `target/` and runs Cargo offline. No provider calls
or canister deployments are part of these checks.

The familiar maintainer release commands are available: `make patch`,
`make minor`, `make release-patch` and `make release-minor`, plus major
and exact-version variants. Use `make release-plan VERSION=minor` to preview
without effects. See [the release guide](docs/releasing.md) and
[development governance](docs/governance/development.md).

Behavioral tests will accompany implementation. Canister and lifecycle tests
must use PocketIC. Passing scaffold checks establishes buildability only.

## Extraction boundary

The planned Canic hard cut removes all blob-specific production surfaces and
dependencies. This bootstrap does not remove, copy or modify Canic code.
Existing installations must account for external objects, pending effects,
balances and ongoing billing before their local obligation records are erased.

The [Canic 0.111 design](../canic/docs/design/0.111-standalone-blob-service-extraction/0.111-design.md)
is the extraction coordination authority in the sibling checkout. Its B1
contract remains open; bootstrap authorization does not accept the 0.110
closeout or begin B2 implementation.

## License

MIT. See [LICENSE](LICENSE).
