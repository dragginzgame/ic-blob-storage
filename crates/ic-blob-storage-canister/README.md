# ic-blob-storage-canister

Experimental standalone Internet Computer host for ic-blob-storage.

This is the independent rewrite of the legacy crate. API and persistence are
experimental; existing installations have no automatic migration. Local
qualification does not establish supported Caffeine enrollment, upload authority
or charge closure.

Build prepared Wasm with `make build-host` in the matching source checkout.
A plain Cargo Wasm output lacks the mandatory Storage native startup transform.
The bounded host exposes the Blob Wire endpoints; upgrades must close/reopen
native roots with Keep memory persistence. See the
[native storage contract](https://github.com/dragginzgame/ic-blob-storage/blob/v0.104.0/docs/design/native-storage.md).
Publication distributes source; it does not deploy or configure a canister.

See the [repository](https://github.com/dragginzgame/ic-blob-storage) for the
design, limits and qualification. Licensed under the Toko Studios Network Licence
1.0; see the packaged `LICENSE.md`.
