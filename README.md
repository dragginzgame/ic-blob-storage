# IC Blob Storage

Clean-start rewrite workspace. The first implemented component is
[bounded object references](crates/ic-blob-storage/src/references/mod.rs): overlapping
application uses, idempotent successful retain/release, permanent released IDs and
irreversible closing on final release. It adds no product dependencies.

This is an in-memory domain component with an experimental Rust API, not an
installed storage service. Authentication, provider-object resolution, persistence,
uploads and Caffeine integration are not implemented.

The previous implementation, Git history and provider/recovery custody are in
`../ic-blob-storage-legacy` (see its `LEGACY.md`) and the private
[legacy GitHub repository](https://github.com/dragginzgame/ic-blob-storage-legacy).
Existing crates.io releases remain the legacy line. The rewrite targets
`0.100.0`, inherited from the root workspace version. Publication remains disabled;
this is a new breaking release line, not a registry version reset or a migration
for existing installations.

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
