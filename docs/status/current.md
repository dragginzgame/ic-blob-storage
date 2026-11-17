# Current status — 2026-11-18

## Delivery and active work

Private `dragginzgame/ic-blob-storage`, branch `main`; **0.104.0 is pushed and
published** for all four product packages at
[`98344f1`](https://github.com/dragginzgame/ic-blob-storage/commit/98344f1e8ce697ca37f57a1f81d8b97928afe3f9),
with annotated tag `v0.104.0`. Qualification remains independent and unpublished.
[Release procedure](../operations/releases.md); [native contract](../design/native-storage.md).

The published 0.104.0 uses OP storage. Current development uses Storage 0.78.0
and ordinary Wasm. Embedded hosts grant named allocations through the
[stable storage contract](../design/stable-storage.md).

The released locks select Platform 0.34.15, Protocol 0.3.5, Runtime/Macros/Guard 0.23.17,
Types 0.21.2, Storage 0.76.2, Wire 0.24.3, Testkit 0.110.0 and Host Process 0.81.0.
Host FS/Artifacts are 0.80.3. Toko 0.45.0's authenticated 92-file snapshot and CLI
binding agree. Rust remains 1.100.0. First-party freshness passed; optional
metadata-editor 0.87.4 adoption is deferred to the next tooling-maintenance pass.
Its compatible 0.73.0 two-lock interface remains qualified.

**Release gate:** all 17 selected stages passed, including strict product and
qualification Clippy/Rustdoc, 40 library tests, 21 CLI tests, four example tests,
Node tests and all eight installed suites (21 tests). Chromium remains opt-in.
The gate took 337 seconds with prepared tools/warm build outputs; callbacks (129s)
and maintenance (63s) dominate. Evidence: `target/validation/blob.spya37/` and
`target/01040-release/release.log`. Capacity stayed at zero stable bytes / 40-MiB
Wasm allocation. `target/storage-review/` retains earlier native observations.

**Publication:** all four downloaded registry archives match prepared bytes and
registry checksums: `target/01040-release/registry-verified.json`. Independent
GitHub retrieval verified packaged source under `target/publication/provenance-01040/`.
Upload custody: `target/publication/publish.selection.UGdk1j/`. No upload was retried.
PATH `blob --version` reports **0.104.0**; install log is in the release evidence.
GitHub push CI was observed in progress at
[run 44185191660](https://github.com/dragginzgame/ic-blob-storage/actions/runs/44185191660);
no hosted cache/upload/macOS or Chromium pass is claimed for this release.

CI now gives browser installation one owner and uses shared authenticated-download
cache and prepared-archive upload, retaining Blob's sanitizer and 14-day artifacts.
Wiring/sanitizer checks passed (69/18 cases; `target/ci-cleanup/`).
[CI ownership](../operations/continuous-integration.md). The scoped release fixture
passed 38 cases plus both-lock metadata projection (`target/dependency-refresh-20261117/`).
[Blob #3](https://github.com/dragginzgame/ic-blob-storage/issues/3) is delivered.

## Next action and provider boundary

The working tree now selects published Storage **0.78.0** in both locks
(`9777121`). Ordinary Cargo Wasm uses named stable cells. The [storage
contract](../design/stable-storage.md) covers host integration and upgrade
refusal. This public persistence and layout change targets the next minor
checkpoint (0.105.0). 0.104.0 remains the delivered version.

Local qualification passed on 0.78.0 across all eight installed suites (21
default tests), 39 library tests, 21 CLI tests, four example tests and 24 Node
tests. Additional installed runs populated hosts built from Blob tags 0.103.3
and 0.104.0 and the interim 0.77.0 worktree. Every incompatible upgrade
preserved the original module, stable bytes, drained attempt and unresolved
exposure. Same-layout upgrades passed at full history with 256 KiB of stable
memory. Build and test evidence is under `target/storage-078/`. Chromium and
live-provider execution were outside this run.

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

Post-push review of `v0.103.3..v0.104.0` covered native root ownership and bounds,
host lifecycle, retained replay/receipts, local release policy and shared CI wiring.
No actionable architecture sediment was found in that scope: no copied allocator,
compatibility backing, generic chunk store, duplicate policy ledger or recovery
framework. Published archives contain no qualification Wasm or private custody.
Live custody is unchanged; no live target, funding, relationship or tag was replayed.

Finite lifetime slots still bound sustained use. Provider acceptance/settlement,
Canic composition and unrestricted-controller snapshot continuity remain unqualified.
The last Chromium witness remains `6a369f8` with the 0.103.2 cohort; local Node
and installed tests do not establish browser profile rollback or multi-device custody.
