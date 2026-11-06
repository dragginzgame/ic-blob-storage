# Current status — 2026-11-06

## Engine 0.74.1 frozen delivery — in progress

The maintainer authorized release/publication of the pending Blob batch from
`/home/adam/projects/ic-blob-storage`, `main`, to `origin/main` and
`engine-v0.74.1`; only `ic-blob-storage` is selected for crates.io. Contracts
0.73.0 and Client 0.73.1 stay unchanged: no public contract or production
implementation changed. The first normal gate from `c8309234` stopped when the
preserved incoming lock changed Platform 0.23.3 to 0.23.4. The adapter regression
itself passed, but its source guard correctly refused reuse of that gate. No
0.74.1 plan, release/tag/push or upload was created. Retain
`target/blob-delivery0741.X3oeZj/release.log` and
`target/blob-release-validation.CtNBeF/validation.ZX6tmx` with the original
failure logs; the prerelease registry observation found 0.74.1 absent.

The maintainer then selected Shared's frozen delivery method. The canonical
export now selects Shared 0.30.8 at
`dc9df707b3b1e616973494eab961c1c214d12260` (171 files), including the generated-ID
resume fix. Use the direct CLI in [the release guide](../releasing.md), with an
explicit Engine-only roster and short source-independent scratch directory.
Do not run it through an outer Make exporting normal-checkout Cargo/tool paths.
Its captured source and durable journal keep later lock updates deferred.

Consumer hook qualification and publication controls are retained in
`target/frozen-release-adoption/`; both passed, including all three group hooks,
deferred lock isolation, uncertain publication reconciliation and completed-resume
idempotence. This uses local Git and compiler/upload substitutes, not product
acceptance. Shell lint/syntax, snapshot integrity and documentation links also
pass. The real frozen gate and independent publication readback remain pending. Capture Platform
0.23.4 and Testkit 0.89.1 with the other preserved selections below. No live
service, provider payment, deletion or artifact cleanup is part of delivery.

## Pending Shared 0.30.7 and Testkit 0.89.1

The canonical exporter adopted 171 files from pushed Shared revision
`22a853e26c05ec323f06c57fa316c08b808f252b`; `AGENTS.md` selects the same policy.
The existing Platform target now checks Candid plus CBOR across 836 Rust files,
with 41 external Candid declarations and no CBOR source allowances. Cargo uses
the shared codec inventory; the duplicate local codec rows are removed. That
inventory is a CI checkpoint input and also covers browser manifests/imports
across both package roots and scripts. Lexical/dynamic-generation limits remain
documented; transitive codecs behind their external owner remain permitted.

The interrupted Testkit request is complete at published `0.89.1`, archive
checksum `dd2879282c81fc23e57d00e89dd8f8a87564c624110f0c7bf800e704aaffa944`,
source `7d7ffbfcdd4a39061c0cd2c33809b661cd3c76e5`. Its runtime source matches
0.89.0; the repository II attachment fixture update is not local browser
acceptance. The twelve logging/transitive removals remain, with no new wrapper.
The incoming Platform 0.23.4 selection is preserved alongside prior lock edits;
the initial focused compilation below used 0.23.3.

Focused Linux/Rust 1.99.1 checks pass: snapshot integrity, actual Rust/Cargo and
browser boundaries, 18 browser-fixture cases, release routing for all three
groups, selector/receipt regressions, all-target harness Clippy, two typed-error
unit cases, standalone Wasm and one installed rollback/malformed-ingress case.
CLI setup/check selects PocketIC 16.1.0; dependency pins, formatting and docs pass.
The qualification inputs stayed fixed. Retain the export impact/refresh and
input hashes in `target/shared-tooling-0307/`.

The subsequent frozen-hook adoption and failed normal gate are recorded above.
Archive verification still conservatively checks all three published libraries;
no narrower archive proof or release wall-time savings is claimed. No full
release gate, macOS, new II/browser or live-provider qualification passed yet.
Notes join Engine 0.74.1; the preparation commit is local and undelivered.

## Pending native compilation consolidation

CI/release plans now use strict Clippy as the sole native all-target/all-feature
compilation stage. The optional `make check` recipe has matching packages,
target/profile and feature flags; no owned Clippy-specific compilation branch
was found. It remains available manually, with its mapping and historical
receipt interpretation retained. Feature-boundary, Wasm, Rustdoc, installed and
actual archive checks remain. The automatic roster has 21 stages, eight reusable.
`scripts/ci/test-release-validation.sh` passed: matching native recipe arguments,
four automatic Cargo commands, narrow scopes, failure stops and source-bound
receipt admission. All three current historical group receipts still verify.
Shell syntax/lint, eight-stage reuse-policy admission, documentation links and
diff checks pass. No full compilation gate, new dependency qualification or
release was run.

Read-only Query inspection found its current release roster already omits
native `check`; its normal CI runs Clippy plus distinct Wasm/feature checks.
No Query edits or qualification were performed. Notes join Engine 0.74.1;
no commit or delivery is authorized for this batch.

## Pending reuse diagnostics and scoped admission

Stage fallback now prints concrete eligibility reasons before execution and
retains them even when the fallback fails. Group candidate commands carry the
original plan's hash-bound package/installed selections; receipt admission checks
the binding. Exact managed `POCKET_IC_BIN` selection is preserved in the child
environment. Parallel Make, custom targets and other native hosts remain guarded.
Focused selector tests, Perl/shell checks, snapshot integrity and documentation
links pass. Real Cargo/PocketIC qualification covers first execution and reuse
for Client, Engine and explicit managed-server selections, broader installed
partition separation, scope tampering and all six failed-fallback diagnostics.
Publication still refuses a mismatched actual dependency closure.

Retain `target/blob-stage-reuse.HKeoVT/`: six scoped records and original logs,
including the initial relative-TMPDIR fixture refusal by earlier source admission.
The corrected long ignored temporary path and remaining diagnostics passed via
`scoped-diagnostics-resume.sh`; no product guard was relaxed for that fixture.
This is focused routing/reuse evidence, not a new full gate or native macOS
qualification. Engine 0.74.1 has pending tooling notes; no version, commit or
delivery effect.

A later unrelated lockfile edit selects Runtime/Macros/Guard 0.19.3, Types
0.18.2, Wire/Derive 0.17.4 and flate2 1.1.11. It is preserved, not qualified by
this fixture: the isolated source retained its original captured graph.

## Delivered Client 0.73.1 and Engine 0.74.0

Both changed groups are pushed and published. Registry readbacks verify non-yanked
versions, archive checksums and VCS identities against the exact release tags.
Contracts remains 0.73.0; no unaffected group was republished.

| Group | Release commit | Receipt |
| --- | --- | --- |
| Contracts 0.73.0 | [`2ae0ba1b`](https://github.com/dragginzgame/ic-blob-storage/commit/2ae0ba1b5c60576faa334b9cb8d65c5b2ea8d5ae) | [Contracts](../../releases/contracts/release.json) |
| Client 0.73.1 | [`2e55591e`](https://github.com/dragginzgame/ic-blob-storage/commit/2e55591eeb7af7368761fa7685e6c2d2e7a5e72d) | [Client](../../releases/client/release.json) |
| Engine 0.74.0 | [`a7c6b374`](https://github.com/dragginzgame/ic-blob-storage/commit/a7c6b37402fd1f71a103063bd4c1acecc6332909) | [Engine](../../releases/engine/release.json) |

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
