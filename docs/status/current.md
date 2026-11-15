# Current status — 2026-11-15

## Delivery and scope

Private `dragginzgame/ic-blob-storage`, branch `0.100`; **0.100.8** is prepared
for the requested GitHub checkpoint. Product crates and both locks are coordinated;
publication stays disabled. “Push” means a new GitHub patch checkpoint.

Release validation passed: 46 native tests, 11 installed cases, strict Clippy,
Rustdoc, formatting and links; logs are `target/release-01008-{ci,installed}.log`.
Product LOC remains 3,617; tests 6,392 (+886), examples 396, unclassified Rust 0.
Post-push review remains due after the delivery; these checks do not qualify
live-provider closure or upload admission.

Rust 1.100.0, Tooling 0.44.3 (49-file snapshot), Platform 0.33.7, Runtime 0.22.5,
Types 0.20.4, Wire 0.22.3, Testkit 0.106.0/PocketIC 16.1.0 are selected.
`make blob` installs the current development executable on PATH through Cargo.
[Storage inspection](../operations/storage-inspection.md) owns CLI usage and limits.
Product Service still owns one 64 KiB store and 64 historical uploads/references;
final release retains exposure. Supported [maintenance](../operations/standalone-maintenance.md)
drains calls and retains original identities; unknown execution is not safe retry.
No unrestricted-controller rollback/snapshot continuity guarantee exists.

## Current provider work and next action

The maintainer permits Caffeine calls and the existing `canic-mainnet` identity
when exact operations and durable effect custody are ready. Do not ask again
merely to cross that boundary. No provider issue/email outreach is authorized;
never export the wallet key, import legacy custody or resume legacy obligations.

**Next:** qualify the [local provider-host slice](../design/caffeine-contract-review.md#gateway-preflight-and-next-local-host-gate--2026-11-15)
around the existing durable Service and one explicit callback profile. The probe's
persisted Storage directory has one occupied allocation slot; select and test a
lifecycle path preserving its receipt before any service deployment. Raising
runtime limits cannot expand that persisted capacity. The probe is consumed after a retained provider
refusal; no retry, reset, positive-cycle fallback or new link follows from that.
[The contract review](../design/caffeine-contract-review.md#independent-qualification--2026-11-15)
and [compact observation](../design/caffeine-observation.json) own detailed evidence.
DFINITY's example is pinned at `ef29e8a6e8063c6fe654cac53a3497cab585fefa`; its
no-expiry example did not establish expiry/limit behavior. Static inspection of
the published CLI recovered intended nanosecond expiry and midnight reset rules;
the binary was never executed/adopted. Live expiry/limit enforcement is still open.

D0 stays open for enrollment, callback identity/object scope, enforced bounds,
original-proof recovery/lifetime and charge closure. D1 is partial. D2 now has
funding, installation, allocation, owner-origin refusal and a missing-tree HTTP
404 witness, **no positive gateway-admission or upload witness**. Keep declared bytes, unresolved exposure,
and unfrozen API/persistence. Product Rust source is unchanged in this checkpoint;
product version metadata advances to 0.100.8 without changing supplier selections.

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

All six custody directories are private/Git-ignored, not disposable build output.
The installed-probe bullet above describes its initial state; it is now consumed.
A requested target balance is not an enforced liability cap. Mainnet/root-key
selection is explicit; the existing named signer was used without exporting keys.

## Qualification and integration

The independent unpublished enrollment probe is version 0.100.8. It retains one
immutable zero-attachment Cashier intent/result, records before await, refuses
repeat dispatch, and preserves results through revocation and supported upgrades.
[Probe contract, legacy comparison and live installation](../design/caffeine-contract-review.md#single-use-enrollment-probe--local-qualification-for-01008)
distinguish synthetic provider tests from actual observations.

The release reran `make test-enrollment`, including maximum record size, pending
state reopening, three installed cases and three offline example tests.
`decode-top-up` classifies Candid separately from Wire; it never signs/sends or
infers credit from an Ok tag. Test-only lock additions support held replies and
the offline Clap example; no supplier refreshed. Before the release version bump,
the exact installed-probe inputs were retained under its private installation
custody's `source-before-release-01008/`, verified against the original intent.

Canic adapter/catalog at `b36f6a39a67f487def523c847423fdfdb8d60c50` has five installed
witnesses naming Blob `50fe84b`; do not relabel them for newer suppliers. D3 remains
partial: immutable adoption, explicit lifecycle inputs and fleet maintenance belong
to Canic. No Canic source changed. Legacy stays read-only at
`25bed1352f3d2b9659f0d2583f50fa9af859ea84`; reuse lessons for current consumers,
not its multi-operation journals, refund engine or substitute-service graph.
