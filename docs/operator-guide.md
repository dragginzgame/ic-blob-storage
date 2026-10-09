<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Operator and verifier guide

This guide is organized by operator task. Commands act on an explicitly selected
installation and identity; examples do not grant authority or authorize provider
traffic by themselves.

## Choose a task

| Goal | Start here |
| --- | --- |
| Validate and size an installation | [Check installation inputs offline](#check-installation-inputs-offline), then [size a consumer installation](#size-a-consumer-installation) |
| Prepare and publish several files | [Freeze a publication inventory](#freeze-a-publication-inventory-offline) |
| Admit or resume one upload | [Admit and prepare an upload](#admit-and-prepare-an-upload) |
| Download and verify stored content | [Download a verified file](#download-a-verified-file) |
| Inspect identities, limits and fences | [Identity, trust and service status](#identity-trust-and-service-status) |
| Inspect provider accounts or gateways | [Account inspection](#account-inspection) and [gateway controls](#gateway-controls) |
| Retain, share or release a file reference | [Generate reference inputs](#generate-reference-inputs-offline) and [share a confirmed blob](#share-a-confirmed-blob-within-a-tenant) |
| Verify provider content and submit completion | [Observe provider content](#observe-provider-content), then [submit an attestation](#submit-an-attestation) |
| Resume after an upgrade | [Current-instance recovery](#current-instance-recovery) |

<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-operator-workflow.svg" alt="Operator workflow from preparing and admitting an upload through verification, references, release and reconciliation" width="800">
</p>

## Current-instance recovery

Frozen live 0.7.0 installations restore inspection-only. The current contract,
released in 0.8.0, supports operator-only `blob_resume_current_instance()` with empty arguments.
Same-release upgrade restoration remains synchronous and fenced before deferred
work. Invoke this endpoint as the installed operator on the current canister;
controller status supplies no authority. It reads IC-owned history once with
twenty requested changes, a thirty-second bound and a 64 KiB application reply
bound. The window must cover the immutable actual installation version and contain
no later snapshot load, state replacement or unqualified change. Supplied counters,
backup files and manual overrides cannot establish continuity.

The twenty-change limit is a management-event horizon, not a time guarantee.
Upgrade/controller churn can push installation outside it; repeated successful
recovery reads do not move the anchor. Plan obligation-preserving retirement and
retain the complete surviving inventory before this horizon is exhausted. Current
code offers no active older-backup recovery or reset shortcut.

A successful call clears every owner's fence together, preserving exact IDs,
permissions, reservations, journals, receipts, balances and liabilities. It does
not repeat provider requests or settle uncertain effects. Inspect the retained
original operations afterward and use their existing exact recovery workflows.
Stop/start preserves state and uses the same continuity checks. Standalone queries
conservatively report a fence on platform-version gaps. Before delegation, a
previously active owner may obtain one independent continuity preflight; already
fenced upgrade restoration never activates this way. Intervening execution or
management changes invalidate a history reply and require a fresh explicit read.
Old heap restoration fences before
owner access; `SnapshotRestored`, `IncompleteHistory` or another typed refusal
leave activation unavailable. Do not rotate the anchor or reset the owner to
recover an exhausted history window.

This record/candidate/API/lifecycle hard cut ships in 0.8.0;
cross-release transitions remain reinstall-only after obligation disposition.
Older snapshot/backup activation remains unsupported and needs an independently
surviving complete inventory. See the [gap review](service-gaps.md) for separate
deletion, billing-cessation and consumer adoption requirements.

Use the native `blob-storage` client to inspect a service, submit tenant reference
operations, check content and submit an explicitly trusted verifier's statement. Run examples from the
repository root. Replace environment variables and input files with the exact
installation scope and original saved requests.

| Task | Command | Effect |
| --- | --- | --- |
| Check a complete proposed installation | `installation-check` | Offline shared validation, exact configuration and complete init bytes with hashes |
| Prepare an explicit Cashier account link | `account-link-inputs` | Offline Candid and summary; no signature, submission or funding |
| Save verified upload bytes and service requests | `upload-inputs` | Offline installation consistency/root verification and fresh private files |
| Freeze a bounded upload batch | `publish-inputs` | Offline input-hash, identity and aggregate candidate-capacity checks; serial verified snapshots |
| Check a frozen batch against its live owner | `publish-check` | Signed capacity/content queries, retained replies and conservative blockers; no reservation or provider traffic |
| Prepare one frozen batch file or recover its setup | `publish-prepare`, `publish-prepare-resume` | At most two local updates, four queries; separate tenant/uploader identities and original signed journals, no certificate or provider transfer |
| Reserve, prepare or withdraw an exact upload | `admit-upload`, `prepare-upload`, `revoke-upload` | One local service update and saved signed intent; no provider call |
| Recover original permission or manifest | `upload-permission`, `upload-manifest` | Signed exact query; never redispatches |
| Inspect local counters and restore fences | `status` | Signed service query |
| Observe provider-reported balances or relationships | `inspect-account` | One signed service read update; service queries Cashier |
| Sync gateways, cancel a pending sync or revoke membership | `sync-gateways`, `cancel-gateway-sync`, `revoke-gateway` | One signed service update and retained local intent |
| Diagnose a proposed funding intent | `funding-assessment` | Signed passive query; no reservation or payment |
| Inspect retained funding | `funding-history`, `funding-outcome` | Signed service query |
| Find upload identities | `upload-history` | Signed service query |
| Recover reference results or inspect liveness | `reference-receipt`, `reference-status` | Signed service query |
| Retain or release a saved tenant reference | `submit-reference` | One signed service update and local intent writes |
| Inspect issuance blockers | `certificate-assessment` | Signed service query |
| Check a local body against its declaration | `verify-upload` | Signed service query and local file read |
| Recover a trusted completion receipt | `upload-attestation` | Signed service query |
| Save a verified tenant file | `download` | One replicated descriptor update, then one provider GET and private file writes |
| Check provider bytes and retain a statement | `observe-upload` | Signed service query, provider GET and local evidence writes |
| Submit the saved statement once | `submit-attestation` | Signed service update and local intent writes |

The standalone host exposes the maintained blob method types; consumer frameworks
own their wrappers elsewhere. See [current status](status/current.md) before
selecting a target. Signed local standalone/browser journeys cover setup,
independent verification, attestation, tenant reads and interruption recovery
against a gateway substitute. They do not qualify deployed Caffeine behavior.
Service observations preserve local facts and fences; they do not grant retry or
payment authority. Provider reads require a selected installation, approved origin
and budget under the [probe ledger](evidence/caffeine-probes/README.md).

## Check installation inputs offline

`installation-check` checks the complete shared installation candidate,
including service/operator/payer bindings, portable quotas, reference cleanup
capacity, funding reserves, read limits, project and trusted completion verifier. It emits the complete shared `ServiceInstallationInput`
without opening memory or dispatching installation.

Prepare one `ServiceConfigurationInput` using the maintained Candid type and your
explicitly reviewed values. Candid numeric literals preserve u128 widths; quote
the `service` field name in textual Candid. The
[local fixture](../crates/ic-blob-storage-cli/tests/fixtures/installation/configuration.args)
illustrates syntax only: its principals, namespace and financial values are not
trial defaults. From the repository root:

```sh
didc encode --defs canisters/standalone/service.did \
  --types '(ServiceConfigurationInput)' < trial-configuration.args > trial-configuration.hex
perl -ne 'chomp; print pack("H*", $_)' trial-configuration.hex > trial-configuration.candid
cargo build --offline --locked -p ic-blob-storage-cli --bin blob-storage
target/debug/blob-storage installation-check \
  --configuration trial-configuration.candid --service "$SERVICE" \
  --project "$PROJECT" --verifier "$VERIFIER" --release "$HOST_RELEASE" \
  --run-dir .tmp/trial-installation-check
```

The command accepts at most 16 KiB of Candid with one exact typed value, bounded
decode work, no skipped fields and no trailing values/bytes. Unknown or repeated
CLI options refuse. Complete semantic validation precedes creation of a new
private directory containing the original `configuration.candid`, complete
`installation.candid` and `summary.json` with both byte hashes; existing or partial
output is never replaced. The report keeps full-width namespace as a decimal
string. `host_init_encoded` is true; authentication, actual platform identity,
compiled release and effect facts remain false.
It is a local proposal, not a deployment permit or recovery journal.

The supplied service and release are planned inputs. Deployment must independently
check the actual canister and compiled artifact. Standalone takes the core's
`ServiceInstallationInput { configuration, project, completion_verifier }`;
`installation.candid` is its exact init argument. `configuration.candid` alone is
insufficient. Independently decode the generated carrier before installation:

```sh
perl -0777 -ne 'print unpack("H*", $_)' .tmp/trial-installation-check/installation.candid |
  didc decode --defs canisters/standalone/service.did --types '(ServiceInstallationInput)'
```

Compare the decoded values and both file hashes with the finalized plan. No
identity credential is embedded. External consumer wrappers can import the same
passive DTO without depending on the standalone crate; wrappers still own their
initialization hooks and preserve the complete validator, actual service binding,
compiled release and transport/semantic limits.

## Isolated upload/download trial plan

The [accepted standalone trial contract](standalone-trial.md) bounds one 1 KiB
upload and records the maintainer's 100T-cycle total budget, restricted lifecycle,
unselected account/identity bindings and remaining provider observations.

Use one new isolated storage owner and its existing local journals. No existing
installation/account is selected; keep existing installations and obligations
separate. Use the standalone host here, or a separately qualified consumer-owned
wrapper. This repository supplies no framework adapter or integration suite.


| Stage | Concrete preparation or observation | Required before moving on |
| --- | --- | --- |
| Bind the trial | Select host, controller/deployer, operator, tenant, uploader, verifier, explicit payer, Cashier, project/bucket, gateway origin and cleanup owner | Exact identities, namespace assignment and separately authorized creation/deployment; no implicit account or role defaults |
| Freeze local inputs | Run `installation-check`, prepare `account-link-inputs`, bind exact Wasm/Candid/release hashes and choose a nonempty file within installed limits (1 KiB for the retained first live owner) | Original configuration/terms/body retained; size larger/multi-file installations explicitly and recheck actual service after authorized creation; no fixture defaults |
| Review economics and lifecycle | Record raw price/expiry units, selected spending controls, retained uncertainty and the accepted fresh-owner lifecycle | No guaranteed provider spending cap or replay charges; use trusted participants and preserve all continuing obligations |
| Provision and inspect | After exact action authority, create the isolated owner, recheck configuration against its actual principal, then install and separately submit account/funding actions; inspect configuration, relationship/balance and gateway scope | Actual identities match the finalized plan, namespace is provisioned, obligations and every failed/uncertain action are retained; no automatic retry |
| Admit and prepare | Use maintained SDK preparation, `upload-inputs`, tenant `admit-upload` and uploader `prepare-upload` | Original snapshot/root/permission match; trusted roles and current certificate assessment checked |
| Transfer once | After explicit trial authority and qualified host facts, use `createUploadTransfer` with serial/no-retry settings and original binding | One certificate claim; reviewed request/body/time limits and numeric financial exposure; stop on refusal, uncertainty or budget exhaustion |
| Verify and download | `observe-upload`, `submit-attestation`, then tenant `download`, using separate fresh evidence directories | Whole-body root/length verification and exact trusted receipt; SDK success alone cannot establish completion |
| Close the trial | Release the exact reference, retain provider object/payment/uncertainty records and assign ongoing reconciliation | Logical release is not physical deletion or billing cessation; retire only under the existing contract |

The first paid transfer remains **stopped** until exact live targets and effect
authority are selected. The accepted restricted contract allows local issuance
without claiming provider pre-charge/replay or old-backup guarantees. Experiments need their own recorded
intent, selected targets and budget; deployment/account/funding actions need exact
authority. Public metadata/pricing and client limits do not establish an enforced
financial ceiling.

Before any network trial, record numeric request, aggregate body/response byte,
wall-time and financial limits in the [probe ledger](evidence/caffeine-probes/README.md).
Include setup/inspection, certificate, gateway writes and both verifier/tenant
reads; the small file size is not the total wire or cost budget. The browser hook's
limits govern cooperating client traffic, not an escaped certificate. A lost
result stops effects; inspect original service intent/provider evidence without
resending. Stop/start retains the owner; upgrades remain inspection-only and
snapshot activation is unsupported. Local files are not independent freshness
authority. Keep [trial facts](evidence/caffeine-upload-gates.json) and the
[service retirement contract](service-contract.md) with final disposition evidence.

## Prepare isolated trial provisioning

Use the [maintained configuration envelope](../canisters/standalone/trial/README.md)
and private `.tmp/trial-provisioning-01/proposal.json`. The preparation proposes
`canic-mainnet` solely as deployer/controller/operator, with a fresh repository-local
principal explicitly sharing payer/tenant/original-uploader roles and a distinct
fresh verifier. No existing signing key or account is read; the default identity
remains unchanged. Fresh key files are private candidates, not selected authority
or funded accounts. Before any use, arrange their retention and controlled browser
handoff; never serve identity files from the trial page or put them in its bundle.

The provider candidates remain Cashier `72ch2-fiaaa-aaaar-qbsvq-cai` and
`https://blob.caffeine.ai`. A fresh UUID project/bucket and namespace 1 are proposed
in the private packet. Local representation validation is not provider assignment
or acceptance. A dedicated persistent browser profile, HTTPS localhost page,
unique IndexedDB name and one lifetime journal slot are also proposed. Select
these exact values before issuance and stop on missing or changed history.

| Order | Reviewed operation | Evidence required before the following effect |
| --- | --- | --- |
| Create the owner | One detached standalone canister, explicit controller/identity/network and at most the proposed 10T service allocation | Original create intent and ledger request/result; actual principal, fees and cycle balance. A lost result stops creation; inspect the original transaction, never create another to get a cleaner result |
| Finalize installation | Replace the service placeholder, run installation-check, retain exact source/Wasm/DID/init hashes and inspect installed configuration | Actual host identity/release/roles/project/envelope match; local validation alone is insufficient |
| Inspect the payer route | Query Cashier's `cycles_ledger_deposit_subaccount_v1` with `sender = isolated payer`; review account info/settings and ledger fees with that same explicitly selected payer | Exact reply/subaccount, caller binding, account existence/creation behavior and credit route established; never derive a Cashier deposit subaccount independently |
| Prepare ledger funding | Review one bounded donor transfer directly to Cashier's returned subaccount for the selected payer, followed by a payer-signed notification experiment | Exact source/destination/amount/fee/memo/created_at_time and retained transaction identity; direct credit remains unqualified. Count fees within the initial allocation. No standing approval or automatic refill |
| Notify and inspect | Separately invoke `cycles_ledger_deposit_notify_v1` with explicit isolated account after the recorded transfer | Raw credited amount, balance and returned ledger block retained; `NothingToDeposit`, `SweepFailed` or missing replies do not justify another transfer. This mutation's reconciliation semantics remain unqualified |
| Set and link account | After account existence/authority is evidenced, explicitly review zero overdraft/no target auto-refill and prepare `account-link-inputs` for the actual owner | Raw positive daily limit and absolute expiry with evidenced units; authenticated caller is the payer or an evidenced delegate. A daily limit is not a guaranteed total bill cap |
| Inspect service/provider scope | Maintained scoped relationship/balance inspection and gateway sync, followed by exact configuration readback | Actual paid-canister/payer/raw terms match; accepted project/bucket and gateway scope recorded before the one transfer |

The initial route was a proposal; the later
[exact deposit/notification](evidence/caffeine-probes/deployed/2026-10-02-trial-funding-01/summary.json)
creates and credits the selected fresh payer. This does not establish that
arbitrary callers can fund/control another account. The retained Cashier DID has
deposit-subaccount/notify methods and no explicit account-create method; that
absence does not prove lazy creation. Query inputs are prepared offline. No
mutation has been sent. A later separate
[fresh-candidate observation](evidence/caffeine-probes/deployed/2026-10-02-trial-payer-probe-01/summary.json)
returns a 32-byte deposit subaccount. Anonymous account-info returns NotAuthorized;
the same request signed by the fresh payer returns AccountNotFound, with mainnet
query-signature verification. Retain the provider-returned address, but recheck the
exact selected binding before any transfer. These observations establish neither
lazy creation nor notification/mutation authority or credited funds.

The historical [DFINITY integration example](https://github.com/dfinity/immutable-object-storage-example/blob/ef29e8a6e8063c6fe654cac53a3497cab585fefa/README.md)
describes funded account linkage and a wallet-based top-up. Current
[IC cycles documentation](https://docs.internetcomputer.org/concepts/cycles/)
explains ledger transfers and why a ledger cannot make arbitrary cycle-attached
calls. A direct ledger-deposit route could avoid a wallet/proxy, if Cashier's
account authorization and credit behavior are evidenced. No new funding endpoint,
wallet, proxy, Canic dependency or provider client schema is added here.

The maintainer now preapproves total trial spending up to 100T. Preserve the
conservative 10T service / 1T initial provider / 89T unallocated plan; the 1T gross
initial funding is completed, with 999.8B credited after observed fees. Count
3.0001T total donor debit so far and 96.9999T remaining. Do not request the same
covered spending approval again, and never repeat an uncertain effect merely
because budget remains. Select a monitoring window and
continuing cleanup owner before upload. Reserve-only local attachment policy does
not cap provider spending, terminate billing or establish old-backup activation.
Raw expiry units and project acceptance remain open. Retain every intent/outcome
before advancing and review the exact next effect separately; never run an upstream
one-shot setup script. See the [preparation evidence](evidence/caffeine-probes/local/2026-10-02-trial-provisioning-01/summary.json).

The [selected-payer review](evidence/caffeine-probes/deployed/2026-10-02-trial-funding-review-01/summary.json)
now proposes 999.9B cycles directly from the recovered donor to the provider-returned
isolated deposit address, with an explicit 100M ledger fee: **1T gross donor debit**.
The payer signs the subsequent explicit-account notification, after exact transfer
block and address-balance reconciliation. Funding an intermediate payer ledger
account is unnecessary for this proposed experiment and adds a fee. The unchanged
address, empty balances and signed AccountNotFound observation do not establish
creation, sweep credit or refundability. A conditional 999.8B credit assumes one
100M sweep fee and no other deductions; observe the actual credited/balance/block
reply rather than asserting this prediction.

Use the current method's **record with optional account** for notification; its
whole argument is not optional. Retain exact binary candidates and independently
decode against the current DID. Freeze explicit fee, memo and created_at_time
before an authorized transfer, and capture the signed request before dispatch.
After an uncertain transfer or notification, inspect the original request, exact
ledger transaction and account/address state; never send another deposit or
notification to obtain a clearer result. Expired unsigned preparation can be
replaced in a new linked capture only before any signing/submission uncertainty.
Provider refusals retain funds and cleanup ownership. This preparation authorizes
no funding, mutation, account linkage or upload by itself. The later explicit
approval completes one transfer/notification with exact transaction and account
reconciliation. Separate payer-signed reads confirm zero overdraft and no target
balance; no settings mutation is needed. Preserve these resources and receipts
while preparing linkage/expiry and actual transfer qualification.

The [standalone readiness observations](evidence/caffeine-probes/deployed/2026-10-02-trial-link-01/summary.json)
subsequently confirm one Cashier gateway through shared sync and 999.8B payer balance
through shared account inspection on the actual owner. Neither performs a payment
link. Anonymous and payer-signed budget_check are NotAuthorized; explicit-gateway
budget_get reports OwnerNotFound before linking. These refusals cannot establish
expiry units. Automatic approval review rejects the prepared persistent link before
process launch, requiring explicit approval for that account mutation. Preserve its
expired unsubmitted envelope; prepare fresh inputs in a linked capture after approval.
The concrete proposal contains one 90-second nanosecond-expiry hypothesis with raw
daily limit 1,000,000,000,000, no extension/new funds/certificate/object effects and
bounded relationship/budget reads. Subsequent explicit approval and the
[fresh link experiment](evidence/caffeine-probes/deployed/2026-10-02-trial-link-02/summary.json)
complete that exact add-link once with matching term readback. Relationship and
zero gateway-credit replies remain byte-identical before/after candidate expiry;
expiry enforcement is inconclusive. Zero gateway credit is separate from the
999.8B funded-payer observation. No extension, extra funds, certificate or object
request follows. Retain the relationship and funded resources; billing cessation
is unqualified. Review actual gateway admission/credit and namespace/browser
readiness before another exact effect. Total donor debit stays 3.0001T.

The [actual-service packet and browser journal](evidence/caffeine-probes/local/2026-10-02-gateway-admission-01/summary.json)
now prepare the known 1 KiB body against the installed 0.6.0 carrier. Public source
places budget admission at the gateway; no separate application credit grant or
effect-free dry-run is demonstrated. Do not treat zero gateway credit as an empty
payer account or add funds on that observation alone.

| Next-trial item | Prepared state |
| --- | --- |
| SDK/native inputs | Exact body/root, installation, permission, manifest, browser binding and download/status files in private `.tmp/gateway-admission-review-01/upload-inputs` |
| Browser history | The original slot now retains a verified certificate and one responded HTTP 403 tree claim in `.tmp/trial-browser-profile-01` at `http://127.0.0.1:43023`, database `ic-blob-storage-mainnet-trial-v1`; exact history survives reopening |
| Provider terms | Explicitly approved expiry update succeeds once: raw daily limit unchanged at 1T, nominal expiry 2026-10-02T15:07:15.603Z; expiry enforcement unqualified |
| Current balances | Separate larger deposit raises the original payer to 5.0996T Ledger credit; public owner-account balance is zero with Prepaid debt target, gateway credit/usage zero |
| Transfer bounds | One 1 KiB object, two SDK upload dispatches, 64 KiB per request/128 KiB total, serial operation and no automatic paid retries |
| Completion and disposition | Tree admission refuses; no chunk/download/attestation/release. Preserve the original exposure-possible permission and 1 KiB local physical/liability accounting; refusal does not prove deletion or billing cessation |

The [live trial](evidence/caffeine-probes/deployed/2026-10-02-trial-live-01/summary.json)
acknowledges the exact tenant/permission/manifest and issues one mainnet-verified
certificate. Its one streamed SDK tree request receives HTTP 403 for insufficient
owner balance. At that refusal payer balance is 999.8B and gateway credit zero;
the cause is unresolved. Public example funding of 10T and daily limit of 5T are guidance,
not proved admission thresholds. Do not fund speculatively or resend this transfer.
Inspect credit allocation before a separate reviewed trial; retain this original
profile and service liabilities. Never recreate missing history, serve keys through
the local HTTP server or put them in the journal. Standing total trial spend
authority remains 100T. The subsequently requested
[larger funding comparison](evidence/caffeine-probes/deployed/2026-10-02-trial-funding-02/summary.json)
deposits 4.1T gross and credits 4.0998T, with exact sweep/fee/account reconciliation.
At that capture's close, payer balance is 5.0996T; owner balance and gateway credit
are zero, and the stopped upload is untouched. Total donor debit is **7.1001T**, remaining
authority **92.8999T**; the selected donor has only **1.654605097235T** liquid.
The subsequently [approved 1T -> 5T allowance update](evidence/caffeine-probes/deployed/2026-10-02-trial-limit-01/summary.json)
preserves expiry and succeeds once. Readback shows 1T payer-to-owner allocation:
**4.0996T payer**, **1T owner Ledger credit**, **1T relationship period spend**
and raw **333333333333 gateway credit**, with all usage zero. Other account settings
and relationship fields are unchanged. No new funds or upload request occurs;
internal allocation is not another donor debit. The server algorithm, earlier
refusal cause, expiry enforcement and upload admission remain unqualified.
Prepare a separately reviewed fresh-owner trial while retaining the old claim,
profile, link, balances and exposure. Never replay its transfer or reset its slot.

The maintainer has authorized and completed the one-owner 2T creation effect. Its
[preflight](evidence/caffeine-probes/deployed/2026-10-02-trial-create-preflight-01/summary.json)
observes sufficient source-account cycles and the ledger fee, but initially cannot
load the password-protected deployer in a non-interactive terminal. The subsequent
[creation](evidence/caffeine-probes/deployed/2026-10-02-trial-create-02/summary.json)
uses the explicitly selected recovered identity at the same principal, preserving
one exact request and block before preparing init for the actual empty service.
For password-protected signers, resolve access
using an explicitly supplied password-file path (`--identity-password-file`) or
maintainer local signing; public principal metadata cannot unlock or authenticate
the key. Retain the exact signed creation envelope before submission. A direct
cycles-ledger `create_canister` request can use explicit amount/controller/settings
and `created_at_time`, while `icp message send` carries that saved request. The
unsigned encoding sample is not a dispatch/retry permit. If a request has already
been submitted with a lost result, inspect its original identity instead of
generating a new timestamp. Count the ledger fee in the source-account debit and
observe the created canister's balance after its creation cost; 2T offered is not
proof of 2T remaining. This authority does not include minting, installing or
provider-account funding. The later separately authorized
[standalone installation](evidence/caffeine-probes/deployed/2026-10-02-trial-install-02/summary.json)
uses that actual owner and exact reviewed init, with compiled 0.6.0/configuration
readback and fresh zero local activity. Supported gzip submission has its own
module hash; decompression matches the reviewed raw Wasm. Record both hashes.
For saved management requests, verify the effective destination against the signed
argument: generic CLI signing may retain the management principal as routing
metadata. Correct only destination metadata before submission, retain both files
and the unchanged signed envelope/request ID. Never infer provider account/project
acceptance from installation or authorize funding/certificates/uploads from it.

## Size a consumer installation

Certificate issuance uses the validated `ServiceResourceInput`, with no extra
1 KiB or single-object cap. Choose limits before installing the immutable owner.
The first live trial and its template deliberately retain their small envelope.

| Configuration | Sizing input |
| --- | --- |
| `max_object_bytes` | Largest consumed asset; 10 MiB covers the recorded Miner maximum of 8,362,256 bytes |
| Object, tenant and manifest-leaf capacity | Lifetime unique objects/leaves across the selected release horizon, including cancelled work; capacity is not recovered by releasing a reference |
| Physical/liability and tenant logical bytes | Overlapping releases and uncertain stored obligations; logical release does not delete provider bytes or end billing |
| References and receipts per object | Reuse across releases, exact retain/release history and retained cleanup capacity |
| Active upload and read limits | Explicit publisher concurrency and read-session/byte/reply budgets; these are independent of the object maximum |

Client upload and whole-download budgets must also cover each selected object.
Keep one configured trusted publisher and exact tenant permissions; controller
status grants no upload authority. This sizing change requires a minor release
and an appropriately configured consumer installation, not an in-place reset of
the old trial owner. It does not qualify provider economics or public delivery.

### Larger inventories and a dedicated storage owner

For a million distinct blobs, plan a dedicated storage canister owning tenant
policy, permissions, references, durable journals and physical/billing accounting.
Caffeine holds the blob bytes; the publisher runs off-canister. The core can also
be embedded in an application canister, but shared heap, instruction and lifecycle
budgets then need joint qualification. No additional journal/controller canister
is needed for this design.

Keep three independent bounds: service lifetime objects/leaves, browser lifetime
attempts, and files/bytes in each publication batch. Current browser journals
can be configured up to 1,000,000 attempts; native batches remain at most 4,096
files with explicit byte budgets. Process larger datasets as separately frozen,
bounded batches with globally distinct original IDs and all journals retained.
Cancelled attempts and overlapping release history need headroom beyond the live
blob count. Never erase an old batch/journal to reclaim history or repeat work.

This is a sizing direction, not million-object qualification. Production indexes
are stable-memory B-trees, but reopening synchronously scans root/permission,
manifest, reference and receipt history and builds temporary validation sets.
Measure stable bytes, peak heap, admission/lookup instructions and full reopen
instructions at increasing populated sizes before advertising that capacity.
Also measure browser transaction-count latency, disk/quota and native setup I/O:
each indexed preparation currently reverifies the full selected frozen batch.
`publish-prepare-batch` performs one verification pass for a serial setup run;
active-reservation capacity can stop it before every file is prepared. It does
not interleave provider transfer/completion to free those reservations.
At large scale, bounded reopening or partitioning may be needed; do not weaken
fences or obligation validation to fit an instruction budget.

## Generate account-link inputs offline

The isolated standalone owner is installed and its payer account is funded. This command
prepares a proposed `payment_account_canister_add_v1` request using the maintained
Cashier wire schema. It does not create an account, deploy a canister, authenticate
the planned caller or contact any network. Supply the exact proposed bindings:

```sh
cargo build --offline --locked -p ic-blob-storage-cli --bin blob-storage
target/debug/blob-storage account-link-inputs \
  --cashier "$CASHIER" --caller "$CALLER" --owner "$SERVICE" --payer "$PAYER" \
  --daily-limit "$PROVIDER_DAILY_LIMIT" --expiry "$PROVIDER_EXPIRY" \
  --run-dir .tmp/account-link-inputs
```

All principals must be canonical and neither anonymous nor management. The payer
and expiry are explicit: neither defaults to the caller nor to an indefinite
link. Daily limit and expiry are canonical positive decimal strings, bounded to
u128 and u64 respectively. They are **raw provider inputs**; this tool does not
convert currencies, infer timestamp units, validate future expiry or establish
enforcement. Do not choose trial values from the maximal numbers in test fixtures.

The fresh private directory contains `account-link.candid` and `summary.json`,
retaining the target, planned signer, paid canister, payer, raw terms, method and
request hash. The caller is planned local context, not an encoded authority claim.
Missing, duplicate or invalid arguments refuse before output creation; an existing
directory, including partial output, is never overwritten. Files are proposed
inputs, not an effect journal, spending guarantee, receipt or retry permission.

Account submission, funding and deployment require separately reviewed exact
targets and authority. Current public Cashier metadata/pricing match retained
replies, but an older onboarding guide uses request-unit labels different from
the live price list. Neither observation proves the selected account's limit,
expiry, charging behavior or namespace provisioning. See the
[recorded observations](evidence/caffeine-probes/deployed/2026-10-01-cashier-preflight-01/summary.json)
and [trial gates](evidence/caffeine-upload-gates.json).

## Generate upload inputs offline

`upload-inputs` converts the upstream Caffeine preparation's `manifestJSON` and
your application's original binding into the Candid files used by the signed
commands below. The required `--body` file is root-verified and saved as the same
exact buffers, so subsequent source edits cannot change the prepared snapshot.
Caffeine still owns upload preparation and transfer; this command reuses the
core's maintained preparation decoder, service validators and streaming verifier.

Save `manifestJSON` as `manifest.json`. Supply a `binding.json` with these fields:

```json
{
  "format": "ic-blob-storage/upload-inputs:original-preparation",
  "preparation": { "content_type": "image/png", "filename": "original.png" },
  "service": "SERVICE_PRINCIPAL",
  "project": "INSTALLED_PROVIDER_PROJECT",
  "bucket": "SELECTED_PROVIDER_BUCKET",
  "namespace": "1",
  "tenant": "TENANT_PRINCIPAL",
  "uploader": "UPLOADER_PRINCIPAL",
  "upload": "APPLICATION_ALLOCATED_UPLOAD_ID",
  "object": "APPLICATION_ALLOCATED_OBJECT_ID",
  "incarnation": "APPLICATION_ALLOCATED_INCARNATION_ID",
  "first_reference": "APPLICATION_ALLOCATED_REFERENCE_ID",
  "root": "sha256:LOWERCASE_64_HEX_CHARACTERS",
  "bytes": "ORIGINAL_FILE_BYTE_LENGTH",
  "expires_at_ns": "ORIGINAL_EXCLUSIVE_EXPIRY_NANOSECONDS"
}
```

Replace placeholders with the application's exact original values. Numeric fields
are canonical positive decimal **strings**, preserving u128 IDs and u64 lengths/
expiry. Principals and the provider root use their canonical text representations.
Unknown fields, invalid principals, noncanonical numbers and inconsistent
metadata/leaves/root refuse before claiming output. The root and length must come
from the same prepared file; a raw SHA-256 content digest is not a Caffeine root.

The required `format` identifies this frozen layout. `upload-inputs`, batch
preparation and reopened publication sessions use the same strict reader. There
is no version dispatch or alternate reader. Preserve existing snapshots and
operation journals with their original binary; do not relabel or convert them
to make the new reader accept them. New identities must not replay uncertain work.

The current native contract requires `preparation`: record exactly the SDK
`prepareFile` arguments as optional `content_type`, `filename` and `cache_control`
strings, each at most 4,096 UTF-8 bytes. Use `{}` when all were omitted. Empty
strings remain empty; null and unknown hint fields refuse. The complete binding
has a 12 KiB byte ceiling. Do not infer hints from manifest headers: SDK defaults,
filename encoding and explicit cache metadata are preparation decisions. `cache_control`
supplies the original hashed `Cache-Control` value; omission adds no header.
The browser names this hint `cacheControl` and passes it as the SDK's fourth
`prepareFile` argument. Changed or omitted required metadata refuses root
agreement before certificate intent. Frozen binding hashes cover these hints and
the session's transfer descriptor carries them unchanged. The required hint object
entered in the 0.11.0 hard cut; the optional cache hint extends the current model
without replacing existing fields or their meaning. There is one reader. Retain old
operation journals and obligations; changing input format never authorizes replay.

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  upload-inputs --installation reviewed-installation/installation.candid \
  --binding binding.json --manifest manifest.json --body source.bin \
  --max-bytes 10485760 --run-dir new-upload-inputs
```

Use complete `installation.candid` from the reviewed `installation-check` output,
not its configuration-only file. The command decodes one bounded exact current
`ServiceInstallationInput`, applies the shared candidate validator and requires
the original service, local namespace and project to match. The exact upload
permission independently names the uploader; installation grants no user identity.
The candidate's object/chunk/header bounds constrain preparation alongside the
caller ceiling. This checks a proposed installation, without observing actual
installed state, remaining capacity, tenant authority, release or provisioning.
The host and authenticated service workflows still make those decisions.

The fresh private directory retains exact `installation.candid`, both JSON inputs, a verified `body.bin`,
`permission.candid`, `manifest.candid`, first-reference `download.candid` and
`reference-status.candid`, browser `certificate-binding.json`, and a final `summary.json` with input/output
hashes and the raw content digest. It opens a regular source file once, checks its
declared length, then hashes and copies the same 64 KiB frames. Complete EOF/root
verification and file sync precede publication without replacement. Source-path
replacement cannot select a different open file; in-place changes are checked
against the original root and length. Use this snapshot for subsequent file checks
and transfer preparation, rather than reopening a potentially changed source.
If rebuilding a Caffeine prepared handle from this snapshot, supply the original
metadata and require its computed root/length to match the saved permission before
certificate issuance or gateway requests.

`certificate-binding.json` is the existing browser certificate client's input:
canonical service/tenant/uploader, upload ID as its `operation`, root, exact
permission Candid as a byte array, and `${service}:${tenant}:${operation}` as
`key`. Its hash is in the final summary. Load it as `binding` when creating the
[browser client](../clients/browser/README.md); the application supplies its
authenticated identity, trusted IC root/origin and durable intent store separately.
Required project and bucket are also retained in this binding. Project must match
the candidate carrier; review that carrier against the actual installation and
bucket against the selected provider account. The offline command cannot prove
that provisioning. Both require 1–256 UTF-8 bytes,
no controls or surrounding whitespace; project must fit the SDK HTTP-header
ByteString contract too. Invalid namespace values refuse before creating output.
The browser transfer reads these immutable values from the certificate intent,
so a separate setup argument cannot redirect the same saved upload after a reload.
The original `permission.candid` remains the input for signed service admission.
This JSON is neither a certificate nor a ready-to-upload grant, and it contains
no gateway origin, provider account or qualification override. Keep it with the
verified snapshot under the application's intent policy.

Existing directories refuse, including partial runs. Failed verification retains
private `body.part` and `failure.json`, with no published body, Candid request files
or summary. A later write failure can leave verified output without a summary;
retain the incomplete run and do not treat it as complete preparation. Input bindings
are capped at 4 KiB, upstream JSON at 256 KiB, ordered leaves at 1,024 and metadata
at 16 headers/4 KiB. The selected content maximum is at most 1 GiB; the service may
have smaller installed limits. Generated permission/manifest Candid is capped at
4/64 KiB for the native upload commands.

No signer, URL or root key is needed. This command neither allocates nor proves
fresh IDs, authenticates tenants, checks the current service clock/expiry or admits
an upload. Keep the snapshot and binding under the application's intent/allocation
policy. Saved files remain mutable local data: use signed `verify-upload` against
the service's original declaration before future effects; snapshot creation does
not authenticate subsequent file edits. No certificate or provider request occurs.

## Freeze a publication inventory offline

`publish-inputs` prepares multiple caller-bound files through the same pipeline
as `upload-inputs`. First prepare each file with Caffeine's SDK and save its
original binding, manifest and body under one input directory. Supply a bounded
inventory with their lowercase raw SHA-256 hashes:

```json
{
  "schema": 1,
  "files": [
    {
      "binding": "asset/binding.json",
      "binding_sha256": "LOWERCASE_64_HEX",
      "manifest": "asset/manifest.json",
      "manifest_sha256": "LOWERCASE_64_HEX",
      "body": "asset/source.bin",
      "body_sha256": "LOWERCASE_64_HEX"
    }
  ]
}
```

Paths are relative regular files beneath `--root`; absolute paths, parent-directory
components and symlinks refuse. The inventory has at most 4,096 files
and 2 MiB of JSON; combined binding/manifest input is limited to 16 MiB.
The original IDs are supplied by the consumer's surviving allocation authority,
never generated by this command. Upload, object and first-reference IDs must each
be unique within the batch, as must each proposed provider root. All entries must
share one service, local namespace, tenant, per-batch uploader, project and bucket.
Different users prepare separate batches against the same installation.
Duplicate roots refuse before output even with distinct operation IDs: permanent
root claims require an exact retain/recovery path. This command does not
deduplicate or discover existing objects. Equal raw body digests with different
original metadata can produce distinct provider roots and remain separate proposed
objects and bytes. Reopening frozen batches applies the same guard.

```sh
blob-storage publish-inputs --inventory inventory.json --root frozen-inputs \
  --installation reviewed-installation/installation.candid \
  --max-bytes 10485760 --max-total-bytes 335544320 \
  --run-dir new-publication-inputs
```

Preflight checks bounded metadata hashes and per-file declarations, then sums
objects, retained leaves and physical/liability/tenant-logical bytes against the
validated candidate and explicit total ceiling. It plans one concurrent upload.
This checks whether the batch fits a **fresh installation**; it neither observes
remaining live capacity nor reserves anything. Cancelled history, other tenants
and overlapping releases can consume additional capacity on an installed service.
Per-file maximum is at most 1 GiB; total maximum is at most 1 TiB.

After preflight, a new private directory retains `inventory.json`, exact
`installation.candid` and `plan.json`. Serial `file-0000`, `file-0001`, … directories
contain the same verified body and request files as single-file preparation.
Each raw body digest must also match the inventory before usable request files
are written. Root/digest failure preserves that file's evidence and the batch's
`failure.json`, alongside earlier completed files. Only complete success writes
the final batch `summary.json`; existing and partial directories refuse overwrite.
Do not consume an incomplete batch automatically or interpret its completed files
as uploaded objects. Frozen local files still require verification before effects.

No signer, network, certificate, provider request or publication occurs. Live
serial admission/transfer/recovery and atomic publication of the consumer's media
map remain separate publisher work. Caller-supplied IDs are retained in frozen
inputs; authoritative reservations belong to the service's admission journal.

### Check a frozen batch against live capacity

`publish-check` consumes a **complete** `publish-inputs` directory. Before identity
loading or networking, it rechecks inventory/installation hashes, each saved body
against its Caffeine root and raw digest, and exact saved permission, manifest and
browser-binding packets. Missing summaries, changed files and symlinks refuse.

```sh
blob-storage publish-check --network ic --url https://icp-api.io \
  --identity tenant.pem --actor TENANT --service SERVICE --namespace 1 \
  --inputs publish-inputs --max-bytes 10485760 --max-total-bytes 1073741824 \
  --max-queries 817 --timeout-seconds 1800 --run-dir publish-check
```

Select finite limits for the actual batch: one capacity query plus one indexed
discovery per file, at most 4,097 queries and one hour overall. Each query also has
a 30-second deadline. The same authenticated tenant and trusted IC root apply to
every query. Intent, original inventory/installation, exact arguments and raw
replies survive in a private create-new directory. Failure leaves no complete
summary; existing/partial runs refuse overwrite. A later inspection needs a new
directory and an explicitly bounded observation window.

| Observation | Publisher implication |
| --- | --- |
| `not_visible` | Count proposed object, leaf and byte demand against observed headroom; admission is still unproved |
| `recover_existing_operation` | Retain the discovered original IDs and inspect the original permission/journal; no fresh upload or automatic retry |
| `live_requires_retain` | Reuse needs explicit reference-capacity checks and a tenant retain; this upload-only check remains blocked |
| `retired_root` | Preserve lifetime history and provider/billing obligations; do not reallocate that root |

Fences, suspension, duplicate planned roots, object/metadata ceilings, lifetime
history/leaves/bytes and lack of an active slot appear as separate blockers.
Capacity JSON renders `remaining_logical_bytes`, `remaining_physical_bytes` and
`remaining_liability_bytes` as exact decimal strings. `remaining_bytes` remains
their minimum. The three byte blockers are `logical_byte_capacity`,
`physical_byte_capacity` and `liability_byte_capacity`; every exceeded dimension
is reported. Logical release can restore tenant headroom while physical or
billing headroom stays exhausted. Provider deletion can restore physical
headroom while billing remains exhausted. Only separately evidenced settlement
restores billing headroom. Inconsistent capacity minima refuse as invalid replies;
clients do not repair, clamp or default missing fields.
Concurrency is a serial-upload ceiling, not the number of files in the batch.
`blocked: true` is a successful **observation**, so exit zero does not permit a
publisher to proceed. Queries are sequential, not a transactional snapshot; even
an empty blocker list reserves no capacity, allocates no IDs, proves no actual
installed project/uploader, and authorizes no certificate, publication or retry.
The service must validate and persist each exact admission before effects.

### Prepare one indexed file with surviving setup intent

`publish-prepare` reverifies the **entire** complete batch, then checks the selected
file's current root and independent capacity before local admission/preparation.
First inspect the complete batch with `publish-check`; a selected-file check is
not an aggregate reservation. Choose the original zero-based inventory index and
explicit tenant/uploader PEM identities. Both must authenticate before any output
claim or service mutation. IDs and expiry remain exactly those frozen in the batch.

```sh
blob-storage publish-prepare --network ic --url https://icp-api.io \
  --identity tenant.pem --actor TENANT --uploader-identity uploader.pem \
  --service SERVICE --namespace 1 --inputs publish-inputs --file-index 0 \
  --max-bytes 10485760 --max-total-bytes 1073741824 \
  --timeout-seconds 120 --run-dir publication-file-0000

blob-storage publish-prepare-resume --network ic --url https://icp-api.io \
  --identity tenant.pem --actor TENANT --uploader-identity uploader.pem \
  --service SERVICE --namespace 1 --inputs publish-inputs --file-index 0 \
  --max-bytes 10485760 --max-total-bytes 1073741824 \
  --timeout-seconds 120 --source-run publication-file-0000 \
  --run-dir publication-file-0000-recovery-01
```

Initial setup makes at most two updates; resume makes at most one **previously
unclaimed** preparation update. Each invocation performs at most four queries,
with 30-second per-request deadlines and an explicit overall deadline of at most
one hour. Admission requires the tenant; preparation uses the separately checked
uploader. The existing maintained setup commands own signed request, raw response
and outcome journals under the original run's `admission/` and `preparation/`.
Each claim is durable before its exact signed envelope is sent once.

Resume binds the original inventory, installation, index, scope, transport/root,
permission, manifest and signed admission envelope. It observes the original
permission/manifest once, without polling or resubmitting either claimed step.
It may complete preparation only when that directory has never been claimed.
An existing empty/partial claim, unprepared/unknown result or expired ingress
cannot authorize another attempt. New inspection output is create-new; the source
must remain the original preparation run, never an intervening recovery report.
Keep all journals; deleting them or starting a new initial run after uncertainty
is outside this recovery contract. The service still enforces admission and IDs.

| State | Meaning |
| --- | --- |
| `prepared` | Exact original declaration is saved; no certificate or provider bytes |
| `admission_unobserved` | Original update is pending; preserve its signed claim and resume inspection |
| `preparation_unobserved` | Preparation is pending or historically unobserved; never resubmit it |
| `blocked` / `permission_inactive` | Stop and preserve the original root, journals and obligations |

Inspect the state even on exit zero. Historical preparation does not renew expiry,
confirm current issuance authority or establish publication. Live/retired roots
and fences remain blocked. Drive one file through certificate/transfer/verification/
completion and retained references before admitting the next when concurrency is
one. The browser [frozen-file helper](../clients/browser/README.md#transfer-a-frozen-publication-file)
now composes certificate exposure and SDK transfer for the selected file. It
does not drive a complete batch or publish the confirmed media mapping.
No provider call, funding or account-link effect occurs during
setup; ordinary service updates still consume the service canister's cycles.

### Prepare a complete batch with one verification pass

`publish-prepare-batch` verifies every frozen body and request packet once before
claiming output or sending setup requests. It then calls the same per-file
workflow serially, preserving the original identities and signed journals in
`file-0000/`, `file-0001/`, and so on. This avoids rehashing the entire inventory
for every file in this invocation. Single-file preparation and recovery continue
to reverify the whole frozen batch independently.

```sh
blob-storage publish-prepare-batch --network ic --url https://icp-api.io \
  --identity tenant.pem --actor TENANT --uploader-identity uploader.pem \
  --service SERVICE --namespace 1 --inputs publish-inputs \
  --max-bytes 10485760 --max-total-bytes 1073741824 \
  --timeout-seconds 3600 --run-dir publication-setup-batch
```

The setup stage has one overall deadline, at most one hour, and at most two
updates/four queries per file. Each file still observes current headroom. Preparing
a manifest leaves its reservation active: use this command only for a batch that
fits active capacity, or accept a stopped batch. When concurrency is one, use
indexed setup with transfer/completion between files instead. No provider calls,
certificate or confirmed media map are produced by this command.

Stop at the first blocked, pending or failed file. Inspect `all_files_prepared`
even on exit zero; it does not establish upload completion or publication.
Preserve the batch and all per-file journals. For uncertain setup, invoke
`publish-prepare-resume` with that file's original index and
`--source-run publication-setup-batch/file-NNNN`, writing a new recovery directory.
There is no whole-batch replay or automatic retry. Files not yet started can use
ordinary indexed preparation with fresh output directories; never reissue a
claimed step. Reverify bytes immediately before transfer through the maintained
browser helper, including after any change since the batch's verification pass.

### Produce a complete confirmed-reference map

After each indexed file has transferred and the configured verifier has independently
observed and attested its complete bytes, use `publish-map` for the final
batch observation. The command reverifies all frozen files once before signing
queries or claiming output. Authenticate the tenant and operator independently;
the expected operator comes from the frozen installation input. It must match the
actual installed configuration, project, verifier, uploader and compiled library
release. A proposed installation alone cannot supply that proof.

```sh
blob-storage publish-map --network ic --url https://icp-api.io \
  --identity tenant.pem --actor TENANT --operator-identity operator.pem \
  --service SERVICE --namespace 1 --inputs publish-inputs \
  --gateway https://REVIEWED_GATEWAY \
  --max-bytes 10485760 --max-total-bytes 1073741824 \
  --max-queries 1352 --timeout-seconds 3600 --run-dir publication-map
```

Allow at least `2 × files + 2` queries (1,352 for Miner's 675 distinct blobs):
installed configuration, tenant capacity/enrollment, then exact verifier receipt
and first-reference status for each file. Each query has a thirty-second deadline;
the whole observation has the explicit deadline, at most one hour. Query replies
are bounded and the exact intents, arguments and raw results survive failures.
There are no service updates, provider requests or automatic retries.

Inspect `all_references_live`, including on exit zero. Missing or unmatched
completion, an inactive reference/tenant or a restore fence blocks the map;
malformed/foreign replies and transport errors fail with retained evidence.
`media-map.json` appears only when every file passes. It binds original file
indices and upload/reference identities to the installed project, shared Caffeine
request target, raw body SHA-256 and exact original headers. No partial success
list is a complete map. New observations use new directories and never overwrite
the original upload, observation, attestation or browser journals.

This is a sequence of authenticated observations, not an atomic snapshot or
publication lease. References may change afterward. It does not perform the
consumer's asset transaction, create overlapping-release references or qualify
public MIME/CORS/cache/CSP/retention. Consumers must join every emitted asset
identity to the original frozen index, retain the selected release references,
verify bounded bytes before decoding and own final publication/cancellation.
The command checks first references; additional release references use the exact
maintained retain/receipt/status workflows. Logical release still does not prove
provider deletion or billing cessation.

### Complete one file before preparing the next

At concurrency one, use indexed `publish-prepare`, then the maintained browser
`createPublicationUpload`, independent `observe-upload` and exact
`submit-attestation` before moving to the next original index. SDK success does
not free a reservation. A lost SDK reply is reconciled through complete download
observation; it never authorizes another upload. Retain each file's original setup,
browser, observation and signed-attestation journals, including uncertain claims.
Use the approved provider read budget and handle pending attestations through
`upload-attestation` with the saved statement, without resubmitting it.

`publish-file-status` supplies the authenticated per-file check:

```sh
blob-storage publish-file-status --network ic --url https://icp-api.io \
  --identity tenant.pem --actor TENANT --operator-identity operator.pem \
  --service SERVICE --namespace 1 --inputs publish-inputs --file-index 0 \
  --gateway https://REVIEWED_GATEWAY \
  --max-bytes 10485760 --max-total-bytes 1073741824 \
  --max-queries 4 --timeout-seconds 30 --run-dir confirmed-file-0000
```

Require `file_live:true` for the exact original index, digest and first reference.
The command authenticates installed configuration/release with the operator,
tenant enrollment/capacity, the configured verifier's accepted whole-body digest
and current unfenced reference status. Its shared decoders and target builder are
the same as `publish-map`; it sends no update or provider request. Missing
completion, release, suspension or fencing blocks progress. Exit zero alone is
insufficient. A successful file check sets `batch_complete:false` and never writes
`media-map.json`. Finish with the complete `publish-map` observation after every
file passes, then perform the consumer's asset transaction separately.

Independent CLI invocations reverify the complete frozen batch; this is bounded
but repeats file I/O. The persistent session below retains one validated batch
and reuses these phase owners. The current commands and local serial harness
are components, not a complete production headless publisher.
The local two-file trials use an owned HTTPS HTTP/2 substitute, synthetic bytes
and authored PNGs, including distinct full/partial chunks. Chromium fetches the
canonical native URLs and checks MIME/CORS/decoded bytes locally; these checks
do not qualify Miner's real media or deployed cache/CSP/retention behavior.
Logical release refuses new service descriptors but cannot revoke a saved public
URL. Treat the URL and downloaded copies according to the consumer's access policy.
Keep independent physical/liability limits: confirmation frees active concurrency,
while stored bytes and billing obligations remain accounted for.

### Hold one validated batch across publication phases

`publish-session` keeps the validated batch in one native process.
It authenticates the tenant, uploader and operator, compares the actual installed
configuration/release, and refuses a fenced host before its `ready` event.
It handles setup, indexed completion checks and final-map observations through
the same maintained command implementations. Optional `--verifier-identity`
authenticates the installed completion verifier before `ready` and enables a
`verify` phase that composes the existing whole-download observer and one-shot
attestation owner. Certificate/SDK transfer stays with the browser. This command is the native
phase controller. The browser [publication worker](../clients/browser/README.md#run-jobs-in-a-browser-worker)
supplies a fixed-authority job boundary. Released 0.11.0 adds a
[selected-signer browser host/bootstrap](../clients/browser/README.md#launch-the-maintained-worker-with-a-selected-signer).
That release also provides the [native Chromium bridge](../clients/browser/README.md#launch-chromium-from-a-native-parent)
with selected SDK identity JSON, bounded file loading and fixed profile/origin.
Released 0.13.0 persists the original immutable launch binding inside the profile
before Chromium opens, rejecting changed signer/scope/trust/journal/assets or
missing history. Keep older profiles with their original launcher and bundles;
the new reader does not create a binding for pre-existing history.
Complete durable parent phase/restart coordination remains unfinished.
Released 0.14.0 native `next_frame` guidance and the bridge's
[`driveSession`](../clients/browser/README.md#follow-native-phase-guidance)
now select and run phases around the existing owners. Native process/key selection
and explicit restart remain caller responsibilities.

```sh
blob-storage publish-session --network ic --url https://icp-api.io \
  --identity tenant.pem --actor TENANT --operator-identity operator.pem \
  --uploader-identity uploader.pem --service SERVICE --namespace 1 \
  --verifier-identity verifier.pem \
  --inputs publish-inputs --gateway https://REVIEWED_GATEWAY \
  --max-bytes 10485760 --max-total-bytes 1073741824 \
  --max-steps 2701 --timeout-seconds 3600 --run-dir publication-session
```

A parent process supplies one JSON object per newline on stdin and reads flushed
JSON lines on stdout. Keep stdin open until the final map or an intentional stop;
EOF ends with a typed transport failure and retained journals. Wait for `ready`
before sending frames. Each phase produces an `event:"phase"` line containing
`step`, `next_index` and `report`. Unreleased adds `next_frame`: a passive derived
continuation, or null when this phase has no automatic continuation. Map completion or step exhaustion then emits
the final CLI result and exits. Errors emit the normal redacted failure result.

| Control frame | Required handling |
| --- | --- |
| `{"phase":"prepare","index":0}` | Require `report.prepared:true`; a browser-selected session next requires `transfer`; native-only setup retains its descriptor |
| `{"phase":"transfer","index":0}` | With a browser selection, retain `report.native_phase` and pass its directory to the launcher; a repeated phase requests recovery only |
| `{"phase":"transfer","index":0,"source_transfer":"/absolute/original/step-NNNN"}` | Recover the exact original handoff; no certificate submission or upload dispatch is authorized |
| `{"phase":"status","index":0}` | After independent verification/attestation, require `report.file_live:true` before moving to index 1 |
| `{"phase":"verify","index":0}` | With a selected verifier, perform at most one bounded GET and one attestation; advance only if the exact current reference passes |
| `{"phase":"verify","index":0,"source_observation":"/absolute/original/verification-0000/observation"}` | Inspect the original complete observation and immutable attestation history, then current reference; no GET or submission |
| `{"phase":"prepare","index":0,"source_run":"/absolute/original/setup"}` | Recover the exact original per-file setup, preserving its signed journals; never name a recovery-report directory |
| `{"phase":"map"}` | After every original index passes, inspect `all_references_live`; only complete success writes the root `media-map.json` |

Frames are bounded to 8 KiB including newline; unknown fields/phases and truncated
frames fail. The control queue holds one frame. Choose a positive step budget of
at most `8 × files + 1`; 2,701 allows four phases per file plus the map for 675
files. Include transfer/recovery phases in the budget. A blocked phase consumes
a step; there is no automatic polling.
The finite session deadline includes configuration checks, control waits and
network phases after synchronous startup input verification. Filesystem I/O and
stdout writes are synchronous; this is not a preemptive filesystem deadline.
Idle stdin does not keep the process alive beyond the tested control deadline.

The session starts at index zero and advances only on authenticated exact
completion/reference evidence. It rejects out-of-order setup/status and premature
maps. A restarted session must inspect already-confirmed original indices again;
Released 0.13.0 `--source-session /absolute/retained-session` recovers original setup
and observation/transfer paths for omitted `source_run`/`source_observation`/
`source_transfer` frame fields.
Use a new `--run-dir`, the same frozen batch, principals, service/namespace,
gateway and trust root. The original installed configuration/release must match,
and fresh authenticated preflight still precedes readiness. Control budgets may
change within their bounds; they never renew effect authority.
The existing setup owner can still execute an originally unclaimed preparation
after matching admission evidence; it never repeats a claimed or uncertain step.

The strict native intent format is
`ic-blob-storage/publication-session:retained-browser-handoffs`. Source provenance
uses `ic-blob-storage/publication-session-sources:retained-browser-handoffs`.
No old session intent
reader or conversion exists. Retain original binaries and claims for old runs.
Recovery records carry direct original paths, including unused sources through a
status-only run, without following a session chain. A required provenance file,
control record or claim directory that is missing/partial refuses; an explicit
unattempted ordering fact distinguishes a phase that never called its owner.
History traversal and the provenance file are bounded; provenance is at most
8 MiB. Existing summaries do not establish completion or replay authority.
Without a source session, name each original `step-NNNN/setup/` or observation
explicitly in its frame. Both routes converge on the same source and phase owners.
Within a running session, repeated preparation uses its pinned original directory.
With a browser selection, setup does not return a transfer descriptor. The
`transfer` phase validates the original setup's input and signed admission/
preparation claims, then saves `transfer.json` before returning its canonical step
directory. This passive handoff is at most 512 KiB; no service query/update or
provider request runs in this phase. The certificate endpoint still checks fresh
authority when issuance occurs. The first handoff can be consumed once by the
maintained parent; repeat/recovery handoffs name the original directly and request
certificate recovery only, even if the browser journal is absent. Preserve the
original handoff; an archived first handoff is never retry permission. Browser
claims, not native summaries, remain authoritative for paid-effect dispatch.
Neither a restarted process, expired signed ingress nor a lost browser reply
authorizes redispatch. An already-exposed permission refuses preparation; reconcile
the original object through the independent verifier instead. Keep original
browser profiles and all setup/observation/attestation journals.

### Bind browser selection to the native session

Released 0.13.0 `publish-session --browser-selection FILE` accepts one passive JSON
selection, at most 16 KiB. The complete native intent has the same bound and is
checked before output allocation. The selection is copied into that intent before
configuration checks or phase execution; it never contains signer keys or effect
claims. Omit this flag for native-only inspection/verification use.

| Field | Required selection |
| --- | --- |
| `format` | `ic-blob-storage/browser-selection` |
| `session` | Absolute canonical planned original session directory, equal to the initial `--run-dir` |
| `profile` | Distinct absolute canonical planned profile directory; both paths have existing canonical parents |
| `project`, `bucket` | Exact original namespace for every file in this browser batch |
| `asset_port` | Fixed positive loopback port, at most 65535 |
| `signer_sha256` | Lowercase SHA-256 of `JSON.stringify(selectedBootstrap.signer)`; preserve its exact selected serialization |
| `host_sha256`, `worker_sha256` | Lowercase SHA-256 of the actual trusted bundle bytes |
| `database`, `max_slots` | Original IndexedDB database name and positive capacity, within maintained browser bounds |

The first native session requires a fresh profile path. Wait for its `ready`
event before calling the launcher with `nativeSession` set to the original session
directory and journal `mode:'create'`. This persists a profile binding to the exact
original intent and checks namespace, identity, root and inputs before Chromium.
The launcher accepts only the complete current intent/ready shapes; it never
reconstructs partial native history.

On native restart, pass the same selection file plus `--source-session` and a new
`--run-dir`. The original profile/session directories must survive, and saved
source intent must match the selection exactly. The browser reopens with
`mode:'open'`, still pointing at the original native session; no profile or binding
is regenerated. Browser-only operation uses explicit `nativeSession:null` when
first creating its profile and cannot bypass an existing native binding.
Keep signer keys separately. Neither startup readiness nor provenance establishes
current completion or permission to repeat an uncertain effect. Phase control,
original setup/observation/attestation claims and fresh authenticated completion
remain authoritative; automatic parent phase/restart coordination is unfinished.

### Verify original session uploads

Verification binds the original permission, raw body digest, configured gateway
and installed verifier before signing. Each original index has one create-new
`verification-NNNN` directory, separate from process step numbers. Corrupt bytes,
partial observations, refused/uncertain submissions and exhausted deadlines stop
the session without advancing. Pending submissions can produce `file_live:false`;
inspect status explicitly, without repeating `verify`. A second verification of
that index refuses its existing directory. Recovery retains the original observer
and signed attestation files; it never regenerates a statement or submits it again.
An absent historical receipt does not authorize another update. A complete
observation without a submission can still be passed explicitly to the existing
one-shot `submit-attestation` tool; its original create-new claim owns dispatch.
A new session/root is not authority to repeat an interrupted verification: use
`source_observation`, or inspect already confirmed indices with `status`.

Full-batch bodies and metadata are validated once at startup. Setup rehashes the
selected body against that frozen digest; completed and unrelated bodies are not
rescanned for each phase. The returned `transfer` includes cached binding,
manifest JSON, original preparation hints, raw digest/byte count and the body path. The browser must
snapshot that body and recheck raw digest and SDK root before certificate intent,
using `createPublicationUpload`; a path alone is not verified content. Final maps
describe the initially validated intent and current reference observations, not
the present integrity of every local body copy or an atomic publication lease.

Intent, exact control requests and query replies are saved under a private
create-new root, with existing signed setup journals below each step. The recorded
maximum is two setup updates per original file, plus one attestation when a verifier
is selected, and a conservative `5 × max_steps + 2 × files + 3` service-query bound.
With a verifier, budget at most one provider GET per original file; reads may incur
charges. Without that explicit identity, verification refuses before effects.
Recovery performs no provider request or attestation update. External transfers
need their own budgets; no funding, automatic retry or guaranteed spend cap follows.
Exit zero may describe blocked/incomplete
work; inspect typed report fields before continuing or publishing.

## Admit and prepare an upload

These commands make the service setup callable without a test harness. Start
with a saved binary Candid `UploadAdmissionRequest` (`permission.candid`) and
`UploadManifestRequest` (`manifest.candid`). The latter contains that exact
permission and the prepared original headers/ordered leaves. Obtain these from
[`upload-inputs`](#generate-upload-inputs-offline), the integrating application's maintained Rust DTOs and
[existing file preparation](local-tools.md#prepare-one-file), or the upstream
browser preparation bridge. No file body is sent to the service. IDs and expiry
must come from the application's allocation/intent policy; the CLI never allocates
them or renews them. The operator must already have enrolled the tenant.

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  admit-upload --network ic --url "$IC_API_URL" --identity "$TENANT_PEM" \
  --actor "$TENANT_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --request permission.candid --run-dir new-admission

cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  prepare-upload --network ic --url "$IC_API_URL" --identity "$UPLOADER_PEM" \
  --actor "$UPLOADER_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --request manifest.candid --run-dir new-preparation
```

Admission and withdrawal require the actual named tenant; preparation requires
the exact admitted uploader. Both identities are explicit even when they happen
to match. Canister tenants instead use `ReplicatedUploadAdmissionClient` through
their own application authorization; a PEM cannot impersonate that tenant.
Preflight reuses the core's permission, metadata/length/leaf/root validators.
Preparation input/reply is bounded to 64 KiB, 1024 leaves, 16 headers/4096 header
bytes and 1 GiB declared content; permission input/reply is bounded to 4 KiB.
Service configuration may impose smaller limits. Transport retains the common
256 KiB HTTP ceiling, thirty-second deadline and identity/trust rules below.

Each mutation claims a fresh private run, writes canonical `request.candid`,
`permission.candid`, `signed-request.cbor` and `intent.json` before dispatch, then
records `outcome.json`. `acknowledged`, typed `refused`, `pending` and `uncertain`
remain distinct. Existing/partial runs refuse reuse; no polling or automatic retry.
An unusable acknowledgment can follow a committed reservation or preparation.

Recover with `upload-permission --request new-admission/permission.candid` as the
tenant, or `upload-manifest --request new-preparation/permission.candid` as tenant
or uploader, using the same identity/trust/service/namespace flags and no run-dir.
These signed queries leave the original submission artifacts unchanged. They
report retained history, not which lost call produced it, current readiness,
renewed expiry or authority to resend. Unknown permission and a `null` unprepared
manifest never authorize repeating an uncertain effect.

`revoke-upload` takes the same original permission and a fresh run as the tenant.
Before exposure it cancels the reservation and releases bytes while retaining
operation/root/manifest history. Escaped effects and confirmed references retain
their separate obligations; withdrawal is not provider deletion or billing stop.
An exposed upload may still be independently verified and confirmed after
withdrawal, so its actual stored bytes can be accounted for. Keep an explicit
release intent when cancelling publication: once that exact object is confirmed,
release its reference through `submit-reference`. Attestation replay retains the
original receipt and does not reactivate a released reference. Inspect liveness
separately; a revoked upload permission alone does not release a live reference.
Restored owners permit exact inspection and refuse every mutation. Successful
setup never issues a certificate or completes an upload; installed restricted
certificate facts and current-owner checks still apply.

## Download a verified file

Use the tenant's own PEM and the exact current `DownloadRequest`, Candid-encoded
from the asset's saved service/tenant/namespace/root/object/incarnation/reference.
For the first reference, use `download.candid` from `upload-inputs`; for another
explicit reference use the output of [`reference-inputs`](#generate-reference-inputs-offline).
Obtain the expected project and approved gateway origin from installation policy;
the command never discovers them from the provider response.

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  download --network ic --url "$IC_API_URL" --identity "$TENANT_PEM" \
  --actor "$TENANT_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --request download.candid \
  --project "$CAFFEINE_PROJECT" --gateway "$APPROVED_GATEWAY_ORIGIN" \
  --max-bytes 10485760 --run-dir new-download
```

The command claims a fresh private directory and saves the request/plan before
one `blob_download_descriptor` update. It authenticates the IC update certificate
and checks the exact live reference, installed owner/project, declared size and
original hash headers before any provider GET. A service refusal issues no GET.
Canister tenants instead use `ReplicatedDownloadClient` through their own
application's authorization; a PEM cannot impersonate a canister tenant.

One GET streams into private `body.part`. Successful EOF, length and Caffeine-root
verification, followed by file sync, publishes `body.bin` without replacement.
`summary.json` and stdout report its path, raw content digest and original headers.
HTTP metadata cannot replace the service's original hash metadata. Failure or
interruption can leave partial files and retained intent; never use `body.part` as
verified output. Interruption after verified publication can leave `body.bin`
without a summary. Existing or interrupted run directories refuse reuse.

`http-response-headers.json` separately retains parsed response observations for
MIME/length/encoding/range, cache validators, Vary, CORS, content disposition,
nosniff and cross-origin resource policy. Each entry has a lowercase `name` and
`value_bytes` array; duplicate values remain separate. Capture stops before a
whole occurrence would exceed 32 entries or 8 KiB of combined name/value bytes,
setting `complete: false`. Remaining selected names can then be unobserved.
Completeness concerns only these selected headers; unrelated headers are omitted.
This sidecar is retained before HTTP-status/body admission and does not replace
the existing status artifact or attestation inputs. An incomplete capture does
not invalidate an independently verified body.

These are parsed native observations, not an HTTP wire transcript or proof that
a browser accepts the origin, MIME, CORS, CSP or cache policy. Original service
metadata still owns the content root. Production serving and the consumer's
certified media-map/asset publication need their own acceptance evidence.

Both the descriptor wait and GET have thirty-second deadlines. The service
transport has a 256 KiB response ceiling; request/reply Candid is bounded to 4 KiB
with decoding-work limits. `--max-bytes` is an explicit positive ceiling, at most
1 GiB, and also bounds the independently declared body. No redirects, automatic
retries, range response or content decompression are accepted. Local mode requires
literal loopback origins and an independently supplied `--root-key`; IC mode
requires HTTPS and the built-in IC trust root. Select the origin and provider-read
budget under the [probe ledger](evidence/caffeine-probes/README.md); charges remain
unknown rather than being treated as free.

A descriptor is a snapshot, not a publication lease. Releasing a reference cannot
recall an already downloaded file. Verified bytes establish observed content only,
without future retention, confidentiality or billing-cessation guarantees.
Successful signed managed coverage uses labelled local exposure and content;
standalone proves unconfirmed/fenced refusal. Both production hosts still require
the existing issuance/provider/recovery gates before a real upload can complete.

## Identity, trust and service status

The unpublished native `blob-storage` command authenticates to the shared service
API with an explicit Ed25519 or secp256k1 PEM identity. Set these variables from
the installation's configuration and the operator's own identity:

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  status --network ic --url "$IC_API_URL" --identity "$OPERATOR_PEM" \
  --operator "$OPERATOR_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --cashier "$CASHIER_PRINCIPAL" \
  --payer "$PAYER_PRINCIPAL"
```

The PEM's principal must match `--operator` before any network request, and the
service must authorize that caller. Namespace is a positive canonical decimal
u128. The complete installed scope is checked in both request and response. IC
mode requires an HTTPS origin and uses ic-agent's built-in IC root key. Local
mode requires `--network local`, a literal loopback origin and `--root-key` naming
a trusted DER root obtained independently from the local replica owner; root
overrides are rejected in IC mode. No root is fetched automatically. URL paths,
credentials, query strings, fragments and HTTP redirects are rejected.

The `status` command only queries `blob_local_status`. It verifies IC query signatures;
the result is a local observation, not certified state, provider credit, readiness
or dispatch authority. It exposes separate upload/funding/gateway/read fences,
including after restore. All counters and amounts are decimal strings, optional
identities remain `null`, and query failures never become zero balances. Exit 0
means an observation was returned (even if fenced), 2 means invalid arguments,
and 3 means a file, identity, transport, decoder, binding or service refusal. Errors
are structured JSON codes; private key contents and remote diagnostics are omitted.
The call has a 30-second deadline, a 256 KiB HTTP response ceiling and a 64 KiB
Candid reply bound with decoder work limits. It never falls back to an update.
Run `make test-standalone` for the local signed HTTP subprocess evidence.

## Account inspection

Choose one observation with `inspect-account --kind balance|relationship` and
supply the same identity/trust and complete operator scope as `status`:

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  inspect-account --network ic --url "$IC_API_URL" --identity "$OPERATOR_PEM" \
  --operator "$OPERATOR_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --cashier "$CASHIER_PRINCIPAL" \
  --payer "$PAYER_PRINCIPAL" --kind balance
```

The client submits the existing `blob_inspect_account` service update once;
that workflow performs a replicated Cashier query. It attaches no provider cycles,
changes no service credit/allocation and starts no payment. The client verifies
its IC update certificate, then checks the exact echoed request and observation
kind; this authenticates the service reply, not a separate provider signature or
future provider guarantee. Local mode uses the independently supplied root as
above. A restored installation refuses with `account_fenced`.

JSON preserves reported total, prepaid, promotional and ledger balances separately;
it does not recompute the total. Relationship amounts retain arbitrary-width
signed decimal strings, counters use decimal strings and absent expiration is
`null`. Present relationships must match both the service and payer. Missing
accounts, absent reported relationships and typed provider errors are explicit
`provider_report` observations, exit 0. `no_relationship_reported` does not prove
provider absence. Transport, malformed/oversized reply, authority, scope and fence
refusals exit 3. No failure becomes a zero balance. Every successful output sets
`provider_credit` and `spendability` to `not_established`, and
`retry_authorized: false`.

The command has a thirty-second deadline, 256 KiB HTTP response ceiling and 4 KiB
Candid reply bound with work/type/header limits and no skipped fields. Waiting
may inspect the exact IC request ID; it never resubmits, automatically refreshes,
funds or writes a journal. A timeout or failed certificate validation can follow
execution of this read update: treat the result as unobserved, not proof the
Cashier was never queried. Native Agent transport also returns HTTP 429/503
backpressure without automatically resending queries or updates.

Recorded historical signed host journeys retain complete stable memory across
observations/refusals and check same-release restore. Cashier replies are local
substitutes; see [account evidence](evidence/core-primitives.md#signed-native-account-inspection-and-bounded-transport--2026-10-01).
Live provider observations still require the selected scope and budget recorded
in the probe ledger.

## Gateway controls

Use the same operator identity, trust and complete scope as `status`. Each
command requires a new `--run-dir` under an existing durable parent directory:

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  sync-gateways --network ic --url "$IC_API_URL" --identity "$OPERATOR_PEM" \
  --operator "$OPERATOR_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --cashier "$CASHIER_PRINCIPAL" \
  --payer "$PAYER_PRINCIPAL" --run-dir ./gateway-sync-intent
```

| Decision | Required choice | Service effect |
| --- | --- | --- |
| `sync-gateways` | No guessed sequence or gateway | Persist pending identity, query installed Cashier once, then replace membership if still current |
| `cancel-gateway-sync` | `--sequence DECIMAL` from observed pending status | Cancel only that read-only sync, preserving allocated sequence history |
| `revoke-gateway` | `--gateway PRINCIPAL` | Remove local membership and invalidate pending sync/read observations, even if already absent |

For cancellation or revocation, replace `sync-gateways` in the example, add the
required choice and use a distinct run directory. Cancellation sequences must be
positive canonical u64 decimals; principals must be concrete canonical identities.
These are current operator decisions. Revocation is local membership removal;
it does not delete provider bytes, stop billing, release read slots or permanently
ban future re-addition. Neither cancellation nor revocation queries the provider.
Sync attaches no provider cycles; normal IC execution costs remain separate.

Before network dispatch, the client atomically claims a private directory and
syncs `request.candid`, `signed-request.cbor` and `intent.json`, binding the
complete scope/operator/decision, method, request ID, expiry and hashes of request,
signature and trusted root. Existing or partial directories refuse before sending.
These files are operator-owned recovery evidence; they are not an independent
freshness authority after copying/restoring. Keep signed artifacts private.

The client submits once with a thirty-second deadline and does not poll, retry,
allocate sync IDs or automatically inspect status. HTTP backpressure also refuses
without resubmission. A verified, bounded Candid acknowledgment is saved with the
original request; sync scope/positive sequence and revocation scope/gateway must
match. Cancellation's unit reply acknowledges the saved exact request under its
IC request ID. Native limits are 256 KiB HTTP and 4 KiB Candid with work/type/header
bounds and no skipped fields. Oversized replies remain uncertain without storing
unbounded bytes. Bounded malformed replies remain retained.

`outcome.json` distinguishes `acknowledged`, `pending`, `refused` and
`uncertain`. A typed service refusal exits 3; it can still leave pending work
from a failed sync. Pending admission exits 0 but is not an acknowledgment.
Transport/trust/decoder uncertainty exits 3 and may follow a committed effect.
Local file failure can leave a partial claim without outcome; retain it and inspect
rather than dispatching again. Every outcome keeps `retry_authorized: false`,
with provider deletion/billing cessation unestablished and historical receipt
unavailable. The current service does not retain gateway mutation receipts.

Use a separate signed `status` command to inspect membership, last sequence and
pending identity. A cleared pending identity or absent member shows current state;
it cannot prove which operation changed it or authorize repetition of an uncertain
revocation/sync. A failed sync's exact observed pending ID can be the input to a
new explicit cancellation decision; stale IDs conflict. Same-release restoration
preserves pending history but fences all three controls. Local signed journeys
in the recorded historical host evidence use a query-only Cashier substitute; see
[gateway evidence](evidence/core-primitives.md#signed-native-gateway-controls--2026-10-01).
Production provider/provenance and operational recovery qualification remain open.

## Funding history

Use `funding-history` in place of `status` with the same authentication and scope
flags to read one descending page from `blob_funding_history`. It accepts at most
32 entries and reuses the library decoder to validate request echo, scope, ordering,
amounts, refunds and continuation. The JSON `entries` preserve `prepared`,
`uncertain`, `not_enqueued` and `callback` phases; callback refunds do not prove
provider credit. Empty and fenced pages still exit 0.

For another page, save the non-null `next` object from the prior result as a JSON
file and pass `--cursor FILE`. The file is limited to 2 KiB and includes the complete
scope plus a decimal-string `before_operation`; a changed scope rejects before
transport. No cursor means a fresh sweep. `next: null` ends this local range; it
does not prove complete provider-account activity. Pages are current observations,
not a snapshot: restart from the beginning to see changes behind a saved cursor.
The command never automatically paginates, retries a payment or writes a journal.
Local tests cover populated history through the shared durable storage fixture;
its payment outcomes are controlled substitutes, not deployed Cashier evidence.

## Passive funding assessment

Use the installed operator identity to diagnose a proposed intent:

```bash
cargo run --locked -p ic-blob-storage-cli --bin blob-storage -- \
  funding-assessment --network ic --url "$IC_API_URL" --identity "$OPERATOR_PEM" \
  --operator "$OPERATOR_PRINCIPAL" --service "$STORAGE_CANISTER" \
  --namespace "$NAMESPACE" --cashier "$CASHIER_CANISTER" --payer "$PAYER_PRINCIPAL" \
  --operation "$PROPOSED_OPERATION" --offered "$OFFERED_CYCLES"
```

Add `--target-balance DECIMAL` only for an exact positive provider target;
absence stays absent. Operation and offer must be positive full-width decimal
integers. The proposed identity is not allocated by this query and is not evidence
of freshness, especially after restore. Changed amounts/target for a retained
identity return `funding_conflict`; use the original funding outcome to inspect it.

One `blob_funding_preparation_assessment` query returns the exact request, maintained
local journal totals and independent blockers. It reports retained/stale identity,
lifetime capacity, local attachment allowance, uncredited/uncertain amounts and
actual fencing. The hosts have no trusted production evidence acquisition path:
`provider_unqualified`, `recovery_unknown`, `funding_unknown` and
`spendability_unknown` remain even when the local offer fits an empty journal.
Caller flags cannot provide these facts. Reported provider balances do not supply
credit, complete account activity or spendability.

Exit 0 means a successfully observed assessment, including its blockers; typed
refusals exit 3 and invalid arguments exit 2. JSON uses decimal strings for amounts
and identities. The CLI checks exact echo, bounded decoding and consistency of
local blockers, with a thirty-second deadline, 256 KiB HTTP and 4 KiB Candid
ceilings. Query signatures require the same explicit trust as status. No reservation,
payment, provider query, automatic retry or restore-fence release occurs;
preparation/dispatch/retry authority are always false. Never cache the report as
future authorization. Local both-adapter evidence keeps the Cashier stopped and
occupied unrelated owners intact through restored inspection. See
[assessment evidence](evidence/core-primitives.md#passive-funding-preparation-assessment--2026-10-01).

## Exact funding outcomes

Inspect an exact original funding intent with `funding-outcome`:

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  funding-outcome --network ic --url "$IC_API_URL" --identity "$OPERATOR_PEM" \
  --operator "$OPERATOR_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --cashier "$CASHIER_PRINCIPAL" \
  --payer "$PAYER_PRINCIPAL" --operation "$FUNDING_OPERATION" \
  --offered "$ORIGINAL_OFFERED_CYCLES"
```

Use the original decimal-string operation, offer and optional target from funding
history or your retained intent. Supply `--target-balance DECIMAL` only when the
original intent had a target; omission means `None`, with no inferred default.
Changed retained amounts or target produce `funding_conflict`, not absence.
All supplied numbers must be positive canonical u128 decimals.

One signed `blob_funding_outcome` query returns `outcome: found` with a `record`,
or `outcome: absent` with `record: null`. Absence supplies no fence information,
even on a restored installation; inspect status separately for owner fences.
A found record retains local phase, exact callback refund, optional structured
response, conservative reconciliation and its restore fence. `response: null`
means no structured reply was retained. Reported success balances and typed
provider errors remain separate from attachment accounting.

`transfer_unknown` keeps the entire original offer potentially spent.
`credit_required` names the exact accepted attachment still needing independent
provider-credit evidence. `no_transfer` concerns attached cycles only; execution
fees are separate. `credit_confirmed` carries the accepted amount and original
receipt SHA-256 fingerprint retained by the trusted host; it sets
`provider_credit: host_confirmed`. Other states retain `not_established`.
Every output sets `retry_authorized: false`. Empty, uncertain and fenced observations
exit 0; authenticated service refusals exit 3. The bounded decoder checks exact
intent and transport/reconciliation consistency within 4 KiB, with the same
30-second signed-query and 256 KiB HTTP limits as status. No payment, provider
query, polling, journal write or automatic retry occurs.

Hosts can use the [internal credit-confirmation contract](funding-credit.md) after
independently establishing exact provider credit. No standalone credit setter is
exposed. Status separates lifetime `transport_accepted` from `uncredited_accepted`;
confirmation clears only the covered local blocker, without replenishing the
funding allocation or releasing recovery fences. A separate host-authorized
[bounded budget grant](funding-credit.md#bounded-host-authorized-allocation-increases)
can increase authorization within the installed ceiling after complete credit
reconciliation and before the next intent. It preserves spent totals and lifetime
slots. Status shows cumulative authorization and the ceiling; exact outcomes show
the original intent's immutable grant.

## Upload history

`upload-history` recovers retained upload identities without saved upload requests:

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  upload-history --network ic --url "$IC_API_URL" --identity "$OPERATOR_PEM" \
  --operator "$OPERATOR_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --filter outstanding
```

The configured operator observes the installed service-wide history. `--filter`
is required: `all` includes cancellation and settlement, `active` includes reserved
or possibly exposed uploads, `deletion-pending` selects unresolved physical deletion,
and `outstanding` also retains live/provider-deleted content with unresolved
obligations. Physical deletion alone does not establish billing cessation.
No Cashier or payer flags are needed for this local history query.

Each command queries `blob_upload_history` once, accepting at most 64 inspected
rows and 32 matching entries in 64 KiB of bounded Candid. JSON preserves all original
upload/object/incarnation/first-reference identities, roots, byte lengths, local
states, the independent restore fence and decimal-string `scanned` count.

Save the non-null `next` object as JSON and pass `--cursor FILE` for another page.
The file is limited to 2 KiB and binds service, namespace, service-wide observer
scope, filter and last inspected tenant/upload ID; malformed or changed scope
rejects before identity/network access. Empty filtered pages may still have a
continuation. `next: null` ends this local traversal, not provider reconciliation.
Pages are current observations; start without a cursor to see changes behind it.

The shared decoder checks ordering, identity scope, duplicate roots/object lifetimes,
filter results, work bounds and cursor progress. Signed queries use the same explicit
IC/local trust and deadline as status. Empty and fenced observations exit 0;
service refusals remain errors. The command reads no provider content, performs
no mutation and never grants retry, serving or recovery authority.

## Generate reference inputs offline

Use the exact original `permission.candid` from upload preparation. Select the
reference and operation through the application's allocation/intent policy; these
canonical positive decimal u128 values are supplied explicitly, never allocated
by this tool. For release, name the reference being released. For retain, name
the application's fresh reference. Preserve that exact operation for recovery.

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  reference-inputs --permission permission.candid --action release \
  --reference "$REFERENCE_ID" --operation "$OPERATION_ID" --run-dir new-reference-inputs
```

The fresh private directory saves the exact permission, `reference.candid` for
`submit-reference`/`reference-receipt`, `reference-status.candid` for status and
`download.candid` for the same reference, followed by a hashed `summary.json`.
Use `--action retain` for a retain command. It checks bounded Candid and the
maintained core binding invariants before claiming output. Permission input is
capped at 4 KiB with bounded decoding work; malformed, noncanonical and duplicate
options refuse. Existing or partial directories never resume or overwrite.

No identity, URL or root key is needed. This is local request preparation, not
authenticated authority, admission, completion, current liveness or retry approval.
An expired upload permission remains usable as the original cleanup binding; its
expiry is neither renewed nor used to authorize another upload. A generated
download request may name a released or unconfirmed reference: the signed service
handler still enforces current authority and liveness before any provider GET.
Saved files remain mutable local data under the application's intent policy.

## Submit a reference

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  submit-reference --network ic --url "$IC_API_URL" --identity "$TENANT_PEM" \
  --actor "$TENANT_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --request reference-command.candid \
  --run-dir ./reference-dispatch
```

The input is one binary Candid `ReferenceCommand`, with the original upload,
explicit positive reference/operation IDs and exact retain/release action.
Generate it with [`reference-inputs`](#generate-reference-inputs-offline) and use
its `reference.candid`, or encode the same maintained DTO in the application.
The client validates its tenant and scope before signing, claims a new private
directory and syncs `request.candid`, `signed-request.cbor` and `intent.json`
before sending one `blob_apply_reference` update. The intent binds signer,
target, network/trust, full operation, signed expiry/request ID and file hashes.
There is no polling, automatic resend, ID allocation or provider call. Keep
the signed artifacts private. A canister tenant uses `ReplicatedReferenceClient`.

Read `outcome.json` and the JSON output together: `recorded` means the service
returned an exact receipt; `result.state` is either `success` or a stored
transition `failure`. A recorded failure exits zero and does not mean retain/release
succeeded. `pending` and `uncertain` require `reference-receipt` with the saved
`reference-dispatch/request.candid`; `refused` preserves the typed service refusal.
If receipt inspection itself refuses (`reference_unknown`, `reference_unconfirmed`
or another typed error), the original pending/uncertain outcome stays unresolved.
Do not replace it with absence or assume non-execution. Standalone tests exercise
this boundary with broader installations that refuse restricted issuance; a
prepared reservation cannot become a successful reference through this command.
Existing/partial directories refuse, including an interrupted empty claim.
Preserve them and the original input for reconciliation. A new directory is not
retry authority; neither an absent receipt nor expired ingress permits resend.
Local files have no independent freshness authority and do not coordinate an
application outbox or asset publication. Historical retain success is separate
from current liveness; logical release establishes neither provider deletion nor
billing cessation.

## Reference receipts and current status

Tenant-owned reference inspection uses separate saved boundary requests:

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  reference-receipt --network ic --url "$IC_API_URL" --identity "$TENANT_PEM" \
  --actor "$TENANT_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --request reference-command.candid

cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  reference-status --network ic --url "$IC_API_URL" --identity "$TENANT_PEM" \
  --actor "$TENANT_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --request reference-status.candid
```

`reference-command.candid` is the original binary Candid `ReferenceCommand` saved
before dispatch. It includes the complete original upload, independent reference
and operation IDs and exact retain/release action. `reference-status.candid` is a
binary Candid `ReferenceStatusRequest` containing that original upload and the
reference to inspect; it has no mutation operation or action. Both inputs are
bounded to 4 KiB and validated against explicit service, namespace and tenant
before identity/network access. The PEM must sign as that tenant. Operator and
controller status grant no tenant authority. A canister tenant cannot be
impersonated with a PEM; use its existing `ReplicatedReferenceClient` integration.

`reference-receipt` signs one `blob_reference_receipt` query. JSON distinguishes
`absent` from `found`, with an original success (`changed`/`unchanged`) or recorded
transition failure. Lookup refusals remain errors, including `reference_unknown`,
`reference_unconfirmed` and `reference_conflict`. A successful retain receipt stays
successful after release or settlement. This reply carries neither a current
liveness observation nor a restore fence; both are labelled `not_observed`.
Absence supplies no retry authority and does not prove that an upload is confirmed.

`reference-status` independently signs one `blob_reference_status` query. It
returns the exact reference's current local `live` flag and owner's `fenced` flag.
False includes never-retained and released references; it never makes an identity
reusable. Even true with a clear local fence is no publication lease, provider
availability guarantee or independent recovery proof. The two observations are
separate in time; neither command silently performs the other query.

Both commands preserve full-width decimal identities/amounts, verify query
signatures with explicit IC/local trust, and use a 4 KiB reply limit, 256 KiB HTTP
ceiling and 30-second deadline. Exit 0 means an observation, including absence or
recorded failure; exit 3 means transport, decoding, binding or lookup refusal.
They set retry/publication authority to false and perform no mutation, provider
request, journal write, polling or automatic retry.

## Share a confirmed blob within a tenant

For two application assets using the same confirmed blob, retain a second reference
before releasing the first. Keep the original permission and allocate a fresh
reference and operation in the application; this does not grant another tenant
access or upload the body again.

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  reference-inputs --permission permission.candid --action retain \
  --reference "$SECOND_REFERENCE_ID" --operation "$RETAIN_OPERATION_ID" \
  --run-dir second-reference-inputs
```

Submit `second-reference-inputs/reference.candid` using
[`submit-reference`](#submit-a-reference) and a fresh dispatch directory. Confirm
`result.state: success`; after a lost/pending reply, inspect the exact dispatch's
saved `request.candid` with `reference-receipt` and inspect current liveness with
`second-reference-inputs/reference-status.candid`. Keep the uncertain artifacts;
receipt success does not overwrite the transport outcome or authorize resend.
Use `second-reference-inputs/download.candid` for its verified download.

| Local step | First reference | Second reference | Accounting |
| --- | --- | --- | --- |
| Confirm original upload | Live | Not retained | Logical, physical and liability bytes remain charged |
| Retain second reference | Live | Live | The existing blob remains stored |
| Release first reference | Released | Live | The second reference still permits download |
| Release second reference | Released | Released | Logical bytes released; physical and billing liabilities remain |

Each release needs its own original operation ID and generated command. Once the
last reference is released, both downloads refuse; historical retain success does
not revive it. Same-release restored owners expose fenced/inactive history and
refuse mutation. Downloads already delivered remain on disk. Reference status and
descriptors are observations, not leases or future-availability guarantees.
The [local signed journey](evidence/core-primitives.md#shared-native-reference-downloads--2026-10-01)
checks this flow using the existing labelled exposure/content substitute; real
provider issuance, deletion and billing cessation remain separate gates.

### Overlapping application releases and lifetime capacity

Use the application’s existing release/asset manifest to associate each supported
release with its exact service, tenant, permission and reference. References are
the storage owner; this recipe adds no service-side release journal. Keep the old
release’s reference while its assets or rollback remain supported. Before
advertising a new release, retain a fresh reference for each reused confirmed root
and verify its current status. Publish the consumer mapping using the consumer’s
own transaction. Only after that transaction and its rollback window finish may
the old reference be released. A URL is not the reference identity.

Inspect `blob_reference_capacity` using the tenant-authenticated
`ReferenceCapacityRequest { scope, root }` and its typed response. This is an
existing standalone/core boundary; the CLI has no dedicated capacity command.
It returns `headroom: None` for unknown, foreign or unconfirmed roots, which does
not prove global absence or authorize a replacement allocation. Check the
separate root discovery state and original records. Retired roots cannot be
uploaded again; restored owners are inspection-only.

For media removed from the consumer mapping and later restored, the saved exact
object identity and its **current** lifecycle determine the supported path:

| `UploadContentState` | Supported action |
| --- | --- |
| `Reserved`, `ExposurePossible` | Recover the original operation and its journal; expired evidence does not authorize another provider effect |
| `Live` | Retain a new exact reference if current tenant authority and capacity permit, then commit the consumer mapping |
| `Cancelled`, `DeletionPending`, `ProviderDeleted`, `Settled` | Typed retained history establishes retirement; reintroduction in this installation is unsupported and the root stays claimed |

The native discovery result `retired_root` represents these retired states; it is
not a deletion/re-upload permission. Even `Settled` preserves lifetime identity
history. If rollback or later restoration must remain possible, keep a live
reference throughout that support window. If none survives, stop the publication
and choose an explicitly reviewed retirement/new-installation plan. Do not reset
the old owner or discard unresolved deletion or billing obligations.

| Headroom field | Planning use |
| --- | --- |
| `reference_slots` | Remaining lifetime identities; released IDs remain occupied |
| `unreserved_receipts` | Receipt slots not already committed to active-reference cleanup |
| `release_reserved_receipts` | Cleanup slots held for existing live references |
| `fresh_retains` | For a live root, `min(reference_slots, unreserved_receipts / 2)`; otherwise zero |

Each new retain consumes a receipt and reserves another for eventual release.
Three free reference slots and five unreserved receipts permit two fresh retains.
Releasing old references does not recycle IDs or historical receipts. Budget
release churn over the installation lifetime, rather than only simultaneous
releases. Headroom is an observation, not a reservation: enrollment, scope,
identity, current liveness and fencing are checked again at mutation time.
A capacity refusal stops publication; changing operation IDs or resetting the
owner is not a remedy. Preserve cleanup reservations and apply the
[retirement runbook](retiring-installations.md) at lifetime exhaustion.

| Interruption | Recovery using existing owners |
| --- | --- |
| Retain reply lost or pending | Inspect the saved exact command’s receipt and current reference status; keep dispatch uncertain until reconciled |
| Retain succeeded, mapping not published | Resume the consumer transaction from original evidence, or release that exact unused reference with its saved cleanup operation |
| Mapping publication outcome uncertain | Inspect the consumer transaction before cleanup; keep references needed by either possible advertised result |
| Old-release cleanup reply uncertain | Inspect the exact release receipt; historical retain success cannot establish current liveness |
| Final reference released | Both service downloads refuse; physical bytes and liabilities persist until separate provider disposition |

Upload-only `publish-check` reports `live_requires_retain` for reused content;
it does not perform the reference transaction. `publish-map` observes original
first references and is neither an atomic availability lease nor the consumer’s
publication commit. Additional-reference mappings belong to that consumer.
The local media overlap regression checks stored bytes and cleanup liability;
consumer adoption and actual deletion/final billing remain open under
[#6](https://github.com/dragginzgame/ic-blob-storage/issues/6).

## Certificate assessment

`certificate-assessment` inspects missing issuance prerequisites for a saved permission:

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  certificate-assessment --network ic --url "$IC_API_URL" --identity "$UPLOADER_PEM" \
  --actor "$UPLOADER_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --permission permission.candid
```

The input is one binary Candid `UploadAdmissionRequest` (4 KiB maximum). Service,
namespace and original uploader must match before transport. The command signs one
`blob_upload_certificate_assessment` query, verifies query signatures and decodes
at most 4 KiB against the complete saved permission, including independent object
identities and expiry. JSON preserves decimal-string integers, host assessment time
and every blocker. Assessment reports `namespace_binding`,
`current_owner`, `durability` or `stale_observation` when a local
prerequisite fails. These are not provider charge/provisioning guarantees.
Object and release capacity follow the validated installation configuration;
admission and preparation enforce quotas before certificate assessment or issuance.

Exit 0 means an assessment was observed, including blocked uploads; it never
authorizes issuance or retry. Even an empty list cannot reserve a later update.
Unprepared, expired, revoked and restored permissions refuse with distinct JSON
error codes and exit 3. The same explicit IC/local root trust, 30-second deadline
and 256 KiB HTTP ceiling apply. No certificate update, provider request, file
rewrite or service mutation occurs. This does not qualify provider behavior or
enable the standalone certificate endpoint.

## Verify a local file

`verify-upload` checks a saved local file against the service's original manifest:

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  verify-upload --network ic --url "$IC_API_URL" --identity "$UPLOADER_PEM" \
  --actor "$UPLOADER_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --permission permission.candid \
  --body downloaded.bin --max-bytes 10485760
```

`permission.candid` contains one binary Candid `UploadAdmissionRequest` saved by
the integrating application (for example, `candid::encode_one(permission)`), not
JSON or hexadecimal text. It is bounded to 4 KiB and binds the full original
operation, uploader and expiry. The signer must be that uploader or the tenant;
operator status supplies no override. The command queries `blob_upload_manifest`,
reuses its bounded exact-reply decoder and hashes file bytes in native 64 KiB frames.
`--max-bytes` is required and capped at 1 GiB; file size must equal the original
declaration. Metadata is bounded to 16 headers/4 KiB and the manifest to 1,024 leaves.
File changes during the read are still subject to length/root verification.

Exit 0 means the bytes read matched the authenticated historical declaration.
The JSON preserves full-width identities and a computed raw digest, and explicitly
reports provider completion/availability as unestablished/unobserved. It does not
fetch provider content, attest completion, write a destination or grant retry
authority. Inspection works after restoration without clearing fences. This is
separate from the configured verifier's trusted availability attestation described
in the [standalone contract](../canisters/standalone/README.md).

## Recover an attestation receipt

`upload-attestation` recovers historical evidence using the exact statement saved
before an attestation was sent:

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  upload-attestation --network ic --url "$IC_API_URL" --identity "$VERIFIER_PEM" \
  --actor "$VERIFIER_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --verifier "$VERIFIER_PRINCIPAL" \
  --statement statement.candid
```

The statement is one binary Candid `UploadAttestationRequest`, bounded to 4 KiB.
It contains the complete original permission, raw digest and observation time.
Obtain `--verifier` from the trusted installation configuration; the command does
not discover or install a verifier. The signer may be that verifier, the tenant
or the original uploader. No original body file is required, and the saved
statement is never overwritten. The service remains responsible for caller checks.

The signed `blob_upload_attestation` query reuses the library's bounded decoder
and checks the exact scope, permission, verifier and receipt chronology. JSON
`outcome` is `matched`, `conflict` or `absent`; `receipt` is null only for absence.
Conflicts retain both the expected digest/time and the accepted digest/time.
All three observations exit 0, including when fenced; callers must inspect the
outcome. Failures exit 3 and never become absence. IDs and times remain decimal
strings. Limits are 30 seconds, 256 KiB HTTP and 4 KiB Candid with bounded decoding.

A matched receipt proves the service retained that exact trusted statement. It
does not establish current availability, reference liveness or billing cessation.
Absence does not prove a pending update cannot still complete, and cannot authorize
resending an uncertain effect. Every outcome reports `retry_authorized: false`.
The command sends no update or provider request and writes no journal.

## Observe provider content

For a browser application's signer ownership, private worker, job correlation
and restart procedure, see the [completion verifier recipe](completion-verifier.md).

`observe-upload` performs the verifier's independent provider read and saves the
statement for explicit submission. Run it only against an explicitly approved gateway
and installation with a read budget; provider charges remain unknown, including
when a loopback origin forwards requests. No live trial was performed here.

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  observe-upload --network ic --url "$IC_API_URL" --identity "$VERIFIER_PEM" \
  --actor "$VERIFIER_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --permission permission.candid \
  --gateway "$APPROVED_GATEWAY_ORIGIN" --max-bytes 10485760 --run-dir new-observation
```

The service authenticates the verifier through `blob_verification_plan` and returns
its installed owner/project and exact original declaration. The command validates
the signed reply and uses the maintained Caffeine request-target encoder. The host
must have already recorded exposure. Standalone exposes an uploader-only
`blob_upload_certificate_assessment(root)` and the canonical certificate update,
with the accepted project-approved uploader/current-owner contract; see
the [host contract](../canisters/standalone/README.md).
Unexposed, confirmed or restored work rejects before any provider GET. Revoked or
suspended exposed uploads remain eligible for reconciliation.

One GET is allowed, with no redirects, HTTP retries, credentials or automatic
decompression. IC mode requires HTTPS origins; local mode accepts literal loopback
origins only. Both query and GET have 30-second deadlines. Service Candid is bounded
to 64 KiB, 1,024 leaves and 16 headers/4 KiB metadata. The explicit content budget is
at most 1 GiB and is checked before fetching. Whole-body streaming checks the
original root, metadata and exact length off-canister. HTTP metadata cannot replace
the original declaration; partial/encoded responses and incomplete EOF reject.
These request/byte limits are not a billing guarantee or a bound on transport overhead.

The run directory must be new and its parent must exist. Intent is synced before
the query and GET. Artifacts include `plan.json`, `permission.candid`, the bounded
`service-response.candid`, `download-request.json`, HTTP status and download outcome.
Only complete verification writes and syncs `statement.candid`, followed by
`summary.json` with its hash and observation time. The runner fingerprint covers its
observation/transport/argument sources and lockfile; artifact hashes establish local
integrity, not portable IC/provider signatures. Download bodies and identity keys
are never retained. Keep this directory in controlled storage.

Errors retain `failure.json` when writable. Abrupt interruption can leave partial
files with no summary; neither failed nor interrupted runs are resumed or overwritten.
Preserve their evidence before deciding on any separately budgeted new attempt.
A saved statement binds the exact original permission and locally observed UTC time;
the service still checks its admission/acceptance time bounds. The command sends no
attestation or upload, and reports `attestation_dispatched: false` and
`retry_authorized: false`. `verify-upload` local-file
output cannot be promoted to a provider observation.

## Submit an attestation

Submit a completed observation explicitly with the same service URL, service,
namespace and verifier identity:

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  submit-attestation --network ic --url "$IC_API_URL" --identity "$VERIFIER_PEM" \
  --actor "$VERIFIER_PRINCIPAL" --service "$SERVICE_PRINCIPAL" \
  --namespace "$SERVICE_NAMESPACE" --run-dir new-observation
```

The command validates every required observation artifact, the original declaration,
owner/project, complete download result, exact statement, hashes and time ordering.
Records are bounded and must be regular files; a failure marker rejects the run.
These files remain trusted verifier input: local hashes do not prove authenticity
against someone who can rewrite the entire run. Keep the directory under the
verifier's control, including during submission. No provider read occurs here.

An exclusive `attestation/` directory claims the run. Before one signed update,
the command syncs an exact `statement.candid` copy, `signed-request.cbor` and
`intent.json` containing the runner version/source fingerprint, request ID, expiry,
root-key and artifact hashes. The fingerprint covers the native submission,
observation-reader, shared transport/record/argument sources and lockfile.
Keep the signed envelope private; it can authorize that exact request until expiry.
The update has a 30-second deadline, disabled transport retries and no automatic
polling. A certified reply must match the complete statement before reporting
`accepted`. A queued request reports `pending`; transport or verification failure
leaves an `uncertain` outcome when writable. An authenticated service refusal is
recorded separately. The reply and outcome are retained without changing the
observation's original summary.

Every existing claim, including an empty directory left by a killed process, refuses
resubmission with `submission_already_claimed`. Do not remove or copy the claim to
retry. This is a local single-submission guard, not a distributed lock or global
exactly-once guarantee. Recover with `upload-attestation` using the saved
`attestation/statement.candid` and independently configured verifier. If interruption
preceded that copy, the original observation statement remains available for lookup.
An absent receipt, expired ingress request or local error never authorizes another
submission. No statement, digest or observation time is regenerated. Exit zero can
mean `pending`; inspect the outcome before treating it as acceptance.
