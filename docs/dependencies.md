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
| `ic-stable-structures` | 0.7.2 | Stable-memory storage primitives |
| `ic-testkit` | 0.10.0 | Native dev dependency of the unpublished PocketIC harness; shared helpers and full re-export |
| `pocket-ic` | 16.0.0 | Transitive through `ic-testkit`; no direct dependency |

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

`make test-pocketic` builds `blob-authority-probe` and `blob-gateway-source` into
this repository's Wasm release target, then runs the unpublished host harness.
The latter canister deliberately calls back before returning its old list, so
race tests rely on actual inter-canister calls rather than sleeps or tick counts.
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
