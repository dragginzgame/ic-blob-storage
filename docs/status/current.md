# Current status — 2026-11-14

## Delivered checkpoint

Development version **0.100.1** was committed and pushed to `origin/0.100` at
[8cb3c40](https://github.com/dragginzgame/ic-blob-storage/commit/8cb3c40bdd09e36f4f92c410f8773a361f398706).
GitHub branch readback confirmed that exact SHA. That checkpoint includes the
local model and example. The post-push handoff, application boundary and permanent
root-retirement, root-registration and inspection changes remain local;
the index is empty. No tag, crates.io publication or live-provider effect occurred.
`publish = false` remains inherited.
The private repository is `dragginzgame/ic-blob-storage` (ID 1457703306), default
branch `0.100`; origin is `https://github.com/dragginzgame/ic-blob-storage.git`.

One unpublished product crate uses Rust 1.100.0 / edition 2024, Toko Tooling
0.43.0, `sha2` 0.11.1 and `ic-platform-types` 0.20.3 (default features disabled).
There are no endpoints, durable service, deployment runner or network integration.
Detailed prior validation, research and the first checkpoint audit are preserved
in the [pushed handoff](https://github.com/dragginzgame/ic-blob-storage/blob/8cb3c40bdd09e36f4f92c410f8773a361f398706/docs/status/current.md).

## Accepted scope and next work

The [design](../design/ic-blob-storage-design.md) selects Caffeine-first storage,
one service library, a thin standalone host, direct client SDK transfers and a
Canic-owned adapter. API and persistence remain unfrozen. The accepted local
qualification location is `qualification/crates/` in the root workspace, when
real installed behavior needs it; no placeholder package exists.

The maintainer approved an experimental local model while provider gate D0 is
open: no certificates/deployment, declared-byte accounting, no settlement claim.
`LocalModel` composes grants, provisional root matching, references and accounting.
Only the application authority authorizes/retains/releases; the bound uploader
registers. Hosts must supply captured identities. The maintainer selected one
upload lifetime per admitted root: cancellation/expiry frees unexposed byte
reservations but permanently retains the root. New IDs/uploaders cannot restart
it. Open objects still accept additional references. The root points to its original
upload, which owns declaration/disposition; references store only their root target.
Application-only inspection projects that target and current status as `ReferenceView`.
The example resolves catalog assets from their reference IDs without another target map.
Known-root `ObjectView` inspection adds original-grant history, optional local retention
and live-use count; it projects the existing owners without another state store.
Registered roots remain owned after final release, with unresolved usage retained.
This heap model MUST NOT back real upload authority or provider obligations.

The [Caffeine contract review](../design/caffeine-contract-review.md) records
adapted first-party legacy integrity code, SDK 1.1.2 vectors, manifest checks and
provider enforcement gaps. Manifest consistency cannot authenticate declared length.
Streaming verification needs the original trusted declaration AND successful EOF.
The [offline walkthrough](../design/ic-blob-storage-design.md#offline-application-walkthrough)
exercises these boundaries, overlapping uses, publication retries, cancellation,
catalog collisions and stale removal. Publication means application catalog entry.

The new [application boundary](../design/application-boundary.md) selects that asset
catalog as the reference consumer, distinguishes A's application calls from direct
uploader calls, and defines remote cancellation/publication/release ordering.
It adds no installed application. Unknown references cannot finish cleanup while
authorization is unresolved; delayed Live observations cannot override cancellation.
Permanent root ownership prevents a later grant from replacing a cancelled one.
Local registration now takes a decoded root, resolves that original lifetime and
rechecks its uploader, disposition and deadline in the synchronous transition.
It replaces upload-ID registration; no eligibility token or alternate path remains.
Next, qualify the actual provider binding with D0 before a real host: text decoding,
actual caller capture and usable certificate replies remain unimplemented.
Close D0 before real provider identity, durable service integration or authority exposure.
Independent enrollment/support, exclusive deletion scope, byte enforcement,
recovered-proof acceptance, SDK resume and authority-lifetime closure remain
unqualified. The maintainer has no separate provider agreement/contact. Historical
uploads and the November deletion follow-up do not prove closure. A held or recovered
original certificate cannot be retracted merely by final local release.
Select the restricted management path before installed lifecycle tests: proposed
stop/drain upgrades and unsafe-restore refusal do not constrain a controller bypass.
Future durable integration must replace this owner's backing or retire it, never
add a parallel policy engine. Do not create speculative provider abstractions.

## Post-push architecture check

Trigger: push of `8cb3c40`; comparison: first checkpoint `ad0b926` (0.100.0).
Method: Toko Tooling [complexity review](https://github.com/dragginzgame/toko-tooling/blob/626da7cd9626cc2e152594862a2d82ea82ef51da/audits/complexity-and-technical-debt.md)
at revision `626da7cd9626cc2e152594862a2d82ea82ef51da`, with the
[pushed overlay](https://github.com/dragginzgame/ic-blob-storage/blob/8cb3c40bdd09e36f4f92c410f8773a361f398706/AGENTS.md).
Source inspection covered changed product owners, reference composition, tests,
example callers, dependency/Make wiring and design boundaries. The working tree
matched the pushed source during inspection. Verdict: **PASS within this local
structural scope**, with no actionable sediment found; installed behavior is excluded
because no host/provider integration exists, not because it has been qualified.

| State/decision | Canonical owner and consumers |
| --- | --- |
| Reserved/Registered/Attached/Cancelled/Expired grants, caller rules, R/L/E limits | `LocalModel`; consumed by tests and the offline example. Only Reserved registers; only unexposed reservations retire |
| Live/Released references and derived Open/Closing | `ObjectReferences`; composed by the model. The global map holds immutable targets/original grants, not duplicate liveness |
| Root/metadata consistency and stream integrity | Shared Caffeine tree/metadata helpers; manifest admission and read verifier use the same algorithm |
| Catalog insertion/replay/removal | Private example helpers; current Blob liveness and expected reference guard publication/removal without a receipt ledger |

Retained IDs prevent resurrection and stale cancellation. Three cached byte counters
avoid scanning history; record counts derive from maps. No settlement mode, generic
retry framework, SDK copy, legacy recovery engine or second production state engine
was added. Targeted legacy reuse and its requalification are recorded in the design.
Two source rehearsals: actual ingress capture belongs in a thin host calling this
owner; durable storage must retain this owner and its accounting/reference semantics.
Both depend on the selected provider/lifecycle contract; neither warrants another
local abstraction now. Catalog policy continues to belong to the application.

Fresh `make deps-list`/`toko-cloc` report one crate: **694 product / 1,152 test /
344 example** Rust lines, zero consumer tooling or unclassified Rust. Baseline was
81 product / 113 test / zero example. Growth implements bounded integrity, one local
control owner and failure/replay witnesses; no new package or managed tooling files.
This is a scoped source review, not a full repository audit or reliability verdict.

## Validation and custody

Unchanged focused evidence was reused: nine model tests, twelve integrity tests,
seven reference tests, four example tests and the offline walkthrough passed.
Native/test and Wasm library Clippy, example Clippy, strict Rustdoc, rustfmt and
local links passed in their affected batches. Delivery newly admitted all 49 tooling
files and installed tools; staged whitespace and pre-commit formatting passed.
No full CI, fresh SDK execution, installed-provider test, rollback or D0/D1 completion
is claimed. No product code changed during delivery.

Latest local batch: all eleven `make test-local` cases, four `make test-example-local`
cases and `make example-local` pass. The cases retain root retirement, delayed
authorization, exact original grant resolution and accounting proofs. Object-view
assertions now cover reservation, cancellation/expiry, overlapping uses, final release,
caller refusal, unknown roots and full-capacity inspection. Reference targets remain
checked through cleanup and rejected rebinding. The example observes Open after scene
removal, then Closing after final release while Registered history and exposure remain.
Native library/test/example Clippy and strict Rustdoc pass; formatting, local links
and whitespace pass. Unchanged integrity/reference tests retain their prior evidence;
no full CI or new installed proof is claimed.

LOC is now **718 product / 1,374 test / 396 example**. Object inspection adds 21
product lines over the preceding local batch, with no stored state, dependency or
package. It reuses `RetentionState` and the reference owner's live count. Permanent
root claims still replace per-reference grant pointers/duplicate byte declarations;
there is no second registry. Legacy root, certificate, reference and upload-status
sources/tests were read only; reuse/exclusions are in the design. No stable index,
text parser, certificate response, assessment, settlement or restore mechanism was
copied. These changes remain unpushed and outside the post-push audit above.

Legacy custody stays in private `dragginzgame/ic-blob-storage-legacy`, ID 1387534636.
That repository was untouched. The expected sibling path is absent here; relocation
and frozen journals were not reverified. Read its `LEGACY.md` before any separately
authorized recovery. Never import/resume legacy provider obligations in this rewrite.
Bootstrap logs remain under `target/bootstrap-20261114/`; credentials are ignored.
