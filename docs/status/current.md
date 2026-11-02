# Current status — 2026-11-02

## Published 0.68.1

The delegated Contracts, Client and Engine patch delivery is complete. All three
packages are published, indexed and non-yanked on crates.io. Independent archive
checksums, VCS commits and package paths match their exact release tags:

| Group | Tag | Release commit | Validated source |
| --- | --- | --- | --- |
| Contracts | `contracts-v0.68.1` | `dc506009f0bf4d5edeadb01b2308297f6088551e` | `bca695b0fde5941ae41b0f9374c0f59658ccb489` |
| Client | `client-v0.68.1` | `7449b6ac9e06f1b663d16e7b774ea7ab030928d5` | `dc506009f0bf4d5edeadb01b2308297f6088551e` |
| Engine | `engine-v0.68.1` | `2ff86ef40f3e4aeca17772c877a26c8762825505` | `7449b6ac9e06f1b663d16e7b774ea7ab030928d5` |

Each existing runner validated, prepared only its group, committed/tagged and
atomically pushed main plus that tag. Each separate publication ran its required
compiler/Clippy/Cargo archive checks. All thirteen members are now **0.68.1**;
there is still no aggregate workspace version or automatic dependent release.
Resume/publish an older group from its exact tag/tools, never a later descendant.

## Frozen supplier graph

| Supplier | Delivered selection |
| --- | --- |
| Platform / Runtime / exact guard / Types | **0.18.0 / 0.18.1 / 0.18.1 / 0.17.0** |
| Wire / Derive | **0.15.1** |
| Storage | **0.65.3**, owning backing, grants and collections |
| Metrics | **0.27.2**, one selected identity |
| Testkit / PocketIC server | **0.80.2 / 16.1.0** |
| Build / Host Foundation / Host Tools | **0.74.1 / 0.74.2 / 0.78.1** |
| Shared Tooling | **0.27.28**, `8075b3fd4803d560a1be04768f76701ada3ae8fe`, **150 files** |
| Compiler / browser tooling | Rust **1.99.1**, Node **24.21.0**, npm **12.2.1** |

Selections use published packages and the committed canonical Shared export;
mutable sibling VERSION/notes were excluded. Public identities remain unified,
and deployment admission excludes host/schema/test dependencies and retired
Memory/stable-structures. The Caffeine SDK override and application Wire bytes,
method identities, limits, authority and exact retry policy are unchanged.

## Validation and release evidence

Every group passed the full first-plan roster: **688 native tests**, **380
PocketIC tests / 10 documented ignores**, strict Clippy, compiler/pin/boundary
checks, snapshot/tool/hook/metadata/recovery fixtures, formatting, Rustdoc, Wasm
and exact mixed-version package archives. No failing gate was reused.
The generated TypeScript check passed **27 methods / 137 frozen Rust frames**,
including malformed input, identity, direction, limit and decimal checks.

Released Engine **0.68.1** standalone/probe Wasms and the CLI were rebuilt from
the exact tag. Five lifecycle/history/fencing/obligation cases and the signed
installation-carrier case passed with independently selected host release 0.68.1.
Their hashes and logs are retained. These are local installed-IC checks, not
native macOS or live Caffeine acceptance. Hosted native/tooling CI remains
asynchronous; see the [Engine-source run](https://github.com/dragginzgame/ic-blob-storage/actions/runs/41535847989)
and [current workflow](https://github.com/dragginzgame/ic-blob-storage/actions/workflows/tooling.yml).

Evidence: `.tmp/blob0681.cgzub5/` retains the frozen original source/lock,
per-group release/publication logs and registry metadata/archive/VCS readbacks,
Wire checks, released Wasm hashes and installed cases. The isolated checkout is
selected by its `release-checkout.current`; Git's release-state retains each
`<group>-0.68.1.plan` and structured proof. Raw stage logs remain in that
checkout's `.tmp/blob-release-validation.*`; failed logs retain their own paths.
Prior **0.68.0** delivery/qualification remains `.tmp/upstream-platform018.ZS89rK/`;
its source-bound receipts, registry archives, hashes and logs were preserved.
Required release artifacts and failed evidence were not cleaned.

## Validation selection now active

Blob owns `ci/release-validation.json` and `scripts/release/validation.pl`;
Shared admits/checkpoints/executes their complete plan. The three first deliveries
ran every stage and established structured `release-verify-plan-v1` baselines.
Later fully classified changes may omit unchanged publication/recovery fixtures;
only tenant-client-only changes may omit remaining Engine IC scenarios. Other
Client APIs/CLI have wider actual consumers. Shared, manifest/lock, compiler,
recipe, unknown or ambient build-configuration changes retain the full roster.
Configuration changes block preparation; failed/blocked/missing proof cannot pass.

Routine releases retain every original-artifact installation/recovery case and
omit the repeated original/O3/Os/Oz matrix. Linux hosted compatibility and manual
checks retain it. All three native hosts retain CLI/install/archive/reopen and
real metadata rollback checks. Current mapping, evidence and recovery are in
[the release guide](../releasing.md#validation-scope). Historical full-matrix
results do not qualify the new graph; no new optimizer or native macOS result is
claimed by this local delivery.

## Preserved work and service boundary

**Storage 0.65.4 arrived in the original working lockfile after the release graph
was frozen.** Preserve that later edit as unstaged, unqualified work when bringing
the original checkout forward. It is outside the published 0.68.1 qualification;
its original bytes and delta are retained with delivery evidence. Do not fold it
into historical release receipts or overwrite it with the frozen 0.65.3 lock.

Library publication does not deploy a live service. Cross-release installations
require safe retirement and fresh whole backing, preserving provider objects,
uncertain effects, balances and billing custody under the
[retirement contract](../retiring-installations.md). Same-release recovery remains
required. Canic owns its adapter, fleet limits and production rollout. Blob
#5/#31/#6/#34 and Canic #622/#494 retain application/provider acceptance; no live
provider call, paid effect, service reset or sibling edit occurred here.
