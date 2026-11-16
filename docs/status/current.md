# Current status — 2026-11-16

## Delivery and active work

Private `dragginzgame/ic-blob-storage`, branch `main`; **0.102.0 is pushed and
published** for all four product packages at
[`548d9c3`](https://github.com/dragginzgame/ic-blob-storage/commit/548d9c3de7a3a224e66693685634d79cad191406),
with annotated tag `v0.102.0`. Qualification is independent and unpublished.
[Release procedure](../operations/releases.md); [application guide](../design/reference-application.md).
Release 0.103.0 is selected for the custom-root CLI compatibility change;
GitHub/crates.io delivery is authorized and in preparation. No live canister change.

**Platform adoption:** Types 0.21.2, Runtime/Macros/Guard 0.23.14 and Platform
0.34.8 are selected in both workspaces. Product Protocol 0.2.7 supplies management
install/status and Caffeine codecs; **no legacy Platform remains in the product**.
The native API owns signing, one-attempt dispatch, original-request recovery,
signed queries and exact proof authentication. Custom roots require an explicit
root-subnet principal. Wire/stable formats and original custody are preserved.
Storage 0.74.1 retains Runtime 0.22.5's IC0-only edge, not another executor/guard.

**Testkit 0.107.3:** managed server preparation, owned instance deletion/process
teardown and bounded held replies replace the old helpers. Its exact Protocol
0.2.6 requirement keeps qualification on that version. Legacy Platform 0.33.7
remains only for independent malformed-input fixtures, enrollment decoding, metrics
and opaque top-up outcome inspection. Callback/certificate and management-target
codecs now use Protocol; legacy management types and the local reply DTO are removed.
Evidence: `target/platform-testkit-adoption/`; aggregate CI and all installed
migration cases pass, including old CLI custody and populated predecessor upgrades.
[Migration contract](../design/application-boundary.md#platform-replacement-prerequisites--2026-11-16).

**Tooling 0.44.11:** both pins/locks, authenticated 84-file snapshot and CLI binding
are updated. CI retains bounded sanitized failure output, preserves original status
and reports upload failures. Prior tooling checks passed under the earlier Platform
cohort (`target/tooling-04411-adoption/`); hosted upload/macOS remains for CI.
Rust 1.100.0, Wire 0.24.3 and Host FS/Artifacts 0.80.2 remain selected.

## Next action and provider boundary

Next: finish 0.103.0 GitHub/crates.io delivery, then offline signed installation
custody/readback using the adopted Platform APIs;
review private codec replacements when their supplier surfaces exist. Supplier
tracking: [Platform #17](https://github.com/dragginzgame/ic-platform/issues/17),
[Platform #18](https://github.com/dragginzgame/ic-platform/issues/18),
[Platform #11](https://github.com/dragginzgame/ic-platform/issues/11).
Canic owns fleet integration in [#12](https://github.com/dragginzgame/canic/issues/12)
and [#13](https://github.com/dragginzgame/canic/issues/13); the existing Service API
is sufficient, with no duplicate Blob adapter. The retained live Storage image
still needs [ic-storage-new #1](https://github.com/dragginzgame/ic-storage-new/issues/1).
Keep the consumed owner/probe unchanged; no sibling source was changed or executed.

Caffeine calls and the existing `canic-mainnet` signer are authorized once the exact
operation and effect custody are ready. No outreach, key export, custody import,
implicit retry/reset, new owner, funding or link is implied by local progress.
[Contract review](../design/caffeine-contract-review.md#independent-qualification--2026-11-15)
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

Local adoption passed `make ci` (`ci-final.log`) and installed standalone, capacity,
endpoint, signed caller, application, enrollment, callback and management cases.
`custody-final.log` covers released-CLI preparation and expired original recovery;
`callbacks-response-final.log` covers final dispatch/transfer handling. Management
passed three cases in `management-final.log`; `target-final.log` passes the corrected
controller-loss diagnostic case.
Optional Chromium was not rerun. PATH `blob` is rebuilt locally at version 0.102.0.

Release 0.102.0 passed `make ci`, all eight installed suites and archive/provenance
checks (`target/01020-release/`). All four registry archives match the prepared
packages; readback: `target/01020-release/registry-verified.json`.
The opt-in Chromium journeys last passed at 0.100.12; they were not rerun for
this release. PATH `blob --version` reports 0.102.0. Hosted CI is not a delivery wait.
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

Delivery review (`6efe329..548d9c3`) covered host ownership, shared status transport,
ingress custody, tests and dependencies. Stale pending-release prose is corrected
in this handoff update; no additional code sediment found in the delivered scope.
The current migration removes obsolete native adapters and product legacy dependencies.
Private codec uses remain explicit. Live/legacy custody is untouched.
