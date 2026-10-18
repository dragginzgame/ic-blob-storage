# Issue maintenance and gateway fixture dependency

2026-10-18. HEAD remains released 0.33.0 at
`55d70cb0351093b5cc25fcc7b9c27a04e610a1f8`; pending compatible notes remain 0.33.1.
The incoming Platform/Shared Tooling work and selected Metrics 0.14.1 are preserved.
Before-images, input hashes, API/CI readbacks, comments and raw checks remain in
`.tmp/review-validation/issues0331-01/`. No source commit, version, release,
registry publication, deployment or paid effect ran in this pass.

## Local dependency repair

[#82 — Review the unused core dependency in the gateway-source fixture](https://github.com/dragginzgame/ic-blob-storage/issues/82)
is implemented locally. The complete gateway-source canister has no direct core
import, and actual Wasm compilation confirms no generated/feature-only reference
needs that declaration. Remove only its `ic-blob-storage` manifest declaration
and matching lock dependency edge; no external version is selected. Root catalog
ownership and real core consumers remain. Shared `blob-test-protocol` still uses
core, so this does not claim to remove core from the resolved transitive graph or
reduce deployed bytes.

Fresh gateway-source, standalone and storage-probe release Wasms build on the
incoming graph. Actual Rust 1.91 Wasm checking of gateway-source passes too. The actual installed gateway scope/cancellation, account
scope/state-preservation and malformed-chunk/read-capacity cases pass against
these fresh artifacts. The first gateway attempt stopped before calls because
the sandbox denied PocketIC's localhost bind; its log is retained separately.
The retry with local-server access passes. All three are controlled local peers,
not deployed provider qualification. Formatting, Platform direct-dependency boundaries, runtime-free Contracts/CLI
graphs, dependency pins, documentation links and the Shared 0.24 snapshot checks
pass. The exact commands, output and artifact hashes remain in the raw evidence
directory. Native macOS and full CI are not
rerun for this dirty repair; #82 remains open for delivery/required-host acceptance.

## Delivered adoption evidence

The [workflow for released 0.33.0](https://github.com/dragginzgame/ic-blob-storage/actions/runs/39108113676)
passes Rust 1.91 and native Ubuntu 24.04, macOS 15 Apple Silicon and macOS 15 Intel.
Each native lane includes actual coordinated archive qualification, native
CLI/installation/Metrics checks and original/optimized Wasm installation and
restoration. Its lock selects Build 0.16.0 and Host 0.27.1. These are remote
results for that exact delivered source, not the current dirty graph.

That source contains the published Host directory/descriptor-read and buffered
output owners, the Host line framing owner in native control/browser support,
and Build's complete CLI source fingerprint capture. The earlier source-input
membership experiment and reader refusal evidence retain their original graphs
in [the CLI receipt](canic-issues0310.md) and [reader receipt](host-readers0310.md).
The delivery/native/archive gaps recorded there are now cleared at 0.33.0,
supporting closure of
[#69 — Adopt published Host custody/output APIs and IC Build archive qualification](https://github.com/dragginzgame/ic-blob-storage/issues/69),
[#74 — Adopt published Host framing for native control channels](https://github.com/dragginzgame/ic-blob-storage/issues/74) and
[#75 — Adopt published IC Build source fingerprints with a qualified compiler graph](https://github.com/dragginzgame/ic-blob-storage/issues/75).

The selected published Host 0.27.2 still has retained-file creation/publication,
not the proposed retained write-completion API. That separate adoption remains
[#76 — Adopt Host retained-record write completion after publication](https://github.com/dragginzgame/ic-blob-storage/issues/76);
this pass does not recreate it locally. Canic's shared-workflow issue remains
open for actual consumer acceptance; [draft PR #603](https://github.com/dragginzgame/canic/pull/603)
records the concrete adoption and Memory caller blocker. Provider/account,
application users, completion-worker operation, serving and media-map acceptance
remain distinct from these delivered mechanics and local fixture checks.
