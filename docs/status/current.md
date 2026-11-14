# Current status — 2026-11-14

This is the independent clean-start Blob Storage rewrite. One unpublished
`ic-blob-storage` crate implements an experimental local grant/reference/accounting
model, streaming Caffeine root verification and manifest preflight. Direct product
dependencies are `sha2` 0.11.1 and `ic-platform-types` 0.20.3, both with default
features disabled. There are no endpoints, persistent service or network integration.
Rust 1.100.0 / edition 2024 and authenticated Toko Tooling 0.43.0 remain selected.

## GitHub checkpoint and next development

The first checkpoint, `0.100.0`, was pushed to branch `0.100` at
[ad0b926](https://github.com/dragginzgame/ic-blob-storage/commit/ad0b92603fdae0ca5ce268fd9ab912d0b923efdb).
GitHub readback confirmed that exact branch head. The fresh repository is private,
ID 1457703306, with default branch `0.100` and collaborator-only pull requests.
Local `origin` is `https://github.com/dragginzgame/ic-blob-storage.git`.
No legacy commits/refs were imported, and no tag, registry publication or
provider effect occurred. This is a GitHub development checkpoint only.

The maintainer requested starting the next version: `0.100.1` is prepared in the
working tree with the local model, verifier, manifest check, offline application
example, focused test targets, lockfile and documentation. These changes remain
uncommitted and unpushed on local `0.100`.
The root workspace owns the version; the product inherits it. Publication remains
explicitly disabled. The crates.io index checked before the checkpoint listed
`0.82.1` as the highest published stable version and no `0.100.0`; recheck the
requested version during an eventual authorized publication.

## Accepted direction and next work

The maintainer accepted Caffeine-first scope, one reusable service library,
standalone hosting, direct SDK transfers and a Canic-owned adapter. The
[design](../design/ic-blob-storage-design.md) selects the ownership model and
proposes identity, authority, closure, accounting and lifecycle rules. API and
persistence remain unfrozen; no legacy contract or storage format is adopted.

The [crate layout](../design/ic-blob-storage-design.md#selected-crate-layout)
adds a thin standalone host when endpoints exist. Installed tests will use
`qualification/crates/` within the root workspace/lockfile. No host exists yet,
so no placeholder package was created. Native reference tests stay with their
owner; Canic owns its composition tests and browser tests belong to the client.

The [local verifier/reuse record](../design/caffeine-contract-review.md#implemented-local-read-verification)
documents adapted first-party legacy streaming/metadata code and frozen SDK vectors.
It selects only the reviewed 1.1.2 integrity algorithm, not provider identity or
admission semantics. It checks exact original length, metadata and root using
fixed retained hash state. The caller still owns successful transport EOF and
private output until verification passes. npm latest remains 1.1.2; SDK recovery
and provider-contract gaps remain open. No legacy recovery records were imported.

The [manifest reuse record](../design/caffeine-contract-review.md#implemented-local-manifest-preflight)
adds bounded consistency checking adapted from the first-party legacy constructor
and failure cases. It borrows leaves without retaining them and shares tree and
metadata code with the verifier. A consistent manifest can still lie about byte
length; the regression explicitly preserves this distinction. No authority,
provider storage bound, wire decoder or persistent representation is established.
The local model now calls this preflight on fresh authorization; host/client
integration remains future work.

The
[field enforcement matrix](../design/caffeine-contract-review.md#field-enforcement-required-before-registration)
assigns declaration checks to Blob and the required enforcement to Caffeine.
The linked qualification sequence separates supported-contract selection (D0),
installed local behavior (D1) and authorized live observations (D2), including
malicious size assertions, proof recovery, scope substitution and late uploads.
Read-only inspection of the pinned legacy October investigations and November 5
deletion follow-up found no closure proof: the latter still reported DeletionPending
and no recorded acknowledgment. No old object or provider endpoint was accessed.

The maintainer explicitly approved proceeding with a local model while D0 is open:
no deployment/certificate issuance, declared-byte accounting, no settlement claim,
and unfrozen API/persistence. The [implementation and legacy reuse record](../design/ic-blob-storage-design.md#experimental-local-service-model)
describes the resulting single local owner. It composes the existing reference
component instead of introducing a parallel reference engine. One immutable
application authority authorizes/retains/releases; only the bound uploader registers.
The host must supply actual captured Principals. Root matching is provisional
within one hypothetical binding, not a qualified deletion identity.

Cancellation/expiry retires IDs and frees unexposed reservations. A fresh grant can
reuse an unexposed root, but old releases stay bound to the old grant. Registered
roots remain owned permanently, including after final release. An additional upload
ID attaches a reference to an Open owner without a second registration. Final release
frees live declared bytes only; no operation settles unresolved usage. The heap model
must never back real certificates or obligations. Its callers are the nine composed
tests and the offline application example; real application/host integration remains
future work.

The [offline application walkthrough](../design/ic-blob-storage-design.md#offline-application-walkthrough)
now runs with `make example-local`. Two synthetic uploaders share a 5 MiB + 17 byte
fixture between two assets. The application owns its catalog and read completion;
Blob owns grant/reference/accounting state. It demonstrates pre-registration
cancellation, publication only after verified EOF, corruption refusal, cleanup of
a fresh reference after a catalog collision, and final release retaining unresolved
declared usage. Its two private catalog helpers also distinguish exact publication
replay from a new conflicting draft, recheck liveness after read verification,
and compare expected references before removal. Delayed removal/republication
cannot affect an asset-name replacement. It does not implement a browser/SDK
transfer or remote transaction. Four tests selected by `make test-example-local`
check read completion, cancellation after verification and nonmutating refusals.

Next, use this concrete journey to select the actual application/host boundary.
D0 must still close before selecting real provider identity, durable service
integration or exposing upload authority. The approved local scope remains useful
for domain work but does not authorize a service deployment.
The [Caffeine review](../design/caffeine-contract-review.md)
traces historical standalone uploads, pinned issuer/manifest/client source and
provider SDK behavior. Issuance is a synchronous host update whose IC-certified
reply the client uses; no host-to-provider certificate acquisition is implied.
Independent enrollment/support, exclusive deletion scope, byte enforcement,
recovered-proof acceptance, SDK resume and authority-lifetime closure remain
unqualified. The maintainer has no separate provider agreement/contact. Final
release cannot retract a held or independently recovered original certificate.

Select a concrete application asset and the restricted management entrypoint
before installed lifecycle tests. The proposed policy drains/stops before
same-schema upgrades and refuses unsafe state replacement through the supported
management path; it makes no guarantee against an unrestricted controller bypass.
Do not create speculative provider abstractions or import legacy recovery code.

## Post-push architecture check

Trigger: the initial push above. Method: Toko Tooling
[complexity review](https://github.com/dragginzgame/toko-tooling/blob/626da7cd9626cc2e152594862a2d82ea82ef51da/audits/complexity-and-technical-debt.md),
with the pushed [AGENTS.md](https://github.com/dragginzgame/ic-blob-storage/blob/ad0b92603fdae0ca5ce268fd9ab912d0b923efdb/AGENTS.md) overlay. Source inspection covered
all product code at the pushed commit, its tests, Makefile and planned ownership.
There is no earlier rewrite commit for a comparable growth baseline.

No actionable architectural sediment was found in the implemented slice.
`ObjectReferences` alone owns live/released IDs, the lifetime slot bound and the
live count; Open/Closing is derived. Retain/release policy has no other semantic
consumer or duplicate store. At that commit callers are the seven owner-local tests;
there is no host caller yet. Retained released IDs prevent replay resurrection,
and the cached live count avoids a history scan on each retention inspection.
Both are justified state, not obsolete compatibility machinery.

Two near-term rehearsals: registration must compose this owner with its first
reference/accounting, blocked on provider identity/bounds; persistence must place
this same owner behind admitted storage rather than create a parallel engine.
Host caller capture and memory/lifecycle composition remain outside the library's
business rules. No provider mode, generic retry framework or test-only engine
has accumulated. These rehearsals qualify no unimplemented behavior.

`make deps-list` and `toko-cloc` after the push report one crate, 81 product Rust
lines, 113 test lines, zero consumer Rust tooling and zero unclassified code.
The initial 65-file commit also includes 49 authenticated shared tooling/policy
files; that installed payload is not Blob product code. Snapshot admission passes.
No full-repository audit or code-suite rerun was needed for this source review.

## Validation and retained history

For the local model, all nine tests passed using the exact command exposed by
`make test-local` (Make wiring checked with a dry run). Native Clippy including
tests, Wasm library Clippy, strict Rustdoc, rustfmt and documentation links pass.
The twelve streaming/manifest and seven reference tests retain their prior evidence;
the new tests also exercise their composition. No full CI, fresh SDK execution,
installed host/provider test, rollback or D0/D1 completion is claimed.

After the model batch, LOC was 694 product / 998 test Rust lines, versus 336 / 537 before the
local model and 81 / 113 at the GitHub checkpoint. The addition is one local
control owner and its failure cases, with no new crate, transfer engine or consumer
tooling. Record counts derive from the maps; retained byte counters avoid scanning
history. Reference liveness stays in the existing reference owner, and the global
reference map adds immutable targets and original-grant bindings. Both are needed
to prevent cross-object reuse and stale cancellation; no duplicate reference state
is stored. Root/object entries are bounded by lifetime upload slots.

The application example and its retry extension leave product code unchanged:
current totals are **694 product / 1,152 test / 344 example**. This continuation
adds 137 example and 114 test lines over the initial walkthrough. Two conditional
catalog operations replace unconditional map writes; they add no receipt ledger,
reference-state copy or generic retry mechanism.
`make example-local` and all four `make test-example-local` cases passed.
Example-target Clippy including test compilation, rustfmt, local links and whitespace
checks pass; the actual Make entrypoints selected the scoped run and tests.
No product suite, Wasm check or Rustdoc rerun was needed for the unchanged library.
No dependency, crate, installed qualification package or live-provider effect was
added. Legacy publication and reference receipt/submission tests were inspected
read-only; exact-intent/current-liveness lessons were retained without the patched
SDK, receipt stores or recovery machinery.

Platform Types 0.20.3 was verified as the latest stable non-yanked registry release
on 2026-11-14, then locked/fetched offline. Its identity-only selection introduces
13 lockfile packages including itself; no IC runtime, Candid or external SDK was
added. This is a local implementation review, not a new post-push audit. No push
occurred; the earlier post-push review applies only to its recorded commit.

Legacy custody remains in private `dragginzgame/ic-blob-storage-legacy`, repository
ID 1387534636. That repository was untouched by delivery. Its expected sibling
path is absent here; relocation/frozen-journal facts were not reverified. Read its
`LEGACY.md` before any separately authorized recovery. Never import or resume its
provider obligations through this rewrite. The initial checkpoint's
[handoff](https://github.com/dragginzgame/ic-blob-storage/blob/ad0b92603fdae0ca5ce268fd9ab912d0b923efdb/docs/status/current.md)
preserves the bootstrap/relocation and earlier validation details; local bootstrap
logs remain in `target/bootstrap-20261114/`. Machine-local credentials are ignored.
