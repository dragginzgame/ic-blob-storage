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
Application authentication, durable storage and deployed Caffeine qualification
are still integration requirements.

```js
import { createCertificateClient } from './certificate.js';

const client = await createCertificateClient({
  host, identity, rootKey, binding, intents,
});
// An explicit first attempt; throws if already dispatched or cancelled.
const observation = await client.issue();
// After an uncertain response, use client.recover(), never issue again.
```

For Caffeine's upload path, pass `client.certificateAgent` as its existing agent
constructor argument. This is a real SDK HttpAgent with its public call boundary
guarded by the same intent persistence and verification. It refuses a cancelled
observation before handing the response to Caffeine. Caffeine continues to own
certificate extraction and gateway requests. Use the patch's static preparation
before service admission, then its `uploadPrepared` with explicit per-client
transport, cancellation and disabled retries. The local composition is tested;
production consumer integration and gateway-effect journaling are still open.

On the Rust side, `ops::caffeine::preparation::decode_prepared_manifest` converts
the upstream `manifestJSON` into the existing service declaration within explicit
JSON/content/leaf/header limits. It reuses metadata and root checks; it grants no
tenant or uploader authority. The receiving transport must bound buffering too.
The Chromium fixture passes that actual browser output through tenant admission
and uploader preparation in PocketIC before returning the permission. This local
handshake is not a production consumer API, and never sends file bytes to the service.

The caller supplies an authenticated SDK identity, canonical service/tenant/uploader
principals and a trusted IC root key as a Uint8Array. The client retains the IC
origin and root key in the binding as `icOrigin` and `icRootKey`; a changed trust
context conflicts with the existing intent. The client never discovers
trust from the endpoint. HTTPS is required except for explicit loopback HTTP.
All requests are confined to that origin and the exact service's v4 update or v3
read-state route. Redirects and automatic update fallback are refused.

`binding` contains `service`, `tenant`, `uploader`, `operation`, `root`, `permission`
and `key`. Operation is a canonical decimal u128 **string**; root is `sha256:`
followed by 64 lowercase hex characters. Permission is the full original permission's Candid bytes as a
byte array, retained opaquely without reconstructing the Rust contract in JS.
Key is `${service}:${tenant}:${operation}`. The caller must obtain these fields
from the admitted permission. Local binding checks do not grant tenant authority;
the service authenticates and validates the actual issuance request.

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
but cannot turn an unsafe store into durable storage. The two-slot IndexedDB store
in tests/browser is a fixture, not a production implementation or sizing decision.

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

Run the actual Chromium/PocketIC checks with `make test-browser`, after the setup
in [dependency documentation](../../docs/dependencies.md#browser-certificate-evidence).
