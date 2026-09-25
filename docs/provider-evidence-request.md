# Provider evidence needed to freeze B1

Target: latest official Caffeine integration pinned in
[provider-baseline.json](provider-baseline.json). This is a local request
checklist, not a message sent to the provider or permission for paid operations.
The public integration sources establish client behavior; the following facts
need an authoritative server contract and deployment evidence.

| Area | Exact information needed | Why it gates this service |
| --- | --- | --- |
| Deployment | Intended gateway origin, network, Cashier principal, account/project/bucket namespace and accountable operator; exact server/interface version or source revision | Bind credentials, callbacks, paid effects and observations to the same provider installation |
| Wire contract | Current gateway HTTP schema and error/status definitions; current Cashier Candid including balance, gateway list, top-up and settlement; required callback Candid | Resolve the observed differences from Canic and verify full balance/readiness/funding functionality |
| Upload identity | Which fields define the exact paid operation, when charging occurs, how duplicate requests are handled, and which callers/instances can use the namespace | Prevent retry, stale-instance and older-backup identity reuse |
| Completion | Authoritative upload/transfer result lookup, incomplete-object behavior and retained receipt fields; distinguish durable completion from HTTP success | Recover lost responses without a second uncertain charge |
| Retention | Numeric provider evidence-retention bounds and behavior after expiry; evidence available after local backup restoration | Freeze supported receipt/retry/restore horizons and non-repeat protection |
| Deletion | Who initiates deletion, callback authority, object-specific authoritative completion, duplicate/stale callback semantics and bounded reconciliation | Prevent deleting live references and prematurely releasing physical capacity |
| Billing stop | Object/operation-specific final charges and evidence that future charges stop; account minima and charges surviving object removal | Keep logical release, physical deletion and financial completion distinct |
| Funding | Exact transfer identity and completion lookup after lost responses; fees, accepted/refunded cycle amounts and residual-balance return/disposition | Replace Canic's transient-only funding lock with a provable durable workflow |
| Restore | An identity/lease/evidence authority that survives the supported backup boundary, concurrent-instance exclusion, and required reconciliation inventory | Fence stale accounting and recover all outstanding effects before admitting work |
| Serving | Hash vectors including empty data and metadata, size/chunk/session limits, MIME/disposition behavior, active-content isolation and public URL access semantics | Bound the client and service contract without inventing byte privacy or provider behavior |

For every answer retain its source URL/revision or deployed interface hash,
collection date, exact operation/namespace, observed result and limitations.
Evidence from official integration code, server source, deployed observations
and PocketIC substitutes must be distinguished. A method's presence in Candid
does not prove its idempotency, retention or economics.

The older Canic Cashier principal and Toko consumer evidence are historical
references, not confirmation of the intended current deployment. Do not issue
top-ups, uploads or deletion experiments against a guessed deployment. Read-only
interface inspection can continue once its identity is known; effectful provider
qualification needs its own explicit authority and bounded test resources.

No missing field can be closed by adding local retry logic. If the provider
cannot support a required capability, record a supported narrower contract or
another provider decision. Under the maintainer's parity requirement, dropping
an existing capability also leaves Canic removal blocked until explicitly
resolved.
