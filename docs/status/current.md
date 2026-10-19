# Current status

Date: 2026-10-19

## Delivered source and ownership

Default branch is released **0.35.0**, commit
`583947545a3afde00c4be68dcc6bca1caec8daee`. Contracts, Client and Core 0.35.0
are published, unyanked and checksum/VCS-verified against that commit. Workspace
versions remain 0.35.0. Compatible catalogue repair notes target 0.35.1; no release
or dependency update is part of this handoff refresh.

Blob owns tenant policy, data/references, quotas, provider economics, retained
obligations and recovery. Contracts owns passive DTOs, method catalogue,
application codecs and verification. Client owns explicit replicated calls;
caller intent and recovery policy remain consumer-owned. Standalone owns its
explicit endpoints, authentication, lifecycle and memory custody. Linking Core
never mounts endpoints or lifecycle hooks. Consumer frameworks own their wrappers
and installed integration qualification in their repositories.

The released lock selects Platform 0.1.15, Memory 0.48.2, Storage 0.5.4,
Wire 0.4.4, Host 0.27.6, Build 0.18.0 and Testkit 0.54.1. Shared Tooling remains
0.24.0, reviewed revision `cab12cd3dfa4fdb34846b25397f0d72881bb9ae0`, with 131
selected payloads. Platform owns supported Candid/CDK APIs; no direct exceptions
remain. Memory owns allocation/backing governance; Storage owns collections and
record codecs. Wire owns the two tenant frames; the other 29 owned methods and
standalone lifecycle inputs retain bounded CBOR. Provider/management Candid and
persisted encodings remain separate contracts. Wire owner updates are pending;
this refresh adopts none of them.

## Completed producer work

[#87](https://github.com/dragginzgame/ic-blob-storage/issues/87#issuecomment-6263167596)
is closed for 0.35 producer delivery and acceptance. Its exact-source
[CI matrix](https://github.com/dragginzgame/ic-blob-storage/actions/runs/39160412347)
passes Linux, both native macOS hosts and Rust 1.91 MSRV. Testkit 0.54.1 now
admits locally. An untouched Git-archive snapshot of the release rebuilds
consumer/storage Wasms and passes all thirty client cases (19/4/2/5), using
original lock SHA-256
`cf267a7da81080fd5d3c385f74e05ffbbbf399610349c7f4efe29330d746ac9e`.
Raw source/archive/registry receipts and successful and failed attempts remain
under `.tmp/review-validation/issues0351/`, separate from the pending patch.

[#86](https://github.com/dragginzgame/ic-blob-storage/pull/86) is closed as
incorporated in 0.35.0: all 26 private consumer controls use bounded CBOR, current
Rust/browser callers agree, and the client extraction supersedes its old Core
imports. Public DTO traits and persisted consumer records were preserved.

Earlier producer issues [#72](https://github.com/dragginzgame/ic-blob-storage/issues/72),
[#77](https://github.com/dragginzgame/ic-blob-storage/issues/77),
[#81](https://github.com/dragginzgame/ic-blob-storage/issues/81) and
[#82](https://github.com/dragginzgame/ic-blob-storage/issues/82) are closed with
their delivered-source evidence. Development Rust is 1.99.1; MSRV remains 1.91.
Generic installer/runner matrices from
[#83](https://github.com/dragginzgame/ic-blob-storage/pull/83) are incorporated;
Blob retains its real snapshot, tool admission, hook, publication and recovery
checks. PRs #73, #78 and #80 were closed as incorporated, and #79 as superseded.

## Pending catalogue repair

[#88](https://github.com/dragginzgame/ic-blob-storage/issues/88) and
[PR #89](https://github.com/dragginzgame/ic-blob-storage/pull/89) repair catalogue
macro registration before a following Candid service export. The released
compiler regression produces an empty service; exact-path dispatch registers the
provider method before export. Standalone can then export after mounting instead
of exporting inside its certificate emitter. Names, signatures, codecs, authority,
lifecycle and storage remain unchanged.

Focused Linux evidence passes the compiler regression, strict scoped Clippy,
Rust 1.91 regression and extraction-feature Wasm checks, actual native provider
DID export, fresh production standalone Wasm, exact catalogue/export parity and
two installed certificate/authority/recovery cases. Initial failures remain
retained. These checks qualify the patch source and original released lock, not
later incoming lock edits, native macOS for the patch or installed Canic.
Keep #88 open until default-branch integration and corrected publication. Hosted
results must be checked against the final PR head before integration.

## Consumer adoption and remaining owners

[Canic draft #603](https://github.com/dragginzgame/canic/pull/603), stacked on
#591, retains Blob/Contracts 0.34 and Platform 0.1.13. At `3108ebcac`, its
previous derive-order blocker is fixed. Preflight/security pass, but the exact
[CI run](https://github.com/dragginzgame/canic/actions/runs/39148766791) still
fails fifteen Canic Core Memory API callers in checks/MSRV/Apple Silicon. Intel
macOS stops during Testkit setup with exit 28; that log does not establish its
underlying cause. Ordinary, PocketIC and release-build jobs are skipped.
Earlier 48 macro cases, native/Wasm/MSRV expansion and two byte-contract proofs
used explicit framework substitutes. They do not qualify dedicated/embedded
managed installation or retained-data restoration. Coordinate existing framework
Memory work with #595, consume the published #88 repair, then run those actual
journeys. [Canic #444](https://github.com/dragginzgame/canic/issues/444) owns this
acceptance; normal sibling checkouts are not part of the contribution.

[IC Crates draft #11](https://github.com/dragginzgame/ic-crates/pull/11) and
[its adoption issue #12](https://github.com/dragginzgame/ic-crates/issues/12)
replace two registry Core client imports with Client. The 0.35 publication
prerequisite is satisfied; its refreshed lock resolves the real published
Client/Contracts family without source overrides. Existing PR #7 owns its Canic/database/Memory/Timers convergence.
The draft's current hosted native jobs stop during cold-cache tool setup before
application compilation; [#9](https://github.com/dragginzgame/ic-crates/issues/9)
owns that ordering problem. The refreshed locked native check reaches the same three
registry Memory mismatches as baseline. Keep the draft separate from framework
work and qualify actual managed artifacts/recovery after convergence. The actual
lock/check state belongs to that draft's current handoff, not a source override.

[#76](https://github.com/dragginzgame/ic-blob-storage/issues/76#issuecomment-6263150182)
awaits a published Host retained-write completion helper. Producer PR #172 is
merged, but the inspected registry FS/Process release is still 0.27.6 and lacks
the helper. Retain partial-evidence persistence until the published replacement
is available; do not reconstruct it locally or change staging guarantees.

[#84](https://github.com/dragginzgame/ic-blob-storage/issues/84) remains the
coordinated public/persisted DTO Candid cut. Current journal consumers still
require those traits. Keep the allocation-instrumented private probe until a
Memory observation/fault API preserves attribution; related
[Memory #101](https://github.com/dragginzgame/ic-memory/issues/101) is not proof of
that API. Do not reset uncertainty, funding history, restore fences or retained
obligations to simplify adoption. Provider qualification, GC, billing, verifier
and application acceptance retain their own issues and effect authorization.
The [probe ledger](../evidence/caffeine-probes/README.md) governs provider evidence.

## Evidence and validation limits

The [archived handoff](handoff-2026-10-18.md) and
[pre-refresh released handoff](https://github.com/dragginzgame/ic-blob-storage/blob/583947545a3afde00c4be68dcc6bca1caec8daee/docs/status/current.md)
retain earlier source-bound checks, failed attempts and historical prerequisites.
They are evidence, not current instructions to repeat delivered work. Existing
receipts remain at their owners, including
[Shared adoption](../evidence/shared0240331.md),
[Platform runtime adoption](../evidence/platform01110332.md),
[fixture ownership](../evidence/release-fixture-ownership.md) and
[Canic follow-up](../evidence/canic-dispatch0331.md).

Direct dependency checks cover aliases, kinds, targets and disabled optional
declarations. Role labels and runtime-free library checks do not enforce general
standalone/transitive test-tool deployment exclusions or bind inspected features
and lock selections to deployed bytes. That admission remains missing.
This documentation refresh changes no code, package/version selection, deployed
service, provider effect or consumer acceptance. Its checks are links, snapshot,
formatting and diff consistency; code evidence above retains its original inputs.
