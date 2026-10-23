# Current status

Date: 2026-10-22

## Source and current batch

Released Blob **0.42.0** is local HEAD `aeb884d` (release commit), following
implementation `b1b1d55`. Memory/Storage now select 0.50.11; earlier implementation
logs selected 0.50.10. The current adoption selects published Wire/derive 0.6.2
and Host/Build/Testkit 0.64.1; Platform stays 0.6.7, Metrics 0.15.5 and Shared
Tooling 0.26.4. Incoming lock edits also select Platform 0.6.8; preserve them.
Do not treat old focused checks as qualification of a changed graph.

Pending **0.43.0** migrates the two finite tenant methods to Wire's native codec.
Their new identities are `0043/001a` and `0043/001b`; failure tags are zero-based.
Borrowed 29-byte principal bounds and the 512-byte frame limit remain; native
traversal budgets are explicit. Removed Inspect/Update markers have no shim.
Other method identities/bytes and persisted representations remain unchanged.
Package release versions remain 0.42.0. Pending documentation fixes are preserved.

## Implemented boundary

[Funding](../funding-credit.md) retains a matching Cashier success as provider
acknowledgment, sufficient for another distinct bounded payment. Accepted cycles
remain spent. Missing/malformed replies and accepted errors remain unresolved;
no timeout, balance delta or repeated dispatch repairs an uncertain payment.
Independent credit confirmation remains available for unacknowledged acceptance.

[GC callbacks](../gateway-gc-contract.md) share the existing root/reference owner.
Public liveness protects reservations, live references, active reads and unknown
roots. Listing/acknowledgment require current scoped gateway authority. All owner
restore fences apply. Acknowledgment stops repeated listing without clearing
physical bytes or billing liabilities. No additional durable owner was added.

The 27 owned methods now share native Wire; non-tenant identities stay `0042`.
Certificate issuance and three provider GC methods use Candid.
Installation, funding allocation and lifecycle schemas require
[retirement/reinstall](../retiring-installations.md), preserving old tools,
objects, uncertain payments, balances and liabilities. No migration or dual
reader exists. Same-release recovery remains fenced until continuity checks pass.
Trusted registration still does not prove stored bytes.

## Canic adoption and remaining ownership

Read-only review on 2026-10-22 inspected Canic's dirty working tree based on
`cf2d3d1fba84a00a76cdbc541ae7e68fe36e9bb5`, its root/two independent locks,
provider emitters, GC tests and retained `/tmp/canic-blob042/` logs.
Published Blob/contracts 0.42.0 are selected. Dedicated/embedded managed GC and
restore runs pass; emitters use the shared decoder/workflow and exact Candid
unit acknowledgment. No duplicated GC policy or Blob API blocker was found.
No Canic files were changed and its tests were not rerun here.

[Canic #494](https://github.com/dragginzgame/canic/issues/494) now has local guarded
prepare/dispatch/credit/renewal composition and refunded/concurrent-call tests.
Accepted-payment, partial-refund, positive-credit and renewal qualification awaits
published cycle-accepting fixtures from Tooling #235. Canic has adopted Wire 0.6
for optional metadata but still consumes released Blob 0.42/Wire 0.5; after Blob
publication it must update all locks and qualify the new tenant frames.
[Canic #622](https://github.com/dragginzgame/canic/issues/622) reports both mounted
callback races qualified on Testkit 0.64.1 and awaits delivery. Canic's uncommitted local
qualification is not default-branch delivery, deployment or provider acceptance.
See [the adoption recipe](../funding-consumer-qualification.md).

[Blob #32](https://github.com/dragginzgame/ic-blob-storage/issues/32) is closed after
default-main delivery; deployed deletion requirements remain on #6.
[#34](https://github.com/dragginzgame/ic-blob-storage/issues/34) retains payer-link
and external funding qualification. Actual application/provider qualification stays
with #31/#4/#5/#6. Payer linkage, physical deletion and billing cessation remain
unverified. [Caffeine #16](https://github.com/caffeinelabs/skills/issues/16) and
[#17](https://github.com/caffeinelabs/skills/issues/17) ask about stronger guarantees;
answers no longer gate the narrower supported local profile.

## Validation evidence and limits

Prior implementation logs remain in `.tmp/review-validation/provider-hardcut042/`:
342 core tests, contracts/golden vectors, focused CLI tests, strict workspace
Clippy, Rust 1.91 core/contracts and standalone Wasm, isolated production Wasm
surface, mounted GC and two-payment PocketIC cases passed on their recorded graph.
Failures and successful focused reruns are retained, not erased.

Canic's retained logs report dedicated/embedded PocketIC, strict Clippy and
adapter Rust 1.91 Wasm/native consumer checks. Managed consumer Wasm used the
development compiler; native macOS and deployed provider qualification are absent.
This pass changes only documentation and runs documentation links/diff checks.
No live provider/account request, paid effect, release or deployment was made.

## Delayed-reply fixture review

Testkit 0.64.1's held-reply API was reviewed against Blob's existing gateway
source and transport/recovery callers. Keep the local durable source: pending
upgrade refusal, old source-journal restoration and bounded exhaustion are
required tests outside the generic held controller's lifecycle contract.
Existing transport tests already cover cancellation/revocation before late replies.
See [fixture ownership](../dependencies.md#delayed-provider-fixtures). No new
fixture, dependency feature or runtime change is justified by this review.
The subsequent explicit adoption selects published Testkit 0.64.1, without
enabling held-reply fixtures. Preserve incoming dependency edits.

Adoption checks pass: full locked metadata, declaration/runtime boundaries,
strict workspace Clippy, contracts including frozen vectors, 342 core tests and
Rust 1.91 standalone Wasm compilation. Logs are retained under
`.tmp/review-validation/tooling0641-wire0522/`. Testkit/Build CLI setup completed.
Full CI, fresh installed PocketIC execution and native macOS were not rerun.

Wire 0.6.2 migration qualification is in `.tmp/review-validation/wire062/`:
121 contracts unit tests, integration/doctests and all 140 frozen cases pass;
342 core tests, strict workspace Clippy, dependency boundaries, Rust 1.91
standalone Wasm and the release Wasm build pass. The installed standalone tenant
ingress case passes, including retired-frame refusal without mutation and valid
authorized updates. Initial native-budget refusals remain in the earlier logs;
the final explicit budgets admit full-width values and all failure variants.
No full CI, native macOS or Canic build was run for this pending migration.
