# ic-blob-storage

An independent blob-storage service library for Internet Computer canisters.
The core verifies raw-content and Caffeine-tree identities, tracks checked chunks,
and lists missing chunks within explicit work limits. Ordered reads check each
leaf before final raw-digest verification. It also validates billing inputs and
decodes bounded Caffeine replies, including opaque audit pages for inspection.
Audit rows are not interpreted as proof of funding credit.

A transient multi-object catalog owns lifecycle, root claims and request receipts,
with tenant quotas, separate physical/billing accounting and
bounded deletion pages. Pure tenant/gateway policy protects local reads and
preserves revocation. A transient upload owner reserves shared tenant/global
capacity before exposure, retains exact operation history and keeps uncertain
uploads accounted. Confirmation transfers the reservation into the same catalog.
Authorized tenant pages list active uploads within explicit work/result limits;
gateway observations include pending roots without granting deletion permission.
Separate bounded tenant pages list unsettled confirmed objects, retaining deleted
and zero-byte objects until billing stops. Tenant indexing keeps other tenants'
objects out of scan budgets and cursor metadata.
Native tests cover rejection/replay; a test-only PocketIC
fixture exercises actual IC caller checks and gateway revocation. A controlled
source canister tests stale sync replies and reentrant membership changes.
The same local harness executes byte-verification vectors inside Wasm and checks
an explicit instruction budget for ordered chunk appends.
Upload fixtures also check real caller isolation, cancellation and retained uncertainty.
A connected local journey now covers certificate admission through deletion and
separate billing cessation, including actual inter-canister callbacks and rollback.
Its bounded files, explicit metadata and chunks are verified across messages
before certificate admission, with tenant-only progress and checked retries;
provider completion and billing remain supplied facts, without live uploads.
Local readback verifies chunks returned by a controlled canister and rechecks
tenant/reference/gateway authority after the await, including held stale replies.
Stop/start preserves these fixture journals; unsupported upgrades are rejected.
A trapped read callback retains its pending slot instead of silently retrying.
Both local fixtures restore their bounded ic-memory journals into permanent
inspection-only fences. The authority retains all three catalogs, receipts,
accounting, private verification checkpoints and pending operation identities.
An operator-only probe reconstructs a disposable verifier without resuming service
authority. Operational reconciliation and snapshot-load safety remain unimplemented.
Shared operator diagnosis composes billing and recovery blockers without effects.
The fixture's operator-only status query reports retained work and separate byte
charges; unknown balances/funding remain unknown, including after fenced restore.
Explicit local balance refreshes now bind service, namespace, source and account,
using the shared Caffeine reply decoder against independently encoded fixture bytes.
Status displays bounded history without making calls; stale, expired and restored
reports cannot become current balances. Reported cycles do not establish spendable
funds, payment credit or qualified provider behavior.
Diagnostic billing limits are validated and tied to that observation's exact
configuration revision. Shared policy reports balance shortfalls independently
of local funds; unknown spendability remains an explicit blocker, even when the
reported balance meets the minimum. No status query refreshes or funds an account.
Shared funding accounting separates callback refunds, proven enqueue failure and
unknown transfers. Its reconciliation policy never treats accepted cycles as
provider credit. PocketIC funding fixtures exercise actual transfers, enqueue
failure, callback rollback and same-release journal upgrades through ic-memory.
Their driver-only status query retains every transfer's credit/uncertainty diagnosis
without effects. Missing recovery authority and provider economics remain explicit;
funding restores now validate service/release bindings and journal consistency, then
permanently fence both sending and receiving. Old journals and late callbacks cannot
resume payment authority. This does not qualify whole-canister snapshot loads.

Production persisted workflows, provider transports, clients and canister
adapters are not implemented yet. Local bookkeeping and decoded provider reports
do not establish a qualified storage service.

The planned service owns tenant authorization, references, quotas, provider
access, billing, retention and deletion. This repository will own standalone
and Canic-managed adapters using the same handlers and blob API. The core
builds without Canic; Canic owns generic deployment and lifecycle.

Memory dependencies align through the re-exported `ic_memory` crate and its
`ic_stable_structures` substrate. The host owns bootstrap and allocation policy;
linking this library declares no stores or memory IDs. See
[memory composition](docs/dependencies.md#memory-composition-with-canic-and-icydb).

## Local development

Rust 1.98.1 and edition 2024 are pinned. Run `make deps` to fetch locked
dependencies; rustup installs the declared Wasm target, rustfmt and Clippy.
See [dependency setup](docs/dependencies.md) for PocketIC provisioning.

| Command | Check |
| --- | --- |
| `make check` | Compilation |
| `make fmt-check` | Formatting |
| `make clippy` | Strict workspace linting |
| `make test-native` | Native core tests and doctests |
| `make test-pocketic` | Build and run the local authority, sync and funding fixtures |
| `make test` | Both suites, sequentially |
| `make cloc` | Rust runtime/test file LOC and test function counts under `crates/` |

Validation uses offline Cargo and this repository's `target/`. These checks
make no provider calls or network deployments. PocketIC installs a test-only
local canister; production service journeys remain unimplemented.

The unpublished `blob-fixture-status` client attaches to an existing local PocketIC
instance. It requires a literal loopback address, instance ID, canister and
simulated caller; there are no inferred identities or targets. For example, with
these variables set from your running fixture harness:

```sh
cargo run --offline --locked -p ic-blob-storage-pocketic-tests --bin blob-fixture-status -- \
  status --server "$FIXTURE_SERVER" --instance "$FIXTURE_INSTANCE" \
  --canister "$FIXTURE_CANISTER" --caller "$FIXTURE_OPERATOR" \
  --kind authority --namespace 1
```

For the funding fixture use `--kind funding --peer "$FIXTURE_PEER"` instead of
the authority kind/namespace pair. `status` emits JSON and exits 0 on a valid read;
`check` emits the same report and exits 4 when reported blockers exist. Neither
assesses overall service qualification. Exit 2 means invalid arguments; exit 3
means transport, query, permission, decoding or binding failure. Unknown amounts
are `null`; full-width amounts and counters are decimal strings. The client calls
only the `operator_status` query, leaves the instance running and never falls back
to an update. It supplies no production authentication, discovery or provider access.

Release/publication preserve build output. Only explicit
`make clean` removes it.

Library publication to crates.io is enabled through the maintainer's
[release workflow](docs/releasing.md); it does not establish service readiness.
See [development governance](docs/governance/development.md) for command and
validation authority.

The optional LOC helper requires `cloc` and `jq`, and also works when invoked
outside the repository. Classification is by file path: inline test LOC remains
in `runtime_loc`, while `inline_fns` reports those test functions separately.
Canister fixtures and the harness outside `crates/` are excluded from this report.

## Documentation

- [Current status](docs/status/current.md): implemented scope, validation and next work.
- [Service contract](docs/service-contract.md): unresolved design decisions and implementation gates.
- [Canic parity](docs/canic-parity.md): required capabilities, source inventory and removal obligations.
- [Acceptance plan](docs/acceptance-plan.md): observable cases needed to qualify the service.
- [Provider review](docs/provider-review.md): pinned Caffeine findings and missing deployment evidence.
- [Core evidence](docs/evidence/core-primitives.md): implemented boundaries and source-bound test results.

Canic removal requires working replacements and separate installation
retirement evidence. No Canic code has been removed by this repository's work.

## License

MIT. See [LICENSE](LICENSE).
