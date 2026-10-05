<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Caffeine probe ledger

This ledger preserves source review, local substitutes and deployed observations
as different evidence classes. It is intentionally detailed and ordered with the
newest records first. Start with [current status](../../status/current.md) for the
maintained release and open work.

**Quick links:** [recording rule](#recording-rule) ·
[probe command](#probe-command) · [run index](#run-index) ·
[acceptance plan](../../acceptance-plan.md) ·
[service gaps](../../service-gaps.md)

## Management-canister types 0.11.0 — 2026-10-05

The [intent](local/2026-10-05-management-types-011-01/intent.json) precedes scoped
local PocketIC recovery checks after the authorized dependency upgrade. Relative
to the captured pre-upgrade lock, only management types change from 0.10.0 to
0.11.0. Existing ic-memory 0.25.5 and ic-testkit 0.15.4 updates are preserved;
PocketIC 16 owns its separate native 0.8.0 types. The core Wasm path selects
0.11.0 through the workspace declaration, without an adapter or version reader.

The upstream source diff leaves our `canister_info` contracts unchanged. The new
instruction metric aggregates previous-round subnet work, including scheduler
and non-Wasm charges; it cannot replace operation profiling, independent history
or Caffeine accounting. P-256 ECDSA has no current consumer here. No new metrics
call, polling loop, state owner or runtime flow is introduced.

Three existing continuity unit tests and five existing actual PocketIC cases
pass: ordinary stop/start/repeated upgrades and operator refusal, truncated
history, a management change during the await, retained physical/billing
obligations, and snapshot rollback refusal without repair. Strict scoped core,
standalone-Wasm and harness Clippy plus matching builds pass. The
[summary](local/2026-10-05-management-types-011-01/summary.json) binds exact commands,
source archive/patch, before/after dependency inputs, upstream source comparison,
frozen executable/Wasm/server and logs under `.tmp/management-types-011-01`.

This is local history/recovery evidence with explicit local completion facts,
zero live provider calls and paid cycles. No production Rust source, public API,
persisted layout, package version, toolchain, declared MSRV or release receipt
changes. Full CI, native macOS, scale and deployed provider behavior are not
qualified. Earlier profiles retain their original graphs. No deployment,
sibling edit/message or cleanup occurs; consumer adoption, certified asset
registration and provider retirement remain separate obligations.

## Multi-tenant and multi-chunk restoration — 2026-10-05

The [intent](local/2026-10-05-restoration-shapes-01/intent.json) precedes extending
the existing opt-in sixteen-store profile on compiled 0.14.9. The private fixture
uses one current bounded installation record; ordinary callers keep two objects,
two tenants and ten-byte objects. Existing native Caffeine manifest preparation
supplies complete declarations, so population no longer hashes bodies in the
canister or returns requests already owned by the caller. No production model,
public API, memory layout or default CI workload changes.

The first 100-object packet traps at the fixture's existing 4 KiB ingress bound
before population. Its log, refusal and frozen artifacts remain; that first
packet was not saved. The [follow-up intent](local/2026-10-05-restoration-shapes-01/followup-intent.json)
keeps all decoder byte/header/work/type limits. Sixteen small-object or one
multi-chunk-object batches pass, retaining each exact request before dispatch.
All 795 population packets measure 703–2,519 bytes. No failure is overwritten.

| Lifetime operations | Tenants | Object bytes | Instrumented initialization instructions | Allocated heap | Physical stable memory |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 100 | 1 | 10 | 119,709,644 | 1,376,256 B | 18,939,904 B |
| 1,000 | 1 | 10 | 1,437,289,093 | 1,376,256 B | 22,085,632 B |
| 10,000 | 1 | 10 | 17,328,500,885 | 1,376,256 B | 72,417,280 B |
| 1,024 | 32 | 10 | 1,495,367,763 | 1,376,256 B | 21,037,056 B |
| 32 | 4 | 16 MiB | 45,040,110 | 1,376,256 B | 18,939,904 B |
| 4 | 1 | 64 MiB | 12,638,294 | 1,376,256 B | 18,939,904 B |

Every workload passes actual same-release upgrade with equal uncertain, live,
logically released and cancelled populations. Global accounting and selected
tenant-bound permissions, exact full manifests, overlapping references and
historical receipts survive. Request IDs deliberately overlap across tenants;
roots and leaves are distinct. Foreign callers refuse, and restored mutation
refuses without altering accounting or restoration counters. Normal application
limits are unchanged; no independent freshness/resume claim follows.

The [summary](local/2026-10-05-restoration-shapes-01/summary.json) binds frozen
artifacts, source patch, dependencies, exact successful population requests,
checkpoints, results and all failed attempts under `.tmp/restoration-shapes-01`.
Scoped fixture/harness Clippy, matching builds, formatting and the final ordinary
rollback/reopen regression pass. Initial lint and local loopback-binding failures
remain recorded. This is synthetic local completion evidence, with zero live
provider requests or paid cycles; no provider availability or retention claim.

These different populations do not isolate tenant or manifest overhead, and
instrumentation includes observer overhead. Allocated linear memory is not live
heap or peak working memory. No million-object, populated funding/read-history,
production-cost, macOS or stale-backup qualification follows. The new measurements
justify no allocator, unsafe path, cache or partial-restoration change. Production
sources, version and receipt are unchanged by this work. A concurrent task
updated workspace dependencies after the frozen profile build; its new graph is
not qualified by these artifacts. No full CI, deployment,
sibling edit/message or cleanup occurs. Consumer adoption, certified asset
registration and provider retirement remain separate obligations.

## Funding fixture upgrade correction — 2026-10-04

The [intent](local/2026-10-04-storage-funding-upgrade-01/intent.json) precedes
reproducing a reported local funding failure. All five reported cases use one
helper still sending the superseded bare-principal upgrade input. The callback
case reaches actual post-upgrade decoding refusal after its funding assertions
pass. Using the shared current installation encoder fixes the helper; all eleven
funding transport/guarded-dispatch cases now pass, including acceptance, refunds,
callback uncertainty and restored fences. No reader, policy or production change.

The [summary](local/2026-10-04-storage-funding-upgrade-01/summary.json) retains the
failed reproduction, frozen before/after harnesses, unchanged fixture Wasm, hashes
and scoped build/Clippy/formatting results in `.tmp/storage-funding-upgrade-01`.
There are zero live requests or paid cycles. This extends the 0.14.8 draft; no
full CI, release/version mutation, commit, deployment or cleanup occurs.

## Transfer memory and restoration read attribution — 2026-10-04

The [intent](local/2026-10-04-transfer-memory-01/intent.json) precedes local
read attribution and full media memory measurements on the 0.14.7 graph.
[Restoration](local/2026-10-04-transfer-memory-01/restoration-candidate-intent.json)
and [media](local/2026-10-04-transfer-memory-01/media-candidate-intent.json)
candidate intents precede the subsequent changes and fresh comparisons. Every
run uses normal PocketIC application limits or owned Chromium with local HTTPS
HTTP/2 provider substitutes; there are zero live requests and paid cycles.

The fixture counts safe logical-memory reads during synchronous store opening.
One-tenant 100/1,000/10,000-operation mixtures preserve exposure, live overlapping
references, released objects with physical/billing liabilities and cancellation.
Keeping each decoded activation generation in the existing per-tenant scratch
totals removes repeated enrollment reads; each permission still checks generation.
There is no persistent cache, format/API change, new recovery path or relaxed
fence. Independent tenants, older activations and a later future-generation row
are tested; corruption refuses without memory repair.

| Operations | Instrumented initialization before / after | Tenant-memory reads before / after |
| ---: | ---: | ---: |
| 100 | 126,735,060 / 119,735,434 | 920 / 29 |
| 1,000 | 1,506,327,172 / 1,435,064,894 | 9,020 / 29 |
| 10,000 | 17,986,173,576 / 17,265,226,781 | 90,020 / 29 |

Every tier passes actual same-release upgrade, accounting, exact selected
permissions/references/receipts and fenced refusal. Allocated heap and stable
pages are unchanged. At 10,000 operations, initialization drops 4.0%; wrapped
reads account for about 13% of the instrumented baseline. These counters exclude
traversal, decoding and other work. Their safe-read wrapper also differs from
production RuntimeMemory's unsafe-read delegation, so neither the totals nor
the attribution are production costs. The earlier uninstrumented baseline remains
separately recorded. Pinned dependency source shows destination initialization,
not a temporary array, in the default unsafe-read path. No allocator or unsafe
implementation change is justified by this review.

A shared bounded read-only observer serves fresh 1/8/32 MiB preparation and both
existing full media journeys. Preparation still refuses before certificate intent.
Before and after changes, GLB/WebP completes with eleven PUTs/nine GETs; PNG/JPEG
lost-final-reply/control-interruption recovery uses seven PUTs/ten GETs and no
additional PUT on recovery. Original roots/leaves/bytes/cache metadata, recovered
maps, public delivery and retained physical/liability accounting agree.

Full GLB delivery identifies a fixture-only numeric byte bridge and duplicate
decode. Native downloads already equal original bytes exactly; whole-response
SHA-256 binds the browser's public-response decode to the same bytes. Removing
that bridge keeps MIME/cache/CORS/CSP/opaque-origin assertions and decodes once.

| GLB reopened delivery context | Before | After |
| --- | ---: | ---: |
| Sampled Node peak RSS | 2,293,227,520 B | 252,026,880 B |
| Sampled summed Chromium RSS growth | 767,049,728 B | 126,234,624 B |

This improvement belongs to the fixture. Actual upload code and its observed
worker backing-storage peak (about 50.9 MB) are unchanged. Samples are lower
bounds; process RSS can double-count shared pages and observer requests can wait
behind worker CPU work. No sustained-load, eviction, power-loss, many-tenant,
maximum-manifest, occupied funding/read-history or million-object claim follows.

The [summary](local/2026-10-04-transfer-memory-01/summary.json) binds exact selected
commands, before/after artifacts, source archive/patch, dependency snapshots,
checkpoints, raw samples, complete results and retained logs/profiles under
`.tmp/restore-read-profile-01` and `.tmp/transfer-memory-profile-01`.
Strict core/fixture/harness Clippy, upload-owner unit tests, matching Wasm/CLI/browser
builds, independent installation readback, four PocketIC rollback/reopen/client
regressions and both fresh media journeys pass. The initial function-length Clippy
failure remains alongside the correction. This extends the 0.14.8 draft while
compiled artifacts remain 0.14.7. No full CI, version mutation, commit, publication,
deployment, live probe, sibling edit/message or build cleanup occurs. Consumer
adoption, certified asset registration and provider retirement remain open.

## Bounded service and browser resource profile — 2026-10-04

The [intent](local/2026-10-04-service-resources-01/intent.json) precedes local
measurements on the 0.14.7 graph. The existing unpublished storage probe installs
one bounded current configuration and opens the same sixteen service memories
through `ServiceStores::open`. It adds no production hook, record layout, cache,
partial restore, retry or extra controller. The fixture's explicit installation
record replaces its bare-principal input; every caller is updated, with ordinary
tests retaining their two-object bounds.

| Lifetime operations | Initialization instructions | Allocated heap after reopen | Physical stable memory |
| ---: | ---: | ---: | ---: |
| 100 | 113,980,058 | 1,376,256 B | 18,939,904 B |
| 1,000 | 1,317,267,806 | 1,376,256 B | 22,085,632 B |
| 10,000 | 15,476,426,969 | 1,376,256 B | 72,417,280 B |

All tiers pass actual same-release upgrades on a normal application subnet.
Equal populations retain possible exposure, two-reference live objects, released
objects with continuing physical/billing liabilities and unexposed cancellation.
Aggregate accounting, selected exact permissions, reference liveness and historical
receipts agree after restoration; mutations still refuse behind the fence.
Population uses bounded 100-operation steps and preserves checkpoints before
reopen. Counters separate host initialization from store install/open; heap means
allocated linear memory, and stable pages include allocation overhead. These are
single local observations with one tenant and ten-byte single-leaf objects, not
maximum-manifest/many-tenant/funding/read-history or million-object qualification.

Two fresh actual Chromium profiles measure 1/8/32 MiB preparation through the
maintained launcher, worker and patched SDK. The [follow-up intent](local/2026-10-04-service-resources-01/followup-intent.json)
precedes preserving in-progress samples on failure and correcting process-relative
timestamp labels; the first profile remains untouched. The follow-up's 32 MiB
sample reaches 174,799,063 B worker backing storage and a 375,865,344 B rise in
summed owned Chromium RSS over baseline. Worker backing storage returns to
734,630 B after forced GC. This indicates temporary amplification in this run;
it is neither a sustained-load leak test nor proof that an ownership copy can
safely disappear. Peaks are sampled lower bounds; worker replies can wait behind
CPU work and summed RSS can double-count shared pages. Every upload refuses its
synthetic wrong root before certificate intent. Successful-transfer memory remains
unmeasured; no provider request or paid cycle occurs.

The [summary](local/2026-10-04-service-resources-01/summary.json) binds exact
commands, source/artifact hashes, completed tiers, both browser profiles and
retained preparation failures. The resource fixture is opt-in, excluded from
default CI. Strict scoped Clippy, matching 0.14.7 builds and four focused PocketIC
rollback/reopen/client checks pass. There is no full CI, version change, commit,
publication, deployment, live probe, sibling edit/message or cleanup. Production
validation and trust-boundary copies remain intact; scan-cost isolation and
successful-transfer memory are the next evidence gaps, alongside consumer
adoption/certified registration and provider deletion/final billing.

## Matching 0.14.6 media rehearsal — 2026-10-04

The [intent](local/2026-10-04-released-media-v0146-01/intent.json) precedes rebuilding
and freezing the released 0.14.6 CLI, standalone Wasm, maintained PocketIC harness
and paired browser tools. Production and selected harness sources match the
released commit before/after execution; Cargo/receipt stay unchanged. The graph
locks ic-memory 0.25.0 and ic-testkit 0.14.11. PocketIC independently checks the
Wasm's compiled release against explicitly selected 0.14.6 and rejects installation
inputs bound to the wrong actual service before allocation.

| Journey | PUTs / GETs | Retained physical / liability bytes | Result |
| --- | ---: | ---: | --- |
| Original GLB/WebP completion | 11 / 9 | 8,389,774 / 8,389,774 | Complete verified downloads and recovered map |
| Original PNG/JPEG lost reply/control interruption | 7 / 10 | 4,318,116 / 4,318,116 | Original uncertain claim survives; no additional PUT on recovery |

All four original roots, chunk manifests, whole-content hashes and immutable cache
headers agree with retained original-cache preparation. Actual Chromium/worker,
native verifier and standalone service participate. Reopening preserves original
bindings and journals. Occupied local map output refuses without undoing service
completion; another output recovers the complete map. Overlapping references keep
delivery after first release; final release refuses tenant downloads while retaining
physical/billing obligations and historical receipts. Sessions retain the shared
120-second bound and distinct whole-fixture 180-second bound.

The [summary](local/2026-10-04-released-media-v0146-01/summary.json) binds the released
source archive, exact frozen artifacts, selected original media, commands, logs,
requests/results and retained original profiles under `.tmp/released-media-v0146-01`.
A preparation command used the unsupported `version` argument; its Node child
error/typed CLI refusal and the subsequent missing-runner attempt are retained.
Direct `--version` readback corrects selection before any journey. Installation
readback and both actual journeys pass; no failed provider effect is replayed.

This is local IC/provider-substitute evidence from repository-built tools, not a
second clean-prefix/no-Git installation check, consumer adoption, game rendering,
production-origin qualification or deployed Caffeine behavior. It adds no runtime
flow or schema and makes no successful-upload performance or peak-memory claim.
Earlier clean-source and deadline records retain their original 0.14.3 identities.
No full CI, version/dependency change, commit, publication, deployment, live provider
request, paid cycle, sibling edit/message or build cleanup occurs. Certified consumer
registration, deployed retention/deletion/final billing and large-scale service
restoration remain open.

## Populated browser journal profile — 2026-10-04

The [intent](local/2026-10-04-browser-journal-profile-01/intent.json) precedes an
actual maintained IndexedDB journal run on released 0.14.6. The first attempt
populates 675/5,000 rows, then reaches the whole fixture's 180-second deadline
following the 17,500-row checkpoint of its 20,000-row tier. Its failure, checkpoints
and original profile remain intact. No per-transaction timeout is reported and no
restart qualification is claimed for that run.

The [follow-up intent](local/2026-10-04-browser-journal-profile-01/followup-intent.json)
precedes a fresh 675/5,000/10,000-row pass. The maintained opt-in fixture now reopens
each completed tier before starting another, retaining completed evidence if a
later workload fails. It uses public journal methods and strict individual commits,
with synthetic compact bindings/envelopes and no certificate or gateway dispatch.

| Lifetime rows | Population | Reopen including CDP call | Median selected read after restart |
| ---: | ---: | ---: | ---: |
| 675 | 2.08 s | 13.3 ms | 1.4 ms |
| 5,000 | 22.60 s | 39.4 ms | 4.4 ms |
| 10,000 | 63.67 s | 20.7 ms | 7.0 ms |

Each is one local instrumented observation; reopening is not an OS cold-cache
measurement and the timings are not production thresholds. Thirty sequential
selected reads per tier are measured inside the page, excluding the outer CDP
handoff. Population blocks contain at most 500 strict saves. Page heap excludes
native IndexedDB storage and browser RSS; it is not a peak-memory measurement.

Exact uncertain certificate, permanently cancelled saved and observed-certificate/
uncertain-gateway rows survive process restart. New admission at full lifetime
capacity, certificate replay, cancelled dispatch and continuation after an uncertain
gateway request return their typed refusals. Existing save at capacity returns the
unchanged original row. These are synthetic client records, not authenticated
certificates, provider objects or canister lifecycle evidence.

The [summary](local/2026-10-04-browser-journal-profile-01/summary.json) binds released
source/receipt, original and final fixture sources, bundle, failure, checkpoints,
results and original profiles under `.tmp/browser-journal-profile-01`. Paired tools
build; scoped syntax/changelog/diff and artifact checks pass. All requests are owned
loopback asset GETs, with zero live IC/provider requests or paid cycles. Production
capacity/durability/replay checks and schema remain unchanged; no counter, cache,
retry or reset is added. The documentation no longer implies that IndexedDB count
is guaranteed constant-cost or cannot scan internally. Precise engine cost remains
unattributed; these timings do not justify deleting a correctness check.

No full CI/Rust build, version/dependency change, commit, deployment, sibling edit/
message or build cleanup occurs. Million-row capacity, large real envelopes,
eviction/power loss/profile rollback, full worker/browser memory and service-canister
reopen/instruction budgets remain unqualified. Consumer adoption and certified asset
registration remain open; no retained effect profile is replaced or erased.

## SDK owned Blob preparation — 2026-10-04

The [intent](local/2026-10-04-sdk-owned-blob-01/intent.json) precedes a focused
source review and local SDK/Chromium checks. Caffeine 1.1.2's maintained patch
copied the input before MIME detection awaited, then constructed another full
array for Blob. The first copy remains necessary to own caller bytes across that
await. Blob now snapshots the already-private array directly; upstream sniffing,
chunking, hashing, metadata, wire formats and one-shot handles remain unchanged.
Installed npm files remain untouched; the build verifies original upstream hashes
and applies the maintained patch to its private copy.

Actual Chromium explicit/sniffed MIME preparation preserves identical roots and
manifests for a selected 1 MiB + 479-byte view with NUL/high bytes and a distinct
tail, even after immediate caller mutation. Caller backing storage remains attached.
The maintained SDK probe exercises selected-view mutation in all six substitute
journeys: exact single/multi-chunk payloads, incomplete-status return, lost final
reply, failed HTTP reply and byte-budget refusal. Original journals retain their
uncertain/responded states and no retry is dispatched. The retained native verifier
accepts original bytes and rejects corrupted/truncated bytes; its earlier binary
hash is recorded separately, without claiming current Rust qualification.

The [summary](local/2026-10-04-sdk-owned-blob-01/summary.json) retains original
patch/generated sources, final source/bundle/result hashes, SDK request/response
records and Chromium profiles under `.tmp/sdk-owned-blob-01`. Frozen-input/worker
checks and actual Chromium launcher boundaries pass. Bundles rebuild and scoped
syntax/changelog/diff checks pass. No probe fails; zero live IC/provider requests
or paid cycles occur. There is no measured peak-memory saving, successful deployed
upload, consumer adoption, provider retention/billing or million-object qualification.
No Rust build/full CI, version/dependency change, commit, deployment, sibling
edit/message or cleanup is performed. Retained profiles keep their original bundles;
new tools do not replace their executable bindings or effect history.

## Browser body handoff profile — 2026-10-04

The [intent](local/2026-10-04-browser-handoff-profile-01/intent.json) precedes an
actual Chromium/maintained SDK baseline on released 0.14.5. A separate
[follow-up intent](local/2026-10-04-browser-handoff-profile-01/followup-intent.json)
precedes the base64 handoff and binary-byte checks. Both profiles retain identical
body hashes and 64 KiB raw-frame bounds. Deliberate root mismatch follows original
SHA/SDK preparation and refuses before certificate intent; inspection finds no
journal row. Only three owned loopback asset GETs occur per profiling run.

| Body | Released byte-array bridge | Base64 bridge |
| --- | ---: | ---: |
| 1 MiB | 2.99 s | 0.08 s |
| 8 MiB | 22.88 s | 0.48 s |
| 32 MiB | 86.41 s | 1.83 s |

These are single instrumented ascending passes through preparation/refusal, not
successful uploads or guaranteed production timings. Worker wait/preparation is
similar (32 MiB: 0.36 s before, 0.37 s after), supporting the inference that the
byte-array CDP serialization dominated the original client handoff. No Canic
execution was measured. Page assembled V8 used heap at 32 MiB falls from
264,107,808 to 47,873,000 bytes; this excludes worker heap and browser process RSS,
and is not a peak-memory measurement. Node peaks are sampled separately.

The [summary](local/2026-10-04-browser-handoff-profile-01/summary.json) binds
original samples, logs, source and bundle hashes under
`.tmp/browser-handoff-profile-01`. Both profiling runs and maintained actual
Chromium launcher checks pass, including NUL/high binary bytes, original profile
bindings, cancellation and failure boundaries. Browser bundles rebuild; syntax,
changelog and diff checks pass. The opt-in fixture adds no default CI run.
This is local Chromium/SDK evidence with zero provider/IC requests and paid
cycles. No successful transfer, populated store, full worker/process memory,
concurrent publisher or million-object qualification is claimed. No release,
dependency change, commit, deployment, sibling change/message or cleanup occurs.

## Publication fixture session deadline — 2026-10-04

The [intent](local/2026-10-04-publication-deadline-01/intent.json) precedes two
local IC/Chromium journeys after a fixture-only deadline consolidation. The
released fixture supplied 30 seconds to a complete CLI session but 120 seconds
to browser/subprocess owners. Miner's [reported earlier timeout](https://github.com/dragginzgame/ic-blob-storage/issues/5#issuecomment-5979572702)
and original claims remain with that consumer; successful runs do not erase it.

One fixture constant now supplies 120 seconds before original/recovery intent,
browser bootstrap and subprocess ownership. Single-step preparation/query calls
remain at 30 seconds, and the distinct whole-fixture timer remains 180 seconds.
No production default, authority, claim, retry policy or provider limit changes.

Matching rebuilt fixture harness/browser tools use the retained released 0.14.3
CLI/Wasm. Original cached GLB/WebP completes with eleven PUTs/nine GETs and
8,389,774 physical/liability bytes. PNG/JPEG lost-reply/control-interruption
recovery passes with seven PUTs/ten GETs and 4,318,116 retained bytes; it adds no
PUT. Original/recovery commands, saved intents and browser results agree on the
120-second selection. Complete maps, roots/leaves/whole bytes/cache metadata and
reference cleanup preserve their maintained behavior. Strict affected Clippy,
formatting, syntax and exact retained artifact/result hashes pass.

The [summary](local/2026-10-04-publication-deadline-01/summary.json) retains exact
source/artifact/profiles/results under `.tmp/publication-deadline-01`. Both cases
pass without a failed probe. This is unreleased fixture evidence, distinct from
the clean-release check below. Full CI remains pending; no live request, paid
cycle, release, publication, deployment, sibling change/message or cleanup occurs.

## Clean released source-tool installation — 2026-10-04

The [intent](local/2026-10-04-released-tools-01/intent.json) precedes a clean
Git archive of `v0.14.3` at `cba9f3da3846758e0bb6aa5fe114786867426911`.
The released receipt/file hashes match; local tag and remote main agree. A fresh
native installation prefix reports compiled 0.14.3. Fresh pinned npm dependencies
build the paired browser tools with Git discovery disabled. Actual SDK/native
snapshot handoff, repeat-output and corrupt-source refusals pass offline.

Fresh matching release Wasm/harness and exact installed tools complete original
cached GLB/WebP and recover PNG/JPEG lost replies/control interruption without
another PUT. Counts and retained bytes match the two deadline journeys above;
all four original roots, ordered leaves, whole-content digests, cache headers,
complete maps and original uncertain claims are verified independently.

The [summary](local/2026-10-04-released-tools-01/summary.json) binds all 1,341
released source files, archive/receipt, toolchain/profiles, binaries/bundles,
commands, results and original browser profiles under `.tmp/released-tools-01`.
Both cases pass without a failed probe. These original artifacts retain the
released fixture's 30-second native and 120-second browser/parent selections.
This qualifies the clean release recipe in this repository, using existing Cargo
cache and provisioned Node/Chromium/PocketIC; it is neither a cold machine nor
consumer-owned installation/adoption. It does not qualify consumer registration,
game rendering, deployed Caffeine serving/deletion/billing or registry publication.
No live/paid request, commit, release/deployment, sibling change/message or cleanup
occurs. Earlier unreleased-source and failed consumer records remain unchanged.

## GitHub feedback review — 2026-10-04

The [review](local/2026-10-04-gh-issues-review-01/summary.json) captures all seven
open issues and seventeen comments, verifies the released 0.14.2 registry package
and checks both native FIFO boundaries locally. These are source/registry/native
file facts, not new provider probes. The latest
[consumer report](https://github.com/dragginzgame/ic-blob-storage/issues/5#issuecomment-5979572702)
also records original-cache-root GLB/WebP through existing model/image loaders,
verified cached bytes, Blob revocation and zero local-shell CSP violations. That
reported qualification was not independently rerun here; its earlier failed
attempts and original artifacts remain with the consumer. It does not establish a
certified registration transaction, deployed serving or final deletion/billing.
The [current triage](../../status/current.md#current-checks-and-next-work) separates
resolved upstream implementation from release/adoption/operating acceptance and
new decoder-budget coverage #7. No GitHub write or live/paid effect occurs.

## Current memory initialization and lifecycle — 2026-10-04

The [intent](local/2026-10-04-current-memory-01/intent.json) precedes a separate
current-source capture with ic-memory 0.24.10 and ic-testkit 0.14.7. The concurrent
lock update changes the memory/constructor graph; native Clippy alone cannot
establish Wasm initialization or synchronous owner restoration. Matching fresh
standalone Wasm and harness builds pass, followed by three existing PocketIC cases.

Actual installation, stop/start and repeated same-release upgrades preserve
installation, reservations and all owner fences; replacement configuration is
rejected. Current-execution operator recovery preserves released physical/billing
liabilities and original admission, while released certificate access refuses.
Rolled-back prepared/earlier snapshots stay fenced and cannot use current-instance
recovery; rejected recovery preserves stable state. No production change or new
test mechanism is needed.

The [summary](local/2026-10-04-current-memory-01/summary.json) binds frozen source,
lock, exact Wasm/harness, selected test cases and full output under
`.tmp/current-memory-01`. Test state is transient PocketIC state, not a retained
live provider installation. The package remains compiled release 0.14.2 from
unreleased source. Separate affected strict Clippy on this graph passes in the
isolated-tool record. These checks do not relabel earlier media journeys as
0.24.10 or qualify cross-release restoration, consumer adoption or deployed
provider behavior. Full CI remains pending. No failed case, live request, paid
cycle, release/publication/deployment, sibling edit/message or cleanup occurs.

## Isolated source tool installation — 2026-10-04

The [intent](local/2026-10-04-isolated-tools-01/intent.json) precedes a separate
1,327-file source snapshot, fresh CLI prefix and fresh pinned npm installation.
The browser build passes both nested under a checkout and with Git discovery
disabled. The installed CLI reports compiled release 0.14.2 and passes the actual
cached two-chunk SDK/native snapshot handoff, repeated-output refusal and corrupt
source refusal. The installation recipe now retains machine-readable release,
Node/Rust versions and build profile alongside source and artifact identities.

Matching fresh Wasm/harness/browser artifacts from that snapshot complete cached
GLB/WebP with eleven PUTs/nine GETs and 8,389,774 physical/liability bytes. Cached
PNG/JPEG lost-final-reply recovery passes with seven PUTs/ten GETs and 4,318,116;
the original uncertain final request remains recorded and recovery adds no PUT.
Exact selected bundle hashes, all four original roots/ordered leaves, complete
body hashes/cache headers, completed maps and overlapping-reference cleanup pass.
The same maintained native/browser/effect owners are used; no installation bug,
replacement dispatcher or extra journal is introduced.

The [summary](local/2026-10-04-isolated-tools-01/summary.json) binds exact source,
installed binary, bundles, matching Wasm/harness, inputs, logs, profiles and
original claims/maps under `.tmp/isolated-tools-01`. The source is the unreleased
draft with ic-memory 0.24.7 and ic-testkit 0.14.7, using the retained Cargo cache
and provisioned Chromium/PocketIC. Fresh npm dependencies do not establish a cold
machine, a clean released tag or downstream adoption.

The initial sandbox DNS fetch failure remains retained; network preparation
succeeds before probes. The workspace lock changes independently to ic-memory
0.24.10 during execution. A post-probe equality assertion refuses; both locks and
the failure remain recorded. Separate locked fetch and strict affected Clippy
pass on that current graph. No probe is rerun and no runtime observation is
relabelled as 0.24.10. Frozen-source strict Clippy and formatting also pass.
Local serving does not qualify deployed retention/cache, consumer registration
or game rendering. Full CI remains pending. No commit, version change, publication,
deployment, live/paid provider effect, sibling edit/message or cleanup occurs.

## Original cache metadata — 2026-10-04

The [intent](local/2026-10-04-cache-metadata-01/intent.json) precedes offline
SDK/native checks and four bounded local IC/Chromium journeys. The same SDK
preparation now accepts an optional explicit cache hint before upstream metadata
hashing/tree construction. Native `preparation.cache_control` is preserved in
saved bindings and transfer descriptors; launcher/host/worker snapshots carry
browser `cacheControl`. Omission adds no header. This optional addition preserves
existing field meanings and serialization when absent; no incompatible layout,
alternate reader, inferred policy or parallel preparer is introduced.

Actual SDK checks reproduce all four retained consumer PNG/JPEG/GLB/WebP roots,
exact headers and ordered leaves. Changed or omitted required cache hints refuse
before intent access. Native hint bounds/omission tests, the two-chunk SDK/native
snapshot handoff, publication/worker refusals and launcher checks pass. The native
snapshot re-prepares from its saved hints, including filename/cache, after the
original source changes. Strict affected CLI/standalone Clippy, formatting,
JavaScript syntax and fresh CLI/harness/browser builds pass.

Original cached GLB/WebP completion makes eleven PUTs/nine GETs with 8,389,774
retained physical/liability bytes. PNG/JPEG lost-final-reply recovery makes seven
PUTs/ten GETs with 4,318,116; recovery adds no PUT. Browser reads expose the exact
hashed `public, max-age=31536000, immutable` value and matching complete bytes.
PNG corruption stops before attestation/next admission with five PUTs/one GET and
3,554,987 retained bytes. The authored uncached regression retains five PUTs/ten
GETs and 1,050,157 bytes, with no cache header. Reference cleanup/history and
occupied native output recovery retain their established behavior.

The [summary](local/2026-10-04-cache-metadata-01/summary.json) binds frozen inputs,
source, CLI/harness/browser artifacts, original profiles, journals, maps, request
histories and downloads under `.tmp/cache-metadata-01`. The unchanged standalone
Wasm is retained from the 0.14.2 representative-media record; new native/browser
artifacts also retain compiled package release 0.14.2. Both artifact-capture
failures remain recorded: hashing blocked on the launcher's refusal-test FIFO,
then an active stdout log changed after its initial hash. The owned blocked
process was stopped, special entries excluded, and a final manifest seals closed
regular artifacts. Initial captures, exclusions and verification failures remain;
no provider probe is rerun or omitted.

These local substitute observations do not qualify deployed cache operation,
future retention, game rendering, complete consumer registration or adoption.
Producers must explicitly retain their original cache hint and select matching
tools. Full CI and clean released downstream tool installation remain pending.
No version mutation, commit, publication/deployment, live provider request, paid
cycle, sibling edit/message or build cleanup occurs; old live owners are unchanged.

## Publication transfer-budget preflight — 2026-10-04

The [intent](local/2026-10-04-transfer-budget-01/intent.json) precedes offline
actual SDK and two bounded local IC/Chromium attempts. Frozen-file preparation
now compares valid shared gateway limits against one tree plus all SDK chunks,
the largest retained SDK chunk and total body bytes before the certificate client
saves intent. The patched SDK reports its own chunk size; hashing, chunking and
wire construction retain their original owner. Opaque tree/certificate overhead
is still checked by the existing gateway guard at dispatch; these lower bounds
are not a sufficiency or provider spending guarantee.

Single- and multi-chunk shortages refuse with zero intent access or transport.
Exact-fit lower-bound controls reach the original deliberately refusing store;
no certificate is issued. The worker preserves its initial claim inspection,
then returns finite `transfer-budget` with no save/claim/transport. Existing
snapshot, authority and private-error redaction checks pass.

Local GLB/WebP completion makes eleven PUTs/nine GETs and retains 8,389,774
physical/liability bytes. PNG/JPEG lost-final-reply recovery makes seven PUTs/ten
GETs and retains 4,318,116; no repeat PUT occurs. Native verification, complete
maps and overlapping-reference cleanup pass using retained 0.14.2 CLI/Wasm/harness
artifacts and newly captured browser bundles. Final worker error propagation is
qualified separately offline; it does not relabel the earlier browser bundle.

The [summary](local/2026-10-04-transfer-budget-01/summary.json) binds each attempt,
source, bundle, native artifact, original profile/journal and request history
under `.tmp/transfer-budget-01`. An exclusive artifact capture refused a basename
collision between source and bundled `worker.mjs`; partial capture and typed
failure remain retained, and a fresh capture preserves relative paths. No probe
is rerun for that capture failure. Local substitutes do not establish consumer
adoption or deployed serving/billing. No full CI, live/paid request, release,
publication/deployment, sibling edit/message or cleanup occurs.

## Source tool installation — 2026-10-04

The [intent](local/2026-10-04-source-tools-01/intent.json) precedes a fresh-prefix
locked installation of the existing CLI in the development profile, using this
repository's retained target directory. Offline `blob-storage --version` reports
the compiled library identity as JSON without identities or requests. The
[selected-checkout recipe](../../local-tools.md#install-the-native-and-browser-tools)
keeps native installation, browser modules, pinned SDK build and artifact hashes
under one selected source identity; it adds no registry package or retry owner.

The installed binary passes the existing actual SDK/native/browser snapshot
handoff with 1,049,055 bytes/two chunks, exact re-preparation, repeat/corrupt-source
refusal and zero network requests. Pinned browser builds, strict affected CLI
Clippy, formatting and production JavaScript syntax pass. This uses captured
dirty source with compiled release 0.14.2 and an existing dependency cache, not
a clean released checkout or fully cold environment.

The [summary](local/2026-10-04-source-tools-01/summary.json) binds source snapshots,
installed binary, version, bundles/modules, command logs and original handoff
artifacts under `.tmp/source-tools-01`. Clean downstream installation, consumer
adoption and original hashed cache metadata remain open. No full CI, release,
registry publication, deployment, live/paid provider request or cleanup occurs.

## Representative emitted media — 2026-10-04

The [intent](local/2026-10-04-representative-media-01/intent.json) precedes seven
bounded local IC/Chromium journeys using the owned HTTPS/HTTP2 substitute. The
read-only consumer review finds Miner HEAD
`0ce77a555f7d106ab127bfe1e78b889de4a46c5b` with 34 dirty paths; those unrelated
changes are excluded from released-source claims. Active feedback moved to
GitHub [publisher #4](https://github.com/dragginzgame/ic-blob-storage/issues/4),
[delivery #5](https://github.com/dragginzgame/ic-blob-storage/issues/5) and
[lifetime #6](https://github.com/dragginzgame/ic-blob-storage/issues/6). Exact bodies,
comments, source identities and dirty-state captures are retained privately;
no upstream message or issue change occurs.

Four selected emitted bodies match the clean asset checkout at
`a91b709edc80554fa758f5e5a972179f48172ec7` and Miner's saved raw digests: a
3,554,987-byte PNG, 763,129-byte JPEG, 8,362,256-byte GLB and 27,518-byte WebP.
The existing emitted inventory/preparation is retained and hashed, not rebuilt
or qualified as a current application release. Private copies are frozen before
the probes; no consumer asset is committed here. The same two-file fixture now
derives installation byte limits and uses each original MIME. Native and SDK
hash/manifest owners remain authoritative; the substitute serves uploaded metadata.

Offline actual SDK/helper execution confirms a consumer boundary: Miner's
original hashed `Cache-Control` cannot be reproduced by current preparation
hints. All four original roots refuse with `root` before intent-store access or
certificate requests. The trial explicitly prepares the same bodies under current
MIME/length metadata and retains both sets of roots/headers; it does not silently
drop metadata or adopt the original prepared map. Supported version/provenance-bound
native/browser tool installation is also still requested under #4.

Additional source review retains [transfer-budget #2](https://github.com/dragginzgame/ic-blob-storage/issues/2):
the current publication helper rebuilds exact SDK metadata but does not compare
locally predictable transfer demand before composing certificate/transfer owners.
Its supplied negative reproduction was not rerun here. These trials use sufficient
derived budgets and do not qualify refusal of insufficient budgets; the handoff
and integration backlog retain that next implementation action.

PNG/JPEG completion and lost-final-reply recovery each make seven PUTs/ten GETs,
retaining 4,318,116 physical/liability bytes. Chromium observes correct MIME,
complete digests, dimensions/pixels, ordinary image loading under authored CSP
and refusal with zero provider requests. GLB/WebP completion/recovery each make
eleven PUTs/nine GETs, retaining 8,389,774 physical/liability bytes. The eight-chunk
GLB has matching complete native/browser digests and observed container structure;
WebP decodes at 256 × 256. Structural inspection is not game rendering acceptance.
Neither lost reply adds a PUT; original final claims remain uncertain.

Before reference changes, occupied native map output refuses `new_run_required`
and preserves the original marker. Fresh signed queries then recover a complete
map without another upload. Second references survive first release; final release
refuses descriptors and fresh maps while preserving physical/liability bytes and
historical receipts. These are native output/reference facts, not a completed
application registration transaction or a serving lease.

Tail corruption of the real PNG stops before attestation/next admission with five
PUTs/one GET and 3,554,987 retained bytes. Authored PNG and original synthetic
regressions retain five PUTs/ten GETs and four PUTs/four GETs. All seven browser
journeys pass. Fresh CLI/Wasm/browser/harness builds, strict affected Clippy and
two installation/Candid checks pass with independently expected release 0.14.2.
The exact final installation capture retains Candid/configuration/readback bytes.

Every failed development check remains retained: the initial fixture compilation
used u128 for u64 read bounds; strict Clippy then required an explicit justified
line-limit expectation for the coherent constructor. An installation capture used
an unsupported report variable, then a precreated output directory was rejected;
a fresh output capture passes. None is omitted or described as a provider failure.
The [summary](local/2026-10-04-representative-media-01/summary.json) binds all source,
artifact, request, original profile/journal/map/download and failure histories
under `.tmp/representative-media-01` with separate hash manifests.

Consumer metadata/adoption/tool installation, actual application transactions and
verified Blob lifetimes, deployed MIME/CORS/cache/CSP, game rendering, retention,
deletion and billing cessation remain open. No full CI, version mutation, commit,
publication/deployment, live provider request, paid cycle, sibling edit/message
or build cleanup occurs. Both frozen live owners retain their original obligations.

## PNG image loading under CSP — 2026-10-04

The [intent](local/2026-10-04-media-csp-01/intent.json) precedes seven bounded
local IC/Chromium/SDK attempts using the owned HTTPS/HTTP2 substitute. One test
helper now owns PNG expectations, downloaded-byte decoding, direct fetch checks
and ordinary anonymous image loading. It consumes canonical retained native
download URLs; provider targets, native verification and paid-claim journals
retain their existing owners. No product API, wire/storage layout or retry owner
changes.

Separate same-publication-origin pages use `default-src 'none'` with an explicit
allowed provider `img-src`. Images decode with expected dimensions/pixels and
readable canvas data. A page with `img-src 'none'` refuses the image, reports the
enforced directive and makes zero provider GETs. Raw policy events remain
retained; sampled pixels and DOM decoding do not replace native whole-content
verification. Direct fetch and opaque-origin CORS checks remain maintained.

All six final journeys pass: multi-chunk completion/lost-final-reply recovery,
small-PNG completion/recovery, tail-corruption refusal and original synthetic
restart. Multi-chunk successes each make five PUTs/ten GETs; small-PNG successes
make four PUTs/nine GETs. No recovery adds a PUT. Tail corruption retains three
PUTs/one GET and stops before the next admission; synthetic restart retains four
PUTs/four GETs. Both large success cases retain 1,050,157 physical/liability bytes
after final first-object reference release, with 1,102 logical bytes. Saved public
URLs and ordinary images still work after release although descriptors refuse;
neither CORS nor CSP grants service revocation of public URLs or copies.

The initial large completion attempt failed after five PUTs/nine GETs because
the fixture expected an origin-only CSP `blockedURI`; Chromium reported the full
canonical URL. The corrected assertion checks its parsed origin and typed
directive/disposition, retaining the raw event. The failed attempt, exact earlier
source/bundle and request trace remain preserved. Bounded private traces are now
written even when a fixture assertion fails; original histories remain intact.

Fresh CLI/Wasm/harness/browser builds, syntax and Rust formatting pass. Two
installation/Candid checks pass with independently selected expected release
0.14.1, exact configuration readback, controller denial and wrong-service
refusal. These artifacts use ic-memory 0.24.5 and ic-testkit 0.14.4; previous
records retain their original 0.14.0 binaries. The
[summary](local/2026-10-04-media-csp-01/summary.json) binds every attempt, source,
artifact, request trace, original profile/journal and downloaded bytes under
`.tmp/media-csp-01`; public intent/summary hashes join the local manifest.

Owned processes/sockets have exited and temporary TLS keys are removed; retained
evidence and build artifacts remain. Authored PNGs and policy do not qualify
consumer assets/transactions, deployed Caffeine replies/MIME/CORS/cache/CSP,
future retention, deletion or billing cessation. No full CI, sibling edit/message,
release/publication, deployment, live provider request, paid cycle or build cleanup
occurs. Both frozen live owners retain their original obligations unchanged.

## Direct browser URL delivery — 2026-10-04

The [intent](local/2026-10-04-public-delivery-01/intent.json) precedes six bounded
local IC/Chromium/SDK journeys using the owned HTTPS/HTTP2 substitute. PNG cases
now fetch the exact retained native `download-request.json` URLs from the selected
publication origin: omitted credentials, refused redirects, no-store and a
ten-second deadline. Chromium observes CORS responses and checks PNG MIME,
declared length, SHA-256, dimensions and pixels. No new provider route builder,
product API, stable layout, journal or retry owner exists.

Successful multi-chunk completion and lost-final-reply recovery each make five
PUTs/eight GETs, including two browser reads and one opaque-origin CORS refusal.
Signed reference operations already refuse both released descriptors; afterward
the saved public URL still serves the first image under the substitute. Logical
bytes remain 1,102 and physical/liability bytes remain 1,050,157. CORS controls
browser reading; neither it nor logical release establishes confidentiality or
revokes saved URLs/copies. No provider deletion or billing cessation is inferred.

All six journeys pass: multi-chunk completion/recovery/corruption, small PNG
completion, original synthetic restart and a current-lock multi-chunk recovery.
Small PNG completion makes four PUTs/seven GETs. Corruption and synthetic
restart retain three PUTs/one GET and four PUTs/four GETs respectively. Strict
affected Clippy, browser build, current harness build, syntax, formatting and
the undated 0.14.1 changelog check pass. Cargo/receipt remain 0.14.0.

The [summary](local/2026-10-04-public-delivery-01/summary.json) retains every
attempt, observed serving metadata, exact sources/binaries/bundles, hashes,
original profiles/journals, native requests and downloaded bytes under
`.tmp/public-delivery-01`. The first five journeys use the retained prior
harness; a concurrent ic-testkit 0.14.2 lock update is preserved separately and
the sixth uses the newly built current harness. Production CLI/Wasm remain the
same retained 0.14.0 artifacts with ic-memory 0.24.3. No attempts failed.

Owned processes/sockets have exited; histories remain retained. These authored
fixtures and substitutes do not qualify deployed MIME/CORS/chunk replies,
consumer adoption/transactions, cache/CSP, access policy or retention. No full
CI, sibling edit/message, release/publication, deployment, live provider request,
paid cycle or build cleanup occurs. Both frozen live owners remain unchanged.

## Multi-chunk PNG and overlapping references — 2026-10-04

The [intent](local/2026-10-04-multichunk-driver-01/intent.json) precedes bounded
local IC/Chromium/SDK checks through the existing native driver, verifier and
reference owners. The shared owned HTTPS/HTTP2 substitute now validates each
bounded chunk's index, manifest hash, project and exact frozen byte slice; it
assembles complete content only after all ordered chunks. Caller transport limits
and journal request counts derive from the selected bodies. Test installation
limits remain bounded at two objects/one active upload, with room for two
references and their exact cleanup receipts. No product API, wire/stable layout,
dependency or dispatch/retry owner changes.

The authored 1,024 × 256 PNG is 1,049,055 bytes: a full 1,048,576-byte chunk and
a distinct 479-byte final chunk. Paired with the smaller cool PNG, success and
lost-final-reply recovery each make five PUTs/five GETs, retaining 1,050,157
physical/liability bytes. Both independently verified tenant downloads decode in
Chromium. Recovery retains responded/responded/uncertain first-upload claims
and makes no additional PUT. Corrupting byte 1,048,586, beyond the first complete
chunk, stops before attestation or the second admission with three PUTs/one GET
and 1,049,055 retained bytes.

Successful cases submit signed exact second-reference retain and both release
commands through the maintained native CLI. The second reference downloads after
first-reference release; all released references refuse descriptors. A fresh
signed map is incomplete although the original map and historical retain receipt
remain retained. Logical bytes fall to the second image's 1,102 bytes; all
physical/liability bytes remain. These are local reference/accounting facts,
not provider deletion, billing cessation or an application asset transaction.

Eight browser/IC attempts pass: initial complete journey, three final multi-chunk
cases, both original synthetic cases, small-PNG completion and withdrawal/late
attestation cleanup. Strict affected Clippy, browser/harness builds, syntax and
formatting pass. Two failed Clippy attempts are retained: expanded fixture line
count/large value passing, then manual byte-equality assertions. Final fixtures
reuse the existing length/first-difference helper, keeping large failure output
bounded. The [summary](local/2026-10-04-multichunk-driver-01/summary.json) binds
every attempt, initial/final sources and harnesses, exact bundles, original
claims/reference intents and downloaded bytes under `.tmp/multichunk-driver-01`.
Production CLI/Wasm are the retained 0.14.0 artifacts from the prior record.

Owned test processes have exited and histories remain retained. Small authored
media and provider substitutes do not qualify consumer assets/adoption, deployed
Caffeine chunk acknowledgments or direct public MIME/CORS/cache/CSP/retention.
No full CI, sibling edit/message, deployment, release/registry action, live
provider request or paid cycle occurs. Both frozen live owners remain unchanged.

## Distinct PNG publication and verified decoding — 2026-10-04

The [intent](local/2026-10-04-media-driver-01/intent.json) precedes five local
IC/Chromium journeys through the maintained SDK, driver and owned HTTPS/HTTP2
substitute. Publication fixtures now freeze arbitrary selected bytes through one
body-based manifest owner; budgets/accounting derive from those files. Native
tenant downloads are compared with the frozen originals. Two authored PNGs have
different dimensions, pixels and roots; Chromium decodes the verified downloaded
bytes and checks dimensions/first pixels. No new production flow, dependency,
journal, retry owner or contract layout is introduced.

Complete and explicit lost-reply recovery both retain 2,103 physical/liability
bytes and make four PUTs/four GETs, with no extra upload. Corruption stops before
attestation/next file with two PUTs/one GET and 1,001 retained bytes. Both original
synthetic driver journeys also pass, preserving their request/accounting totals.
The [refusal intent](local/2026-10-04-media-driver-01/refusal-intent.json) records
two additional corruption journeys after strengthening checks to query actual
service facts: the first upload stays exposure-possible and the second exact
admission is Unknown. Both pass with new profiles/artifacts; all original attempts
remain retained. No failed/inconclusive attempt occurs in this batch.

The [summary](local/2026-10-04-media-driver-01/summary.json) identifies measurements,
exact fixtures, sources, binaries/bundles, original histories and logs under
`.tmp/media-driver-01`. Fresh 0.14.0 CLI/Wasm/harness builds, strict affected
Clippy, syntax and formatting pass with the preserved maintainer ic-memory 0.24.3
lockfile update. Two installation/Candid checks independently confirm actual
Wasm release 0.14.0, wrong-service refusal and exact carrier installation.
Owned test processes have exited; keys/profiles/journals/downloads stay retained.
The [retention record](local/2026-10-04-media-driver-01/retention.json) distinguishes
initial and strengthened source snapshots, including the expected old-manifest
mismatch against the changed current source. Both snapshots match their exact
retained hashes; initial test artifacts remain alongside the strengthened harness.

These small authored images establish local PNG format handling, not consumer
assets, distinct multi-chunk media, direct public delivery or deployed provider
MIME/CORS/cache/CSP, retention or economics. Consumer adoption/transactions,
overlapping references, deletion and billing cessation remain open. No full CI,
release, registry action, sibling edit/message, deployment or paid/live provider
effect occurs. Both frozen live owners and their obligations are unchanged.

## One publication completion boundary — 2026-10-04

The [intent](local/2026-10-04-driver-final-01/intent.json) precedes focused pipe,
Chromium and two actual local IC/provider-substitute journeys. The current
`browser.driveSession(nativeControl)` captures ready/phase/finish before awaits
and owns final acceptance. Only an exact current `publish_map` with original
input hashes, all ordered indices, no blockers and neutral lease/serving flags,
plus an identical final native report and exit zero, returns `complete`. Stopped
outcomes retain their native result. The interim caller-owned final comparison
is removed from current code/examples; old run records stay with their exact bundles.
Native Rust still owns authenticated completion and references. No new journal,
retry, cursor, provider contract or asset-registration owner is added.

Eleven real child-pipe substitute checks and twenty-one actual Chromium boundary
cases pass. Final-record cancellation terminates the owned native child; the
browser retains its single-job gate through final exit. Metadata-only scripted
peers reject changed maps, missing/wrong indices, blockers, lease claims,
failed/mismatched final records and lost final replies; they establish boundary
behavior, not service authority. Both retained actual IC/Chromium direct-process
journeys pass: explicit source-session restart after a lost upload reply completes
two files without another upload (four PUTs/four GETs, 3,072 physical/liability
bytes); corruption stops before attestation/next file (two PUTs/one GET, 1,024 bytes).
These use synthetic image-labelled bytes and owned HTTPS/HTTP2/CORS substitutes.
They do not establish consumer asset registration, deployed Caffeine or real media.

The [summary](local/2026-10-04-driver-final-01/summary.json) binds exact sources,
bundles, logs, profiles and phase/final results in `.tmp/driver-final-01`. Original
keys, claims and profiles survive; owned test processes/sockets have exited.
The journeys deliberately use the retained prior 0.13.0 CLI/Wasm/test harness.
After the maintainer's dependency update, a separate source review/check builds
the core/standalone/CLI against direct management types 0.10.0, ic-memory 0.24.0
and ic-testkit 0.14.1 without adaptation. Fresh Wasm then passes four actual local
IC lifecycle/recovery cases and snapshot rollback refusal, with separate intent,
logs and artifact hashes under `.tmp/management-types-010`. Those checks exercise
platform history and fences; they are not a Caffeine upload/provider probe.
No full CI, registry/release action, deployment, sibling edit, live provider request
or paid cycle occurs. Consumer key/history selection, authenticated asset
transactions, real serving, rollback resistance and provider economics remain open.

## Private native subprocess control — 2026-10-04

The [intent](local/2026-10-04-native-control-01/intent.json) precedes focused private
pipe checks and two local IC/Chromium journeys. `startPublicationSession` owns one
explicitly selected native binary over bounded private pipes, accepts one phase
at a time, stops on cancellation/deadline and requires final output plus exit.
It snapshots arguments/environment without logging them; keys remain selected
native PEM paths and separate browser SDK identity JSON. No additional effect
journal, persisted cursor, automatic restart or retry is added.

Ten real child-process substitute checks pass under retained Node 24.21.0,
including split UTF-8, coalesced phase/final records, incomplete/malformed output,
unsolicited replies, oversized records/frames, busy refusal, startup/phase abort,
deadline and a successful-looking final record without process exit. The first
sandboxed attempts (system Node 18 and retained Node 24) could not receive Node
child stdout; a minimal child-write diagnostic isolated that execution restriction.
The same initial sources pass with local socket-backed stdio outside the sandbox.
Those failures/logs and the initial source snapshot are retained, not discarded.

The first IC test invocation failed because its evidence parent directory was
missing, before browser/provider access. A separately retained invocation uses
the already built test executable and a new existing parent. Both actual
IC/Chromium journeys then pass through maintained native pipes, replacing the
fixture's Rust phase proxy. The fixture retains ready/events/final results and
checks independent tenant downloads; it does not choose phases. Lost-reply
interruption is explicitly selected by the fixture, finishes the original native
run, then starts a new source-session and reopens the original profile. It confirms
two files with four PUTs/four GETs and 3,072 physical/liability bytes without another
upload. Corruption finishes with native `content_mismatch`/exit 3 before attestation
or the next file, with two PUTs/one GET and 1,024 retained bytes.

The [summary](local/2026-10-04-native-control-01/summary.json) binds exact artifacts,
logs, phase/results and current source. Profiles, original native claims, keys and
bodies remain in `.tmp/native-control-01`; owned test processes/sockets are closed.
The integrated bundle precedes the final additional configuration-type guards;
those guards and the non-exiting-child case pass the final pipe suite. Actual
journeys use retained 0.13.0 CLI/Wasm; concurrent dependency changes to ic-memory
0.24/ic-testkit 0.14 are preserved and reflected in the new harness, not qualified
as rebuilt service artifacts. Strict affected fixture Clippy passed before that
dependency update. No full CI, release, sibling edit, deployment, paid effect or
deployed Caffeine request occurs. These are synthetic bytes and owned substitutes;
consumer key/history selection, asset transaction, real-media serving, rollback
resistance and provider economics still require separate evidence.

## Native phase guidance and browser driver — 2026-10-03

The [intent](local/2026-10-03-session-driver-01/intent.json) precedes focused checks
and bounds three local IC/Chromium journeys plus one launcher suite. No live
provider request or paid cycle is planned. Native `next_frame` guidance now derives
from checked phase results and original sources: fresh status starts at zero,
original observation/handoff recovery precedes setup, and current exact completion
advances the cursor. Indexed phases share one ordering guard, preserving durable
unattempted facts for claim-owning phases. Existing phase/effect owners remain.

The callable bridge `driveSession` follows that guidance through the existing
worker/verification/map paths. Current ready/input binding, complete phase replies,
metadata and steps are bounded. One execution gate excludes concurrent jobs;
closing/deadline interrupts a pending control wait. Correlation IDs remain process
metadata, not a new operation identity. No persisted cursor, dispatch journal,
retry controller, native key/process discovery or automatic restart is added.

Seven native units, seven retained actual native IC cases, twenty actual Chromium
boundary cases and strict affected CLI/harness Clippy pass. Metadata-only native
peer substitutes in the launcher suite test control rejection, budget exhaustion
and cancellation; they do not establish IC authority. Fresh 0.13.0 CLI/Wasm artifacts
then pass two actual coordinated IC/Chromium cases: restart after a lost SDK reply
and before verification recovers the original handoff, confirms two files with
four planned PUTs/four GETs and keeps 3,072 physical/liability bytes. Corrupt bytes
stop before attestation/next file with two PUTs/one GET and 1,024 retained bytes.
The third maintained manual verifier/observation recovery journey also passes
with four PUTs/four GETs. No upload is retried. All three use synthetic image-labelled
bytes and owned HTTPS/HTTP2/CORS/provider substitutes; they do not qualify deployed
Caffeine, real media, consumer registration, serving or billing guarantees.

The [summary](local/2026-10-03-session-driver-01/summary.json) binds source/artifact,
log, profile and phase evidence. Private keys/profiles, original/resumed native
journals and runtime artifacts remain under `.tmp/session-driver-01`. Initial
native/test line-count lint failures and an initial launcher syntax-check failure
remain recorded before correction. A final documentation-only launcher comment
does not change its built serial bundle; byte equality is checked before the
third journey. Owned browsers and test IC/socket processes close; retained build
and evidence artifacts are not cleaned.

Service endpoint, native intent/source/profile and stable layouts are unchanged.
Phase events add derived guidance without another stored reader or version branch.
Consumer adoption and caller-owned process/control integration remain open, as do
rollback resistance and measured large-inventory behavior. No live provider,
payment, full CI, version/dependency change, commit/publication/deployment, sibling
edit or build cleanup occurs.

## Native browser transfer handoffs — 2026-10-03

The [intent](local/2026-10-03-transfer-handoff-01/intent.json) precedes the focused
checks and bounds two local IC/Chromium journeys plus one launcher suite, with
no live provider requests or paid cycles. A browser-selected native session now
retains the exact transfer handoff after validating its original signed setup,
before returning the step directory to its parent. Preparation no longer returns
a competing raw descriptor. Repeated/recovered phases name the original directly
and request certificate recovery through the existing browser journal only. The
launcher checks original/current native binding, exact frame/index and complete
handoff before worker execution; native-bound direct uploads refuse.

Seven native session unit checks, seven actual local IC cases, nineteen actual
Chromium boundary cases and strict affected Clippy pass. An empty browser journal
refuses handoff recovery with `history-missing`, never a fresh upload. Native
interruption preserves the first handoff; partial original history refuses before
new output allocation. The lost-reply actual IC/Chromium journey completes two
files with four planned PUT arrivals, four GETs, original claims and 3,072 retained
physical/liability bytes. A second journey refuses corrupt content before
attestation or the next file, with two PUTs, one GET and 1,024 retained bytes.
These are synthetic image-labelled bytes and owned HTTPS/HTTP2/CORS/provider
substitutes, not deployed Caffeine, real-media or billing qualification.

The preliminary native suite used temporary fixture directories: its log survives,
but those directories do not. The [retention intent](local/2026-10-03-transfer-handoff-01/retention-intent.json)
records this gap before a focused retained rerun. That rerun preserves the original
handoff bytes separately before the intentional corruption check. Private
profiles, original/resumed native journals, CLI/Wasm/bundles and all logs remain
under `.tmp/transfer-handoff-01`; the [summary](local/2026-10-03-transfer-handoff-01/summary.json)
binds their identities. Two line-count lint failures and a pre-probe metadata
heredoc syntax error are recorded before correction. Initial summary generation
also refused a Node child-process spawn in the sandbox; no manifest or summary
was written. Its missing-manifest check log remains, and shell enumeration
replaces that metadata step. Owned browsers/test sockets
close; build and evidence artifacts remain.

One current native intent/source layout uses `retained-browser-handoffs`; the
profile keeps its current `native-session-selection` shape. Old contracts stay
with their original artifacts/binaries; no old reader or conversion exists. This
joins the minor native/browser contract cut with unchanged service wire/stable
layouts and reused Wasm. The handoff is passive provenance, not a new paid-effect
journal, completion cursor or retry controller. An archived first handoff is not
retry permission. Automatic parent launch/phase selection, local-history rollback
resistance and consumer adoption remain unfinished. No live provider, paid effect,
full CI, version/dependency change, commit/publication/deployment, sibling edit or
build cleanup occurs.

## Native session browser selection — 2026-10-03

The [intent](local/2026-10-03-session-browser-01/intent.json),
[final native intent](local/2026-10-03-session-browser-01/final-intent.json) and
[strict-reader intent](local/2026-10-03-session-browser-01/strict-reader-intent.json)
precede their respective checks. Native intent now owns the original browser
selection: session/profile/port, signer/bundle fingerprints, journal and exact
batch project/bucket. Initial paths are fresh and have canonical existing parents;
source-session recovery retains the same selection rather than creating another.
The launcher checks the complete current original intent/ready shapes, actual
input hashes and scope/trust before Chromium, then binds the exact original intent
bytes into the profile. Explicit browser-only creation cannot bypass an existing
native binding. No new dispatcher, retry owner or completion cursor is introduced.

Seven final session unit cases, eighteen actual Chromium boundary cases, six
existing native PocketIC recovery/refusal cases and strict affected Clippy pass.
The maintained final native/IC/Chromium verifier lost-reply journey also passes
with the joint selection: four planned PUT arrivals, four GETs, original browser
and verifier claims retained, no upload retry, a confirmed map and 3,072 retained
physical/liability bytes. Native intent is bounded before allocating output.
These are synthetic image-labelled bytes and owned HTTPS/HTTP2/CORS/provider
substitutes, not deployed Caffeine, real-media or billing qualification.

The [summary](local/2026-10-03-session-browser-01/summary.json) retains exact source,
artifact, log and binding hashes. Private profiles and original/resumed native
journals remain under `.tmp/session-browser-01`. Intermediate passing checks and
their CLI/launcher bundles remain separately from final artifacts; the test-only
closure-semicolon lint failure is retained before correction. The first journey
uses the earlier equivalent fresh-profile expression and precedes the complete
intent-writer bound; the final journey uses the fresh final CLI and strict reader.
Full PUT bodies are not separately retained. Owned browsers, servers and test IC
processes close; retained source/build/evidence artifacts are not cleaned.

One current native intent/profile layout replaces the intermediate source-only
and launch-only forms. Retain older artifacts with their original binaries;
no conversion, alternative reader or inferred provenance exists. This is a minor
native/browser contract cut with unchanged core/standalone wire and stable layouts;
retained Wasm is reused unchanged. The parent still needs its keys/original session
location, explicit phase selection and fresh completion/reconciliation decisions.
Provenance grants no replay permission or rollback resistance. No live provider
request, paid effect, full CI, version/dependency change, commit/publication/
deployment, sibling edit or build cleanup occurs. Consumer adoption remains open.

## Browser profile launch binding — 2026-10-03

The [intent](local/2026-10-03-browser-binding-01/intent.json) precedes the
initial focused checks; the [final intent](local/2026-10-03-browser-binding-01/final-intent.json)
precedes final single-snapshot checks. One passive profile record binds the
selected canonical profile/origin, signer fingerprint, scope, root, journal and
trusted executable bundles before Chromium opens. It grants no effect, retry or
completion authority. Exclusive private creation and file/directory synchronization
precede browser access; missing, partial, changed, symlinked or relocated provenance
refuses without replacement. Runtime budgets/deadlines remain per execution.

Sixteen final actual Chromium boundary cases pass, including selected-input
mutation during launch, changed authority/assets refusal before context creation,
original-profile reopening and retained binding before an injected launch failure.
The maintained Chromium/IC/native verifier lost-reply journey passes: four planned
PUT arrivals, four GETs, original gateway/certificate and verifier claims retained,
no upload retry, a confirmed map and 3,072 physical/liability bytes still accounted.
These use synthetic image-labelled bytes and owned HTTPS/HTTP2/CORS/provider
substitutes; they establish no deployed Caffeine, real-media or billing guarantee.

All attempts remain in `.tmp/browser-binding-01`. The initial IC invocation fails
before browser start because its report parent is missing; it also selects the
persistent-session driver rather than the maintained launcher. No coverage is
claimed from that invocation. The corrected selection uses a fresh fixture and
the maintained launcher. Intermediate passing logs/bundles remain; final checks
follow removal of a redundant bootstrap clone. The initial directly imported
launcher snapshot was not frozen separately. Final source/artifact/log hashes,
private profiles and native journals are retained; see the
[summary](local/2026-10-03-browser-binding-01/summary.json).

This is a minor browser contract cut. Older profiles stay with their original
launcher/bundles; no binding is inferred for existing history. Service wire/stable
layouts and retained provider obligations are unchanged. The caller still selects
the profile/port/key and native source session; coordinated parent phase/restart
ownership and rollback resistance remain open. No live provider request, paid
effect, full CI, dependency/version change, commit/publication/deployment, sibling
edit or build cleanup occurs. Owned browser/server processes close; profiles and
evidence remain. Full PUT bodies are not separately retained.

## Native session original sources — 2026-10-03

The [intent](local/2026-10-03-session-sources-01/intent.json),
[browser intent](local/2026-10-03-session-sources-01/browser-intent.json) and
[final provenance intent](local/2026-10-03-session-sources-01/provenance-intent.json)
precede their respective checks. Explicit source-session recovery binds original
native input bytes, roles, gateway/root and installed release before a new output
run. Resolved original phase paths and unused sources survive repeated and
status-only restarts. One owner selects sources; existing setup, observer and
attestation claims still authorize phases. Unattempted ordering facts extend the
existing step records without a second dispatch/retry journal. Missing or conflicting
history refuses; a fresh authenticated completion cursor still starts at zero.

Five session unit cases, one maintained map correlation case, six actual owned IC
session cases and strict CLI/harness Clippy pass. Two final actual Chromium/IC
journeys recover exposed setup and original verifier observation after lost replies
without upload replay or another recovery GET/attestation update. Each retains
3,072 physical/liability bytes after four PUTs and four GETs. These are synthetic
media-labelled bytes, owned HTTPS/HTTP2/CORS and scripted provider facts; they
establish no new deployed Caffeine or full consumer guarantee.

The native session intent has one strict frozen identity. This requires a minor
native-format hard cut; old artifacts remain with original binaries. Service
endpoint/stable layouts and uploaded-object/balance/billing obligations are unchanged.
The [summary](local/2026-10-03-session-sources-01/summary.json) retains source,
artifact and log hashes and all attempts, including intermediate passing checks
before final provenance and function-length lint failures before correction.
Private final journals/profiles remain under `.tmp/session-sources-01/browser-final`;
the intermediate browser journey has retained logs/journals but no frozen CLI copy.
No live provider call, paid effect, deployment, version/commit/publication, sibling
edit, full CI or build cleanup occurs. Browser signer/profile/origin provenance,
complete parent integration, rollback resistance and consumer acceptance remain open.

## Descriptor serving owner — 2026-10-03

The forwarding public descriptor API and operational wrapper are removed. The
canonical endpoint handler delegates to the upload owner's checks and presents
the existing retained descriptor under the same borrowed installed scope. Client
target construction remains authoritative; historical inspection retains its
different authorization/lifecycle contract. This is a minor source API cut with
unchanged endpoint wire shapes and native/stable frozen layouts.

Twelve focused unit cases, six standalone and three storage PocketIC cases,
semantic exported/deployment Candid equality and strict core/standalone/harness/CLI
Clippy pass. They exercise exact tenant/reference/owner binding, original metadata,
suspension, release, restored fences and refusal before provider GET. Fresh
standalone/probe Wasm and the retained current-wire native client were used. These
are owned local IC fixtures with scripted provider facts; they establish no new
deployed Caffeine, media, retention, deletion or billing guarantee.

The [summary](local/2026-10-03-descriptor-owner-01/summary.json) records source,
artifact and log hashes. The intended pre-run intent file was not saved: this is a
recording gap, not evidence of a pre-recorded probe. The retained logs are not
overwritten or rerun to conceal that gap. No deployed provider request, paid effect,
deployment, version change, full CI, commit, sibling edit or cleanup occurred.
Private artifacts remain under `.tmp/descriptor-owner-01`; disposable PocketIC
case directories retain their logs. External consumer adoption remains unverified.

## Frozen format identities — 2026-10-03

The [intent](local/2026-10-03-format-identity-01/intent.json) precedes local checks
for the native/stable-format hard cut. One strict native binding reader now requires
`ic-blob-storage/upload-inputs:original-preparation`; all maintained producers and
single/batch/session consumers use it. One immutable installation model requires
`ic-blob-storage/installation:platform-anchor`, independent of exact release/service
checks. The allocation key is retained so incompatible occupied memory cannot be
hidden or replaced. No V2, alternate reader, migration or effect replay is added.

Native preparation/batch/journal and installation checks pass, including wrong/
missing identity refusal and unchanged owner bytes. Fresh standalone Wasm, strict
affected Clippy, semantic Candid equality and the offline 1 KiB SDK/native handoff
pass. Actual local IC checks retain all fences through same-release stop/start and
repeated upgrade, recover original signed setup after session control loss without
another update, and recheck selected bytes before setup. These use owned PocketIC
and scripted provider facts, not deployed Caffeine guarantees.

An archived binding is refused before output creation; original inputs, binaries
and selected live-trial records still match their hashes. The initial evidence
checker expected exit 3; argument refusal correctly returns 2. Its log/checker
correction are retained, with no repeated invocation. The
[summary](local/2026-10-03-format-identity-01/summary.json) records artifacts,
source/log hashes, exact scopes and read-only local Canic/Miner inspection. No local
adopter was found in the inspected code; deployment/external adoption is unknown.
Miner's separate `prepare_upload` example contract is unchanged.

This requires a minor release and fresh installation after complete retirement.
Any existing 0.11.0 installation, and the frozen 0.6.0/0.7.0 live owners, must remain
on their original release until object/effect/balance/billing disposition is proven.
Original artifacts are not converted or relabelled. There is no full CI/release,
version change, commit, publication, deployment, deployed provider request, paid
effect, sibling edit or build cleanup. Private reports and exact tested artifacts
remain in `.tmp/format-identity-01`; disposable PocketIC case directories leave logs.

## Simplification follow-up — 2026-10-03

The [intent](local/2026-10-03-simplification-02/intent.json) records offline SDK,
substituted-store, actual local browser and native contract checks before execution.
This batch shares UTF-8 metadata bounds, removes an intermediate selected-body copy
and consolidates Candid equality. The [retirement intent](local/2026-10-03-simplification-02/retirement-intent.json)
also records removing the fixture-only reference format/save/lock flow. Its service
guarantees stay covered by maintained signed native tests; original artifact files
and the production signed-packet owners remain intact.

Offline worker/publication and actual Chromium/IndexedDB bootstrap checks pass,
including Unicode bounds, selected views, mutation, cancellation, restart and
redaction. Semantic Candid equality, signed-packet claims, remaining operator unit
checks and local reference history/recovery pass; strict affected Clippy and Rust
1.88 compilation pass. Local provider facts are substitutes, not deployed deletion
or billing evidence. No deployed IC/provider request, paid effect, release or
cleanup occurs. Profiles, exact CLI/Wasm copies and attempted logs remain under
`.tmp/simplification-02`. The [summary](local/2026-10-03-simplification-02/summary.json)
records exact scopes, hashes and dependency identities, including a later ic-testkit
update and focused revalidation. The first Chromium attempt hit sandbox EPERM;
approval review then rejected the old audit scope before accepting the current
implementation instruction. Both refusals are retained rather than omitted.

## Native Chromium process bridge — 2026-10-03

The [intent](local/2026-10-03-native-browser-01/intent.json) and
[local record](local/2026-10-03-native-browser-01/summary.json) separate source/offline
checks, actual Chromium/IndexedDB and owned PocketIC/HTTPS substitutes. No deployed
Caffeine call, payment or live-owner change occurs. Native input bindings hard-cut
to exact original preparation hints; service wire/state, host Wasm and dependencies
remain unchanged. The next release must be minor for that native input contract.

The maintained process bridge uses a selected SDK identity and trusted host/worker
bundles, fixed asset port/origin, explicit create/open profile and bounded selected
body/digest checks. It exposes no HTTP credentials/body route and adds no dispatch,
retry journal or completion authority. Original journals and native phase owners
still authorize effects/advancement; profile loss never licenses a replacement.

Ninety native cases, eleven actual bridge cases, five PocketIC sessions, a real
adapted SDK/native 10 MiB input round trip and strict CLI/harness lint pass. Bridge
cases include request ownership/concurrency, body/hint bounds, digest changes,
symlink/directory/FIFO refusal, occupied port before profile creation, changed-origin/
missing/symlink profile refusal, original journal reopening and deadline termination.
Three actual browser/native-verifier journeys pass through the bridge at one
active reservation. Completion and exact lost-reply recovery each retain 3,072
physical/liability bytes with four PUTs/four GETs; corruption stops before attestation
or the next file, retaining 1,024 bytes after two PUTs/one GET. No upload replay occurs.

`.tmp/native-browser-01` retains profiles, native setup/observer/submission/control
records, gateway fingerprints, exact tested artifacts, source/log/file hashes and
all attempts. These include the wrong native `--lib` target, socket-denied unit
checks, a zero-case storage filter, an unadapted SDK import, unresolved browser-only
bundle import, a banner identifier collision and a lint length failure, followed
by corrected checks. The five separate native-session fixtures use disposable
directories; only their logs survive. Full PUT bodies are not separately captured.
Synthetic image-labelled bytes and owned CORS allowances do not qualify real media,
deployed provider CORS/retention/deletion/billing, full Miner adoption or production
spend. Large-body heap/CDP latency, hostile local-user/key isolation and rollback
resistance remain unmeasured. Complete durable parent restart/phase coordination
remains open; Canic adoption is deferred. No full CI/release, version/commit/
publication, deployment, sibling edit, upstream message or cleanup occurs.

## Maintained browser host and signer bootstrap — 2026-10-03

The [local record](local/2026-10-03-worker-bootstrap-01/summary.json) binds
[intent](local/2026-10-03-worker-bootstrap-01/intent.json) and
[final payload intent](local/2026-10-03-worker-bootstrap-01/payload-intent.json) to source review,
offline worker boundaries, actual Chromium/IndexedDB and owned PocketIC/HTTPS
substitutes. No deployed provider request or paid effect occurs. Core, native CLI,
host Wasm and dependency versions remain unchanged this batch.

Replace the fixture-only bootstrap and private-port control with the maintained
browser host/entry. Send only an explicitly selected SDK identity over a transferred
private port, recompute its key pair through the pinned SDK, and validate scope/
root/budgets before journal creation. Existing strict IndexedDB and certificate/
gateway owners retain dispatch. Shared job snapshots copy bounded selected views
before cloning/transfer; host close/abort/deadline terminates its worker while
retaining history and uncertainty.

Pinned peer/source/patch builds, 31 offline worker cases, 15 actual bootstrap cases
and three native-verifier Chromium/PocketIC journeys pass. Bootstrap checks cover
both signer kinds, malformed/mismatched payloads with no journal created,
snapshots, concurrency, foreign worker assets, pre-aborted startup, pending close,
deadline and reopening the same profile. Actual serial journeys retain original
setup/observer/submission/profile captures, exact original-statement recovery,
foreign/conflicting observation refusal and corruption preventing attestation/
the next file. Success/recovery each retain 3,072 physical/liability bytes and
four PUTs/four GETs; corruption retains 1,024 after two PUTs/one GET. No retry occurs.

`.tmp/worker-bootstrap-01` retains all bootstrap runs, tested bundles, native/
browser original phase and dispatch records, profiles, request fingerprints,
source/artifact/log hashes and a private manifest. The serial fixture counts
certificate calls from actual browser requests; it no longer relies on diagnostic
worker events. No failed test/probe occurs this batch. Full PUT bodies are not
separately captured. Local substitutes and synthetic image-labelled bytes do not
qualify deployed retention/deletion/billing or real media/public serving. Native
OS/browser-process launch, signer-input selection, original preparation-hint
persistence, durable parent restart coordination and full Miner acceptance remain
open. Canic adoption stays deferred. No full CI/release, version/commit/publication,
deployment, sibling edit or build cleanup occurs.

## Native session verifier composition — 2026-10-03

The [local record](local/2026-10-03-native-verifier-01/summary.json) binds
[intent](local/2026-10-03-native-verifier-01/intent.json) and
[exact-statement regression intent](local/2026-10-03-native-verifier-01/exact-intent.json)
to owned PocketIC/Chromium/HTTPS substitutes. No deployed Caffeine request,
paid effect or live-owner change occurs.

An explicitly selected installed verifier enables session composition of the
maintained observer, one-shot attestation and typed current-reference check.
Original permission, raw digest and gateway bind before signing. One create-new
verification directory per original index preserves intent and partial artifacts.
Recovery uses the original complete observation and exact immutable receipt;
it performs no GET or submission. Absence/conflict cannot be replaced by another
statement with the same content digest. SDK/IndexedDB dispatch owners remain intact.

Strict CLI/integration lint, 89 native unit cases, five native session cases and
three final dedicated-worker journeys pass. The latter cover two-file completion
at one active reservation, lost upload reply and native/browser restart, original
observation recovery, foreign-upload rejection, and an explicitly separate altered
observation-time input refusing despite an already live reference. Final success/
recovery each use four gateway PUTs and four GETs, without retries; corruption
stops at two PUTs/one GET before attestation or the second file. Physical/liability
bytes remain 3,072 after completion and 1,024 after corruption.

`.tmp/native-verifier-01` retains initial/final/exact browser captures, original
session/observer/signed-submission records, profiles, gateway fingerprints, both
native binaries, 903 source hashes, logs and a private file manifest. Initial lint
length failures and extraction compile errors survive before correction. Existing
native session fixtures use disposable directories; only their logs survive.
Full PUT bodies are not separately retained. Shared Cargo package-cache waits
do not change the selected repository target directory. These local trials do not
qualify deployed retention, deletion, billing cessation, real media or public
serving. Native browser parent/bootstrap and complete Miner acceptance remain
open. No full CI/release, version mutation, commit, publication, deployment,
sibling edit or cleanup occurs; Canic adoption remains deferred.

## Native identity and metadata simplification — 2026-10-03

The [local record](local/2026-10-03-native-cleanup-01/summary.json) binds
[intent](local/2026-10-03-native-cleanup-01/intent.json) to scoped native and actual
PocketIC/Chromium checks against owned loopback substitutes. No deployed provider
traffic or paid effect occurs. The prior simplification work and evidence remain
intact; this follow-up changes native tools only.

Four report consumers share the existing full-width upload-identity formatter.
Local byte verification still proves the original provider root before reporting
that identity. Setup, local verification, provider observation and attestation
submission share the same manifest reply envelope with their original content
bounds, distinct authority checks and failure mappings. Source provenance includes
the shared helper sources. Reference receipt inspection reuses the exact validated
request loader without a second scope/tenant check. No wire or journal format changes.

All 89 native unit cases, strict native lint and nine selected integration cases
pass. Reference cases preserve acknowledged, lost and pending replies; other
checks cover 10 MiB verification, paged/restored history, certificate blockers and
setup recovery. Two retained browser journeys observe/attest exact content, verify
tenant downloads and retain physical/liability bytes after logical release. A lost
chunk reply survives stop/start and certificate recovery without another upload.

`.tmp/native-cleanup-01` retains the intent, native binary frozen before integration,
browser/reference original requests and replies, profiles, gateway fingerprints,
source/artifact/log hashes and a private file manifest. Existing standalone
verify/history/certificate/setup fixtures use disposable directories; only their
logs survive. Full provider PUT bodies are not separately captured. Local substitutes
do not qualify deployed Caffeine semantics, deletion, retention or billing cessation.
No full CI, release, commit, publication, deployment, live-owner change, sibling
edit or build cleanup occurs. Native parent/verifier automation, complete Miner
qualification and deferred Canic adoption remain open.

## Implementation simplification regressions — 2026-10-03

The [local record](local/2026-10-03-simplification-01/summary.json) binds
[original intent](local/2026-10-03-simplification-01/intent.json),
[fixture correction](local/2026-10-03-simplification-01/regression-intent.json) and
[refusal-order regression intent](local/2026-10-03-simplification-01/ordering-intent.json)
to offline, actual IndexedDB and owned PocketIC/HTTPS gateway evidence. No
deployed Caffeine call or paid effect occurs. SDK 1.1.2, certificate/gateway
journals, configured verifier and native operation claims retain their roles.

Shared binding and budget predicates reject malformed worker jobs before store
access. Frozen files carry their raw digests, native publication decisions use
typed observations, and synchronous download serving drops its repeated content
lookup. JSON reports and journal formats remain unchanged. Original operation
identity still rejects before a recovery fence refusal; uncertainty never permits
redispatch. Existing authority, cancellation, restore and accounting checks remain.

Native unit/core read checks, all selected standalone publication/download cases,
31 offline worker cases, frozen-input checks and actual IndexedDB persistence pass.
All thirteen Chromium journeys pass. Four indexed recovery cases and the worker
lost-reply journey pass after the final refusal-order correction, with the final
hashed CLI. Strict core/CLI lint and scoped native/Wasm builds pass. The earlier
browser suite's native binary was not separately frozen; its claims, replies and
profiles remain, and the record distinguishes the final artifact validation.

`.tmp/simplification-01` retains original journals, query evidence, profiles,
request fingerprints, failed/successful logs and source/private file manifests.
The separate IndexedDB profile path is recorded and hashed. Initial sandbox
listener refusals, missing capture/path setup, function lint limit, full-width
test overflow and capture-helper child-process failure survive with corrections.
Full gateway PUT bodies are not separately captured. Owned servers/browsers close.
No full CI, release, commit, publication, deployment, live-owner change, sibling
edit or cleanup occurs. Complete Miner serving and Canic adoption remain open.

## Private-port browser worker — 2026-10-03

The [local record](local/2026-10-03-publication-worker-01/summary.json) binds
[original intent](local/2026-10-03-publication-worker-01/intent.json) and
[regression intent](local/2026-10-03-publication-worker-01/regression-intent.json)
to offline boundary checks and actual standalone/PocketIC/native/Chromium
DedicatedWorker journeys against the owned HTTPS HTTP/2 substitute. No live
Caffeine request or paid effect occurs. The existing patched Caffeine 1.1.2 SDK
and strict IndexedDB certificate/gateway journals remain dispatch owners.

The worker binds exact signer/service/tenant/project/bucket, trust root and
origins before jobs, permits one active job, snapshots input and bounds jobs/
deadline/body/requests. Private-port replies omit credentials and raw provider
messages; `service_completion_checked:false` prevents an upload result being
mistaken for native completion evidence. Finite public error codes and validated
projected phase/status values prevent exported-error payloads or corrupt custom
store values leaking into replies. Inspection/recovery/cancellation do not
create missing history; claimed or cancelled uploads cannot redispatch.
Offline checks use a substituted store and prove no provider/claim behavior.

Three dedicated-worker cases and all thirteen affected browser regressions pass.
Lost replies preserve uncertain gateway rows across browser/native restart.
Exact exposed setup recovery performs zero updates; independent observation/
attestation/reference checks advance without another PUT. Cancellation survives
reopening; corrupt bytes block the next transfer/map and retain 1,024 exposed
physical/liability bytes. Complete cases retain 3,072 bytes. No deletion or billing
cessation is established. Worker transport cancellation is cooperative; process
termination and profile-loss/rollback fencing remain host responsibilities.

Strict integration lint, pinned SDK/source/patch browser build, formatting and
whitespace checks pass. Private profiles, native signed claims/raw replies,
frozen inputs, opaque gateway fingerprints, redacted worker results and a file
hash manifest survive in `.tmp/publication-worker-01`. Full provider PUT bodies
are not separately captured; their durable fingerprints and certificate claims
survive. The initial successful captures used a misleading fixed `file_live:false`
field, replaced directly with `service_completion_checked:false`; intermediate
captures remain historical evidence. A thirteen-case regression passes, followed
by three final worker regressions after the error-code/projection safeguards;
final offline cases also refuse private code strings and oversized projections.
The first documentation patch rejection is recorded; no effect occurred.

Native parent launch/signing/bootstrap, durable phase intent and verifier
automation remain required for complete noninteractive publication. Qualify real
Miner media, release overlap and public serving separately. Core/host/native
artifacts and live owners remain unchanged; concurrent dependency changes survive.
No full CI, version/release/commit/publication, deployment, sibling write or build
cleanup occurs. Canic adoption remains deferred; owned local servers/browsers close.

## Persistent native publication phases — 2026-10-03

The [local record](local/2026-10-03-publish-session-01/summary.json) binds the
[original intent](local/2026-10-03-publish-session-01/intent.json),
[recovery intent](local/2026-10-03-publish-session-01/recovery-intent.json) and
[capture/regression intent](local/2026-10-03-publish-session-01/regression-intent.json)
to actual native/PocketIC and owned HTTPS HTTP/2 Chromium results. No live
Caffeine request or paid effect occurs. The existing patched Caffeine 1.1.2 SDK
and strict IndexedDB journals retain certificate/gateway dispatch ownership.

`publish-session` validates one complete frozen batch, then holds it across
bounded setup/status/map frames. Only selected bodies are rehashed before setup;
the browser snapshots and verifies raw digest/SDK root before certificate intent.
Exact configured-verifier completion and live first references gate advancement.
Current-reference observations gate the final whole map. Cached intent is not
a fresh verification of every current local body copy or a publication lease.

Native cases pass for cached batch reuse, changed selected bytes before admission,
finite steps, idle deadline with stdin still open, EOF/process restart with exact
original signed claims/zero updates, and tampered-source refusal. Three persistent
browser cases and three affected indexed regressions pass with two original roots
at one active reservation. Lost chunk replies retain uncertain gateway history
across browser and native restart. Original exposed setup refuses recovery with
zero updates; independent byte observation/attestation reconciles without another
PUT. Corruption blocks the second transfer and any complete map. Complete/lost
cases retain 3,072 physical/liability bytes; corruption retains 1,024 exposed bytes.

Targeted native CLI and affected setup/map/recovery PocketIC checks, strict
CLI/integration lint, pinned browser build, formatting and whitespace checks pass.
Private original signed setup/attestation requests, raw query replies, control
frames, frozen inputs, gateway request hashes and browser profiles are retained
under `.tmp/publish-session-01`. A private file-hash manifest and source/artifact/
log hashes bind the captures. Initial disposable native captures lacked retained
requests; their logs survive and the supplemental final run retains originals.
The decoder's ignored extra field, strict helper lint and missed capture-helper
import failures remain separately recorded before corrected runs.

The native controller is ready for composition; production browser-worker and
verifier-phase automation remain unfinished. Full Miner media, reference overlap
and public MIME/CORS/cache/CSP/open-browser retention still need consumer
acceptance. No full CI, version/release/commit/publication, deployment, sibling
write or cleanup occurs. Live exhausted owners and original obligations survive;
Canic adoption remains deferred. Owned local servers and browsers close.

## Serial completion and browser restart — 2026-10-03

The [local record](local/2026-10-03-serial-publication-01/summary.json) binds the
[original intent](local/2026-10-03-serial-publication-01/intent.json) and separate
[regression intent](local/2026-10-03-serial-publication-01/regression-intent.json)
to actual standalone PocketIC, native CLI and Chromium results against the owned
HTTPS HTTP/2 substitute. No live Caffeine request or paid effect occurs. SDK
source/hash/patch checks precede the browser runs; the same Caffeine 1.1.2 client
owns preparation/signing/transfer and existing journals own dispatch.

Two original files (1,024 and 2,048 bytes, distinct roots) qualify serial progress
at one active reservation. SDK success or a lost whole-chunk reply leaves that
reservation occupied. Independent complete-byte observation, exact configured
attestation and authenticated `publish-file-status` permit the next setup; only
the final full-map command writes a complete map. Whole-browser restart preserves
both strict IndexedDB histories and their original gateway phases, including the
lost first reply, without another PUT. Corrupt first observation prevents a second
transfer and any complete map. Completed cases retain 3,072 physical/liability
bytes; corruption retains the exposed first 1,024 bytes. No deletion or billing
cessation is established.

Three serial journeys and the four affected single-file regressions pass, together
with native/indexed-map checks and strict CLI/integration lint. Private original
requests, signed claims, raw replies, frozen bodies, browser profiles and bounded
gateway request/hash records survive under `.tmp/serial-publication-01`; the record
includes source/artifact/log hashes. Socket-denied native cases, PocketIC's
already-live localhost URL, ambient Node 18 and test-helper lint/extraction compile
failures remain recorded before corrected runs. Production loopback/TLS policy
and dispatch ownership are unchanged; owned local servers/browsers close.

This is serial local composition, not a complete production headless publisher or
Miner acceptance. Independent CLI phases still reverify the full batch; the future
coordinator must retain one validated batch and original per-file journals.
Qualify actual consumer media, overlapping references and MIME/CORS/cache/CSP/
open-browser retention separately. No full CI, release/version/commit/publication,
deployment, sibling write or build cleanup occurs. Existing live owners/profiles
and physical/billing obligations remain untouched; Canic adoption stays deferred.

## Frozen-file transfer and journal capacity — 2026-10-03

The [local record](local/2026-10-03-publish-transfer-01/summary.json) connects
native frozen batch/setup to maintained Caffeine SDK preparation and the existing
browser certificate/gateway journal. Raw body SHA-256, exact metadata/leaves and
rebuilt provider root must match before intent access. Offline corrupt/tampered/
aborted inputs refuse; caller mutation cannot change the snapshotted body/root.
No provider format, new dispatch journal or retry owner is introduced.

Actual standalone PocketIC/Chromium success, corrupt-read, lost-final-reply and
withdrawn/late-completion journeys pass against an owned HTTPS HTTP/2 gateway
substitute. Independent verification, attestation, tenant download and logical
release reuse maintained native commands. Lost replies survive stop/start and
operator recovery without another upload; SDK success alone cannot confirm.
These are local composition results, not deployed provider observations.

The configurable browser lifetime ceiling is now 1,000,000, independent of service
capacity and 4,096-file batches. Actual strict IndexedDB commits retain 675
synthetic rows across whole-browser restart. A separate one-row journal proves
the million configuration/cancelled history survives reopening, not populated
million-row performance. The [sizing review](../../operator-guide.md#larger-inventories-and-a-dedicated-storage-owner)
records stable indexes, synchronous full validation, temporary heap sets and
native full-batch I/O as remaining scale measurements. Preserve existing owners,
profiles, exhausted capacity and liabilities; full Miner publication/serving and
provider deletion/billing remain open. No live write/provider call, payment,
deployment, version/commit/publication, full CI, sibling edit or cleanup occurs.

## Miner follow-up and indexed setup recovery — 2026-10-03

The [local record](local/2026-10-03-publish-prepare-01/summary.json) separates
read-only Miner feedback from actual signed PocketIC setup/recovery. Miner local
HEAD is `3354dfc6b9fe791884ec69e8dd344de313b36940`; its clean feedback records
679 emitted identities, 675 distinct blobs, 221,173,950 bytes and 763 leaves.
No consumer build or provider experiment is rerun. Complete publication, public
serving, release/open-browser retention and retired-root reintroduction remain
open; the [consumer backlog](../../canic-parity.md#integration-feedback) adds these
and the immutable installation anchor's finite management-history horizon.

`publish-prepare` rechecks complete frozen inputs and independently authenticates
tenant/uploader before bounded indexed admission/preparation. Resume binds the
original signed claims and never resubmits either claimed update; it may send only
a previously unclaimed preparation. Real local HTTP fault transport drops IC
admission/preparation replies: exact recovery and repeated inspection pass without
redispatch. Wrong roles, journal changes, revocation and partial claims refuse or
block. Existing capacity/discovery/restore and single-setup proxy regressions pass.
Native focused tests, strict CLI/test lint and formatting/whitespace checks pass.
No provider substitute or deployed provider response is used for this setup step.

Compile visibility/import, parser/style lint, missing test macro, bool conversion
and one rejected documentation patch remain recorded and corrected in
`.tmp/publish-prepare-01`. Exact signed local journals, requests, raw replies and
uncertain/refused outcomes survive privately with retained development artifacts.
No live network request, service mutation, provider call, deployment, payment,
version/commit/publication, full CI, sibling write or cleanup occurs. Existing live
owners, expired nominal links, browser profiles and physical/billing obligations
remain unchanged. These local results do not qualify full publication or serving.

## Service gap review and current-instance recovery — 2026-10-02

The [review record](local/2026-10-02-service-gap-review-01/summary.json) separates
source and local IC evidence. Source intent precedes three bounded public GitHub
reads at revision `98cbeb63aceaf3e1ae74d9dff64235445643bbdb`; reviewed Caffeine
deletion callbacks are root-only authorized prune/GC acknowledgements without
final object billing evidence. This establishes neither deployed deletion nor
cessation. The [gap matrix](../../service-gaps.md) names required provider evidence
and consumer-owned Canic/Toko actions. No downstream dependency is added.

Local recovery intent follows the maintainer's selection of ordinary current-instance
upgrades. Actual PocketIC qualifies operator-only activation after synchronous
fenced reopening, retained reservations/exposure/released liabilities/pending
gateway work, controller denial, expired history, management changes during the
await and old-heap snapshot refusal. Native core and memory/binding checks, the
standalone suite, declared/exported Candid and strict affected lint pass. The
affected Chromium lost-final-reply journey verifies and downloads through
stop/start/recovery without redispatch: two local PUTs and two GETs; physical and
billing bytes survive logical release. Gateway/verifier facts are labelled local
substitutes, never deployed Caffeine deletion/billing evidence.

Initial Candid mismatch, concurrent unfinished test compilation, denied loopback,
ordinary-traffic false fences, rejected duplicate exports and local build/lint/
selection/patch failures survive in `.tmp/service-gap-review-01`. The maintained
guard independently qualifies ambiguous active execution gaps before delegation;
upgrade-restored owners require explicit operator recovery. Intervening execution
invalidates history. No old-backup activation, provider retry, version/release/
commit/publication, deployment or build cleanup occurs. Owned local servers/browser
close; trial owners, private journals, failed preflights and unsigned profile survive.
Three earlier read-only provider observations remain in the separate live capture;
this review submits no live mutation. The required v1 schema/API/lifecycle change
joins a future minor release; current target bytes are Unreleased.

## Authenticated frozen-batch check — 2026-10-02

The [new record](deployed/2026-10-02-publish-check-01/summary.json) separates native
checks, actual signed PocketIC queries against frozen 0.7.0 Wasm and three
read-only mainnet service queries. Intents precede the selected live reads in
`.tmp/publish-check-01/live-read-intent.json`; no Caffeine provider call or
creation/funding/link/certificate/transfer/attestation/reference effect occurs.
This does not repeat either original paid transfer or alter a completed capture.

Unreleased `publish-check` reverifies complete batch snapshots and exact request
packets before bounded tenant capacity/discovery. It retains query intent,
arguments, replies, original IDs and explicit blockers; partial failures produce
no complete summary. The live owner reports both exact sample roots as
`DeletionPending`, zero remaining lifetime objects and zero unseen demand.
The checker correctly blocks `retired_root`, preserving continuing obligations.
Query signatures verify; independent Candid decode confirms scope, IDs, roots,
byte lengths, states and headroom. This is deployed service observation, not
new provider-behavior or billing qualification.

Affected native tests, unchanged-memory/cancellation/restore PocketIC checks,
strict CLI and standalone-test target lint, formatting and evidence hashes pass.
Initial directory-write, function-length and test-style failures survive, as does
the correctly refused non-ByteString project fixture before its ASCII correction.
A brief build-lock wait leads to host-visible PID/cwd checks; concurrent recovery/
dependency/host/test work is preserved. No full CI, sibling edit, release/version/
commit/publication, new donor debit, identity export or build cleanup occurs.
Serial publication/recovery and consumer serving remain open; see the
[operator recipe](../../operator-guide.md#check-a-frozen-batch-against-live-capacity)
and [consumer feedback](../../canic-parity.md#integration-feedback).

## Approved fresh 0.7.0 live journey — 2026-10-02

Intent precedes effects in `.tmp/trial-v070-live-01/intent.json`. The maintainer
explicitly approves the selected fresh owner and covered actions below 100T total
or 20T/day without repeat confirmation; this run stays within both limits.
One 1T creation amount plus 100M ledger fee, one new 5T/day nominal two-hour
payer link, two bounded uploads, four whole-file GETs, exact verifier attestations
and logical reference releases are selected. No automatic paid retries; retain
old owner, original failed browser history and all provider/billing obligations.

The original preserved build unexpectedly reports 0.6.1. Before tenant enrollment,
linkage or certificate exposure, verified zero activity and absent provider account
justify one explicitly recorded correction: rebuild at 0.7.0, independently
qualify compiled-release readback in PocketIC, then reinstall only the empty new
owner. Original installation, mismatch, correction intent and outcomes survive.
The new maintained release guard also rejects that stale artifact locally.
Neither the old owner nor a completed capture is rewritten.

The actual fresh owner is `5gsmp-viaaa-aaaak-qzhfa-cai`, namespace 2, with project
`d0770e45-29b6-4449-b499-961aea5daa47`. The new link allocates 1T internally
from payer to owner; donor creation debit and internal credit are counted separately.
Available balances are below 100T and the maintainer has been reminded.
The [completed live record](deployed/2026-10-02-trial-v070-live-01/summary.json)
qualifies both 1 KiB and ten-chunk 10 MiB samples: thirteen successful PUT
dispatches, four independently verified whole-file GETs, two accepted verifier
attestations and two exact logical releases. All bytes match original snapshots;
final service accounting retains 10,486,784 physical/liability bytes with zero
logical/reserved bytes. Two lifetime object slots are consumed; do not reset or
reinstall this owner to publish more files. The large body repeats one chunk
hash, so distinct consumer media and deduplication economics remain open.

Twenty-eight verified logical provider queries use 56 bounded transports.
Payer credit remains 3.0996T after link allocation; new owner provider credit
falls from 1T to 0.995381342792T. Gateway available credit falls, but its usage
counters remain zero despite successful transfer; meaning/freshness and itemized
billing are unqualified. Gross donor debit is 8.1002T, remaining total authority
91.8998T; internal allocation is not counted twice. Donor liquidity is
0.654505097235T, and the new canister has 0.474432286137T. These balances are
below 100T; the reminder is delivered without asking permission again.

Local syntax/path/budget/manifest-review failures remain separately retained;
corrected preparations and final observations succeed. Compiled-release guard,
targeted strict standalone-test lint, formatting and evidence hashes pass.
No automatic paid retry, full CI, version/release/commit/publication, sibling edit,
key export/default-identity change or build cleanup occurs. Owned browsers/servers
close and ephemeral password files are removed. Provider objects, accounts,
certificates, claims and liabilities survive; no deletion, billing cessation,
future retention or enforced provider-spending cap is established.

## Fresh 0.7.0 offline batch and trial preparation — 2026-10-02

Intent precedes preparation in `.tmp/trial-v070-preparation-01/intent.json`:
zero network requests/effects, bounded SDK/native invocations, frozen 0.7.0
source/artifacts and separate trial inputs. The
[preparation record](local/2026-10-02-trial-v070-preparation-01/summary.json)
retains exact hashes, independent declared-DID encode/decode and a complete
local-only installation/account-link candidate. New project/bucket selection
does not establish provider provisioning. The service principal is a local
stand-in; none of these candidate inputs may be deployed.

The Unreleased native `publish-inputs` command shares single-file validation and
streaming verification. Actual Caffeine SDK 1.1.2 preparations at 1 KiB and 10 MiB
produce eleven leaves and 10,486,784 verified snapshot bytes with zero network
requests. Targeted native/lint checks cover unique identities, common scope,
candidate aggregate capacity, metadata/body hashes, path refusal, partial failure
and overwrite refusal. Initial blocked local HTTP listeners, patch-selection
failures and lint failures remain; authorized loopback/final checks pass.
An initial private manifest includes its own unfinished output and fails verification;
the failed manifest/reason survive, and the corrected output-excluding manifest verifies.
No complete large transfer or remaining-live-capacity evidence is established.

The [fresh-trial proposal](../../standalone-trial.md#fresh-070-trial-proposal)
is reviewable: original trial roles/payer, one new 0.7.0 owner, up to 1.0001T gross
creation debit, new 5T/day nominal two-hour link, then small verified journey before
the 10 MiB file. Existing 100T spending approval persists; current fees, liquidity,
allocation and raw expiry need fresh observation before effects. The old link's
recorded nominal expiry has passed without qualifying server enforcement.
No live observation/effect occurs in this capture; preserve old owner/profile/
payment/link/exposure/liability records. Canic integration stays deferred and
[consumer feedback](../../canic-parity.md#integration-feedback) remains open.

## Configurable issuance — local implementation, 2026-10-02

The maintainer requests relaxing the 1 KiB cap for Miner integration. The
[local evidence](../configured-upload.json) records removal of the extra trial
envelope in favor of installed admission quotas, plus actual PocketIC certificate
issuance for a 10 MiB object and a second multi-chunk object. Quota, authority,
replay, rollback and restore checks remain. This is local IC implementation
evidence and read-only consumer feedback, not a deployed-provider probe or Miner
acceptance. No provider/account call, funding, deployment or old-claim replay
occurs. The public semantic/API cut requires a minor release. Original live
captures, owner configuration, payment balances and stopped obligations remain.
Initial local selection/lint failures are retained in `.tmp/configured-upload-01`.

## Approved 5T daily allowance comparison — 2026-10-02

Intent: `.tmp/trial-limit-01/intent.json`, before new queries/effects. The maintainer
explicitly approves changing only the existing linked-payer allowance **1T -> 5T**,
preserving raw expiry `1790953635603000000` (nominal 2026-10-02T15:07:15.603Z).
Reuse the exact unsigned candidate retained in the prior funding capture; sign
fresh only after exact payer/relationship/gateway and nominal expiry checks.
At most one update, four baseline and three post-update verified logical queries,
four transports/64 KiB each, 30-second query deadlines and 60-second update/final
observation bounds. No retry, attached cycles, new funds, certificate, upload,
owner/deployment or account-role switch. Changed bindings/terms/expired nominal
scope, unexpected usage or uncertain/wrong response stops and preserves the original
request. Keep the old certificate/403 journal and 1 KiB exposure; deletion and
billing cessation are not inferred. Gross donor debit remains 7.1001T, remaining
standing 100T authority 92.8999T. Original private payer key is used in place;
no password, global identity, sibling change, release/commit or build cleanup.

Outcome: the [completed allowance comparison](deployed/2026-10-02-trial-limit-01/summary.json)
records one successful original-payer update, request
`1d6698003bdf90619aa53a274b800db4aa09af2bf93f5e383c71b2db4558fc03`.
Readback confirms exactly 5T daily allowance and unchanged expiry. Payer credit
falls **5.0996T -> 4.0996T**, relationship period spend rises **0 -> 1T**, and
gateway available credit rises **0 -> raw 333333333333** with all usage zero.
A separately recorded read-only followup intent precedes one public owner-balance
query to reconcile this unexpected debit: owner now holds **1T Ledger credit**,
matching the debit, unlike its historical zero/Prepaid account. Other payer settings
and relationship fields remain unchanged. This is observed internal credit
allocation after the update, not a new donor transfer or proof of an upload bill.
Do not double-count it: gross donor debit stays 7.1001T, authority 92.8999T.

Eight logical queries use sixteen bounded mainnet-verified transports, including
the additional accounting read; the three planned post-update queries finish in
8.228 seconds. An initial local record verifier incorrectly requires a Result
from the gateway-list method; failure and corrected official-IDL decode are retained.
Delivery failures for verifier-script permissions and an overstrict prose assertion
are also retained; private permissions and structured delivery checks now pass.
No network retry, new funding, certificate, upload/download or owner change occurs.
The original stopped browser claim and service liability survive. Positive credit
does not qualify admission, an allocation algorithm/minimum or expiry enforcement.
Prepare a separately reviewed fresh-owner trial; do not replay the old claimed
transfer, reset its lifetime slot or erase continuing balances and obligations.

## Larger isolated payer balance — 2026-10-02

Intent: `.tmp/trial-funding-02/intent.json`, recorded before new requests. The
maintainer requests testing with more cycles under standing total 100T trial
authority. Propose one additional **10T gross** direct deposit to the original
Cashier-returned isolated subaccount and one exact payer notification. At unchanged
fees, payer balance would rise from 999.8B to 10.9996T, above the documented 10T
funding example. Gross donor total would be 13.0001T, leaving 86.9999T authorized.
Refresh payer/relationship/gateway, fee, donor/deposit balances and exact subaccount
before signing; verify each transfer/sweep/credit before the next effect.
The pre-query plan substitutes twenty verified SDK queries and zero CLI reads for
the initial combined twenty-four-query allowance (four transports, 64 KiB each,
30 seconds). Bound two public source lookups and a 60-second final observation
window. One transfer and notification only; uncertainty stops with original requests
retained. Keep daily limit/expiry unchanged and preserve the stopped upload's
certificate/403 claim and exposure. No new owner, certificate, upload retry, reset
or billing-cessation inference is included. Original keys and ephemeral credential
handling remain private; no build, release, sibling change or artifact cleanup.

Fresh donor balance is only 5.754605097235T, so the original 10T maximum proposal
cannot be funded. Before signing, `selected-proposal.json` reduces the experiment
to **4.1T gross**, leaving 1.654605097235T in the donor. At the observed unchanged
fees, it would credit 4.0998T and bring the original payer to **5.0996T**; gross
trial donor total would be 7.1001T, remaining authority 92.8999T. This larger balance
does not meet the example's 10T guidance or establish a minimum. No source-account
switch, daily-limit/expiry change or stopped-upload retry is proposed.

Outcome: the [larger funding summary](deployed/2026-10-02-trial-funding-02/summary.json)
records one fresh successful transfer and payer notification, exact signatures/
blocks/deltas/sweep/fees, emptied deposit and **5.0996T** balance in the same payer.
Gross donor debit totals **7.1001T**, remaining authority **92.8999T**; original
donor liquidity is 1.654605097235T. No automatic top-up settings or link terms
change. A public verified owner-account balance is zero with Prepaid debt target,
while the payer has 5.0996T Ledger credit; gateway credit/usage stay zero initially
and later. This is a funding/read comparison, not a second upload-admission test
or proof of the earlier refusal's cause. Nineteen verified logical queries use
thirty-eight query/verification transports; final window is under 60 seconds.
The failed web lookup is retained; source guidance comes from the prior fresh
immutable README. Original profile/certificate/403 history and 1 KiB liability
stay unchanged. Original keys remain private; ephemeral password removed.

At this capture's close, the unsigned proposal was daily allowance 1T -> 5T on the existing link,
expiry unchanged, no new funding, certificates or uploads. Official IDL encoding
and independent decoding agree; its request hash/targets/budgets are retained in
`daily-limit-proposal.json`. Exact provider term selection remains pending because
earlier approval kept the daily limit at 1T. No signature or mutation occurs for
that proposal in this funding run; the approved comparison above completes it.
Keep separate balances and continuing obligations; Canic feedback
is recorded without sibling work or upstream messages.

## Approved two-hour link and bounded transfer — 2026-10-02

Intent retained before network effects in `.tmp/trial-live-01/intent.json`.
The maintainer explicitly approves one fresh nominal two-hour expiry update of
the existing payer/owner relationship, preserving the raw daily limit 1T. The
standing total isolated-trial spending authority is 100T; gross donor debit is
3.0001T and remaining authority 96.9999T. No new funding is planned.
Refresh exact relationship/gateway/installation/roles before setup. At most one
link update, three service setup updates, one certificate and two serial streamed
gateway upload dispatches are proposed for the retained 1 KiB packet/profile;
at most two whole 1 KiB downloads follow. No automatic paid retry is allowed.
Twelve logical provider queries are bounded individually by four transports,
64 KiB each and 30 seconds. Changed bindings, expired permission, uncertainty,
incorrect readback or byte mismatch stop new effects. Keep the installed owner,
funded payer, provider relationship and original journals; local server/browser
close after use. Expiry enforcement, billing cessation and future retention are
not assumed. Outcome and immutable manifest follow in this run's capture.

Outcome: the [live summary](deployed/2026-10-02-trial-live-01/summary.json)
records one successful exact renewal, local tenant/admission/manifest setup and
one real mainnet-verified certificate. The SDK's single 7,456-byte streamed tree
PUT receives readable HTTP 403 for insufficient owner balance; no chunk, retry,
download, attestation or release follows. Original certificate/root/owner/project/
bucket independently match. The payer and relationship replies stay unchanged,
gateway credit/usage/spend zero. Owner-account inspection refuses the payer as
NotAuthorized; API-version read is empty, neither establishing the cause.
Nine verified Cashier logical queries use eighteen captured verification/query
transports; service reads and setup have their own retained journals. The existing
browser profile reopens with the exact certificate and responded 403 claim.
Service exposure and 1 KiB reserved/logical/physical/liability accounting remain;
they do not prove stored provider bytes or cessation. Gross donor debit stays
3.0001T, remaining 96.9999T; no new funds or reset occurs.

The fresh retained DFINITY README recommends 10T funding and illustrates a 5T
daily limit, without establishing a minimum or gateway credit algorithm. This is
source guidance, not the observed refusal's cause or authority to retry. Investigate
allocation before another reviewed trial. Local field-shape/parse/SDK-check failures,
timestamp correction and private-wrapper credential redaction are retained;
corrected checks repeat no paid effect. Servers/browsers close, temporary passwords
are removed, original keys remain private and current history remains owned.
Canic feedback records separate admission/refusal handling; adoption stays deferred.
An offline capture check initially matches its own PEM-marker source literal;
the retained corrected check scans actual multiline private-key material and passes.
No network request or effect repeats for that correction. Independent certificate,
tree/refusal, accounting, budget, original artifact and public/private record checks
pass, alongside evidence manifests, changelog structure and diff whitespace.

This is the entry point for every Caffeine investigation. The maintainer selected
independent qualification on 2026-09-29; direct provider cooperation is unavailable
as a planning assumption. Release acceptance depends on demonstrated behavior and
explicitly reviewed operating limits, not obtaining an answer from Caffeine.
Provider correspondence remains useful evidence if it becomes available.

## Recording rule

Before a probe, record its question, evidence class, source revision or deployment,
actor and owner/project/account bindings, exact requests, request/byte/time/cycle
budgets, stop conditions and expected cleanup. Distinguish zero attached cycles
from unknown provider charges. Paid trials require an explicitly selected isolated
account/installation and an authorized budget; this plan is not that authorization.

After each request, retain its outcome before issuing the next. Record failures,
timeouts, redirects, truncation and inconclusive results as well as success. Never
erase an uncertain operation or replay it merely to get a cleaner result. Record
raw response bytes where safe, hashes, timestamps, decoder/source versions,
observations, interpretation and what the result cannot prove. Secrets, upload
certificates and private account data belong in controlled storage; put a redacted
manifest and the accountable artifact location here, never credentials. Hashes
of locally recorded responses establish file integrity, not provider signatures.

Every run gets a new directory/ID and an index entry below. Completed artifacts
are not edited; corrections or repeated experiments get new linked entries.
An interrupted run with no summary stays incomplete. Update the capability table
and handoff after a material result, and mark conflicting/drifted evidence for
review instead of treating previous observations as permanent guarantees.
"Constant tracking" means this is part of every investigation, not an unattended
poller or recurring paid job. Routine reruns of the same local test need not copy
all logs here; link their maintained test and record material changed findings.

## Current operating decisions and experiments

### Gateway admission and credit review — 2026-10-02

Intent: `.tmp/gateway-admission-review-01/intent.json` investigates public Caffeine
source and maintained SDK behavior after the zero-credit expiry experiment.
Bound four primary-source lookups and four one-MiB/30-second source fetches; review
local retained sources and prepare offline checks where supported. Distinguish
gateway credit allocation from funded payer balance. No provider update, extension,
funding, object request or gateway impersonation is planned. A header named dry-run
is not proof of zero effects; qualify its source contract before any such probe.
Preserve the installed owner, funded payer and existing relationship obligations.

Outcome: [source and actual-service preparation](local/2026-10-02-gateway-admission-01/summary.json)
refreshes two public trees and two immutable blobs within the recorded fetch limits.
Caffeine SDK bytes are unchanged; DFINITY README differs only by a trailing newline.
Its architecture/protocol describes budget checking at gateway upload admission,
not a separate application credit-grant operation. No effect-free dry-run is
demonstrated. These source observations do not explain the deployed zero-credit
algorithm; zero gateway credit is not presumed empty payer balance or a mandatory
positive-credit preflight. Unhelpful searches and the provider repository's 404
remain retained alongside successful sources.

SDK/native tools prepare a known 1 KiB file using the exact actual-service carrier.
Independent decoding checks permission/manifest/browser/root and download/status
bindings, plus an unsigned operator tenant-enrollment candidate. Planned IDs are
not allocated. A one-slot IndexedDB row with those exact inputs survives browser
process restart at the recorded private profile/origin/database, without keys,
signatures or provider calls. The first probe creates the row then uses unsupported
`reopen`; the corrected `open` follow-up preserves the same history and failed run,
within a newly retained four-launch total limit. Signer is a setup-only facade;
actual signing, admission, streamed upload and provider behavior remain unqualified.

The concrete plan prepares one existing-link update to nominal two-hour expiry,
unchanged raw 1T daily limit, no new funds and one 1 KiB real trial. That provider
term change remains unsubmitted pending exact approval; prior approval included
zero extensions. It is a new trial proposal, not a repeat expiry experiment. Total
donor debit stays 3.0001T; 96.9999T remain under standing 100T spending authority.
Preserve the profile/capture, existing relationship, funded account and owner.
No Cargo build/full CI, release/commit, sibling edits or build cleanup occurs.

### Explicitly approved payment-link expiry experiment — 2026-10-02

Intent: `.tmp/trial-link-02/intent.json` records the maintainer's explicit "yes
please" to the concrete payer-to-owner Cashier link and 90-second expiry experiment.
This resolves the earlier automatic approval rejection for this exact mutation;
the old expired, unsubmitted request remains immutable and is never replayed.
Refresh absent relationship and exact gateway within two bounded queries. Prepare
fresh current-native inputs and retain/independently verify one payer-signed add
request with raw daily limit 1,000,000,000,000 and fresh absolute-nanosecond expiry
hypothesis. Submit once, then at most four relationship/explicit-gateway budget
queries before/after candidate expiry in a 180-second window. Each SDK query has
mainnet node-signature verification, no retries, four transports/64 KiB/30 seconds
maximum; update wait is bounded to 60 seconds. Refusal/drift/uncertainty/inconclusive
evidence stops new effects. No extension, funding, certificate or object request
is included. Preserve funded/link resources and uncertain obligations; neither
expiry nor raw daily limit proves a total spending cap or billing cessation.

Outcome: [the approved link](deployed/2026-10-02-trial-link-02/summary.json)
succeeds once at request
`7b00e6c905a6fd41033b6a543536e9dde31305d376cc9295c13d1424b995999c`.
Verified readback matches the exact payer/owner, raw 1T daily limit and raw expiry
`1790943900147000000` (candidate 2026-10-02T12:25:00.147Z). Provider creation time
is consistent with Unix nanoseconds within the retained preparation/read interval;
the returned expiry alone does not establish its interpretation or enforcement.
Relationship and budget responses before/after candidate expiry are byte-identical:
relationship remains visible, gateway available credit is zero, all usage is zero.
Owner recognition/linkage is observed, **expiry enforcement is inconclusive**.
Zero gateway credit does not mean the funded payer is empty, and visible relationship
metadata after its timestamp does not prove effective spending authority persists.

Six mainnet-verified logical queries use twelve bounded query/verification transport
calls with no SDK retries; one CLI update submission is retained, while physical
wire attempts are not measured. The observation window is 130.265 seconds from
preparation through final reply. A local check initially assumes creation time is
within 30 seconds of the later reply; it fails at 30.642 seconds. Corrected checks
use the actual retained preparation/read interval, not that invented threshold;
failure is retained and no request/effect repeats. No extension, donor debit,
funding, certificate or object request follows. Total donor debit remains 3.0001T,
remaining 96.9999T under standing 100T approval. Preserve the provider relationship,
funded payer and owner; billing cessation is not proved. Next: review actual
gateway credit allocation/admission and namespace/browser readiness, without
impersonating a gateway or extending this inconclusive probe. Canic stays deferred.

### Owner linkage, observed expiry and gateway membership — 2026-10-02

Intent: `.tmp/trial-link-01/intent.json` uses persistent 100T trial authority with
the installed owner and already funded isolated payer. No refill is planned.
Read gateway membership/absent relation; use frozen native CLI/current Cashier DID
for one payer-signed link with 1T raw daily limit and a 90-second absolute-nanosecond
expiry hypothesis. Retain the exact request/signature before one dispatch, then
read relationship and empty-unit budget before and after expiry within 180 seconds.
This is an expiry experiment, not a prior claim about units or enforced total cost.
Only a supported conclusive result permits a separately retained extension; stop
on uncertain updates or inconclusive authority/expiry. Operator sync may follow
only within installed bounds. No certificate, gateway object upload, new funds,
owner reset or cleanup is planned; keep all linked/funded obligations and evidence.

Outcome: the [readiness capture](deployed/2026-10-02-trial-link-01/summary.json)
finds one live gateway within installed bounds and an absent relationship.
Anonymous and mainnet-verified payer budget_check return NotAuthorized; explicit
gateway budget_get returns OwnerNotFound. These are typed baseline refusals,
not expiry evidence. Two primary source searches find no Cashier-specific unit
guidance. Automatic approval review rejects the link submission before process
launch because a persistent provider payment relationship is not clearly covered
by continuation plus the spending ceiling. The signed link expires unsubmitted;
no bypass/retry, add-link or extension occurs in that run. Subsequent explicit
approval and the separately captured experiment above resolve its mutation gate.

Unaffected work completes once: independently verified operator gateway sync stores
the exact sole gateway at sequence 1 with no pending sync; local owners stay
unfenced with no upload/read/funding activity. One shared standalone balance
inspection reads the exact installed payer's 999.8B ledger/total and zero prepaid/
promotional balances from Cashier. Actual inter-canister discovery and inspection
work; gateway origin/transport, namespace/project/bucket, expiry and object delivery
remain unqualified. There are five direct provider logical queries (four anonymous,
one signed), two shared-handler provider reads and one local status query. Signed
budget verification uses two bounded transport calls; physical CLI wire attempts
are not measured. No donor transfer, account link, certificate or object request
occurs. Total debit remains 3.0001T under standing 100T authority. Password files
are removed; funded resources and immutable private capture remain. Canic stays
deferred. The ad-hoc signed-budget helper's success log mislabels its method as
account-info; its saved request and result correctly identify budget_check.

### Authorized direct deposit and notification — 2026-10-02

Intent: `.tmp/trial-funding-01/intent.json` records the maintainer's explicit
approval of the reviewed 1T-gross direct deposit and one payer notification, plus
standing total trial expenditure authority up to 100T cycles. This supersedes
the earlier planning-only budget; do not request the same spending approval again.
Count the prior 2.0001T creation debit and reserve this 1T before signing. Refresh
fee, donor balance, provider-returned address and empty destination; retain the
unchanged reviewed memo/timestamp and one exact signed transfer before dispatch.
Inspect the exact ledger block, donor debit and deposit balance before the
isolated payer signs/submits one explicit-account notification with no attached
cycles. Capture credited/balance/sweep block or refusal before subsequent reads.
Inspect account/settings, remaining deposit and exact returned block. Never repeat
an uncertain transfer/notification or substitute a new timestamp. Temporary
password material is removed; existing candidate keys are not exported/imported.
Retain funded resources/obligations. The approved budget is not evidence of a
provider billing cap, credit, account authority or project acceptance.

Outcome: [funding and credit](deployed/2026-10-02-trial-funding-01/summary.json)
complete with one saved donor transfer at ledger block 16,749,490, exactly 1T gross.
The exact transaction, donor debit and deposit balance reconcile before one saved
payer notification. It credits 999.8B and returns sweep block 16,749,515; the exact
sweep/100M fee and zero remaining deposit match. Verified payer account/settings
queries confirm the correct new account, same balance, zero overdraft and no
target auto-refill; no settings mutation is needed. A local settings capture-name
collision refuses before dispatch and is retained; a new namespace succeeds.
No paid request repeats. Password files are removed, candidate keys stay private
and installed/funded resources remain owned. Gross total donor debit is 3.0001T
with 96.9999T remaining under the now-standing 100T total approval. Service burn
and provider credit are not counted again as new donor debits. Direct creation/
credit is observed for this account; replay/lost-response recovery, linkage/expiry,
gateway/project acceptance and uploads remain unqualified. Canic stays deferred.

### Selected payer funding review — 2026-10-02

Intent: `.tmp/trial-funding-review-01/intent.json` bounds six read-only logical
queries after actual standalone installation: current ledger fee, donor/payer
balances, the selected payer's provider-returned deposit subaccount, that address's
ledger balance and payer-signed account-info. Retain each result before continuing;
use the existing private candidate key with mainnet query-signature verification,
no retries or replacement root. Review one-hop versus staged funding against the
same returned address, fees and at most the proposed 1T gross provider allocation.
Encode exact unsigned candidates and recovery/stop rules; provider creation/credit
remain unproven. No transfer, notification, account mutation, certificate or upload
is authorized. Source review and signed query observations stay distinct from
persistent receipts. Stop on binding drift or uncertainty and retain failed data.

Outcome: the [review](deployed/2026-10-02-trial-funding-review-01/summary.json)
retains all six observations. Payer and returned-address ledger balances are zero;
mainnet-verified payer account-info is AccountNotFound. Returned subaccount is
unchanged and the fee is 100M cycles. Exact unsigned direct-transfer and explicit
notification candidates match independently generated current-DID encodings.
One direct transfer proposes 999.9B plus 100M fee, within 1T gross; an intermediate
payer ledger transfer adds a fee without qualifying creation/credit. One payer
notification would follow exact transaction reconciliation, subject to explicit
approval. Conditional 999.8B credit assumes one 100M sweep deduction and no others;
actual creation/credit/sweep/refund semantics remain unqualified. Initial outer-opt
and textual numeric-annotation failures remain; corrections precede signing/effects.
No transfer/update/account mutation, certificate or upload occurs. Retain funds
and original evidence on uncertain/refused results; no new deposit or notification
retry is implied. Canic and its consumer actions stay deferred.

### Authorized standalone installation — 2026-10-02

Intent: `.tmp/trial-install-02/intent.json` records explicit maintainer approval
to install frozen 0.6.0 on existing empty `4wyfo-qaaaa-aaaam-qjlpq-cai` with the
reviewed 565-byte init. Check controller, empty module and retained artifact hashes;
sign and independently inspect one exact management install_code request before
one submission. Gzip is a supported management transport encoding; decompression
must equal the approved Wasm byte for byte. Retain both encoding hashes and the
observed module hash. Use explicit install mode, no reinstall/upgrade or automatic
retry. Stop on uncertainty; inspect the original request and owner instead.
Read back module, compiled release, every configuration/role and fresh local
accounting. IC execution consumes the owner's existing allocation; no new cycles
transfer, Caffeine funding, account mutation, certificate or upload is authorized.
Remove temporary password files; retain installed resource and controlled evidence.

Outcome: [installation and readback](deployed/2026-10-02-trial-install-02/summary.json)
succeed from one saved submission. Tagged official CLI source establishes the
effective management destination; only routing metadata changes, preserving the
signature/request ID. The observed module hash matches the submitted gzip encoding,
whose decompression equals the approved raw Wasm. Every init field/role matches;
compiled release is 0.6.0, all local owners are unfenced, activity is zero and
anonymous configuration inspection returns typed Denied. Three local validation/
syntax failures remain with corrections; none repeats the installation or a
provider effect. Service queries are uncertified observations. Balance is
1,487,607,279,785 cycles and reported idle burn is 1,404,810,943/day; the pre/post
12,380,829,273-cycle delta includes installation, status and elapsed burn, not an
isolated price. Password files are removed, keys are not exported, and the resource
remains owned. No Caffeine call/funding, certificate or upload occurs; account/
project provisioning and bounded provider effects remain separately reviewable.

### Authorized isolated owner creation — 2026-10-02

Continuation intent: `.tmp/trial-create-02/intent.json` records the maintainer's
selection of canic-mainnet-recovered with supplied signer access, under the existing
one-owner/2T creation approval. Confirm its public principal matches the planned
controller, refresh balance/fee observations, and retain one explicit timestamped
signed cycles-ledger request before one submission. Password material stays in an
ephemeral private file and is removed after signing; it is never included in the
probe capture or logs. Stop on identity/budget/signing mismatch or uncertain result.
No repeated creation, installation, minting or provider effects are authorized.

The [completed creation](deployed/2026-10-02-trial-create-02/summary.json) returns
`4wyfo-qaaaa-aaaam-qjlpq-cai` at block 16,749,045 from one exact saved submission.
The recovered signer matches the original principal; independent signature,
request-ID, caller/controller/amount/timestamp checks pass. One local decoder
failure on a Node Buffer's backing offset is corrected with an exact-byte copy;
the signed request is unchanged and no extra paid request occurs. Both temporary
password files are removed; password material/exported keys are absent from the
capture, while signed envelopes stay private. Exact ledger block and account
delta agree on the 2.0001T debit. Authenticated status confirms the expected
controller, zero compute/memory allocation and no module, with observed balance
1,499,997,545,813 cycles and continuing maintainer-owned IC costs. Physical
transport attempt counts are unmeasured. No install, mint/top-up or Caffeine call.

The [offline installation packet](local/2026-10-02-trial-installation-01/summary.json)
then validates and independently decodes a 565-byte complete init carrier for the
actual created service using frozen 0.6.0 artifacts and proposed isolated roles.
Source/Wasm/DID/CLI/init hashes remain explicit. Namespace/project consistency is
not provider provisioning. Installation requires its own authority; the created
owner remains empty and account/funding/certificate/upload are unperformed.
Its [metadata clarification](local/2026-10-02-trial-installation-01/clarifications.json)
corrects one excess parent in the original creation-evidence pointer without
overwriting the summary or changing artifact/authority facts.

Intent: `.tmp/trial-create-01/intent.json` records the maintainer's explicit
authorization for one detached empty mainnet canister using canic-mainnet,
2T initial cycles and fees checked against the proposed 10T service allocation.
Inspect that identity's cycles-ledger balance, current fees and official CLI/
ledger behavior before submission; retain exact intent/request identity and one
creation outcome. Stop on insufficient balance, unsupported capture or uncertain
result; no repeated creation, mint/top-up, install or provider funding. Creation
does not qualify Caffeine. Retain any resulting principal/balance as continuing
maintainer-owned resources; no deletion/reset authority. Source/metadata observations
and the paid IC effect remain separate evidence classes.

The [preflight result](deployed/2026-10-02-trial-create-preflight-01/summary.json)
retains a failed local signer load before any dispatch: canic-mainnet needs a
password and the non-interactive CLI has no terminal. Public metadata still
matches the expected principal and default identity is unchanged. Two separate
anonymous cycles-ledger queries observe sufficient funds and a 100M fee;
raw account balance stays private. Current official DID arguments encode/decode
offline with explicit controller, zero compute allocation and an unsubmitted
sample timestamp. No creation is signed/sent, no principal/block exists and no
cycles are spent. Primary source distinguishes the ledger fee from creation
cost deducted from initial canister funding; remaining balance/subnet is not
observed. The tagged CLI source lookup fails and remains recorded. Signer access
is pending an explicitly supplied password-file path or maintainer local signing;
creation authorization persists without another permission question. No install,
mint/top-up, provider calls or external cleanup.

### Released standalone trial artifacts — 2026-10-02

Review intent: `.tmp/trial-release-01/plan.json` binds local preparation to released
0.6.0 source. Freeze source/Wasm/CLI/DID, encode the maintained trial template
against the current DID, and exercise offline CLI carrier validation plus actual
PocketIC installation/readback. Check the observed compiled release against the
retained release receipt. Local roles and service principals are substitutes;
their init bytes must never be deployed. No network/provider requests, real cycle
attachments, deployment, funding or live certificate/upload. Stop on source drift,
build contention, binding/readback failure; retain failed runs and all artifacts.

The [completed review](local/2026-10-02-trial-release-01/summary.json) freezes
released production source and artifacts separately from the modified test source.
The maintained trial template encodes through the declared DID, which equals the
host-exported interface; actual local installation reads back the exact envelope
and roles with release 0.6.0. Wrong-service installation preserves the original
owner, controller readback refuses, and restricted local assessment succeeds.
The focused case passes in 3.43 seconds, affected strict lint/formatting and
independent DID decodes pass. A separate offline candidate retains the proposed
roles with a local-only service stand-in; neither init file can be deployed.
Initial system-Node child-process EPERM and formatting failure remain recorded.
The original plan's zero network/install effect fields mean external/mainnet;
local PocketIC calls/installations are the explicit target and their physical
HTTP count is unmeasured, clarified in a separate retained record. No live effect,
new provider qualification or external cleanup. The receipt binds release files,
not Wasm; the packet records exact artifact hashes independently. Creation of the
isolated live owner remains separately authorized; no automatic install/funding.

### Trial configuration and account provisioning — 2026-10-02

The [retained preparation](local/2026-10-02-trial-provisioning-01/summary.json)
and [maintained configuration template](../../../canisters/standalone/trial/README.md)
prepare the restricted resource envelope and concrete private role/browser/provider
candidates. Two fresh local Ed25519 candidates have SDK/public-DER principal
agreement and a locally verified signature; private files stay 0600 under 0700,
without existing-key export, global identity import/default changes or network use.
The maintained CLI validates the template with an explicit local service stand-in;
independent current-DID decoding passes. Live service remains unset and these
local-only installation bytes must never be deployed. Reserve-only local allocation
prevents service cycle offers, not provider spending or continuing charges.

The retained Cashier interface advertises cycles-ledger deposit-subaccount/notify
and account settings/linkage. Corresponding query/notify/zero-overdraft/no-target
request candidates encode/decode offline. There is no advertised account-create
method; lazy creation, authorization and credit semantics are not inferred.
Primary IC documentation describes ledger transfers without arbitrary attached-cycle
calls; the historical DFINITY example describes account linkage and wallet funding.
A direct deposit route could avoid a wallet/proxy if its semantics are established.
Raw expiry, actual deposit address/fees and project/bucket acceptance remain open.
No upstream setup script, wallet, proxy or provider API implementation is added.

Initial ICP identity metadata fails on its external settings lock; a fresh
metadata-only invocation succeeds with the same default. First template encoding
fails on unquoted Candid service field; corrected local-02 validates. First
independent decode mistakenly supplies a filename rather than hex; fresh stdin
decode succeeds. Missing guessed local paths and unavailable web/code searches
remain recorded; no server behavior is inferred. Public cached documentation and
retained source are separate from fresh deployed evidence. No account-specific or
provider-object request, IC update, funding, deployment or live certificate occurs.
No external resources require cleanup; private candidate keys and build artifacts
remain. Full CI/release/version/commit/sibling work remains unauthorized.

#### Review intent

The subsequent [fresh-candidate Cashier queries](deployed/2026-10-02-trial-payer-probe-01/summary.json)
return a provider-supplied 32-byte deposit subaccount, anonymous account-info
NotAuthorized and signed account-info AccountNotFound. The signed SDK query
verifies node keys/reply against the mainnet root, using one query and one
read_state fetch, no root replacement or SDK retries. This reports current absence;
it does not create an account or establish future credit/mutation authorization.
Physical wire attempts/provider charges remain unmeasured. No notification,
ledger transfer, funding, account mutation, deployment or provider object request.
Raw signed requests, responses and generated authoritative DID bindings remain in
private `.tmp/trial-payer-probe-01`. Initial signed hex is JSON-quoted and fails
direct didc decoding; retain it and the failure, then decode a fresh hex-only
derivative without another network call. Typed responses also decode through
bindings generated from the retained DID. No external cleanup or global identity
import/change occurs; only the newly generated candidate key signs this query.

After completing the offline packet, start a separate fresh
`.tmp/trial-payer-probe-01` observation: at most two anonymous Cashier query
invocations, first cycles_ledger_deposit_subaccount_v1 with sender equal to the
new proposed isolated participant, then account_info_get_v1 for that same fresh
principal. Use retained DID, explicit mainnet root and icp-api.io; twenty seconds
and 64 KiB output per invocation, no configured command retries or redirects.
Retain the first result before the second; stop on transport/timeout/size failure.
No existing/private account is queried. No signer, cycles attachment, notification,
linkage, settings update, ledger transfer, deployment or provider object request.
These query observations do not select live roles or prove persisted account
creation, authorization, project provisioning, credit or enforced spending limits.
Physical wire attempts/provider charges are unmeasured. No external cleanup.

The deposit-subaccount query succeeds with a 32-byte result. Anonymous account-info
returns typed NotAuthorized(anonymous), retained without retry. Before proceeding,
extend this separate observation with one signed account-info query using only the
freshly generated participant identity. Use the retained Candid request and installed
SDK 5.4.0 with mainnet root/query-signature validation, no fetched replacement root,
zero SDK retries and at most four bounded HTTPS transport calls (30 seconds total,
64 KiB request/response each) for that query and its verification metadata. This
tests caller authorization, not credit, persistent creation or mutation authority.
Keep signed artifacts private; stop on refusal/limit/transport failure. No funds,
account linkage, notification, settings mutation or provider object traffic.

2026-10-02 trial configuration/provisioning review intent:
`.tmp/trial-provisioning-01` will retain local public identity metadata and existing
Cashier DID/source/account-deposit observations, plus bounded public documentation
lookups for account onboarding and cycles-ledger deposits. No identity switching,
private-key reading/export, account-specific query, provider mutation, deployment,
funding or upload. Reuse maintained offline encoders for the concrete installation
envelope and separately reviewed requests; missing live service/payer/project/raw
terms remain explicit, never replaced by fixture identities. Distinguish client
source, retained deployed interface and documentation from server guarantees.
Public source requests have no attached cycles; no live account/provider-object
request is planned. Retain unavailable lookups and exact source hashes.

### Same-origin HTTPS and public gateway metadata — 2026-10-02

The [anonymous deployed metadata](deployed/2026-10-02-gateway-stream-01/summary.json)
retains three configured curl invocations: verified TLS/HTTP2 for all, root HEAD
400 and tree/chunk preflight OPTIONS 200 with PUT/SDK headers advertised. No
namespace values, credentials, certificate bodies, object GET/PUT, account setup
or attached cycles are sent. Advertised X-Dry-Run has unknown semantics and is
unused. Browser-path negotiation, real application-origin behavior, authenticated
streaming, provisioning, replay economics and retention remain unqualified.
The [linked clarification](deployed/2026-10-02-gateway-stream-01/clarifications.json)
preserves the original summary: curl --head saves headers in its output file,
whose original hash is not a transferred entity-body hash; physical wire attempts
are not independently counted. Cookie values remain private and are not reused.
Raw outcomes, requests, hashes and feature inspection remain in
`.tmp/gateway-stream-review-01`; no external objects need cleanup.

The [HTTPS journey evidence](local/2026-10-02-https-journey-01/summary.json)
records the CLI-only HTTP2 feature gap and its explicit rustls/http2 fix without
dependency version changes. All four actual standalone/Chromium/native journeys
pass through one owned TLS/H2 origin serving the received upload bytes. Native
verification remains enabled with child-scoped Linux test roots; unrelated trust
reaches no HTTP GET. A correctly trusted REFUSED_STREAM read fails after one GET,
retaining its failed observation without a statement or implicit retransmission;
a separately started complete read then verifies successfully. The four owners
receive two PUTs each and seven total GETs. Corrupt data, pre-header upload reply
loss and withdrawal retain the existing accounting and exact-release behavior.

All 66 native CLI tests and strict affected CLI/standalone lint pass. The
[eight-case transport propagation](local/2026-10-02-https-journey-01/transport.json)
also passes with the new CA/leaf helper. Its initial sandbox-denied Chromium
startup remains in transport-01, before requests; fresh transport-02 is separate.
`.tmp/https-journey-01` retains requests, known fixture keys, public test roots,
artifacts, sources and failed observations. Owned resources close and temporary
TLS private keys are removed; builds/evidence remain. No paid effect, deployment,
full CI, minimum-toolchain rerun, commit/version mutation or sibling edit occurs.
Canic remains deferred; live target/browser/cleanup selection is still open.

#### Pre-effect intents and run progression

2026-10-02 gateway stream compatibility intent: `.tmp/gateway-stream-review-01`
records at most three anonymous requests to source-listed `https://blob.caffeine.ai`:
HEAD `/`, OPTIONS `/v1/blob-tree/` and OPTIONS `/v1/chunk/`. Preflights use synthetic
Origin `https://localhost:7443`, PUT and the pinned SDK's header names, without
project values, namespace/account identifiers, credentials or certificate bytes.
Bound each request to 15 seconds/64 KiB, no retries/redirect following; stop on
transport/limit/redirect failure. Retain every outcome before advancing. No upload,
object GET, IC call, payment, account setup or attached cycles. This anonymous
transport/CORS observation does not select live bindings or qualify streaming-body
acceptance, provisioning, replay economics or retention; no external cleanup.

2026-10-02 same-origin HTTPS rehearsal intent: `.tmp/https-journey-01/intent.json`
records replacement of the split local upload/read origins with one TLS HTTP/2
gateway. CLI-only dependency inspection shows missing HTTP/2; explicitly enable
its existing reqwest rustls/http2 features without changing versions or allocator.
Generate a fixture CA/leaf and pass the public CA through each native child's
SSL_CERT_FILE, preserving normal certificate verification and global environment.
Run the four existing fresh 1 KiB owner journeys (two PUT arrivals each, six GETs
total), plus a mismatched-root refusal before an HTTP GET. Retain that failed
observation separately from a newly authorized read with correct trust. No live
provider/account/IC effect or payment; normal current recovery/release/accounting
remains. Close owned local resources and remove ephemeral private TLS keys, keeping
public trust, build/evidence artifacts and all failures. Targeted validation only.

Before running the HTTPS journeys, extend the local negative cut to one HTTP/2
REFUSED_STREAM GET after correct TLS validation. The newly enabled native HTTP/2
client must fail without redispatch; retain its distinct failed observation before
the separately budgeted complete verifier read. This adds one owned GET (seven
across the four cases); the mismatched-root attempt reaches no HTTP handler.

Before closing this capture, rerun the owned connection-close transport matrix
with the new test CA/leaf helper: eight fresh HTTP/1.1/H2 browser cases, at most
three 1 KiB arrivals each, 10 seconds per case/90 seconds overall. Retain results
in `.tmp/https-journey-01/transport-01`; no provider or payment effects. This checks
the shared TLS helper's propagation; it does not repeat live metadata requests.

The sandbox refuses Chromium startup before any socket request in transport-01;
its plan and failed log remain. A fresh transport-02 uses the same local budgets
outside that restriction. No failed or completed capture is overwritten.

2026-10-02 browser replay repair intent: `.tmp/browser-replay-01/intent.json`
records a comparison of buffered fetch, explicit keepalive settings, XHR and
one-shot streaming bodies on owned HTTP/1.1 and TLS HTTP/2 gateways. Each fresh
browser context warms its connection, sends one 1 KiB PUT and loses the reply
before headers. Bound eight cases to three arrivals each, 10 seconds per case
and 90 seconds overall. Retain every arrival, outcome and source-review failure;
do not infer exactly-once behavior or provider economics from a candidate fix.
Up to four additional anonymous Chromium source GETs are bounded to 2 MiB and
20 seconds each without retries/redirects. Earlier web lookups include unavailable
tagged source pages. No live provider/IC/payment/deployment effects or identities
are involved. Close owned sockets/browser and remove generated temporary TLS
keys while retaining artifacts. Preserve the SDK's PUT wire contract and journal.

The first eight-case matrix warms navigation rather than the fetch credentials
pool: XHR repeats, but the first buffered fetch has one arrival. A distinct
eight-case correction warms the exact credentials-omit connection; buffered
fetch, keepalive:false and XHR repeat on both HTTP/1.1 and HTTP/2. Streams have
zero HTTP/1.1 arrivals and one HTTP/2 arrival in that complete-body connection
cut. Before adopting a streaming fix, a third fresh eight-case matrix replaces
the HTTP/2 connection loss with REFUSED_STREAM after body receipt (HTTP/1.1
retains its original reset). Same per-case/run limits; this is an intentionally
misbehaving local substitute to test replay behavior, not a real provider result.

Both complete-body HTTP/2 cuts yield one streaming PUT arrival and a retained
uncertain claim. A fourth fresh matrix closes immediately upon the first 1 KiB
data event, before request EOF, to check whether that apparent mitigation depends
on completion timing. Same eight cases, three-arrival/10-second ceilings; retain
its distinct plan/results before any production transport change.

The fourth matrix again has one streaming HTTP/2 arrival and an uncertain claim.
Repair intent: send each snapshotted SDK PUT body through an immediately closed
ReadableStream with duplex:half, require HTTPS and detect stream support before
issuance. Never fall back to buffered fetch. Rehearse the four installed standalone
journeys using an owned TLS HTTP/2 upload gateway, preserving the exact SDK wire
bytes and restoring the original pre-header reset. Native verifier/download use
a separate HTTP loopback origin serving those same received bytes (TLS/HTTP/2
provider support and same-origin deployed compatibility remain unqualified).
Also propagate through the existing ten local IC/browser gateway cases and
eight SDK-substitute cases, within their maintained request/time/byte limits.
Retain every initial failure and corrected run separately; no live effects or
provider guarantee are authorized. Browser instrumentation observes traffic
without request interception, which could rewrite streams into buffered bodies.

2026-10-02 standalone-trial review intent: after released 0.5.0, review the four
certificate blockers against current public source and retained Cashier interfaces,
and prepare a concrete bounded trial contract using the existing tools. The fresh
`2026-10-02-trial-source-01` run permits at most four anonymous official GitHub/npm
GETs, each at most 2 MiB and 30 seconds, with no retries or redirects. Up to four
explicit primary-source web opens review onboarding/spending guidance; retain
failed lookups and source-only limitations. Intent and runner hash are recorded
in `.tmp/standalone-trial-review-01`. No payer/service, financial ceiling or provider
authority is selected; no deployment, account change, certificate, payment,
gateway/object request or qualification override is authorized.

2026-10-02 pricing-unit review intent: make one top-level anonymous
`pricelist_get_v1(())` query against candidate Cashier
`72ch2-fiaaa-aaaar-qbsvq-cai`, using the retained public interface and mainnet
trust. A fresh `deployed/2026-10-02-pricing-units-01` directory records the exact
request, tool/interface hashes and outcome. Bound the process to 30 seconds plus
a five-second termination grace, and output files to 128 KiB each. ICP CLI internal
request/retry counts are not independently measured; no second command/fallback
is authorized. No payer/owner/account arguments, updates or attached cycles. This
public pricing observation cannot qualify enforcement or authorize provider effects.

2026-10-02 standalone-trial review outcome: the maintainer selects a 100T-cycle
total planning budget. The [restricted contract proposal](../../standalone-trial.md)
records one fresh trusted-owner/uploader journey, a 1 KiB body, proposed
10T service / 1T initial provider / 89T unallocated budget and unselected live
bindings. The existing offline SDK/native rehearsal passes at 1 KiB and its
10 MiB default; invalid sizes reject before output. A one-object/1 KiB candidate
validates and the complete host init independently encodes/decodes. The first
decoder command uses an unsupported flag and fails; corrected stdin decoding
and the failed attempt remain retained. No service is installed.

The fresh public-source run captures four requests under its actual 1 MiB reply
cap, a tighter bound than the initial review ceiling. Its available 0.4.16 runner
hash/version are retained; backend sources and npm 1.1.2 integrity match the prior
pins. Two explicit primary web lookups return cache misses. Retail credit help
is observed separately and supplies no cycles-account cap or retention guarantee.

The anonymous unit query outputs independently decodable `PricelistGetResult`
data reporting cycles as terminal internal currency, then the process fails
SIGXFSZ (exit 153). Its stdout is 1,240 bytes, stderr 43 bytes; the failing write
target is unknown, so the output-file bound cannot explain it as reply truncation.
Preserve the failed process and valid reported data separately. No retry/fallback,
complete successful command, independently established query authenticity or
provider enforcement follows. All four certificate facts remain false; no paid,
account-mutation, certificate, gateway/object or provider-download effect occurs.
See the [retained review](local/2026-10-02-standalone-trial-review-01/summary.json)
and [failed query record](deployed/2026-10-02-pricing-units-01/summary.json).

2026-10-01 isolated-trial plan intent: `.tmp/isolated-trial-plan-01/intent.txt`
bounds repository source review, an offline complete installation check and a
concrete trial sequence. Reuse the shared installation validator; no duplicate
host init/provider schema or local qualification override. Proposed service,
project, verifier and compiled release remain unauthenticated planned inputs.
No new network/account/object/payment probe, deployment or funded trial is
authorized; all four host facts stay false and existing captures stay immutable.

2026-10-01 isolated-trial plan outcome: native `installation-check` reuses the
shared complete validator over bounded one-value configuration Candid and explicit
planned service/project/verifier/release. Three targeted cases and strict affected
CLI lint pass; actual executable output preserves exact bytes and independently
decodes, with repeat refusal and no overwrite. An unquoted reserved field in the
independent textual fixture initially fails, leaving empty bytes and dependent
test failures; failed input/result and corrected encoding/checks are retained in
`.tmp/isolated-trial-plan-01`. No provider contract, init schema or qualification
override is added. The [sequence](../../operator-guide.md#isolated-uploaddownload-trial-plan)
requires exact selected identities/budgets, existing four facts and separate effect
authority before the paid transfer. No new network probe/account/payment/object or
external cleanup obligation occurs. Concurrent ic-memory work is preserved;
current dependency/runtime blocker CF-02 remains open, with no new Canic defect.

2026-10-01 trial-preflight intent: `.tmp/caffeine-trial-preflight-01/intent.txt`
bounds primary onboarding/spending review and anonymous current Cashier interface/
pricing observation. Use the known candidate Cashier with no payer/account/owner
arguments, at most one metadata and one selected public price query; no silent
method fallback or retries. Retain failures/limitations and distinguish public
figures from an enforced financial cap. Isolated trial bindings are requested,
but deployment, certificate, account changes and paid effects remain outside scope.

2026-10-01 trial-preflight outcome: the maintainer reports no existing isolated
trial canister or funded account. One anonymous metadata command and one public
`pricelist_v1` query against candidate Cashier `72ch2-fiaaa-aaaar-qbsvq-cai`
succeed with exact retained bytes. Native CLI internal HTTP request/retry counts
were not independently captured. The
[deployed public capture](deployed/2026-10-01-cashier-preflight-01/summary.json)
retains requests, replies, exits, hashes and limitations. A pinned guide lookup
returns cache miss; the retained historical guide describes requests per thousand,
while the live list labels them `M`. Units, caps, expiry and charges remain
unqualified; no account relationship, gateway or paid effect is probed.

Offline `account-link-inputs` separately encodes explicit proposed bindings and
positive raw terms without dispatch. Two codec cases match an independent `didc`
fixture; two CLI cases cover valid output and invalid/repeated/partial-output
refusal. Strict affected lint and core rustdoc pass after retained initial lint
failures are corrected. Actual executable output decodes independently and repeat
refuses without overwriting. Intent, commands, logs, sample artifacts and identities
remain in `.tmp/caffeine-trial-preflight-01`; sample identities/maximal terms are
local fixtures, not trial bindings. Host facts stay false and no funding,
deployment, account mutation or external cleanup obligation is created.

2026-10-01 upload-gate review intent: following release 0.4.16,
`.tmp/upload-gates-01/intent.txt` bounds a fresh anonymous public-source refresh
and review of the four certificate blockers. Distinguish established client facts,
unobserved server guarantees and local recovery choices for one minimal upload/
verified-download prototype. No gateway, account, payment or deployment action;
zero provider-effect budget. Retain failed/inconclusive lookups separately and
keep host facts false unless the evidence and operating contract justify them.

2026-10-01 upload-gate review outcome: the first public-source attempt fails on
sandbox transport and remains in
[public-source-01](runs/2026-10-01-public-source-01/summary.json). A separately
planned network-enabled capture
[public-source-02](runs/2026-10-01-public-source-02/summary.json) records four
anonymous GETs: official main `4ebf43c518c9fc7b090ed4d27677fea26d6b98f3`, byte-identical
Mixin/Storage and npm 1.1.2 with the retained integrity. The existing probe binary
reports 0.4.15; its exact source/binary identity is retained, not relabelled as a
0.4.16 build. Mops/server/account/gateway behavior is not refreshed. A primary
April-guide lookup returns cache miss; no guarantee is inferred from it.

The follow-up shared browser transfer uses the existing patched SDK and gateway
journal, with root/owner binding and fixed serial/no-retry policy. Six maintained
SDK scenarios pass over labelled certificate/gateway/store substitutes; their
[requests/results](local/2026-10-01-sdk-transfer-01/summary.json) retain preflight
refusals, exact owner/project/bucket, lost final response and verified local bytes.
The managed refusal journey passes (9.35 seconds), and all ten existing browser
scenarios pass (53.90 seconds). Initial fixture abort-timing regression and its
correction remain separate in `.tmp/upload-gates-01`. Owned local processes exit;
build/evidence remain. No provider writes, payments, account changes or external
cleanup obligations; no successful managed exposure or deployed byte observation.
See [the unresolved trial facts](../caffeine-upload-gates.json); these results do
not turn any host qualification flag on or qualify production browser persistence.

2026-10-01 managed browser intent: `.tmp/managed-browser-01/intent.txt` records
the pinned local SDK/browser/framework inputs and bounded trial. Connect actual
consumer admission/uploader preparation to the maintained managed certificate
endpoint without changing its false provider/recovery prerequisites. Require one
claimed issuance across competing tabs, no gateway traffic, exact uncertainty
through reload/rejection inspection, and explicit consumer cancellation/withdrawal.
All traffic stays on owned PocketIC/page origins, with zero deployed-provider or
payment budget. This is local browser/IC integration, not Caffeine qualification.

2026-10-01 managed browser outcome: the actual managed-host Chromium journey
passes in 9.25 seconds. One certificate update is refused, with zero gateway
requests; exact uncertainty survives reload/certified rejection without resending.
Explicit cancellation/withdrawal releases unexposed capacity, retaining cancelled
history through fenced restore. Two initial harness assertions wrongly expected
no consumer Reserved state and no global physical reservation accounting; both
fail before issuance and are retained separately with their corrections. All ten
earlier browser scenarios and both affected strict-lint targets pass. Intent,
separate logs/reports and source/artifact/package hashes remain in the fresh
`.tmp/managed-browser-01` capture. Local instances/browser processes exit; build
artifacts remain. No external objects, payment or cleanup obligations are created.
The four provider/recovery prerequisites stay false; see
[evidence](../core-primitives.md#managed-browser-setup--2026-10-01).

2026-10-01 managed operator regression: `.tmp/managed-operator-fix-01` records
the pre-run intent, isolated failing reproduction and corrected source/results.
Valid Candid at the 4 KiB transport ceiling can exhaust the separate decoding-work
budget; the old test incorrectly expected scope validation. Updated coverage proves
small skipped input reaches typed Binding, valid over-budget input traps and
4097-byte input rejects at ingress, with service/source stable bytes unchanged.
All four related managed operator journeys and affected strict lint pass.
Production limits and adapters are unchanged; no new Canic action is established.
These use the existing labelled local gateway/Cashier substitute, not deployed
Caffeine, with no provider request, payment or exposure. Earlier evidence remains
unchanged; see [handoff](../../status/current.md).

2026-10-01 published Canic adoption: pre-run intent and separate attempt logs are
retained in `.tmp/published-canic-49-01`. Registry Canic/core/macros 0.110.49 build
the managed artifact without source overrides. The same six focused certificate,
decoder, manifest, lifecycle rollback and Candid cases plus both affected strict
lint lanes pass. Initial sandbox loopback refusal and the stopped test attempt
are retained; the loopback-enabled repeat passes. Package/VCS, lock, source and
artifact hashes are captured. This closes
CF-01; it adds local framework/IC
evidence only. Four provider/recovery prerequisites remain false, with no successful
certificate exposure, provider request, paid effect or deployed Fleet. Full release
validation remains separate. See
[evidence](../core-primitives.md#published-canic-adoption--2026-10-01).

2026-10-01 managed hook adoption: pre-run `.tmp/local-canic-02/intent.txt` records
frozen framework source, zero provider/payment budget, bounded builds/checks,
refusal/decoder/lifecycle questions and instance cleanup. All six focused cases
and both affected strict-lint lanes pass against Canic `32da629d0214bf791541a9b3c1832dbef13ece29`.
The canonical certificate method's plain record matches standalone; actual issue
calls refuse missing prerequisites, wrong actors and restored owners without
exposure. Valid hostile Candid type/header envelopes, malformed/oversized input,
exact manifest boundaries and occupied lifecycle rollback are locally exercised.
Initial testkit-accessor compilation failure and corrected result are retained.
This is local framework/IC evidence, not successful certificate exposure or
deployed Caffeine behavior. Host facts remain false; no qualification override,
provider call, paid effect, live Fleet or sibling edit. AGENTS.md now requires
stored Canic feedback and delivery
reminders; CF-01 covers normal published dependency adoption. See
[evidence](../core-primitives.md#managed-certificate-and-decoding--2026-10-01).

2026-10-01 local Canic composition: the maintainer selected local development
while Canic release/deployment work continues separately. The pre-run intent in
`.tmp/local-canic-01/intent.txt` bounds offline metadata, one canonical artifact
build, one native harness build and two focused managed PocketIC cases, with no
provider requests or paid effects. Committed Canic `32da629d0214bf791541a9b3c1832dbef13ece29`
is frozen here; sibling sources stay read-only and only copied workspace locks
resolve path overrides. The canonical managed build and existing certificate/
admission/fenced-restore cases pass. Initial copy/output/cache preflight refusals
and corrected results are all retained there alongside source/artifact hashes.
This is local framework/substitute evidence, not adoption evidence for the new
hooks or deployed Caffeine behavior. The four certificate prerequisites remain
unqualified; no paid exposure is authorized. Instances drop; builds/capture stay.
See [handoff](../../status/current.md).

2026-10-01 native/browser handoff intent: emit the existing browser certificate
client's exact binding JSON alongside verified upload inputs, deriving its IDs,
root and opaque Candid permission from the maintained Rust DTO. No new JS schema,
signer, intent journal, certificate or gateway implementation. Execute actual
pinned Caffeine 1.1.2 preparation (maintained repository patch) on a 10 MiB local
file, pass its manifest/binding/body through the native CLI, and consume the
generated binding in the existing browser client with an explicitly in-memory
store that permits setup only. Rebuild preparation from the native snapshot and
original metadata; require the same root/length before any potential future use.
Global and explicit network transports throw, with no certificate issue/recovery
or gateway upload. At most three native invocations, two SDK preparations and
32 MiB local data; preserve no-clobber and corrupt-body failures in fresh
`/tmp/ic-blob-storage-browser-handoff-evidence-01`. Retain outputs/logs/source and
binary/package hashes; no deployed request, payment, attachment, private key file,
account or external cleanup. This is local SDK/native/client evidence, not a
production store, authorization, readiness or provider guarantee.

2026-10-01 native/browser handoff outcome: five affected CLI cases and final
strict CLI all-target lint pass. The first lint found a 102-line run function;
its log is retained, and snapshot publication now has a separate private helper
with the same verification/failure behavior. Actual pinned SDK preparation passes
through native conversion for ten 1 MiB chunks. The existing browser source client
accepts the generated full-width binding/opaque permission in Node, using only
setup/inspection with an in-memory substitute and deliberately non-IC trust bytes.
No signature/certificate/IC behavior is exercised. After the source changes,
snapshot preparation reproduces exactly the root, ten-MiB length and manifest.
Native repeat refuses with every completed file hash unchanged; corrupt source
retains private partial/failure with no body, Candid, browser binding or summary.
Three native invocations return 0/3/3, two SDK preparations and zero network calls.
Fresh [evidence](../core-primitives.md#native-browser-certificate-binding--2026-10-01)
retains commands/results, package/source/artifact bindings and failed/final logs.
No full CI, Chromium/PocketIC rerun, upstream refresh or provider experiment.
Known test identity lived in memory only; no key file or external cleanup.

2026-10-01 shared native download intent: extend the existing managed completion/
download journey with an offline-generated second retain, a deliberately dropped
reply around the actual signed update, exact receipt/status recovery without
redispatch, and another verified download after first-reference release. Generate
and submit the final release, distinguish local logical bytes from physical/billing
liabilities, and inspect both historical receipts/current liveness through fenced
restore. Retain uncertain artifacts unchanged, including a refused same-run submit.
Use fresh `/tmp/ic-blob-storage-shared-download-evidence-01`, at most 40 tenant CLI
plus two verifier invocations, seven bounded ten-byte local source GETs, thirty-second
command deadlines and one proxy retain update. Existing fixture exposure/content
remain substitutes; no qualification override, SDK refresh, deployed provider
request, payment or attachment. Preserve failures/commands/results/source hashes;
drop owned temporary keys/proxy/gateway/progress/instances and retain capture/build.
No external cleanup obligation or new live-effect authority.

2026-10-01 shared native download outcome: the extended existing managed journey
passes in 14.16 seconds with 37 total CLI invocations and seven local ten-byte
source GETs. Ten bytes is the client/expected-body bound; existing fault responses
include zero/nine/eleven bytes (sixty total source content bytes, largest eleven),
with no oversized body published. Generated second-reference retain is dispatched once through the
drop proxy; exact signed-query receipt/status recover success and liveness with
all uncertain artifacts unchanged. Same-run submission refuses. First release
refuses its download while the second delivers the identical verified file.
Final release changes local logical bytes from ten to zero, preserving ten physical
and ten liability bytes. Fenced restore preserves exact retain/release history,
inactive status, all stable bytes and both files; refused mutation/GETs grant no
provider deletion or billing cessation. Targeted managed-harness strict lint,
formatting, diff and draft checks pass; production code/dependencies are unchanged.
No repeated native unit/full CI/release gate or upstream/provider refresh. Fresh
capture is recorded in [evidence](../core-primitives.md#shared-native-reference-downloads--2026-10-01).
Owned keys/proxy/gateway/progress/instances dropped; evidence/build retained and
zero deployed provider requests/payments/attachments or external cleanup.

2026-10-01 offline reference input intent: generate first-reference download/status
files with the verified upload snapshot, and explicit retain/release command plus
read files from the exact saved permission. Reuse maintained core validators and
Candid; no allocation, signer, dispatch, expiry renewal or retry authority. Run
both existing signed setup journeys (32 CLI/eight local updates/16 MiB service
traffic each, 10 MiB standalone and ten-byte managed snapshots) with generated
unconfirmed-reference requests and zero source GETs. Extend the existing managed
ten-byte download journey to consume generated reference inputs and perform signed
release/receipt/status, including fenced restore (24 tenant CLI plus two verifier
invocations, six bounded local source GETs, thirty-second command deadlines).
Existing exposure/content remain labelled local substitutes; qualification facts
stay false. Fresh `/tmp/ic-blob-storage-reference-inputs-evidence-01` retains
commands, results, failures and final source/artifact bindings. No deployed provider
request, SDK refresh, payment or attachment. Drop owned keys/servers/progress;
retain evidence/build, with no external cleanup.
Three additional offline binary checks use a copied retained abc/text permission:
full-width retain inputs (exit 0), repeat refusal
(exit 3) and noncanonical reference refusal before claim (exit 2). No network,
body read, service/provider call or cleanup resource.

2026-10-01 offline reference input outcome: all 57 CLI and three probe-tool units,
native binary build and final strict CLI/harness lint pass. The first lint attempt
rejected constant `chunks_exact` in a fixture helper; its log is retained and the
maintained `as_chunks` form passes. Standalone and pinned managed setup journeys
pass in 7.61/8.47 seconds, 25 CLI invocations each, with generated first-reference
status/download refusals and zero source GETs. The existing managed download
journey passes in 11.92 seconds, 21 total invocations and six local substitute
GETs: generated inputs drive download, signed release, exact receipt and current
status; release prevents another fetch and fenced restore preserves inactive
status, receipt and all stable bytes. Logical release establishes neither provider
deletion nor billing cessation. Three actual offline binary smoke outcomes are
0/3/2. Fresh capture and hashes are recorded in
[evidence](../core-primitives.md#offline-reference-and-download-inputs--2026-10-01).
Owned temporary keys/servers/progress/instances dropped; no external cleanup,
deployed provider call, payment, attachment, SDK refresh or new qualification fact.

2026-10-01 verified upload snapshot outcome: all 54 CLI and three probe-tool units,
native binary build and strict CLI/harness lint pass. Existing standalone 10 MiB
and pinned managed ten-byte journeys now consume actual generated Candid and
verified snapshots and pass in 7.19/8.86 seconds (23 CLI invocations each). Source
edits leave snapshots unchanged; signed service-manifest verification succeeds.
Lost/pending recovery, cancellation and fenced accounting remain intact; all four
certificate blockers remain false host prerequisites. No exposure/completion or
provider qualification is supplied by tests. Three offline binary smoke results
retain corrupt output, unchanged partial evidence on repaired-source refusal and
a successful fresh snapshot. Capture `/tmp/ic-blob-storage-upload-snapshot-evidence-01`
contains 230 manifested files, seven logs and 22 final source/artifact bindings.
Zero provider requests/GETs/payments/attachments. Manifest JSON is a local Rust
fixture substitute in upstream format; no SDK/upstream/provider refresh. Owned
servers/proxies/progress/instances/PEMs dropped, builds and evidence retained;
no external cleanup. See
[evidence](../core-primitives.md#verified-native-upload-snapshots--2026-10-01).

2026-10-01 verified upload snapshot intent: extend the unreleased offline
`upload-inputs` contract to require a regular body file, stream it through the
maintained Caffeine root verifier and save those same buffers. Only complete
EOF/root verification and file sync may publish `body.bin` and the Candid inputs;
failed output stays private with no usable summary. Reuse the same local-file
verifier in signed `verify-upload`; no second hashing/tree/chunk/upload SDK.
Run the existing signed standalone (10 MiB) and pinned managed (ten-byte) setup
journeys using actual generated inputs, then verify the snapshot against the
service manifest after deliberately changing its original source. Manifest JSON
is an explicit local upstream-format substitute from the maintained Rust fixture,
not a fresh SDK/provider observation. Per host: at most 32 native invocations,
eight local service updates, 16 MiB service traffic, one bounded body snapshot,
thirty-second command deadlines and zero provider requests/GETs/payments/cycles.
Use fresh `/tmp/ic-blob-storage-upload-snapshot-evidence-01` children; retain
failures/logs/source bindings, drop owned temporary keys/proxies/servers and retain
evidence/build. Qualification facts remain false; no fixture exposure/completion.
Three additional offline smoke invocations use the retained independent abc/text
vector: corrupt three-byte body, repaired-source refusal of the existing partial
run and successful fresh snapshot. No service/provider requests; keep every result
and failed partial, with no keys or external cleanup.

2026-10-01 offline upload-input conversion outcome: four new native cases pass;
all 51 CLI and three existing probe-tool units, strict CLI all-target lint and
binary build pass. Three actual binary invocations emit exact Candid, preserve
all output hashes on refused repeat and reject changed metadata before claim.
Fresh `/tmp/ic-blob-storage-upload-inputs-evidence-01` retains 30 manifested files
and fourteen final source/artifact bindings, including failed compile/lint and
sandbox attempts. This is local conversion/vector evidence, not an upstream SDK
execution or provider qualification. Zero smoke service/provider requests or paid
effects; existing native regressions use owned local HTTP substitutes only. No key
or external cleanup obligation. See
[evidence](../core-primitives.md#offline-native-upload-inputs--2026-10-01).

2026-10-01 offline upload-input conversion intent: reuse the maintained Caffeine
prepared-manifest decoder and shared service validators; do not introduce hashing,
chunking or transfer logic. Qualify the native binary with the existing independent
Caffeine 1.1.2 abc/text vector, explicit full-width IDs, preserved input/output
hashes, no-clobber repeat and inconsistent-manifest refusal before output claim.
Three smoke invocations, inputs capped at 4 KiB binding/256 KiB manifest and ten
content bytes; zero service/provider requests, payments or attachments. Native
regressions use their existing local HTTP substitutes only. Retain each failed
compile/lint/sandbox attempt and final logs in a fresh capture; no provider behavior
or qualification refresh is implied.

2026-10-01 signed upload setup outcome: actual native tenant admission, uploader
preparation, exact recovery and tenant cancellation pass through standalone and
managed handlers in 6.66/7.74 seconds. Each uses twenty-one CLI invocations under
its pre-effect plan, preserving original uncertain artifacts through query recovery,
rejecting corrupt declarations before claim, releasing unexposed bytes and retaining
history under same-release restore fences. No fixture exposure/completion or
production host evidence override. All 47 native units, four affected core cases,
current CLI/host builds and final strict affected Clippy pass; targeted checks only.

Captures `/tmp/ic-blob-storage-upload-setup-evidence-01` and `-02` retain 92/111
manifested files. First managed enrollment failed before the native client because
the test used a hard-coded operator; then-current hashes/log/result are retained in
01 alongside successful standalone records. Corrected managed uses fresh 02 with
all development logs, exact commands and final input bindings. Owned local proxies,
gateways/progress/instances/temporary PEM dropped; build/evidence retained. Zero
provider GETs, deployed requests, payments or attachments and no external cleanup.
Read-only local Canic HEAD `70a0bc9a435a7695d8931a7c66c745576e678597` has the same
relevant macro files as pinned 0.110.48, retaining both generic issuance/decoder
gaps. No upstream/registry/provider refresh or sibling mutation. See
[evidence](../core-primitives.md#signed-native-upload-setup--2026-10-01).

2026-10-01 signed upload setup intent: make local admission/preparation/cancellation
usable through the native client before qualified certificate issuance. Reuse
maintained DTOs, handlers, pure validators and exact recovery decoders, with one
saved signed update per fresh run and no automatic polling/retry. Exercise the
actual standalone and supported managed service over loopback PocketIC: distinct
tenant/uploader identities, exact full-width permission/manifest, lost admission,
pending preparation, exact read-only recovery, cancellation and fenced restoration.
Per adapter budget: at most 32 CLI invocations, eight local updates, 16 MiB service
traffic, thirty seconds per command; zero provider GETs/requests/payments/attachments.
Preserve requests/results/public trust/failures and original uncertain artifacts;
drop owned proxies/gateways/progress/instances/temporary PEM, retain build/evidence.
No fixture exposure/completion hook, provider source refresh or paid trial.

2026-10-01 tenant download outcome: native tenant output uses the maintained
replicated descriptor/decoder, canonical provider path and shared streaming root
verifier. All 45 native units pass (0.62 seconds), actual signed managed
completion/download/release/restore passes (12.80 seconds), standalone
unconfirmed/restored refusal passes (5.15 seconds), and all three existing managed
verifier receipt/recovery regressions pass (12.06 seconds). Wrong scope/signer,
corrupt/short/long/redirect replies and run reuse never yield a new verified file;
released/fenced descriptors issue no GET. Download/refusal phases preserve complete
service stable bytes. This is local platform and labelled exposure/content evidence,
not a real upload, certificate, deployed availability or operational recovery claim.

Fresh captures `/tmp/ic-blob-storage-tenant-download-evidence-01`, `-02`, `-03`
retain twelve, thirty-one and 156 manifested files respectively. First failed
managed setup repeated exposure already performed by the fixture (zero GETs);
second asserted the wrong submission label after accepted completion (one GET).
Both failed attempts retain their then-current input hashes and logs. Final managed
uses fourteen CLI invocations/six GETs, standalone two/zero within their recorded
plans. Existing verifier regressions add three local GETs with temporary raw
captures and retained validation logs. Final source/artifact bindings and all
development failures remain in 03; manifests and hashes are recorded in
[evidence](../core-primitives.md#verified-native-tenant-downloads--2026-10-01).
Owned listeners/gateways/progress/instances/temporary PEM dropped, evidence/build
artifacts retained; zero deployed Caffeine requests, payments or provider cycle
attachments and no external cleanup. No provider/upstream source refresh occurred.
Next work is the real upload path's existing qualification/framework gates, not
an assumed free read or an unapproved live trial.

2026-10-01 tenant download implementation intent: prioritize a usable byte path.
Reuse the maintained replicated descriptor, canonical provider URL and streaming
Caffeine root verification rather than create another provider client. Add one
signed tenant descriptor update followed only on exact authenticated success by
one explicitly selected origin GET, with original headers/length/root, no redirect,
retry or content decoding. Save intent before each effect; retain incomplete bytes
privately and publish a usable file only after complete EOF/root verification.
Exercise native socket corruption/truncation/oversize/redirect/encoding cuts and
actual signed managed completion-to-download, release and restored refusal over
the existing labelled ten-byte exposure/content substitute; standalone can exercise
real unconfirmed/fenced refusal until qualified issuance exists. Per local journey:
at most 24 CLI invocations, six local source GETs, 10 MiB reply/download traffic,
thirty seconds per request; zero deployed provider requests/payments/attachments.
Retain fresh requests/results/public trust/raw local source/failure artifacts; clean
owned sockets/gateways/progress/instances/temporary PEM, retain build artifacts.
No upstream/provider source refresh is implied; unchanged maintained provider path
is locally exercised, not newly qualified. Production issuance/provider/recovery
prerequisites stay explicit. This does not authorize paid trials or sibling edits.

2026-10-01 passive funding assessment outcome: shared synchronous query and
signed native command report exact current local limits and mandatory missing
qualification/recovery/account-activity/spendability evidence. No reservation,
funding mutation or provider call is exposed. All 28 affected core funding units
and 42 native units pass; synthetic accepted/uncertain history is labelled model
evidence. Both actual signed IC journeys pass (standalone 6.19 seconds, managed
10.06 seconds) with 21 CLI invocations each, under the recorded 24-invocation
budget. Cashier stays stopped; occupied upload/funding/read/gateway facts and
complete service/source stable bytes remain unchanged during refusals and after
same-release fenced restore. Full-width proposals/optional target, scope/identity,
untrusted replies and rejected caller qualification flags are covered. Current
Candid types/modes, both existing signed status regressions, native/standalone/
supported managed builds and final strict affected/isolated Clippy pass.

The fresh 110-file capture at
`/tmp/ic-blob-storage-funding-assessment-evidence-01` retains 44 files per adapter
and 22 validation files, including all failed intermediate lint/compile attempts,
input hashes and exact commands. Immutable manifest SHA-256:
`f21accdda21a2d987624aea8e6515752af65855b919270f1168c179327c300fb`.
Neither failed identity-constructor compile attempt started a signed journey.
No PEM remains; owned local gateways/progress/instances/temporary inputs were
cleaned up and capture/build artifacts retained. Zero Cashier queries, deployed
Caffeine requests, payments or provider cycle attachments; no external cleanup.
No full CI/release validation, dependency/allocator change or upstream refresh.
Missing trusted production evidence acquisition, qualified dispatch and recovery
remain open independently of provider balances or local empty history. See
[assessment evidence](../core-primitives.md#passive-funding-preparation-assessment--2026-10-01).

2026-10-01 passive funding assessment intent: expose current preparation-policy
blockers through the same synchronous shared handler in standalone and managed
adapters, then inspect with the signed native client. Authenticate exact operator/
service/namespace/Cashier/payer and proposed operation/offer/optional target.
Provider qualification, recovery, complete account activity and spendability stay
unestablished by these hosts; do not infer them from balances, status, local clear
history or caller assertions. No reservation, dispatch, provider query or payment.
Exercise native local journal occupancy/retained identity/capacity/accounting and
actual signed IC scope/trust/fenced restoration; preserve occupied unrelated
owners and complete stable bytes. Per adapter journey: at most twenty-four CLI
queries, thirty-second deadlines, 256 KiB HTTP/4 KiB service replies; zero Cashier/
deployed Caffeine requests, payments or provider cycle attachments. Record plans,
requests/results/failures before advancing; retain fresh captures/public trust,
clean owned gateway/progress/instances/temporary PEM and keep build artifacts.
This extends the existing unreleased gateway batch after 0.4.12. It does not
authorize provider trials, sibling changes, operational unfencing or qualification.

2026-10-01 signed native gateway controls outcome: all three commands retain
canonical request/signed intent before one service update. Actual signed shared
journeys pass through standalone (13.77 seconds) and managed (14.70 seconds),
with thirty-nine CLI invocations and six local Cashier queries each, within the
recorded budgets. Exact cancellation/absence revocation acknowledgments stay
separate from dropped/pending results; repeat/partial claims do not send again.
Current signed status shows membership/pending work without settling original
uncertainty or authorizing retry. Invalid/oversized provider replies preserve
pending identity; busy/stale cancellation refuse. Unrelated occupied owners and
pending history survive same-release restoration; all controls refuse the fence
and full service/source memories stay unchanged during refusals. Actual wrong
signer and namespace/Cashier/payer refuse; malformed decisions/identity mismatch
fail before a durable claim.

Thirty-nine native units, CLI build/final strict CLI Clippy, both new signed
journeys, both existing standalone sync cases and final strict harness Clippy
pass. The initial harness lint rejected a long journey helper; it was split and
the failure is retained. Fresh `/tmp/ic-blob-storage-gateway-controls-evidence-01`
retains 204 files per adapter and validation inputs/logs/commands (420 files total)
with immutable SHA256SUMS. Signed raw requests, bounded Candid replies, explicit
mode intentions and command/result/status/outcome JSON remain; public root trust
is retained and PEM identities stay temporary. Owned proxy/gateway/progress/
instances/temporary inputs are closed or dropped, with no external cleanup.
No deployed Caffeine request, payment or attached provider cycles occurred; all
source replies are query-only substitutes, not provider qualification. Existing
released service artifacts were reused; no new schema/endpoint, allocator,
dependency, version mutation or full CI/release gate. See
[gateway control evidence](../core-primitives.md#signed-native-gateway-controls--2026-10-01).
Framework/provider/provenance/recovery and funding admission remain independent
gates; no upstream probe, message or sibling edit was made.

2026-10-01 signed native gateway controls intent: compose sync, exact pending-sync
cancellation and local gateway revocation through the existing shared service
handlers. Claim a fresh private directory and save canonical request, signed update
and scoped intent before one dispatch; retain acknowledged/pending/uncertain/typed
refusal separately, with no polling, retry or locally predicted sync identity.
Use signed status only as current inspection, not a retained revocation receipt or
permission to repeat an uncertain action. Run actual signed standalone and managed
journeys over the existing query-only Cashier substitute; exercise lost/pending
acknowledgments, partial claims, invalid replies, scope/caller refusals and occupied
same-release restore. Per journey: at most forty CLI invocations, twenty local
provider queries, thirty-second client deadlines, 256 KiB HTTP/4 KiB service replies
and bounded existing provider decoding. Zero deployed Caffeine requests, payments
or attached provider cycles. Retain plans/requests/results/failures in fresh capture
children, stop on unexpected requests/effects and close owned sockets/gateway/
progress/instances/temporary PEM. Public roots and evidence/build artifacts remain.
This continuation starts from released 0.4.12, with no provider source refresh or
qualification inferred from the local substitutes. No new deployment/paid authority.

2026-10-01 signed native account inspection outcome: actual native balance and
relationship journeys pass through standalone (7.03 seconds) and managed
(10.82 seconds) adapters, fifteen CLI invocations per journey within their
pre-effect budgets. Exact operator/account binding, arbitrary-width signed/Nat
relationship fields and independent reported balances survive. Missing accounts,
no reported relationship and provider errors remain observations. Invalid/oversized
source replies, signer/scope/trust refusals and fenced same-release restore retain
complete stable state without credit/payment/retry authority. Update trust failure
can follow execution and does not prove an unsent read. The first managed attempt
refused setup because the fixture helper assumed the old operator; it remains
retained, and helpers now bind the installed operator explicitly.

The transport review found ic-agent 0.49.2's implicit HTTP 429/503 retries despite
TCP retries zero. Public middleware now delegates to the existing no-retry client.
An owned socket counter regression proves one service query/update under each
backpressure response; the first fixture incorrectly counted separate trust-key
reads and its failure is retained. Thirty-five native units, CLI build/final strict
Clippy, both new signed journeys, existing standalone/managed signed clients and
occupied managed operator regression, and final strict harness Clippy pass.
Fresh roots `/tmp/ic-blob-storage-native-account-evidence-01` (62 files including
44 standalone captures and validation) and `-02` (44 managed captures) have
immutable manifests. All failed lint/unit/managed attempts are retained. Owned
instances/gateway/progress/sockets and temporary PEM were cleaned up; reports and
build artifacts remain. All Cashier reports are local query-only substitutes;
zero deployed requests/payments/provider cycle attachments and no external cleanup.
See [signed account evidence](../core-primitives.md#signed-native-account-inspection-and-bounded-transport--2026-10-01).
No full CI/release validation, publication, schema/version/dependency or allocator
change occurred. Complete account activity, credit and operational restart remain
unqualified.

2026-10-01 managed certificate framework review outcome: upstream main/HEAD and
peeled v0.110.48 resolve to `8d37c74c9a4457b9e2bd47ee883f98fd2889d63b`.
Four immutable source fetches succeeded; access/expansion/parser files exactly
match cached pinned registry 0.110.48. Source still lacks supported plain-record
Fleet refusal and work/type/header decoder hooks. Normal framework dispatch remains
intact; no workaround, upstream edit/message or redundant failed build follows.
The four metadata requests include successful GitHub discovery/refs, an unavailable
web registry opening and explicit registry HTTP 403/empty body; latest registry
version is unverified. Every failure/source response/header/stderr is retained in
`/tmp/ic-blob-storage-certificate-framework-review-01` with eighteen-file manifest
and exact comparison bindings. Read-only sibling revision/dirty state is recorded;
no sibling was altered. Zero Caffeine requests, payments, attached provider cycles
or cleanup obligations. See [source review](../core-primitives.md#managed-certificate-framework-review--2026-10-01).
Framework support remains separate from provider/provenance/recovery qualification.

2026-10-01 signed native account inspection intent: add one explicitly selected
balance or payment-relationship observation through the existing shared
blob_inspect_account service update. Preserve operator/service/namespace/Cashier/
payer binding, signed transport/root trust and bounded service reply validation;
do not recreate provider requests or send attached cycles. Native tooling submits
one read operation and waits only for that exact IC request, with a thirty-second
deadline and no redispatch/retry. Report balances, absence and provider errors as
observations, never credit, spendability or payment/retry authority. Exercise the
actual standalone and managed artifacts over their existing local query-only
source with distinct identities and explicit fixture account configuration.
Per journey: at most twenty-four CLI invocations and twenty-four local provider
queries, 256 KiB transport/4 KiB account replies, zero payment/attached provider
cycles/deployed Caffeine requests. Retain fresh canonical requests/responses and
refusals, including malformed/oversized replies and fenced restoration. Close
owned gateway/progress/instances/temporary identities; retain reports/artifacts.

2026-10-01 managed certificate framework review intent: inspect the pinned public
Canic endpoint contract, the read-only local checkout and current registry/upstream
metadata for supported plain-record rejection and bounded decoder hooks. Preserve
the Caffeine reply format, normal Fleet/activation/preflight/instrumentation and
single synchronous commit/reply; no internal-method classification or copied
framework dispatch. Source/registry observations are distinct from local IC and
deployed-provider evidence. At most four metadata requests and four source/archive
fetches, thirty-second deadlines and 2 MiB per response (source archives 4 MiB),
zero deployed Caffeine requests/attached provider cycles/payments. Retain exact
responses, revisions/hashes and failures in a fresh local review directory. Sibling
repositories remain read-only; no dependency change or paid trial follows merely
from finding a newer release. If support is absent, specify the precise supported
framework gate and continue useful work available in this repository.

2026-10-01 unresolved application restore outcome: all four new interruption cases
and both existing application cases pass in 57.45 seconds. Committed retain/release
effects remain distinct from absent application acknowledgments through either
upgrade order and repeated same-release restoration. Exact asset/payload/cleanup
bindings, tombstones, service receipts, liveness and ten logical/physical/liability
bytes survive. Fenced application mutation/recovery calls refuse without resolving
uncertainty; controller disclosure/recovery refuses. Tenant-scoped service queries
remain passive and both complete stable memories match before/after inspection
and refusals. Final strict affected Clippy passes; the initial missing-trait compile
failure is retained. The fresh 83-file report preserves all six cases and is bound
in [unresolved-outbox evidence](../core-primitives.md#managed-unresolved-outbox-restoration--2026-10-01).
Local instances/temporary resources were dropped; reports/build artifacts remain.
No provider GET/payment/deployed request, external object or cleanup obligation
was created. Application/exposure/completion are local substitutes; this closes
the local restore test gap, not operational restart, stale-snapshot activation,
production consumer serving or deployed provider qualification.

2026-10-01 unresolved application restore intent after 0.4.11: extend the existing
managed application substitute with committed retain/release effects whose callback
traps before its acknowledgment can persist. Keep a published fresh asset and a
cancelled reuse asset's exact unresolved outbox intent. Upgrade application first
and service first in separate fresh cases, then repeat same-release upgrades of
both owners. Verify exact asset/tombstone/pending histories, original service
receipts/current reference liveness, denied callers and all-owner fences. Refuse
registration, admission, cleanup, recovery and new uses without clearing fences
or fabricating acknowledgment. Passive service inspection uses explicit tenant
identity in PocketIC; it is not recovery performed by the fenced application.
The existing probe and configured ten-byte exposure/completion are local substitutes.
Per case: at most sixty-four explicit application/service updates, sixty-four
inspection queries, thirty-second client deadlines and 4 KiB replies; no hold,
provider GET, deployed request, attached provider cycles or payment. Retain exact
Candid in fresh report children and keep each attempt/log separately. Drop owned
instances/temporary resources and retain reports/build artifacts. No operational
restart, production Toko acceptance or external journal is introduced.

2026-10-01 standalone reference outcome: all three acknowledged/dropped/pending
journeys pass in 35.21 seconds against the actual standalone artifact. Four signed
intents per journey retain exact original commands and typed Unknown/Unconfirmed/
Fenced refusals. Refused receipt inspection leaves lost/pending outcomes unresolved;
repeat submission never resends. All four owner fences and the 10 MiB reservation/
physical/liability accounting survive restoration and passive inspection. Final
strict affected Clippy and artifact builds pass; earlier lint failures remain.
The fresh 82-file report contains twelve signed intents and raw replies, bound in
[standalone evidence](../core-primitives.md#standalone-signed-reference-submission-and-refused-inspection--2026-10-01).
Temporary identities and owned sockets/gateway/progress/instances were cleaned up;
reports/build artifacts remain. No provider GET/payment/deployed request or external
object was created. Production successful completion remains disabled here.

2026-10-01 managed application outcome: the initial outbox case passes in 12.14
seconds; both final outbox/publication-race cases pass in 20.06 seconds. Actual
tenant calls use maintained admission/reference/descriptor clients, exact original
intents and receipt recovery after committed-effect callback traps. Cancellation
wins before the delayed callback; new uses refuse and the other asset stays live
until its own release. Cleanup at capacity during suspension retains physical/
billing liabilities. A pending release recovers passively under a service mutation
fence; subsequent consumer restore preserves its own fence/history. Final strict
affected harness Clippy, consumer build and exact managed Candid comparison pass;
the initial lint failure remains. Fresh 15/29-file sets preserve both runs with
raw canonical Candid and immutable manifests in
[application evidence](../core-primitives.md#managed-application-outbox-and-publication-race--2026-10-01).
Local instances/temporary resources were dropped; no provider GET/payment/deployed
request, external object or outstanding external cleanup was created. This remains
a local application/exposure/completion substitute, not Toko, operational restart
or provider deletion/billing qualification. Next restore occupied application
state while its outbox is unresolved; no fence-clearing authority is inferred.

2026-10-01 managed application/outbox intent: install the existing bounded consumer
probe as an actual canister tenant beside the public Canic fixture. Reuse its
canonical replicated admission/reference/descriptor clients and its own local
durable asset/outbox record; add no production component or external journal.
Explicitly configure the service with the probe's maintained fixture project.
One fresh asset and one reuse asset share a ten-byte object over labelled local
exposure/completion, with distinct service, tenant, operator, uploader and verifier.
At most sixty-four explicit application/service updates and sixty-four inspection queries, thirty-second client
deadlines and 4 KiB client reply bounds; zero provider GETs, attached provider
cycles/payments or deployed requests. Check retained admission/retain/release
intents, callback traps, original receipt recovery without redispatch, atomic
tombstone/outbox, use guards, reserved cleanup capacity during suspension and
separate physical/billing liabilities. Restore both consumer and service within
the current release, preserving their fences/history without operational restart.
Save plans/exact Candid in fresh capture directories; retain any failed attempt.
Drop owned instances/temporary resources; retain reports and build artifacts.
The consumer is a local application substitute, not Toko or production acceptance.

2026-10-01 managed application publication-race intent: extend the same local
probe/managed-service journey with the probe's bounded post-descriptor hold.
Cancel the reuse asset while registration waits, release its reference, then
resume the delayed callback and require it to refuse publication. A new use of
the tombstoned asset must refuse. The fresh asset/reference remains live until
its own cancellation/release; subsequent restore preserves the race outcome.
Same local targets/sixty-four explicit application/service update and inspection
query budgets, plus the maintained hold's bound of 128 management `raw_rand`
calls; ten declared bytes,
zero provider GETs/attached cycles/payments/deployed requests. Capture this new
case and reruns in fresh children/parents; retain the first successful outbox
report unchanged. This probes application interleaving, not provider deletion.

2026-10-01 standalone reference intent: run native one-shot tenant submission
against the maintained standalone Wasm with real signed IC updates. Production
issuance/exposure remains disabled; do not inject a confirmed object or add a
completion hook. Use unknown, admitted/prepared and same-release fenced states.
Per journey: at most eight signed updates and forty CLI invocations, thirty-second
client deadlines, 256 KiB transport/4 KiB reference reply bounds, one owned
10 MiB local manifest/reservation, zero provider GETs/attached cycles/payments/
deployed requests. The shared fault proxy forwards one update, passing its typed
refusal or dropping/replacing its acknowledgment. Preserve pending/uncertain
outcomes when receipt inspection itself refuses; neither unknown/unconfirmed nor
an empty/partial claim authorizes resend. Retain all intent/result evidence in
fresh report children. Inspect all restore fences and complete stable bytes.
Stop/drop owned proxy/gateway/progress/instances and temporary identities, retaining
reports/build artifacts. This is local production-refusal evidence; successful
retain/release and provider guarantees remain separate acceptance work.

2026-09-30 managed tenant reference intent: add one-shot native
`submit-reference` over the maintained service contract and exercise signed
tenant retain/release beside signed verifier completion in the public Canic
fixture. Use distinct fixed test signers for tenant and verifier, one labelled
local exposure and one owned ten-byte source GET. Per journey: at most ten signed
updates and sixty CLI invocations, thirty-second client deadlines, 256 KiB
transport/4 KiB reference reply bounds, zero attached provider cycles/payments
and zero deployed Caffeine requests. Retain exact request/signed intent before
dispatch; inspect original receipts after acknowledged/dropped/pending replies.
Check partial/existing claim, caller/scope refusal, stored transition failure,
cleanup at capacity during suspension, historical success versus current liveness,
remaining physical/billing liabilities and passive fenced restoration. Reports
get fresh directories; failed/inconclusive runs remain. Stop/drop owned source,
proxy, gateway/progress and instances; retain reports and build artifacts. This
does not establish a production consumer outbox or deployed provider deletion.

2026-09-30 managed tenant reference outcome: all three new journeys pass in
33.45 seconds; all seven affected managed signed-client cases then pass in
73.13 seconds. Each tenant journey uses one local ten-byte GET and eight saved
signed update intents, including verifier completion, explicit historical replay
and inactive/fenced refusals. The fault proxy forwards exactly one reference
update; original receipt inspection resolves dropped/pending replies without
resend. Empty/existing claims and foreign scope/signers refuse. Stored failures
remain distinct; releases at capacity during suspension and replay retain dead
references and physical/billing liabilities. Same-release restore preserves
historical receipts, liveness and all-owner fences with stable memory unchanged
during reads/refusal. All twenty-nine native CLI unit cases, strict affected
Clippy/build, three existing verifier cases and one existing signed receipt/status
case pass. Initial lint and sandbox-socket failures remain separate retained logs.
The fresh 277-file report manifest and bound commands/artifacts are in
[tenant reference evidence](../core-primitives.md#managed-signed-tenant-reference-submission-and-cleanup--2026-09-30).
Signed request/argument/trust hashes and scope/budget records validate; PEM keys
are temporary. Owned sources, proxies, gateway/progress and instances were closed/
dropped; reports/build artifacts remain. No deployed provider request, payment,
external object or outstanding external cleanup was created. Production consumer
coordination and real deletion/billing cessation remain open.

2026-09-30 managed verifier intent: exercise maintained `observe-upload`,
`submit-attestation` and `upload-attestation` against the public Canic fixture.
Install the fixed signing identity solely as verifier, with distinct tenant,
uploader and operator. An operator-only labelled fixture exposure hook prepares
the phase; it issues no certificate and supplies no production qualification.
Reuse the existing native HTTP fault proxy and an owned loopback ten-byte source.
Per journey: at most one source GET, one signed update and sixteen CLI invocations;
30-second client deadlines, 256 KiB service replies, ten-byte content budget,
zero attached provider cycles/payments and no deployed Caffeine contact. Check
unexposed/foreign-verifier refusal before GET, observation persistence before
effects, acknowledged/dropped/pending replies, exact receipt recovery without
resend and fenced same-release restoration. Save each initial outcome in a fresh
local report directory; never reuse an uncertain submission. Stop/drop sockets,
proxy, gateway/progress and owned instances, retaining reports and build artifacts.

2026-09-30 managed verifier outcome: three new cases pass initially in 33.37
seconds; after completing raw response/tamper/recovery capture, all four managed
signed-client cases pass in 41.86 seconds. Each verifier journey issues one local
source GET and one signed IC update (zero deployed provider requests/payments),
then resolves acknowledged/dropped/pending replies by exact receipt inspection
without resend. Foreign verifier, damaged observation and fenced new fetches
refuse. The shared proxy/source extraction also passes all three existing native
fixture cases in 12.63 seconds. Retained evidence sets 01/02 contain 88/103 files,
bound by immutable SHA256SUMS manifests; the first set predates raw response and
additional refusal/recovery captures. No record was overwritten. Logs, failed
compile/lint attempts, complete bindings/budgets and capture hashes are in
[managed verifier evidence](../core-primitives.md#managed-signed-verifier-observation-submission-and-recovery--2026-09-30).
Owned local resources were closed/dropped; there are no new external objects or
cleanup obligations. Local exposure/bytes and trusted metadata completion do not
qualify deployed provider behavior, future retention, billing cessation,
production verifier provenance or concrete consumer/outbox acceptance.

2026-09-30 managed signed-client intent: run the maintained native CLI against
the public Canic application-only PocketIC fixture. Bind the fixed test signing
identity explicitly as operator and uploader; pin its undelegated application
subnet key through the independently owned local control API. Query status,
funding/upload history and certificate assessment beside one ten-byte reservation;
verify local file bytes against its signed original manifest and refuse changed,
short or long files, without claiming provider availability or completion;
check identity/scope/trust refusal, exact saved permission and all-owner fenced
same-release restore. At most 32 signed CLI invocations, each bounded by the
maintained 30-second client timeout and 256 KiB response budget. No certificate
issuance, provider requests, payments or deployed Caffeine contact; attached
provider cycles are zero. Stop progress/drop instances and temporary test files;
retain every attempt in core evidence. Local signing is not production Fleet or
provider qualification.

2026-09-30 managed signed-client outcome: the final managed suite passes all ten
cases in 114.87 seconds, and the existing signed standalone status/history case
passes in 6.24 seconds. The new journey invokes the maintained CLI 22 times,
including refusals before transport, against one prepared ten-byte reservation.
Pinned local subnet trust verifies queries without an NNS or a signature bypass.
Wrong identity/scope/trust, changed permission and corrupt/short/long files refuse;
historical inventory/byte checks remain passive after all-owner fenced restore.
No provider request, exposure, completion or payment is produced. Three failed
fixture/URL attempts and an earlier lint failure remain distinct captures; the
subsequent successful narrower run predates the added byte cases. Full commands,
artifact/source/log hashes and limitations are in
[managed signed-client evidence](../core-primitives.md#managed-signed-client-and-local-byte-verification--2026-09-30).
Owned progress/gateway/instances and temporary input files were cleaned up; there
are no new provider objects or external cleanup obligations. Production Fleet,
verifier submission and concrete consumer/deployed-provider acceptance remain open.

2026-09-30 managed adapter intent after 0.4.10: extend the existing Canic
composition suite with certificate refusal and gateway/account endpoints over
shared workflows. Use one owned local managed operator installation, one prepared
ten-byte reservation and the existing query-only Cashier substitute. Provision the
selected source through the fresh empty fixture's authenticated application carrier
before creating any tenant or obligation. At most eight gateway-list and eight
account reads per journey, 30-second call timeout, 64 KiB gateway and 4 KiB account
reply budgets, no attached cycles, payments or deployed provider contact. Check
operator/scope denial, exact failed-sync cancellation, passive account observations
and retained pending work/all-owner fences after same-release upgrade. Stop/drop
owned instances; retain every attempt in the managed core evidence. This is generic
adapter composition over existing labelled source modes, not new deployed Caffeine
qualification or authority to repeat an uncertain paid effect.

2026-09-30 managed adapter outcome: all nine Canic composition cases pass in
107.47 seconds. The new operator journey executes five gateway-list and five
account queries against the existing owned query-only substitute, each with zero
attached cycles. It preserves failed-sync identity, exact cancellation and passive
reports beside a prepared ten-byte reservation, then restores all owners fenced.
Certificate assessment reports existing blockers without exposure. A compiler/
pinned-source finding changed the implementation plan: public Canic default Fleet
guards require Result and cannot preserve Caffeine's plain certificate reply.
The attempted issuance adapter was removed; no internal-endpoint workaround was
introduced. Retained attempts, commands, hashes and limits are in
[managed operator evidence](../core-primitives.md#managed-certificate-assessment-and-operator-queries--2026-09-30).
This records generic managed composition over maintained local wire substitutes,
not new deployed Caffeine behavior. Owned instances were dropped; no provider
objects, balance changes or outstanding external cleanup were created.

2026-09-30 standalone lifecycle/acceptance review intent: inspect maintained
host/store restoration and reconcile acceptance/parity claims with current local
evidence. One owned standalone PocketIC canister, one admitted/prepared 10 MiB
manifest, one stop/start, one rejected configuration-bearing upgrade and two
same-release empty-argument upgrades. Compare complete installation, retained
admission/manifest and all four owner fences; after restoration attempt only
manifest preparation, gateway revocation and account inspection, expecting typed
Fenced refusals before any source call. No snapshot load, provider request,
funding, deployment or unfence. Drop the owned local instance; record material
outcomes and any failure as standalone-lifecycle-01. Local current-state evidence
does not qualify active recovery or Canic parity.

2026-09-30 native reference inspection intent: exercise separate receipt and
current-status queries on one owned standalone installation and one populated
durable storage fixture through signed local PocketIC HTTP. Fixed test tenant
identity, explicit root DER, exact original saved Candid requests; at most 32
native invocations, each one query with a 30-second deadline, 256 KiB HTTP ceiling
and 4 KiB input/reply limits. Check unknown/unconfirmed refusal, caller/scope/trust
binding, absent and recorded results, retain then release, local liveness,
settlement and passive fenced upgrade. The populated fixture uses its installed
two-object budget and three receipts per object, preserving final-release slots.
Completion, deletion and settlement use
labelled fixture facts, with no provider requests, payment or retry. Stop owned
HTTP instances and drop canisters and temporary files; preserve failed/inconclusive
outcomes before correction and retain material evidence as reference-native-01.

2026-09-30 funding CLI validation follow-up: the broader native CLI suite's
existing download substitute could not bind loopback in the sandbox. Retain
funding-native-01 before a loopback-enabled rerun of that unchanged suite. No
deployed provider contact or funding occurs; temporary local servers/files are
dropped. The funding-outcome PocketIC intent below remains within its budget.

2026-09-30 exact funding-outcome CLI intent: inspect one owned standalone
installation and one populated durable storage fixture through signed PocketIC
HTTP queries as fixed test operators with explicit local root trust. At most
sixteen native invocations total; each makes at most one query with a 30-second
deadline, 256 KiB HTTP ceiling and 4 KiB decoded outcome limit. Check absent
history, original amounts/optional target, partial refund, uncertain attachment,
scope/identity refusals and passive fenced same-release inspection. Fixture
funding phases are controlled substitutes, never real payments or evidence of
Caffeine credit. No provider requests or attached cycles. Stop local HTTP
instances and drop owned canisters and temporary files. Retain any failed run
before correction; record material outcomes as funding-outcome-cli-01.

2026-09-30 upload-history follow-up intent: retain the first run's setup Capacity
refusal, then declare a 65-chunk global/tenant fixture budget for its 65 retained
one-byte declarations. Rerun the native journey alone as `upload-history-cli-02`
under the original twelve-invocation budget. Production limits/accounting and
provider facts stay unchanged; no provider requests or paid effects.

2026-09-30 local upload-history tooling intent: inspect one owned PocketIC
standalone installation with a fixed test operator identity and explicit local
root trust. Retain 64 cancelled one-byte declarations followed by one full-width
active identity to exercise empty filtered progress, explicit pagination and
same-release fenced inspection. At most twelve native invocations, one query
per invocation, 30-second deadline, 256 KiB HTTP and 64 KiB decoded reply limits;
no manifests, file/provider transfer, certificate updates or paid effects.
Saved cursor scope must reject before identity/transport access. Stop the local
HTTP instance and drop owned canisters/temporary input files. This tests local
inventory observations, not deployed provider state or complete recovery freshness.

2026-09-30 certificate CLI follow-up intent: retain the first local run's final
incorrect state assertion, then compare the complete post-revocation record
before and after upgrade. Repeat only the maintained native journey as a new
`certificate-cli-02` run within the original twelve-attempt local budget.
Authentication, trust, state and all provider qualification facts remain unchanged;
no provider calls or paid effects. Retain the first failure below.

2026-09-30 native certificate-assessment intent: exercise the current standalone
assessment through a real signed HTTP query using a fixed local uploader identity
and explicitly trusted PocketIC root. One owned local installation, one 10 MiB
manifest and at most twelve query attempts; each native command is bounded to
30 seconds, a 256 KiB HTTP response and 4 KiB decoded reply. Check full permission
binding, missing preparation, qualification blockers, revocation and upgrade
fencing without changing stable memory. No certificate update, Caffeine request,
provider transfer, attached payment or paid effect. Temporary test credentials
and input files are deleted with their directory; stop the local HTTP instance
and drop the owned canister. Retain failures and material conclusions below.

2026-09-30 maintainer scope decision after the recovery design review: keep one
authoritative storage owner and local durable journals until a clear use case
justifies more machinery. The external journal/controller proposal is deferred;
it does not change the synchronous certificate boundary or enable issuance.
Current-state durability and receipt/lifecycle recovery take priority. Old snapshot
loads remain unsupported for operation and require complete independent
reconciliation before activation. This decision makes no provider/recovery
qualification claim and does not close the outstanding backup/restore requirement.
The historical review artifact below remains unchanged. No new probe or effect.

2026-09-30 recovery authority design intent: inspect the current certificate,
exposure, funding, gateway and restore boundaries plus retained provider artifacts;
review the official IC System API, snapshot and asynchronous-call specifications.
This is source/design evidence, not a provider experiment. At most six public
documentation reads, no account, gateway or paid calls. Determine whether a
separate asynchronous witness can protect the existing synchronous certificate
reply, what complete inventory must survive, and which component must own the
provider identity. Local adversarial models may exercise permit replay and partial
inventories; they cannot establish an external authority or deployed behavior.
Retain references, conclusions, limitations and any failed review here. No external
cleanup is expected; production qualification facts remain false.

2026-09-30 whole-canister rollback intent: use PocketIC 16 through ic-testkit to
take/load actual management snapshots of the current standalone and durable
storage fixture. Fixed local principals only; compare stop/start, same-release
upgrade and rollback across admission, revocation and simulated exposure. At most
two isolated canisters, three snapshots and four loads; at most two 10 MiB local
manifests and no file/provider transfer. Record whether saved heap owners bypass
post-upgrade fencing and whether later history disappears. The fixture's qualified
exposure facts remain substitutes. No live network, credentials, attached payment
or Caffeine requests; no external cleanup. Local snapshot IDs are deleted and
the owned test instances dropped. Stop on unexpected platform failures and retain
the failed outcome here before adjusting the scenario.

2026-09-30 certificate/exposure host integration intent: review the retained
official Mixin/Storage and SDK evidence, then capture current public source in a
fresh `2026-09-30-public-source-01` run using the maintained `public-source`
command. Four anonymous official GitHub/raw/npm GETs maximum, 1 MiB and 30 seconds
per request, no redirects/retries, accounts, gateway effects or attached cycles;
stop and retain any failure. Compare hashes before changing host assumptions.
Local PocketIC work will exercise the real standalone certificate ingress and
read-only assessment with fixed test principals, prepared manifests, malformed
requests, refusal, revocation and restoration. No fixture evidence may enable the
production host: unqualified provider/recovery facts remain blockers. There is no
live upload or payment budget, and no external cleanup is expected.

2026-09-30 planned local verifier submission evidence: extend the signed PocketIC
observation journey with the native one-shot `submit-attestation` command. Retain
the existing fixture owner/project, fixed test verifier identity and ten-byte local
HTTP substitute. Check missing/failed/changed artifacts before signing, intent and
signed-envelope persistence before one update, no repeat submission, and historical
receipt recovery. A local HTTP fault endpoint will exercise uncertain transport
without a deployed provider. Bound each command to 30 seconds and one update;
no live Caffeine, attached payment or provider cleanup is involved. Test artifacts
live in temporary controlled directories; maintained scenarios and material outcomes
are retained here. These tests cannot establish deployed provider availability,
retention, deletion or billing behavior.

On 2026-09-29 the maintainer explicitly selected a configured external verifier
trust model. The verifier must independently retrieve complete content from the
installation's owner/project binding and verify original metadata/root/length.
Its signed canister call attests observed content availability, not guaranteed
future retention, billing cessation or paid-operation identity. The new receipt
path retains all physical/economic obligations and existing exposure/restore gates.
This is an explicit service trust decision, not newly discovered Caffeine behavior.

| Capability / question | Experiment and decisive evidence | Current limit until demonstrated |
| --- | --- | --- |
| Source/interface drift | Capture official revision, package metadata, source/Candid hashes; compare to the reviewed baseline before behavior changes | A matching interface/source is not proof of the deployed implementation |
| Actual-service browser preparation | [Unsigned packet/profile](local/2026-10-02-gateway-admission-01/summary.json) verifies one 1 KiB SDK/native packet and exact IndexedDB row through process restart at a retained origin/profile | Setup-only signer; actual admission, signing, gateway streaming and provider behavior remain open. Open existing history; do not reset missing or uncertain work |
| Live account and gateway inspection | [Standalone readiness](deployed/2026-10-02-trial-link-01/summary.json) observes shared gateway sync and exact funded payer balance; [approved link](deployed/2026-10-02-trial-link-02/summary.json) succeeds with exact readback and retained before/after budget queries | Discovery/balance inspection and exact payer linkage work for these bindings. Zero gateway credit gives no expiry-enforcement signal; project/bucket, gateway admission/delivery and billing cessation remain unqualified |
| Upload completion | Isolated known files spanning empty/single/multiple chunks; capture actual tree/chunk responses, interrupt the final response, then independently download and verify complete bytes/metadata against the expected root | Browser progress/hash alone cannot confirm completion. Verified reads establish observed content availability, not future retention or a trusted canister fact. Keep the current service completion gate until the evidence bridge and narrower semantics are implemented/reviewed |
| Resume and retry charges | One variable per trial: duplicate tree, duplicate chunk, interrupted chunk, completed root. Correlate request logs with isolated audit/account observations; wait through billing aggregation | Assume repeats can cost money. No automatic uncertain paid-effect retry; bounded manual disposition must preserve prior liability |
| Funding | Exact offered/accepted/refunded transport evidence plus provider audit correlation, including deliberately lost response | Never infer exact credit from aggregate balance movement. Uncorrelated outcomes stay uncertain and block automatic retry |
| Deletion and billing | Release one isolated root; record authenticated callbacks, subsequent availability and account observations over the applicable billing interval | Failed GET is not proof of deletion. Preserve physical/economic obligations separately; no unsupported deadline for billing stop. Retain immutable root history to prevent stale-callback reassignment |
| Old-backup recovery | Actual local management snapshot tests below demonstrate heap restoration bypassing upgrade fencing; live evidence must bind records outside restored state, including delayed callbacks/certificates | Upgrade reopening fences owners, but snapshot loading can restore an unfenced heap and erase later obligations. Snapshot operation remains unsupported; standalone certificates stay disabled. A local counter or clear fence cannot authorize reopening |

These limits retain current safety behavior. They do not silently remove Canic
parity, authorize a new completion state or weaken existing acceptance cases.
If an unavailable guarantee makes a feature impossible, explicitly revise that
feature's supported contract and acceptance case before enabling it. Verification
work stays off the storage canister where possible; no return to mandatory full
file hashing in the canister is implied.

## Probe command

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin caffeine-probe -- \
  public-source docs/evidence/caffeine-probes/runs/NEW-RUN-ID
cargo run --offline --locked -p ic-blob-storage-cli --bin caffeine-probe -- \
  verify docs/evidence/caffeine-probes/runs/NEW-RUN-ID
```

The parent directory must exist and the run directory must not. The command
persists `plan.json`, then each `request-N.json` before its HTTPS GET. It retains
`response-N.body`, its byte count/SHA-256/status/outcome in `response-N.json`, and
a final `summary.json`. Each source fetch uses the commit observed in request 0,
not another moving branch read. Runner version and a source/manifest/lockfile
fingerprint are recorded. Four requests maximum, 1 MiB per response, 30 seconds
per request, no redirects or application retries. Only official public GitHub/raw
source and npm endpoints are selected by the command. It sends no credentials,
gateway upload, account query, cycles or payment. File/directory sync is required;
failure stops the run. No existing run is resumed or overwritten.

`verify` checks recorded response hashes and structural completion; failed and
incomplete runs remain labelled. It does not promote source captures to behavioral
qualification. Review source/package drift separately; never update dependency
pins just because latest metadata changed. Native tests use a local HTTP substitute
to exercise intent-before-request, HTTP errors, refused redirects and body limits.
`make probe-check` verifies public-source `runs/` directories offline and is part of
the repository validation gate. Failed/incomplete experiments remain valid records;
corrupt, orphaned or structurally inconsistent recorded evidence fails the check.

Local SDK scenarios use the same pinned patched package as the browser fixture:

```sh
BLOB_BROWSER_NODE=/path/to/node24 make test-sdk-probe \
  BLOB_SDK_PROBE_REPORT=.tmp/NEW-SDK-RUN
```

The report's parent must exist and its directory must be new. This opt-in target
uses installed dependencies, with no downloads, sockets or provider calls. Its
supplied agent response, gateway and in-memory store are substitutes, explicitly
identified in `plan.json`. The SDK prepares and transfers the files; the test
does not reconstruct its upload protocol. The Rust verifier checks captured bytes
against the SDK root and original metadata, outside the storage canister.
The bundle/verifier hashes, request fingerprints, raw synthetic response bodies,
verification outputs and results are retained. `failure.json` preserves failed
cases; interrupted captures without a summary are incomplete. Files in the
material local runs below are covered by `local/SHA256SUMS` in `make probe-check`.
That checksum check establishes artifact integrity, not scenario correctness or
provider authenticity; rerunning the opt-in test establishes current local behavior.

## Run index

`trial-v070-preparation-01` / 2026-10-02: offline frozen artifacts, complete
stand-in installation/link candidate and SDK/native 1 KiB/10 MiB batch verification;
[hashes, failed checks, results and limitations](local/2026-10-02-trial-v070-preparation-01/summary.json).
Zero network/effects; no live owner or relationship selected. Private
`.tmp/trial-v070-preparation-01` retains the packet and immutable manifest.

`standalone-trial-review-01` / 2026-10-02: public source, retained-interface and
offline preparation observations stay separately identified in the
[review record](local/2026-10-02-standalone-trial-review-01/summary.json).
The [source run](runs/2026-10-02-trial-source-01/summary.json) captures four GETs,
with unchanged Mixin/Storage hashes and npm pin. The
[unit query](deployed/2026-10-02-pricing-units-01/summary.json) produces reported
currency data but exits 153 under its file-size guard; failed outcome is preserved,
without retry. The 1 KiB and default 10 MiB offline handoffs pass. No effect
authority, charge cap, namespace, production persistence or recovery is qualified.

`recovery-design-01` / 2026-09-30: source review and architectural inference;
[retained source references/hashes, findings and limits](local/2026-09-30-recovery-design-01/summary.json).
Official IC version, message execution, management history and snapshot rules were
reviewed against the current code and retained snapshot experiment. The historical
proposal considered an external complete journal/controller and an asynchronous
certificate prelude. The maintainer subsequently deferred that architecture in the
[current scope decision](../../service-contract.md#recovery-scope--maintainer-decision-2026-09-30);
the artifact's original recommendation remains preserved, not an active instruction.
This review is not runtime qualification. No provider requests, paid effects,
instances or cleanup; cached permits and sequence watermarks remain insufficient.
Production facts remain false.

`snapshot-01` / 2026-09-30: actual PocketIC management snapshot operations against
standalone and the durable fixture; [retained outcome and source/Wasm hashes](local/2026-09-30-snapshot-01/summary.json).
Both maintained scenarios pass (3.47 and 1.82 seconds). Rollback bypasses the upgrade
hook, revives withdrawn permission and loses later reservations/history. The fixture
can repeat simulated exposure when its host again substitutes qualified freshness;
standalone still traps certificate issuance without changing state. These are
negative recovery qualification results, not supported rollback behavior or deployed
provider observations. Three snapshots deleted; no provider requests, paid effects
or outstanding external cleanup. Strict targeted Clippy passes.

The `verifier-observation` local experiment is planned for 2026-09-29 against the
current workspace. Tests use the maintained Caffeine target encoder and streaming
verifier, a fixed PocketIC verifier identity, fixture owner/project and loopback HTTP
substitutes. Record intent before one bounded GET; exercise complete, corrupt,
truncated, oversized, non-200, redirected and encoded responses. Test refused service
plans, interruption/no-clobber behavior and a verified statement carried to receipt
recovery through an explicit fixture attestation. Per invocation: one service query,
one provider GET maximum, 30 seconds each, explicit content bound at most 1 GiB,
64 KiB service Candid. No deployed provider/account, credentials or attached cycles;
no external cleanup. Stop on failure and retain failed outcomes; these local cases
do not qualify the deployed provider or authorize live reads.

The `attestation-recovery` local contract check is planned for 2026-09-29 against
the current workspace and `storage_attestation_cli` PocketIC test. A fixed test
tenant signs bounded receipt queries; the fixture operator is the explicit verifier.
The plan saves the exact statement before a local attestation, discards its reply,
and inspects absence, acceptance, changed intent, wrong verifier/root trust and
settled/restored history. Exposure, content and deletion/settlement facts are
labelled substitutes. Each CLI query is limited to 30 seconds, 256 KiB HTTP and
4 KiB Candid; no query retries. No deployed account, gateway request, provider
charge or attached cycles are involved. Stop on failure; only temporary local
canisters/files require cleanup. The completed result is indexed below.

| ID / date | Class and target | Status / evidence | Conclusion and obligations |
| --- | --- | --- | --- |
| standalone-certificate / 2026-09-30 | Actual standalone and local storage-fixture PocketIC endpoints; no deployed provider | [Host refusal case](../../../tests/pocketic/tests/standalone_certificate/mod.rs) passes (4.27 seconds); six existing exposure/certificate cases pass (12.77 seconds), including real signed ingress verification, rollback and lost-reply recovery; optional Chromium case not rerun. Five core certificate tests, Candid comparison, strict affected Clippy, warning-free core/host docs and release Wasm builds pass | Standalone reports four unqualified prerequisites and refuses certificate issuance; role/malformed/unprepared/revoked/restored cases preserve state. Stop/start grants no new authority. Assessment does not reserve issuance. Shared blocker DTO/conversion replaces the fixture duplicate; positive issuance facts remain labelled fixture substitutes. No provider requests/attached cycles, external charges or cleanup obligations; no provider or old-backup qualification |
| public-source-03 / 2026-09-30 | Anonymous official source/registry capture, four bounded GETs | [Plan](runs/2026-09-30-public-source-01/plan.json), [summary](runs/2026-09-30-public-source-01/summary.json); captured at commit `14ab9511fd73258a380bc1a6861d6da6e548ebbb` | Upstream HEAD moved; retained Mixin/Storage SHA-256 values still match public-source-02, and npm remains 1.1.2 with the same integrity. The root-only certificate reply has no owner/project, admitted size, deadline or operation identity fields; source refresh adds no deployed pre-charge, replay, provisioning or recovery evidence. No provider/account effects or external cleanup |
| verifier-submission / 2026-09-30 | Native artifact validation and signed PocketIC through a bounded local fault proxy; fixture exposure and ten-byte local content | [Maintained journeys](../../../tests/pocketic/tests/storage_observe_cli/mod.rs) pass (4.49 seconds): normal acknowledgment, dropped real acknowledgment and forced HTTP 202. Native CLI tests and strict affected Clippy pass; historical settled/restored recovery (5.28 seconds) and signed standalone status/history regression (6.46 seconds) pass | Proxy compares the exact signed wire body with saved intent before forwarding one update. Competing submitters send once; incomplete, failed or changed observations reject before claiming. Killed provider reads remain undispatchable. A dropped acknowledgment stays uncertain; 202 stays pending. Both recover the exact immutable receipt through signed queries without resending. Request IDs, source/root/artifact hashes and outcomes are retained in temporary controlled test runs; scenario definitions remain in source. The pinned ic-agent 0.49.2 implementation was inspected locally for one-call transport, certificate verification and disabled retries. No live provider/account, attached cycles, external charges or cleanup obligations; deployed retention/billing/exposure remain unqualified |
| historical-recovery / 2026-09-26 | Source inspection, substituted HTTP replies, Candid codecs and anonymous metadata | [Existing record](../caffeine-recovery-review.json), indexed retrospectively; not rerun | SDK progress is insufficient; root-only callbacks and payment correlation need conservative handling. No paid effects were recorded in that review; no fabricated modern run metadata |
| historical-deployment / 2026-09-25 | Anonymous deployed interface/gateway/price observations | [Existing record](../caffeine-deployment-observation.json), indexed retrospectively; use its own timestamps/scope | Interface reachability is not upload, payment or billing qualification |
| public-source-01 / 2026-09-29 | Anonymous public source/registry capture; four recorded GET invocations, no account/paid effects | [Plan](runs/2026-09-29-public-source-01/plan.json), [summary](runs/2026-09-29-public-source-01/summary.json); captured, integrity checked | Official commit `78781961e52b8c9c874becd473402950429d4818`; both backend hashes match the prior review; npm 1.1.2/integrity unchanged. This runner had no application retries but still inherited reqwest protocol retries; actual wire request count was not measured. Preserved as captured; current runner explicitly disables transport retries |
| public-source-02 / 2026-09-29 | Repeat public-source capture after explicitly disabling HTTP transport retries; four requests maximum | [Plan](runs/2026-09-29-public-source-02/plan.json), [summary](runs/2026-09-29-public-source-02/summary.json); captured, integrity checked | All four response hashes match 01. New runner/source fingerprint retained; 01 and its retry limitation remain unchanged. No provider effects |
| local-sdk-01 / 2026-09-29 | Pinned patched SDK with substituted certificate-agent reply, gateway and in-memory intent store | Failed before runner initialization: ESM bundle attempted dynamic require of Node `tty`; no run directory or requests. Fixed bundle with Node `createRequire` | Planned six scenarios did not start. No network/account access or cleanup obligations |
| local-sdk-02 / 2026-09-29 | Same local substitutes; independent native Rust content verification | [Failure](local/2026-09-29-sdk-02/failure.json): first verification subprocess timed out; partial request/reply records retained | Node synchronous subprocess input did not reach EOF in this environment (reproduced with `cat`); asynchronous closed input works. No completed case, network/account access or cleanup obligations |
| local-sdk-03 / 2026-09-29 | Same local substitutes, asynchronous verifier subprocess | [Plan](local/2026-09-29-sdk-03/plan.json), [summary](local/2026-09-29-sdk-03/summary.json); all six scenarios pass | SDK sends all chunks despite `existing_chunks` and returns successfully for a non-complete status. Lost final reply stays uncertain despite verified bytes; replay blocked. Aggregate budget stops before the next request. No real network/account access or cleanup obligations |
| verifier-contract / 2026-09-29 | Local model/CLI and PocketIC; fixed test verifier principals and substituted content observations | 60 upload-store and two codec tests pass; all 32 standalone cases pass (141.72 seconds), plus `storage_completion` (5.05 seconds) | Exact authority/permission binding, immutable receipt replay, four stable-write rollback cuts, late revocation, release/settlement and fenced restoration pass. The signed CLI verifies 10 MiB against original metadata without mutating service state. Local canisters only; zero provider requests/attached cycles, no external cleanup or deployed availability claim |
| attestation-recovery / 2026-09-29 | Local saved-intent decoder/CLI and signed PocketIC receipt queries; fixed test tenant and fixture verifier | `storage_attestation_cli` passes (5.33 seconds), including a deliberately discarded successful update acknowledgment | Absent, matched and conflicting evidence remain distinct. Changed permission/verifier and wrong root trust reject; settled/restored receipts remain immutable. Queries preserve stable bytes and the saved intent. This discards a test acknowledgment, not a simulated network packet; content/exposure/settlement are substitutes, with no deployed provider availability claim. No external requests, charges or cleanup obligations |
| verifier-observation / 2026-09-29 | Native loopback HTTP substitutes and signed PocketIC installed-plan/statement journey; fixture exposure only | 14 native CLI tests pass, including multi-chunk streaming and HTTP corruption/EOF/size/encoding/redirect refusals. `storage_observe_cli` passes (4.00 seconds); standalone completion/plan refusals pass (5.37 seconds) | Intent exists before GET; complete verification saves the exact statement without dispatch. Killing the subprocess mid-body leaves no statement/summary and forbids reuse of that run. Explicit fixture dispatch accepts the independently checked statement and receipt recovery matches. Service bytes stay unchanged during observation. Local artifacts are temporary test outputs; maintained tests retain the scenario definitions. No real provider/account access, charges or cleanup obligations; no deployed qualification |
| certificate-cli-01 / 2026-09-30 | Signed local native queries with current production host facts | [Failed test outcome](local/2026-09-30-certificate-cli-01/summary.json) | All transport, blocker, intent, revocation and fence checks reached; final test assertion wrongly expected Reserved after explicit revocation (actual Cancelled). First outcome and source/binary hashes retained. No provider requests or paid effects; local instance stopped and temporary artifacts dropped |
| certificate-cli-02 / 2026-09-30 | Same local signed journey with corrected post-revocation baseline | [Passed test outcome](local/2026-09-30-certificate-cli-02/summary.json), 6.40 seconds | Complete revoked record survives upgrade unchanged; signed assessment retains all four blockers and rejects changed intent/trust, revocation and restoration. No state changes, certificate updates, provider requests or paid effects; local instance stopped and temporary artifacts dropped. No deployed qualification |
| upload-history-cli-01 / 2026-09-30 | Local PocketIC inventory setup; no native query reached | [Failed setup outcome](local/2026-09-30-upload-history-cli-01/summary.json) | Fixture requested 65 retained declarations but kept lifetime chunk limits at 20; admission correctly returned Capacity. Existing service history tests passed. Source/binary hashes and first failure retained; owned instance dropped, no provider/paid effects |
| upload-history-cli-02 / 2026-09-30 | Signed local queries with corrected fixture history budget | [Passed outcome](local/2026-09-30-upload-history-cli-02/summary.json), 6.56 seconds | Empty filtered scan advances across 64 cancelled declarations; explicit pages retain full-width active identity and cancelled history through fenced upgrade. Saved cursor/scope/identity refusals preserve stable bytes. Local instance stopped and temporary files dropped; no provider/paid effects or recovery qualification |
| funding-native-01 / 2026-09-30 | Native CLI validation in network-restricted sandbox | [Failed environment outcome](local/2026-09-30-funding-native-01/summary.json) | New funding cases passed; existing download substitute could not bind loopback. Failure retained before unchanged loopback-enabled rerun; no provider contact or paid effects |
| funding-outcome-cli-01 / 2026-09-30 | Signed local standalone and controlled durable funding observations | [Passed outcomes](local/2026-09-30-funding-outcome-cli-01/summary.json), 10.93 seconds PocketIC | Exact history-to-outcome binding, partial refund, uncertain attachment, typed conflicts, absence and fenced upgrade preserve stable bytes. Fifteen native invocations, fourteen query attempts, no provider requests/payments. Native unit rerun passes with local loopback; all local instances/files dropped. No provider-credit or recovery qualification |
| reference-native-01 / 2026-09-30 | Signed local standalone and controlled durable reference inspection | [Passed outcomes](local/2026-09-30-reference-native-01/summary.json), 18.51 seconds PocketIC | Separate history/current queries preserve success and recorded failure through release, substituted settlement and fenced upgrade. Wrong intent/scope/role/trust refuses, queries preserve stable bytes, installed receipt/cleanup bounds unchanged. Twenty-two native invocations, twenty query attempts; no provider requests/payments. Owned local instances/files dropped; no availability, publication or recovery qualification |
| standalone-lifecycle-01 / 2026-09-30 | Source review and local standalone lifecycle | [Passed outcome and review](local/2026-09-30-standalone-lifecycle-01/summary.json), 6.37 seconds | Stop/start preserves current installation/state; rejected replacement settings preserve the previous owner; two same-release upgrades retain metadata and all four fences. Operational refusals preserve stable bytes. Funding/read owners are empty in this case. Acceptance/parity summaries corrected without qualifying active recovery, provider, managed parity or retirement. Owned instance dropped; no provider/paid effects |

Other retained investigations are indexed here without inventing missing request
logs or replaying their effects. Use each record's own dates and evidence classes:
[client observations](../caffeine-client-observations.json),
[Mops verification](../caffeine-mops-verification.json),
[funding review](../caffeine-funding-review.json),
[contract refresh](../caffeine-contract-refresh.json),
[installation review](../caffeine-installation-review.json),
[browser reuse](../caffeine-browser-reuse.json), and
[gateway/account transport review](../caffeine-gateway-transport.json).
The older records retain their original schemas; they are research history, not
backward-compatible product state or a claim of modern capture completeness.

The maintained Chromium/PocketIC suite also passed its ten scenarios with the
aggregate-budget contract (49.12 seconds). The initial sandbox run could not bind
the local server; the loopback-enabled rerun passed. This uses actual IC certificate
and IndexedDB behavior with a local gateway substitute. It does not turn the SDK
probe's synthetic certificate into deployed-provider evidence.

## Current trial proposal

The [first standalone trial proposal](../../standalone-trial.md) replaces the
earlier two-file upload/resume draft. It targets one nonempty file of at most
1 KiB, a fresh isolated owner and trusted participants, within the maintainer's
100T-cycle planning ceiling. Its account, namespace, financial terms and operating
contract remain unselected or unaccepted; no paid effect is dispatched. The
proposal distinguishes client request bounds from provider spending guarantees
and retains all four current certificate blockers.

Lost-response/replay, deletion/billing and long-retention experiments require
separate recorded intents and observation windows after the first trial. They
are not part of its upload budget or automatic retry authority. Preserve every
object and uncertainty record until its continuing obligations are evidenced.

## Restricted contract implementation — 2026-10-02

The maintainer explicitly accepts the trusted-uploader/fresh-owner contract,
without a provider spending cap or operational old-backup recovery. Intent in
`.tmp/restricted-contract-01/intent.txt` bounds local implementation and checks;
no new provider query, account mutation, deployment or paid effect is authorized.
Replace former provider-qualification flags directly with current local issuance
prerequisites. Persist required uploader trust in the current v1 installation,
retain exposure-once/revocation/accounting and inspection-only restoration, and
keep actual provider guarantees independently unqualified. This requires a minor
release; no version mutation occurs.

Local validation includes actual standalone issuance/refusal, signed native
inspection and stop/start/restoration/snapshot behavior. The first sandbox run
fails before canister creation because loopback binding is denied; its log remains.
An explicit loopback-enabled run then finds that the restricted fixture still uses
its original 10 MiB manifest: admission correctly refuses Capacity. A corrected
small-file helper initially fails compilation converting u64 directly to
NonZeroUsize; that attempt remains before the explicit checked usize conversion.
The initial strict lint also rejects an empty-vector assertion; corrected typed
comparison introduces no suppression. All are local failures, not provider probes.

The earlier sealed review/source/query artifacts remain unchanged. The accepted
[contract](../../standalone-trial.md) replaces their former issuance semantics;
source/npm observations and the failed public unit query do not prove provisioning,
pre-charge or replay controls. The current implementation's retained summary and
final local results are recorded separately after validation completes.

A wider standalone run first passes actual restricted issuance and broader-profile
refusal, then the entire shared `target/` disappears during validation. Nine cases
pass and 39 fail, predominantly before installation on missing fixture inputs.
No cleanup command is issued by this work; the cause is unestablished. Preserve
that run without treating missing artifacts as service failures. Rebuild only after
checking for active builders and retain exact Wasm/CLI copies before the next run.

The maintainer subsequently confirms running `cargo clean` during that wider run,
explaining the artifact loss. The agent initiates no cleanup. This explanation
supplements the retained failed outcome; it does not erase or rerun a provider probe.

Final fixed-artifact outcome: all 48 standalone cases pass (268.95 seconds), including
actual restricted issuance, untrusted-uploader refusal and supported lifecycle. The
unsupported snapshot case shows that rollback forgets later exposure/revocation and
restores local eligibility; this is evidence against active snapshot use. Seven
shared exposure/IC-certificate rollback/recovery cases pass (36.77 seconds) using
explicit fixture host facts. The ten-scenario Chromium/SDK suite passes (56.50
seconds) with actual IC/IndexedDB and a local gateway substitute. Its two-slot store
is not a qualified production client store. Current required-uploader host init
independently encodes/decodes, offline installation and targeted core/CLI checks
pass, as do strict affected lint and formatting. Exact retained artifact hashes and
all failed attempts are in `.tmp/restricted-contract-01`; the
[separate retained summary](local/2026-10-02-restricted-contract-01/summary.json)
records each evidence class. No new live provider/account/object or paid effect,
version/release operation, allocator change, downstream dependency or sibling edit.

## Maintained browser journal — 2026-10-02

Intent in `.tmp/browser-intents-01/intent.txt` bounds repository implementation and
actual local Chromium/PocketIC checks only. No source refresh, external query,
deployment, account mutation, funding or provider object request occurs. The
maintained `clients/browser/intents.js` now owns the certificate/gateway journal;
the two-slot upload fixture calls it with test-only platform abort injection rather
than duplicating transitions. Its explicit create/open modes refuse missing history
and changed capacity, and strict transactions retain lifetime slots and uncertainty.

Real Chromium checks pass for competing tabs, graceful browser-process restart,
retained cancelled capacity, origin/body bounds and deliberately corrupted-history
refusal. The ten-scenario local IC/SDK upload suite passes in 49.94 seconds with
actual aborted transactions, cancellation races and a local gateway substitute.
The first upload run fails on extra fixture identities beyond its deliberately
maximum-u128 permission; the corrected test and failed log remain retained.
No production identity allocation or replay rule changes. The
[separate retained summary](local/2026-10-02-browser-intents-01/summary.json) records
all outcomes and `.tmp/browser-intents-01` holds logs and hashes. Actual trial
profile/origin/database, authentication, eviction, power loss and rollback behavior
remain unqualified. This removes a fixture-only implementation gap; it does not
establish deployed Caffeine availability, economics, retention or billing cessation.

## Complete local restricted-host journey — 2026-10-02

Intent in `.tmp/standalone-browser-01/intent.txt` bounds two fresh local owners,
three distinct fixed test identities and maintained browser/native tooling. Each
owner has one 1,024-byte object, at most two serial PUTs bounded to 64 KiB each and
128 KiB aggregate, and at most two complete GETs. No external account/gateway
request, deployed instance, funding or paid effect is authorized or performed.

`make test-browser-standalone` passes both cases in 8.46 seconds. Actual installed
standalone prerequisites issue the IC-signed certificate, and the maintained SDK
uploads through the one-slot IndexedDB journal. Reload recovers historical evidence
without reissue or resumed transfer. The local gateway serves bytes received from
the upload. HTTP success alone refuses tenant download; an independent native
whole-body GET produces a statement, a distinct configured verifier submits it,
and the tenant then downloads verified bytes. Exact reference release retains
physical bytes and liabilities; it does not delete the substitute's object.

The separate corrupt-body case uses one GET, records content_mismatch without a
statement/attestation, and leaves download unavailable. Tenant withdrawal and
browser cancellation preserve ExposurePossible and 1,024-byte obligations. Neither
case retries a provider request. The initial digest-format and nested-revocation
compiler mistakes, constructor warning, editing context misses, first successful
run and corrected full target remain in the capture. Strict affected integration
lint, formatting and current Wasm/CLI builds pass; Wasm/CLI hashes match the earlier
retained artifacts. The [summary](local/2026-10-02-standalone-browser-01/summary.json)
and separate success/corrupt records retain exact outcomes and limitations.

This is actual host/browser/native integration with a provider substitute, not
qualification of deployed acceptance, provisioning, billing or retention. Select
actual live identities/account/browser environment and provider authority before
effects. Project values must satisfy the SDK HTTP-header check as well as service
metadata/URL validation. No SDK wire implementation, second journal, downstream
framework dependency, allocator, version or production API is added.

## Complete offline installation carrier — 2026-10-02

Intent in `.tmp/installation-carrier-01/intent.txt` bounds one passive shared init
DTO, CLI packaging, independent Candid decoding and targeted actual local host
installation. Offline `installation-check` now preserves complete init bytes and
their hash, without host/compiled-release authentication or effect authority.
Actual local installation consumes those bytes unchanged; wrong actual service
binding traps before allocation and preserves original heap/config/stable state.
Configured project/verifier/trusted uploader read back, controller-only inspection
refuses, and restricted tenant/uploader preparation has no local blockers.
No live certificate or provider request occurs.

The maintainer's concurrent ic-memory 0.15.2 update is preserved. Its cached
published changelog matches the clean local release; source review finds addressed
IcyDB lint feedback without a new read/API/schema contract. The linked issue-body
web lookup fails, without a retry or status inference. One runtime resolves;
native installation tests, current Wasm builds, affected strict lint and actual
typed growth refusal/rollback/exact retry/fenced restore pass. Wrong CLI target,
lint, fixture-variable and sandbox metadata-lock attempts stay in the fresh capture.

Read-only local identity discovery finds `canic-mainnet` and retains its public
principal; default identity is unchanged and no credential is exported or signer
tested. Live roles/account/project/browser remain unselected. The
[summary](local/2026-10-02-installation-carrier-01/summary.json), exact offline report
and upstream-review record distinguish these local facts from deployed Caffeine
qualification. Earlier captures remain immutable; no deployment, payment, account
change, Caffeine object/query, full CI, version, commit, sibling edit or build cleanup.

## Immutable browser namespace handoff — 2026-10-02

Intent in `.tmp/browser-namespace-01/intent.txt` bounds native preparation and
local journal/SDK/standalone checks only. Project and bucket now belong to the
original native upload-input JSON and immutable certificate intent. The maintained
transfer derives them from that owner instead of accepting independent namespace
arguments. Current v1 records are replaced directly; no extra journal, endpoint,
provider wire implementation, allocator or compatibility reader is introduced.

Six targeted native cases and strict affected CLI/integration lint pass. Actual
Chromium journal checks preserve namespace fields through restart and refuse drift,
invalid Unicode, controls and UTF-8/header bounds before claims. The pinned SDK's
six local substitute cases and a 1 KiB native/browser snapshot handoff pass without
network. Both actual standalone/browser/native journeys pass in 8.37 seconds
against the final bounded client; the earlier 9.27-second run remains retained,
including project/bucket conflicts before certificate dispatch and correct emitted
tree fields, successful verification/attestation/download/release and corrupt-body
refusal preserving exposure/liabilities. The retained Wasm matches the previous
current artifact; CLI/browser artifacts are fresh. Initial multi-operation patch
and function-length lint failures remain, with corrected outcomes and no suppression.

The [summary](local/2026-10-02-browser-namespace-01/summary.json) separates exact
local facts, offline SDK substitutes and gateway/account substitutes. Original
project must still match installation and bucket must match real provisioning;
immutable local history cannot establish either. Live targets remain unselected.
No live source refresh/query, Caffeine object/account/funding/deployment effect,
full CI, version, commit, upstream message, sibling edit or build cleanup occurs.

## Installation-bound upload preparation — 2026-10-02

Intent in `.tmp/upload-installation-01/intent.txt` bounds shared offline candidate
validation, native/SDK preparation and actual existing local standalone browser
and signed-setup journeys. Require the complete current init carrier, reject
service/namespace/project/trusted-uploader mismatches and constrain manifests with
its resource bounds before producing usable upload artifacts. Retain exact carrier
bytes/hash; actual installed-state, provisioning, remaining capacity and provider
behavior remain separate. The gateway/account are local substitutes with the
existing two-PUT, two/one-GET budgets, serial execution and no retries. No new live
provider/deployment/account/payment effect or sibling work is planned.

Outcome: targeted native candidate/binding/bounds cases and strict affected lint
pass. Offline pinned SDK/native handoffs pass at 1 KiB and 10 MiB with networking
refused. Both actual local standalone/browser/native journeys pass in 10.02 seconds;
exact carrier bytes/hash match, independent verification/attestation/download/release
succeeds, and corrupt bytes preserve exposure/liability. Signed lost/pending replies,
cancellation and fenced restoration pass in 8.07 seconds with the declared uploader.
Initial invalid metadata-budget and outdated trust-assertion failures remain in
the capture alongside corrected results; no uncertain provider effect is repeated.
See the [summary](local/2026-10-02-upload-installation-01/summary.json). Release history,
receipt and toolchain remain unchanged. Canic adoption is explicitly deferred;
actual installation/provisioning/account/browser/cleanup selection still requires
separate evidence. No live provider/deployment/payment, allocator, full CI, version,
commit, sibling change or build cleanup occurs.

## Browser replay repair — 2026-10-02

The maintainer asks to fix the retained Chromium pre-header duplicate. Intent and
every capture are in `.tmp/browser-replay-01`. Four exploratory matrices distinguish
navigation's credentials pool from the warmed fetch pool, complete-body loss from
immediate data-close, and HTTP/2 REFUSED_STREAM. Buffered fetch, keepalive:false and
XHR repeat after receipt on reused connections. Streams refuse HTTP/1.x and show
one arrival under the three HTTP/2 cuts. The first unrepresentative pool remains
recorded rather than treated as a fix. Three direct source GETs succeed against
Chromium tag 153.0.8010.12 after web-tool errors; their bytes/hashes remain. Source
shows a streamed-body replay cache, limiting any universal inference.

The current transport snapshots/fingerprints as before, then emits one immediately
closed stream with duplex:half. HTTPS and stream support are required; missing
support refuses before issuance, and HTTP/1.x negotiation fails without buffering.
Current maintained regression runs pass all three owned TLS cuts, including
unsupported-stream construction/history preservation and buffered/XHR controls.
This changes browser compatibility in the existing minor-release draft, without
changing the SDK's application method/URL/headers/payload, core schema or owner.

All four actual local standalone/browser/native journeys pass (18.06 seconds) with
the original pre-header loss restored, exactly the two planned PUT arrivals per
owner, retained uncertainty and verifier recovery without another upload. Native
reads use a separate HTTP loopback origin serving the same received bytes. Ten
existing IC/browser certificate/gateway scenarios pass (53.11 seconds), and all
six SDK substitutes pass. Initial standalone uploads succeed but the recovery
probe uses the wrong origin; all four assertion failures and the corrected rerun
remain. Browser request observation replaces interception so tests preserve the
streaming network path. Owned browsers/sockets close and temporary TLS keys are
removed; retained Wasm/CLI/build/evidence artifacts remain.

See the [summary](local/2026-10-02-browser-replay-01/summary.json),
[transport observations](local/2026-10-02-browser-replay-01/transport.json) and
[standalone observations](local/2026-10-02-browser-replay-01/standalone.json).
This fixes the reproduced local path, not deployed provider economics or every
possible browser/intermediary retry. Live HTTPS/stream/CORS support, other browsers,
HTTP/3 and persistent environment remain unqualified. No provider/IC mainnet
request, payment, live deployment, release/commit, full CI, sibling work or build cleanup.

## Standalone transfer interruption — 2026-10-02

Intent in `.tmp/standalone-interruption-01/intent.txt` bounds four actual local
standalone/Chromium/native cases with owned gateway/account substitutes, at most
two serial PUTs per fresh 1 KiB owner and six GETs across all cases. New cuts lose
the final chunk response only after retaining received bytes, and withdraw tenant
permission after a valid whole-body observation but before its signed attestation.
Keep the uncertain gateway claim, exact certificate history, exposure and byte
obligations; recovery never repeats an upload. Use the existing handlers and
journal, with supported stop/start and typed service refusal evidence. No live
provider/query/account/deployment/payment effects or downstream work is planned.

Source review corrects the initial late-attestation-refusal assumption before
runtime probing: the maintained lifecycle explicitly permits actual completion
after revocation, and verification plans permit reconciliation of exposed work.
The maintained withdrawal case accepts the exact statement, keeps permission
revoked, explicitly releases its reference and proves an exact attestation replay
leaves that reference inactive. No product semantics are changed to fit a test.

The first local pre-header-loss experiment fails both new cases: the journal's
final request is responded rather than uncertain. A separately bounded diagnostic
observes three PUT arrivals (tree, chunk, byte-identical chunk), from two journal
claims with disabled SDK retries. The substitute retains only the planned writes
and rejects the repeat with HTTP 500. This is Chromium 153 loopback transport
evidence, not a deployed gateway observation or provider replay-charge fact.
All failed logs, diagnostic source and request fingerprints remain retained.
Budgets are now described precisely as fetch-hook dispatch/body limits; transport
retransmissions and exactly-once wire behavior are outside that meter.

A distinct final-response-body truncation experiment passes all four maintained
standalone/browser/native journeys in 17.84 seconds, with two PUT arrivals per
owner and six GETs across the cases. The final chunk was received before its 200
reply body truncated; the browser records failure and preserves the uncertain
claim across reload/read-state recovery. Actual owner stop/start preserves exposure,
and independent verifier/tenant reads succeed without another upload dispatch.
The withdrawn case accepts late reconciliation, releases its reference and keeps
physical/liability bytes despite attestation replay and browser cancellation.
Strict affected integration lint, formatting and diff checks pass. The initial
nonexistent refusal-variant compiler error and correction remain in the capture.
See the [summary](local/2026-10-02-standalone-interruption-01/summary.json) and
[transport diagnostic](local/2026-10-02-standalone-interruption-01/transport-diagnostic.json).
Wasm/CLI/SDK artifacts are reused unchanged; no live provider/account/deployment/
payment, new owner/API/allocator, full CI, version, commit, sibling change or cleanup.
