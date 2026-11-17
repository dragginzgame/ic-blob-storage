# Standalone capacity measurements

**Native OP requalification — 2026-11-17:** 0.104.0 uses Storage 0.76.2
and the [prepared native lifecycle](native-storage.md). The same three full-history
workloads pass with **zero stable bytes and 41,943,040 Wasm bytes (40 MiB)** at
every reported boundary. Inspection, replay, capacity refusal, cleanup and Keep
upgrades remain usable. Current results are in
`target/storage-review/native-installed-final.log` and `native-capacity-final.log`.
These allocation sizes include the supplier's reserved native windows; they are
not provider bytes or encoded record sizes. The native memory ceiling is 128 MiB.
The measurements below retain their original **Storage 0.74.1 baseline** identity.

Measured 2026-11-14 against the unmodified production host Wasm, using
`make test-capacity`. The current **64 lifetime upload / 64 lifetime reference**
profile remains usable at full history: inspection, historical replay, cleanup
and a drained same-artifact upgrade all pass. No limit was raised.

The 2026-11-15 workspace split moved this test under the qualification manifest.
The measurements and identities below retain their original pre-split provenance;
`make test-capacity` now resolves its harness through `qualification/Cargo.lock`.

Stable memory remained **655,360 bytes (640 KiB)** from empty installation through
full history and cleanup. Maximum observed Wasm heap was **1,245,184 bytes
(1,216 KiB)**, versus 1,179,648 bytes (1,152 KiB) initially. These are allocated
memory sizes, including runtime/storage overhead, not encoded record size or file
storage. That baseline host's stable-memory ceiling was 16 pages / 1 MiB.

## Workload and results

Three fresh canisters exercise different shapes, each with 64 upload IDs and 64
reference IDs: 64 registered distinct objects, one registered object with 64 uses,
and 64 unconsumed reservations. Requests use 29-byte application/uploader/manager
principals, IDs near `u128::MAX`, and `u64::MAX` deadlines. Each declaration is
1 MiB with one synthetic leaf/root and no metadata. No body is uploaded or read.
Limits allow 64 MiB reserved plus live bytes and 64 MiB reserved plus unresolved
bytes. The shared object counts once, not 64 times.

Values below are **millions of cycles**, rounded to three decimal places.
Each cell is instruction-execution charge / total balance debit.

| Operation | 64 distinct objects | 64 uses of one object | 64 reservations |
| --- | ---: | ---: | ---: |
| Authorize the first use | 5.706 / 9.056 | 5.706 / 9.056 | 5.706 / 9.056 |
| Authorize the 64th use | 6.977 / 10.327 | 6.875 / 10.225 | 6.893 / 10.243 |
| Inspect full usage, replicated | 5.250 / 8.302 | 5.250 / 8.302 | 5.250 / 8.302 |
| Refuse the 65th upload | 5.502 / 8.852 | 5.502 / 8.852 | 5.502 / 8.852 |
| Release the final reference | 6.946 / 10.040 | 6.931 / 10.025 | 6.940 / 10.034 |
| Upgrade at full history | 39.614 / 890.088 | 39.122 / 889.596 | 39.235 / 889.709 |

[All unrounded samples](standalone-capacity.tsv) include authorizations at 1, 16,
32 and 64 slots, registration, reference-capacity refusal, drain, first/final
release and post-cleanup inspection. `empty` rows report initial memory with zero
deltas; they do not measure installation cost. These are individual observations,
not averages, confidence intervals or throughput measurements.

The test verifies that:

- The 65th authorization refuses without a stable-state change. An independent
  `Retain` also refuses when reference history is full on either registered shape.
- All usage counters match the declared object topology at capacity. Both ordinary
  reference queries and replicated usage inspection remain available.
- Drain, stop, ordinary upgrade and start preserve stable bytes and accounting.
  All 64 original declarations replay after reopening while admission is drained.
- Every reference can be released/retired while full and drained. Releasing the
  first shared use retains the object for the other 63 uses.
- Cleanup releases reservations and live logical usage, but keeps registered
  unresolved exposure and all historical slots. After explicit activation, a new
  upload still refuses: **64 is a lifetime limit, not a concurrent-use limit**.

## Measurement method and limits

The [maintained test](../../qualification/ic-blob-storage-tests/tests/capacity.rs)
uses Testkit's selected PocketIC client and a production-configured application
subnet, without relaxed benchmarking instruction limits. No production endpoint,
instrumentation feature, alternate service or dependency was added. The focused
target is separate from the ordinary native CI aggregate.

Instruction-use evidence is the before/after difference of the platform's
`canister_metrics.cycles_consumed.instructions` counter, decoded through Platform's
bounded external-protocol codec. It measures **instruction-execution charges in
cycles, not raw instructions**. The selected client does not expose a per-call raw
instruction counter; no count is inferred from a pricing formula. Separate
`canister_status` observations supply balance and allocated Wasm/stable memory.
See the official [metrics/status contract](https://docs.internetcomputer.org/references/management-canister/#canister_metrics).

Each measured operation completes between observations on a canister with no
timers, heartbeats or provider calls. Metrics queries do not execute Blob code.
Inspection cost uses the query export in replicated update mode, not free query
latency. Upgrade measurements cover the upgrade call after stopping; they exclude
drain/stop/start, and are not an isolated post-upgrade-hook count. Total balance
debit includes other platform charges, not just instruction execution.

This qualifies the stated full-history shapes and current small-store policy.
It does **not** establish worst-case manifest/header decoding or hashing cost,
larger capacity, production asset churn, mainnet latency/pricing, cross-version
migration, raw instruction headroom, provider bytes or billing. D4 remains open:
finite retained identities still prevent sustained use without an accepted
permanent stale-request rule. File bodies remain outside the host.

## Reproduction and source identity

Run `make install-tools` if preparation is needed, then `make test-capacity`.
The target admits the snapshot, applies strict host/test Clippy, builds the real
host and emits measurements to stdout. It does not contact Caffeine.
The final run passed in 10.52 seconds (harness wall time, not canister latency).
A preceding passing run lacked the independent `Retain` exhaustion assertion and
was superseded by this final run.

Selection: pending work after `3c26f6c5c9bd0a32e0ad1a4c7d644f056c6c5e46`, workspace
0.100.5; Rust 1.100.0 (`0ef1bc386059596dada8fb65b50425aab2ea637a`), LLVM 23.1.5,
default Cargo release Wasm32 build without postprocessing; x86_64 Linux runner;
Testkit 0.106.0 / PocketIC 16.1.0; Platform 0.33.5, Runtime 0.22.5, Storage 0.74.1,
Wire 0.22.2; Toko Tooling 0.43.2 admitted unchanged. The existing Wire lock edit
was preserved, not introduced by this measurement.

| SHA-256 input | Digest |
| --- | --- |
| Production Wasm | `f64dab8b61876aa98c25cf961af69fc476647e281301e1be8bd832234d260b65` |
| Cargo.lock | `38059326dd07929003c4c673e0bd6de5f1e26a342ea588ccb54df3382a2e53e2` |
| Capacity test source | `fbc076ba76780d34fa9bf6609d829041e62a0b31475e377c2211e016294296ac` |
| PocketIC server | `b44e1eccd66e02328146b3e209e3403e3a3428a25d25555aca2f67500e5ae9db` |

The [legacy installed capacity checks](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/qualification/ic-blob-storage-pocketic-tests/tests/standalone_capacity/mod.rs)
and [capacity-boundary regression](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage/src/ops/service/uploads/tests/capacity_boundary/mod.rs)
preserve read-only observations, full-width identities and cancellation headroom
across reopening. Those invariants carry over; tenant enrollment, restore fences
and quota DTOs do not. The [legacy codec benchmark](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/docs/evidence/wire-bench0320.md)
measured instrumented codec regions in a private harness. Its raw instruction
numbers are not comparable to these unmodified whole-host execution charges.
