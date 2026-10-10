# AGENTS.md

This file is normative for automated contributors.
Read and apply [the shared engineering baseline](DRAGGINZGAME.md) first.
[The snapshot manifest](.shared-tooling.snapshot) records its reviewed revision
and exact files. This file is the local overlay for service contracts and commands;
it does not override shared rules or depend on a mutable sibling checkout.

## Session handoff and scope

- Read docs/status/current.md first. Continue from that handoff rather than
  reconstructing old conversation history.
- Work only in this repository. Sibling repositories, including Canic,
  ic-memory and ic-timers, are read-only unless separately named and authorized.
  Inspect/review/audit requests never authorize sibling edits.
- GitHub issues in the owning repository track integration and shared-tooling
  feedback. Handoffs link to them; evidence remains with its existing owner.
  Relevant issue work follows the shared maintenance rules. Sibling file edits,
  unrelated messages and release effects retain their separate authorization
  requirements.

## Delivery and release

- Follow [development governance](docs/governance/development.md) for command
  authority, validation, batch cadence, changelog and release policy.
- Read [the release guide](docs/releasing.md) before version or publication work.
  Agents may inspect plans and test helpers, but must not execute
  release-patch, release-minor, release-major or release-resume: the common
  runner owns commit creation, including interruption recovery.
- Registry publication is enabled for crates.io and remains an explicit
  maintainer action. B1 ownership and service qualification do not gate library
  publication; publishing the package does not qualify the service.

## Service hard-cut disposition

- Cross-release transitions are reinstall-only. Same-release interruption
  recovery, retry, backup and restore remain required within the frozen contract.
- Source allocation removal and installation retirement are separate. Never
  erase the only records of provider objects, uncertain effects, balances or
  continuing billing. Apply the retirement contract before reset.

## Ownership and layering

- Prefer one authoritative storage owner and its local durable journals. Defer
  external journals, recovery-controller canisters and extra metadata calls until
  a clear consumer/recovery use case justifies them. This preference does not
  authorize stale-backup activation, clearing fences or losing obligations.
- The service core builds without Canic and owns tenant policy, data,
  references, quotas, provider economics, retention and deletion.
- This repository owns the service core and standalone adapter. Consumer
  frameworks own their wrappers and integration tests in their own repositories.
  Do not add downstream framework dependencies, adapter crates, fixtures or tests
  here. Linking the core must not take endpoint or lifecycle ownership.
- Dependency direction: endpoints call workflow; workflow calls policy and ops;
  ops may call model. Policy never calls ops.
- DTOs are passive boundary data. Request/mutation DTOs have no Default unless
  neutral. Records own persisted schemas and end in Record; views are read-only.
- Model owns storage invariants. Ops owns state access, conversion and approved
  single-step platform effects. Workflow orchestrates without constructing or
  mutating records. Policy is pure: no async, storage, DTOs or serialization.
- Endpoint adapters authenticate and delegate. Tenant, actor, service and
  provider-namespace bindings must be explicit. Controller status and content
  digests are not tenant authority.
- Linking a library must not silently export endpoints or seize lifecycle
  ownership. Lifecycle adapters restore synchronously before deferred work.

## Safety and evidence

- Caffeine qualification proceeds independently of provider cooperation. Read and
  maintain [the probe ledger](docs/evidence/caffeine-probes/README.md) whenever
  investigating provider behavior. Record intent/target/budget before a probe;
  retain requests, results, hashes, failures, limitations and outstanding cleanup
  afterward. Record source review, local substitutes and live observations as
  different evidence classes. Never silently rerun, overwrite or omit a failed
  or inconclusive probe. This requirement does not authorize paid effects.
- Persist intent before effects; bind retries to exact operation identity.
  Expired evidence never authorizes repeating an uncertain paid effect.
- Fence restored/stale instances until reconciliation proves safe identity
  allocation and accounting. A counter from the same old backup is insufficient.
- Bound objects, references, sessions, receipts, reservations and liabilities.
  Releasing tenant quota cannot erase still-stored bytes from global accounting.
- Distinguish logical release, provider deletion and billing cessation.
- Describe only guarantees backed by evidence. Label provider substitutes;
  they do not establish the deployed provider's behavior.
- Provider request/callback authority has one owner. Do not publicly expose
  credentials or recreate provider contracts independently in clients.

## Style and checks

- Use Rust edition 2024, directory modules with mod.rs and named boundary types.
  Do not use path attributes to work around module layout.
- Maintainer-approved physical-layout exception (2026-10-07), under
  [the shared workspace rule](rules/rust-workspaces.md): keep the library and CLI
  in `crates/`, standalone and probe canisters in `canisters/`, and packaged test
  support in `tests/pocketic/` and `tests/protocol/`. The maintainer explicitly
  reversed the crates-only moves. Do not relocate these packages during routine
  adoption; a future redesign requires separate explicit approval. All 12 members
  still share one virtual root, lockfile and inherited versions/dependencies.
  Formatters and source/release inventories cover Cargo's complete member roster.
  Ordinary browser tests and frozen inputs remain under `tests/`.
- Document public types and meaningful invariants. Prefer expect over allow
  for lint suppressions. Keep authority predicates readable and independently
  testable rather than long mixed boolean expressions.
- Run targeted checks during development and the documented full validation
  suite before delivering code as ready, under the adopted shared baseline.
  Inspection-only tasks do not authorize that suite; release effects retain
  their separate explicit authority.
- Keep unit tests next to code and integration tests in tests/. Canister
  creation/install/lifecycle/inter-canister tests use PocketIC.
- Use Rust for substantial durable tooling. The release helper uses Perl for
  bounded text/JSON handling.
