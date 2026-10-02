# Current status

Date: 2026-10-02

Released baseline: [0.6.1](../../CHANGELOG.md), dated 2026-10-02, at
`a1e2c9ba69cc058526541e38da5f9884c6a2baac`. Package, tag and receipt agree;
the maintainer completed the release. New work joins Unreleased; no next version
is named. The isolated deployed owner still runs frozen 0.6.0; its DID is byte
identical to released 0.6.1. This continuation does not update that installation.
Commits and release operations belong to the maintainer.

Canic integration is deferred by the maintainer on 2026-10-02 until useful work
within this repository is exhausted. Continue local core/standalone/native/browser
implementation and evidence; the [feedback list](../canic-parity.md#integration-feedback)
is a future consumer backlog, not a prerequisite for this local work. Siblings
remain read-only and no upstream message is authorized.

## Configurable upload limits for Toko Miner — 2026-10-02

The maintainer explicitly requests relaxing the 1 KiB cap because it delays Miner
integration. Current source removes the extra certificate-policy envelope, including
its single-tenant/object/reference restrictions. Sizes and capacity now use the
validated installation's existing resource configuration. Admission reserves
object/tenant/global bytes and lifetime history/leaves; preparation and issuance
recheck the exact original permission/reservation. Uploader trust, namespace,
one-time exposure, durability, deadlines/revocation and restore fences remain.

The former policy envelope, host-evidence field and public blocker are removed
from core, standalone Candid, native JSON, fixtures and tests. There is no parallel
mode or bypass. This public semantic/API hard cut joins Unreleased and requires
a minor release; package/release receipt and the deployed 0.6.0 owner remain frozen.
Its stopped browser claim, configured 1 KiB slot, link/balances and liabilities
are not reset or upgraded. Implementation authority here includes no deployment
or new provider traffic.

Read-only Miner feedback at local HEAD
`dcc4b0131a928092f39be45d06d5397f3045f8ff` is **uncommitted**, retained SHA-256
`cd187187521a88392b052eed265ee7d4dc16130120e5f892a369123760703c35`.
It reports 816 media files/271,176,671 bytes and a largest 8,362,256-byte asset;
a 10 MiB object maximum covers that reported file, but full release history,
overlapping physical/liability capacity and client/read budgets need separate sizing.
This is consumer source feedback, not measured live provider or Miner acceptance.

See [current sizing](../standalone-trial.md#current-upload-sizing),
[configuration guidance](../operator-guide.md#size-a-consumer-installation) and
the [consumer feedback](../canic-parity.md#integration-feedback). Consumers must
adopt the removed field/variant after publication and choose their own explicit
limits. Canic integration remains deferred and sibling repositories remain read-only.

Focused validation passes: core upload/admission/manifest/exposure/recovery cases,
native assessment JSON, release standalone/storage-probe Wasm, actual IC multi-file
issuance and quota rejection, caller/trust/replay refusals, rollback/lost-response
checks, stop/start and restore fencing, signed setup recovery, and declared/exported
Candid through the installation carrier. Strict affected-package all-target lint
passes after extracting test helpers. Initial wrong test-target/empty-filter
selections and the test-length lint failure remain in `.tmp/configured-upload-01`.
The [evidence record](../evidence/configured-upload.json) distinguishes local
certificate issuance from live Caffeine transfer and consumer acceptance.
No full CI, version/release/commit, provider effects, sibling edits or build cleanup.

## Earlier approved allowance update; owner and gateway gain credit — 2026-10-02

The maintainer explicitly approves the existing daily allowance **1T -> 5T**, with
expiry unchanged. The [completed comparison](../evidence/caffeine-probes/deployed/2026-10-02-trial-limit-01/summary.json)
records one successful payer-signed update, request
`1d6698003bdf90619aa53a274b800db4aa09af2bf93f5e383c71b2db4558fc03`.
Fresh verified readback confirms 5T and the same raw expiry `1790953635603000000`
(nominal 2026-10-02T15:07:15.603Z); expiry enforcement remains unqualified.

Readback also shows **1T allocated from payer to owner**: payer falls from 5.0996T
to **4.0996T**, relationship period spend rises from zero to 1T, and the owner
reports **1T Ledger credit**. Its earlier zero/Prepaid observation remains historical.
Gateway available credit rises from zero to raw **333333333333**, with all reported
usage zero. The account sums reconcile to 5.0996T. This temporal comparison does
not prove the server algorithm, minimum allowance, earlier HTTP 403 cause or upload
admission. No new donor transfer occurs; gross donor debit remains **7.1001T** and
remaining standing authority **92.8999T**. Do not count internal allocation twice.

Four baseline and three planned post-update queries complete; the latter window
is 8.228 seconds. Before one additional public owner-balance query, a separate
read-only accounting-followup intent records the unexpected payer debit. Eight
logical queries use sixteen bounded, mainnet-verified query/verification transports.
An initial local verifier incorrectly assumes the gateway list returns a Result;
its failure and corrected official-IDL decoding are retained without network retry.
Delivery checks also catch verifier-script permissions and an overstrict prose
assertion; both are corrected, with their failed checks retained privately.
Private `.tmp/trial-limit-01` retains requests, signatures, typed reply and hashes.
No funding, certificate, upload/download, deployment, build or cleanup occurs.

Next: prepare a separately reviewed fresh-owner upload/download trial now that
positive owner/gateway credit is observed. Preserve the original owner, payment
accounts/link, stopped certificate/403 browser claim and 1 KiB exposure/liability;
never resend its claimed transfer or reset its lifetime slot. A fresh deployment or
new link is a separate effect, not authorized by this allowance update. Canic stays
deferred; its [open feedback](../canic-parity.md#integration-feedback) records that
allowance updates can move provider credit even without an upload request.

## Earlier larger payer balance; owner/gateway credit still zero — 2026-10-02

The maintainer requests testing with more cycles under the standing total 100T
trial spending authority. The [new funding capture](../evidence/caffeine-probes/deployed/2026-10-02-trial-funding-02/summary.json)
refreshes exact account/link/gateway, deposit address, ledger fee and source balances.
The selected recovered donor has only 5.754605097235T liquid, so the initial 10T
maximum is reduced before signing to **4.1T gross**, retaining 1.654605097235T.
No other source or identity is selected; the 100T approval is not a liquid balance.

One fresh independently verified deposit succeeds: request
`36d459d0ffc1d72be35ec0d75ae6876ee35ce6974a88c173a1dfa0102871c423`,
block 16,750,551, amount 4.0999T plus 100M fee. Exact sender, isolated Cashier
subaccount, amount/fee/memo/time and donor/deposit deltas match. One original-payer
notification succeeds: request
`49ad8b56101406da75bcf56d55175915e44c913134087febf85b34352bacba34`,
sweep block 16,750,574, **4.0998T credit** after 100M sweep fee. The deposit is empty;
the same payer now reports **5.0996T**, unchanged account identity, zero overdraft
and no target auto-refill. Gross trial donor debit is **7.1001T**, remaining standing
authority **92.8999T**. Donor liquidity remaining is only **1.654605097235T**.

The daily limit remains 1T and nominal expiry 2026-10-02T15:07:15.603Z; exact
relationship metadata/period spend stay unchanged. Initial and later gateway budget
replies are zero credit/usage. A mainnet-verified public `account_balance_get_v1`
for the storage owner independently returns an existing zero-total/prepaid/promo/
ledger account with debt target Prepaid. The linked payer instead holds 5.0996T
ledger credit. Keep owner, payer and gateway balances distinct; these observations
do not prove allocation rules or the earlier HTTP 403's cause. No upload admission
is retested. The source's 10T funding example is still not reached.

Nineteen verified logical queries use thirty-eight captured query/verification
transports; the recorded final two-query observation window stays below 60 seconds.
One public web lookup fails with cache miss and is retained separately from the
fresh prior immutable README. Native/block/signature and record/budget checks apply;
no build/full CI, release/commit, sibling edit, account-term change or build cleanup.
Private `.tmp/trial-funding-02` retains intents, receipts, official IDL sources and
hashes. Original identities remain private, temporary password removed and global
identity unchanged. The previous owner/certificate/profile/403 claim and 1 KiB
exposure/liability remain untouched; no reset, release or cessation is inferred.

Then-proposed comparison, subsequently approved and completed above, was unsigned
and independently encoded in
`.tmp/trial-funding-02/daily-limit-proposal.json` and `.candid`: one original-payer
`payment_account_canister_update_v1` changing **only the daily allowance 1T -> 5T**,
matching the documented example. Keep expiry exactly unchanged, zero attached
cycles, no new funds/certificates/uploads, at most one submission and bounded
relationship/budget readback. At that point it needed exact term selection: earlier explicit
link approval changed expiry while preserving 1T. Standing spend authority persists.
Re-read terms/roles and nominal expiry before signing; changed/expired state or any
uncertainty stops without a fresh retry. Do not fund the owner directly, change
the configured payer or bypass the stopped browser history. Canic stays deferred;
its [feedback](../canic-parity.md#integration-feedback) records separate balances.

## Earlier live certificate succeeds; gateway refuses credit — 2026-10-02

The maintainer explicitly approves the proposed existing-link two-hour expiry
update. The [live capture](../evidence/caffeine-probes/deployed/2026-10-02-trial-live-01/summary.json)
retains one successful payer-signed update and exact verified readback: request
`b30a64b9c35804181ce029a32ea47b9213afb8359529be6913495d4d756b9ea4`,
raw daily limit unchanged at 1T and nominal expiry 2026-10-02T15:07:15.603Z.
This resolves the pending term approval below; expiry enforcement is unqualified.

Fresh owner configuration/roles/fences and unused state match; the payer reports
999.8B. The recovered operator enrolls the original tenant at generation 1; native
tools acknowledge the exact original 1 KiB permission and manifest once. The same
profile/origin/database opens with its exact unsigned row and a real payer identity
loaded only in memory. Offline signing independently verifies before any dispatch.

One actual `_immutableObjectStorageCreateCertificate` succeeds, request
`cafa3f62d248a44af1cdc8a50689d7447542676633ae213bebee7680f455473e`.
The maintained browser client verifies the mainnet certificate and plain upload/root
reply; independent historical verification also matches request/signature/root.
The SDK sends one 7,456-byte streamed tree PUT with that exact certificate and
owner/project/bucket. Caffeine returns HTTP 403, "Owner does not have sufficient
balance". Stop occurs before the chunk; no retry, download, attestation or release.
The readable response establishes this selected HTTPS request/CORS path, not
namespace acceptance, full transfer compatibility or a universal wire-attempt cap.

Post-refusal payer and relationship replies are unchanged; explicit gateway credit,
usage and relationship spend remain zero. A payer-signed owner-account read returns
NotAuthorized, not missing account; public API-version read is empty. These do not
prove the refusal's cause. The freshly retained public example recommends 10T
funding and illustrates a 5T daily limit, but proves neither a minimum nor the
gateway's allocation algorithm. Do not add funds or change terms speculatively.

Private `.tmp/trial-live-01` retains requests, results, failed local preparations,
corrections and hashes. Reopening the original browser profile preserves the exact
observed certificate and one responded 403 claim. The service remains unfenced with
one exposure-possible reservation and 1 KiB reserved/logical/physical/liability
accounting; this is conservative exposure, not proof of provider-stored bytes.
No reset, upgrade/reinstall or deletion/billing-cessation inference is permitted.
Owner, funded payer, relationship, original identities and mutable profile remain
owned resources; local browsers/servers close and temporary credentials are removed.
Execution-wrapper credential literals are redacted before sealing; the redacted
helpers are not retry authority. Gross donor debit remains 3.0001T, authority 96.9999T.

Next: investigate actual owner/gateway credit allocation and documented funding/
daily-limit guidance before a separately reviewed trial. Preserve this stopped
history and its liabilities; do not resend or reset its lifetime slot. Canic stays
deferred, with updated [consumer feedback](../canic-parity.md#integration-feedback)
on admission/refusal handling. Targeted browser/signature/decoding/integrity checks
apply; no build/full CI, version/release/commit, sibling change or build cleanup.

## Earlier actual-service upload packet and browser journal — 2026-10-02

The [source/offline readiness capture](../evidence/caffeine-probes/local/2026-10-02-gateway-admission-01/summary.json)
refreshes public Caffeine/DFINITY trees and immutable blobs. Current Caffeine SDK
bytes match the reviewed 1.1.2; the example README changes only a trailing newline.
The example places budget admission at the gateway during upload. No supported
application credit-grant or effect-free dry-run is demonstrated. Zero gateway
credit remains separate from payer balance and is not a proved readiness blocker;
actual gateway admission and expiry enforcement still require deployed evidence.
Two web searches are unhelpful and the public object-storage repository returns
404; all outcomes and four bounded source fetches remain captured.

Frozen native CLI and maintained SDK prepare/verify a known **1 KiB** file for the
actual installed carrier, project/bucket and trusted tenant/uploader. Permission,
manifest, browser certificate binding, download/status and unsigned tenant-enrollment
candidate independently decode and agree. IDs 1 and the two-hour permission are
planned inputs, not allocation/admission authority. See private
`.tmp/gateway-admission-review-01/upload-inputs` and `live-trial-plan.json`.

A retained one-slot IndexedDB journal now holds those exact unsigned inputs in
`.tmp/trial-browser-profile-01`, origin `http://127.0.0.1:43023`, database
`ic-blob-storage-mainnet-trial-v1`. It survives actual browser-process restarts;
SDK snapshot/root/manifest match native files. The first probe creates the row,
then mistakenly requests unsupported mode `reopen`; corrected `open` sessions
reuse the same profile/origin and preserve the failed run. No history is recreated.
The signer is explicitly a setup-only principal facade: no private key, signature,
certificate or provider request occurs. Real in-memory signing and gateway stream/
CORS/admission remain unqualified. The stopped local server must be restarted at
the exact origin for this journal; port conflict or missing history stops reuse.

Next concrete provider proposal: one payer-signed `payment_account_canister_update_v1`
on the existing relationship, keeping raw daily limit 1,000,000,000,000 and setting
a fresh nominal two-hour expiry for the bounded real trial. The unsigned candidate
and full run plan are retained; exact provider term-change approval is pending
because the earlier approved experiment included **zero extensions**. This is a
new trial proposal, not another expiry-enforcement experiment. Standing 100T
spending authority remains; no covered-spending approval is requested again.
Refresh all state/roles and permission freshness before effects; any uncertain
update/upload stops and preserves the original request and liabilities.

No term change, admission, certificate, upload/download or donor transfer occurs.
Total donor debit stays 3.0001T, remaining 96.9999T. Keep the installed owner, funded
payer, relationship, capture and private unsigned profile as owned resources;
reference release will not prove provider deletion/billing cessation. Targeted
SDK/native decoding, actual browser persistence and integrity checks apply here;
no Cargo build/full CI, release/commit, sibling change or build cleanup occurs.
Canic and its [consumer feedback](../canic-parity.md#integration-feedback) remain deferred/open.

## Approved payer link; expiry experiment inconclusive — 2026-10-02

The maintainer explicitly approves the proposed one-link/90-second experiment,
resolving the earlier automatic approval rejection for this exact mutation. The
[fresh approved capture](../evidence/caffeine-probes/deployed/2026-10-02-trial-link-02/summary.json)
retains refreshed baseline, native inputs, independently verified signature/request
and one successful add-link submission:
`7b00e6c905a6fd41033b6a543536e9dde31305d376cc9295c13d1424b995999c`.
Verified readback exactly matches payer/owner, raw daily limit 1,000,000,000,000 and
expiry `1790943900147000000`, candidate 2026-10-02T12:25:00.147Z. Provider creation
time fits the actual preparation/read interval as Unix nanoseconds; this does not
prove the expiry field's interpretation or enforcement. The old request remains
expired, immutable and unsubmitted; no repeat approval is needed for the completed link.

Before/after candidate expiry, relationship and budget replies are byte-identical:
the relationship remains visible, gateway available credit is zero, and usage/spend
is zero. Exact linkage and owner recognition work, but **expiry enforcement is
inconclusive**. Zero gateway credit does not mean the funded payer is empty;
metadata visibility after the timestamp does not prove continuing spending authority.
Six mainnet-verified logical queries use twelve bounded query/verification transports
without retries, within a 130.265-second observation window. A failed local invented
30-second timestamp threshold is retained; corrected checks use the actual
preparation/read interval without repeating requests/effects.

Private `.tmp/trial-link-02` retains all intents, replies and failure/verification.
No extension, new donor debit, funding, certificate or object request follows.
Total debit stays 3.0001T, remaining 96.9999T under standing 100T authority.
Preserve the relationship, funded payer and installed owner; billing cessation
is not proved. Original payer keys stay private; no password/global identity changes.
No build/CI, version/release/commit, sibling edit, reset or artifact cleanup occurs;
frozen 0.6.0 is reused and concurrent ic-memory 0.15.3 work preserved.

Next: review actual gateway credit allocation/admission and project/bucket/browser
readiness before selecting another exact link term or upload. Do not impersonate
the listed gateway or extend this inconclusive probe. Canic and its
[consumer feedback](../canic-parity.md#integration-feedback) remain deferred/open.

## Live standalone gateway and balance observations — 2026-10-02

The [link-readiness capture](../evidence/caffeine-probes/deployed/2026-10-02-trial-link-01/summary.json)
observes exactly one Cashier gateway, within the installed two-entry/one-unique
bounds. One independently verified operator `blob_sync_gateways` request completes
at sequence 1; local readback contains that exact principal, no pending sync and
all owners unfenced. Upload/read/funding activity remains zero. One independently
verified `blob_inspect_account` Balance invocation through the actual standalone
shared handler reports the installed payer's exact **999,800,000,000 cycles**:
ledger equals total, prepaid/promotional zero. These actual inter-canister reads
establish this discovery/inspection path, not gateway object delivery or future
provider credit guarantees. Local status is an uncertified query observation.

The relationship baseline is absent. Anonymous and mainnet-verified payer-signed
`budget_check_v1` both return typed NotAuthorized; anonymous `budget_get_v1` with
the actual listed gateway returns OwnerNotFound. Do not assume the payer can call
budget_check or interpret either refusal as a balance/expiry result. Two primary
source searches find no Cashier-specific expiry guidance. No link/expiry behavior,
project/bucket acceptance, certificate or object transfer is qualified.

Automatic approval review rejects the saved `payment_account_canister_add_v1`
submission before process launch: it treats the persistent payment relationship
as an account mutation not clearly authorized by continuation plus the spending
ceiling. Do not bypass/resubmit that rejected action. The signed candidate is now
expired, retained and **never submitted**; its historical request is not a retry
candidate. Standing 100T spending authority remains in force. This batch adds no
donor debit: total stays 3.0001T, remaining 96.9999T. The funded payer and installed
owner remain continuing resources.

That run proposed the concrete provider relationship mutation prepared in
`.tmp/trial-link-01/payment-link-proposal.json`: one link from the
installed payer to `4wyfo-qaaaa-aaaam-qjlpq-cai` through installed Cashier, raw daily
limit 1,000,000,000,000 and a freshly retained 90-second absolute-nanosecond expiry
hypothesis. Use a new linked capture and exact fresh signature only after approval;
observe the relationship and explicit-gateway budget before/after, within 180
seconds. Inconclusive/zero/refused evidence stops; no extension, funding, certificate
or object request is included. Never infer total cost or billing cessation from
expiry. One uncertain update stops new effects and requires original-request review.
The subsequently approved experiment above completes that exact add-link action;
expiry remains inconclusive and no extension follows.

Private `.tmp/trial-link-01` retains intents, signatures, typed refusals, successful
shared-handler replies, the automatic rejection and the expired unsubmitted link.
Temporary password files are removed and original participant/verifier keys stay
private. Preserve concurrent Cargo.toml/Cargo.lock ic-memory 0.15.3 work; frozen
0.6.0 artifacts are used without compilation. No full CI, version/release/commit,
sibling change, reset or build cleanup occurs. Canic and its
[consumer actions](../canic-parity.md#integration-feedback) remain deferred/open.

## Funded isolated payer; standing 100T spending authority — 2026-10-02

The maintainer explicitly approves the reviewed deposit/notification and states
"anything up to 100T is preapproved". This is persistent **total isolated trial
spending authority**, superseding the earlier planning-only budget. Do not ask
again for covered spending; preserve selected targets, evidence and budget checks.
It does not authorize repeating uncertain effects, commits, releases or sibling
work. Extra resources/reset/retirement still need their actual scope and obligation
checks. The 100T ceiling is not a proved external provider billing cap.

One exact saved donor transfer succeeds: request
`83ae22b4ab46f99e7906cf829f3b071f22428ddc00d32a0ca37ab00199a8eed9`,
ledger block 16,749,490. Amount is 999.9B cycles and explicit fee is 100M, exactly
1T gross debit. Independent signature/request and exact block sender/destination/
amount/fee/memo/timestamp checks pass; donor debit and isolated deposit balance
reconcile before notification. One payer-signed explicit-account notification
then succeeds: request
`b8b84ae1d10f077fb6b5f29d89fc13774515b9628b528b822f83d8e6449fd335`.
Cashier credits **999,800,000,000 cycles**, with sweep block 16,749,515. Exact
sweep from the isolated subaccount to Cashier's default account and 100M fee
match; deposit balance returns to zero. No intermediate payer ledger transfer,
extra deposit or notification retry occurs.

Mainnet-verified payer account/settings queries confirm the correct new account,
same 999.8B balance, zero overdraft and no target auto-refill. No settings mutation
is needed. One local settings capture collides with an existing filename and
refuses before network dispatch; a separate namespace succeeds. Failure/original
captures remain, and no paid effect repeats. Private `.tmp/trial-funding-01`
retains all intents, signed envelopes, replies, exact ledger blocks and checks.
Temporary donor password files are removed; no key export/global identity import,
deployment, build/CI, release/commit, sibling edit or artifact cleanup occurs.

Total gross donor debit is **3,000,100,000,000 cycles**, including prior 2.0001T
creation and this 1T funding; **96,999,900,000,000 cycles** remain authorized.
Canister execution consumes its already-funded balance; provider credit is not
counted again as another donor debit. Preserve the owner, funded account, keys and
receipts as continuing resources. See the
[funding evidence](../evidence/caffeine-probes/deployed/2026-10-02-trial-funding-01/summary.json).

Next: review/link this payer to the actual owner with explicit raw daily limit and
expiry, then qualify gateway membership/project/bucket and the bounded transfer.
The exact fresh-account direct deposit/credit path is observed; notification replay,
lost-response recovery, future bills and billing cessation are not qualified.
No account linkage, certificate or live upload has occurred. Canic stays deferred;
its [consumer actions](../canic-parity.md#integration-feedback) remain open.

## Earlier selected payer funding proposal — 2026-10-02

The [read-only route review](../evidence/caffeine-probes/deployed/2026-10-02-trial-funding-review-01/summary.json)
observes the installed payer's unchanged provider-returned 32-byte deposit
subaccount. Payer ledger and Cashier deposit-address balances are zero; a fresh
payer-signed, mainnet-verified account-info query reports AccountNotFound. Ledger
fee is 100M cycles and the donor can cover the proposed amount. Five anonymous
CLI queries and one signed provider query run with retained bounded requests/
replies; the signed query uses one query plus one verification read_state, no
retries/root replacement. These are observations, not persistent credit receipts.

Private `.tmp/trial-funding-review-01/funding-proposal.json` prepares one direct
donor-to-Cashier-returned-address transfer: 999,900,000,000 cycles plus explicit
100,000,000 fee, for exactly 1T gross debit under the proposed initial provider
allocation. Donor is canic-mainnet-recovered; isolated payer remains the installed
participant. An intermediate payer ledger transfer would add a fee without proving
provider credit semantics. The direct route is a bounded proposed experiment,
not qualified credit behavior. Only after exact ledger transaction/destination
reconciliation would one notification, signed by the payer with its explicit
account and zero attached cycles, test creation/credit. If there is one 100M
sweep deduction and no others, credit would be 999.8B; this is conditional, not
an established fee/credit guarantee.

Current-DID SDK encoding and independent didc encoding match every candidate
field. Initial optional-wrapper and textual annotation mistakes are retained
alongside corrected inputs; no signing/submission occurs. Freeze the reviewed
exact request/memo/timestamp before an authorized effect; a lost result stops all
new effects. No second deposit, automatic notification retry, refill or presumed
refund is allowed. Never replace an uncertain transaction with a fresh timestamp.
Retain stranded deposit funds and every original request/refusal as obligations.

The subsequently authorized and completed funding above resolves this proposal.
Account linkage, expiry units, project/bucket acceptance and uploads remain open.
This earlier preparation performs no provider
mutation, transfer, certificate, deployment, build/CI, release/commit, key export,
sibling change or artifact cleanup occurs. Canic stays deferred; its
[consumer actions](../canic-parity.md#integration-feedback) remain open.

## Installed isolated standalone owner — 2026-10-02

The maintainer explicitly approves the reviewed installation. Frozen standalone
0.6.0 is installed once on `4wyfo-qaaaa-aaaam-qjlpq-cai` in install mode with the
exact 565-byte actual-service carrier. Private `.tmp/trial-install-02` retains
intent, empty-owner preflight, signed request and reply, status/configuration/local
state, local failures and independent verification. Request ID is
`08d7490c65db8787384ea1651741362447e36017e9bbf9a81f26abf25f045be7`.
No reinstall/upgrade, repeated installation or additional cycles transfer occurs.

Supported gzip transport decompresses byte for byte to the approved raw Wasm
`0766e2b39c1b8eaaf5edbf6cde90908e99125aca3fb77b2975399735a82ef2a2`.
Observed module hash binds its submitted encoding:
`c070f18a5b7c723f9aa22e31631d9d647b6f7eaf8a1cc3e9160af55ee757fbfb`.
The generic CLI's saved management destination is corrected to the signed
argument's effective canister using tagged official source; signed envelope and
request ID remain unchanged. Exact init/configuration/roles read back, compiled
release is 0.6.0 and all local owners are unfenced. Upload/funding/gateway/read
activity is zero, attachment allowance is zero, and anonymous configuration
inspection returns typed Denied. Service query observations are uncertified and
establish no provider readiness. Three local field/URL/Candid checks initially
fail; corrected reads use retained data or a request that never dispatched. No
paid effect is repeated, and all failed observations remain recorded.

Authenticated status confirms Running and the expected single controller.
Balance is 1,487,607,279,785 cycles, reserved cycles zero and idle burn is
1,404,810,943 cycles/day. The 12,380,829,273-cycle pre/post delta includes install,
status execution and elapsed burn; it is not an isolated installation price.
These are snapshots. Prior gross creation debit remains 2.0001T within the
proposed 10T service allocation. Temporary password files are removed; no keys
are exported. Preserve this maintainer-owned resource and the private evidence.
See the [installation evidence](../evidence/caffeine-probes/deployed/2026-10-02-trial-install-02/summary.json).

Next: review the isolated payer's bounded funding/deposit/notification and actual
provider account/project binding. No Caffeine funding, mutation, certificate or
upload is authorized/performed by installation. Canic stays deferred; its
[consumer actions](../canic-parity.md#integration-feedback) remain open. No build,
full CI, release/commit, sibling edits or artifact cleanup occurs.

## Earlier creation and installation preparation — 2026-10-02

One detached empty mainnet canister is created: `4wyfo-qaaaa-aaaam-qjlpq-cai`.
The maintainer selects canic-mainnet-recovered, whose unlocked signer matches the
planned controller `o5trf-oqyg7-cawjp-xs4pw-aomb3-iwki5-hyezf-qahfz-j3ffd-jh4fc-oqe`.
The exact signed request is retained privately before one submission. Independent
request-ID, signature, caller, amount, controller and timestamp checks pass.
Creation succeeds at ledger block 16,749,045, request ID
`68060dd89c931fcf73180ccfed7ed8b2ed714a9002f5b4a84ed58f3d5a73bf1e`.
There is no repeat. A local SDK decoder initially mishandles a Node Buffer's backing
offset; its failure and corrected exact-byte copy are retained without re-signing
or another paid request. Temporary password files are removed after use; password
material and exported keys are absent from the capture/public evidence.

The queried exact block matches caller/timestamp/2T amount/100M fee, and the account
delta is exactly 2,000,100,000,000 cycles. Authenticated status confirms Running,
the one expected controller, zero compute/memory allocation and no installed module.
Observed balance is 1,499,997,545,813 cycles, reserved cycles zero, with reported
idle burn 864,319,605 cycles/day. These are snapshots, not fixed future balances.
Count the gross 2.0001T debit within the proposed 10T service allocation; the
created resource and continuing IC costs remain maintainer-owned. No deletion,
reset, top-up, provider funding, certificate or upload is authorized/performed.
See the [creation evidence](../evidence/caffeine-probes/deployed/2026-10-02-trial-create-02/summary.json).

Private `.tmp/trial-installation-01/installation-proposal.json` binds the actual
service to frozen released 0.6.0 Wasm/DID/CLI and a validated 565-byte complete
init carrier. Operator/controller is the recovered principal; the fresh isolated
payer/tenant/uploader and distinct verifier remain proposed, with the original
project candidate, namespace 1 and one-tenant/object/reference/1 KiB envelope.
Local attachment allocation is fully reserved. Independent DID decoding and
artifact/init hashes pass; local stand-in inputs are not reused. See the
[offline installation evidence](../evidence/caffeine-probes/local/2026-10-02-trial-installation-01/summary.json).

The separately authorized installation and readback above complete this proposal.
Creation authority is fulfilled and permits no second owner; provider/account
effects remain separate. Canic stays deferred, with
[consumer actions](../canic-parity.md#integration-feedback) open. No build, full CI,
release/commit, sibling edits or artifact cleanup occurs.

## Authorized creation preflight — 2026-10-02

The maintainer explicitly authorizes one isolated detached canister using
canic-mainnet, 2T initial creation funding and fees counted against the proposed
10T service allocation. This authority persists; do not ask for it again.
No installation, provider funding, certificate or upload is authorized by that reply.

Private `.tmp/trial-create-01` retains the original intent, current official
cycles-ledger DID, CLI help, public metadata, balance/fee replies and failures.
Stored public identity metadata matches the proposed controller/operator and the
default identity is unchanged. Anonymous queries report sufficient cycles and a
100M ledger fee: the proposed source-account debit is 2,000,100,000,000 cycles.
Primary IC cost documentation distinguishes the creation fee deducted from the
new canister's funding; its actual remaining balance/subnet must be observed,
not reported as an untouched 2T. No mint/top-up is authorized or needed by this
balance observation. All values are preflight snapshots, not transaction receipts.

The original signer-loading attempt fails before any network request because
canic-mainnet requires a password and the non-interactive CLI has no terminal.
That preflight signs/submits no creation call. The maintainer later resolves
access with canic-mainnet-recovered; the subsequent run above creates the one
authorized owner. Do not search for or export passwords/keys, change the default
identity, or treat public metadata as proof of signer access. The
[preflight evidence](../evidence/caffeine-probes/deployed/2026-10-02-trial-create-preflight-01/summary.json)
keeps that failure distinct from the successful anonymous observations.

That run's next step was signer access and refreshed preflight before one signed
cycles-ledger create_canister submission; the run above completes it.
The retained argument template fixes amount/controller/compute allocation and
requires an explicit created_at_time for transaction identity. Its encoded sample
is unsubmitted schema evidence, not the eventual paid request. This is the same
authorized detached creation effect as the higher-level CLI proposal. An uncertain
reply requires inspection of the original request/block; never create another
owner or regenerate a paid transaction to obtain a clearer result. Capture any
returned principal, creation block and actual balance before offline init generation.
Installation remains separately authorized; Canic stays deferred and its
[consumer actions](../canic-parity.md#integration-feedback) remain open.

## Released artifact review and next step — 2026-10-02

The private `.tmp/trial-release-01` packet freezes the released source archive,
receipt, toolchain/configuration, standalone Wasm, native CLI, DID and trial
template with exact hashes. Production source matches the released commit;
the changed test source is retained separately. The release receipt verifies its
release files but does not itself bind Wasm, so the packet records that hash
separately rather than treating a version label as artifact identity.

The maintained installation test now encodes the actual trial template against
the declared DID, checks equality with the host-exported interface, and installs
the CLI-produced carrier in PocketIC. Exact configuration, project, uploader and
verifier readback pass; the Wasm reports 0.6.0. Wrong-service installation refuses
without changing the existing owner, controllers cannot inspect configuration,
and the template admits the restricted local certificate assessment. The focused
case passes in 3.43 seconds; strict affected integration lint and formatting pass.
An independent DID decoder reads both the actual local readback and the separately
prepared candidate init. Both sets of init bytes use local service stand-ins and
must never be deployed. See the
[release-artifact evidence](../evidence/caffeine-probes/local/2026-10-02-trial-release-01/summary.json).

The one-canister creation proposal remains in
`.tmp/trial-provisioning-01/creation-proposal.json`: explicit canic-mainnet,
2T initial cycles, fees/balance checked against the proposed 10T service allocation,
no automatic retry or install. The maintainer subsequently authorizes creation;
the creation above is complete. After successful creation,
replace the stand-in with the actual principal and
recheck the exact init before separately authorized installation. No deployment,
funding, provider request, live certificate/upload, full CI or cleanup occurs here.
Canic stays deferred; its [consumer actions](../canic-parity.md#integration-feedback)
remain open. The implementation history below records pre-release artifacts;
its earlier 0.5.0 labels do not identify the current release packet.

## Current trial preparation and next step — 2026-10-02

The original packet is authorized for configuration/account-provisioning preparation;
the separately authorized owner creation is completed above.
The [maintained standalone envelope](../../canisters/standalone/trial/README.md)
sets one lifetime tenant/object/reference and one 1 KiB chunk, two retained
receipts, one upload/read slot and 1 KiB buffers. Local cycle attachment allocation
is entirely reserved (one cycle allocated/reserved); it permits no service offer.
This is local funding policy, not an external spending cap or effect.

Private `.tmp/trial-provisioning-01/proposal.json` proposes existing canic-mainnet
as deployer/controller/operator, one freshly generated repository-local principal
explicitly sharing payer/tenant/uploader roles and a distinct fresh verifier.
Private PEM/browser identity files are 0600 inside a 0700 directory; SDK/public DER
principal agreement and a local signature check pass. No existing private key is
read/exported and the global identity manager/default remains unchanged. Fresh
UUID project/bucket, namespace 1 and dedicated profile/origin/one-slot database are
proposed, not selected or provider-provisioned. Keep these keys and history private.

The exact envelope validates through the maintained offline installation-check
with an explicitly labelled local service stand-in. Independent DID decoding
passes; those local-only init bytes must never be deployed. Deposit-subaccount,
notification and zero-overdraft/no-target request candidates also encode/decode
against the retained Cashier DID without dispatch. The
[preparation evidence](../evidence/caffeine-probes/local/2026-10-02-trial-provisioning-01/summary.json)
retains initial sandbox/encoding/decoder failures and corrected independent runs.

Next: select the proposed roles and reviewed source/Wasm/DID; obtain the actual
service principal only through separately authorized creation, then recheck init.
Cashier's advertised direct cycles-ledger deposit route could avoid a wallet/proxy,
but account creation/credit/mutation-caller semantics, actual fees, project
acceptance and raw expiry units remain unresolved. Never infer lazy creation from
the absence of an account-create method. Follow the
[provisioning sequence](../operator-guide.md#prepare-isolated-trial-provisioning)
before funding; no account-specific/provider-object query, mutation, deployment,
funding or certificate/upload occurs in the offline packet. In a separate bounded
[Cashier observation](../evidence/caffeine-probes/deployed/2026-10-02-trial-payer-probe-01/summary.json),
the fresh payer candidate receives a 32-byte provider deposit subaccount. Anonymous
account-info is NotAuthorized; a signed query using only the newly generated key
returns AccountNotFound with mainnet query-signature verification. It uses one
query and one read_state transport call, no SDK retries/root replacement. Raw signed
artifacts remain private in `.tmp/trial-payer-probe-01`; the account is not created,
funded or linked. Query observations are not persistent creation/credit receipts.
First deposit/notification remains a separately reviewed effect with unresolved
credit/authorization/fee semantics. The 100T planning split is
unchanged and no initial funding amount/refill is selected. Monitoring and
continuing-obligation disposition remain open. Canic stays deferred; its
[consumer actions](../canic-parity.md#integration-feedback) remain a future backlog.

## Current transport result — 2026-10-02

Uploads, independent verifier reads and tenant downloads now share one owned
HTTPS/HTTP2 gateway in all four actual local restricted journeys. CLI reqwest
explicitly enables rustls/http2; its CLI-only feature graph previously lacked
HTTP2. Dependency versions and the maintainer's ic-memory 0.15.2 remain unchanged.
Normal native TLS validation uses a generated public test CA in each Linux child.
An unrelated CA refuses before HTTP; an H2 REFUSED_STREAM read fails without
implicit retransmission. Both retain failed observations without statements before
the separately budgeted successful verification. Lost upload replies and withdrawal
retain the same uncertainty, reference-release and byte-liability behavior.

Four journeys pass (20.02 seconds), with two PUT arrivals per owner and seven GETs
total, including the refused read. All 66 native CLI tests, strict affected CLI/
standalone lint and formatting pass. The shared TLS helper's eight transport cases
also pass; initial sandbox-denied Chromium startup is retained separately.
See the [HTTPS evidence](../evidence/caffeine-probes/local/2026-10-02-https-journey-01/summary.json)
and [ledger](../evidence/caffeine-probes/README.md#same-origin-https-and-public-gateway-metadata--2026-10-02).

Three anonymous curl invocations at source-listed blob.caffeine.ai negotiate
verified TLS/H2: root HEAD returns 400; tree/chunk OPTIONS return 200 advertising
PUT and SDK headers. The [public metadata](../evidence/caffeine-probes/deployed/2026-10-02-gateway-stream-01/summary.json)
and [immutable clarification](../evidence/caffeine-probes/deployed/2026-10-02-gateway-stream-01/clarifications.json)
qualify this metadata only. X-Dry-Run semantics, authenticated streaming,
provisioning and economics remain unqualified. No object request, account binding,
certificate issuance, deployment or payment is performed against the live provider.

Next: finalize isolated live roles/account/project/gateway and persistent browser/
cleanup ownership, then review exact bounded effects. Local transport readiness is
established; further repetitions of the same substitute cannot qualify Caffeine's
authenticated behavior. Future [consumer actions](../canic-parity.md#integration-feedback)
include preserving streams, normal TLS trust and native HTTP2 support; Canic stays
deferred. No full CI, minimum-compiler rerun, release/version/commit, sibling edits
or build cleanup occurs in that run. Those changes shipped in 0.6.0.

## Active work — accepted restricted upload contract

On 2026-10-02 the maintainer explicitly accepts the
[restricted contract](../standalone-trial.md): one trusted uploader, one fresh
storage owner/tenant/object/reference and a nonempty file of at most 1 KiB,
without provider spending-cap/replay guarantees or operational old-backup recovery.
This breaking semantic/init/API change shipped in the maintainer's 0.6.0 minor
release. Its original validation used pre-release builds labelled 0.5.0; the new
artifact review above uses the released source and observed 0.6.0 Wasm.

The current certificate evidence/DTO/CLI blockers are TrialBounds, NamespaceBinding,
TrustedUploader, CurrentOwner, Durability and StaleObservation. Required immutable
`trusted_uploader` is validated before allocation, retained in the current v1
installation record and configuration readback, and checked against the original
tenant-approved permission. Shared installation derives local evidence; standalone
has no hardcoded provider flags or operator bypass. Broad installation envelopes
refuse issuance. Exact preparation, activation, expiry, phase, one-time exposure,
revocation/uncertainty accounting and all-owner restoration fences remain.

Provider provisioning, certificate lifetime/replay charges, funding controls,
retention, deletion and billing cessation are still unqualified, not newly true.
Loading an older management snapshot restores the heap too and can resurrect local
eligibility and lose later exposure/revocation/accounting. This demonstrates why
snapshot activation is unsupported; no local unfenced flag proves freshness.

The 100T-cycle total planning ceiling and proposed 10T service / 1T initial provider /
89T unallocated split remain recorded. Actual principals/account/namespace/gateway,
selected persistent browser environment, raw provider terms, monitoring/retention window and
cleanup owner remain unselected. Contract acceptance does not select targets or
authorize deployment, funding, certificates or provider requests in a live trial.
Capture: `.tmp/restricted-contract-01`; the probe ledger records local evidence
separately from provider qualification. Full CI/release validation is not authorized.

All 48 standalone integration cases pass against retained Wasm/CLI/test-executable
copies, including successful issuance, rejected untrusted uploaders, typed local
refusals, signed CLI workflows, stop/start and inspection-only restore. The snapshot
case explicitly demonstrates lost exposure/revocation and resurrected eligibility
on unsupported heap rollback. The shared upload-store, installation, envelope and
CLI tests pass; strict affected all-target/all-feature Clippy and formatting pass.
Current required-uploader host init independently encodes/decodes using the DID;
the offline installation checker preserves exact candidate inputs without effects.
The earlier target-loss run remains retained; the maintainer confirms running
cargo clean during it. This agent initiates no cleanup. Seven actual local IC
shared-exposure cases and the ten-scenario Chromium/SDK gateway-substitute suite
also pass. The [retained summary](../evidence/caffeine-probes/local/2026-10-02-restricted-contract-01/summary.json) separates actual local facts,
fixture substitutes, unsupported snapshot observations and live limitations.

The next local browser step is complete: `clients/browser/intents.js` supplies the
maintained bounded IndexedDB certificate/gateway journal. Explicit create/open
refuses existing/missing databases and changed capacity; opening a missing store
does not leave an empty replacement. Strict serialized transactions preserve
uncertainty, cancellation and bounded gateway history. Rows are validated and
caller arguments snapshotted; no reset, eviction, migration or automatic retry API
exists. The former fixture implementation is replaced by calls to this same owner,
with fault injection only in the test platform wrapper.

Actual Chromium checks pass for concurrent claims, graceful browser-process restart,
retained cancelled capacity, malformed requests and corrupt-history refusal. The
ten-scenario Chromium/PocketIC/SDK suite also passes (49.94 seconds), including real
aborted claims/observations and late cancellation against a local gateway substitute.
The initial upload run fails because extra fixture identities overflow its deliberate
u128::MAX permission; corrected bounded fixture inputs and the failed log remain in
`.tmp/browser-intents-01`. No Rust source/build or live provider request is involved.
See the [retained browser summary](../evidence/caffeine-probes/local/2026-10-02-browser-intents-01/summary.json).
The actual trial profile/origin/database and authenticated application still need
selection; eviction, power loss, rollback and hostile-origin scripts are unqualified.

The complete local restricted-host journey now passes through
`make test-browser-standalone`: actual installed standalone certificate facts,
Chromium SDK preparation/upload, maintained one-slot journal and reload recovery,
native whole-byte verifier observation, a distinct verifier's signed attestation,
tenant verified download and reference release. Logical release leaves 1,024
physical/liability bytes. A separate fresh owner receives corrupt provider bytes:
no statement or attestation is created, tenant download stays unavailable, and
withdrawal/browser cancellation retain ExposurePossible and its byte obligations.
Both cases pass (8.46 seconds) with actual local IC/Chromium/native tools and an
owned gateway substitute, two bounded PUTs per owner and two/one GETs respectively.
The provider serves uploaded bytes, not an independent prefilled fixture body.

Strict affected standalone/storage test lint, formatting and the target's builds
pass. Current Wasm/CLI hashes match the earlier retained artifacts. Initial compiler
mistakes (digest hex formatting and the nested revocation response field) and
corrected outcomes remain in `.tmp/standalone-browser-01`; the
[retained summary](../evidence/caffeine-probes/local/2026-10-02-standalone-browser-01/summary.json)
separates host facts from provider substitutes. The original provider-fact fixture
suite remains separate. No production API, version, live deployment/funding,
provider call, allocator or downstream dependency changes in this step.

## Complete offline installation input and upstream review — 2026-10-02

The core now owns passive `dto::configuration::ServiceInstallationInput` with
configuration, project, completion verifier and trusted uploader. The standalone
input is removed without an alias; exported Candid and every Rust consumer use
the shared type. Lifecycle/authentication/allocation remain host responsibilities.
`installation-check` emits complete `installation.candid` and its hash beside the
unchanged original configuration after full shared validation. Encoding is true;
authentication, actual platform/compiled-release checks and every effect remain false.
This public API hard cut joins the existing minor-release draft; version stays 0.5.0.

Actual local installation accepts the CLI-generated bytes unchanged, reads back
all roles/project/release and permits the restricted preparation assessment. A
different proposed service validates offline but the actual host rejects it,
preserving its previous heap/configuration/stable bytes. Independent DID decoding,
shared/CLI/exported-contract tests, strict affected lint and current Wasm builds
pass. The first growth invocation has a wrong fixture variable and fails before
installation; the corrected actual refusal/rollback/exact retry/fenced restore
check passes. Initial command, lint, patch and metadata-lock failures are retained,
with no silent retry of a provider effect. See the
[current carrier evidence](../evidence/caffeine-probes/local/2026-10-02-installation-carrier-01/summary.json)
and `.tmp/installation-carrier-01`.

The maintainer updates Cargo to ic-memory 0.15.2; that change is preserved.
One registry runtime resolves. The cached published changelog and clean upstream
`e2fe658` agree: 0.15.2 addresses IcyDB lint feedback and typed diagnostic/Wasm
test cleanup, retaining read forwarding, API/schema and declared Rust 1.88 MSRV.
There is no unsafe read override here to adapt. The linked GitHub issue-body fetch
fails; no issue status/body is inferred. Development Rust 1.99 targeted checks
pass, including actual installation and typed growth refusal/retry/restoration.
The minimum compiler is not rerun; earlier MSRV evidence remains historical.
See [dependency review](../dependencies.md#memory-composition).

Read-only identity metadata confirms local `canic-mainnet` at public principal
`o5trf-oqyg7-cawjp-xs4pw-aomb3-iwki5-hyezf-qahfz-j3ffd-jh4fc-oqe`.
The default stays `toko-miner-local`; no keys are exported, signer capability
tested, identity changed or roles assigned. This is a candidate, not selected
deployment/account/provider authority. Next: finalize exact isolated live roles,
account/project/namespace, browser history and cleanup ownership, then review
the complete carrier and concrete bounded action before any live effect.

## Immutable browser namespace handoff — 2026-10-02

Native `upload-inputs` now requires explicit project/bucket in its original JSON
and emits both in `certificate-binding.json`. The existing browser certificate
intent/journal retains them before issuance; transfer derives owner/project/bucket
from that intent, with no independent namespace options. Changed values refuse
without dispatch across setup, tabs and reopening. The current v1 schema is replaced
directly, joining the minor-release draft without aliases, migration or a new owner.
Core permission/configuration/provider wire and the allocator remain unchanged.

Native and real Chromium checks cover 256-byte namespace bounds, header-compatible
project/UTF-8 bucket, invalid controls/Unicode and unchanged history after conflicts
and browser restart. Strict affected CLI/integration lint and formatting pass.
Pinned SDK substitute checks and the 1 KiB native/browser snapshot handoff pass.
Both actual standalone Chromium/native journeys pass (8.37 seconds in the final
bounded-client run; the earlier 9.27-second run is retained), with namespace
conflicts before certificate dispatch, correct tree project/bucket, verified download
and release, and corrupt-download refusal retaining exposure/liabilities. The local
gateway and account setup remain substitutes; no live provider facts are qualified.
Initial patch and function-length lint failures remain in `.tmp/browser-namespace-01`.
See the [retained summary](../evidence/caffeine-probes/local/2026-10-02-browser-namespace-01/summary.json).

Consumers must supply these original fields and use the tightened transfer API;
the [feedback list](../canic-parity.md#integration-feedback) records that open action.
Review project against actual installation and bucket against provider provisioning.
Live roles/account/project/browser/cleanup targets are still unselected; existing
`canic-mainnet` metadata supplies only a signer candidate. No deployment, payment,
version, commit, sibling mutation or build cleanup occurs.

## Installation-bound upload preparation — 2026-10-02

`upload-inputs` now requires `--installation` with complete current
`ServiceInstallationInput` Candid. One shared bounded exact candidate decoder
serves installation-check and upload preparation. Shared validation and original
service/local namespace/project/trusted-uploader matching precede output; candidate
object/chunk/header bounds constrain manifests alongside existing CLI ceilings.
Successful output retains exact init bytes/hash. This is a local consistency check,
not actual installed-state, remaining-capacity, release or provisioning authority.
Core ABI/stable schema, provider wire, allocation and owners remain unchanged.
This command hard cut joins the current minor-release draft; version stays 0.5.0.

Targeted upload-input and installation-check cases, strict affected CLI/integration
lint, formatting and diff checks pass. Offline pinned SDK/native handoffs pass at
1 KiB and the unchanged 10 MiB default, with all networking refused. Both actual
standalone Chromium/native journeys pass (10.02 seconds), including exact carrier
retention, verification/attestation/download/release and corrupt-download exposure.
Signed lost/pending-reply recovery, cancellation and fenced restore pass (8.07
seconds) using an installation that declares its actual uploader. An invalid
metadata-budget fixture and outdated untrusted-uploader assertion fail initially;
failed logs and corrected cases remain retained. The
[summary](../evidence/caffeine-probes/local/2026-10-02-upload-installation-01/summary.json)
and `.tmp/upload-installation-01` separate host facts from gateway/account substitutes.
No live effect, full CI, minimum-compiler rerun, release/commit or cleanup occurs.

Canic adoption remains explicitly deferred. Its future consumer must supply the
complete reviewed carrier when invoking native preparation; the
[feedback list](../canic-parity.md#integration-feedback) records that action without
requiring sibling work now. Continue repository-local readiness work before that
integration. Live account/gateway/project/browser/roles/cleanup remain unselected.

## Standalone interruption and transport limits — 2026-10-02

The maintained standalone/browser rehearsal now covers loss of the final response
body after the gateway receives the chunk. Certificate recovery preserves the
uncertain gateway claim; supported owner stop/start preserves configuration and
exposure. Independent verification, explicit attestation and tenant download work
without another upload dispatch. A separate case withdraws tenant permission after
whole-byte observation: the current contract permits late completion for accounting,
then exact reference release ends liveness. Attestation replay leaves that reference
inactive; browser cancellation and physical/liability bytes remain retained.
No production API/schema, owner, allocation, provider wire or dependency changes.

A failed pre-header-loss experiment and fresh diagnostic establish a separate
local limitation: Chromium 153 sends one chunk twice after the reset, below the
guarded fetch hook, despite disabled SDK retries. The server sees three PUTs from
two journal claims and rejects the repeat. Client budgets bound fetch dispatches
and claimed bodies, not all wire transmissions or replay charges. Do not treat
the successful truncated-body experiment as qualifying pre-header retry safety.
The initial late-refusal assumption is corrected from maintained source, rather
than changing the lifecycle to fit it. All failed runs/source/fingerprints remain.

All four actual local standalone/browser/native cases pass in 17.84 seconds with
the distinct body-truncation cut; normal/corrupt/lost/withdrawn cases have two PUTs
each and two/one/two/one GETs. Strict affected integration lint, formatting and
diff checks pass. Unchanged Wasm/CLI/SDK artifacts are reused. The
[summary](../evidence/caffeine-probes/local/2026-10-02-standalone-interruption-01/summary.json),
[transport diagnostic](../evidence/caffeine-probes/local/2026-10-02-standalone-interruption-01/transport-diagnostic.json)
and `.tmp/standalone-interruption-01` separate actual host facts, local browser
transport observations and gateway/account substitutes. No live effects, full CI,
minimum-compiler rerun, version/release/commit, sibling work or build cleanup.

Canic adoption remains deferred. The [feedback list](../canic-parity.md#integration-feedback)
adds future cancellation cleanup and transport-budget actions: consumers must keep
exact release intent for exposed work and cannot infer wire-attempt caps from SDK
retry settings. Continue useful repository-local readiness work; live account/
gateway/project/browser/roles/cleanup selection and provider economics remain open.

## Browser replay repair — 2026-10-02

The maintainer requests a fix for hidden Chromium PUT repetition. Current browser
transport now emits each snapshotted SDK payload through an immediately closed
ReadableStream with duplex:half, requires HTTPS/request-stream support and never
falls back to buffering. HTTP/1.x refuses; actual HTTP/2-or-3 negotiation and CORS
must be qualified on the selected live gateway. SDK formats, core schemas, journal
owner and dependencies remain unchanged. This transport hard cut joins the
minor-release draft; package/version/release receipt remain 0.5.0.

Owned Chromium 153 TLS checks pass for connection loss, immediate receipt-close
and REFUSED_STREAM: the current stream has one arrival and an uncertain claim;
buffered fetch, keepalive:false and XHR controls repeat. The earlier first matrix
warms the wrong credentials pool; its distinct corrected runs remain. Exact-tag
Chromium sources show stream replay caching, so these checks establish a mitigation,
not universal exactly-once or provider charging guarantees. Web-source errors and
three successful bounded exact-tag source fetches are retained.

All four actual standalone/browser/native journeys pass in 18.06 seconds with the
original pre-header cut restored, two planned PUT arrivals per owner and no new
upload during verification/reload/stop-start. Native reads use a separate HTTP
loopback origin serving the same uploaded bytes; deployed same-origin/TLS compatibility
is unqualified. Ten existing certificate/gateway scenarios pass in 53.11 seconds;
six SDK substitute cases pass with unchanged application payloads. An initial
recovery probe targets the page rather than upload origin and fails all four
assertions; corrected source/run and failures remain in `.tmp/browser-replay-01`.
See the [repair summary](../evidence/caffeine-probes/local/2026-10-02-browser-replay-01/summary.json)
and [ledger](../evidence/caffeine-probes/README.md#browser-replay-repair--2026-10-02).

Future consumers must preserve the outgoing stream in trusted fetch hooks and
select compatible HTTPS gateways/browsers; the [feedback list](../canic-parity.md#integration-feedback)
records this alongside exact cancellation/reference-release ownership. Canic
adoption remains deferred. Live bindings/stream support, provider economics and
persistent-browser qualification remain open. No live effects, release/commit,
sibling mutation, allocator change or build cleanup occurs.

## Earlier trial preparation — retained 2026-10-02

The prior [review capture](../evidence/caffeine-probes/local/2026-10-02-standalone-trial-review-01/summary.json)
remains immutable and describes the then-unaccepted proposal and former four-false
host gate. Offline SDK/native rehearsals pass at 1 KiB and unchanged 10 MiB default,
including saved snapshots, repeat/corrupt-source refusals and invalid sizes before
output. Shared configuration validates; its prior host-init encoding predates the
new required uploader field and must not be used as a current installation carrier.
Fresh source/npm hashes remain unchanged at Caffeine 1.1.2; two primary web lookups
fail. One public Cashier unit query yields valid reported cycles data then fails its
file-size guard (exit 153), without retry. The earlier 0.5.0 Wasm/refusal check remains
historical. All failures, replies and hashes are retained in the
[probe ledger](../evidence/caffeine-probes/README.md), without paid effects.

## Completed in 0.5.0 — Rust compatibility and development toolchain

The maintainer requests a lower justified MSRV and development Rust 1.99.0.
The workspace minimum is now 1.88.0 for the library, standalone, CLI, examples
and local canister fixtures. The unpublished PocketIC harness alone declares
1.89.0 because its native journal uses standard-library file locking. Locked
dependency declarations and maintained `slice::as_chunks` use establish the
1.88 floor; actual native and Wasm checks verify the supported packages.

Development Rust, Clippy and rustfmt are independently pinned to 1.99.0. Its
strict all-target/all-feature workspace lint, formatting and Wasm check pass.
The new assertion lint is addressed with equivalent comparisons and one
result-use annotation; five recovery delays use the same 24-hour value through
the older seconds constructor. No lint suppression or dependency update is added.
The [dependency guide](../dependencies.md#rust-versions) records repeatable
minimum-toolchain checks. Intent, metadata, compiler identities, initial failures,
fixer limitations and final logs remain in `.tmp/msrv-toolchain-01`.
These checks predate the maintainer's 0.5.0 release; the capture remains unchanged.

## Completed in 0.5.0 — independent library and ic-memory 0.15

The maintainer explicitly removes Canic 100% from this repository, including its
adapter and tests. The Canic adapter crate, managed canister fixture, complete
managed PocketIC suite, dependencies, targets and framework-specific inventory/
parity documentation are deleted. Core and standalone remain, alongside native
and browser tooling and framework-free local IC fixtures. AGENTS now assigns
consumer wrappers and integration tests to the consumer repository. No sibling
repository has been edited. There is no opt-in Canic suite or framework fallback
left here; ordinary validation and releases require no Canic tool/publication.

The official Cargo index identifies ic-memory 0.15.0 as the latest non-yanked
release. The root pins =0.15.0 and resolves one runtime identity without any Canic
package. Direct growth callers use typed results; generic stable-memory wrappers
retain the upstream trait contract. The real refusal fixture accepts only typed
BackingRefused with unchanged extent before deliberately trapping after writes.
Shared installation, tenant rules, grants and all-owner restore fences remain.

Workspace native check and strict all-target/all-feature lint pass. Shared-store
and installation unit cases, standalone exported Candid and release Wasm builds
pass. Actual typed growth-refusal rollback and standalone stop/start/repeated fenced
restore checks pass. Intent, official index, graph, logs and prior editing failure remain in
`.tmp/memory-independent-release-01`; earlier sealed captures remain unchanged.
Framework-free Wasm check, formatting, warning-free core/standalone rustdoc,
package verification and maintained documentation/graph checks also pass.
These recorded implementation checks predate the maintainer's 0.5.0 minor release,
which includes the public memory API change and complete adapter removal.

## Maintained functionality and next work

The library owns durable tenant enrollment, upload permissions/manifests, references,
receipts, indexed accounting, read sessions, gateway state, funding intents and
uncertain provider effects. The standalone host delegates to shared handlers and
restores synchronously into inspection-only fences. Native tooling prepares exact
inputs/snapshots, authenticates service requests, verifies complete downloads and
records/retrieves verifier statements. Browser tooling composes the maintained
Caffeine SDK with one certificate/gateway journal and disabled retries.

A complete live Caffeine upload/download trial remains unqualified. Restricted local
issuance is implemented under the accepted trusted-uploader/fresh-owner contract.
Select exact live identities, account/namespace, raw provider terms, bounded client
traffic, the actual persistent browser environment and funded obligation ownership
before effects. Use the maintained journal with one lifetime slot for the trial,
explicitly reopen it after reload, and stop if the selected history is missing.
The 100T-cycle planning ceiling is not a guaranteed maximum external bill.
Offline installation-check requires --trusted-uploader, writes complete init bytes
and does not deploy.
See the [trial plan](../operator-guide.md#isolated-uploaddownload-trial-plan),
[acceptance](../acceptance-plan.md), [contract](../service-contract.md) and
[probe ledger](../evidence/caffeine-probes/README.md).

## Consumer integration feedback

The [current action list](../canic-parity.md#integration-feedback) records these
open Canic consumer actions; no sibling work or messages are authorized.
Wrappers now import `ServiceInstallationInput` from the core and independently
check actual service/compiled release; the former standalone type has no alias.

Canic owns its own wrapper and integration tests against the public library;
new consumers must supply explicit trusted_uploader installation authority and
adopt the restricted certificate DTO/semantic hard cut, without controller fallback.
Consumers selecting browser upload projects must also satisfy the SDK's project
HTTP-header validation and preserve the same owner/project/root for both download
paths; service UTF-8 project validation alone is insufficient for the browser SDK.
That work has not been performed in its repository. Consumers must adopt one
ic-memory package identity, supply explicit caller/service and allocation grants,
use shared workflows, and preserve synchronous restoration and fences. A framework's
activation or controller status cannot grant tenant, verifier or provider authority.
This is consumer work, not a library release dependency. No upstream messages were
sent and no sibling edits are authorized by this removal.

Toko review uses remote development; the local checkout is absent/stale. The
[source review](../evidence/toko-0.2-review.json) and
[Miner findings](../roadmap.md#toko-miner-feedback--2026-09-27) retain consumer input,
not approved adoption or production sizing. Real application transactions,
consumer outbox/worker acceptance and operational restart remain open.

## Constraints and history

Pre-1.0 is a hard cut: current APIs/schema only, no compatibility/migration paths.
Cross-release transitions are reinstall-only after obligations are preserved or
discharged. Same-release recovery and retry remain required. Removing source cannot
erase external objects, uncertain effects, balances or continuing billing.
The allocator is unchanged. Do not deploy, pay, commit, publish or clean builds
without the appropriate explicit authority. Use targeted checks during development.

Earlier source-bound implementation and removed integration observations remain
in [core evidence](../evidence/core-primitives.md), immutable release history and
sealed captures. Their former adapters, commands and acceptance claims are historical,
not a maintained integration contract. This batch retains the fresh public source
review and failed read-only unit query separately from local preparation. No paid
effect, live certificate issuance or provider object request occurs.
