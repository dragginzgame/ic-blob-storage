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
Set `BLOB_PREVIOUS_REFERENCE_APP` to a retained predecessor Wasm to install that
artifact first and exercise catalog reopening with the current build during upgrade.
There is no public deployment command, browser UI or live provider upload in this slice.
`blob app prepare-installation` prepares the paired init arguments offline;
the installed test consumes those exact files, rather than reconstructing them.
`make test-callbacks` additionally needs Node 24+ and connects this application's
actual Wasm to the local issuer, transfer client and owned loopback HTTP peer.

Create both canister IDs before installing. Blob's immutable application authority
must be the reference application's canister ID. Install the application with
`Installation { blob, users }`, using the bounded Wire frame for `app_init`.
Keep the selected Blob manager separate from the application's uploaders.
Only the configured users can operate their own drafts/catalog; allowlist changes
are unsupported in this first version. Controllers are trusted for lifecycle only.

### Prepare fresh installation inputs

After `make blob`, supply independently selected canister IDs and principals. Run
from the repository root; the output's parent directory must already exist:

```sh
blob app prepare-installation \
  --canister "$APPLICATION_CANISTER" --blob-canister "$BLOB_CANISTER" \
  --manager "$MANAGER_PRINCIPAL" --user "$USER_PRINCIPAL" \
  --max-content-bytes 8388608 --max-chunks 8 \
  --max-headers 8 --max-header-bytes 512 \
  --uploads 64 --references 64 \
  --logical-bytes 16777216 --exposure-bytes 16777216 \
  --output ./application-inputs
```

These example limits are local declared-byte admission policy, not provider storage
or spending bounds. Every limit is explicit and immutable after installation. The
application retains its own eight-user/sixteen-draft limits and 2 KiB input bound;
a larger Blob limit does not expand them. Repeat `--user` for additional users.
The preparer requires distinct application, Blob, manager and user principals,
with one to eight unique users; special principals and invalid bounds reject
before creating output. Historical upload/reference limits cannot exceed 64.

The new directory contains `blob-init.wire`, `app-init.wire` and a human-readable
`inputs.txt` with the selected principals, limits and both SHA-256 hashes. The CLI
encodes the existing `blob_init` and `app_init` request types. It generates no
new installation schema, state ledger, signed request or deployment capability.
No credentials, network access or cycles are needed. Existing or partially written
directories refuse replacement; preserve partial output for inspection. Success
is reported only after all three files have been published.

The principal arguments describe the intended pairing, not authenticated target
identity or proof that either canister is empty. The init frames bind the peer and
authorities; they do not encode their own destination or a Wasm hash. Installation
still requires independently admitted artifacts, actual target/controller checks,
durable signed-request custody and readback. Never reinstall an existing canister
with these files or use them to bypass the consumed live enrollment probe.

Targeted review of the legacy [offline installation checker](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage-cli/src/native/installation_check/mod.rs)
and [installed CLI-carrier regression](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/qualification/ic-blob-storage-pocketic-tests/tests/standalone_installation_cli/mod.rs)
retains their distinction between a local proposal and authenticated installation,
and their test of installing exact CLI output. No legacy code, project/release
identity framework, provider configuration or reinstall workflow is copied. The
current host's init contract lacks the legacy explicit own-target field; this
preparation does not claim its wrong-target refusal guarantee.

### Check proposed targets

Before preparing signed installation custody, observe the two selected canisters:

```sh
blob app check-targets \
  --url https://icp-api.io \
  --canister "$APPLICATION_CANISTER" --blob-canister "$BLOB_CANISTER" \
  --identity "$CONTROLLER_PEM"
```

The signer must currently be the sole controller of both targets. This controller
is distinct from the maintenance-manager role named in the init arguments, though
an operator may deliberately choose the same principal. Local gateways require an
independently trusted `--root-key` and `--root-subnet`; no root is fetched from the selected endpoint.

The command uses replicated management status, the existing bounded decoder and
a 30-second overall deadline. It refuses equal/special targets, installed modules,
additional controllers and stopping targets. It prints a report only when both
observations pass; no partial success report, install, reinstall, creation, funding
or controller change is emitted. The installed management fixture covers correct
and incorrect roots, controller changes and pre-existing modules, and confirms
that target configuration and stable bytes remain unchanged by inspection.

These are two separate observations, not an atomic reservation or authorization
for a later install. A canister without a module may have been used and uninstalled;
require the original fresh-creation custody and recheck before any future effect.
The check does not bind Wasm, init arguments or a signed install request. That
custody and final readback remain the next implementation step. It retains the
legacy distinction between proposed identity and actual platform observation,
without copying its deployment/recovery framework.

For Canic's embedded catalog, use the
[Canic integration handoff](application-boundary.md#fleet-integration-handoff--2026-11-16).
That application owns a different combined init type; these paired files do not
configure it. The shared `app` CLI spelling does not unify application schemas.

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

`blob app catalog` and `blob app draft --id ...` expose these
owner-only queries through the native CLI. Select the application canister and
the user's signing key; the [inspection guide](../operations/storage-inspection.md#your-reference-application-catalog-and-drafts)
owns command examples and observation limits. The CLI imports the existing protocol
types rather than duplicating them. It does not create drafts or publish/cancel them.

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
It prepares the two init files with a fresh CLI process, compares them against the
independently specified configuration and installs their unchanged bytes before
exercising caller separation, catalog decisions, cleanup and upgrades.
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

### Transfer and verified-read composition — development 0.100.9

The [installed composition](../../qualification/ic-blob-storage-tests/tests/callbacks/issuance/publication.rs)
now joins this unchanged application to the candidate issuer, original CLI proof
recovery, permanent `claim-transfer` custody and the actual client. Each scenario
uses a fresh installation with one 5 MiB + 17 byte, six-chunk declaration. The
application authorizes it under the captured user's identity; bytes never pass
through either canister. Rust and JS independently prepare the retained root vector.

The client waits for `verifyRead` to return the exact immutable bytes before asking
the user-authenticated `app_publish` to publish the original draft ID. Tests show:

- HTTP upload acknowledgments alone leave the catalog empty.
- A verified complete read precedes successful catalog publication.
- Corruption in the second chunk prevents the publication call.
- Cancellation while the tree response is pending cleans the draft; aborting the
  local client prevents any chunk PUT. Even a stale publish command refuses.
- Cancellation while a read remainder is pending cleans the draft. A deliberately
  delivered valid remainder still verifies, but its late publication request refuses.
- Losing the final chunk's HTTP response after every byte arrived leaves the
  draft unpublished; the client records uncertainty rather than reporting success.
- A connection cut during readback returns no verified file or publication request.
- Killing the client process while its first chunk reply is pending preserves the
  original draft and transfer claim. A fresh process after this or either network
  interruption refuses before another PUT; original intent/proof bytes stay unchanged.
- After a lost final upload reply, an explicit read-only reconciliation inspects
  the original draft and starts a fresh reader without signer, proof or custody
  inputs. It GETs the already-received bytes and verifies the saved declaration.
  Success permits the owner's separate publication call; corrupt content or
  cancellation during that read leaves the catalog empty. No certificate reissue
  or PUT follows; the original upload remains permanently claimed.
- Final cleanup leaves the full declared provider exposure unresolved in every case.

Rust owns IC signing and Wire encoding; the Node driver coordinates barriers over
its private child-process pipes. These are test controls, not application endpoints
or a second catalog. The local HTTP peer only stores fixture bytes. This qualifies
client ordering and real application decisions, not provider acceptance, a deployed
browser, or server-side read attestation. An authorized user can still explicitly
publish under the existing application policy without presenting verification proof.
The ten scenarios run on independent local installations; restart cases reopen
their original custody and use a fresh prepared handle and loopback gateway.
They demonstrate process termination and connection loss, not power-loss durability,
provider retention or automatic recovery of an uncertain transfer. The explicit
reconciliation reuses the original HTTP peer's received chunks; it does not seed a
new peer from source bytes. Provider reads and catalog publication leave Blob's
stable state unchanged; application cancellation separately releases the reference.

Applications should commit cancellation at the catalog and abort their local
transfer/read. A lost or late abort cannot revoke issued authority, retract a PUT,
or reopen a cancelled draft. Keep original transfer custody after either outcome.
The pinned [legacy publication helper](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/clients/browser/publication.js)
also separated immutable preparation and transfer from publication. Its SDK patch,
browser journal and worker framework are not needed for this consumer; no legacy
code or recovery records are imported.

### Chromium and the installed catalog — development 0.100.12

`make test-browser-application BROWSER=/absolute/path/to/chrome` reuses the installed
composition and the real Chromium runtime fixture. Each of its two independent
installations supplies a newly recovered original proof and its trust root to the
browser. Chromium authenticates it, commits its IndexedDB claim, sends the tree and
six chunks, reloads, refuses upload replay and verifies the received bytes against
the original declaration. The actual application catalog stays empty after PUT
acknowledgments. Successful verification then precedes the owner's signed
publication call.

Both cases hold the read remainder through a supported same-format upgrade of the
application and callback host: stop the application, drain/stop Blob, upgrade both,
start Blob with admission still closed, explicitly activate the same maintenance
attempt, then start the application. The fixture compares both stable images before
and after upgrade; it never replaces state or reissues upload authority.

The cancellation case first cleans the actual draft, then performs that upgrade.
Chromium still verifies the valid remainder, but the ensuing signed publication
returns `Cancelled` and the catalog remains empty. Original request, dispatch and
certificate bytes are unchanged; cleanup retains the full declared provider
exposure in both cases. Readback and the subsequent publication decision leave
Blob's post-maintenance stable image unchanged.

The private test pipe carries only synchronization bytes. Rust continues to own IC
signing and Wire application calls; Chromium owns proof authentication, custody and
HTTP transfer/read execution. This is installed local composition, not a browser
signing integration, deployed UI, read attestation or Caffeine acceptance witness.
No production code, second catalog or new recovery ledger is added. This retains
the legacy publication helper's ordering invariant cited above and the
[legacy lifecycle regression's](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/qualification/ic-blob-storage-pocketic-tests/tests/standalone_lifecycle/mod.rs)
preservation of original obligations and closed admission across upgrades. No
legacy code or IC-history recovery mechanism is imported: the rewrite qualifies
ordinary upgrades under trusted controller custody, not snapshot restoration or
arbitrary rollback. The test is explicitly selected and excluded from routine
Node/installed callback runs because it requires an externally supplied browser.
