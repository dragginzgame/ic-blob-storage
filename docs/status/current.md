# Current status — 2026-10-28

## Delivered baseline

Blob **0.52.1** is on default main/tag at
`3513b5b0faeae57f554e2430e9c3a92ad3b109c7`; its release receipt names validated
source `5aec5468aa7b2a7791c9871a837a38ba41065b5d`. The checkout was clean at
inspection. The agent performed no commit, push, release, publication or deployment.
The published client archive's VCS metadata names the exact release commit.
Pending compatible notes are **0.52.2**; package versions and lockfile are unchanged.

## Issue reconciliation

- **#107 closed:** the published 0.52.1 client archive includes a regular LICENSE,
  byte-identical to root MIT text, with matching MIT metadata. Its official index
  checksum is `fa8ab6b1c76e0ae0622fdcaf7442735c1390ec0e68195a999798036c344057bd`.
  Readback: `/tmp/blob521-issues.7XiLpB8r/`. The old 0.52.0 archive remains
  immutable and missing that file. Packaging checks regular-file status and equality;
  the original symlink attempt violated shared policy and was corrected before release.
- **#105 delivered, open for complete hosted qualification:** interrupted history
  recovery now accepts uninterrupted platform observations from the same live owner.
  A private Arc identity is invalidated by a missed/reversed version, explicit fence
  or reopened heap. Complete independent IC history, post-await authority/anchor
  checks and exact-callback sealed-proof consumption remain mandatory. Stored
  schemas, journals and obligations are unchanged; the public low-level proof API
  retains its strict single-step guard. Unobserved host executions refuse safely.
- **#106 delivered:** the hosted core/client job actually passes **339 + 4 tests**,
  with separate nonempty selections. Deliberately failing either library failed the
  extracted real job body locally. Existing contracts/native-host matrices remain.
  It stays open for the containing required matrix; no synchronous publication wait
  was introduced.
- **#108 open:** converge generic replicated-call mechanics on Platform's selected
  canonical ICC API. Preserve method codecs, authority, explicit wait policies,
  single-attempt dispatch, attachment/refund accounting and uncertain-effect journals.
  [Platform #75](https://github.com/dragginzgame/ic-platform/issues/75) owns the
  shared contract. Published 0.7.1 already supplies all four raw execution paths;
  the remaining adapters carry domain policy. No competing Blob executor was found.
  New regressions preserve SysUnknown/future rejection codes versus non-submission.
  All six client tests, strict client Clippy, Rust 1.91 builds of four probe Wasms
  and 25 installed tenant/query/funding cases pass. Evidence (including initial
  sandbox bind failures): `/tmp/blob108-qualification.9lvipGpf/`.
  Direct and runtime/deployment boundaries pass with unchanged Cargo inputs.
  Final owner API selection/publication and installed bounded-timeout/unknown-response
  qualification remain; native classification is not an installed timeout proof.
  No new macOS transport execution is claimed. The client still gets Candid through
  Contracts' provider codecs/certificate/account types, independently of raw execution.

Eight issues remain; no open PRs were found. #4/#5/#6/#31/#34 still require actual
application/provider acceptance: media-map transactions, browser serving, reference
lifetimes/deletion/billing, two authenticated users and payer linkage/reconciliation.
Local substitutes and framework publication do not close those outcomes. Reuse the
[coordinated acceptance run](../acceptance-plan.md#next-application-acceptance-run).
That checklist now includes explicit #4 media-map recovery and #6 reference/deletion/
billing steps. These are prepared procedures, not new live acceptance evidence.

## Qualification and limits

At inspection, [0.52.1 tooling CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/40672576194)
has passing Linux, core/client, MSRV and generated-client jobs; both macOS
qualification jobs are still running. Published example installation run 40672576197
passes. Do not call the incomplete matrix green or poll solely to wait for completion.
Revisit #105/#106 on the next requested inspection if the complete matrix passes.

Local source evidence: `/tmp/blob-issues105-107.zRpGdxat/` retains failed attempts,
339 core/4 client tests, strict affected-package Clippy, five installed lifecycle
cases, real snapshot-rollback refusal, exact exports and Rust 1.91 native/Wasm checks.
The installed interleaving reproduction queues sixteen denied updates during history
lookup and preserves 1024 physical/1024 liability bytes. Four native observation
regressions cover gaps, fences and reopened owners without changing stable bytes.
Hook/archive correction evidence: `/tmp/blob-license-hook-fix.KoY5miUj/`.

## Contracts and Canic

The [0.52 record](../evidence/simplification0520.md) owns source-bound simplifications,
measurements and proofs. Funding uses a multirow receipt-index map; installation uses
fifteen canonical grants and its singleton-cell owner. Owned schemas/CLI status stay 1.
Descriptor/client downloads remain supported. Cross-release transitions require
[retirement/reinstall](../retiring-installations.md), retaining uncertain operations,
provider objects, balances and billing. Same-release recovery and exact receipts remain.

Last reviewed Canic **0.112.0** main/tag is `5c236ab130c0d63892ef281b45bdc3fa353e012f`,
selecting Blob/Contracts 0.51.0. Its release/publication and retained dedicated/embedded
and funding-profile tests pass. Two internal constructors still use ServiceReadInput;
Canic needs 0.52 adoption and its own composition qualification. No copied grant roster
or production chunk-read transport was found. Its CI 40659441082 was incomplete at
that earlier inspection; no new Canic CI result is claimed here. #444/#622/#494 retain
their framework/hosted/independent-credit scopes. Acknowledged funding does not require
independent credit after every success; registration does not prove stored bytes.

## Dependencies and retained evidence

Minima remain Tooling 0.69.2, Memory/Storage 0.51.12 and Platform 0.7.0. The released
lock selects Tooling 0.69.4, Memory/Storage 0.51.13, Platform 0.7.1, Wire 0.7.18 and
Metrics 0.16.3. Browser Platform 0.1.1/Core 6.1.1/Caffeine 1.1.2 are unchanged.
[Platform #50](https://github.com/dragginzgame/ic-platform/issues/50) and
[Persistence #136](https://github.com/dragginzgame/ic-persistence/issues/136) retain
their supplying acceptance scopes. No sibling files were edited.

Shared **0.27.3**, revision `cc81a58a5b2dd287c577cbffd2c576c178b6785c`: 131 payloads.
Canonical source comparison, offline integrity, formatting/pins and hook/release/Testkit
callers pass; evidence `/tmp/blob-shared0273.Q4qkZ1zY/`. Older implementation captures
remain in `/tmp/blob-simplify052-20261027.wmcqnujw/`,
`/tmp/blob-cell052-20261027.qOTAMW0S/`, `/tmp/blob-tooling0692-20261027.XFd9u5hk/`
and `/tmp/blob-retired-read-tests-20261027.t7LYiMA5/`. No evidence was overwritten.
