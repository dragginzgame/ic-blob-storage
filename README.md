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
Provider-object resolution, supported live account binding and a production
management runner remain unimplemented. API and persistence are experimental;
these local tests do not establish D0/D1 completion or provider closure.

The previous implementation, Git history and provider/recovery custody are in
`../ic-blob-storage-legacy` (see its `LEGACY.md`) and the private
[legacy GitHub repository](https://github.com/dragginzgame/ic-blob-storage-legacy).
Existing crates.io releases remain the legacy line. The rewrite's first GitHub
checkpoint is `0.100.0`; this checkpoint is `0.100.2`, inherited from the root
workspace version. Publication remains disabled. This is a new breaking release
line, not a registry version reset or a migration for existing installations.

The workspace has two product crates under `crates/`, Rust 1.100.0 / edition 2024
and published Toko Tooling 0.43.1. GitHub development uses the `0.100` branch of
`dragginzgame/ic-blob-storage`. There is no release/deployment configuration.

The [selected crate layout](docs/design/ic-blob-storage-design.md#selected-crate-layout)
keeps the host thin. Installed tests under `qualification/crates/` use the real
production Wasm and share the root workspace/lockfile. Domain tests remain with
the library. Canic owns its adapter and composed tests in its own repository.

```sh
make install-tools
make test-references
make test-caffeine
make test-local
make example-local
make test-example-local
make test-standalone
make deps-list
toko-cloc
```

Run these from the repository root. `make install-tools` explicitly downloads and
authenticates the selected tools and local PocketIC server; `make install-test-server`
can prepare that server alone. `make test-standalone` lints/builds the production
Wasm and exercises callers, cleanup and repeated upgrades in a fresh local instance.
It requires `wasm32-unknown-unknown` and makes no Caffeine calls. `make ci` is the
separate aggregate native gate; installed testing is an explicit command.

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
