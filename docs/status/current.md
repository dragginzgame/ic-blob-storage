# Current status

Date: 2026-10-13

## Released source and pending batch

Blob **0.23.1** is released at `dc169f9bfe6c1e736c189e1ce5ee478cfe81b745`,
validated parent `12b13f868d2483d5ce1035c26d5197299f0f8052`. Remote main and
peeled tag match; the finalized receipt records `release-verify`. The Candid
export split, Testkit-owned startup and small-reply decoder cleanup are delivered.
[Original size evidence](../evidence/candid-export0231.md) and
[harness evidence](../evidence/harness0231.md) retain their own graphs.
The released lock selects Testkit 0.37.0, Host 0.17.0, Memory 0.36.1 and
private Metrics 0.8.4. Hosted [38381638982](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38381638982)
has passed Linux and MSRV. Intel macOS exhausted the 60-minute job limit during
`test-release.sh`, before package/native qualification; Apple Silicon subsequently passed.
The overall run is cancelled because Intel did not complete. Failure-only tooling retention skipped the cancelled lane.
Pending 0.23.2 gives the native job 120 minutes, bounds its tooling fixture step
at 45 minutes to leave collection time, and retains evidence on failure or
cancellation. Release-adapter cases now announce their names before execution.
The hosted outcome remains unqualified until the corrected workflow runs.
Local release-adapter fixtures, tooling-evidence tests, ShellCheck, YAML selection
checks and documentation links pass; logs and the original Intel job log remain
in `.tmp/review-validation/continue0232/`. No hosted rerun or full CI was executed.
#63 and #56–#59 still require their native acceptance;
release delivery alone does not qualify those checks.

Pending compatible **0.23.2** adopts Shared Tooling **0.10.2** at
`626fc2bd3125195520fb870b92abfec11b01454d`, retaining 112 snapshot files.
Both catalogs were already vendored, and every existing tool version/digest
matches. No exception file is needed. Redundant local Make defaults are removed;
all common selectors now inherit from the reviewed snapshot. Compiler, Testkit and
release metadata remain unchanged by the Shared adoption. Explicit setup and
focused tooling qualification are recorded in the [adoption evidence](../evidence/shared0100232.md).
[#66](https://github.com/dragginzgame/ic-blob-storage/issues/66) retains delivery
and native qualification. The [subsequent refresh](../evidence/upstream0232.md)
qualifies Shared 0.10.2 and the incoming Host 0.17.2 graph separately from the
original 0.10.0 and Metrics/Host 0.17.1 evidence.

The same pending batch selects published Metrics 0.9.0 privately in the storage
probe, plus incoming Memory 0.36.2 and Host 0.17.2, retaining Testkit 0.37.0.
Metrics overflow getter calls are updated; public Blob types and probe wire names
stay unchanged. [Focused qualification](../evidence/metrics0900232.md) addresses
[#65](https://github.com/dragginzgame/ic-blob-storage/issues/65); release delivery
remains pending. The operator guide now includes the
[prior-admitted canister-tenant recipe](../operator-guide.md#continue-an-upload-already-admitted-by-a-canister-tenant)
requested by [#4](https://github.com/dragginzgame/ic-blob-storage/issues/4), including
explicit original MIME hints. The consumer still owns real application transaction,
provider/verifier and lost-success acceptance; that issue is not complete.

## Canic boundary

The 2026-10-13 read-only refresh identifies Canic HEAD
`23d27f96f4e518f0313b6d730f53724ba836ffbe` with incoming Cargo/status edits.
The remote 0.111.0 tag still selects Blob/contracts 0.22.2 and Memory 0.35;
no remote 0.112.0 tag was observed. Current source selects Blob 0.23/Memory 0.36,
while package metadata remains 0.111.0. README and dependency instructions now
separate those families explicitly. The sibling composition guide's older version
prose remains with Canic; this review changes no sibling and runs no consumer build.
Issues #31/#33/#4 still need actual application/provider acceptance. No new Blob
runtime gap was found in the reviewed upload preparation and recovery owners.


Read-only review confirms fresh Canic-owned dedicated and embedded managed
Blob tests pass with Blob/contracts 0.23.0 and Memory 0.36.1. The recorded root
manifest and three lock hashes still match. The tests cover permissions, exact
installation decoding, endpoint parity and restoration fences; the embedded
case also retains application state. Dedicated build:
`39021f01483ac31d8be5ca364ca32605ce38d382c13ad0ee0ae7315d6a5069e8`;
embedded: `b6bbcd1b93783b92f3f4325e00f416976495ce228c29f4e783e4551ea52bf6d0`.
Original evidence remains in Canic `.tmp/memory-036/blob023-{dedicated,embedded}.log`.
This clears the local composition blocker. Family delivery and actual application/
provider acceptance remain under [#527](https://github.com/dragginzgame/canic/issues/527)
and [#444](https://github.com/dragginzgame/canic/issues/444). No sibling edit or
new Canic execution occurred here; older proof below retains its original graph.

Canic's wider Auth/Backup/Blob batch remains with its own owner under
[#517](https://github.com/dragginzgame/canic/issues/517) and
[#444](https://github.com/dragginzgame/canic/issues/444). The prior scoped #524
quota-zero fix remains. Its two native regressions now pass; prepared managed
cases test installed unknown-field/extra-argument rejection and rollback in both
compositions, using exact retained protected envelopes. Old locks/attempts remain.

Both installed cases now pass against frozen Canic/Testkit working-source inputs
in `/tmp/canic-524-isolated.v5md68w9/`, with their own target and explicit published
Testkit 0.35 CLI server owner. They retain the required fence after rejected
reinstall, preserve stable bytes/configuration/app state, and use actual operator
same-instance recovery. Scoped Canic/testing Clippy and final native cases pass.
Corrected source/required fixture locks are applied back only to unchanged, idle
original files. #524 remains open for family delivery. The standard source-path
runner refusal, interrupted check, first unfenced-view assertion and earlier
Timers 0.21 caller failure remain with their distinct recorded inputs. This is
local frozen composition proof, not the entire evolving Canic graph or provider.

Application-parent initialization remains Canic #493; guarded funding composition
remains #494, dependent on authenticated provider credit/payer facts in Blob #34.
Actual application two-user/verifier acceptance remains Blob #31/#33; media-map
transaction #4; serving #5; provider deletion/GC/billing #32/#6. No downstream
wrapper/fixture, fake provider evidence or funding bypass belongs in Blob.

[Prior handoff](review-before-02215.md) retains earlier Canic composition hashes,
probe evidence, Shared history and exact release identities. Historical guarantees
stay bound to their original sources and independent locks.
