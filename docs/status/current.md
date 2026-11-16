# Current status — 2026-11-16

## Delivery and active work

Private `dragginzgame/ic-blob-storage`, branch `main`; **0.101.0 is pushed and
published to crates.io** for all four product packages at
[`7a9f723`](https://github.com/dragginzgame/ic-blob-storage/commit/7a9f723421b4b0a2f2a6fc64cbcdf2b8643bd4d4),
with verified annotated tag `v0.101.0`. Qualification remains independently
versioned and unpublished. [The release procedure](../operations/releases.md)
owns Git/registry delivery; the [application guide](../design/reference-application.md)
owns `blob app` and offline paired installation inputs. Release 0.102.0 is now requested.

**Pending 0.102.0, local only:** `blob app check-targets` observes both proposed
canisters through authenticated management status, requiring absent modules and
the signer as sole controller. It shares maintenance transport/decoding and creates
no install authority. [The guide](../design/reference-application.md#check-proposed-targets)
owns the freshness and non-atomic-observation limits.

**Partial Platform adoption:** both workspaces select Types 0.21.2 and authored hosts/probes
now use Runtime/Macros/Guard 0.23.13. Explicit bounded entrypoints preserve Wire
frames, trap rollback and lifecycle policy; consuming calls retain 30-second waits
and uncertainty. Runtime supplies canonical identities directly, removing temporary
host conversions. Wasm-only host modules/dependencies keep native DTO use independent.
Storage 0.74.1 retains Runtime 0.22.5's IC0-only edge: no second executor or guard.
Pending public identity changes select 0.102.0; package versions stay 0.101.0.

Latest evidence: `target/platform-0346-adoption/`. CLI and native qualification
Clippy, all five Wasm builds/Clippy, 13 CLI tests and installed standalone, callback
and signed-maintenance suites pass. Original IDs, late recovery and populated
pre-migration host upgrades survive. Both locked graphs, pins, binding, format and
links pass; `blob` is installed on PATH. Earlier host migration evidence remains in
`target/runtime-02311-adoption/` and `target/runtime-02312-adoption/`.
Chromium was not rerun. No live effects occurred.

**Native boundary:** Platform 0.34.6 now owns original ingress admission and request
IDs in the CLI and qualification; unknown authentication fields refuse without repair.
`ic-platform-legacy` explicitly identifies remaining 0.33.7 signing, transport,
certificate integration and external codecs in both workspaces and private probes.
The old direct `ic-protocol` feature is removed. Protocol 0.2.5's chunk APIs do not
cover Blob's management/Cashier formats; no unused dependency was added.
[Migration requirements](../design/application-boundary.md#platform-replacement-prerequisites--2026-11-16)
own remaining work. Provisioning stays paused; old native uses are not a frozen exception.

Rust 1.100.0, Tooling 0.44.9 (84-file snapshot), Platform 0.34.6/legacy 0.33.7,
Runtime 0.23.13, Types 0.21.2, Wire 0.24.2, Host FS/Artifacts 0.80.2 and Testkit 0.106.0 /
PocketIC 16.1.0 remain selected. Testkit owns its older internal Host edges.
Tooling 0.44.9 updates both exact pins/locks and the CLI fingerprint; executable
tool bytes are unchanged. Both-workspace admission and 13 CLI tests pass;
evidence: `target/tooling-0449-adoption/`. Named-supplier adoption includes native
and qualification callers; remaining Platform migration is explicitly partial.
npm Platform 0.2.0 is still current. The development CLI is installed on PATH.

## Next action and provider boundary

**Next:** supplier gaps now have Platform [#17 — signed transport/recovery](https://github.com/dragginzgame/ic-platform/issues/17),
[#18 — Caffeine codecs](https://github.com/dragginzgame/ic-platform/issues/18), and
[#11 — management codecs with Blob's added requirements](https://github.com/dragginzgame/ic-platform/issues/11#issuecomment-6927085084).
Adopt their APIs across both workspaces and qualify the CLI. Provisioning stays
paused; signed install custody/readback remain future work. Canic's fleet work is owned by
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

Release 0.101.0 passed `make ci`, all eight installed suites and archive/provenance
checks (`target/01010-release-corrected.log`, `target/01010-publish-check.log`).
Verified registry readback: `target/01010-registry-verified.json`.
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

Latest delivery review (`bbb37fc..7a9f723`) found no remaining actionable sediment
in that scope. Current target-observation work reviewed the pinned legacy checker
via GitHub; live/legacy custody remains untouched.
