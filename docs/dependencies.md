<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Dependency setup

## Current dependency owners

The root [Cargo catalog](../Cargo.toml) owns all direct dependency requirements;
all thirteen members inherit package versions and dependencies. The root
[lockfile](../Cargo.lock) owns this checkout's selected graph. Member manifests
select features and targets. Registry consumers resolve their own compatible
lockfiles; local qualification does not cover every future resolution.

| Owner | Locked selection | Responsibility |
| --- | --- | --- |
| Blob core | Workspace version | Tenant authority, references, quotas, records, provider accounting, retention, deletion and recovery workflows |
| Blob client | Workspace version | Explicit replicated role clients; no service storage, endpoints or lifecycle |
| Blob Contracts | Workspace version | Runtime-free DTOs, method catalog, Blob codec projections, content identities and verification |
| IC Memory | 0.48.2 | Allocation ledger, grants and exclusive runtime leases; the host supplies backing and one allocation pool |
| IC Storage | 0.5.3 | Governed stable collections and codec traits; the stable-structures dependency stays upstream |
| IC Platform | 0.1.13 | Entrypoint/export macros, supported CDK calls/system APIs, Principal, external Candid encoding/decoding, management DTOs and native Agent/parser/certificate identities |
| IC Wire | 0.4.1 | Bounded method/direction framing for the two tenant contracts |
| IC Metrics | 0.14.4 | Private probe measurement arithmetic; not a public Blob API |
| IC Testkit | 0.54.1 | PocketIC client/server selection and managed lifetime; bounded Candid decoding uses Platform |
| IC Host artifacts/fs/process/tools | 0.27.6 | Native artifacts, descriptor-rooted reads, durable publication, processes, framing and interface inspection |
| IC Build | 0.18.0 | CLI source fingerprints and optional archive verification; absent from public libraries and production Wasm |
| PocketIC | 16.1.0 | Testkit-selected local IC execution; no separate Blob server catalog |

The standalone owns endpoint registration, caller capture, lifecycle and memory
composition. The core exports none of these implicitly. Consumer frameworks own
their wrappers and composition tests; see [Canic integration](#canic-integration).
The [service contract](service-contract.md) owns supported service invariants.

Contracts owns provider JSON boundary codecs, including
[`provider::upload`](../crates/ic-blob-storage-contracts/src/provider/upload/mod.rs).
For 0.34, replace `ic_blob_storage::ops::caffeine::upload` imports with
`ic_blob_storage_contracts::provider::upload`. The decoder, status and error
semantics are unchanged; Core no longer declares `serde_json` directly.
The CLI retains its required Serde/JSON dependencies and Host test helpers,
but no longer declares Testkit. PocketIC retains Testkit and owns its server
selection; removing the CLI declaration does not retire simulator setup.

Collection constructors take exclusive `RuntimeMemory<Backing>` handles from the
host runtime. Store/workflow generic parameters name `Backing`; passive memory
bundles carry handles without granting access. Production hosts compose all
seventeen Blob requests before a single bootstrap. Cross-release installations
require [safe retirement](retiring-installations.md), not automatic migration.

Native fixture custody uses Memory's published `testing::fixture_runtime` with
retained backing and explicit leases. Raw bytes remain available only to native
corruption/restore fixtures. Memory fixture support is selected by core unit tests
and isolated probes; the default standalone build must exclude those selections.
Do not infer production feature admission from a workspace-wide unified build.

Host owns descriptor-rooted inventory and journal reads, retained creation,
validated publication and bounded subprocess framing. Blob retains input bounds,
request/reply correlation, JSON contracts, uncertainty records and error mapping.
Held descriptors do not freeze contents: callers exclude concurrent writers and
verify complete body digests. Interrupted probe evidence deliberately uses retained
creation rather than an atomic writer that discards partial bytes. These are
necessary consumer policies, not independent filesystem implementations.

The optional archive verifier and HTTP fixtures use the same Host process package
identity. Its normal dependency also supplies the default CLI's bounded framing;
`package-verification` activates Build, not Host process. HTTP fixture support
remains development-only. Build's source inventory is descriptive provenance,
not executable attestation or retry authority. Testkit's transitive dependencies
retain their own identities; Blob must not pass typed values across incompatible
package versions. Installed tool selection is separate from the application lock:
run `make install-testkit` then `make testkit-check` after changing Testkit.

The [service wire guide](service-wire.md) describes the implemented protocol:
tenant inspection/update use published IC Wire; the other 29 owned methods and
standalone lifecycle use bounded CBOR through Ciborium. This mixed surface is
explicit in the canonical method catalog, with no codec fallback. Candid remains
for external provider/management contracts and existing persisted records.
Do not remove Ciborium or Candid as migration leftovers while those consumers exist.
Provider decoders retain their own work/type/byte budgets and authority checks;
Platform owns supported independent decoding, including Testkit 0.54 simulator
call limits and typed decoder failures. The retired Testkit decoder package is
forbidden by the direct-boundary policy; neither tool owns production provider
transport or its authority.

All 26 private consumer-probe controls use the same bounded CBOR codec as other
Blob-owned CBOR boundaries, with a 4 KiB ingress ceiling and exact whole-value
decoding. Their command/status envelopes belong to test protocol support, not
the public service catalog. The Rust callers and the opaque packets supplied to
Chromium use that codec; JavaScript defines no duplicate fixture schemas.
They forward actual tenant operations through the unchanged method-bound Wire
client; funding, manifest and asset operations retain their canonical service
CBOR clients. Transient scheduling/reply types no longer derive CandidType.
Lifecycle arguments and the stored consumer record still use Candid. Other
current probes and public application DTO consumers need the coordinated cut in
[#84 — Remove application DTO Candid coupling after migrating current probes](https://github.com/dragginzgame/ic-blob-storage/issues/84).
No persisted journal, provider contract or production installation byte changes.
For example, `ManifestIntent` retains CandidType because the consumer's
`ManifestIntentRecord` stores it inside the Candid-encoded `ConsumerRecord`.
Public trait removal must account for these actual stored DTO consumers as well
as endpoint callers; moving incoming controls alone cannot retire that coupling.

The reviewed [Shared snapshot](../.shared-tooling.snapshot) is 0.24.0 at
`cab12cd3dfa4fdb34846b25397f0d72881bb9ae0` (131 selected payloads, including the dependency-boundary checker). It owns common Make,
installation, tool storage, formatting, hooks and release mechanics. Blob owns
product inputs, release metadata/publication order, validation selection, Testkit
forwarding and evidence selection. Snapshot files are reviewed distribution,
not independent local implementations; do not edit them in place.
Common versions and digests belong to `ci/tool-versions.env` and `ci/ic-tools.tsv`;
Blob has no common-tool exception. Compiler and application dependencies remain
local selections. The [setup guide](local-setup.md) and
[development governance](governance/development.md) own executable commands.

Earlier source-bound receipts remain in [evidence](evidence/), including
[the Host reader adoption](evidence/host-readers0310.md),
[the initial Wire candidate](evidence/icwire0320.md) and
[Shared/Host qualification](evidence/shared023-host0270321.md).
Their older graphs and preparation blockers are historical evidence, not current
setup requirements. See [current status](status/current.md) for validation limits.

## IC Platform boundary

The pending lock selects published IC Platform **0.1.11** under the workspace
`0.1.11` requirement, with no root default features. Core and the seven canisters
select `cdk` for canonical entrypoint/export macros and supported runtime APIs;
this also enables Candid and the exact upstream runtime/executor identities.
Other members select `candid` for external DTOs, encoders, bounded decoders
and upstream configuration/staged decoding. Core additionally selects
`management-canister-types` for continuity-history DTOs. The native CLI selects `agent`;
PocketIC's native dev declaration also selects `agent`, `candid-parser` and
`certification`. None of those host selections enter default standalone Wasm.
Candid derives and service-export generators name the published contract.

The [0.1.11 follow-up](evidence/platform01110332.md) records removing the last
direct CDK dependencies through published cycle/counter exports. The
[0.1.10 follow-up](evidence/platform01100332.md) retains its published archives,
entrypoint migration and focused checks against that earlier graph. The
[0.1.8 follow-up](evidence/platform0180331.md) retains its earlier published
selection, unchanged used APIs and focused recovery/graph checks. Its new snapshot
and status DTO exports require no new Blob call or feature selection. The
[0.1.7 follow-up](evidence/platform0170331.md) retains Agent-response and native
dynamic-fixture adoption evidence against its original graph. The
[0.1.6 follow-up](evidence/platform0160331.md) retains installed continuity-recovery
qualification against its original graph. The
[0.1.5 routing receipt](evidence/platform0150331.md) and
[earlier adoption](evidence/platform0331.md) retain their original graphs. Core,
Contracts, protocol support, CLI, native PocketIC support and all canisters have
no direct Candid declaration. No workspace member declares Agent directly.
Configured decoders use the same upstream DecoderConfig/IDLDeserialize identities,
optional/default quotas, diagnostics and staged instruction observations. Existing
default-decoder fixture calls explicitly select upstream defaults through that
API. There is no local decoder facade or new set of automatic limits.
Application Wire/CBOR codecs, field labels, persisted bytes, storage and memory
remain unchanged.

Platform's management module now exports CanisterInfoArgs, CanisterInfoResult,
ChangeDetails and CodeDeploymentMode from exactly 0.11.0. Recovery imports these
identities directly through Platform; no local DTO, call wrapper or conversion
is added. The original 30-second call, 20-change horizon, 64 KiB reply limit,
decoder quotas, post-await version checks and refusal/fence policy remain.
The direct management dependency is removed and forbidden by the boundary policy
([Platform #12](https://github.com/dragginzgame/ic-platform/issues/12)).
ICRC ledger types and raw debug-print additions have no new Blob consumer and
are not enabled by this adoption.

Native dynamic fixtures use Platform's `candid_parser::{IDLValue, IDLField,
TypeSerialize, ValueSerializer}` with the existing native-only parser selection.
The original constructed values, type/header refusal packets and test admission
limits remain. Native signed-update and headless certificate evidence use
`agent::{CallResponse, TransportCallResponse}` with unchanged matching, certificate
verification, request identity, deadlines and no-retry policy. Direct native
Candid/Agent declarations and their policy exceptions are removed. The upstream
packages remain transitive; these exports do not reduce the wire or runtime surface.

No direct Candid/CDK exceptions remain. Platform 0.1.11 exports the original
`msg_cycles_refunded`, `msg_cycles_available`, `msg_cycles_accept`,
`instruction_counter` and `call_context_instruction_counter` helpers. Core and
five probes now consume those identities through Platform, removing the final
six direct CDK declarations and root catalog entry. Version, liquid balance and
stable size continue through Platform's exact raw upstream functions. All
entrypoints, traps, caller, time, argument/reply and supported call APIs use
Platform. Its generated paths use Platform's Candid contract while runtime
implementation remains upstream CDK 0.20.3, with executor 2.0.0 and ic0 1.2.0.
No local wrapper, counter-selector copy or cycle ABI implementation is added.

`make platform-boundary-check` runs Shared's canonical identity checker against
[local policy](../ci/cargo-dependency-boundaries.json), also in CI/release validation.
It observes aliases and every direct dependency kind/target/optional declaration.
The policy forbids direct Candid, the CDK family and raw ic0 everywhere. Direct access to
the internal Platform macro package is also forbidden. Agent and direct
management types are forbidden everywhere.
Allowances are package-level, not per-use/feature/kind permissions. Do not expand
them merely to avoid a migration. No upstream crate alias or compatibility conversion is used.
The same policy forbids direct `ic-stable-structures` declarations in every member,
including renamed, dev, build, optional and target-specific dependencies. IC Storage
owns collection mechanics; IC Memory owns allocation/backing mechanics. Their
transitive substrate dependencies remain supported. The runtime-free Contracts/CLI
normal-graph check rejects both Memory and Storage, including indirect reachability.

Standalone still reaches Candid 0.10.38, upstream CDK/macros 0.20.3,
Platform macros 0.1.11 and one executor 2.0.0;
CLI still reaches Candid and Agent transitively. These are controlled identities,
not claims that their code is absent from binaries. Inspected standalone/CLI
normal/build graphs exclude parser/Testkit packages. The separate executor-only
selection stays Candid/full-CDK-free. Automatic transitive deployment exclusions
and binding source/lock/target/features to deployed bytes remain separate missing
consumer admission, not supplied by this direct-policy check.

## Dependency roles and deployment admission

The root catalog labels `Production dependencies` and `Tests and tooling only`.
Production includes transitive use, normal and build dependencies, and supported
optional features/targets. IC Build belongs here because the CLI uses it during
building and optional package verification. Hex and Serde byte/CBOR helpers also
occur transitively in production. Member-role notes sit above the globally sorted
member array; blank-separated dependency groups use `cargo sort --workspace
--grouped` in both formatting modes and release metadata checks.

The [0.24 adoption receipt](evidence/shared0240331.md) records locked normal/build
graph inspections for default standalone Wasm, Core/Contracts Wasm and the native
CLI with package verification. No test/tool catalog dependency enters those
graphs. The selected standalone root is excluded from the comparison: its catalog
entry is consumed only by tests, while the package itself is a production root.
The provider interface-export feature was checked separately; it is an inspection
configuration, not a supported deployment artifact.

**Automatic deployment exclusion enforcement is missing.** The existing
`make contracts-boundary-check` rejects service runtime dependencies in Contracts
and CLI normal graphs across targets. It does not inspect standalone, build edges
or arbitrary test/tool transitive reachability. Formatting, snapshot and metadata
checks preserve declarations; they do not enforce role labels. Blob builds a local
artifact and has no deployment command. Before deployment, the consuming host or
operator must enforce exclusions for its exact locked target/features and bind
source, lock and inspection evidence to the final artifact digest. The successful
local build and retained digest are evidence for this graph, not proof of deployed
bytes or every supported configuration.

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
   independently use `make fmt-check`. Both use grouped catalog sorting across all thirteen manifests and format
   Rust, without fetching dependencies, compiling or cleaning artifacts.
5. Run `make test-native` for native development checks, or
   `make test-native-host` for CLI/examples, actual PocketIC installation and
   the operator-owned Metrics restoration-read window.

Keep `$HOME/.cargo/bin` and `$HOME/.local/bin` on PATH when using user-local tools.
Release shell checks require ShellCheck; `SHELLCHECK=/absolute/path` may select
an already prepared binary. Install it with `sudo apt-get install shellcheck` on
Debian/Ubuntu or `brew install shellcheck` on macOS. The common local executable
setup covers jq/yq/rg/cloc, IC tools and cargo-sort/cargo-sort-derives. Candid extraction is no longer
a common installation; Blob has no extractor executable consumer.
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
to select candid_parser 0.4.1 transitively, ic-agent 0.49.2, sha2 0.11.1 and thiserror 2.0.18.
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
| Development, formatting and lint | 1.99.1 | Pinned in `rust-toolchain.toml` |
| Library, standalone, CLI and examples | 1.91.0 | Inherited from `workspace.package.rust-version`; the selected Memory 0.48 line requires 1.91 |
| Unpublished PocketIC harness | 1.91.0 | Inherits the workspace MSRV |

The selected `ic-memory` 0.48 dependency requires Rust 1.91.0, which is now
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
compose all namespaces in one Memory 0.48 host-wide allocation pool and open
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
IC Storage's `adapter::StableBTreeMap` owns collection access. Blob's
`ServiceInstallation<M>`, store owners and workflow/transport associated memory
types now use the backing `M`; their memory bundles carry `RuntimeMemory<M>`.
For example, an owner formerly annotated with `RuntimeMemory<DefaultMemoryImpl>`
uses `DefaultMemoryImpl` as its type argument. Supply the same runtime leases;
do not introduce another production manager or use a dependency rename to expose
raw collections. Record bytes, stable keys and production collection layouts
remain unchanged. The private instrumented storage probe alone uses nested
fixture managers, so its read counts include those managers and cannot be
compared directly with older profiles.
Lifecycle restoration is synchronous and leaves the service fenced until the
maintained recovery workflow proves safe activation. Use `LIBRARY_VERSION` for
installation and exact same-release reopening.

### Canic integration

Canic owns its adapter, dedicated shell, embedded wrapper and composition tests
in its repository. Blob owns the framework-independent core and standalone host.
Do not add Canic dependencies, wrappers or fixtures here. The
[Canic composition guide](https://github.com/dragginzgame/canic/blob/main/docs/features/blob-storage/README.md)
owns current framework mounting APIs. Blob's local tests do not qualify Canic.

For consumer adoption of this source:

1. Select compatible Blob core/Contracts packages and the host's same Memory 0.48
   identity. Store/workflow type parameters now name backing memory; constructors
   take exclusive runtime leases. Freeze the application's actual lock and every
   independent fixture lock; sibling paths are not registry-delivery evidence.
2. Include Blob's seventeen requests in the host's single declaration snapshot
   and grant the `blob.` namespace before bootstrap. Validate installation for the
   allocated Principal before opening stores. Mounting initializes nothing.
3. Preserve the host's authenticated initializer and one synchronous lifecycle.
   Translate its admitted configuration to `ServiceInstallationInput`; standalone
   initialization is CBOR, not a template for the framework's outer envelope.
   A Candid outer initializer still requires zero skipping quota and exact argument
   count, including nested application configuration.
   Retain exact target/operation bytes for lost-reply reconciliation.
4. Mount both request and response codecs from the current method catalog:
   tenant methods use Wire, other owned methods use CBOR, and the provider
   certificate uses Candid. Check actual Wasm names/modes and the provider-only
   Candid subset. A Candid service file does not describe the complete Blob API.
5. Run dedicated/embedded PocketIC initialization, target/caller refusal, rollback,
   lost-result, metrics and same-image recovery cases in Canic. Apply the
   [retirement contract](retiring-installations.md) before a cross-release reinstall.
   Same-release recovery never authorizes upgrading a retained installation.

The [2026-10-16 receipt](evidence/canic-issues0310.md) retains its historical
source, stale fixture-lock failures and proposed caller repairs. It does not
establish current integration. The latest read-only source observations and
inspection limits are recorded in the
[architectural sediment audit](reports/audits/2026/10/18/architectural-sediment/01/report.md).
No sibling source or lockfile is changed by Blob adoption.

Managed installation does not complete application acceptance. The application
must authenticate two users, issue exact tenant-approved permissions, operate the
[completion verifier](completion-verifier.md), qualify deployed provider reads and
commit its media map only after exact completion/live references. See
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

## Replicated clients and service storage

The `ic-blob-storage-client` package owns the existing role-specific replicated
calls and their common one-call mechanics. Its normal graph includes Contracts
and Platform execution, but excludes Core, Memory, Storage and stable structures;
`make contracts-boundary-check` enforces that distinction across targets.
Contracts and the native CLI retain their separate runtime-free checks.

Use `ic_blob_storage_client::{admission,download,funding,manifests,references,status,tenants}`
for remote calls. The old Core client paths are removed in the pending 0.35.0
public cut. Core owns installed service state and workflows; consumers retain
intent, retry decisions, application publication and uncertain-outcome recovery.
The client package installs no endpoints, lifecycle hooks, stores or timers.
Publishing proceeds Contracts, Client, Core. The client shares the workspace's
current version during development; only maintainer release preparation advances
that version. It is not yet an available registry dependency before publication.
