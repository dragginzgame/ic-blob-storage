# First standalone Caffeine trial

The maintainer **accepted this restricted contract on 2026-10-02**. The implementation
replaces the 0.5.0 gate semantics and requires a minor release. Acceptance does not
authorize deployment, funding or provider traffic. The objective
is one actual upload, an independent verified fetch, and a tenant download.
The [existing run sequence](operator-guide.md#isolated-uploaddownload-trial-plan)
and [probe ledger](evidence/caffeine-probes/README.md) own command/evidence handling.

## Local issuance prerequisites and provider limitations

| Blocker | What we have | What is still needed |
| --- | --- | --- |
| Trial bounds | Installed lifetime limits enforce one tenant/object/reference and at most 1 KiB | Select this envelope for the actual installation; broader configurations refuse certificate issuance |
| Uploader trust | Required immutable `trusted_uploader`, scoped to service and local namespace | Select a trusted participant and exact tenant-approved permission; tenant approval alone cannot grant this trust |
| Local namespace | Original service owner/root and explicit installed project/local namespace | Select and provision the actual payer/project/bucket relationship; local matching does not prove provider provisioning |
| Current owner/durability | One synchronous exposure commit/reply; stop/start and inspection-only upgrade restoration | Keep the original owner active without snapshot loading; old-backup activation is unsupported and can restore lost authority |

The current local blockers are `TrialBounds`, `NamespaceBinding`, `TrustedUploader`,
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

## Accepted restricted prototype contract

These assumptions apply together:

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

This replaces the maintained issuance contract coherently in a minor release,
with corresponding rejection/interruption evidence. Pre-1.0 remains a hard cut: no parallel trial mode,
old/new gate generations, compatibility path or manually supplied true flags.

## Inputs to finalize

The [configuration envelope](../canisters/standalone/trial/README.md) and
[provisioning sequence](operator-guide.md#prepare-isolated-trial-provisioning)
now have a concrete private proposal in `.tmp/trial-provisioning-01/proposal.json`:
canic-mainnet is the deployment/operator candidate, with fresh repository-local
payer/tenant/uploader and separate verifier candidates, a fresh proposed project/
bucket and dedicated browser history. No live roles, account, service or provider
namespace are selected by preparation. The actual service and raw expiry remain
unset. Shared validation and independent decoding pass with a labelled local
service stand-in; those installation bytes are never live inputs.

| Input | Current state |
| --- | --- |
| Host | Standalone proposed; no live instance created or selected |
| Service, controller/deployer, operator, tenant, uploader and verifier | Local `canic-mainnet` identity exists as a signer candidate; no live role assignment or service selection. Syntax-test principals are not live targets |
| Cashier, payer and account controller | Unselected; retained Cashier is a candidate, not an approved account binding |
| Gateway origin, project and bucket | Unselected; must match provisioning and both download paths |
| Total trial budget | Maintainer selected at most 100T cycles (100,000,000,000,000); planning ceiling, not a proven maximum bill |
| Proposed allocation | At most 10T for service/IC execution, 1T for initial provider funding, 89T held unallocated; proposal, not a price estimate or target spend |
| Monitoring/retention window | Unselected; must cover continuing obligations rather than just transfer time |
| Relationship limit/expiry, overdraft and gateway credit handling | Raw provider terms and enforcement unqualified |
| Browser identity and persistent intent store | Maintained IndexedDB journal is locally tested; select its actual profile/origin/database with one lifetime slot. Reopen explicitly; missing history stops the trial. Setup-only in-memory store cannot dispatch |
| Cleanup/reconciliation owner and continuing storage disposition | Unselected; logical release alone cannot close the trial |

Treat gross cycle allocations and outstanding charge exposure conservatively;
provider funding is not proof of either credit or final cost. Do not count a
refund or remaining provider balance as released budget without exact evidence.
The unallocated 89T is not automatic refill authority. The 100T planning decision
does not select an account, establish price/expiry units or authorize the separate
effects described below. Contract acceptance is separately recorded above.

Read-only `icp identity list` confirms `canic-mainnet` has public principal
`o5trf-oqyg7-cawjp-xs4pw-aomb3-iwki5-hyezf-qahfz-j3ffd-jh4fc-oqe`.
The default remains `toko-miner-local`. No key was exported, identity changed,
signing capability tested or live account selected. Naming this identity does not
assign it to operator, tenant, uploader or verifier roles.

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
