# Current status

Date: 2026-10-18

## Source and scope

The delivered baseline is released 0.34.0 at
`926e91fe6fe98bb1caba6a3a343b5f940b4db714`; workspace versions are 0.34.0.
That release includes the provider decoder move, Platform 0.1.11, Shared's
reduced test roots and development Rust 1.99.1. Earlier receipts retain their
original sources and graphs; they are not fresh results for this release.
This maintenance continuation changes documentation only. It does not change
dependencies, package versions, release metadata or provider state.

The [archived handoff](handoff-2026-10-18.md) preserves all earlier source-bound
results, failed attempts and consumer prerequisites. Its pending-release and
“next work” statements belong to their recorded batches. Original evidence stays
with its owner; archived release labels never qualify current bytes.

## Ownership boundary

Blob owns tenant policy, data/references, quotas, provider economics, retained
obligations and recovery; Contracts owns passive DTOs, method catalog, application
codecs and verification. Standalone owns explicit endpoints, authentication,
lifecycle and memory custody. Native CLI owns Blob operations and their evidence.
Linking the core never mounts endpoints or lifecycle hooks. Consumer frameworks
own their wrappers and installed integration qualification in their own repos.

Memory 0.48.2 owns allocation/backing governance; Storage 0.5.3 owns collections
and codecs. The delivered lock selects Platform 0.1.11 for supported principal,
Candid/native Agent/certificate, entrypoint and call/system APIs. No direct
Candid/CDK exceptions remain; runtime implementation retains its upstream owner.
Wire 0.4.1 owns the two tenant method frames. Other 29 owned methods and standalone
lifecycle inputs still use bounded CBOR. Provider/management Candid and persisted
record encodings retain their separate contracts. No storage or wire migration
was part of that release's cleanup.

## Delivered dependency and tooling adoption

- Published Platform 0.1.8 owns configured/staged decoding and supported
  record/service generator paths. Core, Contracts, protocol support and CLI drop
  direct Candid. Native Agent/certificate/parsing imports use the supported
  published APIs. Dynamic native fixture serializers and Agent response enums now
  use Platform too; native direct Candid and all direct Agent declarations are
  removed. CDK macros/runtime retain documented
  exceptions and owning issues in
  [the dependency guide](../dependencies.md#ic-platform-boundary).
  Management-history DTOs now route through Platform's exact upstream identities;
  the direct dependency and exception are removed. The
  [0.1.8 receipt](../evidence/platform0180331.md) records its original published
  selection and recovery/graph checks; new snapshot/status DTOs add no Blob
  execution path. The
  [0.1.7 receipt](../evidence/platform0170331.md) retains its native fixture and
  signed-update qualification against its original graph. The
  [0.1.6 receipt](../evidence/platform0160331.md) retains its published API and
  installed recovery checks against its original graph. The
  [0.1.5 receipt](../evidence/platform0150331.md) retains its earlier native/MSRV,
  vector/example and thirteen installed-case evidence against its original graph.
- Shared Tooling 0.24.0 at `cab12cd3dfa4fdb34846b25397f0d72881bb9ae0` now has
  131 selected payloads after the delivered test-root retirement
  (137 in released 0.33.1), including the direct-boundary checker. Role
  labels, grouped formatting and prepared/selected-commit release checks agree.
  Common Cargo setup omits the unused extractor executable.
  [The adoption receipt](../evidence/shared0240331.md) records the initial snapshot
  qualification; [Platform adoption](../evidence/platform0331.md) records adding
  the checker. That audit retained Host 0.27.2, Build 0.17.1 and Testkit 0.53.0.
- Delivered Testkit 0.54.0 uses Platform decoder limits and
  typed causes; the retired decoder
  dependency and its allowance are removed. All existing reply budgets remain.
  Incoming Host 0.27.3, Build 0.17.3 and Wire 0.4.1 selections are preserved.
  Published Testkit source is `ff88087d6757121ac92e0dec4705ecaccfd8ebed`; its archive checksum is
  verified against the lock. Native all-target/all-feature checks and strict lint,
  Rust 1.91 checks, three installed account/gateway cases, fresh standalone/peer
  Wasms and boundary/format/tool checks pass. Native macOS/full CI are not rerun.
  Raw before-images and checks remain under `.tmp/review-validation/testkit0540331/`.
- [The fourth ownership audit](../reports/audits/2026/10/18/architectural-sediment/04/report.md)
  finishes the substrate declaration guard and runtime-free Storage exclusion.
  It routes trial preparation to the maintained strict-JSON-to-CBOR command and
  separates historical trial state from current host instructions. Source,
  before-images, fixture regressions and focused checks remain under
  `.tmp/review-validation/sediment0331-04/`.

## Current checks and next work

The full open backlog and PR discussions were reviewed against default main
0.34.0. [The 0.33.1 tooling matrix](https://github.com/dragginzgame/ic-blob-storage/actions/runs/39126998913)
now passes Linux, Apple Silicon, Intel macOS and MSRV. That completes the
previously unfinished delivered-source acceptance for
[#82 — Review the unused core dependency in the gateway-source fixture](https://github.com/dragginzgame/ic-blob-storage/issues/82).
That issue is closed. The delivered Memory identity/release guidance in
[#77 — Align exposed Memory types with 0.41 and classify the public hard cut](https://github.com/dragginzgame/ic-blob-storage/issues/77)
and producer/standalone workflow ownership in
[#72 — Own shared host continuity and certificate workflows used by Canic and standalone](https://github.com/dragginzgame/ic-blob-storage/issues/72)
are also complete and closed; Canic's newer graph/managed acceptance remains
with its linked owners below, rather than keeping completed Blob work open.

The [0.34.0 tooling workflow](https://github.com/dragginzgame/ic-blob-storage/actions/runs/39138592222)
passes Linux and MSRV at inspection; Apple Silicon and Intel macOS remain in
progress. Its [published-example workflow](https://github.com/dragginzgame/ic-blob-storage/actions/runs/39138592220)
passes all three hosts. These observations do not declare the unfinished matrix
green. Native compiler acceptance remains with #81 below.

The released 0.34.0 provider codec consolidation moves chunk-reply decoding and
its status/error types from Core's public `ops::caffeine::upload` module to
`ic_blob_storage_contracts::provider::upload`, with no compatibility re-export.
The decoder and its tests are byte-identical; limits, JSON refusals and provider
authority semantics are unchanged. Core drops direct `serde_json`; the CLI drops
unused development Testkit while retaining its required JSON dependencies.
The lockfile changes only those two edges. Core, Contracts and CLI tests pass;
three CLI loopback fixtures initially failed sandbox socket admission and pass
outside that restriction. Strict scoped Clippy, native Rust 1.91 checks for all
three packages, Rust 1.91 Wasm checks for Core/Contracts/standalone, formatting,
dependency boundaries/pins and documentation links pass. The CLI's inspected
all-feature/all-target normal/build/dev graph no longer includes Testkit.
Logs and starting files remain under
`.tmp/review-validation/dependency-consolidation0340/`. Full CI, native macOS and
installed-IC qualification were not run in that local relocation pass; hosted
release checks are separately bounded above.

[The fifth ownership audit](../reports/audits/2026/10/18/architectural-sediment/05/report.md)
corrects CLI/browser format guidance left by the JSON/CBOR cut and removes a
doctest that only asserted an old model path was absent. Three actual offline
installation cases and the four remaining boundary doctests pass. It preserves
the current graph, all runtime implementations, independent vectors and recovery
journals; no further production deletion was justified. Before-images and scoped
checks remain under `.tmp/review-validation/sediment0332-05/`.

Published Platform 0.1.11 finishes the runtime boundary: its exact upstream
cycle/counter helpers replace the last direct CDK imports in core and five probes.
All direct Candid/CDK declarations and exceptions are removed, including the root
catalog. The policy rejects their reintroduction and direct internal macro access.
[The adoption receipt](../evidence/platform01110332.md) records published identities
and focused qualification; [0.1.10](../evidence/platform01100332.md) retains its
earlier macro migration evidence and now-resolved helper gaps. Existing Wire/CBOR,
provider Candid, runtime/executor identities, storage and memory are unchanged.
The 0.1.11 graph passes strict lint, native/Wasm Rust 1.91 checks, seven fresh
Wasms, 348 core tests, five installed funding/counter/lifecycle cases and separate
provider DID/export parity. The hook's dependency-sorting fixture now selects
remaining catalog entries; sorting, rollback and lock preservation pass.

Released 0.34.0 adapts [PR #83 — Keep generic installer and
release-runner regressions at Shared](https://github.com/dragginzgame/ic-blob-storage/pull/83)
to the unchanged Shared 0.24 revision. Canonical export retires five generic test
roots and their unused cache-key companion; Blob retains actual tool admission,
Make/Testkit forwarding, hooks, evidence and release/publication recovery checks.
All retained focused regressions pass. PR #83 is closed as already incorporated:
its older branch has no unique change left to merge, and its original hosted
matrix passes. Current 0.34.0 native acceptance remains separately bounded above.
PRs #73, #78 and #80 were already incorporated; the old Testkit 0.51 draft #79
was superseded.

[#81 — Qualify Rust 1.99.1 as the development compiler](https://github.com/dragginzgame/ic-blob-storage/issues/81)
is delivered in 0.34.0, including the frozen-example CI override. MSRV remains
1.91; that compiler-only change alters no package/dependency version or lock.
New compiler-bound
formatter/Testkit tool receipts are prepared. Native production-package checks
and strict lint, 348 core plus 118 Contracts unit cases, fresh standalone/peer
Wasms and three installed account/rollback/lifecycle cases pass on Linux.
Formatting, boundary/pin/snapshot/tool checks and the substitute-Cargo example
adapter pass. Keep #81 open until the exact 0.34.0 native macOS jobs above pass;
earlier 0.33.1 used the previous development compiler and cannot qualify this change.
Before-images, canonical export records, CI observations, failed prerequisites
and successful checks remain under `.tmp/review-validation/issues0332/`.

[The issue-maintenance receipt](../evidence/issues0331.md) retains #82's original
fixture-removal evidence. The fix is delivered in 0.33.1, whose exact tooling
workflow is now complete as recorded above. Its source-bound receipt retains
the original pending status; that historical observation is not rewritten. Public DTO
Candid removal remains the separate coordinated hard cut in
[#84 — Remove application DTO Candid coupling after migrating current probes](https://github.com/dragginzgame/ic-blob-storage/issues/84).

For #84, current private probes and their callers still expose application DTOs
through Candid. Migrate those consumers to canonical Wire/CBOR before removing
public traits, and retain provider/management Candid and frozen-image refusal
proofs. The Canic draft below owns its separate codec adoption. This is a new
coordinated public cut after released 0.34.0, not a compatible declaration cleanup.

[#76 — Adopt Host retained-record write completion after publication](https://github.com/dragginzgame/ic-blob-storage/issues/76)
still awaits a published completion API. Host's original PR #132 was consolidated
into [Host PR #172 — Integrate remaining artifact, filesystem and process contributions](https://github.com/dragginzgame/ic-host-tooling/pull/172),
which remains open. Delivered Host FS 0.27.4 supplies retained creation/publication
but not the proposed complete retained write. Keep Blob's bounded producer and
file/directory synchronization until that API is available; an atomic writer's
failure cleanup would discard required partial probe evidence.

The Platform 0.1.8 receipt records focused native and installed recovery checks,
fresh standalone Wasm, native/MSRV/lint/boundary checks and unchanged upstream
feature/dependency selections. Earlier Platform and ownership-audit results retain
their original graphs. Full CI, package publication, native macOS, browser journeys,
live provider behavior and installed Canic acceptance have not been requalified by this batch.
The canonical direct checker observes aliases, normal/build/dev/target-specific
and disabled optional declarations. Contracts/CLI normal graphs reject runtime
owners, including Storage. Automatic standalone/transitive test-tool deployment
exclusions and binding inspected source/lock/features to deployed bytes remain
missing; role labels and direct checks do not supply that admission.

The approved Canic follow-up is now [draft PR #603 — Adopt Blob shared dispatch
and canonical endpoint codecs](https://github.com/dragginzgame/canic/pull/603),
stacked on its protocol-neutral entrypoint contribution #591. The isolated
contribution removes Canic's duplicate workflow, provider access adapters and
continuity checkpoint, retaining host endpoint/lifecycle/memory ownership. All
three maintained lockfiles now resolve published Blob/Contracts 0.33.0, Memory
0.48.2 and Storage 0.5.3. The normal dirty Canic checkout remains untouched.

All 48 actual macro tests and strict macro lint pass. Endpoint expansions compile
against published Blob natively and for Wasm, including Rust 1.91 Wasm. Two
installation byte-contract cases pass with the actual Canic decoder/limits in an
isolated fixture. Framework substitutes limit those checks to compilation and
byte contracts. Full adapter checking reaches Canic Core and then fails on 15
Memory caller errors; actual dedicated/embedded managed installation and retained
data restoration remain unqualified. Finish that existing Memory adoption before
landing the draft. The [follow-up receipt](../evidence/canic-dispatch0331.md)
records exact source identities, successful checks and failed attempts. Earlier
[implementation evidence](../reports/audits/2026/10/18/architectural-sediment/03/report.md)
retains its original unpublished dependency and publication-blocker status.

Retain the allocation-instrumented private probe until Memory provides an
allocation-level observation/fault hook that preserves attribution. Existing
[Memory #101](https://github.com/dragginzgame/ic-memory/issues/101) is related owner
work, not proof of that API. Do not replace exact journal recovery, uncertain
funding records or restore fences with reset/retry shortcuts. Provider acceptance,
retirement and continuing-billing obligations remain governed by their service
contracts and the [probe ledger](../evidence/caffeine-probes/README.md).
