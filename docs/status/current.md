# Current status — 2026-10-29

## Current batch and source identity

HEAD remains released **Blob 0.53.1**, `a0563c79f5772193003280489f5e11e8905c0fd1`.
Pending **0.53.2** implements standalone heap telemetry for
[#109](https://github.com/dragginzgame/ic-blob-storage/issues/109): six fixed
operation rows, an operator-only Wire query and signed `blob-storage metrics`.
The service core remains framework-independent; Canic owns its adapter and policy.

The user requested published IC Metrics transport adoption. **Metrics 0.17.0**
now owns report counters/aggregates, Serde numeric admission and native Wire
metadata. Local numeric DTOs and their conversion layer are removed; Blob retains
roster, attribution, units, authority, lifecycle and CLI decimal JSON projection.
The report is still unreleased. Existing released method bytes, persisted schemas,
installation and callback/retry contracts are unchanged. No compatibility reader,
optional recording feature, new stable allocation or Canic dependency was added.
See [the metrics guide](../metrics.md).

Current selection: **Memory/Storage 0.53.1**, **Metrics 0.17.0**, **Platform 0.7.4**,
**Tooling/Host/Build/Testkit 0.69.9**, incoming **Wire/derive 0.7.21**.
Browser Platform 0.1.1, SDK Core 6.1.1/Caffeine 1.1.2 and Shared Tooling 0.27.3
(`cc81a58a5b2dd287c577cbffd2c576c178b6785c`, 131 files) remain unchanged.
No staging, commit, push, package version change, release, deployment or paid probe
was performed. Sibling checkouts remain read-only.

## Latest focused qualification

Tooling **0.69.9** is verified published. Incoming Cargo.lock already selected the
family; root minimums now match. Testkit CLI/server and Build release editor were
explicitly prepared and admitted from the lock, without changing shared tool pins.
Published Host/Build implementation sources match 0.69.8; Testkit's cached snapshot
repair adds no required Blob caller changes because this repo uses neither the
baseline pool nor capture-set helpers. Direct snapshot fencing remains intentional.

Tooling adoption evidence is `/tmp/blob-tooling0699.ZMrc8f/`: native CLI (including
package verification) and standalone-test strict Clippy, Rust 1.91 native CLI,
command-forwarding/failure fixtures, actual tool admission and direct/deployment
boundaries pass. Three installed cases pass: snapshot rollback stays fenced,
metrics/replay/lifecycle behavior, and the actual signed CLI. The standalone Wasm
was reused from the unchanged runtime graph. Formatting, local links and diff
checks pass. No full gate, macOS or installed-IC/provider qualification was run.

Prior Metrics-specific qualification follows, scoped to its captured graph.

Published Metrics archive checksum matches the official crates.io index;
API/source and adoption logs remain `/tmp/blob-metrics017.0ia0hl/`.
Strict affected-package and standalone-test Clippy pass. Focused tests cover
upstream numeric rejection, full-width bounded round trips, native Wire metadata,
CLI decimal output/scope, heap windows and private probe arithmetic. Frozen
released Wire vectors pass. Four installed cases pass: metrics authorization,
replays/refusals, malformed ingress without stable writes, ordinary update
continuity and upgrade reset/fencing; actual signed CLI; gateway-query failures;
and repeated GC acknowledgments. Provider substitutes are not live evidence.

Release standalone/gateway/storage-probe Wasm and native CLI builds pass.
Rust **1.91.0** checks standalone Wasm and native CLI. Direct/deployment boundaries
pass: Contracts/CLI stay runtime-free; deployed Wasm excludes host schema,
parser, Testkit and private-fixture features. Manifests and lock stayed fixed.
Formatting, local links and diff checks pass. No full gate, native macOS or
installed-IC/provider qualification was run for this pending graph.
Prior #109 evidence, including failed attempts, remains `/tmp/blob109.cvHNQx/`;
the pre-trim handoff is retained with the Metrics adoption evidence.

Only explicit lifecycle initialization resets heap totals. The IC version also
advances on ordinary updates/callbacks: lifecycle-start and observed versions are
metadata, not a snapshot detector. Snapshot restore can roll back both totals and
window identity; metrics never clear fences or establish lifetime continuity.
#109 stays open until delivered implementation and remaining acceptance are verified.

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
with the owner's final Persistence graph and preserve #599 quality work. Never
replace the newer owner's locks with the older isolated contribution's locks.

Original provider/configuration, strict Clippy, Rust 1.91 library/Wasm and dedicated
heap/embedded stable composition evidence belongs to that clone and its PR;
earlier failures remain `/tmp/canic-metrics-storage-*.log`. Its snapshot report
relabels windows with the current platform version; the
[owner correction](https://github.com/dragginzgame/canic/pull/708#issuecomment-6490479772)
is required before claiming interval comparison semantics. Published Tooling
0.69.9 supplies [#263's snapshot/timer repair](https://github.com/dragginzgame/ic-tooling/issues/263).
Canic must adopt it, stop/drain the complete captured sets in both Root baseline
recipes, restart only after complete success, and requalify its retained sequence.
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
