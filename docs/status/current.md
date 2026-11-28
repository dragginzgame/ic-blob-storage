# Current status — 2026-11-28

## Delivery and active work

**0.114.0 is published** for all four product crates. Every crates.io download
matched the prepared archive and registry checksum. See the
[release tag](https://github.com/dragginzgame/ic-blob-storage/tree/v0.114.0),
`target/01140-release/registry-verified.json` and `target/publication/provenance-01140/`.

All 17 local release stages passed, including an ordinary upgrade from the
checksum-verified 0.113.2 standalone Wasm. See
`target/validation/blob.X4Jm3z/proof.json` and the
[release commit’s hosted CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/46132579082).

## Next action and provider boundary

Embedding hosts can now mount the [Caffeine certificate adapter](../../crates/ic-blob-storage/README.md#embedding-the-caffeine-upload-update).
The installed qualification host accepted the Caffeine SDK 1.1.2 request only for
the reserved uploader. Project integration and live gateway upload/readback remain downstream work.

Blob 0.113.0 uses Storage 0.80 for fresh installations and
same-format upgrades. Keep existing 0.110.0 installations on their original
backing and code until a separate migration is qualified. PocketIC verified
that refusing the upgrade from the tagged 0.110.0 build to 0.111.0 preserved
the original module, stable bytes, maintenance attempt and outstanding exposure.
See the [storage contract](../design/stable-storage.md) and
`target/01110-release/exact-tag-refusal.log`.

The Canic adapter owns the report projection and qualification with its
application catalog. To adopt Blob 0.113.0 it needs a Storage 0.80 lease and
IC Logging 0.26's `Level` type for its recorder. See the
[metrics source and logging hook](../../crates/ic-blob-storage/README.md#optional-operational-metrics).
Rust hosts using Blob’s public codec types should select Wire 0.32
and match admission failures as `Error::Limit(kind)`. The eight callable Wire
manifests match the 0.113.2 baseline byte for byte.

The release metadata editor remains at IC Build 0.73.0. Evaluate 0.87.8 in
the next release-tooling maintenance pass with the two-lock metadata fixture.
Populated 0.103.3 or 0.104.0 installations still require separate disposition.

[Storage #6](https://github.com/dragginzgame/ic-storage/issues/6) tracks native
consumer acceptance; [Storage #1](https://github.com/dragginzgame/ic-storage/issues/1)
separately owns the old consumed image. Canic owns fleet integration in
[#12](https://github.com/dragginzgame/canic/issues/12) and
[#13](https://github.com/dragginzgame/canic/issues/13); no sibling source changed.
Its older `0167a30` witness pins Blob `decde204` (0.100.6) and does not qualify
this stable-memory boundary. Do not add another Blob adapter here.

[Blob #6](https://github.com/dragginzgame/ic-blob-storage/issues/6) was closed as unsupported. Reconsider it when the provider can supply the
[catalogue completion witness](../design/caffeine-contract-review.md#catalogue-completion-and-recovery).
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

The tagged Wasms matched the reviewed callable API. The eight Wire manifests
add fixed-layout annotations to the existing Serde layouts.
Hosts own log retention and the metrics heap, and their Storage leases carry
the durable state. The published CLI archive fingerprints the installed Toko
Tooling snapshot. The native `inspection` feature exposes the supplier-owned schemas used by Canic.
The [API qualifier](../../qualification/api/README.md) checks the mounted contracts.

Finite lifetime slots still bound sustained use. Provider acceptance/settlement,
Canic composition and unrestricted-controller snapshot continuity remain unqualified.
Chromium qualification covers a single browser profile. Profile rollback and multi-device custody remain open.
