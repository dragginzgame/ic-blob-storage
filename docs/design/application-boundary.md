# Application and host boundary

Design continuation, 2026-11-14. This specifies the integration boundaries of the
[offline asset catalog](../../crates/ic-blob-storage/examples/local_asset/main.rs),
and the [installed reference application](reference-application.md), with local
catalog qualification distinguished from the unimplemented provider journey. The first reference
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
| Browser transfers/reads file bytes | Selected provider protocol | SDK owns transfer; original declaration anchors read verification |

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

The next composition witness should embed Blob in one Canic-managed application
child alongside its small asset catalog. This exercises the local composition
boundary; the existing two-canister reference app remains the standalone witness.
It does not place Blob in Fleet's management engine or make provider support a
property of Canic. The adapter and installed witness belong in Canic; this repo
continues to export the same library and standalone host.

The current experimental supplier surface is sufficient to start that local witness:

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

Read-only consumer inspection on 2026-11-14 found Canic's management endpoint
adapter described in [Canic #2](https://github.com/dragginzgame/canic/issues/2),
but no Blob mount. Its working tree has pending changes; none were edited or
adopted here. That fixed management macro is not a general Blob adapter.
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
