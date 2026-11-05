# Current status — 2026-11-05

## Delivered Contracts, Client and Engine 0.73.0

The authorized Testkit adoption and #117 delivery are complete on `origin/main`.
All 13 members select 0.73.0; all three public packages are published, non-yanked,
and independently verified against registry checksums and exact tagged sources.

| Group | Release commit | Receipt |
| --- | --- | --- |
| Contracts | [`2ae0ba1b`](https://github.com/dragginzgame/ic-blob-storage/commit/2ae0ba1b5c60576faa334b9cb8d65c5b2ea8d5ae) | [0.73.0](../../releases/contracts/release.json) |
| Client | [`5382ca23`](https://github.com/dragginzgame/ic-blob-storage/commit/5382ca2389c1af5609fce484f7a97fca942f9834) | [0.73.0](../../releases/client/release.json) |
| Engine | [`6811018e`](https://github.com/dragginzgame/ic-blob-storage/commit/6811018e57c4424a812d3eb94ce5e00a2fcef43b) | [0.73.0](../../releases/engine/release.json) |

[#117 is closed/completed](https://github.com/dragginzgame/ic-blob-storage/issues/117#issuecomment-6665892409).
Only `CaffeineUploadCertificateResponse`'s unused Wire metadata implementation was
removed. The external Rust path, Candid/Serde support, provider method and
`method`/`blob_hash` record remain; the owned assessment retains its Wire schema.
The host trait removal is the explicit 0.73 API cut. Endpoint identities, external
Candid bytes, application Wire frames, limits and saved records are unchanged.
No replacement DTO, compatibility wrapper or automatic migration was introduced.

## Adopted boundaries and qualification

Published Testkit **0.87.0** uses async calls/setup/lifecycle helpers directly.
Tests own runtimes and isolated managed servers; ordered original-operation,
chunk and journal checks remain sequential. Pure subprocess helpers stay sync.
Rust client Drop performs no network deletion; the owned server stops its isolated
instances, including panic cleanup. No blocking compatibility facade was added.
The older native Platform 0.22 edge is gone; the graph selects Platform **0.23.0**.
Host Tools **0.83.0**, Storage **0.67.1**, Wire/Derive **0.17.2**, Runtime **0.19.2**
and Types **0.18.1** retain their canonical ownership. Direct Rust CBOR is removed;
external IC/provider codecs remain Platform-owned and application protocols Wire.

Shared Tooling **0.30.4**, committed `2aa41c7394300302fe802b69b31b855bdc89b671`,
has 168 verified snapshot files. The existing Platform check enforces Candid source
and dependency boundaries: 41 exact external-protocol allowances across 836 Rust
files; lexical checking does not establish macro expansion/type resolution.
Rust **1.99.1** is both minimum and pinned compiler. Browser Platform **0.2.0**
supplies the pending typed ingress/fixture APIs; SDK Core remains **6.1.1** through
Platform and the explicit external Caffeine override. Request IDs, original saved
envelopes, bounds and application trust/retry policy are preserved.

The 22-stage Linux release gate passed. Its projected final 0.73 cohort executed
**644 native cases and 352 installed PocketIC cases** (82 Client, 270 Engine),
strict Clippy, schemas/service declarations, frozen frames, the #117 compile-fail
boundary, Wasm and actual archive checks. Client and Engine each admitted all nine
eligible retained candidate stages; actual-source version/CLI identity, metadata,
rollback, dependency/compiler, archive and registry checks remained fresh.
All three publications admitted retained Clippy qualification. Supplier identities
and the browser lock stayed fixed from the qualified source preparation.

Additional prerelease checks passed browser bundles, 21 Node cases, three installed
certificate cases and the opt-in Chromium certificate test's ten scenarios.
The latter corrected obsolete verifier-era assertions: registration publishes
retention intent and charges reserved usage; withdrawal preserves the original
reference, physical bytes and liabilities. Production behavior did not change.
Native macOS, actual Canic and live-provider qualification were not rerun.

Retained evidence: `target/testkit087-migration/` (release/publication logs,
`release-reuse-summary.json`, `registry-readback-01/delivery-readback.json`),
`target/blob-release-validation.EfH3HF/validation.VLyndM` (first gate),
`target/blob-release-validation.Jzeudv/validation.To2FwU` (Client) and
`target/blob-release-validation.aq8MnK/validation.qXsAAp` (Engine). Original candidate
proofs/logs remain under `target/release-stage-cache/`; group plans and receipts
remain in Git release-state and the committed release files. Artifacts were kept.
The [evidence index](../evidence/README.md) explains retired diaries and historical
custody limits; tracked provider/recovery payloads and frozen fixture bytes remain.

## Remaining application/provider acceptance

Four issues remain open: [#5 publication/browser serving/integrity](https://github.com/dragginzgame/ic-blob-storage/issues/5),
[#6 reuse/capacity/deletion/billing](https://github.com/dragginzgame/ic-blob-storage/issues/6),
[#31 actual multi-user transfers](https://github.com/dragginzgame/ic-blob-storage/issues/31)
and [#34 attributable original paid-operation credit/replay](https://github.com/dragginzgame/ic-blob-storage/issues/34).
Local integration or publication does not satisfy their live acceptance.
Next select the actual application origin/network/service binding with Canic's
owner and follow [the acceptance plan](../acceptance-plan.md#next-application-acceptance-run).
The last read-only Canic review found Blob 0.71.1, no selected demo/test Blob role,
and no actual application binding; it is not a new Canic build/adoption claim.

The standalone **0.69.0** trial at **`4h7vw-4qaaa-aaaau-ahieq-cai`** is unchanged.
Its [probe ledger](../evidence/caffeine-probes/README.md) owns immutable phase
records; [November 5 readback](../evidence/caffeine-probes/deployed/2026-11-05-trial-v069-gc-readback-01/summary.json)
shows first object `Live`, second `DeletionPending`, zero gateway acknowledgments,
and logical/physical/liability bytes **1024/2048/2048**. A possibly cached HEAD 200
proves neither origin deletion nor billing cessation. Both lifetime slots remain
occupied. User 1's certificate/tree PUT 403 stays held: no retry, replacement
certificate, reset or release. User 2 verified all 1024 original bytes; only its
reference was logically released with original operation 7 and receipt inspection,
without resend. That is one isolated test-key success, not two application users.

Retain **1.001T gross / 1.0006T credited**, original deposits/receipts and the 400M
fees. The approved 5T daily payer relationship **expired November 3, 17:48:30 UTC**;
no implicit renewal, refill or repeated uncertain paid effect is authorized.
Recorded payer/owner/gateway balances are time-bound, not fresh billing evidence.
No provider acknowledgment was fabricated and no physical settlement was invoked.
Private custody remains `.git/private-operations/blob069-trial-01/`, including
original user profiles/journals, frozen CLI/Wasm, `gc-release-01/` and
`gc-readback-20261105/`. Historical raw scratch `target/post069-provider/` is absent
in this checkout; resume from private copies, without claiming an external backup.
Keep the failed first root, released second root and older 0.6/0.7 obligations.
Cross-release changes require retirement with original tools before fresh whole
backing; no fallback reader, stale activation, automatic reset or financial retry.

Keep Blob's native publication/session controller and supported browser integration
per the November 4 maintainer decision until an actual Canic application selects
its publishing path. Framework adapter non-use alone does not establish dead code;
Canic owns its wrapper and application acceptance. No sibling source was edited.
Earlier completed and superseded preparation is preserved at the
[committed pre-delivery handoff](https://github.com/dragginzgame/ic-blob-storage/blob/4eb56823b75ea8dd140e81d675518722ff62f08f/docs/status/current.md),
not appended to this current summary. This delivery published libraries; it did
not reinstall the frozen live service or perform new provider/financial effects.
