# Caffeine gateway GC callbacks

The released 0.42 hard cut restores the three external Candid callbacks through
Blob's [shared endpoint catalog](../crates/ic-blob-storage-contracts/src/endpoints/mod.rs).
The service core owns their policy and durable state; standalone and consumer
frameworks explicitly mount that catalog. Linking the library exports nothing.
This implements the supported local protocol for
[#32](https://github.com/dragginzgame/ic-blob-storage/issues/32), without claiming
qualification of the deployed gateway's failure handling, deletion or charging.

## Supported tree-root profile

Keys are the original 32-byte Caffeine tree roots admitted with each upload.
The upload manifest builder and SDK comparison already distinguish these from
raw-content digests and chunk hashes. The retained
[SDK/source review](evidence/caffeine-probes/local/2026-10-22-gc-contract-discovery-01/summary.md)
and [follow-up](evidence/caffeine-probes/local/2026-10-22-gc-credit-contract-02/summary.md)
remain source evidence, not a deployed gateway observation. A different or unknown
provider key is protected, never guessed to be a released file or chunk.

| External method | Contract | Local behavior |
| --- | --- | --- |
| `_immutableObjectStorageBlobsAreLive` | `(vec blob) -> (vec bool) query` | Public, positional and duplicate-preserving. Known reservations and live references are protected. Unknown roots return true. Malformed batches refuse. |
| `_immutableObjectStorageBlobsToDelete` | `() -> (vec blob) query` | Current scoped gateway members only. Returns at most 10,000 known released tree roots, excluding read-held or already acknowledged lifetimes. |
| `_immutableObjectStorageConfirmBlobDeletion` | `(vec blob) -> ()` | Current scoped gateway members only. Records an acknowledgment on each exact original released lifetime. It does not erase records or settle physical/billing obligations. |

The explicit public liveness query follows the provider mixin's access pattern;
it discloses only retention booleans, not tenant/request identities or manifests.
Operators, controllers and tenants cannot impersonate gateway membership for
listing or acknowledgment. The registry remains the existing durable owner,
refreshed from the configured Cashier; no second membership cache is introduced.

All three operations bind the actual service and the installation's namespace
and Cashier. Registry, upload and read owners must share the same configuration
and be unfenced. Suspension, expired permissions and a revoked uploader do not
erase registered retention or escaped upload authority. Releasing the final
reference queues logical deletion; an outstanding local read still protects it.

## Bounds, acknowledgments and recovery

The shared provider codec bounds raw ingress to 400,000 bytes, roots to 10,000
and combined key bytes to 320,000. Candid work, type/header and skipping budgets
are explicit. The no-argument query has its own 1,024-byte decoding envelope.
The endpoint guard bounds ingress before the Platform argument-buffer copy.
Malformed keys, extra arguments and excess batches refuse before root mutation.

Deletion selection traverses the bounded lifetime lifecycle history in its stable
key order. Acknowledged entries disappear from this projection, so subsequent
polls can reach later entries without a new cursor or durable deletion index.
The work bound is the installed lifetime history; this is not a constant-time
scan and no history slots are reclaimed by acknowledgment.

An acknowledgment validates the complete batch before writes, including native
backing. Live/read-held, malformed and unknown roots refuse the whole batch.
Duplicates are deduplicated for mutation. Exact retries after a lost reply remain
idempotent, including after later independently established physical deletion or
billing settlement. Acknowledgment cannot release another incarnation: permanent
root claims and the original request/index binding remain retained. Current
membership and all fences are checked again on every call, including empty input.
Stable write traps must propagate for whole-message IC rollback.

The acknowledgment bit lives in the existing lifecycle record. Same-release
restoration validates it and opens fenced; a live record cannot carry it.
Cross-release transitions require retirement/reinstall with the old obligations
and tools retained. There is no migration, dual reader or automatic reset.

## What the callback does not establish

Gateway acknowledgment does not invoke `confirm_provider_deleted` or
`confirm_billing_stopped`. Physical bytes and financial liability remain charged
until their existing host-internal evidence boundaries are satisfied. No physical
or billing confirmation is manufactured from a timeout, missing download, empty
list, acknowledged callback or zero balance.

The provider's Motoko implementation prunes local weak-reference bookkeeping and
triggers local GC. Rust uses explicit durable references; it does not reproduce
that runtime table or GC trigger. Unknown roots are deliberately protected rather
than using Motoko's absent-entry false result. If the gateway supplies other key
kinds, this can retain data; that conservative difference needs deployed acceptance.

[Caffeine #16 — GC callback contract](https://github.com/caffeinelabs/skills/issues/16)
asks for the remaining key mapping and missing/rejecting/malformed callback,
remote deletion, replay and billing semantics. An answer is not a prerequisite
for local implementation and testing of this narrower profile. It is still needed
for stronger claims about the deployed provider, or those claims need separately
authorized observations. Local substitutes cannot supply those guarantees.
