# Current status — 2026-11-17

## Delivery and active work

Private `dragginzgame/ic-blob-storage`, branch `main`; **0.103.3 is pushed and
published** for all four product packages at
[`3b5e0b5`](https://github.com/dragginzgame/ic-blob-storage/commit/3b5e0b52e7c72d0ffb44f50a5682566b907a71f5),
with annotated tag `v0.103.3`. Qualification is independent and unpublished.
[Release procedure](../operations/releases.md); [application guide](../design/reference-application.md).

**Platform adoption:** Types 0.21.2, Runtime/Macros/Guard 0.23.17 and Platform
0.34.15 are selected in both lockfiles. Protocol 0.3.5 supplies management and
Caffeine codecs; **no legacy Platform remains in product or qualification**.
The native API owns signing, one-attempt dispatch, original-request recovery,
signed queries and exact proof authentication. Custom roots require an explicit
root-subnet principal. Wire calls and original custody are preserved; native persistence changes below.

**Tooling 0.45.0:** both pins/locks, authenticated 92-file snapshot and CLI binding
are updated. CI has one browser install owner, shared authenticated-download cache
and prepared-archive failure upload (14 days), retaining Blob’s bounded sanitizer.
Wiring/sanitizer checks pass (69/18 cases; `target/ci-cleanup/`). Hosted cache/
upload/macOS remain unobserved. [CI ownership](../operations/continuous-integration.md).
Rust 1.100.0 and Wire 0.24.3 remain selected; Host FS/Artifacts use 0.80.3.
Qualification selects Host Process 0.81.0 and Testkit 0.110.0; FS/Artifacts 0.80.3
are still latest. Only the requested supplier selections changed in both locks.

## Next action and provider boundary

**Pending 0.104.0 — native OP adoption:** both workspaces select Storage 0.76.2,
source `f9fa03611ebb8641412a2af0f8620b80909c164f`. No old backing alias/runtime
remains. Service/Callbacks take supplier grants through concrete Access; the
standalone host, reference app and probes use bounded domain rows, source close/
target reopen and prepared native Wasm. The CLI signs explicit Keep upgrades.
[Native contract](../design/native-storage.md) owns layouts, composition, build
admission and the hard cut from 0.74.1. Packages stay 0.103.3 until release.
Testkit 0.110.0, Runtime/Macros/Guard 0.23.17 and Host Process 0.81.0 are adopted.

**Native OP baseline (before this dependency refresh):** all eight installed suites
passed (21 tests), plus 40 library tests, strict Clippy/Rustdoc and examples.
Capacity passed at zero stable bytes / 40-MiB Wasm allocation; five artifact
identities remain in `target/storage-review/`. Native root/projection checks replace
old stable-memory assertions. No runtime Wasm dependency changed in this refresh.
**Current refresh:** both locked metadata graphs, all harness tests/examples under
strict Clippy, host/Rust/simulator tool admission and 21 CLI tests pass.
Installed endpoints (native trap rollback) and enrollment (held replies/receipts)
and the signed CLI lifecycle pass (six installed tests). All 38 release fixtures
and both-lock metadata projection pass. Evidence: `target/dependency-refresh-20261117/`.
Full CI/other installed suites, Chromium, macOS and Canic were not rerun.
[Storage #6](https://github.com/dragginzgame/ic-storage/issues/6) owns consumer
acceptance. [Storage #1](https://github.com/dragginzgame/ic-storage/issues/1)
separately owns the old consumed image: no live target/custody was changed.

**Local release-flow fix ([Blob #3](https://github.com/dragginzgame/ic-blob-storage/issues/3)):**
One roster drives CI and scoped releases; shared/unknown inputs select full.
The native preparer is compiled once per Make invocation and artifacts select
admitted prepared bytes. All 38 disposable real-Make dispatch/recovery checks
and two-lock metadata projection pass (`tooling-consumers.log` in the refresh). No release
or product speedup is claimed. Next: run the 0.104.0 gate and publish on main.
Canic owns integration in [#12](https://github.com/dragginzgame/canic/issues/12)
and [#13](https://github.com/dragginzgame/canic/issues/13); its adapter must adopt
Blob’s native grant/root boundary and the supplier build/lifecycle contract.
No duplicate Blob adapter or sibling source change is needed here.
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

Release 0.103.3 passed the complete gate and archive/provenance checks.
All four downloaded registry archives match the prepared packages; readback:
`target/01033-release/registry-verified.json`. Source provenance fetched the exact
GitHub commit independently (`target/publication/provenance-01033/`).
Both opt-in Chromium targets last passed on `6a369f8` with the 0.103.2 code and
locks: three transport cases and two installed catalog/upgrade cases.
Evidence: `target/browser-refresh-01032/qualification.log`; Chrome for Testing
156.0.8078.4, Node 24.21.0, npm Platform 0.2.0 and esbuild 0.28.2.
Transport/replay, verified reads and upgraded-catalog cancellation pass; no fix was needed.
PATH `blob --version` reports 0.103.3. Chromium/macOS were not rerun for this release;
hosted CI was not awaited. Browser profile
rollback/eviction, multi-device custody, live acceptance and settlement are unqualified.

Service keeps a 64-KiB logical budget and 64 historical uploads/references; cleanup retains
exposure. [Maintenance](../operations/standalone-maintenance.md) preserves original
identities and closed admission; unknown execution is not safe retry. There is no
unrestricted-controller snapshot/rollback continuity guarantee. The consumed probe
stays 0.100.8; original source remains in installation custody's
`source-before-release-01008/`.
Current committed Canic `0167a30` pins Blob `decde204` (0.100.6) immutably; its older
composition evidence is not current-supplier qualification. The two Canic issues
own real fleet initialization and maintenance. No Canic source changed or ran.

Release freshness: selected first-party libraries/tools are current. The compatible
metadata editor stays 0.73.0; optional 0.87.4 adoption is deferred to the next tooling
maintenance boundary. Its existing two-lock interface remains qualified.
Live custody is byte-identical to HEAD. No sibling, tag, push or live effect changed.
