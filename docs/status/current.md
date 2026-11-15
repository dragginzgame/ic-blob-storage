# Current status — 2026-11-15

## Delivery and scope

Private `dragginzgame/ic-blob-storage`, branch `0.100`; **0.100.9 is pushed** at
[`c78dbcd`](https://github.com/dragginzgame/ic-blob-storage/commit/c78dbcd3e6d3f20de51e0e4167a270d2e54b47fa).
Remote branch and annotated `v0.100.9` peeled tag were verified. Four product
packages and both locks select 0.100.9; qualification versions remain independent.
Publication stays disabled. The delivery-only review belongs to this checkpoint.

Rust 1.100.0, Tooling 0.44.4 (49-file snapshot), Platform 0.33.7, Runtime 0.22.5,
Types 0.20.4, Wire 0.24.0, Testkit 0.106.0/PocketIC 16.1.0 are selected.
Both workspaces use Wire's root codec API and explicit payload/frame encoding.
Defer Wire 0.24.1's tooling-only refresh to next dependency maintenance; other direct
first-party releases match Cargo's latest (`target/01009-release-suppliers.log`).
`blob 0.100.9` is installed on PATH; `make blob` updates it through Cargo.
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

The [local callback/issuer candidate](../design/caffeine-contract-review.md#local-callback-candidate--development-01009)
uses Service and bounded receipts, retaining exposure after acknowledgment.
Original certified results remain recoverable after release; this neither revokes
authority nor qualifies provider acceptance. CLI retains one root-bound request before
one guarded Agent invocation; concurrent or partial claims refuse replacement.
Agent HTTP 429/503 retries retain the same packet; no exactly-one-HTTP-attempt claim.
Read-only recovery preserves original identities and existing output. The capacity
requirement is filed as [ic-storage-new #1](https://github.com/dragginzgame/ic-storage-new/issues/1).
The maintainer says Storage will be replaced; work around it for now. Its
`d409068` scaffold supplies no retained-image transition. No sibling edits.
The maintainer approved replacing the external-SDK-only rule with a minimal owned
[browser transfer client](../../clients/browser/README.md). It now snapshots bounded
bytes, prepares the declaration, takes existing proof and sends serial PUTs after
a durable claim, then verifies reads; no automatic retry or Storage dependency.
No legacy patch, SDK fork, private-method override or global fetch hook is adopted.
The installed journey authenticates original proof through offline `claim-transfer`,
sends actual loopback PUTs, verifies a six-chunk read and publishes through the real
reference app. Corruption, interrupted replies and termination leave unpublished
drafts; restarts cannot replay uploads. Explicit `readVerified` reconciliation now
uses one GET and the original declaration before a separate catalog decision;
cancellation still wins. **Next:** review the current Storage replacement for the
retained-image transition and select a supported host/client deployment before
live upload. Browser and provider qualification remain separate.
Keep the live probe unchanged: its one-slot directory cannot host this composition;
reuse needs a Storage-owned migration and positive lifecycle witness. No retry,
reset, funding or new link follows from the local callback implementation.
[The contract review](../design/caffeine-contract-review.md#independent-qualification--2026-11-15)
and [compact observation](../design/caffeine-observation.json) own detailed evidence.

D0 stays open for enrollment, callback identity/object scope, enforced bounds,
original-proof recovery/lifetime and charge closure. D1 is partial. D2 now has
funding, installation, allocation, owner-origin refusal and a missing-tree HTTP
404 witness, **no positive gateway-admission or upload witness**. Keep declared bytes, unresolved exposure,
and unfrozen API/persistence. The callback owner adds library behavior;
standalone exports and Service format remain unchanged.

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

Tooling was installed through its pinned installer; package paths match their names.
Both graphs have one Wire 0.24.0 identity. Final-version `make ci` and all eight
installed suites pass (`target/01009-release-{ci,installed}.log`), including 22 browser
tests and ten catalog scenarios. Earlier retained Wire 0.22.4 host/app/CLI checks
prove exercised record/custody reopening (`target/01009-wire-installed.log`), not
arbitrary compatibility or a live probe upgrade. Node 24+ is required; no npm install.
No deployed browser, provider acceptance or settlement claim is made. The
[enrollment probe](../design/caffeine-contract-review.md#single-use-enrollment-probe--local-qualification-for-01008)
remains 0.100.8; its live consumed attempt and custody are unchanged.
Exact live-probe inputs remain in installation custody's `source-before-release-01008/`.

Canic adapter/catalog at `b36f6a39a67f487def523c847423fdfdb8d60c50` has five installed
witnesses naming Blob `50fe84b`; do not relabel them for newer suppliers. D3 remains
partial: immutable adoption, explicit lifecycle inputs and fleet maintenance belong
to Canic. No Canic source changed. Legacy stays read-only at
`25bed1352f3d2b9659f0d2583f50fa9af859ea84`; reuse lessons for current consumers,
not its multi-operation journals, refund engine or substitute-service graph.

## Post-push architecture review — 0.100.9

Reviewed `86fc663..c78dbcd` after push using the installed baseline's pinned
complexity method and this release's AGENTS overlay: callbacks, CLI custody,
client/catalog callers, Wire adoption and qualification. No actionable sediment
found in that scope. Service owns Open/Closing references and exposure; callbacks
only retain gateway acknowledgments. CLI root/dispatch/transfer claims refuse
partial or repeated work. Client attempt/read outcomes never decide publication.
Protocol changes belong to client/issuer boundaries; retention changes belong to
Service. Reusing the live probe still needs Storage's retained-image transition.
Permanent claims and authenticated recovery remain necessary; no generic journal,
second catalog or codec compatibility layer was added. Legacy lessons cited in
the contract review are requalified locally; no legacy custody was imported.
Current Rust: 4,299 product, 9,091 tests, 396 examples, zero tooling; JS client: 244.
Physical-line growth versus 0.100.8: crates +1,166 (with tests), qualification +2,525,
clients +1,039 (with docs/tests). These are not code-LOC deltas. Growth covers real
interruption/replay boundaries. Live provider/browser qualification remains open.
