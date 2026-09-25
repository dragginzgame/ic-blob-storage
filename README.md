# ic-blob-storage

An independent blob-storage service library for Internet Computer canisters.
The core provides content identities, incremental raw-content verification and
pure billing policy. Storage workflows and provider effects are not implemented yet.

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

- `crates/ic-blob-storage`: validated content/provider hash types, chunked raw-byte
  verification, numeric funding limits, and pure funding/readiness policy,
  with native tests.
- [Core evidence](docs/evidence/core-primitives.md): implemented boundaries,
  test references and remaining service coverage.
- [Dependencies](docs/dependencies.md): pinned libraries and local setup.
- [Service contract](docs/service-contract.md): B1 decisions and acceptance map.
- [Extraction readiness](docs/extraction-readiness.md): source inventory,
  required safety corrections and provider evidence gaps.
- [Acceptance plan](docs/acceptance-plan.md): proposed observable service cases.
- [Canic parity](docs/canic-parity.md): runtime, operator and diagnostic
  capabilities required here before Canic removal.
- [Provider review](docs/provider-review.md): pinned Caffeine source findings
  and unresolved deployed-contract evidence.
- [Provider baseline](docs/provider-baseline.json): verified latest upstream
  integration target; Canic's historical bindings do not define this contract.
- [Current status](docs/status/current.md): scope, gates and next work.
- [Agent instructions](AGENTS.md): repository boundaries and workflow.

Only the core package is scaffolded. Protocol, client and adapter package names
will be fixed during B1. Cargo permits publication of the implemented library
to crates.io through the explicit maintainer release/publication workflow.
Library publication does not establish service qualification. The public repository is
[dragginzgame/ic-blob-storage](https://github.com/dragginzgame/ic-blob-storage).

## Local development

Rust 1.98.1 and edition 2024 are pinned for the bootstrap. A lower supported
Rust version has not been qualified.

Run `make deps` once to download the locked Rust dependencies. The toolchain
file also installs the Wasm target, rustfmt and Clippy through rustup.

Run `make check` for compilation, `make fmt-check` for formatting and
`make clippy` for strict library linting. The Makefile
uses this repository's own `target/`; validation runs Cargo offline. No provider calls
or canister deployments are part of these checks.
Release and publication retain build artifacts. Run `make clean` only when you
intend to remove them.

The familiar maintainer release commands are available: `make patch`,
`make minor`, `make release-patch` and `make release-minor`, plus major
and exact-version variants. Use `make release-plan VERSION=minor` to preview
without effects. See [the release guide](docs/releasing.md) and
[development governance](docs/governance/development.md).

Run `make test` for native behavior and documentation tests. Canister and
lifecycle tests must use PocketIC; none are implemented yet. Native billing
diagnostics do not establish upload authority or deployed provider guarantees.

## Extraction boundary

The planned Canic hard cut removes all blob-specific production surfaces and
dependencies. This bootstrap does not remove, copy or modify Canic code.
Existing installations must account for external objects, pending effects,
balances and ongoing billing before their local obligation records are erased.

The [Canic 0.111 design](../canic/docs/design/0.111-standalone-blob-service-extraction/0.111-design.md)
is the extraction coordination authority in the sibling checkout. Human 0.110
acceptance is recorded in this repository's handoff. The B1 contract remains
open. The maintainer authorized the bounded identity/pure-policy slice before
B1 closure; service workflows and provider integration still depend on its
remaining evidence and decisions.

## License

MIT. See [LICENSE](LICENSE).
