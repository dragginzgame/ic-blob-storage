# Current status — 2026-11-16

## Delivery and scope

Private `dragginzgame/ic-blob-storage`, branch `0.100`; **0.100.12 is prepared for
the requested GitHub release**. Four product packages and both locks select
0.100.12; qualification versions stay independent and publication stays disabled.
Previous delivered checkpoint: [0.100.11](https://github.com/dragginzgame/ic-blob-storage/tree/v0.100.11).

Rust 1.100.0, Tooling 0.44.5 (61-file snapshot), Platform 0.33.7, Runtime 0.22.5,
Types 0.20.4, Testkit 0.106.0/PocketIC 16.1.0 remain selected.
Both graphs use Wire 0.24.1 root codecs and explicit frames; release gates pass.
Local locks now select Host FS/Artifacts 0.80.2; Testkit retains older internal Host edges.
`blob` 0.100.12 is installed on PATH.
[Storage inspection](../operations/storage-inspection.md) owns CLI usage and limits.
Service owns one 64 KiB store and 64 historical uploads/references; release retains exposure. Supported [maintenance](../operations/standalone-maintenance.md)
drains calls and retains original identities; unknown execution is not safe retry.
No unrestricted-controller rollback/snapshot continuity guarantee exists.

Registry freshness review in `target/010012-release-suppliers.log` confirms current
first-party direct selections. Testkit's private Host 0.79 edges remain supplier-owned;
its latest 0.106.0 has no newer supported selection. No optional refresh is pending.
`make dependency-pins-check` now runs in Linux CI; hosted acceptance awaits a push.

**0.100.12:** `blob application catalog` / `draft --id ...`
inspect the signing user's existing reference-app records without mutation.
They reuse its protocol types and the CLI query transport; no new endpoints/state.
[Chromium/catalog composition](../design/reference-application.md#chromium-and-the-installed-catalog--development-010012)
also holds readback across stopped/drained upgrades, preserving original state/proof
and cancellation. Browser checks are opt-in; Rust owns signing. No deployment claim.

## Current provider work and next action

The maintainer permits Caffeine calls and the existing `canic-mainnet` identity
when exact operations and durable effect custody are ready. Do not ask again
merely to cross that boundary. No provider issue/email outreach is authorized;
never export the wallet key, import legacy custody or resume legacy obligations.

The [local callback/issuer candidate](../design/caffeine-contract-review.md#local-callback-candidate--development-01009)
uses Service and bounded receipts, retaining exposure after acknowledgment.
Original certified results remain recoverable after release, without revocation or provider acceptance. CLI retains one root-bound request before
one guarded Agent invocation; concurrent or partial claims refuse replacement.
Agent HTTP 429/503 retries retain the same packet; no exactly-one-HTTP-attempt claim.
Read-only recovery preserves original identities and existing output. The capacity
requirement is filed as [ic-storage-new #1](https://github.com/dragginzgame/ic-storage-new/issues/1).
Committed Storage 0.1.13 (`92af0d1`) adds installed native record-vector qualification,
but still no transition for this retained image; its owning issue remains open.
Reviewed committed source only; no sibling changes or execution.
The maintainer-approved [browser client](../../clients/browser/README.md) prepares
immutable bytes, accepts existing proof, claims before serial PUTs and verifies
reads; no automatic retry or Storage dependency.
Installed journeys join authenticated `transfer claim`, loopback PUT/read and the
reference catalog. Failures retain unpublished drafts; restarts refuse replay.
Explicit read-only reconciliation cannot override cancellation. Chromium also
authenticates local IC proof and retains 64 permanent claims under one fixed binding.
**Next:** qualify fresh application provisioning/deployed custody and the Storage-owned
retained-image transition before live-owner reuse; local upgrades do not close these gates.
Keep the live probe unchanged: its one-slot directory cannot host this composition;
reuse needs a Storage-owned migration and positive lifecycle witness. No retry,
reset, funding or new link follows from the local callback implementation.
[The contract review](../design/caffeine-contract-review.md#independent-qualification--2026-11-15)
and [compact observation](../design/caffeine-observation.json) own detailed evidence.

D0 stays open for enrollment, callback identity/object scope, enforced bounds,
original-proof recovery/lifetime and charge closure. D1 is partial. D2 now has
funding, installation, allocation, owner-origin refusal and a missing-tree HTTP
404 witness, **no positive gateway-admission or upload witness**. Keep declared bytes, unresolved exposure,
and unfrozen API/persistence. Standalone exports and Service format remain unchanged.

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

Final-version 0.100.12 `make ci` passes (`target/010012-release-ci.log`). All eight
installed suites, both Chromium/catalog upgrade journeys, three Chromium transport
cases and PATH installation pass (`target/010012-release-installed.log`). Original
custody prepared by the flat-command 0.100.10 CLI remains recoverable. The new
application queries check owner isolation, escaped names and unchanged stable state.
Hosted acceptance of this checkpoint remains pending delivery.
Browser evidence is local Linux only; no live/provider custody changed.
Node 24+ and explicit `make install-browser-tools` select npm Platform 0.2.0
and esbuild 0.28.2. Profile rollback, eviction, multi-device custody and live provider
acceptance/settlement remain unqualified. The consumed enrollment probe remains
0.100.8; source inputs stay in installation custody's `source-before-release-01008/`.

Canic adapter/catalog at `b36f6a39a67f487def523c847423fdfdb8d60c50` has five installed
witnesses naming Blob `50fe84b`; do not relabel them for newer suppliers. D3 remains
partial: immutable adoption, explicit lifecycle inputs and fleet maintenance belong
to Canic. No Canic source changed. Legacy stays read-only at
`25bed1352f3d2b9659f0d2583f50fa9af859ea84`; reuse lessons for current consumers,
not its multi-operation journals, refund engine or substitute-service graph.

## Latest delivery review

Reviewed delivered `a7b0805..f61c414`: Host custody, grouped help, direct CLI/test
callers, docs and CI. No actionable sediment found in that scope. Old adapters and
flat commands are removed; predecessor handling is test-only. Testkit's older Host
graph remains a supplier boundary. Live custody and ownership remain unchanged.
