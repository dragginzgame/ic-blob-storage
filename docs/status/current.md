# Current status — 2026-11-06

## Delivered Engine 0.74.1 — frozen batch

Engine **0.74.1** is pushed and published. The annotated `engine-v0.74.1` tag
and remote main resolve to
[`3a4ff2d0`](https://github.com/dragginzgame/ic-blob-storage/commit/3a4ff2d0b83bfc4ec040a6c5b80bfa122c8603dc).
Independent registry readback admits the non-yanked archive, checksum
`32eeaa36b17e16a4ab0816ae3fe9ea8ca8220f28598179a73a1ba8f1fd3be04e`
and exact release VCS identity. Only `ic-blob-storage` was published; Contracts
**0.73.0** and Client **0.73.1** stay unchanged. The
[Engine receipt](../../releases/engine/release.json) records all 21 passing stages.
No live service, provider payment, deletion or artifact cleanup occurred.

The maintainer selected Shared's frozen method after the original normal gate
from `c8309234` stopped on an incoming Platform 0.23.3 → 0.23.4 lock change.
That attempt created no release plan/tag/push or upload. Retain its original
`target/blob-delivery0741.X3oeZj/release.log` and
`target/blob-release-validation.CtNBeF/validation.ZX6tmx`; its adapter regression
passed before source admission correctly refused the changed inputs.

The successful batch **`batch-QfoyYuxC`** captured clean source `7f7c0d5b` once.
Its delivery checkout is `target/release-frozen/batch-QfoyYuxC`; durable Git,
complete intent and standard release plan remain under
`.git/release-state/frozen/batch-QfoyYuxC/`. Use the direct CLI in
[the release guide](../releasing.md), with an explicit supplier-first roster,
ordinary shell context and short source-independent scratch. The child rebuilds
its own source/tool routes; never publish a stage-only prospective candidate.
Later normal-checkout lock edits did not change the captured source or archives.
The completed batch must not be uploaded again.

The canonical snapshot selects **Shared 0.30.8** at
`dc9df707b3b1e616973494eab961c1c214d12260` (171 files), including generated-ID
resume support. Consumer hook qualification passed for all three groups, deferred
invalid lock isolation, uncertain publication reconciliation and idempotent
completed resume. Those tests use local Git and compiler/upload substitutes;
the actual release independently passed locked builds, installed behavior and
all package archive checks. Retain `target/frozen-release-adoption/` for export,
fixture, delivery and authenticated readback logs. Published archive/readback
artifacts are under the captured checkout's `target/publication.hMnJBJ` and
`target/publication.jgzyTI`.

The **21-stage Linux/Rust 1.99.1 gate** passed with **644 native tests** and
**352 installed tests** (82 Client / 270 Engine), plus the actual-source standalone
CLI carrier smoke check. Retain
`target/release-frozen/batch-QfoyYuxC/target/blob-release-validation.8Q9Bk1/validation.9M4tL5`
and that checkout's stage-cache records/builds. Publication admitted retained
final-candidate Clippy rather than running full lint again. All three libraries'
actual archives remain checked; no narrower archive proof or elapsed-time saving
is claimed. Native macOS, new II/browser, Canic and new live-provider acceptance
were not rerun. Hosted [example installation](https://github.com/dragginzgame/ic-blob-storage/actions/runs/42235651929)
and [release/formatting tooling](https://github.com/dragginzgame/ic-blob-storage/actions/runs/42235651948)
were still in progress at readback; these are separate from the completed local gate.

The released graph includes **Testkit 0.89.1 / Platform 0.23.4**, Runtime/Macros/
Guard 0.19.3, Types 0.18.2, Wire/Derive 0.17.4, flate2 1.1.11 and cc 1.11.1.
Build 0.75.0, Storage 0.68.0, Foundation 0.74.4 and Host Tools 0.83.0 remain.
Twelve unused logging/transitive packages disappeared; no replacement subscriber
or compatibility wrapper was added. PocketIC remains **16.1.0**.

Shared CBOR inventory/source checks replace duplicate local codec rules; the
current lexical scan covers 836 Rust files with 41 external Candid declarations
and no CBOR source allowances. Strict all-target/all-feature Clippy replaces the
redundant automatic native check. Manual `make check`, distinct feature/Wasm/
Rustdoc/installed/archive checks and historical receipt admission remain. Eight
stages are eligible for source-bound reuse, including qualified Linux group
scope and exact managed PocketIC selection; parallel Make, custom targets and
other hosts stay guarded. Reuse refusals are printed and retained. Earlier focused
scope/tamper/failure qualification remains in `target/blob-stage-reuse.HKeoVT/`,
including its original relative-TMPDIR refusal and corrected diagnostic run.

The normal branch was safely fast-forwarded to the delivered commit. Its later
**Cargo.lock changes remain uncommitted and deferred**, not part of this release:
serde_spanned 1.1.3, TOML 1.2.1, toml_datetime 1.1.3 and toml_edit 0.25.19.
The exact pre-reconciliation bytes/patch are retained with the durable batch.
A structured comparison verified all 423 lock records were preserved, changing
only Engine member versions for the delivered release. These incoming dependency
updates have not been qualified by the frozen gate; do not attribute its evidence
to them. Preserve the deferred lock and every retained provider obligation below.

## Delivered Client 0.73.1 and Engine 0.74.0

Both changed groups are pushed and published. Registry readbacks verify non-yanked
versions, archive checksums and VCS identities against the exact release tags.
Contracts remains 0.73.0; no unaffected group was republished.

| Group | Release commit | Receipt |
| --- | --- | --- |
| Contracts 0.73.0 | [`2ae0ba1b`](https://github.com/dragginzgame/ic-blob-storage/commit/2ae0ba1b5c60576faa334b9cb8d65c5b2ea8d5ae) | [Contracts](../../releases/contracts/release.json) |
| Client 0.73.1 | [`2e55591e`](https://github.com/dragginzgame/ic-blob-storage/commit/2e55591eeb7af7368761fa7685e6c2d2e7a5e72d) | [Client](../../releases/client/release.json) |
| Engine 0.74.0 | [`a7c6b374`](https://github.com/dragginzgame/ic-blob-storage/commit/a7c6b37402fd1f71a103063bd4c1acecc6332909) | [Historical Engine receipt](https://github.com/dragginzgame/ic-blob-storage/blob/a7c6b37402fd1f71a103063bd4c1acecc6332909/releases/engine/release.json) |

The graph selects **Build 0.75.0 / Storage 0.68.0 / Testkit 0.88.0**, Platform
0.23.2, Wire/Derive 0.17.3, Runtime/guard 0.19.2 and Types 0.18.1. Foundation
0.74.4/Tools 0.83.0 remain; the incoming cc 1.11.1 patch was included in the gate.
Nine unused transitive packages are gone. Shared Tooling remains **0.30.4** at
`2aa41c7394300302fe802b69b31b855bdc89b671`; Rust **1.99.1** is minimum and pinned.
Storage changes public identity, collection images and failure behavior: retire
whole old backing with original tools before fresh installation. Malformed stored
values trap; framing/byte-limit refusals remain typed. Propagate IC traps; native
mutation panics require discarding the runtime. No migration/reset/wrapper was added.

The complete 22-stage Linux gate passed with **644 native and 352 installed cases**
(82 Client / 270 Engine). Engine reused all nine eligible final-candidate stages;
actual-source identity, metadata, tools and archive checks remained fresh. Both
publications admitted retained Clippy. Hosted [CI 42201285250](https://github.com/dragginzgame/ic-blob-storage/actions/runs/42201285250)
was in progress at readback; its generated-client job passed. Native macOS, Canic
and new live-provider acceptance are not established. No live installation was
changed, and no provider payment or artifact cleanup occurred.

The first gate at `3a94dc10` failed only the old checkpoint corruption expectation.
The correction at `3f5faafb` covers retained copies and the owned decoder's refusal,
unchanged live state and original-byte recovery; no production checksum was added.
Retain `target/blob-release-validation.0tYTOP/validation.UOhp90` (failed),
`target/blob-release-validation.WK1eXo/validation.wNwYnP` (Client full gate),
`target/blob-release-validation.rgN3fH/validation.1wWF95` (Engine), and
`target/blob-delivery.Co3u1H/` (logs and `delivery-readback.json`). Registry archives
remain in `target/publication.6nhMT3` / `target/publication.51Se9T`; original proofs
remain under `target/release-stage-cache/`, with builds in `target/release-stage-build/`.
The frozen 0.73 qualification and closed #117 retain their scope in the
[previous delivery handoff](https://github.com/dragginzgame/ic-blob-storage/blob/6ff224ed8acb5ac846150e7f48cb59d3c6006105/docs/status/current.md).

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

## Remaining application/provider acceptance

Four issues remain open: [#5 publication/browser serving/integrity](https://github.com/dragginzgame/ic-blob-storage/issues/5),
[#6 reuse/capacity/deletion/billing](https://github.com/dragginzgame/ic-blob-storage/issues/6),
[#31 actual multi-user transfers](https://github.com/dragginzgame/ic-blob-storage/issues/31)
and [#34 original paid-operation credit/replay](https://github.com/dragginzgame/ic-blob-storage/issues/34).
Publication does not satisfy their live acceptance. Follow the
[application plan](../acceptance-plan.md#next-application-acceptance-run).
The earlier November 6 read-only Canic review found Engine 0.72.2 / Contracts
0.72.0 in three locks at base `e6a061999`, local-only fixtures with delegated-token
Auth disabled and no actual application binding. Reported older composition
results keep their own locks; their named raw output is unavailable here.
[Canic #622's delivered-family follow-up](https://github.com/dragginzgame/canic/issues/622#issuecomment-6682566587)
records Engine 0.74/Contracts 0.73 adoption, compatible Storage peers, both managed
shapes, five funding profiles and explicit generated-client qualification. Canic
owns its wrapper/application; no sibling source was edited or built here.

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
