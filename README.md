# IC Blob Storage

Clean-start rewrite workspace. [Bounded object references](crates/ic-blob-storage/src/references/mod.rs)
support overlapping application uses, historical replay and irreversible closing.
[Streaming Caffeine verification](crates/ic-blob-storage/src/caffeine/mod.rs) checks
downloaded bytes against the application's original root, length and metadata
without retaining the file. It uses the reviewed SDK 1.1.2 tree profile and `sha2`.
[Manifest preflight](crates/ic-blob-storage/src/caffeine/manifest.rs) checks bounded
chunk counts, metadata and tree consistency without retaining the chunk list.
It does not authenticate the declared byte length or authorize an upload.

The [experimental local model](crates/ic-blob-storage/src/local/mod.rs) composes
these rules with application/uploader permissions, exact grant replay, shared
root ownership and separate reserved/live/unresolved declared-byte accounting.
Each admitted root has one upload lifetime. Cancellation or expiry frees its byte
reservation but permanently prevents restarting that root, even under another
uploader or upload ID. Open objects still support additional application references.
Local registration resolves a decoded root to that original grant and checks its
uploader and deadline; an attached use cannot obtain another registration right.
Application-only reference inspection returns the bound root and current status,
including after release, so a catalog can identify assets from its reference IDs.
Known-root inspection reports the original grant, local retention and live-use
count, keeping released references distinct from a fully Closing object.
It emits no certificates and cannot settle provider obligations. Its heap state
must not back real uploads; API, provider identity and persistence stay unfrozen.

Run `make example-local` for the [offline application walkthrough](crates/ic-blob-storage/examples/local_asset/main.rs).
Two synthetic users share one 5 MiB + 17 byte object between two catalog assets.
It demonstrates cancellation, verified publication, corrupt-read refusal, cleanup
after a catalog collision and final release with unresolved declared usage retained.
Exact publication retries preserve the existing use; commit rechecks liveness and
removal compares the expected reference so delayed requests cannot remove a replacement.
The application owns its catalog; Blob owns its grants and references. No service
or SDK transfer is installed or contacted by this example.

The [durable service](crates/ic-blob-storage/src/local/persistence/mod.rs) now backs
the same rules with a bounded, versioned IC Storage record. The experimental
[standalone host](crates/ic-blob-storage-canister/src/lib.rs) captures IC callers
and time, and mounts `blob_update` / `blob_query` using Wire. Its 64 upload and
64 reference slots include historical identities. Reopening rebuilds indexes and
accounting from validated records; no empty-state fallback exists.

The configured manager drains admission before upgrades and activates the exact
current maintenance attempt afterward. Cleanup and historical replay remain
available while drained. The manager-only `blob_management` Wire update reports
the bound installation principal, immutable configuration, record format and
current maintenance state without writing storage. Management callers must
authenticate its replicated result; the value alone proves neither module identity
nor continuity after a snapshot restore.
The host emits no Caffeine certificate or provider call.
Provider-object resolution and supported live account binding remain unimplemented.
The native maintenance runner below is locally qualified; fresh provisioning and
live deployment are not. API and persistence are experimental;
these local tests do not establish D0/D1 completion or provider closure.

The native [admin command](crates/ic-blob-storage-admin/src/main.rs) supports
authenticated inspection and guarded drain/stop/upgrade/start/activation. Run
`make test-management` for the actual binary against
a disposable local IC HTTP gateway, or view its inputs with:

```sh
cargo run --offline --locked -p ic-blob-storage-admin -- inspect --help
```

`inspect` requires the endpoint and canister, manager Ed25519 PEM, independently
trusted network root key (DER), original `blob_init` Wire request, reviewed
uncompressed Wasm and expected state format (`1`). It never discovers a trust root
from the endpoint. It verifies the module hash before and after the replicated
inspection and refuses mismatches. Success reports observations, not continuity
or permission to activate. `prepare-drain` takes the same target inputs plus
`--attempt` and `--record`: it writes one original signed request and its binding
into a new private directory, without submitting it. `observe-drain` takes those
target inputs and the original `--record` and authenticates that request's status.
It never renews or resubmits it. An empty/partial directory stays claimed; preserve
it on failure. Unknown, pending or forgotten outcomes are refusals, not permission
to retry. A historical drain acknowledgment does not describe current admission.
The claim's parent must exist and remain under the operator's exclusive control;
inputs must be bounded regular files. A prepared request expires after two minutes,
but expiry neither deletes the claim nor establishes what happened to it.
`dispatch-drain` takes the original target inputs and `--record`, plus `--custody`
and `--checkout`. It admits the trusted checkout's tooling snapshot, locks the
target, rechecks its configuration and active attempt, and durably claims the
attempt before submitting the original unexpired packet. All managers must use
one operator-controlled custody root. Preserve it and the request directory:
even an empty dispatch claim bars another submission or replacement request.
After interruption, use `observe-drain`; an unknown outcome cannot authorize retry.
This deliberately may strand a request that was never sent. The checkout and its
execution environment are trusted inputs, not a sandbox for arbitrary Makefiles.
`prepare-stop`, `dispatch-stop` and `observe-stop` use the same target inputs and
request custody. Preparation takes the exact current drained `--attempt`; dispatch
also takes the shared `--custody` and trusted `--checkout`. The manager must be a
platform controller. Stop refuses active admission or a newer attempt and records
its own per-attempt claim under the same target lock. Observation works while Blob
is stopped: it reports original request status and authenticated platform state
separately. A stopped observation does not establish that the original request
executed or that restarting is safe. No automatic start or stop resubmission exists.
`prepare-upgrade`, `dispatch-upgrade` and `observe-upgrade` also take `--stop-record`
and `--next-wasm`. Keep `--wasm` as the original artifact; supply the reviewed
replacement separately (uncompressed, at most 1 MiB). Preparation requires the
original stop acknowledgment and a stopped target; dispatch additionally matches
that stop's durable claim in the shared custody root. The request uses ordinary
upgrade with the fixed Wire post-upgrade argument. No install/reinstall, argument
override, skipped hook or state replacement option exists. Observation requires
the original upgrade acknowledgment and replacement module hash, even for a
same-Wasm upgrade. Rejection or uncertainty preserves custody and bars redispatch.
`prepare-start`, `dispatch-start` and `observe-start` retain the same inputs and
add `--upgrade-record`. Start requires acknowledged original stop and upgrade
requests plus a stopped target with the replacement artifact. Dispatch checks both
predecessor claims under the shared lock before claiming its own request. After
starting, it verifies the running platform, replacement artifact, configuration,
record format and current drained attempt. `observe-start` repeats that verification
against the original start acknowledgment; running state alone is insufficient.
An active or newer attempt refuses, and no command automatically activates Blob.
`prepare-activate`, `dispatch-activate` and `observe-activate` add `--start-record`.
Activation requires the original start acknowledgment and the current drained
attempt on the admitted replacement host. Dispatch checks the stop, upgrade and
start claims under the same lock, then claims the activation before sending it.
The host checks the manager and exact attempt again at execution. Observation
requires the original activation acknowledgment and matching current active state;
a later drain invalidates that observation without authorizing another submission.
There is no restore command. Native transport/crypto dependencies stay out of the
canister graph.

`make test-management` exercises both same-Wasm maintenance and a populated lifecycle
with a different artifact hash. The latter adds only a labelled custom section to
the production Wasm: it tests artifact binding and preserved obligations, not a
schema migration or compatibility with arbitrary replacement code.

The previous implementation, Git history and provider/recovery custody are in
`../ic-blob-storage-legacy` (see its `LEGACY.md`) and the private
[legacy GitHub repository](https://github.com/dragginzgame/ic-blob-storage-legacy).
Existing crates.io releases remain the legacy line. The rewrite's first GitHub
checkpoint is `0.100.0`; the current checkpoint is `0.100.5`, inherited from the root
workspace version. Publication remains disabled. This is a new breaking release
line, not a registry version reset or a migration for existing installations.

The workspace has four product crates under `crates/`, Rust 1.100.0 / edition 2024
and published Toko Tooling 0.43.2. GitHub development uses the `0.100` branch of
`dragginzgame/ic-blob-storage`. There is no release/deployment configuration.

The [selected crate layout](docs/design/ic-blob-storage-design.md#selected-crate-layout)
keeps the host thin. Installed tests under `qualification/crates/` use the real
production Wasm and share the root workspace/lockfile. Domain tests remain with
the library. Canic owns its adapter and composed tests in its own repository.
The standalone host uses Platform's generic entrypoint hooks and bounded Wire
codecs directly; neither Wire Endpoint nor Canic is required.

```sh
make install-tools
make test-references
make test-caffeine
make test-local
make example-local
make test-example-local
make test-standalone
make test-endpoints
make test-signed
make test-application
make deps-list
toko-cloc
```

Run these from the repository root. `make install-tools` explicitly downloads and
authenticates the selected tools and local PocketIC server; `make install-test-server`
can prepare that server alone. `make test-standalone` lints/builds the production
Wasm and exercises callers, cleanup and repeated upgrades in a fresh local instance.
It requires `wasm32-unknown-unknown` and makes no Caffeine calls. `make ci` is the
separate aggregate native gate; installed testing is an explicit command.
Set `BLOB_PREVIOUS_WASM=/absolute/path/to/previous.wasm` for a predecessor-to-current
upgrade witness. `make test-endpoints` checks raw frames on the actual host and
uses a separate, small qualification canister to force reply-encoding failure
after a stable write. The latter tests rollback without a production fault mode.
`make test-signed` exercises the production host through an owned local HTTP gateway
with separate application, manager and uploader signing identities. It checks caller
separation, original-upload inspection after a discarded reply, replay refusal and
reference cleanup, with response verification against the fixture's independent root.
It models reply loss by discarding a completed response; it supplies no certificate
recovery, live provider or application-canister forwarding guarantee.
`make test-application` runs the [standalone reference application](docs/design/reference-application.md)
and the real Blob host. The application owns bounded durable drafts and a per-user
catalog; Blob remains the reference/accounting owner. The test exercises replicated
inspection, cancellation during publication, interrupted cleanup, shared objects,
capacity exhaustion and application upgrades. It performs no provider transfer.

`make deps-list` lists workspace package versions and release groups. `make update`
delegates to Toko Tooling's `make deps-update` (`cargo update -v`), updating locks
within the manifest requirements. To select one dependency, use
`make update DEPENDENCY_PACKAGE=toko-tooling`; `DEPENDENCY_PRECISE` optionally
selects a version allowed by its manifest requirement.

Toko Tooling remains exactly pinned. Adopting a newer release requires updating
the root pin and Cargo.lock, then running `TMPDIR=/tmp make toko-tooling-preview` followed by
`make toko-tooling-update`. `toko-cloc` uses this checkout's authenticated reporter
through the installed PATH launcher and reports product/test/tooling LOC separately.

The [design](docs/design/ic-blob-storage-design.md) records the accepted scope and
ownership: a Caffeine-first library usable independently and with Canic. Revision
2 specifies proposed identity, permission, closure, accounting and lifecycle rules.
Supported independent Caffeine access is the first feasibility gate; API and
persistence remain unfrozen. The local reference component does not close that
gate or qualify the standalone service journey.

The [Caffeine contract review](docs/design/caffeine-contract-review.md) records the
observed protocol, selective legacy reuse decisions and remaining enrollment,
SDK recovery, enforcement and closure requirements for the next implementation.

[AGENTS.md](AGENTS.md) and [the handoff](docs/status/current.md) describe the setup
and scope.
