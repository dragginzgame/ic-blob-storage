# Application and host boundary

Design continuation, 2026-11-14. This specifies the next integration of the
[offline asset catalog](../../crates/ic-blob-storage/examples/local_asset/main.rs),
not an installed host, wire schema or provider qualification. The first reference
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
4. Finish cleanup only after the exact known reference is released/retired and no
   unresolved authorization can create it later. `UnknownReference` alone is not
   completion. Resolve/replay the original authorization, then release that same
   first reference; replay may create an otherwise absent reservation solely to
   retire it. Never request upload authority during this recovery. If authorization
   remains uncertain, keep the draft unpublished with cleanup pending.

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
| Cancellation while publication waits on Blob | Installed application callback must refuse publication even if its delayed observation says Live |
| Catalog removed but release reply lost | Installed application retains exact cleanup intent; replacement asset and reference remain untouched |
| Browser impersonates A, or A proxies a bound U call | Actual signed endpoint calls must refuse claimed identity substitution |
| Two reserved roots have different uploaders/deadlines | Local root-resolution regression checks the original grant, first reference and accounting; another grant or attachment cannot substitute authority |
| G1 cancelled/expired, G2 attempts root reuse, G1 registration arrives late | Owner-local regression refuses G2 without reserving its IDs and refuses G1 registration; an installed root-only endpoint must preserve that rule |

The first and final two rows have local regressions. The first models an unresolved authorization
by applying it after an unknown release, not an IC transport schedule. Existing
example tests cover local publication/refusal. Add installed checks with a real
host and application after D0; no empty qualification package or simulated host is
needed to record these obligations.
