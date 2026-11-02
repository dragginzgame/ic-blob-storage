# Current status — 2026-11-02

## Released baseline and pending adoption

Contracts, Client and Engine **0.67.0** are published. Their exact sources are
`contracts-v0.67.0` (`430cc5cb`), `client-v0.67.0` (`3613147d`) and
`engine-v0.67.0` (`7a027212`). [Delivery record 6e31c513](https://github.com/dragginzgame/ic-blob-storage/commit/6e31c51360116aece3686da46aaabbd1a5f27a69)
retains the detailed source-bound release evidence. The compiler floor and exact
pin are Rust **1.99.1**. [Native CI run 41438599507](https://github.com/dragginzgame/ic-blob-storage/actions/runs/41438599507)
passed on that delivered source: Linux, both macOS architectures, library and Wire
jobs. [#112](https://github.com/dragginzgame/ic-blob-storage/issues/112) is closed.
Those hosted results do not qualify the pending supplier graph below.

The maintainer requested all new published upstream packages. The adoption source is prepared for the maintainer-authorized release and
publication; package versions remain 0.67.0 until the normal group runners prepare them. All three group
ledgers now describe pending **0.68.0** hard cuts. The existing Client direct
Types/Runtime migration and incoming supplier edits are preserved. Its original
backup remains `.tmp/issue112-delivery-recovery.8cQudg/`; nothing from that backup
was discarded, staged or overwritten by delivery.

## Selected published graph

Official registry metadata and checksummed published archives establish:

| Supplier | Selection |
| --- | --- |
| Storage | **0.65.0**, replacing the retired Memory owner and stable-structures |
| Metrics | **0.27.0** |
| Build | **0.74.0** |
| Host artifacts / fs / http / process | **0.74.1** |
| Host Tools | **0.78.0**, only `ic-interface`; optional Platform 0.18 disabled |
| Testkit | **0.80.0**; PocketIC server **16.1.0** unchanged |
| Wire / Derive | **0.15.0** |
| Platform / Runtime / exact guard / Types | **0.18.0 / 0.18.0 / 0.18.0 / 0.17.0** |
| Shared Tooling | **0.27.28**, committed `8c7fae6605c69a930e3b1f14e1ff5c4c9dafbcb7`, **150 files** |
| Browser package manager | npm **12.2.1**; Node **24.21.0** unchanged |

All external direct Rust requirements and browser dependencies were checked.
Other direct third-party/browser packages are already current. The explicit
Caffeine SDK Core override stays local; first-party browser code uses Platform.
Native Testkit 0.80 retains arithmetic-only Metrics 0.26.1; no metrics values
cross that boundary. Host Foundation is now unified at 0.74.1. Deployment has
only Metrics 0.27 and excludes Testkit/Host.

The published family now resolves with one executor guard and Types identity.
Storage 0.65 selects Runtime 0.18 directly, resolving
[Storage #152](https://github.com/dragginzgame/ic-storage/issues/152).
Wire 0.15 supplies Types 0.17 schemas, resolving the supplier gap in
[Wire #43](https://github.com/dragginzgame/ic-wire/issues/43). There are no source
patches, second executors or schema bridges. Storage 0.65's B-tree map format also
refuses 0.64 AVL maps, sets and regions; whole-backing retirement remains required.

## Storage and consumer disposition

Storage owns backing identity, allocation grants, collections and fallible codecs.
Blob publicly re-exports `ic_storage`; no direct or transitive Memory or
stable-structures remains. Host composition uses bounded `StorageConfig`, named
`AllocationGrants` and the same fourteen logical requests. Passive private key
records implement Storage's codec using bounded Wire; no foreign Principal
implementation or compatibility re-export is retained. Private journal fixtures
use canonical governed cells with explicit load ceilings.

**Retire the old installation and use fresh whole backing.** Allocation,
collection and private key formats are incompatible. No legacy reader, reset-on-
error or automatic migration exists. Preserve provider objects, uncertain paid
effects, balances and billing custody independently under the
[retirement contract](../retiring-installations.md). Endpoint bytes, identities,
authentication and exact retry bytes are unchanged. Cell loading refuses malformed
values without repair; mutation/backing failures retain IC rollback semantics.
Native memory provides no transaction guarantee.

Canic owns its adapter, dedicated/embedded tests and production limits. Its Blob
0.66 local qualification does not qualify this new Storage composition. Native
library adoption cannot establish live Caffeine or application acceptance. Keep
Blob #5/#31/#6/#34 and Canic #622/#494 tied to their actual evidence owners.

## Local qualification and remaining checks

Current native and seven Wasm Clippy selections pass, as do **694 focused native
tests including doctests**, the **27-method / 137-frame** frozen TypeScript suite,
compiler, snapshot, dependency-role/pin, direct and deployment boundaries with
negative fixtures, documentation and formatting. Testkit 0.80 was explicitly
prepared/admitted using the unchanged server identity. The standalone was built
in isolation from probe features; all six probe Wasms and native CLI also built.
No frozen vectors were regenerated. The first Contracts gate stopped before
version/ref effects at a checkpoint-corruption test that expected an inner error;
Storage now refuses its damaged outer cell earlier. The corrected installed test
requires the checksum rejection, unchanged damaged bytes and preserved live state,
then successful recovery from the original bytes. Its four checkpoint cases and
the 250 installed cases not reached by that gate pass. Failure evidence remains
under Git's `release-state/validation-failures/20261102T180048Z-1566564-16-test.log`.
Restart complete normal gates from the corrected committed source; partial stages
and the older family's installed results do not substitute for that gate.

Evidence for this continuation is `.tmp/upstream-platform018.ZS89rK/`:
before-state, sparse-index records, checksummed published sources, committed Shared
export and all command logs. The earlier Storage 0.64 adoption and its 13 local
installed cases remain in `.tmp/upstream-20261102.6ReUDL/`; they do not qualify the
new family. Both preserve receipts outside `target` across `cargo clean`; build
scratch remains under this repository's `target`. The original dirty-state backup
and historical 0.67 delivery records remain unchanged. An isolated npm 12.2.1
executable in the earlier evidence directory supplies `BLOB_BROWSER_NPM`; global
npm remains 12.2.0 to avoid changing sibling tooling. Shared 0.27.28 now supplies
the missing guide companion through its canonical closure, fixing
[Shared #346](https://github.com/dragginzgame/shared-tooling/issues/346).

The maintainer authorized deployment of these crate changes: complete normal
0.68.0 release/publication supplier-first for Contracts, Client and Engine.
Full release/archive/optimizer gates remain required; no hosted macOS, mainnet
or live-provider qualification is claimed for this pending family. No live
service reset, paid effect or sibling edit is authorized by library delivery.
Shared's optional group validation plans are not activated. Use explicit
`RELEASE_GROUP` and the [release guide](../releasing.md); retain each group's
original source, tag, receipt and publication readback.
