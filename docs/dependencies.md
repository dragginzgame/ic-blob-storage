<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Dependency setup

## Quick setup

For ordinary development from the repository root:

1. Install `rustup`; the checked-in `rust-toolchain.toml` selects the maintained
   development compiler and required components.
2. Install the bootstrap packages described in [local setup](local-setup.md),
   including Perl's Digest::SHA, curl, archive utilities and xz. Run
   `make install-tools` to provision checksum-pinned jq/yq and the common IC
   executables under `.tools/`, then `make tools-check` to verify them offline.
3. Run `make deps` to fetch the exact locked Rust dependencies.
4. Prepare the manifest formatter with `cargo install cargo-sort --version 2.1.4
   --locked`, then run `make install-hooks` once per clone (also after updating
   developer setup). The reviewed hook uses `make fmt`; CI and release validation
   independently use `make fmt-check`. Both sort all twelve manifests and format
   Rust, without fetching dependencies, compiling or cleaning artifacts.
5. Run `make test-native` for native development checks, or
   `make test-native-host` for CLI boundaries and an actual PocketIC installation.

Keep `$HOME/.cargo/bin` and `$HOME/.local/bin` on PATH when using user-local tools.
Release shell checks require ShellCheck; `SHELLCHECK=/absolute/path` may select
an already prepared binary. Install it with `sudo apt-get install shellcheck` on
Debian/Ubuntu or `brew install shellcheck` on macOS. The common local executable
setup covers jq/yq and the IC tools; ShellCheck and the Rust formatter are separate
prerequisites. Missing prerequisites fail rather than being installed
during a hook or validation. Cargo-sort's exact reviewed version is recorded in
[tool versions](../ci/tool-versions.env). That file also owns jq/yq selections;
[the IC matrix](../ci/ic-tools.tsv) owns executable versions and archive digests.
Make prepends `.tools/host/bin` and `.tools/ic/bin` to PATH. Missing local tools
fail validation; provisioning is explicit and separate from locked Cargo fetching.
This consumer uses the shared host-tool bundle for jq/yq; the optional upstream
standalone yq installer is not included in its snapshot.
`make release-tools-check` verifies ShellCheck availability and the reviewed
cargo-sort version plus working rustfmt without compiling. The shared
`check-format-tools.sh` performs both formatter probes offline, using the same
consumer pin as setup and CI. Both `fmt` and `fmt-check` depend on this check.
Release preflight runs the same check
before entering full validation. It gives setup commands and supports the
explicit `SHELLCHECK` selection, including executable paths containing spaces.

Browser and standalone rehearsals additionally require PocketIC, Node, browser
packages and Chromium. Follow [setup and checks](#setup-and-checks) for those
paths. Consumers embedding the library should also read
[memory composition](#memory-composition) before assigning stable memory.

## Dependency selector checks

`make dependency-pins-check` checks declarations and tracked workspace lockfiles
without downloading, upgrading or changing them. It runs in the complete gate
and the Linux/macOS tooling workflow. Prepare Git and run `make install-host-tools`
before invoking it. `make host-tools-check` verifies the selected local jq 1.8.2
and Mike Farah yq 4.47.2 without downloading.
The [shared pinning rules](../rules/dependency-pinning.md) define checked inputs
and scoped exceptions. This repository currently needs no pinning exception.

The common installer verifies downloaded bytes before execution and checks exact
versions before activating the complete local tool set. Interrupted or refused
installation preserves the prior active set and its failed preparation evidence.
Validation never installs missing prerequisites.

Registry requirements use compatible ranges. The maintained lockfile continues
to select candid_parser 0.4.1, ic-agent 0.49.2, sha2 0.11.0 and thiserror 2.0.18.
PocketIC 16's own exact thiserror requirement remains part of the locked native
test graph; duplicating it in the library catalog is unnecessary. Shared Tooling
adoption itself does not reselect dependencies. Published library consumers
resolve their own compatible graphs, so the repository's tests do not qualify
every future release.

## Locked dependency inventory

The root `Cargo.toml` owns all package and direct dependency version requirements
and local dependency paths. Every member, including unpublished fixtures, inherits
its package version and dependencies from the workspace. Members select features
and target conditions; `Cargo.lock` locks the resolved graph. Versions were checked against
crates.io on 2026-09-25. On 2026-09-26, `serde_json` became a direct dependency
at its existing locked version for bounded provider reply parsing. The table reflects
the maintainer-selected released 0.15.2 graph on 2026-10-06;
availability does not establish provider qualification or service readiness.

| Dependency | Version | Purpose |
| --- | --- | --- |
| `candid` | 0.10.37 (locked) | IC boundary encoding and principal types |
| `serde` | 1.0.229 | Serialization derives for explicit boundary/record schemas |
| `serde_json` | 1.0.151 | Bounded Caffeine chunk-status JSON decoding; reused the existing lockfile version |
| `sha2` | 0.11.0 | SHA-256; optional allocation/OID features disabled |
| `thiserror` | 2.0.18 (locked) | Typed error derives; PocketIC constrains its own requirement exactly |
| `ic-cdk` | 0.20.3 | IC platform operations for the ops layer |
| `ic-management-canister-types` | 0.11.0 (direct, locked) | Bounded current-instance IC-history request/reply types |
| `ic-memory` | 0.30.0 (locked) | Sole allocation runtime; current ownership ledger and public typed growth API |
| `ic-metrics` | 0.2.5 (locked) | Private storage resource probe only; allocation-free measurement arithmetic, no platform reader |
| `ic-stable-structures` | 0.7.2 | Exact transitive substrate owned/re-exported by `ic-memory` |
| `ic-testkit` | 0.19.2 (locked) | Native dependency of the unpublished PocketIC harness; shared helpers and full re-export |
| `pocket-ic` | 16.0.0 | Transitive through `ic-testkit`; no direct dependency |
| `ic-agent` | 0.49.2 | Native CLI and harness signing and verification of ingress certificates |
| `ic-host-artifacts` | 0.3.1 (direct, locked) | Native CLI raw SHA-256 identities and example-only bounded JSON streams; optional archive/gzip/Wasm features disabled |
| `ic-host-fs` | 0.3.1 (direct, locked) | Native CLI bounded descriptor/no-follow reads and private durable create-new records |
| `ic-host-tools` | 0.1.14 (harness transitive) | Owned by ic-testkit; no direct CLI facade dependency |
| `candid_parser` | 0.4.1 | Native harness only; official Candid parser for native request fixtures |

### Memory 0.30 and example stream collection — pending 0.16.0

The task-entry lock selects published Memory 0.30.0 and private-probe metrics
0.2.5. Memory's registry source matches publisher `2fdeec4`; every runtime source
file and runtime dependency declaration matches 0.28.4. The public re-export,
grant requests and runtime handles nevertheless change Rust crate identity.
Hosts must use Memory 0.30 in one composed graph. A private compile probe accepts
that identity and rejects assigning these requests to 0.28-owned types. This
requires a minor library release, without introducing a new durable schema,
endpoint owner or lifecycle adapter. Cross-release reinstall and retirement
rules still apply. [#20](https://github.com/dragginzgame/ic-blob-storage/issues/20)
retains publication and downstream managed qualification with their owners.

Native examples inherit ic-host-artifacts only as a target-specific development
dependency. Its `read_reader` owns bounded collection and interrupted-read retry;
each caller keeps `File::open`, JSON schema and limit diagnostics. Inventory
paths still resolve from the supplied file's directory, including file symlinks.
I/O errors keep their original type; allocation failures are reported without
infallible vector growth. Production/native-library and Wasm runtime graphs do
not acquire this edge. [#21](https://github.com/dragginzgame/ic-blob-storage/issues/21)
tracks this adoption. Metrics 0.2.5 runtime source matches 0.2.4; earlier immutable
records keep their original locks and artifact identities.

### Initial ic-metrics adoption for the earlier 0.15.4 candidate

The initial private storage resource probe adoption used published ic-metrics 0.2.4 through an
inherited `0.2` registry requirement. `record_sample` owns completed-read count
and instruction-total arithmetic; requested bytes remain a separate unit. The
host still reads IC call-context counter 1, selects each logical memory and
starts/stops the synchronous reopen window. The counter retains three u64 fields
and the existing Candid/report shape. Counts and totals saturate independently;
saturated diagnostic values are unavailable for exact interval arithmetic.

The dependency has no features, runtime reads, allocations or Cargo dependencies.
It is absent from the published core and production standalone graphs. Service
balances, quota/capacity accounting and durable journals retain their exact
arithmetic; their obligations are not measurement summaries. No metrics endpoint
or registry is added. The [initial source-bound record](evidence/metrics-0154.json)
retains its 0.2.3 graph and checks. The later
[workspace adoption record](evidence/shared-014-0154.json) binds the already-selected
0.2.4 lock and fresh local PocketIC window/installation/recovery checks; it does
not relabel the earlier evidence.
[#19](https://github.com/dragginzgame/ic-blob-storage/issues/19) owns committed
consumer/native qualification. No instruction or cycle savings are claimed.

### Released 0.15.3 host crate split

The earlier 0.15.4 candidate lock refresh selected published artifacts/fs 0.3.1. Their
published sources match committed host release `38a2a51`; the existing CLI
callers retain bounded reads, native-only placement and private create-new
publication. The refactored writer and shorter staging names stay under that
shared owner. New gzip, path, lock-wait and streaming replacement APIs are not
introduced into unmatched provider/probe flows. The
[refresh record](evidence/refresh-0154-02.json) binds the selected graph and
focused Linux qualification; pending-source native macOS checks remain required.
The following original adoption evidence keeps its 0.3.0 bindings.

The maintainer-selected direct ic-host-tools 0.3.0 removed the old artifact
facade. Its four published host crates were reviewed against cached registry
0.3.0 source and the sibling's committed extraction provenance. Native callers
now depend directly on ic-host-artifacts and ic-host-fs. This removes the unused
direct ic-host-tools 0.3.0 and ic-host-process 0.3.0 packages from the lock;
ic-testkit's older transitive owner remains unchanged. Native-only dependency
placement keeps these crates out of production Wasm.

Run records use the shared private atomic create-new writer after checking the
already-claimed directory. Existing destinations and directory substitutions
refuse; records publish complete bytes with file/directory synchronization.
A sync failure after publication remains a failure, without automatic retry.
Private body.part output, EOF/provider-root verification, create-new body
publication and interruption evidence remain local. The shared writer owns
its staging cleanup; a process interruption can retain staging inside the
claimed run, which cannot be reopened as a fresh run. Trusted ancestors and
exclusion of concurrent directory writers remain caller responsibilities.
Probe record writes remain local because their exact failure stages and
interruption inventory are part of the retained probe contract.

The two remaining 0.3 crates do not replace a current matching abstraction:
ic-host-process admits bounded one-shot tools with null stdin; the browser
driver exchanges continuing messages. ic-host-tools owns explicit ICP CLI
response decoding and Candid extraction; this CLI uses ic-agent and the service's
bounded Candid decoder. Neither crate owns provider authority, durable service
journals or the provider-root verifier. No unused direct dependency or new
execution wrapper is introduced. Dirty sibling 0.3.1 APIs are excluded.

The [focused adoption record](evidence/host-owners-0153.json) binds the actual
selected graph, including the pre-existing ic-memory 0.28.4, h2 0.4.20 and
hyper 1.12.0 selections. CLI/probe, FIFO, Clippy and Rust 1.88 checks pass on
Linux; retained probes and checksums verify read-only. These checks qualify
native callers, not new service deployment or full core/PocketIC behavior.
Earlier qualification retains its original graphs. The released 0.15.3 native
Linux and both macOS jobs now pass. That result does not qualify the pending
0.15.4 graph.

The maintainer-selected ic-memory 0.27 upgrade removes history/timestamp APIs
and changes the durable ledger layout exposed through the public re-export.
Released 0.15.0 contains that hard cut, carrying forward the
tooling work originally drafted for 0.14.13. Hosts must update affected callers
and fixtures; retained installations require obligation disposition and
reinstall, never a ledger reset or compatibility reader. See the
[persisted contract](service-contract.md#current-persisted-boundaries) and
[new validation record](evidence/release-preflight-0150.json). Earlier evidence
continues to qualify only its original selected graph.

The original 0.15.1 adoption batch added published ic-host-tools 0.1.11 to the
native CLI only; the maintainer's released lock selects 0.1.12. It delegates
bounded reads from the locally selected descriptor
and raw lowercase SHA-256 formatting. Native inputs retain link following,
nonblocking special-file refusal and empty-input refusal. Probe records retain
their error codes and reject final-component links on Unix, including at open.
This is not path confinement or a content snapshot; ancestors may follow links
and concurrent writers remain possible. Caffeine content trees, credentials,
recovery journals and service ownership remain local.

The [adoption record](evidence/tooling-host-0151.json) binds its original registry
source, locked graph and local checks. Four package identities were added: ic-host-tools
0.1.11, tar 0.4.46, filetime 0.2.29 and wasmparser 0.253.0. No existing locked
package version was reselected in that recorded batch. It does not qualify later
dependency updates or the released graph by itself. The production core and
Wasm graph do not acquire
this host dependency; the workspace's minimum Rust version remains 1.88.0.

The [0.15.2 native qualification record](evidence/caffeine-probes/local/2026-10-06-native-download-0152-01/summary.json)
binds ic-memory 0.28.2, ic-host-tools 0.1.14 and ic-testkit 0.19.1 to their actual
cached sources, selected lock, matching artifacts and focused Linux checks.
ic-memory's published runtime source is unchanged from 0.28.0. ic-host-tools
adds typed rejection of impossible reader/writer byte counts. ic-testkit 0.19's
removed executable resolver and changed bounded Wasm reader have no callers
here; the used harness APIs compile and execute. Its breaking APIs are confined
to the unpublished harness, so the pending library patch remains compatible.
Native CLI, actual installation/funding/restore, strict relevant Clippy and
Rust 1.88 native/Wasm checks pass without lock reselection. The new native macOS
run remains required; Linux substitutes do not qualify the supported Apple hosts.

The later [release-guard and host record](evidence/release-guards-0152.json)
separately qualifies the task-entry ic-testkit 0.19.2 selection. Its published
runtime source matches 0.19.1; the patch changes developer/release tooling.
All 106 CLI and two actual installation cases, harness Clippy and Rust 1.88
native harness compilation pass. The lock stays unchanged during this batch;
earlier funding/restore and Wasm records retain their original source and graph.

Headless ingress tests add pinned `ic-agent` 0.49.2 (default features disabled),
plus the locked `reqwest` 0.13.5, `tokio` 1.53.2 and `serde_cbor` 0.11.2
as native dev dependencies in `crates/ic-blob-storage-pocketic-tests`. The public crates.io index confirmed
0.49.2 as the latest non-yanked agent release on 2026-09-28. Its cryptographic graph
adds host-only lockfile entries without changing existing package versions, the
published library dependency graph, Wasm allocation or memory grants. Tests trust
the explicitly owned PocketIC NNS key, never a root key fetched from mainnet, and
send no request to a Caffeine gateway. This is headless Rust evidence, not browser
or production uploader qualification.

The retained 2026-10-04 management-types checks used the then-direct 0.10.0.
The [2026-10-05 upgrade record](evidence/caffeine-probes/README.md#management-canister-types-0110--2026-10-05)
separately qualifies the direct 0.11.0 graph with ic-memory 0.25.5 and ic-testkit
0.15.4. Three continuity unit tests, strict scoped Clippy, matching Wasm/harness
builds and five actual PocketIC recovery cases pass on that frozen graph.
PocketIC 16 still brings its own native management-types 0.8.0 transitively;
it is separate from the core's direct 0.11.0 and is not a second memory runtime.
These records retain their actual dependency and artifact identities; they do
not qualify a later lockfile change or replace full release or MSRV validation.

The selected lock also retains the maintainer's powerfmt 0.2.0-to-0.2.1 update.
Its declared Rust floor rises from 1.67.0 to 1.79.0, below this workspace's 1.88.0
MSRV. Source review finds formatting-buffer implementation and inlining changes
without changing the selected public API; optional macros are not selected by
this graph. Current graph checks are recorded in the release handoff rather
than attributed to the earlier recovery artifacts.

The [0.14.10 batch preflight](evidence/release-preflight-01410.json) passes the
complete configured `make ci` gate and actual native/Wasm checks with Rust
1.88.0 on its captured 0.25.5/0.15.4 graph, still compiled as 0.14.9. These Linux results
preserve the source patch, artifacts and logs; release preparation from clean
committed source remains a separate transaction. Native macOS and opt-in browser
or scale profiles are not rerun by this gate.

The [0.14.11 occupied-history record](evidence/caffeine-probes/README.md#larger-occupied-fundingread-histories--2026-10-05)
separately qualifies scoped local restoration and recovery on selected ic-memory
0.25.9/ic-testkit 0.15.8, still compiled as 0.14.10. Memory's checked metadata
construction/decoding owns validity formerly rescanned later; this service uses
none of the removed APIs. Testkit tightens artifact acquisition and simplifies
install retry bookkeeping; our explicit Make builds use `--release --lib` and
the maintained harness uses an explicitly selected server. Neither dependency
change introduces a consumer adapter or another state owner here.

Thirty core funding/read tests, seven standalone recovery/history/snapshot cases,
the combined-owner regression and ordinary/scale profiles pass. Scoped actual
Rust 1.88 native checks cover core, standalone and harness tests; scoped Wasm
checks cover core, standalone and storage fixture. These Linux checks preserve
the selected lock and do not replace full CI, full-workspace MSRV, native macOS
or deployed-provider qualification. Prior evidence retains its original graph.

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

The original [0.10.0 export](https://docs.rs/ic-testkit/0.10.0/ic_testkit/index.html)
exposes the complete PocketIC crate. Keep the dependency under native dev
dependencies; neither the production library nor its Wasm build needs testkit.
With the dedicated harness in place, the core package's native dev graph also
excludes it; testkit is owned by `crates/ic-blob-storage-pocketic-tests/Cargo.toml`.
The unpublished `crates/ic-blob-storage-pocketic-tests` harness uses these exports. This shares version
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
frozen-file helper's offline body/manifest/metadata/root/transfer-budget/abort refusals and input
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
For the serial media driver, `BLOB_BROWSER_EXECUTABLE=/absolute/path/to/browser`
can explicitly select an installed Chromium-family executable. Record its actual
version and binary hash separately from the pinned Playwright package; this does
not qualify it as Playwright's bundled Chromium. The default selection is
unchanged, and each run still owns a fresh profile. Image delivery assertions
permit HTTP-cache reuse while checking exact object bytes, origins and credentials.

The target builds the standalone Wasm/CLI and explicitly runs ignored cases against a
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

For a separate old-ledger refusal check, `make test-hard-cut` requires an explicit
`BLOB_PRE_CUT_STANDALONE_WASM` and fresh `BLOB_HARD_CUT_REPORT` beneath an existing
parent. See the [pinned fixture](../tests/fixtures/standalone-pre-cut/README.md).
It runs one ignored local PocketIC test; the historical Wasm is retained externally
and default CI cannot establish this check without it. It compares full stable
bytes and old obligations after actual upgrade rejection, with no provider calls.

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

Released 0.12.0 selected `ic-memory 0.21.0`; current development selects 0.24.5.
The current lockfile resolves one registry memory package and its
`ic-stable-structures` 0.7.2 substrate.
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
`make deps` fetches the locked graph and may use the network. The complete
`make ci`/`make validate`/`make release-verify` gate first verifies the reviewed
shared snapshot, verifies local executables and dependency declarations, then
checks local documentation links and shared digest/installer fixtures before
running this fetch step. Prerequisite or fetch failure
stops the gate. Rust checks
use `--offline --locked` and this repository's `target/`. Scoped targets such as
`make check` and direct `cargo --offline` commands still require a populated cache;
run `make deps` before them after dependency changes or cache removal. Updating
a dependency requires an intentional manifest/lockfile change; setup does not
select new versions. Full validation remains separately authorized.

Testkit and PocketIC are excluded from the production/Wasm graph. Their Rust
libraries are fetched by `make deps` and compiled by the native check. The local
canister test additionally needs a compatible PocketIC server: this library
accepts >=16.0.0,<17 and defaults to 16.0.0.
The original Linux x86_64 installation at
`.tmp/tools/pocket-ic-16.0.0/pocket-ic` and its
[provenance record](evidence/pocketic-toolchain.json) remain historical evidence.
The original record covers tool installation only; the later
[authority fixture evidence](evidence/core-primitives.md#pocketic-authority-probe-after-018)
records actual local canister execution.

Make exports `.tools/ic/bin/pocket-ic` as the default `POCKET_IC_BIN`, preventing automatic
server downloads during tests. A caller-supplied `POCKET_IC_BIN` overrides it.
The binary is ignored local tooling, so fresh checkouts need provisioning.
Provision the complete reviewed IC tool set from the repository root:

```sh
make install-ic-tools
make ic-tools-check
```

The [common setup](ic-tools.md) and [single pin matrix](../ci/ic-tools.tsv)
select platform assets for Linux and macOS on x86_64 and ARM64. The current
matrix selects PocketIC 16.0.0, Quill 0.5.4, ICP CLI 1.6.0, didc 0.6.2,
ic-wasm 0.11.1 and wasm-opt 132. Installation verifies archives before extraction,
checks versions and activates a complete set; `ic-tools-check` is offline.
The new Linux PocketIC executable matches the earlier retained binary hash.
Only PocketIC is exercised by the service installation tests; setup and version
checks alone do not qualify deployment or other tools' product workflows.
`make deps` fetches Cargo packages only; it does not provision executables.

The library and its tests have no downstream framework dependency. The
[Caffeine baseline](provider-baseline.json) remains the upstream integration
reference. No npm/Motoko package enters the Rust/Wasm dependency graph;
browser peers belong to the maintained private client and its concrete evidence
harness.

## Release and formatting host checks

[The tooling workflow](../.github/workflows/tooling.yml) prepares the selected Rust
and cargo-sort/ShellCheck tools explicitly, installs and verifies local host/IC executables,
then independently checks the snapshot, declarations, evidence, formatting,
local documentation links, shared digest/installer fixtures, release adapters
and real consumer hook behavior on
Ubuntu 24.04 and macOS 15 (Apple Silicon and Intel). Both local formatting targets
use cargo-sort 2.1.4. The declared matrix does not establish a passing native run;
its matching GitHub execution is required for qualification. This focused job
neither publishes a release nor replaces the complete `make ci` gate.
It then fetches the selected Cargo lock and runs `make test-native-host` offline
to exercise both CLI binaries and an actual standalone installation. The
native step sets `TMPDIR` to the runner's artifact directory so retained
`nonempty-cargo-test.*` logs match the failure uploader's selection. The
[0.15.1 adoption record](evidence/tooling-host-0151.json) records Linux execution;
the [0.15.2 run](https://github.com/dragginzgame/ic-blob-storage/actions/runs/37488854847)
now passes on Linux and both native macOS hosts, including native CLI/PocketIC.
The pending 0.15.3 snapshot/caller changes still require their own matching
consumer run; upstream CI and an earlier consumer run do not qualify new source.

The [0.14.12 adoption record](evidence/shared-tooling-adoption.md#01412-release-and-formatting-adoption)
records scoped Linux execution separately from native macOS qualification.
The local consumer hook fixture keeps the existing Cargo.lock and uses the actual
Make formatter; upstream's synthetic suite assumes its shell-only source tree.

`make evidence-check` independently verifies the retained local/deployed SHA-256
manifests, without building Rust or making provider requests. It uses the reviewed
checksum helper, choosing GNU `sha256sum` when available and Perl `shasum` on
macOS. Manifests contain lowercase digests, a space and text/binary marker, then
the exact filename; escaped filenames and malformed or empty manifests refuse.
The regression fixture also requires `shasum` to exercise that fallback explicitly.
The focused host matrix runs this check; `make probe-check` includes it after
checking recorded probe directories. Existing evidence stays unchanged.

Failed host tooling fixtures are uploaded for 30 days under an artifact name
containing the host and run attempt. This preserves command logs, inputs and
fixture state beyond the ephemeral runner. Local failures remain at the path
printed by their helper; successful checks clean only their own temporary files.
The failure uploader also retains `file-digests.*`, `ic-tools-test.*` and
`release-commands.*` fixtures under the native step's selected `TMPDIR`.
The [0.14.13 record](evidence/checksum-portability-v01413.json) distinguishes local
checksum/failure checks from the original native failures and pending corrected
macOS execution.

The [0.15.2 snapshot record](evidence/shared-tooling-adoption-0152.json) adopts
48 exact files from Shared Tooling `47cd2cc`. `make documentation-links-check`
checks root Markdown and Markdown under docs, audits and rules without fetching
links or building Rust; anchors and full Markdown grammar are outside its
contract. `make release-commands-check` supplies only the copied Cargo manifest
and release-data script needed by Make's read-only version expression, with a
substitute runner and no Git effects. `make shared-tooling-tests` exercises both
GNU/Perl digest backends when available and finite IC installer substitutes.
The IC pin matrix, active local tools and Cargo dependency selections are unchanged.

The [Bash 3.2 follow-up](evidence/bash32-v01413.json) checks explicit release
identity/publication refusals with the actual older shell on Linux. It also
records the pinned hook's formatter-failure defect and a passing isolated
one-line proposal. The subsequent [committed adoption](evidence/shared-tooling-recovery-v01413.json)
first adopts Shared Tooling `9437bab`, then refreshes to `cb86188` with 24 files,
including the fixed hook, user-triggered agent maintenance rules and validation
logger. Late release callbacks export
`RELEASE_COMMIT` and verify its original files, source parent and tag even when
HEAD contains newer fixes. The release gate retains actual failed command logs
across retries. Upstream native CI for `9437bab` passes on all three hosts; the
newer revision has a macOS temporary-path alias regression in its snapshot test
fixture; consumer native qualification remains pending. The refresh and focused
local consumer checks pass. Host CI
stops at snapshot failure, then collects all remaining focused check outcomes
before reporting failure and retaining available fixtures.

The later shared-owner/issue acceptance batch preserves the concurrently selected
direct ic-host-tools 0.2.0 and harness-transitive 0.1.14. Fresh offline native CLI
checks, the FIFO subprocess, strict CLI Clippy and Rust 1.88 compilation pass;
[the bound record](evidence/shared-tooling-owners-0152.json) retains that graph.
The earlier 0.1.14 records are historical, not qualification of 0.2.0.
