# CLI record custody, Shared 0.9.7 and Testkit 0.35

Date: 2026-10-12. Released base is Blob 0.22.14,
`54d8186d8db47fbb00ec7531163f34f45ad2f2f9`, validated parent
`5dc6fa2d2d666482bc9b5f81d980630c0d158958`. Pending compatible notes are 0.22.15;
all twelve package versions and finalized receipt remain 0.22.14.

## Ownership and contract

CLI Run retains the originally claimed directory descriptor. Complete byte and
JSON records share Host's descriptor-relative create-new writer, with private
0o600 files. The caller's named run must still be the same non-symlink directory;
a deleted, symlink-substituted or real replacement directory cannot inherit it.
Host admits a single filename component, owns staging/sync/publication/cleanup,
and never recreates the run's parents. Ancestor custody and exclusion of concurrent
namespace writers remain caller responsibilities; this is not an adversarial
filesystem sandbox or compare-and-swap claim.

Body transfers retain their separate recovery contract: body.part is kept after
verification/transfer failure, and body.bin publishes only after complete verified
EOF and synchronization, without replacement. Their directory synchronization now
uses the same held handle and checks named ownership before further effects.
No new storage endpoint, lifecycle owner, durable schema or production metric is
introduced. Existing named CLI output files and redacted failures remain.

Incoming Testkit 0.35.0, Host 0.15.1, Memory 0.35.19 and Metrics 0.8.0 selections
are preserved byte-for-byte from the starting manifests/lock. Testkit no longer
owns generic build APIs or re-exports Host tools. Blob has no callers of the
removed APIs; its private interface checker already selects Host tools directly,
with defaults disabled and only ic-interface. Host now has one version in the
graph; Blob gains no IC Build dependency. Public Blob APIs, supported build floor
and stored contracts remain unchanged.

## Shared adoption and fixture disposition

Committed Shared 0.9.7 `4d10a06d50e44d302f00d5b7b706cac5f9b0127c` matched remote
main when selected. A clean detached temporary clone excluded sibling dirty work.
Canonical preview/refresh retains the previous 108 files and adds exact artifact
discovery and the local retry fixture, for 110 final payloads. The manifest has
unchanged old roots plus those explicit additions; no baseline payload is retired.

The reviewed changes centralize Host/IC installer cleanup and bounded transient
asset retries, refuse malformed checksum/path bytes and incomplete Git identities,
and retain boolean TOML Cargo offline policy before string extraction. Explicit
setup prepares the changed Cargo-tool policy and Testkit CLI; old generations
remain retained. Pin files, package versions, finalized receipt, index and Git refs
are not changed by the batch.

The hosted tooling workflow waits for the exact uploaded ID to become visible,
with actions:read and a two-minute step bound. It preserves the same repository,
run and artifact ID into the existing pinned download, digest refusal and independent
archive SHA-256 readback. Discovery never uploads again or selects by name.

Initial qualification exposed two fixture invocation mistakes, retained in the
raw packet: the store fragment needs its host/IC parent fixture, and the producer
artifact fixture contains Shared's own workflow assertions. The former is covered
through both correct parents. The latter is executed at the frozen producer source;
its temporary consumer selection was retired through the canonical exporter.
Blob's own workflow is inspected independently and linted, without copying the
producer's workflow policy into this consumer.

The actual consumer-root Cargo fixture fails because its newly added direct
fake-compiler calls see Blob's real rust-toolchain.toml. A traced rerun preserves
the failing context. The exact canonical fixture passes through a neutral-cwd
local invocation adapter on Bash 5.2 and genuine Bash 3.2.57. The shared payload
is unchanged; [Shared #143](https://github.com/dragginzgame/shared-tooling/issues/143)
owns the minimal producer correction. Remove this adapter after a reviewed
canonical fix supports the consumer-root invocation.

## Focused execution

Raw incoming/final inputs, logs and artifacts are retained outside Cargo clean at
`.tmp/review-validation/cleanup02215.dFOUKG/`. Qualification uses Rust 1.99.0,
MSRV 1.88.0, published Testkit CLI 0.35.0 and PocketIC 16.1.0 on local Linux.

- Locked fetch passes, preserving incoming Cargo.toml/Cargo.lock exactly.
- All 113 CLI binary tests pass, including complete records, no replacement,
  private permissions, removed/substituted run refusal, failed producer cleanup,
  namespace confinement and retained partial upload/download evidence.
- Scoped all-target/all-feature denied-warning Clippy and Rust 1.88 checks pass
  for CLI and private harness, exercising the unified Host/Testkit graph.
- Fresh release probe/standalone Wasm and CLI binary builds pass. Actual local
  PocketIC restoration/reset and independent funding-credit/replay read-window
  cases pass. Both standalone installation-carrier cases pass, including actual
  rebuilt CLI output, canonical Candid/callable Wasm parity, wrong-service refusal,
  stable-byte preservation and correct empty-fixture installation.
- Snapshot, consumer Make, checksums, Host/IC installers and their store fragment,
  neutral Cargo fixture (including relative/aliased temporary parents with clean
  cleanup) and local-transport retry fixtures pass on modern Bash
  and genuine Bash 3.2. The latter is Linux portability evidence, not native macOS.
- Isolated common release-runner checks pass on modern Bash and Bash 3.2;
  the existing Blob release adapter target passes without real release effects.
- Producer snapshot distribution and exact artifact-discovery scenarios pass at
  the frozen source; producer PR-release recovery/refusal checks pass separately.
- The initial offline tools check refuses the old Cargo-tool policy selection.
  Explicit common Rust and Testkit setup succeeds; complete offline admission then
  passes, preserving prior installations. No pin or hidden online-check change is
  made. Initial actionlint path lookup fails; the already-installed pinned 1.7.12
  executable then passes the actual workflow. Shell syntax and ShellCheck pass.
- Static workflow inspection verifies exact ID/repository/run propagation,
  discovery ordering and bound, success-only pinned download and digest refusal.
  Blob's actual tooling evidence fixture passes its original retention/refusal
  behavior. Live CI upload/readback of this new workflow remains future evidence.

The matching released 0.22.14 hosted run is
[38280107658](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38280107658):
Linux and MSRV succeed; native macOS remains incomplete at inspection. Its finalized
receipt/parent/file hashes and delivered Metrics source match the original frozen
qualification, so [#62](https://github.com/dragginzgame/ic-blob-storage/issues/62)
is closed. #56–#59 retain their independent native/archive/tooling acceptance;
queued or cancelled lanes are not passes.

No full Blob CI/release gate, optimized PocketIC, native macOS, publication,
deployment, paid effect, staging, commit or push is performed by this batch.

## Canic decoder qualification

[Canic #524](https://github.com/dragginzgame/canic/issues/524) remains open pending
family delivery. Its two native dedicated/embedded decoder regressions now pass
after the previous Auth blocker cleared. The managed fixture retains exact
original initialization bytes; the installed cases vary only nested application
arguments under the original protected envelope. This is host-side test support,
not reset authority. Independent locks align directly required Blob/Auth/Timers
selections; original locks and failed attempts remain.

Original-checkout attempts are retained with Canic in `.tmp/issue-524/recheck.*`:
stale locked metadata, an interrupted waiting check during independent owner builds,
published-CLI admission refusal for source-path Testkit, and an explicit published
CLI local-server attempt that stops in 16 E0061/E0277 errors while the owner moves
from Timers 0.19.3 to 0.21.0. No warning suppression, source-path fallback or timer
repair is added by this decoder work.

Concurrent source/build activity is resolved with frozen temporary checkouts at
`/tmp/canic-524-isolated.v5md68w9/`. Canic and its selected Testkit source dependency
are captured with before/after file hashes; compilation uses that checkout's own
target. The published, explicitly admitted Testkit 0.35 CLI owns PocketIC 16.1.0;
its identity is distinct from the native library's frozen source-path input.
Independent fixture locks select Blob/contracts 0.22.14, Auth/protocol 0.5.7,
Memory 0.35.14, Metrics 0.8.1 and Timers 0.21.0; they are distinct from Blob's
native graph and historical Canic fixture selections. Both managed composition
cases pass, including complete current managed artifact construction, endpoint
parity, permissions/capacity and same-release restoration:

- Dedicated build `f665751a199eb49b43d11b0a38b7c59263ac44dc6d5900e27140a9f858d3f85b`;
  corrected local case 36.22s including build-worker reuse (32.70s).
- Embedded build `e0225bbcdcb30a50a24c3b28e246abde37fb7fbf3f25d25f2625acc763ccd752`;
  local case 42.51s including worker build (38.22s).

Each rejects unknown fields and extra Candid arguments with an IC trap, preserves
exact stable bytes/configuration and retains the Blob recovery fence. The embedded
case also preserves the original app count. The maintained operator same-instance
recovery path then reactivates the original installation; no direct fence clearing
or successful reinstall is attempted. The first dedicated assertion incorrectly
expected an unfenced view after management refusal. Its failed 230.15s attempt and
build `3ad01a38929f662cb947db986c647371b5154b20196ad11c48e690087f9779e2` remain;
the corrected test requires the observed fence and actual recovery.

Scoped denied-warning Canic/testing Clippy and both final native decoder cases
pass on the frozen graph. The fence-aware source and directly required fixture
locks are applied to the original checkout only after checking it is idle and the
files match the captured baseline. The current source-path standard runner and
wider evolving Canic batch retain their own qualification obligations. No full
Canic suite, source version mutation, Git delivery, deployment or paid effect is
performed. Local substitute credit/managed actors do not qualify a real application
or deployed storage provider.
