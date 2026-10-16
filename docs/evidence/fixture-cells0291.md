# Probe cells, retained signers and offline dispatch

2026-10-16. Clean starting HEAD: `f0555981ed6dda5d19f4fbf2b679964276f2f9a6`.
Own versions remain 0.29.0; compatible pending notes remain 0.29.1. The committed
graph already selects direct Host 0.22.1, Build 0.9.0, public Memory 0.43.0,
Metrics 0.11.5, Testkit/Candid 0.48.0 and PocketIC 16.1.0. Only the three probe
manifests gain direct Memory dependencies with explicit test support; locked
external versions are unchanged.

Memory's published fixture cell replaces three copies of runtime/bootstrap/cell
ownership. Owner/grant/key strings, 16-page bucket configuration, Candid fields,
byte/work/type limits and missing-state refusal remain. Source review confirms
the same byte-cell layout; installed tests exercise current same-release restore,
pending obligations and fences. No migration, reset or compatibility reader is added.
The production standalone feature tree excludes Memory test support.

Download/observer commands retain the identity checked before their artifact
claim. Shared agent construction still reads trust/configuration after plan
persistence, before dispatch; no second identity-file read occurs. Offline
commands share their existing entrypoint match. Resource evidence uses Host's
explicit create-only/0o666 publication API, preserving umask and occupied-path refusal.

Linux passes: 119 CLI/five probe-tool cases, strict CLI/three-probe/storage-harness
lint, warning-denied Rustdoc and actual Rust 1.91 CLI/probe-Wasm checks. Fresh
artifacts pass six probe-restoration cases, one native download case, three
observer/submission cases and the exact-byte/occupied-resource regression.
Formatting, pins and documentation links pass. Before-images, raw logs and
input/source/artifact hashes remain in `.tmp/review-validation/fixture-cells0291/`.
The initial three sandbox socket refusals remain beside the passing loopback run.
Copied older Wasms retain their prior receipt's Host 0.21 scope, not this graph.

The unchanged optional archive verifier fails all-feature compilation: direct
Host process 0.22 types differ from Build 0.9's Host 0.21 types. The failure is
retained in `cli-all-features.log`; [#69 — Adopt Host custody and archive qualification](https://github.com/dragginzgame/ic-blob-storage/issues/69)
owns alignment. Full CI, archive/release qualification, native macOS and deployed
provider/Canic acceptance were not run. Installed results use local substitutes.
No sibling edits, index/ref changes, own-version changes, release or paid effects ran.
