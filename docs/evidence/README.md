# Evidence ownership and retention

This directory keeps compact conclusions and evidence with a continuing purpose.
The [Shared retention rule](../../rules/engineering-details.md#changelogs-and-artifact-preservation)
owns experimental budgets; the [current handoff](../status/current.md) owns the
active batch. Evidence is not a second API guide or an adoption checklist.

## What remains

| Material | Purpose and owner |
| --- | --- |
| [Caffeine probe ledger](caffeine-probes/README.md) and its payloads | Intent, failures, source reviews, deployed observations and outstanding provider obligations; retain under its recording contract. |
| `caffeine-*.json`, `caffeine-*.did`, `caffeine-*.sha256` and [provider review](provider-review-2026-09-28.md) | Reviewed external provider contracts and browser-source provenance used by that investigation; historical observations do not establish current deployment guarantees. |
| [Implementation conclusions](core-primitives.md) | Compact ownership/recovery conclusions and precise historical chapter references. |
| [Shared adoption boundary](shared-tooling-adoption.md) | Current policy/caller ownership and historical adoption references; the snapshot remains the authoritative file inventory. |
| [Tenant codec comparison](wire-bench0320.md) and [borrowed Wire comparison](borrowed-wire0390.md) | One compact summary per distinct experiment, with selected absolute results, tool/source identities and limitations. Neither qualifies current endpoint performance. |
| [0.14.10 preflight provenance](release-preflight-01410.json) | Small source/artifact identity record required by the maintained [pre-cut regression fixture](../../tests/fixtures/standalone-pre-cut/README.md); keep until that consumer is retired. |

Protected provider and recovery records are not experimental clutter. This
cleanup does not authorize clearing fences, dropping liabilities, resending
uncertain operations, resetting installations or deleting provider objects.
Release receipts and maintained frozen fixtures retain their separate owners.

## Retired run archive — 2026-11-05

The former flat directory accumulated per-version dependency/adoption reports,
routine check transcripts, source inventories, test-output hash lists and
superseded synthetic measurements. They duplicate current documentation and
owning history rather than maintain another implementation contract.

Those obsolete reports are removed from the current tree. Historical citations
now address their exact pre-cleanup Git objects at
[`e23dfec0e06a23ae834c2fd27a9b845818f45c2a`](https://github.com/dragginzgame/ic-blob-storage/tree/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence),
including failed attempts and the retired benchmark machine tables. Git history
is not rewritten, and no replacement archive or duplicate payload directory is
created. The two append-only diaries are compacted into the summaries above.

Useful conclusions remain explicit:

- Runtime/storage suppliers and downstream frameworks own their capabilities;
  Blob keeps its policy and necessary integration. The [service contract](../service-contract.md),
  [dependencies](../dependencies.md) and [retirement contract](../retiring-installations.md)
  describe current behavior. Old verifier, read-session and Memory-era reports
  are not supported architecture or cross-release migration instructions.
- Generic tooling regressions belong to Shared; Blob verifies its actual
  callers and product constraints. Old successful test counts and compiler
  results do not qualify a different graph or prove reusable release admission.
- Synthetic benchmarks and provider substitutes do not establish live provider
  credit, deletion, billing cessation, production capacity or Canic readiness.
  The provider ledger preserves the stronger distinctions and unresolved facts.

Historical `.tmp`/`target` paths and hashes are records of past observations,
not promises that raw artifacts remain available. Removed raw output is no
longer retained in the working tree; summaries alone cannot reproduce or
independently verify those measurements. The Git-linked historical text is
retained, but its referenced untracked artifacts have no new custody guarantee.

## New work

Routine validation uses the existing handoff, changelog or owning issue/CI
result, without another tracked report. New raw experimental output belongs
under `target/evidence/`. A useful study may keep one summary, normally no more
than 150 lines, plus at most two small data files: at most 100 KiB per topic and
5 MiB across the repository. These are ceilings, not targets; do not commit bulk
samples, source snapshots, binaries or compressed bundles merely to fill them.

Durable raw-evidence exceptions must identify the claim, minimal payload, size,
owner, destination and retention/review period. Active provider/recovery records
follow their existing contract; this experimental policy cannot erase them.
