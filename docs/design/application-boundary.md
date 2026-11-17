# Application and host boundary

Design continuation, 2026-11-14. This specifies the integration boundaries of the
[offline asset catalog](../../crates/ic-blob-storage/examples/local_asset/main.rs),
and the [installed reference application](reference-application.md), with local
catalog qualification distinguished from live provider acceptance. The local
[transfer/read/catalog composition](reference-application.md#transfer-and-verified-read-composition--development-01009)
now joins those boundaries with the real application and an owned HTTP peer. The first reference
consumer is a public asset catalog with two uploaders and distinct references for
`scene` and `preview`. It is a reference application, not a selected production
Dragginz asset pipeline. The [main design](ic-blob-storage-design.md) owns scope,
provider gate D0 and lifecycle requirements.

## Callers and placement

For the standalone journey, application canister A owns its catalog and is the
configured Blob application authority; separate host B owns Blob state. The browser
authenticates as uploader U. Inter-canister forwarding presents A, not U, to B;
the application must authorize the exact principal U will use directly at B.
User/session mapping stays with A. See [IC caller identity](https://docs.internetcomputer.org/concepts/principals/).

A calls the existing `blob_query` from an update to obtain a replicated observation.
The [IC specification](https://docs.internetcomputer.org/references/ic-interface-spec/)
permits update-context inter-canister calls to ordinary query methods. No additional
inspection endpoint or composite query is needed. B still captures A as caller;
the originating browser's identity is not forwarded. The response describes B's
state at execution, not a lock across A's later callback or catalog mutation.

| Call | Captured caller at destination | Responsibility |
| --- | --- | --- |
| Browser asks A to create/cancel/publish an asset | U at A | A checks its user/asset policy and immutable draft identity |
| A authorizes, inspects objects/references, retains or releases at B | A at B | Blob checks its configured application authority |
| Browser obtains upload authority or inspects its upload at B | U at B | Blob checks the uploader bound to the grant; no reference-management rights |
| Browser transfers/reads file bytes | Selected provider protocol | Minimal owned client handles transfer; original declaration anchors read verification |

Do not proxy certificate acquisition through A and insert U into a caller field.
Neither a DTO, controller status nor Canic membership supplies an application role.
For embedded placement, the application's checked policy path calls the same
library locally. Its public uploader endpoint still captures U independently.
The Canic adapter supplies that host boundary; it gains no separate policy engine.

The current Rust model accepts trusted caller/clock arguments for local use. Those
arguments must never become client-selected fields in exported endpoints. Blob's
own transport needs bounded decoding before allocation and a deliberate reply
budget; the model's slice/work limits do not qualify an RPC decoder.

## Canic adapter handoff

The composition witness embeds Blob in one application canister alongside its
small asset catalog. Canic's adapter is implemented; fleet-managed creation and
maintenance remain open as described below. This exercises the local composition
boundary; the existing two-canister reference app remains the standalone witness.
It does not place Blob in Fleet's management engine or make provider support a
property of Canic. The adapter and installed witness belong in Canic; this repo
continues to export the same library and standalone host.

The current experimental supplier surface supports that local witness:

| Adapter responsibility | Existing Blob surface | Required composition |
| --- | --- | --- |
| Allocate durable control state | `local::persistence::{STORAGE_AUTHORITY, STORAGE_KEY}` | Include `blob.control.v1` under `blob` in the child's one Storage bootstrap, alongside its catalog allocation; pass the granted `RuntimeMemory<M>` lease |
| Fresh installation | `Service::create(lease, installation, Installation)` | Bind the actual child principal; use that principal as application authority for checked local catalog operations, with an explicitly configured manager |
| Reopen | `Service::open(lease, installation)` | Load the original record under the same child identity; trap on failure, never fall back to `create` or replace configuration |
| Domain mutation | `Service::execute(caller, Command, now)` | Capture IC time and uploader identity; use the internal application-authority path only after the application's own policy check |
| Read domain state | `Service::inspect(caller, Inspection)` | Preserve upload/reference inspection permissions; a public query never receives application authority merely because it runs inside the child |
| Maintenance | `Service::inspect_management(caller)` and `Service::is_drained()` | Preserve Blob's exact drain/activation attempt and configured manager; combine its readiness with the catalog and actual runtime-call draining |

Use the supplier's `protocol::{Command, Inspection, Installation, Outcome,
Observation, RPC_LIMITS}` at the selected boundary. Do not copy DTOs, add a Blob
adapter crate here, or depend on the standalone canister executable. Align shared
Storage/Platform type identities in the actual Canic build; a matching type name
or this table is not compilation evidence. The current record remains bounded to
64 historical uploads/references, with declared-byte accounting and no settlement.

For an embedded catalog, distinguish two paths explicitly. A public uploader
operation passes the captured ingress principal to Blob unchanged. An application
operation first checks the real user's catalog permission, then calls Blob locally
as the configured application authority. Never rewrite every ingress caller to
the child principal or accept an authority principal from request data. Controller
or Canic membership alone grants neither role. Do not make a self-canister call
to reach `Service`; it would add an await and lose local transaction composition.

Validate catalog conflicts and capacity before mutating Blob. A synchronous catalog
and reference update can commit in one IC message, provided subsequent persistence
or reply-encoding failures trap so both stores roll back. An ordinary Rust error
after the first mutation does not roll it back. Keep provider awaits outside this
local transaction and preserve their separate durable obligations when implemented.

Mount only exports with real callers through Canic's Platform/Wire composition.
Endpoint generation must not bootstrap storage, install timers, select a manager
or activate Blob. The child owns one init/pre/post-upgrade sequence. Ordinary
upgrade reopens both owners and leaves Blob drained until the current attempt is
explicitly activated; Canic readiness cannot substitute for that service decision.

The first installed Canic witness must demonstrate two users sharing an object,
refusal of cross-user and forged application authority, atomic catalog/reference
failure, release without harming the other reference, and stop/drain/upgrade/reopen
with both owners preserved. Include malformed endpoint admission and stale
activation refusal through the actual adapter. Reuse supplier unit evidence for
unchanged algorithms; qualify composition itself in Canic. The provider journey
and D3 completion still depend on D0 and an actual integrated transfer/read path.

### Composed witness reviewed — 2026-11-15

Read-only inspection of clean Canic commit
[`b36f6a3`](https://github.com/dragginzgame/canic/commit/b36f6a39a67f487def523c847423fdfdb8d60c50)
now finds the optional `blob` feature and
[`mount_blob_endpoints!`](https://github.com/dragginzgame/canic/blob/b36f6a39a67f487def523c847423fdfdb8d60c50/crates/canic/src/blob/mod.rs).
The macro supplies three bounded endpoints, capturing the ingress caller and time;
the application supplies synchronous handlers. It owns no Service instance,
storage bootstrap, lifecycle hook or activation policy. The embedded
[`asset-catalog`](https://github.com/dragginzgame/canic/blob/b36f6a39a67f487def523c847423fdfdb8d60c50/qualification/asset-catalog/src/canister/state.rs)
grants separate catalog and Blob leases from one Storage bootstrap. Its checked
user path calls Service as the actual installation principal without an await.
No Blob supplier change is needed for this composition.

Canic owns the [five installed cases and their evidence](https://github.com/dragginzgame/canic/blob/b36f6a39a67f487def523c847423fdfdb8d60c50/docs/demos/blob.md#qualified-inputs-and-evidence--2026-11-15):
shared-object caller isolation, atomic catalog/reference rollback, malformed
frames, drain/reopen/exact activation across distinct artifacts, and refusal of
missing/corrupt/foreign state. This review inspected the test source and retained
passing log (five tests, 7.27 seconds); it did not rerun Canic's suite here.
The retained before/after input receipts agree, and both consumer locks and Wasm
hashes match the recorded evidence. All 21 remaining supplier files in the receipt,
including the library sources, still match; only the four workspace manifests/locks
changed during the subsequent Tooling adoption. This is historical composed
evidence, not fresh qualification of that newer supplier context. The receipt
identifies Blob HEAD `50fe84b4ed303bda1802fc7a379105edbb95614e`; Canic's prose still
names an earlier HEAD. The content hashes, not that stale prose label, identify
the observed input. At that review the path dependency was development-only;
the committed follow-up below supersedes that dependency disposition.

### Fleet integration handoff — 2026-11-16

Read-only review of committed Canic
[`0167a30`](https://github.com/dragginzgame/canic/commit/0167a300451a8e0431f0c6b47a0726e87d613dfe)
confirms that immutable supplier selection is implemented: its workspace pins
Blob 0.100.6 at `decde204acb6d7c8362a0003e904beef72ff89b6`, replacing the sibling
path. The optional adapter and qualification catalog remain in Canic. Concurrent
uncommitted Auth/qualification edits were excluded; no sibling code was changed
or executed. This source review does not relabel the older installed evidence.

Canic's [current composition guide](https://github.com/dragginzgame/canic/blob/0167a300451a8e0431f0c6b47a0726e87d613dfe/docs/demos/blob.md)
documents adapter Wire 0.24.1 alongside the pinned supplier's older persistence
codec. Adoption of the available [Blob 0.100.12 revision](https://github.com/dragginzgame/ic-blob-storage/tree/9d0e343cd0d1be1f31dc20c81b67e4112d3c1023)
must qualify the actual consumer graph and retained predecessor state. Matching
DTO names or unchanged state-version numbers do not prove codec compatibility.
No registry publication or moving local source is required for that work.

The [Host install path](https://github.com/dragginzgame/canic/blob/0167a300451a8e0431f0c6b47a0726e87d613dfe/crates/canic-host/src/operation/mod.rs#L450)
and inspected management-engine creation/upgrade paths still supply empty
application arguments. The existing embedded catalog requires its own explicit
installation type (two users, manager and Blob limits) and method-bound upgrade
unit. Blob's `blob app prepare-installation` instead prepares inputs for the
separate reference application and standalone Blob host. Its files are not the
embedded Canic catalog's configuration or a signed deployment capability.

The remaining integration belongs to Canic's existing fleet approval/effect owners:

| Boundary | Required next behavior |
| --- | --- |
| Supplier adoption and fresh installation | [Canic #12 — Provision embedded Blob applications through approved fleet installation arguments](https://github.com/dragginzgame/canic/issues/12): qualify the selected immutable supplier, carry exact explicit arguments and actual target/controller bindings through existing plan/apply custody |
| Managed maintenance and reopening | [Canic #13 — Integrate Blob drain, upgrade and explicit activation into fleet maintenance](https://github.com/dragginzgame/canic/issues/13): preserve the original attempt and outcomes through drain/stop/upgrade/start/inspection and separately authorized activation |

The current simulator witness uses a trusted controller and an explicit manager.
It does not demonstrate Root-owned creation or fleet plan/apply custody. Canic's
current empty lifecycle arguments are deliberately refused by this application.
Do not add a second management engine or relax Blob admission to bridge that gap.
D3 is partial: embedded local composition is witnessed, while managed deployment
and the provider upload/read journey remain unqualified. Standalone use remains
independent; no Canic dependency or duplicated composition suite is added here.

No additional Blob supplier API is identified for these two tasks. `Service`
already owns create/open, execute/inspect, manager inspection and drained-state
admission; Canic owns the actual deployment route and endpoint mounting. A manager
canister's caller identity must match Blob's configured manager; a host operator's
control of that manager is not equivalent. Fresh local fleet qualification can
proceed on the existing Storage contract. It does not depend on resolving the
consumed live enrollment probe's allocation-growth migration or establishing
Caffeine acceptance. Those obligations remain with their existing owners.

The [legacy library's host-grant contract](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage/src/lib.rs)
already separated allocation bootstrap from the complete service owner. Keep that
ownership rule; use the rewrite's one `Service` record rather than importing legacy
component factories, funding journals or lifecycle orchestration.

## Application decisions across delayed replies

A owns one bounded draft record per attempted asset use: the original authorization,
asset name, uploader and first reference, its publication/cancellation decision,
and any unresolved authorization or release. These are application coordination
facts, not another Blob liveness/accounting ledger. Reserve their cleanup capacity
before dispatch. Keep retired identities until an eventual qualified stale-request
rule permits compaction; no TTL-only eviction or generic receipt framework.

1. Persist the exact draft before asking Blob to authorize it. Retry that same
   authorization after an uncertain reply; never allocate a fresh ID as recovery.
   A definitive rejection before admission differs from a missing/uncertain reply.
2. For publication, require the original declaration and exact draft/reference.
   Use an authenticated current Blob observation, not browser-supplied liveness.
   After any await, re-read the draft and asset binding before the local catalog
   commit. Cancellation permanently prevents publication of that draft.
3. All releases go through A's same draft/catalog policy. Record cancellation, or
   remove the exact matching catalog binding and record its release obligation,
   before dispatching a remote release. A late liveness response cannot undo this.
   Keep the reference retained while published; a diagnostic observation alone
   does not lock it against concurrent release.
4. Finish cleanup only after the exact known reference is released/retired, or its
   authorization is conclusively expired without admission; neither case may leave
   unresolved authorization that can create it later. `UnknownReference` alone is not
   completion. Resolve/replay the original authorization, then release that same
   first reference; replay may create an otherwise absent reservation solely to
   retire it. Never request upload authority during this recovery. If authorization
   remains uncertain, keep the draft unpublished with cleanup pending.

The experimental host's exact `Authorize` returns retained IDs before checking
expiry. Its typed `Expired` refusal therefore means the ID was never admitted and
the deadline has passed at Blob. That one result permits completion without a
release. Immutable saved input and [monotonic canister time](https://docs.internetcomputer.org/references/ic-interface-spec/canister-interface/#system-time)
prevent later admission of the same request. An admitted grant must still be
released, even after its deadline. Neither the app's clock nor an expiry error from
another operation supplies this proof. This depends on the supported state-preserving
lifecycle; reinstall/restore and incompatible replacement code are outside it.

During the service's [upgrade drain](ic-blob-storage-design.md#admission-while-draining-and-reopening),
existing-reference cleanup and historical replay remain permitted when state is
valid. A replay that would create a previously absent grant is new admission and
must wait. Keep that draft cancelled with cleanup pending; do not allocate new IDs
or treat a maintenance refusal as proof that authorization never happened.

The remote removal order deliberately differs from the synchronous heap example:
catalog removal and Blob release cannot be one cross-canister transaction. A failed
release may leave excess retention, but must not leave a published asset whose
reference A has already released. IC responses execute separately and other messages
can interleave; see [message execution](https://docs.internetcomputer.org/references/message-execution-properties/).
Embedded composition can be atomic only when both actual stores share the message's
rollback semantics; ordinary Rust errors must still precede mutations.

The cooperating example verifies reads before publication. A browser saying that
verification passed is not proof to A that it read those bytes. The initial reference
application may accept an authorized uploader's publication decision, explicitly as
application policy. A production application requiring independent verification
must select its trusted verifier before claiming that stronger publication rule.
Blob stores no global Verified flag, and publication supplies no access secrecy.

The local catalog stores reference IDs only. Application-authorized reference
inspection returns their bound roots and current local status, including after
cleanup; the example resolves both published assets through that owner. This does
not make a prior Live observation safe to publish after an await or permit an
uploader to inspect application-wide references.

## Provider-facing identity gate

The reviewed SDK calls the certificate method with only a root. Local registration
now accepts a decoded root and resolves its original grant; the earlier upload-ID
registration entrypoint has been replaced. A host must never select the latest
grant. Besides simultaneous ambiguity, consider a delayed root-only request
from U after cancellation of grant G1 and admission of G2 for the same root/U.
It carries no G1/G2 identity. One currently eligible grant does not resolve that
historical ambiguity.

The maintainer selected the conservative rule: every admitted root keeps its
original upload lifetime, including after cancellation/expiry before registration.
The local model now refuses G2, including for a different uploader. Matching uses
of an Open object attach references without another registration right. The root
claim occupies bounded history permanently; this restriction means cancelled
content cannot restart under the same root. There is no configuration switch.
The local registration transition resolves that lifetime and rechecks its original
uploader, state and deadline before changing references/accounting. It exposes no
reusable eligibility permit and selects no attached use. An eventual host must
preserve this rule and never treat a prior assessment as a lock. Actual provider
scope, original-proof recovery and closure remain D0 requirements; the local rule
does not qualify a certificate endpoint or retract an already issued certificate.

## Evidence and acceptance

Read-only legacy review at `25bed1352f3d2b9659f0d2583f50fa9af859ea84` covered
[host caller capture](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage-canister/src/api/mod.rs),
[certificate boundary tests](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage/src/ops/service/uploads/tests/certificate_boundary/mod.rs),
and [reference outcome tests](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage-cli/src/native/submit_reference/tests/mod.rs).
The [legacy certificate resolver](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage/src/ops/service/uploads/certificate/mod.rs)
also resolves the root's original permission before checking its actual uploader
and committing exposure. This continuation retains that lookup/authorization order
in the existing local transition, without the stable-index machinery, text parser,
assessment endpoint or provider reply. There is no parallel upload-ID entrypoint.
Retain captured identity, exact original permission, rechecking after assessment,
and the separation of historical result/current liveness/uncertainty. Do not import
endpoint macros, tenant routing, CLI dispatch journals or recovery stores. No source
code from these files is copied in this continuation.

| Witness | Present evidence or required installed check |
| --- | --- |
| Unknown release, delayed authorization, then cleanup | New owner-local regression exercises cleanup both before and after local registration; late authorization replay cannot resurrect the use |
| Cancellation while publication waits on Blob | Installed real application refuses a pending cross-subnet publication after local cancellation |
| Catalog removed but cleanup reply discarded or remote call fails | Installed application retains exact cleanup intent across upgrade; replacement asset and reference remain untouched |
| Browser impersonates A, or another uploader knows an ID | Signed host tests cover direct identities; installed application captures its caller and denies another user's draft operations |
| Two reserved roots have different uploaders/deadlines | Local root-resolution regression checks the original grant, first reference and accounting; another grant or attachment cannot substitute authority |
| G1 cancelled/expired, G2 attempts root reuse, G1 registration arrives late | Owner-local regression refuses G2 without reserving its IDs and refuses G1 registration; an installed root-only endpoint must preserve that rule |

The first and final two rows have local regressions. The first models an unresolved authorization
by applying it after an unknown release, not an IC transport schedule. Existing
example tests cover local publication/refusal. The installed application now
qualifies its durable decisions and actual callbacks separately from the D0 gate;
an integrated provider transfer/read journey still requires that contract.

The [signed standalone witness](../../qualification/ic-blob-storage-tests/tests/signed.rs)
now exercises the existing production host through an owned PocketIC HTTP gateway.
Distinct deterministic signing identities represent application, manager and two
uploaders. Wrong callers cannot authorize/register; uploaders cannot release or
retain application references or inspect application-wide usage. A second uploader's
attached use does not acquire the original uploader's registration right.
After deliberately discarding a completed registration reply, a fresh Agent inspects
the same upload and a repeat registration refuses. Application cleanup and historical
authorization replay retain released references and unresolved declared exposure.
Queries verify signatures and updates verify certified outcomes against the fixture's
independently obtained root; a wrong root must fail certificate verification.

This carries forward the inspected legacy certificate-boundary rule that the root
locates the original permission, plus its reference-result distinction between an
old outcome, current liveness and retry authority. It uses existing Platform Agent
and Wire APIs, with no CLI journal, provider proof, adapter or product change.
The application authority in that test is a native signer; it does not qualify
canister forwarding, cancellation during a callback, network interruption or recovery
of a usable provider certificate.

`make test-application` separately installs the real Blob host and the product
[reference application](reference-application.md). The application has immutable users/target, durable drafts
and bounded-wait calls, with no automatic retry. It exposes catalog operations,
not arbitrary Blob command forwarding. The installed test qualifies publication
and cancellation across actual calls, delayed authorization, exact retry after a
discarded cleanup reply, stopped-host failures, same-format upgrade and capacity
exhaustion with cleanup still possible. Publication leaves Blob stable bytes unchanged.

The targeted legacy review rechecked its host caller capture and
[reference-status regressions](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage/src/ops/service/uploads/tests/reference_status_boundary/mod.rs).
Retain exact caller/target checks and inspection after release or suspension; use
the existing rewrite projection instead of legacy tenant/binding DTOs and fences.
Its invalid/changed-input refusals also motivate the app's installed boundary checks:
invalid drafts preserve their IDs, while wrong-method, trailing-byte and oversized
frames at each of its five hooks preserve both app and Blob stable state. These
qualify this host's wiring, not Wire's full codec matrix or legacy DTO compatibility.
The [legacy browser publication flow](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/clients/browser/publication.js)
and its [regressions](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/tests/browser/publication.mjs)
reinforce original-input binding and cancellation rechecks after awaits. The
[recovery orchestration](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage/src/workflow/installation/recovery/mod.rs)
also revalidates its original binding after asynchronous effects. Retain those
invariants here through one immutable draft and final local checks; omit the
legacy catalog framework, recovery fences and platform-history machinery.
No legacy code was copied. These witnesses qualify the local reference application,
not a live provider, arbitrary state replacement or production asset pipeline.

The [legacy installed lifecycle regression](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/qualification/ic-blob-storage-pocketic-tests/tests/standalone_lifecycle/mod.rs)
also checks that rejected replacement settings preserve the prior installation and
obligations. The reference-app journey retains that regression through its existing
unit upgrade argument contract, without legacy owner fences or recovery orchestration.
It additionally proves that the app's pending-call guard traps an actual upgrade
while a cross-subnet Blob call is outstanding, then allows that original call to
finish with stable state unchanged. No fixture host or production fault mode is used.
The same rejection-preserves-state principle now covers three disposable reference
applications: empty memory, invalid storage and a populated record copied from a
different installation. These are new cases for this app's reopening boundary,
not copied legacy recovery logic. The simulator injects bytes only into disposable
recipients; the original catalog, cleaned history and Blob ledger remain intact.
This does not qualify recovery from an older snapshot of the same installation.

The [legacy admission path](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage/src/ops/service/uploads/admission/mod.rs)
binds exact permission recovery to its original uploader/deadline. Retain that
no-renewal invariant, without copying its permission/fence machinery. The narrow
nonadmission meaning of `Authorize::Expired` is established by this rewrite's own
lookup order, not inferred from legacy errors. The installed journey reproduces
the previously stuck cancellation and proves cleanup without Blob mutation,
renewal refusal and explicit release of a grant admitted before its deadline.

### Types 0.21.2 integration

Blob’s 0.102.0 public records and Service arguments select registry Types
0.21.2. Canic must use that same Rust type at direct Service crossings. Blob’s
standalone host now uses canonical caller/canister identities directly from Runtime
0.23.14; the temporary host byte conversions have been removed. The library does
not depend on Runtime. The native CLI uses the canonical identity type; there is no
legacy Platform conversion in product code.

### Platform replacement prerequisites — 2026-11-16

Both workspaces select Platform 0.34.13, Protocol 0.3.5, Runtime/Macros/Guard
0.23.16 and Types 0.21.2. Testkit 0.108.0 shares Protocol 0.3.5 for held replies.
Host Artifacts/FS 0.80.3 and Process 0.80.4 retain the existing custody and teardown
APIs. API and persistence remain unfrozen.

| Consumer boundary | Selected owner and retained contract |
| --- | --- |
| CLI signed transport and certificate custody | Platform owns PEM signing, exact packet admission, one-attempt dispatch, original-request observation, signed queries and proof authentication. Blob owns custody, deadlines and admission. No legacy Agent or certificate adapter remains; the new anonymous APIs do not broaden Blob's caller policy. |
| External Caffeine protocols | Protocol owns root/certificate, liveness/deletion, account-info and top-up codecs. Blob retains root/owner checks and provider uncertainty. |
| External management protocols | Protocol owns status/start/stop and ordinary install/upgrade arguments, preserving absent upgrade flags and exact module/argument bytes. Status decoding requires the canister version. |
| Standalone and reference-app hosts | Runtime owns explicit entrypoints, caller/self/time, pre-copy input bounds and consuming calls with a 30-second bounded wait. Domain and record formats are unchanged. |
| Stable allocation backing | Storage 0.74.1 retains Runtime 0.22.5's IC0-only edge; no second executor or guard. Keep Storage-issued handles. |
| Installed qualification | Testkit owns prepared-server admission, bounded process lifetime, explicit instance deletion/process teardown and held replies. Blob selects deadlines and asserts cleanup. A tiny synthetic WAT peer supplies raw enrollment replies. |

[Protocol 0.3.5](https://docs.rs/crate/ic-platform-protocol/0.3.5), published from
[`379789a`](https://github.com/dragginzgame/ic-platform/tree/379789a1b9db1e9348a5673ed847896e5011f81c),
supplies the final qualification codecs requested in
[Platform #24](https://github.com/dragginzgame/ic-platform/issues/24). They are now
adopted; `ic-platform-legacy`, Platform 0.33.7 and Types 0.20.4 are absent from both
graphs. No generic Candid implementation or direct upstream dependency was added.

| Qualification consumer | Adopted boundary |
| --- | --- |
| `tests/capacity.rs` | `decode_canister_metrics` retains arbitrary execution-cycle charges until checked u128 conversion. Balance differences remain a separate observation. |
| Enrollment example's `cashier.rs` | `decode_top_up_result` preserves an opaque, validated `Ok` payload or typed refusal. The JSON still makes no credit, acceptance or settlement claim; the retained actual TopUpWithoutCycles reply remains the regression vector. |
| `tests/enrollment.rs` | `decode_top_up` checks the actual dynamic owner and target, preserving the outer option, field options and canonical Principal identity. |
| Callback/enrollment refusal fixtures | Literal Candid schemas exercise short/long roots, wrong profiles, extra arguments and wrong result types independently of the supplier encoder. Candidate replies must match the exact `vec blob` profile, including root lengths. |

Existing byte/header/type/work/allocation caps remain explicit. Vector and
skipped-value caps are supplied to the new decoders. Callback fixtures contain
only their fixed test schemas and one-byte counts, not a general wire codec.
The legacy sibling is unavailable locally; migration preserves the rewrite's
existing regression contracts and retained vectors rather than importing a runner.

Current evidence is under `target/platform-full-adoption/`: aggregate CI,
installed capacity, four enrollment cases, the offline enrollment example and all
seven non-Chromium callback cases pass. This includes authorization refusal,
unchanged stable memory on malformed input, delayed replies after final release,
retained original-request recovery after real expiry, serial transfer and
fresh-process replay refusal. Lock hashes and Cargo metadata confirm the final
graph. Optional Chromium, native macOS and live-provider qualification are not
claimed; no full release gate or publication was performed.

Prior broader release acceptance remains in
`target/01031-release/release-final.log`; the previous Host/Testkit adoption's
application and server qualification remains in `target/supplier-refresh/`.
Those observations retain their original supplier versions, not the current
cohort. Older migration evidence, including populated predecessor upgrades and
released-CLI custody, remains in `target/platform-testkit-adoption/`.

Offline installation preparation/readback retains exact signed requests under
`blob app`; see the [installation guide](reference-application.md#prepare-and-inspect-one-signed-install).
Guarded dispatch/observation admits current target/controllers, claims before one
submission and retains original certified outcomes before current module/controller
readback. It still requires operator-owned fresh-creation custody and supplies no
independent application-configuration read or continuity proof. Live owner/probe
custody remains unchanged.

Custom networks now require both an independently trusted `--root-key` and
`--root-subnet`. The simulator's root subnet comes from its owned topology, never
an untrusted HTTP response. Mainnet inspection uses the public root DER and NNS
subnet constant retained from published ic-agent 0.49.3; these are trust data,
not a transport implementation. Current network observations admit certificates
up to 300 seconds old and 30 seconds ahead; delegations up to one day old.
Offline retained-proof checks admit historical time back to the epoch with the
same future bound. Neither policy proves current availability or provider acceptance.

Original bytes, expiry and request IDs survive migration. Invalid metadata or
signatures refuse without repair or replacement requests. Recovered certificate
files retain the exact authenticated bytes. The late-recovery fixture now waits
for actual ingress expiry instead of advancing simulator time alone, so it also
checks the explicit network freshness window. npm Platform 0.2.0 is unchanged.
The legacy sibling is unavailable in this checkout; this migration preserves the
existing rewrite's regression contracts and previously retained protocol vectors.

Acceptance evidence is under `target/platform-testkit-adoption/`: aggregate CI,
all eight installed suites’ cases, populated predecessor upgrades and exact
released-CLI request custody. The final scoped reruns cover current certificate
dispatch/transfer and the controller-loss case; network rejection stays explicitly
uncertified. Runtime’s pre-upgrade trap replaces the old panic prose while the
fixture still proves refusal, resumed original calls and unchanged durable state.
Optional Chromium and live-provider qualification are not claimed.

#### Concrete host contract

The standalone [codec hooks](../../crates/ic-blob-storage-canister/src/codec.rs)
check input size and the fixed 11-byte Wire method/direction header before copying
the full argument. Runtime must support that ordering, either through bounded raw
inspection or equivalent guard composition. Blob retains Wire decoding and its
limits. A synchronous output-encoding failure must trap in the same execution,
rolling back the handler's stable mutation; converting it into an ordinary error
reply would change the transaction contract.

The [standalone host](../../crates/ic-blob-storage-canister/src/lib.rs) needs init,
pre-upgrade, post-upgrade, query and update exports. The
[reference application](../../crates/ic-blob-storage-reference-app/src/host.rs)
also needs a 30-second bounded awaited call with owned arguments. It records
intent before dispatch and classifies failure as uncertain; Runtime must not
retry, resume durable business operations or turn timeout into proof of no effect.
Its in-flight guard must be released on normal completion and callback cleanup;
stop/drain still precedes upgrade. Capture authenticated caller/self before any
await and use IC system time for grant deadlines. The private enrollment probe
also needs controller membership checks. The protocol does not permit a delayed
reply to reopen a cancelled draft.

These requirements overlap the supplier's
[next Runtime priority for Auth](https://github.com/dragginzgame/ic-platform/blob/c12cb56e78e9aaeb982201d0bde597480aaf2816/docs/requirements.md#next-runtime-release-priority-auth-execution-requirements).
Use that single system/export/executor owner. Blob adds the specific Wire
pre-copy and rollback witnesses, not another entrypoint generator.

The authored Wasm modules and their Runtime dependencies are target-scoped; native
DTOs and the enrollment record-capacity test remain available. Workspace unsafe
code is denied, permitting only Runtime's narrowly expected generated export
attribute. The core, CLI and native qualification roots retain explicit
`forbid(unsafe_code)`. No unsafe body was added. The rollback probe uses Storage's
existing memory backing for its one raw qualification page; production storage
continues to use allocation grants.

Read-only legacy research revisited its [host ownership contract](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage/src/lib.rs)
and [installed host regression](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/qualification/ic-blob-storage-pocketic-tests/tests/standalone_installation_cli/mod.rs).
Retain actual-host identity checks, exact method/mode boundaries and host-owned
lifecycle composition. No old runner, recovery framework or custody was copied.

#### Concrete native protocol contract

The current CLI selects `BasicIdentity` from PEM and an explicit network trust
root. The replacement must sign an exact ingress envelope offline, expose its
request ID and expiry, inspect its content strictly, dispatch the retained bytes,
and authenticate the original request status without re-signing or renewing it.
Keep effective routing separate from the signed recipient: management requests
target the management principal but route through the operated canister.
Document any HTTP-level retries; the current Agent may repeat identical signed
bytes for 429/503, so application-level single dispatch is not a one-HTTP-request
guarantee. Retain those distinctions in the replacement's tests and documentation.

The required external codecs are bounded projections, not application Candid:

| Protocol | Existing inputs and admitted outputs |
| --- | --- |
| Management status/start/stop | `CanisterIdRecord`; status/controller/module-hash fields consumed by admission, and exact empty acknowledgement for start/stop |
| Management `install_code` | Ordinary `Upgrade(None)`, exact target/Wasm/Wire upgrade args, absent sender version; no reinstall fallback or skip-pre-upgrade override. Current Wasm allowance is 1 MiB and encoded request allowance 2 MiB. |
| Upload certificate method | One root string; exact one-result `{ method, blob_hash }` matching `upload` and the original root. Preserve original CBOR request/certificate bytes and request-status proof. |
| Cashier `account_info_get_v1` | Account principal; bounded result with matching owner, signed integer cycle balance, four usage gauges and last-charge timestamp. Preserve known error variants and refusal of malformed/unknown results. This remains a query observation, not settlement evidence. |
| Qualification enrollment | Exact `account_top_up_v1` request bytes already stored by the private probe, zero attached cycles, bounded raw reply retention and controller check. No new live experiment is needed for migration. |

The new Platform already supplies bounded certificate decoding and direct-root
or delegated authentication with explicit time and authority inputs. Reuse it
when the native transport migration is possible; do not list BLS verification as
missing or reimplement it. Admission of an old retained certificate must select
an explicit historical-evidence time policy, separately from current network
freshness; authentication alone neither proves provider acceptance nor permits
another transfer. Test wrong root, wrong effective canister, mismatched original
request/reply, expired ingress recovery, missing status and malformed evidence.

#### Migration order and completion evidence

1. **Completed:** Types 0.21.2, significant-byte host crossings, local and installed
   predecessor-state checks. Keep the independent library usable without Canic.
2. **Runtime implemented:** 0.23.13 supplies system/lifecycle composition through
   its existing owner. Qualify standalone `test-endpoints` and
   `test-standalone`, then the reference application's `test-application` and
   affected callback/enrollment paths. Recheck Storage's actual Wasm backing
   edge; an old IC0-only dependency is not itself a second executor. Initial migration
   logs are under `target/runtime-02311-adoption/`. The endpoint fixture now checks each
   malformed call's rejection and unchanged stable bytes, not old decoder prose.
3. **Native migration:** adopt the available signed transport and exact protocol
   codecs together, then qualify `test-signed`, `test-management` and certificate
   callback/recovery cases. Read previously generated synthetic custody without
   rewriting it. Do not use live `.operations/` custody as a migration fixture.
4. Remove each obsolete host byte conversion or old dependency when its actual
   boundary has migrated. Admit both locked graphs and public Principal exchanges;
   no blanket same-version requirement for independent native test suppliers.
   Provisioning resumes only after the required migration is qualified or the
   maintainer selects a different course. The supplier issues above own the missing APIs.

Runtime 0.23.11 acceptance passed: both native Clippy graphs, all five Wasm roots,
and the standalone, endpoint, application, enrollment, callback, capacity and
signed maintenance installed suites. Standalone/application upgrades use the
retained pre-migration Wasm artifacts. Capacity remains within existing limits;
no budgets were raised. Browser/Chromium and live-provider acceptance are not
claimed. These results qualify the authored Runtime migration, not the missing
native Agent/codec replacement.

Runtime 0.23.12 requalification passed all five authored Wasm roots under strict
Clippy and the standalone, endpoint, application, enrollment and callback suites,
including populated pre-migration host/app upgrades. Its executor now distinguishes
method-reply roots from global-timer roots; Blob adds no timer use. Existing
reply rollback, delayed completion and signed-request recovery witnesses pass.
Both lockfiles already selected the new cohort when this refresh began; both
manifest minimums now match. Locked metadata, dependency pins, tooling binding,
formatting and local documentation links pass. Evidence is under
`target/runtime-02312-adoption/`; the successful installed run required local
loopback access for PocketIC. No native replacement, Chromium run or live-provider
qualification is claimed by this patch refresh.

Platform 0.34.6 / Runtime 0.23.13 / Types 0.21.2 acceptance is recorded under
`target/platform-0346-adoption/`. Strict CLI and native qualification Clippy,
all five Wasm roots, 13 CLI tests and installed standalone, callback and management
suites pass. The new envelope checks preserve IDs from Agent-produced historical
requests and refuse unknown authentication fields, trailing data and truncation.
Populated pre-migration host state reopens; signed maintenance preserves its
original claims and request recovery. Both locked graphs, dependency pins,
tooling binding, formatting and documentation links pass. The updated CLI is on
PATH. Chromium and live effects were not run; remaining legacy transport/codec
uses keep overall Platform adoption partial.
