# Current status — 2026-10-29

## Current batch and source identity

HEAD is the maintainer's checkpoint `18602ee3` (message `0.58.0`), not a new
release tag. Last released source is **0.57.0**, `3865668c`; workspace package
versions remain 0.57.0. Pending **0.58.0** adopts published **Memory/Storage 0.56.0**,
superseding the unpublished 0.57.1 minimum-only notes. Official crates.io index
entries and both publication archive checksums were verified. Published APIs,
error contracts and the producer's Wire-ledger migration guidance were inspected.
No source override, sibling edit, staging, commit, push, release or paid effect.

Memory's allocation ledger and enclosing Cell value now use IC Wire with `ICML`
and `ICMWIRE1` framing. Earlier CBOR values are retired. Blob supports
retirement/reinstall across releases, with objects, balances, uncertain effects
and continuing liabilities retained before reset. Never clear only the ledger
while leaving collection bytes behind. No reset, legacy reader, converter or
migration was introduced. Blob-owned schemas remain 1 and endpoint/record bytes
are unchanged; manager and collection headers do not imply old-ledger compatibility.
Embedding hosts must align the public Memory/Storage 0.56 identities and rebuild.

Current graph: **Memory/Storage 0.56.0**, **Platform 0.8.2**, **Metrics 0.17.4**,
**Tooling/Host/Build/Testkit 0.70.2**, **Wire/derive 0.7.23**. Incoming selections
outside Memory/Storage were preserved. There is one Platform identity/executor;
normal/build standalone Wasm now excludes `serde_cbor` and `half`. Native IC
transport/certificate tests still need their external CBOR. No size or performance
gain is claimed. Browser Platform 0.1.1, SDK Core 6.1.1/Caffeine 1.1.2 and Shared
Tooling 0.27.3 (`cc81a58a5b2dd287c577cbffd2c576c178b6785c`, 131 files) are unchanged.

## Latest focused qualification

Evidence: `/tmp/blob-memory056/`. Core/standalone all-target/all-feature strict
Clippy and selected standalone-test Clippy pass. Two public memory-composition
cases pass on Rust **1.91.0**; five upload restoration/integrity unit cases pass.
Fresh release standalone Wasm and Rust 1.91 standalone Wasm check pass. Nine
installed cases pass: fresh installation, stop/start/repeated restore, independent
history refusals, concurrent denied updates, preserved physical/billing obligations,
tenant/global capacity, snapshot fencing and heap metrics.

The explicit pinned 0.14.9/Memory 0.25.5 and single-uploader 0.18.4 cases also
pass, with exact before/after bytes, old owner readback and obligations preserved.
The first now submits the current lifecycle frame and asserts actual memory
admission refusal (`host memory bootstrap: StableCellLedger(Record(Malformed))`).
It no longer passes on malformed lifecycle input. Historical admissions are
validated in their frozen schema and compared as exact reply bytes rather than
converting retired `Confirmed` to current `Registered`. A first run caught that
obsolete projection; its failed log/report is retained beside the passing fresh
report. These older fixtures are not a claim covering every retired format.
No production compatibility reader was added.

Direct/deployment boundaries, formatting, documentation links and diff checks
pass. Lock-selected Testkit 0.70.2 setup/admission completed. Manifest/lock hashes
stayed fixed. No full gate, native macOS, browser, live IC or provider qualification
was rerun. Earlier evidence remains `/tmp/blob-tooling-wire-next/`,
`/tmp/blob-platform-latest/` and `/tmp/blob-persistence-next/`, on their captured
graphs. #4/#5/#6/#31/#34 retain separate application/provider acceptance; this
adoption does not close them or qualify Canic's changed composition.

Release-CI follow-up: the retained 20261029T185636 test log reports one journey
failure out of 49: `operator_query_denies_controller_and_reports_missing_economics_without_effects`
received `CanisterInstallCodeRateLimited` at its fixture upgrade, not a ledger
or state assertion. `upgrade_authority` now uses Testkit's existing
`retry_install_code` with two total attempts and a 300-second simulated cooldown,
only for that exact error. Wasm/init bytes and sender are retained unchanged;
traps and other rejections still fail immediately. No production retry or recovery
policy was changed and no real-time sleep or replica-limit bypass was added.

Evidence: `/tmp/blob-install-cooldown/`. Focused journey strict Clippy passes;
the exact reported case and all **49** journey cases together pass. Retained
fixture Wasms and manifest/lock hashes stayed fixed. The original full failure
log remains under `.git/release-state/validation-failures/`. Formatting, links and
diff checks pass. The full release gate was not rerun by the agent.

Persistence's Metrics Stable codec cut is separate; Blob has no stable metrics
store. Hosts selecting that optional adapter follow its explicit metrics-map
disposition without resetting Blob state or allocation history.

## Released simplification

0.55.0 removed manufactured certificate host-evidence objects, policy/result
wrappers, substitution scenarios and a private preview endpoint. Installed-owner
state owns current eligibility; synchronous commit preserves authority, expiry,
revocation, phase, rollback and restoration fences. Passive service Wire shapes,
provider replies and stored formats remained unchanged. CLI names, root-planning
labels and HTTP defaults use existing owners. Earlier qualification remains
`/tmp/blob-simplify.7hFZcO/` (58 upload, 19 native CLI, 12 planning and 14 installed
cases) and `/tmp/blob-host-audit.Wi2ibe/` (Host adoption).

Conditional candidates remain intentionally: cross-language checks guard separate
trust boundaries, historical DTOs serve six frozen-image/malformed-input suites,
heap models provide independent differential oracles, and concrete errors retain
useful public distinctions. Transport/decoding already share mechanics. No new
schema generator or generic error wrapper was added to hide apparent duplication.

## Retained Blob boundary and evidence

Memory is publicly re-exported, so hosts must align their direct Memory/Storage
identity with the selected Blob release. Allocation custody and Blob-owned
schemas are unchanged; Memory's retained format now has the Wire hard cut above. Stable metrics remain a
separate Persistence capability; Blob itself adds no stable metrics store. Previous qualification remains
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
clear fences or replace financial records. [#109 is closed](https://github.com/dragginzgame/ic-blob-storage/issues/109#issuecomment-6495725447)
after verifying default-main delivery and successful [0.54.0 hosted qualification](https://github.com/dragginzgame/ic-blob-storage/actions/runs/40847579158)
at `12a397e361ee92b5281194cdb552367cb1809e50` (all six jobs, including both macOS
architectures and MSRV). The delivered metrics implementation/tests are unchanged
through current 0.55.0 main. The subsequent [0.55.0 hosted run](https://github.com/dragginzgame/ic-blob-storage/actions/runs/40861696292)
also passed; the newer 0.55.1 run remains separate.

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
Memory/Storage with Blob's selected public identity and preserve #599 quality
work. Optional Metrics Stable 0.55 has a separate persisted-format cut: its owner
must explicitly disposition the old metrics map or supply a fresh one, rather
than silently clearing it during Blob adoption. Never replace the newer owner's
locks with the older isolated contribution's locks.

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

The 2026-10-29 acceptance follow-up corrected stale current guidance in
`service-gaps.md` and `funding-consumer-qualification.md`: Canic 0.112 delivered
the adapter; current 0.113 working-tree selections are not released/current-graph
installed qualification. IC Crates still has verifier configuration, attestation
publication and retired worker commands at local base `3c67db3`; the concrete
migration and checksum/availability/retry acceptance were recorded in its existing
[IC Crates #1 tracker](https://github.com/dragginzgame/ic-crates/issues/1#issuecomment-6495954765). No sibling edits or builds. The trusted-registration guide now assigns
remaining deployed deletion/billing evidence to #6, rather than completed #32.
The five acceptance issues remain open. The public application URL, current Blob
service principal and selected deployment configuration were requested; none was
available in the reviewed current handoffs. Historical deployed principals are
not authority for a fresh run. No deployment, paid effect, live-provider query or
new substitute run occurred. Documentation links and diff checks pass; existing
code qualification is reused because this follow-up changes guidance only.

## Recovery invariants

Owned schemas remain 1. Cross-release transitions require
[retirement/reinstall](../retiring-installations.md), retaining provider objects,
balances, uncertain effects and continuing billing. Same-release exact retry,
backup and interruption recovery remain required; never activate stale backups.
`MemoryBacking` aliases must delegate one retained physical identity. Release all
collection/detached leases before replacing runtimes. Full manager checks occur
at construction/bootstrap/lease opening/growth; ordinary IO trusts exclusive
custody and bounds/poisoning. Raw alias edits need not be detected on the next IO.
