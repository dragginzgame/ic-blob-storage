# Current status — 2026-10-30

## Active batch and ownership

Working HEAD is `fd20a61a` (maintainer's `0.60.0` commit); last verified released
source is **0.59.1**, `70eeb697`. Pending **0.60.0** includes public Platform,
Memory, Metrics and Wire schema identity cuts. Workspace versions remain 0.59.1.
No agent staging, commit, push, release, publication, deployment, paid effect or
sibling edit occurred. Preserve the pending nested-validation fixture fix.

Official crates.io index entries and checksum-verified published archives select:
**Build/Host/Tools/Testkit 0.72.0**, **Memory/Storage 0.58.0**, **Metrics 0.21.0**,
**Platform 0.11.1**, **Wire/derive 0.10.0**. Root minimums and lockfile match.
Unrelated dependency selections remain intact. No source override, compatibility
alias, local schema bridge or metric mirror is used. Supplier working checkouts
were read-only evidence; unreleased source versions were not selected.

Principal/Account schemas now belong to Wire's `platform-types` integration.
Contracts retains its own native `wire-schema` feature, selecting that integration
instead of Platform's removed schema feature. Existing DTO derives and generated
contracts need no local implementation changes. The deployment gate explicitly
rejects `platform-types`, with regression cases for isolated and combined flags.

The resolved graph has **one Platform and one Wire version**, including native
Testkit, Persistence and public metadata. Production selects one Platform executor
and its exact-version internal guard. No external IC0/executor, upstream native
management DTO package, implicit Candid printer, host tool or schema feature enters
the standalone deployment selection. Required external-provider Candid remains
behind Platform. No raw IC0 package remains in the lockfile.

Frozen endpoints, frames, bounds, record layouts, retry bytes and schema numbers
remain unchanged. Memory/Storage implementation source matches 0.57; Metrics
implementation matches 0.19 except documentation. Schema ownership/dependency
identity changed, not arithmetic or retained representations. Blob selects no
Logging checkpoint adapter, so that separate supplier cut is inapplicable.
Consumers align exposed Rust identities and rebuild; all embedding runtime users,
including Timers, must use Platform 0.11. Never bridge handles between executors.

## Qualification and tooling

Current evidence: `/tmp/blob-latest-refresh/`, including registry records,
verified archives, exact commands/logs, input hashes and fresh artifact hashes.
Focused strict Clippy passes for core, Contracts, client, CLI and standalone
(all targets/features), plus standalone/journey/funding integration targets.
Tests pass: core **338**, Contracts **125** plus the frozen Wire suite, client
**6**, CLI **122**, memory composition **2**. Rust **1.91.0** standalone Wasm
and native CLI checks pass. Fresh standalone/authority/gateway/funding release
Wasm and native CLI builds pass. Generated TypeScript passes **27 methods /
137 frozen Rust frames**, including malformed/identity/direction/limit/decimal
checks through the new schema integration. Prior generated artifacts were retained.

Installed qualification passes **52** selected cases against those fresh artifacts:
metrics **2**, lifecycle **5**, malformed ingress **4**, certificates **3**,
gateway synchronization **2**, journey recovery **33**, funding refunds/traps **3**.
A lock-selected server was admitted once before the batch; each test owns its local
instance. These are local IC/provider substitutes, not deployed-provider evidence.
Direct dependency/deployment boundaries, new feature refusals, actual Testkit/
release-editor forwarding, ShellCheck, formatting, documentation and diff checks
pass. Inputs and artifacts remained fixed. No complete release gate, new native
macOS run, real browser upload, deployed-IC run or performance gain is claimed.

Canonical setup explicitly prepared Testkit and Build release editor **0.72.0**.
Reviewed Shared Tooling remains **0.27.11**, revision
`62edfbd43087a718bf0283e9040676a49211976d`, **136** files. Publication policy
stays local; the supplied adapter remains opt-in. Browser Platform **0.1.1** /
SDK Core **6.1.1**, Caffeine **1.1.2** and browser package files are unchanged.

The retained 20261030T151437 gate failure omitted the runner's checkpoint companion
from a local fixture. The pending fix uses the canonical companion-closure copier.
Complete shared-tooling consumer tests passed with loopback access at
`/tmp/blob-shared-tooling-tests-followup-local-access.log`. Failed originals and
the sandbox socket refusal remain retained. Earlier graph-specific evidence and
handoff are preserved under `/tmp/blob-latest-refresh/previous-handoff.md` and the
prior `/tmp/blob-cohort-refresh/` and `/tmp/blob-persistence-refresh/` folders.

Previous supplier schema blockers are resolved locally by the published cohort:
[Platform #105](https://github.com/dragginzgame/ic-platform/issues/105) and
[Observability #103](https://github.com/dragginzgame/ic-observability/issues/103).
An earlier combined adoption-comment attempt was rejected by automatic approval
review because Observability destination approval was unclear. Nothing was posted
by that attempt; its optional approval question remains pending. The old prepared
comment is retained in `/tmp/blob-cohort-refresh/`; local adoption is complete.

## Remaining application acceptance

[#110](https://github.com/dragginzgame/ic-blob-storage/issues/110) and
[#111](https://github.com/dragginzgame/ic-blob-storage/issues/111) were closed from
0.59.1 after six [hosted jobs](https://github.com/dragginzgame/ic-blob-storage/actions/runs/41047479232)
passed. That captured release run does not qualify this pending dependency cut.

Last reconciled application/provider acceptance issues are
[#4](https://github.com/dragginzgame/ic-blob-storage/issues/4),
[#5](https://github.com/dragginzgame/ic-blob-storage/issues/5),
[#6](https://github.com/dragginzgame/ic-blob-storage/issues/6),
[#31](https://github.com/dragginzgame/ic-blob-storage/issues/31) and
[#34](https://github.com/dragginzgame/ic-blob-storage/issues/34), under
[the coordinated run](../acceptance-plan.md#next-application-acceptance-run).
No current origin, deployed service/configuration or paid trial budget is selected.
Canic owns its adapter/composition and adopts these public cuts there. Registration
proves no stored bytes; gateway acknowledgment proves no physical deletion or
billing cessation; funding acknowledgment is not independent account credit.

Cross-release transitions remain [retirement/reinstall](../retiring-installations.md),
retaining provider objects, balances, uncertain effects and continuing billing.
Same-release recovery, exact retries and restoration fences remain required.
Never clear an allocation ledger while leaving collection bytes behind.
