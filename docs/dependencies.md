<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Dependency setup

## Current dependency owners

The root [Cargo catalog](../Cargo.toml) owns all direct dependency requirements;
all twelve members inherit package versions and dependencies. The root
[lockfile](../Cargo.lock) owns this checkout's selected graph. Member manifests
select features and targets. Registry consumers resolve their own compatible
lockfiles; this repository's qualification does not cover every future resolution.

| Owner | Current locked selection | Scope |
| --- | --- | --- |
| IC Memory | 0.35.19 | Public memory types, host allocation pool and committed grants |
| IC Metrics | 0.8.0 | Private storage probe; tracked overflow quality |
| IC Testkit | 0.35.0 | Native PocketIC client, server setup and managed lifetime |
| IC Host artifacts/fs/process/tools | 0.15.1 | Native artifacts, filesystem and test/tool boundaries |
| PocketIC | 16.1.0 | Selected and admitted by Testkit; no separate Blob server pin |

The reviewed [Shared snapshot](../.shared-tooling.snapshot) is 0.9.10 at
`7e55adecf6f0b196c050f104b37f8c69b6bf434e`, with 112 payloads. Common installation,
checks, hooks and formatting use `make/consumer.mk`; product targets and Testkit
extensions stay local. The canonical Rust-tool fixture runs directly from this
consumer. Snapshot adoption does not automatically adopt Shared's CI cache policy.
See [current status](status/current.md) for released versus prepared identities and
[the adoption evidence](evidence/shared091002215.md) for source-bound results.
Earlier dependency reviews remain in their existing evidence records and the
[frozen former guide](https://github.com/dragginzgame/ic-blob-storage/blob/0d720d1745c1c9e87fe2a9cb922ba21c1dec5293/docs/dependencies.md);
those graphs are not current setup instructions.

## Quick setup

For ordinary development from the repository root:

1. Install `rustup`; the checked-in `rust-toolchain.toml` selects the maintained
   development compiler and required components.
2. Install the bootstrap packages described in [local setup](local-setup.md),
   including Perl's Digest::SHA, curl, archive utilities and xz. Run
   `make install-tools` to provision the pinned host, IC and Cargo executable
   sets under `.tools/`, then `make tools-check` to verify them offline.
3. Run `make deps` to fetch the exact locked Rust dependencies.
4. Run `make install-hooks` once per clone (also after updating
   developer setup). The reviewed hook uses `make fmt`; CI and release validation
   independently use `make fmt-check`. Both sort all twelve manifests and format
   Rust, without fetching dependencies, compiling or cleaning artifacts.
5. Run `make test-native` for native development checks, or
   `make test-native-host` for CLI/examples, actual PocketIC installation and
   the operator-owned Metrics restoration-read window.

Keep `$HOME/.cargo/bin` and `$HOME/.local/bin` on PATH when using user-local tools.
Release shell checks require ShellCheck; `SHELLCHECK=/absolute/path` may select
an already prepared binary. Install it with `sudo apt-get install shellcheck` on
Debian/Ubuntu or `brew install shellcheck` on macOS. The common local executable
setup covers jq/yq/rg/cloc, IC tools and cargo-sort/cargo-sort-derives/candid-extractor.
ShellCheck and the Rust toolchain remain separate prerequisites.
Missing prerequisites fail rather than being installed
during a hook or validation. Cargo-sort's exact reviewed version is recorded in
[tool versions](../ci/tool-versions.env). That file also owns jq/yq selections;
[the IC matrix](../ci/ic-tools.tsv) owns executable versions and archive digests.
Make prepends `.tools/host/bin`, `.tools/ic/bin` and `.tools/rust/common/bin` to PATH. Missing local tools
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

The unpublished PocketIC harness owns `ic-testkit`; production library/Wasm
code has no Testkit dependency. Import its maintained helpers and PocketIC types
through `ic_testkit::pic` and `ic_testkit::pocket_ic`. Use `make msrv-check` for
the repository's public, native and Wasm minimum-compiler checks after explicit
compiler/target and locked-cache preparation.

## Browser certificate evidence

`tests/browser` is a private fixture, with npm-locked `@caffeineai/object-storage`
1.1.2, `@icp-sdk/core` 5.4.0, Playwright 1.63.0 and esbuild 0.28.2. SDK 5.4.0
is within Caffeine's declared `^5.3.0` range; it replaces the earlier fixture's
6.1.0 pin so the composition uses one supported SDK. The Caffeine package is the
latest verified provider package.
The supported runtime range remains Node >=20.19.0. Reproducible fixture setup
selects Node 24.21.0 from `tests/browser/.nvmrc` and npm 12.2.0 from that build
root's `packageManager` declaration. Earlier browser evidence used Node 24.21.0 and
Playwright's Chromium 153.0.8010.12 (revision 1243). Upload fixtures also need
`openssl` on PATH to create temporary loopback TLS keys. Chromium trusts only the
generated certificate's public-key pin; unrelated TLS validation is unchanged.
Setup is explicit:

```sh
make browser-tools-check
(cd tests/browser && npm ci --ignore-scripts --no-audit --no-fund)
node tests/browser/node_modules/playwright/cli.js install chromium
make test-browser
```

Select those exact tools on PATH for setup; the build root's `.npmrc` explicitly
selects the public npm registry. `BLOB_BROWSER_NODE=/absolute/path/to/node` and
`BLOB_BROWSER_NPM=/absolute/path/to/npm` select prepared tools for Make checks.
All opt-in browser/SDK Make targets check both tools and manifest/lock declarations
before their effects. Direct bundle builds check Node and manifest/lock inputs;
they use prepared dependencies and do not invoke npm.
The private source client in `clients/browser` is consumed by this build root;
it has no independent install/build or registry publication workflow.
The target bundles existing local
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

`make test-browser-standalone` exercises the installed standalone service,
maintained native publisher, browser worker and IndexedDB journal with two local
users and a labelled local provider. It requires the prepared CLI, Testkit server,
Node/packages/Chromium and owned loopback TLS prerequisites. The
[operator guide](operator-guide.md) and [browser client contract](../clients/browser/README.md)
own the command and retained phase/recovery requirements. This is local IC/browser
proof, not application login, deployed provider behavior or billing acceptance.

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

## Memory composition

Use `ic_blob_storage::ic_memory` for the library's public storage types. Hosts
compose all namespaces in one Memory 0.35 host-wide allocation pool and open
permanent keys through its committed runtime. A source-identical package with a
different Cargo identity does not share Rust types. Never place a second manager
on the same backing memory.

The host owns the runtime, allocation policy and explicit grants. Include
`ops::service::installation::requests(authority)` before bootstrap: one
configuration key and sixteen service keys. Neither this helper nor linking the
library registers declarations, chooses physical IDs, initializes stores or
exports endpoints. `grants::open` uses the committed lookup; `open_default`
requires an already bootstrapped default runtime. Standalone owns its bootstrap.
Lifecycle restoration is synchronous and leaves the service fenced until the
maintained recovery workflow proves safe activation. Use `LIBRARY_VERSION` for
installation and exact same-release reopening.

### Canic integration

Canic owns its adapter, dedicated shell, embedded wrapper and composition tests
in its repository. Blob owns the framework-independent core and standalone host.
Do not add Canic dependencies, wrappers or fixtures here. The
[Canic composition guide](https://github.com/dragginzgame/canic/blob/main/docs/features/blob-storage/README.md)
owns current framework mounting APIs; [current status](status/current.md#canic-boundary)
links source-bound composition results and remaining owner work.

For consumer adoption:

1. Select one coherent published Canic family, matching Blob runtime/contracts and
   one Memory 0.35 identity. Freeze the application's actual lock and all independent
   fixture locks; sibling paths are not registry-delivery evidence.
2. Include Blob's seventeen requests in the host's single declaration snapshot
   and grant the `blob.` namespace in its allocation pool before bootstrap.
   Validate the complete installation for the allocated Principal before opening.
3. For a dedicated Component, require application initialization, encode the exact
   `ServiceInstallationInput` and bind it through Root's authenticated initializer.
   Retain the original operation, target and bytes for lost-reply reconciliation.
   For embedding, keep one application lifecycle and compose synchronous Blob
   installation/restoration plus its metrics sampler. Mounting initializes nothing.
4. Before producing Blob's passive DTO, enforce zero Candid skipping quota and
   exact argument count, including nested application initialization. Ordinary
   query/update decoding retains its own independent limits. Build normal Wasms
   with one memory runtime and check canonical Candid types and callable exports.
5. Run dedicated/embedded PocketIC initialization, target/caller refusal, rollback,
   lost-result, metrics and same-image recovery cases. Apply the
   [retirement contract](retiring-installations.md) before a cross-release reinstall.
   Same-release recovery never authorizes upgrading a retained installation.

Managed installation does not complete application acceptance. The application
must authenticate two users, issue exact tenant-approved permissions, operate the
[completion verifier](completion-verifier.md), qualify deployed provider reads and
commit its own media map only after exact completion/live references. See
[multi-user authority](multi-user-upload-authority.md),
[confirmed-reference publication](operator-guide.md#produce-a-complete-confirmed-reference-map)
and [funding qualification](funding-consumer-qualification.md). Provider payer,
credit, GC, deletion and billing facts cannot be replaced with local substitutes.

## Setup and checks

`make install-tools` provisions the complete shared host/IC/Cargo set followed by
Blob's lock-selected Testkit CLI/server. `make tools-check` admits those selections
offline. For a changed Cargo-tool policy run `make install-rust-tools`; for a
changed Testkit selection run `make install-testkit`. Setup retains old generations
and failed evidence. Use the same shared-store setting for setup, checks and
failure collection; see [local setup](local-setup.md#shared-executable-storage).

Scoped builds/tests use `--offline --locked`. After dependency changes or cache
removal, run `make deps` explicitly before them; it fetches only the selected
Rust source and does not install executable tools. An explicit offline setting
remains authoritative. Missing dependencies or binaries fail rather than being
installed, replaced or skipped by tests.

PocketIC callers obtain the admitted absolute server path through `make testkit-check`.
Explicit `POCKET_IC_BIN` overrides remain caller-owned byte admission; no fallback,
implicit download or independent server-version equality gate exists here. Testkit
owns client/server compatibility and lifetime. The common five-tool IC bundle is
pinned separately by [ci/ic-tools.tsv](../ci/ic-tools.tsv).

## Release and formatting host checks

The [tooling workflow](../.github/workflows/tooling.yml) qualifies setup/admission,
formatting/hooks, exact-ID tooling archive upload/readback, coordinated package
archives, native CLI/PocketIC and original/optimized Wasms on Linux, Apple Silicon
and Intel macOS. Rust 1.88 has its separate minimum-version lane. Declaring those
jobs is not a passing result; require matching native execution on the actual source.
Cancelled or earlier-source results do not qualify new inputs.

`make evidence-check` and `make probe-check` verify retained records offline;
they make no new provider requests. Failed tooling and package/Wasm evidence stays
at the printed paths and is uploaded by the hosted collectors. Successful fixture
cleanup removes only its own disposable inputs. See
[development governance](governance/development.md) for command authority and the
[release guide](releasing.md) for maintainer validation, delivery and publication.
