# Caffeine gateway GC contract review

This is the source-derived wire contract and implementation prerequisite for
[#32](https://github.com/dragginzgame/ic-blob-storage/issues/32). Blob 0.19.0
does not export these provider methods. Its authenticated root observations are
local lifecycle views, not provider deletion permission. This review does not
enable GC or establish deployed retention or billing behavior.

## Reviewed provider interface

On 2026-10-09, the public `caffeinelabs/skills` main revision was
`1b72e52699b9092d8ba16c4cb2f2d99920662afc`. The exact
[Mixin.mo](https://github.com/caffeinelabs/skills/blob/1b72e52699b9092d8ba16c4cb2f2d99920662afc/packages/object-storage/backend/src/Mixin.mo)
and [Storage.mo](https://github.com/caffeinelabs/skills/blob/1b72e52699b9092d8ba16c4cb2f2d99920662afc/packages/object-storage/backend/src/Storage.mo)
bytes match the earlier 2026-10-02 review. The
[retained capture](evidence/caffeine-probes/local/2026-10-09-gc-contract0191-01/summary.json)
records hashes, request bounds and limitations. Its Candid file is a review
projection of these three methods, not the Blob service interface.

| Method | Candid arguments and result | Source behavior |
| --- | --- | --- |
| `_immutableObjectStorageBlobsAreLive` | `(vec blob) -> (vec bool) query` | Maps Motoko's `Prim.isStorageBlobLive` over the input, preserving order and duplicates. The reviewed method has no gateway-membership check. |
| `_immutableObjectStorageBlobsToDelete` | `() -> (vec blob) query` | Requires current gateway membership, then returns at most 10,000 roots from `Prim.getDeadBlobs`. There is no cursor or namespace argument. |
| `_immutableObjectStorageConfirmBlobDeletion` | `(vec blob) -> ()` update | Requires current gateway membership, prunes Motoko's retained dead-root list, then awaits the local GC trigger. It receives no deletion operation, incarnation or billing receipt. |

The reviewed membership owner replaces its in-memory list with Cashier's
`storage_gateway_list_v1` reply. Blob already owns a scoped durable registry,
operator revocation and correlated sync; those remain authoritative here.
Mounting these callbacks must not introduce a second membership cache or copy
Motoko's runtime primitives into a Rust service.

The public source contains the application-side methods. It does not contain
the deployed gateway's callback caller, retry loop or failure policy. A method
name containing "ConfirmBlobDeletion" and successful empty Candid output alone
do not establish physical deletion or cessation of charges.

## Required Blob decisions before mounting

The endpoint supplies actual caller/service and an explicit installed provider
scope. Membership, namespace and both durable-owner fences are checked before
reading any roots, including empty, malformed and unknown batches. Controllers,
operators and tenant identities do not impersonate gateways. If gateway callers
need the upstream unauthenticated liveness query, qualify that requirement before
changing Blob's authenticated observation boundary.

Wire blobs must use the provider's established root encoding. Configure finite
decoded-byte, root-count and retained-history limits; reject excess input before
lookup rather than silently truncating a positional reply. An authenticated
empty liveness request may return an empty vector. Malformed, unknown or foreign
roots cannot be turned into `false` or deletion candidates. Unknown-root handling
needs the gateway failure contract because rejection might itself affect retention.

| Retained local state | Safe implementation constraint |
| --- | --- |
| Reserved, ExposurePossible | Protect pending and uncertain work from deletion; absence of a confirmed reference is insufficient. |
| Live | Protect every live reference, including overlapping application release and read sessions. |
| DeletionPending | A possible candidate only after exact root/scope authority and the deletion protocol are qualified. Physical bytes and financial liability remain charged. |
| Cancelled, ProviderDeleted, Settled | Retained history is not permission to delete or reassign a root. A duplicate confirmation needs the exact original evidence. |
| Unknown, malformed, foreign | Refuse disclosure/deletion authority; never infer an absent object or safe cleanup. |

The no-argument deletion-list method requires one installed exclusive provider
namespace. It must enumerate a bounded deterministic selection from the same
durable lifecycle owner; a tenant's logical release is not an external deletion
receipt. An operator scan cursor is not a provider callback cursor. Confirm that
repeated bounded lists eventually advance under the actual acknowledgement rules
before relying on them for continuing cleanup.

Root-only confirmations require the complete permanent root-claim history and
verified namespace exclusivity, including earlier provider usage. Resolve a root
to its original binding, never to a new incarnation. Revocation, foreign scope,
active references and restore fences refuse before mutation. Same-contract
interruption must preserve the original deletion evidence and duplicate outcome.
The existing `confirm_provider_deleted` host API requires independently
authenticated physical-deletion evidence; a prune acknowledgement cannot be
connected to it until the provider establishes that meaning. Billing settlement
continues through its separate evidence boundary.

## Missing, rejecting and malformed methods

No deployed-provider result is available for any of these cases:

| Callback outcome | Provider evidence still required |
| --- | --- |
| Method absent or query/update mode wrong | Does the gateway retain bytes, retry, delete, or continue charging? Which caller and interval apply? |
| Authorization rejection, trap or malformed reply | Are failures treated as retention, liveness, deletion permission or an unavailable application? |
| Liveness vector length differs from input | Does the gateway reject the whole result or use a partial positional interpretation? |
| Confirmation reply lost after local mutation | What exact original deletion may be acknowledged again, and what happens to the provider's pending list? |
| Empty deletion list | Does this end one poll only, or affect retention/billing independently? |

A local missing-method reject or provider substitute would qualify only that
local outcome. It cannot answer these gateway questions. Obtain authoritative
gateway source/documentation or record a separately authorized, budgeted deployed
experiment before exposing deletion authority. Preserve physical/economic
obligations on uncertainty; do not install false/empty placeholder handlers.

## Consumer and provider acceptance

Blob owns the shared protocol implementation and safety tests once the provider
meaning is established. Canic mounts it without recreating wire DTOs or policy.
Toko retains application references and grants; Motoko heap liveness is not its
replacement Blob reference owner. Qualify live/reference protection, final release,
bounded batches, lost replies, duplicate/late confirmations, gateway revocation,
foreign roots and restored fences against exact committed artifacts. Record
provider deletion and billing cessation separately from all local results.
