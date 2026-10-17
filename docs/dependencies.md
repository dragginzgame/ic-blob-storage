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
| IC Memory | 0.48.0 | Public host runtime and exclusive allocation leases; backing custody stays with the host |
| IC Metrics (direct) | 0.12.0 | Private storage probe measurements; Testkit retains its own transitive selection |
| IC Testkit | 0.50.0 | Native PocketIC client, bounded fixture capture, server setup and managed lifetime |
| IC Testkit Candid | 0.50.0 | Native fixture reply decoding with explicit caller limits; no public library dependency |
| IC Host artifacts/fs/process/tools (direct) | 0.24.0 | Native artifacts, filesystem and test/tool boundaries |
| IC Build | 0.12.1 | CLI build-time source fingerprints and opt-in native archive qualification; absent from public libraries and production Wasm |
| PocketIC | 16.1.0 | Selected and admitted by Testkit; no separate Blob server pin |

Memory 0.48 changes the protected allocation-ledger format. Earlier installations
require retirement before fresh backing; no automatic clearing or migration is
provided. Private probe journals explicitly create bounded cells on installation
and reopen retained cells on restore, preserving payload validation and fences.

Native input opening requires Host fs 0.17.9 or newer. That owner also supplies
the retained creation/publication used by verified body staging.

Published Host 0.23 supplies descriptor-rooted descendant opening. Inventory
metadata and body selection retain one admitted input root instead of returning
canonical paths to reopen later. Historical preparation-journal reads use the
same Host owner with their existing bounds and binding failures. Explicit
single-file input paths retain their original following policy. Held descriptors
do not freeze file contents or prove current pathname containment: callers must
exclude concurrent source writers, and complete digest verification still owns
content admission. [The reader qualification](evidence/host-readers0310.md) records
the selected registry graph, source-replacement refusals and installed recovery.

Published Host now owns whole-request and phased request-line/completion capture
for native provider, signed-update and backpressure fixtures. This applies
[PR #78](https://github.com/dragginzgame/ic-blob-storage/pull/78)'s payload locally
with explicit native test-support features. Testkit 0.50 removes its old testing
module and requires explicit reply limits on installed Candid calls. Retained Candid
fixture calls supply the test-owned `support::REPLY_LIMITS`: 4 MiB wire,
20 million decoding work, 1 million skipping work, 512 type entries and 64 KiB
header. These finite local-fixture bounds cover status/history and byte-bearing
probe replies; production clients retain their own narrower policies. No
former-signature wrapper or production default is added.
The separate Testkit Candid crate supplies bounded fixture reply decoding without
transport or lifecycle ownership. Blob retains binding checks and error mapping;
the [fixture cleanup receipt](evidence/fixture-convergence0291.md) records adoption.
After changing the selected Testkit CLI, explicitly run `make install-testkit`
before ordinary offline tool gates. Scoped installed evidence may use an explicit
server binary with caller-owned byte admission; that does not qualify the missing
lock-selected CLI installation.

The reviewed [Shared snapshot](../.shared-tooling.snapshot) is 0.15.1 at
`4f3eec42f52d4ec308bb68ab2daa3e5266b3175a`, with 131 payloads. Common installation,
checks, hooks and formatting use `make/consumer.mk`; product targets and Testkit
extensions stay local. The canonical Rust-tool fixture runs directly from this
consumer. Generic formatter-prerequisite and digest backend matrices run at
Shared; Blob retains real hook rollback, selected-tool admission and archive
checks. Their unused snapshot entrypoints are retired through the exporter. Snapshot adoption does not automatically adopt Shared's CI cache policy.
The [current adoption receipt](evidence/cleanup0280.md) records the selected
source, required local adapters and published Host qualification.
The [0.23.3 cleanup/adoption evidence](evidence/next-three0233.md) records its original
graph; [client/reply cleanup qualification](evidence/fourth-cleanup0233.md) records
focused checks of its original selection. The following
[view/intent/accounting cleanup](evidence/fifth-cleanup0233.md) records the incoming
Testkit 0.38/Host 0.17.7 graph and explicit server-tool preparation.
See [current status](status/current.md) for released versus prepared identities and
[the adoption evidence](evidence/memory036-host016-shared0913.md) for source-bound results.
The earlier private Metrics 0.9 overflow getter adoption is
recorded in [the probe qualification](evidence/metrics0900232.md). Metrics types
do not cross the public Blob package boundary; probe wire fields are retained.
The [Shared 0.10.2 / Host 0.17.2 refresh](evidence/upstream0232.md) records
the subsequent focused checks; earlier graph evidence is retained separately.
Earlier dependency reviews remain in their existing evidence records and the
[frozen former guide](https://github.com/dragginzgame/ic-blob-storage/blob/0d720d1745c1c9e87fe2a9cb922ba21c1dec5293/docs/dependencies.md);
those graphs are not current setup instructions.

IC Build 0.12.0 and direct Host now share process 0.24.0. The optional archive
verifier and native HTTP tests use the single `ic-host-process` dependency name;
`package-verification` activates its normal dependency, while HTTP test-support
remains development-only. The former separate Build-family alias is removed:
using two names for this now-identical crate makes Cargo reject all-feature
checks. No dependency selection or lockfile change is required for this repair.
[The caller repair receipt](evidence/ci-api0310.md#build-012-dependency-name-repair)
records current CLI compilation, lint, MSRV and archive replacement/refusal tests.
Earlier installed recovery results in that receipt retain their original graph.
Actual Blob archives and required native-host acceptance remain under
[#69](https://github.com/dragginzgame/ic-blob-storage/issues/69). The
[earlier receipt](evidence/decoder-commands0310.md) retains the original failures
before these repairs. Earlier
[0.11/0.23 compilation success](evidence/content-hex-origins0310.md) retains its
own graph and does not qualify this newer selection.

The initial [IC Wire adoption](evidence/icwire0320.md) retains its prepublication
source qualification. Wire/derive use the published 0.3 registry line; Cargo.lock
owns the exact selection, without a source override. The
[Testkit caller repair](evidence/testkit0510320.md) records package compilation,
strict lint, Rust 1.91 and seven selected installed cases on its then-selected
Wire 0.3.2 graph. The [inventory repair](evidence/inventory-codec0320.md) records
the later locked graph and adds only the admission probe's direct Serde edge.
Neither relabels the earlier candidate benchmark or qualifies every service method.
Remaining CBOR methods still need conversion before freezing the service contract.

The [0.32 incoming protocol](service-wire.md) still uses the already locked Ciborium
0.2.2 through Contracts for unconverted CBOR methods and replies. Serde is explicit in
the standalone, storage and admission probe manifests. This codec adds no external selection. Incoming maintainer edits separately select
Memory 0.47. Contributor Build 0.15 now matches selected Host 0.26 process types;
[the verifier repair](evidence/build0150320.md) records focused qualification.
Collection code names the already locked
`ic-stable-structures` 0.7.2 directly after Memory removed its blanket substrate
re-export; host backing types use Memory's supported root exports. Runtime leases
remain host-owned. Unconverted clients, native packets and owned probe methods
retain CBOR; tenant inspection/update use Wire. Candid remains at provider,
management and persisted-record owners. Public Memory changes belong to the
pending minor hard cut; these focused caller checks do not qualify all storage contracts.
Canic must coordinate its endpoint emitters, raw callers and interface checks before
adopting this hard cut. Its own integration results remain consumer-owned.

Contracts retains explicit Candid decoder configuration for external provider
contracts; owned service boundaries select Wire or CBOR explicitly. Each provider
boundary retains its byte/work/skipping/type/header choices, authentication and
error mapping; exact CBOR inputs check byte/depth limits and exhaustion locally. Provider URL components use the already locked
percent-encoding crate directly, with the original RFC 3986 unreserved set.

Fixture hex decoding uses the already locked `hex` crate as a direct development
dependency of the library, Contracts, CLI and PocketIC test package. It adds no
normal dependency to those packages. Frozen `.hex`/JSON inputs retain their
original owners and byte content; provider hash parsing remains a separate contract.
The [boundary cleanup receipt](evidence/boundaries0291.md) retains the initial
Memory 0.44 compilation failures and isolated Memory 0.43 results. The
[Memory 0.44 qualification](evidence/memory044.md) records their repair: remove
the default-runtime adapter and use the existing explicit committed lookup.
The public dependency and allocation-ledger format cut shipped in 0.30.0;
hosts must retire earlier installations before reinstalling with the new format.
Actual archive/release qualification remains separate; see
[#69 — Adopt Host custody and IC Build archive qualification](https://github.com/dragginzgame/ic-blob-storage/issues/69#issuecomment-6210216854).
The [earlier cleanup receipt](evidence/assessment-finalize0290.md) retains the
Build 0.7 mismatch; [current qualification](evidence/publication-planning0290.md)
records the confirmed Build 0.8/Memory 0.43 graph without relabelling old evidence.
IC Build captures the complete CLI source inventory at build time. Native records
retain `sha256:` digests and public-source probe records retain raw hexadecimal.
The inventory includes source-directory membership, the build script, package/root
manifests and root lock, using workspace-relative evidence. It is descriptive
provenance, not executable attestation or retry authority. The default CLI runtime
graph excludes Build; the optional archive verifier uses it at runtime. Public
libraries/contracts and production Wasm have no Build dependency.
[Current qualification](evidence/canic-issues0310.md) records the source-change
invalidation check and the published graph.
Memory's `test-support` feature is selected directly only by the authority,
funding and gateway-source probes. Production standalone compilation excludes
those probe selections; its Memory graph does not enable fixture support.
Testkit's separately installed CLI uses the published packaged lock; explicitly run
`make install-testkit` then `make testkit-check` after changing its selection.
Changing the application lock alone does not update the installed tool. Earlier
[issue-repair qualification](evidence/integration0234.md#published-opener-and-native-ci-setup-repair)
retains its original graph. Host 0.25 includes Candid service inspection without
a feature; interface tests enable only `ic-interface` for Wasm export projection.
Host Tools defaults are empty; disabling them no longer excludes Candid parsing.
No Candid extraction feature is needed by Blob. Memory request factories return
`MemoryDeclarationError`; hosts supply their complete owned `Vec<MemoryRequest>`
to `SealedDeclarationSnapshot::new` before the single runtime bootstrap.
No global registry or constructor is involved.

Descriptor-rooted inventory/journal reads remain a prepared consumer replacement,
pending publication of
[Host PR #114 — Root-relative opening](https://github.com/dragginzgame/ic-host-tooling/pull/114).
The [inspection cleanup receipt](evidence/inspection0290.md) separates ordinary
registry qualification from the isolated Host-source candidate. Do not copy its
private dependency overrides or lockfile into normal application setup.

Common tool versions and digests belong to the reviewed snapshot catalogs in
`ci/tool-versions.env` and `ci/ic-tools.tsv`. Blob has no common-tool exception.
Make, standalone installers, formatting, freshness inspection and evidence
collection use those same defaults. Any future host, Rust or IC exception must
be explicitly selected independently; a host override does not select Rust pins.
Compiler, application dependencies and Testkit/PocketIC remain locally owned.

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
Release preparation uses Shared’s literal manifest renderer without Cargo-edit.
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
| Library, standalone, CLI and examples | 1.91.0 | Inherited from `workspace.package.rust-version`; selected Memory 0.46 requires 1.91 |
| Unpublished PocketIC harness | 1.91.0 | Inherits the workspace MSRV |

The selected `ic-memory` 0.46 dependency requires Rust 1.91.0, which is now
the workspace's consumer floor. The harness inherits that same minimum.
Earlier Rust 1.88 checks remain evidence for their original source and graphs;
repeat checks with the declared minimum after source or dependency changes
rather than treating dependency metadata as execution evidence.

To repeat the locked native and Wasm compatibility checks after explicitly
provisioning Rust 1.91.0 and its `wasm32-unknown-unknown` target:

```sh
make msrv-check
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
compose all namespaces in one Memory 0.46 host-wide allocation pool and open
permanent keys through its committed runtime. A source-identical package with a
different Cargo identity does not share Rust types. Never place a second manager
on the same backing memory.

The host owns the runtime, allocation policy and explicit grants. Include
`ops::service::installation::requests(authority)` before bootstrap: one
configuration key and sixteen service keys. Neither this helper nor linking the
library registers declarations, chooses physical IDs, initializes stores or
exports endpoints. Supply `grants::open(|key| runtime.open_memory(key))` from the
host's retained, already bootstrapped runtime. No default singleton adapter remains.
Standalone owns its bootstrap.
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

The released 0.30.0 public cut requires the same Memory 0.44 identity in Blob
and the host. [#77 — Align exposed Memory types with 0.41](https://github.com/dragginzgame/ic-blob-storage/issues/77)
retains the earlier 0.41 acceptance; it does not qualify 0.44. Canic owns fresh
root/private-fixture lock and adapter qualification for this family. Local Blob
checks do not qualify that consumer. Testkit's native Host identity does not
replace the application's public Memory identity.

**Historical 2026-10-14 review:** Canic's released `v0.111.0` points to
`25b73cd403b20b4e148c6e340f8c760a26d97f6e`. Its catalog requires Blob/contracts
`0.22.2` and Memory `0.35`; Cargo cannot resolve those requirements to this
Blob 0.23 / Memory 0.36 family. Adding this Blob version directly to an application
does not upgrade the adapter's types or its memory runtime. Do not combine the
two graphs or substitute a path override as published integration evidence.

**Read-only review, 2026-10-16:** Canic HEAD
`600a831e0d8241796b70773ea8c4d8fbd63e422c` was clean at initial inspection;
concurrent owner edits later changed its root lock/Host callers. Its Blob/Memory
selections and both private locks remain as reviewed. The root
lock selects Blob/contracts 0.30.0 and Memory 0.44.0. Both independent Blob
fixture locks still select Blob/contracts 0.27.1 and Memory 0.41.0; the embedded
fixture also declares Memory 0.41 directly. Both fixture workspaces refuse
`cargo metadata --offline --locked` because their locks need updating. This is
an observed preparation failure, not an installed test failure. Canic must align
all three graphs and repeat its dedicated/embedded composition cases; earlier
0.27.1 installed successes do not qualify the current root. See
[Canic #517](https://github.com/dragginzgame/canic/issues/517) and
[Blob #77](https://github.com/dragginzgame/ic-blob-storage/issues/77).

Pending Blob 0.31.0 additionally replaces the two limit types used in Canic's
`integrations/blob-service/src/ops/account/mod.rs` and `ops/gateways/mod.rs` with
`ops::caffeine::decoding::CaffeineReplyLimits`. Preserve the existing numeric
budgets; there is no wire or persisted-format change from this consolidation.
Apply that caller change only with the selected 0.31 family, following
[the public upgrade guidance](service-contract.md#explicit-provider-decoder-budgets).
The [current review receipt](evidence/canic-issues0310.md) records the exact
source, locked metadata refusals and a checked, unapplied caller patch. Canic's
configuration decoder already uses zero skipping quota; parent authority,
guarded funding and actual application/provider acceptance retain their owners.

For consumer adoption of the current Blob line:

1. Select one coherent published Canic family, matching Blob runtime/contracts and
   one Memory 0.45 identity for pending 0.31. Freeze the application's actual lock and all independent
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
and Intel macOS. Rust 1.91 has its separate minimum-version lane. Declaring those
jobs is not a passing result; require matching native execution on the actual source.
Cancelled or earlier-source results do not qualify new inputs.

`make evidence-check` and `make probe-check` verify retained records offline;
they make no new provider requests. Failed tooling and package/Wasm evidence stays
at the printed paths and is uploaded by the hosted collectors. Successful fixture
cleanup removes only its own disposable inputs. See
[development governance](governance/development.md) for command authority and the
[release guide](releasing.md) for maintainer validation, delivery and publication.
