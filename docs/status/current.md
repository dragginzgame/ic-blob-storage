# Current status

Date: 2026-10-12

## Released source and current batch

Released Blob is `9398b06bfd2f451ca9e883f18d06a1589f1e3a47` (0.22.10),
validated parent `aaa067becd925735ac304db049f77f82ddb1b24d`. All twelve package
versions and the finalized receipt remain 0.22.10; receipt file hashes match.
Both runtime/contracts packages are observed published and un-yanked with Rust
1.88.0. The released lock selects Memory 0.35.12, private Metrics 0.6.6,
Testkit 0.34.0 and all four Host 0.14.1. That release records Shared 0.6.3
`4fd979d13c32adb3cb2f51f213ebfc4f98339b3d`, 102 selected files.
Subsequent incoming lock edits select all four Host crates 0.14.2, Candid/derive
0.10.38 and unicode-ident 1.0.27. They are preserved without reselection.
Further incoming Cargo edits now select Memory 0.35.13, Metrics 0.7.0 and
Testkit 0.34.1; the requested update now selects all four Host crates 0.14.4.
New focused native/boundary tests and Rust 1.88 checks pass on this held graph.
Full product qualification remains pending: the Metrics 0.7 storage probe
still requires the coordinated wide-total adaptation under #60.

Pending adoption now records Shared 0.6.6
`a15eb9d406d287697df9bc8fa96db8cda6721a71`, 105 files, through clean isolated
canonical preview/refresh. Blob uses `make/consumer.mk` once; common hook and
snapshot recipes are delegated, while product gates, ordered Testkit extensions
and direct release admission remain local. CI/release validation now use the
canonical declared-input checkpoints at every stage, covering all twelve member
manifests plus lock, compiler/tool pins, snapshot/Make and browser selectors.
Nested gates retain their original baseline; drift stops later work without an
automatic fetch/retry. The prior [common Make/input-guard record](../evidence/shared06502211.md)
retains its original inputs. The [latest adoption record](../evidence/shared066-host014402211.md)
qualifies the new derive formatter and Host patch selection, with raw
`target/review-validation/shared066-host0144-02211/`.

Pending compatible 0.22.11 corrects Canic integration guidance and acceptance
boundaries. It records the actual Root-driven installation proof and the separate
adapter memory-owner cleanup, replacing stale current-graph claims. Dependency,
Memory and funding recipes now identify the actual released graph. The batch
also adopts the common Make/input guard and retains the earlier Host/Candid
qualification below. Shared adoption changes no library API or stored schema.
Incoming Metrics requirements and caller/report qualification remain separate.
See [the continuation review](../evidence/canic02211.md) and raw
`target/review-validation/canic02211/`. The core/contracts/standalone Rust source
diff from released Blob 0.22.8 to 0.22.10 is empty; new dependency graphs still
require their own execution evidence. Cross-release transitions remain
retirement/reinstall-only, including compatible library updates.

## Canic integration and next work

Published Canic/adapter remains 0.111.0 at
`25b73cd403b20b4e148c6e340f8c760a26d97f6e`. Its newer integration changes are
uncommitted. The inspected dirty root selects Blob/contracts 0.22.9, Memory
0.35.12, Metrics 0.6.6 and Timers 0.17.14. Both independent managed fixtures
remain on qualified Blob 0.22.8, Memory 0.35.9, Metrics 0.6.4 and Timers 0.17.10.
Do not attribute root compiler results to those managed artifact graphs.

[Canic's Root-driven proof](https://github.com/dragginzgame/canic/issues/444#issuecomment-6110213100)
now qualifies actual controller-authorized dedicated installation, exact target/
configuration, restart and discarded binding reply reconciliation, conflicting
binding refusal and effect-free installed replay. The retained original log
passes the Root case in 13.72 seconds; the governed runner takes 23 seconds.
Its managed artifact still uses Blob 0.22.8. This is local execution of the
production binding path, not live deployment or per-project parent acceptance.

[The adapter cleanup](https://github.com/dragginzgame/canic/issues/444#issuecomment-6110870768)
uses upstream `ServiceInstallationMemories` directly, retaining validation-before-
open and synchronous restoration. Original native/Wasm library checks pass
against Blob 0.22.9/Memory 0.35.11. No new execution is performed during this
read-only review; original evidence retains its identities.

Next code work belongs to Canic under [#444](https://github.com/dragginzgame/canic/issues/444):
freeze the intended released graph in root and both independent fixture locks,
rerun complete managed builds/identity/Candid checks and Root/dedicated/embedded
recovery journeys, then deliver the pending integration under Canic's own release
policy. This repository owns no downstream wrapper, dependency or fixture.
Canic's pending batch changes exposed Auth/Backup type identities under
[#517](https://github.com/dragginzgame/canic/issues/517); its maintainer must resolve
the minor-release boundary before publishing. Qualification of the Blob
composition does not establish patch compatibility for the whole Canic batch.
The requested continuation still observes GitHub main and the local receipt at
0.22.10; the 0.22.11 notes remain a local draft. A scope question is pending for
the root/two-fixture lock alignment and managed checks in Canic, because this
repository's AGENTS.md keeps siblings read-only without explicit authorization.
The additional source review confirms the target-bound initializer remains
controller-only; [Canic #493](https://github.com/dragginzgame/canic/issues/493#issuecomment-6111747688)
owns the unimplemented application-parent route. The passing controller proof
does not complete it. The adapter still mounts only passive funding inspection;
#494 remains dependent on Blob #34's provider-evidence prerequisite.

The actual application then qualifies two authenticated browser users and exact
tenant grants, independent verifier completion and its publication transaction.
[#31](https://github.com/dragginzgame/ic-blob-storage/issues/31),
[#33](https://github.com/dragginzgame/ic-blob-storage/issues/33) and
[#4](https://github.com/dragginzgame/ic-blob-storage/issues/4) retain those owners.
Provider GC/deletion/billing and authenticated credit/complete account activity
remain [#32](https://github.com/dragginzgame/ic-blob-storage/issues/32),
[#34](https://github.com/dragginzgame/ic-blob-storage/issues/34) and
[Canic #494](https://github.com/dragginzgame/canic/issues/494). No substitute receipt,
placeholder callback or paid probe is authorized by this continuation.
[Toko Miner #6](https://github.com/dragginzgame/toko-miner/issues/6) retains its
production/frozen Release-profile acceptance; Toko's per-project journey is separate.

## Validation and retained work

The latest Shared 0.6.6 refresh changes three selected files and retains the
105-file companion closure. Explicit setup admits cargo-sort-derives 0.14.1;
complete tools, declarations, root formatting and actual hook checks pass.
Blob has no exported macros, dollar-prefixed macro paths or corruption-workaround
markers to remove. A separate-crate fixture exercises direct and conditional
macro derives through the actual formatter, comparing exact sorted bytes and
compiling the consuming crate. Installer fixtures also pass.

The four Host 0.14.4 archives match their locked checksums and release
`f93cd8854d500616a73eda713d215a20ff0617c7`; their Rust sources are unchanged
from 0.14.2. Only those four lock selections change during this requested update.
635 core/contracts/CLI/example tests and two maintained native Candid/process
boundary tests pass. Rust 1.88 all-target/all-feature checks pass for
core/contracts/CLI/standalone. These results use the current held graph, including
Memory 0.35.13, Metrics 0.7.0 and Testkit 0.34.1; they do not execute the storage
probe, PocketIC instances, Canic, a new Wasm check or the full product gate.
Prior MSRV/Wasm results below retain their earlier graph identities. No Rust
source cleanup is justified by this tooling-only Host patch.

The selected Host 0.14.2 archives match their locked checksums and release
`fed8724e79862d8273d01823619fe648c981c019`; all four Rust source trees are unchanged
from 0.14.1. Candid 0.10.38 changes integer decoding and type-resolution error
handling; derive source is unchanged. The exact archives and diff are reviewed,
rather than inferring unchanged behavior from patch version numbers.

Explicit locked preparation passes. On held lock SHA-256
`64c75a4c2251b958e7a3a44dcf460245b76821c658195cfa0d4334420728d06f`,
635 native core/contracts/CLI/example tests pass with Rust 1.99.0. Maintained
standalone Candid-parity and buffered-child-output cases add two passing tests;
neither creates a PocketIC instance. The initial library-only selector runs zero
tests and is correctly refused; its log remains retained and is not acceptance.
The existing buffered reader helper remains necessary to preserve prefetched
protocol bytes and stderr/nonzero completion; Host owns child cleanup.

Actual Rust 1.88 native all-target checks pass for core/contracts/CLI/standalone;
Wasm checks pass for core/contracts/standalone/storage probe. Complete offline
tool admission and runtime-free contracts/CLI graph checks pass. Manifest, lock
and index remain unchanged throughout qualification. Inputs, original attempts,
archive checks/source diffs and successful commands remain in
`target/review-validation/host0142-candid01038-02211/`. No full gate or consumer
framework execution is inferred. These results precede the newer dependency
edits and Shared adoption; they are not relabelled as the current product graph.

Final common Make, installer/staged-receipt, parallel Testkit, actual hooks and
consumer nested input-guard fixtures pass, including genuine Bash 3.2.57 on
Linux. Complete Blob release-adapter/routing checks pass with substitute effects.
Original failed bootstrap and optional Shared-only suite attempts remain retained;
the bootstrap now substitutes scripts while keeping real Make recipes. Actual
focused CI dispatch passes snapshot, complete tools, declarations and formatting.
Explicit Testkit 0.34.1 setup reuses the admitted shared payload after an initial
held-lock refusal; final offline admission passes. Earlier selections remain.
No full product gate or new Canic execution is run.

[Matching 0.22.10 hosted CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38155999623)
is now completed with a cancelled conclusion. #56–#59 native/archive/tooling obligations are not
closed by Git/registry publication or this documentation review. The prior
cache failure and explicit successful six-Wasm retry remain in
`target/review-validation/metrics06502210/`, bound to their original lock.
[Shared #129](https://github.com/dragginzgame/shared-tooling/issues/129) owns the
early input-drift owner, now adopted at Blob's actual stage boundaries; existing
release guards remain.
[Blob #60](https://github.com/dragginzgame/ic-blob-storage/issues/60) tracks Metrics
0.7 wide-total adoption separately; no new dependency hard cut starts here.
The earlier Metrics 0.7.0 registry refusal is historical. Its incoming API now
requires `record_sample` totals as `u128`; the storage test probe still supplies
`u64`. Its counter, protocol/report and assertions need coordinated adaptation
before a current product gate can pass. That work remains #60, not completed by
tool admission or earlier Metrics 0.6 runtime results. No issue closes here.
Read-only findings remain in `target/review-validation/canic02211/more-issues-review.json`.

Documentation links and diff/source consistency checks also pass. No full
CI/release gate, sibling file edit, consumer build, Git delivery, version mutation,
publication, deployment, provider effect or cleanup is performed.

[Prior handoff](review-before-02211.md) preserves 0.22.10 preparation and the
preceding Shared/Host/cache qualification records without relabelling them.
