# Service acceptance plan — proposed B1 assignments

These cases specify observable acceptance for the bounded journey. They are
not implemented tests or passing evidence. Named owners, bounds and provider
evidence are still required by the [service contract](service-contract.md).
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

Retain exact source revision, tool/provider versions, artifact/Candid hashes,
commands and results for executed cases. Record substitutes separately from
actual provider observations. A07's Canic-owned replacement fixture evidence
does not authorize changes in that repository. Passing service tests alone
does not close provider suitability, installation retirement or publication.

[The Canic parity contract](canic-parity.md) expands the preservation requirement
into concrete runtime, operator and diagnostic capabilities. Its source-test
references are not replacement test results.
