# Current status — 2026-10-29

## Current batch and source identity

HEAD is the maintainer's commit **d04efc4d** (message `0.53.2`), which arrived
during adoption. Cargo package versions still say **0.53.1**; no later release or
publication was verified. Pending notes now target **0.54.0**, because publicly
re-exported Memory types change package identity. Keep the prior telemetry work
in this minor batch; the agent did not stage, commit, bump versions or release.

Published **Memory/Storage 0.54.0** are selected with root-owned minimums. Official
archive checksums match crates.io, and their implementation sources match 0.53.1.
Stored formats, allocation custody and Blob schemas are unchanged. Embedding
hosts must align their direct Memory/Storage dependencies with Blob's identity.
Cross-release retirement/reinstall remains required; no migration or reset was
added. Persistence's published `ic-metrics-stable` 0.54.0 now uses Metrics 0.17,
removing the earlier numeric-type mismatch for Canic's optional stable backend.
Blob itself gains no stable metrics store dependency.

The pending telemetry slice remains under
[#109](https://github.com/dragginzgame/ic-blob-storage/issues/109): six fixed
heap rows, operator-only standalone Wire report and signed `blob-storage metrics`.
Metrics 0.17 owns numeric transport/validation, replacing local numeric DTOs.
Blob retains attribution, authority, units and lifecycle; Canic owns its adapter
and storage policy. See [the metrics guide](../metrics.md). Only lifecycle init
resets totals. Platform versions also advance on ordinary updates/callbacks;
neither versions nor timestamps detect snapshot rollback or prove continuity.
Metrics never authorize retries, clear fences or replace financial records.
#109 remains open pending verified delivery and remaining acceptance.

Current graph: **Memory/Storage 0.54.0**, **Metrics 0.17.0**, **Platform 0.7.4**,
**Tooling/Host/Build/Testkit 0.69.10**, **Wire/derive 0.7.21**. Tooling 0.69.10
arrived in the maintainer's lock; it was preserved and explicitly fetched.
Matching Testkit CLI/server and Build release editor are prepared. Root Tooling
minimums remain 0.69.9. Browser Platform 0.1.1, SDK Core 6.1.1/Caffeine 1.1.2
and Shared Tooling 0.27.3 (`cc81a58a5b2dd287c577cbffd2c576c178b6785c`, 131 files)
remain unchanged. Sibling checkouts stayed read-only; no paid effects occurred.

## Latest focused qualification

Evidence is `/tmp/blob-persistence-refresh.YSJYAU/`: published source/checksum
review, scoped update/fetch logs, native Clippy for core/standalone/storage probe
and CLI (including package verification), **339 core tests**, and both public
memory-composition cases on actual **Rust 1.91.0** pass. The native core graph is
unchanged by the incoming host-tool update. Fresh release standalone/storage-probe
Wasms build, and Rust 1.91 checks the standalone Wasm graph.

Nine installed PocketIC cases pass: five lifecycle/recovery cases retaining all
owner fences and physical/billing obligations; snapshot rollback refusal; two
metrics/actual signed-CLI cases; and backing-growth refusal with neighbor/admission
rollback followed by exact retry. These use the current Testkit selection and
fresh Wasms; provider substitutes remain separate from deployed evidence.
Direct/deployment boundaries pass, including one Memory identity and exclusion
of host/schema/Testkit features from deployed Wasm. Manifests/lock stayed fixed
through the final installed checks. Formatting, local links and diff checks pass.
No full gate, native macOS or installed-IC/provider qualification was run.

The initial offline boundary attempt refused missing incoming Build 0.69.10;
its log is retained. Explicit locked fetch/setup resolved preparation without
changing selections; the boundary subsequently passes. Earlier evidence remains
`/tmp/blob-tooling0699.ZMrc8f/`, `/tmp/blob-metrics017.0ia0hl/` and
`/tmp/blob109.cvHNQx/`, scoped to their original graphs. The prior handoff is
retained with current adoption evidence. No build or recovery evidence was deleted.

## Canic provider policy and owner integration

Accepted policy: heap by default, immutable stable overrides per provider, one
host-governed map and shared recording/projection API. Metrics owns arithmetic;
Persistence's `ic-metrics-stable` owns stable codec/store. Components own semantic
attribution. Quotas, balances, liabilities and fences remain authoritative records.

[Canic #707](https://github.com/dragginzgame/canic/issues/707) and
[draft PR #708](https://github.com/dragginzgame/canic/pull/708) track the contribution.
Its isolated clone is `/tmp/canic-metrics-storage-policy`, topic branch
`feat/provider-metrics-storage`, source `86c23f97a6a01c8f3c39dac2be7ff573d3bb813d`,
based on verified Canic 0.112 main `5c236ab130c0d63892ef281b45bdc3fa353e012f`.
Its original graph uses Metrics 0.16.3 and Persistence 0.51.13, not this adoption.
Owner integration must preserve relocation to `crates/canic-blob-service`, align
Memory/Storage and Metrics Stable 0.54 with Metrics 0.17, and preserve #599 quality work. Never
replace the newer owner's locks with the older isolated contribution's locks.

Original provider/configuration, strict Clippy, Rust 1.91 library/Wasm and dedicated
heap/embedded stable composition evidence belongs to that clone and its PR;
earlier failures remain `/tmp/canic-metrics-storage-*.log`. Its snapshot report
relabels windows with the current platform version; the
[owner correction](https://github.com/dragginzgame/canic/pull/708#issuecomment-6490479772)
is required before claiming interval comparison semantics. Published Tooling
0.69.9 supplies [#263's snapshot/timer repair](https://github.com/dragginzgame/ic-tooling/issues/263).
The latest read-only Canic inspection already found Tooling 0.69.9 and stop/start
wiring in both Root baseline owners; complete-set semantics and the retained
sequence still require owner qualification.
Blob adoption and direct snapshot tests do not qualify those cached baselines.
No other provider is silently adopted, and there is no runtime storage switch,
automatic migration, new recovery controller or implicit core endpoint ownership.

## Released closures and remaining acceptance

[#108](https://github.com/dragginzgame/ic-blob-storage/issues/108) is closed from
released default main 0.53.1 after all six
[hosted jobs](https://github.com/dragginzgame/ic-blob-storage/actions/runs/40725645496)
passed, including both macOS architectures and the actual bounded-timeout/no-resend
case. #105/#106 closed from released 0.52.2 CI; #107 from published license archives.
Persistence #137's backing-claim lifetime fix shipped in 0.53.1 and both macOS
archive lanes subsequently passed. Original evidence remains
`/tmp/blob-persistence-adoption.TlBKfyOu/`, `/tmp/blob108-timeout.KeNYrDMh/`,
`/tmp/blob053-*-packages.zip` and `/tmp/blob-journal-custody.TEliOFmZ/`.

[#4](https://github.com/dragginzgame/ic-blob-storage/issues/4),
[#5](https://github.com/dragginzgame/ic-blob-storage/issues/5),
[#6](https://github.com/dragginzgame/ic-blob-storage/issues/6),
[#31](https://github.com/dragginzgame/ic-blob-storage/issues/31) and
[#34](https://github.com/dragginzgame/ic-blob-storage/issues/34) retain real
application/provider acceptance under [the coordinated run](../acceptance-plan.md#next-application-acceptance-run).
Registration proves no stored bytes/availability; GC acknowledgment proves no
remote deletion/billing cessation; funding acknowledgment is not independent
account-credit evidence. No target, credentials or paid budget was selected.

## Recovery invariants

Owned schemas remain 1. Cross-release transitions require
[retirement/reinstall](../retiring-installations.md), retaining provider objects,
balances, uncertain effects and continuing billing. Same-release exact retry,
backup and interruption recovery remain required; never activate stale backups.
`MemoryBacking` aliases must delegate one retained physical identity. Release all
collection/detached leases before replacing runtimes. Full manager checks occur
at construction/bootstrap/lease opening/growth; ordinary IO trusts exclusive
custody and bounds/poisoning. Raw alias edits need not be detected on the next IO.
