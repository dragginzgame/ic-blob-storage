# Browser certificate client

Integration direction: reuse Caffeine's upload implementation. This private module
is the narrow certificate/intent boundary, now composed locally with the published
package and a [small preparation/transport patch](patches/README.md).
It must not grow into a second hashing/chunking/upload SDK.
See the [reuse assessment](../../docs/provider-review.md#browser-reuse-assessment--2026-09-28)
for existing extension points and the specific missing controls.

`createCertificateClient` is the reusable browser certificate transport. This
private source package uses the exact `@icp-sdk/core` peer version in package.json;
it is not published to npm. The Chromium fixture imports this implementation.
The peer is SDK 5.4.0, within Caffeine 1.1.2's declared `^5.3.0` dependency range.
Application authentication, selection of a persistent browser environment and
deployed Caffeine qualification are still integration requirements. The package
now supplies the bounded IndexedDB journal described below.

Certificate, journal and worker boundaries share the same canonical binding
rules; gateway and journal budgets also share their bounds. Each boundary keeps
its own role checks, input snapshots and durable transaction requirements.
Malformed worker bindings refuse before journal access, including with a custom store.

```js
import { createCertificateClient } from './certificate.js';
import { createIndexedDBIntentStore } from './intents.js';

// Explicit first-time setup in the selected browser profile and origin.
const intents = await createIndexedDBIntentStore({
  database: 'isolated-upload-intents-v1', maxSlots: 1, mode: 'create',
});

const client = await createCertificateClient({
  host, identity, rootKey, binding, intents,
});
// An explicit first attempt; throws if already dispatched or cancelled.
const observation = await client.issue();
// After an uncertain response, use client.recover(), never issue again.
```

For Caffeine's upload path, use `createUploadTransfer` from `./transfer.js` with
the maintained [patched SDK](patches/README.md). The private package requires
Caffeine 1.1.2 and SDK 5.4.0; the unpatched npm artifact refuses with `sdk`.
Prepare with Caffeine before service admission, then create the transfer after
authenticated admission/preparation using the same certificate client and store:

```js
import { createUploadTransfer } from './transfer.js';

const transfer = await createUploadTransfer({
  certificate: client, intents, origin: gatewayOrigin,
  maxRequests, maxRequestBytes, maxTotalRequestBytes, signal,
});
const result = await transfer.uploadPrepared(prepared, onProgress);
```

The helper derives the SDK owner, project and bucket from the immutable certificate
binding and refuses a different prepared root before issuance.
It fixes `retry: false` and `concurrency: 1`, using the existing gateway journal.
`transfer.transport` is that same guarded fetch hook for request inspection or
direct composition; it shares the SDK's session and budgets. It does not create a
second request owner. Caffeine owns preparation, handle consumption, chunking,
certificate extraction and wire formats. The existing guarded real SDK HttpAgent
refuses cancelled observations before handing certificates to Caffeine.

The returned `{ hash }` and progress are SDK observations, not verified availability
or permission to publish. Complete download verification, configured-verifier
attestation and the tenant's authenticated asset transaction remain separate.
Explicit project/bucket strings do not qualify provider namespace ownership;
client request bounds do not prove provider pre-charge or replay charging limits.
Production consumer integration and persistence qualification remain open.

For native-prepared inputs, load `certificate-binding.json` from
[`blob-storage upload-inputs`](../../docs/operator-guide.md#generate-upload-inputs-offline)
as `binding` above. It derives the existing key/service/tenant/uploader/operation/
root fields and opaque permission bytes from one validated Rust permission, after
complete body verification. Native preparation requires complete reviewed
`installation.candid`, checks the proposed service/namespace/project/trusted uploader
and resource bounds, and retains those exact bytes/hash. Actual installed state
and provider provisioning remain separate checks.
`operation` is the original upload ID. The application
still supplies its own identity, trusted IC origin/root and qualified durable store;
the file grants no certificate, dispatch or retry authority. Confirm signed service
admission/preparation before issuance, including current host prerequisites.
Use the saved `body.bin` if rebuilding a Caffeine prepared handle, retain the
original metadata and require the rebuilt root and byte length to match before
any certificate/gateway effect. The SDK continues to own preparation and transfer.

### Transfer a frozen publication file

For an indexed `publish-inputs` file after authenticated `publish-prepare`, use
`createPublicationUpload` (package subpath `./publication`). The consumer must
validate the full batch/setup report and select the original file; this helper
accepts one body and owns no ID allocation or setup journal. Independent native
commands reverify the full batch; `publish-session` retains one
validated batch and supplies the selected cached transfer descriptor instead.

```js
import { createPublicationUpload } from './publication.js';

const upload = await createPublicationUpload({
  host, identity, rootKey, binding, intents,
  body: new Uint8Array(savedBody), bodySha256: savedRawSha256,
  manifestJSON: originalSdkManifestJSON, contentType, filename, maxBodyBytes,
  origin: gatewayOrigin, maxRequests, maxRequestBytes, maxTotalRequestBytes, signal,
});
const result = await upload.upload(onProgress);
```

Load `binding` from the selected `file-0000/certificate-binding.json`, body from
`body.bin`, raw SHA-256 from the frozen inventory, and SDK manifest from
`manifest.json`. Load original hints from `binding.json`'s required `preparation`
object (`content_type`/`filename`), or the native session transfer descriptor.
Preserve original preparation hints, including omitted filename;
the SDK rebuild must reproduce exact headers, ordered leaves, root and body length.
The helper snapshots body/binding/trust root before any await and checks the saved
raw digest before saving certificate intent. Its body bound is at most 1 GiB;
actual browser memory and gateway budgets need separate qualification.

The existing certificate and gateway journals own dispatch, with no retry and
one SDK upload per helper. Reopening supports `recoverCertificate()` for historical
certificate inspection; `.upload()` refuses claimed or cancelled rows and cannot
resume an uncertain gateway effect. `inspect()` and `cancel()` use the same journal.
SDK success remains an observation: independently download/verify, submit the
configured verifier attestation, and confirm tenant references before publishing
the media map. Keep all native and browser journals after any failure.

The standalone Chromium fixture now uses frozen native batch preparation and this
helper. Its HTTPS HTTP/2 gateway is an owned substitute; success, lost-final-reply,
corrupt-read and withdrawn/late-completion journeys establish local composition,
not production provider behavior or a complete Miner publication.

The serial local fixture drives two original frozen files at one active service
reservation, with independent native verification/attestation between transfers
and `publish-file-status` before advancing. Whole-browser restarts preserve the
existing strict IndexedDB rows. Lost chunk replies reconcile without another PUT;
corrupt observation stops before the second upload or any complete media map.
The [serial recipe](../../docs/operator-guide.md#complete-one-file-before-preparing-the-next)
describes the remaining consumer orchestration boundary. This helper still owns
one file's SDK transfer; it does not own service setup, verifier credentials or a
second batch dispatch journal. The persistent-session fixture additionally holds
one validated batch across setup/status/map, rechecks selected bytes before setup,
and reopens original signed setup journals after native process interruption.
An exposed permission refuses setup recovery with zero updates; independent
verification reconciles the original lost-reply upload before advancement. See
the [session protocol](../../docs/operator-guide.md#hold-one-validated-batch-across-publication-phases).
The maintained SDK and strict IndexedDB rows remain the certificate/gateway owners;
the worker below is implemented. The native session can now compose the maintained
observer and one-shot attestation with an explicitly selected verifier, or inspect
an original observation without another GET/submission. The maintained host below
boots a selected SDK signer over its private port. The Chromium bridge below owns
process launch and checks the native session's immutable selection. Automatic
key discovery and durable parent phase coordination remain open.

### Run jobs in a browser worker

`createPublicationWorker` (package subpath `./worker`) supplies the
browser job boundary for a headless publisher. It runs in a real DedicatedWorker
as well as a trusted browser context. Bootstrap with an explicitly selected
signer and existing strict IndexedDB store. Bind service, tenant, uploader,
project, bucket, IC trust root and reviewed gateway before jobs. A job or content
digest cannot select another tenant or gateway.

```js
import { createPublicationWorker, servePublicationWorker } from './worker.js';

const worker = createPublicationWorker({
  host, identity, rootKey, service, tenant, uploader, project, bucket, intents,
  origin: gatewayOrigin, maxBodyBytes, maxRequests, maxRequestBytes,
  maxTotalRequestBytes, maxJobs: 32, timeoutSeconds: 120, signal,
});
// Inside an application-bootstrapped worker, accept only its trusted private port.
const stopServing = servePublicationWorker(worker, trustedPort);
```

The host owns worker creation, signer loading and private-port bootstrap. The
library adds no public window listener, HTTP credential route or identity loader.
Keep private signing material out of static assets, logs and public responses.
Stopping the port does not delete history, cancel the service operation or prove
a stopped effect. The caller owns browser termination and loss/rollback fencing.

Send passive structured-clone jobs over the private port, or call `execute`
directly in a trusted context. An upload job is:

```js
const result = await worker.execute({
  schema: 1, id: 1, action: 'upload', index: 0, binding,
  snapshot: {
    body: selectedBytes, bodySha256: originalRawSha256,
    manifestJSON: originalSdkManifestJSON, contentType, filename,
  },
});
```

The body must be a nonempty `Uint8Array` within the configured limit. Supply the
original metadata hints, including omitted filename; the SDK rebuild must match
the frozen metadata/leaves/root exactly. The native session's cached descriptor
supplies body path, digest, manifest and binding; the consumer loads that body and
supplies original hints. Require native setup's `prepared:true` before uploading.
No array-of-bytes fallback or alternative provider decoder is introduced.

| Job action | Result and effects |
| --- | --- |
| `upload` | `transfer-observed` after SDK success; independent verification still required. Claimed, cancelled or started gateway history refuses before another transfer |
| `inspect` | `inspected` with a bounded journal projection, including `present:false`; no new intent or provider request |
| `recover-certificate` | Verify the exact historical certificate; return `certificate-observed` and its byte count, without certificate redispatch or provider upload |
| `cancel` | Persist cancellation in the existing row; separate from tenant withdrawal, reference release, deletion and billing cessation |

Other jobs contain the same `schema`, increasing `id`, original `index` and
`binding`, without `snapshot`. Recovery/cancellation require exact existing
history and never create a missing row. Unknown fields/actions, foreign scope
and malformed snapshots fail. Body, binding and trust bytes are snapshotted before
awaits. One active job is allowed; concurrent requests refuse without clearing
another job's guard. Every call consumes the finite budget (maximum 32,769).
IDs increase per worker and index is bounded to the current batch ceiling.
These process-local numbers correlate jobs; existing signed certificate/gateway
claims remain durable dispatch authority.

Replies echo bounded `id`, `index` and `action`, with state or a redacted error
code from a finite public vocabulary; arbitrary exported-error code strings are
also redacted. Journal projections validate cancellation, certificate phase and at most
256 gateway request index/phase/status entries. They omit envelopes, certificates,
permission bytes, URLs, headers and provider/SDK error messages. Every reply states
`service_completion_checked:false` and `retry_authorized:false`. Require independent
whole-download observation/attestation, then the native session's `file_live:true`
before advancing; finish with its map. Neither SDK success nor certificate
recovery establishes service completion.

The configured deadline (at most one hour) aborts network requests and fences new
jobs after idle time. Crypto, IndexedDB and scheduling remain cooperative; the
host must own process termination. Preserve the original profile and all native
journals after failure. Reopen the same durable store; never select a new or
rolled-back profile to replay uncertainty. Job/request bounds do not guarantee
a provider spending cap. Cancellation alone cannot dispose of stored bytes.

Actual Chromium/PocketIC tests use this implementation in DedicatedWorkers at one
active reservation. Browser/native restart reconciles lost replies without new
uploads. Cancellation survives reopening; corrupt observation blocks the next
transfer/map while preserving exposed bytes. Offline control tests use a substituted
store and establish boundary checks only. Automatic parent phase coordination,
real Miner media and serving acceptance remain unfinished;
the owned gateway is not deployed Caffeine.

### Launch the maintained worker with a selected signer

Released 0.11.0 `createPublicationWorkerHost` (`./bootstrap`) launches the explicitly
named same-origin module from `./worker-entry`. Bundle that entry with the reviewed
Caffeine patch and pinned peers, and serve it from a trusted application asset
origin. The host transfers one private port; SDK identity JSON and configuration
cross only that port. There is no public window handler, HTTP credential route,
identity discovery or default signer.

```js
import { createPublicationWorkerHost } from './bootstrap.js';

const host = await createPublicationWorkerHost({
  workerURL: '/publication-worker.js', signal,
  bootstrap: {
    schema: 1, operation: 'bootstrap',
    signer: { kind: 'ed25519', json: JSON.stringify(selectedIdentity.toJSON()) },
    configuration: {
      host: icOrigin, rootKey, service, tenant, uploader, project, bucket,
      origin: gatewayOrigin, maxBodyBytes, maxRequests, maxRequestBytes,
      maxTotalRequestBytes, maxJobs, timeoutSeconds,
    },
    journal: { database: originalDatabase, maxSlots, mode: 'open' },
  },
});
const report = await host.execute(originalWorkerJob);
host.close();
```

Use `ed25519` or `secp256k1` with that SDK identity's canonical JSON. The bootstrap
recomputes the public key through the SDK, checks the advertised pair and exact
configured uploader, then validates bindings/budgets before opening IndexedDB.
The trust root is an explicit `Uint8Array`; preparation hints remain the original
worker job's `contentType`/`filename`, including their omission. A new journal
requires explicit `mode:'create'`; reopening uses `open`. Missing/rolled-back
profiles never authorize a replacement journal or another transfer.

The host owns one outstanding request, exact reply correlation and worker
termination on close, abort or deadline. It snapshots bounded jobs and transfers
its own body buffer, leaving caller bytes attached. Small views of larger buffers
copy only the selected bytes. Closing rejects outstanding work and preserves the
existing profile/journals; it does not prove that an already exposed effect stopped.
Replies remain the worker's redacted observations with no completion/retry authority.
Private signer material lives in trusted process memory; JavaScript key erasure
and hostile asset-origin isolation are not guarantees of this helper.

Actual Chromium checks cover both signer kinds, malformed/mismatched bootstrap
before journal creation, body snapshots, concurrency, hard worker termination and
reopening the same profile. Serial PocketIC uploads use this host and maintained
entry with independent native verifier completion. Release 0.11.0 also adds the native
Chromium bridge below. Complete durable parent restart/phase coordination remains
unfinished; select SDK identity JSON explicitly rather than inventing a PEM parser.

## Launch Chromium from a native parent

Released 0.11.0 `launchPublicationBrowser` (`./launcher`) owns a persistent Playwright
Chromium context and the maintained host/worker. Supply your installed Chromium
engine, explicitly selected signer/bootstrap, trusted built bundles, original
absolute profile path and fixed loopback port. The private client adds no Playwright
or framework dependency to the Rust/Wasm graph; the evidence harness uses pinned
Playwright 1.63.0. Build the reviewed bundles with `tests/browser/build.mjs`.

```js
import { chromium } from 'playwright';
import { launchPublicationBrowser } from './launcher.mjs';

const browser = await launchPublicationBrowser(chromium, {
  profile: originalAbsoluteProfilePath,
  assetPort: originalLoopbackPort,
  hostBundle: absolutePublicationHostBundle,
  workerBundle: absolutePublicationWorkerBundle,
  bootstrap: selectedBootstrap, // same selected-signer contract above
  nativeSession: originalAbsoluteSessionPath, // explicit null for browser-only operation
});
try {
  const report = await browser.execute({
    id: 1, index: 0, action: 'transfer', nativePhase: transferPhase.report.native_phase,
  });
  // Independently verify/attest and inspect the current reference in the native
  // session. This browser report never grants completion or retry authority.
} finally {
  await browser.close();
}
```

The native `prepare` phase must have returned `prepared:true` first. For a
browser-selected session, request its `transfer` phase next and pass the returned
absolute `native_phase` step directory. Native-bound launchers refuse direct
`upload` commands. They read the retained control/handoff records, check the exact
original session and current immutable inputs, and send one initial upload or
certificate recovery to the existing worker. Repeated/recovered native phases
never request another upload; missing browser history returns `history-missing`.
Keep original handoffs and profile claims. Reusing an archived initial handoff is
not retry permission; the existing worker also refuses an already claimed upload.

With explicit `nativeSession:null`, browser-only callers use
`{id,index,action:'upload',transfer}` from their independently validated setup.
The transfer
contains `binding`, absolute `body`, `body_sha256`, `manifest_json`, decimal `bytes`
and the original `preparation` object. The bridge owns this request before awaits,
opens only the selected regular file with an explicit body ceiling, checks exact
length/EOF and raw digest, and sends at most 64 KiB per CDP body value. The worker
then recomputes the SDK root/manifest before certificate intent. No body, signer
or configuration is exposed through the asset HTTP server; it serves only the
fixed page and two trusted JavaScript bundles, with no cache, redirects or fallback
routes. Caller-selected assets and the local processes remain trusted.

For `inspect`, `recover-certificate` or `cancel`, supply `{id,index,action,binding}`.
Only one job runs at a time. Request IDs are execution correlation, not durable
operation identity. Replies stay redacted and retain `service_completion_checked`
and `retry_authorized` as false. Local body refusals cause no dispatch; a browser
transport/control failure closes the context and leaves its effects uncertain.

First launch requires both a new profile and explicit journal `mode:'create'`.
Unreleased adds a private `publication-binding.json` in that profile before
Chromium starts. Its frozen `ic-blob-storage/browser-profile:native-session-selection`
identity binds the canonical profile path, asset origin, selected signer JSON
fingerprint, host/service/tenant/uploader/project/bucket/gateway, root-key fingerprint,
journal database/capacity and both executable bundle fingerprints. No signer key
is stored. Exclusive creation, file synchronization and profile-directory
synchronization precede browser access. Failed launch retains the record.

For native publication, first start `publish-session --browser-selection FILE`
using the [selection contract](../../docs/operator-guide.md#bind-browser-selection-to-the-native-session)
and wait for its ready event. Then launch with `nativeSession` pointing to that
original session directory. Before profile creation or Chromium access, the
launcher checks the complete current intent/ready shapes, actual input hashes,
scope/trust and selected signer/journal/bundles. Its profile record retains the
original session path and exact intent-byte fingerprint. Recovered native sessions
carry the same selection; keep the launcher pointed at the original session.
An explicit `nativeSession:null` selects browser-only operation at first creation.
Once created, a profile cannot switch between those contracts or change its native
session. Neither the ready record nor the profile fingerprint grants a lease,
fresh completion, rollback resistance or replay permission.

Reopening requires the original profile/port/database, matching immutable inputs
and `mode:'open'`. Missing, partial, changed or relocated bindings refuse before
Chromium starts; the launcher never synthesizes or repairs provenance. Keep old
profiles with their original launcher/bundles; there is no conversion or adoption
of existing history by the new reader. This is a minor browser contract cut.
Runtime request/body/job budgets and deadlines may change on each execution;
the worker still validates them before jobs. Port
contention refuses before profile creation; it never chooses another origin.
Missing history refuses without creating a replacement journal. Closing, deadline
or a process loss never authorizes another upload. Keep every original native
setup/observer/attestation claim as well as the browser profile. This does not
prove a profile wasn't rolled back; stale-backup fencing remains required.

Chromium launch/navigation waits and the running context have explicit deadlines;
local file I/O and Playwright process shutdown are not preemptively bounded.
Large-file peak heap/CDP latency and hostile local-user/key-memory isolation are
not qualified. This is a callable process bridge, not a complete noninteractive
publisher: the durable native parent must still retain its keys and original
session location, select the next phase and reconcile uncertainty on restart.
Native intent and profile now retain the immutable joint selection. They do not
discover keys, select the next phase or replace the original phase claims.
The native session's Unreleased
[`--source-session`](../../docs/operator-guide.md#hold-one-validated-batch-across-publication-phases)
retains original setup/verification/transfer paths and rejects lost provenance; it does not
select or recover browser profiles, signer keys or origins. The operator, tenant and verifier still need their original
native identities and journals; no automatic key discovery or paid authority follows.

Run the scoped owned-profile checks with `make test-browser-launcher
BLOB_LAUNCHER_REPORT=NEW_DIRECTORY`. The selected serial verifier journeys run
this bridge through owned PocketIC/HTTPS substitutes and whole-browser restart;
they do not qualify deployed provider behavior or full Miner/public serving.

On the Rust side, `ops::caffeine::preparation::decode_prepared_manifest` converts
the upstream `manifestJSON` into the existing service declaration within explicit
JSON/content/leaf/header limits. It reuses metadata and root checks; it grants no
tenant or uploader authority. The receiving transport must bound buffering too.
The consumer Chromium fixture passes that actual browser output through signed admission
to the existing consumer canister, which retains asset intent and admits as the
tenant. The browser then signs uploader preparation directly to the service before
certificate issuance. Rust supplies/validates opaque Candid using maintained types;
the fixture does not duplicate those schemas in JavaScript. This consumer handshake
is not a production consumer API, and never sends file bytes to the service.
After transfer attempts, the fixture calls the existing consumer registration
handler and verifies that HTTP success cannot publish an unconfirmed asset.
Cancelled cases separately persist consumer cancellation and withdraw tenant
permission. These signed application calls leave the browser journal unchanged;
the exposed reservation remains charged. They do not supply production Toko auth.

The caller supplies an authenticated SDK identity, canonical service/tenant/uploader
principals and a trusted IC root key as a Uint8Array. The client retains the IC
origin and root key in the binding as `icOrigin` and `icRootKey`; a changed trust
context conflicts with the existing intent. The client never discovers
trust from the endpoint. HTTPS is required except for explicit loopback HTTP.
All requests are confined to that origin and the exact service's v4 update or v3
read-state route. Redirects and automatic update fallback are refused.

`binding` contains `service`, `tenant`, `uploader`, `operation`, `root`, `project`,
`bucket`, `permission`
and `key`. Operation is a canonical decimal u128 **string**; root is `sha256:`
followed by 64 lowercase hex characters. Permission is the full original permission's Candid bytes as a
byte array, retained opaquely without reconstructing the Rust contract in JS.
Key is `${service}:${tenant}:${operation}`. The caller must obtain these fields
from the admitted permission. Local binding checks do not grant tenant authority;
the service authenticates and validates the actual issuance request.

Project and bucket are selected before certificate setup and retained immutably in
this same intent. Load them from the reviewed native `certificate-binding.json`;
project must match the installed service and bucket must match the selected provider
namespace. The transfer obtains both from the intent and has no separate namespace
options. A changed project or bucket conflicts before issuance/transfer, including
after reopening the journal. Both are bounded to 256 UTF-8 bytes, without controls,
surrounding whitespace or malformed Unicode; project must additionally be an HTTP
header ByteString. These are representation/binding checks, not provisioning proof.

## Durable intent store contract

The caller supplies async methods below. Rows have `key`, `binding`, `cancelled`
and `phase` (`saved`, `uncertain`, `observed`); dispatched rows additionally retain
the exact signed `envelope` byte array and lowercase hex `requestId`.

| Method | Required atomic behavior |
| --- | --- |
| `save(binding)` | Insert an uncancelled saved row within bounded lifetime capacity, or return the exact existing binding unchanged. Reject conflicts. |
| `inspect(binding)` | Return the retained row, checking the exact binding. |
| `claim(binding, envelope, requestId)` | Check binding and uncancelled saved phase, then persist uncertain phase and exact request in one transaction. Resolve only after commit. |
| `observe(binding, requestId)` | Check binding, uncertain/observed phase and exact retained request, then persist observed phase while preserving cancellation and the envelope. |
| `cancel(binding)` | Check binding and permanently set cancellation, preserving all history and capacity. |

Every method returns the resulting row. Concurrent clients/tabs must serialize
these transitions in the store; checking outside the transaction is insufficient.
No overwrite, tombstone eviction or reset may make a dispatched operation eligible
again. Storage failure must reject before dispatch. The client checks returned rows
but cannot turn an unsafe store into durable storage. The maintained implementation
is `createIndexedDBIntentStore` in `intents.js` (package subpath `./intents`). The
Chromium fixture uses this implementation with two slots and test-only platform
fault injection; it no longer duplicates journal transitions.

Create a journal exactly once with `mode: 'create'`. On subsequent loads, use
`mode: 'open'` with the same database name and capacity. Creation refuses an existing
database; opening refuses a missing one without leaving a new empty database.
Never catch that refusal by creating a replacement or choosing another name for
the same permission. Retain the selected database/profile/origin outside transient
page state and preserve any outstanding obligations when retiring it.

`maxSlots` is an immutable lifetime bound from 1 to 1,000,000. Cancelled and dispatched
rows still occupy slots; capacity is never refunded. The current v1 schema has no
migration, deletion, reset or old-profile activation API. `close()` releases the
connection only. The optional `indexedDB` factory is a trusted platform boundary,
defaulting to the browser's own IndexedDB implementation.

The ceiling does not preallocate rows or qualify a million-row workload. Actual
Chromium evidence covers 675 synthetic lifetime rows across browser restart and
the one-million configuration with one cancelled row. Browser disk/quota,
transaction-count latency and profile durability still require consumer sizing.
Native publication batches remain bounded to 4,096 files; service object capacity
is configured independently. Never reset an exhausted journal to repeat an attempt.

Writes require a `strict` durability transaction and resolve after transaction
completion, never after an individual request succeeds. Binding, phase, gateway
history and capacity checks share that transaction across tabs. The implementation
uses a point lookup and count, never a full-store scan, validates bounded rows and
snapshots caller-owned arguments before storage awaits. Store opening and each
transaction have ten-second local timeouts. Missing/configuration-conflicting/
structurally corrupt records refuse; `IntentRefusal.code` reports local failures,
and platform/storage errors also propagate. Envelope authentication remains the
certificate client's responsibility; the journal does not grant upload authority.

The real Chromium tests cover competing tabs, graceful browser-process restart,
cancelled tombstones, request budgets and corrupt-history refusal. They establish
local IndexedDB behavior in that tested profile, not resilience to power loss,
eviction, profile rollback or hostile same-origin code. Select and review the actual
trial browser/origin, protect its signed envelopes, and arrange persistence and
obligation ownership before effects. A missing journal is a stop condition.

`issue()` persists the signed request before fetch. A failure after that commit
leaves uncertainty, including a lost reply or an unsupported v4 endpoint. Recover
with `recover()` (read-state only) or `observeCertificate(bytes, requestId)` for a
retained response. Both verify certificate signature/delegation/time, exact saved
request, uploader, service, method and root before persisting observation. Responses
are bounded to 256 KiB before decoding, envelopes to 8 KiB and certificates to
128 KiB. An expired or pruned reply leaves uncertainty; it cannot permit reissue.

Observation returns the retained row plus verified `certificate` bytes. It is
historical issuance evidence, not gateway dispatch permission, provider acceptance,
completed storage or asset publication. `cancel()` prevents an unclaimed dispatch;
it cannot recall a request after claim commits. Late observations retain cancellation.
Cancellation does not revoke tenant permission or release service quota. Consumers
must coordinate those separate workflows and must not infer upload permission from
the returned certificate alone.

The store and optional `fetch` implementation are trusted application boundaries.
Same-origin script compromise, profile loss/rollback and browser eviction are outside
these guarantees. `CertificateRefusal.code` identifies local refusals; SDK, network
and storage errors also propagate. Errors never authorize retrying issuance.

Run the journal-only checks with `make test-browser-store` or the actual
Chromium/PocketIC checks with `make test-browser`, after the setup
in [dependency documentation](../../docs/dependencies.md#browser-certificate-evidence).
`make test-browser-standalone` additionally rehearses the complete restricted host,
SDK upload, independent verifier and tenant download against a local gateway
substitute. It includes corrupt bytes and a pre-header connection loss after
the gateway receives the file. Historical certificate recovery preserves the
uncertain gateway claim; verification can establish observed content without
another upload dispatch. The same trusted verifier can reconcile stored bytes
after tenant withdrawal. Explicit reference release ends local liveness; replay
of the accepted attestation leaves the released reference inactive. The fixture
uses the maintained clients, one-slot store and test identities; it supplies
neither production authentication nor live provisioning.
Upload, verifier and tenant reads now use the same TLS HTTP/2 substitute, with
native certificate verification against its explicit fixture CA. Unrelated roots
and a refused stream retain failed observations without implicit read retries.

## Gateway request coordination

`createGatewayTransport` in `gateway.js` (package subpath `./gateway`) supplies the
patched Caffeine client's `fetch` option. Keep `retry: false` and `concurrency: 1`.
It accepts `{ certificate, intents, origin, maxRequests, maxRequestBytes,
maxTotalRequestBytes, fetch? }`;
`certificate` is the existing certificate client and `intents` is the same store.
The selected upload origin must use HTTPS with HTTP/2 or HTTP/3 and a browser
supporting streaming requests. Construction detects missing request-stream support
before issuance; HTTP/1.x negotiation fails without a buffered fallback. Local
Chromium evidence uses a temporary certificate pinned only to owned TLS fixtures.
This hook snapshots
opaque PUT bodies/headers, confines requests to that origin, refuses redirects,
omits cookies and records fingerprints. It neither builds nor interprets Caffeine
trees, chunks, certificates, namespace fields or provider completion responses.
The integrating application still owns the correct bucket/project/provider binding.

The request budget is selected before dispatch: at most 256 requests, 2 MiB per
body, 4096 URL characters and 16 headers totalling at most 4096 name/value characters.
The required `maxTotalRequestBytes` independently caps summed body bytes across
all claims and cannot exceed `maxRequests * maxRequestBytes`. Committed claims
consume this budget even when dispatch or its response is uncertain; it is never
refunded by an HTTP observation. This bounds bodies passed to the guarded fetch
hook, not all network transmissions or provider charges. SDK retries are disabled.
Incoming bodies must be strings or Uint8Arrays; the fingerprinted snapshot is
emitted once through an immediately closed ReadableStream with `duplex: 'half'`.
The optional trusted fetch hook must preserve this stream and must not buffer,
clone, retry or redirect the request. Application headers, method, URL and payload
remain those of the pinned Caffeine SDK; this does not introduce a provider API.

The earlier buffered transport's Chromium 153 diagnostic observes two identical
chunk PUTs from one claim after a pre-header reset. Current stream checks preserve
one uncertain claim and one arrival on owned HTTP/2 connection-close, immediate
data-close and REFUSED_STREAM cuts; buffered fetch, keepalive:false and XHR controls
repeat. The complete standalone journey also recovers the original pre-header
loss without another upload dispatch. This is a tested mitigation, not an
exactly-once wire or replay-charge guarantee: Chromium has an internal stream
replay cache, and other timings, browsers, HTTP/3, intermediaries and the deployed
gateway remain unqualified. See the [repair ledger](../../docs/evidence/caffeine-probes/README.md#browser-replay-repair--2026-10-02).
Anonymous [gateway metadata](../../docs/evidence/caffeine-probes/deployed/2026-10-02-gateway-stream-01/summary.json)
observes HTTP/2 and successful PUT preflights for both SDK endpoints. This advertises
CORS support; it does not establish acceptance of streamed authenticated bodies.
The additional advertised X-Dry-Run header has unqualified semantics and is unused.
Run owned transport checks with `make test-browser-transport
BLOB_BROWSER_TRANSPORT_REPORT=NEW_DIRECTORY`. Responses are bounded to 64 KiB with a
20-second request deadline. The fixture selects two requests, 1 MiB per body and
2 MiB in total.
These are local transport limits, not provider limits or evidence of accepted size.

The store adds two methods to the certificate row, returning the whole resulting row:

| Method | Required atomic behavior |
| --- | --- |
| `claimGateway(binding, scope, owner, index, request)` | Recheck the exact binding, observed certificate and uncancelled state in the same transaction as cancellation. On the first claim, retain `scope` and `owner`; thereafter require both unchanged. Require `index` to equal retained request count below the scope budget, every previous request responded with HTTP 2xx, and no previously claimed URL. Check summed retained `request.bodyBytes` plus this body against `scope.maxTotalRequestBytes`. Append `{ request, phase: 'uncertain' }` and resolve only after durable commit. |
| `observeGateway(binding, scope, owner, index, request, status)` | Match the exact retained binding, scope, owner and last uncertain request. Record `{ request, phase: 'responded', status }`, preserving cancellation and all earlier history. Reject mismatches; this observation does not release capacity or authorize replay. |

`scope` is `{ origin, maxRequests, maxRequestBytes, maxTotalRequestBytes }`. A request records its exact
URL, method, normalized headers, body byte length and SHA-256 fingerprint. Body
bytes and certificates are not copied into this gateway journal. The `owner` is a
random local execution token retained by the store and by that hook instance;
it is not a service identity, secret credential or operational restore authority.
A new tab, reload or recreated hook receives a different token and cannot claim
more requests for that transfer. There is deliberately no automatic resume path.

An aborted claim sends nothing. Once committed, even an abort before fetch leaves
uncertainty. Network/body/observation-write failure also retains that claim. A
complete bounded HTTP response is recorded before Caffeine receives it; non-2xx
responses block further requests. Late responses may update history but cannot
clear cancellation or allow the next request. Local cancellation after a claim
cannot recall its dispatch. Certificate recovery does not change gateway history.

An uncertain upload response does not prove the provider lacks the object. With
the separately approved read budget and configured verifier, use `observe-upload`
on the original permission to check complete bytes, then explicitly submit that
saved statement. This is reconciliation, never a new upload or an automatic
retry. Tenant withdrawal cannot erase an already stored object; if it is later
confirmed, retain its obligations and explicitly release its exact reference.
Consumers must preserve this cleanup intent when an application cancels publication.

The caller must validate and retain these bounded records together with certificate
intent and cancellation. Do not evict, reset or restore an old row to regain an
execution token or capacity. The maintained IndexedDB store demonstrates transactions,
tab loss, reload and graceful browser restart, not power-loss durability, eviction
or rollback recovery in a production environment.
Provider reconciliation, completion, accounting and production application storage
remain required before live use. `GatewayRefusal.code` reports local refusals;
HTTP status alone proves neither stored content nor absence of a paid effect.

`make test-sdk-probe BLOB_SDK_PROBE_REPORT=NEW_DIRECTORY` exercises the pinned
patched SDK with local certificate/gateway/store substitutes, including multiple
chunks, ignored resume hints, non-complete replies, lost final responses and byte
exhaustion. It records synthetic requests/replies and independently verifies bytes
with the Rust download example, including corrupt/truncated rejection. See the
[probe ledger](../../docs/evidence/caffeine-probes/README.md) for evidence and limits.
