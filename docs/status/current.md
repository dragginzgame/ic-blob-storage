# Current status — 2026-10-26

Released baseline: `9777e07b` (0.49.0). Pending compatible notes target 0.49.1;
package versions remain 0.49.0. This batch changes no endpoint identity, Wire
bytes, persisted schema, saved filename, authentication or retry policy. The
[released handoff](https://github.com/dragginzgame/ic-blob-storage/blob/9777e07b/docs/status/current.md)
retains the preceding browser Platform adoption evidence; earlier implementation
records remain in [handoff history](history.md).

## Current changes

- `workflow::funding::prepare` authenticates an exact intent before calling the
  host's evidence callback without an installation borrow, then rechecks and
  reserves synchronously. Hosts can pair it with `ServiceWorkflow::<Host>::ACCESS`.
  Canic owns adopting this helper and removing its private journal-access adapter;
  its checkout has not been edited.
- Funding restoration, exact lookup and outcome inspection reuse the record's
  validated view within the same read. Validation at each read/mutation boundary
  and all credit, refund, uncertainty and restore-fence obligations remain.
- Installed operator scope has one projection; gateway and operator handlers
  retain independent authentication. Six provider decoders share private bounded
  Candid mechanics, preserving semantic checks, quota values and error ordering.
- Preparation recovery reads its existing JSON status through a typed projection.
  Malformed JSON remains `Binding`; valid but unprepared/invalid status remains
  `Unprepared`. Exact signed-attempt and input checks still precede this read.
  Setup accepts only its four operations. Permission/manifest document writers
  and readers use one byte ceiling per document; reply/transport caps stay separate.
- Fresh browser requests and retained gateway intents share pure target/header
  admission. Retained canonical URL/header checks, body snapshots, authenticated
  bindings, durable claims and no-resubmission rules remain independent.
- Reviewed native pipe state consolidation and retained the existing model:
  startup, an outstanding phase, stdin closure and process exit overlap. A single
  phase enum would require additional states rather than simplify behavior.
- Dependency guidance now describes current owners rather than old alias/import
  migrations. Historical schemas, independent heap models, provider substitutes
  and corruption/recovery tests remain necessary and are retained.

## Dependency and consumer boundaries

The [root catalog](../../Cargo.toml) now requires published Host/Build/Testkit
0.68.7; the incoming [lock](../../Cargo.lock) already selected that coordinated
release and is preserved. Rust Platform remains 0.6.18, Memory/Storage 0.51.3,
Wire/derive 0.7.16 and Metrics 0.16.2. Browser product/test/build imports use
published `@tokostudios/ic-platform` 0.1.0, with one Core 6.1.1 identity and the
existing exact Caffeine override. Service-owned protocols stay on Wire; required
external/provider and persisted Candid contracts retain their owners.

[Blob #103 — Adopt the published Platform browser SDK Core boundary](https://github.com/dragginzgame/ic-blob-storage/issues/103) tracks
browser Platform acceptance; its migration is present in the released baseline.
[Canic #637 — Browser Platform adoption](https://github.com/dragginzgame/canic/issues/637) owns Canic's browser
adoption. [Blob #102 — Native Agent advisory](https://github.com/dragginzgame/ic-blob-storage/issues/102)
retains the separate native Agent advisory. This cleanup makes no new provider,
security, production-capacity or complete Canic acceptance claim.

## Upstream package and Shared Tooling adoption

Shared Tooling 0.26.20 is exported from clean committed source
`a7f85d45d4212d4c39225f58597580560a333550`, with the opt-in npm SDK checker.
Blob delegates direct SDK declaration/import admission to that owner while
retaining its installed SDK singleton and Platform/Caffeine identity checks.
The 0.68.7 Build and Testkit releases were verified against crates.io's sparse
index; API queries returned HTTP 403. Unpublished Tooling 0.68.8 changes are not
consumed. Existing features and application contracts remain unchanged.
Normal releases start fresh from committed metadata; only explicit
`release-resume VERSION=X.Y.Z` recovers an older attempt. Local guidance and the
actual release-adapter recovery fixture follow that contract. Shared's compact
baseline makes detailed accounting opt-in and reserves broad local gates for
release or explicit full validation. Existing required CI/native lanes remain.

## Validation

The preceding cleanup used Tooling 0.68.6: 710 native cases, seven installed
preparation cases, four installed Chromium journeys and twelve pipe tests pass,
as do workspace lint, Rust 1.91 native/Wasm, formatting and documentation checks.
Logs remain at `/tmp/blob-cleanup0491-*`. The new wrong-scope fixture overflow was
corrected before rerunning; the initial sandboxed pipe attempt exited early.
These remain earlier-graph evidence, not qualification of changed Tooling.

The 0.68.7 adoption passes focused CLI all-target/all-feature and standalone-test
Clippy, 125 CLI/probe tests, seven installed publication-session/recovery cases,
and CLI all-target/all-feature checks on Rust 1.91. The existing unchanged service
Wasm passes actual endpoint-surface inspection with Host 0.68.7; no new service
artifact is claimed. Published Testkit and Build editor setup/admission pass.
PocketIC remains 16.1.0. All installed providers are local substitutes.

The 129-file shared snapshot, dependency declarations/pins, standalone production
exclusions, browser build, fifteen browser boundary results, grouped formatting,
ShellCheck and documentation links pass. The isolated release-adapter suite and
current 0.49.1 changelog check pass. Its obsolete dirty-source and staged-index
fault timing were aligned with early admission; prior failures remain retained.
Normal-versus-explicit-resume and frozen receipt/log preservation are covered.
The incoming lock and prior cleanup implementation are unchanged. Registry records,
source capture and logs remain at `/tmp/blob-adopt-tooling-20261026/`.

Native macOS, other engines, full CI and deployed-provider qualification were not
rerun. The GitHub description matches the prototype scope. No staging, commit,
push, release, publication, deployment or sibling edit occurred in this checkout.
