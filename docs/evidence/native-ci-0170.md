# Native CI repair and pending Memory identity cut

Reviewed 2026-10-07. Entry HEAD is
`aadfe16216ddfc20f8dd68de048b0846c54dcfb3`. The maintainer authorized the three
findings in [#24](https://github.com/dragginzgame/ic-blob-storage/issues/24), then
explicitly selected **0.17.0** to preserve the existing Memory 0.31 requirement.
The complete former 0.16.1 draft is carried forward. Package metadata and the
release receipt remain 0.16.0.

## Remote failure evidence

[Exact-source run, attempt 1](https://github.com/dragginzgame/ic-blob-storage/actions/runs/37606775023):

- Linux completes the configured checks.
- [ARM macOS](https://github.com/dragginzgame/ic-blob-storage/actions/runs/37606775023/job/112744064921)
  fails on the HTTP fixture's response-write `BrokenPipe`, then its thread join.
  This is a mock-server failure; the log does not establish production retries.
- [Intel macOS](https://github.com/dragginzgame/ic-blob-storage/actions/runs/37606775023/job/112744065069)
  exceeds the configured 30-minute limit. Setup takes 6m54s, tooling checks
  13m45s, cache preparation 28s, and the native target receives only 8m30s before
  cancellation during CLI tests. Examples and PocketIC do not execute there.

The raw failed/Intel logs, job summary and timeout annotation are preserved under
`.tmp/native-ci-0161-01/`. That directory retains its initial patch-target name.

## Applied repair

The HTTP fixture bounds request lines/headers to 16 KiB, service bodies to
256 KiB and accepted sockets to 16. It drains each complete service body before
writing a complete 429/503 response. Subnet `read_state` lookup sockets remain
open until shutdown: the SDK's concurrent query/key lookup can cancel one when
the other fails. The fixture therefore tests service backpressure independently
of the cancelled lookup. Only empty disconnected sockets after a service response
may be skipped; partial request lines and service response errors still fail.
The existing exactly-one-service-operation assertion remains for query/update.

New regression coverage sends a fragmented 8 KiB service request alongside a
cancelled lookup, checks the exact backpressure response, and checks typed
oversized-header/body and truncated-body refusals. The workflow budget increases
to a bounded 60 minutes without dropping matrix hosts or checks. The release
guide now records released 0.16.0 and pending 0.17.0.

## Focused local qualification

Linux, selected locked graph: Memory 0.31.1, direct Host artifacts/fs 0.4.1,
private-probe Metrics 0.2.7 and testkit-owned Host 0.3.3. The entry working lock
already selected these updates; Cargo.toml, Cargo.lock and docs/release.json
remain byte-identical to the saved entry copies.

- `cargo fetch --offline --locked` prepares the selected cached graph.
- The first HTTP test attempt compiles but socket binding is denied by the
  sandbox. Its complete log is retained as `http-tests-01-sandbox.log`; its
  compiler warning for an unnecessary `mut` is also retained and corrected.
- The permitted retry of the three HTTP tests passes. Full focused CLI binary
  regression passes: 105 blob-storage and five caffeine-probe cases.
- Strict CLI binary/test Clippy passes, as does Rust 1.88 compilation of those
  targets. Targeted rustfmt, workflow actionlint, local documentation links and
  diff whitespace checks pass.

Final source/tool/dependency/log identities are retained in
`.tmp/native-ci-0161-01/SHA256SUMS`; the two final tested CLI test executables are
bound there too. The initial sandbox-denied attempt is failure evidence, not a
successful source or platform qualification. Previous adoption records remain
unchanged and keep their original source, dependency and artifact identities.

Cached published Memory 0.31.1 runtime files match 0.30.0 byte for byte; the
source comparison and file hashes are retained. Nevertheless, Blob publicly
re-exports Memory types, so changing the selected crate line from released 0.30
to 0.31 requires hosts to align and rebuild their composed graph. No durable
schema change is introduced. Cross-release retirement and reinstall still apply.

These local checks do not qualify native macOS, the complete new Wasm/PocketIC
graph or downstream managed compositions. #24, [#19](https://github.com/dragginzgame/ic-blob-storage/issues/19)
and [#21](https://github.com/dragginzgame/ic-blob-storage/issues/21) await their
matching committed native runtime checks. No full CI/release gate, workflow
rerun, commit, version mutation, publication, provider effect or sibling file
edit occurs. No existing named function, method or type is removed.
