# Standalone Candid cleanup and Testkit 0.36 qualification

Reviewed 2026-10-13 over Blob 0.23.0,
`4277ed1ec059c1eb36e4bc5e32e7b6b7eb74ecc2`. Incoming root Cargo selections were
already Testkit 0.36.0, Memory 0.36.1 and Metrics 0.8.3, with one Host 0.16.0
graph. Copies and the incoming diff remain in
`.tmp/review-validation/cleanup0231/`; the task leaves those selections unchanged.

## Build boundary and measurement

[Issue #63](https://github.com/dragginzgame/ic-blob-storage/issues/63) concerns
optional extraction machinery in deployable Wasm. Default builds now omit it.
Native `candid_interface()` and the maintained export example remain available;
Wasm extraction explicitly selects the standalone `candid-export` feature.
The native declaration uses Candid's exporter directly; only extraction Wasm
needs the CDK pointer wrapper. No endpoint codec or service declaration is copied.

The baseline was rebuilt on the incoming graph before source edits. Both final
variants use the same lock, compiler and release profile. Binaryen applies
identical `--enable-bulk-memory-opt -Oz` arguments to baseline and production.

| Artifact | Before bytes | After bytes | Reduction |
| --- | ---: | ---: | ---: |
| Raw release Wasm | 4,236,286 | 4,172,431 | 63,855 |
| Binaryen Oz | 2,917,030 | 2,868,902 | 48,128 |

These are local standalone artifact sizes, not Canic/Toko measurements or
instruction-count claims. Hashes, build/optimizer logs and before/after `ic-wasm
info` outputs remain in the raw packet. The only removed function export is the
optional `get_candid_pointer`; callable IC and lifecycle exports remain unchanged.
Opt-in extraction Wasm is retained separately. Its extracted DID and native
example output are byte-identical. Default production output is rebuilt afterward.
The maintained parity case also rejects extraction machinery in deployment Wasm.

## Qualification and boundaries

Locked fetching, explicit Testkit CLI/server setup and offline admission passed.
Workspace all-target/all-feature denied-warning Clippy and the full advertised
Rust 1.88 native/Wasm checks passed. All affected fixture Wasms are rebuilt before
local PocketIC execution. Raw and Oz artifacts each pass interface parity, actual
installation/wrong-service refusal and stop/start/repeated-upgrade fence
preservation: six passing case executions. Formatting, tool admission, dependency
declarations and runtime-free contracts checks pass.
No full CI/release gate, native macOS, Canic build, provider operation or paid
effect is claimed. Package versions, index and Git refs remain user-owned.

Blob #64 is closed against delivered 0.23.0 and its original evidence. Canic's
root and both consumer locks now select Blob 0.23.0 / Memory 0.36.1; the source
composition and exact decoder were inspected read-only. Fresh managed composition
proof and family delivery remain Canic #527/#444/#524. Historical results cannot
qualify its newer dirty graph. Hosted native acceptance remains Blob #56–#59.
