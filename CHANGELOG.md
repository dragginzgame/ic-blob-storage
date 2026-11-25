# Changelog

## [0.111.0]

### ⚠️ Breaking changes

- Use IC Storage 0.80 named allocations and Cells. Fresh installations and
  same-layout upgrades use the new format. Existing Storage 0.78 installations
  need a separately qualified migration before they can move to this release.
  Rust applications embedding Blob should use Storage 0.80 for their leases.

### 📦 Dependencies

- Align both Cargo workspaces on Platform 0.35.1, Runtime 0.23.27, Protocol
  0.3.10, Types 0.21.5, IC Wire 0.30.2 and Toko Tooling 0.46.4.
- Both lockfiles select IC Logging 0.25.21 and IC Metrics 0.30.22.

## [0.110.0] - 2026-11-24

### ⚠️ Breaking changes

- Hosts using Blob’s public Wire types should select IC Wire 0.30. Match
  admission failures as `Error::Limit(kind)` to inspect the exceeded allowance.

### 📦 Dependencies

- Adopt Toko Tooling 0.46.3 and align both workspaces on Platform 0.34.19,
  Runtime 0.23.25, Protocol 0.3.8 and Types 0.21.4.

## [0.109.0] - 2026-11-21

### ⚠️ Breaking changes

- Blob now exposes IC Wire 0.28 types. Hosts that pass `RPC_LIMITS` to Wire or
  handle Blob’s Wire errors should select IC Wire 0.28.

### Performance

- Uploader overviews scan application references once, preserving ID order
  and shared-root references across uploaders.
- Empty and fully acknowledged callback batches skip stable writes. Mixed
  batches still validate every root before persisting new receipts.

## [0.108.1] - 2026-11-20

### 🔧 Tooling

- PR CI now compares the mounted request and reply schemas for all eight Wire
  methods alongside the release Wasm export names and call modes. Schema
  generation runs in the private qualification workspace.
- Both Cargo lockfiles now select `url` 2.5.9 and `form_urlencoded` 1.2.3.

## [0.108.0] - 2026-11-20

### ⚠️ Breaking changes

- Blob’s public Rust codec types now use IC Wire 0.27. Hosts that pass Blob’s
  `RPC_LIMITS` to Wire or handle its codec errors should also select Wire 0.27.

### Added

- The optional `metrics` library feature lets embedded hosts supply a heap store
  for command outcomes and registered object sizes on creation and reopening.
- The independent `logging` feature emits Blob lifecycle events and selected refusals
  through a host-supplied recorder compatible with Canic’s shared log.

### 🔧 Tooling

- Adopt Toko Tooling 0.46.0 and IC Wire 0.27.1 in both workspaces. PR CI
  compares callable method names and modes in the standalone and reference-app
  release Wasms with the 0.107.1 baseline.

## [0.107.1] - 2026-11-19

### 🔧 Tooling

- CI uses Toko’s shared formatter for failure excerpts and streams short progress
  lines as they arrive. Evidence collection preserves the command’s exit status.
- Refresh the locks to IC Wire 0.26.1, Platform Runtime 0.23.23 and Types 0.21.4.
  Qualification uses Testkit 0.110.3.

## [0.107.0] - 2026-11-19

### ⚠️ Breaking changes

- The public Rust codec types now come from IC Wire 0.26. Hosts that pass Blob’s
  `RPC_LIMITS` to Wire or handle its codec errors should select Wire 0.26.

### 🔧 Tooling

- Adopt Toko Tooling 0.45.21 in both workspaces. Platform Runtime 0.23.22 lets
  native Clippy check the same five canister entrypoint implementations that ship
  on Wasm. Their headers now identify admission guards and request and reply budgets.

## [0.106.3] - 2026-11-19

### 🔧 Tooling

- Adopt Toko Tooling 0.45.18 in both workspaces. Each canister library root now
  lists its IC endpoints together with their caller permissions and request bounds.
- Use published Platform Runtime, Macros and Runtime Guard 0.23.20 across
  the product and qualification workspaces. Both lockfiles select Platform
  0.34.18, Storage 0.78.6 and Wire 0.25.2.

## [0.106.2] - 2026-11-19

### 🔧 Tooling

- Adopt Toko Tooling 0.45.16 in both workspaces. Native builds now check the
  standalone Wire codecs, callback codecs, catalog state and enrollment storage.
  Wasm target guards cover the IC entrypoints and the reply probe's trap hook.

## [0.106.1] - 2026-11-19

### 🔧 Tooling

- Adopt Toko Tooling 0.45.15. CI redacts Bearer credentials and access-key values
  from console output and retained diagnostics, including key material following
  an oversized opening line.
- Installed release preparation rejects incomplete validation plans before
  invoking Cargo or building artifacts.
- Refresh both lockfiles to IC Storage 0.78.4, Platform 0.34.17 and Wire 0.25.1.
  Qualification now uses Testkit 0.110.2.

## [0.106.0] - 2026-11-18

### ⚠️ Breaking changes

- Blob now uses IC Wire 0.25. Rust hosts that pass Blob’s `RPC_LIMITS` to Wire
  or handle its codec errors should select the same Wire minor version.

### Fixes

- Browser transfer and durable custody now check provider bindings with the same
  rules. A gateway ending in an empty query or fragment is rejected before a claim.
- The CLI escapes filenames in input errors and reports the filesystem reason
  when it cannot resolve a path.

### 🔧 Tooling

- Adopt Toko Tooling 0.45.8. Cargo checks continue to use locked dependencies
  and can now fetch missing packages. Set `CARGO_NET_OFFLINE=true` to request
  an offline run.

## [0.105.0] - 2026-11-17

### ⚠️ Breaking changes

- Adopt registry Storage 0.78.0 in both workspaces. Embedded hosts grant named
  leases for bounded Wire records. The standalone host and CLI support ordinary
  Wasm builds and upgrades. Remove the OP transform, linker layout and root rows.
- **Existing installations.** Blob 0.103.3 and 0.104.0 each use an incompatible
  layout. Keep populated installations on their original version until their
  data and outstanding obligations have a migration path.
  [Storage contract](docs/design/stable-storage.md).

## [0.104.0] - 2026-11-17

### 🔧 Release validation

- Scope release checks to changed components and their installed consumers using
  Toko's plan runner. Shared/unknown inputs retain full product qualification;
  unchanged metadata regressions no longer run on every product release.
  Repository CI retains its native coverage. Qualifies dispatch and recovery for
  [Blob #3 — Scoped release qualification](https://github.com/dragginzgame/ic-blob-storage/issues/3).
- Refresh authenticated release helpers to Toko Tooling 0.45.0 and requalify
  scoped dispatch, recovery and the independent qualification-lock projection.
- Give browser dependency installation one CI owner, cache authenticated tool
  downloads through Toko and delegate failure uploads to its prepared-archive
  action. Preserve bounded sanitized diagnostics and explicit incomplete/upload
  failures; shared artifact retention is fourteen days.

### 🗄️ Native OP storage

- Adopt Storage 0.76.2 retained roots throughout the library, standalone host,
  reference catalog and probes. Replace generic stable leases with supplier grants
  and bounded domain rows; preserve historical IDs, accounting and receipt rules.
- Prepare/admit Wasm through Storage’s supported transform. Upgrades close/reopen
  native state and the CLI explicitly signs Keep memory persistence.
- Adopt Testkit 0.110.0, Runtime/Macros/Guard 0.23.17 and Host Process 0.81.0.
- **Breaking persistence/API change:** pending 0.104.0 is a fresh native layout,
  without a 0.74.1 image migration. The consumed live probe stays unchanged;
  [Storage #1](https://github.com/dragginzgame/ic-storage/issues/1) remains open.

### Try it

- Build prepared local artifacts with `make build-host build-reference-app`.
  Follow the [native storage contract](docs/design/native-storage.md) and
  [maintenance procedure](docs/operations/standalone-maintenance.md).

## [0.103.3] - 2026-11-17

### 🧪 Qualification

- Share simulator setup and cleanup through the existing qualification library,
  removing all eight test module path overrides without duplicating helpers.

### 🔧 Dependencies

- Retain the concurrent zeroize 1.9.2 lockfile update in both workspaces.

## [0.103.2] - 2026-11-17

### 🔧 Module layout

- Move five CLI modules with child directories into `mod.rs`, including nested
  application/install modules. Module paths and implementation bytes are unchanged.

### 🔧 Platform dependencies

- Adopt Platform 0.34.13, Protocol 0.3.5 and Runtime/Macros/Guard 0.23.16 in both
  workspaces. Management status decoding now requires the platform's canister version;
  Blob's application interfaces, saved requests and durable state are unchanged.
- Remove qualification's legacy Platform alias and Types 0.20.4. Protocol now
  decodes execution-cycle metrics, optional top-up arguments and opaque-success /
  typed-error top-up replies, preserving bounds and the retained provider refusal.
  Adopts [Platform #24 — Final Blob qualification codecs](https://github.com/dragginzgame/ic-platform/issues/24).
- Replace legacy-generated malformed inputs with independent literal wire fixtures;
  callback responses must match the exact `vec blob` profile. No generic codec added.
- Adopt Testkit 0.108.0 and unify its held-reply codecs with Protocol 0.3.5,
  removing qualification's duplicate Protocol 0.2.8.
- Adopt Host Artifacts/FS 0.80.3 in both workspaces and Host Process 0.80.4 in
  qualification. Filesystem custody and simulator teardown behavior are unchanged.
- Declare both workspaces' Platform minimum explicitly and use Protocol's
  empty-argument encoder for ordinary callback requests. Storage's separate
  IC0-only Runtime 0.22.5 dependency remains unchanged.
- Retain the concurrent Clap 4.6.8 / derive 4.6.8 / lex 1.1.2 lockfile refresh;
  aggregate CI and installed management qualification pass on the resulting graph.

## [0.103.1] - 2026-11-16

### ▶️ Try it

From any directory, install the CLI once (requires registry access), then inspect
the new workflow's inputs offline:

```sh
cargo install ic-blob-storage-cli --version 0.103.1 --locked
blob app prepare-install --help
blob app dispatch-install --help
blob app observe-install --help
```

The help lists the required artifacts, custody and controller inputs without
loading credentials or contacting a canister. Follow the
[installation guide](docs/design/reference-application.md#prepare-and-inspect-one-signed-install)
to prepare exact bytes; actual dispatch requires a signer, fresh target and matching
admitted checkout, and performs an installation that consumes canister cycles.

### 🔧 Qualification dependencies

- Select Platform 0.34.9 and Testkit 0.107.4 in qualification; both lockfiles resolve
  Protocol 0.2.8 under compatible 0.2.7 declarations. Testkit's updated requirement
  removes the former exact 0.2.6 constraint. Retain scoped legacy codec fixtures/diagnostics.
- Retain qualification's Host Process 0.80.3 update, including its macOS libproc
  0.15 dependency. Product dependency selections are unchanged by this update.
- Registry freshness review confirms the selected first-party runtime suppliers.
  Defer the optional IC Build editor 0.73.0 → 0.87.3 refresh to the next tooling
  maintenance batch; preserve the tested two-lock release adapter.

### ➕ Guarded installation requests

- Add `blob app prepare-install` and `inspect-install` for one exact signed fresh
  installation, with explicit role, paired targets, controller, Wasm and init inputs.
- Retain one immutable network/target claim, including partial failures. Offline
  inspection verifies the signature and selected bindings without a private key;
  expired requests remain inspectable and cannot be renewed by preparation.
- Qualify changed artifacts, targets, signatures and operations, partial claims and
  cross-process replay refusal. The installed local witness consumes exact retained
  requests for both product hosts and checks their resulting configuration.
- Add `dispatch-install` with admitted tooling, explicit fresh-creation custody,
  authenticated target checks and a permanent claim before one submission.
- Add `observe-install` to retain and authenticate the original terminal outcome
  before checking current module/controller. Pending, failed or lost replies never
  permit resubmission; partial claims and corrupt outcomes remain preserved.
- Race two CLI dispatchers against the local gateway, drop the actual installation
  HTTP response and recover its certified result in a fresh process. Assert one
  submission and preserve original custody through rejection and later state drift.
- No canister creation, funding, reinstall fallback or live-provider effect.

## [0.103.0] - 2026-11-16

### 🔧 Tooling and CI diagnostics

- Adopt published Toko Tooling 0.44.11 in both workspaces, its authenticated
  snapshot and the CLI binding. Selected executable tooling bytes are unchanged.
- Retain bounded, sanitized setup/check output on failed CI jobs, preserving
  original command status and reporting collection/upload failures separately.
  Retain only diagnostic text for seven days; no tool-store or workspace uploads.
- Qualify issue references with their owning repository.

### Platform and Testkit adoption

- Adopt Platform 0.34.8 and Runtime/Macros/Guard 0.23.14 in both workspaces,
  Protocol 0.2.7 in the product and Testkit 0.107.3 / Protocol 0.2.6 in qualification.
- Replace legacy Agent and certificate integration with signed native Platform
  calls, original-request recovery and exact authenticated proof custody.
- Move Caffeine and management codecs to Protocol, removing legacy Platform
  entirely from the product. Private fixture/diagnostic codecs remain.
- Breaking CLI change: custom trust keys now require an explicit `--root-subnet`
  principal; select 0.103.0 for this compatibility change.
- Move qualification callback, certificate and management-target codecs to
  Protocol; remove the legacy management-types feature and local reply projection.
- Registry review confirms Platform 0.34.8 is current. Qualification Protocol
  0.2.7 cannot resolve with Testkit 0.107.3’s exact 0.2.6 requirement; retain the
  qualified graph for this release and revisit with the next Testkit update.
  Metrics/top-up-result decoders and malformed-input oracles still need legacy codecs.
- Defer the optional IC Build release-editor refresh (0.73.0 to 0.87.3) to the next
  tooling maintenance batch; its existing two-lock metadata fixture remains required.
- Prepare local simulators through Testkit’s managed-server API and assert
  instance deletion/process teardown; use its bounded held-reply fixtures.
- Qualify populated predecessor upgrades and recovery of exact released-CLI
  requests after expiry, plus transfer, lifecycle and caller-isolation regressions.

## [0.102.0] - 2026-11-16

### ⚠️ Platform Types and Runtime migration

- Adopt published `ic-platform-types` 0.21.2 in both workspaces. Public Rust
  records and domain state now use that identity; this dependency-type change
  selects a minor release. No intentional Wire or stable-record format change.
- Adopt Runtime/Macros/Guard 0.23.13 for the standalone host, reference app and
  qualification probes. Explicit bounded lifecycle/query/update entrypoints keep
  Wire encoding failures as traps; consuming calls preserve timeout uncertainty.
  Remove the temporary host Principal copies and scope host code to Wasm.
- Adopt Platform 0.34.6 ingress admission in the CLI and installed qualification:
  strict original envelope decoding and request IDs preserve signed custody,
  reject malformed authentication fields and never renew an expired request.
  Remove the old direct `ic-protocol` feature. Remaining 0.33.7 uses are explicitly
  named `ic-platform-legacy`: signed transport, certificate DTOs and external codecs.
- Storage retains its IC0-only transitive Runtime 0.22.5; the host has one new
  executor/guard generation. Full Platform adoption remains partial: 0.34.6 has
  no signed transport and Protocol 0.2.5 lacks Blob's management/Cashier codecs.
  No live effect or stable-record format change is introduced.

### ➕ Application target observations

- Add `blob app check-targets` for two proposed canisters: authenticated replicated
  status, absent modules and the signer as sole controller. Refuse equal targets,
  installed modules, additional controllers and stopping targets. No installation,
  creation, funding or controller changes are dispatched.
- Reuse the maintenance status transport and bounded decoder. Observations are
  separate and can become stale; an absent module does not prove fresh creation
  or absence of historical provider obligations.

### 🛠️ Tooling

- Adopt Toko Tooling 0.44.9 in both Cargo workspaces and its authenticated
  84-file snapshot; refresh the CLI's embedded tooling fingerprint.
- Apply repository-wide named-supplier adoption scope, including qualification
  and native callers; continue to report the unfinished Platform migration as partial.
- Record the remaining native signed transport/protocol requirements separately
  from the completed host migration in the application boundary.
- Release freshness review keeps the qualified Testkit 0.106.0 and release-editor
  IC Build 0.73.0 selections. Optional upgrades to Testkit 0.107.2 and IC Build
  0.87.3 are deferred to the next dependency-maintenance batch; other direct
  first-party selections are current apart from the documented legacy Platform use.

### ▶️ Try it

```sh
make blob
blob app check-targets --help
```

## [0.101.0] - 2026-11-16

### ⚠️ Breaking

Rename the CLI group from `blob application` to `blob app`, matching Canic.
Update catalog, draft and preparation commands in scripts; the old spelling is
removed. Stored records, Wire endpoints and installation formats are unchanged.

### ▶️ Try it

From the repository root, prepare tools once with `make install-tools`, then:

```sh
make blob
blob app prepare-installation --help
```

The command describes the explicit authorities, limits and output directory.
[The installation guide](docs/design/reference-application.md#prepare-fresh-installation-inputs)
has a complete example. Preparation writes local files; it needs no signing key,
network access or cycles and does not create or install canisters.

### 📦 Registry distribution

- Enable coordinated crates.io publication for the library, standalone canister,
  reference application and CLI; qualification packages remain private.
- Add package descriptions, READMEs, versioned local dependencies and the CLI’s
  exact tooling snapshot fingerprint to its archive. Install with
  `cargo install ic-blob-storage-cli --locked`; the executable remains `blob`.
- Use Toko’s shared release/publication engines with Blob’s complete local and
  installed gate, two-lock metadata projection and explicit direct delivery policy.
  Registry availability does not freeze the API or qualify Caffeine support.

### ➕ Offline application installation inputs

- Add `blob app prepare-installation` to encode the existing Blob and
  reference-application init frames with explicit pairing, users and limits.
  Reject invalid roles/bounds before writing; never replace completed or partial
  output. Include a readable description and hashes for review.
- The installed application journey now consumes the exact CLI-produced files.
  Proposed identities remain distinct from authenticated targets; signed install
  custody and deployment admission are still pending. No service or state format
  changes, funding, provider operations or legacy custody imports are introduced.

### 📚 Canic integration handoff

- Record Canic's completed immutable supplier pin and separate its existing adapter
  from remaining [Canic #12 — Fleet installation arguments](https://github.com/dragginzgame/canic/issues/12)
  and [Canic #13 — Blob fleet maintenance](https://github.com/dragginzgame/canic/issues/13).
  Clarify that standalone paired init files do not configure Canic's embedded
  catalog. These issues track Canic implementation; they are not completed here.

## [0.100.12] - 2026-11-16

GitHub development checkpoint; registry publication remains disabled.

### ▶️ Try it

From the repository root, prepare tools once with `make install-tools`. Then:

```sh
make blob
blob application --help
blob application catalog --help
blob application draft --help
```

Help shows `Blob 0.100.12` and the required application, network and signing-user
inputs. Help needs no credentials or network access. Actual inspection uses signed,
read-only network queries; it does not upload, publish or perform paid provider calls.

### 🔎 Reference-application inspection

- Add `blob application catalog` and `blob application draft --id ...` for the
  signing user's existing application records. Show the original root, asset,
  declared length and publication/cleanup disposition; escape asset names for
  terminal display. Unknown or other-user draft IDs fail without partial output.
- Reuse the application protocol and CLI query transport. No new canister endpoint,
  catalog state, publication command or provider operation is added. An installed
  two-user journey checks isolation, empty/published/cleaned views and unchanged
  application/Blob state after inspection.

### 🔧 Tooling and dependency declarations

- Adopt Toko Tooling 0.44.5 in both workspace pins/locks through its authenticated
  exporter. Keep the existing baseline and add the shared declaration checker's
  required runtime files to the managed snapshot.
- Add `make dependency-pins-check` to the Linux CI aggregate. Check both Cargo
  workspaces, inheritance, tracked lockfiles and action pins without treating the
  exact tooling pin or private qualification versions as policy exceptions.

### 🧪 Browser and application qualification

- Add opt-in `make test-browser-application BROWSER=/absolute/path/to/chrome`,
  joining actual Chromium proof verification, IndexedDB custody, serial upload,
  reload replay refusal and verified readback to the installed reference catalog.
  Upload acknowledgments alone leave the draft unpublished. Cancellation during
  readback prevents publication even when the late complete read verifies. Both
  journeys hold readback across supported stop/drain/upgrades of the application
  and callback host; original state/proof survive and activation stays explicit.
- Reuse the existing browser and installed-application fixtures. Rust owns signed
  application calls; no product state, public endpoint or dependency is added.
  Cleanup preserves outstanding provider exposure. This is local qualification,
  not a live Caffeine acceptance, snapshot recovery or browser signing witness.
- Preserve the existing Host FS/Artifacts 0.80.2 lock selections and qualify their
  direct CLI callers. Blob does not adopt the new retained-stage recovery API;
  incomplete custody still refuses replacement or automatic recovery.

## [0.100.11] - 2026-11-16

GitHub development checkpoint; registry publication remains disabled.

### ⚠️ CLI command paths

Replace flat command names in scripts with the grouped paths below. Stored request
records and service formats are unchanged. This explicitly selected 0.100.11
development checkpoint remains in the rewrite’s unfrozen 0.100 line.

### ▶️ Try it

From the repository root, prepare the tools once with `make install-tools`. Then:

```sh
make blob
blob --help
blob certificate --help
blob maintenance inspect --help
```

Help shows `Blob 0.100.11 : <description>` and the available command groups.
These help commands need no credentials and make no network or paid calls.

### 🧭 CLI navigation and hosted checks

- Reduce `blob --help` to `overview` and four command groups: `provider`,
  `certificate`, `transfer` and advanced `maintenance`. Group help shows the
  available operations and certificate/maintenance ordering. All help pages show
  `Blob <version> : <description>`, matching Canic's banner. Update scripts to
  the grouped paths; old flat spellings are removed. Custody and effect rules
  are unchanged, including recovery of records prepared by 0.100.10.
- Add GitHub Actions wiring for the existing Linux aggregate, native macOS tests,
  production Wasm builds and one full-history secret scan, addressing
  [BlobStorage #2 — Complete hosted CI wiring](https://github.com/dragginzgame/ic-blob-storage/issues/2).
  Reuse Toko 0.44.4's authenticated CI installers and compiler setup; no simulator,
  live-provider calls or deployment in routine CI. The only secret-scan exceptions
  identify five reviewed historical public test fixtures by exact fingerprint.
  Hosted acceptance remains pending the first pushed workflow run.

### 🔧 Native custody and dependencies

- Adopt published Host FS and Artifacts 0.80.1 for the CLI and direct qualification
  callers. Bounded reads use Artifacts; filesystem custody uses held directories,
  explicit create-only publication and nonblocking sidecar locks.
- Preserve request/intent formats and permanent claim names. Retain failed
  publication stages as `.NAME.pending`; never clean them up or retry implicitly.
  Known occupied outputs refuse before staging. Only confirmed publication permits
  subsequent dispatch. Ordinary operator input paths keep their prior semantics;
  custody paths still refuse traversal and symlinks.
- Testkit 0.106.0 remains the latest published release and internally requires
  Host FS/Artifacts/Process 0.79.0. Keep those private qualification edges; no
  public Host types cross into Blob's 0.80.1 callers. Process 0.80.2 adoption there
  requires a Testkit release. No unused Host packages or supplier patches added.

### 🧪 Testing

- Check held-directory replacement, bounded/no-follow custody reads, preserved
  ordinary input paths, duplicate claims and retained partial publication.
- Exercise grouped help without loading identities, and retain dispatch admission
  requirements. Installed certificate, transfer, catalog and maintenance journeys
  use the grouped commands, including custody prepared by the released 0.100.10 CLI.
- CI workflow validation and the history scan run locally; GitHub/macOS acceptance
  remains pending the first pushed workflow run. Release evidence belongs to the
  [current handoff](docs/status/current.md).

## [0.100.10] - 2026-11-15

GitHub development checkpoint on branch `0.100`; registry publication stays disabled.
Adds browser verification of original IC proof and a bounded permanent upload-claim
store, with real Chromium transfer and replay qualification.

### ▶️ Try it

With Node 24+ and an installed Chromium executable, run from the repository root:

```sh
make install-browser-tools
make test-browser-runtime BROWSER=/absolute/path/to/chrome
```

Expect real-browser multi-chunk transfer/read verification and refusal checks against two
owned loopback origins. It uses a disposable browser profile; no wallet or provider
access is needed; explicit setup installs locked npm dependencies without scripts.

### 🧪 Testing

- Qualify immutable preparation and read verification in actual Chromium using
  Web Crypto, cross-origin Fetch and Streams. Check corruption, short/excess bodies,
  partial status, redirect/CORS refusal, stream deadlines and pre-cancellation;
  the peer verifies one GET per attempted read. Keep this opt-in browser check
  separate from Node CI and from live-provider qualification.
- Exercise actual cross-origin PUT preflight, serial tree/chunk transfer and
  received-byte readback. The experimental strict IndexedDB claim commits before transfer
  and refuses replay after page reload, including when the final HTTP reply breaks
  after all bytes arrive. Explicit read-only reconciliation verifies those bytes
  without another PUT. Authenticate retained local IC proof through published
  Platform 0.2.0, then retain the original request/proof in a fixed 64-claim store.
  Refuse missing/replaced custody, changed bindings, altered proof and competing
  claims. No current authority, profile rollback or multi-device guarantee.
- Cancel after the first chunk reaches the real browser peer while its reply is
  pending. Check that no later chunk arrives, reload cannot restart the transfer,
  and read-only reconciliation rejects the incomplete bytes.

### 🔬 Provider and dependency evidence

- Both Caffeine upload routes return HTTP 200 to bounded public PUT preflights;
  advertised CORS is not positive upload admission. No object, proof or payment
  was issued. Keep the live owner unchanged pending a Storage-owned transition.
- Review committed Storage 0.1.7: native/checkpoint progress does not yet import
  the retained 0.74.1 image. The dirty supplier checkout remains untouched.
- Lock npm Platform 0.2.0 and qualification bundler esbuild 0.28.2, both current
  stable at selection. Both Cargo graphs select Wire 0.24.1; final-version CI and
  all eight installed suites pass. The first-party registry review found no newer
  stable suppliers than the selected versions. Native macOS remains unqualified.

## [0.100.9] - 2026-11-15

GitHub development checkpoint on branch `0.100`; not published to crates.io.
Adds local callback ownership, certificate custody and a minimal transfer/read
client joined to the application catalog. Live Caffeine admission remains unqualified.

### ▶️ Try it

With Node 24+ on PATH, run `make test-browser` from the repository root for the
minimal browser-client checks, including owned loopback HTTP. No npm installation,
credentials or live provider calls are needed. Expect immutable preparation,
serial transfers, original-root read verification and failure/refusal checks to pass.

From the repository root, one-time setup is `make install-tools`; build/install
the current CLI with `make blob`. For the local qualification issuer, retain an
original signed request offline with the uploader PEM:

```sh
blob prepare-certificate --url "$IC_API" --canister "$ORIGINAL_ISSUER" \
  --uploader "$UPLOADER" --identity "$UPLOADER_PEM" --root "$ORIGINAL_SHA256_ROOT" \
  --custody "$CERTIFICATE_CUSTODY" --root-key "$TRUSTED_ROOT_DER"
```

Expect a private permanent root claim and a printed request path; no request is
submitted. `blob dispatch-certificate --help` describes the separate mutating
step, which requires the same custody and an admitted checkout. After dispatch,
use that printed path as `original-request.cbor` below:

```sh
blob recover-certificate --url "$IC_API" --canister "$ORIGINAL_ISSUER" \
  --uploader "$UPLOADER" --identity "$UPLOADER_PEM" --root "$ORIGINAL_SHA256_ROOT" \
  --request original-request.cbor --output recovered-certificate.cbor
```

Expect a new private certificate file only for an authenticated matching historical
reply. This makes a read-state network call and writes the local output, but submits
no upload/update or paid provider request. Loopback requires `--root-key` from the
owned simulator. The root-only issuer profile is locally qualified; the product
standalone host does not export it and live provider acceptance remains open.

`blob claim-transfer --help` describes the new offline bridge from that retained
proof to one permanent transfer claim. See the
[full command](docs/operations/storage-inspection.md#claim-one-transfer-offline).
With Node 24+ and the local simulator installed, `make test-callbacks` exercises
the real CLI/client composition without a live provider.

### ✨ Added

- Add the minimal owned browser transfer client, independent of Storage and Canic.
  Prepare immutable bounded files before authorization, accept existing proof and
  require a caller-owned durable claim before serial PUTs. No automatic retries;
  HTTP acknowledgment remains distinct from verified availability and settlement.
- Add `verifyRead` against the original declaration, with exact length/EOF and root
  checks before returning immutable file bytes. Bound memory and read duration;
  downloaded headers cannot change expected metadata.
- Add `readVerified` for one explicit GET under a shared fetch/verification deadline.
  Snapshot the original declaration before network waits, refuse redirects and
  omit credentials. Read existing objects without issuing authority, replaying
  uploads, changing custody or automatically publishing a catalog entry.
- Add offline `blob claim-transfer`: authenticate the saved original proof, bind
  its digest and provider selectors, then publish a permanent Host FS claim before
  PUTs. A partial claim, new process or different gateway cannot restart the root.
- Add read-only `blob recover-certificate` for retained original ingress. Bind
  uploader, issuer, method and root before authenticating the original result;
  bound external decoding and publish proof through Host FS without overwriting
  evidence. Reuse Platform/Host FS instead of a generic recovery journal.
- Add offline `prepare-certificate` and guarded `dispatch-certificate`. Preserve
  one root-bound intent/packet and permanently claim dispatch before network I/O;
  refuse replacement, concurrent dispatch and partial custody. Agent HTTP 429/503
  retries reuse the same packet; this is not an exactly-one-HTTP-attempt guarantee.

### 🔧 Changed

- Adopt Toko Tooling 0.44.4 through the authenticated 49-file snapshot in both
  workspaces. Package directory names already match their Cargo identities; no
  layout changes are needed.
- Adopt IC Wire 0.24.0 in both independent locks. Migrate product and qualification
  callers to root-level `encode`/`decode` with explicit `Encoding::Payload` or
  method/direction frames; retain existing byte/work limits and endpoint identities.
  Remove the old `native` API use without a compatibility shim. Downstream Rust
  callers using that API must migrate too; this repository has no generated Wire
  TypeScript client to replace.

### 🧪 Testing

- Exercise a retained Wire 0.22.4 host and reference application before upgrading
  to the new artifacts, and original custody prepared by the retained CLI before
  current dispatch/recovery. These qualify Blob's exercised records and endpoints,
  not arbitrary Wire schema migrations or the live enrollment probe's upgrade.
- Reconcile a lost final upload response through a fresh reader process, original
  draft inspection and separate authenticated catalog publication. Use only the
  original peer's received bytes; qualify corrupt content and concurrent cancellation.
  No new issuance/PUT or Blob state change occurs during successful read/publication.
- Extend the catalog journey with a lost final upload response, interrupted
  readback and client-process termination during a chunk. Restart against the
  original claim refuses before any PUT, preserves proof/intent bytes and leaves
  the draft unpublished. Explicit cleanup still retains provider exposure.
- Join the real reference application to original proof recovery, durable transfer
  custody, six-chunk upload/read and catalog publication. A corrupt read never
  requests publication; cancellation during tree transfer or readback survives
  late results. Reuse the application API and state unchanged. `make test-callbacks`
  now builds its actual Wasm and runs this Node-assisted local composition.
- Connect the installed local issuer, recovered proof, actual native transfer
  claim, Node Fetch PUTs and verified readback. Corrupted proof sends no PUT;
  a fresh process refuses replay and final release retains unresolved exposure.
  Cover multi-chunk read corruption/truncation against a retained protocol vector.
- Qualify refusal of an incompatible enrollment-probe → Blob-host upgrade using
  both actual Wasm artifacts. Preserve module, controller, stable bytes and the
  consumed refusal receipt; replay remains blocked after a supported reopen.
  A copied installed image also refuses a second allocation under larger runtime
  limits. `make test-enrollment` now supplies both artifacts. This establishes a
  local lifecycle boundary, not a migration or live provider admission.
- Correct that upgrade regression's method-bound argument from `blob_upgrade` to
  `blob_post_upgrade`, so the refusal reaches storage admission instead of stopping
  at Wire decoding. The corrected installed check passes.
- Add a local `vec blob` callback candidate and `make test-callbacks`. Reuse Service
  for retention/accounting and a second bounded library-owned record for the
  immutable gateway binding and exact acknowledgments. Qualify unknown-root
  protection, overlapping references, full-history cleanup, malformed/unauthorized
  batches, retained exposure and upgrade replay. Product standalone exports and
  Service format stay unchanged; live provider compatibility remains unqualified.
- Mount the root-only certificate-reply candidate in the local qualification host.
  Check caller/root admission and synchronous commit-before-reply through Service,
  final release before client retrieval, and refusal of fresh issuance after upgrade.
  Recover the original certified request through Platform's Agent after release
  and acknowledgment; reject incorrect trust roots, altered proofs and mismatched
  reply/request bindings. No production issuer or provider PUT is added.
- Extend that witness with CLI preparation, competing dispatch processes and
  independent CLI processes after final release. Check unavailable status, wrong
  bindings/trust, partial output and repeat publication; reverify the saved proof.
- Qualify cancellation after signing but before dispatch, plus a real local
  gateway outage. Rejected and unknown outcomes publish no certificate, preserve
  original request custody and refuse resubmission after connectivity returns.
- Extend recovery past the original ingress expiry and authenticate a later
  certificate for the same reply with unchanged outstanding exposure. Distinguish
  grant, ingress, certificate and provider lifetimes in the design. This is local
  evidence, not Caffeine proof acceptance or a provider cutoff.
- Exercise the actual three-key enrollment/Service/callback composition against
  the installed probe image. Refusal preserves the original allocation identity,
  generation and readable receipt. Specify the missing Storage-owned capacity
  expansion separately; no raw-memory workaround or live upgrade is introduced.

## [0.100.8] - 2026-11-15

GitHub development checkpoint on branch `0.100`; not published to crates.io.
Product behavior, API and durable format are unchanged. The enrollment probe is
independently versioned at 0.100.8; existing qualification packages stay at 0.100.5.

### 🧪 Testing

- Create and reconcile one fresh, empty mainnet owner for Caffeine enrollment
  qualification within the approved cycle budget. Retain its identity and exact
  request custody for reuse. The temporary payment binding below is observed;
  usable Rust-host upload enrollment and uploads remain unqualified. No product
  API or deployment runner is added.
- Add a single-use owner enrollment probe and `make test-enrollment`. Retain
  exact intent and bounded raw replies across upgrades, refuse repeat dispatch,
  and preserve results when the operator loses authority during a call. Local
  installed checks use synthetic Cashier replies. Install the qualified probe
  on the existing mainnet owner and verify its module/configuration before its
  single provider attempt below; payer-link expiry and limits remain unqualified.
- Add an offline qualification example to encode lifecycle/method Wire frames
  and decode retained inspection replies. Reuse the named-identity signer;
  add no wallet-key export, product funding API or deployment runner.
- Recover relationship expiry/limit documentation by static inspection of the
  published provider CLI. Correct the earlier interpretation of a null target:
  it selects the default refill target, rather than disabling refill. Keep
  documented semantics separate from live enforcement evidence.
- Qualify one temporary payer link: it immediately allocates 1B of existing
  credit to the owner. Retain the owner-origin `TopUpWithoutCycles` refusal,
  nonzero Cashier-reported gateway budget and confirmed link removal. No upload,
  extra funding or closure claim; the probe remains consumed. Add an offline
  decoder regression from the actual reply.
- Record a bounded missing-object gateway read and absent callback-version
  observation. Specify the next local Service/callback qualification cases,
  including incompatible deletion-list shapes and preservation of the consumed
  probe's single-allocation state. These reads do not establish upload admission.

### ▶️ Try it

From the repository root, after `make install-tools` and the documented locked
dependency preparation, run `make test-enrollment` for local installed checks.
The offline helper below prints frames only; it needs no identity credentials,
network or cycles and does not install, sign or send anything:

```sh
cargo run --offline --locked --manifest-path qualification/Cargo.toml \
  -p ic-blob-storage-enrollment-probe --example enrollment -- prepare \
  --operator o5trf-oqyg7-cawjp-xs4pw-aomb3-iwki5-hyezf-qahfz-j3ffd-jh4fc-oqe \
  --cashier 72ch2-fiaaa-aaaar-qbsvq-cai --target-balance 1000000000
```

For a retained response, use the same example with
`decode --method inspect --reply-hex HEX`. These are private qualification frames,
not a live enrollment recipe. See the [probe contract and installation witness](docs/design/caffeine-contract-review.md#single-use-enrollment-probe--local-qualification-for-01008).
Classify the retained provider refusal offline with:

```sh
cargo run --offline --locked --manifest-path qualification/Cargo.toml \
  -p ic-blob-storage-enrollment-probe --example enrollment -- decode-top-up \
  --reply-hex "$(cat qualification/ic-blob-storage-enrollment-probe/examples/enrollment/fixtures/top-up-without-cycles.hex)"
```

This reports `Err` / `TopUpWithoutCycles`, without signing, sending or retrying.

## [0.100.7] - 2026-11-15

GitHub development checkpoint on branch `0.100`; not published to crates.io.
The CLI crate is now `ic-blob-storage-cli` and the executable is `blob`.
Uploader overview appends a Wire inspection variant; older hosts cannot serve it.
Durable state stays at format 1. Qualification packages remain independently 0.100.5.

### 🔧 Changed

- Rename the existing `ic-blob-storage-admin` crate to `ic-blob-storage-cli`
  and its executable to `blob`, with `make blob` / `make test-cli` and the
  `BLOB_CLI` qualification input. Keep one CLI; no compatibility binary is added.
- Make `make blob` build and install the development executable through Cargo
  into its normal bin directory on PATH. Re-running it updates the local source
  installation; `make build-cli` remains build-only for qualification and CI.
- Extend the existing admin CLI with a principal overview of one Blob installation,
  read-only Caffeine account storage inspection and an offline enrollment proposal.
  Shared roots count once; references remain application-owned and Closing history
  retains unresolved bytes. Add no persistent state, provider effects or CLI crate.
  Qualify full-history inventory through the installed host and a drained upgrade;
  distinguish provider authorization failures from empty storage.
- Establish a published independent Rust enrollment candidate from DFINITY's
  example and observe Caffeine's current public interface, gateway and prices.
  Update the design to proceed through independent qualification without provider
  outreach; owner enrollment, upload and closure remain unqualified.
- Validate fresh payer funding with a bounded 10.1B-cycle debit and acknowledged
  9.9B provider credit. Retain exact requests and distinguish payer credit from
  owner/gateway capacity; zero overdraft and a null target were reported. The
  0.100.8 investigation corrects null to mean the default refill target.
  No funding API, automatic refill or provider upload is added to Blob.
- Recover Canic legacy's historical Cashier ABI and self-funded Rust account path
  from pinned inventories and original Toko source. Record the deletion-list ABI
  difference and unsafe historical policies to exclude; live-provider gates remain open.
- Record Canic's committed optional Blob adapter and embedded catalog witness;
  advance D3 to partial using inspected source and retained installed evidence.
  Keep immutable supplier adoption, fleet lifecycle custody and provider transfer
  open. No supplier API or product code change is needed for the local composition.
- Adopt Toko Tooling 0.44.3 in both workspaces through its authenticated snapshot
  installer. Default `toko-cloc` and `-d 0` show repository totals; `-d 1` expands
  crates and `-d 2` expands modules. Retain the existing entrypoints and independent
  qualification; JSON counts are unchanged.
- Qualify the pending Platform 0.33.7 lock selection in native administration and
  installed signed-call/maintenance consumers. Runtime 0.22.5 and Types 0.20.4
  remain unchanged; preserve the existing Wire 0.22.3 selection.

### Validation

`make tools-check ci` passed, including 42 native behavior tests. All eight
installed cases passed across standalone, capacity, endpoints, signed calls,
reference application and management/overview targets. `make blob` installed
`blob 0.100.7` on PATH. Evidence is from Linux with Rust 1.100.0; it does not
qualify live provider closure or native macOS installation.

### ▶️ Try it

After `make install-tools`, run `toko-cloc` or `toko-cloc -d 0` for repository
totals, `toko-cloc -d 1` for crates, and `toko-cloc -d 2` for modules.

From the repository root, install with `make blob` (normally `~/.cargo/bin/blob`).
Cargo's bin directory must be on PATH; repeat the command to update. Inspect available
inputs without credentials or network access:

```sh
blob overview --help
blob provider-inspect --help
blob prepare-enrollment --help
```

For a real Blob inventory, set `BLOB_CANISTER`, `UPLOADER` and `APPLICATION_PEM`
to your installation, selected uploader and its configured application authority's
Ed25519 PEM, then run:

```sh
blob overview --url https://icp-api.io \
  --canister "$BLOB_CANISTER" --principal "$UPLOADER" --identity "$APPLICATION_PEM"
```

This signed read-only query reports local roots, grants, references and declared
accounting, not verified provider files. Caffeine inspection requires access to
the selected account. See [storage inspection](docs/operations/storage-inspection.md)
for provider commands, trust inputs and offline enrollment; no command above spends cycles.

## [0.100.6] - 2026-11-15

GitHub development checkpoint on branch `0.100`; not published to crates.io.
Product API and durable formats are unchanged. Qualification packages retain
their independent private versions at 0.100.5.

### Added

- Qualify reference-app upgrade refusal for empty, invalid and foreign stable
  memory, preserving rejected bytes and the original application's catalog and
  Blob state. Extend the installed journey without new runtime code or restore APIs.
- Add a standalone maintenance runbook with the existing admin command sequence,
  original-artifact custody and interruption decisions; replace the long README
  command reference with a link. No runtime or recovery behavior changes.
- Add `make test-capacity` for the actual standalone Wasm at 64 distinct objects,
  64 shared uses and 64 reservations. Record execution-cycle charges and allocated
  memory, and qualify inspection, cleanup and drained upgrade at full history.
  Preserve the lifetime limits and unresolved exposure; no product code is added.
- Qualify reference-app upgrade refusal during a real pending Blob call, preserved
  callback completion and stable state, and rejection of replacement installation
  arguments. Extend the existing installed journey without new product code.
- Qualify all five reference-app endpoints rejecting wrong-method, trailing-byte
  and oversized frames without changing application or Blob state. Invalid draft
  bounds also preserve the ID for a subsequent valid request.
- Specify the Canic adapter handoff against the existing Service API: one granted
  store, distinct uploader/application authority, atomic local catalog changes and
  composed lifecycle qualification. No adapter or Canic dependency is added here.

### Changed

- Move installed qualification into its own Cargo workspace and lockfile, with
  packages directly under `qualification/` and independent private versions.
  Update build, simulator setup, formatting and CI commands for both workspaces;
  preserve supplier selections through the split and keep private versions
  independent of the product checkpoint.
- Include the pending Platform 0.33.6 and Wire 0.22.2 patches, plus thiserror
  2.0.21 in the product lock (qualification retains 2.0.18). No broad dependency
  refresh accompanies this checkpoint.

### Fixed

- Finish reference-app cancellation when the exact Blob authorization conclusively
  expired without admission. Previously these drafts stayed pending forever.
  Admitted grants still require release after expiry; uncertain outcomes stay pending.

## [0.100.5] - 2026-11-14

GitHub development checkpoint on branch `0.100`; not published to crates.io.

### Added

- Add a standalone reference application with durable user-owned drafts, catalog
  publication and irreversible cancellation before remote reference cleanup.
  `make test-application` exercises the real application and Blob Wasm together,
  including delayed publication, interrupted cleanup and application upgrades.
  Standalone use remains independent of Canic.
- Add `make test-signed` to qualify application/uploader identity separation through
  the installed host's local HTTP gateway, verified responses, original-upload
  inspection after a discarded registration reply and cleanup without resurrection.
  This changes no product API or dependencies and makes no provider-recovery claim.

### Changed

- Adopt Toko Tooling 0.43.2 through the authenticated snapshot installer, Host FS
  0.79.0 (with matching Artifacts/Process), and Testkit 0.106.0. Qualify the final
  checkpoint lock's Platform 0.33.5 and Runtime/macros/guard 0.22.5 patches; preserve
  existing filesystem/simulator interfaces without compatibility wrappers.
- Reconcile the design with the implemented host and full native maintenance path.
  Replace accumulated implementation history with current rules, a compact legacy
  reuse record and explicit D0–D4 evidence gaps; retain detailed history by revision.
- Replace retiring Wire Endpoint attributes with Platform's generic guard and
  codec hooks in the standalone host. Preserve method IDs, bounded Wire frames,
  caller policy, reply traps and lifecycle/storage behavior; Canic stays optional
  ([BlobStorage #1](https://github.com/dragginzgame/ic-blob-storage/issues/1)).
- Remove `ic-wire-endpoint` and its macro package from the dependency graph.
- Add `make test-endpoints` for installed raw-frame compatibility and a separate
  qualification-only reply probe for rollback after encoding failure. Standalone
  upgrade tests accept `BLOB_PREVIOUS_WASM` to qualify a real predecessor artifact.

### ▶️ Try it

Run the standalone reference application and Blob host together in local PocketIC:

```sh
make install-tools
make test-application
```

See the [reference application guide](docs/design/reference-application.md) for
its endpoints, limits and cleanup rules. This journey makes no provider calls.

## [0.100.4] - 2026-11-14

GitHub development checkpoint on branch `0.100`; not published to crates.io.

### ➕ Added

- Extend original-request custody to stopping a drained host. Stop requires the
  current closed maintenance attempt and manager/controller identity; observation
  checks platform status without restarting the canister or renewing the request.
- Prepare, dispatch and observe one ordinary upgrade linked to the acknowledged
  stop request. Bind exact replacement Wasm and fixed arguments, preserve a durable
  claim before transmission, and require the original outcome even for same-Wasm
  upgrades. Rejection never triggers reinstall, restart or another submission.
- Add start request custody linked to the original stop and upgrade. Validate both
  dispatch claims, then verify the running host's replacement artifact, configuration,
  format and current drained attempt. Historical start acknowledgment cannot approve
  an active or later attempt; automatic activation remains absent.
- Add explicit activation chained to verified reopening and the same custody root.
  Require original start acknowledgment and all predecessor claims, retain the
  activation request before dispatch, and distinguish its historical outcome from
  current admission. A delayed activation cannot reopen a newer drain.
- Extend native lifecycle qualification with reserved, live and closing objects,
  overlapping/released references and two uploaders. Upgrade to a distinct Wasm
  hash using labelled custom metadata, then check preserved declarations, permissions,
  accounting, historical replay and cleanup before accepting fresh work.

### 🔧 Dependencies

- Adopt Platform 0.33.3 in the root catalog, matching the existing lock selection.
  Preserve the existing Runtime 0.22.4 / Types 0.20.4 graph and other dependencies;
  enable management types only for the native admin. No Blob API migration is needed.

## [0.100.3] - 2026-11-14

GitHub development checkpoint on branch `0.100`; not published to crates.io.

### ▶️ Try it

After `make install-tools`, run `make test-management` to exercise the native
inspection and guarded drain commands against a disposable local host through signed IC requests.
For its explicit inputs, run `cargo run --offline --locked -p ic-blob-storage-admin -- inspect --help`.
`prepare-drain --help` and `observe-drain --help` describe immutable request
preparation and original-outcome observation; neither command submits the drain.
`dispatch-drain --help` describes the tooling checkout and shared custody root
required to submit that original request.

### ➕ Added

- Add a native management inspection command. It authenticates the installed
  Wasm hash and replicated host response against an explicitly supplied trust root,
  then compares the admitted identity, configuration and record format. It reports
  current maintenance state without activating or upgrading the installation.
- Qualify the actual binary through a local IC HTTP gateway, including wrong-root,
  artifact, configuration, format and caller refusals. Provider operations remain
  outside this slice.
- Prepare one signed drain request in a new private claim directory, binding its
  target, trust root, configuration, artifact and exact maintenance attempt.
  Observe its authenticated original outcome without renewal or resubmission;
  incomplete claims and unavailable outcomes remain unresolved.
- Dispatch that original drain under an exclusive target lock after tooling and
  host admission. A durable per-attempt claim precedes submission and bars another
  request ID from bypassing uncertainty. Installed qualification covers competing
  processes, tooling refusal and a simulated interruption before transmission.
  Stop/upgrade/start and activation remain unimplemented.

### 🔧 Tooling

- Adopt Testkit 0.105.0 and rebuild its matching server helper. Our fresh-instance
  tests already avoid the retired baseline-pool APIs; PocketIC stays at 16.1.0.
  Run `make install-test-server` to refresh the prepared helper after updating.

## [0.100.2] - 2026-11-14

GitHub development checkpoint on branch `0.100`; not published to crates.io.

### ▶️ Try it

From the repository root, prepare the Rust 1.100.0 `wasm32-unknown-unknown`
target and run `make install-tools` once. Setup downloads authenticated tooling
and PocketIC. Then run `make test-standalone`: it builds the production Wasm and
checks caller refusals, shared references and state preservation across local
upgrades. No credentials, Caffeine calls or paid provider effects are involved.

### ✨ Added

- Add an experimental thin standalone host and its installed PocketIC test package.
  The library persists a bounded versioned control record through IC Storage,
  preserving historical identities and unresolved declared exposure across upgrades.
- Gate new work during maintenance while allowing cleanup and historical replay.
  Upgrades require a prior drain; activation binds the current attempt, so an old
  activation cannot reopen a later drain. Provider integration and the production
  management runner remain unqualified; API and persistence remain experimental.
- Add manager-only replicated inspection of the installed identity, immutable
  configuration, state format and current maintenance attempt. Inspection does
  not rewrite storage; installed tests check caller refusal and preservation
  across repeated upgrades. This is a management preflight input, not a continuity proof.

### 🔧 Tooling

- Update the exact Toko Tooling selection and authenticated managed snapshot to
  0.43.1. Existing dependency, formatting and LOC commands remain unchanged.

### 📚 Documentation

- Align issuance/release races with Caffeine's synchronous certified-reply path.
  Retain exposure for delayed original replies; recovery uses the original proof
  or reports blocked, with no fresh-issuance fallback or fictitious acquisition call.
- Trace an independent public Caffeine consumer's self-funded account setup and
  narrow the Rust enrollment question. Keep provider support, enforcement and
  closure unqualified; adopt no third-party code or automatic funding machinery.
- Specify one admission gate during draining/reopening, including cleanup and
  late-result handling. Bind activation to the current maintenance attempt so a
  delayed request cannot reopen a later drain.
- Define lost stop/upgrade/start reply handling and forbid automatic destructive
  recovery. Record legacy lifecycle lessons; the local host now exercises ordinary
  upgrades, while the production management runner remains future work.

## [0.100.1] - 2026-11-14

GitHub development checkpoint on branch `0.100`; not published to crates.io.

### ⚠️ Changed

- Give each admitted root one permanent upload lifetime, including cancellation
  and expiry before registration. New IDs/uploaders cannot restart that root;
  existing Open objects still accept distinct references. Reserved bytes are
  released without forgetting the root or claiming provider settlement.
- Replace local upload-ID registration with root-based registration through the
  root's original grant. Additional uses cannot replace its uploader or deadline;
  the local transition still emits no upload authority.
- Return the immutable root together with current status from reference inspection.
  Catalogs can resolve asset targets without duplicating Blob's reference mapping;
  released/retired targets remain inspectable by the application authority.

### ➕ Added

- Add application-only inspection of a known root's original grant, retention and
  live-reference count. Distinguish unregistered retirement, overlapping live uses
  and Closing without claiming provider deletion or settlement.
- Add bounded streaming verification against the application's original Caffeine
  root, length and metadata. Adapt the legacy implementation into one local
  verifier, without retaining file bytes or introducing transfer/recovery state.
- Add bounded manifest preflight using the same tree/metadata rules. Reject
  inconsistent declarations without retaining their chunk lists; success does
  not authenticate declared length or establish provider-enforced limits.
- Add an experimental local service model for application-authorized grants,
  uploader-only registration, shared object references and declared-byte limits.
  Cancellation/expiry preserve retired identities; final release retains unresolved
  usage. No persistence, certificate issuance or settlement is implemented.
- Add an offline application example that verifies a multi-chunk read before
  catalog publication, shares an object between assets, and releases abandoned or
  rejected uses without claiming provider settlement. Run `make example-local`.
  Exact publication retries preserve the committed use, while conditional removal
  protects replacements from delayed requests; released uses cannot be republished.

### 🧪 Testing

- Add `make test-caffeine` and twelve integrity tests covering frozen SDK vectors,
  uneven trees, metadata normalization, corrupt/truncated/reordered content and
  rejection without state changes. Keep provider availability and admission
  enforcement outside these local proofs.
- Cover manifest count/work limits, reordered/substituted leaves and metadata,
  and consistent manifests with false declared lengths.
- Add `make test-local` with eleven composed lifecycle cases covering cross-user
  refusal, immutable replay, duplicate object use, cancellation/registration order,
  stale releases, expiry and cleanup at byte/record capacity.
- Add a delayed-authorization regression: an unknown reference is not completed
  cancellation; resolve the original grant and preserve any registration exposure
  before cleanup, without resurrection on late retries.
- Cover permanent root retirement after cancellation/expiry and update the offline
  walkthrough to cancel a distinct small object before sharing the multi-chunk one.
- Check root resolution with distinct reserved grants, uploaders, deadlines and
  first references; wrong-root authority and attached-user registration refuse.
- Check reference targets across lifecycle transitions, refused rebinding and
  exhausted capacity, and resolve published example assets through inspection.
- Check object inspection through reservation, cancellation/expiry, overlapping
  references and final release, including caller refusal and full-capacity reads.
- Add four example checks for read completion, cancellation after verification,
  exact request binding, and catalog refusals that preserve existing uses.
  `make test-example-local` selects these checks; CI also runs the walkthrough,
  including publication replay and stale removal after an asset-name replacement.

### 📚 Documentation

- Specify local versus provider enforcement for upload declarations and the
  qualification sequence for size mismatches, lost authority and final release.
  Incorporate the legacy deletion follow-up without treating it as closure proof.
- Define the standalone application/host call boundary and remote catalog cleanup
  ordering, including cancellation during unresolved authorization and the
  provider root-only request's ambiguity across successive grants.

## [0.100.0] - 2026-11-14

GitHub development checkpoint on branch `0.100`; not published to crates.io.

### ⚠️ Breaking

- Select 0.100.0 for the clean-start rewrite of the existing `ic-blob-storage`
  crate. Legacy APIs and persistence formats are not retained; this development
  version remains unpublished and is not an upgrade path for legacy installations.

### Setup

- Establish an independent, unpublished rewrite workspace with Toko Tooling
  0.43.0. Preserve the prior implementation and recovery custody in the private
  `ic-blob-storage-legacy` repository. Bootstrap began without product behavior.

### 🔧 Tooling

- Expose `make update` through Toko Tooling's shared dependency updater and
  document package inventory, update selectors and `toko-cloc`. Confirm the
  installed 0.43.0 snapshot is the latest non-yanked stable registry release.

### ✨ Features

- Implement bounded in-memory references for one registered object: overlapping
  uses, replay without resurrection, irreversible closing and cleanup at capacity.
  Keep provider identity, authentication and persistence outside this first slice;
  add no product dependencies or installed-service claims.

### 🧪 Testing

- Add seven reference behavior tests, run with `make test-references` and included
  in CI. Adapt legacy lifetime/release regression scenarios to the new replay
  semantics without importing its framework or provider state.

### 📚 Documentation

- Record the accepted library/standalone-host layout and installed-test boundary.
  Keep qualification packages deferred until there is a real host to exercise;
  clarify that persistent service state belongs to the library.
- Trace the candidate Caffeine protocol and legacy standalone evidence in the
  [provider contract review](docs/design/caffeine-contract-review.md). Identify
  reusable legacy rules, SDK capability gaps and unresolved provider guarantees
  before selecting the upload service API or persistence.
- Record accepted rewrite scope and revise the [design](docs/design/ic-blob-storage-design.md)
  with provider feasibility gates, unique object ownership, application reference
  permissions, closure/accounting rules and a restricted management lifecycle.
  API, persistence and provider qualification remain open.
- Require targeted legacy reuse evaluation and an architecture check after every
  push, with findings and justified retention recorded in the delivery handoff.
