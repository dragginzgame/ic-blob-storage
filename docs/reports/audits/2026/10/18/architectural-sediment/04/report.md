# Architectural sediment — current ownership follow-up

## Scope, identities and verdict

2026-10-18. Authorized repository-local inspection and cleanup at HEAD
`55d70cb0351093b5cc25fcc7b9c27a04e610a1f8`, released 0.33.0. The existing dirty
Platform/Shared adoption is preserved; task comparisons use complete starting
before-images, not HEAD's accumulated diff. Package versions stay 0.33.0 and the
pending compatible batch stays 0.33.1. No dependency version is selected here.

Method: `audits/flow-convergence-and-duplication.md`, with authorized
`audits/module-cleanup.md`, under Shared 0.24.0 revision
`cab12cd3dfa4fdb34846b25397f0d72881bb9ae0`. Local authorities are `AGENTS.md` and
`docs/governance/development.md`. Scope: the twelve member manifests, public
entrypoints, service/standalone dispatch, native preparation and artifact helpers,
probe storage/calls, protocol support, browser handoff, generators, build/CI/release
adapters and active versus historical guidance. This is source-led ownership
review, not exhaustive correctness or deployed-provider qualification.

Verdict: **PASS WITH FINDINGS**. Local remnants are corrected without changing
public APIs, wire bytes, durable schemas, lifecycle or recovery state machines.
No further production implementation removal is justified by this pass. Known
consumer adoption and deployment-admission gaps remain explicitly outside its proof.
Raw before-images, incoming diff, source identity, regression harness, logs and
final task delta are under `.tmp/review-validation/sediment0331-04/`.

## Verified owners and entry-to-result trace

| Capability | Canonical owner and consumers |
| --- | --- |
| Service records, tenant authority, quotas, references, funding and recovery | Core `model`, `policy`, `ops`, `workflow`; standalone calls `workflow::service::ServiceWorkflow<Host>` through explicit host access |
| Passive DTOs, method catalog, codec and verification contracts | Contracts `endpoints`, `wire`, `dto`, provider/content validators; host endpoint emitter, core clients, CLI and local protocol tests consume them |
| Authentication, endpoint registration, lifecycle and memory custody | Standalone `api`/`ops`; external frameworks own their own composition; linking core registers nothing |
| Allocation/backing and stable collections | Published Memory 0.48.2 and Storage 0.5.3; host-issued exclusive leases feed collection owners; no Blob substrate implementation |
| Platform and application codecs | Published Platform 0.1.5 supplies supported Candid/principal/native APIs; Wire 0.4 owns two tenant frames; Contracts retains bounded CBOR for the other 29 owned methods/lifecycle; external provider and durable contracts remain separate |
| Native mechanics and local IC execution | Host 0.27.2 supplies fs/process/artifact operations; Build 0.17.1 captures source evidence; Testkit 0.53 owns simulator helpers/lifetime; Blob retains command policy, failure interpretation and substitute scenarios |
| Common tooling | Reviewed Shared snapshot and `make/consumer.mk`; Blob supplies product policy, selectors, archive ordering and effect-free adapter qualification |

Published cached manifests/source establish these available APIs. Read-only sibling
inspection supplements that evidence: Canic HEAD `5583496bfa95069809249e8e5d16c77f1ffd744f`,
Memory `7594a26d09beb2e1524a0217aa357ae603aa7d0b`, Storage
`249da24992da229464b809d44ffed7f5ab8d4bae`, Platform
`d6edec2661f510297c8c9c77c5b9fa01741092e8`. Sibling working changes are not
published qualification. Canic's working catalog selects Blob 0.33 and Platform
0.1.5, but its `integrations/blob-service/src/workflow/mod.rs` still implements
its own operation dispatch. The earlier [host-dispatch receipt](../03/report.md)
tracks the prepared contribution; nothing in this audit edits or qualifies Canic.
Toko's framework endpoints do not establish a separate service implementation.

## Findings and completed cleanup

1. **LOW — extracted Storage owner missing from existing guards.**
   `scripts/ci/check-runtime-free-contracts.sh` originated before the Storage
   extraction and rejected Memory/substrate/core/CDK but permitted Storage in
   Contracts or CLI's normal graph. It now rejects Storage transitively too.
   The existing canonical direct-dependency policy now also forbids
   `ic-stable-structures` everywhere, including aliases and non-normal/disabled
   declarations. Memory/Storage retain their legitimate transitive substrate
   edges. No new checker, compatibility wrapper or competing policy engine is
   introduced. Both existing Make targets already participate in CI. Seven
   real-Cargo scenarios compare the original and repaired guards, including an
   indirect Storage edge, aliased normal/dev/build dependencies, disabled
   target-specific optional dependency and allowed dev-only Storage.
2. **LOW — abandoned installation instructions remained executable guidance.**
   The trial README referenced removed `configuration.args.template`, used the
   provider-only DID for removed installation types and selected `.candid` init
   files. History `aa0f133` and the actual `native/installation_check`/standalone
   decoder prove that current preparation is strict JSON followed by CBOR.
   The guide now delegates commands to the operator guide, retains the small
   proposal and artifact-binding requirements, and links original trial evidence
   as historical. Standalone and operator instructions no longer present old
   release-cut statements as future work; recovery explicitly requires CBOR unit.
   The core Rustdoc no longer claims the implemented standalone adapter is absent.
   No template values, historical packets, provider observations or effects change.
3. **LOW — active handoff doubled as a release-history owner.**
   `docs/status/current.md` accumulated 1,387 lines of batch-specific directions
   back through 0.23, including superseded dependency selections. Its complete
   starting contents are preserved verbatim beneath an archival warning in
   `docs/status/handoff-2026-10-18.md`, linked from the history index. Current status
   now names the actual source, pending graph, owner map, exact receipts and
   remaining obligations. No failed evidence or historical authorization is erased
   or promoted into current authority. This is archival consolidation, not deletion
   of retained evidence or a claim of implementation reduction.

The baseline Markdown checker already passed before cleanup: it checks supported
local link targets, not command validity or every template extension. The invalid
Candid instructions were a verified documentation defect despite that PASS.

## Intentional retention and unresolved boundaries

- The public Memory re-export preserves host type identity. Record codecs, provider
  admission budgets, exact effect journals and recovery fences are active contracts,
  not expired migration scaffolding. Private probe persistence retains its own
  bounded same-release envelopes; replacing it with a differently encoded fixture
  cell would require a private-format change without demonstrated overall benefit.
- `tests/pocketic/src/service_calls` translates method-specific Wire/CBOR for local
  tests; raw Testkit transport cannot replace that adapter alone. Browser packets
  are opaque and JS provider certification has its own trust boundary. Frozen
  refusal vectors and historical provider DIDs remain independent evidence.
- Native retained-file adapters preserve partial failed evidence; ordinary atomic
  publication has different semantics. Domain error mappings and authority checks
  remain Blob-owned. Removing these adapters would discard necessary behavior.
- Catalog-generated endpoints/provider DID, Build-owned source capture and immutable
  Shared snapshot scripts are canonical consumers/generated distribution, not local
  copies to rewrite. The twelve-member physical layout remains explicitly approved.
- Finish Canic's shared-dispatch adoption in its owning repository under separate
  scope. Keep allocation-level fault/read instrumentation until an upstream hook
  preserves attribution; generic backing observation does not prove equivalence.
- Direct Platform exceptions remain tracked in the dependency guide. This audit
  neither copies missing platform APIs nor weakens decoder/runtime contracts.
- Automatic standalone/transitive test-tool deployment exclusions and binding
  inspected inputs/features to final deployed bytes are still missing. The repaired
  declaration/runtime-free checks do not provide deployment admission. Recommend
  explicit enforcement by the consuming build/deployment owner, without introducing
  a deployment subsystem into this library repository.

## Verification

New local Linux results: seven real-Cargo guard scenarios; three native installation
validation cases; fresh default standalone release Wasm; two installed PocketIC
interface/template/actual-service-binding cases. The installed cases use the actual
CLI and fresh artifact, not a protocol substitute. Focused format, syntax/lint,
MSRV, direct/normal graph, documentation and snapshot results are retained in the
raw packet. Starting and final manifests/lock/snapshot hashes are unchanged;
archive content equality is checked against the before-image.

No runtime failure was encountered in the selected checks. Full CI, package archives,
all PocketIC cases, Chromium, native macOS, live provider and installed Canic are
not requalified. No sibling edits, Git actions, release, deployment or paid probe
ran. No performance gain or binary dependency absence is inferred from cleanup.

## Task metrics

Rust physical lines: +3 / -2 / +1.
Type declarations: created 0; deleted 0. Type physical lines: +0 / -0 / 0.
Documentation physical lines: +1682 / -1465 / +217.

Task-only comparison uses the retained starting dirty tree, including the existing
untracked boundary policy, and newly authored report/archive. Generated output,
snapshots, lockfiles and raw evidence are excluded. Rustdoc overlaps Rust/docs;
type lines are a subset of Rust. The large documentation churn includes relocating
the complete historical handoff; it is not functionality deleted.
