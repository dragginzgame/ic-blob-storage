# Current status

Date: 2026-09-25

## Latest continuation

The maintainer requested continuation through 0.1.1. Its named, undated
[changelog draft](../../CHANGELOG.md) now groups the content identities,
incremental verification, pure billing policy, registry evidence and release
helper fix. Changes are additive relative to the API-free 0.1.0 scaffold; no
existing public API or semantic contract is removed by this patch candidate.

Release preparation uncovered and fixed a helper bug: undated historical 0.1.0
notes were incorrectly treated as a competing draft. The helper now preserves
undated entries at/below the current package version and rejects other future
drafts using numeric version order. Regression tests cover both Unreleased and
named-target preparation and byte-for-byte preservation of historical notes.
The real 0.1.0 notes remain unchanged.

`make release-verify` passes on the implementation candidate: shell/helper
checks, formatting, compilation, strict Clippy, docs, native tests/doctests,
Wasm and real offline package verification. This is the full current repository
gate, not provider/service qualification. Publication metadata warnings remain
expected while package ownership/publication are unsettled.

`make bump-x VERSION=0.1.1` stops at its clean-source check because this batch is
uncommitted. Cargo and the lockfile remain at 0.1.0, and no release receipt was
generated. The next required action is the maintainer's source commit of this
reviewable batch. Agents must not create that commit. After the tree is clean,
the already-authorized exact preparation command can run, revalidate committed
source, bump Cargo/lockfile, date the draft and generate `docs/release.json`.
No additional version-approval question is needed; no tag/push, registry upload,
deployment, paid provider effect or Canic removal was performed.

The maintainer pushed the setup/changelog as commit `14d5972` (`0.1.0`);
the checkout was clean and main matched origin/main at continuation start.
No version or publication transaction was run by the agent.

Anonymous query calls to the official Mops registry now confirm backend
`caffeineai-object-storage` 1.1.1 as the highest published version. The
manifest and both backend source-file hashes match the pinned Caffeine source.
The [registry evidence](../evidence/caffeine-mops-verification.json) retains
responses, hashes, query identity/network and exact registry source provenance.
This closes the package-publication gap only; deployed gateway/Cashier and
recovery/economic evidence remain missing. No update or paid provider call ran.

The maintainer answered the scoped implementation question with “yes please
keep going”, explicitly authorizing content identities/hash parsing and pure
funding/readiness policy with native tests before B1 closes. That bounded slice
is now implemented: distinct raw SHA-256/provider-root types, strict canonical
parsing, validated numeric limits, full-request reserve checks, typed balance
failures, blockers/warnings and recovery-fence reporting. These are domain values
and diagnostics, not upload or payment authority. Provider calls/bindings,
persisted workflows, publication and Canic removal remain gated.

[Core evidence](../evidence/core-primitives.md) retains exact source hashes,
test references and passing targeted native/Clippy/Wasm/documentation checks.
The notes are now grouped in the named 0.1.1 draft above; no version change or
commit was made.

The next continuation extends the approved content-identity slice with
`ContentVerifier`: incremental SHA-256 verification against a fixed digest and
declared byte length. Exact offsets reject replayed/skipped chunks; oversized
chunks leave hash/count unchanged; completion rejects truncation and corruption.
Memory use does not grow with object size. The SHA-256 length bound is enforced,
but service object/session limits remain separate. Native tests include a fixed
million-byte vector, varied chunk boundaries and rejection/recovery cases.
Native tests, strict Clippy, Wasm, docs and formatting pass. This is transient
raw-byte verification, not persisted resume, provider-tree computation or proof
of storage. No provider integration or B1 scope expansion was made.

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
tests. BLOB-01 now has native identity evidence; BLOB-08/BLOB-11/BLOB-12 have
partial numeric/pure-policy evidence. Other replacement lists remain empty.
Operator functionality is explicitly in scope; acceptance A11/A12 cover operator
parity and removal readiness. No Canic mutation or removal is authorized.

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
sent and no paid provider effects occurred. B1 remains open and gates work beyond
the explicitly authorized core slice.

The latest maintainer direction requires the latest official Caffeine storage
integration, because Canic may have drifted. The
[provider baseline](../provider-baseline.json) now records the verified npm
`latest` client 1.1.2 and current official backend source manifest 1.1.1 at
`e5cacdfe5ce55e939edb02980fca800c0c13f421`. Registry archive SHA-512 integrity
passes; the published client file matches the reviewed artifact's SHA-256.
Latest Mops publication is now verified by the evidence above; deployed
gateway/Cashier versions remain unverified.
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

Human 0.110 acceptance is resolved for work here. B1 is still open; only the
bounded core exception above is implemented. Caffeine is not qualified. The next
useful work is obtaining the intended deployed provider interface and authoritative
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
locations remain unassigned. That earlier setup did not run the full gate;
the current 0.1.1 preparation has now passed `make release-verify` as above.
That earlier evidence establishes scaffold/tooling behavior only. The new
core evidence additionally establishes the bounded native behavior above,
without claiming service or provider qualification.

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
the installed maintainer commands. Content types and pure policy are available;
service workflows, provider effects and canister endpoints are not implemented.
