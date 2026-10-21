# Current status

Date: 2026-10-21

## Source and release

Starting source is released 0.39.0 at `8430be0`; workspace package versions
remain 0.39.0. Pending notes target **0.40.0** because public Platform type
identity moves from 0.5 to 0.6. Hosts must align that family and rebuild.
Endpoint identities, application Wire frames, decoder budgets and persisted
schemas are unchanged. No staging, commit, release or registry effects were
performed in this adoption. Preserve the earlier unrelated pending cleanup.

## Published dependencies and ownership

The maintainer confirmed the graph settled. Locked full metadata admits all
thirteen members: Platform/macros **0.6.1**; Host artifacts/fs/process/tools,
Build and Testkit **0.63.0**; Memory/Storage **0.50.4**; Metrics **0.15.1**;
Wire/derive **0.5.7**. Published sources/checksums were inspected; no sibling
path override, legacy alias or compatibility package is selected.

Platform owns the IC identity/runtime/Candid boundary; Wire owns application
framing/schema; Blob owns tenant policy, storage workflows and recovery.
Host/Build/Testkit belong to [IC Tooling](https://github.com/dragginzgame/ic-tooling),
Memory/Storage to [IC Persistence](https://github.com/dragginzgame/ic-persistence).
Storage allocation and record formats were not migrated here.

Published Tooling 0.63 resolves the earlier Platform 0.6 / Testkit 0.62.1
DecodeLimits mismatch in [Tooling #223](https://github.com/dragginzgame/ic-tooling/issues/223).
Testkit has retired its Candid helpers and Metrics dependency. Current private
fixtures use their existing support owner with Platform codecs, unchanged finite
budgets and one guarded raw query/update attempt. Frozen historical projections
remain separate at their existing owner. No application codec moved to Candid.
The Testkit/server and release-editor tool selections now explicitly enable
`cli`; this does not enable CLI features in application libraries.

## Shared Tooling and earlier cleanup

Shared Tooling **0.26.1**, revision `67d4f25d1a613a27d7dfd7b5a4ae119e0e529fc7`,
was exported from an isolated clean copy of the committed producer source.
The **125-file** snapshot verifies. Its selected changes add focused dependency
adoption guidance and fix multi-tool macOS awk argument construction. No dirty
producer files were imported. Keep local graph/feature selections and existing
checks; the guidance does not require another orchestration layer.

Earlier pending cleanup remains: only `wasm-opt` is selected for IC setup/admission;
the full pin catalog and prior tool generations are retained. Production Wasm
builds separately from probes to exclude their `test-support` feature. Five
unused generic fixture roots and their jobserver companion are retired through
the canonical exporter. Shared retains generic matrices; Blob keeps actual
consumer routing, admission, evidence, hooks and release recovery coverage.
Version requirements stay authoritative in Cargo instead of repeated guide tables.

The signed-update, publication preparation and observation paths already share
owners. Keep exact-intent/uncertainty journals, durable obligations and the
independent heap parity oracle; they protect distinct safety/test boundaries.

## Qualification and limitations

Evidence: `.tmp/review-validation/shared0261-platform/`, including before-images,
registry records/checksums, exporter preview, full metadata, input checkpoints,
artifact hashes and raw logs. After the settled locked fetch, unchanged manifest,
lock, snapshot and Make inputs were checked around every runtime stage.

Passes on the settled graph: dependency declarations/pins and deployment
exclusions; formatting; workspace all-target/all-feature check and strict Clippy;
**680 native cases**; actual Rust **1.91** native/Wasm checks; fresh six-probe and
separate default production release Wasms; rebuilt CLI. **31 installed PocketIC
cases** plus the shared pipe-output regression pass: gateway malformed/oversized
replies, rollback and restored fences, lifecycle and account scope/errors, billing
obligations and the pinned 0.14.9 image's upgrade refusal preserving old bytes.
Current helper migration is exercised without changing frozen schemas.
Explicit CLI setup and offline tool admission, consumer Shared fixtures,
release-adapter recovery and hook checks pass on Linux.

Normal/build deployment, all-feature native CLI and full workspace observations
select one Platform 0.6.1 and one Wire 0.5.7. No retired Platform family or legacy
CDK/macros appears. Candid and Platform's required CDK executor remain transitive;
these are not Candid-free claims. Deployment excludes native Host, Agent, Testkit,
Build, PocketIC and parser owners. Declaration checks cover aliases, kinds and
targets; graph observations do not prove arbitrary final binary contents.

Earlier setup attempts without the new CLI feature and a runtime attempt with
an uncached concurrently selected Platform patch remain retained failures.
They were followed by explicit setup, a settled locked fetch and fresh runtime
qualification; earlier mixed-graph passes are not reused as current evidence.
The initial 0.6/0.62.1 reproduction remains under
`.tmp/review-validation/platform0391/`. Prior 0.5-family optimized-Wasm evidence
under `.tmp/review-validation/upstream0391/` qualifies that earlier graph only.

Full CI/archive and optimized-Wasm qualification for this new graph, native
macOS, installed Canic and live-provider qualification remain unrun.
[Shared #294](https://github.com/dragginzgame/shared-tooling/issues/294) remains open:
its [exact-source producer run](https://github.com/dragginzgame/shared-tooling/actions/runs/39559474466)
passes Linux but fails an unexported fixture assertion on macOS. Do not equate
local Linux admission or the committed awk repair with native macOS acceptance.
Producer Tooling qualification also remains with its owner in #223.

The earlier isolated [PR #94](https://github.com/dragginzgame/ic-blob-storage/pull/94)
contains only the compatible Shared 0.26.0/cleanup scope; its hosted tooling and
example checks pass. It does not deliver this working tree's 0.40 dependency cut.
Evidence remains at `.tmp/review-validation/backlog0391/`; no root index/ref changed.

Canic's committed local 0.112 source adopts Blob 0.39/Platform 0.5; its remote
main and old PR lag that work. This pending 0.40/Platform 0.6 cut needs consumer
alignment and dedicated/embedded acceptance at
[Canic #444](https://github.com/dragginzgame/canic/issues/444). No sibling edits ran.
Local substitutes do not qualify provider cooperation or authorize paid probes;
maintain [the probe ledger](../evidence/caffeine-probes/README.md).

## Earlier evidence

The [2026-10-20 handoff](handoff-2026-10-20.md),
[2026-10-18 handoff](handoff-2026-10-18.md) and [history](history.md) preserve
source-bound earlier observations. Their pending versions and blockers are
historical evidence, not current instructions.
