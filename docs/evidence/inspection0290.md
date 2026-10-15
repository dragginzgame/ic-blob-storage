# Inspection transport, browser values and prepared confined reads

Date: 2026-10-15. Released Blob base:
`e65834ce6e61b38954c6a411edd7417cf0090e5a` (0.28.0).
This task extends the existing dirty 0.29.0 batch; it does not change package
versions. Before-images, source hashes and raw logs are retained locally under
`.tmp/review-validation/inspection0290/`. Historical receipts retain their
original graph and scope.

## Applied consumer changes

Account and download inspection updates share the native transport deadline,
SDK submission/polling and Timeout/Transport mapping. Callers still configure
the agent, bind their own request, decode the response and retain evidence in
the original order. Download reply/body ceilings and account scope checks stay
local. This helper does not grant retry authority or change signed dispatch.

Browser value primitives have one SDK-independent owner, imported by the
worker, configuration, intent, publication, launcher and session code. Positive
launcher bounds and zero-based session indexes remain explicit at each call.
Principal/namespace, gateway, journal and authority policy remain at their
existing boundaries. The session retains its exact enumerable-field count.
The formerly duplicated integer checks and field-shape mechanics are removed.

The source before-image is `before.tar.gz`; `docs-before.tar.gz` additionally
separates incoming dependency/handoff edits from this task's documentation.
Incoming Build 0.7.0, Testkit 0.47.0, the Host-alias removal and their lock changes
were preserved. The final ordinary graph selects registry Host 0.20.1, public
Memory 0.42.0 and private Metrics 0.11.1. `final-inputs.json` binds Rust/browser
sources and Cargo/Node selectors after that incoming graph settled.

Focused Linux evidence:

- `latest-cli-tests.log`: 119 CLI and five probe cases pass.
- `latest-clippy.log` and `latest-msrv.log`: strict CLI all-target/all-feature
  lint and actual Rust 1.91 checks pass, including the opt-in verifier.
- `latest-pocketic-account.log` and `latest-pocketic-download.log`: one installed
  account case and one certified-download case pass using the ordinary CLI.
- `browser-build.log`, `browser-bootstrap.log`, `browser-launcher.log` and
  `browser-store.log`: built browser assets, 17 bootstrap cases, 22 bridge/body
  cases and actual Chromium IndexedDB persistence/bounds cases pass. These use
  unchanged browser inputs across the incoming Cargo-only upgrade and make no
  provider requests. Store evidence covers graceful restart, not power loss or
  eviction; the million-row ceiling was not populated.

The selected Testkit CLI was initially missing (`latest-testkit-check.log`).
Explicit `make install-testkit` succeeded (`testkit-047-setup.log`), followed by
successful offline admission (`latest-testkit-check-rerun.log`). The earlier
sandbox Node/npm refusal and admitted rerun remain separately retained.

## Prepared Host extraction; not active in the ordinary graph

Reuse existing
[Host PR #114 — Root-relative opening](https://github.com/dragginzgame/ic-host-tooling/pull/114),
reviewed at `264a5839a0958428df714f003617d5176f1c4e81`. No duplicate producer PR
or sibling source edit was needed. Published Host 0.20.1 lacks its new read APIs.
The isolated clone path is recorded in `host-checkout`; its source is unchanged.

The refreshed consumer candidate uses one held root descriptor for inventory
metadata and body selection, plus bounded descendant reads for historical
preparation claims. Host owns component traversal, no-follow/type admission and
file opening; Blob retains byte limits, content checks, names, role bindings,
evidence order and failure categories. Bodies open through their selected root
rather than reopening a resolved display path. Historical journal inspection
remains read-only and cannot authorize redispatch.

`consumer-checkout` locates that candidate. Its private dependency patch changes
only the four Host source identities; all selected versions and other registry
selections match the ordinary graph (`candidate-final-lock-review.json`). Those
source overrides and the private lockfile must not enter normal Blob setup.
The source-only `adopt-confined-reader.patch` passes `git apply --check` against
the real checkout. It is not applied. After Host integration/publication, select
the published API-bearing version, apply the patch and requalify that exact graph.

- `host-latest-read.log`: 25 focused read cases pass. Strict lint,
  warning-denied Rustdoc and actual Rust 1.88 pass in `host-latest-*` logs.
- `candidate-final-cli-rerun.log`: 121 CLI, five probe and one package-verifier
  case pass; strict lint and actual Rust 1.91 pass in `candidate-final-*` logs.
- `candidate-final-pocketic-prepare.log`: seven installed preparation/recovery
  cases pass, including changed journals, partial claims and lost replies.
- The initial candidate sandbox refusal for three loopback HTTP cases remains
  in `candidate-final-cli.log`; the admitted rerun above passes all of them.

The seven reused Wasm hashes match the existing convergence artifact inventory
(`reused-wasm-check.json`); these are reused artifacts, not a fresh Wasm build.
Earlier candidate results against PR head `cfbe28b` remain separately retained.
Descriptor custody does not freeze file contents, exclude hard links or prove
current pathname containment after an ancestor rename. Blob's content checks
remain necessary. Native macOS, full CI, archive release qualification, deployed
provider behavior and Canic delivery are outside this focused Linux evidence.

[#69 — Adopt Host custody and IC Build archive qualification](https://github.com/dragginzgame/ic-blob-storage/issues/69)
retains consumer activation/acceptance. Its separate control-path prerequisite
also remains open; this task does not remove Host's guard or claim its migration.
No Blob Git/index, release, publication or paid-provider effect ran.
