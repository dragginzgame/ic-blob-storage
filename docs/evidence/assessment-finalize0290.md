# Funding assessment, signed completion and fixture parsing

Date: 2026-10-16. Starting clean Blob source:
`7366c79d5ef0c6bf39983f80fb666a5589770356`; own package versions remain 0.28.0,
with the existing numbered 0.29.0 draft extended. Before-images, logs, source and
artifact hashes remain in `.tmp/review-validation/assessment-finalize0290/`.
Cargo inputs are unchanged. The starting graph already selects direct Host
0.21.0, Testkit 0.47.0, public Memory 0.42.0 and private Metrics 0.11.3.

## Ownership and behavior

Contracts' funding reply owner now checks bounded assessment encoding, exact
request echo, journal arithmetic and diagnostic blocker consistency. The CLI
retains its 4 KiB bound, 100,000 decoding work, zero skipping, 128 types and a 4 KiB
header limit, transport authentication, JSON presentation and refusal codes.
Local status and assessments reuse the same passive budget arithmetic. These
observations grant no preparation, dispatch, retry, credit or liquidity authority.
The caller still admits the proposed request and authenticates the service.

Signed-update completion owns optional-reply decoding and persists the operation's
outcome before returning its result. Raw replies remain saved before decoding;
claim/intent/permission writes still precede the sole dispatch. The operation
still selects its decoder, authenticated-refusal predicate, labels and report
fields. Missing replies remain pending, and report-write failure takes precedence
without overwriting existing evidence. No retry or polling is introduced.

Inventory selection delegates argument-pair parsing to the existing fixture owner.
Duplicate/missing arguments still refuse. Its command, loopback server, nonzero
port, namespace and caller/tenant checks remain local and unchanged.
No wire DTO, durable layout, identity, lifecycle, fence or retirement rule changes.

## Focused checks and remaining limitation

- `contracts-tests.log`: 110 Contracts cases pass, including the moved protocol
  corruption/blocker matrix. `cli-tests.log`: 119 CLI and five probe cases pass,
  covering boundary error mapping and completion evidence/write-failure behavior.
- `operator-tests.log`: all 23 fixture-operator cases pass, including inventory
  command admission and duplicate/missing argument refusals.
- `clippy-rerun.log`, `rustdoc.log`, `msrv-native.log` and `msrv-wasm.log`: affected
  default-feature lint, warning-denied Rustdoc and actual Rust 1.91 native and
  Contracts-Wasm checks pass. The initial test callback type error is retained in
  `clippy.log`; the corrected run passes. `check.log` is the separate failure below.
- `installed-standalone.log`: 13 selected installed funding-assessment,
  gateway/reference submission and upload/preparation cases pass, including lost
  and pending replies, role mismatches, occupied claims and restore fences.
- `installed-attestation.log`: the signed-attestation recovery case passes,
  retaining lost replies, conflicts, settled history and restored-state refusals.
- `standalone-wasm.log` and `storage-wasm.log`: fresh release Wasms are built
  separately, preserving the production/probe feature boundary. Other selected
  fixture Wasms are unchanged reused bytes; their identities are retained in
  `reused-wasm.json` and `final-wasm.json`. The final CLI is rebuilt before the
  installed cases. No browser or deployed-provider acceptance is inferred.

The first all-target/all-feature check fails in unchanged archive-verifier code:
Host 0.21 `OutputLimits`/`ToolError` do not match IC Build 0.7's Host 0.20 types.
Manifests, lock and verifier source are unchanged from the starting commit. That
pre-existing optional-feature gap is tracked in
[#69 — Adopt Host custody and IC Build archive qualification](https://github.com/dragginzgame/ic-blob-storage/issues/69#issuecomment-6210216854).
Default-feature passes do not qualify it. No dependency workaround or upgrade
was applied as part of these three cleanups.

Full CI, archive/release qualification, native macOS and Canic/deployed-provider
delivery remain separate. No sibling source, Git/index, release, registry
publication, deployment or paid effect ran.
