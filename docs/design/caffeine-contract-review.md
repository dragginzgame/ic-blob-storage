# Caffeine integration contract review

Read-only review, 2026-11-14. This is the provider evidence companion to the
[rewrite design](ic-blob-storage-design.md#d0-first-gate-supported-independent-caffeine-access),
not a selected production protocol or a qualification result for this checkout.

**Finding:** the legacy project demonstrated independently installed Rust-host
uploads. The reviewed public SDK explains how that worked; an independent public
Motoko consumer now supplies a concrete self-funded account integration to compare.
Supported Rust enrollment, provider enforcement, safe recovery and final closure remain unestablished.
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
| [Legacy deletion follow-up][gc-readback] | November 5 summary reports the released object still DeletionPending, zero recorded acknowledgment attempts and an HTTP HEAD 200 | Historical observation only; telemetry is incomplete, HTTP may be cached, and neither origin deletion nor billing cessation was established |
| [Provider SDK][sdk] and [Motoko boundary][mixin] | Expose certificate method, transfer shapes and callback signatures | Client/helper source does not establish the gateway's server-side validation or failure semantics |
| [Current GitHub integration guide][hosting] | Says independently deployed exported apps must replace managed storage | Does not establish support for this lower-level arrangement; successful historical traffic does not settle support |
| [Caffeine #16 — Clarify object-storage GC callback contract for non-Motoko consumers][gc-issue] | Existing question covers key mapping, callback failure, deletion, replay and billing | Still open with zero comments when checked; no provider answer to adopt |
| [Rabbithole storage description][rabbit-storage] and [account integration][rabbit-account] | Independent consumer describes Caffeine-backed personal canisters; its public source funds the canister's own Cashier account | Motoko consumer implementation, not a Caffeine support commitment, server contract or live observation by this review |

Only source, public guidance and retained legacy summaries were read. No
provider data URL, account, old journal, signing key or deployment was accessed.
The legacy trial's live references and unresolved charges stay with its owner.

### Independent-access follow-up for 0.100.2

The November 14 follow-up checked Caffeine's public repository inventory, storage
package guidance and exact protocol names, then followed the independent
Rabbithole consumer. The Caffeine skills head still matched the pinned revision;
issue #16 remained open with zero comments. The [export guide][hosting], updated
November 13, still says independently deployed exports need replacement storage.
Its scope does not establish the terms of a lower-level account integration.

Rabbithole source is pinned to `baa4d86314822711735cc9860220c109210b1ce9`:

- [CashierAccount.mo][rabbit-account] looks up the configured Cashier, reads the
  canister's own account balance and attaches cycles to `account_top_up_v1` for
  that same principal. It also manages account delegates and automatic funding.
- [EncryptedStorageCanister.mo][rabbit-canister] includes the Caffeine storage
  mixin and exposes an authorized setup path that ensures its bootstrap balance.
- [Cashier.mo][rabbit-cashier] declares balance, pricing, overdraft and payment
  relationship interfaces. Those types describe a consumer's expected API; they
  do not specify enforcement, enrollment eligibility or the end of charge exposure.

This is concrete independent-consumer evidence for a **self-funded canister
account candidate**, alongside the legacy trial's separate payer/owner candidate.
Neither account arrangement is selected for this rewrite. Do not infer supported
Rust enrollment from Motoko source, or exclusive retention scope from an account
principal. The inspected consumer uses the Motoko mixin; it supplies no independent
specification of the non-Motoko deletion callbacks. No third-party implementation,
automatic funding, delegation machinery or consumer deployment was adopted.

The provider's [storage setup guidance][storage-guide] describes missing-account
errors and redeployment within its managed setup. That is not an independently
usable enrollment procedure. Its package helper and Rabbithole also declare
different top-up argument/result shapes. Do not choose a binding by copying either
snippet: the selected provider revision and supported ABI need explicit evidence.

The next access question is now specific: **does Caffeine support a fresh,
independently deployed Rust canister using its certified method/root reply and
explicitly provisioned Cashier account, without Motoko weak-reference tracking
or automatic-refill integration?** An affirmative answer must identify account
provisioning, supported ABI/revision, payer and immutable project/bucket binding.
Then the contract table below still needs enforced object bounds, original-proof
recovery/lifetime and retention/charge closure. A self-funded account does not
answer those questions or establish a finite liability cap.

**Disposition:** no D0 requirement is newly qualified by this source review.
It narrows the access candidate and avoids treating independent access as proven
impossible. No additional provider-backed enrollment or server-semantics contract
was found in the inspected sources; this is a search limit, not a claim that none
exists. Further progress on these guarantees needs new provider evidence, not
another broad search of these unchanged sources or an invented local mechanism.
No question was posted and no provider/account call was made.

### Prepared provider clarification — not sent

Local clarification draft referencing [Caffeine issue #16][gc-issue]. The maintainer
declined posting it and subsequently authorized Caffeine calls when ready.
Provider-call approval does not authorize sending this message: retain these
questions as design inputs, not a queued outreach task. The approval removes the
earlier local-only action restriction; it supplies no missing provider guarantee.

> We are narrowing the `ic-blob-storage` rewrite to upload authorization and explicit
> application references, usable from an independently deployed Rust canister.
> Before implementing the provider boundary, could you point us to the supported
> contract/revision for this arrangement?
>
> 1. Is independent Rust access supported using
>    `_immutableObjectStorageCreateCertificate(text)` and the IC-certified
>    `{ method = "upload"; blob_hash }` reply? What is the supported Cashier
>    enrollment and owner/project/bucket binding procedure? We would provision
>    funding explicitly, without automatic refill or Motoko weak-reference tracking.
> 2. What limits the lifetime of that proof and of an accepted, unfinished tree?
>    Can the original recovered proof or delayed chunks recreate storage after
>    final release/deletion? We need the existing expiry/revocation rules, including
>    in-progress writes, rather than a new receipt service.
> 3. What exactly identifies the object whose retention our callbacks control,
>    and which length/tree/chunk bounds does the gateway enforce? What establishes
>    the end of storage and charge exposure? The callback questions above remain
>    relevant, particularly the meaning of deletion confirmation.
>
> The public SDK documents the client requests, and independent Motoko consumers
> provide useful integration examples, but neither establishes these server-side
> guarantees for a Rust host. Documentation or versioned protocol references are
> sufficient; we are not requesting deployment, funding or a provider experiment.

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
The [release/issuance ordering rule](ic-blob-storage-design.md#object-transitions-and-closure)
therefore has no pending acquisition callback: cancellation first prevents issuance;
issuance first records exposure that survives late delivery of the certified result.
The local regression exercises both registration/cleanup orders, not IC delivery.

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

Eligibility must cover historical ambiguity as well as simultaneous grants: a
delayed root-only request must not consume a later grant for the same uploader/root.
The [application boundary](application-boundary.md#provider-facing-identity-gate)
selects refusal of another upload lifetime for every admitted root, including
cancelled/expired unexposed grants. The local model retains the original root claim;
an actual host must preserve that rule when mapping the provider request. This
local restriction does not qualify provider scope, proof recovery or closure.

The root commits to a chunk tree and metadata, not merely the raw file digest.
The SDK constructs 1 MiB chunks and includes a content-length metadata header;
its tree request also carries a separate byte count. These are useful candidate
checks, not evidence that malicious traffic cannot store more than admitted.

## Contract answers still required

| Contract | Available evidence | Required answer before selecting the profile |
| --- | --- | --- |
| Independent enrollment and account | Historical Rust payer/owner setup and transfers; independent Motoko consumer self-funded account source | Is the Rust-host arrangement supported without Motoko tracking/automatic refill? How does a fresh installation obtain its account, supported ABI and permitted project/bucket binding? Which account incurs each charge? |
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

### Field enforcement required before registration

The local manifest and read checks now make the remaining boundary concrete.
This matrix describes the reviewed SDK protocol, not a new wire schema. The
future host must authorize one immutable declaration before returning the
certified method/root reply. That reply alone does not explicitly bind every
field in the transfer request.

| Input | Blob's local responsibility | Provider guarantee still needed |
| --- | --- | --- |
| Caller, upload ID, first reference and grant deadline | Capture the actual caller; resolve one eligible grant; consume it and record possible exposure atomically before reply | Whether the resulting proof is bearer authority; validity after local expiry, revocation or final release |
| Root and ordered leaf hashes | Check bounded count and tree consistency with original metadata | Tree/root and chunk/hash/index validation, including rejection of extra leaves and oversized full/final chunks |
| Declared bytes | Check local byte limit and implied leaf count; compare the immutable declaration on retries | Reconcile `num_blob_bytes`, actual accepted chunk sizes and total accepted bytes; hashes alone do not authenticate length |
| Original metadata | Bound and normalize for the root; the future admission path must check any required Content-Length value against its declaration | Bind the accepted headers to the root and reject conflicting size assertions; the current preflight deliberately does not interpret HTTP headers |
| Owner, project and bucket | Use the immutable selected installation binding | Define which fields are bound to the proof and the actual deletion/charge scope; a client-supplied selector is not an isolated namespace |
| Callback binary key and authenticated sender | Resolve the qualified key to the existing owner; preserve unknown/mismatched obligations | Define key encoding, root-versus-chunk meaning, sender discovery and whether confirmation acknowledges permission, deletion or another event |

Host validation and provider enforcement are separate obligations. A successful
read against a trusted declaration establishes integrity of that read, not an
upper bound on what a malicious upload request could store or charge. Until the
provider bound is established, any byte accounting remains **declared usage**.

### Qualification sequence and stopping points

D0 selects a supported contract from provider-backed evidence; D1 implements it
and checks the installed host with labelled substitutes; D2 exercises the same
contract against a fresh, explicitly selected provider installation. The maintainer
has authorized Caffeine calls when ready and local installed-host testing before
D0 closes, with provider effects disabled during that experiment. D0 does
not require a live test of a host that has not been implemented. Conversely, a
successful D2 sample cannot supply missing general guarantees about old proofs,
namespace isolation or future charges. A protocol change found during D2 returns
to design review before the affected API/persistence model is accepted.

The next useful evidence is supported independent enrollment and a specification
covering the matrix above. The October [legacy discovery][gc-discovery] and
[contract follow-up][gc-contract-followup] already looked for gateway semantics;
they found helper/runtime behavior, not a server contract. The November deletion
follow-up still did not establish closure. These are reasons to keep D0 open,
not evidence that no such arrangement can exist. Do not repeat the old live
queries or resume those objects through the rewrite.

The public [troubleshooting guide][troubleshooting], read on 2026-11-14, describes
background removal after references disappear and client chunk retries. It does
not specify the Rust callback key mapping, certificate cutoff or in-progress
upload closure. Its suggested wait is not a protocol expiry or settlement rule.

Once D0 has answers, use the following cases to prepare D1/D2 qualification.
These are planned witnesses, not runnable tests or passing results. Start with
the existing deterministic 5 MiB + 17 byte fixture and a small file; those are
test inputs, not selected production size limits.

| Witness | Required observation |
| --- | --- |
| Valid control and namespace binding | Fresh supported Rust-host account/binding; two authenticated uploaders; direct SDK transfer and read checked against the original declaration. A second reference shares one owner; first release cannot permit deletion |
| Inconsistent size assertions | Separately vary tree byte count, root-bound Content-Length and full/final chunk lengths; include a root-consistent declaration whose leaf bytes have a different length. Observe the specified rejection boundary and disposition of partial effects, not just an HTTP error |
| Invalid tree or chunk | Substitute/reorder a leaf, append an extra leaf or use an incorrect index; retain the original control result and exact request identity. The gateway must enforce the selected object bound even when the SDK is bypassed |
| Binding substitution | Within separately owned test scopes, vary owner/project/bucket while holding the proof fixed. Outcomes must match the documented scope, without granting an independent deletion owner for shared storage |
| Lost authority reply | Recover the original IC result by request identity and show usable provider acceptance under the selected lifetime rule, or report blocked. Inspection of a local registration is insufficient; do not obtain a fresh certificate to make this case pass |
| Final release and escaped authority | Release while proof delivery or a chunk is delayed; then exercise the permitted expiry/revocation boundary. New local issuance stays refused; any late accepted effect stays recorded. No deletion acknowledgment alone releases exposure |
| Exact closure | Correlate authenticated messages to the exact object and show the selected authority cutoff, origin deletion and separate charge cutoff. Empty/aggregate callbacks, balance changes, HTTP 200/404 or elapsed time cannot substitute for that evidence |

Each eventual live plan must name its fresh account/scopes, exact objects and
request limits, funding/charge terms, permitted replay, observation deadline and
custodian for unresolved obligations. A deadline stops observation; it does not
settle an object. There is no live runner or selected live installation here.
Add `qualification/crates/` when the real standalone host exists; continue to
keep pure checks beside their library owner. No new framework, retained probe
ledger or placeholder package is needed for this plan.

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
| [Manifest model][manifest] and [manifest regressions][manifest-tests] | Adapt the bounded root/metadata consistency check into local preflight below. Keep actual-byte verification in the streaming owner. Its own test accepts a consistent tree with a false declared length, then rejects actual bytes: admission consistency is not byte-length authentication |
| Certificate transport | Retain original-request recovery and authenticated-result matching requirements. Reuse Platform capabilities and only the client state required by the selected protocol; do not import the whole browser/native journal system |
| Callback profile | Retain authentication, conservative unknown handling and acknowledgment/deletion/settlement distinctions. Requalify keys and closure before exposing deletion permission |
| SDK patch, funding and restoration machinery | Do not carry over. SDK capabilities need an external dependency solution; automatic funding is excluded; the rewrite selects a narrower management policy. Old installations keep their original recovery tools |

The initial review carried over no code. The subsequent local integrity checks below
selectively adapt first-party hashing and manifest checks; they adopt no legacy
service state or recovery records.

## Implemented local read verification

In development version 0.100.1, [RootVerifier](../../crates/ic-blob-storage/src/caffeine/mod.rs)
implements the reviewed SDK 1.1.2 tree computation for independently verifying a
received stream. The npm registry was checked on 2026-11-14 and still identifies
1.1.2 as latest; no new SDK preparation/resume capability was found in that API.
This selects a local integrity algorithm, not an account/object identity, upload
protocol guarantee or persistent representation. D0 remains open.

Adapted first-party sources at the legacy revision above are the
[streaming hasher][legacy-hasher], [metadata normalizer][legacy-metadata] and
[root verifier][legacy-verifier]. Retain the fixed-size chunk frontier, odd-tree
padding, original metadata, exact offsets and pre-mutation checks. Combine the
hasher/verifier into one owner. Omit raw-digest computation, leaf observers,
manifest builders, out-of-order/checkpoint modes and transport/file handling:
the current consumer need is one sequential comparison to an already trusted root.
Unlike the old whole-content SHA-256 computation, only individual bounded chunks
are hashed, so declared file length is bounded by the configured u64 limit.
No whole-file allocation follows from that declaration.

The [eight streaming tests](../../crates/ic-blob-storage/src/caffeine/tests.rs) adapt
legacy rejection/integrity scenarios and use selected frozen
[SDK algorithm vectors][legacy-vectors], including multi-chunk, odd-tree,
repeated-leaf and ECMAScript metadata cases. Expected roots are retained external
observations rather than generated by the implementation under test. The vector
record identifies the observed source hash and input recipe; its unusual Unicode
metadata cases test hashing, not HTTP validity. No live-provider observation or
fresh SDK execution is claimed by running these Rust tests.

`sha2` 0.11.1 is the direct dependency for integrity checks, inherited from the root with
default features disabled. The crates.io index identifies it as the latest stable
non-yanked release at selection time. Existing approved SHA-256 owns the provider
commitments; no cryptographic primitive is implemented here. This owner-local
slice needs no installed-host `qualification/` package. Its public API remains
experimental. Browser upload preparation and transfer still belong to the SDK.
The later local control model separately uses Platform Types for caller identities.

## Implemented local manifest preflight

The 0.100.1 checkpoint adds [check_manifest](../../crates/ic-blob-storage/src/caffeine/manifest.rs)
for bounded checking of a proposed declaration before any future authorization.
It adapts the length/count/metadata/root checks from the first-party
[manifest constructor][manifest] at the pinned legacy revision. The caller supplies
root, declared length, ordered leaf hashes and original metadata. Limits reject
excess length/count before hashing; the existing metadata and tree functions own
both streaming and manifest rules. The check borrows the leaves and returns only
success/failure, retaining no manifest, receipt or authority.

This is a local consistency preflight, not an SDK wire decoder or a final admission
decision. Decoders must bound inputs before allocating them. Unlike the legacy
manifest object, this slice needs no owned chunk vector, range API, per-leaf read
verification, builder, checkpoint, serialization or extra dependency. Those had
different consumers; the rewrite currently needs only preflight and sequential
read integrity. The experimental Wire host now calls this preflight through the
library service; the production provider/client boundary remains unimplemented.

Four additional owner-local tests adapt [legacy failure cases][manifest-tests]
and check frozen SDK leaf/root observations. They cover full/partial and uneven
trees, reordered/corrupted/replaced leaves, metadata substitution, budget/count
refusal and a consistent manifest with a false length. An `abc` leaf also matches
declarations of one, four or 1 MiB of content: its hash does not authenticate any
of those lengths. Checking actual bytes rejects the false declaration. Success
therefore cannot justify a conservative exposure bound or issuance by itself;
provider byte enforcement remains a D0 gate. No live provider claim follows.

## Next implementation boundary and acceptance

The maintainer-approved [experimental local model](ic-blob-storage-design.md#experimental-local-service-model)
rehearses the synchronous transition below with declared-byte accounting. Its
[installed profile](ic-blob-storage-design.md#experimental-installed-profile-in-01002)
now adds bounded persistence and a local host, without certificates or settlement.
Root matching remains provisional, not the provider deletion key. After the
contract selects identity and effect bounds, qualify the effect-capable transition:
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

`make test-signed` now covers signed application/uploader calls to the local host,
authenticated responses, original-upload inspection after discarding a registration
reply and refusal of repeated registration. It exercises the existing experimental
transition only: it neither issues nor recovers provider authority. The installed
standalone row therefore remains partial, alongside the existing upgrade witness.

Local and installed tests may use controlled gateway responses, clearly labelled
as substitutes; those cannot establish the last row. A live trial needs separate
effect authorization and a concrete bounded plan after the supported arrangement
is known. No live trial or provider message is initiated by this review.

[trial]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/docs/evidence/caffeine-probes/deployed/2026-11-06-trial-v073-two-users-01/summary.json
[funding]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/docs/evidence/caffeine-probes/deployed/2026-11-06-trial-v073-provider-01/summary.json
[gc-readback]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/docs/evidence/caffeine-probes/deployed/2026-11-05-trial-v069-gc-readback-01/summary.json
[gc-discovery]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/docs/evidence/caffeine-probes/local/2026-10-22-gc-contract-discovery-01/summary.md
[gc-contract-followup]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/docs/evidence/caffeine-probes/local/2026-10-22-gc-credit-contract-02/summary.md
[sdk]: https://github.com/caffeinelabs/skills/blob/59b9b29a4fad3828677c8b80ead0e06fdb40987e/packages/object-storage/frontend/src/StorageClient.ts
[sdk-api]: https://github.com/caffeinelabs/skills/blob/59b9b29a4fad3828677c8b80ead0e06fdb40987e/packages/object-storage/frontend/dist/StorageClient.d.ts
[mixin]: https://github.com/caffeinelabs/skills/blob/59b9b29a4fad3828677c8b80ead0e06fdb40987e/packages/object-storage/backend/src/Mixin.mo
[storage]: https://github.com/caffeinelabs/skills/blob/59b9b29a4fad3828677c8b80ead0e06fdb40987e/packages/object-storage/backend/src/Storage.mo
[hosting]: https://help.caffeine.ai/hc/en-us/articles/46899843980692-GitHub-Integration-Overview
[billing]: https://help.caffeine.ai/hc/en-us/articles/49362898986644-File-Storage-Costs
[troubleshooting]: https://help.caffeine.ai/hc/en-us/articles/46899784111380-File-Storage-Troubleshooting
[gc-issue]: https://github.com/caffeinelabs/skills/issues/16
[storage-guide]: https://github.com/caffeinelabs/skills/blob/59b9b29a4fad3828677c8b80ead0e06fdb40987e/skills/extension-object-storage/SKILL.md
[rabbit-storage]: https://docs.rabbithole.app/how-it-works/storage/blob-storage
[rabbit-account]: https://github.com/rabbithole-app/v2/blob/baa4d86314822711735cc9860220c109210b1ce9/apps/backend/src/BlobStorage/CashierAccount.mo
[rabbit-canister]: https://github.com/rabbithole-app/v2/blob/baa4d86314822711735cc9860220c109210b1ce9/apps/backend/src/EncryptedStorageCanister.mo
[rabbit-cashier]: https://github.com/rabbithole-app/v2/blob/baa4d86314822711735cc9860220c109210b1ce9/apps/backend/src/Types/Cashier.mo
[issuer]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage/src/ops/service/uploads/certificate/mod.rs
[issuer-tests]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage/src/ops/service/uploads/tests/certificate_boundary/mod.rs
[manifest]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage-contracts/src/identity/caffeine/manifest/mod.rs
[manifest-tests]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage-contracts/src/identity/caffeine/manifest/tests/mod.rs
[patch]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/clients/browser/patches/README.md
[certificate-client]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/clients/browser/certificate.js
[gc]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/docs/provider/gateway-gc-contract.md
[legacy-hasher]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage-contracts/src/identity/caffeine/mod.rs
[legacy-metadata]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage-contracts/src/identity/caffeine/metadata/mod.rs
[legacy-verifier]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage-contracts/src/identity/caffeine/verification/mod.rs
[legacy-vectors]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage-contracts/tests/fixtures/caffeine-hashing/vectors.json
