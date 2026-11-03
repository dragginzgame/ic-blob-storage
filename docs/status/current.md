# Current status — 2026-11-03

## Delivery preparation requested

Prepare Contracts, Client and Engine **0.69.0** in the normal checkout on `main`,
in supplier-first order, for Git release and crates.io publication. Local source
commits, prerequisites and full release validation are authorized. Shared 0.27.29
requires separate explicit approval before direct delivery to `origin/main`;
obtain it after preparation. No alternate checkout, sibling delivery, service
reset or live canister deployment is selected. Preserve all release evidence.

## Pending 0.69.0 adoption — Contracts, Client and Engine

The Wire publication unblocks the requested family. Selected published graph:
Wire/Derive **0.16.0**, Platform/Runtime/macros/exact guard/PocketIC **0.19.0**,
Types **0.18.0**, Tools **0.79.0**, Testkit **0.81.0**. Build **0.74.1**, Host
Foundation **0.74.2**, Metrics **0.27.2**, Storage **0.65.5**, Pest **2.9.5** and
Syn **3.0.9** retain their prior selections. Canonical index/archive checksum
and exact Wire tag/VCS readbacks are retained in `target/wire016-adoption/`.
Shared Tooling stays **0.27.29**, committed `18e727032237e4a6540fefce66bb1b90beb31644`
(150 verified files). All thirteen package versions remain **0.68.1**; only the
three draft changelog headings select **0.69.0** for changed public Rust identities.
Dependency adoption itself performed no Git delivery or publication. Delivery
preparation now proceeds under the request above, preserving the selected graph.

[Wire #44](https://github.com/dragginzgame/ic-wire/issues/44) tracks the now-published
supplying schema integration; the former native-startup/Principal-schema blockers
are gone. New typed PocketIC operations preserve ClientError and uncertainty.
Installed refusal assertions explicitly require Canister errors rather than
misclassifying protocol/transport failures. No uncertain update gets an automatic
retry. Generated schemas and all 137 frozen shared-service Rust frames are exact.
Blob's application contracts do not embed the changed owned Platform Account;
Principal bytes, method IDs, bounds, trust and exact retry packets are unchanged.

[Blob #113](https://github.com/dragginzgame/ic-blob-storage/issues/113) is implemented
locally: standalone recording stays on canonical Metrics; report-time projection
uses passive CounterMetrics/AggregateMetrics DTOs. Numeric decoding delegates to
Metrics. Exact prior report bytes, quality flags and JSON fields remain covered.
Contracts no longer forwards Metrics/native-wire, and a native schema graph guard
plus negative fixtures enforce that ownership. CLI uses Metrics directly only in
tests. Do not close #113 until these changes are present on the default branch.
[Observability #128](https://github.com/dragginzgame/ic-observability/issues/128)
owns removal of the supplier's metadata surface; Blob no longer relies on it.

One deliberate residual: Storage 0.65.5 privately uses Wire **0.15.1**, so both
codec releases occur in the installed/deployed graph. No Wire types cross that
owner boundary. Types/Runtime/guard are single identities. Storage owns its codec
upgrade; do not invent a source patch or claim the old codec is merely unselected.

## Preserved accepted simplification

Engine funding records now keep transport progress and matching diagnostics in
one bounded state; credit attribution and renewal accounting remain separate.
Fresh `BLFUPR01` and installation format
`ic-blob-storage/installation:singleton-cells:funding-progress` refuse old backing.
Retire obligations with original tools before fresh whole backing; no migration,
fallback reader or automatic reset. Same-release recovery, refund conservation,
credit receipts and exact operation identity remain required.

The six private operator CLIs/parser/report application are retired; typed
installed scenarios preserve their meaningful authority, funding, sync and
uncertainty coverage. Shared fixture mechanics keep controllers/cycles/init bytes
explicit. Tenant tests use typed codecs. The supported CLI uses typed Clap command
and flag groups without string dispatch or ArgMatches handlers. Its syntax,
limits, offline documents and authority checks remain. Core guidance names Storage.
All these pending changes and incoming selections were preserved.

## Current qualification and remaining delivery checks

Linux/Rust **1.99.1** evidence: `target/wire016-adoption/`. Contracts tests/examples/
doctests and frozen vectors pass; Core **300**, standalone **2**, CLI **122**,
Client **10**, native call-error **2** unit tests pass. The affected PocketIC
package passes **353** tests with **10** existing opt-in ignores. All six probes
and standalone build for Wasm; CLI is rebuilt. Generated TypeScript passes **27
methods / 137 frozen frames**; metadata compares byte-for-byte with the prior
output. Direct boundaries, native schema/deployment exclusions and their refusal
fixtures pass. Prior graph checks remain historical in `target/complexity-cleanup/`.
Strict affected-package Clippy, formatting, shell checks, documentation links and
compiler admission all pass. The final metrics decoder tests also reject impossible
Wire numeric states through canonical Metrics validation. No full release gate,
native macOS, Chromium/live-provider or new optimizer matrix is claimed.

Explicit `make install-testkit` and offline `make testkit-check` now pass for the
lock-selected **0.81.0** CLI; the canonical store reused authenticated source/build
receipts. Default PocketIC **16.1.0** is admitted. Installed tests selected the
independently authenticated retained server with the identical SHA256
`b44e1eccd66e02328146b3e209e3403e3a3428a25d25555aca2f67500e5ae9db`,
from the retained release checkout named in
`target/qualification-cleanup/pocketic-binary.txt`. The newly admitted default
server has exactly those same bytes; no redundant installed rerun was needed.
The browser check used Node **24.21.0** and npm **12.2.1**, explicitly prepared
under `.tools/browser-npm`; browser dependencies/lock were not changed. Select
`BLOB_BROWSER_NPM=$PWD/.tools/browser-npm/node_modules/.bin/npm` when using that setup.
Deferred work belongs in owning issues; source/PR work stays in durable checkouts.
No temporary proposal replaces an issue or holds unique implementation.

## Published 0.68.1 — retained historical evidence

Contracts, Client and Engine 0.68.1 are indexed/non-yanked on crates.io; archive
checksums, VCS commits and package paths match their independent release tags.

| Group | Tag | Release commit |
| --- | --- | --- |
| Contracts | `contracts-v0.68.1` | `dc506009f0bf4d5edeadb01b2308297f6088551e` |
| Client | `client-v0.68.1` | `7449b6ac9e06f1b663d16e7b774ea7ab030928d5` |
| Engine | `engine-v0.68.1` | `2ff86ef40f3e4aeca17772c877a26c8762825505` |

Frozen delivery graph: Platform 0.18.0 / Runtime and guard 0.18.1 / Types 0.17.0;
Wire/Derive 0.15.1; Storage 0.65.3; Metrics 0.27.2; Testkit 0.80.2 / PocketIC
16.1.0; Build 0.74.1 / Host Foundation 0.74.2 / Host Tools 0.78.1; Shared Tooling
0.27.28 (`8075b3fd4803d560a1be04768f76701ada3ae8fe`, 150 files). Rust 1.99.1,
Node 24.21.0 / npm 12.2.1. Caffeine SDK override, application trust and retry
policy remain local. Later Storage 0.65.4 and current 0.65.5 are separate work.

Each first-plan release ran its full roster: 688 native tests, 380 PocketIC cases /
10 documented ignores, strict compiler/boundary/hook/metadata/recovery checks,
formatting, Rustdoc, Wasm and mixed-version archives. Generated TypeScript passed
27 methods / 137 frozen Rust frames. Exact Engine-tag Wasms/CLI passed five
lifecycle/fencing/obligation cases and the signed installation-carrier case.
This is historical local installed-IC evidence, not native macOS/live acceptance.
Evidence: `.tmp/blob0681.cgzub5/` (source/lock, group logs, publication archive/VCS
readbacks, artifacts, `release-checkout.current`); retained release plans under
Git release-state and `.tmp/blob-release-validation.*` in that frozen checkout.
Prior 0.68.0 evidence: `.tmp/upstream-platform018.ZS89rK/`. Do not clean them.

## Validation selection and service boundary

Blob owns `ci/release-validation.json` and `scripts/release/validation.pl`;
Shared admits/checkpoints/executes the complete plan. First deliveries established
`release-verify-plan-v1` baselines. Fully classified changes may omit unchanged
publication/recovery fixtures; only tenant-client-only changes may omit Engine
IC scenarios. CLI, shared, graph, compiler, recipe and unknown changes retain
wider checks. Missing/failed/source-changed proof cannot pass. Routine releases
retain original-artifact installation/recovery; hosted Linux/manual checks keep
the optimizer matrix. See [release validation scope](../releasing.md#validation-scope).

Library publication does not deploy or qualify a live service. Resolve provider
objects, uncertain effects, balances and billing custody under the
[retirement contract](../retiring-installations.md) before fresh installation.
Same-release recovery remains required. Canic owns its adapter, production limits
and rollout. Blob #5/#31/#6/#34 and Canic #622/#494 retain application/provider
acceptance. No live provider call, paid effect, service reset or sibling edit was
performed in this batch. This cleanup does not close those acceptance issues.
