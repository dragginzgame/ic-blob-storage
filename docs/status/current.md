# Current status

Date: 2026-10-04

## Released baseline

Released **0.14.6** is at `edc8a9b4df672569b03e1454d620c70dbbf4fa52`,
with validated source `c23609f264ed34c6885106d08657bb83c062aa32`.
The local tag and [release receipt](../release.json) agree; the maintainer reports
it live. [Cargo](../../Cargo.toml) and [the changelog](../../CHANGELOG.md) own release
metadata. The release does not establish registry publication, consumer adoption
or deployed provider behavior. The lock uses ic-memory 0.25.0 and ic-testkit 0.14.11.

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

## Active 0.14.7 draft

The maintainer requested the undated 0.14.7 changelog. Cargo and the release
receipt remain at 0.14.6; version preparation and full release validation remain
maintainer-operated.

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

## Remaining product work

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
- Measure populated service reopen and full browser/worker memory before promising
  million-object operation. Journal rows count lifetime upload permissions; service
  objects have separate configured limits. The current journal measurements do not
  qualify canister instruction budgets or a dedicated-canister sizing decision.

## Consumer feedback and GitHub disposition

The [last retained issue review](../evidence/caffeine-probes/local/2026-10-04-gh-issues-review-01/summary.json)
found seven issues/seventeen comments and verified remote main at 0.14.3. No new
remote refresh or issue write is claimed; local release records establish 0.14.6.

| Issue | Repository result | Remaining action |
| --- | --- | --- |
| [#1 embedding](https://github.com/dragginzgame/ic-blob-storage/issues/1) | Complete installation requests and library-owned version implemented; published 0.14.2 contents verified | Original request ready to close; wrapper adoption separate |
| [#2 budgets](https://github.com/dragginzgame/ic-blob-storage/issues/2) | Transfer preflight and zero-effect negatives released in 0.14.3 | Original implementation request ready to close; no provider spending-cap claim |
| [#3 FIFO](https://github.com/dragginzgame/ic-blob-storage/issues/3) | Same-descriptor nonblocking reader; both FIFO boundaries confirmed | Original request ready to close; CLI remains source-installed |
| [#4 publisher](https://github.com/dragginzgame/ic-blob-storage/issues/4) | Clean release recipe and callable driver/recovery qualified here | Consumer adoption, certified mapping transaction and deployed bindings |
| [#5 serving](https://github.com/dragginzgame/ic-blob-storage/issues/5) | Original metadata and real-media delivery pass locally | Actual origins/CSP, inventory, transaction, game rendering and deployed serving |
| [#6 lifetime](https://github.com/dragginzgame/ic-blob-storage/issues/6) | Overlapping references, retired-root refusal and bounded history implemented | Consumer retention/retirement; provider deletion/final billing |
| [#7 decoding](https://github.com/dragginzgame/ic-blob-storage/issues/7) | Structurally valid skip/type refusals at real standalone boundaries | Header ceiling subsumed; independent work exhaustion remains unreproduced |

For #7, preserve the [decoder contract](../service-contract.md#standalone-ingress-decoding)
and original `.tmp/decoder-budgets-01` evidence. Do not weaken bounds or add test
hooks to force exhaustion. Native batch publication remains bounded to 4,096 files.
Canic adoption stays deferred; the [feedback list](../canic-parity.md#integration-feedback)
retains wrapper/lifecycle, one-memory-runtime, current-format and recovery actions.
No upstream message or sibling edit is authorized.

The old isolated owner stays frozen at 0.6.0 with original stopped history; the
separate live owner was last verified at 0.7.0. Both retain exhausted lifetime
capacity and complete provider/billing obligations. Never reset or upgrade them
as source cleanup. Cross-release reinstall requires obligation disposition.

[The ledger](../evidence/caffeine-probes/README.md) separates source/local/substitute
and deployed observations. [History](history.md) retains old handoffs without
turning their forecasts into current operating instructions.
