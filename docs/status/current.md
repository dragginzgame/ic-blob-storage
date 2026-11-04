# Current status — 2026-11-04

## Storage/Wire/Testkit adoption and delivery — pending 0.72.0

Delivery is authorized for this Blob checkout: `main` to `origin/main` and
crates.io, supplier-first Contracts, Client, Engine **0.72.0**. Include the pending
release-stage/preparation/archive improvements below. Shared's adopted committed
revision is now confirmed present on GitHub; no sibling source/delivery changes
are needed. The live 0.69 service trial and all provider obligations stay untouched.

Selected and checksum-verified published archives: **Storage 0.67.0**, **Testkit
0.86.0**, **Wire 0.17.1**. Cargo also aligns metadata Derive 0.17.1. Only these four
locked package identities change; other supplier selections remain fixed.
The incoming Storage 0.66.3 lock edit is preserved through its published 0.67
successor, including its production cache/scan improvements. New Wire/Storage
public type identities require the next minor for all three affected Blob groups.
Package versions remain 0.71.1 until the standard group runners prepare them.

Published API review confirms optional metadata derive, already forwarded by
Blob's existing host schema feature. No Endpoint macro dependency is used here.
Storage/Testkit align Wire limits/errors; frame bytes, decoder budgets, stored
layouts and simulator behavior are unchanged by this cohort. No compatibility
reader, automatic migration or reset is introduced. Blob's existing retirement/
reinstall requirement still governs cross-release service transitions.

Archive metadata, checksums, sources and resolution output are retained under
`target/upstream-delivery-20261104/`. Graph/deployment and refusal checks pass,
including metadata-derive exclusions. The frozen Wire surface, 32 service-export
checks and generated TypeScript's 27 methods/137 frozen frames pass. The first
browser tool check refused the unpinned ambient npm; the existing prepared
repository npm 12.2.1 passes without changing global tools. Testkit 0.86 CLI/setup
passes with server 16.1.0 unchanged. No service install or paid effect occurred.
Release gates/publication are the remaining delivery work.

The first Contracts gate at source `f17ba3a8` passed Clippy, graph and metadata
admission, then was invalidated when an incoming lock update selected published
Wire/Derive 0.17.1 during the release-adapter matrix. The user confirmed the graph
is settled at this cohort. The affected disposable stage subtree was terminated;
normal runner failure/lock cleanup completed and no release metadata/tag/push or
upload occurred. Preserve `target/release072/contracts-release.log` and
`target/blob-release-validation.t0hzD9/` plus its Git-side failure logs. Verified
0.17.1 archives/checksums are under the adoption evidence root; these retain the
same APIs, canonical bytes and budgets, with documented small-value codec cost
tradeoffs. Restart the complete gate from a new clean source; do not reuse the
interrupted attempt as a release receipt.

## Installed preparation and archive regression — pending Engine 0.71.2

The two installed partitions now run through one `test-pocketic` stage and one
sequential, fail-fast Make graph. Their common preparation executes once without
new stamps, artifact caches or bypass flags. The admitted partition list retains
tenant-client-only selection and independent developer targets. The separate
actual-version smoke sweep still qualifies a different release/version mix when
candidate evidence is reused. The full roster remains 22 stages: one installed
stage replaces two, and the archive-verifier regression gets its own stage.

`package-regression` owns the same-version/warm-target/refusal fixture. A current
annotated baseline with matching stage roster/configuration can omit it for
classified product-only source changes outside the CLI. CLI/verifier changes and
unknown input, dependency, compiler, tool or policy drift retain it. The explicit
reuse policy can admit it across matching final candidates. `package` always
builds its verifier and verifies the actual applicable archives, independently
of regression selection. Native hosted compatibility selection runs after prepared
tools/compiler admission; it selects the regression on every native host while
retaining actual package verification. Compatibility scratch is under `target/`.
No Shared/sibling change or delivery is part of this follow-up.

Focused qualification passes under Rust 1.99.1: **82 Client + 270 Engine installed
cases**, with exactly one Testkit preparation, probe build, standalone Wasm build
and CLI build in the combined Make graph. Actual archive-verifier regression
passes both same-version generations and all ten graph/status/final-drift refusals.
Updated source-bound selection fixtures pass Client-only omission, CLI/verifier
activation, unknown-input fallback, mandatory actual-package verification and
failure propagation. Parallel/keep-going callers still prepare once and stop
before Engine after Client/preparation failure. Current stage-dispatch and policy
inputs are also in the normal-CI input checkpoint inventory.

Evidence: `target/release-stage-followup-summary.json`,
`target/release-stage-followup-installed.log`,
`target/release-stage-followup-archive-regression.log` and
`target/release-stage-followup-selection-final.log`. The initial selection-fixture
failure at `target/blob-release-selection.tauxYd/` is retained: an old positional
negative test became a no-op after the roster changed; it now selects the omitted
regression by name. The earlier four-stage timing/reuse results below belong to
the prior split dispatch; this follow-up did not repeat that three-group benchmark,
the full 22-stage gate or native macOS qualification. Current code/docs/snapshot
checks are recorded with this follow-up. Package versions/Cargo.lock are unchanged;
all work in this checkout remains unstaged/uncommitted, with no push, publication,
service deployment or paid effect.

## Group-stage reuse and publication admission — pending Engine 0.71.2

The maintainer authorized fixing repeated qualification across group releases,
including the Shared Tooling producer and scoped local Shared commits. Shared
**0.29.6** at `75949bd97f040c311e764c85b511f06e6ed685e6` is adopted through
the canonical exporter (160 files). These Shared follow-ups are committed locally;
no Shared push/release is authorized or claimed. Its unrelated changelog/group-
helper edits are preserved. Blob changes remain uncommitted; all package versions
and Cargo.lock remain at the published 0.71.1 selections.

Shared owns final candidate rendering, exact input observation/execution/admission,
proof/log sealing and publication package closure checks. Blob's thin dispatcher
and explicit stage/host policy consume these interfaces. Pending numbered group
versions are actually compiled in a disposable final candidate; versions are never
stripped from compiled evidence. Nine reusable stages retain per-group admissions
and immutable original logs. Only Linux/x86_64 workspace plans are qualified;
narrow plans, macOS and unsupported flags/paths keep ordinary recipes. Actual
source/index/selection, metadata/rollback, tool/compiler/graph, formatting, package
archives and registry admission remain fresh. A small fresh actual-version sweep
follows every admitted installed Client partition. Ordinary CI/Make targets keep
their existing behavior. The selected proof/log artifacts are retained by hosted
group CI; infrastructure qualification remains conditional rather than running
at every release.

Publication authenticates exact non-yanked existing archives before compilation.
New uploads/dry runs use admitted completed candidate lint only after actual
source/tag/exact changed-path roster, original proof/log custody, candidate inventory
and complete primary closure all agree. Missing local evidence or differing closure
runs current-source Clippy. Historical qualification is not a new execution on the
publication host. Publication avoids candidate rendering/setup again; compiler,
archive creation/verification, upload and registry provenance remain fresh.

Focused checks pass: source selection/refusal and stage partitions; substitute-
registry retry/dry-run/lint-failure ordering; tool forwarding; compiler/native
version binding, checkpoint corruption and actual installed CLI/host identity;
producer source-path-with-spaces, real Cargo execution/two later admissions and
closure checks; generic runner failure/drift/interruption; candidate source and
ignored tool-root refusal; Perl/shell syntax, selected ShellCheck, snapshot,
formatting and documentation checks. Shared docs/version checks pass. No full
22-stage gate, native macOS, mainnet/provider or registry upload was run here.

The first real consumer run (`target/blob-stage-reuse.kQRpWW/`) executed strict
Clippy and 641 native tests once and admitted both for the next two groups; all
three publication lints were reused. Its final summary script was edited during
execution and failed; preserve the raw successful group evidence and that failure.
The broader retry (`target/blob-stage-reuse.hMp5Jm/`) passed Clippy/native then
failed installed startup because its nested temp path exceeded PocketIC's Unix
socket limit (`SUN_LEN`). The failed output/probe state is retained; no receipt
was issued for the failed stage. The dispatcher now uses a short explicit target-
owned temp root and falls back on long unsupported temp roots. The corrected
four-stage real Clippy/native/Client/Engine qualification passed at
`target/blob-stage-reuse.0QmMkH/`. Each large stage executed once and was reused
by both later groups: **641 native**, **82 installed Client** and **270 Engine**
cases, plus fresh actual-version checks for every group. All three publication
closures admitted completed Clippy without rerendering/recompiling. The missing-
original-log fixture then ran real current-source Clippy successfully; its original
log remains beside the missing path as `.fixture-retained`.

| Disposable group | Four-stage gate wall time | Qualification |
| --- | --- | --- |
| Contracts | 597 seconds | Four real executions, including cold compilation |
| Client | 225 seconds | Four reuses plus fresh actual-version sweep |
| Engine | 176 seconds | Four reuses plus fresh actual-version sweep |

These are selected fixture measurements under shared package-cache contention,
not an uncached warm baseline or a full 22-stage release benchmark. Publication
qualification took 20.8–25.3 seconds per group; no lint subprocess ran for those
admissions. Counts, origins and monotonic timings are in `summary.json` and
`publication-summary.json` beside the retained group records. Rendering/hashing
still has overhead; keep `target/` across supplier-first group delivery to retain
reuse. macOS/unsupported configurations keep ordinary execution.

The latest committed Shared revision also passes its real Cargo/source/cache/
closure/refusal fixture from a clean Git export, excluding unrelated dirty Shared
work: `../shared-tooling/target/committed-candidate-check.dhV4al/qualification.log`.
Current Blob capture successfully re-seals actual original physical admissions;
its receipt verifier admits the four-stage record and rejects five missing/
mismatched custody cases. Those refusals are now in the permanent consumer
fixture. Its hosted run stays conditional on changed/uncertain infrastructure;
the full production gate and native macOS were not rerun.

Other evidence: `target/publication-stage-final-check.log`,
`target/stage-reuse-selection-final.log`, `target/stage-tools-forwarding-final.log`,
`target/stage-reuse-fresh-identities.log`, `target/stage-reuse-custody-refusals.log`,
`target/stage-reuse-reseal-capture.json`, `target/stage-reuse-final-four-stage-receipt.log`
and the `target/stage-*-snapshot-*.log`
exporter records. Source/format failures from earlier preparations are retained.
Candidate source and compilation caches stay under target; no retained evidence,
old provider state or unfinished live 0.69 obligations was erased. The actual
Blob checkout remains unstaged/uncommitted; only disposable fixtures exercised
release preparation/Git tags. No push, real release/publication, paid effect or
service deployment occurred.

## Published Contracts, Client and Engine 0.71.1

Authorized delivery completed in this checkout on `main`, supplier-first to
`origin/main` and crates.io. All thirteen members now select **0.71.1**. Each
group passed its complete 22-stage source-bound release gate, followed by
publication compiler/Clippy and fresh archive verification. All attempts passed;
no source repair or validation restart was needed during delivery.

| Group | Release commit | Annotated tag |
| --- | --- | --- |
| Contracts | `7390ec10b786bbe7c9302c3fceb2eb47335aed9d` | `contracts-v0.71.1` |
| Client | `089a177035ffbb8679eaee3f2201156d71a7b2f6` | `client-v0.71.1` |
| Engine | `db19866ca82d86e7f0e6e1bcf5ee8551edb8a791` | `engine-v0.71.1` |

Independent registry/archive readback confirms non-yanked versions, exact
checksums, embedded VCS commits/package paths and matching annotated remote tags.
Evidence: `target/release0711-readback/summary.json`. Source implementation is
`dfb9f238`; successful gates are retained at
`target/blob-release-validation.YR1rsu/` (Contracts),
`target/blob-release-validation.d661lT/` (Client) and
`target/blob-release-validation.Nr8I7Z/` (Engine). Publication captures are
`target/publication.qOCjIt/`, `target/publication.1Wmhjs/` and
`target/publication.8IK1Sh/`, respectively. Immutable group receipts remain
committed with each release tag.

The delivered graph selects published Testkit **0.85.1**, Platform **0.22.0**,
Host Tools **0.82.0**, Wire/Derive **0.16.2**, Build/Host foundation **0.74.3**,
Runtime/macros/guard **0.19.1** and Types **0.18.1**. The selected graph stayed
fixed through all three gates; only authorized group version preparation changed
local package/catalog entries. Wire's unpublished 0.17 sibling draft was not
used. Rust remains **1.99.1** and managed PocketIC server remains **16.1.0**.
No public identity, service frame, stored format, authentication or retry contract
changed in this compatible batch.

Shared **0.29.4**, revision `628ccc75d813bfd9bae777bfbaef55838d25b1b2`, is
adopted through the canonical exporter with all 155 snapshot files verified.
The retained-state guard now covers the shared runner and Blob's direct
verification/publication entrypoints. Refusal preserves evidence and an existing
receipt; failed post-validation admission withdraws only the new selectable
receipt. No retained cache, archive, key, provider journal or obligation was
deleted. Regression fixtures verify refusal before receipt invalidation or
publication lock/tool/registry effects.

Hosted CI now gives portable Contracts tests one Linux owner and Clippy its
three-host native owner. Manual group dispatch delegates Rust/native/installed
obligations to its selected plan instead of appending broad native-host tests.
Generated TypeScript and isolated compiler/feature checks remain independent;
normal main/PR/workspace dispatch retains native CLI/installation/restoration
coverage on all three hosts. Required job names and host roster are unchanged.

Each gate passed **641 native tests/doctests**, **82 installed Client/standalone
cases** and **270 Engine cases**, plus Wasm, graph/boundary, release-adapter,
metadata/rollback and exact mixed-version archive checks. Earlier focused adoption
also passed generated TypeScript qualification for 27 methods/137 frozen frames
and fifteen browser dependency assertions; its exact inputs/results remain in
`target/upstream-20261104-post071/`. Optional Chromium/provider/resource cases were
skipped as declared. This is Linux/PocketIC and cross-build evidence, not fresh
native macOS, live-browser/login or deployed-provider qualification.

No sibling edits/delivery, paid provider effects, canister installation, reset or
funding occurred. The existing live **0.69** trial and its unfinished provider
obligations retain their original tools, artifacts and journals. Release gates
precede version preparation: rebuild and qualify exact released Engine bytes
before any separately authorized service installation. The earlier sections below
retain their original evidence scope and are superseded by this delivery record.

## Published Contracts, Client and Engine 0.71.0

Authorized delivery completed from this checkout's `main` to `origin/main` and
crates.io, supplier-first. All thirteen members now select 0.71.0. Standalone
telemetry is retired and passive Contracts no longer selects Candid. Full service
and CLI integrations select `caffeine-candid` explicitly. The publication controller
remains supported. Each group passed its own 22-stage source-bound gate, then
publication compiler/Clippy and archive verification. No delivery failure required
source changes or a gate restart.

| Group | Release commit | Annotated tag |
| --- | --- | --- |
| Contracts | `654a2f982006835386f609476c01dc8402a60d5f` | `contracts-v0.71.0` |
| Client | `fbac972fc99cbc0b79ee916c52bd50bc7cb7ce79` | `client-v0.71.0` |
| Engine | `98042e2a042a5d3b10feb101eb5895ec3c2853a3` | `engine-v0.71.0` |

Independent readback verifies all three non-yanked registry archives, exact
checksums, embedded VCS source/package paths and annotated remote tags. Evidence:
`target/release071-readback/summary.json`. Successful gates are retained at
`target/blob-release-validation.e4I3mv/` (Contracts),
`target/blob-release-validation.pbMUkR/` (Client) and
`target/blob-release-validation.HNfCwb/` (Engine), with immutable committed group
receipts. Publication captures are `target/publication.vvMOqb/`,
`target/publication.WUrLq7/` and `target/publication.FiHn4W/`, respectively.

Each gate passed 641 native tests/docs, 82 installed Client/standalone cases,
270 Engine cases, Wasm, dependency admission and exact mixed-version archives.
Optional Chromium/provider scenarios were skipped; this is Linux/PocketIC and
cross-build evidence, not new macOS, application/browser or live-provider
qualification. Supplier versions remain fixed at their 0.70 delivery selections.
Rust remains 1.99.1 and Shared Tooling remains 0.29.3.

[#115 is closed/completed](https://github.com/dragginzgame/ic-blob-storage/issues/115#issuecomment-6631894091)
after confirming the implementation on default main and publication. Its checklist
is complete and state independently read back. Passive Contracts' normal/build
graphs exclude Platform facade, Candid, Agent and execution; native dev tooling
and explicit Caffeine consumers still use their required external boundary.

No sibling edit/delivery, new provider request, paid effect or canister install,
reset or funding occurred. The existing live 0.69 trial and all unfinished
obligations retain their original tools and artifacts. Release gates precede
version preparation: rebuild and qualify exact 0.71 Engine bytes before any
separately authorized service installation. The earlier pending sections below
retain their original source/evidence scope and are superseded by this delivery.

## #115 passive Contracts / Caffeine capability — pending 0.71.0

Implemented [#115](https://github.com/dragginzgame/ic-blob-storage/issues/115)
inside this repo while preserving the pending standalone-telemetry retirement.
Owned Principals now import canonical `ic-platform-types` directly, including
self-authentication in tests. Platform is optional under `caffeine-candid`.
That capability gates the complete account-inspection DTO family (including
wide Int/Nat decimal fields), account-link Candid encoder/private types, GC
codecs, decoder configuration and full endpoint mount macro. The passive
certificate response conditionally selects its Candid derive. No serialized field,
variant, method ID or decoder quota was changed. The mount macro's feature is
resolved at its defining owner; downstream crates need no matching local feature.
Full-service schema generation explicitly selects the capability, while provider
JSON, content hashing/verification and passive certificate/ordinary Wire DTOs
remain independent. Full service, native CLI and probe consumers opt in; the
replicated Client and test protocol keep passive direct selections. External
Caffeine types and protocols remain in Contracts/Platform, with no new crate,
wrapper, fallback codec, migration or storage changes.

The normal/build Contracts closures exclude facade, Candid, Agent and execution
on native/all-target and Wasm selections; provider-json independently retains the
same absence. CI now checks this boundary with explicit no-default-feature graph
selections and mutation refusals. Native dev tests intentionally retain Platform
Candid/service tooling and are not evidence of the minimal deployment graph.
Native and Wasm minimal Contracts builds, independent provider-json build/graph,
117 passive library tests, 129 all-feature contract tests (including the independent
full-width account-link fixture), 32 service-export checks and the frozen Wire
fixture pass. Strict Clippy passes for Contracts, replicated Client, Core, CLI,
standalone and PocketIC callers. Fresh standalone Wasm and replicated Client Wasm
checks pass; actual installed Caffeine callbacks and exact Wasm/provider export
parity pass. Dependency guards and their new Candid/facade refusal fixtures,
canonical formatting, documentation and diff checks pass. This is local Linux and
Wasm/PocketIC evidence; no full gate, native macOS or live-provider qualification.
The [implementation update on #115](https://github.com/dragginzgame/ic-blob-storage/issues/115#issuecomment-6630091986)
was posted and independently read back.
No staging, commit, push, release, sibling edit or provider effect is authorized
by this implementation; #115 stays open until the fix reaches default main.

## Standalone telemetry retirement — pending 0.71.0 hard cut

The maintainer approved assigning operational reporting to Canic's existing Blob
metrics adapter. Removed standalone `blob_metrics`, its heap recorders and reset
hooks, response-classification workflow, special instrumentation wrappers, public
Contracts report DTOs/identity, standalone schema/client generator modes and CLI
`metrics` command. The standalone host now implements only the shared service
catalog. Contracts, Client and Engine notes select their next minor, 0.71.0;
package metadata remains at released 0.70.0.

The direct dependency boundary now permits Metrics only for the private storage
probe; the resolved standalone deployment guard also rejects it. The probe's
resource measurements remain separate fixture qualification. Canic stays read-only
and owns its own adapter; this removal adds no framework dependency. Service
accounting, quotas, references, funding journals, operator inspection and provider
callbacks remain. Callback/recovery tests retain business-state assertions while
removing observations of the retired recorder.

The existing 0.69 mainnet trial is untouched. Preserve its original tools, saved
telemetry and signed journals for unfinished provider obligations; current source
retirement does not grant reset/reinstall or discard authority. Historical release
notes and probe records keep their original findings. The publication controller
and its recovery support remain under the retention decision below.

Focused checks pass under Rust 1.99.1: strict all-target/all-feature Clippy for
Contracts, CLI, standalone and PocketIC callers; three schema tests, the schema
example's CLI test and three product CLI grammar/path tests; fresh standalone
Wasm/CLI builds; seven installed gateway/callback cases; exact Wasm export/provider
contract parity; and installed admission/manifest/restoration. The non-UTF-8 path
test now uses the maintained `status` invocation with its full payer/Cashier scope
instead of the removed command. Two incomplete intermediate fixture invocations
failed argument admission before this correction; production parsing is unchanged.
Direct and resolved dependency guards, their refusal fixtures, canonical
formatting, documentation links and diff checks pass. A second source search
finds no retired metrics contracts/commands in current Rust, generators or test
callers. Metrics remains in the workspace lock only for private probe measurement;
it is absent from the standalone deployment graph.

This is local Linux/PocketIC and cross-build evidence; no full workspace/release
gate, native macOS or live-provider qualification was run. No staging, commit,
publication, sibling edit or provider request is part of this implementation.

## Published Contracts, Client and Engine 0.70.0

Authorized delivery completed from this checkout's `main` to `origin/main`,
supplier-first. All thirteen members are now 0.70.0. Each group passed its own
22-stage source-bound release gate, then its publication compiler/Clippy and
archive checks. The three non-yanked registry archives match their published
checksums, embedded VCS commits/package paths and annotated remote tags:

| Group | Release commit | Tag |
| --- | --- | --- |
| Contracts | `4d9041fe543a7799d492d16b41a85f63f8ae111e` | `contracts-v0.70.0` |
| Client | `e07475b12374fb3ea8e7074409fde9e2861b2cf8` | `client-v0.70.0` |
| Engine | `3a18f15e2743c0222956f22ec6d4a298eff821ec` | `engine-v0.70.0` |

Successful gate evidence is in `target/blob-release-validation.BmGCcn/`,
`target/blob-release-validation.flDhhv/` and
`target/blob-release-validation.QTh4kU/`, respectively; immutable group receipts
are committed with each tag. Publication captures are `target/publication.oZAeG6/`,
`target/publication.WeuvDT/` and `target/publication.BbJGgB/`; independent registry
and tag readback is in `target/release070-readback/summary.json`.

The first Contracts attempt retained at `target/blob-release-validation.223YaR/`
failed when the multi-phase installed publication-session test exhausted its
30-second total budget under concurrent load. Its retained `failure.json` reports
`timeout`. Commit `dc32c7d2` gives only that scenario a bounded 120-second budget
and includes the actual control response in assertion failures. The focused case
and all subsequent full gates pass; production deadlines and the separate
one-second idle-expiry test are unchanged. One initial focused invocation omitted
the required CLI artifact environment and failed setup before the corrected run.

Each successful gate ran 651 native tests/docs, 84 installed Client/standalone
cases and 270 Engine cases, plus Wasm, boundaries and exact archive verification.
Optional Chromium scenarios were skipped; this is Linux/PocketIC and cross-build
evidence, not new native macOS, browser or mainnet qualification. The delivered
graph includes Metrics 0.29.1 and Storage 0.66.2 alongside Platform 0.21.0,
Testkit 0.84.1, Host Tools 0.81.0 and Wire 0.16.1; no graph changes occurred during
the successful gates. Shared remains 0.29.3 and Rust remains pinned at 1.99.1.

The publication controller remains supported under the decision below. No
sibling was edited or released, and no canister was reset, installed, funded or
probed during delivery. The existing live 0.69 trial and its provider obligations
remain unchanged. Release validation artifacts precede version preparation;
rebuild and qualify exact 0.70 Engine artifacts before any separately authorized
service deployment. Older pending/qualification sections below retain their
original evidence scope and are superseded by this delivery record.

## Heap catalog retirement — delivered in Engine 0.70.0

The maintainer approved retiring the parallel heap catalog. Removed
`BlobCatalog`/`UploadCatalog`, their private accounting and heap read/cursor
adapters, obsolete error branches and the model-only catalog integration targets.
The admission probe now projects the durable reference failure directly instead
of rebuilding a retired catalog error. Shared capacity facts, pure lifecycle and
root-binding rules remain; no compatibility owner was added.

Authority experiments now use `StableUploads` on separately governed disposable
backing, retaining their runtime leases and explicit tenant/operator roles. The
existing archive still records every experiment lane and restores only fenced
inspection evidence; it never installs a reconstructed owner as live authority.
Fixtures prepare valid metadata-bound manifests before exposure. Anonymous empty
reads still refuse, registered issuance revocation does not become cancellation,
and invalid metadata refuses before reserving capacity. Frozen hashing vectors,
production endpoints and stored records are unchanged. Current docs describe the
durable owner; historical catalog source links select their verified introducing
Git revisions without rewriting the recorded outcomes/hashes.

The old model tests are retired with that API. Their maintained invariants are
covered at the canonical owner: rejected admission/root atomicity and empty-object
refusal, lifecycle/receipt replay through settlement and fenced reopening,
tenant-isolated indexed discovery/scans, filtered cursor progress, cleanup views,
independent callback authority/fences and corrupt-record rejection. All 58
durable upload tests and both authority archive validation tests pass. Actual
PocketIC caller/obligation/cancellation cases pass; all 50 affected journey cases
are covered across the initial run and focused repairs, including a new invalid
metadata/no-allocation regression. The admission reference-history/receipt-capacity
case passes after removal of the obsolete error bridge. Strict Clippy passes for
Core, both affected probes and PocketIC callers. The production Core Wasm graph
also checks with test support disabled; final probe Wasm builds, critical installed
read/malformed-input smoke cases, canonical formatting and documentation links
pass. Source LOC changes from 14,445 to 13,205 for Core and 46,554 to 45,728 across
the repository; authority setup grows to exercise the real owner. Test-source LOC
falls by 3,141. These are file classifications, not a Wasm/performance claim.
Commands, source hashes, coverage crosswalk and artifacts are indexed by
`target/catalog-retirement-20261104/summary.json`. Raw failures and successful
repairs remain under `target/catalog-retirement-20261104/`; the first local server
startup was sandbox-blocked and host execution supplied loopback access.

This is an Engine Rust API hard cut, covered by existing pending 0.70.0 notes.
Package versions remain 0.69.0; dependency selections and Cargo.lock are unchanged
by this retirement. No live-provider action, storage reset, sibling edit, staging,
commit or delivery occurred. The previous test-layout source inventory below
belongs to that earlier source snapshot; it is not the post-retirement test count.
No full workspace/release suite, native macOS or provider qualification was run.

## Publication controller investigation — retain pending Canic application adoption

Frozen batch preparation was added in `3d505561` (0.8.0, October 3), followed by
the session controller in `44737aac` (0.10.0) and the native Chromium/worker bridge
in `cc5c9937` (0.11.0). The original release notes explicitly call the production
headless publisher unfinished while describing local interruption, lost-reply,
corruption and restart qualification. Later source added original-session/profile
recovery. This was a Blob-owned standalone publication tool, not extracted Canic
runtime code. Current operator documentation still supports it and Blob's own
native/browser/PocketIC cases exercise it, so it is not unreachable/dead code.

No batch/session CLI callsite was found in the available Canic or IC Crates
production source. Canic embeds Core/Client; IC Crates has a different publisher
workflow and a still-blocked older Blob worker, which is not qualification of this
controller. This is bounded source evidence, not proof nobody invokes it manually.
The follow-up investigation distinguishes three maintained layers:

- Core, contracts and generated endpoint clients support Canic's service adapter.
- Browser upload, certificate handling and durable retry journals support actual
  application transfers independently of the native controller. Browser-only
  launch selects explicit `nativeSession:null` when creating its profile.
- Native `publish-session` coordinates frozen batch preparation, browser transfer,
  inspection and final asset maps. Its browser subprocess control and native-bound
  profile recovery are coupled to that command; they are not disposable merely
  because Canic's service adapter does not invoke it.

The inspected Canic adapter manifest selects Core and Contracts, not Blob's CLI.
Its generated-browser qualification invokes mounted endpoints through PocketIC,
not a complete application publisher. No controller caller was found in its
manifests, source or scripts. This establishes the current local integration
boundary, not future requirements or the absence of private/manual callers.
The controller was intended to avoid each consumer rebuilding batch orchestration
and interruption recovery; its absence from Canic's framework adapter alone does
not establish redundant ownership. See the maintained
[operator session contract](../operator-guide.md#hold-one-validated-batch-across-publication-phases),
[browser integration guidance](../../clients/browser/README.md) and original
[0.10 release notes](../../releases/history.md).

**Maintainer decision, 2026-11-04: retain the controller and its supported browser
integration while Canic's application publishing workflow takes shape.** This
supersedes the earlier suggestion to consider immediate controller retirement.
Revisit when an actual consumer selects its upload/publication path: trace its
callers, determine whether the controller replaces application scaffolding or
duplicates it, and preserve exact intents, signed-operation journals and original
profile recovery. Ordinary browser uploads do not require adopting the controller;
future adoption is not assumed or required. Do not move application authorization
and retry policy into Host. Nothing in this controller was removed. Investigation
was read-only source/history review, with no new runtime qualification or sibling
edits; this decision note changes documentation only.

## Shared Tooling 0.29.3 and test-source layout

Adopted committed Shared Tooling **0.29.3**, revision
`1fa2d90879650780d23ad8f045f5825771b696ad`, through its canonical pinned exporter.
Remote main matches this revision; dirty supplier files were not exported or
modified. The existing 154-file selection verifies independently. AGENTS follows
this reviewed revision, including the new test-layout and owning-group issue
rules. The supplier's changed CI classifier is producer-owned, not part of Blob's
selected executable surface; Blob's validation rosters remain unchanged.

Extracted all 33 remaining inline unit-test modules into owner-local `tests.rs`
files. Their module/test identities, privacy, bodies and fixture paths are
preserved; formatting-equivalence was checked against saved original sources.
Moved `configuration_tests/mod.rs` to `configuration_tests.rs` under the same
module identity so the existing LOC classifier recognizes it as test source.
Existing `tests/mod.rs` hierarchies and integration targets were retained. Public
feature-selected probe support stays explicit: it is a consumed probe API, not
an exclusively unit-test module. No path attributes, visibility widening,
runtime changes, persistence changes or new wrappers were introduced.

Removed the duplicate pending issue summary from the root changelog; direct
outcome links remain in the owning group notes. The existing instrumentation-only
Metrics dependency cleanup already meets the new rule. The supported standalone
operator report remains intentionally separate from Canic's application-tree
adapter; deleting that endpoint requires a public-contract decision, not a prose
adoption. Existing 0.70.0 notes cover this cleanup; package versions remain 0.69.0.

All 93 tests in affected modules pass across Core, Contracts, CLI, the schema
example and authority probe. Snapshot integrity, canonical formatting and diff
checks pass. Commands, original sources and focused results are retained under
`target/shared0293-adoption/`. The canonical LOC report recognizes all 1,048
test-function attributes in test files (zero classified inline); this is a source
inventory, not a claim that the complete test suite ran. No full suite, new Wasm/PocketIC/provider run,
staging, commit, delivery or sibling edit was performed. Prior runtime evidence
below remains applicable to unchanged production behavior and dependency graph.

## Current upstream adoption — pending Contracts / Client / Engine 0.70.0

Adopted published **Platform 0.21.0, Metrics 0.29.0, Host Tools 0.81.0 and
Testkit 0.84.1**. Build / Host Foundation 0.74.2, Storage 0.66.1 and Wire/Derive
0.16.1 remain selected. Runtime 0.19, Types 0.18 and PocketIC server 16.1.0 are
unchanged. Registry metadata/checksum-verified supplier archives are retained in
`target/upstream-20261104/`. Direct third-party Rust/browser declarations are
already current; npm files were preserved. One oversized Playwright metadata
capture was retained as a failed candidate; the official latest-version endpoint
then confirmed 1.64.1 without changing browser selection.

At completion of that upstream batch, Shared Tooling **0.29.2**, commit
`04a87b366eae3e63b9f032c0b891c17bdc59e69b`,
was independently verified (154 files) after canonical `--revision` export from
the committed tree. Unrelated dirty supplier source stayed untouched. AGENTS
referenced that reviewed baseline (superseded by 0.29.3 above). The adopted native
dependency rule admits
Testkit's owned client and retires the former Platform-client exception, resolving
the previous Shared/Tooling ownership follow-up. Grouped manifest and UTF-8
runner evidence fixes are adopted together; no producer draft was exported.

Metrics is now used only by instrumenting standalone/probe hosts. Removed its
Contracts normal dependency and native CLI test dependency. Contracts owns strict
report decoding and numeric admission, retaining previous refusal priority,
unknown-field rejection and the frozen Wire/JSON representation. No report
reader uses recorder Serde or the governed storage restoration API. Engine
projects recorder fields explicitly; instrumentation-state fixtures select the
explicit restoration API. The direct dependency policy allows Metrics only in
the two instrumenting hosts, and the transitive schema-graph guard rejects it.
Its focused mutation fixture covers default/empty and retired feature selections.

Public `From<MeasurementCounter/MeasurementAggregate>` report conversions were
removed, so all three pending group ledgers now select **0.70.0**. Existing package
versions remain 0.69.0 until authorized release preparation. Report method IDs,
canonical bytes, limits and stored records are unchanged; the standing
cross-release retirement/fresh-backing policy still applies. The existing
standalone operator endpoint is deliberately retained: core linkage owns no
endpoints, and this repository cannot depend on Canic's framework adapter. Any
future endpoint consolidation needs an explicit owning contract decision.

Focused checks pass: 13 native report/instrumentation/CLI tests, strict affected
Clippy, fresh standalone/storage-probe Wasm and CLI builds, four local PocketIC
metrics/install/restoration cases, frozen generated TypeScript report round-trip,
malformed/trailing rejection, dependency/compiler/graph guards, canonical tool
setup/offline admission and all three groups' isolated metadata/rollback adapters.
The first metadata attempt refused an unprepared Build 0.74.2 editor; its log/
fixture were retained, explicit canonical setup completed and the focused
three-group test passed fresh. No generic producer/full workspace matrix was run.
MacOS, hosted release and real-provider acceptance were not rerun. These local
substitute checks neither replace nor invalidate frozen 0.69 live evidence.
No staging, commit, push, release, mainnet/provider action or sibling edit occurred.
Final source/selection/artifact checks and retained logs are indexed by
`target/upstream-20261104/summary.json`. The consumer repair was posted and read
back on [Metrics #130](https://github.com/dragginzgame/ic-observability/issues/130#issuecomment-6626043442);
that issue retains the other consumer/Storage acceptance and future endpoint
coordination. No provider or framework issue was closed by this adoption.

## Active Blob-owned trial: verified transfer and exact logical release

The maintainer continued the fresh standalone/provider trial as **Blob-owned**
work, independently of Canic. The configured `canic-mainnet` development key is
authorized; no existing key was exported, sibling edited, commit made or package
released. All product versions remain **0.69.0**. Default identity stays anonymous.

Fresh mainnet service **`4h7vw-4qaaa-aaaau-ahieq-cai`** has frozen module/init,
release/controller and unfenced readback. Two tenants / two lifetime 1KiB objects
are bounded to 2KiB physical/liability bytes; these are not production defaults.
[Installation](../evidence/caffeine-probes/deployed/2026-11-03-trial-v069-install-01/summary.json),
[initial provider refusal](../evidence/caffeine-probes/deployed/2026-11-03-trial-v069-provider-01/summary.json),
[approved allowance](../evidence/caffeine-probes/deployed/2026-11-03-trial-v069-allowance-01/summary.json),
[delayed allocation](../evidence/caffeine-probes/deployed/2026-11-03-trial-v069-bootstrap-01/summary.json),
[successful second transfer](../evidence/caffeine-probes/deployed/2026-11-03-trial-v069-transfer-01/summary.json)
and [subsequent disposable-object release](../evidence/caffeine-probes/deployed/2026-11-03-trial-v069-gc-release-01/summary.json)
are separate, immutable phases. Later results do not rewrite the earlier zero
credit or 403 observations.

- Two distinct settled deposits remain **1.001T gross / 1.0006T credited**, with
  400M aggregate fees (blocks `17080816/17080834`, `17081076/17081098`). Both returned
  deposit addresses ended empty. No extra deposit, ICP conversion, notification,
  allowance change or fabricated Blob attachment-journal entry occurred this turn.
- The explicitly approved **5T daily** relationship retains original expiry
  `1793728110105542110` (**17:48:30 UTC, November 3**), zero overdraft/no auto target.
  Immediate readbacks were zero owner/gateway credit. At **17:02 UTC**, scoped
  queries instead reconcile payer **600M**, owner **1T**, period spend **1T** and
  gateway raw credit **166666666666**: a delayed internal allocation. The actual
  cadence/minimum and whether the 5T increase was necessary remain unproved.
- **User 1 stays held: observed certificate, first tree PUT 403, no chunk PUT.**
  No retry, new certificate, reset or release. Raw 403 body remains unavailable;
  zero credit was a readiness gap, not proof of the exact refusal reason.
- **User 2's first upload/download succeeded.** Its expired, unused candidate
  remains intact. Fresh inputs kept the same original body/root and operation /
  object / reference 2, with permission expiry 17:35 UTC. Admission/preparation
  and signed assessment passed. One browser certificate call and two PUTs returned
  200; one native descriptor/GET verified the root and **all 1024 original bytes**.
  This is one successful isolated test-key user, not two application users.
- Two local import failures occurred before any browser/paid transfer. Original
  binding-only profile was retained; the canonical empty journal was explicitly
  created with external traffic blocked and zero jobs, then reopened through the
  maintained launcher. No profile/database history was erased or replaced.
  A separate raw query used a saved document rather than endpoint framing and
  trapped; the retained explicit native query corrected it without mutations.
- Post-transfer queries report payer **600M**, owner **999507426465** and gateway
  raw credit **166584571077**. Usage counters still return zero. These are
  time-bound observations, not an itemized upload price or billing evidence.
- The maintainer then requested remaining issue work. **Only successful object /
  reference 2 was logically released**, exact operation **7**, one persisted update
  acknowledged. The receipt query finds its original success; no resend occurred.
  Reference 2 is now not live. Public liveness `[first, second, second, unknown]`
  changed from `[true,true,true,true]` to `[true,false,false,true]`. User 1 stays
  live/untouched. Operator history shows object 1 `Live`, object 2 `DeletionPending`.
- Current local logical bytes are **1024**; physical/liability bytes remain
  **2048**, zero active/reserved bytes, all owners unfenced and zero attachments.
  Both lifetime object slots remain occupied. A single <=1KiB original URL GET
  after release still returns **200 and the exact original bytes**, with
  `application/octet-stream`, wildcard CORS and one-year immutable cache headers.
  `Cache-Control: no-cache` is only a request hint; this may be cached availability.
  Owner/gateway observations remained unchanged; none establishes stopped billing.

No provider acknowledgment was impersonated, forced GC requested, physical/billing
confirmation invoked, refund performed or history reset. No physical deletion or
billing cessation is inferred. Keep failed user 1, the released user-2 root,
original receipt, all provider credit and old 0.6/0.7 obligations. The rejected
null-expiry allowance packet stays unsubmitted. Do not repeat payments, retry
user 1, extend expiry or reset backing to gain capacity.

**What remains:** #6 needs independently attributable real gateway/deletion and
billing evidence; `DeletionPending` alone does not reveal its private acknowledgment
bit, and even a future public 404 would not prove physical origin deletion or
billing settlement. A bounded later observation must have its own recorded intent;
there is no polling daemon or automatic financial/refill action. #5/#31 need an
actual selected application/session/media-publication run; this standalone trial
cannot supply its signed-in users/CSP/transaction. #34 retains lost/uncertain paid
outcome attribution, receipt replay and complete competing activity. No current
implementation defect was reproduced by this release/receipt/liveness run.

Private custody: `.git/private-operations/blob069-trial-01/` retains fresh keys,
frozen artifacts and all original native/browser journals outside `cargo clean`.
User 1: `transfers-01/user-1/`, port **43170**, DB **blob069-acceptance-user-1**.
User 2: `transfers-01/user-2-first-attempt/`, port **43171**, DB
**blob069-acceptance-user-2**. Open only original profiles/bindings. Raw captures:
`target/post069-provider/`; private copies include phase1, approved and bootstrap
capture directories. `gc-release-01/` retains the exact signed release, receipt
and private post-release captures. No external backup is claimed. The retained 100T total / 20T
scoped daily trial allowance and conservative 8T envelope are authority/accounting
bounds, not available liquidity or guaranteed provider bill caps.

Material results were posted and independently read back on
[#34: allocation and spendability](https://github.com/dragginzgame/ic-blob-storage/issues/34#issuecomment-6610548366)
and [#31: one verified live transfer](https://github.com/dragginzgame/ic-blob-storage/issues/31#issuecomment-6610549010).
Both stay open for their remaining acceptance. Focused evidence assertions,
98-file private-copy hashes, unchanged lockfile and documentation checks pass
(1778 local references / 225 documents). The subsequent release phase adds ten successful queries, one release and one
bounded post-release GET; its evidence assertions pass. No Rust code changed or
full gate rerun.

The new [#6 release/deletion findings](https://github.com/dragginzgame/ic-blob-storage/issues/6#issuecomment-6610790546)
and [#5 deployed serving headers/limits](https://github.com/dragginzgame/ic-blob-storage/issues/5#issuecomment-6610790967)
were posted and independently read back. No issue was closed by waiving unfinished
provider or application acceptance. Focused link/diff and evidence/hash checks
pass; the changed checkout graph remains outside this frozen-artifact proof.

## Earlier #6 investigation and standalone operator generation — 2026-11-03

The [bounded later readback](../evidence/caffeine-probes/deployed/2026-11-03-trial-v069-gc-followup-01/summary.json)
uses the existing `blob_metrics` endpoint with the configured ICP identity; no key
was exported. Its reported lifecycle window records **zero successful gateway
acknowledgments**, with 2 admissions, 2 preparations, 2 registrations and one
reference change. A zero count does not exclude trapping/rejected callbacks or
read-only gateway queries; a positive aggregate would not bind an exact object.
Current operator history remains object 1 `Live`, object 2 `DeletionPending`;
logical/physical/liability bytes stay 1024/2048/2048. One body-free original URL
HEAD still returns 200. No funds, IC update, forced GC or repeated upload occurred.
#6 therefore still lacks actual provider deletion and billing evidence.

A small standalone generation helper was initially drafted for Contracts 0.69.1;
it is now included in the pending 0.70.0 batch:
`wire_schema standalone-metadata` / `standalone-client` emit only the existing
standalone metrics schema using canonical DTOs, identity and budgets. The shared
client/mount remains unchanged; native key export, copied metrics DTOs/packets,
new endpoints, persistent state and runtime/deployed code changes are unnecessary.
One failed offline import from the shared-only client is retained separately.
Generated codec requests work against the original deployed 0.69 endpoint.
Focused native schema and CLI-mode tests, strict Contracts/example/test Clippy,
Rust formatting, exact live-response re-encoding and five malformed/bounds cases
pass under Rust 1.99.1 with the incoming Wire 0.16.1 lock. This is optional
host-generator/codec qualification, not full dependency or deployed Wasm adoption.

The [#6 acknowledgment/tooling update](https://github.com/dragginzgame/ic-blob-storage/issues/6#issuecomment-6611065960)
was posted and independently read back. Documentation checks pass (1785 local
references / 225 documents), as do source/artifact hashes and diff checks. Complete
private follow-up custody is retained under `gc-followup-01/`; no Git/package
publication, financial effect or new runtime deployment occurred.

A current read-only Canic review still has base `0509ec870` and substantial local
work. Its Cargo now selects Blob 0.69.0 and Timers 0.39.0, but the handoff continues
to leave the application/network/service acceptance binding unrecorded. No sibling
was edited, rebuilt or newly qualified. #5/#31 application acceptance cannot be
claimed from this standalone test-key trial.

## Earlier Testkit 0.84.0 adoption — 2026-11-03

Verified the latest non-yanked published **ic-testkit 0.84.0** archive against
crates.io checksum `be5b12efa43c51293328a0c2802864e40b05af3e4c23fb51fc593deac26ab0d8`.
It owns the maintained client/DTO/error module `ic_testkit::pocket_ic`; lifecycle
helpers remain in `ic_testkit::pic`. Migrated 55 native source files and removed
the separate `ic-platform-pocket-ic` workspace/member dependency. No compatibility
alias or wrapper was added. The resolved graph loses the retired client package
and its unused `zlib-rs` dependency. Platform still owns external Candid, Agent and
management adapters; Types 0.18, Runtime 0.19 and server **16.1.0** are unchanged.
AGENTS and the direct dependency policy now require the Testkit boundary.

The canonical lock-selected CLI setup reuses an authenticated shared store entry;
offline server admission passes. Strict Clippy across the affected private test
package's targets, two typed-error unit tests and four exact local PocketIC cases
pass: CLI installation/actual-service binding, stop/start/repeated upgrade,
snapshot rollback fences and captured uncertain funding recovery. Dependency
pins, direct IC boundaries and runtime/deployment graph exclusions pass; the
standalone graph contains no Testkit/native client. Rust formatting, documentation
and diff checks pass. Evidence: `target/post069-testkit-adoption/summary.json` and
its retained logs, published package and exact fixture hashes.

These installed cases reuse previously qualified local Blob 0.69 fixture/CLI
bytes: Testkit is native-only and no deployed source changed. They do not freshly
qualify all incoming Build/Wire/production selections, macOS or a full release.
Existing live-provider captures stay bound to their original artifacts. No mainnet
request, provider effect, sibling edit, staging, commit, publication or deployment.
The unrelated Build 0.74.2, Wire/Derive 0.16.1, Metrics 0.28.2 and Storage 0.66.1
selections are preserved. Current lock hash is
`9700fd069cf3be18c86115919c82cb96006405f837ca34a0f6f92ed9bc86d2d8`.
The pinned Shared policy's generic producer exception still names the former
Platform client; its snapshot was left intact. Blob's own direct prohibition and
actual graph checks enforce this consumer migration; upstream policy ownership
must be reconciled by Shared/Tooling, not by patching copied rules here.

## Earlier compatible patch qualification — 2026-11-03

A further incoming lockfile edit arrived during the release probe: **Build 0.74.2
and Wire/Derive 0.16.1**, observed lock hash
`084fba1b42afa326c938b7572032e5075d8f6ef664b3490bca90e2dbf618c149`.
Preserved without adoption/rebuild claims. All live operations above used the
frozen already-qualified CLI/Wasm and their original `d07780b0…` artifact graph;
they do not qualify this changed checkout. The explicit lock-equality assertion
failed and is retained as `gc-release-01/validation-lock-drift.json`; artifact/hash
and live-behavior assertions pass. No effect was repeated to address lock drift.

Incoming **Metrics 0.28.2 / Storage 0.66.1** lock selections are preserved. Verified
published archives have identical production Rust sources to 0.28.1 / 0.66.0;
compatible manifest floors remain unchanged. Evidence: `target/post069-adoption/`.
Strict affected Clippy, compiler/graph guards, 22 native report/installation tests,
fresh Wasm/CLI and five installed cases pass. Four local Chromium scenarios plus
three browser-control tests pass with unchanged inputs/artifacts. They use a local
provider substitute and do not replace the real refused trial above. The template
carrier rehearsal is local-only; actual live init/readback is retained separately.
Operator guidance now distinguishes payer, service-owner and gateway credit before
exposure; that earlier draft is now folded into **0.70.0 Unreleased**. Nothing staged.

## Published Contracts, Client and Engine 0.69.0

The explicit direct `origin/main`, three annotated group tags and crates.io
delivery completed supplier-first; all thirteen member versions are **0.69.0**.

| Group | Release commit | Annotated tag |
| --- | --- | --- |
| Contracts | `02622fdd11c0ba8e3b5dc993cb4151f09f53d694` | `contracts-v0.69.0` |
| Client | `46e0ab13dfba63973b670f99cfee5e38cea479df` | `client-v0.69.0` |
| Engine | `d4aded0b01291534a60b50fcc459971fbc11962b` | `engine-v0.69.0` |

All three non-yanked registry/archive checksums, VCS package paths, commits and
remote tag/peeled readbacks match. Evidence: `target/release069/`; publication runs
`target/publication.tgy1WF/`, `.8rG0dw/`, `.riDV5k/`. Each Git group release plan is
complete. A later handoff commit is not the group's publication checkout/receipt.
Each normal group release passed **22 stages**, including **692 native tests/docs**,
both installed partitions, Wasm, strict Clippy, compiler/graph/tool/deployment
admission and exact mixed-version archive/publication/recovery checks. Attempts:
`target/blob-release-validation.aSBHWU/`, `.5ziODV/`, `.cOWR16/`; source-bound receipts
also remain in Git release-state and each committed group `release.json`.

Host Tools 0.80.0, Platform/PocketIC 0.20.0, Runtime/macros/guard 0.19.0, Types
0.18.0, Wire/Derive 0.16.0, Testkit 0.83.0, Build 0.74.1 and Host Foundation 0.74.2
are the published 0.69.0 selections, preceding the pending adoptions above.
Shared **0.28.2** at `5cf67f81c50c56ac17389ae71a3a87d46e86e8a2`
has 154 verified files. Rust **1.99.1** is both minimum and pinned compiler.
PocketIC server **16.1.0** is unchanged. Deployment excludes native tooling,
retired Memory and stable-structures; canonical execution/type/Wire owners are single.

## Issues, consumer boundary and hard-cut custody

**#113/#114 closed/completed**, independently read back. Default-main hosted run
[41648272239](https://github.com/dragginzgame/ic-blob-storage/actions/runs/41648272239)
at `ae8bc6ec` passed schema/generated clients, Core/Client and Linux/macOS ARM/macOS
Intel tooling. Contracts owns passive Metrics Wire DTOs; Metrics owns arithmetic.
Shared owns roster/baseline/run verification; Blob keeps product mapping and its
self-contained preparation/publication custody. That delivered baseline had no
version projection or gate reuse; the current pending follow-up is described above.

Open **#5/#6/#31/#34** retain publication/serving, deletion/billing, multi-user
transfer and provider funding/credit acceptance. Do not close from installation,
positive payer credit, registration or local substitutes. Current provider findings
are Blob-owned, not waiting on a framework adapter. Canic independently owns its
active composition and application work: the earlier read-only review at base
`0509ec870` still selected Blob 0.68.1 and reproduced a published Timers/Runtime
cohort conflict, recorded in [Canic #622](https://github.com/dragginzgame/canic/issues/622#issuecomment-6606081330).
Preserve that active checkout; it does not block this standalone provider trial.

The Engine cut uses `BLFUPR01` and the installation funding-progress identity.
Cross-release transitions retire obligations with original tools before fresh
whole backing: no fallback reader, migration or automatic reset. Same-release
recovery and exact original intents remain required. Earlier deleted scratch was
not synthesized; historical results retain their original source/graph. Library
publication, IC installation, provider success and application acceptance remain
separate claims with their existing evidence owners.
