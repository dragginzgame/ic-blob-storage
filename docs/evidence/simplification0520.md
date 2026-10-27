# 0.52 simplification and standalone build qualification

This pending hard cut leaves Blob owning admission, tenant policy, original
manifests, references, provider obligations, funding and gateway bookkeeping.
Standalone owns mounting/lifecycle; consumers own their framework adapters and
client body transport. Application codecs remain on Wire; required external
provider/management contracts remain on Platform. No sibling source was changed.

## Implemented boundaries

Published Memory/Storage **0.51.12** now supplies the loader requested by
[Persistence #136](https://github.com/dragginzgame/ic-persistence/issues/136). Blob
uses it for installation configuration and the gateway record. Funding accounting
is not a singleton: its credit-receipt index remains in the existing map.
No local cell parser, default record or compatibility adapter was added. The
supplier issue remains open for its separate macOS native tooling acceptance
under Persistence #135; Blob’s local adoption does not waive that gate.

- Certificate resolution retains its checked permission record; final exposure
  revalidates the original permission and host evidence, then passes that record
  through synchronous policy/commit without rereading it. The normal issuance
  path has two permission-map lookups instead of five by source tracing. This is
  not an IC instruction measurement. Issuance still commits before returning a
  certificate, refuses replay and preserves independent fences/authority checks.
- A manifest has two tagged records in one collection: leaves and headers.
  Descriptor serving decodes only headers; full inspection, preparation replay
  and restoration join both. Original header order, roots and limits remain.
  Preparation now writes two records; restoration rejects missing, reversed or
  orphaned parts without repair. No per-leaf index or extra manifest memory was added.
- Pending unacknowledged deletion candidates have one derived stable index in the
  upload owner. Polling traverses candidates, checking authoritative lifecycle and
  identity records, instead of settled lifetime history. Last release enqueues;
  gateway acknowledgment or authenticated physical deletion removes the candidate.
  Exact release replay does not re-enqueue acknowledged roots. Restoration checks
  exact membership during its existing lifetime validation, without repair.
  This adds one memory and write/index maintenance; it creates no second authority,
  external journal, paid call or automatic physical/billing confirmation.
- The maintainer-approved retirement removes durable canister read sessions,
  chunk transport/callback tickets, read budgets/status and gateway read-generation
  invalidation. Direct download descriptors, browser/native body verification and
  independent transient source-read probes remain. Installation grants fall from
  seventeen to fifteen, including the new candidate index. General logical-memory
  read instrumentation remains; it measures store IO, not retired sessions.

## Hard-cut adoption

Use retirement/reinstall across releases, preserving original uncertain effects,
provider objects, balances and continuing bills. The installation format is now
`ic-blob-storage/installation:singleton-cells`. There is no migration,
reset, dual reader or compatibility re-export. Same-release recovery remains.
A populated captured pre-cut image rejects the new artifact, retains byte-identical
stable memory and continues to serve the original tenant/admission observations.

Consumers remove `ServiceReadInput`, the configuration/status `reads` fields and
read-session owner/transport integrations. Store configuration takes service and
funding inputs; operator inspection borrows three owners. Exposure preparation
and commit take the checked `UploadPermission` rather than reconstructing a DTO.
Hosts obtain the fifteen grants through the canonical request factory.
Configuration, local-status and install Wire identities use `0052`; other owned
method identities, endpoint names/modes and external provider Candid are retained.
CLI local status keeps **schema 1**, with its retired read fields removed. Saved
old proposals/runs keep their original tools; new tools do not silently rewrite them.

Read-only Canic inspection finds no production read-session implementation in
`integrations/blob-service/src/`; its remaining `ServiceReadInput` constructors
are in three testing-internal fixtures. Canic must adopt the new contract and
qualify its own mounted service. Its release root already selects size optimization
and full LTO; Blob's workspace profile does not apply to that consumer build.

## Standalone sizes

Linux x86_64, rustc 1.99.1 (`58cf8f9c3`, LLVM 23.1.1), Cargo 1.99.1.
All artifacts use release optimization level 3, retain symbols, target
`wasm32-unknown-unknown` and receive no Binaryen transform. Gzip level 9 uses
published `ic-host-artifacts` 0.69.1's bounded canonical encoder. Code/data facts
and exact callable exports use its Wasm inspector, not a local parser.

| Standalone-only build | Raw bytes | Code bytes | Gzip 9 bytes |
| --- | ---: | ---: | ---: |
| Captured pre-change, default release | 3,193,027 | 2,287,576 | 700,832 |
| Pre-cell source, former profile (no cross-crate LTO; 16 codegen units) | 3,167,902 | 2,275,042 | 693,247 |
| Pre-cell source, selected ThinLTO / one codegen unit | 2,600,593 | 2,005,334 | 662,316 |
| Current cell source / Storage 0.51.12, selected ThinLTO | 2,585,430 | 1,992,811 | 659,413 |

The selected product is **19.0% smaller raw and 5.9% smaller gzip** than the
captured baseline. The controlled pre-cell-source profile comparison alone saves
17.9% raw and 4.5% gzip. The footprint gain mostly comes from build settings.
Longer linking and less parallel code generation are the tradeoff; no runtime
speed, allocation, peak-heap or IC instruction improvement is claimed. Those
endpoint measurements remain unperformed. The retained headers avoid leaf-vector
decoding structurally; preparation and index writes have the costs noted above.

The captured baseline selected Memory/Storage 0.51.10. The pre-cell products select
0.51.11; the published `src/` trees are byte-identical and their manifests change
only versions/dependency selections. The two pre-cell-source profile builds use
exactly the same 0.51.11 graph and standalone-only feature selection. Earlier
combined standalone/probe builds are retained separately and are not presented
as production feature-selection evidence. Platform 0.7.0 and Wire 0.7.18 remain;
Host/Build/Testkit 0.69.1 are native tooling. The final cell product selects
Memory/Storage 0.51.12, required by its new collection API. Package versions still say 0.51.0
until maintainer release preparation; these are unreleased-source measurements.

Selected standalone SHA-256:
`a54edcafe0673fd381e596c0a59dfa81903355ff6a75cd5e06bda37615e68646`.
Gzip SHA-256:
`524235f0e36df4c34262aff77bed43992eee960e6831ad07cda5cc8001853c34`.
Baseline SHA-256:
`72d04db717862cc0a6132c424c501b0e1812fc40267eaf22f2bd8e952bacde65`.

## Retained boundary and validation

The cell step replaces the two actual singleton maps; funding’s map retains
its distinct receipt-index responsibility. Configuration memory and gateway memory
keep their existing grant keys, and the fifteen-grant inventory is unchanged.
All owned schemas remain unversioned or 1. The obsolete configuration-row-count
error becomes an invalid-cell-record refusal; no row counting remains.

Cell qualification is retained in `/tmp/blob-cell052-20261027.qOTAMW0S/`: the
official unyanked registry row, selected locks, before/after artifacts, 335 native
core cases, strict core/caller Clippy, Rust 1.91 native/Wasm checks and sixteen
focused local artifact/PocketIC cases pass. Those cases cover real sync callbacks,
late replies, rollback, restore fences, provider callback authority and a populated
pre-cell image’s rejection with byte-identical retained obligations. Empty/foreign
configuration, unsupported cell headers and codec corruption refuse without repair
or stranded leases. Published Storage owns the generic loader matrix; Blob tests
its actual owner integration. The additional size reduction is 15,163 raw bytes
and 2,903 gzip bytes, with unchanged build settings. No IC instruction or heap
measurement was added.

Focused native core, contract/golden, CLI and installed PocketIC cases cover
original replay, malformed/scope refusals, two-part preparation rollback,
candidate-index corruption/rollback, descriptor consumers, certificate authority,
provider callbacks, restoration fences and real pre-cut image refusal. Strict
Clippy, Rust 1.91 native/Wasm compilation, boundary/deployment exclusions and
browser TypeScript schema checks also pass. The browser check covers 27 methods
and 137 exported frozen frames; the native corpus additionally includes lifecycle
vectors. This does not qualify native macOS, production capacity, deployed Caffeine
or Canic's application. Full repository CI was not run.

Exact commands, failures/retries, compiler inputs, initial/final patches and locks,
raw artifacts and metric logs are retained in
`/tmp/blob-simplify052-20261027.wmcqnujw/`. The cell step’s inputs, source diff and final artifacts stay in the additional
evidence directory above. The final profile command is:

```sh
cargo build --offline --locked --release --target wasm32-unknown-unknown \
  -p ic-blob-storage-canister --lib
```

The former-profile comparison adds `CARGO_PROFILE_RELEASE_LTO=false` and
`CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16`. Probes are built separately, preserving
the standalone product's exclusion of test-support features. Initial failures
include implementation fixture fallout, sandbox loopback denial and missing
consumer-fixture selection; corrected attempts are retained. An incoming lock
update interrupted a locked contract check; subsequent checks use the preserved
0.51.11 graph. None of those failed attempts is counted as passing evidence.
