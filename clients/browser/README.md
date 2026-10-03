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
reverify the full batch/setup report and select the original file; this helper
accepts one body and owns no ID allocation or setup journal.

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
`manifest.json`. Preserve original preparation hints, including omitted filename;
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
