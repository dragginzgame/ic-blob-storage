# Current status

Date: 2026-10-19

## Source and scope

Local HEAD is committed candidate `91067b4a638395fca7745626a75b03fdc29e22e3`
(subject `0.36.0`); workspace metadata remains 0.35.0. Earlier qualification
below records its original source, including release
`583947545a3afde00c4be68dcc6bca1caec8daee`. Unrelated incoming work is preserved.
Remote main includes [#89](https://github.com/dragginzgame/ic-blob-storage/pull/89)
at `cc99212df4a59f4e04976bf77ad1d04f371f355a` and
[#85](https://github.com/dragginzgame/ic-blob-storage/pull/85) at
`628d17697171a80530fbcc894d95eac06b9a387f`; both passed their four CI lanes.
The local catalogue repair is preserved. The current adoption does not commit,
push, merge, release, publish, deploy or perform paid provider effects.

Workspace package versions remain 0.35.0. Pending notes are **0.36.0** for the
approved owned-protocol hard cut, reusing the implementation from
[#90](https://github.com/dragginzgame/ic-blob-storage/pull/90) at
`296729844176255433a9c8657cd960b30a93df73`. Its prior Linux/macOS/MSRV CI is
source-bound evidence for that PR, not qualification of this extended document
cut or the current dependency graph. No sibling source override is used.

## Ownership and codec cut

Blob owns tenant policy, data/references, quotas, provider economics, retained
obligations and recovery. Contracts owns DTOs, method catalogue, application
codecs and reply verification; Client owns explicit replicated calls without
service storage. Standalone owns authentication, endpoints, lifecycle and memory
custody; the CLI owns Blob operations and retained evidence. Consumer frameworks
own their wrappers and installed integration qualification in their own repos.

Published IC Wire **0.4.5** is selected and its packaged source inspected. The
registry sparse index and cached archive agree on SHA-256
`27c2e310a91659933843cac278f17e1f0dd57dde53cd160133c6b6fc3215c822`.
All 31 owned methods and standalone install/post-upgrade inputs use method- and
direction-bound Wire; finite tenant bytes are preserved. CLI, replicated Client,
probe and browser callers move together. The separate bounded `document` value
identity serves offline files and private controls; it is not a per-control-method
identity or an embedded Rust type tag. See [the protocol contract](../service-wire.md).

New application artifacts use `.wire`. Old saved runs keep their original bytes
and tools for reconciliation; never rewrite or replay them. Cross-release service
replacement remains retirement/reinstall only. Same-release retry, backup,
restore fences and retained obligations remain required.

Blob's Ciborium implementation and direct dependency are removed. Ciborium still
enters the deployed graph through IC Memory 0.48.2's persisted metadata, directly
and through selected Storage 0.5.6. Memory/Storage encodings remain unchanged by
the codec repair. IC-specified
CBOR, native `serde_cbor` transport/certificate fixtures and external
provider/management/Cashier Candid remain with their owners. Platform owns
supported platform contracts/runtime/SDK APIs. Incoming Platform 0.1.17,
Build 0.20.0, Host 0.28.1, Metrics 0.14.5 and Testkit 0.54.1 selections are
preserved. Host owns bounded
Candid service inventories through Platform's parser; PocketIC explicitly enables
its `candid` and `ic-interface` features.

## Current qualification and remaining boundaries

Before-images, published-index evidence, the approved proposal, failures and
focused retry logs remain under `.tmp/review-validation/wire045-adoption/`.
Contracts' 122 unit cases, the CLI's 121 cases, fresh affected Wasms and admitted
Testkit/PocketIC 16.1.0 pass. Current installed qualification covers 23 standalone
installation/ingress/preparation/reference/verification/download cases and all
30 consumer/tenant/funding/manifest client cases. The offline 1 MiB real-SDK/native
handoff passes repeat/corrupt-source refusal with zero network requests.
The real Chromium case passes ten permission/control scenarios, including reload,
competing tabs, cancellation and lost responses. Strict scoped Clippy, actual
Rust 1.91 native/Wasm checks, 68 Core upload cases, 12 inventory cases and the
provider service-export regression pass. Dependency boundaries/pins, runtime-free
Contracts/CLI and storage-free Client graph checks, formatting and local
documentation links pass. Manifest/lock hashes remain unchanged through these
checks. Initial failed assertions, the missed fixture bound and lint failures are
retained alongside their successful focused retries.

The earlier Build 0.19/Host 0.28 package-verifier mismatch is superseded by the
incoming Build 0.20 selection. The maintainer's retained gate log reached admission
after compiling/running all-feature native targets, including that binary. This
admission repair does not change dependency selections or claim a complete gate.

[#88](https://github.com/dragginzgame/ic-blob-storage/issues/88) retains corrected
package-publication acceptance. [Canic #444](https://github.com/dragginzgame/canic/issues/444)
and [IC Crates #12](https://github.com/dragginzgame/ic-crates/issues/12) own their
published-library/runtime/managed integration acceptance. Public DTO/persisted
Candid trait removal remains [#84](https://github.com/dragginzgame/ic-blob-storage/issues/84).
Local PocketIC substitutes do not qualify deployed provider behavior or installed
Canic. Native macOS and complete CI are not rerun for this pending extension.

## Temporary filesystem ownership

The next local cleanup uses published Host FS 0.28.1 without a version upgrade or
sibling override. Its archive matches the official registry checksum
`8769f841440afae535163474cbcce6e06c33be3e15799082b85083e3b8adab1b`.
All direct `tempfile` declarations and maintained source uses are removed across
the fourteen manifests. Testkit and PocketIC still own the transitive package.
Host owns private fixture directories/files and same-descriptor, create-only
publication in the examples. Failed test fixtures remain available during panic;
retained report directories stay caller-owned. Complete snapshot publication is
still not a crash-durable multi-file transaction. Verified output completion
failures retain Host's before/after-publication and cleanup evidence.

The cleanup passes 173 native cases, 12 installed cases (including ten Chromium
scenarios), strict affected Clippy, actual Rust 1.91 native/Wasm checks and the
dependency/format/documentation guards. Installed cases reuse the previously
qualified Wasms: only native fixtures/examples and development dependencies
changed. Contracts' normal Wasm graph excludes both Host FS and `tempfile`.
No registry package versions changed. The initial lint failure and successful
retry, before-images, registry evidence and scoped logs remain under
`.tmp/review-validation/host-temp0360/`. Native macOS and the complete gate are not
rerun. Earlier dependency failures retain their recorded source and graph.

## Admission query codec repair

The maintainer's retained log
`.git/release-state/validation-failures/20261019T111132Z-375720-16-test.log`
reports nine admission failures. The three discovery/upload-capacity/reference-
capacity probe endpoints still decoded/replied with the passive document identity,
while clients used method-bound service frames. Their `Identity` traps prevented
policy and inventory/refusal handling from running. They now use exact method/
direction frames, manual replies and pre-copy byte admission; private fixture
Candid commands and all policy/accounting semantics remain unchanged.

The new installed regression reproduces the failure against the preserved old
Wasm and passes after rebuilding the probe. All 30 admission cases pass on that
initial graph, including the nine reported failures. Incoming Platform 0.1.17
arrived during the run and is preserved. A fresh probe on that selection also
passes all 30 admission cases, including the 704-object cleanup workload. Strict
probe/admission lint and actual Rust 1.91 native/Wasm checks pass. The manifest,
lock and new Wasm hashes are retained with those results. Full CI and native
macOS are not rerun.
Before-images, the old Wasm and failed/successful logs remain under
`.tmp/review-validation/admission-wire0360/`. No release or Git delivery ran.

## Historical checkpoints

The following checkpoints retain their original source, graph and limitations;
references to pending versions, former codecs, missing tools or upstream releases
are historical statements, superseded by the current scope above. The
[archived handoff](handoff-2026-10-18.md) retains earlier source-bound evidence.
Original inputs and failures remain with their recorded owners.

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

Pending compatible 0.34.1 advances
[#84 — Remove application DTO Candid coupling after migrating current probes](https://github.com/dragginzgame/ic-blob-storage/issues/84)
through [PR #86 — Move private consumer-probe controls to CBOR](https://github.com/dragginzgame/ic-blob-storage/pull/86).
All 26 callable controls now use bounded canonical CBOR, including tenant,
funding-observer, manifest and asset admission/publication/cleanup controls.
Actual Rust callers and the opaque Chromium control packets use the same codec;
service calls retain their exact Wire or CBOR clients. Transient fixture types
drop CandidType, and the unused Candid ingress decoder is removed. Public DTO
traits, lifecycle arguments and persisted record encoders remain unchanged.
`Registration`, its source and `ManifestIntent` retain CandidType because the
current consumer journal stores them. The full public cut must inventory these
real stored consumers and pinned framework callers, not just incoming endpoints.

Fresh consumer/storage Wasms pass all nineteen affected consumer transaction
cases, covering authority, trap rollback, lost acknowledgments, cancellation,
late completion, original-operation recovery, capacity and same-release fences.
Malformed/trailing/oversized/unknown-field and legacy Candid controls refuse
before either owner writes. Eleven related tenant/funding/manifest cases pass
on those same fresh Wasms. The opt-in pinned Chromium fixture also passes all
ten scenarios, including full-width IDs, foreign-caller refusal, reload/competing
tabs, gateway failures and consumer cancellation/withdrawal. This is local IC and
browser substitute evidence, not deployed-provider or Canic qualification.
Focused native checks, strict affected Clippy, actual Rust 1.91 Wasm, formatting,
dependency boundaries/runtime-free graphs and documentation links pass.
Earlier tenant/funding/manifest checks remain under
`.tmp/review-validation/probe-controls0341/`; current checks and the initial
sandbox npm-admission failure plus successful retry remain under
`.tmp/review-validation/asset-controls0341/`. The isolated implementation is
`.tmp/worktrees/tenant-probe0341/`. Package, manifest and lock selections are unchanged.


Pending 0.35.0 moves the seven existing replicated client families to
`crates/ic-blob-storage-client/`, with no replacement implementation or codec.
Core's public client paths are removed; this is a minor public API cut. This
isolated batch includes the earlier private-control migration from PR #86 and
preserves its persisted records and public DTO traits. Workspace package versions
remain 0.34.0. The new member inherits those versions and existing dependencies;
this does not make an unpublished client package available to registry consumers.
The actual consumer probe imports the client package. Package verification and
publication selection include Contracts, Client and Core; normal client graphs
reject service storage while Contracts/native CLI remain runtime-free.
Canic shared-dispatch adoption remains in its existing draft PR #603, separately
from the client extraction. Actual downstream adoption needs the published client
family and each application's coherent runtime graph; no source override qualifies
publication or deployment. Evidence stays under `.tmp/review-validation/client-boundary0350/`.


## Client-boundary completion — 2026-10-18

The accepted batch is applied to this normal working tree, including the earlier
private-probe changes from PR #86. The isolated client implementation remains at
`.tmp/worktrees/client-boundary0350/`, based on that PR's `78071e2`. No Blob index
or ref is changed. The incoming IC Build requirement and every registry package
and dependency edge in the incoming lock are preserved; only the new local
Client member and its two probe edges are added.

The frozen baseline passes four client unit cases, strict affected Clippy, actual
Rust 1.91 native/Wasm checks, fresh consumer/storage Wasms and thirty installed
consumer/tenant/funding/manifest cases. All three archives pass tests and exact
graph/final integrity admission. Warm-cache same-version execution and archive,
source, manifest and lock tamper refusals pass. The release adapter fixtures pass
Contracts/Client/Core publication order, individual selection, dry runs and
each stage's failures/lost replies. Client and Core have separate Contracts
readback directories. The initial missing Storage-probe dependency and reused
readback-directory failures remain in their original failed logs.

The combined normal graph selects Platform 0.1.13, Host 0.27.6, Metrics 0.14.4,
Build 0.18.0 and Testkit 0.54.1. Its client unit tests, affected all-target checks,
strict Clippy, Rust 1.91 native/Wasm checks, fresh Wasms, three-archive tests,
formatting, documentation links, ShellCheck, pins and dependency/storage boundary
checks pass. After the README layout-index addition, all three archives were regenerated;
the final Core payload differs only in README bytes, with every Rust, manifest,
lock and other entry byte-identical to the tested payload.
Installed qualification on this graph stops at offline tool admission: the
prepared Testkit CLI is 0.54.0, not the selected 0.54.1. Run explicit
`make install-testkit` and `make testkit-check`, then the thirty affected cases
before claiming this graph's installed acceptance. The frozen-graph installed
results are not relabeled as current-graph evidence.

[Canic draft PR #603](https://github.com/dragginzgame/canic/pull/603) now has the
0.34/Platform 0.1.13 follow-up at `3b46d119b`. It preserves Canic endpoint,
lifecycle and memory custody, the sole installation cell and initial continuity
observation. Forty-eight actual macro cases, strict macro lint, native/Wasm
expansion including Rust 1.91, two isolated byte-contract cases and selected-file
formatting pass. Its base still fails fifteen Canic Core Memory caller errors;
framework substitutes do not qualify actual dedicated/embedded installation.
The normal Canic checkout's newer framework work was not copied or edited.

[IC Crates draft PR #11](https://github.com/dragginzgame/ic-crates/pull/11) replaces
the registry's two Core client imports with the client-only package. It awaits
Client/Contracts 0.35 publication, a real registry lock refresh and its existing
framework/database convergence in PR #7. Candidate-source qualification reaches
the same three Memory type errors as the unchanged baseline. No path override or
lock change is committed there, and no passing installed registry is claimed.

Public/persisted DTO Candid traits remain required and unchanged. Full CI, native
macOS, installed Canic/registry journeys, live provider behavior, release and
publication are not qualified by this batch. All attempts and scoped logs remain
under `.tmp/review-validation/client-boundary0350/`.


Adoption tracking is explicit: [Blob #87](https://github.com/dragginzgame/ic-blob-storage/issues/87)
owns final Testkit/tool/installed qualification and producer publication readiness;
[Canic #444](https://github.com/dragginzgame/canic/issues/444#issuecomment-6252848643)
owns the managed dispatch/codec checklist for draft #603;
[IC Crates #12](https://github.com/dragginzgame/ic-crates/issues/12) owns draft #11's
published-client/lock/runtime adoption. Existing application acceptance and #84's
deferred public DTO trait cut retain their separate owners. No new upstream
Testkit defect is asserted for the missing local prepared tool.


## Current issue qualification — 2026-10-19

The #88 compiler regression fails on released 0.35 with an empty Candid service
and passes with the exact-path catalogue repair from PR #89. The pending fix
also passes strict scoped Clippy, actual Rust 1.91 regression/extraction-feature
Wasm checks, native provider DID export, fresh production standalone Wasm and
complete catalogue/export parity. Two installed standalone certificate cases
preserve authority, quotas, one-shot issuance, uncertainty and restoration.
The initial failure and unfulfilled lint-expectation attempt remain separately
retained. The local follow-up removes an unused lint allowance from the PR's
fixture and uses an expected diagnostic for its declaration-only function.
#88 remains open through default-branch integration and corrected publication;
these Linux/standalone results do not qualify installed Canic or native macOS
for the pending patch. Package versions, manifests and lockfile are unchanged.

[#87 is closed](https://github.com/dragginzgame/ic-blob-storage/issues/87#issuecomment-6263167596)
for producer delivery and acceptance. All three authenticated 0.35 archives name
release `583947545a3afde00c4be68dcc6bca1caec8daee`; its exact-source native
[CI matrix](https://github.com/dragginzgame/ic-blob-storage/actions/runs/39160412347)
passes Linux, both native macOS hosts and MSRV. The normal Testkit 0.54.1 tool
now admits. An untouched Git-archive snapshot of that release, with original
lock SHA-256 `cf267a7da81080fd5d3c385f74e05ffbbbf399610349c7f4efe29330d746ac9e`,
rebuilds consumer/storage Wasms and passes all thirty client cases (19/4/2/5).
This is separate from the dirty catalogue patch. Canic #444 and IC Crates #12
are updated with publication availability and retain their own runtime/managed
acceptance; #88 is their newly explicit catalogue prerequisite.

[#76](https://github.com/dragginzgame/ic-blob-storage/issues/76#issuecomment-6263150182)
still awaits a published Host completion API. Producer PR #172 is now merged,
but registry Host FS/Process still end at 0.27.6 and the selected FS archive has
no retained-write completion helper. Keep existing partial-evidence persistence
until a published replacement is available. The later public DTO/owned-wire
cut remains #84; provider, GC, billing, verifier and application acceptance
retain their existing issues and effect authorization. No broad local gate,
release, deployment, paid provider action or Blob Git operation ran here.
