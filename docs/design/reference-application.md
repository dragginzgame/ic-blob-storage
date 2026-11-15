# Standalone reference application

`ic-blob-storage-reference-app` is the first installed application consumer of the
experimental Blob host. It owns a small durable asset catalog and its cleanup
decisions. It is independent of Canic and uses the existing `blob_update` and
replicated `blob_query` methods. Blob alone owns grants, references and accounting.
This is a local reference application, not the production Dragginz asset pipeline.

## Run and install

Run `make install-tools`, then `make test-application` from the repository root.
The latter builds both Wasm artifacts and installs fresh canisters on separate
local PocketIC subnets. `make build-reference-app` builds only the application;
Cargo artifact paths are recorded in `target/blob-reference-app-artifacts.jsonl`.
There is no public deployment command, browser UI or provider upload in this slice.

Create both canister IDs before installing. Blob's immutable application authority
must be the reference application's canister ID. Install the application with
`Installation { blob, users }`, using the bounded Wire frame for `app_init`.
Keep the selected Blob manager separate from the application's uploaders.
Only the configured users can operate their own drafts/catalog; allowlist changes
are unsupported in this first version. Controllers are trusted for lifecycle only.

The [public protocol](../../crates/ic-blob-storage-reference-app/src/protocol.rs)
contains the actual request/result types. These are experimental Wire endpoints,
not Candid methods or a frozen client contract:

| Method | Input | Rule |
| --- | --- | --- |
| `app_create` update | `DraftInput` | Capture uploader, persist exact draft, then authorize its upload and first reference at Blob |
| `app_publish` update | Draft ID | Inspect original upload and live reference, recheck local state after awaits, publish in this user's catalog |
| `app_cancel` update | Draft ID | Unpublish irreversibly before calls, reconcile exact authorization, release the original reference |
| `app_draft` query | Draft ID | Return current durable disposition to its owning user |
| `app_catalog` query | Unit | Return this user's published drafts |

Choose a globally unique nonzero 128-bit draft ID. It also names the Blob upload
and first reference and is never reused. Knowledge of it grants no permission.
The saved input includes the asset name, root, length, deadline, leaves and headers;
the caller cannot supply another uploader. An exact create retry reconciles the
same draft; changed input or a different owner cannot replace it. Concurrent users
must coordinate unique IDs; this reference application has no allocator service.

## Publication and cleanup

After create succeeds, the bound uploader calls Blob's experimental root-only
registration directly. Registration performs a local transition only: no file
transfer, certificate or provider acceptance is implied. A second use of an already
Open matching object attaches its own reference without registering again.

Publication accepts the owning user's decision. It checks Blob itself, not a
client-supplied liveness claim; it does not independently verify file bytes. A
published name is unique within that user's catalog. Another draft can replace it
only after the old draft is cancelled. Old releases always target the old ID.
Catalog queries are owner-only; publication is catalog inclusion, not a file access
restriction or a promise of continuing availability.

`Draft → Published → Cancelled → Cleaned` is the usual path. An unpublished Draft
can also be cancelled. Cancellation is permanent and removes catalog membership
before any remote cleanup. Delayed create/publication callbacks cannot revive it.
`Cleaned` means the first reference was released/retired, or its exact authorization
expired without ever being admitted. It never means provider deletion or settlement.

On a missing reply or `Uncertain`, query the same draft and retry the same input or
cancel ID. There is no automatic timer/retry, replacement ID or receipt eviction.
An unknown reference does not prove cleanup: cancel first resolves the exact saved
authorization, then releases that reference. This can create and immediately retire
an otherwise absent reservation. While Blob is drained, new admission must wait;
already admitted authorization replay and reference cleanup still work.

Only a typed `Expired` reply to that exact `Authorize` can finish cancellation
without a release. Blob resolves retained upload IDs before checking expiry, so
this reply establishes nonadmission and a passed deadline at Blob. With immutable
input and Blob's monotonic IC time, delayed calls cannot admit it later. An admitted
grant still resolves to its historical outcome and must be released after expiry.
The app never infers this from its own clock, an unknown reference or a transport
failure. See the [boundary contract](application-boundary.md#application-decisions-across-delayed-replies).

All attempts retain their lifetime draft slot, including this resolved case. Other
unresolved refusals remain cancelled with cleanup pending; there is no administrative
override to forget them. A new ID cannot bypass Blob's permanent root lifetime rule.

## Bounds and lifecycle

One versioned application record uses an `ic-storage` grant, bounded to 64 KiB.
There are at most eight configured users and sixteen lifetime drafts total, including
failed/cancelled attempts. Each input is at most 2 KiB encoded, with an asset name
of 1–64 UTF-8 bytes, at most eight leaves and eight headers; Blob's selected limits
also apply. These are reference-app limits, not a sustained-use capacity claim.
Existing inspection/cleanup needs no new slot, even after admission fills up.
The allowlist is trusted to share this small global capacity; there are no tenant
quotas. The catalog is derived from Published drafts, with no second catalog index
or persisted copy of Blob's liveness/accounting.

Supported maintenance is **stop and drain, ordinary upgrade, start** under trusted
controller custody. A runtime call counter refuses upgrades while tracked calls
are pending; it is not a durable workflow or snapshot-continuity proof. Bounded
remote waits can leave uncertain domain outcomes, which remain in saved drafts.
Reopen checks format, installation, configuration, bounded inputs, unique IDs and
catalog-name invariants. Missing/corrupt/foreign state traps; it never becomes fresh
state. Pass a unit Wire frame for `app_post_upgrade`; configuration replacement and
arbitrary migration are unsupported. The Blob admin CLI does not manage this app.
Reinstall, old snapshot restore and untrusted controller bypass are outside this
supported procedure. No app-owned recovery framework is introduced.

## Evidence

The [installed test](../../qualification/ic-blob-storage-tests/tests/application.rs)
uses both real Wasm modules, with no forwarding/fault-mode application substitute.
It checks wrong-method/trailing/oversized frames at all five application endpoints,
and invalid draft bounds without consuming an ID or changing either canister's state.
It also covers caller separation, exact retries, name collisions, shared-object uses,
actual pending cross-subnet publication and authorization races, discarded cleanup
reply, stopped-host failures, never-admitted expiry cleanup without Blob mutation,
refusal to renew that draft, admitted-grant release after expiry, durable cleanup
after upgrade, full lifetime capacity
and cleanup while Blob is drained. It also attempts upgrade while a real Blob call
is pending: the app's own guard refuses, the original callback completes and stable
bytes remain unchanged. Replacement installation arguments are rejected after
draining; the original user's draft remains accessible. Publication leaves Blob stable bytes unchanged.
Disposable recipients also exercise reopening with empty stable memory, a zeroed
storage page and the original populated application's bytes under another canister
identity. Each upgrade must trap without rewriting the supplied bytes or changing
the original application/Blob state; the original catalog and cleaned draft remain
readable. Direct simulator injection is a negative fixture, not a supported restore
procedure or a proof that an older same-canister snapshot can be detected.
Upgrade evidence is same-format and after drained runtime calls; it does not prove
arbitrary migration, an adversarial controller or provider availability/recovery.

Legacy comparison and the broader acceptance boundaries remain in
[application-boundary.md](application-boundary.md#evidence-and-acceptance).
