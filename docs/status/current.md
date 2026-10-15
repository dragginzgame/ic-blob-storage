# Current status

Date: 2026-10-15

## Shared 0.15 and retained-write ownership — pending 0.27.0

The [adoption receipt](../evidence/shared01500270.md) binds reviewed Shared main
`3bef3f2daec7f64f875397ed4d53d5b9e2e68a08` to 128 payloads and the unchanged
common tool pins. Local release fixtures carry the new companions and valid
next-release note placement; formatting input discovery delegates to Shared.
Configuration decoding and browser hex encoding reuse their canonical local
owners without changing budgets, cryptography or authority checks. Cargo-edit
remains required by the selected committed metadata helper; Shared PR #205's
pending implementation is outside this adoption.

Host 0.18.6 was already selected; its published registry graph is qualified with
Memory 0.40.0, Metrics 0.10.1 and Testkit 0.44.0. Focused configuration/CLI/probe,
buffered output, strict lint and actual Rust 1.91 checks pass. Fresh CLI/Wasm
passes seven installed session cases, three observation/recovery cases and the
ten-scenario Chromium certificate journey. Snapshot, tool/pin admission,
release adapters, formatting preservation and installer/evidence substitutes
pass. Full CI, native macOS, Canic and live provider qualification remain separate.

The isolated retained-write producer is [Host PR #132](https://github.com/dragginzgame/ic-host-tooling/pull/132)
at `549670adfd2110a2b93063b9351f795496053d71`. Its five retained-file cases,
strict filesystem lint/Rustdoc and Rust 1.88 checks pass. The isolated consumer
passes five probe cases, strict CLI lint and Rust 1.91; its replacement applies
cleanly but awaits publication under [#76](https://github.com/dragginzgame/ic-blob-storage/issues/76).
Normal Blob has no source override. Earlier dirty work, own versions, index and
HEAD are preserved. The Host topic commit/PR leaves sibling working checkouts
untouched. Before-images, failed/final logs, graph/artifact receipts and the
consumer patch remain under `.tmp/review-validation/adoption0270/`.
The final graph checkpoint detected six Windows-only edges returning to their
starting selection. The starting lock is preserved and focused compilation,
fresh builds and all installed journeys pass again on that final graph; earlier
and isolated candidate receipts retain their separate inputs.

## Rust 1.91 minimum — pending 0.27.0

The maintainer selects Rust 1.91.0 as the workspace MSRV, matching the incoming
public Memory 0.40 dependency's declared minimum. All twelve members inherit
that floor. CI explicitly prepares/selects 1.91; Make's existing minimum check
covers separate public library paths, all native targets/features and supported
Wasm libraries. Development, formatting and lint stay on Rust 1.99.0.

The complete focused `make msrv-check` passes with actual Cargo/rustc 1.91.0 on
the selected Memory 0.40.0, Metrics 0.10.1, Host 0.18.5 and Testkit 0.44.0 graph.
Five fixture day-long advances use the newly available standard hours constructor
with the same elapsed time; strict workspace Clippy passes at the raised floor.
Root lockfile and development-toolchain pin remain byte-identical. This compiler
qualification does not replace runtime, retained-installation or Canic evidence.
Earlier 1.88 results and 0.26.1 draft references below retain their original
source/graph scope; the combined pending batch is now classified as 0.27.0 for
its raised consumer minimum and exposed Memory family. Own package versions,
Git refs/index and sibling files are unchanged. Full CI/release effects did not
run. Before-images, compiler logs, initial lint/format refusals and input receipts remain under
`.tmp/review-validation/msrv191/`.

## Process output and browser boundary cleanup — pending 0.26.1

The integration harness delegates final pipe draining to published Host through
Testkit's existing re-export. Blob selects terminating ceilings of 16 MiB stdout
and 4 MiB stderr; scenario/session owners retain deadlines. Buffered control
readers, nonzero/cached child status, closed stdin and process-group cleanup
remain intact. Native sessions now hand over their actual remaining stdout.

Browser transports share bounded response collection and cancellation, retaining
the certificate's required body/256 KiB ceiling and the gateway's optional body/
64 KiB ceiling. Fetch policy, refusal classes and gateway claim/observation order
remain local. Worker and durable-journal boundaries share passive strict-field
and safe-integer predicates while retaining independent validation and refusals.
The TLS transport fixture now serves the bundled gateway with its shared imports.

Focused Linux checks pass buffered-output retention, three response-stream cases,
worker authority/refusals, actual Chromium store/reopening and 17 bootstrap cases,
eight TLS transport cases, strict PocketIC-package lint and actual Rust 1.88.
Fresh CLI/Wasm selections pass seven installed publication-session cases, three
installed observation/recovery cases and the ten-scenario Chromium certificate
journey. These are local/substitute results; full CI, native macOS, Canic and
deployed-provider qualification remain separate. No paid effects ran.

The starting lock already selects Host 0.18.5, Memory 0.39.0, Metrics 0.10.0 and
Testkit 0.44.0; root manifest/lock and tested artifact hashes remain unchanged.
Earlier pending work is preserved. Before-images, failed sandbox socket attempt,
raw logs and input receipts remain under `.tmp/review-validation/drain0261/`.
No dependency, own-version, sibling-file, index or Git-ref changes ran.

## Fixture decoding and native simplification — pending 0.26.1

Released base is 0.26.0 `c7fbef66188398c84cdc0a8eccdaae9220604bd0`.
Authority and storage probes now share the private `fixture_chunk` blob decoder.
Their byte/work limits, refusal categories, measurement boundaries, authenticated
call bindings and manifest verification remain local. Direct Serde-bytes edges
move from those two probes to their existing private protocol owner; registry
versions and own package metadata are unchanged. Two decoder cases, strict
affected-package lint, actual Rust 1.88 Wasm compilation, six installed readback
cases and the installed tenant-download case pass on the normal registry graph.

[Host #128](https://github.com/dragginzgame/ic-host-tooling/pull/128) at
`2bfe6e67dd045a99b11f3386e93b2703d30284ea` owns bounded newline framing.
[Build #41](https://github.com/dragginzgame/ic-build/pull/41) at
`1fd96686437d91a43881a169ee0e69eae3991d20` owns relocatable captured-input
fingerprints. Scoped topic commits/PRs leave normal sibling checkouts untouched.
Producer checks pass three framing and eighteen input-evidence cases, strict
lint/Rustdoc and actual Rust 1.88 against their separate committed graphs.

The isolated combined candidate passes 118 native CLI and five probe cases,
strict CLI lint, actual Rust 1.88, seven installed publication-session cases and
the ten-scenario Chromium journey. Final binaries, Wasm and graph hashes are
unchanged through installed qualification. Real Cargo rebuilds detect added or
renamed source files and recover the original digest after their removal. The
fingerprint covers CLI sources, its build script/package manifest and root
manifest/lock; it remains descriptive provenance and preserves historical
records. No provider observation, paid effect or release ran.

[#74](https://github.com/dragginzgame/ic-blob-storage/issues/74) and
[#75](https://github.com/dragginzgame/ic-blob-storage/issues/75) own activation.
Independent `adopt-host.patch` and `adopt-build.patch` apply separately and together
and pass manifest/Rust formatting. They remain inactive until actual published
API versions and the compiler graph are admitted; normal Blob has no source
override or new CLI build dependency. The candidate uses published-main Build
0.3.3 with cargo-platform 0.3.2; the producer's separate pending 0.4/Rust 1.91 work
was not copied or used to raise Blob's floor. Full release/native macOS and future
registry-graph checks remain separate. Source archives, final/failed logs and
input receipts remain under `.tmp/review-validation/three0261/`, with originals
in `/tmp/blob-three-consumer-20261015/` and its two isolated producer checkouts.

## Browser consolidation and bounded consumer cell

The certificate browser driver now uses the existing queued JSON channel;
shutdown runs on success or failure. The journal profile delegates payloads to
the existing asset server while retaining exact GET/path refusal, no-store
headers and its browser route allowlist. Three direct Node channel cases and
the ten-scenario installed Chromium certificate journey pass. The actual
675/5,000/10,000-row profile passes population and process reopening, retaining
uncertain gateway history, cancellation and capacity refusals. These are local
substitute results, with no live IC/provider requests or paid cycles.

The consumer's custom envelope retirement is prepared in an isolated candidate,
with the bounded extension to [Memory #105](https://github.com/dragginzgame/ic-memory/pull/105).
The normal checkout keeps its published dependency and existing journal until
that API is published. Blob retains Candid decoding/work limits, identity and
model validation, fencing and persistence before heap replacement. The candidate
uses Memory's Cell envelope and is fresh-fixture-only across the format cut;
no retained image is reset and no compatibility reader is added.

The bounded producer extension is published on the existing draft PR at
`67bffaf488cd39f5c961570051b8e494877f8831`. Focused producer checks pass 224
library cases (five unrelated cases ignored), strict feature lint/Rustdoc and
actual Rust 1.88 Wasm compilation on its committed Host 0.18.2 graph. The
candidate separately passes strict probe lint, actual Rust 1.88 and 18 installed
consumer cases plus the combined ten-scenario Chromium journey on final Wasm.
It uses a local Memory source override only in isolation; the remaining incoming
graph is unchanged. `adopt-bounded-memory.patch` applies cleanly but is inactive
until publication. Full producer delivery/native acceptance remains outstanding.

Incoming Memory 0.39.0, Metrics 0.10.0 and Testkit 0.44.0 selections are preserved.
The public Memory change classifies the complete pending batch as 0.26.0;
package versions are unchanged. Earlier 0.25.2 evidence below still belongs to
its original graph. Focused logs and the pending consumer replacement are under
`.tmp/review-validation/cleanup0260/`. Full CI, native macOS, Canic and deployed
provider acceptance remain separate.

## Released-issue acceptance reconciliation

Closed seven completed producer/consumer issues: coordinated archives
[#56](https://github.com/dragginzgame/ic-blob-storage/issues/56), hook temporary
parents [#58](https://github.com/dragginzgame/ic-blob-storage/issues/58), shared
storage/evidence [#59](https://github.com/dragginzgame/ic-blob-storage/issues/59),
Candid export separation [#63](https://github.com/dragginzgame/ic-blob-storage/issues/63),
snapshot catalogs [#66](https://github.com/dragginzgame/ic-blob-storage/issues/66),
Memory 0.37 [#68](https://github.com/dragginzgame/ic-blob-storage/issues/68) and
Memory 0.38 [#71](https://github.com/dragginzgame/ic-blob-storage/issues/71).
The queue now has eleven open issues.

Exact released 0.25.1 source `438d17698575c3c3a67fb423d9bf2a1b79eeaaab`
passes [Linux, Apple Silicon, Intel and Rust 1.88 CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38613096924).
Native raw logs confirm archive integrity, actual tooling/retention fixtures,
exact-ID evidence transport and original/O3/Os/Oz installation/restoration.
Intel package, optimizer and tooling ZIPs were independently downloaded by ID,
matched to GitHub's digests and inspected. Reviewed Shared remains 0.11.0
`a15ff323449f4b6190f647aec0de43cbcfb314ba` with 116 payloads; linked Shared
owners received consumer completion receipts. Memory 0.37 closes against its own
published 0.24.0 source and scoped composition/compile evidence. Memory 0.38 has
published producer/native acceptance and Canic #559's separately frozen caller
and installed evidence; Canic's broader uncommitted delivery stays with its owner.

Keep #57 open: exact 0.22.6 Intel acceptance was cancelled; no historical rerun
or waiver occurred. #69/#70 still need Blob adoption, #72 needs publication and
Canic qualification, and #4/#5/#6/#31/#32/#33/#34 retain application/provider
acceptance. Raw issue/job/artifact readback remains under
`.tmp/review-validation/issues0251-close/`, with originals at
`/tmp/blob-issue-close-20261015/`. Incoming Memory 0.39, Metrics 0.10 and Testkit
0.43 edits are untouched and unqualified by this audit; the preceding local
0.25.2 qualification remains bound to its original graph. No code, dependency,
package metadata, Git, CI-dispatch or provider effects ran in this reconciliation.

## Shared installed-host continuity and certificate owner

Released 0.25.1 at `438d17698575c3c3a67fb423d9bf2a1b79eeaaab` has a
successful [complete hosted run](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38613096924).
Pending compatible 0.25.2 implements the producer/standalone part of
[#72](https://github.com/dragginzgame/ic-blob-storage/issues/72).
Installation owns version-gap fencing and previously-active proof selection;
shared recovery workflows own pre-update and explicit recovery completion.
The host keeps its heap checkpoint, actual platform observations, synchronous
memory/lifecycle custody and endpoint authentication. The new continuity access
trait composes with the existing recovery installation access; no global holder
or implicit lifecycle is installed. Certificate composition resolves permission
before evidence through an immutable assessment or synchronous issuance borrow.
Existing lower-level primitives remain available to independently composed hosts.

Focused Linux qualification passes 15 installation/recovery cases, nine installed
standalone cases, strict library/standalone lint and Rustdoc, and actual Rust 1.88.
Fresh standalone and gateway-source Wasm plus the explicit existing PocketIC 16.1
bytes own signed-uploader, assessment/issuance, snapshot-refusal, replicated account
and same-release recovery results. The first account selector included a nested
CLI case without its required artifact; its failure is retained and the full four-
case selector passes with a freshly built, explicit CLI. Manifest/lock, source
and artifact checks retain the unchanged graph and tested bytes. Raw logs and
before-images remain under `.tmp/review-validation/host0252/`, with originals at
`/tmp/blob-host72.uxO97y/`.

Canic remains read-only at `644d4b13012b395d6211a6907e2e6aafdf391db9` plus its
incoming integration edits. Its host still needs to adopt the published APIs and
qualify dedicated/embedded consumers; #72 remains open for that acceptance.
Its active handoff now records Blob 0.25/Memory 0.38 integration evidence, while
later incoming Testkit/Timers edits retain separate qualification obligations.
No Canic build, full local CI, native macOS, deployed-provider or paid effects ran.
No dependency selection, own package version, sibling file or Git effects changed.

## Probe conversion, byte accumulation and session budgets

Admission and storage probes share checked upload-request projection in the
private test protocol. Operation/object identity, incarnation/reference one and
the fixed root remain unchanged; each probe owns its failure mapping and actual
service/tenant authority checks. The protocol gains no service-core dependency.
Restoration byte totals use published Metrics' scalar counter, with checked
report narrowing that preserves exact caps, sticky overflow and window resets.
Publication-session startup, argument admission and retained-history validation
share checked step/query formulas. Original history budgets are admitted before
permitted restart controls replace them; these checks grant no retry authority.

Focused Linux qualification passes 118 CLI cases, four resource-counter cases,
strict affected-package lint, actual Rust 1.88 and three installed PocketIC
journeys covering admission identity/authority, read-window restoration and
write-fault rollback. Fresh admission/storage Wasm and the explicitly verified
existing PocketIC 16.1 bytes own these local/substitute results. Host 0.18.2,
Memory 0.38.0, Metrics 0.9.6 and Testkit 0.42.0 selections remain unchanged.
Before-images, graph/source/artifact hashes and raw logs, including initial
sandbox socket refusals and corrected public-helper lint, remain under
`.tmp/review-validation/fixture0251/`, with originals at
`/tmp/blob-fixture3.4jA0oC/`. Full CI, native macOS, Canic and deployed-provider
qualification were not run. Incoming Cargo edits and the prior cleanup remain
preserved. No dependency, own-version, sibling or Git effects ran for this slice;
compatible pending notes remain 0.25.1.

## Inventory and root/cell extraction preparation

Initial preparation and frozen-batch loading now share inventory decoding and
schema/file-count admission. Their independent hashes, body verification and
retained request checks remain intact. Eight focused inventory cases, strict CLI
lint, formatting and actual Rust 1.88 pass on the incoming Testkit 0.42 graph.

The authorized producer work is prepared in isolated topic branches, leaving
normal sibling checkouts untouched: Memory's opt-in `testing::FixtureCell` retains
one explicit runtime/cell; Host admits directory capabilities and opens regular
files through retained no-follow parents. The normal Blob checkout still uses
the published APIs until these contributions are integrated and published.
Following explicit publication approval, the Memory tracking issue is
[#104](https://github.com/dragginzgame/ic-memory/issues/104). The isolated branches
are published as draft [Memory #105](https://github.com/dragginzgame/ic-memory/pull/105)
at `d8ffd8f55151ef4c746ee6c8bb5d8ddb5286dac2` and draft
[Host #114](https://github.com/dragginzgame/ic-host-tooling/pull/114) at
`31d9a251feb0025d600118a93b0d089752dcef05`, both targeting main. Readback confirms
both are open drafts with those exact heads. Prepared external descriptions
and the source-bound consumer replacement remain under
`.tmp/review-validation/extract0251/`. Host's read subset relates to
[Host #80](https://github.com/dragginzgame/ic-host-tooling/issues/80); its broader
Query extraction and native acceptance are not resolved by this work.

The isolated replacement passes 118 CLI cases, strict affected-package lint,
Rust 1.88 and six installed callback/restoration journeys plus the retained-output
regression. Probe keys, byte/work limits, codecs and lifecycle calls are unchanged.
One input-root descriptor serves each batch; each body opens through it at the
actual read, avoiding per-file descriptor retention. The refreshed candidate uses
Memory 0.38.0 and Host 0.18.2 plus the unpublished extensions, and registry Testkit
0.42.0. It retains separate earlier graph receipts. Fresh Wasm, root/lock hashes,
verified existing PocketIC 16.1 bytes and raw logs remain at
`/tmp/blob-rootcell.stAtRo/`, with copies beneath the consumer evidence directory.
Memory's two fixture cases and Host's 21 read cases, strict lint/docs and Rust
1.88 pass. These are Linux/local-substitute results; full delivery, native macOS,
Canic and deployed provider qualification were not run. Select actual published
versions containing both APIs before applying the patch and reconciling the
registry lock; prospective floors must move if unrelated releases occupy them.
Incoming Testkit/Host lock updates are preserved. Scoped commits, topic pushes
and GitHub publication were approved; no own-version, merge, release, registry
publication or paid effects ran. The compatible numbered draft is 0.25.1.

## Provider decoding, fixture control and update outcome consolidation

Six provider decoders share private Candid configuration without changing their
byte checks, work/skipping/type budgets, binding order or typed reply failures.
Four native update reports share pure outcome classification with their original
success labels and operation-owned authenticated refusal predicates. Record
fields, intent ordering and retry restrictions remain with each operation.
Three browser fixture drivers share JSON control transport, including queued
messages, explicit EOF rejection and channel shutdown; scenario deadlines and
browser ownership remain local. The channel regressions run in the existing
standalone browser target.

Focused Linux qualification passes 47 provider cases, 117 CLI cases, three
channel cases, strict library/CLI lint, Rust 1.88 and four installed Chromium
journeys covering upload/release, lost reply, serial verification and corruption.
Fresh standalone Wasm/CLI and the explicit existing PocketIC 16.1 bytes own these
local/substitute results. Incoming Host 0.18.1, Memory 0.38.0 and Metrics 0.9.6
selections are preserved; manifest/lock/artifact hashes remain unchanged through
qualification. Logs and input hashes remain under
`.tmp/review-validation/consolidation0250/`, including the incorrect CLI library
selector and sandbox loopback refusals before the successful focused reruns.
Full CI, native macOS, Canic and deployed provider checks were not run. No sibling,
own-version or Git effects ran. The public Memory cut selects pending notes
0.25.0; this focused cleanup does not qualify that complete release batch.

## Memory 0.37 composition repair

The reported workspace check failed because the composition test read the
bucket size directly from `MemoryAllocations`. Memory 0.37 keeps that persisted
measurement under `summary`; the assertion now follows that API and still checks
the host-selected 16-page bucket profile. Both composition tests and the complete
offline, locked workspace all-target/all-feature check pass on the current graph.
Root manifest/lock hashes remain unchanged. Logs are retained under
`.tmp/review-validation/memory037/`; the original failed validation logs remain.
This repair changes only test coverage and notes. Full CI, native macOS and
service/provider qualification were not run; pending release notes stay 0.24.0.

## Command, browser and HTTP fixture cleanup

Native commands now derive actor/operator flags from their parsed variant and
reject unknown operator commands explicitly. Upload history keeps its operator
identity and tenant reference commands keep actor identity; unknown/extra flags,
scope and URL trust refusals remain unchanged. Browser fixtures share frozen
asset serving with their own routes/cache/observation policies, while the SDK
preparation crosses the Rust control boundary as its exact three-field projection.
Opaque handles, TLS, upload faults and strict Rust decoding remain with their owners.

Published Testkit 0.41.0 delegates provider GET and signed-update proxy capture
under the original byte budgets. The release arrived during this work and its
registry VCS receipt names `5abb2d6cde750386647817741f7ccec403bc7fca`.
The native backpressure fixture must retain incomplete trust-lookup sockets;
its Testkit request-line/completion extension is applied upstream for 0.41.1.
[Blob #70](https://github.com/dragginzgame/ic-blob-storage/issues/70) owns adoption
after publication. Its qualified replacement is retained at
`.tmp/review-validation/cleanup0240/phased-http.patch`; prospective metadata,
path configuration and lock stay isolated, never in the ordinary graph.

The final live graph selects Host 0.18.0, Memory 0.37.0, private Metrics 0.9.5
and Testkit 0.41.0. Incoming Host/Metrics edits are preserved and excluded from
this task's source delta. Final manifests/lock stay byte-identical through focused
Linux qualification: 117 CLI cases, strict CLI/PocketIC lint, Rust 1.88, fresh
storage/consumer Wasm, three installed observation/recovery cases and the actual
Chromium certificate journey's ten scenarios. Chromium journal/bootstrap suites
also pass; bootstrap covers 17 cases. The initial operator-role regression,
strict-control mismatch, sandbox socket refusal and missing selected-tool
admission remain retained. Source before-images, exact graph/artifact selection,
failed attempts and logs remain at `/tmp/blob-cleanup3.OxzNfJ/`, with copied logs
under `.tmp/review-validation/cleanup0240/`.

Installed journeys use explicit PocketIC 16.1 bytes authenticated against the
published Testkit archive pin, matching decompressed payload and version. The
lock-selected Testkit CLI is not prepared: run `make install-testkit` before
ordinary tool gates. The isolated phased candidate passes 116 CLI cases, strict
lint, Rust 1.88 and the three installed recovery cases on its separately retained
graph. Testkit passes 77 library units (four governed cases ignored), its focused
socket/framing cases, strict library lint/docs and Rust 1.88. These are local/
substitute results; native macOS, full Blob CI, Canic and deployed provider
qualification remain separate. No task-owned version, release, publication or
Git effects ran; the current numbered draft stays 0.24.0.

## Qualified extraction awaiting producer publication

The maintainer authorized extracting private run-directory custody and retained
child-output completion into Host, and coordinated Cargo archive qualification
into IC Build. Their producer implementations are applied in the authorized
sibling working trees, tracked by
[Host #94](https://github.com/dragginzgame/ic-host-tooling/issues/94) and
[Build #25](https://github.com/dragginzgame/ic-build/issues/25).
[Blob #69](https://github.com/dragginzgame/ic-blob-storage/issues/69) owns activation.

The isolated Blob replacement requires prospective Host 0.17.14 and Build 0.3.1.
IC Build is feature-gated in the private CLI's contributor-only package verifier;
public libraries/contracts and production Wasm gain no dependency on the engine.
The prepared replacement removes generic directory checks, threaded pipe draining
and shell/Perl archive orchestration while keeping Blob package/version selection,
authority, record names, recovery and release/publication policy.

Linux qualification passes 115 CLI cases, strict CLI lint, the buffered-output
regression, three installed observation/recovery cases and the actual same-version
archive/refusal fixture. The fresh storage probe and explicitly admitted Testkit
server own the installed evidence. Host passes its fs/process suites, strict
lint/docs and Rust 1.88. Build passes its full Linux gate, followed by focused
malformed-lock qualification; owner handoffs retain the exact scope.
Before-images, prospective graph/configuration, failed attempts, artifacts and raw
logs remain under `/tmp/blob-extract3.lPJtvW/`. The source-bound consumer patch is
retained in `.tmp/review-validation/extract0235/adopt-blob.patch`; its temporary
path overrides and prospective lock are qualification inputs, not adoption inputs.

These extractions leave normal Blob manifests and existing mechanics intact.
Incoming independent Cargo edits now select Memory 0.37.0 and Testkit 0.40.0;
they arrived after the frozen candidate's Memory 0.36.4/Testkit 0.39.0 checks.
Those selections are preserved and not qualified by this extraction. Because
Memory types are public, the current numbered draft is 0.24.0 rather than a
compatible 0.23.5. The following cleanup receipts retain their original graph.
Activate all three replacements only after their registry APIs are published,
then select the real producer versions and reconcile the registry lock together.
No sibling path override or temporary compatibility flow is active here. Native
macOS, full Blob CI, published adoption, deployed provider and Canic acceptance
remain separate qualifications. No release, staging, commit, push or publication
ran; actual package metadata remains 0.23.4.

## Released source and pending batch

Blob **0.23.4** is tagged at `b94dc09122b19cbfd0f21227be26933c1896bd56`.
Its release-verify source is `36e451647a1ddc3deae86dd7b352c4c573503211`.
The released lock selects Host 0.17.10, Testkit 0.39.0, Memory 0.36.4 and
Metrics 0.9.3. Shared 0.11.0 remains at
`a15ff323449f4b6190f647aec0de43cbcfb314ba`, with 116 reviewed payloads.

The originally qualified compatible **0.23.5** cleanup shares native HTTP client policy, parses
fixture arguments once and consolidates identical small fixture reply decoding.
Redirects/retries remain disabled and local traffic bypasses environment proxies;
Agent middleware and download compression policy stay explicit at their callers.
Fixture commands retain their own required fields, authority and unknown-flag
refusals. Small replies retain their 4 KiB, work, skipping and type limits;
status and inventory keep their distinct budgets. Public library/CLI interfaces
and stored service contracts are unchanged.

The preceding cleanup's incoming Host 0.17.11 selection was captured before its
checks and stayed unchanged through that qualification. Application dependencies
remain separate from
Testkit 0.39's packaged tool lock, which selects repaired Host 0.17.8 and owns
PocketIC 16.1.0. Offline selected-tool admission passes. All 115 CLI cases,
23 fixture unit cases and 27 selected installed PocketIC cases pass against
fresh artifacts; strict CLI/PocketIC lint, formatting and documentation checks
pass. Before-images, input/artifact hashes, selected commands and raw logs remain
in `.tmp/review-validation/transport0235/`. These are Linux/local-substitute
results. Full CI, native macOS, deployed provider and Canic acceptance remain
separate qualifications.

The following 0.23.5 cleanup shares descriptive native provenance inputs,
including HTTP implementation, and one browser gateway journal transition owner.
IndexedDB retains argument/row validation and strict transaction commits; the SDK
probe keeps its separate memory persistence and refusal adapter. Upload setup
holds either a permission or manifest payload and derives its permission from
that selection, retaining exact roles, request encoding and reply budgets.
Historical fingerprints and records remain unchanged.

Incoming Host 0.17.12, Metrics 0.9.4 and rustls 0.23.46 were already selected at
this task's captured baseline and remain unchanged. All 115 CLI cases, strict
CLI lint, six SDK fault-probe cases and the real Chromium journal suite pass.
Fresh artifacts pass 19 selected installed PocketIC cases covering preparation,
publication recovery, reference/gateway decisions and attestation. Raw logs,
before-images, input/artifact hashes and selections remain in
`.tmp/review-validation/state0235/`. The initial sandbox browser-tool refusal,
zero-match gateway selector and missing fixture-path attempt remain retained;
the prepared-tool check and correctly configured gateway case pass. Formatting,
dependency declarations and documentation checks pass separately. This is focused
Linux/Chromium/local-substitute qualification, with no full CI, native macOS,
Canic execution, deployed provider or paid effect.

The [0.23.4 integration evidence](../evidence/integration0234.md) retains original
body-custody, upstream-opener and CI setup qualification with its actual graphs
and failed attempts. Published Host owns regular-file opening and retained
publication; Blob retains private run claims, response binding and recovery.
[#67](https://github.com/dragginzgame/ic-blob-storage/issues/67) is closed against
delivered 0.23.4 and its matching Linux/Apple Silicon file-boundary checks.
[Host #88](https://github.com/dragginzgame/ic-host-tooling/issues/88)
retains the producer's native test-fixture scope. The earlier
[0.23.3 CI run](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38525659636)
failed macOS Testkit setup and Linux Cargo-edit admission. The selected Testkit
packaged lock and explicit native CI release-tool setup repair those causes;
that failed run remains historical evidence. Archive and tooling acceptance
[#56](https://github.com/dragginzgame/ic-blob-storage/issues/56) and
[Testkit #77](https://github.com/dragginzgame/ic-testkit/issues/77) retain their
owning required-host evidence.

The matching [0.23.4 run](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38544669237)
has complete successful Linux, Apple Silicon and Rust 1.88 lanes. Intel macOS
has passed tooling setup and exact evidence upload/readback and is running the
snapshot/formatter/release-hook fixtures at this inspection. Its archive, native
and optimized-Wasm steps remain pending. #56/#58/#59/#63/#66 retain their required
host acceptance. Exact 0.22.6 and 0.22.13 runs were cancelled on Intel; their
successful other lanes remain bound to those original graphs. #57's exact older
graph is not retrospectively qualified by this newer run. No current CI defect
was observed; dirty pending 0.24.0 has no hosted result.

## Canic boundary

The read-only 2026-10-14 review identifies Canic HEAD
`644d4b13012b395d6211a6907e2e6aafdf391db9` plus active 0.112 integration edits.
Released Canic 0.111.0 still selects Blob/contracts 0.22.2 and Memory 0.35;
no remote 0.112 tag was observed. Working root, dedicated and embedded locks now
select Blob/contracts 0.23.4 / Memory 0.36.4 in the uncommitted 0.112 batch.
Canic's current design status records completed framework journeys on 0.23.3 and
continuing 0.23.4 installed qualification. Existing results retain their own
compiled graph; lock selection alone does not qualify the new source or an
actual application/provider run.

Source review confirms exact installation decoding remains in place, including
zero skipping quota. Parent-driven dynamic initialization is still controller-only
under [Canic #493](https://github.com/dragginzgame/canic/issues/493).
Guarded funding composition [Canic #494](https://github.com/dragginzgame/canic/issues/494)
still requires authenticated provider credit/payer evidence under
[Blob #34](https://github.com/dragginzgame/ic-blob-storage/issues/34).

Family delivery remains [Canic #527](https://github.com/dragginzgame/canic/issues/527);
application/provider acceptance remains [Canic #444](https://github.com/dragginzgame/canic/issues/444).
Actual two-user/verifier acceptance is Blob #31/#33; media-map transaction #4;
serving #5; provider GC/deletion/billing #32/#6. No downstream wrapper,
fabricated provider receipt or funding bypass belongs here.

[The preceding source-bound handoff](https://github.com/dragginzgame/ic-blob-storage/blob/cedea53e13f04b1a271a5fb9d3220e0d3b062fb1/docs/status/current.md)
retains original Canic decoder-repair, dedicated/embedded composition and
incident evidence. This batch changes no Canic source or lock and runs no Canic build.
