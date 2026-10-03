# Dependency setup

The root `Cargo.toml` owns direct dependency version requirements. The library
inherits them, and `Cargo.lock` locks the resolved graph. Versions were checked against
crates.io on 2026-09-25. On 2026-09-26, `serde_json` became a direct dependency
at its existing locked version for bounded provider reply parsing. The table reflects
the current lockfile on 2026-10-03, including concurrent dependency updates;
availability does not establish provider qualification or service readiness.

| Dependency | Version | Purpose |
| --- | --- | --- |
| `candid` | 0.10.37 (locked) | IC boundary encoding and principal types |
| `serde` | 1.0.229 | Serialization derives for explicit boundary/record schemas |
| `serde_json` | 1.0.151 | Bounded Caffeine chunk-status JSON decoding; reused the existing lockfile version |
| `sha2` | 0.11.0 | SHA-256; optional allocation/OID features disabled |
| `thiserror` | 2.0.18 | Typed error derives; matches PocketIC's exact requirement |
| `ic-cdk` | 0.20.3 | IC platform operations for the ops layer |
| `ic-memory` | 0.20.0 (locked) | Sole allocation runtime; public typed growth API |
| `ic-stable-structures` | 0.7.2 | Exact transitive substrate owned/re-exported by `ic-memory` |
| `ic-testkit` | 0.13.0 (locked) | Native dependency of the unpublished PocketIC harness; shared helpers and full re-export |
| `pocket-ic` | 16.0.0 | Transitive through `ic-testkit`; no direct dependency |
| `ic-agent` | 0.49.2 | Native CLI and harness signing and verification of ingress certificates |
| `candid_parser` | 0.4.1 | Native harness only; official Candid parser for native request fixtures |

Headless ingress tests add pinned `ic-agent` 0.49.2 (default features disabled),
plus the locked `reqwest` 0.13.5, `tokio` 1.53.2 and `serde_cbor` 0.11.2
as native dev dependencies in `tests/pocketic`. The public crates.io index confirmed
0.49.2 as the latest non-yanked agent release on 2026-09-28. Its cryptographic graph
adds host-only lockfile entries without changing existing package versions, the
published library dependency graph, Wasm allocation or memory grants. Tests trust
the explicitly owned PocketIC NNS key, never a root key fetched from mainnet, and
send no request to a Caffeine gateway. This is headless Rust evidence, not browser
or production uploader qualification.

The native CLI explicitly enables reqwest 0.13.5's `rustls` and `http2` features;
CLI-only builds must support HTTPS/HTTP/2 independently of the harness or agent's
transitive feature choices. Platform certificate validation and no-retry settings
remain enabled. The newly reached h2 0.4.19 and hyper 1.11.1 already exist in the
lockfile and declare Rust 1.63, below the maintained CLI minimum; no version or
MSRV is changed. Current development-toolchain checks do not rerun the minimum
compiler evidence.

At the initial registry check, the selected releases were current except `thiserror`, where PocketIC 16 pins
2.0.18 and prevents selecting 2.0.21 in this graph. Development builds use the
repository's Rust 1.99.0 toolchain. SHA-256 byte hashing does not by
itself implement or qualify Caffeine's provider-specific hash tree.

## Rust versions

The development toolchain and minimum supported Rust version (MSRV) serve
different purposes:

| Scope | Rust version | Reason |
| --- | --- | --- |
| Development, formatting and lint | 1.99.0 | Pinned in `rust-toolchain.toml` |
| Library, standalone, CLI and examples | 1.88.0 | Inherited from `workspace.package.rust-version` |
| Unpublished PocketIC harness | 1.88.0 | Workspace MSRV; fixture-only file locking removed |

The locked dependencies require at most Rust 1.88.0, including `ic-cdk`,
`ic-memory`, `ic-agent` and `ic-testkit`. The maintained content codecs also use
`slice::as_chunks`, stabilized in 1.88. The fixture-only file-lock journal is
retired, so the harness inherits the workspace floor. Earlier release checks
used the actual older compilers; repeat them for changed source or dependency
resolutions rather than treating dependency metadata as execution evidence.

To repeat the locked native and Wasm compatibility checks after explicitly
provisioning Rust 1.88.0 and its `wasm32-unknown-unknown` target:

```sh
cargo +1.88.0 check --offline --locked --workspace --all-targets --all-features
cargo +1.88.0 check --offline --locked --workspace --all-features --target wasm32-unknown-unknown
```

Keep supported source and dependencies within their declared floor when updating
the development toolchain. These checks cover this lockfile; a consumer's own
dependency resolution must also satisfy its Rust version.

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
Playwright's Chromium 153.0.8010.12 (revision 1243). Upload fixtures also need
`openssl` on PATH to create temporary loopback TLS keys. Chromium trusts only the
generated certificate's public-key pin; unrelated TLS validation is unchanged.
Setup is explicit:

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
The installed package is not modified. Application authentication and selection of
the persistent browser environment are caller-owned; the source package supplies
the bounded IndexedDB journal. Its independent `make test-browser-store` target
requires the same provisioned Node/packages/Chromium but no Rust build or PocketIC;
it tests explicit create/reopen and a graceful browser-process restart with an
isolated retained profile under `.tmp/`.
See the [client contract](../clients/browser/README.md).

`make test-browser-launcher BLOB_LAUNCHER_REPORT=NEW_DIRECTORY` checks the maintained
native Chromium bridge with owned loopback assets and an explicitly retained
profile. It covers bounded selected-body reads, FIFO/symlink refusals, fixed-origin
reopening, missing-history refusal, port contention and deadlines without IC or
provider traffic. It uses the same provisioned Node/Playwright/Chromium; the private
client accepts the caller's Chromium engine and adds nothing to Cargo's graph.
`build.mjs` emits maintained `publication-host.js` / `publication-worker.js` plus
the adapted native serial evidence driver. Its Chromium-only `/host.js` import
remains external to the native bundle; executable SDK peer/source/patch checks
still run before every build. See the [process bridge contract](../clients/browser/README.md#launch-chromium-from-a-native-parent).

`make test-browser-publication BLOB_PUBLICATION_REPORT=NEW_DIRECTORY` runs the
frozen-file helper's offline body/manifest/metadata/root/abort refusals and input
snapshot checks. It requires the pinned packages and Node, without Chromium,
Rust builds or network traffic. The refusing store/transport are explicit test
substitutes; the standalone browser target supplies actual IC/IndexedDB evidence.

`make test-browser-transport BLOB_BROWSER_TRANSPORT_REPORT=NEW_DIRECTORY` runs
owned TLS HTTP/1.1 and HTTP/2 replay checks without Rust/PocketIC builds. Each
case has a fresh context and one maintained journal claim. Buffered/XHR controls
intentionally replace the outgoing stream to reproduce replay; production uses
only the stream, refuses HTTP/1.x and preserves uncertainty after connection loss.
Request instrumentation observes rather than intercepts upload bodies. Other cuts
can be selected directly with `node tests/browser/transport.mjs NEW_DIRECTORY
refused-stream` or `data-close`. These are local transport observations, not
deployed gateway, billing or universal browser guarantees.

Rust owns the PocketIC installation and test identity. Browser traffic is confined
to that local IC endpoint and the owned page/gateway substitute. The maintained IndexedDB store has two lifetime
slots and no reset/eviction path; it tests transaction ordering, competing tabs,
reload, cancellation and verified historical replies. Caffeine performs real HTTP
tree/chunk requests only against the local substitute. Failed/aborted transfer does
not retry. This is not production sizing, power-loss/eviction durability, rollback recovery, a deployed gateway upload,
or qualification of a consumer's CSP/authentication/storage environment.

### Complete local standalone rehearsal

`make test-browser-standalone` uses the same provisioned browser tools and PocketIC.
It builds the standalone Wasm/CLI and explicitly runs four ignored cases against a
local gateway substitute. Actual installed host facts issue the certificate; no
operator-supplied provider flags or consumer framework are involved. One-slot
IndexedDB survives reload, while native tools verify uploaded bytes, submit the
distinct verifier's statement and download as the tenant. HTTP upload success
alone cannot serve an asset. Release retains physical bytes and liabilities.
A separate fresh owner receives a corrupt GET, records failure without attestation,
and retains exposure after withdrawal/cancellation. The interruption cases lose
reply headers, verify without a new upload and preserve release through late
completion/replay. One TLS HTTP/2 origin receives uploads and serves those same
bytes to both native readers. The fixture generates a CA and leaf certificate;
on Linux, each native child uses only that public CA through SSL_CERT_FILE with
normal validation. Unrelated roots refuse before an HTTP GET; a REFUSED_STREAM
read fails without redispatch, before a separately budgeted complete observation.
No process-global trust or certificate-validation bypass is introduced.

To retain exact commands and artifacts, set `BLOB_STANDALONE_BROWSER_REPORT` to a
fresh **existing parent directory**; the tests create `success/`, `corrupt/`,
`lost-final/` and `withdrawn/`
and refuse existing child directories. Otherwise they use temporary directories.
The capture contains fixed test keys and signed local requests; these identities
are never deployment identities. The ordinary Rust/CI suite does not require this
opt-in browser target, and no deployed provider/account request occurs.

### Offline native/browser handoff

With the same local packages and Node 24, run the optional preparation check:

```sh
make test-sdk-inputs BLOB_BROWSER_NODE=/absolute/path/to/node \
  BLOB_SDK_INPUTS_REPORT=/tmp/new-browser-inputs-evidence
```

It bundles the hash-checked pinned SDK/patch, builds the native CLI offline and
prepares a 10 MiB file. A passive local configuration fixture feeds
`installation-check`; its exact complete carrier and actual SDK output feed
`upload-inputs`. The generated
`certificate-binding.json` is consumed by the existing browser source client in
Node with an in-memory setup-only store. Snapshot repreparation, no-clobber repeat
and corrupt-source refusal retain exact output/logs in the new directory. All
network calls throw; no certificate issue/recovery or gateway upload occurs.
This check needs neither Chromium nor PocketIC and remains outside ordinary CI.
It establishes no production storage durability, service authorization or provider
guarantee. Existing evidence directories refuse reuse; build artifacts are kept.

For the proposed first standalone trial, set `BLOB_SDK_INPUTS_BYTES=1024` on this
same offline target. The optional size accepts canonical positive byte counts up
to 10 MiB and rejects invalid input before creating output. The default remains
10 MiB. Both envelopes exercise the same SDK/native/snapshot/refusal checks;
neither emits a certificate or sends service/gateway traffic. See the
[trial review](standalone-trial.md) for the accepted operating contract and open inputs.

## Memory composition

Released 0.9.0 selects `ic-memory 0.15.4`; the current working tree selects 0.20.0
through the maintainer's concurrent dependency update. The lockfile resolves one registry
package and its `ic-stable-structures` 0.7.2 substrate.
Direct `RuntimeMemory::grow` returns a typed result;
generic `Memory` wrappers preserve the upstream -1 sentinel contract.

The published 0.15.2 changelog reports addressing IcyDB feedback in
[#8](https://github.com/dragginzgame/ic-memory/issues/8): narrow raw-read and
registration-hook lint exceptions become justified expectations, unsupported-format
tests compare typed diagnostics, and Wasm declaration-count tests avoid overflow.
The clean local upstream release `e2fe658` and cached registry package agree on
the changelog. Source comparison with 0.15.1 preserves raw-read forwarding and its
tests; it introduces no new consumer read API, memory format or allocator choice.
This repository defines no unsafe read override to change. Upstream retains its
declared Rust 1.88 minimum. The GitHub issue-body fetch fails, so this review does
not independently summarize that body or claim its current resolution status.
Targeted current core/CLI/host lint, native installation and actual local IC
installation/growth-refusal/retry/restoration checks pass with 0.15.2. Retained
evidence is in `.tmp/installation-carrier-01`; older MSRV captures remain historical.

Use `ic_blob_storage::ic_memory` for storage types. The host owns one runtime,
its allocation policy and explicit grants. Use
`ops::service::installation::requests(authority)` for the complete seventeen-key
inventory and `LIBRARY_VERSION` for the compiled library contract. Neither helper
registers declarations, selects physical IDs or bootstraps memory. The installation needs its configuration
key and sixteen shared service requests; linking the library registers nothing.
`grants::open` adopts the host's committed lookup; `open_default` requires an
already committed default runtime. Standalone owns its own explicit bootstrap.
Restoration is synchronous and leaves every service owner fenced.

Consumer frameworks own their wrappers and integration tests externally. This
repository has no downstream framework dependency, managed adapter or managed
fixture, and its validation requires no framework CLI. Host integration must
use one memory package identity; this library does not qualify a consumer's
composition or permit a second manager over the same backing memory.

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

The library and its tests have no downstream framework dependency. The
[Caffeine baseline](provider-baseline.json) remains the upstream integration
reference. No npm/Motoko package enters the Rust/Wasm dependency graph;
browser peers belong to the maintained private client and its concrete evidence
harness.
