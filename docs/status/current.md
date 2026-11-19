# Current status — 2026-11-19

## Delivery and active work

**0.106.2 is published** for the library, standalone canister, reference application
and CLI. The annotated [GitHub tag](https://github.com/dragginzgame/ic-blob-storage/tree/v0.106.2)
points to `b7dc3cece8c2cda8251abf49588e644fbd21f049`. The CLI on PATH reports
`blob 0.106.2`. Both workspace locks select Storage 0.78.4, Wire 0.25.1 and
Toko Tooling 0.45.16. Qualification stays private.

All 17 local release stages passed, including eight installed suites. This run
covered the Node browser client. The optional Chromium and historical-layout
cases were outside the run. See `target/validation/blob.Tu4dtQ/proof.json` and
the [release commit’s hosted CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/44504517777).

All four archives downloaded from crates.io matched the prepared bytes and
registry checksums. See `target/01062-release/registry-verified.json`,
`target/publication/provenance-01062/` and `target/publication/publish.selection.kDZP5C/`.
The release metadata editor remains at IC Build 0.73.0. Evaluate 0.87.5 in the
next release-tooling maintenance pass with the two-lock metadata fixture.

## Next action and provider boundary

Canic can integrate the released named-lease API and qualify its own composed catalog.
Populated 0.103.3 or 0.104.0 installations still require separate disposition.

[Storage #6](https://github.com/dragginzgame/ic-storage/issues/6) tracks native
consumer acceptance; [Storage #1](https://github.com/dragginzgame/ic-storage/issues/1)
separately owns the old consumed image. Canic owns fleet integration in
[#12](https://github.com/dragginzgame/canic/issues/12) and
[#13](https://github.com/dragginzgame/canic/issues/13); no sibling source changed.
Its older `0167a30` witness pins Blob `decde204` (0.100.6) and does not qualify
this stable-memory boundary. Do not add another Blob adapter here.

D0 remains open for supported enrollment, namespace/scope, enforced bounds, proof
recovery/lifetime and charge closure. D1/D3 remain partial; D2 has no positive live
gateway/upload witness. Preserve declared-byte accounting, unresolved exposure
and unfrozen API/persistence. [Contract review](../design/caffeine-contract-review.md#independent-qualification--2026-11-15)
and [compact observation](../design/caffeine-observation.json) own provider evidence.
Caffeine calls and `canic-mainnet` remain authorized only with concrete bounded
operations and original effect custody. No outreach, key export, implicit retry,
reset, new owner, funding or link is implied by release or local qualification.

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

## Review and limitations

Post-push review of `v0.106.1..v0.106.2` confirmed that native and Wasm builds
use the same catalog and enrollment state and the same standalone and callback
codecs. Four IC host modules and the reply probe’s trap hook retain target guards
for Wasm-only Platform Runtime APIs. Enrollment now keeps its record and stable
cell together. The release introduced no second storage implementation.

Finite lifetime slots still bound sustained use. Provider acceptance/settlement,
Canic composition and unrestricted-controller snapshot continuity remain unqualified.
Chromium qualification covers a single browser profile. Profile rollback and multi-device custody remain open.
