# Current status — 2026-11-06

## Pending Build, Storage and Testkit adoption

Adopted published **Build 0.75.0 / Storage 0.68.0 / Testkit 0.88.0**, with registry
checksums verified against all three cached archives. Incoming Platform 0.23.2
and Wire/Derive 0.17.3 selections are preserved; nine unused transitive packages
drop out. Rust 1.99.1 and package versions remain unchanged. Engine's draft is
**0.74.0** (public Storage identity, collection formats and trap behavior); Client's
is **0.73.1** (native Build consumers); Contracts needs no new release.
Retire old whole backing before fresh installation. No converter/reset or wrapper
is added. Malformed stored payloads trap; framing/byte-limit refusals stay typed.
Two corruption tests now verify that contract and unchanged retained bytes.

Focused Linux checks pass: 153 service-owner cases, two native composition cases,
strict all-target/all-feature Clippy for Engine/CLI/PocketIC, standalone/probe
release Wasm builds, and installed CLI-carrier, same-release lifecycle and
write-trap rollback cases. Dependency pins/Platform boundaries and the standalone
normal/build Wasm exclusions pass. Lock-selected Testkit/Build tools are prepared
and admitted offline. No full release gate, native macOS, Canic or live-provider
rerun is claimed. Existing live trials retain their frozen tools/backing below;
these development artifacts are not replacements. No Git/release effects.

## Completed bounded 0.73 provider trial — 2026-11-06

The selected real-Caffeine two-uploader run passed on new standalone service
**`j7h7p-gyaaa-aaaaa-qhxwa-cai`**. Both original certificates/four PUTs succeeded;
two independent downloads verified all **2048 original bytes**. Cross-uploader
assessments are denied. Exact module/configuration/release/controller and
anonymous-refusal readback pass; incoming Platform/Wire lock edits were not used.
[The summary](../evidence/caffeine-probes/deployed/2026-11-06-trial-v073-two-users-01/summary.json)
and [ledger](../evidence/caffeine-probes/README.md#new-073-two-user-provider-trial--2026-11-06)
own intents, failures, the preserved certified early capacity refusal and explicit
unchanged-operation re-evaluation after capacity/absence checks. No certificate/PUT
was replayed. Both references stay live, zero active/reserved bytes, logical/physical/liability
**2048**, owners unfenced; both lifetime object slots are occupied. No release,
delete, settlement or reset occurred. Maintain these new continuing obligations.

One 2 ICP conversion, exact 2.0001T creation, one 1.001T gross external deposit
(1.0008T credited), and one new expiring 5T nominal daily relationship are retained.
Conservative conversion-plus-allocation count is 6.5288T within the selected 8T
envelope and retained 100T total / 20T daily trial authority; no further refill.
The original link expires **2026-11-06 09:56:32 UTC**, without renewal or a claim
that expiry ends billing. Final payer/owner/gateway observations are time-bound.
Private custody `.git/private-operations/blob073-two-users-01/` retains frozen
tools/keys/profiles/original journals and an inventory of 610 files (164,174,592 bytes),
excluding private keys. No external backup; development key never exported, default anonymous.
The 0.69 and older trials below are untouched. #5/#31 still need real application
login/CSP/catalog acceptance; #6/#34 retain deletion/billing and lost-payment
evidence. The trial changed no implementation/dependency or package release.

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

## Delivered 0.73 boundaries and qualification

Delivered Testkit **0.87.0** uses async calls and caller-owned runtimes/isolated
servers, without a blocking facade or network deletion from client Drop. Its
frozen graph selects Platform **0.23.0**, Host Tools **0.83.0**, Storage **0.67.1**,
Wire/Derive **0.17.2**, Runtime **0.19.2** and Types **0.18.1**. Direct Rust CBOR
is removed; external IC/provider codecs remain Platform-owned and app protocols Wire.

Shared Tooling **0.30.4**, committed `2aa41c7394300302fe802b69b31b855bdc89b671`,
has 168 verified snapshot files. The Platform check enforces lexical Candid source
and dependency boundaries; it does not establish macro expansion/type resolution.
Rust **1.99.1** is minimum and pinned compiler. Browser Platform **0.2.0** supplies
typed ingress/fixture APIs; SDK Core **6.1.1** remains behind Platform and Caffeine's
explicit override. Saved bytes, limits, bindings and retry policy are preserved.

The 22-stage Linux release gate passed. Its projected final 0.73 cohort executed
**644 native cases and 352 installed PocketIC cases** (82 Client, 270 Engine),
strict Clippy, schema/frame/#117 boundaries, Wasm and actual archive checks.
Client/Engine reused nine admitted candidate stages; actual-source identity,
dependency/compiler, archive and registry checks remained fresh. Publication
admitted retained Clippy. Supplier identities/browser lock stayed fixed.

Additional prerelease checks passed browser bundles, 21 Node cases, three installed
certificate cases and ten opt-in Chromium scenarios, corrected for trusted
registration and retained physical/billing obligations. They establish neither
native macOS, actual Canic nor live-provider acceptance.

Retained evidence: `target/testkit087-migration/` (release/publication logs,
`release-reuse-summary.json`, `registry-readback-01/delivery-readback.json`),
`target/blob-release-validation.EfH3HF/validation.VLyndM` (first gate),
`target/blob-release-validation.Jzeudv/validation.To2FwU` (Client) and
`target/blob-release-validation.aq8MnK/validation.qXsAAp` (Engine). Original candidate
proofs/logs remain under `target/release-stage-cache/`; group plans/receipts remain
in Git release-state and committed release files. The [evidence index](../evidence/README.md)
explains historical custody limits; provider/recovery and frozen fixtures remain.

## Remaining application/provider acceptance

Four issues remain open: [#5 publication/browser serving/integrity](https://github.com/dragginzgame/ic-blob-storage/issues/5),
[#6 reuse/capacity/deletion/billing](https://github.com/dragginzgame/ic-blob-storage/issues/6),
[#31 actual multi-user transfers](https://github.com/dragginzgame/ic-blob-storage/issues/31)
and [#34 attributable original paid-operation credit/replay](https://github.com/dragginzgame/ic-blob-storage/issues/34).
Local integration or publication does not satisfy their live acceptance.
Next select the actual application origin/network/service binding with Canic's
owner and follow [the acceptance plan](../acceptance-plan.md#next-application-acceptance-run).
November 6 read-only Canic review finds Engine 0.72.2 / Contracts 0.72.0 in all
three locks at unchanged base `e6a061999`. Its handoff reports dedicated/embedded
and five-profile tests on an earlier lock; named raw output is unavailable here.
The 0.73 family, explicit generated-client case and final graph remain Canic #622
acceptance; [the owner follow-up](https://github.com/dragginzgame/canic/issues/622#issuecomment-6680798989)
records the reviewed graph and completed provider sample. Both private consumers are local-only with delegated-token Auth
disabled; no demo/test Blob role or actual application binding was found. No
Canic files/builds, new deployment, provider effects or package changes occurred.

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

Keep the native publication/session controller and browser integration per the
November 4 decision until an actual Canic application selects its publishing path.
Adapter non-use alone does not establish dead code. Canic owns its wrapper and
application acceptance; no sibling source was edited. Older preparation remains
in the [committed pre-delivery handoff](https://github.com/dragginzgame/ic-blob-storage/blob/4eb56823b75ea8dd140e81d675518722ff62f08f/docs/status/current.md).
