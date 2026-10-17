# Dependency caller repair for pending 0.31.0

Date: 2026-10-16. Base commit `831874938b2ba92653d828221f61b0496def6b53`
(subject 0.31.0, own metadata still 0.30.0), plus the preserved incoming dirty
batch documented in [the preceding receipt](decoder-commands0310.md).

The reported `20261016T174836Z-136201-11-check.log` predates the working-tree
Memory constructor/exclusion and direct Host HTTP repairs. Those removed imports
are absent now. This task finishes the remaining installed Candid and optional
archive-verifier callers without changing the selected external graph.

## Applied repair

All 393 typed installed query/update calls supply the explicit test-owned
`support::REPLY_LIMITS`: 4 MiB reply, 20,000,000 decoding work, 1,000,000 skipping
work, 512 type entries and 64 KiB header. The shared finite fixture ceiling covers
status/history pages and byte-bearing probe replies. It is not a production
provider/operator default or a transport/execution deadline. Exact callers,
arguments, method modes, binding checks and lifecycle sequencing remain local.
There is no former-signature trait or compatibility wrapper.

Build 0.11.1's archive API exposes Host process 0.23.1 types while ordinary
native fixtures use Host 0.24.0. The private optional verifier names Build's
existing selected family explicitly through `ic-build-host-process`; its former
unused normal Host 0.24 dependency is removed. Native HTTP tests still select
Host 0.24 test-support directly. No external version/source/checksum selection
changes; Cargo.lock gains only the private CLI edge to existing Host 0.23.1.
Review this coupling on future Build upgrades. Output ceilings, retained evidence
and exact exit/signal mapping are unchanged.

## Focused qualification

Raw logs, command manifests, task-start before-images and artifact hashes are in
`.tmp/review-validation/ci-api0310/`.

- `cargo check --offline --locked` for core, CLI and PocketIC packages with
  `--all-targets --all-features`: pass, including optional verifier and installed
  test compilation.
- The same package/target/feature selection with strict Clippy: pass. Initial
  lint attempts are retained; explicit call arguments lengthened four existing
  lifecycle scenarios beyond the line threshold. Local documented expectations
  retain those coherent scenarios, without changing their assertions.
- The same selection on actual Rust 1.91.0: pass.
- Fresh release Wasm builds for authority, funding, gateway-source and storage
  probes: pass against Memory 0.45.0. Artifact hashes are in `artifacts.json`.
- Installed PocketIC: 13 funding lookup/preview cases, one restoration-read case,
  one authority archive/recovery case and one content-vector case pass (16 total).
  The content test binary also passes its existing native buffered-output case.
  Coverage includes lost replies, exact identity conflicts, uncertain callback
  effects, stale-backup refusal, restored fences and byte-bearing content probes.
- Fresh native package-verifier build and the real Cargo archive fixture pass:
  same-version replacement with warm prior artifacts, plus all ten graph/drift/
  command refusal cases. Original statuses 23 and 143 are preserved. The fixture
  path and complete partial attempts remain in `package-regressions.log`.

The explicit PocketIC 16.1.0 archive SHA-256 is
`131219d90dcf9bf6f3ed8ee02d8f55504ea24a6db382f5680675b7bd6b7ed3bf`;
the decompressed payload equals the selected executable with SHA-256
`b44e1eccd66e02328146b3e209e3403e3a3428a25d25555aca2f67500e5ae9db`.
These match the selected published Testkit provisioning contract, and version
output is `pocket-ic-server 16.1.0`. No setup/download ran. Caller admission does
not establish the separately installed lock-selected Testkit CLI/toolset.

GitHub readback finds no workflow runs for this exact base SHA; the dirty source
has no hosted result. Earlier failure logs remain intact. Previously prepared
GitHub reports were rejected by automatic approval review and remain unposted;
these local repairs supersede their failure descriptions. No retry or workaround
publication ran. Existing #69 still owns delivery and required archive/native
acceptance; these focused substitute archives are not actual Blob archive proof.

Full CI, the complete installed suite, actual Blob archives, native macOS,
optimized Wasms, Canic and deployed-provider acceptance were not run. The prior
public Memory/storage and decoder cuts still require pending 0.31.0; no package
version, Git index/ref, release, sibling or paid-provider effects ran.

## Build 0.12 dependency name repair

On 2026-10-17, clean base `0b79b11b5d57113ab66e054f3a5037e8cce2eb1c`
already selects Build 0.12.0, direct Host 0.24.0 and private Metrics 0.12.0.
The reported `20261017T061429Z-2957393-11-check.log` shows Cargo refusing Host
process 0.24.0 under both `ic-build-host-process` and `ic-host-process` names.
The earlier two-family alias is obsolete because Build now exposes Host 0.24.

Remove that alias from both manifests, import the verifier's types from
`ic_host_process`, and activate the same optional normal dependency through
`package-verification`. Native development tests retain that dependency's
`test-support` feature. Output budgets and original status mapping are unchanged.
Cargo.lock is byte-identical; external selections and own versions are untouched.

CLI `--all-targets --all-features` check, strict Clippy and actual Rust 1.91.0
check pass, followed by a fresh verifier build. The real Cargo archive fixture
passes warm same-version replacement and all ten graph/drift/command refusal
cases, including original exit 23 and signal 143. Its retained root is
`target/package-fixture.7dk3Or`; logs, commands and before-images are under
`.tmp/review-validation/host-name0310/`. Formatting, dependency declarations,
documentation links and diff checks pass. No new PocketIC, full CI, actual Blob
archives, native macOS, version, Git/release or sibling effects ran. Earlier
qualification above remains bound to its original Build 0.11 graph.
