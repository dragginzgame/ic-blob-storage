# Sibling reuse and local driver cleanup

Date: 2026-10-08. Verdict: **PASS WITH FINDINGS** within the scope below.
Method: [flow convergence and duplication](../../../../../../../../audits/flow-convergence-and-duplication.md),
adopted Shared Tooling `3ecc48e579f6cf6e6ab01a6645d8a250fc8c6934` (0.1.20),
under the current [local overlay](../../../../../../../../AGENTS.md).
Trigger: maintainer request to recheck siblings and identify cleanup.

## Source, scope and comparison

Started at `704b8ebf6bea85a715e465e32e34758b601852ec` with the publication repair
and incoming Testkit 0.22.0 / Host 0.5.1 / Metrics 0.2.11 edits. The maintainer
committed work during this turn as `114f1b885394941b31388f07bafe6153422efdd9`.
Compiled/package identity remains **0.18.0**, pending compatible **0.18.1**.
The final local delta collapses one browser cleanup condition for Clippy and
completes evidence/docs; it does not change library or service contracts.

Scope: native/browser fixture process custody, final pipe output, selected
sibling library changes, and local file/publication mechanisms that might be
superseded. This is not a whole-system correctness or security verdict.
State/accounting/provider authority, downstream wrappers and deployed browser /
provider acceptance are outside the cleanup scope and retain their owners.
Sibling files were read-only. Dirty upstream work was not adopted.

| Reviewed owner | Exact source | Consequence here |
| --- | --- | --- |
| Host 0.5.1 | `81f9809861159def2fd0987fcb7961cda4afd969` | Shared child-group ownership is available; bounded file/hash APIs retained |
| Testkit 0.22.0 | `2951fd19e58799580e60ec0f6f5864d296271a62` | Existing public reexports select Host 0.5; incoming lock has one Host line |
| Metrics 0.2.11 | `69b110b8fbefdac4773eac7631796f9dcb3f41a0` | Rust implementation unchanged from 0.2.9; qualify incoming private probe graph separately |
| Memory 0.31.3 | `692fbc81d698f4b8253565ff3036f3ad3fb42cd2` | Current selection; Rust runtime unchanged from 0.31.2 |
| Shared Tooling 0.1.23 committed source | `0ba0ad00ed94848e54ecc82629b6b7873b7284c0` | New final release/tag/resume guards available; snapshot adoption remains separate |

Timers 0.14.15, Backup 0.7.0 and Query 0.48.1 were inspected as peers; their
newer work introduces no equivalent service-owned mechanism to delete here.
Canic's catalog selects Testkit 0.22 / Host 0.5 / Backup 0.7; Toko still declares
Testkit 0.7.6 and Miner pins Blob 0.15.1. Those consumer migrations belong to
those repositories; no compatibility bridge or downstream dependency is added.

Locked Cargo metadata selects the exact cached source despite duplicate registry
cache roots. Official downloaded archives match lock checksums and all published
Rust source bytes match their committed sibling source: Host artifacts/fs/process/
tools **20/18/16/17**, Memory **75**, Metrics **5**, Testkit **64** files.
Raw review/source/CI inputs remain in `.tmp/sibling-cleanup-0181-02`.
The initial exactly-one-cache assertion failed; the metadata-selected correction
is recorded independently rather than silently treating it as source proof.

## Owner trace and dispositions

| Behavior | Previous local decision | Current owner / retained projection |
| --- | --- | --- |
| Native/browser start, poll, finish, unwind cleanup | Raw Child plus repeated kill/wait Drop | Host OwnedChild through Testkit; caller keeps executable, control protocol and deadlines |
| Final stdout/stderr collection | Restore raw pipe, then std wait_with_output | One local pipe drain while Host waits/cleans; retain reader-prefetched bytes |
| Input regular-file admission | Nonblocking open then actual descriptor check | Retained: plain shared reader uses pathname precheck; no-follow reader changes allowed final symlinks |
| Download body verification/publication | body.part survives errors; verified body.bin is create-only | Retained: shared random staging does not preserve these exact artifact/recovery contracts |

1. **LOW — duplicate child cleanup, fixed.** NativeSession and BrowserDriver now
   use `OwnedChild` via Testkit; remove raw child optional state and both local
   Drop implementations. Owned groups are cleaned on finish/poll/unwind without
   replacing readiness, budgets, control framing or caller-selected IO. Host
   cleanup has no wall-clock bound and does not contain escaped descendants.
2. **MEDIUM — final browser output can lose buffered bytes, fixed.** Restoring
   `BufReader::into_inner()` discards prefetched final output. A private released-
   pattern control fails with empty output where `final\n` is required. The final
   pipe test preserves that response, stderr and status. One shared fixture helper
   drains both streams concurrently while Host waits; cleanup diagnostics remain
   separate from original protocol/wait failures. No generic process engine.
3. **LOW — duplicate Host lock edges, resolved by the incoming upgrade.**
   Preserve the maintainer's Testkit 0.22 selection. All four Host packages resolve
   to 0.5.1; no force-patch or new direct process/tools dependency is needed.
4. **MEDIUM — newer shared release guards not yet adopted.** The committed
   0.1.23 source has successful owner CI and fixes final payload/tag admission and
   completed-resume readback. Snapshot remains 0.1.20; adopt the reviewed file set
   and qualify consumer adapters as a separate tooling outcome. Dirty archive-
   evidence helpers and other sibling changes are not part of that source.

## Verification and limits

The logged native target passes **114** CLI/probe, **103** contracts, **21** example,
**two** installation/carrier and **one** Metrics restoration cases. All **seven**
maintained native publication-session cases pass, including idle stdin deadline,
step bounds, recovery after lost control and exact-original upload intent.
Compiled readback is 0.18.0. Native CLI and both tested Wasms are copied before
further builds. Earlier 0.17.2 / Host 0.5.0 evidence is not relabelled.

The final buffered-control test, affected strict Clippy and Rust **1.88** native /
Wasm checks pass. Clippy's first collapsible-if failure is retained; only its
browser stop/cleanup condition is changed afterward. Native-session logic and
service/CLI artifact bytes are unchanged. The initial native attempt failed with
exit 2 and incorrectly discarded diagnostics; its unknown cause is explicitly
preserved in the correction record and cannot establish qualification.

Host/Memory/Metrics owner source CI succeeds at the reviewed identities. Testkit
has one successful and one queued exact-source run at inspection; this report
makes no blanket native-macOS acceptance claim. Linux pipes/PocketIC are not
Chromium, deployed-provider or consumer production qualification. No measured
instruction/size gain, full gate, commit/push/release/publication, paid effect or
sibling edit by the agent. Original intent, correction, frozen inputs/artifacts,
logs and failures are bound by the
[probe summary](../../../../../../../evidence/caffeine-probes/local/2026-10-08-host051-cleanup-0181-02/summary.json).

## Removed Rust methods

- `<NativeSession as Drop>::drop`, formerly
  `tests/pocketic/tests/native_session/mod.rs`: duplicate raw-child kill/wait;
  replaced by the Host `OwnedChild` field's destructor.
- `<BrowserDriver as Drop>::drop`, formerly
  `tests/pocketic/tests/browser_driver/mod.rs`: same duplicate cleanup;
  replaced by Host `OwnedChild`, including owned browser descendants.

No other function, method or type is removed. Driver types and their maintained
control APIs remain. Pipe collection is caller-owned IO coordination, not another
lifecycle owner. Protective file/content checks, partial artifacts and all
provider/accounting/retirement records remain with their current owners.
