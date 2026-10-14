# Current status

Date: 2026-10-13

## Released source and pending batch

Blob **0.23.2** is released at `7ad76317fb1ed2695c16ede4aabee655ded84187`,
validated source `5b0c76258313b5581603e52fd033821ffcf590e5`. The finalized
receipt records `release-verify`. Shared 0.10.2, Metrics 0.9, the Canic graph
clarification and native CI timeout/evidence changes are delivered. Historical
[Shared](../evidence/shared0100232.md), [Metrics](../evidence/metrics0900232.md)
and [Host refresh](../evidence/upstream0232.md) results retain their original inputs.
The prior 0.23.1 hosted run passed Linux, MSRV and Apple Silicon but timed out on
Intel; no current-release native acceptance is inferred from the release receipt.

Pending compatible **0.23.3** shares Candid limit construction, carries typed
publication digests and gives local/provider body verification one limits owner.
Native CLI fixtures share command setup and Host-owned process cleanup.
Shared Tooling **0.11.0** at `a15ff323449f4b6190f647aec0de43cbcfb314ba`
is adopted through 116 verified payloads, including the Node/npm checker from
the prior cleanup. CI evidence download now binds the upload ID and digest with
bounded retry and retained failure attempts. Incoming registry Host 0.17.7,
Testkit 0.38.0, Metrics 0.9.1 and Memory 0.36.3 selections are preserved;
no path dependency is used.

The next cleanup delegates Cargo preparation to Shared's metadata helper using
admitted Cargo-edit 0.13.13; Blob retains all-four-file receipt rollback. Query
routing has one publication owner, and recovery packet reads use Host no-follow
opening while retaining caller-specific ancestor and evidence checks.
[Qualification](../evidence/third-cleanup0233.md) records the focused results.

The following cleanup reuses canonical cursor number parsing, shares decoder
construction across eleven contract reply boundaries and consolidates single-call
mechanics across seven replicated clients. Public types, role checks, validation
order, reply budgets and recovery contracts remain unchanged.
[Focused qualification](../evidence/fourth-cleanup0233.md) records the checks and limits.

Synchronous exposure inspection and completion replay now carry validated
permission views. Signed-update dispatch derives common intent facts, and heap
and stable accounting share the lifecycle byte classifier. Public getters, flat
intent JSON, independent obligations and mutation/corruption checks are retained.
[Qualification](../evidence/fifth-cleanup0233.md) includes source-bound read measurements.

[The current cleanup/adoption evidence](../evidence/next-three0233.md) records
focused checks and retained failures. No full Blob CI/release gate ran.
The previous [owner extraction evidence](../evidence/cleanup0233.md) remains
historical. Its Node adapter portion is now delivered here; the retained-staging
Blob filesystem replacement remains pending adoption; Host 0.17.7 is now selected. Do not apply the whole old consumer patch over this batch.

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
