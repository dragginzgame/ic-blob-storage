# AGENTS.md

This file is normative for automated contributors.

## Session handoff and scope

- Read docs/status/current.md first. Continue from that handoff rather than
  reconstructing old conversation history.
- Work only in this repository. Sibling repositories, including Canic,
  ic-memory and ic-timers, are read-only unless separately named and authorized.
  Inspect/review/audit requests never authorize sibling edits.
- Preserve unrelated dirty worktree state.
- The bootstrap does not accept Canic's 0.110 closeout. Follow the remaining
  implementation gates recorded in the status and service contract.

## Delivery and release

- Follow [development governance](docs/governance/development.md) for command
  authority, validation, batch cadence, changelog and release policy.
- Never create or amend Git commits, including through scripts or one-shot
  Make targets. The maintainer owns commits.
- Do not infer versioning, publication, deployment or paid provider authority
  from ordinary implementation, continuation or push-readiness requests.
- An explicit maintainer instruction is sufficient for the exact action it
  names, subject to the commit prohibition. Do not demand a magic phrase or
  repeat confirmation when the target and effect are clear.
- Keep implementation, adversarial/recovery evidence, propagation and cleanup
  together. Do not allocate one version per focused slice.
- Maintain CHANGELOG.md for completed meaningful code/behavior/tooling changes.
  Extend the current draft; leave version mutation to the requested release flow.
- Read [the release guide](docs/releasing.md) before version or publication work.
  Agents may inspect plans and test helpers, but must not execute
  release-commit or commit-producing release-* targets.
- Registry publication is enabled for crates.io and remains an explicit
  maintainer action. B1 ownership and service qualification do not gate library
  publication; publishing the package does not qualify the service.
- Release and publication commands preserve build artifacts. Run cleanup only
  when explicitly requested; never append cargo clean to a release/deployment.

## Pre-1.0 hard cuts

- Remove superseded APIs, schemas, state paths and tests completely. No
  deprecated aliases, shims, dual readers, legacy fallbacks or migration engine
  unless the maintainer explicitly authorizes an exception.
- Breaking public API or semantic changes require a minor version; a hard cut
  does not justify publishing an incompatible patch.
- Product protocol/config/stable-state generations remain v1 before 1.0.
- Cross-release transitions are reinstall-only. Same-release interruption
  recovery, retry, backup and restore remain required within the frozen contract.
- Source allocation removal and installation retirement are separate. Never
  erase the only records of provider objects, uncertain effects, balances or
  continuing billing. Apply the retirement contract before reset.
- No anti-resurrection tests for removed forms; test the maintained contract.

## Ownership and layering

- The service core builds without Canic and owns tenant policy, data,
  references, quotas, provider economics, retention and deletion.
- This repository owns both standalone and Canic adapters. Both use the same
  service handlers, blob API and tenant rules; neither duplicates workflows.
- Canic owns generic deployment/lifecycle. Do not add blob-specific production
  dependencies back to Canic.
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
- Document public types and meaningful invariants. Prefer expect over allow
  for lint suppressions. Keep authority predicates readable and independently
  testable rather than long mixed boolean expressions.
- Check for an active build before compilation or source mutation; never
  compete for a build lock. Use this repository's own target directory.
- Run targeted checks during development. Do not infer full CI/release
  validation authority from generic continuation or readiness wording.
- Keep unit tests next to code and integration tests in tests/. Canister
  creation/install/lifecycle/inter-canister tests use PocketIC.
- Assert typed errors or observable behavior, not error text or fixed aggregate
  test counts. Do not fake platform behavior through production cfg(test).
- Guards enforce structured facts, versions, hashes and executable behavior;
  never freeze explanatory prose or require manual status-marker rotation.
- No Python tooling. Keep shell wrappers small; use Rust for substantial
  durable tooling. The release helper uses Perl for bounded text/JSON handling.
