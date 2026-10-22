# Current status

Date: 2026-10-22

## Source and accepted scope

Latest released source is **0.40.4**, `ba779305b66acc87bf7af1e4f5a4a4f3ee5f24c7`.
Local main includes the pending batch at `c1103db`; no 0.41.0 tag is present.
The working batch targets **0.41.0**; package versions remain 0.40.4 for the
maintainer's release runner. Preserve the incoming Cargo.lock changes and the
October 22 source-discovery evidence. No staging, commit, push or release is
part of this implementation.

The maintainer approved the [trusted-registration hard cut](../trusted-registration.md).
Certificate issuance atomically registers retention intent and the first live
reference. Remove mandatory verifier configuration, attestation state/endpoints,
CLI observer/submission commands and the session `verify` phase. Keep local body
verification and verified downloads. Registration does not prove uploaded bytes,
availability or future provider retention. Reference maps explicitly report this.

The installation discriminator and owned native Wire identities change. Old
installations require retirement/reinstall; old saved runs keep their original
tools. No dual readers, automatic migration or repeat dispatch is introduced.
Same-release restoration, exact retry records, lifetime quotas and physical/billing
obligations remain required. Retirement cannot discard the only uncertain records.

Shared Tooling **0.26.4**, revision `dc1ced7e240e90f0c6daa89fb6248fcb074164eb`,
remains the 125-file reviewed snapshot. The latest native adoption selects
published Host/Build/Testkit 0.64.0 together; Platform 0.6.7, Wire 0.5.20,
Metrics 0.15.5 and Memory/Storage 0.50.10 remain unchanged. Earlier hard-cut
installed artifacts use Tooling 0.63.3, Wire 0.5.19 and Metrics 0.15.4.
Ordinary CI is offline after explicit
preparation; released [#98](https://github.com/dragginzgame/ic-blob-storage/issues/98)
is resolved, including native macOS/MSRV qualification of its exact source.

## Ownership and consumer boundary

Blob owns tenant policy, durable upload/reference/accounting workflows, restoration,
the standalone adapter and native/browser primitives. Frameworks own mounting,
lifecycle composition and their integration tests. Applications own authentication,
grants, asset transactions and publication/availability decisions. Platform owns
IC identities/runtime and external Candid; Wire owns application framing.
Host/Build/Testkit belong to IC Tooling; Memory/Storage to IC Persistence.

Historical Canic parent `d9d70322ade2ba6fbd17520acf93acad66dcf4aa` registered a live
root inside certificate issuance, before upload. That verified source behavior
motivates the approved cut; it proves no deployed provider deletion/billing facts.
Canic's [0.40.4 local mounted/recovery report](https://github.com/dragginzgame/canic/issues/620#issuecomment-6323164353)
and [metrics report](https://github.com/dragginzgame/canic/issues/621#issuecomment-6322744750)
retain their original verifier-bearing graph. They do not qualify the new contract.
Verified Canic main `4caeec25db7d81f6121db1c95ba7a2d91d319c60` declares Blob 0.35;
its newer dirty checkout is not delivered source. Consumer owners must update
installation DTOs, state handling, generated bindings and removed calls together.
Sibling checkouts remain read-only. Do not duplicate their adapters/tests here.

## Outstanding acceptance

- [#33](https://github.com/dragginzgame/ic-blob-storage/issues/33#issuecomment-6336781863) tracks disposition
  of the mandatory verifier architecture. It is retired by this approved batch,
  pending delivery; do not operate another worker to satisfy the removed model.
- [#32](https://github.com/dragginzgame/ic-blob-storage/issues/32) retains gateway
  caller/failure behavior and independent physical-deletion/billing semantics.
  [October 22 discovery](../evidence/caffeine-probes/local/2026-10-22-gc-contract-discovery-01/summary.md)
  proves Motoko weak-reference pruning, not remote deletion or billing cessation.
  The completed source/public-package capture remains immutable; no provider
  canister/gateway was queried and no payment or deployment occurred.
- [#34](https://github.com/dragginzgame/ic-blob-storage/issues/34) retains unique
  credit acquisition and project-payer linkage; Canic #494 owns later composition.
- [#31](https://github.com/dragginzgame/ic-blob-storage/issues/31),
  [#4](https://github.com/dragginzgame/ic-blob-storage/issues/4),
  [#5](https://github.com/dragginzgame/ic-blob-storage/issues/5) and
  [#6](https://github.com/dragginzgame/ic-blob-storage/issues/6) retain actual users,
  media-map transactions, serving/CSP/cache, overlapping references and provider
  cleanup acceptance. Local substitutes or registration alone cannot close them.
  Canic #444/#620/#621 retain adapter delivery. Canic #493 remains future dynamic
  parent provisioning, separate from the current shared-service topology.

## Validation and continuation

Before-images, reviewed Wire inputs, artifacts and focused logs are retained under
`.tmp/review-validation/hardcut041/`. Passing checks cover 338 core, 120 contract,
4 client and 113 CLI unit cases; the frozen test checks 139 literal vectors.
Scoped installed evidence covers 30 admission, 49 journey, 74 standalone and
121 storage cases (including the two corrected targeted reruns), plus all four
opt-in Chromium registration journeys. Worker checks and 22 actual launcher/profile
cases pass. These use controlled local provider bytes, not deployed Caffeine.

Strict affected-package Clippy with all targets/features, scoped formatting,
release Wasm builds, Rust 1.91 public/core/client/standalone Wasm compilation,
Platform/runtime/deployment boundaries, documentation links and whitespace pass.
The final changed-test lint check is retained separately. Earlier sandbox, missing
CLI-artifact and obsolete-expectation failures remain in their original logs;
they are not successful qualification. No complete release gate was run.

Incoming Cargo.lock updates continued after those runs. Wire 0.5.20 passes all
139 frozen vectors, and 338 core unit cases pass on the subsequent incoming graph;
Metrics 0.15.5 changes documentation only. The earlier missing ICU cache is no
longer blocking the native rerun below. Preserve the incoming graph; it is not
covered by the installed artifact results above. `qualification.json` separates
those artifacts from the later lock snapshot. Qualify the settled graph before
release; do not reuse earlier installed evidence for a changing lockfile.

The current-compiler managed Testkit selection is not provisioned. Local installed
checks explicitly select the retained PocketIC 16.1.0 binary, matching Testkit's
published server requirement; this does not qualify the shared setup selection.
No native macOS, real installed IC or deployed Caffeine qualification is claimed.
Released-source hosted results remain linked in the [October 20 handoff](handoff-2026-10-20.md)
and [history](history.md); they do not qualify this dirty hard cut.

The committed regression repair at `c1103db` corrected idempotent observations,
registration accounting and reservation listings; its native evidence remains at
`.tmp/review-validation/registration-observation-fix/` on Tooling 0.63.3.

The new Host 0.64 adoption uses published registry sources only. Native tests import
PocketIC directly from Platform and call Testkit's `fake_principal`; startup,
lifecycle and transport extensions stay with Testkit. A remaining inventory test
incorrectly classified Live content as pending/retired; remove that negative case,
retaining the separate live-content reference-capacity test. Production storage,
wire contracts and package versions are unchanged.

Evidence under `.tmp/review-validation/host064/` covers locked metadata, one 0.64.0
identity for each of the seven Tooling packages, deployment exclusions, strict
affected-package Clippy, Rust 1.91 all-target/all-feature native checks, 121 CLI
cases, 25 native PocketIC unit cases and four installed certificate cases using
the retained 16.1.0 server. The certificate CLI case also passes with the rebuilt
0.64 Host CLI; standalone Wasm is unchanged and its default Wasm check passes.
The first inventory test failure is retained separately.
No new native macOS or full release gate is claimed. Explicitly prepare the locked
tools with `make install-testkit` and `make install-release-editor` before the
maintainer's release gate; this adoption does not install executables.
