# Stable storage

The standalone host and embedded applications use the published ic-storage 0.78.0 named allocation directory. The host declares all names in one Storage runtime, bootstraps their authorities and passes each exclusive memory lease to its owner. An embedded application can keep its own catalog in a separate lease and call Blob locally. [Supplier release](https://github.com/dragginzgame/ic-storage/tree/97771217f32b58adf56f5f31e82bbbca6c067667)

| Owner | Stable key | Wire payload limit |
| --- | --- | --- |
| Blob service | `blob.control.v1` | 64 KiB for configuration, maintenance attempt, up to 64 lifetime uploads and 64 lifetime references |
| Callback qualification | `blob.callbacks.v1` | 16 KiB for the gateway binding and acknowledgment history |
| Reference app | `catalog.control.v1` | 64 KiB for configuration and 16 lifetime drafts |
| Enrollment qualification | `enrollment.control.v1` | 16 KiB for the original intent, attempt and outcome |

Each owner writes one bounded Wire payload to a governed stable cell. Reopening checks the cell envelope and declared length before allocating the payload, then validates the record and installation binding. Blob reconstructs reference counts, indexes and declared-byte usage from the retained records. The finite history limit makes rewriting and validating the complete record practical. Final release preserves historical identities and unresolved exposure.

## Installation and upgrade

Use `make build-host` or `make build-reference-app`. The selected Cargo Wasm artifact is installable directly.

1. Fresh installation requires empty physical backing and creates each cell in an empty lease.
2. Drain admission and finish outstanding runtime calls. Stop the canister and issue an ordinary upgrade with the source hook enabled. The CLI signs `Upgrade(None)`.
3. Before constructing Storage, the target requires existing stable pages. Load the original cells, validate the expected installation and preserve the drained maintenance attempt. Start the host, inspect its state and explicitly activate the current attempt.

A populated 0.104.0 OP host may have zero stable pages. It cannot upgrade to this layout. Storage 0.78.0 also rejects the prior 0.77.0 stable directory. Missing cells and malformed records fail reopening. Existing installations require their own disposition. The consumed live enrollment probe remains under [Storage #1](https://github.com/dragginzgame/ic-storage/issues/1). The previous release has its own [OP contract](native-storage.md).

## Qualification

`make test-standalone` exercises ordinary upgrades with live and released references. Two opt-in tests take incompatible predecessors built with each tag's own `make build-host`. Supply the prepared 0.104.0 module in `BLOB_OP_WASM` or the 0.103.3 stable module in `BLOB_INCOMPATIBLE_STABLE_WASM`. Each test populates a disposable instance, drains it and attempts an ordinary upgrade. After refusal it checks the installed module against the original SHA-256, compares stable bytes and reads back the maintenance attempt and outstanding exposure. The same stable-directory test covers the interim Storage 0.77.0 development build.

`make test-capacity` exercises distinct, shared and reserved objects at full history. [Standalone measurements](standalone-capacity.md)

The bounded control record and lifecycle guard adapt Blob's [0.103.3 implementation](https://github.com/dragginzgame/ic-blob-storage/tree/v0.103.3/crates/ic-blob-storage/src/local/persistence) to the new Storage directory and upstream cell layout. Closed admission and retention of obligations across upgrades also follow the [legacy lifecycle regression](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/qualification/ic-blob-storage-pocketic-tests/tests/standalone_lifecycle/mod.rs).
