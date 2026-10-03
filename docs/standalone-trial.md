# First standalone Caffeine trial

The maintainer **accepted the initial restricted contract on 2026-10-02**; it
shipped in 0.6.0. The later request to relax the 1 KiB limit for Toko Miner changes
the current source contract to use validated installation resource limits.
The first live owner's frozen 0.6.0 configuration and evidence remain intact.
Contract acceptance does not
authorize deployment, funding or provider traffic. The objective
is one actual upload, an independent verified fetch, and a tenant download.
The [existing run sequence](operator-guide.md#isolated-uploaddownload-trial-plan)
and [probe ledger](evidence/caffeine-probes/README.md) own command/evidence handling.

## Local issuance prerequisites and provider limitations

| Blocker | What we have | What is still needed |
| --- | --- | --- |
| Resource admission | Installed object size, tenant/global byte quotas, lifetime history, leaves and concurrency limits bound each admitted/prepared reservation | Size the installation from the consumer inventory and retention horizon; larger and multi-file configurations may issue certificates |
| Uploader trust | Required immutable `trusted_uploader`, scoped to service and local namespace | Select a trusted participant and exact tenant-approved permission; tenant approval alone cannot grant this trust |
| Local namespace | Original service owner/root and explicit installed project/local namespace | Select and provision the actual payer/project/bucket relationship; local matching does not prove provider provisioning |
| Current owner/durability | One synchronous exposure commit/reply; stop/start and inspection-only upgrade restoration | Keep the original owner active without snapshot loading; old-backup activation is unsupported and can restore lost authority |

The current local blockers are `NamespaceBinding`, `TrustedUploader`,
`CurrentOwner`, `Durability` and `StaleObservation`. Provider pre-charge, duplicate
charging, provisioning and backup-freshness guarantees are outside this contract;
they are not facts set true from local success. No ingress qualification flags or
operator bypass are available. An empty assessment does not reserve issuance.

Cashier's retained public interface exposes daily relationship limits, expiry,
account overdraft settings and gateway credit. This identifies controls to inspect;
it does not prove a global cap, pre-charge enforcement or timely gateway revocation.
The historical cycles integration guide and the current retail credit help page
describe different billing products. Do not import credit prices, prepaid periods
or balance-exhaustion behavior as facts about this cycles account.

A fresh anonymous [unit query](evidence/caffeine-probes/deployed/2026-10-02-pricing-units-01/summary.json)
produced independently decodable unit data identifying cycles as Cashier's internal
terminal currency. The process then failed its file-size guard; the failed outcome
and reply are both retained, without a retry. This does not resolve request-factor
semantics, account-limit enforcement or expiry units.

## Current upload sizing

The service imposes no separate 1 KiB or single-object certificate limit. Hosts
select `ServiceResourceInput` explicitly; admission and manifest preparation
enforce those limits before issuance. A 10 MiB object maximum covers the largest
asset in the recorded Miner feedback (8,362,256 bytes), but a complete release also
needs adequate object/leaf/reference/receipt capacity and physical/liability
budgets across overlapping releases. Read-session and client transfer/download
limits are configured separately; raising an object limit does not raise them.
Uploader trust, exact permission, one-time exposure, current-owner fences and
continuing billing obligations still apply. Larger live transfers remain unqualified.
Removing the former public trial gate requires a minor release and cross-release
reinstall after obligations are preserved or discharged; it does not reconfigure
the existing live owner or authorize deploying a consumer installation.

## Fresh 0.7.0 trial proposal

The [offline preparation record](evidence/caffeine-probes/local/2026-10-02-trial-v070-preparation-01/summary.json)
retains frozen source/Wasm/DID/CLI artifacts and a complete validated candidate.
It uses the local service stand-in `rrkah-fqaaa-aaaaa-aaaaq-cai`; **never deploy
its init or use its account-link/upload inputs on mainnet**. Regenerate exact
bindings for the fresh creation receipt's principal and a current deadline.
Existing identities remain in their original private files; no keys are exported.

| Selection | Proposed value |
| --- | --- |
| Operator/controller and source | Existing `canic-mainnet-recovered`, `o5trf-oqyg7-cawjp-xs4pw-aomb3-iwki5-hyezf-qahfz-j3ffd-jh4fc-oqe` |
| Payer, tenant and trusted uploader | Existing isolated `jaqw3-42pqs-5bivi-nj3xg-5row5-gpluj-sbn54-vthud-pyb5v-sm2rs-bae` |
| Independent verifier | Existing `n67bb-yqikw-jbye7-anfpz-tgws7-akhqy-kdpln-ihuix-hzdjf-3ghwy-4ae` |
| Provider | Cashier `72ch2-fiaaa-aaaar-qbsvq-cai`, gateway `https://blob.caffeine.ai`, verified mainnet IC root |
| Owner | One newly created standalone 0.7.0 canister; no upgrade/reinstall of the old owner |
| Namespace / project | Local namespace `2`; candidate project `d0770e45-29b6-4449-b499-961aea5daa47` |
| Bucket | `ic-blob-storage-trial-d0770e45-29b6-4449-b499-961aea5daa47`; provider acceptance unqualified |
| Creation proposal | 1T amount, maximum 1.0001T gross source debit including the historical 100M ledger fee; stop if refreshed fee, liquidity or creation cost cannot support it |
| New relationship | Existing payer to the fresh owner; raw daily allowance 5T, nominal expiry two hours from fresh selection, one submission; no alteration of the old link |
| Installation capacity | One tenant, two lifetime objects, 20 retained leaves, 10 MiB per object, 20 MiB physical/liability/logical bytes; one active upload |
| Cleanup history | Two references / three receipts per object; retain provider/billing obligations after logical release |
| Read capacity | One session, 10 MiB byte budget, 1 KiB reply bound; native streaming/verifier limits independently 10 MiB |
| Service attachments | One-cycle allocation entirely reserved; no attachment offer or automatic provider top-up |

The previous 2T creation yielded about 1.5T in the owner before later runtime
costs. Creation amount is not retained canister balance; this comparison does
not establish a current creation price. Last donor liquidity is only 1.654605097235T.
The existing 100T total trial authority persists, with 7.1001T gross donor debit
recorded; neither number proves current liquid funds. No additional payer deposit
or alternative funding identity is selected. Refresh original payer/donor/owner/
gateway balances, actual fee, link expiry and terms before signing anything.

Run the small known 1 KiB file first. Only after independent verification,
verifier attestation, successful tenant download and reconciled financial readback
may the second 10 MiB/ten-chunk file proceed. The offline SDK/native preparation
checks both bodies, eleven aggregate leaves and 10,486,784 total bytes; it does
not qualify complete large-file transfer. New upload IDs/object IDs/first-reference
IDs `1` and `2` are proposed for the fresh owner only, never allocated by local
preparation. Select a new persistent browser profile/database with two slots;
preserve the old profile and its certificate/403 claim unchanged.

Bound the two transfers together to two certificate claims, thirteen serial PUT
dispatches, 2 MiB per PUT and 16 MiB aggregate request bodies. Bound independent
verification and tenant downloads to four GET dispatches and 20,973,568 content
bytes total, with existing request/reply deadlines. Capture live account/setup
queries and updates under a new intent with separate finite transport/time budgets
before effects. Stop at any refusal, mismatch, uncertainty or exhausted budget;
no automatic retry or fresh operation replaces an uncertain one. Release each
exact first reference after its successful download, preserving exposure, physical
bytes and liability; this is not provider deletion or billing cessation.

The maintainer explicitly approves this fresh-owner trial and covered actions
below 100T total or 20T/day, without repeat confirmation. Keep this selected run
within both limits; remind the maintainer when available balances are below 100T.
The approved actual owner is `5gsmp-viaaa-aaaak-qzhfa-cai`, with the same roles,
namespace and project. Its new link is 5T/day, nominal expiry
`2026-10-02T18:40:17.685Z` (raw `1790966417685000000`); expiry enforcement
remains unqualified. The run is retained in `.tmp/trial-v070-live-01`.
Actual principal, fresh terms and verified readback replace stand-in inputs before
setup. The old 0.6.0 owner, old link, balances, failed transfer and continuing
obligations survive. No additional permission is needed for covered actions.

The [completed live record](evidence/caffeine-probes/deployed/2026-10-02-trial-v070-live-01/summary.json)
qualifies both selected sample journeys, including exact tenant body comparison
and logical release. The large sample repeats one chunk hash ten times; distinct
consumer assets and complete publication batches remain open. Final accounting
retains 10,486,784 physical/liability bytes and zero logical/reserved bytes.
Both lifetime object slots and browser slots are consumed; do not reset/reinstall
the owner or browser history to make another upload. Preserve surviving provider
objects, credits and billing obligations until owned disposition is proved.

The preserved preparation Wasm initially reports 0.6.1. Before any tenant/link/
certificate exposure, the empty new owner is independently verified and corrected
once with rebuilt, PocketIC-qualified 0.7.0 bytes. Original mismatch and outcomes
remain. Deployments must independently check the selected compiled release and
module hash; a tag or release receipt cannot prove preserved target bytes.

## Recorded 0.6.0 prototype envelope

The original isolated trial selected these assumptions together. They describe
that frozen installation and its retained history, not current library size limits:

- One freshly created standalone owner, one isolated payer, one tenant, one upload
  and one first reference. Operator, uploader, verifier and controllers are explicit
  trusted trial participants; no public uploader or shared production account.
- One known nonempty file of at most **1,024 bytes**, with one content chunk. The
  service's lifetime object capacity is one; physical, liability and tenant logical
  byte budgets are 1,024. One active reservation/read session and retained release
  receipt capacity remain available. Metadata and framing have separate bounds.
- One certificate claim, one serial transfer with no retries, at most two gateway
  fetch-dispatched PUTs, 64 KiB per request body and 128 KiB aggregate claimed request bodies. The existing
  hook bounds each reply to 64 KiB and each request to 20 seconds. These are client
  limits, not total wire-byte limits or provider charge guarantees. The transport
  requires HTTPS, request-stream support and HTTP/2 or HTTP/3; it emits each
  snapshot through one immediately closed stream and never falls back to buffering.
  This mitigates the reproduced Chromium pre-header replay locally, without
  guaranteeing all wire attempts or remote replay economics.
- Two independently budgeted complete GETs: the verifier's observation and the
  tenant download, each using `--max-bytes 1024`. Their existing 30-second provider
  deadlines and service/IC response limits remain distinct from the PUT budget.
  Account/setup observations, ingress polling, headers and continuing storage need
  their own reviewed totals; a two-PUT limit does not bound the whole trial.
- Stop on refusal, timeout, lost response or exhausted local budget. Preserve the
  exact original requests; inspection cannot authorize automatic paid-effect retry.
  Certificate bytes and signing keys stay in controlled storage.
- Use the current owner without upgrades, reinstall or snapshot operations during
  effects. Stop/start is supported. An upgrade remains inspection-only; an old
  snapshot cannot reactivate the service. A stopped service does not recall a
  certificate, stop provider serving or terminate billing.
- The verifier establishes observed availability after whole-content verification.
  SDK progress is not completion. Release the exact first reference after the
  download and preserve provider/uncertainty/billing obligations until separately
  evidenced disposition. No deletion deadline or automatic account reset.

The trusted-uploader and restricted-lifecycle assumptions could make a first
prototype useful without solving general production recovery or adversarial
certificate reuse. They do **not** establish provider economics. A daily limit,
expiry, zero overdraft or small balance remains an intended control until its
applicable semantics are evidenced. A monitored stop budget is not a guaranteed
maximum bill. The maintainer accepted this residual risk for the restricted
contract; actual target selection, funding and live effects still require their
own exact authority.

The 0.6.0 release replaces the maintained issuance contract coherently,
with corresponding rejection/interruption evidence. Pre-1.0 remains a hard cut: no parallel trial mode,
old/new gate generations, compatibility path or manually supplied true flags.

## Inputs to finalize

The [configuration envelope](../canisters/standalone/trial/README.md) and
[provisioning sequence](operator-guide.md#prepare-isolated-trial-provisioning)
have an original private proposal in `.tmp/trial-provisioning-01/proposal.json`:
canic-mainnet is the deployment/operator candidate, with fresh repository-local
payer/tenant/uploader and separate verifier candidates, a fresh proposed project/
bucket and dedicated browser history. That preparation selected no live roles,
account, service or provider namespace; actual service and raw expiry were
unset. Shared validation and independent decoding passed with a labelled local
service stand-in; those installation bytes are never live inputs.

The separately authorized creation now supplies actual service
`4wyfo-qaaaa-aaaam-qjlpq-cai`, controlled by canic-mainnet-recovered at the same
planned principal. [Creation evidence](evidence/caffeine-probes/deployed/2026-10-02-trial-create-02/summary.json)
retains the exact request/block, 2.0001T debit and empty module/controller status.
The [new offline init proposal](evidence/caffeine-probes/local/2026-10-02-trial-installation-01/summary.json)
binds frozen 0.6.0 artifacts to this actual service. The separately authorized
[installation](evidence/caffeine-probes/deployed/2026-10-02-trial-install-02/summary.json)
now succeeds once with exact configuration/roles, release 0.6.0 and fresh zero
local activity. Its submitted gzip module hash and byte-identical raw Wasm are
retained separately. Subsequent funding and shared-handler observations below
establish only their recorded paths; project acceptance and object transfer remain
unqualified.

| Input | Current state |
| --- | --- |
| Host | Frozen standalone 0.6.0 installed on the detached mainnet owner; all local owners unfenced |
| Service, controller/deployer, operator, tenant, uploader and verifier | Service `4wyfo-qaaaa-aaaam-qjlpq-cai`; recovered controller/operator and exact uploader/verifier match reviewed init. Tenant enrollment, original permission/manifest and one actual certificate succeed |
| Cashier, payer and account controller | Installed bindings match. Initial and later deposits credit 5.0996T total. The approved allowance update is followed by 1T payer-to-owner allocation: payer 4.0996T, owner 1T Ledger credit. No new donor debit; enforcement and uncertain-response recovery remain unqualified |
| Gateway origin, project and bucket | One Cashier gateway is synced at sequence 1. A streamed HTTPS tree PUT reaches `https://blob.caffeine.ai` and returns readable HTTP 403 for insufficient owner balance; project/bucket acceptance and full transfer remain unqualified |
| Total trial budget | Maintainer explicitly preapproves total spending up to 100T cycles; 7.1001T gross donor debit recorded, 92.8999T authority remains. Original donor liquidity is 1.654605097235T; approval is not available funds or a proven maximum external bill |
| Allocation | Conservative 10T service / 1T initial provider / 89T unallocated plan. One 1T gross deposit is complete; account credits 999.8B after 100M transfer and 100M sweep fees |
| Monitoring/retention window | Unselected; must cover continuing obligations rather than just transfer time |
| Relationship limit/expiry, overdraft and gateway credit handling | Approved daily allowance is now 5T; raw expiry remains 1790953635603000000 (nominal 2026-10-02T15:07:15.603Z), enforcement unqualified. Zero overdraft/no target refill unchanged. Relationship period spend is 1T, gateway credit raw 333333333333 and all reported gateway usage zero |
| Browser identity and persistent intent store | Real original payer signs in memory; original profile/origin/database retains the verified certificate and one responded 403 claim across restart. Open existing history explicitly; missing history stops |
| Cleanup/reconciliation owner and continuing storage disposition | Maintainer controls original operator/payer and retained histories. One exposure-possible 1 KiB reservation/liability remains; no release, reset, deletion or billing cessation is inferred from refusal |

Treat gross cycle allocations and outstanding charge exposure conservatively;
provider funding is not proof of either credit or final cost. Do not count a
refund or remaining provider balance as released budget without exact evidence.
The original planning-only decision did not authorize funding. The later explicit
100T total preapproval now covers scoped trial spending; do not ask again for it.
Remaining budget never permits repeating uncertain effects or presuming provider
enforcement/expiry units. Contract acceptance is separately recorded above.

The [selected-payer funding review](evidence/caffeine-probes/deployed/2026-10-02-trial-funding-review-01/summary.json)
retains zero payer/deposit-address ledger balances, unchanged provider-returned
subaccount and a mainnet-verified AccountNotFound reply. Exact direct-deposit and
payer-notification inputs independently encode against the retained current DID.
That preparation precedes the separately approved
[funding result](evidence/caffeine-probes/deployed/2026-10-02-trial-funding-01/summary.json):
one exact transfer and one notification succeed, with reconciled ledger blocks,
credited account and empty deposit address. Verified account/settings reads confirm
zero overdraft and no automatic target refill. A missing/refused credit reply still
stops effects and preserves original funds/requests as continuing obligations.

The [live standalone readiness capture](evidence/caffeine-probes/deployed/2026-10-02-trial-link-01/summary.json)
confirms bounded gateway discovery and the shared balance handler's exact 999.8B
observation. Anonymous/payer budget_check return NotAuthorized; explicit-gateway
budget_get returns OwnerNotFound before linkage. Automatic approval review blocks
the initial request, which expires unsubmitted. Subsequent explicit approval and
the [fresh link experiment](evidence/caffeine-probes/deployed/2026-10-02-trial-link-02/summary.json)
complete one link with exact payer/owner/raw limit/expiry readback. Relationship and
zero gateway-credit replies remain identical before/after candidate expiry; no
usage/spend is reported. Linkage/owner recognition works, but expiry enforcement
is inconclusive. Zero gateway credit is separate from the funded-payer balance;
visible metadata does not prove spending authority persists. No extension, new
funds, certificate or object transfer follows. Preserve the relationship, funded
resources and rejection; billing cessation remains unqualified. Next review
gateway admission/credit and namespace/browser readiness.

The [actual-service preparation](evidence/caffeine-probes/local/2026-10-02-gateway-admission-01/summary.json)
retains the known 1 KiB body, SDK manifest, exact installed carrier and native/browser
handoff. An unsigned one-slot journal survives browser-process restart in the
recorded profile/origin/database, with no keys or provider traffic. Source review
finds no separate application credit-grant requirement or effect-free dry-run;
zero gateway credit does not prove upload refusal. The retained live-trial plan
proposes a separately approved two-hour relationship expiry before real admission,
one certificate, two SDK upload dispatches and independent verified downloads.
Limits remain client bounds, not a total provider bill cap. Keep failed probes,
unsigned history and all funded/relationship obligations; check candidate freshness
and exact state before any effect.

The subsequently approved [live trial](evidence/caffeine-probes/deployed/2026-10-02-trial-live-01/summary.json)
updates that link once, admits/prepares the original packet and verifies one actual
certificate. Caffeine refuses the tree with HTTP 403 for insufficient owner balance;
the chunk and both planned downloads are not attempted. Payer/relationship replies
remain unchanged while gateway credit/usage are zero. Source guidance recommends
10T funding and illustrates 5T daily allowance; neither a minimum nor the refusal
cause is established. Investigate credit allocation before another reviewed trial.
Do not retry this transfer or clear its retained 1 KiB exposure/history.

The subsequently requested [larger payer funding](evidence/caffeine-probes/deployed/2026-10-02-trial-funding-02/summary.json)
credits another 4.0998T after two fees, with original account/settings and exact
link terms unchanged. Separate owner/payer/gateway reads identify zero owner
Prepaid balance, 5.0996T payer Ledger credit and zero gateway credit. No upload
is retested, so these do not establish the earlier refusal's cause. The selected
donor cannot fund the example's 10T balance. The [subsequently approved comparison](evidence/caffeine-probes/deployed/2026-10-02-trial-limit-01/summary.json)
changes daily allowance 1T -> 5T, preserving expiry. It succeeds once; verified
reads reconcile 1T allocated from payer to owner, leaving payer 4.0996T and owner
1T Ledger credit. Gateway credit becomes raw 333333333333; reported usage stays
zero. Internal allocation is not additional donor spending or proved upload cost.
No admission retest occurs. Prepare a separately reviewed fresh-owner trial;
retain the prior exposed operation, browser claim, link and balances without
retrying its claimed transfer, resetting capacity or inferring billing cessation.

The earlier read-only `icp identity list` confirms `canic-mainnet` has public principal
`o5trf-oqyg7-cawjp-xs4pw-aomb3-iwki5-hyezf-qahfz-j3ffd-jh4fc-oqe`.
The default remains `toko-miner-local`. That metadata check did not test signing
capability or select roles. Later creation/installation uses the explicitly
selected recovered signer at the same principal with no key export/default change;
its operator role is explicit in the approved init. Controller status does not
assign tenant, uploader, verifier or provider account authority.

Finalize identities before producing live inputs. Obtain the actual created
service principal before installation, then rerun `installation-check`; bind
Wasm/DID/release hashes and independently decode the complete host init argument.
Account linking/funding and provider probes require their exact selected targets
and budgets. None is performed by this preparation.

## Offline rehearsal and ready tools

The 0.5.0 CLI and maintained patched Caffeine 1.1.2 SDK passed a 1 KiB preparation
rehearsal with networking refused. Current native `upload-inputs` additionally
requires the complete reviewed installation carrier, validates its original
service/namespace/project/trusted uploader and applies its resource bounds. It
preserves exact init bytes/hash without claiming actual installed-state observation.
It verifies the body and
saves a snapshot, original permission, manifest, browser binding and first-reference
download/status requests. Snapshot repreparation matches after source edits;
repeat and corrupt-source attempts refuse without replacing the original inputs.
The existing 10 MiB rehearsal remains the default.

```sh
make test-sdk-inputs \
  BLOB_BROWSER_NODE=/absolute/path/to/supported/node \
  BLOB_SDK_INPUTS_BYTES=1024 \
  BLOB_SDK_INPUTS_REPORT=.tmp/new-one-kib-rehearsal
```

The output uses **local fixture identities and an unqualified setup-only store**.
It is not a selected live upload. Use a fresh output directory every time.

A one-object candidate passes the shared installation validator. The maintained
offline `installation-check` now produces complete `ServiceInstallationInput`
bytes in `installation.candid`, with project, verifier and required trusted uploader
alongside the configuration. Actual local PocketIC installation accepts the exact
generated carrier and rejects a proposed service that differs from the host;
independent DID decoding also passes. `configuration.candid` alone is not init.
The earlier review's carriers and financial values remain historical syntax
evidence, without live trial authority.

The earlier standalone 0.5.0 release Wasm is retained with its DID/source hashes. Its
actual PocketIC certificate-boundary check passes: the four real blockers remain
visible, issuance refuses without state mutation, and stop/start/revocation/restore
keep the maintained authority and fences. This is local IC evidence, not Caffeine
gateway acceptance or a deployed trial installation. It describes the former gate.
The accepted contract's current 48-case standalone suite passes, including actual
local issuance/refusal, signed tooling and supported lifecycle. The unsupported
snapshot case demonstrates loss of later exposure/revocation and renewed local
eligibility; it establishes no safe operational rollback path. Current complete
host-init encoding includes the required trusted uploader.

After review and qualification, reuse `account-link-inputs`, `upload-inputs`,
`admit-upload`, `prepare-upload`, `certificate-assessment`, `createUploadTransfer`,
`observe-upload`, `submit-attestation`, `download` and exact reference release.
There is no need for another uploader, journal owner or downstream framework
dependency. Canic adoption is deferred at the maintainer's request while useful
repository-local implementation and evidence work remains.

The complete local journey is now exercised by `make test-browser-standalone`:
actual restricted installation and certificate issuance, patched SDK upload through
the one-slot journal, reload/read-state recovery, independent native GET/verification,
distinct-verifier attestation, tenant download and exact reference release. The
gateway serves the bytes received from the browser upload; it is a local substitute.
HTTP success alone leaves download unavailable. A separate corrupt-body journey
produces no statement or completion and retains exposure/liabilities after withdrawal
and browser cancellation. This is local integration evidence, not a live provider trial.
The [retained summary](evidence/caffeine-probes/local/2026-10-02-standalone-browser-01/summary.json)
records budgets, actual host facts, substituted provider behavior and failed attempts.

The maintained local rehearsal also loses the connection before reply headers after receiving
the exact chunk. The journal remains uncertain through certificate recovery;
supported owner stop/start preserves exposure, and independent verification can
confirm availability without another upload dispatch. A second case withdraws
permission after the verifier's observation, accepts late completion for accounting,
then releases the reference. Exact attestation replay preserves that release.
Withdrawal, reference release and provider settlement remain separate operations.
The [interruption ledger](evidence/caffeine-probes/README.md#standalone-transfer-interruption--2026-10-02)
retains the initial buffered pre-header failure and the distinct truncated-body
experiment. The [repair ledger](evidence/caffeine-probes/README.md#browser-replay-repair--2026-10-02)
records the current streaming mitigation: one arrival per guarded request under
owned HTTP/2 failures, including the original standalone cut. HTTP/1.x refuses;
buffered and XHR controls repeat. The current local rehearsal uses one TLS HTTP/2
origin for browser upload and both native reads, preserving normal native
certificate verification. Unrelated trust roots and refused streams produce
retained failed observations without implicit provider read retries.
Deployed gateway/TLS/CORS/stream compatibility, other browsers, HTTP/3 and provider
economics remain unqualified. No fallback, new owner or provider method is added.

Three [anonymous gateway metadata requests](evidence/caffeine-probes/deployed/2026-10-02-gateway-stream-01/summary.json)
observe verified HTTPS/HTTP/2 at source-listed blob.caffeine.ai and successful
PUT preflights allowing the SDK headers. The synthetic Origin is not a selected
application, and no certificate/body/account identifiers are sent. This resolves
part of transport readiness, not actual streaming-body acceptance or provisioning.
X-Dry-Run is advertised but its behavior is unknown; it grants no upload authority.

Choose a project value accepted by the SDK's `X-Caffeine-Project-ID` HTTP header
as well as the service's UTF-8 metadata/URL checks. The rehearsal explicitly uses
`standalone-trial`; the older Unicode download fixture is not a browser-upload
configuration. Owner/project/root must stay identical through upload and both GETs.
Native upload-input JSON and the certificate intent now require explicit project
and bucket before issuance. Transfer uses the retained values; changing either
after setup/reload refuses without a certificate or gateway request. Review those
original values against installation/provisioning; immutable local history alone
does not establish a valid provider namespace. Cross-release browser journals are
recreated only after obligations are preserved or discharged, without a dual reader.

The [retained review](evidence/caffeine-probes/local/2026-10-02-standalone-trial-review-01/summary.json)
separates fresh public source, retained interfaces and the offline rehearsal.
It records the unavailable primary guide lookups and the remaining limitations.
