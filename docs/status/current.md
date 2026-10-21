# Current status

Date: 2026-10-21

## Delivered source and pending work

Default main is released **0.40.3**, `5fcb43c1a87b2c2ab02cd86cc2fbc9eb3200080a`.
HTTP adoption [#96](https://github.com/dragginzgame/ic-blob-storage/issues/96) is
closed: the delivered registry-based reader passed Linux, Intel macOS, Apple
Silicon and MSRV qualification. Request/evidence policy remains local, and
provider downloads remain streaming. This establishes no provider acceptance.

Default main now includes [PR #99](https://github.com/dragginzgame/ic-blob-storage/pull/99)
at `2dbfcefda956667aac0b806d4230c11138f3aa46`, pending **0.40.4**. It keeps
dependency fetching in explicit preparation/release preflight, outside
ordinary validation ([#98](https://github.com/dragginzgame/ic-blob-storage/issues/98)).
Its release fixture now consumes a prepared cache and refuses any fetch during
validation, while exercising explicit preparation separately. Lint-first refusal,
input sealing, existing recovery checks and parallel Make behavior remain.
The original PR's three-host failure was a stale expected-fetch assertion in the
intentional Clippy-refusal case, not a product Clippy defect.

The local Shared Tooling 0.26.4 adoption joins those pending notes; package
versions remain 0.40.3. Preserve unrelated incoming Cargo.lock edits. Neither
the fixture repair nor snapshot refresh qualifies that incoming runtime graph.
No product API, schema, wire format or dependency selection changes are needed.

## Ownership and delivered tooling

Blob owns tenant policy, durable service workflows, provider accounting and
recovery, plus its standalone adapter and native/browser primitives. Consumer
frameworks own their wrappers, lifecycle composition and integration tests;
applications own login/grants, verifier operation and asset transactions.
Platform owns IC identities/runtime and required external Candid; Wire owns
application framing/schema. Host/Build/Testkit belong to
[IC Tooling](https://github.com/dragginzgame/ic-tooling); Memory/Storage to
[IC Persistence](https://github.com/dragginzgame/ic-persistence).

Shared Tooling **0.26.4**, revision `dc1ced7e240e90f0c6daa89fb6248fcb074164eb`,
has a verified 125-file snapshot, exported from an isolated clean exact revision.
Its validation logger preserves independent parent errors, bounds diagnostic
excerpts and publishes latest failure logs atomically. Checksum/evidence readers
reject NUL-bearing digests and multiple metadata documents, retaining failed
attempts and existing destinations. Tool pins and snapshot membership are unchanged.
Ordinary authorized integration follows asynchronous hosted CI; enforced checks,
known failures and separate release/service/issue acceptance remain binding.
Task-relevant sibling inspection stays read-only. DFINITY limitations need an
available capability, a Dragginzgame-owned solution or removal of the need, rather
than a plan depending on future DFINITY work. Only `wasm-opt` is selected for IC setup.
Production optimizer builds are separate from probes to exclude `test-support`.
Tooling 0.63 Testkit/server and release-editor setup/admission explicitly select
`cli`; application libraries do not. Retain actual consumer routing, admission,
evidence and release recovery checks; generic matrices remain with Shared.
Cross-release transitions remain retirement/reinstall only; same-release recovery,
exact retries, fences and retained obligations remain required.

## Outstanding issues and current evidence

Eight Blob issues remain: #98's fix is delivered through PR #99, with Intel
macOS qualification still pending in the run linked below;
#4/#5/#6/#31/#32/#33/#34 retain consumer/provider requirements. #84/#95/#96
are resolved on main; PR #94 was superseded and #97 is merged. Do not conflate
producer deliveries with consumer or deployed-provider acceptance. #4's current
owner links now distinguish Canic #444 composition from the application's asset
transaction and IC Crates #1; Canic #452 tracks Fleet Ensure, not publication.

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

[Canic #444](https://github.com/dragginzgame/canic/issues/444#issuecomment-6315345898)
reports local dedicated/embedded composition on published Blob 0.40.3, after
aligning all three graphs and rebuilding actual Wasm. This remains owner-reported
pending local integration, not default-main delivery or an execution here.
Application/provider acceptance remains separate. [Canic #494](https://github.com/dragginzgame/canic/issues/494)
owns guarded funding composition after #34 establishes acquisition.
[IC Crates #1](https://github.com/dragginzgame/ic-crates/issues/1) retains its older
graph and complete archive/provider/verifier/catalog transaction. No excluded
application was inspected and no sibling checkout was edited or executed here.

## Validation and evidence

Delivered 0.40.3 passed its complete [tooling/native and MSRV matrix](https://github.com/dragginzgame/ic-blob-storage/actions/runs/39592967028)
on Linux, Intel macOS and Apple Silicon. Earlier HTTP adoption evidence remains
in `.tmp/review-validation/tooling0632/` in the primary checkout, bound to its
captured source/graph; it does not qualify later incoming lockfile edits.

PR #99 repair evidence stays in the primary checkout at
`.tmp/review-validation/pr99/`. Raw GitHub API logs confirm the Linux/macOS
`bootstrap-lint` failure; both CLI log commands returned empty output, retained
as unavailable observations. The isolated local baseline reproduces that failure.
The repaired full release-adapter fixture passes, and restoring implicit fetch
fails `bootstrap-success`. No real Git/Cargo delivery effects occur in those
fixtures. Existing Testkit/editor forwarding, admitted-server and ordered failure
checks, snapshot integrity, Bash syntax, ShellCheck and documentation links pass.
On exact repair head `f1137bb8ef244db3c1d8ae0b99e59591a5b6ab0d`,
[run 39611955172](https://github.com/dragginzgame/ic-blob-storage/actions/runs/39611955172)
passes Linux, Apple Silicon and MSRV; Intel macOS remains pending. The PR was
merged under the maintainer's existing queue authorization and asynchronous
cadence, with no enforced GitHub check/review requirements bypassed. #98 retains
the owning follow-up; neither the original failed run nor pending coverage is green.

Shared adoption evidence stays in `.tmp/review-validation/shared0264/` with
before-images, exporter preview/output and focused consumer results. The producer's
[exact-revision CI](https://github.com/dragginzgame/shared-tooling/actions/runs/39625117898)
was still pending at source review; it is separate from consumer qualification.
Local Linux snapshot verification, explicit setup/offline tool admission,
formatting, documentation links, changed-helper Bash/ShellCheck, consumer logging
and input checkpoints, parent-log preservation, evidence archive/retention,
loopback download/checksum and hook checks pass. The incoming lock remains
byte-for-byte unchanged. No Rust implementation, package versions, tool pins or
deployment dependency selection changed; no full product suite or new native
macOS/installed-IC/provider qualification was run for this snapshot adoption.
The complete focused release-adapter fixture also passes against the refreshed
helpers, including prepared-cache, publication-refusal and interruption-recovery
cases; all Git/Cargo effects stay inside disposable fixtures.

Historical complete-gate evidence remains bound to its original graph:
`.tmp/review-validation/wire058-gate/ci.log` records all 20 targets passing in
930 seconds with 1,995 tracked file contents unchanged, including PocketIC,
original/O3/Os/Oz installation/restoration and archives. It does not qualify today's
incoming lock. `.tmp/review-validation/backlog0400/` separately retains the
registry-only Blob 0.40.0 / Platform 0.6.1 installation/restore compile probe.
Installed Canic and live-provider acceptance remain separate from native CI.
[Shared #294](https://github.com/dragginzgame/shared-tooling/issues/294) and
[Tooling #223](https://github.com/dragginzgame/ic-tooling/issues/223) retain producer
qualification independently. Earlier graph-specific evidence stays in the
[2026-10-20 handoff](handoff-2026-10-20.md),
[2026-10-18 handoff](handoff-2026-10-18.md) and [history](history.md).
