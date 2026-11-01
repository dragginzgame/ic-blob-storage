# Current status — 2026-11-01

## Released baseline and pending scope

Release **0.64.0**, tag `v0.64.0`, is at
`8f33b9645ae57e9103962f09a8da91a7e655070b`. Its
[receipt](../release.json) identifies validated source
`f0115b4a725d5b5b72ebd46916e7d3bbe783deea`. The maintainer reports it live;
registry publication and hosted CI were not independently reread during adoption.
The root-claim consolidation, gateway supersession cut, saved-source format and
nested logger fixture fix are included in [0.64 notes](../../CHANGELOG.md).

Pending notes target **0.65.0** because Memory's public identity and grant lookup
callback change. Own Cargo versions stay **0.64.0** for maintainer preparation.
No staging, commit, tag, push, release, publication, deployment, sibling edit or
paid provider effect was performed in this adoption.

## Memory/Storage and Wire adoption

Official crates.io sparse-index records and cached archive checksums verify
published **Memory/Storage 0.61.0** and **Wire 0.13.5**. Persistence's sibling
0.61.1 draft is not a published selection. All three root minimums are updated;
the narrow resolution changed only Memory/Storage from the incoming lockfile.
Preserve its existing Build/Host Foundation 0.73.6, Host Tools/Testkit 0.76.2,
Metrics 0.24.5 and Wire/Derive 0.13.5 selections. Other declaration minimums and
own package versions are unchanged.

- Fresh owning collections use Storage's `create`; retained collections use
  `load`. The substrate composition test no longer uses initialize-or-load.
  Only two deliberate corruption fixtures call `reset`; production restore has
  no reset, initialization fallback, repair or automatic migration.
- `stores::grants::open` now takes a lookup over `&ic_memory::StableKey`.
  Blob parses its canonical built-in keys during grant opening; hosts use the
  same published Memory identity and its checked-key runtime API. Bucket setup
  uses `NonZeroU16` without the old fallible configuration constructor.
- Durable keys, fourteen installation grants, record/frame identities, canonical
  bytes, decoder budgets, authority and exact retry bindings are unchanged.
  Wire's upstream cost measurements do not establish Blob endpoint savings.

Hosts must align Memory/Storage and rebuild their grant and bucket setup; see
[dependency ownership](../dependencies.md) and
[service composition](../service-contract.md). Valid byte compatibility in the
supplier does not authorize a cross-release service upgrade. Complete
[retirement/reinstall](../retiring-installations.md) while preserving provider
objects, balances, uncertain effects and continuing billing obligations.

## Validation and evidence

This adoption's evidence is `/tmp/blob-memory-storage-wire.eu8QNC/`: official
index records, original incoming manifests/lock/handoff, narrow lock diff,
source diffs, exact commands, results and artifact hashes. Failed lint and the
initial corruption-fixture run remain separate from corrected passes.

Locked offline metadata, pins, direct/deployment/default-client boundaries and
their refusal fixtures pass. Strict Clippy passes for affected Core/Contracts/
Client/CLI/standalone/probe/protocol/PocketIC selections. Core 297, both memory
composition cases, Contracts unit/integration/frozen-vector cases, twelve fixture
journal cases, five storage-probe cases, CLI 122 plus eight boundary cases and
Client ten plus one documentation case pass. Core's two documentation cases pass.

Rust 1.91 native Core/Client/protocol and standalone Wasm checks pass. Fresh native
CLI and four release Wasms build. Generated TypeScript passes all 27 methods and
137 frozen Rust frames, including malformed/identity/direction/limit checks.
Scoped formatting passes. The lock-selected Testkit 0.76.2 CLI is explicitly
prepared and admitted with PocketIC 16.1.0 before installed checks.

Fresh PocketIC qualification passes twelve cases: backing-growth and write
rollback, manifest atomicity, gateway supersession, admission bindings, five
lifecycle/recovery cases, corrupt restoration and signed gateway CLI decisions.
Exact commands, `installed-results.tsv` and `artifacts.sha256` retain that proof.

The release gate, native macOS/Windows, deployed IC/Caffeine, registry consumer
installation and endpoint performance were not qualified by this focused pass.
Fresh qualification artifacts still carry source package version 0.64.0; they
are not a released 0.65 deployment image.

Prior evidence remains with its owners: `/tmp/blob-deps-adopt.ZKYN6N/` for the
preceding dependency adoption, `/tmp/blob-nested-logging-fix.AwXAaL/` for the
parent-logged tooling stage, and `/tmp/blob-simplify064.1L5ZdK/` for the cleanup.
Its `evidence-custody.txt` records overwritten older temporary logs in
`/tmp/blob-cleanup-1101`; do not use those to qualify the historical release.
The archive-probe candidate remains: disposable permanently fenced inspection
retains unique hash/read/billing cases, not a second live service owner. See the
cleanup's `archive-disposition.txt`; standalone restoration is separately tested.

## Next: Canic adoption and bounded acceptance

The last read-only Canic review found released 0.114.0 (`e75378595`) selecting
Blob 0.61, with local edits selecting 0.63. Its gateway fixture still expected
the retired Busy refusal. Its owner must adopt the released catalog/memory cut
across the root and both consumer graphs, then qualify dedicated/embedded mounts,
generated clients and recovery under
[#622 — Mounted Blob acceptance](https://github.com/dragginzgame/canic/issues/622).
No sibling build or edit was performed during this adoption.

Four Blob acceptance issues remain:
[#5 — Publication and serving](https://github.com/dragginzgame/ic-blob-storage/issues/5),
[#31 — Multi-user uploads](https://github.com/dragginzgame/ic-blob-storage/issues/31),
[#6 — Deletion/billing](https://github.com/dragginzgame/ic-blob-storage/issues/6),
[#34 — Payer linkage and funding](https://github.com/dragginzgame/ic-blob-storage/issues/34).
The last GitHub readback found no open PRs. This adoption closes none of their
provider/application acceptance criteria. The maintainer identifies Caffeine as
the provider; select the actual application/service/network and bounded budget
under [the acceptance plan](../acceptance-plan.md#before-the-acceptance-deployment)
and [probe ledger](../evidence/caffeine-probes/README.md). Historical trial
principals and fixture identities are not production targets. A provider-only
trial may precede a finished frontend; deployment and paid effects need their
own established authorization. No new provider probe was run.
