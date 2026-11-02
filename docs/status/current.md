# Current status — 2026-11-02

## Delegated 0.68.1 delivery

Contracts, Client and Engine patch delivery is authorized. The isolated checkout
freezes Build **0.74.1**, Host Foundation **0.74.2**, Host Tools **0.78.1**,
Metrics **0.27.2**, Storage **0.65.3**, Testkit **0.80.2** and Wire/Derive **0.15.1**.
The full release gates must qualify this graph; the earlier focused results below
used its predecessors. Original user edits remain in the main checkout until
delivery reconciliation. New attempts/readback stay in `.tmp/blob0681.cgzub5/`.

## Pending release-validation change

Shared Tooling **0.27.28** is canonically refreshed to committed
`8075b3fd4803d560a1be04768f76701ada3ae8fe` (**150 files**); its dirty sibling
VERSION/notes were excluded. Blob now selects Shared's strict plan runner through
`ci/release-validation.json` and `scripts/release/validation.pl`. The first
plan-based release of each group retains the full roster. Only a prior structured
receipt and a completely classified source delta can enable omissions. Shared,
manifest/lock, compiler, recipe, unknown or ambient build-configuration changes
keep every stage. Configuration hashes are checkpointed and re-admitted before
preparation; receipts retain actual source/group/host/compiler/log identities.

Routine releases omit the redundant original/O3/Os/Oz matrix. Linux compatibility
CI retains it for Engine/shared or uncertain changes and manual dispatch. All
three native hosts keep original-artifact CLI/install/archive/reopen proof and
real metadata rollback checks. Unchanged release machinery can omit the full
publication/recovery fixture matrix; only tenant-client-only changes may omit
the remaining Engine IC scenarios. Other Client APIs/CLI have broader consumers.
See [selection and evidence](../releasing.md#validation-scope).

Focused checks pass: **380 installed tests / 10 ignores** across the disjoint
partitions, compiler **1.99.1 / 13 members**, source/role/pin/snapshot admission,
ShellCheck/Perl, documentation and Make/Testkit routing. The full disposable
release/publication/resume fixture passes; the final configuration binding then
passes selector/failure/drift fixtures and the actual Engine renderer/rollback
check (**83s**). Completion/retention fixtures pass. Evidence, exact source hashes,
patches and logs: `.tmp/release-validation-scope.saFK5a/`. An earlier metadata run
was invalidated by editing its active script; its failure remains retained and
was not counted as passing. No final whole-repository gate, new optimizer matrix
or native macOS qualification was run. Numbered **0.68.1 Unreleased** notes are
open in all groups; package versions are unchanged at source preparation.

## Published baseline

The authorized upstream adoption is delivered. Contracts, Client and Engine
**0.68.0** are published and indexed on crates.io:

| Group | Tag | Exact release source | Full local gate |
| --- | --- | --- | --- |
| Contracts | `contracts-v0.68.0` | `86aa1d109f65f4ac4385e0952ecb88861b752e9c` | 1297s |
| Client | `client-v0.68.0` | `c2ad414e7502be868372ceded3eb67ef853c607a` | 1233s |
| Engine | `engine-v0.68.0` | `b0574945cd20c05a3ed350d9d65edb803eb32a4e` | 1216s |

Every standard runner validated, prepared versions, committed/tagged and atomically
pushed its own group. Each publication ran its required compiler/Clippy/archive
checks; independent non-yanked registry metadata, archive checksums and VCS source
readback match the exact tags and package paths. All thirteen member versions
are 0.68.0. Publication does not deploy or qualify a live storage service.

## Frozen delivered supplier graph

Read the selected group's tag/receipt for its original source-bound graph:

| Supplier | Delivered selection |
| --- | --- |
| Platform / Runtime / exact guard / Types | **0.18.0 / 0.18.1 / 0.18.1 / 0.17.0** |
| Wire / Derive | **0.15.0** |
| Storage | **0.65.0**, replacing retired Memory and stable-structures |
| Metrics | **0.27.0**; native Testkit retains arithmetic-only **0.26.1** |
| Testkit / PocketIC server | **0.80.0 / 16.1.0** |
| Build / Host Foundation / Host Tools | **0.74.0 / 0.74.1 / 0.78.0** |
| Shared Tooling | **0.27.28**, `8c7fae6605c69a930e3b1f14e1ff5c4c9dafbcb7`, **150 files** |
| Compiler / browser tooling | Rust **1.99.1**, Node **24.21.0**, npm **12.2.1** |

There is one Runtime/guard, Types and Wire identity. Native Host is unified;
production excludes native tooling, old Memory/stable-structures and schema/test
features. The explicit Caffeine SDK override is unchanged. All consumption uses
published dependencies; siblings remained read-only. Storage #152, Wire #43 and
Shared #346 supplying gaps are resolved by this family and canonical export.
The Client's preserved direct Types/Runtime migration is included.

## Qualification and failures retained

Each full gate passed **688 native** and **380 PocketIC** tests, with **10 explicit
ignores**, plus strict Clippy, compiler/pin/boundary checks, tooling/release/hook
fixtures, formatting, Rustdoc, Wasm and exact extracted archives. Original/O3/Os/Oz
installation/restoration passed in each gate. Focused generated TypeScript checks
also passed **27 methods / 137 frozen Rust frames**, including malformed input,
identity, direction, limit and decimal checks; no vectors were regenerated.

Released-version standalone/probe Wasms and the CLI were rebuilt from the Engine
tag. Five lifecycle/history/fencing/obligation cases and the signed installation
carrier pass against actual Engine 0.68.0. This supplements the pre-version gates;
it does not establish native macOS or live Caffeine behavior. Hosted
[Engine-source CI 41512505836](https://github.com/dragginzgame/ic-blob-storage/actions/runs/41512505836)
is still in progress at readback. Earlier 0.67 native-host acceptance and closed
[#112](https://github.com/dragginzgame/ic-blob-storage/issues/112) remain historical.

Two unsuccessful original-checkout attempts remain retained: a corruption test
expected an inner checkpoint error after the new outer cell checksum already
refused the bytes; the corrected test requires rejection, unchanged damaged bytes,
preserved live state and recovery from the original image. A later gate detected
an incoming Runtime 0.18.1 lock edit and refused reuse. The authenticated additive
patch was retained, then all complete gates ran from isolated frozen source.
Neither partial attempt was treated as a passing gate or a release.

Evidence and before-state: `.tmp/upstream-platform018.ZS89rK/`. Its
`isolated-delivery.current` selects the full release/publication logs;
`release-checkout/.git/release-state/` retains the three plans and gate evidence.
The `contracts068-readback`, `client068-readback` and `engine068-readback`
directories retain official metadata/archives/VCS checks. Released Wasm hashes
and six installed checks are retained beside them. Earlier adoption and dirty-work
backups remain `.tmp/upstream-20261102.6ReUDL/` and
`.tmp/issue112-delivery-recovery.8cQudg/`. Compiler scratch stays in each checkout's
own `target`; provenance outside it survives `cargo clean`. Global npm is unchanged;
`BLOB_BROWSER_NPM` selects the retained isolated 12.2.1 tool when needed.

## Preserved incoming work and next acceptance

Before the delegated 0.68.1 snapshot, the original checkout contained pending
**Metrics 0.27.1 / Testkit 0.80.1 / Storage 0.65.1 / Host Artifacts 0.74.2** were
already selected when the release-validation work began. Cargo.lock remains
byte-identical to that captured before-state. The installed tests above use this
graph; Testkit setup explicitly reused its prepared CLI/server. This is focused
evidence, not a complete new dependency-adoption/release gate. Earlier restoration
and recovery-stash evidence remains with the original delivery records.

**Old installations require safe retirement and fresh whole backing.** Storage's
map/region/collection and private key formats are incompatible; no legacy reader,
automatic migration or reset-on-error exists. Preserve provider objects, uncertain
paid effects, balances and billing custody under the [retirement contract](../retiring-installations.md).
Application Wire bytes, endpoint identities, authentication and exact retries
remain unchanged. Same-release recovery remains required.

Canic owns its adapter, dedicated/embedded proof and production limits; its old
Blob 0.66 results do not qualify this new composition. Blob #5/#31/#6/#34 and
Canic #622/#494 retain application/provider acceptance. No live service reset,
provider call, paid effect or sibling edit occurred. Future group delivery stays explicit under
[the release guide](../releasing.md); later source cannot replace an older tag's
receipt or publication evidence.
