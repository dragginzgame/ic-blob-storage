# Current status

Date: 2026-10-11

## Released source and current batch

Released Blob is `bc2b9e7e78bdc5116fdb8f838dd37a1f5c957185` (0.22.9),
validated parent `e10b65982848af332d4ee3f0bd2934166f64fa33`. All twelve package
versions and the finalized receipt remain 0.22.9. Its lock selects Memory
0.35.10, private Metrics 0.6.4, Testkit 0.33.7 and all four Host 0.13.5.
The committed continuation is `cdb2e9dd8bd6db5f8e1957dfcba36664dfff2150`:
direct Host Artifacts/FS 0.14.1, Memory 0.35.11, Testkit 0.33.8,
cc 1.8.0 and serde_json 1.0.153. Preserve the subsequent working-lock edits
selecting Metrics 0.6.5 and serde_json 1.0.154. No dependency reselection is
performed.

The maintainer's retained `20261011T150509Z-1337626` CI attempt fails at the
offline Wasm fixture build because Metrics 0.6.5 is missing from its cache.
Its earlier stages compile serde_json 1.0.153, so their results do not qualify
the current 1.0.154 graph. Explicit `make deps` now passes, both updated archives
match their lock checksums, and `CARGO_BUILD_JOBS=2 make test-fixture` builds all
six release Wasm fixtures offline with Rust 1.99.0. The lock stays unchanged at
SHA-256 `32cf16cfe5deae89d2a386ba9d127f9262b499e8262141da2c1769f8837c48a0`.
Inputs, original failure, cache checks and successful commands remain in
`target/review-validation/metrics06502210/`; original release-failure logs are
untouched. Settle and commit the lock before fresh complete maintainer validation;
this focused retry is not a complete gate. Tests remain offline and never fetch
implicitly.
Read-only review of current Shared 0.6.3 and a substitute dispatch fixture are
reported under [Shared #129](https://github.com/dragginzgame/shared-tooling/issues/129).
The existing release runner rejects changed source after successful validation;
the proposal concerns earlier refusal at consumer-owned stage boundaries.

Pending compatible 0.22.10 now adopts committed Shared 0.6.3
`4fd979d13c32adb3cb2f51f213ebfc4f98339b3d`. Eight selected files change from
0.6.2 within the same 102-file roster. Normal direct releases can supersede
preparation after a clean descendant commit records local metadata disposition;
the old plan is retained and current metadata receives fresh preflight/full
validation. Explicit resume and later stages keep exact recovery. The existing
conditional Cargo failure and padded-count fixes remain adopted. No real release
or recovery is run. Public Blob contracts, durable schemas, tool pins and Rust
1.88 consumer support remain unchanged.

Earlier reviewed direct Host Artifacts/FS 0.14.0 archives match their recorded
lock checksums and released source `4723befe8d65c760c00e05c42652759d691fbd5e`.
Their source APIs are unchanged
from 0.13.5; no public Blob Host type or CLI behavior changes. All-target native
CLI/contracts/example tests pass, as does actual Rust 1.88 all-target compilation
for those packages. Their normal graph stays runtime-free. Published Testkit
0.33.8 retains its Host 0.13.5 family; [Testkit #61](https://github.com/dragginzgame/ic-testkit/issues/61)
owns the unpublished 0.34 migration needed for full Host alignment. Preserve
that owner boundary rather than adding dependency overrides or sibling paths.

Released Linux archive CI fails because the real Cargo fixture pins cfg-if 1.0.4
while locked cache preparation fetches 1.0.5. The exact retained artifact/logs
prove the missing-cache refusal. The fixture now selects the workspace-locked
version; original failure and corrected success are exercised with a fresh
private cache containing only 1.0.5. Corrected warm-target/current-value and all
archive/metadata/drift refusals pass on Bash 5.2 and genuine Bash 3.2.57.

Explicit Testkit 0.33.8 preparation and Shared 0.6.2 common setup remain
recorded against their original inputs. Current complete offline tool admission
passes; 0.6.3 changes no installer/pin and needs no reinstall. PocketIC stays
Testkit-owned 16.1.0 and previous installations remain retained. Current focused
release-adapter checks pass on Bash 5.2; the matching shared runner and Blob
premature-completion fixtures pass on genuine Bash 3.2.57. Snapshot/declaration,
link and shell checks pass. See [the current adoption record](../evidence/shared063-host01402210.md)
and raw `target/review-validation/shared063-host01402210/`. The
[preceding record](../evidence/shared06202210.md) and
`target/review-validation/continuation02210/` retain 0.6.2 setup, collector and
cold-cache archive results. No old compiler/evidence result is relabelled.

Canic's published adapter 0.111.0 remains authenticated in
[the release review](../evidence/canic0229.md). Its owner now reports both
independent managed fixture locks qualified with Blob/contracts 0.22.8,
Memory 0.35.9, Metrics 0.6.4 and Timers 0.17.10 in the pending uncommitted
0.111.1 batch. Complete Fast artifacts, canonical Candid parity, tenant/uploader
authority, discarded-result recovery, fences and same-release embedded
application-state restoration pass in that report. Fresh fixture installs do
not qualify production Root binding, cross-release upgrades or actual provider
behavior. The service-gap table no longer describes the adapter as undelivered.

No full CI/release gate, sibling edit, staging/commit/push, version transaction,
publication, deployment, paid provider effect or cleanup is performed here.
The maintainer owns complete validation and release execution.

## Issues and next work

- Matching [0.22.9 CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38144539773)
  has a successful Rust 1.88 lane, Linux archive failure and both macOS lanes queued
  at inspection. The Linux cache defect is repaired locally above. This is not a
  complete green gate and cannot qualify dirty 0.22.10 inputs. [#59](https://github.com/dragginzgame/ic-blob-storage/issues/59)
  retains matching native Shared setup/admission and artifact upload/readback;
  [#58](https://github.com/dragginzgame/ic-blob-storage/issues/58) retains matching
  hook temporary-path acceptance. #56/#57 retain their older exact archive/graph
  requirements. Cancelled or artifact-only lanes are not complete acceptance.
- [Canic #444](https://github.com/dragginzgame/canic/issues/444#issuecomment-6109464895)
  owns pending fixture delivery and remaining application/provider acceptance.
  [Toko Miner #6](https://github.com/dragginzgame/toko-miner/issues/6) owns production
  installation and frozen Release-profile acceptance. Adapter publication and
  disposable application acceptance are delivered; the Toko per-project journey
  is separate from Toko Miner.
- [#31](https://github.com/dragginzgame/ic-blob-storage/issues/31) retains actual
  two-user application/provider upload; [#33](https://github.com/dragginzgame/ic-blob-storage/issues/33)
  retains real verifier operation. Framework fixtures do not establish successful
  provider completion.
- [#32](https://github.com/dragginzgame/ic-blob-storage/issues/32) needs authoritative
  provider GC/missing-method/deletion/billing meaning before safe callbacks.
  [#34](https://github.com/dragginzgame/ic-blob-storage/issues/34) needs authenticated
  payer/credit/complete account activity before [Canic #494](https://github.com/dragginzgame/canic/issues/494)
  can qualify guarded repeated funding. Preserve journals and fences; do not
  manufacture receipts, placeholder deletion responses or paid probes.
- #4/#5/#6 retain consumer media-map, browser serving and physical/economic
  lifetime acceptance. Core primitives and operator recipes already exist.

[Prior handoff](review-before-02210.md) retains the 0.22.9 preparation and Testkit
0.33.7 prerequisite repair. Historical handoffs are evidence, not current authority.
