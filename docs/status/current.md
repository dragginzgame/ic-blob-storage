# Current status

Date: 2026-10-02

Released baseline: [0.5.0](../../CHANGELOG.md), dated 2026-10-02, at
`03715ebecc608f6f679ca0e753fc95a8fd5eae7b`. Package version and release receipt
are 0.5.0. Completed work is in the maintainer-selected undated 0.6.0
changelog draft, with Unreleased empty. Required trusted-uploader installation,
shared init DTO, upload-input and browser contract changes are breaking: under
[version policy](../governance/development.md#versions-and-changelog), this batch
uses the selected minor release. Draft naming does not bump package/receipt
versions; release preparation remains a separate step.
Commits and release operations belong to the maintainer.

Canic integration is deferred by the maintainer on 2026-10-02 until useful work
within this repository is exhausted. Continue local core/standalone/native/browser
implementation and evidence; the [feedback list](../canic-parity.md#integration-feedback)
is a future consumer backlog, not a prerequisite for this local work. Siblings
remain read-only and no upstream message is authorized.

## Current trial preparation and next step — 2026-10-02

The maintainer authorizes configuration/account-provisioning preparation only.
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
or build cleanup occurs. Work remains in the current 0.6.0 draft.

## Active work — accepted restricted upload contract

On 2026-10-02 the maintainer explicitly accepts the
[restricted contract](../standalone-trial.md): one trusted uploader, one fresh
storage owner/tenant/object/reference and a nonempty file of at most 1 KiB,
without provider spending-cap/replay guarantees or operational old-backup recovery.
This is a breaking semantic/init/API change requiring a minor release; the
package remains 0.5.0. The maintainer selected 0.6.0 for the changelog draft;
the named draft retains the breaking notes and meets the minor-version requirement.

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
