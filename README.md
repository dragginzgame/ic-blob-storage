# ic-blob-storage

0.1.19 is the released foundation. The [0.2 delivery plan](docs/roadmap.md) tracks
the remaining work to a usable service; [current status](docs/status/current.md)
separates implemented behavior from outstanding milestones.

An independent blob-storage service library for Internet Computer canisters.
The core verifies raw-content and Caffeine-tree identities, tracks checked chunks,
and lists missing chunks within explicit work limits. Ordered reads check each
leaf before final raw-digest verification. It also validates billing inputs and
decodes bounded Caffeine replies, including opaque audit pages for inspection.
Audit rows are not interpreted as proof of funding credit.
The library also encodes explicit Cashier balance, payment-relationship and
gateway-list queries, and inspects relationship replies against the expected
storage owner and payer. Encoding/decoding performs no provider call and grants
no spending or account-link authority.
The 0.2 service admission model binds each project-approved upload to an exact
uploader and deadline, sharing existing reservations and reference accounting.
It checks root-only certificate requests against retained permissions and keeps
possibly exposed uploads charged after expiry, revocation or failed asset creation.
Operator-managed tenant enrollment gates fresh work; suspension preserves cleanup
and accounting, and reactivation cannot renew older uploader permissions.
The service uses bounded manifest authorization for direct browser-to-Caffeine
upload. No file chunks or whole-file raw digest are required by service admission.
Manifest consistency, possible exposure and independently confirmed provider
completion remain distinct. Global and per-tenant lifetime manifest-leaf budgets
survive cancellation and settlement, independently of released object bytes.
Service metadata requires a canonical `Content-Length` matching the reservation,
unique names ignoring case and values without controls or surrounding whitespace.
This validates the declaration; actual stored length still needs provider evidence.
The unpublished IC probe covers a 10 MiB declaration, exact retries, suspension,
expiry, retained uncertainty, stop/start and atomic rejection of unsupported upgrades.
Its measured admission/preparation/retry/exposure sequence is about 4M instructions
with no file-byte transfer. It does not persist grants or issue certificates;
provider size enforcement, replay and completion still require qualification.
Candidate configuration derives manifest limits from its admitted object size.
The local PocketIC journey covers 10 MiB files; its provider completion and billing
observations remain substitutes, not deployed Caffeine qualification.
Replies can be checked against the original encoded request, including its
Cashier, method and expected account bindings. Gateway application additionally
requires the exact registry scope and pending sync token. Explicit account-scoped
audit queries retain page limits, filters and opaque cursors; their replies do
not authenticate CSV contents or prove payment outcomes.

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
A separate local integrity journey covers certificate admission through deletion and
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
| `make test-pocketic` | Build and run the local admission, authority, sync and funding fixtures |
| `make test-admission-resources` | Admission input bounds and local Wasm resource report in `.tmp/admission-resources.json` |
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

The separate `blob-fixture-refresh` tool previews or requests one controlled-source
balance read. Take the current revision and next lifetime attempt number from the
fixture status, and explicitly supply the configured source and account:

```sh
cargo run --offline --locked -p ic-blob-storage-pocketic-tests --bin blob-fixture-refresh -- \
  dry-run --server "$FIXTURE_SERVER" --instance "$FIXTURE_INSTANCE" \
  --canister "$FIXTURE_CANISTER" --caller "$FIXTURE_OPERATOR" \
  --kind authority --namespace 1 --source "$FIXTURE_SOURCE" \
  --account "$FIXTURE_ACCOUNT" --revision "$FIXTURE_REVISION" --sequence "$FIXTURE_NEXT_ATTEMPT"
```

Replace `dry-run` with `refresh` to request the local read. Preview grants no future
admission: the update checks the same bindings, revision, next sequence, capacity,
pending work and restore fence before persisting intent. Repeating a consumed
request cannot dispatch again. Both commands leave the existing instance running.
Refresh may advance simulator rounds; it attaches no provider cycles and never funds.

JSON separates `action` from `post_status`. Exit 0 means eligible preview or completed
refresh, 2 invalid arguments, 5 a typed operation failure, 6 unknown/uncertain outcome,
and 7 completed refresh with failed post-status. A typed failed observation may have
consumed its attempt; inspect status. No outcome triggers an automatic retry or a new
sequence. Failed/malformed update acknowledgements remain uncertain even if the
subsequent status read succeeds. Dry-run only queries `preview_balance_refresh`;
the existing status/check tool remains query-only. These are local test transports,
not production credentials or evidence of deployed Caffeine behavior.

Gateway synchronization uses `--bin blob-fixture-sync -- dry-run` (or `sync`) with
the same explicit target flags, `--source`, `--revision` and `--sequence`, and no
`--account`. Read `sync_source`, `sync_revision` and `last_sync` from status; the
requested sequence is `last_sync + 1`. Sync revisions begin at 0. A null revision
means exhausted admission, not revision zero. Each revocation invalidates previous
previews, including when the member was already absent. Only a later explicit sync
may re-add it. Both action tools share the JSON/exit behavior described above.
The local reentrant fixture assigns its source canister the operator role; the CLI
must name that simulated caller explicitly. This is not a production role binding.

Funding admission has a passive preview:

```sh
cargo run --offline --locked -p ic-blob-storage-pocketic-tests --bin blob-fixture-funding-preview -- \
  dry-run --server "$FIXTURE_SERVER" --instance "$FIXTURE_INSTANCE" \
  --canister "$FIXTURE_SENDER" --caller "$FIXTURE_DRIVER" \
  --kind funding --peer "$FIXTURE_RECEIVER" --id 1 --amount 1000000 \
  --revision "$FIXTURE_BUDGET_REVISION"
```

Read `budget.revision` from funding status for `FIXTURE_BUDGET_REVISION` (initially
0). It changes on intent admission and completion, including full refunds. The
preview checks that revision, exact identity, protocol amount limit and full
amount without reserving or transferring cycles.
Exit 4 means a valid blocked preview; 2 means invalid arguments and 3 a read,
permission or binding failure. Exit 0 would mean unblocked observations, never
effect authority. The current fixture always reports missing funding prerequisites:
gross cycles are not authoritative spendability and transport acceptance is not
provider credit. Existing identities remain used even after full refunds. This
command cannot invoke the raw transfer experiment, override missing accounting
with flags or fall back to an update.

Use `blob-fixture-funding-lookup` to recover the retained result of an exact raw
experiment request after losing its ingress reply. For an original request with
ID 1, offered 1000000, accept 400000, reply InternalError and no callback trap:

```sh
cargo run --offline --locked -p ic-blob-storage-pocketic-tests --bin blob-fixture-funding-lookup -- \
  lookup --server "$FIXTURE_SERVER" --instance "$FIXTURE_INSTANCE" \
  --canister "$FIXTURE_SENDER" --caller "$FIXTURE_DRIVER" \
  --kind funding --peer "$FIXTURE_RECEIVER" --id 1 --amount 1000000 \
  --accept 400000 --reply InternalError --trap-callback false
```

Every original input is required. This command queries `lookup_funding` only.
Exit 0 means an exact retained transport observation; exit 4 means absent or
pending evidence. Exit 2 is invalid arguments and exit 3 a failed, denied or
conflicting lookup. None is a retry permit or provider credit. `Absent` means
missing from this journal, including an old restored backup; `Pending` also covers
callback rollback. Restored owners remain fenced even when an older journal omits
a paid operation. Lookups do not call the peer, reload stable memory, consume an
identity or change accounting. These are local PocketIC tools, not payment clients.

The funding fixture requires `(peer, driver, FundingBudgetInput { allocated,
reserve, operating_reserve, other_liabilities })` at installation. Its positive
`reserve` limits transfer attachments;
status and preview label this `budget.scope = "local_attachment_budget"`. Available
allocation excludes accepted cycles and full unresolved attachments. Callback
refunds and proven unsent offers are reported separately. Incoming receipts or
added gross cycles never increase it. Separately, the transfer guard uses current
platform liquid cycles and call costs, preserving positive `operating_reserve`
slack and explicit `other_liabilities`. These holds cannot be reset or released by
a refund. Costs are sampled after intent persistence and before dispatch. A
`LiquidityBlocked` outcome consumes the identity, releases the unsent attachment
and reports no callback refund.
Callback trap controls apply only to actual callbacks; they cannot erase an unsent
refusal. Such refusals still consume the bounded journal's lifetime capacity.
The fixture uses the library's `model::billing::allocation::FundingAllocation`
for amount reconstruction. It requires complete sequential history and rejects
original reserve violations even if the eventual reply returned the full offer.

Preview `liquidity` figures are observations, possibly cached, and can change
without a budget revision. The update rechecks its own exact encoded call. These
local installed holds do not prove complete production liabilities, provider credit
or recovery; top-level spendability remains unknown. Restored owners stay fenced.

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

- [0.2 delivery plan](docs/roadmap.md): milestones, current Toko consumer findings and completion criteria.
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
