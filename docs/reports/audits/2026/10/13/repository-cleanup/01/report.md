# Repository flow and obsolete-surface cleanup

Date: 2026-10-13. Trigger: explicit repository audit and cleanup request.
Verdict: **PASS WITH FINDINGS** within the traced structural scope; the three
maintenance findings below are repaired, with qualification limits retained.
Methods: [flow convergence](../../../../../../../../audits/flow-convergence-and-duplication.md)
and [complexity review](../../../../../../../../audits/complexity-and-technical-debt.md),
implemented through [module cleanup](../../../../../../../../audits/module-cleanup.md).
Reviewed Shared snapshot: 0.9.10
`7e55adecf6f0b196c050f104b37f8c69b6bf434e`; local overlay: repository AGENTS.md
at released `e11780eb0ca13a4cfebe927d26a12b07c7772d3d`.

## Scope and comparison

Repository-wide structural discovery covers the twelve-member workspace,
production contracts/core/workflow and standalone boundaries, native CLI,
browser modules, test fixtures, common/local tooling and current guides. Detailed
entry-to-result tracing concentrates on installation/grants, publication, signed
updates, verified downloads, reference mutation, funding and recovery boundaries.
This is a structural review, not a complete security, paid-provider, performance
or deployed-service audit. Siblings and external packages remain read-only.

The maintainer's 0.22.15 release was active at audit start. Review and preparation
stayed outside its inputs in `/tmp/blob-repo-audit.2ayxxydt/`; no competing build
ran. Cleanup applied only after the owner exited and every affected source hash
still matched. Release `e11780eb0ca13a4cfebe927d26a12b07c7772d3d` is the clean
implementation baseline; pending compatible notes are 0.22.16. Starting/final
bytes and focused logs live outside Cargo clean at
`.tmp/review-validation/audit02216.4z50pw58/`.

## Owner and flow trace

| Behavior | Canonical owner | Consumers and result |
| --- | --- | --- |
| Frozen installation packet | CLI PreparedBatch, delegating exact_candid | Map, session and retained-host comparison project the same bounded candidate; installed reply authentication remains independent |
| Service memory assembly | ops/service/stores/grants | Host committed lookups and store fixtures use one key-to-field mapping; install/open still reject occupied/missing/corrupt stores |
| Signed mutation | CLI signed_update | Setup/reference/attestation/gateway commands persist exact claims before one dispatch; each operation owns authority and reply correlation |
| Whole-body verification | Contracts CaffeineRootVerifier, native local_body/provider_download | Frozen files and provider reads retain distinct transport/EOF/failure evidence; both use the canonical root invariant |
| Reference mutation | Contracts parser, stable upload owner and workflow/references | Endpoint and native clients preserve exact identity, receipt, liveness and accounting boundaries |
| Funding | Pure assessment, durable funding journal and dispatch/credit workflows | Preparation, payment, external credit and renewal remain distinct; passive views cannot authorize effects |
| Setup/release | Reviewed common Make/scripts plus local product adapters | Offline tool admission and consumer validation remain local; one maintainer release owner creates commits/tags |

## Findings and disposition

1. **MEDIUM — obsolete current instructions.** The dependency guide accumulated
   1,325 lines with many earlier graphs and an outdated inventory; release setup
   retained obsolete publication instructions. The current persisted-contract
   section still selected Memory 0.30, conflicting with the current public 0.35
   identity. Replace those current instructions, remove superseded roadmap prose
   and correct stale parent-coordination claims. Exact historical guides remain
   addressable in Git; original evidence, failures and obligations remain intact.
2. **LOW — duplicated frozen-input decoding.** Map/session and their fixtures
   repeated the same installation byte/type/work envelope. PreparedBatch now owns
   that exact projection. Host reply limits, failure mapping and full binding
   comparison stay separate. This narrows decoder-policy edits to one owner; it
   adds no cache, state mode, public API or performance claim.
3. **LOW — duplicate test assembly.** Store fixtures repeated the production
   sixteen-memory field mapping. They now use the existing request/grant owner.
   Per-slot occupied/missing/corrupt-memory and restored-fence assertions remain;
   the cleanup changes fixture construction, not their behavioral obligations.

Rehearsal: changing the frozen candidate envelope now touches its batch projection
and exact decoder rather than three production callers. Changing grant field
assembly has one production owner rather than a second test-only mapping.
Dependency updates now change the current inventory and source-bound status,
without extending a chain of superseded setup instructions. These are source
maintenance observations, not measured runtime improvements.

## Intentional retention and limits

- Request, reply and provider decoders have different skip/work/byte ceilings,
  typed failures and authority roles. Superficial similarity does not permit one
  permissive decoder. Independent host/installed reply validation remains.
- Heap/stable, native/IC and browser-worker boundaries need their own rejection,
  rollback, restart and uncertainty proofs. Configuration/lifecycle and grant
  tests are not obsolete merely because they share fixture literals.
- Probe records, failed fixtures, archived handoffs, published changelogs and
  frozen provider inputs are evidence, not dead implementations. They are retained
  at their owners. No live obligation or recovery history is deleted.
- Browser exports all have maintained consumers. No removed-contract alias or
  legacy schema reader was found in the inspected maintained flows. No downstream
  wrapper, external journal, parser framework or new state mode is introduced.
- Provider/application acceptance remains in the existing GitHub issues linked
  from current status; this audit does not close them or repeat their backlog.

## Verification

All 113 native CLI cases and the 12 focused store/grant cases pass. Scoped
all-target/all-feature Clippy for CLI/core passes with warnings denied. Existing
tests retain request limits, scope correlation, no overwrite, lost-reply history,
transport failure and fenced/corrupt store behavior; no new implementation-mirroring
test is introduced. The first sandboxed CLI attempt passes 110 cases and fails
three when loopback bind is denied. Its complete log remains separate from the
successful explicitly permitted local-HTTP run; no fixture is weakened or skipped.

Formatting, offline tool admission, dependency declarations, snapshot integrity,
runtime-free contracts/CLI graphs, documentation links and guide anchors pass. Final source,
protected input hashes and result logs are in the raw packet. Cargo manifests,
lockfile, snapshot/tool/compiler pins, finalized receipt and real index are
preserved. The maintainer's preceding full release gate qualifies 0.22.15 only.
Full validation, native macOS, deployed-provider and application qualification
are not inferred for this new cleanup. The full symbol inventory belongs only
in the final maintainer handoff, per the reviewed change-reporting rule.

## Task metrics

Compare clean released 0.22.15 with the final task-owned paths, using
`git diff --no-renames --numstat e11780eb0ca13a4cfebe927d26a12b07c7772d3d`
and the added report's physical lines. Distributed Shared copies, lockfiles,
build artifacts and untouched historical evidence are excluded. Rustdoc overlaps
Rust/documentation; type lines are a subset of Rust, so categories are not summed.
Rust physical lines: +28/-85/-57.
Type declarations: +0/-0; moved 0, renamed 0. Type physical lines: +0/-0/0.
Documentation physical lines, including changed Rustdoc: +368/-1628/-1260.
