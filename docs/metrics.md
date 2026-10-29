# Standalone operation metrics

The standalone host records a fixed heap roster: Admission, Manifest, Registration,
Reference, GatewayAcknowledgment and GatewaySync. IC Metrics owns scalar arithmetic,
measurement aggregates, overflow quality and validated numeric transport. Blob owns attribution, platform reads,
reset windows and operator authorization. There are no dynamic labels or stable
allocations. The service core remains framework-independent; Canic owns its
separate adapter, reporting and optional persistence policy.

The report uses published IC Metrics **0.17.0** `MeasurementCounter` and
`MeasurementAggregate` directly. Their Serde readers enforce numeric invariants;
native Wire metadata describes the same fields. Blob owns only the report roster,
units, scope and codec budgets. Recording remains fixed heap state; no optional
recording feature or stable store is enabled. Native metadata support does not
bring TypeScript/schema tooling into the deployment graph.

## Operator report

`blob_metrics` is an operator-authenticated query. Its request is method-bound
Wire unit, with a 64-byte frame limit. Its result is
`Result<ic_blob_storage_contracts::standalone::MetricsReport, HostFailure>`;
the response has a 4096-byte frame limit, a fixed six-row array and bounded
allocation/nodes/depth (256 KiB admission budget, 2048 nodes, depth 12). Controller, tenant and uploader status grant no operator
access. The query neither records nor resets metrics, reads providers or changes
business accounting. Existing service methods, provider Candid and installation
bytes are unchanged. This optional host query is separate from the shared
`endpoints::SERVICE_METHODS` mount and browser method registry.

Use the installed operator identity and explicit local trust:

```sh
blob-storage metrics --network local --url http://127.0.0.1:4943 \
  --identity operator.pem --operator OPERATOR_PRINCIPAL \
  --service SERVICE_PRINCIPAL --namespace 1 --root-key root.der
```

For `--network ic`, select the intended production URL and omit `--root-key`.
The existing CLI verifies query signatures, selected operator identity and reply
service/namespace/roster. It performs one query, with no update fallback or retry.
JSON reports `observation: "standalone_heap_metrics"`, `schema: 1`, service,
namespace, lifecycle-start/observed versions and timestamps, and six
operation rows. Wide numeric fields are decimal strings; quality is explicit.

## Counters and instruction scope

- `attempts` and `request_bytes` record decoded update ingress reaching the
  workflow boundary. Bytes are complete encoded arguments, never uploaded body size.
- `applied` requires a fresh successful admission, preparation, certificate
  registration or reference result. An original failed reference receipt is a
  refusal; its exact replay counts only as `replayed`.
- `replayed` records exact admission, manifest and reference replays without
  another applied operation.
- `provider_failed` distinguishes returned gateway query non-submission,
  rejection and malformed/oversized replies from local typed `refused` outcomes.
  It does not classify financial uncertainty or authorize another payment.
- `acknowledged` counts successful sync/deletion callback returns, including
  repeated acknowledgments. It makes no claim about novel deletions or billing.
- `instructions` contains completed same-window call-context samples, total,
  optional maximum and independent sample/total overflow flags. Decode, encode,
  initial recording and aggregate updates are excluded; local workflow dispatch
  and outcome classification are included. No remote work, cycles or billing is
  inferred. Empty measurements have no maximum; a real zero is a sample.

All scalar totals use u128. Reaching a maximum is exact; actual overflow sets a
sticky saturation flag. No tenant, principal, object, root or error text is a label.
Current quotas, physical bytes, liabilities and recovery fences remain in their
authoritative service records and existing operator reports.

## Windows, queries and traps

Installation and ordinary same-release code restoration start empty heap windows.
Only lifecycle initialization resets the roster. A measurement token belongs to
that window; completion against another lifecycle window is discarded. Incomplete
asynchronous attempts can therefore have no matching completed sample.

`window_version` is the platform version captured at lifecycle initialization;
`observed_version` is the version at the query. Ordinary updates and callbacks
advance that version too, so a mismatch is expected and must not reset totals or
be interpreted as snapshot detection. See the [IC system API specification](https://docs.internetcomputer.org/references/ic-interface-spec/canister-interface/#canister-version).
Heap snapshots can roll back both totals and the window identity. Neither version
nor timestamp proves snapshot continuity: compare only when the operator knows
no restore occurred. These diagnostic metrics never clear restore fences or prove
lifetime continuity, and reporting never resets or relabels stored data.

IC rollback applies to telemetry too. A decoder or synchronous provider refusal
that traps cannot promise a retained attempt/refusal sample. Query-only liveness
and deletion-list callbacks are not recorded: query heap changes would be discarded.
These are committed local observations, not an independent invocation audit.
There is no external journal, timer, migration or public reset endpoint.
