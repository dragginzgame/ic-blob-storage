# Library patch qualification for pending 0.21.1

Released base is `cea2d0f2740e0874a107a6a5e1c75a533b49e94b`, with validated
source `68822abb28d306c975fef90acbacecd07cd9b2cc`. All twelve package versions
and the finalized receipt remain 0.21.0. Preserve the incoming Cargo.lock
selections of Memory 0.33.1 and private Metrics 0.3.2; no agent resolution or
package version change is performed.
This compatible maintenance batch is recorded as pending 0.21.1.

## Source and scope

The published Memory archive checksum matches Cargo.lock:
`0e4f796a4fbe2ccb9a0082a0bf61841682285071259d59ef999e99af3e8283ab`.
Its 75 packaged Rust files and original manifest match owner release
`17372c0d1d71fc3516f415ff8c30057c4441bc31`. Runtime `src/` and the original
crate manifest are unchanged from 0.33.0 owner
`1cbecf601fd24afc64d1fcfccb29609537be36be`. The owner patch fixes its release
preparation and updates tooling/docs; it supplies no new Blob runtime helper.

The published Metrics 0.3.2 checksum also matches Cargo.lock:
`9c02088befd3a964640ed81463022876ed6c03ce8c29fd78a9115c77abf15941`.
Its seven packaged Rust files and original manifest match owner release
`f903c664c395c47485dbedc0f1919c97b9ce72d0`. Arithmetic `src/` and the original
crate manifest are unchanged from 0.3.1 owner
`2383dc0d684800b1610e3eb727449b0a87d562bb`. The owner patch qualifies its
private inspector and updates tooling/docs; Blob uses only the arithmetic crate.

Only Memory's and Metrics' versions/checksums differ from the released lock.
Dependency edges and all other selections remain unchanged: Host artifacts/fs/
process/tools 0.9.2, Testkit 0.27.0 and PocketIC client 16.1.0.
Shared remains the reviewed 92-file 0.2.4 snapshot from
`ffbf665b8481c36b2d9f4d988abec557c3485fa6`. Official registry readback still
selects Testkit 0.27.0 as the latest published stable release; committed but
unpublished 0.27.1 and dirty sibling work are not adopted. Siblings stay read-only.

No service API, Candid, stored schema, public Memory minor line, admission rule,
endpoint or lifecycle ownership changes. No local Rust symbol is removed.
Earlier 0.21.0 records retain their original graph and source identities.

## Focused checks

- `cargo test --offline --locked -p ic-blob-storage --lib ops::service::stores::
  --message-format=json`: 11 existing cases pass. They cover installed/missing
  grants, populated owner/neighbor preservation, changed-binding rejection
  without writes, restored-owner fencing and local operator/account composition.
- Rust 1.88 native and `wasm32-unknown-unknown` checks of `ic-blob-storage`,
  `ic-blob-storage-canister` and the Metrics-consuming `blob-storage-probe`,
  `--lib --all-features --offline --locked`, pass. Compiled package readback
  remains 0.21.0 and selects Memory 0.33.1 / Metrics 0.3.2.
- Snapshot, dependency declarations and maintained documentation links pass;
  whitespace review passes.

Entry Cargo files, locked cache preparation, offline metadata, official registry
and CI/issue readbacks, archive/source comparison, test/compiler logs and the
exact executed native test artifact hash remain in `.tmp/continuation0211-01/`.
Initial checks selected Memory 0.33.1 / Metrics 0.3.1. Input readback then detected
an external lock edit selecting Metrics 0.3.2; preserve both locks and the earlier
results without relabelling them. The corrected final source capture and focused
checks use both patches. All 14,060 final captured Rust/manifest/lock/toolchain
inputs remain unchanged; Cargo.toml, the final incoming lock and the release
receipt remain byte-for-byte intact. Initial source capture assumed an optional
`.cargo/config.toml` that does not exist and failed before compilation. A later
source check compared an annotated tag object with its commit and refused;
qualified review explicitly dereferences the commit. Original failed readbacks
and logs remain separate from successful capture and compilation.

The tests use native vector memory and an in-process query substitute. No
canister is created or installed, no HTTP provider probe or paid effect occurs,
and no server/tool setup or cleanup is performed. They qualify local storage
composition, not deployed provider behavior or instruction-count improvement.

## Acceptance limits

Released 0.21.0 [consumer CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/37932124645)
passes Linux and MSRV at readback. Both macOS jobs are queued;
[#39](https://github.com/dragginzgame/ic-blob-storage/issues/39) and
[#40](https://github.com/dragginzgame/ic-blob-storage/issues/40) stay open for
committed native acceptance. The dirty patch has no remote CI result, new
PocketIC lifecycle qualification or full release gate. Application/provider
acceptance remains with #31/#33, GC behavior with #32 and real funding evidence
with #34. No agent commit, push, release or registry publication occurs.
