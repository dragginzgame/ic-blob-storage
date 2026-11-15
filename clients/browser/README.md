# Minimal Caffeine transfer client

Experimental, unpublished ES module. It runs independently of Canic and IC
Storage. It prepares one immutable file and sends the observed Caffeine tree/chunk
protocol with an existing certificate. No dependency on the provider SDK, patched
package, global fetch interception, wallet, IC endpoint or persistence engine.

From the repository root, with Node 24+ on PATH:

```sh
make test-browser
```

No npm install is needed: the module uses standard Web Crypto, Fetch and Streams.
The tests use Node's Web APIs, retained first-party protocol vectors and an owned
loopback HTTP server. They do not qualify a deployed browser or Caffeine gateway.
`make test-callbacks` adds the real installed reference application: the same
six-chunk declaration goes through authorization, proof recovery, transfer,
verified read and catalog publication. Actual connection cuts and client-process
termination retain unpublished drafts; restarting cannot replay the transfer. See the
[composition evidence](../../docs/design/reference-application.md#transfer-and-verified-read-composition--development-01009).

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

This is integration pseudocode: application authorization, certificate
authentication and `persistOriginalTransferClaim` are required caller capabilities,
not supplied stand-ins. The native `blob` commands retain and authenticate original
IC request results; this module does not parse or authenticate certificates. A
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

For native/Node consumers, [`blob claim-transfer`](../../docs/operations/storage-inspection.md#claim-one-transfer-offline)
supplies this custody using the original CLI request directory. Pass the frozen
plan's certificate digest and provider selectors, the exact proof file, and the
original issuer/uploader/root/trust inputs. Return true only after process success.
It re-authenticates the saved proof before permanently claiming the root's transfer;
even a partial claim prevents another process or gateway selection from replaying.
The installed fixture exercises this real adapter, including a fresh-process refusal.
Browser applications still supply their own durable claim implementation; they
cannot run the native CLI. Application permission and cancellation remain caller
responsibilities: an authentic historical proof does not grant current permission.

This explicit callback works around the current Storage rewrite by keeping
transfer preparation independent of canister persistence. It is not a new storage
adapter or an alternate ledger. The existing live enrollment probe remains
unchanged; this client does not provide its upgrade/migration path.

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
