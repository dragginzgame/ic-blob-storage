# Memory 0.36, Host 0.16 and Shared Tooling 0.9.13 adoption

Reviewed on 2026-10-13 over released Blob 0.22.16,
`de19bbc019fd18dac3206841a404db6b390719e7`. Incoming Cargo.toml/Cargo.lock
already selected Memory 0.36.0, direct Host 0.16.0 and Metrics 0.8.2;
original copies and diff remain under `.tmp/review-validation/adoption0230/`.
Those incoming selections are preserved, including Testkit 0.35.1's internal
Host artifacts/fs/process 0.15.2. No unrelated registry update was performed.

## Reviewed upstream contracts

Shared 0.9.13 is committed at `35cad65677ff989b5fb24a88d947fda37d2366e8`,
matching remote HEAD at selection. Canonical preview/refresh from a clean
isolated clone retains all 112 payloads. Local upstream 0.9.14 edits are excluded.
Store lock and source acquisition errors retain their status and diagnostics;
CI observation requires a real branch or explicit selection. Separate locked
source preparation is permitted; offline checks still do not install tools.

Memory 0.36.0 is published at `834fb71b09d84655dc64bd89b6bfb4c0c9b35c9d`.
Its request snapshot owns the component vectors, and declaration errors use
`MemoryDeclarationError`. Blob preserves its two request factories, seventeen
installation grants, permanent keys, schemas, single host bootstrap owner and
fail-before-write checks. Tests no longer observe the removed global registry;
no-bootstrap and unavailable committed-capability assertions remain.
The public re-export and typed request results make this a Blob **0.23.0** cut.
Cross-release retirement/reinstall requirements remain; the upstream source cut
itself changes no stable ledger encoding. Canic composition remains its owner's
work under [Canic #527](https://github.com/dragginzgame/canic/issues/527).

Host 0.16.0 removes default extraction features. Blob already explicitly disables
defaults and uses text/interface helpers, so no extraction feature is enabled.
The maintained CLI and contracts callers keep their existing filesystem,
artifact and interface boundaries. No new helper abstraction is needed.

## Focused qualification

Logs remain in `.tmp/review-validation/adoption0230/`, outside Cargo clean.
Locked source fetching passed before offline validation. Common Cargo setup
passed. Initial Testkit setup refused another owner's shared-store lock without
removing it; the later explicit retry and admission passed.

- Core unit tests: 346 passed; native memory composition: 2 passed.
- CLI binary tests: 118 passed, including permitted local loopback HTTP fixtures.
- Workspace all-target/all-feature denied-warning Clippy passed.
- Rust 1.88 checks passed for both public libraries, all native workspace targets
  and supported Wasm libraries. The advertised floor remains unchanged.
- Canonical Rust, host and IC installer fixtures passed on Linux; the Rust
  fixture also passed with genuine Bash 3.2. The initial incorrect Bash executable
  invocation is retained separately from its successful retry. The initial
  direct invocation of the sourced-only store fixture was invalid and retained;
  both actual parent fixtures subsequently passed their store checks.

Fresh release Wasm builds passed for the standalone and all six probe canisters.
Formatting, full offline tool admission, dependency declarations, 112-file
snapshot integrity, documentation links and runtime-free contracts checks passed.
The first direct PocketIC invocation lacked Make-owned fixture path exports and
failed before assertions; its log remains alongside the corrected Make invocation.
The corrected run passed both installation/interface cases and the stop/start plus
repeated-upgrade case, preserving installation state and all owner fences.

Initial compile failures for deleted global-registry observations remain in
`check.log`; the repaired source passed Clippy and the tests above. No failed
attempt was overwritten. Native macOS, full CI/release validation, optimized
Wasm, deployed provider and downstream application qualification are not claimed.
No version bump, staging, commit, push, release or sibling source edit occurred.
Delivery tracking: [Blob #64](https://github.com/dragginzgame/ic-blob-storage/issues/64)
and [Shared adoption #59](https://github.com/dragginzgame/ic-blob-storage/issues/59).
