# Canic composition review after Blob 0.22.2

Reviewed 2026-10-10 from released Blob
`35bd0aed37bdea67dfbb9f96706b8c273e685023`. This is read-only source review of
Canic, whose HEAD is `ac55e50334dd6479ec36f404e89e60bcfe9184d6` and whose
working tree is extensively dirty. HEAD alone does not identify those inputs.
Selected file hashes/status remain in
`target/review-validation/shared0350223/canic-inputs.sha256` and `canic-status.txt`.
No Canic file, consumer build, deployment or provider effect is performed here.

## Current source boundary

The adapter belongs to Canic's main virtual workspace, inheriting its package
version, catalog and root lock. Dedicated and embedded consumer examples retain
their two independent locks. All three catalogs/locks now select Blob runtime
and contracts 0.22.2. Their current compilation and managed execution have not
been repeated by this review.

Canic's own handoff records passing dedicated/embedded Blob 0.22.1 / Memory
0.35.0 qualification: complete Rust 1.91 managed builds, eight normal-Wasm role
graphs with one Memory/Timers identity, both strict 32-method Candid projections,
and both managed PocketIC installation/recovery cases. That retained packet is
`canic/target/review-validation/blob0221-20261010/`; it is distinct from the
newer source selections, native macOS and registry-only application acceptance.
Canic #444 subsequently records passing Rust 1.91 native/Wasm adapter checks
after the root-workspace consolidation and 0.22.2 selection, with unchanged
captured inputs. Complete managed 0.22.2 execution remains separate.
[Canic #444](https://github.com/dragginzgame/canic/issues/444) owns delivery;
[#510](https://github.com/dragginzgame/canic/issues/510) owns the framework pool.

`mount!()` contributes Blob's seventeen permanent requests. The application
explicitly grants `MEMORY_AUTHORITY` / `MEMORY_KEY_PREFIX` in its sole
`memory_allocation_pool!`; the dedicated `canister!()` supplies that pool. Embedded
application state has a separate namespace in the same pool. Lifecycle install
and restore are synchronous; endpoint mounting does not bootstrap memory or
initialize the service. Refresh [the adoption recipe](../dependencies.md#canic-integration)
to this contract rather than carrying numeric-range or Memory 0.33 instructions.

The adapter's funding endpoints/workflow still provide history, exact outcome
and preparation assessment. They do not dispatch guarded payments or confirm
provider credit. [Canic #494](https://github.com/dragginzgame/canic/issues/494)
owns that host composition;
[Blob #34](https://github.com/dragginzgame/ic-blob-storage/issues/34) retains the
provider-owned receipt/account-activity prerequisite. The
[funding recipe](../funding-consumer-qualification.md) preserves both boundaries.

Multi-user authority and independent completion verification remain present in
Blob and Canic's earlier neutral managed fixtures. Actual application login,
verifier operation, browser certificate/provider success, GC and billing evidence
remain with [#31](https://github.com/dragginzgame/ic-blob-storage/issues/31),
[#33](https://github.com/dragginzgame/ic-blob-storage/issues/33),
[#32](https://github.com/dragginzgame/ic-blob-storage/issues/32) and
[Canic #444](https://github.com/dragginzgame/canic/issues/444). This review finds
stale Blob-owned guidance to correct; it does not supply missing external evidence
or authorize a new framework dependency here.
