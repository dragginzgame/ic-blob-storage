<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Current status

Date: 2026-10-06

## At a glance

- Released baseline: `0.15.1`; the changelog and release receipt own exact
  release metadata.
- Current development target: `0.15.2`; compatible Shared Tooling verification,
  native fixture repair and dependency qualification. Cargo's package version
  and the release receipt remain at 0.15.1. The selected graph now uses ic-memory
  0.28.2, direct native ic-host-tools 0.2.0 and native harness ic-testkit 0.19.2
  (which retains transitive ic-host-tools 0.1.14).
  The earlier accidental Cargo rollback was restored;
  its diff and original files remain under `.tmp/post-release-0151-01`.
- Product state: working library and standalone prototype with retained live
  upload/download evidence, not a fully accepted production service.
- Current composition: framework-independent core, explicit standalone host,
  native operator tools and browser publication components.
- Recovery: same-release restoration opens fenced; current-instance activation
  requires independent IC history, and older-snapshot activation is unsupported.
- Latest batch: canonical Shared Tooling adoption now records 54 files at
  `d957d1f`, including shared lockfile rewriting, formatting-hook adoption and
  Cargo inheritance checks. Focused Linux Bash 5/Bash 3.2 checks pass; the consumer's
  native macOS run remains required. Fresh checks pass 106 native CLI cases,
  the FIFO subprocess, eight core installation cases, two PocketIC decoder cases
  and 35 offline browser cases. CLI Clippy and Rust 1.88 compilation pass with the
  current direct host-tools selection. Earlier installation/funding/restore/Wasm
  records retain their original graphs. Tool pins, public core/service contracts
  and durable records are unchanged. GitHub owns the reconciled issue disposition.
- Finishing checks now also qualify the actual CLI-generated installation carrier
  against fresh matching standalone Wasm on the direct ic-host-tools 0.2.0 graph.
  Both selected PocketIC cases pass, including wrong-service/state preservation
  and exact configuration readback. Native CI now uploads failed shared metadata
  and lockfile fixtures; controlled refusals retain both complete directories.
- Main remaining work: provider credit evidence and consumer funding integration,
  consumer adoption, complete publication transactions, provider deletion and
  billing evidence, production sizing and retirement.

## Retained release baseline

Released **0.15.1** is at commit `c3e271449753f782a0193314f4ed3d4db21c453f`,
directly following source `ee7eed5298e072a0278886cf06970e10009c3d40`. Annotated
`v0.15.1` is `5554613d3d0034cb2391420729acf4ea412a5a20`; remote main and the
peeled remote tag match the local release commit. The committed receipt hashes
and exact annotated tag verify read-only, independently of dirty Cargo files.
The maintainer reports it live; this is Git release evidence, not independent
registry publication or service deployment evidence.

The released lock selects ic-memory 0.28.0, ic-host-tools 0.1.12 and ic-testkit
0.18.3. Earlier local records below keep their actual 0.27.1/0.1.11 graph and
compiled 0.15.0 identities. They are not relabelled as qualification of these
later dependency selections. The new
[native CI run](https://github.com/dragginzgame/ic-blob-storage/actions/runs/37460326994)
has completed: Linux passes, while both macOS hosts fail the valid large-body
download fixture with `Content`; their PocketIC installation steps are not reached.
Raw logs remain in `.tmp/v0152-review-02`. The pending fixture repair and selected
dependency qualification below are separate local evidence; no green native
macOS matrix or new release is established by them.

The post-release follow-up updates this handoff, the release guide and the
committed dependency inventory. The release receipt/tag, unchanged 43-file
snapshot, restored Cargo files and all 81 supported local references in those
three documents pass focused checks. No Rust source changes or rebuild occur;
the prior full gate is not attributed to a new source or dependency graph.
Its earlier CI readback preceded the completed macOS failures above.

Shared Tooling main is now
[`47cd2cc`](https://github.com/dragginzgame/shared-tooling/commit/47cd2ccaf0e8b428f06e6db0262df76cfc1581de),
one committed batch after the earlier adopted `a37771f`. Source review finds portable
digest generation, IC receipt traversal refusal and optional documentation,
release-command, registry-observation and RustSec preparation helpers. Its
[matching CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37458968809)
has completed with Linux RustSec and both macOS version-file fixture failures.
That initial post-release review left the snapshot unchanged and used the
documentation helper read-only, retaining its source under
`.tmp/post-release-0151-01`. The later 48-file adoption below is distinct; neither
a newer commit nor undated 0.1.7 notes establishes complete native qualification.

### Earlier 0.15.0 baseline

Released **0.15.0** is at commit `6ba6cbc8e6fa87b5e8d2836dd69c0012fffa9e66`,
directly following source `1a1139e5ae4fadb7cd082c7c929b8909b618fae6`. Annotated
`v0.15.0` is `5835e7db7b23f6a1db3245b2f99529fc49db8fb5`. Cargo, dated notes
and the release receipt agree at clean task entry; the read-only tag/receipt
check passes. This is Git release evidence, not registry publication or service
deployment. Earlier funding profiles keep their original compiled versions.

### Earlier 0.14.12 baseline

Released **0.14.12** is at commit `ebd535eaa2015407319fd330f559b2808ea01588`,
directly following source `91ac488a0e04cf219eafec6b396e728ea0a0890a`. Annotated
`v0.14.12` is `e556c96f7e445bf23b43bb9268b5f4a69cbeaf61`; local HEAD and cached
origin/main agree. The maintainer reports the push complete. Its release receipt
and file hashes verify at this task's clean entry; new draft notes do not rewrite
that historical receipt. This does not establish registry publication or service
deployment. Earlier validation artifacts retain their original compiled versions.

### Earlier 0.14.11 baseline

Released **0.14.11** is at commit `b526ca5bcc2e7976281c60880f83a12bd216c12d`,
directly following source `08783a2d537bf25de02da2faabddc6bd2c1eebee`. Cargo,
the dated changelog and [historical receipt](https://github.com/dragginzgame/ic-blob-storage/blob/v0.14.11/docs/release.json)
recorded 0.14.11. The maintainer
reports it live; this handoff does not independently establish registry or
service deployment. The selected graph includes ic-memory 0.25.9 and ic-testkit
0.15.8. Its pre-release profiles retain their actual compiled 0.14.10 identity.

The released 0.14.12 batch is compatible internal maintainer tooling; no published
Rust API, blob CLI, service DTO or persisted layout changes. The old manual
release phase targets are retired in favor of the common runner and saved
recovery; see [the release guide](../releasing.md). Formatting changes source
manifest presentation without reselecting locked dependencies.

### Earlier 0.14.10 baseline

Released **0.14.10** is at annotated tag
`95f102c02b6e6f5f3e4f43e1c132e1928c541055`, directly following validated source
`c3143d6c803b5bc31c2a300f81f5b68deba64953`. The release-file hashes, date and
version in the [historical receipt](https://github.com/dragginzgame/ic-blob-storage/blob/v0.14.10/docs/release.json)
verified at that task's entry. The maintainer reported 0.14.10 live; local main
and cached origin/main then agreed at the subsequent
documentation merge `baee7a13208390f590871e97874337afaf5867b4`. This is not a new
registry or deployment observation. [The changelog](../../CHANGELOG.md) owns
release notes; its then-undated 0.14.11 heading collected post-release work.
The released lock selects ic-memory 0.25.5, ic-testkit 0.15.4, management types
0.11.0 and transitive powerfmt 0.2.1. Earlier retained profiles keep their
original compiled releases and dependency graphs.
During the earlier ordinary-history batch, the maintainer's concurrent lockfile
update selected ic-memory 0.25.9 and ic-testkit 0.15.8. That frozen profile uses
0.25.5/0.15.4; the new 0.14.11 record below separately checks the selected graph.
Retained observations below keep their original source and artifact identities.

The core owns tenant policy, uploads, manifests, references, quotas, provider
economics and local durable journals. Standalone authenticates platform context
and delegates to shared handlers. Consumers own their framework wrappers and
certified asset-registration transactions. Linking the library exports no endpoint
or lifecycle hook; siblings remain read-only.

Trusted uploaders and an explicitly configured external verifier remain required.
Logical release, physical deletion and billing cessation are separate facts.
Restoration validates owners synchronously and fences mutation; independent
current-execution IC history is required to resume. Older-snapshot activation and
cross-release upgrades remain unsupported. Retired roots cannot be reintroduced.

## Qualification carried forward

- The [clean released-source installation](../evidence/caffeine-probes/README.md#clean-released-source-tool-installation--2026-10-04)
  qualifies tagged 0.14.3, a fresh locked native prefix and pinned browser build
  without Git discovery. Matching tools/Wasm complete original-cache GLB/WebP and
  recover PNG/JPEG lost replies/control interruption without another PUT. Original
  roots, leaves, whole bytes, cache headers, complete maps and reference liabilities
  agree. This is repository-owned local/substitute evidence, not consumer adoption.
- The [publication deadline record](../evidence/caffeine-probes/README.md#publication-fixture-session-deadline--2026-10-04)
  gives original/recovery fixture sessions, browser and subprocess owners one
  explicit 120-second bound. Single-step calls remain 30 seconds and the whole
  fixture 180 seconds. Production defaults and provider retry policy are unchanged.
- Released 0.14.5 consolidates bounded native input decoding and permission
  validation, owns browser journal closure/bootstrap snapshots once, and keeps
  phase/final serialization inside finite refusals. Original profiles, uncertain
  claims and shutdown refusals survive. Its scoped native/Chromium records retain
  their original releases and dependency graphs.
- Released 0.14.6 uses bounded base64 body frames and removes one redundant SDK
  array copy before Blob construction. The [handoff profile](../evidence/caffeine-probes/README.md#browser-body-handoff-profile--2026-10-04)
  records 1/8/32 MiB preparation/refusal observations improving from
  2.99/22.88/86.41 s to 0.08/0.48/1.83 s. These stop before certificate intent and
  measure a client bridge, not Canic execution or successful upload latency.
  [SDK/Chromium checks](../evidence/caffeine-probes/README.md#sdk-owned-blob-preparation--2026-10-04)
  preserve selected-view ownership, binary bytes, MIME/metadata, exact substitute
  payloads, one-shot handles, lost replies and budget refusals. Peak memory is not
  qualified. New executable bundles do not replace retained profile bindings.

The [0.14.6 pre-release handoff](history.md#0146-pre-release-handoff--2026-10-04)
retains detailed scoped validation, failures, artifacts and earlier baseline claims.
No release relabels those observations. The new
[matching released-tools rehearsal](../evidence/caffeine-probes/README.md#matching-0146-media-rehearsal--2026-10-04)
separately qualifies complete 0.14.6 service/CLI/browser journeys on ic-memory
0.25.0 and ic-testkit 0.14.11. Earlier journeys keep their original identities.

## Released 0.14.7 batch evidence

The maintainer completed the 0.14.7 release. The observations below retain their
original compiled versions and source identities; the release does not relabel
earlier binaries or profiles. The new resource batch below builds on this baseline.

The [browser journal profile](../evidence/caffeine-probes/README.md#populated-browser-journal-profile--2026-10-04)
uses the maintained store, synthetic bindings and strict individual IndexedDB
commits. Its first 20,000-row population hits the fixture's 180-second deadline
following the 17,500-row checkpoint. That failed run/profile remains intact;
there is no reported per-transaction timeout or provider effect.

The bounded follow-up populates and immediately reopens 675/5,000/10,000-row tiers.
Population takes 2.08/22.60/63.67 s; median selected reads after graceful Chromium
restart take 1.4/4.4/7.0 ms. Exact uncertain certificate, cancelled saved and
observed-certificate/uncertain-gateway rows survive. Full lifetime capacity,
certificate replay, cancellation and gateway uncertainty still refuse; saving an
existing row at capacity preserves its exact history.

The opt-in fixture retains each block and completed tier before proceeding,
without default CI work or production timing thresholds. It introduces no state
counter, cache, schema change, retry or relaxed capacity guard. The documentation
now distinguishes point lookups plus IndexedDB count from constant-cost work;
no JavaScript full-store materialization does not prove an engine avoids scanning.
Paired tools build; scoped syntax/changelog/diff and artifact checks pass.

Original profiles, checkpoints, failures, results and source/bundle hashes remain
under `.tmp/browser-journal-profile-01`. These are single local observations with
compact synthetic history, zero IC/provider requests and paid cycles. Page heap
excludes native storage/browser RSS. Power loss, eviction, profile rollback, large
real envelopes, million-row journals and service-canister restoration remain
unqualified. That journal probe performs no Rust build or full CI.

The subsequent matching released-tools rehearsal rebuilds/freeze-selects the
0.14.6 debug CLI, release standalone Wasm, test harness and paired browser bundles.
Production and selected harness sources match the released commit before/after
the run. PocketIC independently confirms compiled 0.14.6 and rejects a carrier
bound to the wrong service. Complete GLB/WebP uses eleven PUTs/nine GETs; PNG/JPEG
lost-final-reply/control-interruption recovery uses seven PUTs/ten GETs, with zero
additional PUT on recovery. All four original roots/leaves/whole bytes/cache
headers and recovered maps agree. Overlapping reference release preserves
8,389,774/4,318,116 physical/liability bytes respectively. The original uncertain
gateway claim survives restart; logical release still refuses tenant downloads.

The [new record](../evidence/caffeine-probes/local/2026-10-04-released-media-v0146-01/summary.json)
retains exact commands, source archive, artifacts, hashes, requests/results and
original profiles under `.tmp/released-media-v0146-01`. A preparation command first
used `version` instead of `--version`; that failure and the premature missing-runner
attempt are retained, corrected before any journey. Both actual journeys and
release readback pass. This uses repository-built matching tools, not another
clean-prefix/no-Git installation test. No full CI, version change, commit,
deployment, live provider request, paid cycle, sibling edit/message or cleanup
occurs. Consumer adoption and certified asset registration remain open.

The subsequent [restoration simplification](../evidence/upload-restoration-simplification.json)
removes a temporary heap set of every Caffeine blob root. The already validated
root/object owner, root-to-request index and permission key together establish
uniqueness; all three checks remain. Attestation validation reuses the decoded
admission time instead of rereading the permission. Tests reject duplicates within
a tenant and across tenants sharing a request ID, and reject pre-admission
observations without repairing memory. No wire/storage layout changes.

Scoped upload/root/store tests, strict core Clippy, Wasm build and actual PocketIC
restoration checks pass. The broad `restore` filter first selects five cases with
missing gateway/storage fixture paths; its failed log is retained and all five
pass individually after provisioning those fixtures. An initial test compile
mistake is also retained. Sources, commands, artifacts and logs are under
`.tmp/upload-restore-simplification-01`. This tests draft source still compiled as
0.14.6, separately from the earlier frozen released-media run. Restoration still
validates all records synchronously; no populated-canister instruction/heap or
million-object measurement is claimed. The acceptance plan now reflects retained
trials and current recovery rather than obsolete 0.5/0.15/Unreleased assumptions.
Consumer adoption/certified registration and provider retirement remain open.

## 0.14.8 resource measurements and simplifications

[0.14.8](../../CHANGELOG.md) is now dated and Cargo/the receipt record that version.
The observations below retain their original compiled versions and source
identities; this does not establish registry publication or consumer adoption.

The [initial resource record](../evidence/caffeine-probes/README.md#bounded-service-and-browser-resource-profile--2026-10-04)
measures the sixteen-store assembly at 100/1,000/10,000 operations on normal
PocketIC application limits. Equal uncertain, live, logically released and
cancelled populations preserve selected permissions, references, receipts,
accounting and mutation fences through actual same-release upgrades. The
unpublished fixture accepts one bounded installation record; ordinary callers
keep their two-object envelope, without an old reader. Profiling is opt-in and
retains checkpoints before proceeding, outside default CI.

The [follow-up record](../evidence/caffeine-probes/README.md#transfer-memory-and-restoration-read-attribution--2026-10-04)
adds fixture-only per-memory read attribution. At 10,000 operations, repeated
enrollment lookup accounts for 90,020 tenant-memory reads. Keeping the decoded
activation generation in the existing per-tenant scratch totals reduces that to
29; instrumented initialization falls from 17,986,173,576 to 17,265,226,781
instructions (4.0%). Every permission still checks its generation; independent
tenants, older activations and a later corrupt future generation are covered.
Corruption refuses without repair. No persistent cache, record layout, authority
or freshness/resume claim changes. Allocated heap remains 1,376,256 bytes and
physical stable memory remains 72,417,280 bytes at that tier.

Wrapped reads account for about 13% of the instrumented baseline initialization;
they exclude tree traversal, Candid decoding and other work. The test wrapper
also differs from production RuntimeMemory's unsafe-read delegation. These
figures include observer overhead and are not standalone production costs.
Dependency source review finds no temporary array in the stable-memory default
unsafe-read path; no allocator or unsafe implementation change is justified.
Those original measurements did not cover larger manifests, many tenants or
occupied funding/read histories; the later bounded profiles below cover those
shapes separately and retain their own source and artifact identities.

One shared bounded observer now serves preparation and successful-transfer
fixtures. Fresh 1/8/32 MiB preparation still refuses before intent with zero
provider calls. Complete GLB/WebP and PNG/JPEG lost-reply journeys pass before
and after both simplifications, with eleven PUTs/nine GETs and seven PUTs/ten GETs
respectively; recovery makes no extra PUT. Exact original bytes, roots/leaves,
metadata, recovered maps, browser serving and reference liabilities agree.

The full GLB journey identifies a fixture-only numeric byte bridge and duplicate
decode. Removing it keeps native byte equality and browser whole-response SHA-256
plus one public-response decode. Sampled Node RSS in the reopened delivery
context falls from 2,293,227,520 to 252,026,880 bytes. Summed owned Chromium RSS
growth falls from 767,049,728 to 126,234,624 bytes. Actual upload code and its
observed worker backing-storage peak (about 50.9 MB) are unchanged. These sampled
peaks are lower bounds, summed RSS can double-count shared pages, and neither
local substitute delivery nor graceful reopen qualifies deployed provider
behavior, sustained load, eviction, power loss or million-object use.

Scoped upload-owner unit tests, strict core/fixture/harness Clippy, matching
Wasm/CLI/browser builds, both fresh media journeys, the restoration profile and
four PocketIC rollback/reopen/client regressions pass. Initial visibility/doc
lint failures from the first profile and the follow-up core function-length
Clippy failure remain retained with their corrections. Original profiles,
commands, artifacts, sources and hashes stay under `.tmp/service-restore-profile-01`,
`.tmp/browser-memory-profile-01` / `-02`, `.tmp/restore-read-profile-01` and
`.tmp/transfer-memory-profile-01`. No full CI, version mutation, commit, deployment,
live provider request, paid cycle, sibling edit/message or build cleanup occurs.
Consumer adoption, certified registration and provider retirement remain open.

The subsequent [funding fixture correction](../evidence/caffeine-probes/local/2026-10-04-storage-funding-upgrade-01/summary.json)
fixes one missed propagation caller: the shared Cashier test upgrade helper still
sent a bare principal. A retained actual PocketIC reproduction rejects that input
during post-upgrade decoding, after funding assertions pass. The helper now uses
the existing current installation encoder. All eleven funding transport cases,
including the five reported failures, pass; strict storage harness Clippy and
format/changelog checks pass. Production funding behavior and fixture Wasm are
unchanged. No full storage suite or CI rerun, release, live effect or cleanup.

## Retained 0.14.10 tooling work

Released 0.14.9 removes the mandatory Unreleased queue and section-order checks.
Preparation selects one draft in place, preserves history and refuses ambiguous
notes. The former 0.14.10 draft collected the tooling, dependency and
qualification work recorded below. Its earlier source batch at `83a2742` did
not change the package version or receipt; subsequent source was committed at
`c3143d6`, validated from clean input and released as 0.14.10. The following
records describe their original pre-release batches and retain their compiled
versions and dependency bindings.

The 0.14.10 review corrects release-bootstrap fixtures that still expected
dependency fetching as the first gate. The fixtures now observe snapshot
verification before fetching and compilation; snapshot/fetch failures preserve
release files and the retained build sentinel. Release-helper tests, snapshot
integrity, shell checks, formatting, changelog selection and diff checks pass.
The initial failed fixture remains at `target/release-tests.E9wLZs`; initial and
follow-up logs are under `.tmp/v01410-changelog-01`. This review performs no
full CI, version transaction, commit, publication or deployment.

The 0.14.10 batch adopted Shared Tooling 0.1.0 revision
`41e1fd0ba41460bd2127cbf98ac8a4b2b2020d3e` through
[its snapshot manifest](https://github.com/dragginzgame/ic-blob-storage/blob/c3143d6c803b5bc31c2a300f81f5b68deba64953/.shared-tooling.snapshot). The same eleven-file set
includes the canonical DRAGGINZGAME baseline, complete linked guides, integrity
verifiers and workspace-aware LOC script. AGENTS.md is a local service overlay;
build and validation do not depend on a sibling checkout. The
[adoption evidence](../evidence/shared-tooling-adoption.md) retains the initial
`e16c9c99bd800567189c8024eaf4242a5d1c9e29` adoption and this refresh separately.
Snapshot verification runs before dependency fetching in the complete validation
gate. The scoped adoption records do not claim full CI; the later current-graph
preflight below records that validation separately.

The offline LOC report covers all eleven Cargo members rather than two crates.
Make supplies the explicit root and offline metadata policy. Counts are path-based
Rust file/test-attribute observations, not production code size or executed test
counts. This tooling work changes no lockfile, package version or receipt;
dependency changes remain separate from snapshot validation.

For the initial adoption, snapshot verification, all-member coverage/totals,
deliberate isolated corruption refusal, ShellCheck/syntax, instruction links and
draft checks pass. Logs remain under `.tmp/shared-tooling-adoption-01`. At that
adoption the shared checkout acquired further uncommitted changes after selection;
the initial snapshot exported only the selected committed bytes. No Rust build,
full CI, source version change, live effect or cleanup was performed.

The 0.1.0 refresh exports committed blobs from a clean Shared Tooling checkout,
with the reviewed source URL changed from SSH to HTTPS for the same repository.
The first remote-spelling refusal remains retained. The Bash 3.2 verifier fix,
package-relative LOC classification and nested-member exclusion are adopted
without patching shared copies. All eleven snapshot files, scoped upstream
snapshot/LOC regressions, ShellCheck/syntax, member coverage/report totals,
instruction links, changelog selection and diff checks pass locally on Linux.
Package-relative paths reclassify 3,221 Rust lines in the protocol and PocketIC
harness packages; total LOC remains 115,370 and test attributes remain 1,004.
These are lexical/path counts, not production size or executed test counts.

Commands, source archive, package digests, before/after reports and logs remain
under `.tmp/shared-tooling-refresh-01`. Missing cloc/jq and Perl prerequisites
were provisioned there from checksum-pinned Ubuntu packages, without system or
sibling changes. The pre-existing powerfmt 0.2.0-to-0.2.1 lockfile change is
preserved exactly and is not qualified by these tooling checks. Cargo version
and receipt remain 0.14.9. No Rust build, full CI, release mutation, commit,
publication, provider effect or native macOS qualification occurs.

The new shared requirements cover exact cleanup symbol reporting and repository
description review. A public GitHub read observes an empty repository description;
the [adoption record](../evidence/shared-tooling-adoption.md#shared-tooling-010-refresh)
retains that observation and a proposed description. No remote metadata write or
issue submission is authorized by this continuation.

The adopted baseline uses GitHub issues as the sole work tracker. Superseded
local shared-feedback and consumer-integration queues are removed; old history
links bind their released source. Evidence remains with its owning probe/contract,
without new local issue IDs or triage tables. No issue submission or upstream
message is authorized by this adoption. Native macOS behavior and host-specific
prerequisites are not qualified by Linux checks; the shared requirement remains.

## Current restoration qualification

The [multi-tenant/multi-chunk record](../evidence/caffeine-probes/README.md#multi-tenant-and-multi-chunk-restoration--2026-10-05)
extends the existing opt-in sixteen-store PocketIC profile. All six workloads
pass on normal application limits: 100/1,000/10,000 one-tenant operations,
1,024 operations across 32 tenants, 32 operations with 16 MiB manifests across
four tenants, and four operations with 64 MiB manifests. Request IDs overlap
across tenants; roots and leaves remain distinct.

Actual same-release upgrades preserve global accounting and selected tenant-bound
permissions, exact chunk/header declarations, reference liveness and receipts.
Foreign callers refuse; restored mutations remain fenced without changing state.
Instrumented initialization is 17,328,500,885 instructions at 10,000 operations,
1,495,367,763 for the 32-tenant workload, 45,040,110 for 16 MiB manifests and
12,638,294 for 64 MiB manifests. Different populations are not controlled
comparisons. Allocated heap is 1,376,256 bytes in each workload; that is linear
memory allocation, not live heap or peak working memory.

The private fixture now accepts existing client-prepared manifests instead of
hashing bodies in the canister or returning duplicate requests. One current
installation record declares bounded object/tenant/byte ceilings; all callers
are updated and ordinary tests keep their two-object/two-tenant/ten-byte limits.
The first 100-object input exceeds the existing 4 KiB decoder limit and fails
before mutation; it remains retained. Smaller batches preserve every decoder
limit. The successful follow-up retains all 795 exact population packets before
dispatch (703–2,519 bytes), checkpoints, original/follow-up binaries and logs
under `.tmp/restoration-shapes-01`.

Strict scoped fixture/harness Clippy, matching builds, formatting and the ordinary
two-object rollback/reopen regression pass. The failed initial lints and sandbox
loopback attempt also remain recorded. This work changes no production sources,
dependency lock, release version or receipt; artifacts compile as 0.14.9 with the
retained fixture source patch and original released dependency graph. The
concurrent dependency update above is preserved and not qualified by this run.
There is no new cache, partial restore, relaxed fence or default CI workload.
This run is synthetic local completion evidence,
with zero live provider calls or paid cycles. It does not qualify occupied
funding/read histories, production restore costs, macOS, stale-backup activation,
freshness resume, future retention or million-object use. No further production
optimization is justified by this workload extension alone.

## Management-canister types upgrade

The [upgrade record](../evidence/caffeine-probes/README.md#management-canister-types-0110--2026-10-05)
qualifies scoped current-instance recovery on management types 0.11.0, ic-memory
0.25.5, ic-testkit 0.15.4 and PocketIC 16. The workspace owns the version; every
member inherits it. PocketIC's native 0.8.0 dependency remains upstream-owned,
while the core Wasm path selects 0.11.0. Only the authorized package changes
relative to the captured pre-upgrade lock; concurrent updates are preserved.

Upstream source comparison leaves the maintained `canister_info` types unchanged.
The new instruction total measures aggregate subnet work, including scheduler
and non-Wasm charges, with prior-round freshness. It cannot attribute a service
operation, prove continuity or replace provider accounting. The new ECDSA curve
has no current blob-storage consumer. Keep the existing bounded, authenticated
history proof without adding metrics polling, management calls or a new state
owner. No public API, persisted layout or production Rust source changes.

Three continuity unit tests, strict core/standalone-Wasm/harness Clippy, matching
Wasm/harness builds and five existing PocketIC recovery cases pass. These cover
operator denial, ordinary stop/start/repeated upgrades, expired history without
anchor rotation, management changes during the await, retained physical/billing
obligations and actual snapshot refusal. Frozen artifacts, source/dependency
bindings and logs remain under `.tmp/management-types-011-01`. This is local
evidence with no live provider call or paid cycles. It does not relabel the earlier
restoration profiles, qualify new scale/macOS behavior or replace full CI.
The package version, release receipt, Rust toolchain and declared MSRV are unchanged.

## 0.14.10 batch preflight

The [preflight record](../evidence/release-preflight-01410.json) binds base
`8e3704bfee6edbf68c30d467c2c552951521ebbc` plus the retained uncommitted source
patch. Dependency documentation now names the selected 0.11.0/0.25.5/0.15.4
graph and preserves earlier artifact identities. Release setup now correctly
verifies the snapshot before fetching dependencies. The existing transitive
powerfmt update to 0.2.1 is retained: its public API is preserved, optional
features are unselected and its 1.79.0 Rust floor remains below our 1.88.0 MSRV.

The pinned Linux PocketIC 16.0.0 archive and executable match both original
recorded hashes; the version check passes before use. Authorized full `make ci`
passes snapshot/fetch, shell/release helpers, formatting, native compilation,
strict Clippy, retained probe integrity, docs, native/PocketIC tests, Wasm
checking and package verification. Test summaries report 962 passed, zero
failed and 26 intentionally ignored opt-in cases. Actual Rust 1.88.0 native
all-target/all-feature and workspace Wasm checks also pass on the same lock.

Source archive/patch, dependency inputs, upstream powerfmt comparison, logs,
test binaries, CLI, verifier, seven Wasm modules, PocketIC server and package
are frozen under `.tmp/v01410-preflight-01`; hashes bind their actual identities.
All compiled artifacts remain 0.14.9, and Cargo.toml/Cargo.lock/receipt match
task-entry hashes. These are current Linux gate/MSRV observations, not new
native macOS, opt-in browser/scale or deployed-provider qualification.

After this dirty-source preflight, the maintainer committed the batch as
`c3143d6`. Authorized `make bump-x VERSION=0.14.10` passed the full current gate
again from that exact clean source, then prepared only the four release files.
The maintainer subsequently created annotated `v0.14.10`; its release receipt
binds the clean-source validation separately from this earlier preflight.
Preserved gate binaries still compiled as 0.14.9. The new occupied-history
profile below rebuilds and freezes its own 0.14.10 fixtures; neither release
receipt nor tag relabels earlier evidence.

## Occupied funding/read restoration and snapshot integrity

The [intent](../evidence/caffeine-probes/local/2026-10-05-restoration-histories-01/intent.json)
precedes a focused ordinary-envelope PocketIC profile on compiled 0.14.10.
Source review confirms that funding restoration validates retained lifetime
intents in order and reconstructs their exact allocation; read restoration
validates only occupied sessions and rebuilds global/per-tenant counters.
Completed read rows are removed, leaving their monotonic high-water sequence.

Four actual same-release upgrades pass: empty journals, four terminal funding
intents after 64 completed reads, and three terminal intents followed by either
a prepared or uncertain intent with one interrupted read. Exact funding
requests/phases and whole-service accounting survive. Both interrupted cases
retain sequence 65, one occupied slot and its 2,048-byte reservation; capacity
and post-restore mutations refuse without another chunk request or lost state.
Instrumented initialization measures 7,056,415 / 7,996,320 / 8,148,753 /
8,148,232 instructions respectively. Allocated heap stays 1,376,256 bytes and
physical stable memory stays 18,939,904 bytes in all four cases.

The [summary](../evidence/caffeine-probes/local/2026-10-05-restoration-histories-01/summary.json)
binds source, dependencies, frozen test/Wasm/server bytes, exact measured
requests/results, two intentional callback traps and logs under
`.tmp/restoration-histories-01`. The existing combined-owner interrupted-read
upgrade regression, focused builds and strict harness Clippy also pass.
The profile is opt-in; ordinary fixture limits and production code are unchanged.
Four lifetime funding rows and one occupied read do not justify a production
optimization or qualify large occupied histories. No full CI, macOS or deployed
provider observation is claimed; live provider requests and paid cycles are zero.

The post-release documentation merge added banners to seven vendored guides,
causing the offline snapshot guard to fail. Original bytes/diff and failure are
retained. The canonical helper refreshed the existing eleven-file snapshot from
the same clean Shared Tooling revision `41e1fd0`, restoring only the seven guides'
pinned bytes. Source, manifest, file set, tools and repository-owned banners are
unchanged; the offline integrity guard now passes. Keep branding in locally
owned documentation rather than changing declared shared files.

## Released 0.14.11 larger occupied histories and selected dependencies

The [intent](../evidence/caffeine-probes/local/2026-10-05-restoration-scale-01/intent.json)
precedes five larger normal-application PocketIC workloads on selected ic-memory
0.25.9/ic-testkit 0.15.8, still compiled as 0.14.10. The single current private
installation record declares funding, global read and per-tenant read ceilings;
all producers use it. Ordinary tests keep four funding intents and one read.
Frozen earlier fixtures retain their original contracts; this batch uses only
fresh current-contract installations, then upgrades the identical module/input.

At 100/1,000/10,000 lifetime funding intents, instrumented initialization takes
31,018,948 / 225,225,252 / 2,157,705,728 instructions. Four intents plus 32 reads
take 13,417,273; 1,000 intents plus 1,024 reads across 32 tenants take 482,113,535.
Allocated heap stays 1,376,256 bytes. Physical stable memory reaches 35,717,120
bytes at 10,000 funding rows and 23,134,208 bytes in the combined 32-tenant case.

Every actual same-release upgrade preserves whole-service accounting, selected
exact funding requests/phases and every occupied read identity/target through
bounded operator pages. The largest read case retains 2,097,152 reserved bytes.
Tenant capacity refuses while global room remains; global capacity later refuses.
Wrong operators, stale population counters, excessive inspection pages and
restored mutations refuse. A later invalid item traps and rolls back all earlier
funding/read writes in its batch. Population uses maintained journal operations
and read-admission workflow, sends no calls and exports no completion tickets.
These are synthetic outcomes and deliberately undispatched read intents, not
concurrent deployed traffic or provider credit evidence.

The [summary](../evidence/caffeine-probes/local/2026-10-05-restoration-scale-01/summary.json)
binds exact source/lock inputs, upstream source comparisons, frozen test binaries,
three Wasm modules, server, requests/results/checkpoints and logs under
`.tmp/restoration-scale-01`. Two initial Clippy failures are retained and fixed.
The ordinary-profile filter first also selected the new scale test; the ordinary
case completed, then the extra test stopped before effects for its absent report
path. The [follow-up intent](../evidence/caffeine-probes/local/2026-10-05-restoration-scale-01/followup-intent.json)
uses the exact selector, same frozen artifacts and a fresh report directory.
Both attempts remain retained.

Thirty scoped core funding/read tests, seven existing standalone continuity,
history and snapshot cases, the combined-owner regression and both profiles
pass. Strict fixture/harness Clippy and scoped actual Rust 1.88 native/Wasm
checks pass. Upstream metadata checks move to constructor/decode boundaries;
this repository uses none of the removed APIs. Artifact acquisition changes have
no current consumer here; explicit builds use `--release --lib`.

No production restore optimization is selected from this bounded evidence.
Synchronous validation, durable layouts, mutation fences and ownership remain.
This is Linux local/substitute qualification with zero live provider requests
and paid cycles, not full CI, native macOS, production restoration, stale-backup
activation or million-object evidence. Cargo, release receipt, toolchain and
maintainer-selected lock are unchanged by the batch; no commit or publication.

## Retained 0.14.12 tooling batch

The reviewed Shared Tooling revision is `f52c0e2476aee094359ed21de91c468540d3969f`,
exported from a clean isolated copy rather than the sibling's unrelated dirty
work. The snapshot contains 22 committed files, including the common release
runner, rules, formatting hook and exact tool selections. No shared copy is
patched; the local overlay retains service policy and command authority.

Standard patch/minor/major releases have one Git owner. The consumer adapter
keeps the four release metadata files and source-bound receipt, restores a failed
metadata transaction and changes only workspace package versions in Cargo.lock.
Rerunning the same release target reconciles exact saved intent; remote readback
prevents repeating a completed uncertain push. Push explicitly disables implicit
tag following and selects only the saved branch/tag. Publishing and cleanup
remain separate. Agents never invoke commit-producing release or resume targets.

`make fmt` and `make fmt-check` use pinned cargo-sort 2.1.4 and rustfmt across
all twelve manifests. The repository-local hook is activated in this clone;
other clones run `make install-hooks` explicitly. The actual consumer hook test
preserves its lock, unrelated edits and untracked files, rejects partial staging
and leaves the index/working tree intact when formatting fails. The independent
[tooling workflow](../../.github/workflows/tooling.yml) declares Linux and both
macOS 15 architectures. Native results were pending at that batch's handoff;
the subsequent release run is recorded in the 0.14.13 section below.

The [adoption evidence](../evidence/shared-tooling-adoption.md#01412-release-and-formatting-adoption)
retains original failures, source/manifest inputs and scoped checks under
`.tmp/shared-tooling-v01412-01`. Source/dependency/feature/target comparisons
preserve all eleven workspace members; a real isolated version transaction
preserves every external lock byte. Consumer and upstream hook checks, common
runner and local adapter fixtures, formatting, shell checks and workflow lint
pass locally. No Rust build, full CI, package bump, commit, tag, push, publication,
provider effect, sibling edit or consumer artifact cleanup occurs.

## Tooling batch originally prepared for 0.14.13

These records precede the dependency hard cut below. Their original source,
dependency and artifact identities remain unchanged; the pending tooling work
now joins 0.15.0 rather than a separate 0.14.13 release.

The next patch is compatible internal tooling: no published Rust API, blob CLI,
service DTO, wire or persisted layout change. `make evidence-check` checks the
retained local/deployed manifests without Rust builds or provider calls. A small
consumer-owned parser delegates hashing to the unchanged reviewed checksum
helper. Both GNU and forced Perl paths verify all 288 retained files locally;
corruption, missing files, malformed/empty records and unavailable hashing refuse.
The installation recipe uses portable Perl hashing. No original evidence,
manifest, dependency selection or release receipt changes.

The pushed [0.14.12 native tooling run](https://github.com/dragginzgame/ic-blob-storage/actions/runs/37368942870)
finishes with both macOS jobs failing `bootstrap-fetch`; Linux is cancelled.
Both macOS hosts pass snapshot integrity, formatting and the common-runner suite
before that failure. Logs show the injected Cargo fetch refusal continuing to
the cache write. Consumer substitutes now exit explicitly on those failures,
and the validation loop explicitly stops when a prerequisite fails. A new
cached-fetch refusal verifies no Cargo check runs and old cache bytes survive.
Twenty-one adapter cases pass on Linux after the fix; corrected native execution
remains pending. The original runner fixtures are unavailable after teardown;
downloaded native console logs remain bound to the original release/run.

The host workflow now includes evidence checks and retains failed release/hook/
checksum fixtures as artifacts for 30 days, with the host and attempt in each
name. [The batch summary](../evidence/checksum-portability-v01413.json) binds entry
source, unchanged evidence inputs, scoped logs and the tested patch under
`.tmp/checksum-portability-v01413-01`. The first added assertion's ShellCheck
refusal remains retained; the explicit conditional and final shell check pass.
Formatting, actionlint and shared snapshot checks pass locally. No full CI,
Rust compilation, version bump, commit/tag/push, publication, provider probe,
sibling edit or preexisting artifact cleanup occurs. No functions, methods or
types are removed.

The [Bash 3.2 follow-up](../evidence/bash32-v01413.json) builds GNU Bash 3.2.57
from signature-verified source inside the repository's temporary tooling area.
It reproduces a failed conditional guard continuing into simulated registry
publication. Release identity/tag guards and fixture assertions now refuse
explicitly. Twenty-three adapter cases pass on Bash 3.2 and Bash 5; the common
runner and all 288 evidence hashes pass on Bash 3.2. These are Linux shell
observations, not native macOS qualification. Prior logs and failed fixtures remain.

The same run finds a separate pinned Shared Tooling hook defect: on Bash 3.2,
formatter failure still refreshes selected files. The one-line proposal in
`.tmp/bash32-v01413-01/shared-hook-fix.patch` passes the full consumer hook fixture
in an isolated candidate. After explicit authorization, the same fix is applied
in Shared Tooling, with its existing unrelated edits and index preserved. The
required upstream portable suite passes on Bash 3.2, and ShellCheck passes. Both
prerequisite failures remain in the follow-up evidence. The maintainer then
committed the fix in Shared Tooling `9437bab201bb6071da0bdc4de0336daf553113f5`.
The [subsequent adoption](../evidence/shared-tooling-recovery-v01413.json) first
refreshes 23 files from an isolated clean checkout of that exact revision, leaving
new unrelated sibling edits untouched. The requested latest refresh then selects
`cb86188c5956866564de4fb6ec6be67b27981ab9` with 24 files and its validation logger.
The canonical helper protects local destination changes; the two prior generated
shared guide updates are preserved and reconciled before refreshing. The fixed
hook and complete linked agent maintenance rules are adopted. Native consumer
execution remains pending.
The snapshot gate still precedes all host checks; after that gate, a failed check
does not hide the remaining focused results. Shell/workflow/format/snapshot checks
pass. No Rust build, full CI, package bump, real release, commit/push, provider
effect or preexisting artifact cleanup occurs. Temporary Bash/parser tool builds
and their original preparation failures are retained separately.

Late release adapters now export `RELEASE_COMMIT`, verify the original committed
Cargo/notes/receipt bytes and bind the selected sole parent and annotated tag.
The canonical runner reconciles an older committed release before fresh gates
for newer fixes or a different increment; explicit resume finishes only its
selected release. Package publication still checks the clean current HEAD.
The real release gate now runs the same complete CI target through the shared
logger, retaining unique failed logs across retries and preserving temporary logs
if retention fails. The private fixtures keep real Git/Cargo release effects
substituted; a real Git index fixture checks unrelated staged bytes hidden by a
restored working file without creating a commit.
This is compatible maintainer tooling in the existing 0.14.13 draft, with no
library API, stored format, package version or dependency selection change.
Matching [upstream native CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37428740374)
passes on Linux and both macOS architectures; it does not qualify this consumer's
new callbacks. The later [cb86188 CI run](https://github.com/dragginzgame/shared-tooling/actions/runs/37431805988)
has a macOS ARM snapshot-fixture failure: a logical temporary path misses its
fake source identity after physical-path normalization. A symlinked Linux temp
directory reproduces the same refusal. This belongs to the upstream fixture;
the consumer's guarded refresh succeeds. Latest upstream native and new consumer
native acceptance are not established. Local adapter checks pass on Bash 3.2
and Bash 5, including real index inventory and actual failed-gate log retention
across retry; hook, all 288 evidence hashes, formatting and shell checks pass.
The mock tag-path and real-index log-directory setup failures remain retained.
No GitHub issue writes, real release, commit, tag, push or paid
effect is performed.

## Current 0.15.0 dependency hard cut and validation

The maintainer selected ic-memory 0.27.0 and ic-testkit 0.18.3, then explicitly
chose to include the upgrade in the next minor release. The undated changelog
is now 0.15.0 and carries the entire earlier tooling draft; 0.14.13 is not a
separate release. Cargo package versions and docs/release.json remain 0.14.12.
The selected lock also updates want and zerocopy; no dependency is reselected
during these checks.

The [new record](../evidence/release-preflight-0150.json) binds the selected graph,
entry source patch, raw logs and artifact hashes under `.tmp/v01413-final-01`.
The complete local `make ci` gate passes on Rust 1.99.0/Linux, including release
and actual hook fixtures, formatting, all-target compilation, strict Clippy,
retained evidence, Rustdoc, native/PocketIC tests, workspace Wasm and package
verification. Scoped actual Rust 1.88 checks pass for core/standalone/harness
native targets and core/standalone Wasm. No caller adjustment is needed here.
Gate inputs still have the earlier draft heading; subsequent documentation
records the approved 0.15.0 decision. Compiled tools and Wasm still identify
0.14.12. These observations do not relabel earlier artifacts or qualify macOS.

ic-memory 0.27 removes allocation history/timestamp APIs and replaces the
durable ledger layout visible through this library's public memory re-export.
That makes the complete batch breaking before 1.0. Hosts must update affected
callers and fixtures, and retained installations require obligation disposition
before reinstall under [the persisted contract](../service-contract.md#current-persisted-boundaries).
Do not clear allocation ID 0 or add a compatibility reader. Same-release fences,
backup and interruption recovery remain required; frozen live trials retain
their original tools and all provider/billing obligations.

The authorized Shared Tooling fix normalizes the snapshot fixture's temporary
directory before assigning fake Git identities. The alias failure reproduces
before the fix and the complete upstream portable suite passes afterward on
actual Bash 3.2 through the aliased directory; ShellCheck passes. Missing cloc,
jq shared libraries and cloc Perl prerequisites initially prevent the full
suite; all failed logs survive and the prepared-tool retry passes. The upstream
fix and draft note remain uncommitted in Shared Tooling. Commit creation is
maintainer-owned, so consumer adoption of that fix's revision awaits its commit.
The current 24-file pin remains cb86188. Its original native CI has completed
with both macOS jobs failing the fixture; the corrected native upstream and
new consumer jobs still need committed/pushed inputs. No full CI is rerun merely
for a documentation change. No real release, version bump, commit, tag, push,
publication, provider effect, GitHub write or function/method/type removal occurs.

## Current 0.15.0 refusal, media and operator batch

The [new intent and results](../evidence/caffeine-probes/README.md#selected-hard-cut-graph-and-media-rehearsal--2026-10-06)
complete the requested repository-owned hard-cut checks on the selected graph.
An actual hash-pinned preflight image compiled 0.14.9/ic-memory 0.25.5 refuses
upgrade to the current 0.27 image at memory bootstrap. Every stable byte and the
old configuration, released object, pending manifest and conservative holds
survive. `make test-hard-cut` is opt-in and requires the retained historical Wasm;
default CI cannot replace that input with a generated approximation.

Matching current tools still compile as 0.14.12. The installation carrier checks
that release independently and rejects the wrong actual service. Original
GLB/WebP and PNG/JPEG bytes complete local serial publication, verified download,
browser decoding/CORS/CSP and overlapping-reference cleanup. Lost-final-reply and
control recovery add no PUT. The last reference's release refuses service
downloads while stored bytes/liabilities and public substitute delivery persist.
Installed Brave 154.1.96.61 is recorded explicitly; HTTP cache reuse exposed and
corrected two fixed GET-count assertions without weakening byte/origin checks.

The [summary](../evidence/caffeine-probes/local/2026-10-06-hard-cut-media-01/summary.json)
retains every preparation, path, accounting, cache and Clippy failure, successive
artifact generations, signed packets, stable bytes and final results. Fresh
substitute retries exceeded the initial two-owner plan; a follow-up records the
actual four owners and the final bounds. Two core capacity cases, six standalone
reference cases, same-release obligation recovery, strict harness Clippy and
actual Rust 1.88 harness compilation pass. No full CI is repeated for this batch.

The [retirement runbook](../retiring-installations.md) supplies exact inventory,
disposition and stop criteria before reset. The
[overlap/capacity recipe](../operator-guide.md#overlapping-application-releases-and-lifetime-capacity)
uses existing references/receipts and the consumer's existing publication owner;
it adds no journal, release API, compatibility reader or downstream dependency.
[#6](https://github.com/dragginzgame/ic-blob-storage/issues/6) remains open for
consumer acceptance and actual provider deletion/final billing. No live owner,
provider effect, version bump, release, commit, push or GitHub write occurs.
Public function/method/type removals: none in this batch. Shared Tooling adoption
still awaits the maintainer-owned upstream fix commit described above.

## Repeated provider funding review

The [source/native review](../evidence/caffeine-probes/README.md#repeated-funding-credit-gap-review--2026-10-06)
confirmed the pre-implementation guarded funding credit-settlement gap.
After the first positive accepted attachment, `CreditRequired` and maintained
accepted totals block the next guarded top-up with `JournalUncredited`.
Host activity `Clear`, a success/balance report or low-level reservation cannot
safely remove the blocker. The reviewed journal mutation and standalone funding
surface contained no authoritative credit transition. Seven focused existing native
regressions pass; this review changes no production code.

The current lock selects ic-memory 0.27.1; these checks keep that selection and
compile 0.14.12. Earlier 0.27.0 full-gate/media records remain historical and
are not validation of this later graph. Toko locally selects blob 0.14.9 through
Canic's inspection-only funding wrapper; the tagged library has the same gap.
Its automatic component-cycle replenishment is distinct from provider-account
funding. No deployed Toko provider timer or live funding failure was observed.

The review called for an authenticated, exactly bound durable credit-reconciliation
contract and qualified provider evidence, followed by consumer-owned repeated
dispatch integration. Keep transport history, lifetime accounting, uncertainty
and restoration fences. The upload verifier remains separate. Wiring alone
cannot complete this provider funding path; no journal reset or guard bypass is
an acceptable substitute.

## Host-internal funding credit implementation

The maintainer selected the [host-internal confirmation boundary](../funding-credit.md).
`workflow::funding::credit::confirm` and the journal's `record_credit` authenticate
the exact original operation and commit an immutable evidence fingerprint with
confirmed totals. Wrong amounts, conflicting or reused receipts refuse. Replay
is idempotent; lifetime acceptance stays charged, with uncredited acceptance
tracked separately. Existing allocation limits, uncertainty and fences remain.
Standalone exposes inspection only; consumer hosts own evidence acquisition.

The [new evidence](../evidence/caffeine-probes/README.md#host-internal-funding-credit-confirmation--2026-10-06)
retains source/artifact hashes and every failed preparation. Four new actual local
IC cases pass for repeated dispatch, atomic write-trap rollback, refusal and fenced
restoration. All 26 existing storage funding cases and five standalone funding
cases pass across the initial run and focused setup retries. Native funding,
billing policy, installation and all 98 CLI cases pass. Strict relevant Clippy,
Rust 1.88 native/Wasm checks, rustdoc, formatting and generated Candid equality pass.
The pinned pre-cut image still refuses upgrade with all stable bytes/obligations
preserved. These are local/synthetic observations, not deployed provider credit.

Pending 0.15.0 adds mandatory receipt/accounting fields, `uncredited_accepted`
and `CreditConfirmed`; update hosts, codecs and matching tools together. The
installation format changes to refuse prior layouts. Compilation remains 0.14.12
on the unchanged maintainer-selected ic-memory 0.27.1 graph. No full CI, release,
commit, push, publication, live effect, sibling edit or symbol removal occurs.

## Shared Tooling and remaining hard-cut review

The 2026-10-06 read-only recheck finds upstream main at
[`a7efade`](https://github.com/dragginzgame/shared-tooling/commit/a7efade1a68e43f148252a1a73908a46c4cbe9e9),
with an undated 0.1.5 draft and no remote `v0.1.5` tag. This committed source
contains the previously awaited temporary-path fix, nested validation isolation
and dependency-pinning rules/checker. Our 24-file cb86188 snapshot still verifies.
The sibling also has uncommitted audit-method/baseline changes; those are excluded
from the committed review. Its
[matching CI run](https://github.com/dragginzgame/shared-tooling/actions/runs/37443591873)
has successful Linux portable and lint/security jobs; both macOS jobs remain
in progress at inspection. This is not complete native acceptance.

The committed checker was exported by exact Git archive and run read-only against
this repository with hash-matching yq 4.47.2. It flags the four exact constraints
for `candid_parser`, `ic-agent`, `sha2` and `thiserror`; adoption needs compatible
ranges or explicitly reviewed compatibility exceptions, not a dependency upgrade
or blanket exemption. Raw GitHub/checker evidence is in `.tmp/v0150-review-02`.
Snapshot adoption also needs the new policy/checker files, gate and native-host
parser setup. This inspection changes no shared snapshot, dependency or code.

Before freezing 0.15.0, measure receipt-populated confirmation/restoration and
qualify the host receipt acquisition path in its consumer-owned integration.
The continuing-funding contract needs a deliberate operational choice: credit
confirmation does not replenish the finite allocation or lifetime intent slots.
If a consumer requires budget renewal, define its bounded authority and durable
accounting before changing the layout; do not reset history or refund spent cycles.
No additional missing reconciliation API or required hard cut is confirmed by
this review. Existing retirement/reference contracts still require actual
consumer and provider acceptance under
[#6](https://github.com/dragginzgame/ic-blob-storage/issues/6).

## Indexed receipts, bounded grants and tooling adoption

The [new retained record](../evidence/caffeine-probes/README.md#indexed-funding-receipts-and-bounded-grants--2026-10-06)
completes the authorized in-repository batch. The original scan-based credit
implementation measured 291,652,376 instructions at 1,000 intents and exhausted
the 180-second population budget before reaching 10,000. The current index uses
the existing accounting memory and one journal owner; confirmation now measures
3,808,692 / 4,840,460 / 4,949,504 instructions at 100 / 1,000 / 10,000 intents.
Exact replay, refusal, corrupt-index rejection and actual fenced restoration retain
receipt identity and spent accounting. Every failed/incomplete attempt remains.

Synchronous restoration still validates the entire history and index: the current
10,000-intent tier costs 4,763,717,162 instructions, with 1,376,256 measured heap
bytes and 47,251,456 physical stable bytes. This is local fixture evidence with
synthetic receipts and observer overhead, not production sizing or a Toko benchmark.
The improvement compares two unreleased credit implementations; reviewed Toko
still selects blob 0.14.9 and has no qualified provider-credit acquisition flow.

The maintainer selected [bounded host-authorized increases](../funding-credit.md#bounded-host-authorized-allocation-increases).
Every installation supplies a mandatory cumulative `renewal_ceiling`; setting it
equal to initial allocation disables grants. One immutable grant per latest fully
credited intent is bounded by its accepted amount and the installed ceiling.
Exact replay remains unchanged after later intents. Grants preserve lifetime
spend, receipts, refunds, operation identity and fixed intent slots; real cycles
and dispatch authority remain independent. Reopening reconstructs grants in order.
No public grant endpoint, new owner or extra memory grant is introduced.

The frozen installation format is now
`ic-blob-storage/installation:platform-anchor-funding-credit-index-renewal`.
Configuration, cumulative/ceiling status and grant outcome fields are mandatory;
generated Candid, native/browser fixtures and offline installation proposals agree.
Update consumers together and retire old obligations before reinstalling.
The [consumer acceptance recipe](../funding-consumer-qualification.md) records the
read-only Toko/Canic review; provider success/balances remain insufficient credit
evidence. That integration remains with its consumer owner.

Shared Tooling `a7efade1a68e43f148252a1a73908a46c4cbe9e9` is adopted through the
canonical reviewed export with 28 exact files, excluding sibling audit drafts.
The snapshot and new offline pinning checker pass. Four exact registry selectors
are compatible ranges, with the selected Cargo.lock unchanged from batch entry.
The owning gate and native-host parser setup are propagated. Upstream CI run
37443591873 passed Linux, both macOS 15 architectures and lint/security; the new
dirty consumer workflow has not run remotely. Local Bash 3.2/tooling fixtures pass.

Scoped funding/configuration/installation/policy/allocation checks, all 100 CLI
tests, all 32 storage funding and five standalone funding cases, generated Candid
equality and the actual trial-template installation pass. Two new local IC cases
cover bounded grants and rollback after complete synchronous credit/index writes.
The pinned old image refuses upgrade with exact stable bytes and obligations
preserved. Strict relevant and harness Clippy, Rust 1.88 native/Wasm checks,
rustdoc and browser-configuration encoding checks pass. All original setup,
fixture/schema and lint failures are retained with their scoped corrections.
No package version change, full CI, release, commit, push, publication, paid/live
provider effect, sibling mutation or named symbol removal occurs.

## Local host tooling and artifact adoption — 2026-10-06

The [retained record](../evidence/tooling-host-0151.json) adopts Shared Tooling
[`a37771f`](https://github.com/dragginzgame/shared-tooling/commit/a37771f1b6b5fc9a88ed6ab3b705bdda35cd8fa3)
through the canonical export: 43 exact files, including the common audit methods,
provenance guidance, local installers and verification helpers. The matching
[upstream run](https://github.com/dragginzgame/shared-tooling/actions/runs/37450707625)
passes. Its commit subject and undated notes mention 0.1.6; this record does not
assert a published or tagged Shared Tooling 0.1.6 release. Audit adoption adds no
automatic product audit or broad gate. Earlier sections retain their original
pre-release pending versions and compiled-source observations.

`make install-tools` explicitly provisions jq/yq and the common IC executables
under ignored `.tools/`; `make tools-check` verifies them offline. Make and the
Linux/macOS workflow select those local paths, with one reviewed IC pin matrix.
The complete gate checks the snapshot, local tools and declarations before
locked cache preparation; failed prerequisites stop before compilation or
metadata mutation. ShellCheck and cargo-sort remain separate setup prerequisites.
The new default PocketIC 16.0.0 executable matches the earlier retained binary
hash. Prior local tools and evidence remain untouched.

Published ic-host-tools 0.1.11 owns bounded CLI artifact reads and raw SHA-256
formatting. Native inputs still follow selected links and refuse special/empty
files; Unix probe records reject final-component links at open. Existing error
codes and persisted hash formats remain. This is not path confinement, immutable
content, Caffeine verification or a new provider/recovery owner. The core and
Wasm graph exclude the dependency. The lock adds four identities without
reselecting existing versions; the Rust minimum stays 1.88.0.

All 106 CLI cases and two standalone installation/Candid cases pass on the final
dependency, including actual compiled 0.15.0 installation and wrong-service
refusal. Strict CLI Clippy, scoped Rust 1.88 native/Wasm checks, retained probe
verification and all 301 local/deployed checksum bindings pass. Shared installer
and verification refusal fixtures pass on Bash 5 and Linux-built Bash 3.2;
release adapters use isolated effects. The initial missing raw-hash import build
failure is retained alongside its corrected runs. The new Cargo test wrapper
refuses zero passing tests and preserves Cargo/logging failures; the release
adapter delegates exact annotated-tag verification to its shared owner.

Pending 0.15.1 is compatible host/developer tooling, with no service DTO, public
core API or durable layout change and no canister instruction-count claim.
The changed consumer workflow still needs its native macOS 15 ARM/Intel and
Linux remote execution. No full CI, package version change, commit, push,
publication, deployment, paid/live provider effect, sibling edit or named symbol
removal occurs. The latest description readback remains empty; existing
[#8](https://github.com/dragginzgame/ic-blob-storage/issues/8) owns that correction.

## Release prerequisites and full-gate follow-up — 2026-10-06

The [follow-up record](../evidence/release-preflight-0151.json) adds
`make release-tools-check` to release preflight. It checks the existing pinned
formatter owner and the selected ShellCheck executable before full validation,
with explicit setup commands on refusal. Missing/broken ShellCheck and
missing/wrong cargo-sort leave release files, cache and release intent untouched;
corrected retries accept an explicit ShellCheck path containing spaces. No tool
installation or compilation is hidden in preflight.

Native CI now sets `TMPDIR` to `runner.temp`, matching the failure uploader's
`nonempty-cargo-test.*` pattern. Its Linux/macOS bootstrap prepares ShellCheck,
and the focused tooling loop exercises the same prerequisite target. Full native
host qualification of this changed workflow still needs its committed GitHub run.
The released 0.15.0 workflow independently passes all three hosts at
[run 37453740999](https://github.com/dragginzgame/ic-blob-storage/actions/runs/37453740999);
that earlier workflow does not qualify these dirty changes.

The explicitly authorized Linux `make ci` gate passes: all 627 native and 361
PocketIC cases, strict workspace Clippy, rustdoc, retained probes/checksums,
formatting, release/hook fixtures, workspace Wasm checking and package verification.
Sources remain compiled as 0.15.0 on the selected lock. Opt-in browser, hard-cut
and scale cases remain outside that gate and retain their earlier evidence.
A Bash 3.2 follow-up initially found the new ShellCheck test double continuing
after a failed conditional; the failure and fixture are retained. Its explicit
failure return is corrected and checked under the older shell.

The compatible draft remains 0.15.1. The original host-tooling record above is
historical and unchanged; the new record binds this follow-up separately.
No release command, real commit/tag/push, registry publication, deployment,
paid/live provider effect, sibling mutation or named symbol removal occurs.

## Shared Tooling verification adoption — 2026-10-06

The [adoption evidence](../evidence/shared-tooling-adoption.md#0152-verification-adoption--2026-10-06)
and [bound record](../evidence/shared-tooling-adoption-0152.json) cover the canonical
48-file export at `47cd2cc`, the local documentation and release-command targets,
portable digest generation and IC receipt traversal refusal. Local tests cover
GNU/Perl hashing, installer preservation, release routing and the consumer's
unchanged metadata/recovery owner under Bash 5 and Linux-built Bash 3.2. Shell
and workflow lint, snapshot/tool checks and retained evidence checks pass.

The full upstream run has Linux RustSec and macOS ARM version-file adapter
failures, retained separately. Those owners are not adopted. No complete upstream
or native macOS acceptance is claimed; this dirty consumer workflow awaits its
committed native run. Released 0.15.1's owning Linux CI job passed; both macOS
hosts later failed the native download fixture. That run tests the released
snapshot, not this adoption. This adoption batch retained the released Cargo
graph, IC pins and active tools. Pending 0.15.2 is compatible, with no service,
public core API or durable-format change. In this earlier batch no full gate,
Rust build, release, publication, paid/live effect, sibling edit or named symbol
removal occurred.

## Native download and selected dependency qualification — 2026-10-06

The [intent](../evidence/caffeine-probes/local/2026-10-06-native-download-0152-01/intent.json)
and [bound record](../evidence/caffeine-probes/local/2026-10-06-native-download-0152-01/summary.json)
retain the actual selected graph and fresh matching Linux artifacts under
`.tmp/native-dependencies-0152-01`. The local socket substitute models Darwin's
inherited nonblocking accept: a 16 MiB write stops at 2,634,240 bytes with
`WouldBlock`; explicit blocking delivers all bytes. Apple kernel source and
Rust's Darwin accept path support the inferred cause of the two hosted failures.
The fixture now clears nonblocking mode before timed IO and checks the full
write on successful verification. Production downloader behavior is unchanged.

All 106 CLI cases, two actual installation cases, two standalone funding cases,
one foreign/missing-memory restore refusal and 23 Caffeine cases pass. Relevant
all-target/all-feature Clippy, current native/Wasm compilation and Rust 1.88
native/Wasm checks pass. Explicit offline cache preparation succeeds; validation
preserves the selected lock. ic-memory 0.28.2 runtime source matches 0.28.0;
the used ic-testkit 0.19.1 APIs need no caller changes. Host tools 0.1.14 remains
native-only. Compiled artifacts identify 0.15.1 for pending 0.15.2, and earlier
records keep their original graph and artifact identities.

Shared Tooling remote main still resolves to `47cd2cc`. The additional lockfile
rewrite and formatting-hook helpers and Bash 3.2 assertion fixes remain dirty
upstream work. Adoption for [#10](https://github.com/dragginzgame/ic-blob-storage/issues/10)
requires their reviewed committed revision; no dirty upstream bytes or local
copies of those helpers enter the snapshot. The earlier negative Bash 3.2
release-command fixture result remains in `.tmp/v0152-review-02`.
Native macOS confirmation awaits the new committed consumer run. Initial sandbox
loopback denials and the failed sandbox GitHub read remain recorded separately
from permitted local/read-only retries. No full CI, release, commit, push,
publication, paid/live request, sibling edit or named symbol removal occurs.

## Release admission and backpressure follow-up — 2026-10-06

The [bound record](../evidence/release-guards-0152.json) retains the source,
commands, matching native test artifacts and every attempt in
`.tmp/continuation-0152-02`. The old consumer adapter accepts expected version
output from a failed helper and proceeds to the formatter prerequisite check.
The fixed adapter requires successful working-tree, HEAD, parent, version and
receipt-source reads before comparing values. Seventeen failed-read cases refuse
without Cargo dispatch, metadata mutation or publication; existing release and
recovery fixtures also pass under Bash 5 and Linux-built Bash 3.2. All Git/Cargo
release effects in those fixtures are substitutes; no real commits are created.

The native backpressure server now clears inherited nonblocking state and bounds
both reads and writes. Its existing query/update cases still observe one service
operation for both 429 and 503 replies. Production transport is unchanged.
The initial ShellCheck warning, corrected candidate/dispatch assertions and two
evidence-inventory preparation failures remain recorded separately.

The selected ic-testkit 0.19.2 was already in Cargo.lock at task entry. Its cached
published runtime source matches 0.19.1. Fresh native host checks pass all 106 CLI
cases and two actual PocketIC installation cases; CLI/harness Clippy and Rust 1.88
native harness compilation pass. The task-entry lock and snapshot are preserved;
the earlier 0.19.1 evidence is not relabelled. Compiled identity remains 0.15.1
for pending 0.15.2. Native macOS confirmation and committed Shared Tooling helpers
for [#10](https://github.com/dragginzgame/ic-blob-storage/issues/10) remain pending.
The owning CI run is unchanged and failed at the released source; there are no
open owning PRs. No full CI, release, push, publication, paid/live request, sibling
edit or named symbol removal occurs.

## Shared-owner adoption and issue acceptance — 2026-10-06

The [new source-bound record](../evidence/shared-tooling-owners-0152.json) and
[adoption narrative](../evidence/shared-tooling-adoption.md#0152-shared-owners-and-issue-acceptance--2026-10-06)
retain the intermediate `9f8c7c7` and final `d957d1f` canonical exports separately.
The final snapshot includes shared lockfile rewriting, formatting-hook mechanics,
Cargo inheritance and metadata refusal fixtures. Release writes, receipts,
committed source selection and recovery remain local; the mutating-formatter
rollback case remains consumer-owned. Unused CI installer entry points and tag
deletion tooling are not added. Tool pins and active tool selections are unchanged.

Fresh focused checks pass on Linux with Bash 5 and Linux-built Bash 3.2. The current
direct ic-host-tools 0.2.0 selection was a concurrent Cargo edit and is preserved;
106 CLI cases, the FIFO subprocess, strict Clippy and Rust 1.88 compilation pass.
Eight core installation cases, two PocketIC ingress cases and 35 browser refusal/
control cases pass. Browser certificate/provider requests are zero. The first
PocketIC attempt is denied a localhost bind; its retained failure is distinct
from the permitted successful retry. The selected Wasm hash matches the earlier
bound artifact. Earlier 0.1.14 and 0.19.1 evidence is not relabelled.

The crates.io sparse index independently lists published 0.10.0 and 0.15.1;
the version-specific API still returns HTTP 403. This publication observation
does not establish service deployment or consumer adoption. The repository API
now confirms the prototype description. Owning GitHub issues hold the requested
acceptance comments and disposition; this handoff is not another issue queue.
Consumer publication, serving, retirement and billing evidence remain separate.

Final upstream CI passes Linux/lint and fails both macOS portable jobs after IC
installer success, before host fixture success. Logs do not identify the exact
assertion. The changed consumer workflow requires its own committed native
macOS run for [#10](https://github.com/dragginzgame/ic-blob-storage/issues/10) and
[#11](https://github.com/dragginzgame/ic-blob-storage/issues/11). No full gate,
version mutation, commit/tag/push/release/publication, deployment, paid/live
provider effect or sibling mutation occurs. Keep artifacts and original failures.

## Native installation and failure artifact follow-up — 2026-10-06

The [bound record](../evidence/native-installation-0152.json) retains the exact
runtime inputs, selected graph, native executable, standalone Wasm, harness test
binary and generated Candid/readback under `.tmp/installation-0152-03`.
Explicit offline cache preparation succeeds without changing the selected lock.
The actual native CLI produces the installation bytes; the local IC fixture
rejects the wrong actual service without replacing state, accepts the exact
correct carrier, reads back the complete configuration/project/verifier/uploader
and compiled 0.15.1 release, and denies controller-only authority. The declared
and exported Candid services agree. Both selected cases pass. This is local IC
installation qualification for pending 0.15.2, not provider or consumer deployment.

Native CI's failure uploader now includes `cargo-metadata-test.*` and
`local-lock-test.*` under the explicit runner temporary directory. A controlled
offline Cargo refusal makes each maintained fixture fail and retain its inputs
and diagnostics. The actual retained directories match the added upload paths;
workflow lint passes. This verifies retention and path selection, not an actual
GitHub upload from dirty source. Earlier records remain unchanged.

The GitHub recheck finds the existing owning issue discussions unchanged and no
open PRs. [#10](https://github.com/dragginzgame/ic-blob-storage/issues/10) and
[#11](https://github.com/dragginzgame/ic-blob-storage/issues/11) still require the
changed consumer's committed native macOS run; publication/serving/lifetime
acceptance remains with [#4](https://github.com/dragginzgame/ic-blob-storage/issues/4),
[#5](https://github.com/dragginzgame/ic-blob-storage/issues/5) and
[#6](https://github.com/dragginzgame/ic-blob-storage/issues/6). Shared Tooling main
still resolves to adopted `d957d1f`; owning CI still tests the released `c3e2714`
and has its previously recorded macOS download-fixture failures. No new remote
green result is attributed to these working changes. Pending 0.15.2 stays
compatible; no public core/service/durable contract or tool pin changes.
No full CI, release, commit/tag/push, registry publication, deployment, paid/live
provider effect, sibling mutation or named symbol removal occurs in this follow-up.

## Remaining product work

- Qualify host acquisition of uniquely attributed provider credit and complete
  account activity, then adopt the internal confirmation workflow in consumer-owned
  repeated funding integration. Current transport success/balances alone remain
  insufficient; upload verifier configuration stays independent.

- Earlier restoration profiles cover 10,000 lifetime funding rows and
  1,024 occupied reads across 32 tenants under their retained source/graphs.
  The current receipt-populated tiers now cover 100/1,000/10,000 intents; the
  10,000-intent synchronous restore costs 4.76 billion fixture instructions.
  Further performance work needs a
  representative production-host measurement and evidence for the proposed change;
  fixture observer counters do not qualify production restore costs. Preserve
  synchronous validation, durable identities and byte ownership at independent
  boundaries. None of these tiers qualifies million-object operation. Journal
  rows count lifetime operations, not necessarily stored service objects.
- Qualify selected tools in a consumer-owned prefix/build, then replace its registry
  0.7.0 preparer upon adoption. Preserve explicit binary/trust/history selection.
  Recover existing profiles with their original tools and bundles.
- Consumers must complete certified asset registration, overlapping release retention
  and deliberate exact reference cleanup. Miner's model/image shell acceptance does
  not establish a certified transaction, lease, deployed game or production serving.
- Qualify actual origins, MIME/CORS/cache/CSP and access/retention policy. Logical
  release refuses authenticated descriptors but need not revoke saved public URLs.
- Provider deletion/final billing and surviving inventory/freshness remain separate
  gaps. Missing downloads and zero usage counters prove neither deletion nor billing
  cessation. See [service gaps](../service-gaps.md).

## Consumer feedback and GitHub references

GitHub owns issue status and triage. The
[earlier retained issue review](../evidence/caffeine-probes/local/2026-10-04-gh-issues-review-01/summary.json)
keeps its original remote/source observations. The 2026-10-06 read-only review
in the [tooling adoption evidence](../evidence/shared-tooling-recovery-v01413.json)
finds eight open owning-repository issues and no open Shared Tooling issues or
owning-repository PRs. Use the existing issues for
[embedding #1](https://github.com/dragginzgame/ic-blob-storage/issues/1),
[budgets #2](https://github.com/dragginzgame/ic-blob-storage/issues/2),
[FIFO #3](https://github.com/dragginzgame/ic-blob-storage/issues/3),
[publishing #4](https://github.com/dragginzgame/ic-blob-storage/issues/4),
[serving #5](https://github.com/dragginzgame/ic-blob-storage/issues/5),
[lifetime #6](https://github.com/dragginzgame/ic-blob-storage/issues/6) and
[decoding #7](https://github.com/dragginzgame/ic-blob-storage/issues/7) and
[description #8](https://github.com/dragginzgame/ic-blob-storage/issues/8).

Source review finds the complete installation helpers, FIFO refusal, direct
publication budget refusal and standalone decoder-budget cases already present
for #1/#2/#3/#7; their issue acceptance must be reconciled with existing evidence,
rather than duplicating implementations. #4/#5 request complete consumer-owned
same-command publication, mapping and verified delivery; live acceptance remains
separate from retained local media evidence. The GitHub description is still
empty. This review changes no issue status, repository metadata or sibling code.

For #7, preserve the [decoder contract](../service-contract.md#standalone-ingress-decoding)
and original `.tmp/decoder-budgets-01` evidence. Do not weaken bounds or add test
hooks to force exhaustion. Native batch publication remains bounded to 4,096 files.
Canic adoption stays deferred. The [service contract](../service-contract.md)
and original retained evidence define wrapper/lifecycle, one-memory-runtime,
current-format and recovery obligations; consumer issues own adoption status.
No upstream message or sibling edit is authorized.

The old isolated owner stays frozen at 0.6.0 with original stopped history; the
separate live owner was last verified at 0.7.0. Both retain exhausted lifetime
capacity and complete provider/billing obligations. Never reset or upgrade them
as source cleanup. Cross-release reinstall requires obligation disposition.

[The ledger](../evidence/caffeine-probes/README.md) separates source/local/substitute
and deployed observations. [History](history.md) retains old handoffs without
turning their forecasts into current operating instructions.
