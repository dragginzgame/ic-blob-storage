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

These are in-memory components with experimental Rust APIs, not an installed
storage service. Ingress authentication, qualified provider-object resolution,
persistence, uploads and Caffeine network integration are not implemented. Verification proves supplied
byte integrity; the caller must also require successful transport EOF and keep
output private until verification succeeds. It proves no provider availability.

The previous implementation, Git history and provider/recovery custody are in
`../ic-blob-storage-legacy` (see its `LEGACY.md`) and the private
[legacy GitHub repository](https://github.com/dragginzgame/ic-blob-storage-legacy).
Existing crates.io releases remain the legacy line. The rewrite's first GitHub
checkpoint is `0.100.0`; current development is `0.100.1`, inherited from the root
workspace version. Publication remains disabled. This is a new breaking release
line, not a registry version reset or a migration for existing installations.

The workspace has one product crate under `crates/`, Rust 1.100.0 / edition 2024
and published Toko Tooling 0.43.0. GitHub development uses the `0.100` branch of
`dragginzgame/ic-blob-storage`. There is no release/deployment configuration.

The [selected crate layout](docs/design/ic-blob-storage-design.md#selected-crate-layout)
adds a thin `ic-blob-storage-canister` host when endpoints exist. Installed tests
will live under `qualification/crates/` and share the root workspace/lockfile.
Current native reference tests stay with the library; there is no installed host
requiring a qualification package yet. Canic owns its adapter and composed tests
in its own repository.

```sh
make install-tools
make test-references
make test-caffeine
make test-local
make example-local
make test-example-local
make ci
make deps-list
toko-cloc
```

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
