# Host 0.7.1 and Shared Tooling follow-up

Reviewed 2026-10-08 after released Blob **0.18.4**,
`ab15c39d208608d2b43a555fc417dd85a6759942`, validated source
`685548842e36b6fd660dc0911f50b99c220abbe8`. Pending compatible **0.18.5**
preserves the workspace version and release receipt. Incoming Cargo changes were
preserved, rather than attributed to this tooling review.

## Selected graph and retained ownership

The prepared lock selects all four Host **0.7.1**, Testkit **0.24.0**, Memory
**0.31.5** and private Metrics **0.2.13**. Native direct and harness Host identities
now converge; no duplicate 0.6 line, force patch or extra direct process/tools
dependency remains. Public Memory identity stays 0.31. Contracts and pure clients
retain their runtime-free boundary. No service, wire or durable schema changes.

Metadata-selected registry archives match lock checksums. Each unnormalized
manifest and packaged Rust source/test/example matches its recorded owner VCS
commit. Normalized manifests and non-source packaging files are explicitly
excluded from source equality; their complete archive bytes remain checksum-bound.
The first inventory used the wrong cache root, and the second treated a packaged
workspace README as package-local source. Both stopped before writing identities;
the correction is retained separately.

Host's `communicate_child` requires untouched child pipes. Native/browser protocol
callers already own stdout and may have prefetched the final response. Keep
`tests/pocketic/tests/support::wait_with_output` and its real buffered-response
regression. Host still owns process-group lifecycle through Testkit. No named
runtime function, method or type is removed; no speed or instruction gain is claimed.

## Snapshot adoption

The original 81-file export first advanced to committed 0.1.26 source
`75a8a60f49cec11d3f6aecab5c977029c42cc549`. Native qualification and its frozen
source archive belong to that phase. During the run, Shared Tooling committed
0.1.27 source `b866d41041a1986eeec95bde9af4c6ba0853d2e3`, including the previously
authorized Make fixture isolation repair. Its changes also preserve physical
script roots under CDPATH/newline paths and recognize dotted LOC manifests.
No dirty sibling bytes or mutable sibling reference is adopted.

The new installer tests require an evidence fixture, selector and composite-action
definition. Their missing companion declarations allow the old file set to export
successfully, then fail with missing helper / exit **127**. The isolated failed
fixture remains `/tmp/host-tools-test.VX1pot`; raw inputs and output are retained.
Blob explicitly adds those three canonical files, producing an **84-file** snapshot.
The shared copies are unchanged. Reopened
[Shared Tooling #60](https://github.com/dragginzgame/shared-tooling/issues/60)
owns declaration and exporter regression repair.

Direct release delivery, local metadata/receipt authority and the existing
production failure uploader remain unchanged. The new action is selected as an
installer-fixture dependency; ordinary collector routing and hosted archive
acceptance remain with [#29](https://github.com/dragginzgame/ic-blob-storage/issues/29).
The final shared helpers have their own hash record and passing checks. Cargo/Rust
native inputs are byte-exact across the later refresh; earlier archives/results
are not relabelled as the new helper source.

## Qualification and limits

Explicit `make deps` prepares the selected graph without changing the lock.
Focused Linux native-host passes **114** CLI/probe, **103** contracts, **21**
examples, **two** installation/carrier and **one** Metrics restoration cases.
The Make-bound publication-session selection passes **seven** cases; buffered
final-output regression passes **one**. Strict affected Clippy and Rust **1.88**
native/all-target and standalone/storage-probe Wasm compilation pass.
Actual compiled identity is **0.18.4**; CLI, Wasms and harness binaries are frozen.

Final snapshot verification, shell checks, dependency declarations, PocketIC
alignment, selected formatter/installer/metadata/archive/LOC fixtures, full
isolated runner/routing/consumer adapter checks and formatting pass. The reviewed
source's path fixture also passes with inherited CDPATH and a newline checkout.
Git/Cargo/registry release effects in adapter tests are substitutes. No full CI
or release gate ran, and no agent commit, push, release or publication occurred.

Released Blob Linux CI and Host Linux/MSRV pass; native macOS remains pending.
Testkit Linux checks/MSRV/concurrency pass, but its portable job fails at artifact
download after successful upload; [Testkit #33](https://github.com/dragginzgame/ic-testkit/issues/33)
owns that hosted transport defect. New Shared source CI was queued. Previous
0.1.26 passed Linux/ARM macOS/lint but timed out on Intel after its suite reported
passing, retained with [Shared Tooling #71](https://github.com/dragginzgame/shared-tooling/issues/71).
Dirty Blob work has no matching hosted acceptance. No Chromium or deployed provider
run, paid cycles, live provider requests or sibling edits. Owned local process and
PocketIC scopes dropped; failed fixtures and evidence remain.

The [intent](caffeine-probes/local/2026-10-08-host071-0185-01/intent.json) precedes
local runtime effects. Its [summary](caffeine-probes/local/2026-10-08-host071-0185-01/summary.json)
binds commands, graph, source phases, artifacts and failures under
`.tmp/continuation-0185-01/`. The local SHA-256 ledger includes both records.
