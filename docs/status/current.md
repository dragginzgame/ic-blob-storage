# Current status — 2026-11-01

## Released baseline and pending scope

The maintainer reports **0.65.0** live. Tag `v0.65.0` is
`5b91fe7006c1132bcf8e89605201d9ba2645c1f4`; its unchanged
[historical receipt](../release.json) binds validated source
`ebd0903889f9400b90055a42518a4980a10e6804`. Publication/hosted CI was not reread.
[Coordinated history](../../releases/history.md) preserves earlier notes once.

Pending local work includes the accepted independent Contracts/Client/Engine
release groups, explicit native host pin and the supplier adoption below.
All thirteen own Cargo versions remain **0.65.0**. Each group now has
**0.66.0 Unreleased** notes because its public supplied Rust identities change;
these matching candidates do not restore coordinated workspace versioning.
No real source staging, commit, tag, push, release, publication, deployment,
sibling edit or paid provider effect was performed. Disposable fixtures alone
create private local Git commits/tags/remotes.

## Published supplier adoption

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
[Root notes](../../CHANGELOG.md) index the group ledgers. Initial receipts explicitly
record inherited0.65.0, not completed group releases. Historical receipts remain
unchanged; no dual reader reinterprets old coordinated plans.

Release/version/plan/resume/publication require explicit `RELEASE_GROUP`. Group
tags/plans have group prefixes. Shared owns selective rendering and transactional
rollback; Blob owns source receipts and checksum/VCS publication readback. Full
release validation remains workspace-wide. Release Contracts first for this
identity cut; inherited requirements then select it for Client/Engine publication.
See [release commands](../releasing.md). Real release targets remain maintainer-only.

Native `publish-map`, `publish-file-status` and `publish-session` require approved
`BLOB_EXPECTED_HOST_RELEASE`, captured once before file/Agent setup and matched
against fresh and retained replies. Make selects the Engine host catalog version;
direct CLI/launcher callers select their deployment. No saved field, default to
Client/Contracts, weaker release check or new read lane was added. CLI `--version`
reports its own package. The earlier group batch's source/qualification evidence
remains `/tmp/blob-release-groups.sC6joH/`, including its complete `groups-final.log`.

## Qualification and evidence

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

Full workspace CI/release gates, native macOS, live IC/provider acceptance and
endpoint performance were not rerun. No size/performance gain is claimed. Service
transitions remain retirement/reinstall-only; retain provider objects, uncertain
effects, balances, released physical bytes and continuing billing obligations.
The frozen archive-inspection candidate remains permanently fenced, as recorded
in `/tmp/blob-simplify064.1L5ZdK/archive-disposition.txt`; its custody record marks
older overwritten logs that cannot qualify a historical release.

Last Canic review: released0.114 selects Blob0.61, local edits select0.63 and still
expect the retired gateway Busy refusal. Its owner retains
[Canic#622](https://github.com/dragginzgame/canic/issues/622) adoption/acceptance.
Last GitHub readback found Blob issues [#5](https://github.com/dragginzgame/ic-blob-storage/issues/5),
[#31](https://github.com/dragginzgame/ic-blob-storage/issues/31),
[#6](https://github.com/dragginzgame/ic-blob-storage/issues/6) and
[#34](https://github.com/dragginzgame/ic-blob-storage/issues/34), no open PRs.
No acceptance criterion was closed. Select real targets/budgets through the
[acceptance plan](../acceptance-plan.md) and
[probe ledger](../evidence/caffeine-probes/README.md); fixtures are not production.
