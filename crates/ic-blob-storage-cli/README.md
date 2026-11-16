# ic-blob-storage-cli

Blob storage inspection, offline preparation and guarded maintenance CLI for the Internet Computer.

This is the independent rewrite of the legacy crate. API and persistence are
experimental; existing installations have no automatic migration. Local
qualification does not establish supported Caffeine enrollment, upload authority
or charge closure.

Install Rust 1.100.0 or newer, then:

```sh
cargo install ic-blob-storage-cli --locked
blob --help
blob app --help
```

Cargo installs `blob` into its bin directory (normally `~/.cargo/bin`); keep that
directory on PATH. Maintenance additionally requires the matching source checkout
and admitted tools. Inspection and offline preparation do not require a checkout.

For a custom network, pair `--root-key` with the independently trusted
`--root-subnet` principal. Mainnet inspection uses the built-in public trust data.

See the [repository](https://github.com/dragginzgame/ic-blob-storage) for the
design, limits and qualification. Licensed under the Toko Studios Network Licence
1.0; see the packaged `LICENSE.md`.
