# Caffeine provider review — 2026-09-26

Verdict: Caffeine remains unqualified for the required service journey. The
Cashier's deployed Candid and public gateway/pricing queries are now observed;
server revision, paid-effect recovery and final billing guarantees remain open.
Local response decoding and immutable root claims now address the source-level
false-success/reassociation paths; their production prerequisites remain below.

## Selected integration baseline

The maintainer requires this service to target the latest official Caffeine
integration rather than inherit Canic's potentially drifted bindings.
[The baseline record](provider-baseline.json) pins the verified target and
separates release/source observations from deployed-provider qualification.

On 2026-09-25 the [npm registry](https://registry.npmjs.org/@caffeineai%2fobject-storage)
reports `latest` as `@caffeineai/object-storage` 1.1.2, published
2026-09-17. The downloaded package archive passes its registry SHA-512 integrity
check. Its `dist/StorageClient.js` SHA-256 exactly matches the upstream artifact
used for the retained client observations. Those observations therefore also
apply to this file in the latest published package, including the empty-tree
failure; they do not establish whole-package or server behavior.

At this checkpoint, official `main` resolved to the commit below. Its backend
manifest declares `caffeineai-object-storage` 1.1.1. A subsequent anonymous query to the
official Mops registry confirms 1.1.1 as its highest published version. The
registry's hashes for `mops.toml`, `src/Mixin.mo` and `src/Storage.mo` match the
pinned official source. See [the retained registry evidence](evidence/caffeine-mops-verification.json).
The Rust service does not install the Motoko package. Gateway/Cashier deployment versions remain
unverified and must not be inferred from either package number.

The registry address and query schema come from
[Mops network configuration](https://github.com/caffeinelabs/mops/blob/ad36ae3b51616b3c39140674c19ad36481301f59/cli/api/network.ts)
and [its Candid interface](https://github.com/caffeinelabs/mops/blob/ad36ae3b51616b3c39140674c19ad36481301f59/cli/declarations/main/main.did).
Reproduction uses ICP CLI 1.6.0 with that downloaded interface:

```sh
icp canister call oknww-riaaa-aaaam-qaf6a-cai getHighestVersion \
  '("caffeineai-object-storage")' --query --identity anonymous \
  --network https://icp-api.io --root-key mainnet \
  --candid /path/to/main.did --output candid
icp canister call oknww-riaaa-aaaam-qaf6a-cai getFileHashesQuery \
  '("caffeineai-object-storage", "1.1.1")' --query --identity anonymous \
  --network https://icp-api.io --root-key mainnet \
  --candid /path/to/main.did --output candid
```

Retained query responses are observations, not portable certified-state proofs.
The checks issue no update calls or paid provider operations. This closes the
backend publication-version gap, not any deployed-provider safety requirement.

Before provider implementation and qualification, recheck the official source
and registry latest, and update this exact baseline plus affected evidence
together if upstream changed. Builds must use reviewed pins, not resolve a
moving `latest` tag. Do not reproduce superseded Canic bindings or add fallback
variants to bridge the differences. Service protocol/state generations stay
v1; upstream package versions are separate identifiers.

## Provenance

Reviewed public repository: `caffeinelabs/skills`, commit
`e5cacdfe5ce55e939edb02980fca800c0c13f421`.

- [Backend Mixin.mo](https://github.com/caffeinelabs/skills/blob/e5cacdfe5ce55e939edb02980fca800c0c13f421/packages/object-storage/backend/src/Mixin.mo)
  and [Storage.mo](https://github.com/caffeinelabs/skills/blob/e5cacdfe5ce55e939edb02980fca800c0c13f421/packages/object-storage/backend/src/Storage.mo).
- [Backend manifest](https://github.com/caffeinelabs/skills/blob/e5cacdfe5ce55e939edb02980fca800c0c13f421/packages/object-storage/backend/mops.toml):
  `caffeineai-object-storage` 1.1.1, Motoko 1.7.0.
- [Frontend source](https://github.com/caffeinelabs/skills/blob/e5cacdfe5ce55e939edb02980fca800c0c13f421/packages/object-storage/frontend/src/StorageClient.ts),
  [JavaScript artifact](https://github.com/caffeinelabs/skills/blob/e5cacdfe5ce55e939edb02980fca800c0c13f421/packages/object-storage/frontend/dist/StorageClient.js)
  and [manifest](https://github.com/caffeinelabs/skills/blob/e5cacdfe5ce55e939edb02980fca800c0c13f421/packages/object-storage/frontend/package.json):
  `@caffeineai/object-storage` 1.1.2.

These are official application integration/client sources, not the gateway or
Cashier server implementation. That source inspection installed no package,
edited no upstream repository and issued no provider request. The repository's skill files
were not installed or adopted as instructions.

## Differences from Canic's captured contract

Comparison baseline: the Canic revision and Candid/source hashes recorded in
[Canic parity](canic-parity.md#source-checkpoint).

| Surface | Canic's reviewed source/snapshot | Newly reviewed official integration |
| --- | --- | --- |
| Deletion listing | `_immutableObjectStorageBlobsToDelete` returns `vec text` | Returns `[Blob]`, equivalent to `vec blob`; caps the returned dead-blob slice at 10,000 |
| Gateway registry | `storage_gateway_principal_list_v1` | `storage_gateway_list_v1` |
| Cashier top-up | Optional record with optional account/target balance; structured result | Required `{ account : Principal }`; empty result |
| Refill callback | `_immutableObjectStorageFundFromProjectCycles` emitted by Canic billing adapter | `_immutableObjectStorageRefillCashier`, authorized against the configured Cashier principal |
| Liveness/deletion state | Explicit stable root/pending records | Motoko runtime `Prim.isStorageBlobLive`, `Prim.getDeadBlobs`, `Prim.pruneConfirmedDeadBlobs` and GC |

These are differences between inspected sources, not necessarily incompatible
wire contracts. The deployed observations below resolve part of that question.
Use one selected contract, without compatibility fallbacks. Motoko's blob/GC
integration still is not a Rust lifecycle implementation that can simply be copied.

## Toko locator and deployed Cashier observations

Toko's indexed `development` commit `6519b72d2a420564dabaf700fc55f7b8603d9fd3`
defaults to `https://blob.caffeine.ai` and Cashier
`72ch2-fiaaa-aaaar-qbsvq-cai`; its backend delegates to Canic. These are source
defaults, not verified deployment overrides or this service's account selection.
Exact source links, blob hashes, commands and observations are retained in
[deployment evidence](evidence/caffeine-deployment-observation.json).

Anonymous metadata retrieval obtained the [deployed Candid](evidence/caffeine-cashier.did).
Both gateway-list names are advertised as queries. `account_balance_get_v1` is
also a query; top-up retains Canic's optional request and structured result.
The newer `storage_gateway_list_v1` and `pricelist_v1` queries both succeeded.
No private account lookup, update or payment was performed.

`didc` confirms that the deployed top-up function is a subtype of the newer
Motoko wrapper's required-record/empty-result declaration. Thus that difference
does not establish a wire break; the wrapper discards structured result data.
Whole-service subtyping detects different query/update annotations, but the IC
supports calling query methods through replicated calls; see
[replicated query guidance](https://docs.internetcomputer.org/guides/security/data-integrity-and-authenticity/).
Neither check proves a successful funding operation or safe retry.

The interface also advertises audit-log and usage-ledger queries; their presence
does not establish retention or operation-specific reconciliation. The gateway
responded to an HTTP HEAD request with 400, which establishes reachability only.
Server source/version, upload completion, deletion and billing-stop evidence
remain unresolved. Retained query text is not a portable certified-state proof.

## Client algorithm and completion observations

The client uses 1 MiB chunks, domain-separated SHA-256 chunk/metadata/node
hashes, a binary tree and metadata-dependent roots. It sends a certificate with
the tree, uploads chunks, and constructs a direct read URL. Its visible request
bindings include owner, project, bucket, root and chunk/index. These bindings
do not establish provider-side deduplication, operation identity or receipt
retention across a restored service.

`uploadChunk` decodes a completion flag, but `parallelUpload` discards that
flag. `putFile` returns the root after the chunk requests finish. That client
behavior supplies no independent authoritative completion lookup. Its HTTP
retry loop is not proof that uncertain paid operations are safe to repeat.
`getDirectURL` constructs a URL; it performs no byte verification.

An isolated check executed the pinned artifact's hashing classes with Node
v18.19.1 and WebCrypto, excluding imports, the gateway client and network access.
[Recorded observations](evidence/caffeine-client-observations.json) include
the artifact hash, input and output vectors:

- The provider root for `abc` differs from its raw SHA-256 digest.
- Changing only content-type metadata changes the root for those same bytes.
- Building an empty tree rejects: its empty-input branch encodes hexadecimal
  text as 64 UTF-8 bytes and supplies that to a constructor requiring 32 bytes.

This checks the pinned client algorithm only. It is not a provider test or a
service implementation. Empty-object support needs a verified provider vector
or an explicit service exclusion before A02 can be frozen. Do not copy the
client's empty-input failure into the service as the intended hash contract.

For reproduction, retrieve the linked JavaScript at the pinned commit, verify
its SHA-256 against the JSON, and evaluate the slice beginning at
`const MAXIMUM_CONCURRENT_UPLOADS` and ending before
`class StorageGatewayClient` in a fresh Node VM exposing only WebCrypto,
`TextEncoder` and `Uint8Array`. Use `YHash.fromChunk` and
`BlobHashTree.build` with the JSON input, no headers, or `Content-Length: 3`
plus the recorded content types. The empty case uses no chunks or headers.
This bounded experiment adds no maintained JavaScript tooling to the project.

## Recovery findings — 2026-09-26

[Source-bound probes and reproduction details](evidence/caffeine-recovery-review.json)
now turn the earlier concerns into specific integration constraints. Official
GitHub `main` still resolves to the pinned commit; npm still reports 1.1.2 with
the same integrity value. A subsequent anonymous refresh on 2026-09-26 confirmed
Mops still reports 1.1.1 and Cashier's Candid has the identical retained SHA-256.
This refresh did not call top-up, upload or deletion methods.

| Path | Finding | Required integration behavior |
| --- | --- | --- |
| Upload completion | Running the pinned client with substituted HTTP responses returns the same root and 100% progress for both `blob_complete` and an invented non-complete status | Neither the returned hash nor progress may mark an upload confirmed. Obtain authoritative completion evidence and a lost-response lookup contract |
| Funding outcome | A synthesized Cashier `Err(TopUpWithoutCycles)` decodes successfully as the wrapper's empty result; the inspected wrapper reports success and the offered amount after its await | Preserve typed provider outcomes; establish accepted/refunded amounts separately. Successful transport/decoding does not prove successful credit |
| Deletion identity | The official callback supplies only root blobs and authenticates gateway membership; it supplies no object incarnation or operation ID | Never attach the current local incarnation to an otherwise ambiguous callback. Qualify namespace/root association and delayed callback handling first |
| Funding reconciliation | Direct top-up has no typed caller operation ID; the audit query exposes CSV with no column or retention contract in Candid | Balance changes and method presence cannot resolve a specific uncertain payment. Obtain exact server correlation and retention evidence before automatic retry |

The callback constraint matters even with perfect local binding checks: upload
root R, release it, then upload the same root under a newer incarnation. An old
confirmation for R can be indistinguishable from a new one. Blocking live-object
deletion alone is insufficient if the new incarnation is also deletion-pending.
Permanent root non-reuse in an exclusive namespace is one candidate restriction,
and the local claim model now enforces non-reassignment across a service's entire
root history. Its use in a deployed provider contract still requires bounded
durable history that survives the supported restore boundary and verified
namespace exclusivity.

The advertised cycles-ledger deposit route returns a block index and credited
amount. It is worth investigating, but a lost sweep/credit response still needs
server evidence linking that transfer to the credited account exactly once.
Switching payment routes alone would not close the recovery gate.

These findings qualify source/client behavior only. The HTTP/certificate inputs
and Cashier error were substitutes, not live provider outcomes. They identify
the next server-contract questions without proving the provider cannot meet them.

### Local fixes and remaining provider evidence

The maintainer requested resolution of these findings. The library now owns
bounded [response decoders](../crates/ic-blob-storage/src/ops/caffeine/mod.rs):
JSON chunk status never treats progress or a hash as completion, and the complete
Cashier result retains all four advertised error categories. Missing, malformed,
unknown-variant or over-budget funding replies reject; they never become success
or proof of a refund. A valid `Ok` yields a validated balance report with no
invented credited amount. Private provider DTOs prevent clients from becoming
an independent owner of the wire contract. `CompletionReported` is deliberately
an observation; it cannot itself construct a confirmed object.

[Immutable root claims](../crates/ic-blob-storage/src/model/lifecycle/roots/mod.rs)
bind a root to its original service/tenant/namespace/object/incarnation. Exact
claim replay does not authorize another upload, and settlement never frees that
root for a newer object. Native composition exercises a delayed confirmation
against a newer deletion-pending incarnation. The
[core evidence](evidence/core-primitives.md#provider-response-and-root-correlation-fixes)
records tests and independent Candid fixture provenance.

These close the local parsing and reassignment defects. Authoritative completion
lookup, server-side retry/retention behavior, namespace exclusivity, durable
claim/intent storage and final billing evidence remain unresolved. No transport,
endpoint or upstream package was changed; do not describe the existing Caffeine
client as patched. No extra HTTP success check or local map can establish those
remaining provider facts. The service still cannot execute the full journey.

## Serving and deletion evidence

Caffeine's [storage overview](https://help.caffeine.ai/hc/en-us/articles/46899827144852-How-File-Storage-Works-in-Your-App)
describes browser-to-provider upload, direct file delivery and background
cleanup after reference removal. Its
[troubleshooting documentation](https://help.caffeine.ai/hc/en-us/articles/46899784111380-File-Storage-Troubleshooting)
explicitly says URL holders can retrieve files without a delivery-layer login
check, signed/expiring URLs are unsupported, and deletion is asynchronous.

Consequently the first proposed consumer journey should use public assets.
Tenant authorization must still protect mutations and service metadata, but
must not be described as restricting access to publicly served bytes. This
proposal does not select a consumer or authorize changes to it. MIME and
active-content serving still need separate qualification.

Neither document supplies operation-specific deletion receipts, evidence
retention, final-charge settlement or billing-cessation proof. Waiting for a
documented cleanup interval is not authoritative evidence of those outcomes.

## Implementation gate and next evidence

The follow-up public-source search inspected Caffeine Labs' public repository
listing and the pinned integration repository's storage-related tree. It found
no standalone gateway or Cashier server implementation in that search scope.
That is a bounded search result, not proof that no authoritative source exists.

Caffeine's [File Storage Costs](https://help.caffeine.ai/hc/en-us/articles/49362898986644-File-Storage-Costs)
adds relevant economic context: uploads, ongoing storage and downloads are
charged separately; deleted files stop contributing to daily storage charges
from the next billing day; existing files can continue being served and accrue
storage charges after credits run out. This rules out treating an empty balance
as proof that obligations ended. The article does not define a machine-readable
final-charge/deletion receipt or identify the Cashier cycle-account contract.
Do not map retail credits to canister cycles or invent retention guarantees from
that documentation.

## Independent deployment support

On 2026-09-26, Caffeine's official
[GitHub export guidance](https://help.caffeine.ai/hc/en-us/articles/46899843980692-GitHub-Integration-Overview)
(page updated 2026-09-10) states that its file-storage integration depends on
managed Caffeine infrastructure and independently deployed apps need another
storage solution. This is platform guidance, not a gateway protocol specification
or evidence that a separately arranged integration cannot work. Toko's checked
source defaults do not establish its onboarding/support arrangement.

The resulting open question is whether Caffeine supports an independently
deployed Rust service under the selected account/project/bucket, and how that
namespace, billing and callback authority are provisioned. Public client packages,
compatible Candid and reachable query endpoints do not answer it. Obtain the
supported deployment contract alongside the operation/recovery evidence below.
Do not change provider selection or claim technical impossibility from this
general guidance alone. A provider contact/source-access question is pending;
no message has been sent to Caffeine.

## Evidence needed to freeze B1

Target: latest official Caffeine integration pinned in
[provider-baseline.json](provider-baseline.json). This is a local request
checklist, not a message sent to the provider or permission for paid operations.
The public integration sources establish client behavior; the following facts
need an authoritative server contract and deployment evidence.

| Area | Exact information needed | Why it gates this service |
| --- | --- | --- |
| Deployment | Confirm support/onboarding for the independent Rust service; confirm gateway/Cashier target, account/project/bucket namespace and accountable operator; obtain exact server version/source revision | Establish a supported independent deployment and bind credentials, callbacks, paid effects and observations to the same provider installation |
| Wire contract | Gateway HTTP schema/error definitions, required callback Candid, and server behavior behind the retrieved Cashier Candid | Verify upload, balance/readiness/funding and settlement behavior beyond advertised signatures |
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

Toko's source defaults now identify a reachable candidate deployment and its
Cashier interface. The service account, intended deployment bindings and
operator ownership still need selection; paid qualification requires explicit
authority and bounded test resources. Do not infer those decisions from the
read-only observations above.

No missing field can be closed by adding local retry logic. If the provider
cannot support a required capability, record a supported narrower contract or
another provider decision. Under the maintainer's parity requirement, dropping
an existing capability also leaves Canic removal blocked until explicitly
resolved.
