# Caffeine integration contract review

Read-only review, 2026-11-14. This is the provider evidence companion to the
[rewrite design](ic-blob-storage-design.md#d0-first-gate-supported-independent-caffeine-access),
not a selected production protocol or a qualification result for this checkout.

**Finding:** the legacy project demonstrated independently installed Rust-host
uploads. The reviewed public SDK explains how that worked. Supported enrollment,
provider enforcement, safe recovery and final closure remain unestablished.
Proceed with this concrete candidate; do not freeze its identities or persistence
or describe the service as resumable yet.

## Evidence and its limits

Legacy sources below are pinned to
`25bed1352f3d2b9659f0d2583f50fa9af859ea84`. Provider sources are pinned to
`caffeinelabs/skills` at `59b9b29a4fad3828677c8b80ead0e06fdb40987e`, whose checked-in
frontend package identifies itself as `@caffeineai/object-storage` 1.1.2. This is
a source selection for review, not a registry-freshness claim or dependency adoption.

| Evidence | What it establishes | Limit |
| --- | --- | --- |
| [Legacy standalone trial summary][trial] | Reports two distinct authenticated uploaders, one original certificate call and two gateway PUTs each, and independently root/byte-verified 1 KiB downloads | Historical 0.73.0 behavior; private journals/artifacts were not revalidated; no representative multi-chunk or application-catalog acceptance |
| [Legacy provider setup summary][funding] | Reports a separate payer, deposit/sweep, payer-to-owner relationship, owner allocation and spendable gateway credit | No general enrollment guarantee, namespace exclusivity, itemized price or charge cutoff |
| [Provider SDK][sdk] and [Motoko boundary][mixin] | Expose certificate method, transfer shapes and callback signatures | Client/helper source does not establish the gateway's server-side validation or failure semantics |
| [Current GitHub integration guide][hosting] | Says independently deployed exported apps must replace managed storage | Does not establish support for this lower-level arrangement; successful historical traffic does not settle support |
| [Caffeine #16 — Clarify object-storage GC callback contract for non-Motoko consumers][gc-issue] | Existing question covers key mapping, callback failure, deletion, replay and billing | Still open with zero comments when checked; no provider answer to adopt |

Only source, public guidance and retained first-party summaries were read. No
provider data URL, account, old journal, signing key or deployment was accessed.
The legacy trial's live references and unresolved charges stay with its owner.

## Candidate protocol: what actually crosses the boundary

The client calls the Rust host's external Candid method
`_immutableObjectStorageCreateCertificate(text)` with the provider root. Its reply
is a record containing `method = "upload"` and `blob_hash`. The IC certifies that
ingress result; the client extracts the v4 response certificate and supplies it
to Caffeine. The plain record is not itself a signed certificate.
This path needs **no host-to-provider certificate acquisition call**. The legacy
[issuer][issuer] synchronously commits registration and possible exposure before
returning that record. Its [boundary tests][issuer-tests] exercise actual-uploader
checks, expiry/revocation, first-reference creation and repeat-issuance refusal.

| Boundary | Observed shape | Consequence for the rewrite |
| --- | --- | --- |
| Host certificate request | Root only; no Blob upload ID | Root is a locator. Resolve exactly one eligible grant using the captured caller and immutable installation binding. Reject ambiguity; never select an arbitrary upload |
| Certified application reply | Operation and root; no explicit length, application expiry, reference ID or project/bucket | Local checks on those fields are not proof the provider enforces them |
| SDK tree PUT | `/v1/blob-tree/`; tree, bucket, declared byte count, owner canister, project, headers and certificate under `OwnerEgressSignature` | Establish which fields the gateway binds to authority and which are client assertions |
| SDK chunk PUT | `/v1/chunk/`; raw bytes with owner/root/chunk/index/bucket/project selectors; no certificate in this request | Qualify how prior tree authorization bounds later chunks and how long those chunks remain admissible |
| SDK direct read URL | Owner, project and root selectors | A URL shape is not proof of deletion scope, bucket isolation or incarnation identity |

Do not add our upload IDs or expiry fields to the provider reply and assume the
gateway enforces them. Do not infer uploader-bound usage from local acquisition
authorization: the inspected transfer sends the certificate, not a separate
uploader proof. Treat it as potentially bearer authority until the provider
contract establishes otherwise.

The root commits to a chunk tree and metadata, not merely the raw file digest.
The SDK constructs 1 MiB chunks and includes a content-length metadata header;
its tree request also carries a separate byte count. These are useful candidate
checks, not evidence that malicious traffic cannot store more than admitted.

## Contract answers still required

| Contract | Available evidence | Required answer before selecting the profile |
| --- | --- | --- |
| Independent enrollment and account | Historical payer/owner setup and usable transfers | Is this Rust-host arrangement supported? How does a fresh installation obtain its account and permitted project/bucket binding? Which account incurs each charge? |
| One provider-object owner | SDK uses owner/project/root plus bucket on writes; callbacks provide binary keys | What exact tuple identifies deletion scope? Are keys tree roots, chunks or both? Can another installation affect retention? Require exclusive scope or an independent retention handle; a local namespace number is not proof |
| Exact bounded effect | Metadata/tree commitments and client byte count | Does the gateway reject inconsistent length/header/tree/chunk sizes and extra or substituted chunks? Which limits also bound partial/duplicate uploads? Until qualified, counters are declared usage, not a proven physical bound |
| Authority recovery | Original request identity can be retained and its IC result queried | Will the gateway accept the recovered proof, within what window, and can the SDK resume with it without another certificate call or unsafe PUT replay? Absence of a result is not proof of no effect |
| Lifetime and final release | Host can refuse new issuance; no revocation/expiry field in its reply | What invalidates an issued certificate and an accepted unfinished tree? Can either recreate storage or charges after release/deletion? Include delayed chunks and recovered certificates, not just initial PUT |
| Deletion and settlement | Three callbacks exist; helper prunes local runtime bookkeeping | What is authenticated confirmation evidence of? When are origin bytes gone and all attributable charges closed, including late writes? |
| Charging and caps | Historical credit was spendable; retail guide describes upload/storage/download charging | What are this account's actual terms and enforceable limits? Balance exhaustion, relationship expiry, zero usage counters and local release do not prove cutoff |

The [retail billing guide][billing] says existing files continue to be served and
storage charges accrue after credits run out. It is a warning against assuming
prefunding bounds liability, not a specification of the trial's cycles account.

If closure cannot be established, final release remains **Closing**, retains its
outstanding exposure and consumes the relevant finite capacity. No timeout,
operator assertion or locally invented receipt may convert it to Settled.
If supported independent access is ruled out, return that product choice to the
maintainer; successful substitutes cannot resolve it.

## Recovery and SDK disposition

The reviewed [public declarations][sdk-api] expose `putFile` and `getDirectURL`.
They do not expose prepare-before-admission, existing-certificate resume, or
per-client transport/retry controls. The source automatically retries tree and
chunk writes and uses up to ten upload workers. `putFile` returns a root after
workers finish; it does not require a provider-signed completion receipt, and its
worker ignores the chunk result's `isComplete` value. A returned root or 100%
progress must not become Blob's availability flag.

The [legacy patch description][patch] documents preparation of a retained file
handle and per-client transport, retry and concurrency controls. Those supplied
real missing capabilities, not incidental convenience. Its one-use prepared
handle is ephemeral and is not, by itself, a resumable-upload solution. Do not
import this patch or duplicate the SDK's transfer implementation. Select a
supported external package path supplying the required capabilities, or obtain
documented idempotency and adapt the client requirements explicitly. A new SDK
selection requires current registry and compatibility review.

The [legacy certificate client][certificate-client] retains the signed ingress
envelope/request ID before dispatch and can query the original IC request status.
It verifies the certificate, original request and expected reply before recording
the outcome. That is a useful recovery approach to evaluate through Platform.
It does not prove that a later certificate is accepted by Caffeine or that a
transfer may be restarted. Fresh issuance is a distinct authority exposure.
Registration inspection alone never recovers usable proof.

Closing/revocation can stop the cooperating client from requesting or forwarding
new authority. They cannot retract a certificate already held by that client,
nor prevent it from recovering an available original IC result. Provider lifetime
semantics must carry that part of the guarantee. Recording a delayed authentic
result preserves exposure without reopening the object.

## Retention callbacks and selective legacy reuse

The [provider mixin][mixin] exposes a public positional liveness query, a
gateway-authorized deletion list, and a gateway-authorized confirmation update.
The [helper][storage] obtains gateway membership from the configured Cashier's
`storage_gateway_list_v1`; it also contains automatic refill behavior, which is
outside the rewrite's scope. Gateway membership is not equivalent to operator or
controller status.

The legacy [callback profile][gc] protects unknown roots, keeps reservations/live
references protected, bounds batches, validates a whole acknowledgment batch
before mutation and retains acknowledged lifetimes. Its acknowledgment is
deliberately separate from physical deletion and billing settlement. Preserve
these distinctions; qualify the conservative unknown-root response against the
real protocol rather than copying Motoko's weak-reference absence semantics.

| Legacy component reviewed | Reuse decision and reason |
| --- | --- |
| Issuer and its boundary regressions | Retain commit-before-reply, actual-uploader authorization, current-state recheck and no second issuance on uncertain exposure. Adapt to one provider-object owner rather than importing the old installed store graph |
| [Manifest model][manifest] and [manifest regressions][manifest-tests] | Candidate for selective first-party reuse once the tree profile is selected. Keep bounded root/metadata consistency and exact-length/corruption checks. Its own test accepts a consistent tree with a false declared length, then rejects actual bytes: admission consistency is not byte-length authentication |
| Certificate transport | Retain original-request recovery and authenticated-result matching requirements. Reuse Platform capabilities and only the client state required by the selected protocol; do not import the whole browser/native journal system |
| Callback profile | Retain authentication, conservative unknown handling and acknowledgment/deletion/settlement distinctions. Requalify keys and closure before exposing deletion permission |
| SDK patch, funding and restoration machinery | Do not carry over. SDK capabilities need an external dependency solution; automatic funding is excluded; the rewrite selects a narrower management policy. Old installations keep their original recovery tools |

No code was carried over in this review. Existing reference tests remain the
only executable product-behavior qualification in this checkout.

## Next implementation boundary and acceptance

After the contract selects provider identity and effect bounds, implement one
local synchronous registration transition, used by the eventual standalone host:
application authorization/reservation → bound-uploader issuance → one object
owner plus known first reference and possible exposure → irreversible Closing.
For the observed protocol, registration happens **inside issuance before reply**,
not after acquiring a provider certificate. An additional use of the same object
joins its reference owner; another upload ID cannot independently gain deletion
rights or bypass uncertain authority. The root-only endpoint must have an explicit
unambiguous grant-resolution rule before it is mounted.

Use the existing reference component in that production owner. Do not add a
generic provider trait, fake certificate acquisition service or second state
implementation to make a substitute test pass. Keep the present API experimental
and defer provider identity/persistence types until the questions above close.

| Qualification boundary | Smallest useful witness |
| --- | --- |
| Local registration | Wrong caller, expired/revoked grant and conflicting declaration leave state unchanged; registration/reference/accounting/exposure commit together; overlapping upload identities cannot delete another live use |
| Installed standalone endpoint | Two signed callers exercise the actual method and verified IC reply; repeated issuance refuses; a lost reply is inspected by its original identity; real stable reopening preserves it |
| Client/SDK recovery | Recover an original usable proof without new issuance, or return explicitly blocked; interrupt tree/chunk replies and demonstrate only provider-qualified replay; reject corrupt, truncated and wrong-length reads against the original declaration |
| Provider contract | Fresh supported enrollment, immutable/exclusive scope, malicious declaration rejection, final release racing old authority/delayed chunks, and attributable deletion/charge evidence |

Local and installed tests may use controlled gateway responses, clearly labelled
as substitutes; those cannot establish the last row. A live trial needs separate
effect authorization and a concrete bounded plan after the supported arrangement
is known. No live trial or provider message is initiated by this review.

[trial]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/docs/evidence/caffeine-probes/deployed/2026-11-06-trial-v073-two-users-01/summary.json
[funding]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/docs/evidence/caffeine-probes/deployed/2026-11-06-trial-v073-provider-01/summary.json
[sdk]: https://github.com/caffeinelabs/skills/blob/59b9b29a4fad3828677c8b80ead0e06fdb40987e/packages/object-storage/frontend/src/StorageClient.ts
[sdk-api]: https://github.com/caffeinelabs/skills/blob/59b9b29a4fad3828677c8b80ead0e06fdb40987e/packages/object-storage/frontend/dist/StorageClient.d.ts
[mixin]: https://github.com/caffeinelabs/skills/blob/59b9b29a4fad3828677c8b80ead0e06fdb40987e/packages/object-storage/backend/src/Mixin.mo
[storage]: https://github.com/caffeinelabs/skills/blob/59b9b29a4fad3828677c8b80ead0e06fdb40987e/packages/object-storage/backend/src/Storage.mo
[hosting]: https://help.caffeine.ai/hc/en-us/articles/46899843980692-GitHub-Integration-Overview
[billing]: https://help.caffeine.ai/hc/en-us/articles/49362898986644-File-Storage-Costs
[gc-issue]: https://github.com/caffeinelabs/skills/issues/16
[issuer]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage/src/ops/service/uploads/certificate/mod.rs
[issuer-tests]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage/src/ops/service/uploads/tests/certificate_boundary/mod.rs
[manifest]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage-contracts/src/identity/caffeine/manifest/mod.rs
[manifest-tests]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage-contracts/src/identity/caffeine/manifest/tests/mod.rs
[patch]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/clients/browser/patches/README.md
[certificate-client]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/clients/browser/certificate.js
[gc]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/docs/provider/gateway-gc-contract.md
