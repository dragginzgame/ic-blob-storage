# Current status — 2026-11-02

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

The original checkout is fast-forwarded to the delivered Engine source. Incoming
**Metrics 0.27.1 / Testkit 0.80.1** lock edits were backed up, temporarily stashed,
restored and checked for exact preservation of all external package selections.
They remain **unstaged and unqualified**, outside the frozen 0.68.0 delivery.
The recovery stash SHA and pre/post comparison are retained with the evidence.
Review/qualify those compatible follow-up edits before claiming their adoption.

**Old installations require safe retirement and fresh whole backing.** Storage's
map/region/collection and private key formats are incompatible; no legacy reader,
automatic migration or reset-on-error exists. Preserve provider objects, uncertain
paid effects, balances and billing custody under the [retirement contract](../retiring-installations.md).
Application Wire bytes, endpoint identities, authentication and exact retries
remain unchanged. Same-release recovery remains required.

Canic owns its adapter, dedicated/embedded proof and production limits; its old
Blob 0.66 results do not qualify this new composition. Blob #5/#31/#6/#34 and
Canic #622/#494 retain application/provider acceptance. No live service reset,
provider call, paid effect or sibling edit occurred. Shared's optional group
validation plans remain inactive. Future group delivery stays explicit under
[the release guide](../releasing.md); later source cannot replace an older tag's
receipt or publication evidence.
