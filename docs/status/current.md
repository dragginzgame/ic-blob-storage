# Current status

Date: 2026-10-19

## Delivered source and ownership

Local HEAD is release commit `c80c51c` (0.36.1); workspace metadata remains
0.36.1. The preceding merge `047ed02` incorporated release `c32a47f` (0.36.0)
and remote main at `628d17697171a80530fbcc894d95eac06b9a387f`.
The maintainer selected pending **0.37.0** for the incoming dependency hard cut.
Earlier qualification below
retains its original source, graph and limitations, including released 0.35.0
at `583947545a3afde00c4be68dcc6bca1caec8daee`.
Remote main includes [#89](https://github.com/dragginzgame/ic-blob-storage/pull/89)
at `cc99212df4a59f4e04976bf77ad1d04f371f355a` and
[#85](https://github.com/dragginzgame/ic-blob-storage/pull/85) at
`628d17697171a80530fbcc894d95eac06b9a387f`; both passed their four CI lanes.
The resolution preserves the local catalogue repair and the remote delivery/
consumer observations. It does not establish registry publication or downstream
acceptance of 0.36.0.

The finalized **0.36.0** changelog retains the approved owned-protocol hard cut,
reusing the implementation from
[#90](https://github.com/dragginzgame/ic-blob-storage/pull/90) at
`296729844176255433a9c8657cd960b30a93df73`. Its prior Linux/macOS/MSRV CI is
source-bound evidence for that PR, not qualification of the extended document
cut or the merged dependency graph. No sibling source override is used.

## Pending 0.37.0 dependency cut

Incoming edits select public Memory 0.49.0, locked Storage 0.5.7, private Metrics
0.14.6 and Wire 0.4.8. The maintainer accepted 0.37.0 instead of a 0.36.2 patch
because Memory types are public. The official registry confirms Storage 0.5.7
requires Memory ^0.49; Memory's published implementation replaces Ciborium with
serde_cbor while its producer records preserved ledger/snapshot bytes. Blob's
cross-release retirement/reinstall disposition remains unchanged.

The preceding task prepared notes only; dependency edits were preserved without
qualification. The Platform adoption below records its separate selected scope.
Before publication, raise the current Storage ^0.5.2 minimum to the matching
qualified release so consumers cannot retain a 0.48-backed adapter. Native/Wasm,
shared-runtime composition and same-release recovery acceptance remain with
[the Memory alignment issue](https://github.com/dragginzgame/ic-blob-storage/issues/93).
No package version, source, index, Git ref or release state changes in this task.

## Pending Host 0.30 adoption

The incoming direct Host family selects published 0.30.0. All four cached
archives match official sparse-index checksums; FS's only implementation change
gates the existing capability/scoped-tempfile APIs behind independent features.
Process, Artifacts and Tools implementation source is unchanged from 0.29.
Blob uses neither optional API. Explicit empty defaults retain the existing
ordinary reads, retained publication, process limits and native fixtures.
That resolved 0.30 FS selected only test support; older Host 0.29 remained through
then-selected Build 0.21 and Testkit 0.55.2. Other incoming edits were preserved.

The default CLI, Contracts/examples and PocketIC test-package checks/lint pass;
294 native cases and actual Rust 1.91 native checks pass. The standalone's
normal/build Wasm graph excludes Host, Build, Testkit, PocketIC and both optional
filesystem backends. No installed-IC or native macOS qualification is claimed.

The initial all-feature CLI check reproduced two package-verifier type mismatches:
then-published Build 0.21 took Host 0.29 limits/errors, while the caller used 0.30.
That registry observation had no matching Build release. The
[existing owner](https://github.com/dragginzgame/ic-build/issues/9#issuecomment-6269225053)
has the concrete Blob reproduction and archive/status acceptance. Do not weaken
the package gate or introduce a version bridge. Incoming Build 0.22 now aligns
that boundary; all-feature CLI checking/lint and Rust 1.91 checking pass in the
Platform task below. Its archive regression matrix and full gate are not rerun.
Before-images,
registry/feature evidence, the initial failure and scoped successes remain under
`.tmp/review-validation/host0300370/`. No sibling edit, Git delivery or release ran.

## Recorded Platform 0.2.1 adoption

Published Platform and its macro companion 0.2.1 are selected with an explicit
0.2.1 minimum and empty defaults. Both archives match official index checksums;
the runtime is published, without sibling overrides. All incoming Memory/Storage,
Host/Build and Wire edits are preserved; this task's starting lock already selected
Build 0.22 and Wire 0.4.9. The existing 0.37.0 pending release remains selected.

Platform owns entrypoints, calls, callbacks, traps and protected spawning, retaining
the original executor 2.0.0 and IC0 1.2.0. Blob owns explicit method-bound Wire
admission and workflow policy. The endpoint emitters preserve per-method bounds
before ingress copying, and drop raw ingress before awaiting work. Required
provider/management Candid and private fixture contracts retain their original
bytes, decoder policies and reply arity; four former implicit no-argument calls
now explicitly supply Candid arguments. No storage or memory format changed here.

Two published-boundary exceptions retain Platform 0.1.18 via one explicit alias:
native Testkit 0.55 decoder limits pending
[#133](https://github.com/dragginzgame/ic-testkit/issues/133), and the optional
Wasm extraction helper pending
[#37](https://github.com/dragginzgame/ic-platform/issues/37). There are no copied
decoders, replacement runtime types or consumer-owned extraction implementation.
The native-only direct PocketIC exception also remains: 0.2.1 still lacks both
original fixture types tracked by
[#27](https://github.com/dragginzgame/ic-platform/issues/27#issuecomment-6272025393).
The legacy full CDK is selected only for optional extraction, not production.

Fresh production/probe Wasms pass 95 installed Linux PocketIC cases covering
admission, recovery, funding, consumer retries/uncertainty, every owned method's
foreign-frame refusal and provider certificate authority. Another 130 native
Contracts/client/codec cases, the service-export regression and the exact
deployment-surface case pass (132 distinct native cases). Native and extracted
Wasm provider declarations match the fixed DID byte for byte. The standalone
was rebuilt alone and its seven installed cases plus surface check pass on the
exact default artifact. The initial probe-unified build enabled Memory test
support and had a different standalone hash; both artifacts and the failed
same-hash assertion are retained. Fresh standalone graph/artifact qualification
does not reuse that feature-unified build as deployment evidence.
Strict affected native lint and actual Rust 1.91 native/Wasm checks pass. The dependency guard
now separately enforces default production normal/build graph exclusions for
legacy Platform/CDK and native owners; its actual graph and negative substitutes
pass. Candid remains for external contracts and retained DTO traits. One executor
identity remains in production. These are local simulator and compilation proofs,
not native macOS or installed Canic/Caffeine qualification; full CI is not run.

The initial compile/limit-identity/lint failures and installed guard failure remain
with successful focused retries. The guard failure called `msg_method_name` from
a query; final emitters instead bind each guard to its declared method, and no
such inspection call remains. Before-images, official index/archive observations,
source/graph/Wasm hashes and scoped logs remain under
`.tmp/review-validation/platform0190370/`. No sibling source edit, Git delivery or
release ran. Memory minimum alignment before publication still belongs to #93.

After those checks, an incoming Cargo.lock edit selected Tokio 1.54.0 instead of
the validated 1.53.3. It is preserved; the recorded hashes and runtime/native
results must not be reused as qualification of that later lock. The maintainer
also requested removal of the legacy Platform alias. Registry recheck still has
Platform 0.2.1 and Testkit 0.55.2; both missing published boundaries above remain.
The maintainer chose to preserve the compatibility boundaries until matching
dependencies publish. The supported extraction feature remains intact; the
following Testkit adoption removes the now-satisfied native limit exception.

## Recorded Testkit 0.56 adoption

The official registry publishes Testkit 0.56.0 with Platform ^0.2.1 and Host ^0.30.
Its cached archive matches the official checksum. Native test reply budgets now
use Platform 0.2.1 directly, removing their legacy dependency without conversions
or copied decoding. The lock drops Host 0.29 and its unused capability filesystem
tree; incoming Tokio 1.54.0, Wire 0.4.10 and Metrics 0.14.7 remain intact.
The root legacy alias is retained for the existing optional Wasm extractor as
agreed; Host Tools still selects Platform 0.1 transitively for native inspection.
The missing original PocketIC fixture types remain covered by the approved
native-only exception and [Platform #27](https://github.com/dragginzgame/ic-platform/issues/27).
No sibling source or release effects.

The explicit Testkit setup and offline admission pass for the new 0.56 CLI and
unchanged PocketIC 16.1.0 server. All native test-package targets/features pass
strict Clippy and actual Rust 1.91 checking. Twenty-five native library cases,
the Host buffered-output regression and exact compiled deployment-surface check
pass. Fresh six-probe and separately built default standalone Wasms pass 64
installed recovery, funding, skipped-hook, observation, ingress and certificate
cases. The default standalone also checks on actual Rust 1.91 Wasm. Dependency
pins/boundaries, production graph exclusions, formatting and docs links pass.
The initial reused funding Wasm failed its lock-hash binding assertion; rebuilding
against the current lock resolves it without a test change. That attempt, the
initial unprepared CLI and formatting failures, successful retries, before-images,
official index/checksum, source/graph and artifact hashes remain under
`.tmp/review-validation/testkit0560370/`. No full CI, native macOS, installed Canic
or live provider qualification is claimed. Source/lock hashes stayed unchanged
through the focused checks; the earlier late-Tokio gap is covered for this scope.

## Pending legacy Platform retirement

Published Platform 0.2.2 matches the incoming lock and is now the required minimum;
its archive matches the official index checksum. The legacy alias and optional
`provider-candid-export` Wasm feature are removed. The existing native exporter
still produces the fixed provider DID byte for byte. Platform checks that static
schema while Host's `ic-interface` inspects compiled function exports. Host's
optional Candid inventory feature is no longer selected, without copying its
bounded parser implementation. No Platform 0.1, full CDK or old CDK macros remain
in the resolved graph. The existing executor 2.0.0 remains Platform-owned.

Platform 0.2.2 also supplies the original PocketIC fixture types through Testkit
0.56. All six callers now use that boundary; the direct PocketIC declaration and
exception are removed. The graph guard rejects retired Platform/CDK across all
workspace features/targets and separately excludes native owners from default
deployment. These graph checks do not bind arbitrary deployed bytes to a source.
Incoming Wire 0.5.0 is preserved; these scoped results do not constitute complete
qualification of that separate public/offline-document boundary.

Strict affected Clippy, actual Rust 1.91 native/Wasm checks, seven fresh Wasms,
28 native cases and 70 installed Linux PocketIC cases pass. Coverage includes
native provider export, exact callable names/modes, skipped-hook upgrades, held
reads/sync/callbacks, funding and restore fences, passive observations, ingress
and certificate authority. Formatting, declaration/graph boundaries, dependency
pins, shell checks and documentation links pass. Full CI, native macOS, installed
Canic and live providers are not qualified. The obsolete-package update selector
refused after Cargo pruned the removed packages; locked metadata and all subsequent
checks pass without other version changes. Before-images, official index/API
evidence, source/graph/artifact hashes and logs remain under
`.tmp/review-validation/remove-legacy0370/`. No sibling source, Git delivery or
release effects occurred. The earlier compatibility wait is superseded by this
retirement; Wasm extraction is no longer a Blob prerequisite for Platform #37.

## Ownership and codec cut

Blob owns tenant policy, data/references, quotas, provider economics, retained
obligations and recovery. Contracts owns DTOs, method catalogue, application
codecs and reply verification; Client owns explicit replicated calls without
service storage. Standalone owns authentication, endpoints, lifecycle and memory
custody; the CLI owns Blob operations and retained evidence. Consumer frameworks
own their wrappers and installed integration qualification in their own repos.

The earlier codec qualification selected IC Wire **0.4.6**. The original 0.4.5 adoption inspected its
published package; that registry index and cached archive agree on SHA-256
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

Blob's Ciborium implementation and direct dependency are removed. In the earlier
codec graph, Ciborium still entered deployment through IC Memory 0.48.2's persisted
metadata, directly and through selected Storage 0.5.6. Memory/Storage encodings remained unchanged by
the codec repair. IC-specified
CBOR, native `serde_cbor` transport/certificate fixtures and external
provider/management/Cashier Candid remain with their owners. Platform owns
supported platform contracts/runtime/SDK APIs. The 0.36.0 release lock records Platform
0.1.18, Build 0.21.0, direct Host 0.29.0, Metrics 0.14.5 and Testkit 0.54.1.
Earlier checks retain their recorded selections. Host owns bounded
Candid service inventories through Platform's parser; PocketIC explicitly enables
its `candid` and `ic-interface` features.

## Recorded Testkit 0.55 compatibility repair

The retained `20261019T123829Z-3955680-12-check.log` reports six missing imports
after Testkit 0.55 narrowed its PocketIC re-export to Platform's explicit surface.
Published Platform 0.1.18 lacks `CanisterInstallMode` and `RawMessageId`. The
maintainer selected retaining Testkit 0.55.2 with a temporary native-test-only
direct PocketIC 16.1 dependency for those original types. No registry package
versions change. Cargo also normalizes existing compatible Windows dependency
edges in the lock; this is not native Windows qualification.

Six modules use the original types; flags, held ingress identities and recovery
semantics are unchanged. Direct declarations are restricted to the private
PocketIC test package. The new
[Platform evidence](https://github.com/dragginzgame/ic-platform/issues/27#issuecomment-6267665526)
adds Blob and the missing install mode to the existing export issue. Remove the
exception only when both original identities are available in a published release.

All test-package targets/features pass checking, strict lint and actual Rust 1.91
native checking. Four fresh fixture Wasms pass eleven installed admission,
funding, operator and journey cases, covering skipped outgoing hooks and exact
held read/sync recovery. Actual Rust 1.91 checks pass for all four Wasms. Metadata
confirms the direct dependency is native/dev-only; the standalone's normal/build
Wasm graph excludes PocketIC and Testkit. Before-images, index evidence and logs remain under
`.tmp/review-validation/testkit0550361/`. Full CI, native macOS and downstream
application qualification are not rerun. No Git delivery or release ran.

## Recorded local qualification and remaining boundaries

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

## Storage codec and recovery repair

The retained `20261019T113341Z-1111112-16-test.log` reports four storage
failures. The manifest lost-acknowledgment test still sent a passive document
to a method-bound service endpoint. Native attestation submission omitted its
promised original statement copy, while the fault proxy inspected that old
argument path instead of the signed method-bound request. The test now uses
the canonical service encoder; submission syncs the unchanged passive statement
beside its signed request before dispatch. Existing partial claims still refuse
reuse, and pending/lost replies recover the exact receipt without resending.

All three native submission journeys and seven manifest lifecycle cases pass
against the existing installed-fixture Wasms; no canister source changed. Eight
focused native cases, strict CLI/storage lint and actual Rust 1.91 native checks
also pass. Manifest/lock selections remain unchanged. Before-images and logs,
including the initial sandbox localhost refusal and successful permitted retry,
remain under `.tmp/review-validation/storage-wire0360/`. Full CI, native macOS
and deployed-provider qualification are not rerun. No release or Git delivery ran.

## Native package-verifier dependency alignment

The retained `20261019T115907Z-2113410-12-check.log` reports two incompatible
Host type identities: incoming direct Host 0.29 selections crossed Build 0.20's
Host 0.28 boundary. Published Build 0.21.0 selects Host 0.29 and has byte-identical
Rust implementation source to 0.20.0. Its archive checksum matches the official
registry index. Only Build is upgraded; all incoming selections are preserved.
The lock drops the three now-unused Host 0.28 packages. Testkit's separate Host
0.27.6 remains transitive, without values crossing the package-verifier boundary.

The actual verifier builds. Its maintained archive regression passes same-version
replacement, graph/integrity refusals and exact command-exit/signal propagation.
All CLI targets/features pass native checking, strict lint and actual Rust 1.91
checking. Before-images, published evidence and scoped logs remain under
`.tmp/review-validation/build0210360/`. The dependency table records the current
lock; earlier runtime qualification retains its recorded graph. Full CI, Wasm,
installed IC and native macOS are not rerun for this native tooling dependency
repair. No release or Git delivery ran.

## Historical checkpoints

The following checkpoints retain their original source, graph and limitations;
references to pending versions, former codecs, missing tools or upstream releases
are historical statements, superseded by the current scope above. The
[archived handoff](handoff-2026-10-18.md) retains earlier source-bound evidence.
Original inputs and failures remain with their recorded owners.

### Completed producer work (0.35.0)

[#87](https://github.com/dragginzgame/ic-blob-storage/issues/87#issuecomment-6263167596)
is closed for 0.35 producer delivery and acceptance. Its exact-source
[CI matrix](https://github.com/dragginzgame/ic-blob-storage/actions/runs/39160412347)
passes Linux, both native macOS hosts and Rust 1.91 MSRV. Testkit 0.54.1 now
admits locally. An untouched Git-archive snapshot of the release rebuilds
consumer/storage Wasms and passes all thirty client cases (19/4/2/5), using
original lock SHA-256
`cf267a7da81080fd5d3c385f74e05ffbbbf399610349c7f4efe29330d746ac9e`.
Raw source/archive/registry receipts and successful and failed attempts remain
under `.tmp/review-validation/issues0351/`, separate from the pending patch.

[#86](https://github.com/dragginzgame/ic-blob-storage/pull/86) is closed as
incorporated in 0.35.0: all 26 private consumer controls use bounded CBOR, current
Rust/browser callers agree, and the client extraction supersedes its old Core
imports. Public DTO traits and persisted consumer records were preserved.

Earlier producer issues [#72](https://github.com/dragginzgame/ic-blob-storage/issues/72),
[#77](https://github.com/dragginzgame/ic-blob-storage/issues/77),
[#81](https://github.com/dragginzgame/ic-blob-storage/issues/81) and
[#82](https://github.com/dragginzgame/ic-blob-storage/issues/82) are closed with
their delivered-source evidence. Development Rust is 1.99.1; MSRV remains 1.91.
Generic installer/runner matrices from
[#83](https://github.com/dragginzgame/ic-blob-storage/pull/83) are incorporated;
Blob retains its real snapshot, tool admission, hook, publication and recovery
checks. PRs #73, #78 and #80 were closed as incorporated, and #79 as superseded.

### Catalogue repair evidence before integration

[#88](https://github.com/dragginzgame/ic-blob-storage/issues/88) and
[PR #89](https://github.com/dragginzgame/ic-blob-storage/pull/89) repair catalogue
macro registration before a following Candid service export. The released
compiler regression produces an empty service; exact-path dispatch registers the
provider method before export. Standalone can then export after mounting instead
of exporting inside its certificate emitter. Names, signatures, codecs, authority,
lifecycle and storage remain unchanged.

Focused Linux evidence passes the compiler regression, strict scoped Clippy,
Rust 1.91 regression and extraction-feature Wasm checks, actual native provider
DID export, fresh production standalone Wasm, exact catalogue/export parity and
two installed certificate/authority/recovery cases. Initial failures remain
retained. These checks qualify the patch source and original released lock, not
later incoming lock edits, native macOS for the patch or installed Canic.
Keep #88 open until default-branch integration and corrected publication. Hosted
results must be checked against the final PR head before integration.

### Consumer adoption and remaining owners at the recorded revisions

[Canic draft #603](https://github.com/dragginzgame/canic/pull/603), stacked on
#591, retains Blob/Contracts 0.34 and Platform 0.1.13. At `3108ebcac`, its
previous derive-order blocker is fixed. Preflight/security pass, but the exact
[CI run](https://github.com/dragginzgame/canic/actions/runs/39148766791) still
fails fifteen Canic Core Memory API callers in checks/MSRV/Apple Silicon. Intel
macOS stops during Testkit setup with exit 28; that log does not establish its
underlying cause. Ordinary, PocketIC and release-build jobs are skipped.
Earlier 48 macro cases, native/Wasm/MSRV expansion and two byte-contract proofs
used explicit framework substitutes. They do not qualify dedicated/embedded
managed installation or retained-data restoration. Coordinate existing framework
Memory work with #595, consume the published #88 repair, then run those actual
journeys. [Canic #444](https://github.com/dragginzgame/canic/issues/444) owns this
acceptance; normal sibling checkouts are not part of the contribution.

[IC Crates draft #11](https://github.com/dragginzgame/ic-crates/pull/11) and
[its adoption issue #12](https://github.com/dragginzgame/ic-crates/issues/12)
replace two registry Core client imports with Client. The 0.35 publication
prerequisite is satisfied; its refreshed lock resolves the real published
Client/Contracts family without source overrides. Existing PR #7 owns its Canic/database/Memory/Timers convergence.
The draft's current hosted native jobs stop during cold-cache tool setup before
application compilation; [#9](https://github.com/dragginzgame/ic-crates/issues/9)
owns that ordering problem. The refreshed locked native check reaches the same three
registry Memory mismatches as baseline. Keep the draft separate from framework
work and qualify actual managed artifacts/recovery after convergence. The actual
lock/check state belongs to that draft's current handoff, not a source override.

[#76](https://github.com/dragginzgame/ic-blob-storage/issues/76#issuecomment-6263150182)
awaits a published Host retained-write completion helper. Producer PR #172 is
merged, but the inspected registry FS/Process release is still 0.27.6 and lacks
the helper. Retain partial-evidence persistence until the published replacement
is available; do not reconstruct it locally or change staging guarantees.

[#84](https://github.com/dragginzgame/ic-blob-storage/issues/84) remains the
coordinated public/persisted DTO Candid cut. Current journal consumers still
require those traits. Keep the allocation-instrumented private probe until a
Memory observation/fault API preserves attribution; related
[Memory #101](https://github.com/dragginzgame/ic-memory/issues/101) is not proof of
that API. Do not reset uncertainty, funding history, restore fences or retained
obligations to simplify adoption. Provider qualification, GC, billing, verifier
and application acceptance retain their own issues and effect authorization.
The [probe ledger](../evidence/caffeine-probes/README.md) governs provider evidence.

## Evidence and validation limits

The [archived handoff](handoff-2026-10-18.md) and
[pre-refresh released handoff](https://github.com/dragginzgame/ic-blob-storage/blob/583947545a3afde00c4be68dcc6bca1caec8daee/docs/status/current.md)
retain earlier source-bound checks, failed attempts and historical prerequisites.
They are evidence, not current instructions to repeat delivered work. Existing
receipts remain at their owners, including
[Shared adoption](../evidence/shared0240331.md),
[Platform runtime adoption](../evidence/platform01110332.md),
[fixture ownership](../evidence/release-fixture-ownership.md) and
[Canic follow-up](../evidence/canic-dispatch0331.md).

Direct dependency checks cover aliases, kinds, targets and disabled optional
declarations. Separate graph checks now reject retired Platform/CDK across all
workspace feature/target selections and native tooling in the default standalone
normal/build graph. They do not bind inspected features and lock selections to
arbitrary deployed bytes; custom deployment selections still require their own
graph and artifact qualification.
The merge resolution retains the finalized 0.36.0 changelog verbatim and preserves
the Wire endpoint implementation, catalogue registration and dependency selections.
It combines the local repair evidence with the remote handoff's recorded consumer
observations; older duplicated checkpoints remain in the parent history and
retained before-images under `.tmp/review-validation/merge0360/`.
The catalogue export regression, standalone native all-target/all-feature check
and actual Rust 1.91 extraction-feature Wasm check pass. Focused logs remain with
those before-images. Full CI, installed IC and native macOS are not rerun for
this merge resolution; earlier qualification retains its original source/graph.
Those merge checks preceded Testkit 0.55.2; its focused qualification is recorded
above. The maintainer completed the merge in `047ed02`.
