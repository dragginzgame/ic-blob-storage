# Private protocol dependency removal — 0.19.3

Remove the unused `ic-blob-storage` dependency from `tests/protocol/Cargo.toml`
and its corresponding lockfile edge for
[#38](https://github.com/dragginzgame/ic-blob-storage/issues/38). The private
protocol owns passive test DTOs and uses contracts, Candid and Serde directly.
It has no core references, build script, optional features, generated core user
or deliberate core feature-selection requirement. Actual test consumers retain
their explicit core dependencies. No Rust function, method or type is removed.

The review starts at released `59230ee017dcbf9ff38c19f389b0bb03e750852a`
(0.19.2), preserving the prior pending fleet cleanup. Incoming Cargo.lock changes
select Host 0.8.8, Metrics 0.2.18 and Testkit 0.25.4. They are preserved, not
selected by this removal. Public Memory remains 0.31.9. This record does not
qualify unrelated Host/Metric/Testkit runtime behavior or claim build-time savings.

Retained inputs, lock comparison, metadata, compiler/lint logs and hashes live
under `.tmp/protocol0193-01/`. Initial locked offline cache preparation failed
because Metrics 0.2.18 was missing. Explicit `cargo fetch --locked` then prepared
the selected cache; no unlocked online resolution ran. Normal offline Cargo
metadata reconciled only the removed protocol-to-core edge. A parsed complete
lock comparison confirms every other incoming selection is unchanged, and a
subsequent `--offline --locked` metadata check passes.

Focused checks pass:

- Protocol native all-target/all-feature and Wasm library compilation; the
  selected protocol dependency closure no longer contains core or IC CDK.
- All native targets/features of the PocketIC harness and Wasm libraries of all
  six direct test-canister consumers.
- Protocol native/Wasm checks with actual Rust 1.88 and strict affected Clippy.
- Manifest ordering, dependency declarations, documentation links and whitespace.

These are compilation/graph checks, not a new lifecycle or live-provider probe.
No service, public wire/storage contract, package version or release receipt
changes. Pending 0.19.3 remains compatible. Full CI/release and new native macOS
qualification remain outstanding; the prior fleet evidence retains its own
unchanged released graph. No sibling edit, commit, push or publication occurred.

Released [0.19.2 CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/37901765903)
passes MSRV, Linux and Apple Silicon at the final read; Intel macOS remains
running. This result does not qualify the pending dependency removal.
