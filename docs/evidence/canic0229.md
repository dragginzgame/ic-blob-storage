# Published Canic integration review — 2026-10-11

This is registry/archive/source evidence and owner-reported application acceptance.
No Canic or Toko file, consumer build, provider request or paid effect was performed.

## Bound sources and package admission

Blob is released 0.22.8 at `2f1ee44776fc6c59fc44b0b2c3a1d5c28c9c4835`,
validated source `e6b268f8e38698277f438014d96dbe41cef54d63`. Its finalized receipt
selects Memory 0.35.8, private Metrics 0.6.3, Testkit 0.33.6 and Host 0.13.2.
Incoming Host 0.13.3 and Memory 0.35.9 edits remain preserved and are not qualified
by this documentation review. Package versions and release receipt remain 0.22.8.

Live crates.io sparse metadata observes un-yanked `canic-blob-service 0.111.0`,
Rust 1.91.0 and archive SHA-256
`58f3d9c9f3284ad416993e4fce76513972c75c140063293cedbec5411ebbcf43`.
The cached registry archive matches that checksum. Its VCS identity is released
Canic `25b73cd403b20b4e148c6e340f8c760a26d97f6e`, path
`integrations/blob-service`. All ten packaged Rust source files match both that
committed source and the inspected checkout. The normalized manifest contains
registry dependencies, including Canic `^0.111.0`, Blob runtime/contracts
`^0.22.2` and Memory `^0.35`, with no path dependencies.

Canic's released root lock selects Blob/runtime contracts 0.22.4 and Memory
0.35.5, with SHA-256
`b09ad2cbf37b6dbe8f2ac825d536bd3907294bc9a03f3c54389987f506e308d6`.
At inspection its extensive dirty working graph instead selects Blob 0.22.7,
Memory 0.35.8 and Metrics 0.6.3. Both private managed examples still select Blob
0.22.4 and Memory 0.35.5, with private Metrics 0.5.6/0.6.1 identities. These are
separate captured graphs; do not attribute current root results to the examples.
Multiple private Metrics identities alone are not evidence of a public type or
runtime ownership defect. Canic owns selecting and qualifying its next graph.

The Blob core, contract and standalone Rust source diff from 0.22.4 through
0.22.8 is empty. That establishes source continuity, not execution on a new
resolved graph. Existing installations still bind the exact compiled Blob
release; compatible package updates do not authorize cross-release restoration.

Registry metadata also observes un-yanked Blob runtime/contracts 0.22.8 with
Rust 1.88.0. The committed release-file hashes match the finalized receipt and
its source is HEAD's sole parent. This verifies Git/registry metadata availability;
it is not a new Blob archive or compiler qualification.

## Integration boundary

Published `canister!()` supplies the dedicated shell's memory/lifecycle and
endpoints. Published `mount!()` contributes seventeen permanent requests and
endpoints to an embedded artifact; the application must grant the Blob namespace
in its sole allocation pool, compose synchronous installation/restoration and
combine metrics in its sole sampler. Mounting initializes nothing. Installation
checks the actual service Principal and Blob's compiled `LIBRARY_VERSION`.

Endpoints construct actor/service context from the IC platform and delegate to
Blob. Exact tenant-granted uploader permissions and the independently installed
completion verifier remain separate. Certificate refusal traps. The mounted
funding methods remain history, exact outcome and preparation assessment; no
paid dispatch/credit-confirmation/renewal convenience or provider GC callbacks
are added by publication.

[Canic #444](https://github.com/dragginzgame/canic/issues/444) reports complete
coherent 0.111.0 family publication and source-bound release qualification.
[Toko Miner v0.4.0 acceptance](https://github.com/dragginzgame/toko-miner/issues/6#issuecomment-6107209584)
reports registry-only Canic/adapter 0.111.0 with Blob/runtime contracts/preparer
0.22.5, real disposable managed installation/admission, generated-Candid parity
and same-release database restoration. Its pre-bump Fast artifacts retain their
own identity; this review neither reruns them nor calls them deployed 0.4.0.
Frozen Release-profile qualification, production Root installation and actual
provider/deployment acceptance remain open at the application owner.

## Delivery and remaining work

Blob's README and dependency/funding/service guides now distinguish the published
adapter from historical unpublished reviews. They direct consumers to the
released registry family, Rust 1.91 host minimum, one Memory identity, explicit
initialization and source-bound same-release recovery. Blob's own Rust floor
stays 1.88. No framework dependency, consumer fixture or second owner is added.

- [#31](https://github.com/dragginzgame/ic-blob-storage/issues/31): actual two-user
  application/provider upload and independent completion, after application
  authentication and exact grants. Neutral/disposable installation is distinct.
- [#32](https://github.com/dragginzgame/ic-blob-storage/issues/32): provider GC wire,
  deletion correlation and missing/rejected-method behavior. Existing
  [source-derived contract](../gateway-gc-contract.md) does not supply gateway
  deletion or billing evidence; no placeholder handlers are safe completion.
- [#34](https://github.com/dragginzgame/ic-blob-storage/issues/34) and
  [Canic #494](https://github.com/dragginzgame/canic/issues/494): authenticated
  payer/account linkage, uniquely attributable credit and complete account
  activity before guarded repeated dispatch. Publication supplies no receipt.
- [#33](https://github.com/dragginzgame/ic-blob-storage/issues/33): real worker,
  signer, availability and provider completion evidence. Existing verifier APIs
  remain available; application operation is separate.

Raw sparse metadata, authenticated package inventory, exact lock/source captures,
GitHub owner discussions and CI inventory remain under ignored
`target/review-validation/issues0229/`. Documentation links, runtime-free contracts/native CLI graph, snapshot and
dependency declaration checks pass. Diff checks pass, reviewed package source
and incoming Blob lock remain unchanged. No full CI/release or consumer
compilation is inferred. Historical records retain their original sources and failures.
