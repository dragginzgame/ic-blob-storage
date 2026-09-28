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
| `ic-memory` | 0.14.3 | Allocation governance; direct dependency aligned with Canic and IcyDB |
| `ic-stable-structures` | 0.7.2 | Exact transitive substrate owned/re-exported by `ic-memory` |
| `ic-testkit` | 0.10.1 | Native dependency of the unpublished PocketIC harness; shared helpers and full re-export |
| `pocket-ic` | 16.0.0 | Transitive through `ic-testkit`; no direct dependency |
| `ic-agent` | 0.49.2 | Native test-only signing and verification of local ingress certificates |

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

## Memory composition with Canic and IcyDB

The 0.1.12 work uses the published `ic-memory` 0.14.3 package already selected by
Canic 0.110.42 and IcyDB 0.261.11. A combined dependency-resolution check of this
crate, `canic-core` and `icydb-core` resolves one `ic-memory` 0.14.3 and one
`ic-stable-structures` 0.7.2. Those two framework crates are not dependencies of
the blob core; the combined graph is a compatibility check, not an adapter build.
The lockfile retains existing versions and adds ic-memory's missing dependencies.

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
must explicitly own bootstrap itself. This crate currently declares no stores,
IDs, ranges or lifecycle hooks. No raw manager, stable-save path or inferred
allocation layout is introduced by this dependency change.

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

The existing core has no Canic dependency. Move boundary dependencies into
their owning protocol/client/adapter packages when that split is implemented;
keep Canic confined to the managed adapter. The
[Caffeine baseline](provider-baseline.json) remains the upstream integration
reference. No npm/Motoko package is installed into this Rust-only scaffold;
browser dependencies belong to a concrete browser client or compatibility
harness when one is added.
