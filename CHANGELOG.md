# Changelog

## [0.100.8] - Unreleased

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
  owner/gateway capacity; zero overdraft and disabled refill were already reported.
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
  ([#1](https://github.com/dragginzgame/ic-blob-storage/issues/1)).
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
