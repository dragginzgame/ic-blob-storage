# Continuous integration

[The repository workflow](../../.github/workflows/ci.yml) runs on pull requests,
pushes to `0.100` and `main`, and manual dispatch. It has read-only repository
permissions, pinned external action revisions and no deployment credentials.

- Linux owns `make ci`: tooling/pin admission, non-mutating formatting, strict
  product and qualification Clippy, local Rust/Node behavior, Rustdoc and links.
  `make build-host build-reference-app` also checks and builds the production
  Wasm boundaries. Setup prepares the compiler from `rust-toolchain.toml`, pinned
  common tools, both locked Cargo graphs and the browser's locked npm graph.
- macOS runs the existing reference, Caffeine integrity, local service, CLI and
  example tests. This covers native custody on macOS; it does not claim installed
  canister behavior. Linux owns common formatting and lint instead of repeating
  them in a host matrix.
- A separate Linux job validates the workflow and runs Gitleaks once over full
  checked-out Git history, including documentation-only changes. The admitted
  Toko snapshot owns the installers and version/checksum catalog. Findings fail
  the run with fully redacted output. `.gitleaksignore` names only five reviewed
  historical findings: public simulator trust material and deterministic test
  seeds. New occurrences require review; no file or detector is excluded.

CI deliberately selects setup targets instead of `make install-tools`, which
also provisions the optional simulator. Installed PocketIC suites, Chromium,
live-provider qualification, funding and deployment remain separate operations.
Build/formatting failure logs are retained for seven days. Missing or failed
hosted evidence is not a pass.

The initial workflow resolves [#2 — Complete hosted CI wiring](https://github.com/dragginzgame/ic-blob-storage/issues/2).
[Release 0.100.11's first hosted run](https://github.com/dragginzgame/ic-blob-storage/actions/runs/43859650612)
passed all three jobs on `f61c414`, including native macOS and production Wasm.
