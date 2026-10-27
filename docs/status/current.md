# Current status — 2026-10-27

Released baseline: `d02b9277`, Blob **0.50.0**, delivered on default `main`.
The pending adoption targets **0.51.0** because public Principal types now have
Platform 0.7 identities. Own package versions remain 0.50.0 for the maintainer's
release flow. Git delivery does not establish registry publication or service
acceptance. No assistant staging, commit, push, release or deployment occurred.

## Published supplier adoption

Adopted published Rust Platform/macros **0.7.0**, the consumed
Host/Build/Testkit **0.69.0** family and Shared Tooling **0.26.22** at reviewed
commit `5fc904a935d82ad31f0fba14434b71ccb8c10345`. The canonical exporter reviewed
impact and refreshed 129 files. Only five governance payloads changed; no shared
executable, formatter, release helper or tool catalog changed. New guidance
strengthens owned-Candid isolation, error derives and bounded native inspection;
prose adoption does not install new executable guards.

Official sparse-index metadata and cached archive SHA-256 values match for the
selected Platform and Tooling packages. Published source inspection confirms
Platform 0.7 removes three unused Candid compiler exports. Blob has no callers;
required parsing/type checking stays available. Principal, codec and runtime
sources are unchanged from 0.6.21, but Rust type identities differ across versions.
No compatibility bridge, sibling path or patched dependency is introduced.

Incoming Memory/Storage **0.51.9** lock selections are preserved and included in
this qualification; minimums now require that Platform-aligned release. Tooling
0.69 also requires Platform 0.7. Default/all-feature metadata resolve only
Platform **0.7.0**; the older 0.6.21 package is removed without patches or aliases.
The standalone normal Wasm graph retains one executor 2.0.0. Native Testkit
transport uses the common external Principal identity. Published Tooling 0.69
and Persistence 0.51.9 source bodies match 0.68.12/0.51.8 respectively; their
Platform requirements and package identities change, not byte contracts.

The browser registry still supplies `@tokostudios/ic-platform` **0.1.1**. Both
browser manifests/lock remain unchanged, with one SDK Core 6.1.1 and the reviewed
Caffeine 1.1.2 patch/override. Wire 0.7.18, Metrics 0.16.3, Rust 1.91, dependency
roles and existing feature selections remain unchanged.

## Current qualification

Inputs, before-lock, official registry records/archive checksums, source-body
comparisons, default/all-feature metadata, target graphs and focused logs:
`/tmp/blob-tooling069-20261027.aKq0I1wW/`. Manifest/lock/snapshot input hashes remain
unchanged across qualification. Testkit/Build setup and offline admission pass
for 0.69.0; PocketIC retains its 16.1.0 server selection. Substitute command-
forwarding and ordered-failure fixtures pass separately from the real tools.

Snapshot integrity, dependency pins, Platform declaration boundaries, actual
normal deployment exclusions and their refusal fixtures pass. Grouped manifest
ordering/release metadata checks pass against current package version 0.50.0.
Strict all-target/all-feature Clippy passes for the affected CLI and PocketIC
callers. A fresh release standalone Wasm builds; actual Rust 1.91 checks all
native CLI/PocketIC targets/features and standalone Wasm.

Current-graph tests pass: seven memory/lifecycle integration, 118 native CLI and
25 native host-support cases. Fresh installed cases pass: four lifecycle/recovery
and three certificate/authority cases, including two signed uploaders, quotas and
retained uncertainty. Provider replies are local substitutes; no live/paid effect
occurs. Earlier Platform adoption evidence remains under
`/tmp/blob-adoption-20261027.JXPj3tUX/`: Core/Contracts/Client/standalone Clippy,
68 model, 45 external provider-codec and eight Wire boundary cases, plus
generated TypeScript against 137 frozen frames. Source-body
comparisons support reuse of these unchanged contracts; those older graph runs
are not relabelled as new Tooling 0.69 executions.

Initial Tooling 0.69 setup timed out waiting for ic-backup's installation slot;
its subsequent offline admission correctly refused the unprepared selection.
Both logs are retained; setup/admission retries pass after the foreign owner
exited. No foreign lock or installation was removed. Native socket/PocketIC
cases use explicit process permissions.
No full CI, native macOS, Binaryen variants, current-graph Chromium journeys or
coordinated archive qualification was rerun. Browser SDK source/graph is unchanged;
earlier browser evidence retains its recorded scope, not a new Platform 0.7 run.

## Storage and consumer obligations

The released 0.50 cut uses bounded Wire for 17 stored encodings through IC Storage,
with 34 model types free of Candid traits. Installation format remains
`ic-blob-storage/installation:wire-records`. This adoption changes no stored bytes,
endpoint packets, decode ceilings, canonical identities or exact retry policy.
Cross-release adoption remains retirement/reinstall; no migration, dual reader,
automatic reset or stale-backup activation is supported. Retain provider objects,
uncertain effects, balances, access and billing under the
[retirement runbook](../retiring-installations.md).

The storage-cut evidence at `efb0de20` and released 0.50 handoff retain their
original full suites, old-Candid-store refusal and size measurements. Current
lifecycle checks do not relabel that older graph or repeat its size comparison.
The native Agent [advisory #102](https://github.com/dragginzgame/ic-blob-storage/issues/102)
remains unresolved; Platform 0.7 does not supply a crypto correction or exemption.

Canic's reviewed 2026-10-27 handoff selects Blob/contracts 0.49.3 and reports local
managed-browser qualification. Canic owns adopting the released storage cut and
then aligning the new public Platform identity in its composition; no sibling is
edited or qualified here. The [application acceptance plan](../acceptance-plan.md#next-application-acceptance-run)
already specifies remaining #31/#5/#34 evidence. The last #103 review records
browser adoption delivered on 0.50 with its matching hosted qualification pending;
this uncommitted 0.51 work has no hosted result and closes no acceptance issue.
