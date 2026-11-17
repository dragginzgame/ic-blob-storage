# Current status — 2026-11-17

## Delivery and active work

Private `dragginzgame/ic-blob-storage`, branch `main`; **0.103.2 is pushed and
published** for all four product packages at
[`ff1aaaf`](https://github.com/dragginzgame/ic-blob-storage/commit/ff1aaafc75d80cd51c12f232dd758d4883a7aa21),
with annotated tag `v0.103.2`. Qualification is independent and unpublished.
[Release procedure](../operations/releases.md); [application guide](../design/reference-application.md).

**Platform adoption:** Types 0.21.2, Runtime/Macros/Guard 0.23.16 and Platform
0.34.13 are selected in both lockfiles. Protocol 0.3.5 supplies management and
Caffeine codecs; **no legacy Platform remains in product or qualification**.
The native API owns signing, one-attempt dispatch, original-request recovery,
signed queries and exact proof authentication. Custom roots require an explicit
root-subnet principal. Wire/stable formats and original custody are preserved.
Storage 0.74.1 retains Runtime 0.22.5's IC0-only edge, not another executor/guard.

**Testkit 0.108.0:** owns managed server preparation, instance deletion/process
teardown and bounded held replies. It shares Protocol 0.3.5 with Blob;
Protocol 0.2.8, legacy Platform 0.33.7 and Types 0.20.4 are removed. Qualification
uses bounded Protocol metrics/top-up decoders and independent literal refusal
fixtures, retaining exact provider-reply bytes and declared-outcome semantics.
CI, installed capacity/enrollment and all seven non-Chromium callback cases pass
(`target/platform-full-adoption/`), including original-request expiry/recovery.
Both graph snapshots contain one Platform, Protocol and Types identity.
[Migration contract](../design/application-boundary.md#platform-replacement-prerequisites--2026-11-16).

**Tooling 0.44.11:** both pins/locks, authenticated 84-file snapshot and CLI binding
are updated. CI retains bounded sanitized failure output, preserves original status
and reports upload failures. Hosted upload/macOS remains for CI.
Rust 1.100.0 and Wire 0.24.3 remain selected; Host FS/Artifacts use 0.80.3.
Qualification selects Host Process 0.80.4, retaining its macOS libproc 0.15 edge.

## Next action and provider boundary

`blob app` prepares, inspects, dispatches and observes exact
fresh-install requests; saved outcomes never authorize renewal or resubmission.

**Pending 0.103.3:** remove all eight qualification `#[path]` overrides. The existing
unpublished qualification library now owns the unchanged simulator helper; callers
use ordinary imports. Only its existing Testkit/Host Process dependencies change
declaration scope; release preparation also retains zeroize 1.9.2 in both locks.
This is compatible maintenance with no product API or stored-format change. Clippy and linking
pass for all eight targets; installed standalone setup/cleanup and upgrade pass
(`target/path-override-fix/`). Changes remain local; no release or live effects.

Next: Storage's retained-image transition, Canic-owned fleet integration and
outstanding provider qualification below. Current local browser checks pass.
[Platform #24](https://github.com/dragginzgame/ic-platform/issues/24) now has consumer
adoption evidence. Storage alone retains the old IC0-only Runtime edge; no second
executor, legacy Platform facade, direct upstream codec or local generic codec exists.
Release freshness rechecked for 0.103.3: runtime suppliers are current; the optional
IC Build editor 0.73.0 → 0.87.3 upgrade stays deferred to release-tool maintenance.
Canic owns fleet integration in [Canic #12](https://github.com/dragginzgame/canic/issues/12)
and [Canic #13](https://github.com/dragginzgame/canic/issues/13); the existing Service API
is sufficient, with no duplicate Blob adapter. The retained live Storage image
still needs [Storage #1](https://github.com/dragginzgame/ic-storage/issues/1).
The successor repository was renamed from `ic-storage-new` to `ic-storage`.
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

Release 0.103.2 passed the complete gate and archive/provenance checks.
All four downloaded registry archives match the prepared packages; readback:
`target/01032-release/registry-verified.json`. Source provenance fetched the exact
GitHub commit independently (`target/publication/provenance-01032/`).
After release, both opt-in Chromium targets passed on `6a369f8` with unchanged code
and locks: three transport cases and two installed catalog/upgrade cases.
Evidence: `target/browser-refresh-01032/qualification.log`; Chrome for Testing
156.0.8078.4, Node 24.21.0, npm Platform 0.2.0 and esbuild 0.28.2.
Transport/replay, verified reads and upgraded-catalog cancellation pass; no fix was needed.
PATH `blob --version` reports 0.103.2. Hosted CI was not awaited. Browser profile
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

Delivery review (`3993d90..ff1aaaf`) covered supplier ownership, bounded codecs,
malformed-input fixtures, both dependency graphs, unchanged CLI module moves and
release metadata. No actionable architecture sediment found in that scope.
Platform owns the codecs; fixed literal test schemas add no generic codec.
Library/host/client ownership and live custody remain unchanged. Storage's IC0-only
Runtime edge is a deliberate retained dependency. No tag or upload was retried.
