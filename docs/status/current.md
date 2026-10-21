# Current status

Date: 2026-10-21

## Delivered source and pending work

Default main is `18b543cd8ad14b630c6bed513264839ffe557853`, merging
[PR #97](https://github.com/dragginzgame/ic-blob-storage/pull/97) after release
**0.40.1**. Probe exact-bound, truncated-response and timeout-prefix regressions
are delivered; the production HTTP reader has not changed. Pending **0.40.2**
contains declared-compiler qualification and current integration-recipe corrections.
Package versions remain 0.40.1; no release or publication was run by this review.

Preserve incoming Cargo.lock edits. At review they select Platform/macros 0.6.3,
Memory/Storage 0.50.6, Metrics 0.15.2 and Wire/derive 0.5.12. They predate this
batch and were not changed or runtime-qualified here. Keep dependency edits
outside an active validation gate; results from earlier graphs are not reusable
current runtime qualification.

## Ownership and delivered tooling

Blob owns tenant policy, durable service workflows, provider accounting and
recovery, plus its standalone adapter and native/browser primitives. Consumer
frameworks own their wrappers, lifecycle composition and integration tests;
applications own login/grants, verifier operation and asset transactions.
Platform owns IC identities/runtime and required external Candid; Wire owns
application framing/schema. Host/Build/Testkit belong to
[IC Tooling](https://github.com/dragginzgame/ic-tooling); Memory/Storage to
[IC Persistence](https://github.com/dragginzgame/ic-persistence).

Shared Tooling **0.26.1**, revision `67d4f25d1a613a27d7dfd7b5a4ae119e0e529fc7`,
has a verified 125-file snapshot. Only `wasm-opt` is selected for IC tool setup.
Production optimizer builds are separate from probes to exclude `test-support`.
Tooling 0.63 Testkit/server and release-editor setup/admission explicitly select
`cli`; application libraries do not. Retain actual consumer routing, admission,
evidence and release recovery checks; generic matrices remain with Shared.
Cross-release transitions remain retirement/reinstall only; same-release recovery,
exact retries, fences and retained obligations remain required.

## Outstanding issues and current evidence

Eight Blob issues remain; no PR is open at this review. #84 and #95 are resolved
on main for application DTO decoupling and published Platform-family alignment.
PR #94 was superseded; PR #97 is merged. Do not conflate these producer deliveries
with consumer or deployed-provider acceptance.

- [#96](https://github.com/dragginzgame/ic-blob-storage/issues/96): leave HTTP
  adoption waiting on compatible registry publication. The tested candidate and
  unapplied registry patch remain in `.tmp/review-validation/host-http-adoption/`.
  Inspect the actual published archive before selecting it. Preserve timeout
  partial bytes, body-before-status ordering and existing outcomes; distinguish
  allocation failure. Request policy/evidence remain local and verified provider
  downloads remain streaming. No path override is selected.
- [#32](https://github.com/dragginzgame/ic-blob-storage/issues/32) and
  [#34](https://github.com/dragginzgame/ic-blob-storage/issues/34): the
  [fresh provider source review](../evidence/caffeine-probes/local/2026-10-21-provider-contract0402-01/summary.json)
  finds public application GC/top-up implementation unchanged. Gateway failure,
  physical deletion/billing and uniquely attributable credit evidence remain
  unestablished. The README lookup failed with 404 and is retained; no retries,
  live provider calls or paid effects. Do not fabricate handlers or receipts.
- [#33](https://github.com/dragginzgame/ic-blob-storage/issues/33) and
  [#31](https://github.com/dragginzgame/ic-blob-storage/issues/31): actual worker/
  signer operation and two authenticated application users through the deployed
  provider remain open. The verifier recipe now names passive `permission.wire`
  and `statement.wire`, separate method-bound frames and the IC CBOR envelope.
  Old interrupted runs retain their original tools; no conversion is supplied.
- [#4](https://github.com/dragginzgame/ic-blob-storage/issues/4),
  [#5](https://github.com/dragginzgame/ic-blob-storage/issues/5) and
  [#6](https://github.com/dragginzgame/ic-blob-storage/issues/6): consumer asset-map
  transaction, deployed serving/CSP/cache/integrity, overlapping releases and
  independently evidenced deletion/billing remain required. Existing publisher,
  reference/capacity and local browser results do not close those cells.

[Canic #444](https://github.com/dragginzgame/canic/issues/444#issuecomment-6313616819)
now reports local Blob 0.40 dedicated/embedded composition and production Root
initialization/recovery success, with distinct compatible root/private graphs.
This is owner-reported pending local work, not default-main delivery or execution
performed here. Canic must deliver its batch and reconcile its adoption tracking;
provider/application acceptance remains separate. [Canic #494](https://github.com/dragginzgame/canic/issues/494)
owns guarded funding composition after #34 establishes acquisition.
[IC Crates #1](https://github.com/dragginzgame/ic-crates/issues/1) retains its
older graph and complete archive/provider/verifier/catalog transaction.
No sibling checkout or excluded application was changed or executed here.

## Validation and evidence

This documentation/source-review batch uses `.tmp/review-validation/backlog0402/`
for before-images, source identity and focused validation. It changes no Rust,
manifest, decoder, storage schema, authentication or retry behavior. New source
captures and their failed request remain in the probe ledger above. The local
documentation links check passes (1,523 references across 205 documents), all
retained evidence checksums pass, and the affected recipes no longer prescribe
retired application codec paths. No runtime or native macOS test was run for this
documentation-only batch.

Historical complete-gate evidence remains bound to its original graph:
`.tmp/review-validation/wire058-gate/ci.log` records all 20 targets passing in
930 seconds with 1,995 tracked file contents unchanged, including PocketIC,
original/O3/Os/Oz installation/restoration and archives. It does not qualify today's
incoming lock. `.tmp/review-validation/backlog0400/` separately retains the
registry-only Blob 0.40.0 / Platform 0.6.1 installation/restore compile probe.
The PR #97 merged-revision published-example workflow has passed; its broader
release/formatting workflow is still running at review, not green evidence.
Current native macOS, installed Canic and live-provider acceptance remain separate.
[Shared #294](https://github.com/dragginzgame/shared-tooling/issues/294) and
[Tooling #223](https://github.com/dragginzgame/ic-tooling/issues/223) retain producer
qualification independently. Earlier graph-specific evidence stays in the
[2026-10-20 handoff](handoff-2026-10-20.md),
[2026-10-18 handoff](handoff-2026-10-18.md) and [history](history.md).
