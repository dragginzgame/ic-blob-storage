# Current status — 2026-10-31

## Active batch and published graph

The maintainer explicitly delegated **0.61.0 release and crates.io publication**
on 2026-10-31, following the dependency/adoption preparation below. The source
base before preparation is **`7fe95703`**, tagged **v0.60.0**. The common runner
owns version preparation, release commit/tag, validation, push and recovery;
publication is a separate step under this delegation. This handoff is written
before execution: inspect `docs/release.json`, the matching tag/remote and registry
readback for its actual outcome, rather than interpreting intent as delivery.
Preserve the Tooling update and acceptance/tracker cleanup. Deployment, paid
provider effects and sibling source edits remain outside this release task.

Adopted published **Platform 0.15.2 / Wire 0.13.0 / Metrics 0.24.1**, verified
against official registry index entries and release archive checksums. Platform
reexports canonical **Types 0.15.1**, and Wire supplies schemas for that exact
identity. This resolves Blob's former Principal/Account schema blocker tracked in
[Wire #37 — Supply compatible Platform integrations](https://github.com/dragginzgame/ic-wire/issues/37). Existing imports
remain through Platform; no direct Types dependency, aliases, schema bridge,
mirrored DTOs or dual reader was added. Metrics defaults remain disabled;
Contracts' native `wire-schema` explicitly selects `ic-metrics/native-wire` and
Wire's `platform-types`. No owned endpoint or persisted representation changed.

Adopted published **Build/Host Foundation 0.73.1**, **Host Tools/Testkit 0.75.0**
and **Memory/Storage 0.60.1**. Foundation and IC integration have independent
supplier versions; Blob retains matching Build/Host Process identities. Official
index entries, archive checksums and migration guidance were reviewed. Memory's
public Wire error identity aligns with direct Wire 0.13; stored bytes, bounds,
stable-structures 0.7.2 and allocation custody remain unchanged. No data reset or
automatic migration was introduced. Incoming unrelated lock updates are preserved.

The graph now has one **Platform 0.15.2 / Types 0.15.1 / Wire 0.13.0** pairing.
Older Platform 0.11/0.12 and Wire 0.10 copies are removed. Testkit's published
requirement retains arithmetic-only Metrics 0.23.2 alongside Blob's 0.24.0;
its disabled defaults introduce no older metadata/Wire dependency. Deployment
contains only Metrics 0.24.1. Blob's deployed executor and exact runtime guard
are solely **Platform 0.15.2**. Embedding runtime users must coordinate that identity, including Timers;
never bridge task handles between executors. External provider Candid remains
required behind Platform. No external IC0 or registry CDK executor is introduced.

Shared Tooling **0.27.22**, revision
**`f4f431627290e18f0ccc1e5f8525d16b197cf7c3`**, is adopted with **139 files**.
Reviewed impact and immutable-source export retain the new release-group
companions. Blob keeps one coordinated workspace version and root changelog;
no independent groups or package moves were introduced. The root catalog now
lists all thirteen members with their existing full versions, including unused
catalog entries; these add no dependency edges. Publication policy remains local,
and the shared publication adapter is still opt-in. The local restriction on
autonomous releases remains the ordinary-maintenance default; the latest explicit
request delegates this one 0.61 release/publication. Normal Cargo cleanup is unchanged.
The first reviewed Shared commit had a 0.27.22 title but committed VERSION
0.27.21; the subsequent published VERSION-only correction was reviewed and
adopted before finishing. Both impact/export attempts remain in the evidence.

Browser Platform **0.1.1** is still npm's latest published release; SDK Core
**6.1.1**, Caffeine **1.1.2** and browser package files are unchanged.
Sibling checkouts remain read-only; exports came
from clean isolated committed sources rather than their pending work.

## Current acceptance preparation

The incoming lock advanced Memory/Storage from 0.60.0 to 0.60.1 and Metrics from
0.24.0 to 0.24.1 after the preceding batch. These published archives match index
checksums. Production `src/` trees are byte-identical; their manifests change
package versions and Storage's Memory requirement. Preserve those updates;
minimum declarations remain 0.60.0/0.24.0. Earlier receipts describe their earlier
graph, not the new artifact hashes. New standalone Wasm and all-feature native CLI
builds pass. Rehearsal intent, exact inputs, source comparisons, artifact hashes
and reports are under `/tmp/blob-acceptance-1031.QfEhzI/`. All **4** maintained
Chromium/CLI/PocketIC cases and **3** browser controls pass on the current patch
graph with six fixed 1 KiB uploads, two fixture signing identities, verified reads,
corruption refusal, lost-final-reply and withdrawal recovery. Input/artifact
readbacks stayed fixed. This is local-substitute evidence with zero deployment,
deployed-provider requests or paid effects; it proves no production login or serving.

Read-only Canic at `247ba9d3` still selects Blob 0.60, Platform 0.11/Wire 0.10
and Memory/Storage 0.58 across its root/consumer catalogs. Both Blob deployment
environments are local-only; its demo/test configs select no Blob role. IC Crates
at `3c67db3` still has retired completion-verifier/attestation flows. No sibling
was edited or built. The maintainer confirmed Caffeine and has no application/fleet
configuration to supply. Canic/the consuming application owns that configuration;
do not ask the maintainer to invent a service principal or reuse trial identities.
For pending 0.61 deployment, first release/publish Blob through maintainer gates,
then have Canic adopt/qualify its matching graph and mount the selected application
role. A provider-only trial can precede the final application frontend, without
claiming application login or media-publication acceptance. The concrete owner
actions were posted to [Canic #622](https://github.com/dragginzgame/canic/issues/622#issuecomment-6546806175).
[The existing acceptance plan](../acceptance-plan.md#before-the-acceptance-deployment)
now separates pre-deployment prerequisites from observations that require the
deployment. No issue is closed or production acceptance claimed by this preparation.

## Preceding Build/Host/Persistence/Testkit qualification

Evidence is retained under **`/tmp/blob-bhms-1031/`**, including previous inputs,
official indexes and verified release archives. Shared Tooling remains the latest
published revision already adopted above; its unchanged snapshot/runner proofs
are reused. The lock-selected Testkit 0.75.0 CLI/server and Build 0.73.1 release
editor were prepared through canonical setup, then admitted offline. Cached
payloads were reused; setup did not remove earlier tool generations. The server
remains PocketIC 16.1.0. Actual Make routing, admitted-server forwarding and
ordered failure propagation pass with substitute tools. Direct dependency and
resolved deployment boundary checks pass.

Strict Clippy passes for core, Contracts, client, CLI and standalone across all
targets/features, plus selected standalone/journey/storage integration targets.
Native tests pass: core **338**, Contracts **121** under the selected default
test graph, standalone heap metrics **2**, memory composition **2**, CLI **122**
and private probe-tool controls **7**. Rust **1.91** checks pass for standalone
Wasm and all-feature CLI binaries. Fresh standalone/six-probe release Wasms and
all-feature native CLI/archive-verifier binaries build successfully.

Installed tests pass **50** cases: metrics **2**, lifecycle **5**, malformed
ingress **4**, certificates **3**, gateway synchronization **2**, recovery **33**
and restoration-read attribution **1**. These are local IC/provider substitutes.
The Build-backed package regression passes same-version changed archives,
warm-cache execution and archive graph/byte/source/manifest/lock drift refusals.
Its retained fixture is `target/package-fixture.93JnuE`; explicit Cargo cleanup
can remove it, while this batch's logs and input/artifact hashes are outside target.
Formatting, snapshot integrity, dependency pins, documentation links and diff
checks pass. Input and artifact readbacks remain fixed. The complete
release-adapter fixture passes against the newly admitted Build editor, including
refusal, retry, rollback, exact-index and failed guard-read cases with isolated
Git/Cargo effects; no real commits, pushes or publication occurred.

These preceding focused checks supply no complete repository gate, hosted result, native macOS qualification, fresh
Chromium application journey, deployed provider acceptance or performance
improvement claim. They ran before release metadata preparation, with package
versions 0.60.0 and draft notes 0.61.0. Endpoint identities, frames, limits, record layouts, schema numbers
and exact retry bytes remain unchanged; cross-release adoption still requires
retirement/reinstall.

## Previous Platform/Wire/Metrics qualification

Previous graph evidence remains at **`/tmp/blob-latest-1031/`**; its complete
handoff is copied to `/tmp/blob-bhms-1031/previous-handoff.md`. It includes
**27 methods / 137 frozen Rust/TypeScript frames**, malformed/identity/direction/
limit/decimal checks and native schema MSRV qualification. Blob's protocol and
schema sources and their Platform/Wire/Metrics identities are unchanged in this
batch; that focused proof is reused. Previous installed artifacts remain
historical; the current graph has its own fresh builds and installed tests above.
Earlier `/tmp/blob-mpw-next-1031/` and `/tmp/blob-metrics-platform-1031/`
records and failed attempts remain with their existing evidence owners.

The [probe ledger](../evidence/caffeine-probes/README.md#current-browser-rehearsal--2026-10-31)
retains the earlier local browser rehearsal's exact graph and scope. Do not claim
it on the current graph. Optional unresolved Observability comment approval
remains outstanding; no new Observability post occurred.

## Remaining application acceptance

Four Blob issues remain: [#5](https://github.com/dragginzgame/ic-blob-storage/issues/5),
[#6](https://github.com/dragginzgame/ic-blob-storage/issues/6),
[#31](https://github.com/dragginzgame/ic-blob-storage/issues/31),
[#34](https://github.com/dragginzgame/ic-blob-storage/issues/34). #4 closed as
superseded/not-planned tracking after its publication criteria moved into #5;
no production criterion was marked passed. IC Crates
[#1](https://github.com/dragginzgame/ic-crates/issues/1) owns the catalog
migration/transaction, Canic [#622](https://github.com/dragginzgame/canic/issues/622)
its composition and [#494](https://github.com/dragginzgame/canic/issues/494) funding.
See [the coordinated run](../acceptance-plan.md#next-application-acceptance-run).

Caffeine is the only provider. Existing Canic resolves environment/fleet/role
registry targets, not one global Blob ID. Its last-reviewed Blob configurations
were local-only and production application configs selected no Blob role. The
remaining live target is a deployed consuming application/fleet, whose registry
supplies the service binding. Do not substitute historical trial identities,
fixture identities or an assumed paid budget. No provider probe was run here.

Delivered 0.60.0 main has passing [six-job validation](https://github.com/dragginzgame/ic-blob-storage/actions/runs/41111873111)
and [three published-example jobs](https://github.com/dragginzgame/ic-blob-storage/actions/runs/41111873249),
including both macOS architectures; pending 0.61 has no hosted result.
Registration proves no stored bytes, gateway acknowledgment no deletion/billing
cessation, funding acknowledgment no independent credit. Retain provider objects,
balances, uncertain effects and continuing billing through
[retirement](../retiring-installations.md). Same-release exact recovery and restore
fences remain required. Never clear an allocation ledger over retained bytes.
