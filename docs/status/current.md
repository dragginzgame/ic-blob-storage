# Current status — 2026-10-30

## Source and active adoption

Released source is **0.59.1**, `70eeb697`. The maintainer requested the latest
published Metrics, Platform, Persistence, Tooling/Host and Wire packages plus
Shared Tooling. Pending **0.60.0** reflects the public Platform/Metrics Rust type
identity cut. Workspace package versions remain 0.59.1. No staging, commit,
push, release, publication, deployment, paid effect or sibling edit occurred.

Official crates.io sparse-index entries select **Platform 0.9.1**, **Metrics
0.18.1**, **Memory/Storage 0.56.4**, **Host/Build/Testkit 0.70.5** and **Wire/derive
0.7.25**. Direct minimums and the lockfile match these selections. Published
Platform, Metrics, Memory, Storage and Wire archives were downloaded and their
registry checksums verified before API/source review. Existing incoming compatible
lockfile updates were preserved or superseded by the explicitly requested newer
minor. No source override or unpublished supplier is used.

Platform 0.9 embeds IC0 and the executor and owns selected management DTOs on
Wasm. Public Principal and Metrics type identities require embedding hosts to
align and rebuild. The production graph has one executor, no registry executor,
raw IC0, upstream management DTO package or implicit Candid printer. Blob's owned
Wire identities, record layouts and all schemas remain unchanged. No local
adapter/shim, migration, reset or dual reader was introduced.

Published Persistence/Tooling still require Platform 0.8.1 compatibly, selecting
published **0.8.3** alongside direct 0.9.1. In the deployment graph that older
facade enables only stable structures. Native Testkit additionally needs foreign
Candid/PocketIC facilities. It is not another executor. Full facade unification
awaits supplying releases; no lock/source override bypasses their constraints.
Other runtime consumers, including Timers, must align with Platform 0.9 before
embedding; runtime handles must never cross executors. Deployment admission now
explicitly refuses external `ic0`, `ic-cdk-executor`, upstream management DTOs
and native Candid service export, with rejection fixtures.

Metrics 0.18 adds summary/histogram transport while preserving existing aggregate/
counter transport. Blob consumes no Logging checkpoint or stable metrics store,
so Logging's separate retained-budget cut creates no local storage migration.
Memory/Storage's 0.56 patch series preserves the current Wire ledger and custody.

## Shared Tooling

Adopted reviewed **0.27.11**, source
`62edfbd43087a718bf0283e9040676a49211976d`, verified against the remote HEAD;
the clean committed exporter selected **136** files. Impact and exact refresh
receipts are retained. Captured validation checkpoints, coordinated Cargo
admission, structured graph observations and updated engineering rules are
included. Existing grouped dependency formatting/release policy remains active.
The newly supplied publication adapter is opt-in; Blob's existing publication
roster, validation and recovery policy remain local. No Rust `#[path]` wiring or
attribute lint allowances were found in maintained Rust sources. Scoped existing
expectations remain checked by strict lint; no blanket conversion was needed.

Published browser Platform remains **0.1.1**, selecting SDK Core **6.1.1**;
Caffeine **1.1.2**, Node/npm selections and browser package files are unchanged.

## Qualification

Prior adoption evidence: `/tmp/blob-upstream-refresh/`, captured with direct
Platform 0.9.0 / Metrics 0.18.0. Focused strict Clippy passes for core,
Contracts, client, CLI and standalone across targets/features, plus standalone,
journey and storage integration targets. Native tests pass: core **338**,
Contracts **125** plus frozen owned Wire vectors, client **6**, public memory
composition **2**, and CLI **122** including local HTTP cases. Rust **1.91.0**
native library and standalone Wasm checks pass. Fresh standalone, authority,
gateway-source and funding release Wasm and native CLI builds pass. Testkit server and Build
release editor setup/admission select locked 0.70.5.

Actual Testkit/Make forwarding, runtime/deployment graph checks and new forbidden-
runtime fixture cases pass. Hook preservation and grouped formatting checks pass.
The first release-adapter attempt could not inspect a shared tool lock in the
sandbox; its fixture and logs remain. The retry uses normal access to the same
prepared tool store, without deleting a lock. All isolated release-adapter cases
pass, without real commits, pushes or publication. Installed tests pass: **14**
standalone lifecycle, metrics, certificate, gateway, account and upload cases;
**33** journey/recovery cases; and **3** funding refund/trap cases. These **50**
cases use local IC/provider substitutes, not deployed provider acceptance.
Two standalone heap-metrics unit cases also pass. Generated TypeScript checks
cover **27 methods / 137 frames**, including malformed packets, identity,
direction, limits and decimal handling. Captured input hashes remain unchanged;
formatting, snapshot integrity, documentation links and diff checks pass.
No complete repository gate, new native macOS run, deployed IC or live provider
qualification is claimed. No size/performance gain is inferred.

Latest patch qualification: `/tmp/blob-metrics-platform-refresh/`. Published
Platform 0.9.1 and Metrics 0.18.1 archive checksums match the official index;
both packaged runtime source trees are identical to the preceding patches.
Platform moves the executor singleton guard into an exact-version optional
helper, while Metrics aligns its Wire minimum. The incoming lockfile already
selected both patches and was preserved; direct minimums now match. Strict
standalone Clippy, two heap-metrics unit cases, Rust 1.91 standalone Wasm check,
fresh release Wasm builds and both dependency-boundary checks pass. Six installed
cases pass against the fresh standalone Wasm: metrics authentication/replays/
heap reset and five lifecycle/recovery/fence cases. Formatting, documentation
links and diff checks pass; captured manifests, lockfile, snapshot and Wasm hashes
remain unchanged during qualification. Earlier broader evidence above belongs
to its captured graph; full CI/macOS and live deployment were not rerun.

## Issue reconciliation and remaining acceptance

[#110](https://github.com/dragginzgame/ic-blob-storage/issues/110) and
[#111](https://github.com/dragginzgame/ic-blob-storage/issues/111) are closed from
delivered 0.59.1 after all six [hosted jobs](https://github.com/dragginzgame/ic-blob-storage/actions/runs/41047479232)
passed: core/client, MSRV, Linux, both macOS architectures and generated clients.
That CI result qualifies its captured release, not this pending dependency cut.
Prior CLI evidence remains `/tmp/blob-cli-cleanup/` and in the
[released handoff](https://github.com/dragginzgame/ic-blob-storage/blob/70eeb697/docs/status/current.md).

[#4](https://github.com/dragginzgame/ic-blob-storage/issues/4),
[#5](https://github.com/dragginzgame/ic-blob-storage/issues/5),
[#6](https://github.com/dragginzgame/ic-blob-storage/issues/6),
[#31](https://github.com/dragginzgame/ic-blob-storage/issues/31) and
[#34](https://github.com/dragginzgame/ic-blob-storage/issues/34) retain application/
provider acceptance under [the coordinated run](../acceptance-plan.md#next-application-acceptance-run).
No current application origin, deployed service/configuration or paid trial budget
is selected. Registration proves no stored bytes; gateway acknowledgment proves no
physical deletion/billing cessation; funding acknowledgment is not independent
credit. Canic owns its adapter/composition and adopts public dependency cuts there.

Cross-release transitions remain [retirement/reinstall](../retiring-installations.md),
retaining provider objects, balances, uncertain effects and continuing billing.
Same-release interruption recovery, exact retries and restoration fences remain.
Never clear an allocation ledger while leaving collection bytes behind.
