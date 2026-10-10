# Canic integration source review — 2026-10-10

This review checks delivery and the host boundary; it runs no Canic build,
PocketIC case, provider request or paid effect. The incoming Blob lock changes
are preserved and unqualified by this documentation batch.

## Reviewed inputs

Blob HEAD is released 0.21.5,
`9a808cdf3cee2dd7153e50dd6fa10592f72f6137`, with validated source
`7e74799bdc1e8ff31f97cba6e1758e61b1e9a339`. GitHub main matches that HEAD.
The crates.io sparse entries independently observe runtime/contracts 0.21.5,
both un-yanked and declaring Rust 1.88; the runtime requires Memory `^0.33`.
This is registry metadata readback, not new package archive authentication.

Canic's GitHub main is `c4c046f947b2b28f4342cbf6efe9221ba1ed5f70`.
Its committed adapter manifest requires exact Blob 0.17.2 and Memory 0.31.
The local checkout instead has committed HEAD
`ac55e50334dd6479ec36f404e89e60bcfe9184d6` with Blob 0.21.0, contracts 0.21.0
and Memory 0.33 in the independent adapter catalog. The adapter source/catalog
has no local diff, while independent locks and other Canic work remain dirty.
Do not attribute that whole working graph to either commit or describe local
adoption as pushed delivery. The adapter sparse-index request returns HTTP 404;
there is no observed registry `canic-blob-service` entry.

Raw input copies, sparse responses, adapter diff, checkout status and hashes of
the inspected source remain in `.tmp/canic0216-01/`. Reviewed input hashes are
rechecked at completion. The earlier Canic qualification remains at its owner;
none of its compiler or artifact evidence is relabelled as a fresh run here.
Unrelated Canic CLI output/scaffold edits appear during review; entry/final status
captures retain them separately. The inspected adapter inputs remain unchanged.

## Boundary review

- `ops::memory` registers Blob's complete installation request inventory and
  opens the host's already committed runtime by key. It does not bootstrap a
  second manager. The adapter uses seventeen grants, with embedded placement
  selected by its owning application.
- `ops::install` validates the actual service Principal and uses Blob's compiled
  `LIBRARY_VERSION` before opening grants. `ops::restore` synchronously reopens
  that same release and observes platform-version continuity. Linking the core
  takes no lifecycle or endpoint ownership.
- The endpoint macro constructs context from the actual canister and caller.
  Canonical DTOs carry the per-upload permission; no installed single-uploader
  field or framework-specific authority is reintroduced. Certificate refusal
  traps instead of returning a successful provider-shaped error reply.
- The adapter mounts funding history, exact outcome and passive assessment.
  It does not compose paid dispatch, credit acquisition or budget renewal.
  Blob already supplies host-internal confirmation/renewal. Provider receipt and
  complete account-activity evidence remain necessary before repeated dispatch.

[Blob #41](https://github.com/dragginzgame/ic-blob-storage/issues/41)'s recorded
consumer feedback establishes prior dedicated/embedded managed 0.21 acceptance:
one Memory/Timers/Metrics identity, exact 32-method Candid parity, separate
capacity headrooms, two retained uploader permissions and same-image recovery.
That is the owner's execution evidence, not this source review or production
application/provider acceptance.

The practical adoption recipe is maintained in
[the dependency guide](../dependencies.md#canic-integration).
[Canic #444](https://github.com/dragginzgame/canic/issues/444) owns adapter delivery
and managed/application acceptance; its framework family is
[Canic #33](https://github.com/dragginzgame/canic/issues/33).
[The delivery feedback](https://github.com/dragginzgame/canic/issues/444#issuecomment-6095255962)
records the resolved upstream publication prerequisite and concrete next steps.
[The funding recipe](../funding-consumer-qualification.md) links the separate
Canic #494 / Blob #34 evidence boundary. No speculative dispatch endpoint,
compatibility adapter or consumer fixture is added to Blob.

## Verification

Focused documentation-link and runtime-free-contract boundary checks pass.
Reviewed Canic input hashes and the incoming Blob lock diff remain unchanged.
No Rust definitions are removed; public APIs, wire/stored schemas, dependencies
and host behavior are unchanged by this batch. Full CI, managed requalification,
sibling edits and release/publication effects are outside this review.

## Authorized Canic follow-up

The maintainer subsequently authorized updating Canic's adapter and three locks
and running its focused managed tests. That separate Canic batch now selects
published Blob runtime/contracts 0.21.5, Auth/protocol-types 0.2.7, Memory 0.33.4,
Metrics 0.3.5 and Timers 0.16.4; its harness uses Host 0.10.1/Testkit 0.28.0.
The Canic root lock is unchanged. Earlier source-review observations above
retain their original identity; this is an uncommitted consumer update.
[The managed adoption feedback](https://github.com/dragginzgame/canic/issues/444#issuecomment-6095739550)
records the new qualification separately from the original review.

Both complete managed Apps build at Rust 1.91. Eight normal-Wasm graphs have one
selected Memory/Timers/Metrics identity and both Blob Candid projections match
all 32 canonical methods strictly. Two actual managed PocketIC cases pass in
6.98s, covering dedicated/embedded target-bound initialization, caller refusals,
two uploader permissions, capacity and fenced same-image restoration/repeated
resume. The embedded counter retains 42. Successful certificate issuance,
application login and deployed provider/funding behavior remain unqualified.

The guide's embedded `backend` selector builds only one role, with null release
identity/manifest. Keep that compile-only result separately; the corrected recipe
builds the complete App. Canonical allocation-peer refresh/verification, Testkit
0.28 setup and offline/pathless admission, adapter native/test Rust 1.91 and
strict native/Wasm Clippy pass. Canic's current handoff owns its result; source,
graphs, artifact/test hashes, original attempts and final checks stay in
`/home/adam/projects/canic/target/review-validation/blob0215-20261010/`.
The root's captured source inputs stay unchanged; generated consumer manifests
and build-output Rust changes are recorded separately. No Rust definition is
removed. Canic framework minimum/publication and adapter publication still belong
to its coordinated release, separate from this local qualification.
