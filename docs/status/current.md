# Current status

Date: 2026-10-22

## Source and pending hard cut

Checkout baseline: released **0.41.1**, `d4dbfa1006106ae07ce47f207acdd3af294fc7d7`.
The maintainer approved the pending **0.42.0** provider-acknowledgment hard cut.
Package versions remain 0.41.1; no staging, commit, release or deployment is
part of this implementation. Preserve the earlier pending public-source evidence.

Dependencies are unchanged: Wire/derive 0.5.21, Platform 0.6.7,
Host/Build/Testkit 0.64.0, Memory/Storage 0.50.10 and Metrics 0.15.5.
Shared Tooling remains reviewed 0.26.4, 125 files at
`dc1ced7e240e90f0c6daa89fb6248fcb074164eb`.

## Implemented contract

[Funding](../funding-credit.md) now retains a matching successful Cashier callback
as provider acknowledgment. Its accepted attachment stays spent; subsequent
distinct payments and host-authorized renewal can pass the sequential gate within
the existing reserve, ceiling and lifetime capacity. Exact identities cannot be
redispatched. Missing/malformed replies and accepted provider errors remain
unresolved; no timeout or balance delta repairs them. Independent credit
confirmation remains for unacknowledged accepted payments, without fake receipts.
Status adds `provider_acknowledged`; `uncredited_accepted` still includes those
unverified amounts. Outcomes distinguish `ProviderAcknowledged` from independent
`CreditConfirmed`. The existing journal reconstructs disjoint totals on restore.

[GC callbacks](../gateway-gc-contract.md) now use the shared endpoint catalog and
original durable root/reference owner. Public liveness protects reservations,
live references, active reads and unknown roots. Malformed batches refuse.
Deletion listing and acknowledgment require current scoped gateway membership;
all owners' restore fences apply. At most 10,000 binary tree roots are listed.
Acknowledgment is idempotent on the exact retained released lifetime, including
late replay. It stops repeat listing without clearing physical bytes or billing
liability. No additional durable store, dependency or provider effect was added.

The 27 owned Wire methods now use the `0042` native family, while the two finite
tenant methods retain their bytes. Four required external provider methods use
Candid: certificate issuance and the three GC callbacks. Host catalog emitters
must handle the new `provider` query/update entries, use the shared `provider::gc`
decoders and delegate to `ServiceWorkflow`. The standalone adapter does so.
The generated provider DID and current typed wire vectors match this surface;
the 140-case recapture keeps tenant inputs/bytes unchanged. Previous captures
remain in released Git history, with no compatibility reader or test update mode.

Installation, funding allocation and lifecycle schemas change. Apply
[retirement/reinstall](../retiring-installations.md) while preserving old tools,
objects, uncertain payments, balances and billing obligations. There is no migration,
dual reader, automatic reset or authority to activate a stale backup. Same-release
restoration retains acknowledgments and remains fenced until existing continuity
checks succeed. Trusted registration still does not prove stored bytes.

## Ownership and remaining integration

Blob owns policy, objects, references, quotas, provider economics and durable
journals, plus standalone hosting. Platform owns IC/runtime/external Candid;
Wire owns application framing; Tooling owns Host/Build/Testkit; Persistence owns
Memory/Storage. Canic owns its wrappers, lifecycle composition and tests.
No sibling checkout was changed. Existing Canic 0.41 reports do not qualify 0.42;
its catalog emitter, DTOs, locks and guarded funding composition need adoption.
[Canic #494](https://github.com/dragginzgame/canic/issues/494) owns that funding work.

[Blob #32](https://github.com/dragginzgame/ic-blob-storage/issues/32) and
[Blob #34](https://github.com/dragginzgame/ic-blob-storage/issues/34) remain open
while this implementation is local and awaiting delivery. Do not close an issue
before its resolution is present on the default branch. The implementation updates
are on [#32](https://github.com/dragginzgame/ic-blob-storage/issues/32#issuecomment-6340636867)
and [#34](https://github.com/dragginzgame/ic-blob-storage/issues/34#issuecomment-6340658171).
Provider acceptance,
payer-link authorization, actual deletion and billing remain distinct from this
supported local profile; application acceptance remains in #31/#4/#5/#6.

With maintainer authorization, the precise provider questions are posted as
[Caffeine #16 — GC contract](https://github.com/caffeinelabs/skills/issues/16) and
[Caffeine #17 — credit/payer linkage](https://github.com/caffeinelabs/skills/issues/17).
Neither has a response at the last readback. Their answers no longer gate the
narrower local implementation. Stronger deployed guarantees need authoritative
facts or separately authorized observations. No provider/account request, payment
or deployment was made, and existing external obligations are unchanged.

## Validation and continuation

Logs and scratch recapture inputs stay in
`.tmp/review-validation/provider-hardcut042/`, including initial failed attempts.
The core suite passes 342 tests; the final read-owner consistency check has an
additional focused GC owner rerun. Contracts pass 121 unit cases, integration
cases, service export, all 140 golden vectors and doctests. Focused CLI funding
and status tests pass; the two HTTP cases require loopback access and pass outside
the restrictive sandbox. Strict workspace/all-target/all-feature Clippy passes.
Rust 1.91 native core/contracts and Wasm core/contracts/standalone checks pass.
The Linux release Wasm builds pass; standalone is separately rebuilt without
probe-only `test-support`. Deployment/runtime boundaries and documentation links
pass. Original source/provider captures remain immutable.

Mounted GC and two-payment PocketIC tests pass with labelled local substitutes.
The 18-case funding transport/interruption selection has 16 initial passes and
successful focused reruns for its two corrected test expectations. Initial
loopback server refusal, old expectation failures and scratch generator dependency
identity mismatch remain recorded. Local results do not qualify Caffeine or Canic.
The final standalone artifact surface and installed callback checks use the
isolated production selection. No full CI, native macOS or live-provider gate
is claimed. Review this coherent batch, then let the maintainer's release flow
perform its required complete gate and version transaction.
