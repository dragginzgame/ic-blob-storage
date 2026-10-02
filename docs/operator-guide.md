# Operator and verifier guide

Use the native `blob-storage` client to inspect a service, submit tenant reference
operations, check content and submit an explicitly trusted verifier's statement. Run examples from the
repository root. Replace environment variables and input files with the exact
installation scope and original saved requests.

| Task | Command | Effect |
| --- | --- | --- |
| Check a complete proposed installation | `installation-check` | Offline shared validation, exact configuration and complete init bytes with hashes |
| Prepare an explicit Cashier account link | `account-link-inputs` | Offline Candid and summary; no signature, submission or funding |
| Save verified upload bytes and service requests | `upload-inputs` | Offline installation consistency/root verification and fresh private files |
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

The standalone and managed fixture expose the same maintained blob method types;
see [current status](status/current.md) before selecting a target. Service
observations preserve local facts and fences; they do not grant retry or payment
authority. Provider reads require a selected installation, approved origin and
budget under the [probe ledger](evidence/caffeine-probes/README.md).
Local signed managed evidence now covers `status`, `funding-history`,
`upload-history`, `certificate-assessment` and `verify-upload`, including exact
saved permissions, trust refusal and passive fenced restore. These same maintained
commands also have standalone evidence. File bytes in that managed case are local
substitutes. Managed `observe-upload`, `submit-attestation` and
`upload-attestation` now also have actual signed local evidence with one source
GET and one update, including dropped/pending reply recovery without resend.
Those bytes/exposure remain substitutes; deployed provider and complete consumer
acceptance are separate work. See [inspection evidence](evidence/core-primitives.md#managed-signed-client-and-local-byte-verification--2026-09-30)
and [verifier evidence](evidence/core-primitives.md#managed-signed-verifier-observation-submission-and-recovery--2026-09-30).
Managed tenant reference submission/receipt/status now also run with a distinct
signer beside verifier completion. Lost/pending acknowledgments recover without
resend; cleanup at capacity during suspension preserves physical/billing liabilities,
and fenced restore preserves historical results without reviving references.
See [tenant reference evidence](evidence/core-primitives.md#managed-signed-tenant-reference-submission-and-cleanup--2026-09-30).

## Check installation inputs offline

`installation-check` checks the complete shared installation candidate,
including service/operator/payer bindings, portable quotas, reference cleanup
capacity, funding reserves, read limits, project, trusted verifier and trusted
certificate uploader. It emits the complete shared `ServiceInstallationInput`
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
  --trusted-uploader "$UPLOADER_PRINCIPAL" \
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
`ServiceInstallationInput { configuration, project, completion_verifier, trusted_uploader }`;
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
| Freeze local inputs | Run `installation-check`, prepare `account-link-inputs`, bind exact Wasm/Candid/release hashes and choose one known nonempty file of at most 1 KiB | Original configuration/terms/body retained; finalize and recheck actual service principal after authorized creation, before installation; no fixture defaults |
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
principal explicitly sharing payer/tenant/trusted-uploader roles and a distinct
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
| Prepare ledger funding | If the route is established, propose one bounded donor-to-payer ledger transfer followed by one payer-to-Cashier-subaccount transfer | Exact source/destination/amount/fee/memo/created_at_time and retained transaction identity; include all transfer/sweep fees within the initial provider allocation. No standing approval or automatic refill |
| Notify and inspect | Separately invoke `cycles_ledger_deposit_notify_v1` with explicit isolated account after the recorded transfer | Raw credited amount, balance and returned ledger block retained; `NothingToDeposit`, `SweepFailed` or missing replies do not justify another transfer. This mutation's reconciliation semantics remain unqualified |
| Set and link account | After account existence/authority is evidenced, explicitly review zero overdraft/no target auto-refill and prepare `account-link-inputs` for the actual owner | Raw positive daily limit and absolute expiry with evidenced units; authenticated caller is the payer or an evidenced delegate. A daily limit is not a guaranteed total bill cap |
| Inspect service/provider scope | Maintained scoped relationship/balance inspection and gateway sync, followed by exact configuration readback | Actual paid-canister/payer/raw terms match; accepted project/bucket and gateway scope recorded before the one transfer |

This is a proposed route, not a claim that notification creates an account or that
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

The 100T total planning ceiling retains the proposed 10T service / 1T initial
provider / 89T unallocated split. Actual funding amount is still unset pending
fee/credit observations; no refill is authorized. Select a monitoring window and
continuing cleanup owner before upload. Reserve-only local attachment policy does
not cap provider spending, terminate billing or establish old-backup activation.
Raw expiry units and project acceptance remain open. Retain every intent/outcome
before advancing and review the exact next effect separately; never run an upstream
one-shot setup script. See the [preparation evidence](evidence/caffeine-probes/local/2026-10-02-trial-provisioning-01/summary.json).

## Generate account-link inputs offline

There is no existing trial installation or funded account selected. This command
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
  "schema": 1,
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

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin blob-storage -- \
  upload-inputs --installation reviewed-installation/installation.candid \
  --binding binding.json --manifest manifest.json --body source.bin \
  --max-bytes 10485760 --run-dir new-upload-inputs
```

Use complete `installation.candid` from the reviewed `installation-check` output,
not its configuration-only file. The command decodes one bounded exact current
`ServiceInstallationInput`, applies the shared candidate validator and requires
the original service, local namespace, project and trusted uploader to match.
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
fees are separate. Every output sets `retry_authorized: false` and
`provider_credit: not_established`. Empty, uncertain and fenced observations
exit 0; authenticated service refusals exit 3. The bounded decoder checks exact
intent and transport/reconciliation consistency within 4 KiB, with the same
30-second signed-query and 256 KiB HTTP limits as status. No payment, provider
query, polling, journal write or automatic retry occurs.

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
and every blocker. Restricted issuance reports `trial_bounds`, `namespace_binding`,
`trusted_uploader`, `current_owner`, `durability` or `stale_observation` when a local
prerequisite fails. These are not provider charge/provisioning guarantees. Broader
installations remain usable for local service workflows but fail `trial_bounds`.

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
with the accepted restricted trusted-uploader/current-owner contract; see
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
