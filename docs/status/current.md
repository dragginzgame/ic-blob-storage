# Current status — 2026-11-02

## Released baseline and scope

The compiler-floor change in [#112 — Adopt Rust 1.99.1 and consolidate compiler qualification](https://github.com/dragginzgame/ic-blob-storage/issues/112)
is delivered on main. All three affected groups are released and published at
**0.67.0**, with Rust **1.99.1** as their consumer minimum and exact repository
compiler pin:

| Group | Release tag | Release commit |
| --- | --- | --- |
| Contracts | `contracts-v0.67.0` | `430cc5cb9c6adfc4d9729d93e11e7259c0cc4811` |
| Client | `client-v0.67.0` | `3613147d5982389f5dee19aaa9476c27d9c5a5e4` |
| Engine | `engine-v0.67.0` | `7a0272121cfb593ed0dcf6f5f254036dc7d8a592` |

Each normal group runner validated, prepared, committed/tagged and atomically
pushed its own release. Registry metadata and checksummed archives match the
exact release tags, package paths and Rust floor. Matching group numbers reflect
three consumer support-floor cuts; future groups remain independent. No service
installation, application deployment, provider request or paid effect occurred.

## Compiler qualification and remaining acceptance

All thirteen members inherit the same support floor. Compiler admission reads
the owned exact pin and checks actual verbose Cargo/rustc identities plus every
member's effective floor. CI/release validation and publication invoke it;
negative fixtures cover mismatches and moving/duplicate pins. The old 1.91 job
was removed while retaining isolated public native/Wasm graphs, Clippy, frozen
Wire, three native hosts, archives and installed-runtime obligations.

All three complete local release gates passed: Contracts **1316s**, Client
**1216s**, Engine **1310s**. Each ran **687 native** and **380 PocketIC** passing
tests, with **10 explicit ignores**, plus strict Clippy, boundary/tool/pin checks,
release/hook fixtures, formatting, Rustdoc, Wasm, original/O3/Os/Oz installation
and restoration, and extracted package/archive integrity checks. Each separate
publication ran compiler admission, Clippy and packaged-crate verification.

Fresh native CI on source `ede2241e` passed Linux and Apple Silicon, including
archive/native CLI and original/optimized Wasm qualification; library and Wire
schema jobs passed too. Intel macOS had reached archive qualification at the last
observation. The Engine release starts a new hosted run. **Issue #112 remains
open until its latest applicable native acceptance is complete**; pending or
cancelled runs are not passes. Follow the current issue for the latest run links.
The published-example installation run
[41426291724](https://github.com/dragginzgame/ic-blob-storage/actions/runs/41426291724)
passed at the Contracts release source. No live-provider qualification is claimed.

Two CI defects found during delivery were fixed: prepare the locked cache before
offline fixtures on fresh runners; make compiler-fixture assertions explicit on
macOS Bash 3.2. Failure collection now includes the completion harness's parent
directory. Historical failure/source observations remain distinct from subsequent
passes. The first local Contracts attempt was stopped before version preparation
to fix the cache ordering; no partial result was reused.

## Evidence custody and preserved pending work

The original checkout's pending supplier upgrades and direct Client Types/Runtime
migration were excluded from 0.67.0. Their files and hashes are backed up under
`.tmp/issue112-delivery-recovery.8cQudg/`; delivery uses the committed 0.66 supplier
graph below. Those pending changes have no release qualification of their own.
The original working copy is reconciled separately without staging that work;
its Client notes belong after the published 0.67.0 entry.

The earlier `/tmp/blob112-delivery.ffJg3y` checkout and detailed local logs became
unavailable after a runtime transition. Contracts/Client immutable delivery and
registry archives survived and were independently reverified. Engine used a
fresh full gate in `.tmp/issue112-delivery-recovery.8cQudg/repo`, with release and
publication logs beside it and a retained standard Engine plan under `.git`.
No lost partial Engine validation was reconstructed or claimed as a pass.

The revision-bound Shared policy is `e84252f5740bacd708359580455ea2b6fd67df38`.
The executable Shared **0.27.26 / 144-file** snapshot is unchanged. Sibling
repositories remain read-only. Canic's recorded 0.66 local integration evidence
is retained by its owner; it does not qualify a new source/graph automatically.
Current application/provider acceptance remains with Blob #5/#31/#6/#34 and
Canic #622/#494. Preserve all provider objects, uncertain effects, balances and
continuing-billing obligations across retirement/reinstall boundaries.

## Supplier graph retained from 0.66.0

Official crates.io index records and checksummed published archives establish:

| Supplier | Selected version |
| --- | --- |
| Build / Host Foundation | 0.73.6 (already locked; catalog minimums aligned) |
| Host Tools | 0.77.0 |
| Testkit | 0.78.0 |
| Memory / Storage | 0.62.0 |
| Metrics | 0.25.0 |
| Platform / owned native PocketIC client | 0.17.0 |
| Runtime / macros / exact guard | 0.17.1 through Platform |
| Canonical Types | 0.16.0 through Platform |
| Wire / Derive | 0.14.1 incoming lock; minimum0.14.0 |

Shared **0.27.26**, commit `022480c1aae2ed28a7b37247bab0a55d6dcf9ca6`, is
still the latest committed source reviewed in the sibling. The existing canonical
144-file snapshot already selects it; no new exporter refresh or store reclamation
was needed. Dirty sibling source is read-only and is not a consumer dependency.

Published Wire and Testkit now resolve the former Principal/schema and PocketIC
client blockers under [Platform#131](https://github.com/dragginzgame/ic-platform/issues/131#issuecomment-6560257932)
and [Wire PR41](https://github.com/dragginzgame/ic-wire/pull/41). All affected
callers compile without bridges, copied helpers, aliases or source patches.
The root continues to import through Platform. Native tooling uses its owned
PocketIC client; the old registry `pocket-ic` package leaves this graph.

The resolved graph admits one Runtime/guard, Types, Wire and Metrics identity.
Testkit0.78 selects Wire0.14 and arithmetic-only Metrics0.25, removing the older
Metrics0.24.6 lane without overrides. Contracts alone exclude Runtime. The native CLI links
the replicated client's runtime transport types, while excluding Engine/storage.
Boundary checks now reject the extracted Runtime from Contracts and the owned
native PocketIC package from deployment, with mutation coverage.

The maintainer confirmed the incoming **thiserror2.0.21** lock edit is settled.
It is preserved with its derive version; the root's compatible minimum remains
2.0.18. The obsolete registry-PocketIC exact-pin comment is removed. All other
unrelated package identities were preserved. There is one workspace/lock and no
independent consumer workspace to update.

## Release groups and integration contracts

[Group policy](../../ci/cargo-release-groups.json) owns Contracts alone, Client
with CLI, and Engine with standalone, six probes, protocol and PocketIC support.
Members have literal group versions and inherit the common dependency catalog.
[Root notes](../../CHANGELOG.md) index the group ledgers. Each group's receipt
now records its completed 0.67.0 release, validated parent and exact file hashes.
Read receipts at their selected group tag: later groups change shared Cargo
metadata and cannot stand in for an earlier release checkout. Historical receipts
remain unchanged; no dual reader reinterprets old coordinated plans.

Release/version/plan/resume/publication require explicit `RELEASE_GROUP`. Group
tags/plans have group prefixes. Shared owns selective rendering and transactional
rollback; Blob owns source receipts and checksum/VCS publication readback. Full
release validation remains workspace-wide. Contracts was released and published
first, then Client, then Engine. See [release commands](../releasing.md).

Native `publish-map`, `publish-file-status` and `publish-session` require approved
`BLOB_EXPECTED_HOST_RELEASE`, captured once before file/Agent setup and matched
against fresh and retained replies. Make selects the Engine host catalog version;
direct CLI/launcher callers select their deployment. No saved field, default to
Client/Contracts, weaker release check or new read lane was added. CLI `--version`
reports its own package. The earlier group batch's source/qualification evidence
remains `/tmp/blob-release-groups.sC6joH/`, including its complete `groups-final.log`.

## Historical 0.66.0 qualification and evidence

Current evidence: `/tmp/blob-upstream-adopt.EGqoV3/` contains before-state,
official index/archive/source receipts, exact lock deltas, commands and logs.
The accepted graph was rechecked after the incoming thiserror edit. Affected
public/CLI/PocketIC native Clippy and seven canister Wasm Clippy profiles pass.
Contracts132, Engine297, replicated Client6, memory composition2 and frozen Wire
surface tests pass. Generated TypeScript passes27 methods/137 frozen Rust frames
plus malformed, identity, direction, limit and decimal checks. Rust1.91 native
public libraries and standalone Wasm pass. No frozen vectors were regenerated.

The preceding Testkit0.77 managed CLI was explicitly prepared/admitted; the PocketIC16.1.0
server SHA-256 is unchanged. Eight installed cases pass: five lifecycle/history/
obligation checks, native session recovery/tampering, refused backing growth with
exact retry, and replicated consumer trap/recovery. All seven linked canister
artifacts remain byte-identical after the final settled-graph build. Failed
sandbox Node/npm observation is retained; its native prerequisite/schema rerun
passes without installing/updating npm packages. No live provider was contacted.

The follow-up Testkit-only adoption is `/tmp/blob-testkit-adopt.wkYYEK/`:
official non-yanked0.78.0 index/archive checksum, unchanged Rust source compared
with0.77, and exact lock comparison. Only Testkit advances and Metrics0.24.6 is
removed. The pre-existing Wire/Derive0.14.1 selection and all other identities
are preserved; broader Wire0.14.0 qualification above is not a fresh0.14.1 Wasm
qualification. Focused native Clippy, Rust1.91, tooling admission and installed
stop/start/repeated-upgrade fencing all pass. The server and retained Wasm hashes
are unchanged. No runtime/public API shim is needed.

Each actual group release passed its full workspace CI/release gate: 1223s for
Contracts, 1210s for Client and 1154s for Engine. Each ran 687 passing native tests,
380 passing PocketIC tests and 10 explicitly ignored cases, plus strict Clippy,
snapshot/tool/pin and deployment boundaries, shell/hooks/formatting, Rustdoc,
Wasm compilation, original/O3/Os/Oz installation/restoration, release adapters
and exact extracted archive tests/integrity checks. No gate failed or was skipped.
Each publication also passed its own Clippy and Cargo packaged-crate verification.
Plans remain under Git's `release-state/<group>-0.66.0.plan` paths.

Release gates validate pre-version source. Retained canister builds can still
report Engine 0.65.0; rebuild at the released Engine tag before any deployment.
Native macOS, live IC/provider acceptance and endpoint performance were not rerun.
No size/performance gain is claimed. Service
transitions remain retirement/reinstall-only; retain provider objects, uncertain
effects, balances, released physical bytes and continuing billing obligations.
The frozen archive-inspection candidate remains permanently fenced, as recorded
in `/tmp/blob-simplify064.1L5ZdK/archive-disposition.txt`; its custody record marks
older overwritten logs that cannot qualify a historical release.

Current Canic review: its released baseline remains0.114.0. The recorded local
Blob0.66 family passes dedicated installation/recovery, all five embedded funding
profiles and generated clients for both shapes, with the corrected held-reply
fixture budget. Source, locks and logs stay with Canic at
`target/integrations/checks/blob66-20261102/results.json` and its working Blob audit.
[Canic #622](https://github.com/dragginzgame/canic/issues/622) owns current delivery;
[#494](https://github.com/dragginzgame/canic/issues/494) retains funding delivery and
deferred independent-credit evidence, not a happy-path payment prerequisite.
Current root-lock drift and committed source are noted above.
Latest GitHub readback also found the new compiler issue #112, plus Blob
issues [#5](https://github.com/dragginzgame/ic-blob-storage/issues/5),
[#31](https://github.com/dragginzgame/ic-blob-storage/issues/31),
[#6](https://github.com/dragginzgame/ic-blob-storage/issues/6) and
[#34](https://github.com/dragginzgame/ic-blob-storage/issues/34), no open PRs.
No acceptance criterion was closed. Select real targets/budgets through the
[acceptance plan](../acceptance-plan.md) and
[probe ledger](../evidence/caffeine-probes/README.md); fixtures are not production.
