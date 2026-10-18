# Current status

Date: 2026-10-18

## Source and scope

HEAD is released 0.33.0 at `55d70cb0351093b5cc25fcc7b9c27a04e610a1f8`.
Workspace package versions remain 0.33.0; the pending compatible batch is 0.33.1.
The working tree contains the Platform and Shared Tooling adoptions below plus
bounded ownership/documentation cleanup. No new release, publication, deployment
or Git effect is established by this handoff.

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
and codecs. Published Platform 0.1.8 supplies supported principal/Candid/native
Agent/certificate APIs; direct runtime exceptions retain upstream owners.
Wire 0.4.0 owns the two tenant method frames. Other 29 owned methods and standalone
lifecycle inputs still use bounded CBOR. Provider/management Candid and persisted
record encodings retain their separate contracts. No storage or wire migration
is part of the pending compatible cleanup.

## Pending dependency and tooling adoption

- Published Platform 0.1.8 owns configured/staged decoding and supported
  record/service generator paths. Core, Contracts, protocol support and CLI drop
  direct Candid. Native Agent/certificate/parsing imports use the supported
  published APIs. Dynamic native fixture serializers and Agent response enums now
  use Platform too; native direct Candid and all direct Agent declarations are
  removed. CDK macros/runtime and Testkit decoder identity retain documented
  exceptions and owning issues in
  [the dependency guide](../dependencies.md#ic-platform-boundary).
  Management-history DTOs now route through Platform's exact upstream identities;
  the direct dependency and exception are removed. The
  [0.1.8 receipt](../evidence/platform0180331.md) records the current published
  selection and recovery/graph checks; new snapshot/status DTOs add no Blob
  execution path. The
  [0.1.7 receipt](../evidence/platform0170331.md) retains its native fixture and
  signed-update qualification against its original graph. The
  [0.1.6 receipt](../evidence/platform0160331.md) retains its published API and
  installed recovery checks against its original graph. The
  [0.1.5 receipt](../evidence/platform0150331.md) retains its earlier native/MSRV,
  vector/example and thirteen installed-case evidence against its original graph.
- Shared Tooling 0.24.0 at `cab12cd3dfa4fdb34846b25397f0d72881bb9ae0` now has
  137 selected payloads, including the canonical direct-boundary checker. Role
  labels, grouped formatting and prepared/selected-commit release checks agree.
  Common Cargo setup omits the unused extractor executable.
  [The adoption receipt](../evidence/shared0240331.md) records the initial snapshot
  qualification; [Platform adoption](../evidence/platform0331.md) records adding
  the checker. Current manifests and lock retain Host 0.27.2, Build 0.17.1 and
  Testkit 0.53.0 without new selections in this audit.
- [The fourth ownership audit](../reports/audits/2026/10/18/architectural-sediment/04/report.md)
  finishes the substrate declaration guard and runtime-free Storage exclusion.
  It routes trial preparation to the maintained strict-JSON-to-CBOR command and
  separates historical trial state from current host instructions. Source,
  before-images, fixture regressions and focused checks remain under
  `.tmp/review-validation/sediment0331-04/`.

## Current checks and next work

[The issue-maintenance receipt](../evidence/issues0331.md) records the gateway
fixture's unused direct core dependency removal and current installed checks.
The old Host custody/framing and Build fingerprint adoption issues are closed
against released 0.33.0 and its passing Linux, Apple Silicon, Intel macOS and MSRV
workflow. This does not assign remote CI success to dirty 0.33.1. The fixture
removal remains pending delivery; Canic workflow acceptance remains with #72 and
its draft consumer PR, and the deployed application/provider issues remain open.


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
