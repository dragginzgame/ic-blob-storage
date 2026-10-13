<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Development plan

The [current status](status/current.md) owns the exact released baseline and
active implementation work. The maintained product supplies a
framework-independent library and standalone service, configured uploads, exact
verifier completion and native/browser publication components. Consumer owners
supply framework wrappers, deployment integration and their composition tests.
Library publication is separate from qualified live service operation.

## At a glance

- **Available now:** shared policy and durable state, standalone endpoints,
  bounded native/browser workflows and retained live trial evidence.
- **Next acceptance boundary:** a real consumer must integrate the current
  contract and qualify complete publication, serving and recovery journeys.
- **Still open:** provider deletion and billing cessation, production sizing,
  older-backup recovery and accountable retirement of retained installations.

<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-integration-topology.svg" alt="Integration topology showing browser and native clients, embedded and standalone hosting, the shared library, ic-memory, Caffeine and PocketIC" width="800">
</p>

## Current ownership

The framework-independent service owns tenant policy, permissions/manifests,
references, receipts, quotas, economics and durable local journals. Standalone
restores those owners synchronously into fences and delegates endpoints to the
same handlers. Native tooling retains exact signed intents and verified snapshots;
browser transfer composes the pinned Caffeine SDK with certificate and gateway
journals. Provider authority and effect recovery stay with those owners.

| Boundary | Available implementation | Remaining acceptance owner |
| --- | --- | --- |
| Provider contract | Caffeine request/reply definitions and retained source/local/live probes | Provider limits, namespace/payer authority, physical deletion and billing evidence |
| Durable service | Bounded stores, exact replay/history, logical/physical/liability accounting and fenced recovery | Actual consumer operating bounds and independent continuity evidence |
| Standalone | Maintained endpoints, native/browser publisher, independent completion verifier and local IC tests | Selected released artifacts, application media and deployed provider journeys |
| Framework integration | Core handlers, explicit host grants and passive installation DTOs | Canic/application wrappers, Root/parent initialization and coherent family delivery |
| Retirement | Exact receipts, obligation inventory and supported refusal/recovery workflows | Accountable per-installation disposition, settlement and billing cessation |

## Next action

Continue from [current status](status/current.md) for released/prepared identities
and links to the owning GitHub issues. Framework mounting and composition tests
stay with Canic; application authentication and the media-map transaction stay
with the application. Provider completion, payer attribution, GC and billing
cannot be closed by another local wrapper, substitute receipt or dependency bump.

Use [dependency and memory composition](dependencies.md#canic-integration),
[the operator guide](operator-guide.md), [verifier operation](completion-verifier.md)
and [the retirement runbook](retiring-installations.md) for current procedures.
[Service gaps](service-gaps.md) and [the acceptance plan](acceptance-plan.md)
distinguish local evidence from production qualification. The
[Caffeine probe ledger](evidence/caffeine-probes/README.md) retains every probe's
original source, intent, requests, results and outstanding obligations.

## Consumer constraints

- Tenant/uploader/service/provider namespace bindings must be explicit. A root,
  project label or controller role is not tenant authority.
- Upload and application asset registration are separate transactions. A failed
  registration cannot authorize another uncertain upload/payment. Consumers own
  exact references and their release outbox.
- Per-file, batch/concurrency, physical-byte and continuing-liability limits are
  independent. Configure them from the consumed inventory and measured host
  bounds; a UI staging cap or local fixture envelope is not a provider limit.
- Sequential publication observations are not a retention lease. Serving, CSP,
  cached/public URLs and independently verified full reads require their own
  application/provider evidence.
- Passive funding observations cannot authorize paying or repeating a top-up.
  Provider credit/payer facts and original effect identity must be established
  under [the funding contract](funding-consumer-qualification.md).
- Cross-release changes require retirement/reinstall. Same-release interruption
  recovery preserves exact histories, references, balances and billing obligations;
  a counter from a stale backup cannot establish safe activation.

Earlier milestones, consumer measurements and version-specific reviews remain in
[implementation history](status/history.md), their original evidence records and
[the frozen former plan](https://github.com/dragginzgame/ic-blob-storage/blob/0d720d1745c1c9e87fe2a9cb922ba21c1dec5293/docs/roadmap.md).
They do not supply current setup or a second issue queue.
