# IC Blob Storage

Development uses **ic-storage 0.78.0**. Hosts install ordinary Cargo Wasm and
grant Blob a named stable-memory allocation. This layout supports fresh installs
and upgrades from the same stable layout. [Storage contract](docs/design/stable-storage.md)
and [current checks](docs/status/current.md).

Clean-start rewrite workspace. [Bounded object references](crates/ic-blob-storage/src/references/mod.rs)
support overlapping application uses, historical replay and irreversible closing.
[Streaming Caffeine verification](crates/ic-blob-storage/src/caffeine/mod.rs) checks
downloaded bytes against the application's original root, length and metadata
without retaining the file. It uses the reviewed SDK 1.1.2 tree profile and `sha2`.
[Manifest preflight](crates/ic-blob-storage/src/caffeine/manifest.rs) checks bounded
chunk counts, metadata and tree consistency without retaining the chunk list.
It does not authenticate the declared byte length or authorize an upload.

The experimental [browser transfer client](clients/browser/README.md) prepares an
immutable file before authorization and accepts an existing authenticated proof.
It sends serial PUTs after a caller-owned durable claim, with no automatic retries.
It verifies complete reads against the original declaration before returning an
immutable Blob; `readVerified` can reconcile an uncertain upload using only a GET,
followed by the owner's separate catalog decision. It needs neither Canic nor
Storage. Optional Platform-backed proof authentication and a bounded IndexedDB
claim store support one trusted browser installation. Prepare with
`make install-browser-tools`, then run `make test-browser` (Node 24+). Browser
deployment and live Caffeine acceptance remain unqualified.

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
the same rules with a bounded, versioned stable cell. The experimental
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

The native [CLI](crates/ic-blob-storage-cli/src/main.rs) supports
[principal storage overviews](docs/operations/storage-inspection.md), owner-only
reference-application draft/catalog queries, proposed-target checks and offline paired installation inputs,
Caffeine account inspection, offline
enrollment proposals and guarded drain/stop/upgrade/start/activation. The
[standalone maintenance runbook](docs/operations/standalone-maintenance.md) gives
the exact inputs, command sequence and interruption decisions. It covers the
locally qualified experimental host; fresh provisioning and live deployment remain
unqualified. Run `make test-management` for its local gateway witnesses, or install
the executable into Cargo's bin directory (normally `~/.cargo/bin`, on PATH) and
view command inputs with:

```sh
make blob
blob --help
blob overview --help
blob app --help
blob maintenance inspect --help
```

The [CI workflow](docs/operations/continuous-integration.md) runs local checks on
Linux, native behavior on macOS, production Wasm builds and one history secret scan.
Installed simulator and live-provider work remain separate.

The previous implementation, Git history and provider/recovery custody are in
`../ic-blob-storage-legacy` (see its `LEGACY.md`) and the private
[legacy GitHub repository](https://github.com/dragginzgame/ic-blob-storage-legacy).
Crates.io versions through `0.82.1` are the legacy line. The rewrite started with
GitHub checkpoint `0.100.0`; `0.101.0` is its first published registry release. All
four product crates share the root workspace version. This is a breaking rewrite,
not a migration for existing installations. See the [release procedure](docs/operations/releases.md).

Install the published CLI with `cargo install ic-blob-storage-cli --locked`; the
executable is `blob`. Rust 1.100.0 or newer and Cargo’s bin directory on PATH are
required. Guarded maintenance still needs the matching source checkout and tools.

Blob’s public Rust records use Platform Types 0.21.2, with Runtime 0.23.17
in the hosts, Platform 0.34.15 for signed transport/proof custody and Protocol 0.3.5
for Caffeine and management codecs. No legacy Platform remains in the product.
Installed tests select Testkit 0.110.0 and Host Process 0.81.0.
See the [integration boundary](docs/design/application-boundary.md#types-0212-integration).

The workspace has four product crates under `crates/`, Rust 1.100.0 / edition 2024
and published Toko Tooling 0.45.8. Both workspaces select IC Wire 0.25.0's root-level
`Encoding` API; no old `native` API adapter is retained. GitHub development uses the `main` branch of
`dragginzgame/ic-blob-storage`. Releases use the shared Toko workflow; there is no production deployment configuration.

The [selected crate layout](docs/design/ic-blob-storage-design.md#selected-crate-layout)
keeps the host thin. Installed tests under `qualification/` use the real
production Wasm through a separate Cargo workspace and lockfile. Its unpublished
packages have independent versions; the root workspace contains only product crates.
Formatting, dependency inventory and lock updates cover both workspaces. Domain tests remain with
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
make test-capacity
make test-endpoints
make test-signed
make test-application
make test-enrollment
make test-callbacks
make deps-list
toko-cloc
```

Run these from the repository root. `make install-tools` explicitly downloads and
authenticates the selected tools and local PocketIC server, and installs locked
browser npm dependencies without install scripts; `make install-test-server`
can prepare that server alone. `make test-standalone` lints/builds the production
Wasm and exercises callers, cleanup and repeated upgrades in a fresh local instance.
It requires `wasm32-unknown-unknown` and makes no Caffeine calls. `make ci` is the
separate aggregate native gate; installed testing is an explicit command.
`make test-capacity` measures full-history resource use and verifies cleanup and
upgrade at capacity; see the [results and limits](docs/design/standalone-capacity.md).
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
`make test-enrollment` checks the [single-use enrollment probe](docs/design/caffeine-contract-review.md#single-use-enrollment-probe--local-qualification-for-01008)
against synthetic Cashier peers, including concurrent replay, revoked authority,
bounded raw replies, durable reopening and refusal of an incompatible upgrade to
the actual Blob host without losing the receipt. It neither installs on mainnet nor
qualifies Caffeine's payment, upload or charge-closure contract.

`make test-callbacks` checks the [local callback candidate](docs/design/caffeine-contract-review.md#local-callback-candidate--development-01009)
over the same durable Service. It selects `vec blob`, protects unknown roots,
restricts deletion listing/acknowledgment to one configured gateway, and retains
acknowledgments across upgrades without reducing exposure. The standalone product
host exports no provider callbacks. The qualification probe also exercises the
root-only certificate-reply endpoint and authenticated original-request recovery
against the local replica, including fresh `blob certificate recover` processes
using retained request files. CLI preparation and guarded dispatch now supply that
custody, including concurrent-dispatch refusal. With Node 24+, the installed check
also connects recovered proof to offline CLI transfer custody, actual loopback
PUTs and verified readback; a fresh process cannot repeat that transfer. See
[certificate commands](docs/operations/storage-inspection.md#prepare-and-dispatch-an-original-certificate-request).
The [reference application journey](docs/design/reference-application.md#transfer-and-verified-read-composition--development-01009)
uses a six-chunk file and checks catalog publication, corrupt-read refusal and
cancellation while transfer/read responses are pending. Its existing API and
persistence format remain unchanged.
It does not establish live gateway membership, provider
acceptance of recovered proof, deletion or billing closure.

`make deps-list` lists workspace package versions and release groups. `make update`
delegates to Toko Tooling's `make deps-update` (`cargo update -v`), updating locks
within the manifest requirements. To select one dependency, use
`make update DEPENDENCY_PACKAGE=toko-tooling`; `DEPENDENCY_PRECISE` optionally
selects a version allowed by its manifest requirement.
`make dependency-pins-check` admits Cargo declarations/inheritance in both
workspaces, tracked lockfiles and GitHub action pins. It is part of `make ci`;
it does not replace locked dependency preparation or behavior tests.

Toko Tooling remains exactly pinned. Adopting a newer release requires updating
both workspace pins and lockfiles, then running `TMPDIR=/tmp make toko-tooling-preview` followed by
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
