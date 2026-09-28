# ic-blob-storage

0.2.2 is the released library baseline. The [0.2 delivery plan](docs/roadmap.md) tracks
the remaining work to a usable service; [current status](docs/status/current.md)
separates implemented behavior from outstanding milestones.

An independent blob-storage service library for Internet Computer canisters.
The core verifies raw-content and Caffeine-tree identities, tracks checked chunks,
and lists missing chunks within explicit work limits. Ordered reads check each
leaf before final raw-digest verification. It also validates billing inputs and
decodes bounded Caffeine replies, including opaque audit pages for inspection.
Audit rows are not interpreted as proof of funding credit.
Whole streams can also be checked against a fixed trusted provider root, length
and original metadata using `CaffeineRootVerifier`, without a supplied raw digest
or leaf manifest. A prefix alone is not verified content.
For upload clients, `CaffeineManifestBuilder` computes ordered leaves, the root
and raw digest in one bounded streaming pass. It reuses the same hash engine and
retains only the declared leaf list, without buffering the file or a full tree.
Tenant-only `content_descriptor` views recover that original metadata alongside
the exact operation and current lifecycle. They do not reserve references or
provide certified browser delivery.
`retained_content_descriptor` additionally requires confirmed completion and the
consumer's exact live reference in the same read. It remains a current observation;
the consumer must coordinate publication with reference release.
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
`ops::service::tenant::StableTenantEnrollments` begins the durable layer with
individual bounded enrollment records in a host-granted `ic-memory` stable map.
It shares the heap model's transitions, preserves generations and suspension, and
reopens into an enforced mutation fence. The host owns memory grants, installation
identity and lifecycle; linking the library installs nothing. `StableRootClaims`
adds immutable root/object claims with a checked reverse index.
`ops::service::uploads::StableUploads` owns both stores together with exact upload
permissions, immutable manifests, confirmed lifecycles, individual reference
receipts and maintained logical/physical/liability totals. Heap and stable owners
share lifecycle rules. A local PocketIC probe checks partial-write rollback and
same-release upgrade into an inspection-only fence. Completion, deletion and
settlement require independently authenticated facts from the integrating host;
the probe uses labelled substitutes. Real provider integration, durable provider
call journals, deployed read transport and operational recovery remain unfinished.
The durable owner also supports indexed tenant discovery, original metadata and
exact-live-reference descriptors. Tenant and operator traversal bound scanned rows
and results separately; empty filtered pages retain continuation. These are current
observations, including while fenced, not provider capabilities or completed cleanup
proofs. A new sweep is required for changes behind a saved cursor.
Admission/reference headroom comes from maintained counters, sharing the heap
model's quota and reserved-cleanup arithmetic. Bounded operator root observations
preserve pending and retired states for reconciliation. They grant no provider
callback or deletion authority and do not release the restore fence.
`ops::service::funding::StableFundingJournal` adds durable local attachment intents
and exact transport outcomes. It reserves the full offer, keeps uncertain amounts
charged and rejects repeated attempt markers. Reopen validates history and totals
before enforcing inspection only. It binds canonical Cashier top-up arguments,
including explicit account and optional target balance, and checks supplied
transport context before applying outcomes. Bounded operator history recovers
original requests even while fenced. The separate, explicitly invoked
`ops::caffeine::funding::transport` sends one canonical request and captures its
exact IC refund before bounded reply decoding. PocketIC connects it to this journal
through a local Cashier substitute. `record_observation` commits the structured
reply with transport accounting; `outcome` reads it with validated transfer facts
for the workflow's reconciliation policy, including while fenced. `summary` reads
maintained totals and history capacity without scanning intents; older accepted
amounts remain unresolved after newer refunds. Its scope is the local journal.
Shared `workflow::funding::inspect_preparation` reports local constraints alongside
independent host evidence. `prepare_new` re-reads current state before reserving
the exact offer synchronously; a saved preview cannot authorize that mutation.
Missing evidence and local obligations independently block preparation. Integrating
hosts must still establish current, complete account activity and spendability,
qualify the provider and reconcile credit. Preparation sends no provider call;
production dispatch requires its own current checks. Shared
`workflow::funding::attempt::mark_first_attempt` checks the exact existing
reservation with fresh host observations before marking it uncertain. It does not
charge that reservation twice, allow repeated attempts or clear older acceptance.
`workflow::funding::dispatch::dispatch` composes that check with post-write platform
liquidity, one explicit Cashier call and durable callback settlement. Missing host
holds block before mutation; liquidity refusal records a positively unsent call.
The host supplies synchronous journal access and current qualified observations.
PocketIC exercises this composition with synthetic host facts and a local Cashier
substitute; deployed-provider qualification and production evidence acquisition
remain open.
`ops::service::gateways::StableGatewayRegistry` retains bounded membership and
pending sync identity together in one host-granted memory. Operator edits invalidate
older syncs; restoration preserves pending history under an inspection-only fence.
`workflow::gateways` binds the canonical Cashier query to this durable attempt,
then validates bounded encoded replies before committing membership and completion
together. Failed replies retain the pending attempt; transport authentication and
buffering limits remain host responsibilities.
The async query handler checks pending state before host transport and revalidates
after the await. Local IC tests cover delayed replies, concurrent edits and callback
rollback. `ReplicatedGatewayQuery` explicitly selects IC replicated execution for
the canonical query-only method, with service/Cashier checks, bounded wait and no
attachment. This route passes local IC tests; deployed Cashier behaviour and
installation/provider qualification remain open.
This is local registry state, not proof of provider authority or callback freshness.
`workflow::gateways::callbacks::observe_roots` composes current membership with
the durable upload owner under matching configuration and explicit scope. It
returns bounded local phases without tenant/request details, checks known object
bindings and rejects either restored owner. Local IC tests cover caller isolation,
revocation and cleanup phases. Provider liveness/deletion mapping and read-session
generations are separate checks; these observations grant no effect authority.
`workflow::reads` captures and rechecks the original tenant, exact live reference,
object/root and selected gateway against both durable owners. Gateway edits and
successful syncs advance a persisted invalidation counter; tenant reactivation
also invalidates earlier observations. Held-reply IC tests cover revocation,
reference release and rollback. `StableReadSessions` now persists bounded global
and per-tenant slots and reply-buffer reservations through three host-granted
memories. Shared session handlers check chunk range, commit intent before transport
and consume only the exact callback's reservation. Revocation retains capacity
until that callback returns; restoration preserves occupancy under a fence.
`workflow::reads::chunk::read_chunk` composes one host call with these handlers,
checks response source/root/index and verifies exact length and the admitted leaf
hash before returning bytes. It loads one bounded manifest record without rebuilding
the tree. The local IC fixture exercises real chunks, bounded bulk decoding,
malformed replies and delayed callback rejection. Production transport, resource
sizing and operational recovery remain open; the fixture is not Caffeine's wire.
`workflow::reads::download::describe` supports direct client delivery without a
canister body call. It binds an explicit Caffeine owner/project/namespace mapping
to the active tenant's exact live reference, root, length and original metadata.
Suspension and restoration refuse operational delivery; passive inspection stays
available. The relative HTTP target follows the reviewed client format. Hosts
still own provisioned mappings, approved origins, authenticated delivery and
consumer verification/release coordination; no production URL is selected here.
`dto::download` and `workflow::reads::download::handle` now own the descriptor
boundary. `ReplicatedDownloadClient` sends its canonical update once from the
actual tenant canister and validates a bounded, fully bound response. The probe
exports that shared boundary and tests a real consumer-canister call. Linking
the library exports nothing. This is authenticated IC metadata delivery, not a
public certified release mapping, browser adapter or publication lease.
`dto::reference` and `workflow::references::receipt` supply exact historical
receipt recovery through `blob_reference_receipt`. `ReplicatedReferenceClient`
authenticates the service through one IC call and bounds decoding before returning
explicit absence or the original success/failure. Upload/object/lifetime/reference
identities remain independent. Suspension, settlement and restore do not erase
inspection evidence; no result authorizes publication or an uncertain retry.
`workflow::references::apply` and `ReplicatedReferenceClient::apply` expose the
matching `blob_apply_reference` update. Both paths share `ReferenceCommand` and
typed failures; admitted lifecycle failures remain inside the returned receipt.
Persist the exact intent before polling the client future and preserve it through
uncertain replies. Exact mutation retries return history without reviving released
references; restored owners permit inspection only. The client never retries itself.
Consumer asset transactions and release outboxes remain application-owned work.
`workflow::uploads::inspect` and `ReplicatedUploadStatusClient` expose exact tenant
upload history through `blob_upload_status`. Confirmation records creation of the
first reference; it stays historical after release and is not a publication lease.
`workflow::uploads::admission` and `ReplicatedUploadAdmissionClient` provide
`blob_admit_upload` and exact `blob_upload_admission` recovery. Both bind the
uploader and original expiry; replay cannot renew the permission or charge capacity
again. Save the full permission before dispatch and inspect uncertain outcomes.
The separate `blob-consumer-probe` exercises both that first reference and explicit
retains of existing content through real IC calls: bounded durable intent, atomic
publication/tombstones, dependency checks, exact recovery and fenced upgrade.
Fresh registration creates no extra retain receipt. The fixture now sends admission
from its own canister and recovers interrupted acknowledgments without resending.
Its two-slot application substitute is integration evidence; manifest preparation,
exposure, revocation and provider completion are still arranged by the test host.
Production Toko transactions and operational recovery remain open. The storage
library owns no consumer database.
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
Admission maintains reservation/manifest totals and an object-identity index
without rescanning pending or cancelled history. The resource probe also fills
256 lifetime operation slots across two tenants; its measurements are in the
[history-cost evidence](docs/evidence/core-primitives.md#retained-admission-history-after-020).
Confirmed-object totals also use maintained global/tenant usage, including retained
receipts for lifecycle failures. Root-only exposure indexes the original permission
before rechecking the uploader and deadline. Production capacity still needs sizing.
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

Durable bookkeeping and shared funding/gateway transport primitives have local
test evidence. Complete production workflows, clients, canister adapters and
deployed-provider qualification remain open. Local bookkeeping and decoded
provider reports do not establish a qualified storage service.

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
| `make test-read-resources` | Single-slot readback bounds and local Wasm costs in `.tmp/read-resources.json` |
| `make test` | Both suites, sequentially |
| `make cloc` | Rust runtime/test file LOC and test function counts under `crates/` |

Validation uses offline Cargo and this repository's `target/`. These checks
make no provider calls or network deployments. PocketIC installs a test-only
local canister; production service journeys remain unimplemented.

Local headless preparation computes the declaration needed for upload admission:

```sh
cargo run --offline --locked -p ic-blob-storage --example prepare_upload -- \
  declaration.json 10485760 10 < body.bin > prepared.json
```

`declaration.json` supplies `bytes` and original `headers` (`name`/`value` entries),
including canonical `Content-Length` equal to `bytes`. The two numeric arguments
bound content bytes and retained leaves. The example checks the shared service
metadata rules with local limits of 16 headers / 4 KiB framed metadata, reads a
16 KiB maximum declaration and uses a 64 KiB body buffer. Deployment limits still
need independent selection. Only a successful invocation yields a complete JSON
result: `claim`, `chunk_hashes` and `computed_content_digest`. The `claim` object
has the input shape used by `verify_download` below.

This is offline preparation, with no enrollment, certificate, provider upload or
completion claim. Preserve the exact source bytes and metadata for later upload;
the single-file mode does not copy the source or persist resumable operation intent. The
computed raw digest is diagnostic and is not required by service admission.

For multiple files, use `prepare_upload --inventory inventory.json`. Source paths
resolve relative to the inventory file. For example:

```json
{
  "limits": {
    "files": 100,
    "file_bytes": 10485760,
    "file_chunks": 10,
    "total_bytes": 104857600,
    "total_chunks": 100
  },
  "files": [{
    "asset": "example",
    "source": "body.bin",
    "declaration": {
      "bytes": 3,
      "headers": [{"name": "Content-Length", "value": "3"}]
    }
  }]
}
```

The inventory is capped at 1 MiB of JSON. All limits are required positive maxima;
aggregate budgets include duplicate sources. The tool reads files sequentially
and emits `totals`, separate `assets` mappings and one prepared entry per distinct
root in `blobs`, only after all succeed. Asset IDs must be unique; source paths
must be relative, without parent traversal or symlinks, and name regular files.
Use a controlled, unchanged source tree: these checks do not provide a filesystem
sandbox or freeze a snapshot. Distinct-root totals do not establish service
capacity, cross-tenant sharing, fresh-upload eligibility or provider charges.
Existing references, lifetime history and billing obligations require service
inspection; no actual service/tenant/namespace or operation identity is selected.

The shared model's `UploadAdmissions::admission_capacity` supplies the next
planning input: tenant-scoped lifetime object, concurrent upload, manifest-leaf
and byte headroom, plus enrollment and per-object limits. It includes reservations
and continuing billing; freed logical quota alone cannot make those obligations
disappear. These independent counts reserve nothing. Existing blobs still require
content discovery and reference-capacity checks. The unpublished
`blob-fixture-inventory` command connects a prepared report to these queries on an
already-running local admission probe:

```sh
cargo run --offline --locked -p ic-blob-storage-pocketic-tests \
  --bin blob-fixture-inventory -- inspect \
  --server 127.0.0.1:PORT --instance INSTANCE --canister SERVICE \
  --caller TENANT --tenant TENANT --namespace NAMESPACE \
  --inventory prepared-inventory.json
```

Use the output of `prepare_upload --inventory`, or a snapshot's `inventory.json`.
The client requires explicit local targets and a simulated caller equal to the
tenant. It bounds input to 8 MiB, 4,096 assets, 2,048 distinct blobs and 8,192 total
distinct leaves, with at most 16 headers / 4 KiB framed metadata per object. It
checks manifest/root consistency, mappings and recomputed totals before any query;
it never opens source/body paths. There is one outstanding query at a time, at most
one capacity query and two queries per distinct root, with bounded reply decoding.

The JSON report distinguishes content not visible to this tenant, unfinished
operations to recover, live blobs requiring new references, and retired roots.
It budgets one fresh reference per asset for existing live blobs. New-object
reference capacity remains unassessed. Not-visible content is not proof of global
absence or upload eligibility. Aggregate new-byte/object/leaf demand and concurrent
upload headroom are separate: a sequential batch need not fit every upload at once.
Observations are sequential and can become stale; no root, quota or reference is
reserved, and file bytes are not reverified. Exit 0 means a complete observation
with no observed blocker, 4 reports blockers, 2 rejects arguments, and 3 reports
input/query/reply failures without partial results. Production authentication,
provider transport and operation persistence remain outstanding.

For the transient admission fixture, `blob-fixture-reference` can preserve an
exact local intent and inspect its historical receipt without applying it. This
tool uses that fixture's wire, not the durable service's canonical receipt query:

```sh
mkdir -m 700 reference-journal
cargo run --offline --locked -p ic-blob-storage-pocketic-tests \
  --bin blob-fixture-reference -- save --intent request.json --journal reference-journal
# Use the returned "saved" path as INTENT.json below.
cargo run --offline --locked -p ic-blob-storage-pocketic-tests \
  --bin blob-fixture-reference -- inspect --intent INTENT.json \
  --server 127.0.0.1:PORT --instance INSTANCE --canister SERVICE --caller TENANT
```

The strict JSON file (at most 16 KiB) has this shape; supply actual principals,
root and identities rather than the placeholders:

```json
{
  "schema": 1, "scope": "pocketic_fixture", "asset": "image-a",
  "service": "SERVICE", "tenant": "TENANT", "namespace": "1",
  "upload": "1", "object": "1", "incarnation": "1",
  "root": "sha256:ROOT_HEX", "bytes": 3,
  "reference": "2", "operation": "1", "retain": true
}
```

IDs are positive canonical decimal strings, preserving their full width. The
probe fixes object ID to upload ID and incarnation to one; the file records both
explicitly. `retain: false` names a release. The tool does not allocate IDs or
prove freshness. Saving requires an existing, durably created, caller-controlled
local directory. A filename derived from scope/object/lifetime/operation binds one
exact intent. Changing the root, reference, action, size or asset label conflicts;
an exact retry returns the same record, even after its success output was lost.
The tool never overwrites existing records or follows record/lock symlinks.

The writer holds an OS file lock, syncs its private file (0600 on Unix), installs
it without replacing a destination, then syncs the journal directory before
acknowledgment. Concurrent writers fail with `journal_busy`. Never delete or
replace `.writer.lock`, including after a crash: the OS releases the held lock
when the process exits. The journal permits at most 4,096 entries besides the lock;
interrupted staging files count too. Full journals still permit exact recovery.
There is no automatic eviction or residue cleanup.

This path is tested on Linux local storage. Unsupported locking/directory syncing
fails; a storage error can leave a complete record without acknowledgment, so
preserve the directory and retry the exact intent. Files remain mutable and a
copied or rolled-back journal has no freshness or restored-writer authority.
Power-loss behavior depends on the filesystem honoring sync operations; these
tests cover process interruption, not hardware failure. Dispatch, ID allocation
and consumer outbox coordination remain unimplemented.

Inspection checks the selected service and simulated tenant against the file,
then queries only `reference_receipt`. Exit 0 means historical success, 4 means
an absent receipt or recorded lifecycle failure, 2 rejects arguments and 3 reports
storage, lock contention, capacity, binding, conflict or query failures. An old
successful retain can describe
a reference that has since been released. Neither success nor absence authorizes
publication or an uncertain effect; current liveness and consumer coordination
remain separate requirements.

To save the exact hashed bytes for later use, add an existing, caller-controlled
destination directory:

```sh
cargo run --offline --locked -p ic-blob-storage --example prepare_upload -- \
  --inventory inventory.json --snapshot ./snapshots > snapshot.json
```

Success returns `directory` (a fresh absolute path) and `inventory`. The directory
contains `inventory.json` and `bodies/<root hex without sha256:>`, one file per
distinct root. Asset `source` paths in the report remain provenance; use the saved
bodies for later upload. Copying and hashing use the same buffers, so later changes
to the originals cannot change the saved content. Duplicate sources still count
against work limits and are fully checked. Disk use includes saved distinct bodies,
one in-progress body, and the report.

Normal failure removes the current attempt. Each repeat creates a separate
directory and preserves earlier snapshots. On Unix the directory is private (0700)
and body files are 0600. Files are synced, but this is not a portable crash-durable
transaction: interruption can leave incomplete residue or a complete snapshot
without a stdout receipt. No old directory is automatically resumed or deleted.
Keep the saved directory controlled and reverify its files before future effects;
this local copy is mutable and does not persist service operation identities.

The local streaming verification example can save the exact checked bytes:

```sh
cargo run --offline --locked -p ic-blob-storage --example verify_download -- \
  claim.json 10485760 verified.bin < body.bin
```

`claim.json` supplies `root`, `bytes` and `headers` (`name`/`value` entries) from
trusted application data; the second argument is the caller's maximum byte
budget. The example bounds the claim to 16 KiB and uses a 64 KiB receive buffer.
Omit `verified.bin` to verify without retaining the body. With an output path,
the example stages at most the declared length under the existing destination
directory, verifies clean EOF and the root, syncs the file, then publishes without
overwriting a file or symlink. Computed identities print only after success.
Use a caller-controlled directory with no concurrent path replacement or temporary
cleaner; staging uses owner-only directory/file permissions on Unix. Normal
failures clean staging, but interruption can leave residue. A missing stdout receipt
does not prove no file was published: reverify the existing output before using it.
The example does not authenticate the claim, resume interrupted downloads or
promise crash-durable directory updates. Persistence follows
[tempfile's platform guarantees](https://docs.rs/tempfile/3.27.0/tempfile/struct.NamedTempFile.html#method.persist_noclobber).
See the
[consumer download direction](docs/roadmap.md#consumer-download-verification).

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
