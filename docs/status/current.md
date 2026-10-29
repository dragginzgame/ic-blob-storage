# Current status — 2026-10-29

## Current batch and source identity

HEAD is the maintainer's **Release 0.54.0** commit `12a397e3`; package versions
and local tag are 0.54.0. Registry publication and remote hosted acceptance were
not rechecked during this task. Pending **0.55.0** combines simplification with published
**Host/Build/Testkit 0.69.11**, with matching root minimums and lock-selected
Testkit CLI/server and Build release editor. The incoming lock already selected
the family; its unrelated **Platform 0.7.5** selection was preserved.

Official archives match crates.io index checksums. No unpublished/path overrides
were used. Root/package versions, endpoint identities, stored formats and recovery
contracts are unchanged by the host-tool adoption. The cleanup removes public Rust
certificate policy/error wrappers; use the installed-owner workflows. It changes
no service Wire shapes, provider replies or stored formats. The agent did not stage,
commit, push, release, deploy or perform paid effects. Sibling checkouts stayed
read-only. Evidence is `/tmp/blob-host-audit.Wi2ibe/`.

Current graph: **Memory/Storage 0.54.0**, **Metrics 0.17.0**, **Platform 0.7.5**,
**Tooling/Host/Build/Testkit 0.69.11**, **Wire/derive 0.7.21**. Browser Platform
0.1.1, SDK Core 6.1.1/Caffeine 1.1.2 and Shared Tooling 0.27.3
(`cc81a58a5b2dd287c577cbffd2c576c178b6785c`, 131 files) remain unchanged.

## Latest focused qualification and audit

Host adoption: strict native CLI Clippy (all targets/features), seven filesystem
and retained-publication-source tests, Rust **1.91.0** checks for native CLI bins,
locked command-forwarding/failure fixtures, actual tool admission, and direct/
deployment boundaries pass. Fresh standalone release Wasm and CLI build. Three
installed cases pass: operator metrics and exact replay/lifecycle behavior,
actual signed CLI with scope refusal, and snapshot rollback fencing. Manifests
and lock stayed fixed through validation. Formatting, local links and diff checks
pass. No full gate, native macOS or installed-IC/provider qualification was run.

The accepted cleanup removes manufactured certificate host-evidence objects,
policy/result wrappers, substitution scenarios and a private preview endpoint.
One current-state lookup validates uploader, exact permission, manifest, namespace,
phase, expiry, revocation and fence before synchronous atomic commit. Read-only
assessment grants no future issuance authority. The existing passive Wire blocker
field is retained to avoid an unrelated protocol cut. CLI upload/reference names,
retained-root planning labels and native HTTP defaults now use existing local
owners; current dependency prose delegates release history to the changelog.

Evidence: `/tmp/blob-simplify.7hFZcO/`. Strict focused Clippy passes for the core,
probe, native CLI and affected installed suites, including default standalone.
58 upload unit cases, 19 selected native CLI cases, 12 inventory planning cases,
the frozen Wire vector, seven installed storage cases and seven installed
standalone cases pass. Installed cases cover real caller/permission authority,
write/reply trap rollback, lost acknowledgments, signed CLI, stop/start/restore,
snapshot fencing and metrics. Two initial HTTP cases could not bind a local port
in the sandbox; their loopback-enabled rerun passed. Release standalone/probe
Wasm builds and Rust 1.91 standalone Wasm/native CLI checks pass. Input hashes
stayed fixed. The Chromium case remains ignored; no full gate, macOS, live IC or
provider qualification was rerun. No performance or size gain is claimed.

Conditional audit candidates were traced rather than forcibly removed. Private
Rust/browser handoff checks validate independent trust boundaries; adding a schema
generator would add machinery. Historical DTOs still drive six frozen-image and
malformed-ingress suites. Feature-gated heap models remain independent differential
oracles. Concrete client/reply errors retain useful public distinctions while
transport/decoding mechanics are already shared. Replacing these with generic
wrappers would add migration cost without a demonstrated simplification.

## Retained Blob boundary and evidence

Memory is publicly re-exported, so hosts must align their direct Memory/Storage
identity with Blob 0.54.0. Published implementation sources match 0.53.1; schemas,
stored formats and allocation custody are unchanged. Persistence's published
`ic-metrics-stable` 0.54.0 now uses Metrics 0.17; Blob itself adds no stable
metrics store. Previous qualification remains
`/tmp/blob-persistence-refresh.YSJYAU/`: 339 core tests, two public composition
cases on Rust 1.91, fresh Wasm/MSRV checks and nine installed lifecycle/growth/
metrics cases. Earlier evidence remains `/tmp/blob-tooling0699.ZMrc8f/`,
`/tmp/blob-metrics017.0ia0hl/` and `/tmp/blob109.cvHNQx/`, on their captured graphs.
No artifacts were deleted; the pre-update handoff remains with current evidence.

Released source includes six fixed heap operation rows, an operator-only
standalone report and signed `blob-storage metrics`, under
[#109](https://github.com/dragginzgame/ic-blob-storage/issues/109).
Metrics owns numeric transport/validation; Blob owns attribution, authority,
units and lifecycle. See [the guide](../metrics.md). Only lifecycle init resets
totals. Platform versions advance on ordinary updates/callbacks too; versions
and timestamps do not prove snapshot continuity. Metrics never authorize retries,
clear fences or replace financial records. #109 closure still requires verified
default-branch delivery and remaining acceptance, beyond this local tag.

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
