# Current status

Date: 2026-10-11

## Released source and current batch

Released Blob is `2f1ee44776fc6c59fc44b0b2c3a1d5c28c9c4835` (0.22.8),
validated parent `e6b268f8e38698277f438014d96dbe41cef54d63`. All twelve package
versions and the finalized receipt remain 0.22.8. Committed release-file hashes
match that receipt. Registry metadata observes un-yanked runtime/contracts
0.22.8 with Rust 1.88. The release lock selects Memory 0.35.8, private Metrics
0.6.3, Testkit 0.33.6 and all four Host 0.13.2; PocketIC remains 16.1.0.
The pending source commit `9c88d524d730e398ae9413a9aec2675b4a6ff60e` includes
Host 0.13.3/Memory 0.35.9. Preserve the later Testkit 0.33.7 lock edit; earlier
review results do not qualify these changed product compiler inputs.

Shared 0.6.0 remains the reviewed 102-file snapshot at
`75180980a0c4e4252d8a915aaecfaefd92bf9a4e`. The previously missing evidence file
and collector correction are committed in this release. Original failures,
setup/reuse and both Bash qualifications remain in
[their source-bound record](../evidence/shared0600228.md); its earlier untracked
observation is historical. No new tool setup or Shared refresh is performed here.

Pending compatible 0.22.9 corrects stale Canic/Memory guidance and records current
published composition and remaining application/provider acceptance. The
published Canic adapter 0.111.0 archive matches live registry SHA-256 and all ten
Rust source files match its released source and inspected checkout. Its normalized
manifest uses registry dependencies and Rust 1.91, while Blob's own floor stays
1.88. Canic owns the dedicated/embedded adapter, single host memory/lifecycle and
managed integration tests; Blob acquires no downstream dependency or fixtures.

Canic's released graph uses Blob 0.22.4. Its dirty root now selects 0.22.7 while
both independent examples retain 0.22.4; their private Metrics selections also
differ. Preserve and qualify those graphs with their owner rather than attributing
root checks to the examples. Blob core/contracts/standalone Rust source is
unchanged from 0.22.4 through 0.22.8; that is source review, not graph execution.

Toko Miner reports delivered registry-only Canic/adapter 0.111.0 with Blob
0.22.5, disposable managed installation/admission, generated-Candid parity and
same-release database restoration. Production Root installation, frozen
Release-profile and provider/deployment acceptance remain open. See
[the current review](../evidence/canic0229.md) for exact source, package, graph and
owner evidence. Earlier [Canic reviews](../evidence/canic0223.md) retain their
original inputs and limitations.

Documentation links, runtime-free contract/CLI graph, snapshot/dependency
checks and diff checks pass. No full CI/release, consumer compilation, sibling
file mutation, staging/commit/push, deployment, paid provider effect or cleanup
is performed in this batch. Public APIs and durable schemas are unchanged.

## Subsequent Testkit prerequisite repair

The maintainer's retained CI attempt
`.git/release-state/validation-failures/20261011T130201Z-8625-0-ci.log`
compiled Testkit 0.33.6, then refused 0.33.7 at the optimizer's offline tool check:
the lock selection changed during validation. Explicit `make install-testkit`
now prepares the canonical shared 0.33.7 CLI and admits the managed PocketIC
16.1.0 server. Complete `CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0 make tools-check`
passes and the selected lock stays unchanged during setup/checks. Original failure,
selected lock and setup/check logs remain in
`target/review-validation/testkit03370229/`. No tool fallback, implicit validation
setup, lock reselection or cleanup was introduced. Commit the settled source and
restart full validation; preceding passing stages are bound to their earlier graph.
The agent did not rerun full CI or execute a release command.

## Issues and next work

- Matching [0.22.8 CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38139356846)
  has a successful Rust 1.88 lane, Linux in progress and both macOS lanes queued
  at inspection. This is not a complete green gate and cannot qualify dirty
  0.22.9 inputs. [#59](https://github.com/dragginzgame/ic-blob-storage/issues/59)
  retains native Shared 0.6 setup/admission/evidence readback acceptance;
  [#58](https://github.com/dragginzgame/ic-blob-storage/issues/58) retains matching
  hook temporary-path acceptance. #56/#57 retain their exact older archive/graph
  requirements; cancelled or artifact-only lanes are not complete acceptance.
- [Canic #444](https://github.com/dragginzgame/canic/issues/444) owns published
  adapter/application composition and independently locked managed examples.
  [Toko Miner #6](https://github.com/dragginzgame/toko-miner/issues/6) owns remaining
  production application acceptance. Adapter publication itself is delivered.
- [#31](https://github.com/dragginzgame/ic-blob-storage/issues/31) retains actual
  two-user application/provider upload; [#33](https://github.com/dragginzgame/ic-blob-storage/issues/33)
  retains real verifier operation. Disposable application installation does not
  establish successful provider completion.
- [#32](https://github.com/dragginzgame/ic-blob-storage/issues/32) needs authoritative
  provider GC/missing-method/deletion/billing meaning before safe callbacks.
  [#34](https://github.com/dragginzgame/ic-blob-storage/issues/34) needs authenticated
  payer/credit/complete account activity before [Canic #494](https://github.com/dragginzgame/canic/issues/494)
  can qualify guarded repeated funding. Do not manufacture receipts, placeholder
  deletion responses, signer/deployment identities or paid probes.
- #4/#5/#6 retain consumer media-map, browser serving and physical/economic
  lifetime acceptance. Core primitives and operator recipes already exist.

[Earlier review handoff](review-before-0226.md) and
[history through 0.22.4](history-through-0224.md) retain prior source-bound results.
Historical handoffs are evidence, not current command authority.
