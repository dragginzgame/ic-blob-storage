# Minimal Caffeine transfer client

Experimental, unpublished ES module. It runs independently of Canic and IC
Storage. It prepares one immutable file and sends the observed Caffeine tree/chunk
protocol with an existing certificate. The byte-transfer entry point uses only
Web APIs. Optional proof/custody modules
use published Platform verification and IndexedDB; no provider SDK patch, wallet,
signing, IC call or automatic certificate acquisition is introduced.

From the repository root, with Node 24+ on PATH:

```sh
make install-browser-tools
make test-browser
```

Setup runs locked `npm ci --ignore-scripts`: Platform 0.2.0 supplies IC verification;
esbuild 0.28.2 bundles the browser qualification entry point. Both were the latest
published stable versions at selection. Normal tests never install dependencies.
The byte-transfer entry point itself uses standard Web Crypto, Fetch and Streams.
The tests use Node's Web APIs, retained first-party protocol vectors and an owned
loopback HTTP server. They do not qualify a deployed browser or Caffeine gateway.
`make test-callbacks` adds the real installed reference application: the same
six-chunk declaration goes through authorization, proof recovery, transfer,
verified read and catalog publication. Actual connection cuts and client-process
termination retain unpublished drafts; restarting cannot replay the transfer. See the
[composition evidence](../../docs/design/reference-application.md#transfer-and-verified-read-composition--development-01009).

For actual Chromium transport qualification, supply an installed executable explicitly:

```sh
make test-browser-runtime BROWSER=/absolute/path/to/chrome
```

This launches a separate disposable profile, serves the real ES module and reads
from a second loopback origin with CORS. It checks the original six-chunk vector,
input mutation, read corruption/truncation/excess, partial HTTP status, refused
redirects, missing CORS permission, a stream deadline and pre-cancellation. The peer
counts requests to detect retries or redirect following. Diagnostics stay under
`target/browser-runtime-*/`; profiles are removed after the run, including failures.
It also authenticates a retained local PocketIC certificate against its original
request, uploader, issuer and root, then sends one tree and six chunks. The shipped
experimental custody module commits an IndexedDB claim with strict durability
before returning permission to send. Missing custody, changed provider binding,
corrupt or altered proof, replacement provisioning and concurrent claims refuse.
The peer checks CORS preflight, serial PUTs (including a held
chunk reply), exact received bytes and readback. Three isolated runs cover successful
acknowledgments, a broken final HTTP reply after all bytes arrive, and caller cancellation
while the first chunk reply is held. Failures report possible exposure, the interrupted
chunk index and only prior acknowledgments. The cancellation barrier waits for actual
peer receipt; abort must close that request and no later chunk may arrive.
A full page reload recreates the client; the retained claim refuses a second
transfer before any PUT. A separate explicit GET then verifies received bytes,
including after the uncertain final reply; incomplete cancelled bytes must refuse.
No automatic upload retry or publication follows. Each run tests one origin/profile,
not eviction, stale-profile restore, power loss or multi-device coordination.
The retained public proof and local trust root cannot authorize mainnet. This
optional target requires Node 24+ and the explicit npm setup above; it installs no
browser. Native macOS execution remains unqualified.
Chromium 156.0.8078.4 passed locally for development 0.100.10. This is browser
execution against a controlled HTTP peer, not a deployed application or evidence
of Caffeine upload admission or serving behavior. Separate bodyless OPTIONS calls
to its two public upload routes returned HTTP 200 and advertised PUT, wildcard
origin and the required headers; see the [provider observation](../../docs/design/caffeine-observation.json).

## Application boundary

```js
import { prepare } from "./clients/browser/src/index.js";

const prepared = await prepare(fileBytes, { contentType: "image/png" });
// Authorize this exact declaration through the application, then acquire/recover
// and authenticate its original certificate using the selected IC client.
const declaration = prepared.declaration;
// declaration: { root, bytes, chunks: ["sha256:..."], headers: ["Name: value"] }

const result = await prepared.upload({
  gateway, owner, project, bucket,
  certificate: { root: declaration.root, bytes: authenticatedCertificate },
  claim: persistOriginalTransferClaim,
  signal: cancellation.signal,
});
// result.outcome === "http-acknowledged"; not availability or publication approval.
```

This is integration pseudocode: application authorization and
`persistOriginalTransferClaim` must be real capabilities. The byte-transfer module
does not parse certificates. The opt-in browser adapter below supplies historical
proof authentication and a bounded claim store; native `blob` commands separately
retain and authenticate original IC request results. A
matching `certificate.root` is an input binding, not cryptographic proof. The
caller must independently bind the actual proof to its issuer, root, uploader and
original request before passing it here. Keep the proof private and retained.

`claim(plan, signal)` must atomically and durably claim the sole attempt for the
selected object/binding before returning **true**. The frozen plan contains the
gateway origin, owner/project/bucket, full immutable declaration and certificate
SHA-256. The application supplies its already-known grant/reference identity.
Refuse an existing or uncertain claim, including a fresh prepared handle after a
restart. Retain the original proof/intent and unresolved outcome outside the
ephemeral handle. Do not use an in-memory set or a no-op returning true as durable
custody. Cancellation may interrupt waiting for a claim that later commits; that
claim stays consumed and no late network dispatch follows.

For native/Node consumers, [`blob transfer claim`](../../docs/operations/storage-inspection.md#claim-one-transfer-offline)
supplies this custody using the original CLI request directory. Pass the frozen
plan's certificate digest and provider selectors, the exact proof file, and the
original issuer/uploader/root/trust inputs. Return true only after process success.
It re-authenticates the saved proof before permanently claiming the root's transfer;
even a partial claim prevents another process or gateway selection from replaying.
The installed fixture exercises this real adapter, including a fresh-process refusal.
Browser applications can select the experimental adapter below; they cannot run
the native CLI. Application permission and cancellation remain caller responsibilities: an authentic historical proof does not grant current permission.

This explicit callback works around the current Storage rewrite by keeping
transfer preparation independent of canister persistence. It is not a new storage
adapter or an alternate ledger. The existing live enrollment probe remains
unchanged; this client does not provide its upgrade/migration path.

## Experimental browser proof and custody

Select one trusted HTTPS application origin/profile and a fixed issuer, trust root,
gateway, project and bucket. Bundle the npm entry points with the application.
Provision only a deliberately fresh installation, never in response to a failed
upload or missing browser data:

```js
import { initializeCustody, createTransferClaim } from "./clients/browser/src/custody.js";

// One explicit provisioning action, outside upload/recovery flows:
await initializeCustody({ issuer, rootKey, gateway, project, bucket });

// Later, after current application permission is checked:
const custody = await createTransferClaim({
  issuer, uploader, root: prepared.declaration.root, rootKey,
  request: originalSignedRequestBytes, certificate: originalCertificateBytes,
  gateway, project, bucket,
});
const result = await prepared.upload({ gateway, owner: issuer, project, bucket,
  ...custody, signal: cancellation.signal });
```

Platform authenticates the certified reply and delegation against caller-selected
trust bytes. The request content identifies the certified request ID; this is not
separate verification of the ingress envelope's outer signature. The method,
argument root, uploader and issuer must match, and the certified reply must say
`upload` for that root. Certificate time checking is deliberately disabled for
historical recovery. This proves an original reply, never current application
permission, provider acceptance or certificate lifetime. No IC network call occurs.
Input bounds are 8 KiB request, 128 KiB certificate and 1 KiB trust root, copied
before awaits. These are byte limits, not a general decoder allocation guarantee.

The fixed IndexedDB database retains at most **64 permanent root claims**, each
with original request/proof and its authenticated identities. One read/write
transaction checks the installation binding and capacity before inserting the
claim; only its strict-durability completion permits transfer. Each root gets one
attempt across upload IDs and uploaders. There is no eviction, release, retry,
compaction or reset API. Exhaustion refuses new transfers; existing claims remain.
Changing configuration cannot create a second namespace in the same database.
Cancellation after commit consumes the claim even if no PUT follows.

Missing, blocked, incompatible or changed storage refuses. Browser eviction,
manual clearing, rollback and a second profile/device cannot be detected or
coordinated by this local store: preserve the installation and stop if continuity
is unknown; never initialize a replacement as recovery. Application login,
current reference permission and catalog decisions remain outside this adapter.
API and persistence are experimental and unfrozen. A deployed application's
custody/continuity arrangement still needs qualification.

## Exact limits and outcomes

- Preparation copies a non-shared `Uint8Array` before its first await. Empty files
  refuse. Maximum size is 32 MiB; `maxBytes` can lower it. Chunks are fixed at
  1 MiB. This is a bounded in-memory pilot, not an unbounded large-file uploader.
- Metadata contains derived `Content-Length` and explicit `Content-Type`, default
  `application/octet-stream`. MIME is a bounded ASCII type/subtype without
  parameters. No sniffing, filename inference or Content-Disposition is added.
- The immutable declaration is available before authorization. Prepared bytes,
  tree and copied certificate cannot be changed through input mutation.
- One prepared handle permits one attempt, with one in-flight PUT at a time:
  tree first, then chunks in order. Applications serialize separate files as
  needed. No batch scheduler, automatic retry, range resume or fresh issuance.
- HTTPS origins are accepted; HTTP is restricted to loopback for local testing.
  Redirects are refused and ambient credentials omitted. A custom `fetch` is a
  trusted transport that must honor these options and must not add retries.
  The client makes one Fetch invocation per request; underlying browser/transport
  behavior is not an exactly-once HTTP delivery guarantee.
- Total upload deadline defaults to 120 seconds, including the claim; callers
  may lower `timeoutMs`. Reply bodies are bounded to 4 KiB. These are client
  limits, not an enforced provider storage or monetary cap.
- `TransferError` reports `stage` (`claim`, `tree`, `chunk`), `chunkIndex`,
  `acknowledged` HTTP replies and `possibleExposure`. That flag concerns this
  attempt's PUT invocation, not authority already issued or provider acceptance.
  Any stopped attempt consumes the handle. Preserve custody; do not reprepare to
  retry. An abort cannot retract an already dispatched PUT.
- A successful result records HTTP acknowledgments only. It does not interpret
  `blob_complete` as proof of persistence. The application must separately verify
  a successful read against its trusted declaration before catalog publication.
  Verification of one read is available below; catalog publication remains with
  the application.

## Verify a read before using its bytes

```js
import { readVerified } from "./clients/browser/src/index.js";

// The application selects the URL and admits any download cost. Use its original
// trusted declaration, never a replacement supplied by the downloaded response.
const signal = AbortSignal.timeout(30_000);
const verified = await readVerified(selectedReadUrl, declaration, { signal });
// verified.blob contains immutable bytes; verified.root matches declaration.root.
```

`readVerified` snapshots the original declaration and URL before its single GET.
It omits credentials, refuses redirects, and uses one deadline across fetching and
verification. URLs are limited to 4,096 characters, HTTPS or owned loopback HTTP,
without embedded credentials or fragments. A custom `fetch` must honor these
transport options and must not retry. This is one Fetch invocation, not a promise
about underlying transport retransmissions. The caller selects the existing object
URL; this module does not discover provider accounts or derive serving routes.

Both `readVerified` and the lower-level `verifyRead(response, declaration, options)`
require an unredirected HTTP 200 body, exact declared length and EOF,
then recomputes the root with the original metadata. It returns no usable file on
corruption, truncation, trailing bytes, interrupted streams or cancellation.
Response headers cannot redefine the expected MIME or length. The same 32 MiB
in-memory ceiling applies; the deadline defaults to 120 seconds and `timeoutMs`
can lower it. With lower-level `verifyRead`, the deadline starts at verification;
its caller must also bound the preceding fetch.
The returned immutable Blob establishes this read's integrity, not future
availability, current references, application publication or provider settlement.
Only after successful verification should the client ask the application to
publish the original draft. The application rechecks its current reference and
cancellation state. A valid late read does not override cancellation. Commit the
application's cancellation and abort local work separately; neither recalls an
already issued certificate or dispatched PUT.

### Reconcile an uncertain transfer

Inspect the original draft as its owning user and retain its original root, length
and metadata. If the application still permits publication, explicitly read the
original object with `readVerified`. It needs no certificate, signer, upload handle
or custody path. Successful verification can precede a separate `app_publish` for
that same draft; the application checks current references and cancellation again.
“Read-only” describes the provider operation: publication is a separate application
mutation. Neither operation clears the original uncertain transfer claim or proves
settlement. Failure leaves the draft unpublished; do not issue another certificate,
rebuild an upload attempt, or reset custody to repair it.

The installed fixture starts a fresh reader process against the bytes already
received before the lost upload reply. It sends one GET and no PUT, checks the saved
draft first, and covers successful publication, corrupt content and cancellation
during readback. Download cost and serving behavior remain unqualified for Caffeine;
this is an explicitly requested read, not an automatic recovery loop.

The host still accounts declared bytes and retains outstanding exposure. Provider
proof acceptance, namespace/deletion scope, delayed writes, charge cutoff and
closure remain unqualified. No live upload has been made with this client.

## Provenance

This is independently authored code for the external request/hash profile
documented in the [contract review](../../docs/design/caffeine-contract-review.md#candidate-protocol-what-actually-crosses-the-boundary).
The author inspected published SDK 1.1.2 interfaces/source; this is not a clean-room
claim. No SDK implementation, legacy patch or third-party test was copied.
Root expectations are the existing first-party frozen observations also used by
the [Rust verifier tests](../../crates/ic-blob-storage/src/caffeine/tests.rs).
The runtime fixture's commit-before-dispatch rule follows the inspected legacy
[IndexedDB owner](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/clients/browser/intents.js):
transaction completion admits effects; request success alone does not. Only this
invariant is retained, independently exercised here without its journal architecture.
