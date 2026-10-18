# Current status

Date: 2026-10-18

## Source and scope

HEAD is released 0.33.1 at `67b20989ebdb555e9fac6022685be51ba866fb4a`.
Workspace package versions remain 0.33.1; pending notes are 0.34.0 because the
provider chunk-reply decoder moves from Core's public API to Contracts.
The working tree adopts published Platform 0.1.11, narrows Shared's exported test
roots and selects development Rust 1.99.1. Earlier Platform/Testkit and ownership
changes below are delivered;
their receipts retain their original sources and graphs. No new release,
publication, deployment or current-repository Git effect ran in this pass.

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
and codecs. The pending lock selects Platform 0.1.11 for supported principal,
Candid/native Agent/certificate, entrypoint and call/system APIs. No direct
Candid/CDK exceptions remain; runtime implementation retains its upstream owner.
Wire 0.4.1 owns the two tenant method frames. Other 29 owned methods and standalone
lifecycle inputs still use bounded CBOR. Provider/management Candid and persisted
record encodings retain their separate contracts. No storage or wire migration
is part of the pending cleanup.

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
  131 selected payloads after the pending compatible test-root retirement
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

The pending 0.34.0 provider codec consolidation moves chunk-reply decoding and
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
installed-IC qualification were not run for this source-only API relocation.

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

The pending cleanup adapts [PR #83 — Keep generic installer and
release-runner regressions at Shared](https://github.com/dragginzgame/ic-blob-storage/pull/83)
to the unchanged Shared 0.24 revision. Canonical export retires five generic test
roots and their unused cache-key companion; Blob retains actual tool admission,
Make/Testkit forwarding, hooks, evidence and release/publication recovery checks.
All retained focused regressions pass. PRs #73, #78 and #80 are closed as already
incorporated; the old Testkit 0.51 draft #79 is closed as superseded.

[#81 — Qualify Rust 1.99.1 as the development compiler](https://github.com/dragginzgame/ic-blob-storage/issues/81)
is implemented locally, including the frozen-example CI override. MSRV remains
1.91; that compiler-only change alters no package/dependency version or lock.
New compiler-bound
formatter/Testkit tool receipts are prepared. Native production-package checks
and strict lint, 348 core plus 118 Contracts unit cases, fresh standalone/peer
Wasms and three installed account/rollback/lifecycle cases pass on Linux.
Formatting, boundary/pin/snapshot/tool checks and the substitute-Cargo example
adapter pass. Native macOS and full CI for these dirty changes are not established.
Before-images, canonical export records, CI observations, failed prerequisites
and successful checks remain under `.tmp/review-validation/issues0332/`.

[The issue-maintenance receipt](../evidence/issues0331.md) retains #82's original
fixture-removal evidence. The fix is delivered in 0.33.1, whose exact tooling
workflow still has unfinished Linux/macOS jobs at this inspection; keep its issue
open until required delivered-source checks finish. The 0.33.1 frozen-example
workflow and MSRV pass. These results do not qualify the current dirty tree. Public DTO
Candid removal remains the separate coordinated hard cut in
[#84 — Remove application DTO Candid coupling after migrating current probes](https://github.com/dragginzgame/ic-blob-storage/issues/84).

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

Pending compatible 0.34.1 starts
[#84 — Remove application DTO Candid coupling after migrating current probes](https://github.com/dragginzgame/ic-blob-storage/issues/84)
with the private tenant-client controls. Four probe endpoints now decode/encode
bounded canonical CBOR; their private dispatch envelope drops CandidType and
rejects unknown fields. Service tenant calls remain on exact Wire codecs.
The other probe transports, public DTO traits, lifecycle arguments and persisted
journals are unchanged. Fresh consumer/storage Wasms pass all four focused
installed tenant-client cases: authority, query refusal, saved original command,
reply loss/callback trap, stale commands and same-release restore fences, plus
malformed/trailing/oversized/unknown-field refusal before either owner writes.
Focused native checks, strict probe/protocol/harness Clippy, Rust 1.91 Wasm,
formatting, dependency boundaries and documentation links pass.
These are local IC controls, not new Canic or provider qualification. Source,
server admission and check logs remain in `.tmp/review-validation/backlog0340/`;
the isolated implementation is `.tmp/worktrees/tenant-probe0341/`.
