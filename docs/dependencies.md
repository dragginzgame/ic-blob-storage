# Dependency setup

The root `Cargo.toml` owns direct dependency version requirements. The library
inherits them, and `Cargo.lock` locks the resolved graph. Versions were checked against
crates.io on 2026-09-25. On 2026-09-26, `serde_json` became a direct dependency
at its existing locked version for bounded provider reply parsing. Dependency
availability does not establish provider qualification or service readiness.

| Dependency | Version | Purpose |
| --- | --- | --- |
| `candid` | 0.10.36 | IC boundary encoding and principal types |
| `serde` | 1.0.229 | Serialization derives for explicit boundary/record schemas |
| `serde_json` | 1.0.151 | Bounded Caffeine chunk-status JSON decoding; reused the existing lockfile version |
| `sha2` | 0.11.0 | SHA-256; optional allocation/OID features disabled |
| `thiserror` | 2.0.18 | Typed error derives; matches PocketIC's exact requirement |
| `ic-cdk` | 0.20.3 | IC platform operations for the ops layer |
| `ic0` | 1.2.0 | Existing CDK system-API version, now direct for bounded participant argument copying |
| `ic-memory` | =0.14.3 | One runtime identity with published Canic 0.110.49 |
| `ic-stable-structures` | 0.7.2 | Exact transitive substrate owned/re-exported by `ic-memory` |
| `ic-testkit` | 0.10.1 | Native dependency of the unpublished PocketIC harness; shared helpers and full re-export |
| `pocket-ic` | 16.0.0 | Transitive through `ic-testkit`; no direct dependency |
| `ic-agent` | 0.49.2 | Native test-only signing and verification of local ingress certificates |
| `canic` | 0.110.49 | Direct dependency of the managed fixture and native qualification harness only |
| `candid_parser` | 0.4.1 | Native harness only; official parser/type checker for built managed Candid |

Headless ingress tests add pinned `ic-agent` 0.49.2 (default features disabled),
plus the already locked `reqwest` 0.13.5, `tokio` 1.53.1 and `serde_cbor` 0.11.2
as native dev dependencies in `tests/pocketic`. The public crates.io index confirmed
0.49.2 as the latest non-yanked agent release on 2026-09-28. Its cryptographic graph
adds host-only lockfile entries without changing existing package versions, the
published library dependency graph, Wasm allocation or memory grants. Tests trust
the explicitly owned PocketIC NNS key, never a root key fetched from mainnet, and
send no request to a Caffeine gateway. This is headless Rust evidence, not browser
or production uploader qualification.

At the initial registry check, the selected releases were current except `thiserror`, where PocketIC 16 pins
2.0.18 and prevents selecting 2.0.21 in this graph. All selected versions compile
with the repository's Rust 1.98.1 toolchain. SHA-256 byte hashing does not by
itself implement or qualify Caffeine's provider-specific hash tree.

The test dependency was consolidated through the published `ic-testkit` 0.10.0
package on 2026-09-26. Tests should import upstream types through
`ic_testkit::pocket_ic`, and opt into helpers through `ic_testkit::pic`:

```rust
use ic_testkit::pocket_ic::{PocketIc, PocketIcBuilder};
use ic_testkit::pic::{CandidCallExt, CanisterInstallExt};
```

The [published export](https://docs.rs/ic-testkit/0.10.0/ic_testkit/index.html)
exposes the complete PocketIC crate. Keep the dependency under native dev
dependencies; neither the production library nor its Wasm build needs testkit.
With the dedicated harness in place, the core package's native dev graph also
excludes it; testkit is owned by `tests/pocketic/Cargo.toml`.
The unpublished `tests/pocketic` harness uses these exports. This shares version
selection and harness helpers, rather than reducing the total transitive package
count: testkit also brings host-side artifact/locking utilities.

## Local Canic development

Canic publication or a deployed Fleet is not required for local development.
Freeze a committed Canic revision into an ignored directory here, copy this
repository's current Rust workspace into a separate directory, and apply
`[patch.crates-io]` path overrides for `canic`, `canic-core` and `canic-macros`
in both copied workspace manifests (root and `canisters/test/canic_probe`).
Cargo resolves new lockfiles only in those copies. Keep the sibling checkout
read-only and this repository's release manifests/lockfile unchanged. Freeze
source first so a concurrent Canic release cannot change a running build.

Use the maintained `canic build` command against the copied fixture workspace,
with a separate `--icp-root`, `CARGO_NET_OFFLINE=true`, `RUSTC_WRAPPER=` and this
repository's `CARGO_TARGET_DIR`. Do not bypass Canic's role-contract build marker.
Then run the copied PocketIC harness with `BLOB_CANIC_PROBE_WASM` pointing at that
generated artifact and `POCKET_IC_BIN` pointing at the provisioned local server.
PocketIC creates the managed test group; no live Fleet deployment is needed.

The current prepared workspace is `.tmp/local-canic-02/blob`, using committed
Canic `32da629d0214bf791541a9b3c1832dbef13ece29` in `.tmp/local-canic-01/canic`.
Its artifact is `.tmp/local-canic-02/icp/.icp/local/canisters/storage/storage.wasm`.
The [handoff](status/current.md) records executed checks and limitations.
Both maintained workspaces now pin registry Canic 0.110.49, including its core
and macros. The canonical build and six focused managed checks pass against those
packages without path overrides; this local source lane remains optional.
The package-adoption action [CF-01](canic-parity.md#integration-feedback) is closed;
see [published dependency evidence](evidence/core-primitives.md#published-canic-adoption--2026-10-01).
Local success does not qualify provider economics/recovery or certificate exposure.

## Browser certificate evidence

`tests/browser` is a private fixture, with npm-locked `@caffeineai/object-storage`
1.1.2, `@icp-sdk/core` 5.4.0, Playwright 1.63.0 and esbuild 0.28.2. SDK 5.4.0
is within Caffeine's declared `^5.3.0` range; it replaces the earlier fixture's
6.1.0 pin so the composition uses one supported SDK. The Caffeine package is the
latest verified provider package; SDK 6 is not forced into its dependency graph.
Use Node >=20.19.0; this run used Node 24.21.0 and
Playwright's Chromium 153.0.8010.12 (revision 1243). Setup is explicit:

```sh
npm ci --prefix tests/browser --ignore-scripts --no-audit --no-fund
node tests/browser/node_modules/playwright/cli.js install chromium
make test-browser
```

Use a supported Node on PATH for setup. `BLOB_BROWSER_NODE=/absolute/path/to/node`
can select the runtime for `make test-browser`. The target bundles existing local
packages, builds the storage and consumer probes and runs the ignored browser case explicitly.
It downloads nothing. Missing packages/browser/runtime fail rather than skip.
The ordinary Rust/CI/release suite does not run this opt-in browser test.
The fixture imports the private reusable source client in `clients/browser`.
Its peer dependency, fixture SDK pin and installed SDK version must agree before
bundling. Git applies the [pinned Caffeine patch](../clients/browser/patches/README.md)
to a generated copy under `.tmp/browser`, after original package hashes are checked.
The installed package is not modified. Application authentication and production intent storage are caller-owned;
see the [client contract](../clients/browser/README.md).

Rust owns the PocketIC installation and test identity. Browser traffic is confined
to that local IC endpoint and the owned page/gateway substitute. The IndexedDB store has two lifetime
slots and no reset/eviction path; it tests transaction ordering, competing tabs,
reload, cancellation and verified historical replies. Caffeine performs real HTTP
tree/chunk requests only against the local substitute. Failed/aborted transfer does
not retry. This is not production sizing, crash/eviction durability, browser-profile restoration, a deployed gateway upload,
or qualification of a consumer's CSP/authentication/storage environment.

### Managed browser setup

The managed browser check uses the same provisioned packages and Chromium:

```sh
make test-canic-browser BLOB_BROWSER_NODE=/absolute/path/to/node
```

It builds the canonical managed fixture and existing consumer, then runs actual
signed browser admission/preparation through the shared service workflows.
Production certificate facts remain false, so one claimed request refuses with
no gateway traffic; browser uncertainty and cancellation plus service cleanup/
fenced history are checked. This local case is outside ordinary CI and establishes
no deployed Caffeine guarantee. Canic build tools are required; no Fleet deployment
or package/browser download occurs.

### Offline native/browser handoff

With the same local packages and Node 24, run the optional preparation check:

```sh
make test-sdk-inputs BLOB_BROWSER_NODE=/absolute/path/to/node \
  BLOB_SDK_INPUTS_REPORT=/tmp/new-browser-inputs-evidence
```

It bundles the hash-checked pinned SDK/patch, builds the native CLI offline and
prepares a 10 MiB file. Actual SDK output feeds `upload-inputs`; its generated
`certificate-binding.json` is consumed by the existing browser source client in
Node with an in-memory setup-only store. Snapshot repreparation, no-clobber repeat
and corrupt-source refusal retain exact output/logs in the new directory. All
network calls throw; no certificate issue/recovery or gateway upload occurs.
This check needs neither Chromium nor PocketIC and remains outside ordinary CI.
It establishes no production storage durability, service authorization or provider
guarantee. Existing evidence directories refuse reuse; build artifacts are kept.

## Memory composition with Canic and IcyDB

The core pins published `ic-memory` 0.14.3 and its `ic-stable-structures`
0.7.2 substrate, matching registry Canic 0.110.49. Growth uses the upstream
`Memory::grow` sentinel contract. Managed builds must resolve one package identity;
the multiple-runtime guard remains enabled. Adoption of 0.15-only APIs is deferred
so this release does not depend on Canic publication. The earlier adoption and
refusal captures remain historical evidence, not the current dependency selection.
See [alignment evidence](evidence/core-primitives.md#published-memory-alignment--2026-10-01)
and [CF-02](canic-parity.md#integration-feedback).

The earlier 0.1.12 combined resolution of Canic 0.110.42, IcyDB 0.261.11 and
`ic-memory` 0.14.3 remains historical evidence. Neither framework is a dependency
of the blob core; IcyDB composition with 0.15 has not been qualified here.

Use the shared public path for storage types:

```rust
use ic_blob_storage::ic_memory::{
    RuntimeMemory,
    ic_stable_structures::{Cell, DefaultMemoryImpl},
};

type BlobCell = Cell<u64, RuntimeMemory<DefaultMemoryImpl>>;
```

The integrating host owns one runtime/bootstrap for its backing memory, explicit
allocation grants, bucket profile and policy. Future blob stores must participate
in that host's committed allocation authority and open by stable key, alongside
IcyDB. A managed adapter must adopt Canic's runtime rather than bootstrap a second
manager or replace host policy with the generic default. A standalone adapter
must explicitly own bootstrap itself. Linking this crate registers no stores,
IDs, ranges or lifecycle hooks. No raw manager, stable-save path or inferred
allocation layout is introduced by this dependency change.

`ops::service::stores::grants::requests(authority)` now builds the sixteen current
service requests for explicit inclusion in the host's sealed declaration snapshot.
The host supplies their authority/range alongside configuration and other owners,
then bootstraps once. `grants::open` assembles the service mapping through its
committed lookup. `grants::open_default` checks the existing committed capability
before default key opens; it does not choose a
bucket profile, replace policy or construct an absent manager. Standalone uses
the same mapping with its own explicit runtime. The shared installation owner
persists the immutable configuration and assembles all four service owners;
standalone delegates these operations to it. Hosts still supply actual service and
compiled release, authenticate installation, own exclusive grants and call lifecycle
functions synchronously. The explicit Canic composition library now reuses these
operations; full endpoint acceptance and combined IcyDB application behavior
remain unqualified.

Managed `memory::open()` checks the host's committed runtime before opening
configuration and shared grants. `declare_memories!` supplies the explicit
authority requests; Canic owns their range and bootstrap validation. The lifecycle
macro retains its install/restore form. The prepared 0.15-only adoption API and
compact host-capacity endpoint are deferred and removed from the current contract.
No local copy of the framework's new verification or summary API is supplied.

The core and unpublished `ic-blob-storage-canic` composition library have no Canic
dependency. Its opt-in macros emit calls to the public facade in the owning
artifact, which must depend on Canic directly. This follows Canic 0.110.48's role
validation rather than adding a transitive facade dependency through the library.
The managed test must use one `ic-memory` package identity with allocation policy
owned by Canic; the published graph is aligned here.
Managed participants now use the already selected ic0 1.2.0 safe size/copy APIs
to bound their own argument buffer before copying. The native Candid declaration
guard uses official candid_parser 0.4.1, within the published Canic host's 0.4 range;
its parser/code-generation dependencies are native-only. Neither this addition
nor argument copying changes the allocator or published core's dependency graph.

Published Canic host validation requires resolver 2. Only the unpublished managed
fixture is an isolated resolver-2 workspace; the main workspace retains resolver
3. `make prepare-canic-probe` seeds its ignored lockfile from the repository lock,
then projects that graph with offline Cargo metadata. Subsequent fixture lint uses
`--offline --locked`. There is no separately maintained dependency baseline.

Native tests check the re-export's type compatibility with host handles, isolated
cells and unchanged host configuration, and that ordinary library use does not
bootstrap or declare memory. They do not prove blob schema transactions,
same-release interruption/restore or deployed Canic/IcyDB adapter behavior.
`ic-memory` governs allocation ownership; the service must still implement its
own intent/accounting/recovery invariants once the provider contract is settled.

## Setup and checks

With rustup and Cargo installed:

```sh
make deps
make check
make wasm-check
```

The toolchain file declares rustfmt, Clippy and `wasm32-unknown-unknown`.
`make deps` fetches the locked graph and may use the network. Normal validation
uses `--offline --locked` and this repository's `target/`. Updating a dependency
requires an intentional manifest/lockfile change; normal setup does not select
new versions. `make ci` remains a separately authorized full validation gate.

Testkit and PocketIC are excluded from the production/Wasm graph. Their Rust
libraries are fetched and compiled by the native check. The local canister test additionally needs a compatible
PocketIC server: this library accepts >=16.0.0,<17 and defaults to 16.0.0.
The checksum-verified Linux x86_64 server is installed locally at
`.tmp/tools/pocket-ic-16.0.0/pocket-ic`; its
[provenance record](evidence/pocketic-toolchain.json) includes archive and binary
hashes. The original record covers tool installation only; the later
[authority fixture evidence](evidence/core-primitives.md#pocketic-authority-probe-after-018)
records actual local canister execution.

Make exports that path as the default `POCKET_IC_BIN`, preventing automatic
server downloads during tests. A caller-supplied `POCKET_IC_BIN` overrides it.
The binary is ignored local tooling, so fresh checkouts need provisioning.
For Linux x86_64, from the repository root:

```sh
set -e
mkdir -p .tmp/tools/pocket-ic-16.0.0
curl --fail --location --output .tmp/tools/pocket-ic-16.0.0/pocket-ic.gz \
  https://github.com/dfinity/pocketic/releases/download/16.0.0/pocket-ic-x86_64-linux.gz
printf '%s\n' '268ba79ec7fe9a563a575adf4983c69627093cce2711d142e476cdc7ad04249e  .tmp/tools/pocket-ic-16.0.0/pocket-ic.gz' | sha256sum --check
gzip -dc .tmp/tools/pocket-ic-16.0.0/pocket-ic.gz > .tmp/tools/pocket-ic-16.0.0/pocket-ic
chmod u+x .tmp/tools/pocket-ic-16.0.0/pocket-ic
.tmp/tools/pocket-ic-16.0.0/pocket-ic --version
```

Other platforms must use the corresponding official 16.0.0 release asset and
verify its published digest before setting `POCKET_IC_BIN`. `make deps` fetches
Cargo packages only; it does not provision this binary.

Managed fixture builds additionally require the published `canic` CLI 0.110.48,
`ic-wasm` 0.11.1 and Binaryen `wasm-opt` 132 on PATH. They are already installed in
this workspace. For a fresh checkout, provision these tools explicitly; the test
target downloads nothing. `make build-canic-probe` runs Canic's declaration/runtime
validation and artifact finalization with offline Cargo, retaining this repository's
target directory. It suppresses Canic's automatic sccache selection unless the
caller explicitly sets `RUSTC_WRAPPER`. `make test-canic-composition` builds that
artifact, native `blob-storage` CLI, existing query-only Cashier substitute and
existing consumer/outbox probe,
then runs managed PocketIC and signed subprocess journeys. The normal PocketIC suite includes them;
the independent standalone build does not need the Canic CLI. The fixture uses
Canic's public qualification helper, which owns its local instance, rather than
the harness's ordinary explicit server handle.
Signed managed tests pin the application-only instance's subnet key through its
local control API and use a supported non-owning PocketIC handle for a literal
loopback HTTP gateway. Signature checks and CLI origin restrictions stay enabled;
this test trust arrangement does not establish production Fleet provenance.
Managed and durable native verifier cases share the same local HTTP source/fault
proxy. To retain managed verifier records, set `BLOB_MANAGED_VERIFIER_REPORT` to
an existing empty parent directory before running the focused test target. Each
reply mode creates a new child and refuses an existing or partial child;
routine runs use temporary directories. Reports contain labelled local fixture
data and public trust keys, not a deployment identity or provider certificate.
`BLOB_MANAGED_REFERENCE_REPORT` provides the same fresh-child capture behavior for
signed tenant retain/release, original receipts, current liveness and cleanup
journeys beside trusted completion. Use an existing empty parent; normal tests
use temporary files. Reports retain exact signed requests, public trust and
labelled local source bytes; test PEM identities remain temporary. There is no
new runtime/registry dependency or production consumer journal.
`BLOB_STANDALONE_REFERENCE_REPORT` retains fresh per-mode signed refusal/recovery
captures against the actual standalone artifact. `BLOB_MANAGED_CONSUMER_REPORT`
retains fresh `outbox`, `cancel-race`, `retain-consumer-first`,
`retain-service-first`, `release-consumer-first` and `release-service-first`
children with exact application/service Candid and labelled local completion.
The interruption children preserve unresolved outbox histories across both upgrade
orders and repeated same-release restoration. Both variables require existing parents and refuse
retained/partial children; ordinary runs use temporary directories. The consumer
is the existing local application substitute using shared clients and its own
bounded durable record, not a new production dependency or recovery component.

`BLOB_ACCOUNT_REPORT` retains fresh `standalone` and `managed` children for signed
native balance/relationship journeys. Use an existing empty parent; existing or
partial children refuse. Reports retain request/source Candid, raw command/result
JSON and public root trust; identities remain temporary. The source is the same
local query-only Cashier substitute, with no payment or deployed provider request.

`BLOB_GATEWAY_CONTROL_REPORT` retains fresh `standalone` and `managed` children
for the shared signed native gateway journey. Use an existing empty parent;
retained/partial children refuse. Plans, public root, canonical/signed requests,
intent/outcome/response records and explicit signed status captures remain; PEM
identities are temporary. The existing fault proxy verifies saved intent before
forwarding each update, then passes, drops or substitutes pending admission.
All gateway source modes are labelled local query-only substitutes.

`BLOB_FUNDING_ASSESSMENT_REPORT` retains fresh `standalone` and `managed` children
for signed passive funding diagnosis, refusal and restore journeys. Use an existing
empty parent; retained/partial children refuse. Plans, command/result JSON and
public root keys remain; PEM identities are temporary. The Cashier is stopped
throughout these queries, with no provider requests, payment or cycle attachment.
No dependency or allocator changes accompany this endpoint/tooling batch.

`BLOB_TENANT_DOWNLOAD_REPORT` retains fresh `managed` and `standalone` children
for signed tenant file downloads and standalone unconfirmed/restored refusals.
Use an existing empty parent; retained or partial children refuse reuse. Plans,
exact descriptor requests/replies, public trust, command/result JSON, raw local
source exchanges and verified/partial bytes remain. Test PEMs are temporary.
Managed completion uses the existing labelled exposure/content substitute; these
reports establish no deployed provider behavior or paid-effect authority.

`BLOB_UPLOAD_SETUP_REPORT` retains fresh `standalone`/`managed` children for native
admission, preparation, exact recovery and cancellation. Plans/public roots,
command/result JSON, canonical and signed requests, intent/outcome/reply files and
original manifest history remain; offline-generated permission/manifest files,
verified body snapshots and subsequent signed local-file checks are also retained.
The original source is intentionally changed after snapshot creation. Manifest JSON
uses the local Rust fixture's upstream format, rather than fresh upstream SDK or
deployed provider evidence. Temporary PEMs are dropped. Existing/partial
children refuse. The fault proxy forwards each real update once before dropping
or replacing its acknowledgment; it neither supplies provider evidence nor exposes
or confirms uploads. No new dependency or allocator is required.

`make test-pocketic` builds `blob-authority-probe`, `blob-gateway-source` and
`blob-funding-probe` into
this repository's Wasm release target, then runs the unpublished host harness.
The gateway source deliberately calls back before returning its old list, so
race tests rely on actual inter-canister calls rather than sleeps or tick counts.
The funding probe runs as a sender and controlled receiver to measure real cycle
acceptance, refunds and callback rollback, with host-owned ic-memory journals for
same-release upgrade recovery; it does not implement a Cashier service.
The shared `tests/protocol` package owns passive fixture controls and typed outcomes;
it is not a production service or provider interface. Testkit starts the exact
`POCKET_IC_BIN` with a bounded startup deadline and an owned server handle; the
handle shuts the child down after the instance is dropped. No binary downloader
or provider transport is used. A sandbox must permit local loopback binding for
the server. `make test-native` requires no server; `make test` runs both suites
sequentially. The fixture owns sample transient state and exports only test
endpoints; fixtures, protocol and harness are excluded from the published library.
The probe and host harness also reuse the existing serde/JSON packages to read
the pinned content vectors. `make test-pocketic RUST_TEST_NOCAPTURE=1` prints the
measured Wasm ordered-append instruction observations as well as test results.

The existing core has no Canic dependency. Keep Canic confined to the owning
managed artifact and its native qualification harness. The
[Caffeine baseline](provider-baseline.json) remains the upstream integration
reference. No npm/Motoko package is installed into this Rust-only scaffold;
browser dependencies belong to a concrete browser client or compatibility
harness when one is added.
