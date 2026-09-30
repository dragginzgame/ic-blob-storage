# Service acceptance plan

Qualification follows the [Caffeine probe ledger](evidence/caffeine-probes/README.md).
Provider cooperation is not required, but public source review, local substitutes
and deployed observations remain different evidence classes. Unsupported guarantees
need an explicit operating-contract decision; repeated success is not a universal
retry, retention or billing guarantee. This plan authorizes no deployment or paid
trial. No complete acceptance case below is qualified.

## Current local evidence and open gates — 2026-09-30

This review covers the released 0.4.6 inspection/lifecycle source and the
uncommitted shared memory-grant and immutable installation continuation. It supersedes older
implementation-gap summaries here;
historical capability records retain their original dates and limitations.
No Canic/Toko removal-inventory refresh or consumer acceptance is implied.

| Cases | Implemented and locally evidenced | Still required for full acceptance |
| --- | --- | --- |
| A01 — authority | Shared service/tenant rules, explicit operator/uploader/verifier roles, scoped endpoints and signed native queries; controllers do not gain tenant access | Concrete consumer and managed-adapter journey with the same rules |
| A02 — content | Streaming provider-root/content verification, bounded manifest handling, explicit trusted-verifier completion and immutable attestation receipts | Qualified real certificate/upload path and independent deployed byte fetch; empty-object/serving limits remain explicit contract questions |
| A03 — interruption | Durable service and fixture-consumer intents, atomic IC rollback, exact historical recovery and native one-shot verifier submission | Actual provider lost-response/charging behavior and production consumer outbox/worker acceptance; local files are not independent freshness authority |
| A04 — capacity | Bounded objects, references, receipts, sessions, reservations and liabilities; reserved release slots and separate logical/physical/economic accounting | Actual provider pre-charge bounds and production sizing for the selected consumer |
| A05 — restore | Synchronous standalone restoration of all owners into inspection-only fences; stop/start and repeated current-release upgrades preserve local state; whole-canister rollback risk is demonstrated | A qualified operational recovery boundary and complete independently surviving reconciliation; current upgrades do not resume mutations and old snapshot loads are unsupported for operation |
| A06 — release race | Exact reference mutation/receipt/status, tombstones, indexed cleanup, revocation and stale callback rejection over local IC substitutes | Deployed deletion and billing-cessation evidence plus accepted production consumer release coordination |
| A07 — composition | Canic-free core and explicit standalone host use shared handlers; shared immutable installation/persistence, named grant composition and synchronous fenced restoration are implemented | Canic adapter, generic managed lifecycle/guard/allocation replacement evidence and both-adapter acceptance |
| A08 — economics | Maintained Caffeine codecs, exact local transfer/refund journals, liquidity/attachment guards and conservative reconciliation; standalone account inspection is wired | Complete production spendability/liabilities and provider-credit/account activity evidence; reported balances are not receipts |
| A09 — retirement | Contract separates source removal from installation retirement and preserves uncertainty/continuing obligations | Per-installation inventory and an accountable, evidenced settlement/disposition decision; no reset authority is supplied here |
| A10 — serving | Root-bound local verification and explicit reference-qualified descriptors/read sessions; native provider observation has exact budgets and retained artifacts | Consumer MIME/public-serving acceptance, deployed serving evidence and confidentiality boundary |
| A11 — operator parity | Signed native status, funding history/outcome, upload history, certificate assessment, reference receipt/status and verifier tooling; standalone gateway/account handlers also exist | Qualified operator mutation workflows, managed target discovery and both-deployment journeys. Read-only native coverage does not imply sync/funding command parity |
| A12 — removal readiness | Source-bound Canic capability inventory and maintained replacement evidence | Every required behavior working here, Canic-owned generic coverage transfer, refreshed removal inventory and separate retirement review |

Current source-bound local records include:

- [Shared host composition](evidence/core-primitives.md#shared-immutable-installation--2026-09-30):
  whole-candidate validation, grant preflight and passive restoration of retained
  configuration/accounting; focused production standalone lifecycle and authority
  tests. This does not establish managed Canic deployment or provider behavior.
- [Funding inspection](evidence/caffeine-probes/local/2026-09-30-funding-outcome-cli-01/summary.json)
  and [reference history/status](evidence/caffeine-probes/local/2026-09-30-reference-native-01/summary.json):
  actual native signing/trust, typed refusals, exact original intent, passive
  durable history and fenced inspection. Fixture provider outcomes are substitutes.
- [Upload history](evidence/caffeine-probes/local/2026-09-30-upload-history-cli-02/summary.json)
  and [certificate assessment](evidence/caffeine-probes/local/2026-09-30-certificate-cli-02/summary.json):
  bounded inventory progress and the current production blockers, without
  mutation/retry authority or issuance readiness.
- [Whole-host lifecycle](evidence/caffeine-probes/local/2026-09-30-standalone-lifecycle-01/summary.json):
  retained installation and admission/manifest across stop/start and repeated
  fenced upgrades. Funding/read owners are empty in this case; occupied/in-flight
  recovery evidence remains separately scoped in [core evidence](evidence/core-primitives.md).
- [Snapshot findings](evidence/caffeine-probes/local/2026-09-30-snapshot-01/summary.json):
  loading an old whole-canister snapshot can bypass upgrade hooks and forget later
  history. Deferring external journals/controllers does not close this requirement.

Production signing and durable standalone configuration are implemented.
Provider qualification, operational recovery, Canic integration, production
operator mutations, consumer acceptance and installation retirement remain open.
Libraries, local test artifacts and release publication do not qualify those gates.

Run the final service cases through standalone and managed deployments using the
same handlers. Both adapters and the operator client belong here; Canic supplies
generic managed lifecycle/discovery. Fixture clients alone do not qualify
production identity or deployed observations. Signed native tooling now has local
standalone/durable-fixture evidence; managed integration, operator mutation parity
and actual-provider acceptance remain open under A07/A11/A12.

Provider compatibility cases must use the current selected integration in the
[baseline record](provider-baseline.json), refreshed before provider implementation
and qualification. The current local recovery batch did not refresh upstream or
contact a paid provider. Package currency alone establishes no recovery guarantee.

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

## Release-published media consumer

The [Toko Miner feedback review](roadmap.md#toko-miner-feedback--2026-09-27)
adds proposed acceptance inputs alongside Toko; no Miner adoption is approved.
Freeze the consumed media inventory, numeric bounds and explicit principal/namespace
bindings before executing these cases. These extend A01–A11, not executed evidence:

- A01/A03/A11: a noninteractive publisher rejects wrong credentials/targets and
  preserves exact operations through interrupted uploads, lost replies and failed
  frontend publication. Dry-run is effect-free; uncertain work cannot start a new
  paid upload. No frontend map advertises unconfirmed or unretained required media.
- A04/A06/A08: overlapping releases reuse tenant-authorized live objects through
  distinct references. Failed publication, old-reference retirement, removed-media
  reintroduction and late callbacks preserve ownership and liabilities. Exercise
  retained-history exhaustion with cleanup still possible and report remaining
  capacity. Reintroduction needs a supported contract before this case can pass.
- A02/A10: real browser GLB/image/texture loading obeys the chosen delivery trust
  contract, rejects corruption/truncation and exercises MIME, CORS, CSP, cache and
  error behavior. Measure cold/warm startup, bytes and latency; define the bounded
  retention horizon for cached/open browsers before releasing old media.

## Provider qualification sequence after 0.1.12

This is an execution proposal for the selected provider, not an executed test or
authorization to create an account, transfer cycles, upload, delete or deploy.
It makes the [four provider questions](provider-review.md#focused-provider-questions--prepared-not-sent)
actionable. Keep source-contract review, PocketIC substitutes and actual-provider
results distinct. Do not build a provider emulator from guessed server semantics.

The investigation now supplies DFINITY's documented Rust onboarding and linked
payment-account flow; see the [implementation findings](provider-review.md#findings-that-change-the-implementation-plan).
Before production funding, a targeted PocketIC substitute must demonstrate
call-specific refunds under partial cycle acceptance, application errors,
malformed replies and callback traps. Record offered, refunded, transport-accepted
and provider-credited amounts separately. SYS_UNKNOWN cannot establish acceptance
from a zero refund. Compare wait strategies without claiming the substitute
proves Cashier's credit, retry or audit behavior. The unbounded-call subset now
passes, including callback rollback; see [funding evidence](evidence/core-primitives.md#funding-callback-experiment).
Same-release fixture upgrades preserve journals and uncertainty through ic-memory;
timeout/SYS_UNKNOWN and old-backup recovery were not exercised. Select the latest
backend's blob deletion list; qualify deployed interoperability before freezing
the wire adapter, without implementing the older example's text fallback.

### Inputs required before execution

Record the approved service/consumer canister principals, network, gateway,
Cashier, provider project/bucket, billed account and callback authority together.
The provider must establish the relationship between these fields. Toko's current
project-canister owner cannot silently become the new service principal; identify
existing-object/balance disposition separately from a fresh test installation.

Also record exact source/interface revisions, authoritative lookup methods and
receipt fields, numeric evidence-retention and maximum reconciliation intervals,
supported restart/restore boundary and an operator responsible for unresolved
charges. Set explicit total spend, offered-cycle, object/session/request and
elapsed-time caps before running. Missing values block execution; neither fixture
limits nor retail credit pricing supply a production or Cashier-cycle budget.

Proposed byte fixtures are UTF-8 `abc` (3 bytes), a deterministic sequence
`byte[i] = i mod 251` of 1,048,593 bytes (one 1 MiB chunk plus 17 bytes), and an
empty object only if the provider contract qualifies it. Use explicit MIME and
length metadata, independently calculated raw digests and provider roots. These
are test inputs for review, not new supported size defaults. If the provider's
bounds or empty-object behavior differ, resolve that contract decision before
execution. A fixture's expected bytes cannot substitute for provider completion.

### Order and required observations

| Step | Exercise | Evidence required to advance |
| --- | --- | --- |
| 1 — bindings | Review the supported independent-canister arrangement and existing installation inventory | Exact ownership/payment/callback relationship; no placeholder project or inferred account; source revision and accountable operator |
| 2 — protocol | Map certificate issuance, tree/chunk upload, completion lookup, deletion and funding to documented server operations | Request identity, reply/error meanings, retry charging, retention and settlement semantics; Candid method existence alone is insufficient |
| 3 — local composition | Once the contract is frozen, implement host-owned memory, shared handlers and both adapters; run the same failure cuts below in PocketIC | Intent, root claim and reservations commit before effect exposure; restoration precedes deferred work; typed denials and observable accounting agree across adapters |
| 4 — approved provider trial | On the explicitly authorized isolated namespace, upload and verify the byte fixtures; exercise documented lost-response lookup without automatically repeating the write | Independently correlated completion and read bytes, exact observed request count, account charges and retained operation evidence |
| 5 — obligations | Release references, observe physical deletion, then reconcile final billing; separately exercise one approved funding operation and its documented lookup | Distinct logical/physical/economic completion, exact payment/refund amounts, no unresolved balance erased at teardown |
| 6 — disposition | Review every operation, object, receipt and residual balance from the trial | Proven closure or preserved evidence, authority and funded reconciliation ownership; hitting a test limit must stop new effects, never clear outstanding records |

### Failure cuts for the shared journey

For each case, record the immutable request and provider identity, starting and
ending reservations/byte obligations, observed external calls, original receipt
and current state. Keep the raw provider evidence necessary for correlation;
the final service decision must be reproducible from it. A client `blob_complete`
status, `existing_chunks` hint, successful decode or changed balance alone is
insufficient for the transitions below.

| Cut or race | Required result |
| --- | --- |
| Before any authority escapes | A known unexposed reservation may cancel once; exact retry preserves history and cannot issue a second intent |
| After certificate/effect exposure, before completion is known | Retain conservative capacity; transport failure does not reset to unexposed or authorize another paid write |
| Provider completed, response lost | Independently fetch and verify the original owner/project/root through the configured verifier, or retain uncertainty. Persist/reconcile its exact attestation using the service receipt. This does not resolve unknown paid-operation charges or permit a blind upload repeat |
| Completion received, before local completion is durable | Recover the same operation; applying its evidence transfers accounting once and does not reactivate released references |
| Receipt retention expires | Keep the unresolved obligation fenced; expiry is not evidence of failure, refund or safe identity reuse |
| Restore an older backup after an effect | Surviving authority must account for effects missing from the backup and exclude stale/concurrent instances before admission resumes |
| Release while upload is unresolved | Preserve the pending root and capacity until the qualified provider outcome determines the next lifecycle transition |
| Duplicate or delayed root-only deletion callback | Authenticate current gateway/namespace authority and correlate original ownership; never fill the binding from a newer incarnation |
| Physical deletion confirmed, billing continues | Release physical capacity only; keep economic obligations until their distinct authoritative evidence arrives |
| Funding or post-action status reply lost | Keep exact payment intent unresolved or return its established result; a failed diagnostic read never resubmits the payment |

This sequence does not close the provider contract. Caffeine is the sole provider
target. Use the published Canic/Toko and Rabbithole integration patterns recorded
in the provider review to investigate each row; consumer source is not a server
guarantee. Report specific unsupported capabilities before changing scope, and
continue to preserve Canic's removal gate.
