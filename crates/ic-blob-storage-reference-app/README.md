# ic-blob-storage-reference-app

Experimental reference application and Wire protocol for ic-blob-storage.

This is the independent rewrite of the legacy crate. API and persistence are
experimental; existing installations have no automatic migration. Local
qualification does not establish supported Caffeine enrollment, upload authority
or charge closure.

The example catalog is a separate canister paired with a standalone Blob host.
Its public protocol types are also used by the CLI. Canic applications own their
embedded catalog and initialization contract. Build prepared Wasm with
`make build-reference-app` in the matching source checkout; a plain Cargo Wasm
output is insufficient. The
[native storage contract](https://github.com/dragginzgame/ic-blob-storage/blob/v0.104.0/docs/design/native-storage.md)
owns required preparation and Keep upgrade hooks.

See the [repository](https://github.com/dragginzgame/ic-blob-storage) for the
design, limits and qualification. Licensed under the Toko Studios Network Licence
1.0; see the packaged `LICENSE.md`.
