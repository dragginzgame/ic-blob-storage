# Publication regressions and Host 0.4.6 — pending Blob 0.17.2

Reviewed 2026-10-07 on released base
`7c41e3a90996157312aa40985a9861f4c03ca35e`. The user requested implementation
of open product issue work. Raw inputs/attempts remain under
`.tmp/product-issues-0172-01/`; earlier adoption records retain their original
graphs. This is a compatible patch, with workspace identity and receipt still
0.17.1. The first graph's lock stayed unchanged during its checks; an external
Testkit selection later changed to 0.21.2 and is separately qualified below.

## Implemented boundaries

Fresh `publish-inputs` preflight admitted equal provider roots with distinct
operation/object/first-reference IDs even though permanent root claims prevent
fresh reuse. The new regression fails against the old code and passes after
adding root uniqueness to the existing scoped bounded capacity owner. Admission
refuses before claiming an output directory or effects; reopening frozen inputs
uses the same guard. Equal raw bodies with different authoritative metadata roots
remain valid. No automatic deduplication, new retry owner or service schema is added.

Native downloads now persist `http-response-headers.json` beside the unchanged
status record, before status/body admission. Fifteen relevant parsed names retain
exact value bytes and duplicate occurrences, bounded to 32 occurrences and 8 KiB
of combined name/value bytes. An occurrence exceeding either limit is omitted
whole and `complete` becomes false. Completeness covers only selected names.
The actual loopback fixture verifies MIME/cache/CORS/duplicate Vary capture,
oversized-header partial capture with successful body verification, and existing
corruption/truncation/redirect/range/encoding refusals. HeaderMap unit checks cover
non-UTF8 values and both limits. Original metadata still owns the root, and the
status/attestation decoder remains unchanged. These observations do not establish
browser policy acceptance, certified HTTP or deployed provider guarantees.

The [operator guide](../operator-guide.md#overlapping-application-releases-and-lifetime-capacity)
now explicitly maps existing typed content states to original-operation recovery,
live exact retain or unsupported retired-content reintroduction. Lifetime roots,
references, receipts and physical/billing obligations remain retained. The
[Memory composition recipe](../dependencies.md#memory-composition) selects the
published Memory 0.31 contract and leaves Canic's three locks, lifecycle ownership
and managed execution with its owning repository. No named Rust function, method
or type is removed.

## Graph and provenance

Entry selects Host 0.4.6 (all four packages), Testkit 0.21.1, Memory 0.31.1 and
Metrics 0.2.9. The filesystem compatible minimum changes from 0.4.3 to 0.4.5,
excluding the fixed Darwin permission-width defect without selecting new packages.
All four non-yanked official archive checksums match the lock and cached bytes.
Published Rust sources match Host publisher
`0fb05f9e18f032425188d68e1d69317a0f0127d5` and Testkit publisher
`a98d751fb6991c12a5d0cd8faaddabe1e879199d`.
Host's [exact owner run](https://github.com/dragginzgame/ic-host-tooling/actions/runs/37648086908)
passes Linux, Intel/ARM macOS and MSRV, including independently observed native
filename admission for CreateNew and Replace. Upstream acceptance is distinct from
this dirty consumer graph, which has no matching native macOS run yet.

The [probe intent](caffeine-probes/local/2026-10-07-publication-0172-01/intent.json)
preceded local HTTP/PocketIC execution. Its initial Metrics 0.2.8 label copied an
old handoff; the separately retained
[correction](caffeine-probes/local/2026-10-07-publication-0172-01/intent-correction.json)
binds actual entry Metrics 0.2.9 without rewriting intent. Entry lock and compiled
logs independently establish the executed graph. No dependency selection changed.
Evidence classes are source review, local HTTP substitute and local PocketIC;
live provider requests and paid cycles are zero.

## Focused checks

| Check | Result / retained raw log |
| --- | --- |
| Duplicate-root test before fix | Expected failure, `duplicate-root-before.log` |
| Batch preparation and header-record cases | Eight and two pass, `batch-preflight.log`, `header-records.log` |
| Actual `make test-native-host` | 114 CLI, 21 examples, two installation/carrier and one Metrics restoration case pass, `native-host.log` |
| First direct publication command | Eighteen cases refuse missing Make-exported fixture paths before service creation, `publication-pocketic.log`, `preparation-failure.json` |
| Publication retry through Make's exported environment | Eighteen pass, `publication-pocketic-retry.log` |
| Lifecycle, capacity planning and read/cleanup | Ten, two and six pass, `lifecycle-pocketic.log`, `planning-pocketic.log`, `reads-pocketic.log` |
| Strict all-target/all-feature CLI/harness Clippy | Pass, `native-clippy.log` |
| Rust 1.88 all-target/all-feature CLI/harness check | Pass, `msrv-native.log` |
| Workspace formatting, shared snapshot and declaration pins | Pass, `final-maintenance.log` |
| Local documentation links | Initial missing-summary links retained after evidence-preparation failure; separate retry passes 1,085 references, `final-maintenance.log`, `final-links-retry.log` |

Rust execution is offline and locked, following separate locked cache preparation.
Focused retries use an ignored extra Make include to reuse repository-owned paths,
server pin and expected identity; no maintained wrapper/target is added. Existing
publication cases exercise uncertain replies, exact intent, resumption, mapping
refusal and live reference checks on the selected graph. Local lifecycle/read
substitutes cover overlap, retained physical/liability accounting, fences, late
callbacks and bounded cleanup. They do not qualify provider deletion or billing.

The [summary](caffeine-probes/local/2026-10-07-publication-0172-01/summary.json)
binds raw hashes, tested binaries and maintained source. Owned test resources are
dropped; no external provider cleanup is pending. No source was edited beneath
active validation. Full CI/release validation, consumer native macOS qualification,
publication and downstream/deployed acceptance remain separate.

## Later Testkit 0.21.2 selection

While closing evidence, the lock changed externally to Testkit 0.21.2, checksum
`0b018e246813e368355100c80db130f2fa941fdd3735fedbcf2a77f5720ee050`.
No other package selection changed. The separate
[additional intent](caffeine-probes/local/2026-10-07-publication-0172-01/testkit-recheck-intent.json)
precedes another bounded local qualification; its
[summary](caffeine-probes/local/2026-10-07-publication-0172-01/testkit-recheck-summary.json)
and raw `.tmp/product-issues-0172-02/` preserve the second entry and execution.
The first graph's exact tested binaries are copied under the first raw directory's
`artifacts/` and bound by `preserved-artifacts.sha256` before rebuilding.

Published Testkit sources match release commit
`2db7b4f6b616b484408695656e26207628d74c5f`; the non-yanked official archive
matches lock/cache. Changes expand the existing compiled shim/post-link recipe
and its host tests without changing runtime implementation or Blob integration.
Its [publisher run](https://github.com/dragginzgame/ic-testkit/actions/runs/37651807716)
still has Intel checks/portable jobs running at inspection; it does not establish
completed native owner qualification yet.

All affected checks pass again on 0.21.2: actual Make native-host (114 CLI,
21 examples, two installation/carrier, one Metrics), 18 publication, ten lifecycle,
two planning and six read cases, strict CLI/harness Clippy and Rust 1.88.
Their raw names match the first graph's table, within the second raw directory.
The final lock/receipt match this second entry. Both execution records remain
distinct; no earlier result is rebound to the newer graph.

Shared Tooling also committed 0.1.21 at
`45e34e92b43edb9543d5b7212774f87f8334079f` during this batch, adding an opt-in
PR release flow while preserving the direct default. Its run is queued at
inspection. That new independent adoption remains next work; the current
verified snapshot is still 0.1.20. No new release helper is executed or adopted.

## Product issue disposition

- [#4](https://github.com/dragginzgame/ic-blob-storage/issues/4): fresh-root gap
  fixed; exact publication recovery rechecked. Complete consumer mapping/asset
  transaction remains in [Canic #452](https://github.com/dragginzgame/canic/issues/452)
  and [Toko Miner #20](https://github.com/dragginzgame/toko-miner/issues/20).
- [#5](https://github.com/dragginzgame/ic-blob-storage/issues/5): bounded native
  header observations added; production MIME/CORS/CSP/cache and full browser media
  acceptance remain open.
- [#6](https://github.com/dragginzgame/ic-blob-storage/issues/6): supported typed
  lifecycle recipe clarified and local regressions pass; actual deletion/billing
  cessation and downstream overlapping-release acceptance remain open.
- [#20](https://github.com/dragginzgame/ic-blob-storage/issues/20): published
  Memory 0.31 is available and guidance corrected; managed acceptance remains
  with [Canic #444](https://github.com/dragginzgame/canic/issues/444).

These issues remain open. Newly accepted
[#27](https://github.com/dragginzgame/ic-blob-storage/issues/27) is an independent
contracts-extraction design batch with a possible public minor boundary, not part
of this compatible release or a reason to duplicate protocol ownership.
