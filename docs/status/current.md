# Current status — 2026-11-07

## Contracts and Client delivered; next Engine candidate

Contracts **0.73.1** is published from `83b9a9610de66c61909faddd74ff106bf86e399b`
(`contracts-v0.73.1`, batch `batch-E3If_blf`); Client **0.74.0** is published from
`441e466a89390dc69252f826795b3da31248f4ed` (`client-v0.74.0`, batch `batch-FVPHuT6k`).
Independent remote/default-branch/tag and non-yanked registry archive/source checks
pass. Readback records are retained in `target/blob-upstream-delivery-20261107/`.
Both captured full gates passed: 644 native tests, 82 Client installed cases and
162 Engine installed cases, strict Clippy, boundaries, docs, tool/release fixtures,
Wasm and actual archive verification. This is Linux/Rust 1.99.1 evidence, not native
macOS or live Caffeine/browser qualification.

The normal checkout is reconciled to Client delivery. A concurrent lock update was
preserved durably on `preserve/upstream-after-client-capture-20261107` (`4e540cf5`)
and reapplied here. It selects Wire/Derive 0.17.9, Storage 0.68.5, Generic Array
0.14.9 and Crypto Common 0.1.6. Registry releases/checksums and actual source were
reviewed: Wire's codec source is unchanged; Storage combines cell-header writes
without changing layout. This is the next Engine 0.74.4 candidate, not a change
to either completed captured batch or evidence reused across different graphs.
Its full required gate and publication remain pending. No live canister replacement
or paid provider effect is selected. The initial plan/evidence follows below.

## Pending upstream adoption and delegated delivery

The maintainer authorized adoption and publication for this repository. Select
published Build 0.78.0, Wire/Derive 0.17.8, Host Artifacts/FS/HTTP/Process 0.75.0,
Host Tools 0.84.0, Platform 0.24.0 and Testkit 0.91.0; Types 0.18.2 and Runtime
0.19.5 stay canonical. Registry metadata, Cargo.lock and cached crate archive
checksums match for all twelve selected packages. Released APIs were inspected:
Build's retired compiler/snapshot/progress capabilities are unused here; the
explicit gateway decoder handles Platform's retired shortcut. Native-only
Testkit retains Platform 0.23.6; the deployed normal/build graph selects 0.24 and
excludes Testkit/Host/Agent/parser/schema/test-support dependencies.

Shared Tooling 0.31.5 is exported canonically from independently verified remote
main `b2026a118e38de4af1555c527af327cb604eacdf`, with all 171 selected files
verified against that committed source. No sibling source was changed. The review
and proof records are under `target/blob-upstream-delivery-20261107/`.

The affected release plan is Contracts 0.73.1, then Client 0.74.0, then Engine
0.74.4, each on main/origin with crates.io publication. Client requires a minor
because the optional native DownloadError exposes Host FS's changed public
DirectoryCreationError/NamedWriteError identities. Contracts and Engine expose
no changed Platform wrapper identity; canonical Principal, Candid, Runtime and
Storage identities and product bytes/semantics remain unchanged, so they take
patches. The current Shared runner does not represent mixed increments in one
batch. Use separate captured group batches in supplier order, preserving each
batch's source, intents, failure logs, qualification and publication readback;
do not override the runner's saved kind or use mutable normal-checkout release
hooks. Reconcile each verified delivery before capturing the next group.

Focused archive-verifier Clippy, formatting, snapshot/source/dependency boundaries
and deployment exclusions pass on the selected graph. Full required delivery
gates run inside the captured checkouts. Delivery has not started yet. The pending
cleanup below is included; no live canister replacement, paid provider effect,
retirement or destruction of old provider/release evidence is selected.

## Pending Engine 0.74.4 — fixture consolidation and explicit gateway decoding (#119)

The gateway-source fixture uses Platform's `decode_one_with_config` with explicit
`DecoderConfig::new()`, preserving upstream unit/extra-value behavior, quotas,
diagnostics and raw transport error mapping. #119's seven-case actual callback
regression now lives in `tests/pocketic/tests/gateway_source/deletion_reply/mod.rs`
and runs under the provider target. Native Testkit `reply-fixtures` supplies its
bounded raw receiver; no direct Candid dependency or compatibility decoder is added.

The retired read experiment and parallel funding sender are removed. Production
Storage owns outgoing funding budgets, intents, accounting, credit and restoration;
the bounded Cashier receiver and unique receiver recovery coverage remain. Original
focused proof/failures remain under `target/issue119-20261107/` and
`target/blob-cleanup-*.log`; the earlier 20 native / 83 installed passing tests
precede the further consolidation below and are not its final qualification.

### One service qualification owner

The Authority and Admission probe packages are removed. The workspace has eleven
members and four probes, retaining the existing physical layout. Cargo catalogs,
Engine release groups, Make preparation/test partitions and dependency exclusions
select that current roster. Historical release receipts and provider evidence are
unchanged and still refer to their original packages/builds; Shared's exported
stack document remains its reviewed snapshot.

The Storage fixture owns the sole service configuration, memory runtime, allocation
and production stores. Installation supplies the complete passive ServiceResourceInput,
so workload limits have one source. The four previous admission workloads retain
their exact capacities. A 16 KiB private measurement command and 32-entry ephemeral
profile window exercise the same uploads owner; descriptor/discovery/capacity reads
reuse the existing Storage/public endpoints. There is no second service owner or
per-operation compatibility grammar. Admission profiles now reopen under the
production inspection fence instead of enforcing the old fixture's no-upgrade rule.

Coverage disposition:

- Current tenant/uploader/controller checks, manifest/exposure retries, reference
  receipts, physical/billing obligations and restore fences stay in Storage and
  standalone scenarios. The alternate heap catalogs and replay-built authority
  archive are gone; no product obligation records are touched.
- Mandatory verified-before-certificate serving, persisted verifier sessions and
  their checkpoint recovery belonged to the retired fixture model. Contracts keeps
  verifier/checkpoint mathematics; the installed full/final-leaf, metadata and wrong
  digest witness is retained by the content target using Storage, without durable
  verifier state or a registration gate.
- The private balance cache/journal, revision/preview APIs and status projections
  had no counterpart in the current production workflow. Their fixture-only tests
  retire; actual account reads and provider decoder failures remain covered by
  standalone account and independent source consumers. Shared billing/diagnosis
  policy tests remain with the core.
- Provider source readers/journal recovery now run independently in the provider
  target. Production gateway transport retains held replies, stale callbacks,
  revocation/cancellation, failed upgrades and restored captured-source fences.
  #119 is preserved. None of these substitutes qualifies deployed Caffeine.

The accepted changes cut private fixture formats and commands only. No public
product endpoint, product stable format, installation retirement or paid/provider
operation changes. Old runs retain their original tools; no dual reader, reset or
automatic migration is introduced. Pending dependency edits are preserved. No
sibling or provider effects were part of that cleanup pass; the delegated library
delivery above now includes the reviewed changes.

Linux/Rust 1.99.1 qualification passes on the fixed preserved graph: strict
all-target/all-feature Clippy for Storage/Gateway probes and all PocketIC targets;
release Wasm builds of all four remaining probes and standalone; native CLI build;
8 Contracts verifier/checkpoint tests, 5 core certificate tests, 24 billing-policy
tests and 7 protocol tests. The migrated targets pass all 29 admission cases
(including all four resource workloads), the 2 content-target cases and all 12
provider-target cases. Production-path selections pass 16 gateway, 10 lifecycle,
7 exposure, 7 manifest, 8 tenant, 4 standalone account and 7 standalone gateway
cases: 102 integration-target passes in total. One Chromium opt-in case remains
ignored; these local substitutes do not qualify the deployed provider.

Formatting, source/dependency boundaries, deployment exclusions, documentation
links and release-plan/test-partition regressions pass. Input fingerprints stayed
unchanged throughout runtime qualification. Logs, the exact graph fingerprint and
Shared's LOC JSON remain in `target/blob-probe-consolidation/`. The current count
is 100,470 Rust code lines (28,581 application/library; 71,020 tests; 869 examples/
tooling), versus 110,263 at the start of this pass, with zero unclassified files.
Full release CI, native macOS, opt-in browser execution and live acceptance were
not rerun. Earlier proof and all unsuccessful attempts remain retained separately.

## Delivered Client 0.73.2 / Engine 0.74.3

Shared 0.31.1 batch **batch-eSq3MKgb** completed from committed source
`725283169a17bfa7fab682c486d570e0197288d0`. Client **0.73.2** is published at
`b18779f2a88ffdacf89a8af52f0ef6c0ccd41e0d` (`client-v0.73.2`); Engine **0.74.3**
is published at `4370052d60a8480ebf981216a7f22302da612f3e` (`engine-v0.74.3`).
Both tags and default main were read back independently. Non-yanked registry
archives match checksums and their exact Git provenance:

- Client: `0c73cdea170be423fdd9a7f31d6e9b22eb61a3bfd25be75ed834b549b31b0fff`.
- Engine: `cbad121c5a4b62ef70f1c6d8a569b45641b0961cc1f0e456971781a113797e0c`.

Both 21-stage Linux/Rust 1.99.1 gates passed. The final-version candidate executed
644 native tests, 82 Client and 270 Engine installed cases. Engine re-admitted all
eight eligible stage proofs; actual group metadata, policies, tool admission,
source-binding installation sweeps and package verification stayed fresh. Both
publications admitted retained Clippy qualification. Native macOS, opt-in browser
and live-provider acceptance were not rerun. Contracts remains **0.73.0**.
[#118 is closed](https://github.com/dragginzgame/ic-blob-storage/issues/118#issuecomment-6705332626)
on delivered implementation and these consumer checks. #5, #6, #31 and #34 remain
open for their actual application/provider acceptance; no service replacement,
payment, provider deletion or cleanup occurred.

The managed checkout is `target/release-frozen/batch-eSq3MKgb`; durable recovery
and intent remain under `.git/release-state/frozen/batch-eSq3MKgb`. Keep its
original proofs, builds and publication records. The normal checkout was
reconciled with the canonical batch command. Retain
`target/blob-delivery0732-0743.def5087_/` for commands, full release logs,
independent archive/registry/tag readback and deferred-lock custody checks.
Client gate: `target/blob-release-validation.iC3TjI/validation.BPBHIw`; Engine
gate: `target/blob-release-validation.GSiDZJ/validation.RBHQqB` in that managed checkout.
The [hosted workflow for 4370052d](https://github.com/dragginzgame/ic-blob-storage/actions/runs/42361801887)
was in progress at readback; its asynchronous matrix is separate from the
completed local release gates. No hosted pass is inferred from publication.

A concurrent normal-checkout update selected **Platform 0.23.6**, crypto-common
0.1.7 and generic-array 0.14.7. It did not alter the captured validation or
archives, which retain Platform 0.23.5. The update was committed only for custody
on local `preserve/blob-platform0236-20261107` at `cbf3a893`, then reapplied
uncommitted after reconciliation. Its supplier records match the preserved lock
exactly; local package versions retain the delivered releases. This deferred
selection still needs review/qualification and belongs to the next batch. The
following implementation records describe work before this completed delivery;
their earlier no-delivery statements retain that historical scope.

## Delivered upstream implementation record — Client 0.73.2 / Engine 0.74.3

The captured registry-verified graph selects **Build 0.77.0 / Platform 0.23.5 / Storage
0.68.3 / Testkit 0.90.0 / Wire-Derive 0.17.6**. Host Foundation 0.74.4, Tools
0.83.0, Runtime 0.19.5 and Types 0.18.2 are already current. The Build catalog
requirement advances to 0.77; compatible patch selections remain in the shared
lock. Preserve the incoming SmallVec 1.16.5 selection. No package versions were
prepared, and no stage, commit, release, push or provider effect is authorized.

Published Build 0.77's source-evidence, Cargo qualification and release-editor
sources match 0.76 byte-for-byte. Blob has no callers of its retired cache-timing
getters, so no wrapper/caller migration is needed. Platform facade production
source matches the prior qualified patch. Incoming Runtime 0.19.5 and Storage
0.68.3 were already locked when Testkit adoption began. Runtime consolidates
IC system-byte copies and preserves protected-task order; Storage batches small
fixed-width B-tree key reads without changing persisted images. Their focused
qualification and Testkit 0.90's constructor-budget migration are recorded below. The new
Wire field adapters remain unselected: no owned DTO, saved representation,
authentication, decoder limit or exact retry bytes change.

The canonical exporter adopts **Shared 0.31.1** at
`5efbda2ab81ad84e30f490f3137d8ef083c303d2` (171 files), fixing Bash 3.2 parsing
of the checksum NUL reader. VERSION/changelog metadata declare 0.31.1 despite the
commit title `0.31.2`. The runner, batch schema, release defaults and retained
batch custody are unchanged. `AGENTS.md` selects the same policy.

**Earlier Build 0.76 qualification:** tool preparation and offline admission
passed for that editor and Testkit 0.89.4 CLI; PocketIC remains 16.1.0. Linux/Rust 1.99.1 focused
checks passed: strict CLI all-target/all-feature Clippy, 129 Contracts codec and
malformed-input tests, standalone release Wasm, the exact CLI-carrier installation
and snapshot-rollback/fencing cases, dependency/source/deployment boundaries,
metadata-editor render/rollback and checksum evidence. That run
verified all 13 IC selections against official non-yanked registry checksums.
Cargo/lock/compiler/snapshot inputs stayed fixed throughout that qualification. No workspace/full release gate ran.
Retain official registry metadata and focused logs under
`target/upstream-adoption-20261107/`; snapshot impact/refresh is in
`target/shared-tooling-0311/`. Native macOS, new browser/II and live-provider
qualification are not claimed.

The current Build **0.77.0** publication is independently verified at registry
checksum `41883c4420ed9c70d4a0be08718141675001315f1f1964e13169b253bbb3ee01`,
VCS `9d87394919b812cb6de55a9abd6dd2fd026c9070`. Only Build changed in this
follow-up's lock update; all existing supplier selections and cleanup work remain.
The canonical installer reused the admitted published 0.77 editor payload without
removing earlier tool generations. Strict all-target/all-feature CLI Clippy,
offline editor admission, dependency/source boundaries and deployment exclusions
pass. Actual Client metadata render/rollback qualification passed using the real
0.77 editor over all 13 members, preserving supplier entries and unselected groups.
Documentation references and whitespace checks also pass. No full release gate ran.
Retain `target/build077-adoption-20261107/` for registry selection, module hashes,
lock comparison and focused logs. Earlier installed-IC proofs retain their original
Build graph; no new service/browser/provider or native macOS qualification is claimed.
No package versions, stage, commit, push, release or provider effect occurred.

## Delivered Testkit 0.90 adoption — issue #118

The official registry now supplies Testkit **0.90.0**, checksum
`aeeb9dd0920b5a4343aed61f2cb7c0266488823dab5f7b19e0b632b54b7d8f5a`, VCS
`c03161b9f1395c343756fda6b8721e957798c3df`. Platform **0.23.5** is independently
verified as the latest published facade at batch selection and remains in the
captured graph; the later 0.23.6 update is deferred above. No sibling override
or source edit is used. The root Testkit catalog/lock selects 0.90; all 20 fixture
builders explicitly receive Blob's `support::REQUEST_TIMEOUT` (300 seconds),
preserving the published 0.89 default. Startup remains 30 seconds, with independent
signed-CLI and scenario budgets. Existing runtime/server custody and operation
identity are unchanged. No local timeout framework or compatibility alias was added.
The inventory found no direct attachment/listing calls, unbounded setter or
consumers of the other retired Testkit 0.90 API families. Three unused MIME packages
are removed through Testkit's HTTP feature reduction; other suppliers stay selected.
Focused Linux / Rust 1.99.1 qualification passes: strict all-target/all-feature
native Testkit-consumer Clippy, canonical 0.90 CLI/server setup and offline
admission, actual probe/standalone Wasm and native CLI preparation, 129 Contracts
and 280 Engine unit tests, all 77 non-browser standalone cases, and five selected
storage cases. The storage selections cover signed reference/funding history,
restore fences, genuine signed-ingress rejection and saved-request recovery, and
accepted funding whose callback traps. Five opt-in Chromium cases were ignored
(four standalone, one storage); no browser/II result is claimed. Native runtime
custody, same-release recovery and uncertainty remain covered on this graph.
Dependency/source boundaries, pins, deployed dependency exclusions, the unchanged
0.77 editor's refreshed offline admission, formatting and documentation links pass.
Cargo/lock/compiler/snapshot fingerprints stayed fixed through qualification.

[#118](https://github.com/dragginzgame/ic-blob-storage/issues/118) is closed after
verified delivery to default main and publication, as recorded above. Retain
source inventory, registry/lock selections, exact commands and focused logs under
`target/testkit090-adoption-20261107/`. Earlier tests keep their original supplier
graphs; the completed captured release gates are separate evidence above.

## Delivered consumer tooling roster cleanup

`shared-tooling-tests` no longer invokes Shared's generic
`test-tool-downloads.sh` download/retry/checksum matrix. Shared owns that producer
proof; the snapshot-owned script and integrity checks remain intact. Blob keeps
the actual Testkit/Make admission and failure-routing fixture, tooling evidence
archive/retention fixture, compiler, logging, completion and release-validation
checks. Focused Testkit/release-editor forwarding and ordered failure-propagation
checks pass with substitute tools; actual tooling evidence selection, archive
round trips and failure retention pass. Snapshot integrity passes for all 171
files. Logs remain in `target/tooling-roster-20261107/`. This is a target-roster
change, not a tool-admission policy change or a measured speed-up; the full
tooling/release suites were not rerun. No staging, commit or delivery occurred.

## Delivered native CLI simplification

The same Client 0.73.2 / Engine 0.74.3 batch removes the native CLI's intermediate
`Command` enum and separate execution match. The typed Clap `Invocation` now owns
one exhaustive dispatch. Command-specific gateway/release admission still precedes
connection trust validation, runtime creation and handler I/O. The synchronous
offline commands still bypass runtime creation. Existing handler inputs, signed
journals, retry rules, output shapes and exit classifications are unchanged.
Parser tests inspect the actual Clap values; projection-only tests supply explicit
passive options instead of depending on the retired command representation.

The installed history, local-body verification and funding-absence cases share
identity/root-file setup, numeric-loopback startup and argument editing through
the existing authenticated-CLI helper. Scenarios still choose every authority,
installation profile, budget and assertion and own their stop/restart sequence.
No probe, journal or publication/session controller was removed. Focused Linux /
Rust 1.99.1 validation passed: strict all-target/all-feature CLI Clippy, standalone
target Clippy, 122 CLI and seven probe-tool unit tests, 77 installed standalone
cases, formatting and documentation references. The four opt-in Chromium cases
were not run. The initial unit run passed 119 CLI cases; three local-socket cases
were refused by the sandbox and passed on focused retries with local networking.
Both failed and passing logs remain in `target/cli-simplification-20261107/`.
Canonical offline Testkit admission and fixture/CLI preparation passed; dependency,
compiler and snapshot inputs remained fixed. No full workspace/release gate,
native macOS or new live-provider acceptance is claimed. Nothing was staged,
committed, released or pushed; no sibling source or live installation changed.

The production follow-up shares input preparation and execution for the five
native upload commands in the existing connection adapter. Each invocation still
selects its original operation and supplies a journal directory only for mutations.
The CLI-only `upload_setup::run` returns its report directly instead of forcing
five callers to discard a richer result; publication still consumes the unchanged
`execute` / `SetupOutcome` path for typed observations and recovery decisions.
No file format, authority check, decoder bound, effect order or retry rule changed.
Focused Linux / Rust 1.99.1 checks passed: strict all-target/all-feature CLI Clippy,
four upload/request tests, four typed-grammar tests, native CLI build and the exact
installed upload-setup journey covering all five commands, lost/pending replies,
revocation and restore fences. Formatting and documentation links pass. Commands
and logs remain under `target/cli-upload-dispatch-20261107/`. Offline managed
Testkit admission passed; unchanged standalone Wasm and dependency/compiler/
snapshot inputs were retained. No full gate, browser, macOS or live-provider
qualification was rerun; nothing was staged, committed or delivered.

The file-input follow-up consolidates bounded nonempty consumption of an already
admitted descriptor for ordinary input files, frozen publication metadata and
recovery packets. Published Host FS still owns regular-file, size/growth and
allocation checks. Callers retain their existing path-following/confinement rules,
held directory handles and opening-error mapping; the separate no-follow reader
is unchanged. No path is reopened, and no new mode, dependency or saved format was
introduced. Frozen-root coverage additionally checks empty and zero-budget reads
after namespace replacement. Strict CLI Clippy, 31 focused local filesystem/
upload-input/publication-recovery tests, formatting and documentation links pass
on Linux/Rust 1.99.1. Logs/commands remain in `target/cli-opened-reads-20261107/`;
dependency/compiler/snapshot inputs stayed fixed. No installed-IC/full gate or
live-provider checks were rerun. Incoming PocketIC support/certificate-test edits
were left intact; nothing was staged or delivered.

### Further fixture consolidation

The same Engine batch extends those existing helpers through native reference,
publication, account, gateway and upload-setup fixtures. The separate reference
argument editor and four identical private editors are removed; callers use
`authenticated_cli::change` directly. Loopback wrappers which only started the
simulator are gone. Ordinary identity/root fixtures use the original shared
writer; create-new evidence writers and distinct tenant/uploader identities stay
with their scenario owners. The publication-map URL normalization remains local:
PocketIC's already-live URL getter may return localhost, which that scenario
rewrites under its existing literal-loopback policy. No provider callback,
publication controller, paid operation or product code changed in this follow-up.
Two now-unneeded length-lint expectations were deleted. Focused Linux / Rust
1.99.1 checks passed: strict standalone/storage Clippy, 77 installed standalone
cases, the exact storage reference-history/recovery case and exact funding-history/
recovery case, formatting and documentation links. The four opt-in Chromium cases
were compiled but not run. Offline managed Testkit admission preceded installed
checks; the previously prepared CLI and Wasm selection is unchanged. Cargo/lock,
compiler and snapshot hashes remained fixed. Raw checks and exact commands remain
under `target/cli-fixture-consolidation-20261107/`, including initial compiler/lint
refusals and their corrections. Earlier CLI implementation proofs keep their
original source scope; unchanged product checks are reused. No full gate, native
macOS or live-provider qualification was added, and nothing was staged or delivered.

## Delivered Engine 0.74.2 — Shared 0.31 batch

Engine **0.74.2** is pushed and published. The annotated `engine-v0.74.2` tag
resolves to
[`185280a0`](https://github.com/dragginzgame/ic-blob-storage/commit/185280a03f97aa50e94bc8b89122bdc99656e009),
from captured source `875c950c`. Independent readback verifies a non-yanked
registry archive, checksum
`86e277872acc8b973b05f60fe92f73645be3fbb62925d19ad1a33802ee74e53e`,
exact VCS identity and release receipt/tag. Only `ic-blob-storage` was published;
Contracts **0.73.0** and Client **0.73.1** remain unchanged.

The real **21-stage Linux/Rust 1.99.1 gate passed**, including **644 native**
and **352 installed** cases (82 Client / 270 Engine), actual-source
installation/version and CLI smoke checks, strict Clippy, dependency/source
boundaries, Rustdoc, Wasm and all three actual library archives. Publication
admitted retained final-candidate Clippy; it did not run the full lint again.
The [Engine receipt](../../releases/engine/release.json) owns stage timings,
source hashes and retained proof identities. Native macOS, new II/browser,
Canic and new live-provider qualification were not rerun.
Hosted [example installation](https://github.com/dragginzgame/ic-blob-storage/actions/runs/42267789661)
and [release/formatting tooling](https://github.com/dragginzgame/ic-blob-storage/actions/runs/42267789721)
were in progress at readback; they are separate from the completed local gate.

Shared **0.31.0** is captured at
`74313941af2fce4399bec0e6909ca6229673c3f8` (171 verified files), including the
integrated checksum fixes. The canonical exporter replaced the retired runner
and guide. Make, fixtures and current instructions use `RELEASE_GROUPS`,
`RELEASE_PUBLISH_TARGET`, `RELEASE_SETUP_TARGET` and `RELEASE_BATCH_ACTIVE`;
retired settings refuse without aliases. Grouped increments capture by default.
Publication remains explicit in that same batch, not the normal checkout.
The actual command used `make release-patch RELEASE_GROUP=engine
RELEASE_PUBLISH_TARGET=publish` with the qualified short scratch directory.
See [the release guide](../releasing.md).

Batch **`batch-UHl3ez7W`** is complete with a schema-2 journal and recorded
publication commit `185280a0`. Its managed checkout remains
`target/release-frozen/batch-UHl3ez7W`; durable Git, intent, standard release
plan and source pin remain under the shared custody contract. Do not repeat its
publication or retire needed proofs. The normal checkout was clean and the
canonical `release-batch-reconcile` command fast-forwarded it without stash,
reset or conflict resolution. Its missing Testkit 0.89.3 selection was then
prepared through the canonical installer using the existing admitted payload;
offline admission selects PocketIC **16.1.0**. No later normal-checkout edits
were found to reapply.

Retain `target/blob-delivery0742.sL0Ssd/` for prepublication observation,
release, independent archive/registry/tag readback, reconciliation and tool
admission logs. The full gate is under
`target/release-frozen/batch-UHl3ez7W/target/blob-release-validation.NhGcLq/validation.5NswWJ`;
its stage cache/builds and `target/publication.5RhHfw` remain in that captured
checkout. No live service, provider payment, deletion or artifact cleanup occurred.

The qualified lock includes **Storage 0.68.1 / Runtime-Macros-Guard 0.19.4 /
Wire-Derive 0.17.5 / Testkit 0.89.3** and the pending TOML patches. Build 0.75.0,
Platform 0.23.4, Types 0.18.2, Foundation 0.74.4 and Host Tools 0.83.0 remain.
Existing library contracts and collection images are compatible with 0.74.1;
the dependency ranges were not raised merely to publish the tooling batch.
Registry consumers still resolve their own compatible graphs. Blob's application
and storage schemas are unchanged; schema 2 belongs only to release tooling.

The pre-0.31 **`batch-QfoyYuxC`** remains complete and integrated, with its
schema-1 journal, original 0.30.8 captured checkout, Git store, source pin and
proofs unchanged. Current tools reject it; use its original reviewed tools for
historical recovery. No journal conversion, identity inference or old-batch
retirement occurred. Published historical group receipts remain immutable.
Earlier focused 0.30.9/0.30.10/0.31 consumer qualification remains in
`target/shared-tooling-0309/`, `target/shared-tooling-0310/` and
`target/shared-tooling-031/` with its original substitute/runtime scope.

## Delivered Engine 0.74.1 — frozen batch

Engine **0.74.1** is pushed and published. The annotated `engine-v0.74.1` tag
resolves to
[`3a4ff2d0`](https://github.com/dragginzgame/ic-blob-storage/commit/3a4ff2d0b83bfc4ec040a6c5b80bfa122c8603dc).
Independent registry readback admits the non-yanked archive, checksum
`32eeaa36b17e16a4ab0816ae3fe9ea8ca8220f28598179a73a1ba8f1fd3be04e`
and exact release VCS identity. Only `ic-blob-storage` was published; Contracts
**0.73.0** and Client **0.73.1** stay unchanged. The
[historical Engine receipt](https://github.com/dragginzgame/ic-blob-storage/blob/3a4ff2d0b83bfc4ec040a6c5b80bfa122c8603dc/releases/engine/release.json) records all 21 passing stages.
No live service, provider payment, deletion or artifact cleanup occurred.

The maintainer selected Shared's frozen method after the original normal gate
from `c8309234` stopped on an incoming Platform 0.23.3 → 0.23.4 lock change.
That attempt created no release plan/tag/push or upload. Retain its original
`target/blob-delivery0741.X3oeZj/release.log` and
`target/blob-release-validation.CtNBeF/validation.ZX6tmx`; its adapter regression
passed before source admission correctly refused the changed inputs.

The successful batch **`batch-QfoyYuxC`** captured clean source `7f7c0d5b` once.
Its delivery checkout is `target/release-frozen/batch-QfoyYuxC`; durable Git,
complete intent and standard release plan remain under
`.git/release-state/frozen/batch-QfoyYuxC/`. Use the direct CLI in
[the release guide](../releasing.md), with an explicit supplier-first roster,
ordinary shell context and short source-independent scratch. The child rebuilds
its own source/tool routes; never publish a stage-only prospective candidate.
Later normal-checkout lock edits did not change the captured source or archives.
The completed batch must not be uploaded again.

The delivered snapshot selected **Shared 0.30.8** at
`dc9df707b3b1e616973494eab961c1c214d12260` (171 files), including generated-ID
resume support. Consumer hook qualification passed for all three groups, deferred
invalid lock isolation, uncertain publication reconciliation and idempotent
completed resume. Those tests use local Git and compiler/upload substitutes;
the actual release independently passed locked builds, installed behavior and
all package archive checks. Retain `target/frozen-release-adoption/` for export,
fixture, delivery and authenticated readback logs. Published archive/readback
artifacts are under the captured checkout's `target/publication.hMnJBJ` and
`target/publication.jgzyTI`.

The **21-stage Linux/Rust 1.99.1 gate** passed with **644 native tests** and
**352 installed tests** (82 Client / 270 Engine), plus the actual-source standalone
CLI carrier smoke check. Retain
`target/release-frozen/batch-QfoyYuxC/target/blob-release-validation.8Q9Bk1/validation.9M4tL5`
and that checkout's stage-cache records/builds. Publication admitted retained
final-candidate Clippy rather than running full lint again. All three libraries'
actual archives remain checked; no narrower archive proof or elapsed-time saving
is claimed. Native macOS, new II/browser, Canic and new live-provider acceptance
were not rerun. Hosted [example installation](https://github.com/dragginzgame/ic-blob-storage/actions/runs/42235651929)
and [release/formatting tooling](https://github.com/dragginzgame/ic-blob-storage/actions/runs/42235651948)
were still in progress at readback; these are separate from the completed local gate.

The released graph includes **Testkit 0.89.1 / Platform 0.23.4**, Runtime/Macros/
Guard 0.19.3, Types 0.18.2, Wire/Derive 0.17.4, flate2 1.1.11 and cc 1.11.1.
Build 0.75.0, Storage 0.68.0, Foundation 0.74.4 and Host Tools 0.83.0 remain.
Twelve unused logging/transitive packages disappeared; no replacement subscriber
or compatibility wrapper was added. PocketIC remains **16.1.0**.

Shared CBOR inventory/source checks replace duplicate local codec rules; the
current lexical scan covers 836 Rust files with 41 external Candid declarations
and no CBOR source allowances. Strict all-target/all-feature Clippy replaces the
redundant automatic native check. Manual `make check`, distinct feature/Wasm/
Rustdoc/installed/archive checks and historical receipt admission remain. Eight
stages are eligible for source-bound reuse, including qualified Linux group
scope and exact managed PocketIC selection; parallel Make, custom targets and
other hosts stay guarded. Reuse refusals are printed and retained. Earlier focused
scope/tamper/failure qualification remains in `target/blob-stage-reuse.HKeoVT/`,
including its original relative-TMPDIR refusal and corrected diagnostic run.

The normal branch was safely fast-forwarded to the delivered commit. Its later
**Cargo.lock changes remain uncommitted and deferred**, not part of this release:
serde_spanned 1.1.3, TOML 1.2.1, toml_datetime 1.1.3 and toml_edit 0.25.19.
The exact pre-reconciliation bytes/patch are retained with the durable batch.
A structured comparison verified all 423 lock records were preserved, changing
only Engine member versions for the delivered release. These incoming dependency
updates have not been qualified by the frozen gate; do not attribute its evidence
to them. Preserve the deferred lock and every retained provider obligation below.

## Delivered Client 0.73.1 and Engine 0.74.0

Both changed groups are pushed and published. Registry readbacks verify non-yanked
versions, archive checksums and VCS identities against the exact release tags.
Contracts remains 0.73.0; no unaffected group was republished.

| Group | Release commit | Receipt |
| --- | --- | --- |
| Contracts 0.73.0 | [`2ae0ba1b`](https://github.com/dragginzgame/ic-blob-storage/commit/2ae0ba1b5c60576faa334b9cb8d65c5b2ea8d5ae) | [Contracts](../../releases/contracts/release.json) |
| Client 0.73.1 | [`2e55591e`](https://github.com/dragginzgame/ic-blob-storage/commit/2e55591eeb7af7368761fa7685e6c2d2e7a5e72d) | [Client](../../releases/client/release.json) |
| Engine 0.74.0 | [`a7c6b374`](https://github.com/dragginzgame/ic-blob-storage/commit/a7c6b37402fd1f71a103063bd4c1acecc6332909) | [Historical Engine receipt](https://github.com/dragginzgame/ic-blob-storage/blob/a7c6b37402fd1f71a103063bd4c1acecc6332909/releases/engine/release.json) |

The graph selects **Build 0.75.0 / Storage 0.68.0 / Testkit 0.88.0**, Platform
0.23.2, Wire/Derive 0.17.3, Runtime/guard 0.19.2 and Types 0.18.1. Foundation
0.74.4/Tools 0.83.0 remain; the incoming cc 1.11.1 patch was included in the gate.
Nine unused transitive packages are gone. Shared Tooling remains **0.30.4** at
`2aa41c7394300302fe802b69b31b855bdc89b671`; Rust **1.99.1** is minimum and pinned.
Storage changes public identity, collection images and failure behavior: retire
whole old backing with original tools before fresh installation. Malformed stored
values trap; framing/byte-limit refusals remain typed. Propagate IC traps; native
mutation panics require discarding the runtime. No migration/reset/wrapper was added.

The complete 22-stage Linux gate passed with **644 native and 352 installed cases**
(82 Client / 270 Engine). Engine reused all nine eligible final-candidate stages;
actual-source identity, metadata, tools and archive checks remained fresh. Both
publications admitted retained Clippy. Hosted [CI 42201285250](https://github.com/dragginzgame/ic-blob-storage/actions/runs/42201285250)
was in progress at readback; its generated-client job passed. Native macOS, Canic
and new live-provider acceptance are not established. No live installation was
changed, and no provider payment or artifact cleanup occurred.

The first gate at `3a94dc10` failed only the old checkpoint corruption expectation.
The correction at `3f5faafb` covers retained copies and the owned decoder's refusal,
unchanged live state and original-byte recovery; no production checksum was added.
Retain `target/blob-release-validation.0tYTOP/validation.UOhp90` (failed),
`target/blob-release-validation.WK1eXo/validation.wNwYnP` (Client full gate),
`target/blob-release-validation.rgN3fH/validation.1wWF95` (Engine), and
`target/blob-delivery.Co3u1H/` (logs and `delivery-readback.json`). Registry archives
remain in `target/publication.6nhMT3` / `target/publication.51Se9T`; original proofs
remain under `target/release-stage-cache/`, with builds in `target/release-stage-build/`.
The frozen 0.73 qualification and closed #117 retain their scope in the
[previous delivery handoff](https://github.com/dragginzgame/ic-blob-storage/blob/6ff224ed8acb5ac846150e7f48cb59d3c6006105/docs/status/current.md).

## Completed bounded 0.73 provider trial — 2026-11-06

The selected real-Caffeine two-uploader run passed on new standalone service
**`j7h7p-gyaaa-aaaaa-qhxwa-cai`**. Both original certificates/four PUTs succeeded;
two independent downloads verified all **2048 original bytes**. Cross-uploader
assessments are denied. Exact module/configuration/release/controller and
anonymous-refusal readback pass; incoming Platform/Wire lock edits were not used.
[The summary](../evidence/caffeine-probes/deployed/2026-11-06-trial-v073-two-users-01/summary.json)
and [ledger](../evidence/caffeine-probes/README.md#new-073-two-user-provider-trial--2026-11-06)
own intents, failures, the preserved certified early capacity refusal and explicit
unchanged-operation re-evaluation after capacity/absence checks. No certificate/PUT
was replayed. Both references stay live, zero active/reserved bytes, logical/physical/liability
**2048**, owners unfenced; both lifetime object slots are occupied. No release,
delete, settlement or reset occurred. Maintain these new continuing obligations.

One 2 ICP conversion, exact 2.0001T creation, one 1.001T gross external deposit
(1.0008T credited), and one new expiring 5T nominal daily relationship are retained.
Conservative conversion-plus-allocation count is 6.5288T within the selected 8T
envelope and retained 100T total / 20T daily trial authority; no further refill.
The original link expires **2026-11-06 09:56:32 UTC**, without renewal or a claim
that expiry ends billing. Final payer/owner/gateway observations are time-bound.
Private custody `.git/private-operations/blob073-two-users-01/` retains frozen
tools/keys/profiles/original journals and an inventory of 610 files (164,174,592 bytes),
excluding private keys. No external backup; development key never exported, default anonymous.
The 0.69 and older trials below are untouched. #5/#31 still need real application
login/CSP/catalog acceptance; #6/#34 retain deletion/billing and lost-payment
evidence. The trial changed no implementation/dependency or package release.

## Remaining application/provider acceptance

Four issues remain open: [#5 publication/browser serving/integrity](https://github.com/dragginzgame/ic-blob-storage/issues/5),
[#6 reuse/capacity/deletion/billing](https://github.com/dragginzgame/ic-blob-storage/issues/6),
[#31 actual multi-user transfers](https://github.com/dragginzgame/ic-blob-storage/issues/31)
and [#34 original paid-operation credit/replay](https://github.com/dragginzgame/ic-blob-storage/issues/34).
Publication does not satisfy their live acceptance. Follow the
[application plan](../acceptance-plan.md#next-application-acceptance-run).
The earlier November 6 read-only Canic review found Engine 0.72.2 / Contracts
0.72.0 in three locks at base `e6a061999`, local-only fixtures with delegated-token
Auth disabled and no actual application binding. Reported older composition
results keep their own locks; their named raw output is unavailable here.
[Canic #622's delivered-family follow-up](https://github.com/dragginzgame/canic/issues/622#issuecomment-6682566587)
records Engine 0.74/Contracts 0.73 adoption, compatible Storage peers, both managed
shapes, five funding profiles and explicit generated-client qualification. Canic
owns its wrapper/application; no sibling source was edited or built here.

The standalone **0.69.0** trial at **`4h7vw-4qaaa-aaaau-ahieq-cai`** is unchanged.
Its [probe ledger](../evidence/caffeine-probes/README.md) owns immutable phase
records; [November 5 readback](../evidence/caffeine-probes/deployed/2026-11-05-trial-v069-gc-readback-01/summary.json)
shows first object `Live`, second `DeletionPending`, zero gateway acknowledgments,
and logical/physical/liability bytes **1024/2048/2048**. A possibly cached HEAD 200
proves neither origin deletion nor billing cessation. Both lifetime slots remain
occupied. User 1's certificate/tree PUT 403 stays held: no retry, replacement
certificate, reset or release. User 2 verified all 1024 original bytes; only its
reference was logically released with original operation 7 and receipt inspection,
without resend. That is one isolated test-key success, not two application users.

Retain **1.001T gross / 1.0006T credited**, original deposits/receipts and the 400M
fees. The approved 5T daily payer relationship **expired November 3, 17:48:30 UTC**;
no implicit renewal, refill or repeated uncertain paid effect is authorized.
Recorded payer/owner/gateway balances are time-bound, not fresh billing evidence.
No provider acknowledgment was fabricated and no physical settlement was invoked.
Private custody remains `.git/private-operations/blob069-trial-01/`, including
original user profiles/journals, frozen CLI/Wasm, `gc-release-01/` and
`gc-readback-20261105/`. Historical raw scratch `target/post069-provider/` is absent
in this checkout; resume from private copies, without claiming an external backup.
Keep the failed first root, released second root and older 0.6/0.7 obligations.
Cross-release changes require retirement with original tools before fresh whole
backing; no fallback reader, stale activation, automatic reset or financial retry.

Keep the native publication/session controller and browser integration per the
November 4 decision until an actual Canic application selects its publishing path.
Adapter non-use alone does not establish dead code. Canic owns its wrapper and
application acceptance; no sibling source was edited. Older preparation remains
in the [committed pre-delivery handoff](https://github.com/dragginzgame/ic-blob-storage/blob/4eb56823b75ea8dd140e81d675518722ff62f08f/docs/status/current.md).
