# Current status

Date: 2026-10-12

## Released source and pending batch

Blob 0.22.13 is released at `2657177a6e089d66aa7371c5f212e9849a15a8a2`,
validated parent `df97ca3af3b3dbd7595defeef1e8f089c3025d00`. All twelve package
versions and the finalized receipt remain 0.22.13. Its released lock selects
Memory 0.35.17, Metrics 0.7.3, Testkit 0.34.5 and all four Host 0.14.9;
its Shared snapshot is 0.9.0. Publication is separate from this Git identity.

The released snapshot adopts committed Shared 0.9.0
`aec7a149f581a42b9531fc747b46024fc0314fcf`, 108 files. Existing entrypoints
remain; the new Cargo-tool freshness task/checker/fixture is included, with no
payload retirement. Atomic payload replacement, strict manifest parsing,
installer observation safeguards and Rust-catalog formatting admission remain
canonical. The online freshness diagnostic is explicit and read-only, separate
from offline admission and CI; its fixture uses a substitute registry.
Workspace LOC adds complete JSON rows/totals via
`bash scripts/dev/cloc.sh --json`; `make cloc` retains the human report. The
0.9.0 refresh changes the selected verifier, refresh helper and snapshot guide.
Refresh and verification share the manifest reader, rejecting NUL bytes before
splitting records. `--read-manifest FILE` emits records only after complete syntax
admission; it does not establish payload integrity, ownership or source trust.
Existing v1 manifests and Make commands remain supported. An initially observed
0.9.0 commit still recorded VERSION 0.8.4; the final adopted commit corrects it.

Released compatible 0.22.13 extends the standalone Candid check to compare actual
callable Wasm function exports with declared names and query/update modes. The
private harness uses Host artifact inspection; no copied Wasm parser or endpoint
list is introduced. Native generated Candid type equality remains. Existing
native-host, PocketIC and optimizer callers exercise the extended check.

The released lock includes Metrics 0.7.3 and Testkit 0.34.5, retaining Host
0.14.9 and Memory 0.35.17. The private harness directly selects already locked
Host artifacts for Wasm inspection; production dependency requirements are unchanged.
Rust development remains 1.99.0, library MSRV 1.88.0 and PocketIC 16.1.0. Package
versions, finalized receipt, index and refs remain unchanged by this inspection.
Product/native-host qualification remains separate from this focused boundary check.

Incoming Cargo edits now select direct Host artifacts/fs 0.15, Metrics 0.8,
Memory 0.35.18 and Testkit 0.34.7. The latter still owns Host 0.14.10 transitively;
private Host identities remain separate. The incoming selections are preserved. Pending 0.22.14 fixes the reproduced
record_sample removal by using Metrics MeasurementAggregate in the private probe.
Candid and JSON carry sticky actual-overflow flags for calls, bytes and instruction
totals, keeping exact caps distinguishable. Host-owned windows reset all tracking;
old heap values are not treated as tracked state. Rebuild probe and harness together;
no production schema or stable-data reset is introduced.
[Blob #62](https://github.com/dragginzgame/ic-blob-storage/issues/62) remains open
pending delivery; [the focused qualification](../evidence/metrics080-host01502214.md)
records passing native/report boundaries, original release Wasm, actual PocketIC
restoration and independent funding credit/replay windows, strict scoped Clippy,
and Rust 1.88 native/Wasm checks. The original/O3/Os/Oz static endpoint checks pass
and the missing-export substitute fails as expected. Full CI, optimized PocketIC,
native macOS and Canic/provider qualification did not run for these inputs.
The private harness now selects Host tools 0.15.0, defaults disabled with only
ic-interface, to replace manual Candid mode/export-prefix interpretation. Native
type equality and local internal-export policy remain. This is the only new lock
package/edge; existing incoming versions and entries are preserved, apart from
Host tools version disambiguation in Testkit's unchanged dependency.
Host artifacts/fs have no source changes
from 0.14.10; Memory 0.35.18 is tooling-only and Testkit 0.34.6/7 changes tooling
and an isolated lock regression. No upstream minor alone forces a Blob minor;
the delivered package's own public boundary remains the compatibility test.

Next reviewed tooling candidate is committed Shared 0.9.3
`3619de9dc36065411b8e6521f8695e9395a0e0ea`, matching remote main at inspection.
Canonical preview updates ten existing payloads and preserves all 108 selections.
It adds bounded transient-download retries, safe installer lock cleanup and
NUL/CR/LF refusal for evidence/consumer paths. Dirty sibling release/snapshot
changes are excluded. Preview only: 0.9.3 has not been adopted or executed here.
No new independent Blob runtime repair was established by this review.

Pending compatible 0.22.14 clarifies exact raw installation decoding in the
public DTO and host/service guides. [The Canic correctness review](../evidence/canic-service02214.md)
reproduces retired fields and extra application-init arguments being silently
skipped by Canic's quota of 1,024, in dedicated and nested embedded inputs.
[Canic #524](https://github.com/dragginzgame/canic/issues/524) owns the minimal
quota-zero fix and installed rejection/rollback qualification. The maintainer
then explicitly authorized applying it in Canic: the fix and focused regression
cases are now applied. The Canic native test attempt stops before those cases in
five existing Auth dead-code errors; its original log remains with Canic at
`.tmp/issue-524/native-tests.log`. That authorization is scoped to this fix,
not general sibling work. Installed rejection/rollback remains unqualified.
This native codec proof is not Canic installation or an uploader-authority bypass.
Incoming Cargo.lock edits remain untouched.

The batch also documents lexical endpoint callbacks and validation of Candid
against callable Wasm exports. Native compilation alone can miss a deferred
Candid registration. This is integration guidance; no endpoint, library type,
workflow authority, persisted schema or runtime behavior changes.

## Canic completion assessment

The normal Canic adapter now selects Blob/contracts 0.22.12 and uses
`mount_service_endpoints!`, leaving 113 endpoint lines instead of 376. It owns
endpoint attributes, bounded decoding, actual caller context, one memory runtime
and one lifecycle; Blob owns the service. No local Blob source override is used.

Read-only review confirms the original current-composition evidence in Canic
`.tmp/blob-published-20261012/`: dedicated permissions/accounting/recovery passes
in 23.80s, embedded application-state/recovery in 123s, and production Root
binding/lost-reply/conflicting-binding/exact-replay in 15.12s. Both exact 32-method
Blob interfaces pass structural Candid equality. Actual owning tests also check
Candid against every callable Wasm export. A failed imported-emitter attempt is
retained; the normal adapter now uses a lexical callback.

Dedicated build identity:
`f7aca7dd2fc40c90fcdc1a61d54eaa9c5824268e74551150f273669565830db5`.
Embedded build identity:
`72de1096e637bbf67e544f901c0b0a4003c87c598397ff3abdcda2963bfab8ea`.
Both independent fixture locks still match the final recorded hashes and retain
Auth 0.5.1, Memory 0.35.14, Metrics 0.7.1 and Timers 0.19.0 alongside Blob 0.22.11.
Later root patches are not installed in those artifacts. Canic's root lock has
advanced from its qualified `c21bc626...` selection to Memory 0.35.16; its updated
native graph requires its own proof. This review performs no Canic build or edit.

Canic's endpoint reduction and original composition journeys pass for their
recorded inputs, but the newly reproduced exact-installation decoder gap remains
#524. Its wider 0.112.0 batch remains uncommitted; published Canic stays 0.111.0.
[#61](https://github.com/dragginzgame/ic-blob-storage/issues/61) is closed for the
delivered public owner and qualified normal adopter. [Canic #444](https://github.com/dragginzgame/canic/issues/444)
retains Canic family delivery and actual application/provider acceptance.
The whole Auth/Backup release boundary remains [#517](https://github.com/dragginzgame/canic/issues/517).

“100% complete” applies to Canic's framework endpoint/composition slice, not an
end-to-end deployed storage service. [Parent-authorized project initialization](https://github.com/dragginzgame/canic/issues/493)
still lacks its application-parent route: Root binding remains controller-only.
[Guarded funding composition](https://github.com/dragginzgame/canic/issues/494)
remains pending; the adapter exposes passive funding inspection, not dispatch,
credit confirmation or renewal. [Blob #34](https://github.com/dragginzgame/ic-blob-storage/issues/34)
retains authenticated provider credit/account-activity and payer-linkage facts.
No safe provider substitute is inferred from local tests.

Actual application two-user upload/tenant authority, independent verifier
operation and the media-map transaction remain [#31](https://github.com/dragginzgame/ic-blob-storage/issues/31),
[#33](https://github.com/dragginzgame/ic-blob-storage/issues/33) and
[#4](https://github.com/dragginzgame/ic-blob-storage/issues/4). Provider GC,
physical deletion and billing acceptance remain [#32](https://github.com/dragginzgame/ic-blob-storage/issues/32)
and [#6](https://github.com/dragginzgame/ic-blob-storage/issues/6); browser serving
remains [#5](https://github.com/dragginzgame/ic-blob-storage/issues/5).
Do not add consumer wrappers/fixtures, direct funding bypasses or placeholder
provider callbacks here. Those owners need their actual external evidence.

## Validation and evidence

The original 0.22.13 development record has passing original and O3/Os/Oz static endpoint checks, along with
the expected missing-endpoint substitute refusal, affected Clippy and Rust 1.88
checking. [The current boundary review](../evidence/endpoint-surface02213.md)
records exact inputs and limitations; raw evidence remains at
`.tmp/review-validation/canic-issue-pass-20261012/`. This is not fresh PocketIC
installation or Canic/provider execution. No full CI or release gate ran.

The 0.9.0 distribution fixture passes on Bash 5.2 and genuine Bash 3.2.57 on
Linux, covering shared syntax admission, payload corruption, ownership and
interrupted refresh. Adopted-reader ordered records agree on both shells;
NUL input emits no records. Consumer Make routing, final snapshot, declarations,
shell lint and documentation checks pass. Preserved Cargo/tool/receipt hashes
and the index remain unchanged; native macOS and full CI are not run here.

The preceding 0.8.3 preview/refresh, producer distribution, consumer Make, installer and
freshness fixtures pass, including genuine Bash 3.2 on Linux for affected
consumer/installer/freshness cases. Complete offline tool admission passes after
an initial missing Testkit 0.34.4 CLI refusal and explicit setup; all attempts
remain. The online diagnostic reports all observed Cargo tools current without
changing pins or installations. No network freshness gate is added to CI.

[The latest adoption](../evidence/shared09002212.md) records the 0.9.0 reader and
refresh qualification. [The LOC adoption](../evidence/shared08402212.md) records
the 0.8.4 checks. [The preceding review](../evidence/shared083-canic02212.md)
records the Canic assessment and 0.8.3 qualification. Raw evidence is kept outside
Cargo's build tree at `.tmp/review-validation/shared09002212/`,
`.tmp/review-validation/shared08402212/` and
`.tmp/review-validation/shared08302212/`; provider/Canic evidence
stays with its existing owner. New product runtime execution, full CI/release validation,
native macOS, sibling edits, Git delivery, publication and paid effects are not
performed by this batch.

The matching 0.22.13 hosted run is
[38263673289](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38263673289).
Linux and the Rust 1.88 lane succeed, including Linux coordinated archives,
native CLI/installation/Metrics restoration, original/optimized Wasm qualification
and exact tooling archive upload/readback. Both macOS lanes are running at this
inspection; final qualification remains pending. Intel tooling archive
upload/readback already succeeds. The earlier 0.22.11 and 0.22.6 runs are cancelled.
[#56–#59](https://github.com/dragginzgame/ic-blob-storage/issues/59)
retain exact native/archive/tooling acceptance; queued or cancelled lanes do not
qualify their sources. [#60](https://github.com/dragginzgame/ic-blob-storage/issues/60)
is closed against delivered 0.22.11 and its matching validated-parent lock/source.
Earlier raw target paths are absent in this checkout; tracked historical records
remain and are not relabelled as current execution.

[Prior handoff](review-before-02212.md) preserves the preceding work/evidence.
