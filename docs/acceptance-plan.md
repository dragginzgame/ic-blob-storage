# Service acceptance plan — proposed B1 assignments

These cases specify observable acceptance for the bounded journey. None is
fully qualified. The authorized core slice provides native identity and billing
arithmetic evidence toward A02/A04/A08/A11, including incremental raw-byte
length/digest verification and rejection recovery toward A02, recorded in
[core evidence](evidence/core-primitives.md); it does not execute the service
journeys below. Named owners, bounds and provider
evidence are still required by the [service contract](service-contract.md).
Local lifecycle binding and direct-tenant policy tests now provide partial A01
evidence over supplied values; they do not authenticate endpoint callers or prove
durable isolation, delegated access or provider confirmation authority.
The test-only [PocketIC probe](evidence/core-primitives.md#pocketic-authority-probe-after-018)
adds actual caller/controller A01 and release/revocation A06 evidence over sample
transient objects. Forged tenant/service fields grant no access; owner release and
replay preserve the other tenant's usage; revoked gateways lose pending-list access.
This does not qualify either production adapter, persistence or provider behavior.
The same fixture now adds actual inter-canister A01/A06 sync interleavings:
an old reply cannot undo revocation or overwrite a completed newer sync; an
overlapping attempt sends no second source request. Malformed/oversized/empty
replies and transport rejection preserve membership and perform no automatic retry.
The controlled source is a substitute, not Cashier behavior or provider qualification.
Local streaming Caffeine-tree verification adds partial A02 evidence from
independent current-client vectors: chunk boundaries, uneven trees, metadata
ordering and rejection/continuation after invalid input. It computes both raw
digest and provider root without buffering the object. Empty provider objects,
trusted expected identities, serving policy, persisted checkpoints and actual
upload/read completion remain unqualified.
Root-bound chunk manifests additionally provide native A02/A03 evidence for
out-of-order leaf verification, retry after corruption and rejection of changed
chunk order, metadata, position or length. This is stateless byte verification;
it does not execute an interrupted download or persist resume progress.
An in-memory chunk verifier adds unique-position A02/A03 evidence: duplicates
cannot fill a missing position, corrupt retries preserve coverage, and exact
partial-chunk lengths determine byte progress. It retains observations only;
destination writes, interrupted downloads and durable resume remain unqualified.
Ordered composition additionally checks each leaf before advancing a whole-file
raw verifier. Corrupt chunks can be retried at the same position, while skipped
or replayed chunks reject. Full input still requires the separately expected raw
digest at finalization; matching manifest leaves alone cannot bypass that check.
Bounded missing-chunk pages add A03/A04 local scheduling evidence: independent
scan/result limits, exact partial ranges, continuation through empty filtered pages
and current coverage after intervening checks. Pages neither reserve in-flight
reads nor prove completion, and do not establish a provider range-read protocol.
The local PocketIC probe now executes A02/A03 byte checks in actual Wasm using
full-leaf, partial-final-leaf and metadata vectors, corruption retries and typed
wrong-digest/truncated failures. The ordered append has a fixture instruction
budget; this does not qualify ingress handling, production throughput or providers.
Native consumer-reference reads add A01/A06 coverage for tenant checks before
status disclosure, bounded ordered batches and inactive released references
while sibling references stay live. They preserve physical/billing obligations
and do not qualify gateway liveness or provider deletion.
Native multi-object catalog composition adds partial A04/A06/A08 evidence for
atomic local admission, tenant quota, separate global physical/billing-byte caps,
zero-byte obligations and retained receipt/root history. Bounded deletion-page
and root-observation tests add A01/A06 coverage for namespace filtering, caller
revocation between pages and explicit unknown results. They do not execute paid
uploads, monetary accounting, persisted transactions or restore recovery.
Local reference receipts additionally provide partial A03/A04/A06 evidence for
exact retries and reserved release capacity. No interruption/restore or paid
provider retry case has been executed by those native tests.
Tenant catalog reads add native A01/A03/A06 coverage across multiple objects:
ownership checks precede supplied bindings; unknown/foreign roots reject alike;
mixed batches disclose no partial statuses. Exact receipt queries preserve old
successes/failures while current liveness changes, including settlement/capacity,
without recording or reapplying requests. Absent local receipts do not qualify
provider retry or restore recovery.
The scoped gateway registry adds native A01/A06 coverage for membership revocation
racing a previously started sync: stale replies cannot restore revoked membership,
and current callback policy continues to reject that caller. Wrong service,
namespace and Cashier context reject before decoding. Bounded Candid reply
composition preserves membership and the pending attempt on malformed, over-budget
or invalid lists. These tests compose local values only; provider transport and
durable revocation remain unqualified.
Account-balance reply/policy composition adds partial A08/A11 coverage: provider
errors and invalid/wrong-account replies cannot become a zero-balance funding
suggestion, later valid observations recover diagnosis, and recovery fences
remain blockers. This is native codec/policy evidence, not a real status workflow.
Run the same service cases through standalone and managed deployments; only
managed lifecycle integration belongs to Canic. Both adapters live here.

Provider compatibility cases must target the latest official integration pinned
in [the baseline record](provider-baseline.json), refreshed before implementation
and qualification. Test vectors and interface expectations must trace to that
upstream source/release and the verified deployment, not Canic's historical
snapshots. Package currency does not establish paid-effect or restore safety.

| ID | Trigger and observable result | Evidence owner and method |
| --- | --- | --- |
| A01 — authority | Tenant A uploads; tenant B and an otherwise privileged Canic controller cannot resume, access restricted service metadata or release A's reference. Wrong service, provider namespace, actor or epoch rejects without changing reservations, references or invoking provider effects. Public provider URLs convey no byte confidentiality; A10 records that boundary | Service/consumer owners, unassigned; native authority predicates plus PocketIC endpoint cases for both adapters |
| A02 — content | Empty, boundary-sized and multi-chunk bytes produce the specified content digest/provider identity. A changed byte, bad length or invalid checkpoint cannot complete verified upload/read; durable references contain no credentials | Service owner, unassigned; native fixed vectors and checkpoint tests, actual-provider hash/read evidence, both PocketIC journeys |
| A03 — interruption | Interrupt before effect, after provider completion but before response, and before local completion persistence. Restart/retry preserves the same exact operation and conservative reservations. Replayed completion is effect-free; expired evidence cannot authorize a second uncertain paid effect | Service owner, unassigned; native transition tests, PocketIC interruption/replay, separate provider retention/idempotency evidence |
| A04 — capacity | Exhaust object, reference, byte, session, receipt and liability bounds individually, including concurrent requests. Admission rejects before provider effects; failures and retries cannot leak or double-release counters. Tenant quota release leaves still-stored bytes in global accounting | Service owner, unassigned; native arithmetic/transition tests and PocketIC races with observable counters |
| A05 — restore | Restore sequence 40 after paid operation 41 completes. Restored and concurrent stale instances remain fenced until surviving authority/evidence establishes safe identities, balances, references and liabilities. Unsupported restore paths reject before effects; supported same-release restore resumes without identity reuse | Service/operator owners, unassigned; PocketIC backup/restore and concurrent-instance cases plus proof that real authority survives the selected restore boundary |
| A06 — release race | Race upload/resume with release and duplicate/stale deletion callbacks. No live reference is deleted. Logical release, physical deletion and billing stop advance separately; liabilities persist until their own evidence arrives. Consumer release outbox survives application interruption | Service/consumer owners, unassigned; native model invariants and PocketIC interleavings through both adapters |
| A07 — composition | Build the standalone core/canister without Canic. The managed Component delegates to the same service handlers and tenant predicates. Libraries alone export no endpoints; adapters export the same blob Candid plus only their required lifecycle endpoints. Restore precedes deferred work | Service owner plus Canic runtime/testing owners, unassigned; package dependency inspection, exact Wasm/Candid provenance and real PocketIC install/lifecycle journeys |
| A08 — provider economics | Lose a top-up response, delay provider deletion and continue charging after logical release. Exact evidence resolves each operation without duplicate payment; unresolved outcomes stay fenced and accounted. Settle residual balances under explicit operator authority | Service/operator owners, unassigned; actual-provider source/deployed evidence, bounded approved provider qualification, separately labeled PocketIC substitutes |
| A09 — retirement | Attempt destructive reset while uploads, uncertain effects, balances or billing remain. Require no-obligation evidence or completed operator disposition retaining evidence, authority and funded ownership outside erased state | Installation operator, unassigned; per-installation evidence review, service fence tests where applicable; no migration engine or old reader |
| A10 — serving | Exercise the consumer's chosen MIME, download and public-serving policy, including active content and corrupted data. Public-by-reference behavior is explicit; no unsupported confidentiality promise | Consumer/service owners, unassigned; provider serving evidence, client tests and both deployment journeys |
| A11 — operator parity | Run status/check-ready, gateway sync and funding through the real operator client against both PocketIC deployments. Preserve strict input validation, machine-readable results, readiness failure signaling and post-action diagnostics. Prove no mutations on status/diagnostics/dry-run; a failed post-status read never repeats a completed mutation. Reject wrong network/identity, missing methods and ambiguous managed targets before effects | Service/operator owners plus Canic host owner for generic discovery, unassigned; native parsing/target tests, packaged operator client and PocketIC transport/effect observations |
| A12 — removal readiness | Resolve every capability in the source-bound Canic inventory to working replacement behavior and evidence; demonstrate shared implementation, independent artifacts and retained generic Component coverage before authorizing B3 removal | Service and Canic runtime/host/testing owners, unassigned; exact source/artifact/Candid provenance, executed replacement tests and inventory review, separately owned installation retirement |

Before implementing a case, freeze its exact inputs, numeric bounds, expected
typed outcomes and observation points. Tests must observe no unintended effects
as well as returned results. Do not assert error prose, aggregate test counts
or removed schema forms. Keep unit tests beside maintained code; use `tests/`
and PocketIC for canister/install/lifecycle/inter-canister behavior.

The [provider recovery findings](provider-review.md#recovery-findings--2026-09-26)
make three cases explicit: A02 must not confirm upload from client progress or
a returned hash alone; A06 must reject an old root-only callback even when a
newer incarnation is also deletion-pending; A08 must distinguish a structured
Cashier error from transport success and reconcile an uncertain payment before
retry. The retained client/codec probes are partial source evidence, not executed
service or provider acceptance cases.

The local reply decoders now have native tests for A02/A08: missing/duplicate
upload status, all advertised funding error variants and malformed/over-budget
replies. A06 additionally has immutable-root-claim and lifecycle composition
coverage, including a newer deletion-pending object and retained claims after
settlement. These establish local rejection behavior; authoritative upload
completion, persisted claims, callback authentication and actual provider
recovery/settlement remain unqualified.

Retain exact source revision, tool/provider versions, artifact/Candid hashes,
commands and results for executed cases. Record substitutes separately from
actual provider observations. A07's Canic-owned replacement fixture evidence
does not authorize changes in that repository. Passing service tests alone
does not close provider suitability, installation retirement or Canic removal.

[The Canic parity contract](canic-parity.md) expands the preservation requirement
into concrete runtime, operator and diagnostic capabilities. Its source-test
references are not replacement test results.
