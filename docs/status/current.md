# Current status — 2026-10-28

## Released baseline and pending batch

Local HEAD and verified default main are **Blob 0.53.0**, release
`47df5334d0c625dbf6081fd8e172f96c72764a7d`, validated source
`4ec79c64fbf5cc74ba0eeb862d943037b3046830`. The journal-custody repair is delivered.
The undated **0.53.1** notes collect published Persistence 0.53.1 adoption and the
private timeout/native CI work below; Blob package versions are unchanged. No
staging, commit, push, release, publication or deployment was performed in this
batch. Sibling files remain untouched.

Selected published **Memory/Storage 0.53.1**, preserving incoming lock edits for
**Tooling/Host/Build/Testkit 0.69.8** and **Wire 0.7.20**. Platform **0.7.3**,
Metrics **0.16.3**, browser Platform **0.1.1**, SDK Core 6.1.1/Caffeine 1.1.2 are
unchanged. Only Memory/Storage changed from the captured incoming lock; the root
catalog now requires 0.53.1. No broader resolver update was made.
Shared **0.27.3**, `cc81a58a5b2dd287c577cbffd2c576c178b6785c`, remains unchanged
(131 files). Published sources and original adoption evidence remain in
`/tmp/blob-memory-refresh.HLjFtLrY/`; no unpublished source dependency was used.

## Current issue work and local evidence

[#108 — Platform-owned execution](https://github.com/dragginzgame/ic-blob-storage/issues/108)
now has an actual installed bounded-timeout proof. The private storage probe holds
its reply after the real tenant mutation commits; advancing beyond the client's
30-second wait returns original SysUnknown code 6 through the consumer probe.
Its saved command blocks redispatch before and after delivery of the late reply;
inspection sees generation 1 and service bytes remain unchanged by recovery.
The control is heap-only, operator-authorized and bounded. Production transports,
endpoints, persistence, funding accounting and retry policy are unchanged.

The existing native host target builds both probe participants and selects all
tenant-client cases on Linux/macOS ARM/Intel. Its consumer command fixture checks
actual selection and failure propagation. No extra executor or ic-call dependency
was added: Platform's supported raw API covers these static/borrowed method names.
Private controls remain Wire; raw_rand scheduling uses the external management codec.

Original timeout/wiring evidence remains `/tmp/blob108-timeout.KeNYrDMh/`, scoped
to its original graph. New published adoption evidence is
`/tmp/blob-persistence-adoption.TlBKfyOu/`: all **339 core tests**, two public
composition cases on actual Rust 1.91 and **15 installed cases** pass. Installed
coverage comprises eight tenant cases (including timeout/no-resend), growth
rollback, restoration reads and five standalone lifecycle/recovery cases.
Strict affected-package Clippy, metadata/deployment/direct-boundary and pin checks
pass. Rust 1.91 builds both probe Wasms; the default standalone builds separately
on Rust 1.99.1 to exclude unified probe features. Testkit 0.69.8 was explicitly
installed and admitted; PocketIC stays 16.1.0. Input hashes stayed fixed.
Manifest formatting, documentation links and diff checks pass. The first installed
attempt omitted Make's fixture-path environment and failed before exercising the
cases; its log is retained alongside the corrected run. No full gate, native
macOS execution or deployed-provider qualification was run locally.
Do not close #108 before default-branch delivery and required hosted acceptance.

## Current hosted failure

[Released 0.53.0 CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/40701957045)
passes Linux, MSRV, core/client and generated-client jobs, but both macOS native
jobs fail coordinated archive tests. Fresh independent FixtureMemory construction
returns Construction(BackingAlreadyClaimed): ARM core 338/339, Intel 336/339.
Exact retained artifacts are IDs 12824136857 (ARM) and 12823744421 (Intel).
Downloaded ZIPs and extracted failed output remain `/tmp/blob053-macos-packages.zip`,
`/tmp/blob053-intel-packages.zip` and `/tmp/blob053-*-test-targets.log`.

The [canonical Persistence #137 comment](https://github.com/dragginzgame/ic-persistence/issues/137#issuecomment-6472674001)
records the original hypothesis and negative Linux stress control. The supplier
subsequently proved the stale-claim lifetime defect deterministically and published
the fix in 0.53.1 (`1583819343fda5c31bbe62bf058e1b2927189262`). Claims now retire
before backing destruction on normal teardown, refusal and unwind. Public APIs,
valid IO and stored formats are unchanged; Storage source is byte-identical.
Published archives are verified against the official sparse-index checksums in
`/tmp/blob-persistence-adoption.TlBKfyOu/`. No path overrides or local custody
workaround was added. Both native macOS archive lanes still need qualification;
[supplier 0.53.1 CI](https://github.com/dragginzgame/ic-persistence/actions/runs/40719533641)
was in progress at inspection. Do not infer hosted acceptance from publication.

## Other acceptance and consumer scope

[#4](https://github.com/dragginzgame/ic-blob-storage/issues/4),
[#5](https://github.com/dragginzgame/ic-blob-storage/issues/5),
[#6](https://github.com/dragginzgame/ic-blob-storage/issues/6),
[#31](https://github.com/dragginzgame/ic-blob-storage/issues/31) and
[#34](https://github.com/dragginzgame/ic-blob-storage/issues/34) retain actual
application/provider acceptance. Their procedures and Blob primitives exist;
more substitutes do not establish live outcomes. Updated #5's obsolete description
and #6/#31's references to completed #32: remaining deployed deletion/billing
belongs to #6. Registration proves neither stored bytes nor availability; gateway
acknowledgment proves neither remote deletion nor billing cessation. Successful
funding acknowledgment is distinct from independently attributable account credit.
Use the [coordinated acceptance run](../acceptance-plan.md#next-application-acceptance-run).
No live target, credentials, approved provider budget or paid effects were selected.

Canic read-only source is `261e533110e658eb7b97de213d8e95d25fbab5de`, preparing
0.113.0 with Blob/Contracts 0.53.0, Memory/Storage 0.53.0 and Tooling 0.69.7.
Its adapter is now crates/canic-blob-service with integrations/blob-service fixtures.
The latest dedicated attempt refused ChangedInputs during relocation; embedded
and exact-root snapshot checks did not run. Earlier results predate the move.
Local origin/main still points at 0.112; this source is not verified delivered or
published. Framework fixture results are separate from actual application/provider
acceptance; do not duplicate downstream fixtures in Blob.

## Retained recovery and adoption facts

MemoryBacking wrappers must delegate one retained physical identity across aliases.
Release every collection/detached lease before replacing a runtime. Full manager
checks occur at construction/bootstrap/new leases/growth; ordinary IO trusts custody
and checks bounds/poisoning. Raw alias edits need not be detected on the next IO.
Owned schemas remain 1; stored/Wire bytes are unchanged. Cross-release transitions
require [retirement/reinstall](../retiring-installations.md), retaining provider
objects, balances, unknown effects and continuing liabilities. Same-release exact
retry and interruption recovery remain required; never activate stale backups.

#105/#106 are closed from complete released 0.52.2 CI; #107 from verified published
0.52.1 license archives. Their original evidence remains scoped to those inputs.
The released journal repair drops old owners before reopening retained evidence;
all 49 journey and six funding-recovery cases pass in
`/tmp/blob-journal-custody.TEliOFmZ/`. Original failures remain under Git's retained
validation logs. Do not replace archive rereads with cached state or reset records.
