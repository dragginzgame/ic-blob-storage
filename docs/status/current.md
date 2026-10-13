# Current status

Date: 2026-10-13

## Released source and pending batch

Blob **0.23.0** is released at `4277ed1ec059c1eb36e4bc5e32e7b6b7eb74ecc2`,
validated parent `c0e87982a40eefc4825c2563fbf5176ad2199a91`. Remote main and
peeled release tag match. The Memory 0.36 explicit-request adoption is delivered;
[#64](https://github.com/dragginzgame/ic-blob-storage/issues/64) is closed against
that evidence. [Its source-bound qualification](../evidence/memory036-host016-shared0913.md)
remains historical. Hosted [38375231253](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38375231253)
was queued at inspection; #56–#59 retain their separate native acceptance scopes.

Pending compatible **0.23.1** makes standalone Wasm
Candid extraction opt-in through `candid-export`, while native generation stays
available. Production endpoints, Candid, keys, schemas and lifecycle are unchanged.
[Cleanup evidence](../evidence/candid-export0231.md) records raw/optimized size
comparisons and qualification. Package versions and release effects remain
maintainer-owned. Shared remains the reviewed 112-file 0.9.13 snapshot at
`35cad65677ff989b5fb24a88d947fda37d2366e8`.

The next local simplification delegates PocketIC startup to Testkit's owned
constructor, preserving caller topology and instance-first teardown. Incoming
direct Host 0.17 / Metrics 0.8.4 edits are retained; the interface test explicitly
selects Host's `candid-service` feature. Testkit 0.36 still uses Host 0.16 internally.
[Focused evidence](../evidence/harness0231.md) separates this newer graph from the
preceding Candid size measurement and its original Host 0.16 proof.

CLI account, funding-assessment and gateway commands now share their identical
small-reply decoder. Each retains its original oversize error, request binding
and service-failure handling; installation and large-status decoders stay separate.
Focused CLI results are retained at `.tmp/review-validation/replies0231/`.

## Canic boundary

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
