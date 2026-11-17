# ic-blob-storage

Experimental blob authorization, application references and provider-obligation accounting for the Internet Computer.

This is the independent rewrite of the legacy crate. API and persistence are
experimental; existing installations have no automatic migration. Local
qualification does not establish supported Caffeine enrollment, upload authority
or charge closure.

Use the library directly in a Rust host, or through Canic’s host adapter. Blob
owns grants, references and durable obligations; the host captures callers and
mounts endpoints. Declare `blob.control.v1` in the host’s Storage bootstrap and pass
the granted `RuntimeMemory` lease to `Service::create` or `Service::open`. This crate does not transfer file bytes or claim provider settlement.

See the [repository](https://github.com/dragginzgame/ic-blob-storage) for the
design, limits and qualification. Licensed under the Toko Studios Network Licence
1.0; see the packaged `LICENSE.md`.
