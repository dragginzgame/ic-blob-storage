# Current status

Date: 2026-10-13

## Released source and pending batch

Blob 0.22.16 is released at `de19bbc019fd18dac3206841a404db6b390719e7`.
The preceding audit cleanup is delivered. Its hosted run
[38366758067](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38366758067)
was queued during this inspection; it does not qualify the new working tree.

Pending **0.23.0** adopts public Memory 0.36.0 and direct Host artifacts/fs/tools
0.16.0, preserving the incoming Cargo selections. Metrics remains privately
selected at 0.8.2; Testkit 0.35.1 retains Host artifacts/fs/process 0.15.2.
Memory request factories now return `MemoryDeclarationError`; standalone/probe
hosts transfer their complete owned request vectors into the sealed snapshot.
Stable keys, allocation inventory and persisted schemas remain unchanged.
The removed global-registry observations leave the actual no-bootstrap and
committed-capability rejection assertions intact. No compatibility alias is added.

The reviewed Shared snapshot is **0.9.13**,
`35cad65677ff989b5fb24a88d947fda37d2366e8`, with all 112 payloads retained.
Shared's local 0.9.14 edits were uncommitted and excluded. Snapshot refresh uses
an isolated clean committed source; sibling work remains untouched.
The changes preserve lock/source-acquisition errors, authorize separate locked
source fetching and require an explicit CI target when no branch is observable.

[Adoption evidence](../evidence/memory036-host016-shared0913.md) records focused
qualification and retained attempts. Package versions and release effects remain
maintainer-owned. Cross-release retirement/reinstall policy remains unchanged;
Memory's source cut does not itself change the ledger encoding. Canic must align
its own Memory identity under [#527](https://github.com/dragginzgame/canic/issues/527).
Blob [#64](https://github.com/dragginzgame/ic-blob-storage/issues/64) tracks delivery;
#56–#59 still require their exact native/archive/tooling acceptance.

## Canic boundary

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
