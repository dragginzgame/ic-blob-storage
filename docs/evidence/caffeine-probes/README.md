# Caffeine probe ledger

This is the entry point for every Caffeine investigation. The maintainer selected
independent qualification on 2026-09-29; direct provider cooperation is unavailable
as a planning assumption. Release acceptance depends on demonstrated behavior and
explicitly reviewed operating limits, not obtaining an answer from Caffeine.
Provider correspondence remains useful evidence if it becomes available.

## Recording rule

Before a probe, record its question, evidence class, source revision or deployment,
actor and owner/project/account bindings, exact requests, request/byte/time/cycle
budgets, stop conditions and expected cleanup. Distinguish zero attached cycles
from unknown provider charges. Paid trials require an explicitly selected isolated
account/installation and an authorized budget; this plan is not that authorization.

After each request, retain its outcome before issuing the next. Record failures,
timeouts, redirects, truncation and inconclusive results as well as success. Never
erase an uncertain operation or replay it merely to get a cleaner result. Record
raw response bytes where safe, hashes, timestamps, decoder/source versions,
observations, interpretation and what the result cannot prove. Secrets, upload
certificates and private account data belong in controlled storage; put a redacted
manifest and the accountable artifact location here, never credentials. Hashes
of locally recorded responses establish file integrity, not provider signatures.

Every run gets a new directory/ID and an index entry below. Completed artifacts
are not edited; corrections or repeated experiments get new linked entries.
An interrupted run with no summary stays incomplete. Update the capability table
and handoff after a material result, and mark conflicting/drifted evidence for
review instead of treating previous observations as permanent guarantees.
"Constant tracking" means this is part of every investigation, not an unattended
poller or recurring paid job. Routine reruns of the same local test need not copy
all logs here; link their maintained test and record material changed findings.

## Current operating decisions and experiments

2026-10-01 managed operator regression: `.tmp/managed-operator-fix-01` records
the pre-run intent, isolated failing reproduction and corrected source/results.
Valid Candid at the 4 KiB transport ceiling can exhaust the separate decoding-work
budget; the old test incorrectly expected scope validation. Updated coverage proves
small skipped input reaches typed Binding, valid over-budget input traps and
4097-byte input rejects at ingress, with service/source stable bytes unchanged.
All four related managed operator journeys and affected strict lint pass.
Production limits and adapters are unchanged; no new Canic action is established.
These use the existing labelled local gateway/Cashier substitute, not deployed
Caffeine, with no provider request, payment or exposure. Earlier evidence remains
unchanged; see [handoff](../../status/current.md).

2026-10-01 published Canic adoption: pre-run intent and separate attempt logs are
retained in `.tmp/published-canic-49-01`. Registry Canic/core/macros 0.110.49 build
the managed artifact without source overrides. The same six focused certificate,
decoder, manifest, lifecycle rollback and Candid cases plus both affected strict
lint lanes pass. Initial sandbox loopback refusal and the stopped test attempt
are retained; the loopback-enabled repeat passes. Package/VCS, lock, source and
artifact hashes are captured. This closes
[CF-01](../../canic-parity.md#integration-feedback); it adds local framework/IC
evidence only. Four provider/recovery prerequisites remain false, with no successful
certificate exposure, provider request, paid effect or deployed Fleet. Full release
validation remains separate. See
[evidence](../core-primitives.md#published-canic-adoption--2026-10-01).

2026-10-01 managed hook adoption: pre-run `.tmp/local-canic-02/intent.txt` records
frozen framework source, zero provider/payment budget, bounded builds/checks,
refusal/decoder/lifecycle questions and instance cleanup. All six focused cases
and both affected strict-lint lanes pass against Canic `32da629d0214bf791541a9b3c1832dbef13ece29`.
The canonical certificate method's plain record matches standalone; actual issue
calls refuse missing prerequisites, wrong actors and restored owners without
exposure. Valid hostile Candid type/header envelopes, malformed/oversized input,
exact manifest boundaries and occupied lifecycle rollback are locally exercised.
Initial testkit-accessor compilation failure and corrected result are retained.
This is local framework/IC evidence, not successful certificate exposure or
deployed Caffeine behavior. Host facts remain false; no qualification override,
provider call, paid effect, live Fleet or sibling edit. AGENTS.md now requires
stored [Canic feedback](../../canic-parity.md#integration-feedback) and delivery
reminders; CF-01 covers normal published dependency adoption. See
[evidence](../core-primitives.md#managed-certificate-and-decoding--2026-10-01).

2026-10-01 local Canic composition: the maintainer selected local development
while Canic release/deployment work continues separately. The pre-run intent in
`.tmp/local-canic-01/intent.txt` bounds offline metadata, one canonical artifact
build, one native harness build and two focused managed PocketIC cases, with no
provider requests or paid effects. Committed Canic `32da629d0214bf791541a9b3c1832dbef13ece29`
is frozen here; sibling sources stay read-only and only copied workspace locks
resolve path overrides. The canonical managed build and existing certificate/
admission/fenced-restore cases pass. Initial copy/output/cache preflight refusals
and corrected results are all retained there alongside source/artifact hashes.
This is local framework/substitute evidence, not adoption evidence for the new
hooks or deployed Caffeine behavior. The four certificate prerequisites remain
unqualified; no paid exposure is authorized. Instances drop; builds/capture stay.
See [handoff](../../status/current.md).

2026-10-01 native/browser handoff intent: emit the existing browser certificate
client's exact binding JSON alongside verified upload inputs, deriving its IDs,
root and opaque Candid permission from the maintained Rust DTO. No new JS schema,
signer, intent journal, certificate or gateway implementation. Execute actual
pinned Caffeine 1.1.2 preparation (maintained repository patch) on a 10 MiB local
file, pass its manifest/binding/body through the native CLI, and consume the
generated binding in the existing browser client with an explicitly in-memory
store that permits setup only. Rebuild preparation from the native snapshot and
original metadata; require the same root/length before any potential future use.
Global and explicit network transports throw, with no certificate issue/recovery
or gateway upload. At most three native invocations, two SDK preparations and
32 MiB local data; preserve no-clobber and corrupt-body failures in fresh
`/tmp/ic-blob-storage-browser-handoff-evidence-01`. Retain outputs/logs/source and
binary/package hashes; no deployed request, payment, attachment, private key file,
account or external cleanup. This is local SDK/native/client evidence, not a
production store, authorization, readiness or provider guarantee.

2026-10-01 native/browser handoff outcome: five affected CLI cases and final
strict CLI all-target lint pass. The first lint found a 102-line run function;
its log is retained, and snapshot publication now has a separate private helper
with the same verification/failure behavior. Actual pinned SDK preparation passes
through native conversion for ten 1 MiB chunks. The existing browser source client
accepts the generated full-width binding/opaque permission in Node, using only
setup/inspection with an in-memory substitute and deliberately non-IC trust bytes.
No signature/certificate/IC behavior is exercised. After the source changes,
snapshot preparation reproduces exactly the root, ten-MiB length and manifest.
Native repeat refuses with every completed file hash unchanged; corrupt source
retains private partial/failure with no body, Candid, browser binding or summary.
Three native invocations return 0/3/3, two SDK preparations and zero network calls.
Fresh [evidence](../core-primitives.md#native-browser-certificate-binding--2026-10-01)
retains commands/results, package/source/artifact bindings and failed/final logs.
No full CI, Chromium/PocketIC rerun, upstream refresh or provider experiment.
Known test identity lived in memory only; no key file or external cleanup.

2026-10-01 shared native download intent: extend the existing managed completion/
download journey with an offline-generated second retain, a deliberately dropped
reply around the actual signed update, exact receipt/status recovery without
redispatch, and another verified download after first-reference release. Generate
and submit the final release, distinguish local logical bytes from physical/billing
liabilities, and inspect both historical receipts/current liveness through fenced
restore. Retain uncertain artifacts unchanged, including a refused same-run submit.
Use fresh `/tmp/ic-blob-storage-shared-download-evidence-01`, at most 40 tenant CLI
plus two verifier invocations, seven bounded ten-byte local source GETs, thirty-second
command deadlines and one proxy retain update. Existing fixture exposure/content
remain substitutes; no qualification override, SDK refresh, deployed provider
request, payment or attachment. Preserve failures/commands/results/source hashes;
drop owned temporary keys/proxy/gateway/progress/instances and retain capture/build.
No external cleanup obligation or new live-effect authority.

2026-10-01 shared native download outcome: the extended existing managed journey
passes in 14.16 seconds with 37 total CLI invocations and seven local ten-byte
source GETs. Ten bytes is the client/expected-body bound; existing fault responses
include zero/nine/eleven bytes (sixty total source content bytes, largest eleven),
with no oversized body published. Generated second-reference retain is dispatched once through the
drop proxy; exact signed-query receipt/status recover success and liveness with
all uncertain artifacts unchanged. Same-run submission refuses. First release
refuses its download while the second delivers the identical verified file.
Final release changes local logical bytes from ten to zero, preserving ten physical
and ten liability bytes. Fenced restore preserves exact retain/release history,
inactive status, all stable bytes and both files; refused mutation/GETs grant no
provider deletion or billing cessation. Targeted managed-harness strict lint,
formatting, diff and draft checks pass; production code/dependencies are unchanged.
No repeated native unit/full CI/release gate or upstream/provider refresh. Fresh
capture is recorded in [evidence](../core-primitives.md#shared-native-reference-downloads--2026-10-01).
Owned keys/proxy/gateway/progress/instances dropped; evidence/build retained and
zero deployed provider requests/payments/attachments or external cleanup.

2026-10-01 offline reference input intent: generate first-reference download/status
files with the verified upload snapshot, and explicit retain/release command plus
read files from the exact saved permission. Reuse maintained core validators and
Candid; no allocation, signer, dispatch, expiry renewal or retry authority. Run
both existing signed setup journeys (32 CLI/eight local updates/16 MiB service
traffic each, 10 MiB standalone and ten-byte managed snapshots) with generated
unconfirmed-reference requests and zero source GETs. Extend the existing managed
ten-byte download journey to consume generated reference inputs and perform signed
release/receipt/status, including fenced restore (24 tenant CLI plus two verifier
invocations, six bounded local source GETs, thirty-second command deadlines).
Existing exposure/content remain labelled local substitutes; qualification facts
stay false. Fresh `/tmp/ic-blob-storage-reference-inputs-evidence-01` retains
commands, results, failures and final source/artifact bindings. No deployed provider
request, SDK refresh, payment or attachment. Drop owned keys/servers/progress;
retain evidence/build, with no external cleanup.
Three additional offline binary checks use a copied retained abc/text permission:
full-width retain inputs (exit 0), repeat refusal
(exit 3) and noncanonical reference refusal before claim (exit 2). No network,
body read, service/provider call or cleanup resource.

2026-10-01 offline reference input outcome: all 57 CLI and three probe-tool units,
native binary build and final strict CLI/harness lint pass. The first lint attempt
rejected constant `chunks_exact` in a fixture helper; its log is retained and the
maintained `as_chunks` form passes. Standalone and pinned managed setup journeys
pass in 7.61/8.47 seconds, 25 CLI invocations each, with generated first-reference
status/download refusals and zero source GETs. The existing managed download
journey passes in 11.92 seconds, 21 total invocations and six local substitute
GETs: generated inputs drive download, signed release, exact receipt and current
status; release prevents another fetch and fenced restore preserves inactive
status, receipt and all stable bytes. Logical release establishes neither provider
deletion nor billing cessation. Three actual offline binary smoke outcomes are
0/3/2. Fresh capture and hashes are recorded in
[evidence](../core-primitives.md#offline-reference-and-download-inputs--2026-10-01).
Owned temporary keys/servers/progress/instances dropped; no external cleanup,
deployed provider call, payment, attachment, SDK refresh or new qualification fact.

2026-10-01 verified upload snapshot outcome: all 54 CLI and three probe-tool units,
native binary build and strict CLI/harness lint pass. Existing standalone 10 MiB
and pinned managed ten-byte journeys now consume actual generated Candid and
verified snapshots and pass in 7.19/8.86 seconds (23 CLI invocations each). Source
edits leave snapshots unchanged; signed service-manifest verification succeeds.
Lost/pending recovery, cancellation and fenced accounting remain intact; all four
certificate blockers remain false host prerequisites. No exposure/completion or
provider qualification is supplied by tests. Three offline binary smoke results
retain corrupt output, unchanged partial evidence on repaired-source refusal and
a successful fresh snapshot. Capture `/tmp/ic-blob-storage-upload-snapshot-evidence-01`
contains 230 manifested files, seven logs and 22 final source/artifact bindings.
Zero provider requests/GETs/payments/attachments. Manifest JSON is a local Rust
fixture substitute in upstream format; no SDK/upstream/provider refresh. Owned
servers/proxies/progress/instances/PEMs dropped, builds and evidence retained;
no external cleanup. See
[evidence](../core-primitives.md#verified-native-upload-snapshots--2026-10-01).

2026-10-01 verified upload snapshot intent: extend the unreleased offline
`upload-inputs` contract to require a regular body file, stream it through the
maintained Caffeine root verifier and save those same buffers. Only complete
EOF/root verification and file sync may publish `body.bin` and the Candid inputs;
failed output stays private with no usable summary. Reuse the same local-file
verifier in signed `verify-upload`; no second hashing/tree/chunk/upload SDK.
Run the existing signed standalone (10 MiB) and pinned managed (ten-byte) setup
journeys using actual generated inputs, then verify the snapshot against the
service manifest after deliberately changing its original source. Manifest JSON
is an explicit local upstream-format substitute from the maintained Rust fixture,
not a fresh SDK/provider observation. Per host: at most 32 native invocations,
eight local service updates, 16 MiB service traffic, one bounded body snapshot,
thirty-second command deadlines and zero provider requests/GETs/payments/cycles.
Use fresh `/tmp/ic-blob-storage-upload-snapshot-evidence-01` children; retain
failures/logs/source bindings, drop owned temporary keys/proxies/servers and retain
evidence/build. Qualification facts remain false; no fixture exposure/completion.
Three additional offline smoke invocations use the retained independent abc/text
vector: corrupt three-byte body, repaired-source refusal of the existing partial
run and successful fresh snapshot. No service/provider requests; keep every result
and failed partial, with no keys or external cleanup.

2026-10-01 offline upload-input conversion outcome: four new native cases pass;
all 51 CLI and three existing probe-tool units, strict CLI all-target lint and
binary build pass. Three actual binary invocations emit exact Candid, preserve
all output hashes on refused repeat and reject changed metadata before claim.
Fresh `/tmp/ic-blob-storage-upload-inputs-evidence-01` retains 30 manifested files
and fourteen final source/artifact bindings, including failed compile/lint and
sandbox attempts. This is local conversion/vector evidence, not an upstream SDK
execution or provider qualification. Zero smoke service/provider requests or paid
effects; existing native regressions use owned local HTTP substitutes only. No key
or external cleanup obligation. See
[evidence](../core-primitives.md#offline-native-upload-inputs--2026-10-01).

2026-10-01 offline upload-input conversion intent: reuse the maintained Caffeine
prepared-manifest decoder and shared service validators; do not introduce hashing,
chunking or transfer logic. Qualify the native binary with the existing independent
Caffeine 1.1.2 abc/text vector, explicit full-width IDs, preserved input/output
hashes, no-clobber repeat and inconsistent-manifest refusal before output claim.
Three smoke invocations, inputs capped at 4 KiB binding/256 KiB manifest and ten
content bytes; zero service/provider requests, payments or attachments. Native
regressions use their existing local HTTP substitutes only. Retain each failed
compile/lint/sandbox attempt and final logs in a fresh capture; no provider behavior
or qualification refresh is implied.

2026-10-01 signed upload setup outcome: actual native tenant admission, uploader
preparation, exact recovery and tenant cancellation pass through standalone and
managed handlers in 6.66/7.74 seconds. Each uses twenty-one CLI invocations under
its pre-effect plan, preserving original uncertain artifacts through query recovery,
rejecting corrupt declarations before claim, releasing unexposed bytes and retaining
history under same-release restore fences. No fixture exposure/completion or
production host evidence override. All 47 native units, four affected core cases,
current CLI/host builds and final strict affected Clippy pass; targeted checks only.

Captures `/tmp/ic-blob-storage-upload-setup-evidence-01` and `-02` retain 92/111
manifested files. First managed enrollment failed before the native client because
the test used a hard-coded operator; then-current hashes/log/result are retained in
01 alongside successful standalone records. Corrected managed uses fresh 02 with
all development logs, exact commands and final input bindings. Owned local proxies,
gateways/progress/instances/temporary PEM dropped; build/evidence retained. Zero
provider GETs, deployed requests, payments or attachments and no external cleanup.
Read-only local Canic HEAD `70a0bc9a435a7695d8931a7c66c745576e678597` has the same
relevant macro files as pinned 0.110.48, retaining both generic issuance/decoder
gaps. No upstream/registry/provider refresh or sibling mutation. See
[evidence](../core-primitives.md#signed-native-upload-setup--2026-10-01).

2026-10-01 signed upload setup intent: make local admission/preparation/cancellation
usable through the native client before qualified certificate issuance. Reuse
maintained DTOs, handlers, pure validators and exact recovery decoders, with one
saved signed update per fresh run and no automatic polling/retry. Exercise the
actual standalone and supported managed service over loopback PocketIC: distinct
tenant/uploader identities, exact full-width permission/manifest, lost admission,
pending preparation, exact read-only recovery, cancellation and fenced restoration.
Per adapter budget: at most 32 CLI invocations, eight local updates, 16 MiB service
traffic, thirty seconds per command; zero provider GETs/requests/payments/attachments.
Preserve requests/results/public trust/failures and original uncertain artifacts;
drop owned proxies/gateways/progress/instances/temporary PEM, retain build/evidence.
No fixture exposure/completion hook, provider source refresh or paid trial.

2026-10-01 tenant download outcome: native tenant output uses the maintained
replicated descriptor/decoder, canonical provider path and shared streaming root
verifier. All 45 native units pass (0.62 seconds), actual signed managed
completion/download/release/restore passes (12.80 seconds), standalone
unconfirmed/restored refusal passes (5.15 seconds), and all three existing managed
verifier receipt/recovery regressions pass (12.06 seconds). Wrong scope/signer,
corrupt/short/long/redirect replies and run reuse never yield a new verified file;
released/fenced descriptors issue no GET. Download/refusal phases preserve complete
service stable bytes. This is local platform and labelled exposure/content evidence,
not a real upload, certificate, deployed availability or operational recovery claim.

Fresh captures `/tmp/ic-blob-storage-tenant-download-evidence-01`, `-02`, `-03`
retain twelve, thirty-one and 156 manifested files respectively. First failed
managed setup repeated exposure already performed by the fixture (zero GETs);
second asserted the wrong submission label after accepted completion (one GET).
Both failed attempts retain their then-current input hashes and logs. Final managed
uses fourteen CLI invocations/six GETs, standalone two/zero within their recorded
plans. Existing verifier regressions add three local GETs with temporary raw
captures and retained validation logs. Final source/artifact bindings and all
development failures remain in 03; manifests and hashes are recorded in
[evidence](../core-primitives.md#verified-native-tenant-downloads--2026-10-01).
Owned listeners/gateways/progress/instances/temporary PEM dropped, evidence/build
artifacts retained; zero deployed Caffeine requests, payments or provider cycle
attachments and no external cleanup. No provider/upstream source refresh occurred.
Next work is the real upload path's existing qualification/framework gates, not
an assumed free read or an unapproved live trial.

2026-10-01 tenant download implementation intent: prioritize a usable byte path.
Reuse the maintained replicated descriptor, canonical provider URL and streaming
Caffeine root verification rather than create another provider client. Add one
signed tenant descriptor update followed only on exact authenticated success by
one explicitly selected origin GET, with original headers/length/root, no redirect,
retry or content decoding. Save intent before each effect; retain incomplete bytes
privately and publish a usable file only after complete EOF/root verification.
Exercise native socket corruption/truncation/oversize/redirect/encoding cuts and
actual signed managed completion-to-download, release and restored refusal over
the existing labelled ten-byte exposure/content substitute; standalone can exercise
real unconfirmed/fenced refusal until qualified issuance exists. Per local journey:
at most 24 CLI invocations, six local source GETs, 10 MiB reply/download traffic,
thirty seconds per request; zero deployed provider requests/payments/attachments.
Retain fresh requests/results/public trust/raw local source/failure artifacts; clean
owned sockets/gateways/progress/instances/temporary PEM, retain build artifacts.
No upstream/provider source refresh is implied; unchanged maintained provider path
is locally exercised, not newly qualified. Production issuance/provider/recovery
prerequisites stay explicit. This does not authorize paid trials or sibling edits.

2026-10-01 passive funding assessment outcome: shared synchronous query and
signed native command report exact current local limits and mandatory missing
qualification/recovery/account-activity/spendability evidence. No reservation,
funding mutation or provider call is exposed. All 28 affected core funding units
and 42 native units pass; synthetic accepted/uncertain history is labelled model
evidence. Both actual signed IC journeys pass (standalone 6.19 seconds, managed
10.06 seconds) with 21 CLI invocations each, under the recorded 24-invocation
budget. Cashier stays stopped; occupied upload/funding/read/gateway facts and
complete service/source stable bytes remain unchanged during refusals and after
same-release fenced restore. Full-width proposals/optional target, scope/identity,
untrusted replies and rejected caller qualification flags are covered. Current
Candid types/modes, both existing signed status regressions, native/standalone/
supported managed builds and final strict affected/isolated Clippy pass.

The fresh 110-file capture at
`/tmp/ic-blob-storage-funding-assessment-evidence-01` retains 44 files per adapter
and 22 validation files, including all failed intermediate lint/compile attempts,
input hashes and exact commands. Immutable manifest SHA-256:
`f21accdda21a2d987624aea8e6515752af65855b919270f1168c179327c300fb`.
Neither failed identity-constructor compile attempt started a signed journey.
No PEM remains; owned local gateways/progress/instances/temporary inputs were
cleaned up and capture/build artifacts retained. Zero Cashier queries, deployed
Caffeine requests, payments or provider cycle attachments; no external cleanup.
No full CI/release validation, dependency/allocator change or upstream refresh.
Missing trusted production evidence acquisition, qualified dispatch and recovery
remain open independently of provider balances or local empty history. See
[assessment evidence](../core-primitives.md#passive-funding-preparation-assessment--2026-10-01).

2026-10-01 passive funding assessment intent: expose current preparation-policy
blockers through the same synchronous shared handler in standalone and managed
adapters, then inspect with the signed native client. Authenticate exact operator/
service/namespace/Cashier/payer and proposed operation/offer/optional target.
Provider qualification, recovery, complete account activity and spendability stay
unestablished by these hosts; do not infer them from balances, status, local clear
history or caller assertions. No reservation, dispatch, provider query or payment.
Exercise native local journal occupancy/retained identity/capacity/accounting and
actual signed IC scope/trust/fenced restoration; preserve occupied unrelated
owners and complete stable bytes. Per adapter journey: at most twenty-four CLI
queries, thirty-second deadlines, 256 KiB HTTP/4 KiB service replies; zero Cashier/
deployed Caffeine requests, payments or provider cycle attachments. Record plans,
requests/results/failures before advancing; retain fresh captures/public trust,
clean owned gateway/progress/instances/temporary PEM and keep build artifacts.
This extends the existing unreleased gateway batch after 0.4.12. It does not
authorize provider trials, sibling changes, operational unfencing or qualification.

2026-10-01 signed native gateway controls outcome: all three commands retain
canonical request/signed intent before one service update. Actual signed shared
journeys pass through standalone (13.77 seconds) and managed (14.70 seconds),
with thirty-nine CLI invocations and six local Cashier queries each, within the
recorded budgets. Exact cancellation/absence revocation acknowledgments stay
separate from dropped/pending results; repeat/partial claims do not send again.
Current signed status shows membership/pending work without settling original
uncertainty or authorizing retry. Invalid/oversized provider replies preserve
pending identity; busy/stale cancellation refuse. Unrelated occupied owners and
pending history survive same-release restoration; all controls refuse the fence
and full service/source memories stay unchanged during refusals. Actual wrong
signer and namespace/Cashier/payer refuse; malformed decisions/identity mismatch
fail before a durable claim.

Thirty-nine native units, CLI build/final strict CLI Clippy, both new signed
journeys, both existing standalone sync cases and final strict harness Clippy
pass. The initial harness lint rejected a long journey helper; it was split and
the failure is retained. Fresh `/tmp/ic-blob-storage-gateway-controls-evidence-01`
retains 204 files per adapter and validation inputs/logs/commands (420 files total)
with immutable SHA256SUMS. Signed raw requests, bounded Candid replies, explicit
mode intentions and command/result/status/outcome JSON remain; public root trust
is retained and PEM identities stay temporary. Owned proxy/gateway/progress/
instances/temporary inputs are closed or dropped, with no external cleanup.
No deployed Caffeine request, payment or attached provider cycles occurred; all
source replies are query-only substitutes, not provider qualification. Existing
released service artifacts were reused; no new schema/endpoint, allocator,
dependency, version mutation or full CI/release gate. See
[gateway control evidence](../core-primitives.md#signed-native-gateway-controls--2026-10-01).
Framework/provider/provenance/recovery and funding admission remain independent
gates; no upstream probe, message or sibling edit was made.

2026-10-01 signed native gateway controls intent: compose sync, exact pending-sync
cancellation and local gateway revocation through the existing shared service
handlers. Claim a fresh private directory and save canonical request, signed update
and scoped intent before one dispatch; retain acknowledged/pending/uncertain/typed
refusal separately, with no polling, retry or locally predicted sync identity.
Use signed status only as current inspection, not a retained revocation receipt or
permission to repeat an uncertain action. Run actual signed standalone and managed
journeys over the existing query-only Cashier substitute; exercise lost/pending
acknowledgments, partial claims, invalid replies, scope/caller refusals and occupied
same-release restore. Per journey: at most forty CLI invocations, twenty local
provider queries, thirty-second client deadlines, 256 KiB HTTP/4 KiB service replies
and bounded existing provider decoding. Zero deployed Caffeine requests, payments
or attached provider cycles. Retain plans/requests/results/failures in fresh capture
children, stop on unexpected requests/effects and close owned sockets/gateway/
progress/instances/temporary PEM. Public roots and evidence/build artifacts remain.
This continuation starts from released 0.4.12, with no provider source refresh or
qualification inferred from the local substitutes. No new deployment/paid authority.

2026-10-01 signed native account inspection outcome: actual native balance and
relationship journeys pass through standalone (7.03 seconds) and managed
(10.82 seconds) adapters, fifteen CLI invocations per journey within their
pre-effect budgets. Exact operator/account binding, arbitrary-width signed/Nat
relationship fields and independent reported balances survive. Missing accounts,
no reported relationship and provider errors remain observations. Invalid/oversized
source replies, signer/scope/trust refusals and fenced same-release restore retain
complete stable state without credit/payment/retry authority. Update trust failure
can follow execution and does not prove an unsent read. The first managed attempt
refused setup because the fixture helper assumed the old operator; it remains
retained, and helpers now bind the installed operator explicitly.

The transport review found ic-agent 0.49.2's implicit HTTP 429/503 retries despite
TCP retries zero. Public middleware now delegates to the existing no-retry client.
An owned socket counter regression proves one service query/update under each
backpressure response; the first fixture incorrectly counted separate trust-key
reads and its failure is retained. Thirty-five native units, CLI build/final strict
Clippy, both new signed journeys, existing standalone/managed signed clients and
occupied managed operator regression, and final strict harness Clippy pass.
Fresh roots `/tmp/ic-blob-storage-native-account-evidence-01` (62 files including
44 standalone captures and validation) and `-02` (44 managed captures) have
immutable manifests. All failed lint/unit/managed attempts are retained. Owned
instances/gateway/progress/sockets and temporary PEM were cleaned up; reports and
build artifacts remain. All Cashier reports are local query-only substitutes;
zero deployed requests/payments/provider cycle attachments and no external cleanup.
See [signed account evidence](../core-primitives.md#signed-native-account-inspection-and-bounded-transport--2026-10-01).
No full CI/release validation, publication, schema/version/dependency or allocator
change occurred. Complete account activity, credit and operational restart remain
unqualified.

2026-10-01 managed certificate framework review outcome: upstream main/HEAD and
peeled v0.110.48 resolve to `8d37c74c9a4457b9e2bd47ee883f98fd2889d63b`.
Four immutable source fetches succeeded; access/expansion/parser files exactly
match cached pinned registry 0.110.48. Source still lacks supported plain-record
Fleet refusal and work/type/header decoder hooks. Normal framework dispatch remains
intact; no workaround, upstream edit/message or redundant failed build follows.
The four metadata requests include successful GitHub discovery/refs, an unavailable
web registry opening and explicit registry HTTP 403/empty body; latest registry
version is unverified. Every failure/source response/header/stderr is retained in
`/tmp/ic-blob-storage-certificate-framework-review-01` with eighteen-file manifest
and exact comparison bindings. Read-only sibling revision/dirty state is recorded;
no sibling was altered. Zero Caffeine requests, payments, attached provider cycles
or cleanup obligations. See [source review](../core-primitives.md#managed-certificate-framework-review--2026-10-01).
Framework support remains separate from provider/provenance/recovery qualification.

2026-10-01 signed native account inspection intent: add one explicitly selected
balance or payment-relationship observation through the existing shared
blob_inspect_account service update. Preserve operator/service/namespace/Cashier/
payer binding, signed transport/root trust and bounded service reply validation;
do not recreate provider requests or send attached cycles. Native tooling submits
one read operation and waits only for that exact IC request, with a thirty-second
deadline and no redispatch/retry. Report balances, absence and provider errors as
observations, never credit, spendability or payment/retry authority. Exercise the
actual standalone and managed artifacts over their existing local query-only
source with distinct identities and explicit fixture account configuration.
Per journey: at most twenty-four CLI invocations and twenty-four local provider
queries, 256 KiB transport/4 KiB account replies, zero payment/attached provider
cycles/deployed Caffeine requests. Retain fresh canonical requests/responses and
refusals, including malformed/oversized replies and fenced restoration. Close
owned gateway/progress/instances/temporary identities; retain reports/artifacts.

2026-10-01 managed certificate framework review intent: inspect the pinned public
Canic endpoint contract, the read-only local checkout and current registry/upstream
metadata for supported plain-record rejection and bounded decoder hooks. Preserve
the Caffeine reply format, normal Fleet/activation/preflight/instrumentation and
single synchronous commit/reply; no internal-method classification or copied
framework dispatch. Source/registry observations are distinct from local IC and
deployed-provider evidence. At most four metadata requests and four source/archive
fetches, thirty-second deadlines and 2 MiB per response (source archives 4 MiB),
zero deployed Caffeine requests/attached provider cycles/payments. Retain exact
responses, revisions/hashes and failures in a fresh local review directory. Sibling
repositories remain read-only; no dependency change or paid trial follows merely
from finding a newer release. If support is absent, specify the precise supported
framework gate and continue useful work available in this repository.

2026-10-01 unresolved application restore outcome: all four new interruption cases
and both existing application cases pass in 57.45 seconds. Committed retain/release
effects remain distinct from absent application acknowledgments through either
upgrade order and repeated same-release restoration. Exact asset/payload/cleanup
bindings, tombstones, service receipts, liveness and ten logical/physical/liability
bytes survive. Fenced application mutation/recovery calls refuse without resolving
uncertainty; controller disclosure/recovery refuses. Tenant-scoped service queries
remain passive and both complete stable memories match before/after inspection
and refusals. Final strict affected Clippy passes; the initial missing-trait compile
failure is retained. The fresh 83-file report preserves all six cases and is bound
in [unresolved-outbox evidence](../core-primitives.md#managed-unresolved-outbox-restoration--2026-10-01).
Local instances/temporary resources were dropped; reports/build artifacts remain.
No provider GET/payment/deployed request, external object or cleanup obligation
was created. Application/exposure/completion are local substitutes; this closes
the local restore test gap, not operational restart, stale-snapshot activation,
production consumer serving or deployed provider qualification.

2026-10-01 unresolved application restore intent after 0.4.11: extend the existing
managed application substitute with committed retain/release effects whose callback
traps before its acknowledgment can persist. Keep a published fresh asset and a
cancelled reuse asset's exact unresolved outbox intent. Upgrade application first
and service first in separate fresh cases, then repeat same-release upgrades of
both owners. Verify exact asset/tombstone/pending histories, original service
receipts/current reference liveness, denied callers and all-owner fences. Refuse
registration, admission, cleanup, recovery and new uses without clearing fences
or fabricating acknowledgment. Passive service inspection uses explicit tenant
identity in PocketIC; it is not recovery performed by the fenced application.
The existing probe and configured ten-byte exposure/completion are local substitutes.
Per case: at most sixty-four explicit application/service updates, sixty-four
inspection queries, thirty-second client deadlines and 4 KiB replies; no hold,
provider GET, deployed request, attached provider cycles or payment. Retain exact
Candid in fresh report children and keep each attempt/log separately. Drop owned
instances/temporary resources and retain reports/build artifacts. No operational
restart, production Toko acceptance or external journal is introduced.

2026-10-01 standalone reference outcome: all three acknowledged/dropped/pending
journeys pass in 35.21 seconds against the actual standalone artifact. Four signed
intents per journey retain exact original commands and typed Unknown/Unconfirmed/
Fenced refusals. Refused receipt inspection leaves lost/pending outcomes unresolved;
repeat submission never resends. All four owner fences and the 10 MiB reservation/
physical/liability accounting survive restoration and passive inspection. Final
strict affected Clippy and artifact builds pass; earlier lint failures remain.
The fresh 82-file report contains twelve signed intents and raw replies, bound in
[standalone evidence](../core-primitives.md#standalone-signed-reference-submission-and-refused-inspection--2026-10-01).
Temporary identities and owned sockets/gateway/progress/instances were cleaned up;
reports/build artifacts remain. No provider GET/payment/deployed request or external
object was created. Production successful completion remains disabled here.

2026-10-01 managed application outcome: the initial outbox case passes in 12.14
seconds; both final outbox/publication-race cases pass in 20.06 seconds. Actual
tenant calls use maintained admission/reference/descriptor clients, exact original
intents and receipt recovery after committed-effect callback traps. Cancellation
wins before the delayed callback; new uses refuse and the other asset stays live
until its own release. Cleanup at capacity during suspension retains physical/
billing liabilities. A pending release recovers passively under a service mutation
fence; subsequent consumer restore preserves its own fence/history. Final strict
affected harness Clippy, consumer build and exact managed Candid comparison pass;
the initial lint failure remains. Fresh 15/29-file sets preserve both runs with
raw canonical Candid and immutable manifests in
[application evidence](../core-primitives.md#managed-application-outbox-and-publication-race--2026-10-01).
Local instances/temporary resources were dropped; no provider GET/payment/deployed
request, external object or outstanding external cleanup was created. This remains
a local application/exposure/completion substitute, not Toko, operational restart
or provider deletion/billing qualification. Next restore occupied application
state while its outbox is unresolved; no fence-clearing authority is inferred.

2026-10-01 managed application/outbox intent: install the existing bounded consumer
probe as an actual canister tenant beside the public Canic fixture. Reuse its
canonical replicated admission/reference/descriptor clients and its own local
durable asset/outbox record; add no production component or external journal.
Explicitly configure the service with the probe's maintained fixture project.
One fresh asset and one reuse asset share a ten-byte object over labelled local
exposure/completion, with distinct service, tenant, operator, uploader and verifier.
At most sixty-four explicit application/service updates and sixty-four inspection queries, thirty-second client
deadlines and 4 KiB client reply bounds; zero provider GETs, attached provider
cycles/payments or deployed requests. Check retained admission/retain/release
intents, callback traps, original receipt recovery without redispatch, atomic
tombstone/outbox, use guards, reserved cleanup capacity during suspension and
separate physical/billing liabilities. Restore both consumer and service within
the current release, preserving their fences/history without operational restart.
Save plans/exact Candid in fresh capture directories; retain any failed attempt.
Drop owned instances/temporary resources; retain reports and build artifacts.
The consumer is a local application substitute, not Toko or production acceptance.

2026-10-01 managed application publication-race intent: extend the same local
probe/managed-service journey with the probe's bounded post-descriptor hold.
Cancel the reuse asset while registration waits, release its reference, then
resume the delayed callback and require it to refuse publication. A new use of
the tombstoned asset must refuse. The fresh asset/reference remains live until
its own cancellation/release; subsequent restore preserves the race outcome.
Same local targets/sixty-four explicit application/service update and inspection
query budgets, plus the maintained hold's bound of 128 management `raw_rand`
calls; ten declared bytes,
zero provider GETs/attached cycles/payments/deployed requests. Capture this new
case and reruns in fresh children/parents; retain the first successful outbox
report unchanged. This probes application interleaving, not provider deletion.

2026-10-01 standalone reference intent: run native one-shot tenant submission
against the maintained standalone Wasm with real signed IC updates. Production
issuance/exposure remains disabled; do not inject a confirmed object or add a
completion hook. Use unknown, admitted/prepared and same-release fenced states.
Per journey: at most eight signed updates and forty CLI invocations, thirty-second
client deadlines, 256 KiB transport/4 KiB reference reply bounds, one owned
10 MiB local manifest/reservation, zero provider GETs/attached cycles/payments/
deployed requests. The shared fault proxy forwards one update, passing its typed
refusal or dropping/replacing its acknowledgment. Preserve pending/uncertain
outcomes when receipt inspection itself refuses; neither unknown/unconfirmed nor
an empty/partial claim authorizes resend. Retain all intent/result evidence in
fresh report children. Inspect all restore fences and complete stable bytes.
Stop/drop owned proxy/gateway/progress/instances and temporary identities, retaining
reports/build artifacts. This is local production-refusal evidence; successful
retain/release and provider guarantees remain separate acceptance work.

2026-09-30 managed tenant reference intent: add one-shot native
`submit-reference` over the maintained service contract and exercise signed
tenant retain/release beside signed verifier completion in the public Canic
fixture. Use distinct fixed test signers for tenant and verifier, one labelled
local exposure and one owned ten-byte source GET. Per journey: at most ten signed
updates and sixty CLI invocations, thirty-second client deadlines, 256 KiB
transport/4 KiB reference reply bounds, zero attached provider cycles/payments
and zero deployed Caffeine requests. Retain exact request/signed intent before
dispatch; inspect original receipts after acknowledged/dropped/pending replies.
Check partial/existing claim, caller/scope refusal, stored transition failure,
cleanup at capacity during suspension, historical success versus current liveness,
remaining physical/billing liabilities and passive fenced restoration. Reports
get fresh directories; failed/inconclusive runs remain. Stop/drop owned source,
proxy, gateway/progress and instances; retain reports and build artifacts. This
does not establish a production consumer outbox or deployed provider deletion.

2026-09-30 managed tenant reference outcome: all three new journeys pass in
33.45 seconds; all seven affected managed signed-client cases then pass in
73.13 seconds. Each tenant journey uses one local ten-byte GET and eight saved
signed update intents, including verifier completion, explicit historical replay
and inactive/fenced refusals. The fault proxy forwards exactly one reference
update; original receipt inspection resolves dropped/pending replies without
resend. Empty/existing claims and foreign scope/signers refuse. Stored failures
remain distinct; releases at capacity during suspension and replay retain dead
references and physical/billing liabilities. Same-release restore preserves
historical receipts, liveness and all-owner fences with stable memory unchanged
during reads/refusal. All twenty-nine native CLI unit cases, strict affected
Clippy/build, three existing verifier cases and one existing signed receipt/status
case pass. Initial lint and sandbox-socket failures remain separate retained logs.
The fresh 277-file report manifest and bound commands/artifacts are in
[tenant reference evidence](../core-primitives.md#managed-signed-tenant-reference-submission-and-cleanup--2026-09-30).
Signed request/argument/trust hashes and scope/budget records validate; PEM keys
are temporary. Owned sources, proxies, gateway/progress and instances were closed/
dropped; reports/build artifacts remain. No deployed provider request, payment,
external object or outstanding external cleanup was created. Production consumer
coordination and real deletion/billing cessation remain open.

2026-09-30 managed verifier intent: exercise maintained `observe-upload`,
`submit-attestation` and `upload-attestation` against the public Canic fixture.
Install the fixed signing identity solely as verifier, with distinct tenant,
uploader and operator. An operator-only labelled fixture exposure hook prepares
the phase; it issues no certificate and supplies no production qualification.
Reuse the existing native HTTP fault proxy and an owned loopback ten-byte source.
Per journey: at most one source GET, one signed update and sixteen CLI invocations;
30-second client deadlines, 256 KiB service replies, ten-byte content budget,
zero attached provider cycles/payments and no deployed Caffeine contact. Check
unexposed/foreign-verifier refusal before GET, observation persistence before
effects, acknowledged/dropped/pending replies, exact receipt recovery without
resend and fenced same-release restoration. Save each initial outcome in a fresh
local report directory; never reuse an uncertain submission. Stop/drop sockets,
proxy, gateway/progress and owned instances, retaining reports and build artifacts.

2026-09-30 managed verifier outcome: three new cases pass initially in 33.37
seconds; after completing raw response/tamper/recovery capture, all four managed
signed-client cases pass in 41.86 seconds. Each verifier journey issues one local
source GET and one signed IC update (zero deployed provider requests/payments),
then resolves acknowledged/dropped/pending replies by exact receipt inspection
without resend. Foreign verifier, damaged observation and fenced new fetches
refuse. The shared proxy/source extraction also passes all three existing native
fixture cases in 12.63 seconds. Retained evidence sets 01/02 contain 88/103 files,
bound by immutable SHA256SUMS manifests; the first set predates raw response and
additional refusal/recovery captures. No record was overwritten. Logs, failed
compile/lint attempts, complete bindings/budgets and capture hashes are in
[managed verifier evidence](../core-primitives.md#managed-signed-verifier-observation-submission-and-recovery--2026-09-30).
Owned local resources were closed/dropped; there are no new external objects or
cleanup obligations. Local exposure/bytes and trusted metadata completion do not
qualify deployed provider behavior, future retention, billing cessation,
production verifier provenance or concrete consumer/outbox acceptance.

2026-09-30 managed signed-client intent: run the maintained native CLI against
the public Canic application-only PocketIC fixture. Bind the fixed test signing
identity explicitly as operator and uploader; pin its undelegated application
subnet key through the independently owned local control API. Query status,
funding/upload history and certificate assessment beside one ten-byte reservation;
verify local file bytes against its signed original manifest and refuse changed,
short or long files, without claiming provider availability or completion;
check identity/scope/trust refusal, exact saved permission and all-owner fenced
same-release restore. At most 32 signed CLI invocations, each bounded by the
maintained 30-second client timeout and 256 KiB response budget. No certificate
issuance, provider requests, payments or deployed Caffeine contact; attached
provider cycles are zero. Stop progress/drop instances and temporary test files;
retain every attempt in core evidence. Local signing is not production Fleet or
provider qualification.

2026-09-30 managed signed-client outcome: the final managed suite passes all ten
cases in 114.87 seconds, and the existing signed standalone status/history case
passes in 6.24 seconds. The new journey invokes the maintained CLI 22 times,
including refusals before transport, against one prepared ten-byte reservation.
Pinned local subnet trust verifies queries without an NNS or a signature bypass.
Wrong identity/scope/trust, changed permission and corrupt/short/long files refuse;
historical inventory/byte checks remain passive after all-owner fenced restore.
No provider request, exposure, completion or payment is produced. Three failed
fixture/URL attempts and an earlier lint failure remain distinct captures; the
subsequent successful narrower run predates the added byte cases. Full commands,
artifact/source/log hashes and limitations are in
[managed signed-client evidence](../core-primitives.md#managed-signed-client-and-local-byte-verification--2026-09-30).
Owned progress/gateway/instances and temporary input files were cleaned up; there
are no new provider objects or external cleanup obligations. Production Fleet,
verifier submission and concrete consumer/deployed-provider acceptance remain open.

2026-09-30 managed adapter intent after 0.4.10: extend the existing Canic
composition suite with certificate refusal and gateway/account endpoints over
shared workflows. Use one owned local managed operator installation, one prepared
ten-byte reservation and the existing query-only Cashier substitute. Provision the
selected source through the fresh empty fixture's authenticated application carrier
before creating any tenant or obligation. At most eight gateway-list and eight
account reads per journey, 30-second call timeout, 64 KiB gateway and 4 KiB account
reply budgets, no attached cycles, payments or deployed provider contact. Check
operator/scope denial, exact failed-sync cancellation, passive account observations
and retained pending work/all-owner fences after same-release upgrade. Stop/drop
owned instances; retain every attempt in the managed core evidence. This is generic
adapter composition over existing labelled source modes, not new deployed Caffeine
qualification or authority to repeat an uncertain paid effect.

2026-09-30 managed adapter outcome: all nine Canic composition cases pass in
107.47 seconds. The new operator journey executes five gateway-list and five
account queries against the existing owned query-only substitute, each with zero
attached cycles. It preserves failed-sync identity, exact cancellation and passive
reports beside a prepared ten-byte reservation, then restores all owners fenced.
Certificate assessment reports existing blockers without exposure. A compiler/
pinned-source finding changed the implementation plan: public Canic default Fleet
guards require Result and cannot preserve Caffeine's plain certificate reply.
The attempted issuance adapter was removed; no internal-endpoint workaround was
introduced. Retained attempts, commands, hashes and limits are in
[managed operator evidence](../core-primitives.md#managed-certificate-assessment-and-operator-queries--2026-09-30).
This records generic managed composition over maintained local wire substitutes,
not new deployed Caffeine behavior. Owned instances were dropped; no provider
objects, balance changes or outstanding external cleanup were created.

2026-09-30 standalone lifecycle/acceptance review intent: inspect maintained
host/store restoration and reconcile acceptance/parity claims with current local
evidence. One owned standalone PocketIC canister, one admitted/prepared 10 MiB
manifest, one stop/start, one rejected configuration-bearing upgrade and two
same-release empty-argument upgrades. Compare complete installation, retained
admission/manifest and all four owner fences; after restoration attempt only
manifest preparation, gateway revocation and account inspection, expecting typed
Fenced refusals before any source call. No snapshot load, provider request,
funding, deployment or unfence. Drop the owned local instance; record material
outcomes and any failure as standalone-lifecycle-01. Local current-state evidence
does not qualify active recovery or Canic parity.

2026-09-30 native reference inspection intent: exercise separate receipt and
current-status queries on one owned standalone installation and one populated
durable storage fixture through signed local PocketIC HTTP. Fixed test tenant
identity, explicit root DER, exact original saved Candid requests; at most 32
native invocations, each one query with a 30-second deadline, 256 KiB HTTP ceiling
and 4 KiB input/reply limits. Check unknown/unconfirmed refusal, caller/scope/trust
binding, absent and recorded results, retain then release, local liveness,
settlement and passive fenced upgrade. The populated fixture uses its installed
two-object budget and three receipts per object, preserving final-release slots.
Completion, deletion and settlement use
labelled fixture facts, with no provider requests, payment or retry. Stop owned
HTTP instances and drop canisters and temporary files; preserve failed/inconclusive
outcomes before correction and retain material evidence as reference-native-01.

2026-09-30 funding CLI validation follow-up: the broader native CLI suite's
existing download substitute could not bind loopback in the sandbox. Retain
funding-native-01 before a loopback-enabled rerun of that unchanged suite. No
deployed provider contact or funding occurs; temporary local servers/files are
dropped. The funding-outcome PocketIC intent below remains within its budget.

2026-09-30 exact funding-outcome CLI intent: inspect one owned standalone
installation and one populated durable storage fixture through signed PocketIC
HTTP queries as fixed test operators with explicit local root trust. At most
sixteen native invocations total; each makes at most one query with a 30-second
deadline, 256 KiB HTTP ceiling and 4 KiB decoded outcome limit. Check absent
history, original amounts/optional target, partial refund, uncertain attachment,
scope/identity refusals and passive fenced same-release inspection. Fixture
funding phases are controlled substitutes, never real payments or evidence of
Caffeine credit. No provider requests or attached cycles. Stop local HTTP
instances and drop owned canisters and temporary files. Retain any failed run
before correction; record material outcomes as funding-outcome-cli-01.

2026-09-30 upload-history follow-up intent: retain the first run's setup Capacity
refusal, then declare a 65-chunk global/tenant fixture budget for its 65 retained
one-byte declarations. Rerun the native journey alone as `upload-history-cli-02`
under the original twelve-invocation budget. Production limits/accounting and
provider facts stay unchanged; no provider requests or paid effects.

2026-09-30 local upload-history tooling intent: inspect one owned PocketIC
standalone installation with a fixed test operator identity and explicit local
root trust. Retain 64 cancelled one-byte declarations followed by one full-width
active identity to exercise empty filtered progress, explicit pagination and
same-release fenced inspection. At most twelve native invocations, one query
per invocation, 30-second deadline, 256 KiB HTTP and 64 KiB decoded reply limits;
no manifests, file/provider transfer, certificate updates or paid effects.
Saved cursor scope must reject before identity/transport access. Stop the local
HTTP instance and drop owned canisters/temporary input files. This tests local
inventory observations, not deployed provider state or complete recovery freshness.

2026-09-30 certificate CLI follow-up intent: retain the first local run's final
incorrect state assertion, then compare the complete post-revocation record
before and after upgrade. Repeat only the maintained native journey as a new
`certificate-cli-02` run within the original twelve-attempt local budget.
Authentication, trust, state and all provider qualification facts remain unchanged;
no provider calls or paid effects. Retain the first failure below.

2026-09-30 native certificate-assessment intent: exercise the current standalone
assessment through a real signed HTTP query using a fixed local uploader identity
and explicitly trusted PocketIC root. One owned local installation, one 10 MiB
manifest and at most twelve query attempts; each native command is bounded to
30 seconds, a 256 KiB HTTP response and 4 KiB decoded reply. Check full permission
binding, missing preparation, qualification blockers, revocation and upgrade
fencing without changing stable memory. No certificate update, Caffeine request,
provider transfer, attached payment or paid effect. Temporary test credentials
and input files are deleted with their directory; stop the local HTTP instance
and drop the owned canister. Retain failures and material conclusions below.

2026-09-30 maintainer scope decision after the recovery design review: keep one
authoritative storage owner and local durable journals until a clear use case
justifies more machinery. The external journal/controller proposal is deferred;
it does not change the synchronous certificate boundary or enable issuance.
Current-state durability and receipt/lifecycle recovery take priority. Old snapshot
loads remain unsupported for operation and require complete independent
reconciliation before activation. This decision makes no provider/recovery
qualification claim and does not close the outstanding backup/restore requirement.
The historical review artifact below remains unchanged. No new probe or effect.

2026-09-30 recovery authority design intent: inspect the current certificate,
exposure, funding, gateway and restore boundaries plus retained provider artifacts;
review the official IC System API, snapshot and asynchronous-call specifications.
This is source/design evidence, not a provider experiment. At most six public
documentation reads, no account, gateway or paid calls. Determine whether a
separate asynchronous witness can protect the existing synchronous certificate
reply, what complete inventory must survive, and which component must own the
provider identity. Local adversarial models may exercise permit replay and partial
inventories; they cannot establish an external authority or deployed behavior.
Retain references, conclusions, limitations and any failed review here. No external
cleanup is expected; production qualification facts remain false.

2026-09-30 whole-canister rollback intent: use PocketIC 16 through ic-testkit to
take/load actual management snapshots of the current standalone and durable
storage fixture. Fixed local principals only; compare stop/start, same-release
upgrade and rollback across admission, revocation and simulated exposure. At most
two isolated canisters, three snapshots and four loads; at most two 10 MiB local
manifests and no file/provider transfer. Record whether saved heap owners bypass
post-upgrade fencing and whether later history disappears. The fixture's qualified
exposure facts remain substitutes. No live network, credentials, attached payment
or Caffeine requests; no external cleanup. Local snapshot IDs are deleted and
the owned test instances dropped. Stop on unexpected platform failures and retain
the failed outcome here before adjusting the scenario.

2026-09-30 certificate/exposure host integration intent: review the retained
official Mixin/Storage and SDK evidence, then capture current public source in a
fresh `2026-09-30-public-source-01` run using the maintained `public-source`
command. Four anonymous official GitHub/raw/npm GETs maximum, 1 MiB and 30 seconds
per request, no redirects/retries, accounts, gateway effects or attached cycles;
stop and retain any failure. Compare hashes before changing host assumptions.
Local PocketIC work will exercise the real standalone certificate ingress and
read-only assessment with fixed test principals, prepared manifests, malformed
requests, refusal, revocation and restoration. No fixture evidence may enable the
production host: unqualified provider/recovery facts remain blockers. There is no
live upload or payment budget, and no external cleanup is expected.

2026-09-30 planned local verifier submission evidence: extend the signed PocketIC
observation journey with the native one-shot `submit-attestation` command. Retain
the existing fixture owner/project, fixed test verifier identity and ten-byte local
HTTP substitute. Check missing/failed/changed artifacts before signing, intent and
signed-envelope persistence before one update, no repeat submission, and historical
receipt recovery. A local HTTP fault endpoint will exercise uncertain transport
without a deployed provider. Bound each command to 30 seconds and one update;
no live Caffeine, attached payment or provider cleanup is involved. Test artifacts
live in temporary controlled directories; maintained scenarios and material outcomes
are retained here. These tests cannot establish deployed provider availability,
retention, deletion or billing behavior.

On 2026-09-29 the maintainer explicitly selected a configured external verifier
trust model. The verifier must independently retrieve complete content from the
installation's owner/project binding and verify original metadata/root/length.
Its signed canister call attests observed content availability, not guaranteed
future retention, billing cessation or paid-operation identity. The new receipt
path retains all physical/economic obligations and existing exposure/restore gates.
This is an explicit service trust decision, not newly discovered Caffeine behavior.

| Capability / question | Experiment and decisive evidence | Current limit until demonstrated |
| --- | --- | --- |
| Source/interface drift | Capture official revision, package metadata, source/Candid hashes; compare to the reviewed baseline before behavior changes | A matching interface/source is not proof of the deployed implementation |
| Upload completion | Isolated known files spanning empty/single/multiple chunks; capture actual tree/chunk responses, interrupt the final response, then independently download and verify complete bytes/metadata against the expected root | Browser progress/hash alone cannot confirm completion. Verified reads establish observed content availability, not future retention or a trusted canister fact. Keep the current service completion gate until the evidence bridge and narrower semantics are implemented/reviewed |
| Resume and retry charges | One variable per trial: duplicate tree, duplicate chunk, interrupted chunk, completed root. Correlate request logs with isolated audit/account observations; wait through billing aggregation | Assume repeats can cost money. No automatic uncertain paid-effect retry; bounded manual disposition must preserve prior liability |
| Funding | Exact offered/accepted/refunded transport evidence plus provider audit correlation, including deliberately lost response | Never infer exact credit from aggregate balance movement. Uncorrelated outcomes stay uncertain and block automatic retry |
| Deletion and billing | Release one isolated root; record authenticated callbacks, subsequent availability and account observations over the applicable billing interval | Failed GET is not proof of deletion. Preserve physical/economic obligations separately; no unsupported deadline for billing stop. Retain immutable root history to prevent stale-callback reassignment |
| Old-backup recovery | Actual local management snapshot tests below demonstrate heap restoration bypassing upgrade fencing; live evidence must bind records outside restored state, including delayed callbacks/certificates | Upgrade reopening fences owners, but snapshot loading can restore an unfenced heap and erase later obligations. Snapshot operation remains unsupported; standalone certificates stay disabled. A local counter or clear fence cannot authorize reopening |

These limits retain current safety behavior. They do not silently remove Canic
parity, authorize a new completion state or weaken existing acceptance cases.
If an unavailable guarantee makes a feature impossible, explicitly revise that
feature's supported contract and acceptance case before enabling it. Verification
work stays off the storage canister where possible; no return to mandatory full
file hashing in the canister is implied.

## Probe command

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin caffeine-probe -- \
  public-source docs/evidence/caffeine-probes/runs/NEW-RUN-ID
cargo run --offline --locked -p ic-blob-storage-cli --bin caffeine-probe -- \
  verify docs/evidence/caffeine-probes/runs/NEW-RUN-ID
```

The parent directory must exist and the run directory must not. The command
persists `plan.json`, then each `request-N.json` before its HTTPS GET. It retains
`response-N.body`, its byte count/SHA-256/status/outcome in `response-N.json`, and
a final `summary.json`. Each source fetch uses the commit observed in request 0,
not another moving branch read. Runner version and a source/manifest/lockfile
fingerprint are recorded. Four requests maximum, 1 MiB per response, 30 seconds
per request, no redirects or application retries. Only official public GitHub/raw
source and npm endpoints are selected by the command. It sends no credentials,
gateway upload, account query, cycles or payment. File/directory sync is required;
failure stops the run. No existing run is resumed or overwritten.

`verify` checks recorded response hashes and structural completion; failed and
incomplete runs remain labelled. It does not promote source captures to behavioral
qualification. Review source/package drift separately; never update dependency
pins just because latest metadata changed. Native tests use a local HTTP substitute
to exercise intent-before-request, HTTP errors, refused redirects and body limits.
`make probe-check` verifies public-source `runs/` directories offline and is part of
the repository validation gate. Failed/incomplete experiments remain valid records;
corrupt, orphaned or structurally inconsistent recorded evidence fails the check.

Local SDK scenarios use the same pinned patched package as the browser fixture:

```sh
BLOB_BROWSER_NODE=/path/to/node24 make test-sdk-probe \
  BLOB_SDK_PROBE_REPORT=.tmp/NEW-SDK-RUN
```

The report's parent must exist and its directory must be new. This opt-in target
uses installed dependencies, with no downloads, sockets or provider calls. Its
supplied agent response, gateway and in-memory store are substitutes, explicitly
identified in `plan.json`. The SDK prepares and transfers the files; the test
does not reconstruct its upload protocol. The Rust verifier checks captured bytes
against the SDK root and original metadata, outside the storage canister.
The bundle/verifier hashes, request fingerprints, raw synthetic response bodies,
verification outputs and results are retained. `failure.json` preserves failed
cases; interrupted captures without a summary are incomplete. Files in the
material local runs below are covered by `local/SHA256SUMS` in `make probe-check`.
That checksum check establishes artifact integrity, not scenario correctness or
provider authenticity; rerunning the opt-in test establishes current local behavior.

## Run index

`recovery-design-01` / 2026-09-30: source review and architectural inference;
[retained source references/hashes, findings and limits](local/2026-09-30-recovery-design-01/summary.json).
Official IC version, message execution, management history and snapshot rules were
reviewed against the current code and retained snapshot experiment. The historical
proposal considered an external complete journal/controller and an asynchronous
certificate prelude. The maintainer subsequently deferred that architecture in the
[current scope decision](../../service-contract.md#recovery-scope--maintainer-decision-2026-09-30);
the artifact's original recommendation remains preserved, not an active instruction.
This review is not runtime qualification. No provider requests, paid effects,
instances or cleanup; cached permits and sequence watermarks remain insufficient.
Production facts remain false.

`snapshot-01` / 2026-09-30: actual PocketIC management snapshot operations against
standalone and the durable fixture; [retained outcome and source/Wasm hashes](local/2026-09-30-snapshot-01/summary.json).
Both maintained scenarios pass (3.47 and 1.82 seconds). Rollback bypasses the upgrade
hook, revives withdrawn permission and loses later reservations/history. The fixture
can repeat simulated exposure when its host again substitutes qualified freshness;
standalone still traps certificate issuance without changing state. These are
negative recovery qualification results, not supported rollback behavior or deployed
provider observations. Three snapshots deleted; no provider requests, paid effects
or outstanding external cleanup. Strict targeted Clippy passes.

The `verifier-observation` local experiment is planned for 2026-09-29 against the
current workspace. Tests use the maintained Caffeine target encoder and streaming
verifier, a fixed PocketIC verifier identity, fixture owner/project and loopback HTTP
substitutes. Record intent before one bounded GET; exercise complete, corrupt,
truncated, oversized, non-200, redirected and encoded responses. Test refused service
plans, interruption/no-clobber behavior and a verified statement carried to receipt
recovery through an explicit fixture attestation. Per invocation: one service query,
one provider GET maximum, 30 seconds each, explicit content bound at most 1 GiB,
64 KiB service Candid. No deployed provider/account, credentials or attached cycles;
no external cleanup. Stop on failure and retain failed outcomes; these local cases
do not qualify the deployed provider or authorize live reads.

The `attestation-recovery` local contract check is planned for 2026-09-29 against
the current workspace and `storage_attestation_cli` PocketIC test. A fixed test
tenant signs bounded receipt queries; the fixture operator is the explicit verifier.
The plan saves the exact statement before a local attestation, discards its reply,
and inspects absence, acceptance, changed intent, wrong verifier/root trust and
settled/restored history. Exposure, content and deletion/settlement facts are
labelled substitutes. Each CLI query is limited to 30 seconds, 256 KiB HTTP and
4 KiB Candid; no query retries. No deployed account, gateway request, provider
charge or attached cycles are involved. Stop on failure; only temporary local
canisters/files require cleanup. The completed result is indexed below.

| ID / date | Class and target | Status / evidence | Conclusion and obligations |
| --- | --- | --- | --- |
| standalone-certificate / 2026-09-30 | Actual standalone and local storage-fixture PocketIC endpoints; no deployed provider | [Host refusal case](../../../tests/pocketic/tests/standalone_certificate/mod.rs) passes (4.27 seconds); six existing exposure/certificate cases pass (12.77 seconds), including real signed ingress verification, rollback and lost-reply recovery; optional Chromium case not rerun. Five core certificate tests, Candid comparison, strict affected Clippy, warning-free core/host docs and release Wasm builds pass | Standalone reports four unqualified prerequisites and refuses certificate issuance; role/malformed/unprepared/revoked/restored cases preserve state. Stop/start grants no new authority. Assessment does not reserve issuance. Shared blocker DTO/conversion replaces the fixture duplicate; positive issuance facts remain labelled fixture substitutes. No provider requests/attached cycles, external charges or cleanup obligations; no provider or old-backup qualification |
| public-source-03 / 2026-09-30 | Anonymous official source/registry capture, four bounded GETs | [Plan](runs/2026-09-30-public-source-01/plan.json), [summary](runs/2026-09-30-public-source-01/summary.json); captured at commit `14ab9511fd73258a380bc1a6861d6da6e548ebbb` | Upstream HEAD moved; retained Mixin/Storage SHA-256 values still match public-source-02, and npm remains 1.1.2 with the same integrity. The root-only certificate reply has no owner/project, admitted size, deadline or operation identity fields; source refresh adds no deployed pre-charge, replay, provisioning or recovery evidence. No provider/account effects or external cleanup |
| verifier-submission / 2026-09-30 | Native artifact validation and signed PocketIC through a bounded local fault proxy; fixture exposure and ten-byte local content | [Maintained journeys](../../../tests/pocketic/tests/storage_observe_cli/mod.rs) pass (4.49 seconds): normal acknowledgment, dropped real acknowledgment and forced HTTP 202. Native CLI tests and strict affected Clippy pass; historical settled/restored recovery (5.28 seconds) and signed standalone status/history regression (6.46 seconds) pass | Proxy compares the exact signed wire body with saved intent before forwarding one update. Competing submitters send once; incomplete, failed or changed observations reject before claiming. Killed provider reads remain undispatchable. A dropped acknowledgment stays uncertain; 202 stays pending. Both recover the exact immutable receipt through signed queries without resending. Request IDs, source/root/artifact hashes and outcomes are retained in temporary controlled test runs; scenario definitions remain in source. The pinned ic-agent 0.49.2 implementation was inspected locally for one-call transport, certificate verification and disabled retries. No live provider/account, attached cycles, external charges or cleanup obligations; deployed retention/billing/exposure remain unqualified |
| historical-recovery / 2026-09-26 | Source inspection, substituted HTTP replies, Candid codecs and anonymous metadata | [Existing record](../caffeine-recovery-review.json), indexed retrospectively; not rerun | SDK progress is insufficient; root-only callbacks and payment correlation need conservative handling. No paid effects were recorded in that review; no fabricated modern run metadata |
| historical-deployment / 2026-09-25 | Anonymous deployed interface/gateway/price observations | [Existing record](../caffeine-deployment-observation.json), indexed retrospectively; use its own timestamps/scope | Interface reachability is not upload, payment or billing qualification |
| public-source-01 / 2026-09-29 | Anonymous public source/registry capture; four recorded GET invocations, no account/paid effects | [Plan](runs/2026-09-29-public-source-01/plan.json), [summary](runs/2026-09-29-public-source-01/summary.json); captured, integrity checked | Official commit `78781961e52b8c9c874becd473402950429d4818`; both backend hashes match the prior review; npm 1.1.2/integrity unchanged. This runner had no application retries but still inherited reqwest protocol retries; actual wire request count was not measured. Preserved as captured; current runner explicitly disables transport retries |
| public-source-02 / 2026-09-29 | Repeat public-source capture after explicitly disabling HTTP transport retries; four requests maximum | [Plan](runs/2026-09-29-public-source-02/plan.json), [summary](runs/2026-09-29-public-source-02/summary.json); captured, integrity checked | All four response hashes match 01. New runner/source fingerprint retained; 01 and its retry limitation remain unchanged. No provider effects |
| local-sdk-01 / 2026-09-29 | Pinned patched SDK with substituted certificate-agent reply, gateway and in-memory intent store | Failed before runner initialization: ESM bundle attempted dynamic require of Node `tty`; no run directory or requests. Fixed bundle with Node `createRequire` | Planned six scenarios did not start. No network/account access or cleanup obligations |
| local-sdk-02 / 2026-09-29 | Same local substitutes; independent native Rust content verification | [Failure](local/2026-09-29-sdk-02/failure.json): first verification subprocess timed out; partial request/reply records retained | Node synchronous subprocess input did not reach EOF in this environment (reproduced with `cat`); asynchronous closed input works. No completed case, network/account access or cleanup obligations |
| local-sdk-03 / 2026-09-29 | Same local substitutes, asynchronous verifier subprocess | [Plan](local/2026-09-29-sdk-03/plan.json), [summary](local/2026-09-29-sdk-03/summary.json); all six scenarios pass | SDK sends all chunks despite `existing_chunks` and returns successfully for a non-complete status. Lost final reply stays uncertain despite verified bytes; replay blocked. Aggregate budget stops before the next request. No real network/account access or cleanup obligations |
| verifier-contract / 2026-09-29 | Local model/CLI and PocketIC; fixed test verifier principals and substituted content observations | 60 upload-store and two codec tests pass; all 32 standalone cases pass (141.72 seconds), plus `storage_completion` (5.05 seconds) | Exact authority/permission binding, immutable receipt replay, four stable-write rollback cuts, late revocation, release/settlement and fenced restoration pass. The signed CLI verifies 10 MiB against original metadata without mutating service state. Local canisters only; zero provider requests/attached cycles, no external cleanup or deployed availability claim |
| attestation-recovery / 2026-09-29 | Local saved-intent decoder/CLI and signed PocketIC receipt queries; fixed test tenant and fixture verifier | `storage_attestation_cli` passes (5.33 seconds), including a deliberately discarded successful update acknowledgment | Absent, matched and conflicting evidence remain distinct. Changed permission/verifier and wrong root trust reject; settled/restored receipts remain immutable. Queries preserve stable bytes and the saved intent. This discards a test acknowledgment, not a simulated network packet; content/exposure/settlement are substitutes, with no deployed provider availability claim. No external requests, charges or cleanup obligations |
| verifier-observation / 2026-09-29 | Native loopback HTTP substitutes and signed PocketIC installed-plan/statement journey; fixture exposure only | 14 native CLI tests pass, including multi-chunk streaming and HTTP corruption/EOF/size/encoding/redirect refusals. `storage_observe_cli` passes (4.00 seconds); standalone completion/plan refusals pass (5.37 seconds) | Intent exists before GET; complete verification saves the exact statement without dispatch. Killing the subprocess mid-body leaves no statement/summary and forbids reuse of that run. Explicit fixture dispatch accepts the independently checked statement and receipt recovery matches. Service bytes stay unchanged during observation. Local artifacts are temporary test outputs; maintained tests retain the scenario definitions. No real provider/account access, charges or cleanup obligations; no deployed qualification |
| certificate-cli-01 / 2026-09-30 | Signed local native queries with current production host facts | [Failed test outcome](local/2026-09-30-certificate-cli-01/summary.json) | All transport, blocker, intent, revocation and fence checks reached; final test assertion wrongly expected Reserved after explicit revocation (actual Cancelled). First outcome and source/binary hashes retained. No provider requests or paid effects; local instance stopped and temporary artifacts dropped |
| certificate-cli-02 / 2026-09-30 | Same local signed journey with corrected post-revocation baseline | [Passed test outcome](local/2026-09-30-certificate-cli-02/summary.json), 6.40 seconds | Complete revoked record survives upgrade unchanged; signed assessment retains all four blockers and rejects changed intent/trust, revocation and restoration. No state changes, certificate updates, provider requests or paid effects; local instance stopped and temporary artifacts dropped. No deployed qualification |
| upload-history-cli-01 / 2026-09-30 | Local PocketIC inventory setup; no native query reached | [Failed setup outcome](local/2026-09-30-upload-history-cli-01/summary.json) | Fixture requested 65 retained declarations but kept lifetime chunk limits at 20; admission correctly returned Capacity. Existing service history tests passed. Source/binary hashes and first failure retained; owned instance dropped, no provider/paid effects |
| upload-history-cli-02 / 2026-09-30 | Signed local queries with corrected fixture history budget | [Passed outcome](local/2026-09-30-upload-history-cli-02/summary.json), 6.56 seconds | Empty filtered scan advances across 64 cancelled declarations; explicit pages retain full-width active identity and cancelled history through fenced upgrade. Saved cursor/scope/identity refusals preserve stable bytes. Local instance stopped and temporary files dropped; no provider/paid effects or recovery qualification |
| funding-native-01 / 2026-09-30 | Native CLI validation in network-restricted sandbox | [Failed environment outcome](local/2026-09-30-funding-native-01/summary.json) | New funding cases passed; existing download substitute could not bind loopback. Failure retained before unchanged loopback-enabled rerun; no provider contact or paid effects |
| funding-outcome-cli-01 / 2026-09-30 | Signed local standalone and controlled durable funding observations | [Passed outcomes](local/2026-09-30-funding-outcome-cli-01/summary.json), 10.93 seconds PocketIC | Exact history-to-outcome binding, partial refund, uncertain attachment, typed conflicts, absence and fenced upgrade preserve stable bytes. Fifteen native invocations, fourteen query attempts, no provider requests/payments. Native unit rerun passes with local loopback; all local instances/files dropped. No provider-credit or recovery qualification |
| reference-native-01 / 2026-09-30 | Signed local standalone and controlled durable reference inspection | [Passed outcomes](local/2026-09-30-reference-native-01/summary.json), 18.51 seconds PocketIC | Separate history/current queries preserve success and recorded failure through release, substituted settlement and fenced upgrade. Wrong intent/scope/role/trust refuses, queries preserve stable bytes, installed receipt/cleanup bounds unchanged. Twenty-two native invocations, twenty query attempts; no provider requests/payments. Owned local instances/files dropped; no availability, publication or recovery qualification |
| standalone-lifecycle-01 / 2026-09-30 | Source review and local standalone lifecycle | [Passed outcome and review](local/2026-09-30-standalone-lifecycle-01/summary.json), 6.37 seconds | Stop/start preserves current installation/state; rejected replacement settings preserve the previous owner; two same-release upgrades retain metadata and all four fences. Operational refusals preserve stable bytes. Funding/read owners are empty in this case. Acceptance/parity summaries corrected without qualifying active recovery, provider, managed parity or retirement. Owned instance dropped; no provider/paid effects |

Other retained investigations are indexed here without inventing missing request
logs or replaying their effects. Use each record's own dates and evidence classes:
[client observations](../caffeine-client-observations.json),
[Mops verification](../caffeine-mops-verification.json),
[funding review](../caffeine-funding-review.json),
[contract refresh](../caffeine-contract-refresh.json),
[installation review](../caffeine-installation-review.json),
[browser reuse](../caffeine-browser-reuse.json), and
[gateway/account transport review](../caffeine-gateway-transport.json).
The older records retain their original schemas; they are research history, not
backward-compatible product state or a claim of modern capture completeness.

The maintained Chromium/PocketIC suite also passed its ten scenarios with the
aggregate-budget contract (49.12 seconds). The initial sandbox run could not bind
the local server; the loopback-enabled rerun passed. This uses actual IC certificate
and IndexedDB behavior with a local gateway substitute. It does not turn the SDK
probe's synthetic certificate into deployed-provider evidence.

## Next trial — upload/resume, prepared scope, not dispatched

Use an isolated explicitly selected owner/project/payer and diagnostic authorization
surface; standalone does not yet expose the complete real certificate/completion
journey. Preserve original bytes outside the canister. Reuse the pinned upstream
file preparation/transfer code and independently check reads against expected
roots/metadata. Record all SDK-internal requests: a high-level upload call is not
one HTTP request. No SDK retry may bypass the total request/byte budget.

1. Record baseline configuration, current interface/source hashes and available
   account/audit evidence. Stop on binding/version drift, unresolved pre-existing
   obligations or insufficient budget evidence. Do not fund or link accounts as
   an implicit setup step.
2. Upload one known three-byte file; retain every request/response and the exact
   certificate correlation securely. Download and verify it independently.
3. Upload a second known file of 1 MiB + 1 byte. Deliberately discard the final
   chunk response at the client, recording that loss. Inspect current availability
   and the documented resume response. Record each explicit repeated tree/chunk
   request separately; stop if the response cannot be interpreted under the pinned
   contract. Do not turn this experiment into an automatic production retry rule.
4. Download and verify the second file, then capture account/audit observations
   and compare complete versus lost-response paths. If charges cannot be correlated,
   record that result as unresolved. Preserve both roots and their obligations for
   a separately planned deletion/billing observation; do not erase trial records.

Proposed client bounds for this trial: two object roots, at most two certificate
updates, eight read-only service/account observations, 24 gateway HTTP requests,
10 MiB upload and 12 MiB download traffic, 30 seconds per request and 15 minutes
total. Stop at the first exhausted bound, unexpected binding, uncertain new payment
or insufficient charge evidence. These are traffic/time ceilings, not a financial
guarantee. The exact account, operator, certificate authority, provider spending cap
and funded cleanup owner remain unselected. Resolve them in a recorded run plan
before any live effect; this draft is not permission to deploy or spend.

Deletion/billing and long-retention experiments have separate observation windows.
No guessed wait interval or quiet balance sample can establish final settlement.
