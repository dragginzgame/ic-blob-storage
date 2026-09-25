# Current status

Date: 2026-09-25

The maintainer authorized creating and setting up
`/home/adam/projects/ic-blob-storage`, sibling-style AGENTS/release tooling,
and a public `dragginzgame/ic-blob-storage` remote. The initial maintainer
commit is now on main with origin/main tracking. The latest request is to
read Canic's 0.111 design and begin work here. When asked about pending 0.110
acceptance, the maintainer confirmed it and directed continued work in the
order judged appropriate. Work is confined to this repository; Canic remains
read-only. No consumer application mutation is authorized.

## Full-functionality preservation requirement

The maintainer requires all Canic blob functionality to be ready here before
removal there. The [parity contract](../canic-parity.md) and structured
[capability inventory](../canic-capabilities.json) now map all public blob API
methods and emitted endpoints, plus lifecycle, operator, target-resolution,
diagnostic and declaration capabilities to replacement boundaries and source
tests. Every replacement evidence list is still empty. Operator functionality
is explicitly in scope; acceptance A11/A12 cover operator parity and removal
readiness. No Canic mutation or removal is authorized.

The inventory exposes an evidence limit: Canic's scripted operator loop is a
substitute, and its installed-CLI blob proof expects a Coordinator-routing
rejection. Neither is a successful live operator journey. Replacement tests
must use the real operator transport against both PocketIC deployments.
Source API coverage, endpoint/test references and source hashes were checked;
this is inventory validation, not service qualification.

The provider review now includes the official retail storage-billing guidance
and a bounded public-repository search. The guidance does not supply the
required cycle-account/deletion/restore proof. The missing authoritative
deployment/source facts are listed in
[the provider evidence request](../provider-evidence-request.md). An async
question to the maintainer requests that reference; no external message was
sent and no paid provider effects occurred. B1 remains open and still gates B2.

The latest maintainer direction requires the latest official Caffeine storage
integration, because Canic may have drifted. The
[provider baseline](../provider-baseline.json) now records the verified npm
`latest` client 1.1.2 and current official backend source manifest 1.1.1 at
`e5cacdfe5ce55e939edb02980fca800c0c13f421`. Registry archive SHA-512 integrity
passes; the published client file matches the reviewed artifact's SHA-256.
Latest Mops publication and deployed gateway/Cashier versions are not verified.
Use upstream as the integration authority; Canic bindings are historical
extraction evidence. Recheck upstream before implementation and qualification,
then update exact pins and affected evidence together. No moving latest tag
will be resolved during builds. This selects a target, not a qualified provider
or an implemented service.

## Dependency setup

The maintainer then authorized dependency setup. The workspace now pins Candid,
Serde, SHA-256, typed errors, IC CDK and stable structures, plus native-only
PocketIC 16. The [dependency guide](../dependencies.md) records versions and
ownership. PocketIC requires `thiserror` 2.0.18; all other direct crates use
the current stable versions verified from crates.io. The lockfile is resolved
and fetched; `make deps` repeats the locked fetch without changing versions.
The toolchain now declares the Wasm target as well as rustfmt and Clippy.

Native all-target compilation and the Wasm library check pass offline and
locked. The subsequent parity work installed the verified PocketIC 16.0.0
Linux x86_64 server under `.tmp/tools/`, checked its release-asset digest and
version, and retained [tool provenance](../evidence/pocketic-toolchain.json).
Make now exports its explicit path as the overridable `POCKET_IC_BIN` default,
preventing automatic server downloads during tests. No server instance or
canister test ran. No npm/Motoko dependency or Canic dependency was added.
This is dependency preparation, not B2 service implementation.

## Extraction planning started

- Read Canic's 0.111 design/tracker, current handoff, earlier provider inventories
  and the maintained blob API, state, billing, endpoint and test sources.
- Captured a source/hash inventory at Canic commit
  `10d00c6d9494a45b30e66f84d1bd886c8acdd45c`; its tracked checkout was clean.
- Added [extraction readiness](../extraction-readiness.md) with proposed package
  boundaries, behavior classification, removal groups, installation obligation
  requirements and provider gaps. Named owners and the consumer remain open.
- Added [acceptance cases A01–A12](../acceptance-plan.md), covering both adapters,
  tenant authority, content, capacity, uncertainty, restore, release races,
  economics, retirement, serving, operator parity and removal readiness.
  These are planned cases, not passing tests.
- Located official Caffeine integration source at
  `caffeinelabs/skills@e5cacdfe5ce55e939edb02980fca800c0c13f421`.
  The [provider review](../provider-review.md) records interface drift from
  Canic's snapshots, public URL access and incomplete completion/economic proof.
  An isolated client hashing check records distinct raw/provider hashes,
  metadata-dependent roots and the pinned client's empty-tree rejection.

Human 0.110 acceptance is resolved for work here. B1 is still open, so no B2
service implementation has begun. Caffeine is not qualified. The next useful
work is obtaining the intended deployed provider interface and authoritative
retry/deletion/billing evidence, then freezing bounds, recovery and owners.
The first proposed journey uses public assets; the concrete consumer is still
unassigned. Do not infer private byte delivery from tenant authorization.

## Completed setup

- Rust 2024 workspace and ic-blob-storage library scaffold,
  pinned to Rust 1.98.1 with isolated build output.
- MIT license, expanded normative AGENTS, development/release governance,
  changelog and B1 service-contract checklist.
- Patch/minor/major and exact-version preparation, corresponding one-shot
  maintainer releases, individual staging/commit/push commands and separate
  registry publication commands. Effect-free release-plan is available.
- Release preparation validates clean committed input, updates Cargo/lockfile
  and changelog, and generates a source-bound record with release-file hashes.
  Failed preparation restores its inputs. One-shot success ends with cargo clean.

The package remains version 0.1.0 and publish=false. GitHub repository setup
and the maintainer's initial commit/push are separate from service or registry
publication. No version transaction or service publication has run.
Commit-producing targets remain human-only; agents must never execute them.

## Validation

Bootstrap formatting and strict package Clippy passed previously. The release
tooling now passes Bash syntax, ShellCheck, Perl syntax and focused regression
tests for version selection, first-release handling, changelog finalization,
dirty/tagged-source rejection, failed validation, concurrent source changes,
failed lockfile/metadata updates, rollback, staging and receipt tampering.
Substituted Git/Cargo/validation commands also prove one-shot ordering, exact
atomic push selection, success cleanup and failed-push continuation. These
tests create no real commits or network effects.

Make help, release previews and dry-run command graphs resolve. Real offline
Cargo packaging and package compilation pass; Cargo reports the expected
missing repository/homepage/documentation metadata while those publication
locations remain unassigned. Full CI/release validation was not run.
All evidence establishes scaffold/tooling behavior only.

The extraction-planning batch additionally checks local document links, source
inventory hashes, whitespace and the pinned client's isolated hashing behavior.
It runs no Rust build, PocketIC, full CI/release gate or live provider operation.
Client observations do not establish deployed provider guarantees.

## Remaining gates and next work

Finish B1 with named service/consumer/operator owners, the
concrete application, final package/publication plan, removal and obligation
inventories, and actual-provider evidence. Freeze the
[service contract](../service-contract.md) covering deployment, authority,
identity, accounting, restore and retirement. Canic removal remains B3 work
after qualified service publication.

Before enabling registry publication, assign ownership/package metadata
and extend release-verify with the implemented service's actual PocketIC and
recovery qualification. Read [the release guide](../releasing.md) before using
the installed maintainer commands. No service API, provider effects or
canister endpoints are implemented.
