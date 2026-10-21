# Current status

Date: 2026-10-21

## Delivered source and pending work

The pending **0.40.4** contributor-tooling fix keeps dependency fetching in
explicit preparation and release preflight, outside ordinary validation; see
[#98](https://github.com/dragginzgame/ic-blob-storage/issues/98). It changes no
product APIs, dependency selections or stored/wire formats.

Default main is released **0.40.2**, `dc3620f467b9d39ac9b001a6cc2c254553ad65c9`.
[PR #97](https://github.com/dragginzgame/ic-blob-storage/pull/97), probe evidence
regressions and Wire integration-recipe corrections are delivered. Pending
**0.40.3** adopts published IC Tooling 0.63.2 and replaces native probe body
accumulation with its cancellation-safe HTTP reader. Package versions remain
0.40.2; no commit, release or publication was run by this adoption.

Preserve incoming Cargo.lock edits: Tooling 0.63.2, Platform/macros 0.6.4,
Memory/Storage 0.50.7, Metrics 0.15.2 and Wire/derive 0.5.14 were already selected
in the captured before-image. This adoption adds only HTTP and its CLI edge to
that lock, and sets direct Tooling requirement floors to the API-bearing release.
Keep dependency edits outside an active validation gate; results remain bound
to their exact graph.

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

- [#96](https://github.com/dragginzgame/ic-blob-storage/issues/96): published
  `ic-host-http` 0.63.2 is verified against its sparse-index archive checksum and
  packaged VCS `e5b4095150f49069fc51c3b497e192a8f8eda016`. The released Rust
  matches the earlier isolated candidate. Local adoption uses `read_body_into`,
  preserving caller-owned timeout bytes, returned-error prefixes, body-before-
  status ordering and existing labels, with distinct `body_allocation` evidence.
  Request/client policy, hashing/redaction and durable custody remain local;
  provider downloads remain streaming verifier/file sinks. Reqwest remains
  necessary for those callers. HTTP is native-only with empty defaults and does
  not enter either CLI or standalone Wasm. Leave #96 open until this pending
  batch reaches default main; publication is no longer its blocker.
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

Adoption evidence: `.tmp/review-validation/tooling0632/`, including verified
published archives/source match, incoming before-images and exact graph inputs.
Full locked metadata, Platform/dependency/runtime/deployment boundaries and pins
pass. Native CLI all-features normal/build graphs select one HTTP, Reqwest,
Host process and Build identity; CLI/standalone Wasm exclude HTTP/Reqwest.
Strict workspace all-target/all-feature Clippy, 128 native CLI/probe tests,
actual Rust 1.91 native workspace and supported Wasm checks pass. Probe cases
cover truncation, oversize and timeout prefixes; the allocation-record fixture
checks retained evidence but does not induce allocator exhaustion. Production
Wasm and native CLI build against the same graph.

Explicit canonical Testkit/server and release-editor setup/offline admission
pass. The first setup attempt refused a live shared lock owned by IC Persistence;
no lock was removed. Retry followed that owner's completion. Both focused
installed cases pass: complete compiled endpoint surface and CLI-carrier
installation with wrong-actual-service refusal. Formatting and local links pass. This is local Linux evidence, not native macOS,
installed Canic or deployed-provider qualification. Shared Tooling remains 0.26.1.

Historical complete-gate evidence remains bound to its original graph:
`.tmp/review-validation/wire058-gate/ci.log` records all 20 targets passing in
930 seconds with 1,995 tracked file contents unchanged, including PocketIC,
original/O3/Os/Oz installation/restoration and archives. It does not qualify today's
incoming lock. `.tmp/review-validation/backlog0400/` separately retains the
registry-only Blob 0.40.0 / Platform 0.6.1 installation/restore compile probe.
Current native macOS, installed Canic and live-provider acceptance remain separate.
[Shared #294](https://github.com/dragginzgame/shared-tooling/issues/294) and
[Tooling #223](https://github.com/dragginzgame/ic-tooling/issues/223) retain producer
qualification independently. Earlier graph-specific evidence stays in the
[2026-10-20 handoff](handoff-2026-10-20.md),
[2026-10-18 handoff](handoff-2026-10-18.md) and [history](history.md).
