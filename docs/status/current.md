# Current status

Date: 2026-10-14

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
