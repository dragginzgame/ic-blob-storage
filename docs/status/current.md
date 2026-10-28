# Current status — 2026-10-28

## Released baseline and pending batch

Default main remains Blob **0.52.2**, `67dd8a16467ccf7d85940606e9baa02fe2a6f1d2`,
with validated source `3ad18831a217fc42ba3ac951ab3125c9e6694bb9`.
The pending **0.53.0** hard cut adopts published upstream packages. Blob package
versions remain 0.52.2; no staging, commit, push, release, publication or deployment
was performed. Incoming Tooling/Platform/Wire lock edits and prior handoff work
were preserved; the narrow resolver operation additionally updated Memory/Storage.
The maintainer committed that batch locally as `1adfd337` before the release gate.
Its test stage failed; no 0.53 release is established by that commit label.
The journal-custody repair below is uncommitted.

Selected published graph: **Memory/Storage 0.53.0, Wire 0.7.19, Platform 0.7.3,
Tooling/Host/Build/Testkit 0.69.6**. Metrics 0.16.3 and browser Platform 0.1.1 are
already current and unchanged. SDK Core 6.1.1/Caffeine 1.1.2 are unchanged.
Registry metadata and checksum-verified package sources, original manifests/lock,
commands' logs and artifacts remain in `/tmp/blob-memory-refresh.HLjFtLrY/`.
The earlier 0.52.0 adoption evidence remains in `/tmp/blob-upstreams.VRGLGMIc/`.
No sibling source or unpublished path dependency was used.

## Memory boundary and migration

Memory runtime constructors now require `MemoryBacking`. Standard VectorMemory
and canister stable backing support it directly; custom wrappers must delegate
one retained identity across aliases. The storage probe delegates its raw backing
identity and retains each exclusive outer grant in an Rc for nested instrumentation.
A regression checks shared identities for aliases and distinct identities for grants.
Private journal construction now carries the same bound; ordinary store/workflow
parameters remain `Memory` where no backing constructor is needed.

Every component collection/detached handle must drop before replacing the host
runtime. The composed-store test now releases its neighbor; the public composition
case proves duplicate-owner refusal without writes, final-lease release and exact
reopening of both components. Corruption fixtures release their old lease before
reopening so they exercise cell validation rather than only custody refusal.

MGR, ledger, cell, Blob schemas and Wire bytes are unchanged; owned schemas stay 1.
Cross-release Blob transitions still require [retirement/reinstall](../retiring-installations.md),
retaining uncertain effects, provider objects, balances and billing obligations.
Same-release interruption/recovery and exact retries remain required. Memory 0.53
trusts exclusive custody between construction, initial bootstrap, lease-open and
growth checkpoints. Size/read/write check poisoning and bounds without metadata
scans; prohibited raw changes need not be detected before the next ordinary IO.
Detected divergence still poisons every handle. No endpoint gain or isolation
from arbitrary raw IO/dishonest backing identities is claimed.

## Current qualification

The graph was admitted with locked offline full metadata, declaration/deployment
boundaries and pins before lint/artifact checks. It contains one Memory, Storage,
Platform and executor identity (executor 2.0.0). Strict affected-package Clippy
covers all thirteen members; changed Rust and grouped manifests format cleanly.
Testkit/release-editor 0.69.6 setup and offline admission pass through existing
owners; PocketIC remains 16.1.0. Cargo inputs stayed fixed during final tests.

Core, client, Contracts, private journal/probe and native CLI regressions pass,
including the corrected grant/corruption cases. Rust 1.91 runs the two public
memory-composition cases and builds all probe Wasms. The default standalone was
built separately from probes to exclude their unified test-support features.
Its SHA-256 is `d18b1c839b86b1fc948ab357fe8028b87d586684717ab12883794059113ffcc5`.

**33 distinct installed PocketIC cases** pass: growth rollback, restoration reads,
tenant clients, replicated queries, funding/refunds/callback uncertainty, default
standalone lifecycle recovery and exact exports. Generated TypeScript passes all
27 methods and 137 frozen Rust frames, including malformed/direction/limit checks.
Public memory docs build with warnings denied. Initial failed and empty-filter
attempts remain alongside corrected nonempty runs; no known failing case remains.
No full release gate, native macOS or deployed-provider run is claimed for this graph.

## Release-gate journal repair

The full test gate at local source `1adfd337` exposed 21 failing journey cases:
authority archive inspection reopened a FixtureJournal before dropping its old
backing owner. Memory correctly returned BackingAlreadyClaimed. The failed log
remains `.git/release-state/validation-failures/20261028T131735Z-780359-15-test.log`.

Authority, gateway-source and funding journal reopen paths now explicitly drop
the old store before constructing its replacement. Archive queries still reread
retained stable evidence; no cached-read substitution, guard bypass, format
change or record reset was introduced. The consumer initializer already requires
an empty host and needs no replacement path.

All **49 journey cases** now pass, including all 21 reported failures, along with
six funding-recovery cases. Strict Clippy for the three changed probes and their
Rust 1.91 Wasm builds pass; Cargo inputs stayed fixed. Evidence:
`/tmp/blob-journal-custody.TEliOFmZ/`. No full gate or release command was rerun.
Commit this repair before the maintainer restarts the normal release gate.

## Issues and hosted CI

Six Blob issues remain; no open PR was observed. #105/#106 are closed after released
[0.52.2 CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/40681041454)
completed successfully on Linux, both macOS architectures, MSRV, core/client and
generated-client lanes. Actual hosted logs show 339 core and 6 client tests. The
recovery implementation and installed regression are unchanged from 0.52.1;
their original interleaving/refusal/obligation evidence remains retained. This
hosted result does not qualify the dirty 0.53 graph.
#107 is closed from checksum-verified published 0.52.1 license/archive evidence at
`/tmp/blob521-issues.7XiLpB8r/`; that old artifact retains its original scope.

#108 retains installed bounded-timeout/unknown-response qualification. Platform
0.7.2 and optional `ic-call` 0.7.2 are published; publication is no longer a blocker.
Existing raw calls remain supported and already cover Blob's static/borrowed
method bindings, so no extra custody dependency was introduced. Preserve domain
codecs, authority, explicit waits, exact attachments/refunds and single-attempt
execution. Native rejection classification does not prove an installed timeout.
The client still gets Candid through Contracts' external provider/certificate/account
contracts, not the raw executor. Producer #75 retains its own qualification scope.

#4/#5/#6/#31/#34 need actual application/provider evidence. The
[coordinated acceptance run](../acceptance-plan.md#next-application-acceptance-run)
includes media-map recovery and reference/deletion/billing steps. Procedures,
local substitutes and package publication are not live acceptance.

## Consumer and retained evidence

Canic was not reinspected in this batch. The last review was 0.112.0 at
`5c236ab130c0d63892ef281b45bdc3fa353e012f`, selecting Blob 0.51.0. Its owner must
adopt the selected Blob/Memory cut and qualify dedicated/embedded composition.
Keep acknowledged funding separate from independent credit; registration does
not prove stored bytes. The [0.52 record](../evidence/simplification0520.md) retains
its original simplification measurements; the installation still has fifteen grants.
Prior #105/#108 and hook evidence remains in `/tmp/blob-issues105-107.zRpGdxat/`,
`/tmp/blob108-qualification.9lvipGpf/` and `/tmp/blob-license-hook-fix.KoY5miUj/`.

Shared **0.27.3**, `cc81a58a5b2dd287c577cbffd2c576c178b6785c`, is unchanged (131 files).
[Platform #50](https://github.com/dragginzgame/ic-platform/issues/50) and
[Persistence #136](https://github.com/dragginzgame/ic-persistence/issues/136) retain
their supplying acceptance scopes. No producer closure or fleet adoption is inferred.
