# ic-blob-storage

The [0.2 delivery plan](docs/roadmap.md) tracks
the remaining work to a usable service; [current status](docs/status/current.md)
separates implemented behavior from outstanding milestones.

Caffeine is qualified through independent source review and bounded experiments,
with every investigation tracked in the [probe ledger](docs/evidence/caffeine-probes/README.md).
`caffeine-probe` captures public-source evidence with retained requests/responses;
`make probe-check` verifies saved artifacts offline. Provider feedback is useful
but is not a prerequisite for progress. Successful captures do not qualify service
behavior, and live paid trials require their own explicit account and budget.

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
`blob_reference_status` and `ReplicatedReferenceClient::status` separately inspect
an exact reference's current local liveness and restore fence. A successful retain
receipt can remain in history after status becomes non-live. Inspection works while
suspended or restored; a live result is not enrollment, serving or retry authority.
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
The same client's `revoke` method sends `blob_revoke_upload` with the original
permission. Unexposed cancellation frees reservation bytes; possible exposure and
confirmed objects keep their obligations. Local revocation is not provider deletion.
`workflow::uploads::manifests` exposes `blob_prepare_upload` for the actual admitted
uploader and `blob_upload_manifest` for tenant/uploader recovery of the exact first
declaration. Full permission binding, declaration budgets and bounded reply decoding
preserve original leaves and metadata through revocation and restore. Preparation
validates root consistency; it does not transfer bytes or establish completion.
`ReplicatedUploadManifestClient` binds the actual actor, tenant and service, checks
the declaration before preparation dispatch and sends once without automatic retry.
Save the exact intent before polling; recover uncertain replies through inspection.
`workflow::uploads::exposure` adds guarded preview/commit and exact historical
inspection. It binds current host evidence to the original permission and blocks
missing provider/recovery/durability prerequisites before marking possible exposure.
The host must establish those facts independently; equal timestamps alone do not
prove freshness. This workflow emits no certificate or provider call.
`workflow::uploads::certificate` resolves Caffeine's root-only request as the actual
uploader, commits that gate and constructs the reviewed plain update reply. Hosts
must return it synchronously so the IC certifies the committed response. Linking
exports no endpoint; provider qualification and production browser integration
remain open, and successful local IC tests use explicitly substituted host facts.
Headless Rust tests verify real local ingress certificates and recover their saved
request IDs without reissuing. Historical certificates do not renew revoked permission.
The reusable [browser certificate client](clients/browser/README.md) accepts
caller-owned authentication, explicit IC trust and atomic durable intent storage.
It retains exact request identity before issuance and recovers historical replies
without redispatch. The source package is private; production consumer integration
and deployed provider qualification remain open.
The opt-in `make test-browser` checks this client's Chromium signature verification and
IndexedDB intent across competing tabs, cancellation and reload; see
[browser setup](docs/dependencies.md#browser-certificate-evidence).
The upload fixture now reuses Caffeine 1.1.2 through that agent, with a
[small preparation/transport patch](clients/browser/patches/README.md). Hashing,
chunks and HTTP formats stay upstream-owned. Gateway tests use a local substitute;
production admission/transfer coordination and provider qualification remain open.
The separate `blob-consumer-probe` exercises both that first reference and explicit
retains of existing content through real IC calls: bounded durable intent, atomic
publication/tombstones, dependency checks, exact recovery and fenced upgrade.
Fresh registration creates no extra retain receipt. The fixture now sends admission
from its own canister and recovers interrupted acknowledgments without resending.
The consumer also dispatches permission withdrawal after saving its tombstone and
reconciles uncertain acknowledgments through exact inspection. Its two-slot
application substitute is integration evidence. A separate uploader instance now
persists bounded manifest intent before calling the shared client, retaining typed
refusals and recovering uncertain acknowledgments without redispatch. Three-canister
tests join preparation recovery with tenant registration and cleanup. Uploader
cancellation preserves a local tombstone through late replies and recovery; tenant
withdrawal and reference cleanup remain separate operations. Exposed obligations
stay charged until their corresponding cleanup facts are established. Exposure and
provider completion still use private fixture controls. Production browser/headless
integration and intent storage remain open.
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
test evidence. The [standalone host](canisters/standalone/README.md) now owns explicit
installation configuration, memory and synchronous fenced restoration, exposing
tenant, admission, manifest and reference handlers through the shared library.
It also delivers reference-qualified download metadata through the shared update
handler, using an explicit immutable Caffeine project supplied at installation.
The new host init contract requires a minor release and cross-release reinstall.
Operator-only account inspection now uses one bounded replicated Cashier balance
or payer-relationship query through a shared handler. Reports stay separate from
funding and readiness; no cycles attach or account changes occur.
Complete production workflows, clients, the Canic adapter and
deployed-provider qualification remain open. Local bookkeeping and decoded
provider reports do not establish a qualified storage service.

The planned service owns tenant authorization, references, quotas, provider
access, billing, retention and deletion. Both standalone
and Canic-managed adapters belong here and use the same handlers and blob API. The core
builds without Canic; Canic owns generic deployment and lifecycle.

Memory dependencies align through the re-exported `ic_memory` crate and its
`ic_stable_structures` substrate. The host owns bootstrap and allocation policy;
linking this library declares no stores or memory IDs. See
[memory composition](docs/dependencies.md#memory-composition-with-canic-and-icydb).
`ops::service::stores::grants` supplies the named service requests and assembles
the sixteen memories through a host's committed lookup. Standalone now uses that
mapping. `open_default` reuses a framework-owned, already bootstrapped runtime;
it refuses absence before constructing a manager. Shared
`ops::service::installation::ServiceInstallation` owns the immutable configuration
record and service assembly. Standalone delegates validation, persistence and
fenced restore to it. Hosts supply the installation candidate, actual identity and
compiled release; they own allocation policy, exclusive storage access,
authentication and synchronous lifecycle calls.

The unpublished `ic-blob-storage-canic` library now provides explicit memory and
caller-guard macros plus synchronous lifecycle operations for that owner.
The artifact declares its allocation range, invokes the macros and supplies
Canic directly. The library itself remains Canic-free. A managed PocketIC fixture
checks activation, caller authority and fenced restoration alongside neighboring
memory. Managed installation takes explicit bounded policy, project and verifier
bytes inside Canic's authenticated carrier; platform service identity and Canic's
validated release identity bind the shared installation. The controlled artifact
now exposes nineteen blob methods for configuration, tenants, uploads/history,
manifests, references/capacity and passive operator/funding inspection using shared
workflows. Their types match standalone Candid. Prepared uploads and suspended
tenant cleanup history survive fenced restoration. Complete managed endpoints,
query/decoding-work bounds, production Fleet provenance and both-adapter acceptance
remain open. The pinned Canic macros need upstream decoder controls to close
the input-bound gap; see the
[composition contract](docs/service-contract.md#managed-canic-composition).

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
| `make test-canic-composition` | Published Canic lifecycle, authority and shared-owner composition |
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

The shared `blob_upload_capacity` query supplies the next planning input:
tenant-scoped lifetime object, concurrent upload, manifest-leaf and byte headroom,
plus enrollment, per-object limits and the restore fence. The standalone host and
both admission fixtures reuse the same handler. It includes reservations
and continuing billing; freed logical quota alone cannot make those obligations
disappear. These independent counts reserve nothing. Existing blobs still require
the shared `blob_lookup_content` and `blob_reference_capacity` queries. Discovery
returns complete original identities and local lifecycle, with exact request echoes
and restore fencing even for absence. Reference capacity binds
the tenant and root, preserves cleanup receipt reservations and reports its own
restore fence. The unpublished
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

The JSON report marks a restored service as blocked even with spare quota,
including a fence observed by the later reference-capacity query. It
distinguishes content not visible to this tenant, unfinished
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

For local standalone and durable storage installations, `blob-fixture-reference`
can preserve an exact intent and inspect its historical receipt without applying
it. It uses the shared `blob_reference_receipt` API and bounded reply decoder;
transport remains PocketIC with a simulated tenant caller:

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
  "upload": "1", "object": "2", "incarnation": "3", "first_reference": "4",
  "root": "sha256:ROOT_HEX", "bytes": 3,
  "reference": "2", "operation": "1", "retain": true
}
```

IDs are independent positive canonical decimal strings, preserving their full
width. The original first reference is required explicitly; no identity is
inferred from another. `retain: false` names a release. The tool does not allocate IDs or
prove freshness. Saving requires an existing, durably created, caller-controlled
local directory. A filename derived from service/tenant/namespace/upload/operation
binds one exact intent. Changing the object, lifetime, first reference, root,
target reference, action, size or asset label conflicts;
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
then queries only `blob_reference_receipt`. Exit 0 means historical success, 4 means
an absent receipt or recorded lifecycle failure, 2 rejects arguments and 3 reports
storage, lock contention, capacity, binding or query failures. Service refusals
also exit 3 and retain their typed reason under `observation.failure`; an
unconfirmed upload is never reported as an absent receipt. An old
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

The unpublished native `blob-storage` command authenticates to the shared service
API with an explicit Ed25519 or secp256k1 PEM identity. Set these variables from
the installation's configuration and the operator's own identity:

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  status --network ic --url "$IC_API_URL" --identity "$OPERATOR_PEM" \
  --operator "$OPERATOR_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --cashier "$CASHIER_PRINCIPAL" \
  --payer "$PAYER_PRINCIPAL"
```

The PEM's principal must match `--operator` before any network request, and the
service must authorize that caller. Namespace is a positive canonical decimal
u128. The complete installed scope is checked in both request and response. IC
mode requires an HTTPS origin and uses ic-agent's built-in IC root key. Local
mode requires `--network local`, a literal loopback origin and `--root-key` naming
a trusted DER root obtained independently from the local replica owner; root
overrides are rejected in IC mode. No root is fetched automatically. URL paths,
credentials, query strings, fragments and HTTP redirects are rejected.

The `status` command only queries `blob_local_status`. It verifies IC query signatures;
the result is a local observation, not certified state, provider credit, readiness
or dispatch authority. It exposes separate upload/funding/gateway/read fences,
including after restore. All counters and amounts are decimal strings, optional
identities remain `null`, and query failures never become zero balances. Exit 0
means an observation was returned (even if fenced), 2 means invalid arguments,
and 3 means a file, identity, transport, decoder, binding or service refusal. Errors
are structured JSON codes; private key contents and remote diagnostics are omitted.
The call has a 30-second deadline, a 256 KiB HTTP response ceiling and a 64 KiB
Candid reply bound with decoder work limits. It never falls back to an update.
Run `make test-standalone` for the local signed HTTP subprocess evidence.

Use `funding-history` in place of `status` with the same authentication and scope
flags to read one descending page from `blob_funding_history`. It accepts at most
32 entries and reuses the library decoder to validate request echo, scope, ordering,
amounts, refunds and continuation. The JSON `entries` preserve `prepared`,
`uncertain`, `not_enqueued` and `callback` phases; callback refunds do not prove
provider credit. Empty and fenced pages still exit 0.

For another page, save the non-null `next` object from the prior result as a JSON
file and pass `--cursor FILE`. The file is limited to 2 KiB and includes the complete
scope plus a decimal-string `before_operation`; a changed scope rejects before
transport. No cursor means a fresh sweep. `next: null` ends this local range; it
does not prove complete provider-account activity. Pages are current observations,
not a snapshot: restart from the beginning to see changes behind a saved cursor.
The command never automatically paginates, retries a payment or writes a journal.
Local tests cover populated history through the shared durable storage fixture;
its payment outcomes are controlled substitutes, not deployed Cashier evidence.

Inspect an exact original funding intent with `funding-outcome`:

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  funding-outcome --network ic --url "$IC_API_URL" --identity "$OPERATOR_PEM" \
  --operator "$OPERATOR_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --cashier "$CASHIER_PRINCIPAL" \
  --payer "$PAYER_PRINCIPAL" --operation "$FUNDING_OPERATION" \
  --offered "$ORIGINAL_OFFERED_CYCLES"
```

Use the original decimal-string operation, offer and optional target from funding
history or your retained intent. Supply `--target-balance DECIMAL` only when the
original intent had a target; omission means `None`, with no inferred default.
Changed retained amounts or target produce `funding_conflict`, not absence.
All supplied numbers must be positive canonical u128 decimals.

One signed `blob_funding_outcome` query returns `outcome: found` with a `record`,
or `outcome: absent` with `record: null`. Absence supplies no fence information,
even on a restored installation; inspect status separately for owner fences.
A found record retains local phase, exact callback refund, optional structured
response, conservative reconciliation and its restore fence. `response: null`
means no structured reply was retained. Reported success balances and typed
provider errors remain separate from attachment accounting.

`transfer_unknown` keeps the entire original offer potentially spent.
`credit_required` names the exact accepted attachment still needing independent
provider-credit evidence. `no_transfer` concerns attached cycles only; execution
fees are separate. Every output sets `retry_authorized: false` and
`provider_credit: not_established`. Empty, uncertain and fenced observations
exit 0; authenticated service refusals exit 3. The bounded decoder checks exact
intent and transport/reconciliation consistency within 4 KiB, with the same
30-second signed-query and 256 KiB HTTP limits as status. No payment, provider
query, polling, journal write or automatic retry occurs.

`upload-history` recovers retained upload identities without saved upload requests:

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  upload-history --network ic --url "$IC_API_URL" --identity "$OPERATOR_PEM" \
  --operator "$OPERATOR_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --filter outstanding
```

The configured operator observes the installed service-wide history. `--filter`
is required: `all` includes cancellation and settlement, `active` includes reserved
or possibly exposed uploads, `deletion-pending` selects unresolved physical deletion,
and `outstanding` also retains live/provider-deleted content with unresolved
obligations. Physical deletion alone does not establish billing cessation.
No Cashier or payer flags are needed for this local history query.

Each command queries `blob_upload_history` once, accepting at most 64 inspected
rows and 32 matching entries in 64 KiB of bounded Candid. JSON preserves all original
upload/object/incarnation/first-reference identities, roots, byte lengths, local
states, the independent restore fence and decimal-string `scanned` count.

Save the non-null `next` object as JSON and pass `--cursor FILE` for another page.
The file is limited to 2 KiB and binds service, namespace, service-wide observer
scope, filter and last inspected tenant/upload ID; malformed or changed scope
rejects before identity/network access. Empty filtered pages may still have a
continuation. `next: null` ends this local traversal, not provider reconciliation.
Pages are current observations; start without a cursor to see changes behind it.

The shared decoder checks ordering, identity scope, duplicate roots/object lifetimes,
filter results, work bounds and cursor progress. Signed queries use the same explicit
IC/local trust and deadline as status. Empty and fenced observations exit 0;
service refusals remain errors. The command reads no provider content, performs
no mutation and never grants retry, serving or recovery authority.

Tenant-owned reference inspection uses separate saved boundary requests:

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  reference-receipt --network ic --url "$IC_API_URL" --identity "$TENANT_PEM" \
  --actor "$TENANT_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --request reference-command.candid

cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  reference-status --network ic --url "$IC_API_URL" --identity "$TENANT_PEM" \
  --actor "$TENANT_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --request reference-status.candid
```

`reference-command.candid` is the original binary Candid `ReferenceCommand` saved
before dispatch. It includes the complete original upload, independent reference
and operation IDs and exact retain/release action. `reference-status.candid` is a
binary Candid `ReferenceStatusRequest` containing that original upload and the
reference to inspect; it has no mutation operation or action. Both inputs are
bounded to 4 KiB and validated against explicit service, namespace and tenant
before identity/network access. The PEM must sign as that tenant. Operator and
controller status grant no tenant authority. A canister tenant cannot be
impersonated with a PEM; use its existing `ReplicatedReferenceClient` integration.

`reference-receipt` signs one `blob_reference_receipt` query. JSON distinguishes
`absent` from `found`, with an original success (`changed`/`unchanged`) or recorded
transition failure. Lookup refusals remain errors, including `reference_unknown`,
`reference_unconfirmed` and `reference_conflict`. A successful retain receipt stays
successful after release or settlement. This reply carries neither a current
liveness observation nor a restore fence; both are labelled `not_observed`.
Absence supplies no retry authority and does not prove that an upload is confirmed.

`reference-status` independently signs one `blob_reference_status` query. It
returns the exact reference's current local `live` flag and owner's `fenced` flag.
False includes never-retained and released references; it never makes an identity
reusable. Even true with a clear local fence is no publication lease, provider
availability guarantee or independent recovery proof. The two observations are
separate in time; neither command silently performs the other query.

Both commands preserve full-width decimal identities/amounts, verify query
signatures with explicit IC/local trust, and use a 4 KiB reply limit, 256 KiB HTTP
ceiling and 30-second deadline. Exit 0 means an observation, including absence or
recorded failure; exit 3 means transport, decoding, binding or lookup refusal.
They set retry/publication authority to false and perform no mutation, provider
request, journal write, polling or automatic retry.

`certificate-assessment` inspects missing issuance prerequisites for a saved permission:

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  certificate-assessment --network ic --url "$IC_API_URL" --identity "$UPLOADER_PEM" \
  --actor "$UPLOADER_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --permission permission.candid
```

The input is one binary Candid `UploadAdmissionRequest` (4 KiB maximum). Service,
namespace and original uploader must match before transport. The command signs one
`blob_upload_certificate_assessment` query, verifies query signatures and decodes
at most 4 KiB against the complete saved permission, including independent object
identities and expiry. JSON preserves decimal-string integers, host assessment time
and every blocker. Prepared standalone uploads currently report `precharge_limits`,
`provider_namespace`, `replay_charging` and `recovery` as missing prerequisites.

Exit 0 means an assessment was observed, including blocked uploads; it never
authorizes issuance or retry. Even an empty list cannot reserve a later update.
Unprepared, expired, revoked and restored permissions refuse with distinct JSON
error codes and exit 3. The same explicit IC/local root trust, 30-second deadline
and 256 KiB HTTP ceiling apply. No certificate update, provider request, file
rewrite or service mutation occurs. This does not qualify provider behavior or
enable the standalone certificate endpoint.

`verify-upload` checks a saved local file against the service's original manifest:

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  verify-upload --network ic --url "$IC_API_URL" --identity "$UPLOADER_PEM" \
  --actor "$UPLOADER_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --permission permission.candid \
  --body downloaded.bin --max-bytes 10485760
```

`permission.candid` contains one binary Candid `UploadAdmissionRequest` saved by
the integrating application (for example, `candid::encode_one(permission)`), not
JSON or hexadecimal text. It is bounded to 4 KiB and binds the full original
operation, uploader and expiry. The signer must be that uploader or the tenant;
operator status supplies no override. The command queries `blob_upload_manifest`,
reuses its bounded exact-reply decoder and hashes file bytes in native 64 KiB frames.
`--max-bytes` is required and capped at 1 GiB; file size must equal the original
declaration. Metadata is bounded to 16 headers/4 KiB and the manifest to 1,024 leaves.
File changes during the read are still subject to length/root verification.

Exit 0 means the bytes read matched the authenticated historical declaration.
The JSON preserves full-width identities and a computed raw digest, and explicitly
reports provider completion/availability as unestablished/unobserved. It does not
fetch provider content, attest completion, write a destination or grant retry
authority. Inspection works after restoration without clearing fences. This is
separate from the configured verifier's trusted availability attestation described
in the [standalone contract](canisters/standalone/README.md).

`upload-attestation` recovers historical evidence using the exact statement saved
before an attestation was sent:

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  upload-attestation --network ic --url "$IC_API_URL" --identity "$VERIFIER_PEM" \
  --actor "$VERIFIER_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --verifier "$VERIFIER_PRINCIPAL" \
  --statement statement.candid
```

The statement is one binary Candid `UploadAttestationRequest`, bounded to 4 KiB.
It contains the complete original permission, raw digest and observation time.
Obtain `--verifier` from the trusted installation configuration; the command does
not discover or install a verifier. The signer may be that verifier, the tenant
or the original uploader. No original body file is required, and the saved
statement is never overwritten. The service remains responsible for caller checks.

The signed `blob_upload_attestation` query reuses the library's bounded decoder
and checks the exact scope, permission, verifier and receipt chronology. JSON
`outcome` is `matched`, `conflict` or `absent`; `receipt` is null only for absence.
Conflicts retain both the expected digest/time and the accepted digest/time.
All three observations exit 0, including when fenced; callers must inspect the
outcome. Failures exit 3 and never become absence. IDs and times remain decimal
strings. Limits are 30 seconds, 256 KiB HTTP and 4 KiB Candid with bounded decoding.

A matched receipt proves the service retained that exact trusted statement. It
does not establish current availability, reference liveness or billing cessation.
Absence does not prove a pending update cannot still complete, and cannot authorize
resending an uncertain effect. Every outcome reports `retry_authorized: false`.
The command sends no update or provider request and writes no journal.

`observe-upload` performs the verifier's independent provider read and saves the
statement for explicit submission. Run it only against an explicitly approved gateway
and installation with a read budget; provider charges remain unknown, including
when a loopback origin forwards requests. No live trial was performed here.

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  observe-upload --network ic --url "$IC_API_URL" --identity "$VERIFIER_PEM" \
  --actor "$VERIFIER_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --permission permission.candid \
  --gateway "$APPROVED_GATEWAY_ORIGIN" --max-bytes 10485760 --run-dir new-observation
```

The service authenticates the verifier through `blob_verification_plan` and returns
its installed owner/project and exact original declaration. The command validates
the signed reply and uses the maintained Caffeine request-target encoder. The host
must have already recorded exposure. Standalone exposes an uploader-only
`blob_upload_certificate_assessment(root)` and the canonical certificate update,
but issuance remains blocked by independent provider/recovery prerequisites; see
the [host contract](canisters/standalone/README.md).
Unexposed, confirmed or restored work rejects before any provider GET. Revoked or
suspended exposed uploads remain eligible for reconciliation.

One GET is allowed, with no redirects, HTTP retries, credentials or automatic
decompression. IC mode requires HTTPS origins; local mode accepts literal loopback
origins only. Both query and GET have 30-second deadlines. Service Candid is bounded
to 64 KiB, 1,024 leaves and 16 headers/4 KiB metadata. The explicit content budget is
at most 1 GiB and is checked before fetching. Whole-body streaming checks the
original root, metadata and exact length off-canister. HTTP metadata cannot replace
the original declaration; partial/encoded responses and incomplete EOF reject.
These request/byte limits are not a billing guarantee or a bound on transport overhead.

The run directory must be new and its parent must exist. Intent is synced before
the query and GET. Artifacts include `plan.json`, `permission.candid`, the bounded
`service-response.candid`, `download-request.json`, HTTP status and download outcome.
Only complete verification writes and syncs `statement.candid`, followed by
`summary.json` with its hash and observation time. The runner fingerprint covers its
observation/transport/argument sources and lockfile; artifact hashes establish local
integrity, not portable IC/provider signatures. Download bodies and identity keys
are never retained. Keep this directory in controlled storage.

Errors retain `failure.json` when writable. Abrupt interruption can leave partial
files with no summary; neither failed nor interrupted runs are resumed or overwritten.
Preserve their evidence before deciding on any separately budgeted new attempt.
A saved statement binds the exact original permission and locally observed UTC time;
the service still checks its admission/acceptance time bounds. The command sends no
attestation or upload, and reports `attestation_dispatched: false` and
`retry_authorized: false`. `verify-upload` local-file
output cannot be promoted to a provider observation.

Submit a completed observation explicitly with the same service URL, service,
namespace and verifier identity:

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  submit-attestation --network ic --url "$IC_API_URL" --identity "$VERIFIER_PEM" \
  --actor "$VERIFIER_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --run-dir new-observation
```

The command validates every required observation artifact, the original declaration,
owner/project, complete download result, exact statement, hashes and time ordering.
Records are bounded and must be regular files; a failure marker rejects the run.
These files remain trusted verifier input: local hashes do not prove authenticity
against someone who can rewrite the entire run. Keep the directory under the
verifier's control, including during submission. No provider read occurs here.

An exclusive `attestation/` directory claims the run. Before one signed update,
the command syncs an exact `statement.candid` copy, `signed-request.cbor` and
`intent.json` containing the runner version/source fingerprint, request ID, expiry,
root-key and artifact hashes. The fingerprint covers the native submission,
observation-reader, shared transport/record/argument sources and lockfile.
Keep the signed envelope private; it can authorize that exact request until expiry.
The update has a 30-second deadline, disabled transport retries and no automatic
polling. A certified reply must match the complete statement before reporting
`accepted`. A queued request reports `pending`; transport or verification failure
leaves an `uncertain` outcome when writable. An authenticated service refusal is
recorded separately. The reply and outcome are retained without changing the
observation's original summary.

Every existing claim, including an empty directory left by a killed process, refuses
resubmission with `submission_already_claimed`. Do not remove or copy the claim to
retry. This is a local single-submission guard, not a distributed lock or global
exactly-once guarantee. Recover with `upload-attestation` using the saved
`attestation/statement.candid` and independently configured verifier. If interruption
preceded that copy, the original observation statement remains available for lookup.
An absent receipt, expired ingress request or local error never authorizes another
submission. No statement, digest or observation time is regenerated. Exit zero can
mean `pending`; inspect the outcome before treating it as acceptance.

The separate unpublished `blob-fixture-status` client attaches to an existing local PocketIC
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
