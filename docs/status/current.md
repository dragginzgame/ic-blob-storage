# Current status

Date: 2026-09-25

## Implemented and released

The maintainer completed Git release 0.1.2: source `94cbf89`, release
`93a5b6b`, dated changelog and generated receipt. Cargo is 0.1.2; a registry
upload has not been verified. Release/publication retain the build cache;
only explicit `make clean` removes it. Library publication is enabled and
separate from service qualification. Follow [release guidance](../releasing.md).

The core implements distinct content/provider identities, strict hash parsing,
incremental raw-byte length/digest verification and pure numeric funding/readiness
policy. [Core evidence](../evidence/core-primitives.md) records its boundaries
and checks. Provider bindings, persisted workflows, clients, canister endpoints
and adapters are not implemented. No end-to-end service journey is qualified.

Dependencies and the local PocketIC server are pinned; see
[setup](../dependencies.md). No Canic production dependency has been added.

## 0.1.3 preparation

The maintainer requested 0.1.3. The release-test and documentation cleanup is
grouped in the undated [0.1.3 draft](../../CHANGELOG.md), with empty Unreleased.
This is a compatible tooling/documentation patch; storage behavior is unchanged.
Cargo and the existing release receipt remain at 0.1.2 until the source batch
is committed and clean, as required by release preparation. Commits remain
maintainer-owned. After committing this batch, the maintainer can run
`make release-patch`; exact-version preparation for 0.1.3 is also authorized,
but publication remains a separate action.

Release-helper tests use named isolated fixtures, consolidated preparation
checks and exact observable-effect sequences. Unexpected command-failure
statuses reject. Normal output is a fixture notice and one pass summary;
failures identify the case and retain diagnostic logs under `target/`.
Make help now correctly describes retained build artifacts.

`make release-verify` passed for the 0.1.3 source batch: shell/helper checks,
formatting, native compilation, strict Clippy, docs, native tests and doctests,
Wasm compilation and offline package verification. Changelog preflight and
the effect-free exact-version plan accept 0.1.3. Earlier temporary mutation
checks detected a failed assertion, an extra release effect and an unsupported
substitute command. No real commits, tags, uploads or Cargo cleanup ran.

The documentation audit consolidated extraction planning into the parity and
service contracts, provider questions into the provider review, and removed
obsolete release instructions from this handoff. Source inventories and raw
evidence remain unchanged. Local Markdown links/anchors, JSON parsing and
`git diff --check` passed. The release guide's obsolete publication-fix
instructions have also been removed.

## Authority and next work

The maintainer confirmed Canic's 0.110 human acceptance for work here without
changing Canic's handoff. Before B1 closes, the explicit implementation exception
covers content identities, incremental verification and pure funding/readiness
policy with native tests. Provider bindings/effects, persisted workflows and
Canic removal remain gated by the [service contract](../service-contract.md).
The separate removal of B1 as a library-publication gate does not expand that
implementation exception. Agents must not create commits.

All Canic blob functionality, including operator commands and diagnostics,
must work here before removal there. The [parity contract](../canic-parity.md)
owns the source/removal inventory and installation obligations. BLOB-01 has
native evidence; BLOB-08/11/12 have partial pure-policy evidence. Canic and
other sibling repositories remain read-only.

The maintainer selected the latest official Caffeine integration. The
[baseline](../provider-baseline.json) records client 1.1.2 and backend 1.1.1,
verified on 2026-09-25; these versions do not identify a deployed gateway or
Cashier. Recheck upstream before provider implementation and update exact pins
and affected evidence together.

Next obtain the intended deployed gateway/Cashier identity and authoritative
interface, retry, retention, deletion and billing evidence listed in the
[provider review](../provider-review.md#evidence-needed-to-freeze-b1). Then freeze
the consumer, accountable owners, bounds and restore contract. Caffeine remains
unqualified; public assets are a proposed journey, not an assigned consumer.
The [acceptance plan](../acceptance-plan.md) specifies the remaining cases.
