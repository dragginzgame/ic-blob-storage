# IC Blob Storage Contracts

Runtime-free contracts shared by IC Blob Storage, its native CLI and offline
preparation/verification examples. This crate owns passive Candid DTOs, protocol
method names, checked identities, request/reply correlation, bounded decoders,
content verification and immutable installation inputs.

It has no CDK, Memory, endpoint, durable journal, lifecycle, accounting or provider
dispatch dependency. A checked value does not authenticate its caller, prove
provider behavior or authorize an effect. Hosts and the service retain those owners.

Use `ic_blob_storage_contracts::{dto, identity, protocol}` for wire/client data.
Use `ic_blob_storage` for service workflows, policy and durable owners. Both library
packages inherit one workspace version. The 0.18.0 extraction removes the
former service Rust paths without compatibility reexports; Candid and hash identities
remain unchanged. Cross-release service transitions remain reinstall-only.

```sh
cargo run -p ic-blob-storage-contracts --example prepare_upload -- --help
cargo run -p ic-blob-storage-contracts --example verify_download -- --help
```

See the repository's [migration and qualification record](../../docs/evidence/contracts-0180.md).

## Opt-in host endpoint declarations

The pending `mount_service_endpoints!` API shares service method declarations
with a host-owned callback. Invoke it inside an endpoint module and supply its
visible contract-crate alias, workflow adapter, actual caller/service context,
trusted clock/caller/trap paths and bounded query/update attributes. The callback
receives the request/manifest decoding category and the provider certificate's
exact wire name. Keep lifecycle, installation, memory allocation and post-await
continuity in the host. See the standalone API module for a complete expansion.
Linking this runtime-free crate alone exports no endpoint or lifecycle hook.
