# Current status — 2026-11-16

## Delivery and active work

The maintainer selected `main` for ongoing development and releases. The former
`0.100` branch is renamed with its history intact; releases use version tags.

Private `dragginzgame/ic-blob-storage`, branch `main`; **0.101.0 is pushed and
published to crates.io** for all four product packages at
[`7a9f723`](https://github.com/dragginzgame/ic-blob-storage/commit/7a9f723421b4b0a2f2a6fc64cbcdf2b8643bd4d4),
with verified annotated tag `v0.101.0`. Both locks select the product version;
qualification package versions remain independent and unpublished.
[Hosted CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/43894805408)
was still running at the delivery readback. All four registry archives match their
local SHA-256 and exact clean source provenance. The CLI is installed on PATH.

The release renames the CLI group to `blob app`, matching Canic. Offline
`blob app prepare-installation` encodes paired init arguments with explicit
roles/limits; the installed application journey consumes those exact files.
[The guide](../design/reference-application.md#prepare-fresh-installation-inputs)
owns usage and proposed-target limits. Publication adds no provider effects or
API/persistence stability guarantee. [The release procedure](../operations/releases.md)
owns the shared Git/registry commands and packaged tooling fingerprint binding.

Rust 1.100.0, Tooling 0.44.5 (84-file snapshot), Platform 0.33.7, Runtime 0.22.5,
Types 0.20.4, Wire 0.24.1, direct Host FS/Artifacts 0.80.2 and Testkit 0.106.0 /
PocketIC 16.1.0 remain selected. Testkit owns its older internal Host edges.
The registry review in `target/01010-registry-review.json` finds the selected
first-party versions current; no optional supplier upgrade is included.

## Next action and provider boundary

**Next:** qualify target/controller admission and signed install custody for fresh
application provisioning. Offline arguments and local upgrades do not establish
those guarantees. Canic's separate fleet work is owned by
[#12 — Provision embedded Blob applications through approved fleet installation arguments](https://github.com/dragginzgame/canic/issues/12)
and [#13 — Integrate Blob drain, upgrade and explicit activation into fleet maintenance](https://github.com/dragginzgame/canic/issues/13).
The [current handoff](../design/application-boundary.md#fleet-integration-handoff--2026-11-16)
confirms the existing Blob Service API is sufficient; no duplicate adapter here.
The live owner's one-slot Storage 0.74.1 image also needs the
Storage-owned retained-image transition in [ic-storage-new #1](https://github.com/dragginzgame/ic-storage-new/issues/1).
Latest reviewed committed Storage `92af0d1` adds native record-vector qualification,
not that transition. No sibling edits/execution. Keep the consumed live probe unchanged.

Caffeine calls and the existing `canic-mainnet` signer are authorized once the exact
operation and effect custody are ready. No provider outreach, key export, legacy
custody import, implicit retry/reset, new owner, funding or link is authorized by
local progress. [The contract review](../design/caffeine-contract-review.md#independent-qualification--2026-11-15)
and [compact observation](../design/caffeine-observation.json) own provider evidence.
D0 remains open for enrollment, scope, enforced bounds, proof recovery/lifetime and
charge closure. D1/D3 are partial; D2 has no positive gateway-admission/upload witness.
Keep declared bytes, unresolved exposure and unfrozen API/persistence.

## Live custody — preserve, never repeat funding or creation

- Mint completed: **50,000,000,009,024 cycles**, **30.60553552 ICP** including fee;
  ICP block **40446894**, mint block **17206042**. Original requests/results:
  `.operations/mint-50t-20261115/`. Do not repeat.
- Payer funding completed: **10.1B debit**, **9.9B Cashier credit**; transfer
  **17206169**, notification **17206177**. `.operations/caffeine-enrollment-01007/`
  owns custody. Zero overdraft, null target and no relationships were reported;
  **correction:** null uses the default target; it does not disable refill.
  deposit address was emptied. Payer credit is not gateway/owner capacity.
- Existing owner: **`mjjg7-wqaaa-aaaai-ayghq-cai`**, creation block **17206696**;
  sole controller `canic-mainnet`, principal
  `o5trf-oqyg7-cawjp-xs4pw-aomb3-iwki5-hyezf-qahfz-j3ffd-jh4fc-oqe`.
  Acknowledged create request selected 13-node application subnet beginning `brlsh`.
  Debit **1.0001T**, source balance afterward **49,404,879,124,210 cycles**; the
  original **1.0102T source budget is fully consumed**. Creation custody:
  `.operations/caffeine-owner-01008/`. Reuse this owner; never recreate or top up
  merely because provider admission remains unresolved.
- Probe installation completed: one `install`, module SHA-256
  `4fc165878bd4d4ab6334d93ec56114df64309ca79cb9088e2c44a7bafee4e0fe`, 329,459 bytes.
  Status reported **498,369,997,713 cycles**, unchanged sole controller. Existing
  owner funds paid execution; no additional source debit. Replicated inspection
  confirmed expected owner/operator, Cashier `72ch2-fiaaa-aaaar-qbsvq-cai`, target
  **1B**, **no attempt and no outcome**. `.operations/caffeine-install-01008/`
  retains exact module/source hashes, original/routed signed install, read request,
  dispatch claims and results. Routing metadata was corrected before submission
  per icp 1.6.0's format; signed content stayed unchanged. No reinstall fallback.

- Temporary relationship experiment completed in `.operations/caffeine-link-01008/`:
  one 1B daily-allowance link allocated **1B immediately**, before the probe call;
  payer audit **1416836** identifies `RelationshipAdd`. The single zero-attachment
  owner call returned **`Err(TopUpWithoutCycles)`**, retained across a separate
  replicated inspection. One explicit removal returned `Ok`; relationship absence
  was read back. **8.9B payer / 1B owner** remain, plus Cashier-reported gateway
  budget **166,666,666** with zero usage. Removal did not refund or close credit.
  Preserve all 59 retained files/hashes, including the static artifact and exact
  signed requests. No additional funding, certificate or object was introduced.

- Negative gateway read: `.operations/caffeine-gateway-01008/` retains one bounded
  GET for a never-issued root, **404 / 92 bytes**, no redirect; callback-version
  query returned null. No certificate, write or retry. This does not qualify
  upload admission or project scope; `X-Dry-Run` remains unqualified and unused.

Two bodyless public PUT preflights returned HTTP 200 with the required CORS headers;
`.operations/caffeine-preflight-010010/` retains raw replies/hashes, no objects or proof.
All seven custody directories are private/Git-ignored, not disposable build output.
The probe is now consumed; its installation bullet describes the initial state.
A requested target balance is not an enforced liability cap. Mainnet/root-key
selection is explicit; the existing named signer was used without exporting keys.

## Qualification and integration

Release 0.101.0 passes `make ci` and all eight installed suites:
`target/01010-release-corrected.log`. Metadata projection preserves private versions
and external selections; publication refuses private packages and dirty source.
The first release attempt stopped at early Clippy; its formatter fix is included.
Archive builds and clean-source retrieval pass (`target/01010-publish-check.log`,
`target/publication/provenance-01010/`); registry readback is
`target/01010-registry-verified.json`. No upload retries were needed.
The opt-in Chromium journeys last passed at 0.100.12; they were not rerun for
this native CLI/package change. PATH `blob --version` reports 0.101.0.
Node 24+, npm Platform 0.2.0 and esbuild 0.28.2 remain selected. Browser profile
rollback/eviction, multi-device custody, live acceptance and settlement are unqualified.

Service has one 64 KiB record and 64 historical uploads/references; cleanup retains
exposure. [Maintenance](../operations/standalone-maintenance.md) preserves original
identities and closed admission; unknown execution is not safe retry. There is no
unrestricted-controller snapshot/rollback continuity guarantee. The consumed probe
stays 0.100.8; original source remains in installation custody's
`source-before-release-01008/`.
Current committed Canic `0167a30` pins Blob `decde204` (0.100.6) immutably; its older
composition evidence is not current-supplier qualification. The two Canic issues
own real fleet initialization and maintenance. No Canic source changed or ran.

## Latest delivery review

Reviewed `bbb37fc..7a9f723`: offline init preparation reuses existing Wire types
and held-directory publication; the runtime delta is small and introduces no service
state or recovery ledger. Most added lines are Toko-owned release helpers, kept in
the authenticated 84-file snapshot. Blob’s thin metadata adapter owns only its
four product identities/two locks; the existing installed suite checks fingerprint
admission and refusal. The post-push review corrected stale README release wording
and the observed Cargo batch archive path in the runbook. No remaining actionable
sediment in this scope. Live/legacy custody is untouched; the historical sibling
checkout was unavailable during release-tool adoption.
