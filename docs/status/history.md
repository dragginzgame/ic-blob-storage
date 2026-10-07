<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Implementation history

This is retained implementation and evidence handoff history. Release,
dependency, validation and next-step statements below describe their original
batch; they are not current operating instructions. Start with
[the current handoff](current.md) for maintained behavior and remaining work.

## How to use this history

- Search for the release number, date or feature named by the evidence you are
  reviewing.
- Treat each section's commands, dependency graph and conclusions as scoped to
  its recorded source revision.
- Do not use an earlier “next step” as current direction; return to
  [current status](current.md).
- Use the [probe ledger](../evidence/caffeine-probes/README.md) for retained
  provider observations and artifacts.

Date: 2026-10-03

Repository baseline: [0.10.0](../../CHANGELOG.md), dated 2026-10-03, at
`f856a2b138511e695d234c59bb5d9fafcccef794`.
The release receipt binds validated source `317b8a3af006b76b15a8a054438b02cdabb7312f`.
No new registry-publication or service-deployment observation is inferred.
The undated [0.11.0 changelog draft](../../CHANGELOG.md) collects the completed
simplification, verifier composition, signer/bootstrap and Chromium bridge batches
below. Unreleased is empty; Cargo versions and the release receipt remain at
0.10.0. The next release is minor because native bindings now require original
preparation hints. Changelog structure and historical notes are checked for this
documentation-only pass; no new full CI/release validation or release action occurs.
The latest handoff is the [native Chromium bridge](#native-chromium-process-bridge--2026-10-03);
earlier next-step notes describe their original batch and are superseded there.
Earlier sections retain
implementation and evidence history now included through 0.10.0; their dependency
identities and validation scope describe the original runs.
The old isolated owner remains frozen at
0.6.0 with its original stopped history; the separate live owner was last verified
at 0.7.0. Both retain their original obligations and exhausted capacity.
Commits and release operations belong to the maintainer.

Canic integration is deferred by the maintainer on 2026-10-02 until useful work
within this repository is exhausted. Continue local core/standalone/native/browser
implementation and evidence; the [feedback list](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback)
is a future consumer backlog, not a prerequisite for this local work. Siblings
remain read-only and no upstream message is authorized.

## Unreleased: confirmed audit simplifications — 2026-10-03

Download serving checks active tenant authority and retained content once, while
asynchronous reads retain their captured-generation recheck. Browser certificate,
journal, gateway and worker boundaries share pure binding/budget predicates;
role checks, snapshots and durable effect claims remain with their owners.
Each frozen native file carries its raw digest. Native setup, preparation and
completion decisions use typed observations rather than reading presentation
JSON. Public/wire/stable schemas and journal formats remain unchanged.

Focused checks pass: 89 native unit cases, 14 core read cases, 15 standalone
publication cases and all six selected download cases. All thirteen Chromium
journeys pass; four indexed recovery cases and a worker lost-reply journey pass
again after preserving original-identity refusal ordering. Offline worker/frozen
input and actual IndexedDB checks pass, as do strict core/CLI lint and scoped
Wasm/native builds. These are targeted checks, not full CI or release validation.

The [local record](../evidence/caffeine-probes/local/2026-10-03-simplification-01/summary.json)
and [ledger](../evidence/caffeine-probes/README.md#implementation-simplification-regressions--2026-10-03)
retain intentions, source/artifact/log hashes, original journals/profiles and
failed attempts under `.tmp/simplification-01`. Sandbox listener refusals, missing
test capture/path setup, the lint limit, a maximum-width test overflow and a
capture-helper child-process failure are retained before corrections. No live
provider call, paid effect, deployment, version mutation, commit, publication,
sibling edit or build cleanup occurs. Existing live obligations remain intact.

Next product work remains durable native parent phase/restart coordination and complete
Miner acceptance. The [Canic feedback list](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback)
remains deferred consumer work; no sibling changes are included.

### Native follow-up — 2026-10-03

Attestation, certificate assessment, history and local verification now use the
existing authoritative upload-identity formatter in `native::references`. Local
verification still checks the computed provider root against the original root
before presenting that identity. Setup, local verification, provider observation
and saved observation validation for attestation share one native manifest reply
envelope. Caller-selected content bounds, distinct actor checks, decoder work
limits, error mapping and journal/JSON formats remain unchanged. Observer and
submission provenance includes the shared helper sources. Receipt inspection
uses its already validated request loader without repeating scope/tenant checks.

Strict native lint, 89 native unit cases and nine selected PocketIC/browser cases
pass. These include 10 MiB local verification, paged/restored history, certificate
blockers, lost/pending setup and reference replies, verified browser download and
lost-chunk recovery without replay. The [local record](../evidence/caffeine-probes/local/2026-10-03-native-cleanup-01/summary.json)
retains the intent, exact native binary, hashes, browser/reference requests and
profiles under `.tmp/native-cleanup-01`; disposable fixture limitations are explicit.
No deployed provider call or paid effect occurs. Full CI/release validation and
complete Miner qualification remain outstanding; Canic adoption stays deferred.

Review retained the canister read-session owner: callback correlation, occupied
buffer capacity, post-await authority checks and same-release fencing differ from
direct tenant descriptors. The derived public descriptor target is still a current
API convenience; no consumer evidence establishes that removing it is appropriate.
No compatibility branch, framework dependency, new state owner or release change
is introduced. Continue with native parent/verifier automation once this coherent
simplification batch is reviewed; keep the consumer feedback actions above open.

### Native session verifier composition — 2026-10-03

Optional `--verifier-identity` authenticates the installed completion verifier
before session readiness. The `verify` frame composes the maintained bounded
whole-download observer and exact one-shot attestation owner, then checks the
current tenant/reference through the same typed publication observation as
`status`. Original permission, full-width identity, frozen raw digest and configured
gateway must match before signing. One private create-new directory per original
index retains phase intent and the original observer/submission artifacts.

`source_observation` recovery validates the original complete observation and
queries immutable history without another GET or submission. Exact statement
correlation is typed; absence or conflict stops before advancement even if another
observation has the same content digest. Current restore/enrollment/reference
checks remain separate. Missing/wrong verifier, foreign observations, corruption,
pending/uncertain claims and deadlines never authorize replay. Existing manual
verifier tools remain usable with their original one-shot claim ownership.

Strict CLI/integration lint, 89 native unit cases, five native session cases and
three final Chromium journeys pass. Success and lost-reply recovery each retain
3,072 physical/liability bytes; corrupt observation stops before attestation and
the next file while retaining 1,024. Recovery rejects another upload and a distinct
observation time despite identical verified bytes and an already live reference.
The [local record](../evidence/caffeine-probes/local/2026-10-03-native-verifier-01/summary.json)
and [ledger](../evidence/caffeine-probes/README.md#native-session-verifier-composition--2026-10-03)
retain original phase/observer/submission/profile captures, binaries, hashes and
failed lint attempts under `.tmp/native-verifier-01`. No deployed provider call,
paid effect, live-owner change or full CI/release validation occurs.

Native parent browser launch, selected signer/private-port bootstrap, bounded body
loading with original metadata hints and restart coordination remain unfinished.
Continue those around this session and the maintained worker; add no second
certificate/provider dispatcher. Then qualify all 679 Miner identities / 675
blobs, real media, overlapping references and MIME/CORS/cache/CSP/open-browser
retention. Canic consumer adoption remains deferred, with open actions in the
[feedback list](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback). No sibling edits or
upstream messages are included.

### Maintained browser host and signer bootstrap — 2026-10-03

The private browser package now owns `createPublicationWorkerHost`,
`servePublicationBootstrap` and the explicit DedicatedWorker entry. The host
launches a named same-origin module, transfers one private port and sends an
explicitly selected Ed25519/secp256k1 SDK identity plus fixed configuration on
that port only. The SDK recomputes the advertised key pair; the configured uploader,
scope, root and budgets validate before opening the existing IndexedDB journal.
Create/open mode remains explicit; there is no identity discovery, credential
HTTP route, second dispatch owner or replacement-journal fallback.

The fixture-only bootstrap is deleted. Serial Chromium consumers use the
maintained host/entry; only test identities and private evidence projection remain
in the fixture. Certificate call counts come from actual browser requests rather
than diagnostic worker events. Direct and hosted jobs share the same configuration
and snapshot owner. Selected body/root views copy only their bounded bytes,
independent of larger backing buffers. The host correlates one outstanding job
and terminates the worker on close, abort or deadline without erasing history or
declaring an exposed effect stopped.

Pinned SDK/source/patch builds, 31 offline worker cases, 15 actual Chromium/
IndexedDB bootstrap cases and three native-verifier Chromium/PocketIC journeys
pass. Cases include both signer kinds, malformed pairs/configuration refusing
before journal creation, snapshots, concurrency, foreign assets, pre-abort,
outstanding close, deadline/profile reopening, exact attestation recovery and
corruption preserving physical/liability accounting. No deployed provider request,
paid effect, live-owner change or full CI/release validation occurs. The
[local record](../evidence/caffeine-probes/local/2026-10-03-worker-bootstrap-01/summary.json)
and [ledger](../evidence/caffeine-probes/README.md#maintained-browser-host-and-signer-bootstrap--2026-10-03)
retain original profiles, phase/observer/submission captures, tested bundles,
source/artifact/log hashes and all bootstrap attempts under `.tmp/worker-bootstrap-01`.
`make test-browser-bootstrap` provides the scoped opt-in check with an explicit
new `BLOB_BOOTSTRAP_REPORT`; its recipe was reviewed by dry run.

Next implement native OS/browser-process launch and durable parent coordination
around this host and the session. Select the signer input explicitly; the browser
contract accepts maintained SDK identity JSON, and does not require inventing a
PEM parser. Persist original preparation hints and bound selected body loading;
do not derive filenames from provider headers or treat redacted browser success
as completion. Parent restart must retain original profiles/setup/verification
claims and inspect uncertainty without replay. Full Miner media/reference/serving
acceptance remains open. Canic adoption stays deferred with actions in the
[consumer feedback list](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback); no sibling work
or upstream message is included.

### Native Chromium process bridge — 2026-10-03

Unreleased adds `launchPublicationBrowser` around the maintained host/worker.
The caller selects the installed Playwright Chromium engine, trusted bundles,
SDK signer/bootstrap, original profile and fixed loopback port. Create/open mode
is explicit; port contention refuses before profile creation, and missing history
never creates a replacement journal. Only selected regular body bytes are read,
bounded by exact length/EOF/raw digest, then transferred in 64 KiB CDP values.
The browser still recomputes the SDK manifest/root before certificate intent.
One job runs at a time; close/deadline/control failure retains original history
and uncertainty. The asset server serves no keys, configuration or bodies.

Native v1 bindings now require `preparation` with original optional `content_type`
and `filename` strings. Preserve omission and empty strings; reject null/unknown
fields and hints beyond 4,096 UTF-8 bytes. Binding hashes retain these arguments
and selected session transfer descriptors carry them unchanged. This input hard
cut requires a minor release; no compatibility reader or schema-generation branch
is added. Service wire/state, Wasm, dependencies and existing obligations are unchanged.

Ninety native cases, eleven actual Chromium bridge/profile/body cases, five native
PocketIC sessions and three native-verifier Chromium/PocketIC journeys pass.
The real adapted SDK/native 10 MiB snapshot round trip passes; strict CLI/harness
lint, syntax, formatting and whitespace checks pass. Lost-reply recovery performs
no additional upload/GET/attestation; corruption stops before attestation/next file.
Success/recovery retain 3,072 physical/liability bytes; corruption retains 1,024.
The [local record](../evidence/caffeine-probes/local/2026-10-03-native-browser-01/summary.json)
and [ledger](../evidence/caffeine-probes/README.md#native-chromium-process-bridge--2026-10-03)
retain exact binaries/bundles, profiles, requests/replies, hashes and failed attempts
under `.tmp/native-browser-01`. Earlier runs selected zero cases, hit sandbox socket
restrictions or exposed SDK/bundle packaging and lint issues; none is omitted.
No deployed provider call, paid effect, full CI/release, cleanup or sibling edit occurs.

Next implement the durable native parent around this bridge/session: persist the
selected signer/profile/origin and original setup/verification origins before
effects; on parent restart inspect exact retained evidence before advancing, without
replaying an uncertain transfer. This callable bridge is not a complete headless
publisher. Full Miner real media/reference/MIME/CORS/cache/CSP/retention acceptance,
large-body heap/CDP latency and hostile local-user/profile rollback isolation remain
unqualified. The open [consumer actions](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback)
include adopting original hints and retained process history; Canic remains deferred,
with no upstream message sent.

## Released 0.10.0: private-port browser publication worker — 2026-10-03

Add `createPublicationWorker` / `servePublicationWorker` in the private browser
package. Bind uploader signer, service/tenant/project/bucket, IC origin/root,
reviewed gateway and body/request/job/deadline limits before jobs. Accept upload,
inspection, historical certificate recovery and cancellation on a trusted private
MessagePort, including in an actual DedicatedWorker. Reuse the maintained SDK
and strict IndexedDB certificate/gateway owners; add no provider wire decoder,
retry journal or service workflow. Snapshot caller-owned input before awaits.
Concurrent jobs refuse without unlocking another active job; job correlation
and budgets are local execution guards, not persistent effect identities.

Return bounded redacted states and journal phase/status projections, omitting
envelopes, certificates, permissions, request URLs/headers and provider error
messages. Forward only a finite error-code vocabulary; exported error classes
cannot smuggle arbitrary code strings into replies. Validate the projected
phase/status types and request ceiling, including for custom stores.
Every result has `service_completion_checked:false` and
`retry_authorized:false`; only native observation/attestation/reference checks
can authorize advancement. Inspection of absent history never saves a row;
recovery/cancellation require exact original history. Claimed/cancelled uploads
refuse before another transfer. A cooperative deadline aborts transport and
fences new jobs; the host still owns process termination and profile-loss fencing.

Offline boundary cases pass for foreign scope, malformed jobs, content changes,
concurrency, snapshots, budgets/idle deadlines, missing/claimed history, configured
roles/origins and credential redaction. These use a substituted store, not a fake
platform in production. Three actual DedicatedWorker/PocketIC/native journeys
pass at one active reservation. Native/browser restart reconciles a lost reply
without another upload; original exposed setup recovery has zero updates.
Cancellation survives reopening. Corrupt observation blocks the next transfer
and complete map while retaining 1,024 exposed physical/liability bytes; complete
cases retain 3,072. Public-serving/real-media and deployed-provider guarantees
do not follow from these synthetic image-labelled local gateway trials.

All thirteen affected Chromium journeys pass, including existing persistent,
indexed and single-file upload/download/recovery regressions. Strict integration
lint, pinned SDK/source/patch browser builds, formatting/whitespace and public/private
capture checksums pass. Core, host Wasm, native CLI and dependencies are unchanged
this batch; preserve concurrent ic-memory 0.15.6 selection and dirty work. The
[probe record](../evidence/caffeine-probes/local/2026-10-03-publication-worker-01/summary.json)
and [ledger](../evidence/caffeine-probes/README.md#private-port-browser-worker--2026-10-03)
retain intentions, original profiles/claims/replies, opaque gateway fingerprints,
redacted worker results and source/artifact/log hashes under
`.tmp/publication-worker-01`. Intermediate captures used the misleading fixed
`file_live:false` wording; final captures use `service_completion_checked:false`.
Final offline cases refuse exported errors carrying private code strings and
corrupt/oversized projections; three final worker regressions pass after those
guards. The rejected documentation patch is recorded before corrected propagation.
No live/paid provider call, deployment, version/release/commit/publication, full
CI, sibling edit or build cleanup occurs. Live exhausted owners and obligations
remain intact.

Next implement the native parent launcher and durable parent phase intent around
these components: selected signer/bootstrap/private port, bounded body loading
with original metadata hints, independent observation/attestation and exact
per-file journals. Do not claim a complete noninteractive publisher from the
library worker alone. Then qualify all 679 Miner identities / 675 blobs, real
media, overlapping references and MIME/CORS/cache/CSP/open-browser retention.
The [consumer backlog](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) records adoption
and acceptance actions; Canic remains deferred. No sibling changes or upstream
messages are authorized.

## 0.10.0 preparation history — 2026-10-03

The following records the pre-release state; the baseline above is authoritative.

Release-lint repair: move `BrowserPublicationPlan` from the shared browser driver
to its sole consumer, the standalone serial test module. The storage test target
no longer compiles an unused plan type; no lint suppression or behavior change
is needed. Strict offline/locked Clippy passes for all targets/features of the
PocketIC test package with the current dependency selections. Targeted formatting
and whitespace checks pass; this does not constitute full release validation.

The undated [0.10.0 changelog](../../CHANGELOG.md) collects the completed
embedding helpers, authenticated publication map/status checks, persistent native
sessions, private-port browser worker and nonblocking native-input fix.
Unreleased is empty; package versions and the release receipt remain at 0.9.0.
The maintainer owns commits and the release flow.

Concurrent dependency updates select `ic-memory` 0.15.7 and `ic-testkit` 0.11.0
in the lockfile. Preserve these updates. The earlier recorded validations retain
their original dependency identities; they do not validate this newer selection.
This changelog-only pass checks release-note structure and whitespace, without
compilation or full CI. Next work remains native parent/verifier automation and
complete Miner acceptance; Canic adoption remains deferred with open actions in
the [feedback list](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback).

## Unreleased: persistent native publication phases — 2026-10-03

Add `publish-session` as the next headless-publisher component. Keep one fully
validated frozen batch and authenticated tenant/operator agents across bounded
setup/status/map frames. Authenticate uploader and actual installed configuration/
release before ready. Rehash only selected bytes before setup; return cached
original binding/manifest/digest for browser snapshot verification. Reuse the
maintained indexed setup and map owners; add no provider contract, certificate
dispatcher or retry journal. Advance only on exact configured-verifier completion
and current unfenced first reference, then inspect all references before the map.

Persist bounded original intent/control requests before effects. Pin each original
setup directory, recover exact signed claims after process interruption, and
refuse changed journals or already-exposed preparation without redispatch. Status
recovery can inspect confirmed original indices again in a new session. Frames
are at most 8 KiB, the queue holds one, and the explicit step/session deadline
bounds waits. An idle stdin reader cannot hold runtime shutdown past the tested
deadline. Synchronous startup hashing/filesystem/stdout I/O is not preemptively
bounded by that deadline. Maps describe initially validated intent, not fresh
verification of every local copy or an atomic publication/retention lease.

Four native PocketIC session cases and all six affected serial Chromium journeys
pass. Native process/browser restart after a lost chunk response preserves the
original signed setup and strict IndexedDB histories. Exposed setup recovery
performs zero updates; independent complete observation and attestation allow
progress without another PUT. Corruption blocks the next upload and complete map.
Completed cases retain 3,072 physical/liability bytes; corruption retains 1,024
exposed bytes. Native synthetic receipts qualify local service logic only; the
browser cases independently observe an owned gateway substitute.

Targeted native CLI and fifteen affected setup/map/capacity/recovery PocketIC
checks pass, as do strict CLI/integration lint, current CLI/pinned browser builds,
formatting and whitespace. The host Wasm is unchanged. Public and private
retained-evidence checksum checks pass. Retain exact requests, replies, browser
profiles, hashes and failures in `.tmp/publish-session-01`; the
[probe record](../evidence/caffeine-probes/local/2026-10-03-publish-session-01/summary.json)
and [ledger](../evidence/caffeine-probes/README.md#persistent-native-publication-phases--2026-10-03)
separate this local evidence from deployed behavior. The first disposable native
captures lacked retained request journals; their logs survive and the final run
retains originals. Corrected decoder/lint/capture-import failures are recorded.
Preserve concurrent ic-memory 0.15.6 selection and other dirty work. No full CI,
paid/live provider effect, deployment, version/release/commit/publication, sibling
edit or build cleanup occurs. Live owner obligations remain untouched.

Next build the production browser worker and native parent orchestration around
these components, coordinating independent observation/attestation and exact
original journals without automatic upload retries. Then qualify all 679 Miner
asset identities / 675 blobs, real media, overlapping references and public
MIME/CORS/cache/CSP/open-browser retention in the consumer transaction. The
[integration feedback](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) records adoption
and those open acceptance actions. Canic remains deferred; the complete
production headless publisher and Miner readiness are not yet claimed.

## Unreleased: serial completion checks and two-file journeys — 2026-10-03

Add `publish-file-status` as an indexed observation using the existing map workflow,
installed configuration/release authentication, tenant enrollment/capacity,
configured-verifier receipt/digest and exact unfenced first-reference checks.
Require the original canonical index and four query slots before output. Preserve
query arguments/replies, original IDs/digest and blockers; require `file_live:true`
before advancing a serial consumer. It sets `batch_complete:false` and never
emits a complete media map. No provider request, update or retry owner is added.
Full map semantics and existing production origin restrictions remain intact.

Qualify actual standalone/native/Chromium composition with two distinct roots
(1,024 and 2,048 synthetic bytes) at one active reservation. The second setup is
blocked while the first is exposed, even after SDK success. Independent bounded
observation and exact configured attestation free concurrency; indexed status
then allows progress and the final complete map joins both original files.
Whole-browser restart reopens the same strict IndexedDB rows; a deliberately lost
first chunk reply remains uncertain while independent reconciliation confirms the
object without another PUT. Corrupt first observation blocks the next transfer
and any complete map. Confirmation preserves physical/liability accounting:
3,072 bytes for complete/lost cases and 1,024 still exposed for corrupt refusal.
These image-labelled synthetic bodies do not qualify image decoding or serving.

Use one shared owned HTTPS HTTP/2 gateway fixture for maintained single-file and
serial journeys; retain bounded requests, hashes, native signed journals, raw
query/observation replies and browser profiles privately. The
[probe record](../evidence/caffeine-probes/local/2026-10-03-serial-publication-01/summary.json)
and [ledger](../evidence/caffeine-probes/README.md#serial-completion-and-browser-restart--2026-10-03)
separate local substitution from deployed evidence. Failed captures survive in
`.tmp/serial-publication-01`: sandbox sockets, the PocketIC already-live localhost
URL, ambient Node 18, strict test-helper lint and its first extraction compile
failure. Corrected runs use literal loopback, provisioned Node 24 and fresh browser
captures; no production policy or lint suppression is weakened.

Targeted validation passes: native CLI cases, actual indexed/full-map PocketIC
checks and all seven affected browser journeys (three serial and four maintained
single-file success/corrupt/lost/withdrawn cases). Current CLI build, pinned SDK
source/patch browser build, strict CLI/integration lint, formatting and whitespace
checks pass. The prior current host Wasm is unchanged; the concurrent registry
ic-memory 0.15.6 selection is preserved. No full CI/release, live provider/paid
effect, deployment, version/commit/publication, sibling edit or build cleanup.

Next implement the persistent headless coordinator around the existing phase
owners and original per-file journals. Independent CLI invocations still reverify
the full batch; do not assemble 675-file production publication as repeated full
body scans. Retain one validated batch in the coordinator, revalidate the selected
body before transfer and recover only exact original claims/receipts. Then qualify
all 679 Miner asset identities / 675 distinct blobs, release overlap, cancellation
and public MIME/CORS/cache/CSP/open-browser retention in the consumer transaction.
The [integration feedback](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) records released
adoption still pending. Canic remains deferred; live exhausted owners and all
original physical/billing obligations remain untouched.

## Unreleased: upstream fixes and complete reference maps — 2026-10-03

Address the current native-file defect with nonblocking Unix descriptor opening
followed by same-descriptor regular-file validation. Reuse that helper for bounded
metadata reads and streaming body snapshots. Preserve byte bounds, redacted typed
failures and source/snapshot verification; no pathname-only precheck is trusted.
A FIFO-with-no-writer subprocess promptly returns the file refusal before
transport setup; regular, empty, oversize and directory cases remain covered.

Address GitHub issue #1 locally with passive
`ops::service::installation::requests(authority)` and `LIBRARY_VERSION`.
The complete seventeen-request inventory includes configuration and the existing
store mapping without registration, ID selection, bootstrap or another runtime.
Standalone uses both helpers. Store-only assemblies retain their authoritative
sixteen-key helper; that is a maintained composition boundary, not an alias.
Installation tests cover unique complete keys, supplied authority, refusal and
reopen. Actual PocketIC readback verifies the compiled dependency identity
independently of the test consumer's package version and rejects wrong-service
installation without changing the prior state. The GitHub issue remains open;
no upstream message or release/adoption claim is made.

Add `publish-map` as the next publisher component. Reverify the complete frozen
batch once, authenticate tenant/operator independently and compare actual installed
configuration, project, verifier, uploader and compiled release. Then observe active
tenant enrollment and unfenced exact verifier receipts/current first references.
Every accepted digest must equal that frozen body's raw SHA-256. Persist bounded
query intents, arguments and raw replies; write `media-map.json` only when all files
pass. Retain original indices/IDs, headers, digests and shared Caffeine targets.
Blocked/failed/partial runs cannot produce a complete map. These sequential facts
are not an atomic snapshot, publication lease or serving/retention guarantee.

Targeted validation passes: 87 native CLI cases plus the FIFO subprocess, eight
installation and four composed-grant cases, two actual standalone PocketIC map
cases and the existing installation carrier. The map cases cover incomplete then
complete two-file state, released references, restore fences, wrong operator,
insufficient query budget and corrupt later bytes without premature output.
Locally trusted synthetic attestations exercise the service; they do not observe
a provider. Fresh host Wasm/CLI builds, strict affected core/CLI/host and standalone
integration lint, strict core Rustdoc, formatting and whitespace checks pass with the concurrent
ic-memory 0.15.6 selection preserved. Logs in `.tmp/upstream-followthrough-01`
retain first compile/lint failures and the rejected non-loopback local fixture;
corrected runs preserve the production origin restrictions.

Next complete serial transfer/verification/attestation orchestration and exact
interruption recovery, freeing active reservations before the next indexed setup.
Use the complete map within the consumer's asset transaction and qualify all
679 Miner asset identities / 675 distinct blobs, overlapping references and
public MIME/CORS/cache/CSP/open-browser retention. The
[consumer backlog](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) records released adoption
still required; Canic wrapper work remains deferred. No live service/provider
effect, deployment, funding, full CI, version/release/commit, sibling edit or
build cleanup occurs. Existing physical and billing obligations remain intact.

## Released 0.9.0: upstream feedback review — 2026-10-03

Read-only GitHub checks find the same Miner and Canic feedback as their local
files. Returned Git blob identities match local `git hash-object` results:
Miner `5a59d159a6d33503f3a7820b9f1ca31d3cf60c7e`, Canic
`f7a4b3b7301bb6e5ba6442aa692369924f754bef`. Their feedback files are clean;
unrelated Miner gameplay/fleet edits and ic-memory edits remain untouched.
Miner feedback SHA-256 remains
`62b8cede95cfc1dc5bcac8e33eac07c8b20647767ade2aa0875a7eb5dadb1a91`;
Canic feedback SHA-256 is
`53f8ccef8cfd220c53a20bfefb0ba2a9b1febbb27b45751030e003226002b571`.
Miner still selects registry 0.7.0; its BLOB-002/003/004 requirements remain open.
Release 0.9.0 does not establish consumer adoption or complete publication.

Two actionable gaps survive current-source inspection. Canic's
[FIFO report](https://github.com/dragginzgame/canic/blob/main/docs/upstream/ic-blob-storage.md)
still matches native `read`: blocking `File::open` precedes same-descriptor
regular-file validation and transport deadlines. Fix descriptor opening without
a metadata-only race; qualify prompt typed refusal in a subprocess plus maintained
regular/empty/oversize behavior. This review does not rerun the historical FIFO
reproduction. Open [GitHub issue #1](https://github.com/dragginzgame/ic-blob-storage/issues/1)
requests one passive installation inventory of all seventeen memory requests and
a library-owned compiled-version constant. Current store grants supply sixteen;
standalone separately adds configuration and uses its own package version.
These are embedding usability requests, not evidence of an authority bypass.

The local ic-memory release is now 0.15.6 at
`77d228547081904285ee1839e6fdb4ff15610ba3`. Its committed 0.15.5/0.15.6 notes
describe a configured-bootstrap reentrancy fix, bounded decoding and fewer repeated
checksum scans; 0.15.6 also removes an unused error variant. The committed 0.9.0
release uses 0.15.4. During this review another task updates the working-tree
manifest and lock to registry 0.15.6; preserve those edits and leave its dependency
validation to that task. This review makes no independent registry-publication
claim and runs no build. Composition still requires one package identity per host.

Next fix the bounded-file defect and passive embedding helpers together, then
drive serial transfer, independent verification/attestation, exact references and
confirmed media-map output using existing dispatch owners. Interleave completion
to free active reservations; `publish-prepare-batch` alone does not do this.
Qualify actual Miner media, interrupted publication, overlapping releases and
public MIME/CORS/cache/CSP before claiming consumer readiness. The
[feedback list](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) records these actions;
Canic wrapper adoption remains deferred. Refresh current release wording in the
README, roadmap and guides while preserving dated evidence. No provider probe,
paid effect, deployment, build, full CI, sibling edit or upstream message occurs.

## Completed in 0.9.0: code cleanup and one-pass batch setup — 2026-10-03

Work through the maintainer's four accepted audit findings. Add bounded
`publish-prepare-batch`, which opens and verifies the complete frozen batch once
before output or setup effects, then reuses the same indexed service workflow.
Every file retains its own original signed admission/preparation claim under
`file-NNNN/`. Stop on the first blocker, pending result or error; the exact file's
original directory/index remains the source for `publish-prepare-resume`. Retain
at most two updates/four queries per file and one overall setup deadline. There
is no whole-batch replay, provider transfer or publication. Active reservations
still limit setup; this command does not interleave completion to free capacity.
Single-file setup/recovery continues full batch revalidation. Browser transfer
still independently verifies the selected body before certificate/provider work.

Consolidate CLI canonical flags, principal/positive-decimal parsing and authoritative
batch limits without moving role-specific authority into syntax helpers. One
consumed signed-update helper owns durable claims, synced request/envelope/intent,
finite dispatch and bounded saved replies for upload setup, reference mutations,
verifier attestations and gateway decisions. Operation-specific authority, intent
fields and typed reply correlation remain with their commands. Partial claims and
lost/pending outcomes never become retry permission; source fingerprints include
the new shared parsing/dispatch modules.

The heap upload owner is an intentional independent reference model used by
durable accounting/lifecycle comparisons and local primitive fixtures, not dead
service storage. Retain it and its shared pure validators; centralize its repeated
permission-view construction and clarify the absence of persistence/restore
authority. Production handlers continue using the single durable owner. No API
shim, stable schema branch, allocator/dependency change or retired-state reset is
introduced. Extend Unreleased notes and the operator/README recipes.

Targeted validation passes: native CLI suite, heap upload cases, durable upload
store/parity cases, actual PocketIC indexed/batch setup, reference submission,
gateway controls and verifier attestation success/lost/pending regressions.
Fresh standalone Wasm and CLI builds, strict affected core/CLI/integration lint,
formatting and whitespace checks pass. Existing local Cashier/byte-server
substitutes remain labelled; this is transport/service regression evidence, not
a new deployed Caffeine observation. Logs and retained per-file setup journals
are in `.tmp/code-cleanup-w1mwu3`. Preserve the first batch run's one-object
fixture refusals and the restricted native run's loopback-denial logs; corrected
larger-profile and loopback-enabled runs pass without weakening production limits.

The [consumer backlog](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) adds one-pass batch
setup adoption and exact file-journal recovery. Canic adoption remains deferred.
Next useful consumer work remains a complete transfer/completion/reference/media-map
pipeline and consumed-workload serving acceptance. No version mutation, release,
commit, deployment, paid provider call, full CI, sibling edit or build cleanup
occurs. Existing live installations and physical/billing obligations remain intact.

## Unreleased: pre-1.0 hard-cut audit — 2026-10-03

The maintainer requires current-contract-only behavior. Audit maintained Rust
exports, endpoint/init DTOs, CLI flags and JSON journals, stable record readers,
installation release binding and browser stores/exports. No intentional legacy
reader, deprecated/Serde alias, old-payload fallback, migration engine or schema
version-dispatch branch is found. Current records read v1 only; installation
restore requires the exact compiled release. Same-release receipt/restart recovery
is the maintained contract, not cross-release compatibility. Historical evidence,
old live installations and unresolved obligations must still survive retirement.

Remove the redundant public `ic_blob_storage_canister::dto` forwarding namespace
entirely. Host handlers and PocketIC consumers import `HostConfigurationView` and
`HostFailure` directly from `ic_blob_storage::dto::configuration`. Update current
library/host comments and clarify that pre-1.0 cross-release migration is excluded,
not a deferred implementation. Canonical exports from private implementation
modules, the exact ic-memory substrate export and current provider wire naming
remain current API structure; none preserves superseded names or payload forms.

Wait for the other task's validation before source edits; its 0.8.0 commit/tag
and dependency updates are preserved. Keep this new API removal in Unreleased,
leaving completed 0.8.0 notes unchanged. Strict host/standalone-test Clippy, fresh
host Wasm/CLI builds and the existing actual PocketIC installation-carrier case
pass. The latter verifies declared/exported Candid equality, exact core DTO
readback, compiled 0.8.0 identity, wrong-service refusal and controller denial.
Formatting and whitespace checks pass. Audit intent/findings, baseline verification,
local installation/readback and logs are in `.tmp/hard-cut-audit-01`.
No obsolete-form tests, compatibility shim, version mutation, commit, deployment,
provider action, full CI, sibling edit or cleanup is introduced by this audit.
The [consumer backlog](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) records the canonical
import change and released 0.8.0 recovery adoption; Canic remains deferred.

## Completed in 0.8.0: frozen-file transfer and larger journals — 2026-10-03

Browser `createPublicationUpload` now bridges one original native frozen file
to maintained Caffeine SDK preparation, certificate exposure and guarded transfer.
Snapshot body/binding/trusted root before awaits; verify raw SHA-256 and exact
SDK metadata, ordered leaves, provider root and byte length before certificate
intent access. The existing certificate/gateway journals remain dispatch owners.
Reopened claimed/cancelled rows refuse upload; historical certificate recovery
never resumes an uncertain gateway request. Keep complete batch/setup validation
and original identities/journals in the caller. SDK success alone cannot publish.
See the [browser recipe](../../clients/browser/README.md#transfer-a-frozen-publication-file).

The configurable browser lifetime ceiling is now 1,000,000 attempts, independent
of service limits and 4,096-file native batches. Actual Chromium strict IndexedDB
commits preserve 675 synthetic rows, claim/cancellation history and capacity
refusals across a whole-browser restart. A one-row million-capacity journal
reopens with its cancelled row and refuses capacity changes; this does not
qualify a million populated rows. The [sizing review](../operator-guide.md#larger-inventories-and-a-dedicated-storage-owner)
recommends a dedicated metadata/storage owner at that scale, with bytes on Caffeine
and an off-canister publisher. Production indexes are stable B-trees; synchronous
reopen still scans complete retained history and builds temporary validation sets.
Measure stable bytes, peak heap, per-operation and reopen instructions, browser
quota/count latency and full-batch native preparation I/O before claiming support.

All four actual standalone PocketIC/Chromium frozen-batch journeys pass against
the owned HTTPS HTTP/2 gateway substitute: success through independent verification,
attestation, tenant download and logical release; corrupt read refusal; lost-final
reply through stop/start/operator recovery without redispatch; and withdrawn/late
completion preserving release. Offline frozen-input refusal/snapshot tests and
their opt-in Make target, strict affected standalone-test lint, formatting, JS
syntax and whitespace checks pass. The [local record](../evidence/caffeine-probes/local/2026-10-03-publish-transfer-01/summary.json)
and `.tmp/publish-transfer-01` retain exact local requests, snapshots, signed native
setup/recovery journals, browser history and failures. Disk exhaustion temporarily
blocks tools; the maintainer frees space and confirms cleanup finished. A sandbox
loopback refusal and a missing generated Serde file during cleanup remain recorded;
the final test rerun passes without agent cleanup or dependency changes.

Next drive a complete frozen batch through transfer, independent completion,
retained references and a confirmed media map, then qualify actual Miner assets
and public serving. Local gateway results do not qualify deployed provider
retention, deletion, billing or consumer adoption. Canic remains deferred; the
[consumer actions](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) now include frozen-file
adoption and large-inventory operating evidence. No live service/provider effect,
balance refresh, funding, deployment, version/release/commit, full CI, sibling edit
or agent cleanup occurs. Existing live owners, expired nominal links, exhausted
capacity, profiles and all physical/billing obligations remain unchanged. The
prior below-100T balance reminder persists without a new balance observation.

## Unreleased: Miner follow-up and indexed publication setup — 2026-10-03

Read-only Miner review finds local HEAD
`3354dfc6b9fe791884ec69e8dd344de313b36940`; its feedback and scan-log files are
clean. Retained feedback SHA-256 is
`62b8cede95cfc1dc5bcac8e33eac07c8b20647767ade2aa0875a7eb5dadb1a91`.
The recorded consumed workload is 679 asset identities, 675 distinct blobs,
221,173,950 bytes and 763 distinct leaves (767 before deduplication). This review
does not rerun the consumer build. Miner recognises the live sample journeys and
Unreleased batch tooling, while maintaining its registry 0.7.0 preparer and
publication gate. BLOB-002/003/004 remain open: whole-workload publication/recovery,
public MIME/CORS/cache/CSP delivery, release overlap and retired-root reintroduction.
Canic wrapper work remains deferred until its independent blob code's published
removal. Source review is consumer feedback, not provider or adoption evidence.

Native `publish-prepare` fully reverifies the frozen batch and prepares one explicit
zero-based index. Authenticate tenant and uploader independently before claiming
output; retain finite selected-root/capacity observations. Reuse maintained signed
admission/preparation commands and original per-step claims: at most two updates
and four service queries, no certificate or provider traffic. `publish-prepare-resume`
requires the original run, exact inventory/installation/index/scope/transport/root,
request packets and signed admission envelope. It never resubmits a claimed update;
only a previously unclaimed preparation may be sent after observing the original
reserved, unrevoked permission. A partial claim or absence never authorizes retry.
Keep the complete frozen inputs and original journals, including after failure.
Historical `prepared` is not expiry renewal, upload completion or publication.

The [operator recipe](../operator-guide.md#prepare-one-indexed-file-with-surviving-setup-intent)
describes budgets and recovery. The [consumer backlog](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback)
adds exact setup-journal adoption, consumed-workload/serving and the finite
twenty-management-change operating horizon. Recovery never rotates installation
anchoring; plan retirement with surviving obligations before history exhaustion.
Next connect certificate exposure, direct transfer, independent completion,
retained references and confirmed media mapping; then qualify the actual Miner
workload and browser serving. Preserve both existing live owners and exhausted
capacity. No live write, paid/provider request, sibling change, version/release,
commit, full CI or build cleanup occurs in this batch. Validation outcome and
retained local evidence are recorded in the
[completed local record](../evidence/caffeine-probes/local/2026-10-03-publish-prepare-01/summary.json)
and `.tmp/publish-prepare-01`. Six actual signed PocketIC batch tests and the
existing lost/pending single-setup journey pass, along with sixteen focused
native publisher cases, two native setup regressions and strict affected CLI/
standalone-test lint. Tests deliberately drop real admission and preparation
responses: resume uses the original IDs and never repeats either claimed update.
Wrong roles and changed journals refuse; withdrawn permission blocks; a partial
preparation directory remains untouched and prevents any update. Original test
packets, signed claims, raw replies, uncertain/refused outcomes and corrected
compile/lint/patch failures survive. Formatting and whitespace checks pass.

Both live links' recorded nominal expiry is now historical; expiry enforcement
and continuing billing are still unqualified. Do not renew terms, reset either
owner/profile or infer free retention from this local setup evidence. Standing
spend authority persists; the prior below-100T reminder remains based on its
last recorded balances, without a new balance observation in this batch.

## Unreleased: service gaps and current-instance recovery — 2026-10-02

The maintainer confines this work to this repository and selects ordinary
current-instance upgrades as the first active recovery path. The
[gap review](../service-gaps.md) records all five requested areas and explicit
consumer/provider acceptance still open. Production sources and all-feature
dependency resolution have no Canic dependency; the unused `.canic` build-lock
artifact is removed. Framework adapters, Canic composition tests and Toko's
blob/billing hard cut remain consumer-owned; no sibling edit or message occurs.

Shared recovery now obtains one bounded replicated IC `canister_info` reply
against a required immutable actual installation-version anchor in the current v1
record. The installed operator can call `blob_resume_current_instance()` with no
ingress evidence. Complete ordered history must reach installation and exclude
snapshot loads, replacement and unknown changes. A sealed proof is consumed in
the same executing callback/version after rechecking operator and installation.
All owner fences clear together, retaining IDs, reservations, journals, receipts,
balances and liabilities, with no provider effect or automatic retry. The host
fences platform-version reversals/gaps before owner access. A previously active
owner qualifies an ambiguous execution gap once before mutation; already-fenced
upgrade restoration still requires explicit operator recovery. Capture ingress
caller before the await; intervening execution or management invalidates the
history reply. Expired history has no anchor override. The maintained host opens
existing journals; arbitrary backup copying during upgrade is outside this path.

Targeted PocketIC proves repeated ordinary upgrade activation, controller denial,
preserved released physical/billing obligations and pending gateway work,
history-window exhaustion, management-change races and old-heap snapshot refusal.
All 429 core unit tests, seven native composition/binding tests, 54 nonignored
standalone tests, declared/exported Candid and strict affected-package lint pass.
The affected ignored Chromium lost-final-reply journey also passes after
stop/start and operator recovery, with two local PUTs and no redispatch. Initial
ordinary-traffic false fences, the rejected duplicate-export approach and all
other failed selections/builds/checks survive in `.tmp/service-gap-review-01`.
Frozen 0.7.0 remains inspection-only on restore. This required
candidate/record/API/lifecycle hard cut joins a future minor release, with no
version change, commit, deployment, publication or build cleanup. Current target
Wasm is Unreleased despite reporting the unchanged package version; frozen live
bytes remain in `.tmp/trial-v070-live-01/qualified-artifacts` and must not be
replaced by current target bytes in a 0.7.0 deployment.

The separately completed fresh frozen 0.7.0 live record below closes the two
sample upload/download journeys; this session does not repeat its certificate,
transfer, attestation or release. Three earlier bounded verified payer/owner/
gateway reads remain in its private `query-journey-*` captures. Failed independent
offline browser preflights and their separate unsigned profile are retained;
no certificate/upload was sent from those attempts.

Fresh source review at Caffeine revision
`98cbeb63aceaf3e1ae74d9dff64235445643bbdb` retains three bounded public reads.
Reviewed root-only authorized deletion callbacks contain no final object billing
receipt. This is source evidence, not a deployed deletion/cessation result.
Deletion and billing qualification still need separate exact evidence; logical
release and zero gateway usage cannot supply either. Both trial owners and all
physical/liability obligations survive. Consumer actions include the new
installation anchor and explicit host guard/recovery composition after release;
see the [feedback list](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback).

## Next: headless frozen-release publication

Standing authority — 2026-10-02: the maintainer explicitly approves the fresh
trial and covered deployment, account/link and provider work below 100T total or
20T/day, and asks that covered actions proceed without repeat confirmation.
Keep the selected trial within both limits, retain exact intents and reconciliation,
and remind the maintainer when available cycles are below 100T. Authority persists
across continuations; a balance reminder is not a request for permission.
The new live capture is `.tmp/trial-v070-live-01`; no sibling edits, commits,
release/version/publication or build cleanup are authorized by this spending scope.

The [fresh live record](../evidence/caffeine-probes/deployed/2026-10-02-trial-v070-live-01/summary.json)
completes both selected journeys: 1 KiB, then ten-chunk 10 MiB browser uploads,
independent whole-byte verification, accepted verifier attestations and tenant
downloads matching the original snapshots. Thirteen gateway PUT dispatches and
four GETs complete without automatic retry. Both exact first references are
released; final local accounting retains 10,486,784 physical/liability bytes,
zero logical/reserved bytes and two lifetime operations. Do not reset/reinstall
this owner or attempt a third object: its two-object lifetime capacity is full.

Actual new owner: `5gsmp-viaaa-aaaak-qzhfa-cai`, namespace 2, project
`d0770e45-29b6-4449-b499-961aea5daa47`. Its link remains 5T/day with nominal
expiry `2026-10-02T18:40:17.685Z` (raw `1790966417685000000`). Original roles,
selected limits, signed operations, raw responses and the separate two-slot
browser profile `.tmp/trial-v070-browser-01` survive. The 10 MiB body repeats
one chunk hash; distinct consumer media and production serving remain unqualified.

Latest balances are below 100T and the maintainer has been reminded: donor
0.654505097235T, payer 3.0996T, new provider owner 0.995381342792T, old provider
owner 1T, new canister 0.474432286137T. Creation costs 1.0001T gross; cumulative
gross donor debit is 8.1002T, remaining standing authority 91.8998T. The new
1T payer-to-owner allocation is internal, not another donor debit. New provider
credit falls 4,618,657,208 cycles between link and final observations; this is
a temporal debit, not an itemized bill. Gateway usage counters remain zero
despite the proven transfers and credit reduction. Their accounting semantics,
future retention, deletion, billing cessation and expiry enforcement remain open.

Preserved release artifacts initially report 0.6.1 despite the 0.7.0 tag. Before
tenant/link/certificate exposure, verified zero local obligations and absent
provider account permit one recorded correction of only the empty new owner.
Rebuilt 0.7.0 Wasm passes independent PocketIC compiled-release readback, and
actual mainnet module/configuration readback matches. The maintained guard also
rejects the stale artifact locally. Never reuse the earlier preparation Wasm as
a 0.7.0 deployment artifact; its immutable record remains historical.

Prepare concrete installation, account/link, byte/time/spend and surviving-history
inputs before deployment or a new relationship. Standing total trial spend
authority remains 100T; it does not make the donor's liquid balance larger.
Preserve the old owner's stopped certificate/403 browser claim, payment accounts,
allocated credit and exposure/liability. Its one lifetime slot cannot be reset or
replayed to make a fresh trial. The explicit standing authority above now covers
the selected fresh-owner trial; the release notification alone did not.

Complete the headless frozen-release publisher using
the existing service handlers, manifests and journals: inventory/capacity dry run,
serial publication and exact lost-response recovery. Miner then owns its consumer
wrapper and serving/CSP acceptance. Canic adoption remains deferred; the
[open consumer feedback](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) records current
0.7.0 types, resource sizing, lifecycle and delivery obligations.
The feedback list adds independent deployment-artifact checks and limitations of
the successful live samples. No sibling edits or upstream messages occur.

## Unreleased: frozen publication inputs and fresh-trial preparation — 2026-10-02

Native `publish-inputs` now consumes a bounded inventory of exact binding/manifest/
body paths and hashes. It reuses single-file preparation through one shared
installation validator, Caffeine manifest decoder and streaming verifier. Preflight
checks unique upload/object/first-reference identities, one tenant/provider scope,
per-file constraints and aggregate fresh-installation object/leaf/byte capacity.
No ID allocation, live-capacity reservation, certificate or network occurs.
Serial private snapshots require both the Caffeine root and raw inventory digest;
completed files survive a later failure, with retained failure evidence and no batch
success summary. Existing/partial output refuses overwrite. See the
[operator recipe](../operator-guide.md#freeze-a-publication-inventory-offline).

The [new preparation record](../evidence/caffeine-probes/local/2026-10-02-trial-v070-preparation-01/summary.json)
and `.tmp/trial-v070-preparation-01` retain frozen 0.7.0 source and preserved build
artifacts, independent
Candid encode/decode, a complete candidate with 10 MiB upload/read budgets, and
real SDK 1 KiB/10 MiB bodies verified through the native batch. Exact snapshots
match all 10,486,784 bytes and eleven leaves; zero network requests. The batch CLI
is a separately retained Unreleased development artifact, not a released 0.7.0 tool.
Stand-in installation/link/upload bytes are local-only and must never be deployed.
The later live review above discovers that the preserved Wasm reports 0.6.1;
use the independently qualified replacement in the new capture for 0.7.0.

Targeted native tests and strict CLI all-target/all-feature lint pass, including
duplicate IDs/scope changes, aggregate bounds, input hash/path refusals, later-body
corruption and digest disagreement before usable requests. The sandbox initially
blocks two existing local HTTP listeners; an authorized loopback rerun passes.
Initial patch-selection and lint failures remain recorded. Formatting/diff checks
pass. No full CI, paid/live calls, deployment/link, version/release/commit, sibling
edit or build cleanup occurs.

The [concrete fresh-trial proposal](../standalone-trial.md#fresh-070-trial-proposal)
selects the original operator/payer/uploader/verifier, one new 0.7.0 owner, namespace
2 and a separate project/bucket. Propose 1T creation amount (at most 1.0001T gross
with historical fee), a new payer link at 5T/day with nominal two-hour expiry, then
small upload/verification/attestation/download before the 10 MiB file. Refresh
actual fee, liquidity, account allocation and terms before effects; the 100T total
spend approval persists and is not liquid balance. The prior link's recorded
nominal expiry has passed; expiry enforcement is still unqualified.
The maintainer subsequently approves this new deployment/relationship and covered
trial actions under the standing limits above; no further confirmation is needed.
Do not touch the old owner, payment records, browser history or liabilities.

Authenticated batch capacity/discovery now exists in `publish-check`; see below.
Next connect serial admission and preparation to the original frozen permissions
and existing signed-operation journals, then provider transfer and exact
lost-response recovery. Keep IDs owned by the surviving consumer intent and
the service's admission journal; do not derive freshness from an old counter. The
[consumer feedback](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) records complete-batch
and media-map acceptance; Canic integration remains deferred. The selected live
samples pass, while a complete publisher and Miner delivery remain unqualified.

## Unreleased: authenticated frozen-batch check — 2026-10-02

Native `publish-check` opens only a complete `publish-inputs` batch. Reverify its
inventory/installation hashes, complete body roots/raw digests and exact saved
permission/manifest/browser packets before identity loading or networking.
Explicit finite query/time budgets cover one capacity and one indexed content
query per file. Private create-new output retains original inputs, intent, every
argument/reply and a complete blocker report; partial failures remain without a
success summary. No ID allocation, reservation, service update or provider call.

Discovery keeps exact original IDs and distinguishes unseen, pending/exposed,
live-needing-retain and retired roots. Fences, suspension, duplicate planned roots,
object/metadata limits and independent object/leaf/byte/concurrency headroom are
explicit blockers. Exit zero includes blocked observations; sequential replies
are not a transactional reservation, permission to retry or publication authority.
Actual installed project/uploader and provider availability remain unproved.

Focused native input/publisher tests and strict CLI all-target lint pass. Actual
signed PocketIC tests pass against the frozen qualified 0.7.0 Wasm, including
unchanged storage, original reservation discovery, cancellation without history
refund and restored fences. The [new live read-only record](../evidence/caffeine-probes/deployed/2026-10-02-publish-check-01/summary.json)
uses three authenticated queries on the existing owner: both released sample
roots are retired, original IDs survive, remaining lifetime objects are zero,
and fresh demand is zero. No write, provider traffic or additional funding occurs.

The initial fixture uses a non-ByteString project and correctly refuses offline;
the corrected ASCII fixture passes. Function-length and test-style lint failures
are retained and corrected. Concurrent recovery/dependency/host/test work is
preserved; a brief build-lock wait leads to host-visible PID/cwd checks before
subsequent edits/builds. No sibling edit, full CI, version/release/commit or cleanup.
The [operator recipe](../operator-guide.md#check-a-frozen-batch-against-live-capacity)
and [consumer feedback](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) carry the new
contract. Next is serial admission/preparation and exact interruption recovery.

## Completed in 0.7.0: configurable upload limits for Toko Miner — 2026-10-02

The maintainer explicitly requests relaxing the 1 KiB cap because it delays Miner
integration. Current source removes the extra certificate-policy envelope, including
its single-tenant/object/reference restrictions. Sizes and capacity now use the
validated installation's existing resource configuration. Admission reserves
object/tenant/global bytes and lifetime history/leaves; preparation and issuance
recheck the exact original permission/reservation. Uploader trust, namespace,
one-time exposure, durability, deadlines/revocation and restore fences remain.

The former policy envelope, host-evidence field and public blocker are removed
from core, standalone Candid, native JSON, fixtures and tests. There is no parallel
mode or bypass. This public semantic/API hard cut shipped in the 0.7.0
minor release; the deployed 0.6.0 owner remains frozen.
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
the [consumer feedback](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback). Consumers must
adopt the 0.7.0 field/variant removal and choose their own explicit
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
deferred; its [open feedback](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) records that
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
its [feedback](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) records separate balances.

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
deferred, with updated [consumer feedback](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback)
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
Canic and its [consumer feedback](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) remain deferred/open.

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
[consumer feedback](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) remain deferred/open.

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
[consumer actions](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) remain deferred/open.

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
its [consumer actions](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) remain open.

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
[consumer actions](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) remain open.

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
[consumer actions](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) remain open. No build,
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
[consumer actions](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) open. No build, full CI,
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
[consumer actions](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) remain open.

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
Canic stays deferred; its [consumer actions](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback)
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
[consumer actions](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) remain a future backlog.

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
authenticated behavior. Future [consumer actions](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback)
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
the [feedback list](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) records that open action.
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
[feedback list](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) records that action without
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

Canic adoption remains deferred. The [feedback list](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback)
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
select compatible HTTPS gateways/browsers; the [feedback list](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback)
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

The separate 0.7.0 live owner now completes 1 KiB/10 MiB upload, independent
verification, attestation and tenant download; see the current record at the top.
Consumer publication, serving, operational recovery and provider guarantees remain
open. Preserve exact live identities, scope, terms, bounded traffic and surviving
obligation ownership. Reopen the selected journal after restart and stop if its
history is missing. The current two-slot browser trial and two-object service
lifetime history are consumed; do not reset either to make new attempts.
The standing 100T/20T approval is recorded above; no repeat confirmation is needed
for covered actions. It is not a guaranteed maximum external bill.
Offline installation-check requires --trusted-uploader, writes complete init bytes
and does not deploy.
See the [trial plan](../operator-guide.md#isolated-uploaddownload-trial-plan),
[acceptance](../acceptance-plan.md), [contract](../service-contract.md) and
[probe ledger](../evidence/caffeine-probes/README.md).

## Consumer integration feedback

The [current action list](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) records these
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


## Completed in 0.12.0: simplification and frozen formats — 2026-10-03

These are the original pre-release handoff notes. Their draft and validation
statements describe that batch; the released baseline is now 0.12.0.


The simplification batch shares browser UTF-8 bounds and refuses malformed metadata
before journal access. The job helper owns the selected body once, before cloning
metadata. One semantic Candid test replaces the duplicate generated-text check.
The current handoff and roadmap no longer mix historical instructions with active work.

The fixture-only reference save/inspect command, JSON record and append
journal contract are retired. Signed native receipt tests cover its service guarantees;
their journeys retain malformed-ingress checks. Native create-new signed claims
remain authoritative and old artifact files are untouched. Tests for the retired
save/lock mechanism are removed, with no replacement abstraction or reader.
The harness now inherits the workspace MSRV after removing its file-lock requirement.

Focused browser, semantic Candid, native claim/operator and signed reference
recovery checks pass, together with strict affected Clippy and Rust 1.88 harness
compilation. Exact scopes and retained evidence are recorded in
[the local ledger entry](../evidence/caffeine-probes/README.md#simplification-follow-up--2026-10-03).
Concurrent ic-memory/ic-testkit dependency updates are preserved and their tested
identities are recorded in the local evidence; they are separate work.
No version mutation, full CI, deployment or provider effect
is included. The fixture command removal belongs in a minor release.

The format follow-up replaces reused numeric markers with one frozen identity per
current layout: `ic-blob-storage/upload-inputs:original-preparation` for native
bindings and `ic-blob-storage/installation:platform-anchor` for immutable stable
installation records. Current producers and single/batch/session readers converge
on these contracts. Exact release and service checks remain independent. A missing
or mismatched identity refuses without repair, replacement or effect redispatch.
The host grant key still names the same slot, so old allocations are not hidden.

This is a minor native/stable-format hard cut, not a deployment or upgrade plan.
Preserve retained snapshots/journals and original binaries. Existing installations,
including any 0.11.0 deployment, cannot upgrade to this format; retirement and full
obligation disposition precede a fresh reinstall. Consumer adoption and deployed
inventory remain unverified. No V2, dual reader, journal conversion or migration
engine is added. Native preparation, frozen-batch/journal checks, strict affected
Clippy, offline SDK/native handoff, semantic Candid and actual local IC lifecycle/
session recovery pass. The [format evidence](../evidence/caffeine-probes/README.md#frozen-format-identities--2026-10-03)
records exact scopes and unchanged retained-artifact hashes under `.tmp/format-identity-01`.

The descriptor follow-up removes the forwarding public `describe` API, the second
operational view and an HTTP target that the service response discarded. The shared
`handle` checks ingress identity and calls the upload owner's existing operational
checks, then presents the retained descriptor under the same trusted borrowed scope.
Clients retain canonical target construction. Historical inspection remains distinct
from active, unfenced serving. Native/stable frozen layouts and endpoint wire shapes
are unchanged; this public source API cut joins the current minor draft. Twelve
focused unit cases, nine actual local IC cases, semantic Candid equality and strict
core/standalone/harness/CLI Clippy pass. The [descriptor evidence](../evidence/caffeine-probes/README.md#descriptor-serving-owner--2026-10-03)
retains the exact tested artifacts and logs under `.tmp/descriptor-owner-01`, and
explicitly records the missing pre-run intent file. Consumer source adoption remains
unverified; no live provider request or paid effect occurred.

## 0.14.3 pre-release handoff — 2026-10-04

This is the original handoff before the maintainer pushed 0.14.3. Its draft,
release and pending-validation statements describe that earlier workspace.
Historical probe identities and failed attempts remain unchanged.

Date: 2026-10-04

### Released baseline

Released **0.14.2** is at `556b52f`, with validated source
`11ee45684ae90a96d8711fccbb81838a37bbb517`. Read
[Cargo](../../Cargo.toml), [the release receipt](../release.json) and
[the changelog](../../CHANGELOG.md) for authoritative release metadata.
A repository release does not establish registry publication or deployed behavior.

The core owns tenant policy, permissions/manifests, references, quotas, economics
and durable local journals. Standalone authenticates platform context and delegates
to shared handlers. Linking the library exports no endpoint or lifecycle hook.
Consumer frameworks own their wrappers and composition tests externally.

Configured trusted-uploader issuance and explicit verifier completion are maintained.
Logical release, physical deletion and billing cessation remain separate facts.
Same-release restoration validates every owner synchronously and fences mutations;
only an independent current-execution IC-history proof can resume the current
installation. Older-snapshot activation and cross-release upgrades are unsupported.

Released 0.13.0 retains original native setup/observation/transfer sources, exact
native/browser selection and the profile binding before Chromium access. Native
transfer handoffs are passive provenance; the existing browser journal owns paid
claims. Repeated/recovered handoffs request certificate recovery only. Missing or
changed history refuses without replacement, redispatch or an inherited cursor.
Old contracts stay with their original binaries/artifacts; no conversion exists.
The [source](../evidence/caffeine-probes/README.md#native-session-original-sources--2026-10-03),
[binding](../evidence/caffeine-probes/README.md#browser-profile-launch-binding--2026-10-03),
[selection](../evidence/caffeine-probes/README.md#native-session-browser-selection--2026-10-03)
and [handoff](../evidence/caffeine-probes/README.md#native-browser-transfer-handoffs--2026-10-03)
records retain their exact checks, failures, artifacts and qualification limits.

Released 0.14.0 adds native next-phase guidance, the callable Chromium driver
and explicit native subprocess control. One exact current complete map, matching
native final report and exit zero establish driver completion. Native Rust owns
authenticated completion/references; the existing browser journal owns paid claims.
No second dispatch/retry owner, persisted cursor or asset-registration owner exists.
All Cargo members inherit root package/dependency versions and local paths.
The [guidance](../evidence/caffeine-probes/README.md#native-phase-guidance-and-browser-driver--2026-10-03),
[native pipes](../evidence/caffeine-probes/README.md#private-native-subprocess-control--2026-10-04)
and [completion](../evidence/caffeine-probes/README.md#one-publication-completion-boundary--2026-10-04)
records retain exact artifacts, checks, failed attempts and local qualification limits.
The release fixes the stale installation-test package-version inequality; actual
release readback, wrong-service refusal and unchanged state remain checked.

Released 0.14.1 groups the
[distinct-PNG](../evidence/caffeine-probes/README.md#distinct-png-publication-and-verified-decoding--2026-10-04),
[multi-chunk/reference](../evidence/caffeine-probes/README.md#multi-chunk-png-and-overlapping-references--2026-10-04)
and [direct browser-delivery](../evidence/caffeine-probes/README.md#direct-browser-url-delivery--2026-10-04)
checks. These authored local fixtures qualify exact verified bytes, lost-reply
recovery without another PUT, tail-corruption refusal and overlapping-reference
cleanup. Logical release refuses descriptors while preserving physical/liability
bytes; saved public URLs still serve under the substitute. Their original
observations use retained 0.14.0 CLI/Wasm binaries, not a live 0.14.1 installation.
The current 0.14.1 lock selects ic-memory 0.24.5 and ic-testkit 0.14.4; historical
records retain their own exact graph/artifact identities.

Released 0.14.2 groups the locked-dependency bootstrap and
[local image/CSP checks](../evidence/caffeine-probes/README.md#png-image-loading-under-csp--2026-10-04).
The CI/validate/release gate fetches the selected lockfile before offline checks;
scoped offline commands still need cache preparation. One fixture helper owns
media sampling, canonical direct URLs and ordinary anonymous image checks under
an authored policy. Private bounded request traces survive assertion failures.
Its original six final browser journeys and installation observations retain their
0.14.1 CLI/Wasm identities; source release does not relabel historical artifacts.

### Current work — 0.14.3 draft

The [representative emitted-media record](../evidence/caffeine-probes/README.md#representative-emitted-media--2026-10-04)
now covers selected frozen consumer PNG/JPEG/WebP/GLB bytes through the same
native/browser publisher, verifier, journal and reference owners. Fixture MIME
and installation limits derive from the selected bodies. Private assets remain
ignored; no consumer framework dependency, wrapper or build is added here.

Seven local IC/browser journeys pass: both real pairs complete and recover lost
final replies without another PUT, PNG tail corruption refuses before the next
admission, and authored-PNG/synthetic regressions preserve their behavior.
The real image pair retains 4,318,116 physical/liability bytes; the eight-chunk
GLB/WebP pair retains 8,389,774. Native whole-content verification and complete
browser digests agree. Images decode/display under the authored local CSP;
GLB structural inspection does not establish game rendering. Occupied native map
output refuses and preserves existing content, then fresh signed queries recover
a complete map without upload. Overlapping references retain their cleanup/history
and physical/billing obligations. This is not a consumer registration transaction.

Fresh CLI/Wasm/harness/browser builds and strict affected Clippy pass. Installation/
Candid cases and the final exact artifact capture pass with independently expected
compiled release 0.14.2. Initial compilation/lint/capture failures remain retained,
including the rejected precreated capture directory. All exact frozen inputs,
source/bundle/binary identities, profiles, claims, maps and downloads remain under
`.tmp/representative-media-01`; separate manifests retain their hashes. Cargo and
the release receipt remain 0.14.2. No full CI, commit, version mutation, publication,
deployment, live provider request, paid cycle, sibling edit/message or build cleanup
occurs; both old live owners remain unchanged.

### Current checks and next work

The [transfer-budget record](../evidence/caffeine-probes/README.md#publication-transfer-budget-preflight--2026-10-04)
qualifies refusal before certificate-client construction for known tree/chunk
count, largest-chunk and total-body shortages. Chunk size comes from the retained
SDK preparation, without a second chunker or provider serializer. Exact-fit lower
bounds reach the original intent boundary; worker refusal saves/claims nothing
and preserves its finite error code. Existing gateway checks still bound opaque
tree/certificate overhead at dispatch. Actual local eight-chunk model completion
and real-image lost-reply recovery pass without another PUT.

The [source-tool record](../evidence/caffeine-probes/README.md#source-tool-installation--2026-10-04)
qualifies a fresh-prefix locked CLI installation, offline JSON version readback
and existing two-chunk SDK/native snapshot handoff. The
[source installation recipe](../local-tools.md#install-the-native-and-browser-tools)
selects native/browser tools from one checkout and retains their artifact hashes.
This uses captured dirty source and an existing dependency cache; clean released
downstream installation and consumer adoption remain unqualified. Strict affected
CLI Clippy, formatting and browser checks pass. Both probe records retain exact
source/artifact identities; no full CI or live/paid effect occurs.

The [isolated-source tool record](../evidence/caffeine-probes/README.md#isolated-source-tool-installation--2026-10-04)
now qualifies a separate regular-file snapshot, fresh CLI installation prefix and
fresh pinned npm dependencies. The browser build works with Git discovery disabled;
the exact installed CLI passes cached two-chunk SDK/native snapshot checks. Matching
Wasm/harness/browser tools complete the cached eight-chunk GLB/WebP pair and recover
the cached PNG/JPEG lost final reply without another PUT. Original roots, ordered
leaves, complete bodies/cache headers, complete maps and reference liabilities
match the retained consumer evidence. No installation correction or new mechanism
was needed. Toolchain/profile/binary/bundle provenance remains captured.

That source is the unreleased draft with compiled release 0.14.2, ic-memory 0.24.7
and ic-testkit 0.14.7; it reuses this repository's Cargo cache and provisioned
Chromium/PocketIC. During execution the workspace lock changed independently to
ic-memory 0.24.10, retaining ic-testkit 0.14.7. Both locks and the rejected
post-check equality assertion remain recorded. Separate locked fetch and strict
affected CLI/standalone Clippy pass on the current 0.24.10 graph; the successful
runtime journeys keep their original 0.24.7 identities. Initial sandbox DNS fetch
failure and successful network preparation remain retained. This is not a cold
machine, clean released downstream installation or consumer adoption. Full CI
remains pending; no live/paid request, release or sibling edit occurs.

The separate [current-memory record](../evidence/caffeine-probes/README.md#current-memory-initialization-and-lifecycle--2026-10-04)
now qualifies actual 0.24.10 Wasm initialization and same-release restoration.
Matching captured source/Wasm/harness pass the existing stop/start/repeated-upgrade
fence case, recovery with released physical/billing obligations and stale-snapshot
refusal without stable mutation. No production correction or new test is needed.
This is focused local IC lifecycle evidence; media publication keeps its original
0.24.7 artifacts and full current CI remains pending.

The [original-cache metadata record](../evidence/caffeine-probes/README.md#original-cache-metadata--2026-10-04)
now qualifies all four retained consumer roots, headers and leaves with the actual
SDK. Optional native `preparation.cache_control` passes unchanged through saved
bindings/descriptors into browser `cacheControl` and the same SDK preparation.
Omission adds no header and preserves the existing contract; malformed hints and
changed/omitted required cache metadata refuse. No inferred policy, parallel
preparer, layout replacement or compatibility reader is added.

Four local IC/Chromium journeys pass using the original cached roots: GLB/WebP
completion, PNG/JPEG lost-reply recovery without another PUT, PNG corruption before
next admission and the authored uncached regression. Browser fetches observe the
exact cache header and complete bytes. Physical/liability and reference history
remain unchanged by logical release. Native hint tests, SDK/native snapshot
handoff, browser/worker/launcher checks, strict affected Clippy and formatting pass.
Fresh CLI/harness/browser artifacts and the retained unchanged 0.14.2 Wasm remain
under `.tmp/cache-metadata-01`. These are local facts, not deployed cache behavior,
game rendering, a consumer asset transaction or adoption. Full CI remains pending.

The [GitHub issue review](../evidence/caffeine-probes/local/2026-10-04-gh-issues-review-01/summary.json)
reads all seven open issues and discussion against remote main `556b52f`
(released 0.14.2), published package contents and current draft source. The
published crate checksum and both embedding helper sources match 0.14.2.
Both native FIFO/no-writer boundaries return typed `file` without transport.
No GitHub issue/comment is changed. Latest Miner acceptance is reported by its
issue comments, not independently rerun by this review.

| Issue | Current disposition | Next action |
| --- | --- | --- |
| [#1 embedding helpers](https://github.com/dragginzgame/ic-blob-storage/issues/1) | Both helpers implemented since 0.10.0 and verified in published 0.14.2 | Ready to close the upstream request; host adoption remains separate |
| [#2 transfer budgets](https://github.com/dragginzgame/ic-blob-storage/issues/2) | Fixed and qualified in the 0.14.3 draft; absent from released 0.14.2 | Release the current draft before closing as a released fix |
| [#3 FIFO inputs](https://github.com/dragginzgame/ic-blob-storage/issues/3) | Shared descriptor fix and regression in Git release 0.10.0; both reader/body refusals confirmed | Ready to close; the CLI is source-installed, not a registry package |
| [#4 publisher](https://github.com/dragginzgame/ic-blob-storage/issues/4) | Existing driver and isolated installation pass locally; Miner reports real original-root recovery | Clean released downstream installation and adoption; complete consumer mapping transaction |
| [#5 serving/integrity](https://github.com/dragginzgame/ic-blob-storage/issues/5) | Original cache metadata fixed locally; Miner reports model/image loading, verified cache reads and Blob revocation | Certified asset transaction, actual production bindings/CSP, full inventory and deployed serving acceptance |
| [#6 lifetime/reuse](https://github.com/dragginzgame/ic-blob-storage/issues/6) | Reference recipe, retired-root refusal, bounded capacity/history and local overlap implemented | Consumer retention/retirement policy and actual provider deletion/billing evidence; preserve both exhausted owners |
| [#7 decoder budgets](https://github.com/dragginzgame/ic-blob-storage/issues/7) | Valid byte-bounded skip/type refusals now pass across actual standalone boundaries; no reproduced bypass | Retain the new coverage with this draft; independent work-quota exhaustion remains unreproduced, and header exhaustion is subsumed by the byte gate |

For #7, the existing standalone harness now covers valid skipped-text and
excessive type-table requests at tenant query/update, manifest preparation,
certificate assessment/issuance and installation. Each payload round-trips with
only its target budget relaxed. Actual IC refusals preserve stable bytes,
tenant/configuration, prepared permission and certificate eligibility; failed
management reinstall retains its original owner. Small extra-argument controls
reach valid query/update/init handlers. Production code and limits are unchanged.

The [decoding contract](../service-contract.md#standalone-ingress-decoding)
documents that Candid 0.10.37 header exhaustion is subsumed by the equal ingress
ceiling. A 65,379-header manifest fits just below 128 KiB, decodes within the
2-million-unit ceiling with zero skipping and reaches typed domain `Limit`
without mutation. Independent work-quota exhaustion remains unreproduced;
canonical dense input is not proof covering every possible wire subtype.
Do not weaken limits or add production hooks to manufacture that failure.

Both new ingress cases, both extended certificate cases and the existing invalid
installation/malformed-ingress regression pass in actual PocketIC. Strict affected
Clippy passes. Matching harness/source/lock identities and full outputs remain in
`.tmp/decoder-budgets-01`, using the retained current-memory Wasm on the unchanged
0.24.10 graph. Initial compilation, fixture isolation and fresh-timestamp assertion
failures are retained, including the first work candidate which did not exhaust
its quota. These are decoder-only local IC checks, with no provider calls or
new Caffeine observations. Full CI remains pending; no GitHub write, release,
deployment, paid effect, sibling change or cleanup occurs.

- Consumer producers must retain their original hashed cache value as
  `preparation.cache_control` and use the matching native/browser tool source.
  All four selected roots work locally; Miner's latest #5 comment also reports
  original-root GLB/WebP consumer acceptance. Complete production publication
  and deployed serving remain unqualified.
- Qualify a clean released tag downstream and replace Miner's registry 0.7.0
  packaged-preparer installation only upon adoption. Isolated source installation
  and matching native/browser composition now pass locally; they are not published
  tool distribution or a consumer publication transaction.
- Consumer owners must qualify their asset transaction and production fetch/CSP
  bindings. Miner's latest #5 comment reports production loaders exercised in a
  local shell, verified cached image bytes and Blob URL revocation, with zero CSP
  violations. That shell/map is not certified or an asset-registration transaction;
  no game-rendering or deployed Caffeine acceptance is reported.

### Remaining product work

- Consumer adoption of the callable driver and native subprocess helper, with
  explicitly selected native/browser identities, binary/trust inputs, same-release
  restart and exact original history retention.
- Consumer acceptance with real media, complete asset transactions, overlapping
  references and MIME/CORS/cache/CSP/retention behavior. Selected emitted bodies
  pass locally; consumer transactions and deployed serving remain unqualified.
- Provider deletion and final billing evidence; surviving inventory and independent
  freshness for any proposed older-backup activation. See
  [service gaps](../service-gaps.md) and [the contract](../service-contract.md).
- Large-inventory synchronous reopen and browser heap/CDP qualification before
  promising million-object operation. Configured ceilings are not measured scale.

### Consumer integration feedback

Canic adoption remains deferred until useful repository-local work is exhausted.
The [feedback list](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) records wrapper/lifecycle,
one-runtime memory composition, current formats/recovery, original preparation
hints, adoption of native guidance/callable driving/subprocess control and the
public URL/access-policy distinction. Siblings remain read-only; no upstream
message is authorized.

Consumers must also select and qualify their actual image/fetch origins and CSP
with real assets. The new authored-policy image checks are local evidence only;
the [feedback list](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback) retains that action.

The old isolated owner remains frozen at 0.6.0 with stopped original history;
the separate live owner was last verified at 0.7.0. Both retain exhausted lifetime
capacity and original provider/billing obligations. Do not reset or upgrade them
as part of source cleanup.

### Evidence and history

[The probe ledger](../evidence/caffeine-probes/README.md) distinguishes source review,
offline checks, actual local IC/browser execution, substitutes and deployed facts.
The [implementation archive](history.md) retains original commands, failures,
source/artifact identities and superseded next-step notes. Its historical release
and dependency statements must not override this handoff.

## 0.14.6 pre-release handoff — 2026-10-04

This is the handoff retained before the maintainer reported 0.14.6 live. Its draft,
validation and dependency statements describe their original batches. The current
handoff supersedes its forecasts without relabelling any frozen evidence.

Date: 2026-10-04

### Released baseline

Released **0.14.5** is at `8e1cfed48327b9da56aadf417a179549a33c6602`,
with validated source `ec49a86b5a76c9a2bc00f7d083cd6631b25f57eb`.
The local release tag identifies that commit; the maintainer reports the release live. [Cargo](../../Cargo.toml),
[the release receipt](../release.json) and [the changelog](../../CHANGELOG.md)
own release metadata. This source release does not prove registry publication,
consumer adoption or deployed provider behavior.

The core owns tenant policy, uploads, manifests, references, quotas, provider
economics and durable local journals. Standalone authenticates actual platform
context and delegates to shared handlers. Consumers own framework wrappers and
asset-registration transactions externally; linking the library exports no
endpoint or lifecycle hook.

Trusted-uploader issuance and explicit external verifier completion are maintained.
Logical release, physical deletion and billing cessation remain separate facts.
Restoration validates every owner synchronously and fences mutation. Independent
current-execution IC history is required to resume; older-snapshot activation and
cross-release upgrades remain unsupported. Retired roots cannot be reintroduced.

0.14.3 includes original cache metadata through the native/browser/SDK handoff,
known transfer-budget refusal before certificate intent, machine-readable native
version/source installation and structurally valid decoder-budget coverage.
Selected emitted PNG/JPEG/WebP/GLB bytes complete/recover locally through the same
native/browser journal, verifier and reference owners. Their earlier observations
retain original compiled releases and dependency graphs; release does not relabel
those artifacts. The [pre-release handoff](history.md#0143-pre-release-handoff--2026-10-04)
and [probe ledger](../evidence/caffeine-probes/README.md) retain exact history.

### Current checks and next work

The [clean released-tool record](../evidence/caffeine-probes/README.md#clean-released-source-tool-installation--2026-10-04)
now qualifies Git tag 0.14.3 in a frozen archive, a fresh locked native prefix,
fresh pinned npm dependencies and browser build with Git discovery disabled.
Offline version is 0.14.3; actual SDK/native snapshot/repeat/corrupt-source checks
pass. Matching fresh release Wasm/harness and exact installed tools complete
original cached GLB/WebP and recover PNG/JPEG lost replies/control interruption
without another PUT. All four original roots/leaves/whole bytes/cache headers,
complete maps, original uncertain claims and reference liabilities match.

This is a clean released-source check in this repository, with retained Cargo
cache and provisioned Node/Chromium/PocketIC. It is not consumer-owned installation,
a cold machine, an application transaction or deployed provider qualification.
Source/archive/receipt/toolchain/binary/bundle/result hashes and original profiles
remain under `.tmp/released-tools-01`. The source uses ic-memory 0.24.10,
ic-testkit 0.14.7, Rust 1.99.0 and Node 24.21.0.

The [fixture deadline record](../evidence/caffeine-probes/README.md#publication-fixture-session-deadline--2026-10-04)
qualifies one fixture-owned 120-second session deadline for original/recovery native
intents, browser bootstrap and subprocess ownership. Individual preparation/query
calls remain 30 seconds; the separate whole-fixture timer remains 180 seconds.
Production defaults, provider bounds, claims and retry policy are unchanged.
Miner's earlier 30-second failure and all old artifacts remain intact.

Both updated fixture journeys pass. Effective original/recovery commands, saved
intents and browser results agree on 120 seconds. GLB/WebP makes eleven PUTs/nine
GETs and retains 8,389,774 physical/liability bytes; PNG/JPEG recovery makes seven
PUTs/ten GETs and retains 4,318,116, with no redispatch. Strict affected Clippy,
formatting, JavaScript syntax and retained hash checks pass. Exact current fixture
harness/browser and released CLI/Wasm identities remain under
`.tmp/publication-deadline-01`, separately from clean-release observations.
No full CI, version mutation, commit, GitHub write, registry publication,
deployment, live provider request, paid cycle, sibling edit/message or cleanup
occurred in those retained qualification runs. The maintainer subsequently
released the fixture and failure-boundary fixes as 0.14.4; their frozen artifacts
retain their original 0.14.3 identities.

The follow-up browser/native failure-boundary fixes redact Chromium shutdown
diagnostics and preserve the original startup/control refusal during cleanup.
The asset server closes even when context close rejects; original profiles and
claims survive. Native argument/frame encoding failures now return finite
`configuration` refusals without exposing serialization diagnostics or writing
an invalid frame. A subsequent valid frame still starts at step zero.

Scoped native-pipe and real Chromium/profile tests pass, including shutdown
diagnostics during startup, execution, native control and explicit close,
repeat close and original port/profile reopening. Browser bundles rebuild and
JavaScript syntax checks pass. Original failures and final logs remain under
`.tmp/browser-bridge-fixes-01`. The proposed malformed-HTTP asset crash was ruled
out by Node's parser returning 400 before the handler; no asset-route change or
extra test was retained. The first sandboxed pipe run returned exit refusals for
the fixture children; its log is retained alongside the successful subprocess
runs outside that sandbox. Child stderr was deliberately not exposed, so its
failure cause is unqualified.
These are local bridge checks with no IC/provider requests or paid effects.
They do not requalify the frozen media/provider observations above. A rejected
Chromium close does not prove worker termination; consumers still own cleanup
and adoption of the selected tools.

#### 0.14.5 implementation evidence

The maintainer released this batch as 0.14.5. Its pre-release scoped records
below retain their original 0.14.4 source/artifact identities; release does not
relabel those observations.

Browser journal handles now have one close owner for explicit shutdown, platform
version-change and unexpected connection closure. New operations return typed
`store-closed` after invalidation; startup also refuses an invalidated connection.
Already-started transactions may finish, with no cancellation, rollback, reset or
replacement history. A real Chromium version-change event reproduced numeric
DOMException code 11 before the fix; the updated check uses only an empty dedicated
fixture database. All retained effect journals stay intact.

Bootstrap now owns one passive snapshot and validates one scope/trust root for
host jobs and its private payload. Independent worker boundary validation remains.
Actual Chromium store, bootstrap and launcher checks pass, including cross-tab
claims, cancelled/uncertain history, missing-store refusal, 675-row restart
preservation, invalid signer/scope controls before storage and profile/control
failures. Browser bundles rebuild; JavaScript syntax and diff checks pass.
Original failure and final logs/profiles remain under `.tmp/browser-journal-lifecycle-01`
and its recorded store profile. No IC/provider request, paid effect, full CI,
dependency/version change, commit, deployment or sibling change is performed.
Consumer-owned installation/adoption and certified asset transactions remain open.

Native saved-request and exact installation decoding now use one bounded helper,
replacing repeated decoder setup. Each caller retains its byte/work/type ceilings;
exact arity, no skipped fields and redacted diagnostics have one owner. Ordinary
authenticated reply decoders retain their separate contracts. The extra-argument
suspect was ruled out by the original focused test; this is consolidation, not a
claimed arity vulnerability. No packet, journal, wire or stable-state layout changes.

Local byte verification also delegates permission structure to the core validator,
preserving scope/actor checks, its local byte bound and original open-file ownership.
The original verifier fails the new invalid-permission regression before body/query
access; the maintained validator passes. Native CLI tests pass, including original
recovery/signed claims, exact installation checks, corrupt/changing files and owned
loopback transport fixtures. Strict CLI Clippy and formatting pass. Original logs,
failed intermediate checks and final source hashes remain under
`.tmp/native-request-decoding-01`. These are offline/local HTTP checks, with no live
IC/provider calls or paid cycles; they do not qualify deployed provider behavior.

The 0.14.5 driver follow-up also refuses unknown/non-string continuation phases
through the finite `native-control` boundary. Phase and final replies now share
the same serialization/8 MiB guard, removing two parallel size checks. A real
launcher check reproduced a `closed` refusal for an invalid phase before the fix.
Current Chromium launcher checks pass for unknown/object phases, cyclic/BigInt
phase and final values, over-limit replies, matching successful final results,
cancellation and original profile binding preservation. Browser bundles rebuild;
JavaScript syntax, changelog and diff checks pass. Original failure, final logs,
source/bundle hashes and profiles remain under `.tmp/native-phase-validation-01`.
Only owned loopback assets are requested, with no live IC/provider request or paid
cycle. Consumer adoption and certified registration remain open.

The released lock uses ic-memory 0.24.14 and ic-testkit 0.14.11. Earlier scoped
native/Wasm records retain their original dependency identities; the JavaScript
checks above did not requalify that Rust graph.

#### 0.14.6 draft

[The changelog](../../CHANGELOG.md) groups the completed browser performance and
SDK copy fixes, profiling evidence and ic-memory dependency update under undated
0.14.6. Cargo remains at 0.14.5; full release validation and version preparation
remain with the maintainer's release flow. Scoped records below retain their
original identities.

The native-to-browser launcher now sends one bounded base64 string per 64 KiB
raw frame, removing Playwright's per-element byte-array serialization. Native
regular-file ownership, exact length/EOF/SHA checks, worker digest/SDK preparation
and certificate-intent ordering remain unchanged. No alternative handoff, new
profile format, retry or provider contract is introduced.

The [handoff profile record](../evidence/caffeine-probes/README.md#browser-body-handoff-profile--2026-10-04)
retains before/after actual Chromium runs with identical 1/8/32 MiB bodies.
Instrumented execution through deliberate root refusal improves from
2.99/22.88/86.41 seconds to 0.08/0.48/1.83 seconds. Worker wait/preparation stays
similar; these are client bridge observations, not Canic server measurements
or successful upload latency. Each refusal precedes certificate intent and leaves
no journal row. Only owned loopback assets are requested, with zero provider/IC
requests and paid cycles. Original profiles, bodies, logs and source/bundle hashes
remain under `.tmp/browser-handoff-profile-01`.

The existing Chromium launcher checks pass with NUL/high binary body bytes,
profile binding, cancellation, control/serialization failures and shutdown.
Bundles rebuild; scoped syntax, changelog and diff checks pass. An opt-in maintained
profiling fixture records frame counts, elapsed time, sampled Node memory and
page heap without imposing timing thresholds or adding default CI work.
Page heap excludes the dedicated worker/browser RSS; sampled Node memory is not
an exact peak. Full worker/process memory, populated service reopen, concurrent
publishers and million-object scale remain unqualified. No Rust build/full CI,
version change, commit, deployment, sibling edit/message or cleanup is performed.
Consumer adoption of selected tools and certified asset registration remain open.

The [SDK owned-Blob record](../evidence/caffeine-probes/README.md#sdk-owned-blob-preparation--2026-10-04)
also removes one second full-body array copy at Blob construction. The SDK keeps
its private snapshot before awaiting MIME detection. Actual Chromium explicit/sniffed
MIME checks match original roots/manifests for selected views, NUL/high bytes and a
distinct multi-chunk tail after immediate caller mutation. Substitute SDK uploads
preserve exact original binary payloads, one-shot handles, lost replies, failed HTTP
replies and budget refusals. The retained native byte verifier is an earlier binary,
bound by its exact hash, not a requalification of the current Rust graph.
Publication/worker and actual Chromium launcher boundaries pass; bundles rebuild,
syntax/changelog/diff checks pass. Original sources, logs and profiles remain under
`.tmp/sdk-owned-blob-01`, with zero live IC/provider requests or paid effects.
This removes an allocation, without claiming measured peak-memory savings.
The SDK change alters executable bundle fingerprints: select the rebuilt matching
tools for new histories and recover retained profiles with their original tools.
Do not replace profile bindings or journals. Consumer adoption and certified asset
registration remain open.

Concurrent worktree changes now select ic-memory 0.25 and lock 0.25.0. They are
preserved separately; this browser batch does not qualify that Rust graph. Frozen
evidence retains its captured Cargo identities.

#### GitHub issue disposition

The latest read-only refresh still finds seven open issues and seventeen comments,
with no newer discussion than the [retained review](../evidence/caffeine-probes/local/2026-10-04-gh-issues-review-01/summary.json).
That review verified remote main at 0.14.3; the current 0.14.5 baseline is established
from local release records and the maintainer's report. No issue/comment is changed.

| Issue | Repository result | Remaining action |
| --- | --- | --- |
| [#1 embedding](https://github.com/dragginzgame/ic-blob-storage/issues/1) | Complete installation requests and library-owned version implemented since 0.10.0; published 0.14.2 contents verified | Ready to close original request; wrapper adoption separate |
| [#2 transfer budgets](https://github.com/dragginzgame/ic-blob-storage/issues/2) | Fix now in released 0.14.3, with zero-effect negatives and exact-fit controls | Ready to close implementation request; no provider spending-cap claim |
| [#3 FIFO inputs](https://github.com/dragginzgame/ic-blob-storage/issues/3) | Shared same-descriptor nonblocking reader and regression in Git 0.10.0; both FIFO boundaries confirmed | Ready to close; CLI source-installed, not a registry distribution |
| [#4 publisher](https://github.com/dragginzgame/ic-blob-storage/issues/4) | Clean released-source recipe now qualifies here; callable driver and exact recovery work | Consumer-owned installation/adoption, certified mapping transaction and deployed bindings |
| [#5 serving/integrity](https://github.com/dragginzgame/ic-blob-storage/issues/5) | Original metadata and real-media delivery pass locally; Miner reports existing model/image loaders and Blob revocation | Production origins/CSP, full inventory, asset transaction, game rendering and deployed serving |
| [#6 lifetime/reuse](https://github.com/dragginzgame/ic-blob-storage/issues/6) | Overlapping references, typed retired-root refusal and bounded lifetime/history implemented | Consumer retention/retirement policy and actual provider deletion/final billing evidence |
| [#7 decoder budgets](https://github.com/dragginzgame/ic-blob-storage/issues/7) | Valid skip/type refusals cover actual standalone boundaries in released source | Header ceiling subsumed; independent work exhaustion unreproduced, not proven impossible |

For #7, the [decoder contract](../service-contract.md#standalone-ingress-decoding)
retains valid controls, unchanged owner/stable state and failed-reinstall rollback.
A dense near-128-KiB manifest decodes under the work ceiling, then receives typed
domain `Limit`. Do not weaken production bounds or introduce test hooks to force
independent work/header exhaustion. Original scoped logs/artifacts remain under
`.tmp/decoder-budgets-01` with their original 0.14.2 identities.

### Remaining product work

- Qualify the released native/browser selection in a consumer-owned prefix/build
  directory, then replace its registry 0.7.0 packaged preparer only upon adoption.
  Preserve explicit binary/trust/history selection and original recovery artifacts.
- Consumer owners must complete certified asset registration, overlapping release
  retention and deliberate exact reference cleanup. Miner's reported model/image
  shell acceptance is not a certified transaction, lease or deployed game.
- Qualify actual origins, MIME/CORS/cache/CSP and access/retention policy. Logical
  release refuses authenticated descriptors but need not revoke a saved public URL.
- Provider deletion/final billing and surviving inventory/freshness remain separate
  gaps. Missing downloads or zero usage counters prove neither deletion nor billing
  cessation. See [service gaps](../service-gaps.md).
- Measure populated service reopen and full browser/worker memory before promising
  million-object operation. Bounded 1/8/32 MiB CDP observations, configurable ceilings
  and 675-row restart evidence are not that qualification. Keep one storage owner
  and its local journals.

### Consumer integration feedback

Canic adoption remains deferred; the [feedback list](https://github.com/dragginzgame/ic-blob-storage/blob/c3a16529d161547987c138b15bf75cac70b54c55/docs/canic-parity.md#integration-feedback)
retains wrapper/lifecycle, one-memory-runtime, current formats and recovery actions.
Siblings are read-only. Record feedback here; no upstream message is authorized.

The old isolated owner stays frozen at 0.6.0 with original stopped history; the
separate live owner was last verified at 0.7.0. Both retain exhausted lifetime
capacity and full provider/billing obligations. Never reset or upgrade them as
source cleanup. Cross-release reinstall requires obligation disposition.

[The probe ledger](../evidence/caffeine-probes/README.md) separates source/offline,
local IC/substitute and deployed observations. [History](history.md) preserves
old handoffs; their forecasts, release labels and dependency graphs never override
this current handoff.
