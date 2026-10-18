# IC Blob Storage Contracts

Runtime-free contracts shared by IC Blob Storage, its native CLI and offline
preparation/verification examples. This crate owns passive service DTOs and their per-method codecs, protocol
method names, checked identities, request/reply correlation, bounded decoders,
content verification and immutable installation inputs.

It has no CDK, Memory, endpoint, durable journal, lifecycle, accounting or provider
dispatch dependency. A checked value does not authenticate its caller, prove
provider behavior or authorize an effect. Hosts and the service retain those owners.

Use `ic_blob_storage_contracts::{dto, identity, protocol}` for wire/client data.
Use `ic_blob_storage` for service workflows, policy and durable owners. Both library
packages inherit one workspace version. The 0.18.0 extraction removes the
former service Rust paths without compatibility reexports; that extraction preserved
wire/hash identities. The current service uses bounded IC Wire for tenant methods
and CBOR for its other owned methods; see [the protocol](../../docs/service-wire.md).
Provider Candid and hashes remain. Cross-release transitions remain reinstall-only.

```sh
cargo run -p ic-blob-storage-contracts --example prepare_upload -- --help
cargo run -p ic-blob-storage-contracts --example verify_download -- --help
```

See the repository's [migration and qualification record](../../docs/evidence/contracts-0180.md).

## Opt-in host endpoint declarations

The `mount_service_endpoints!` API, available since 0.22.11, shares service declarations
with a host-owned callback. Invoke it inside an endpoint module and supply its
visible contract-crate alias, workflow adapter, actual caller/service context,
trusted clock/caller/trap paths and bounded query/update attributes. The callback
receives the method-specific decoding category and the provider certificate's
exact wire name. Keep lifecycle, installation, memory allocation and post-await
continuity in the host. See the standalone API module for a complete expansion.
Linking this runtime-free crate alone exports no endpoint or lifecycle hook.

Use the exact IC Wire tenant contracts for their two categories and bounded CBOR
for unconverted ordinary/manifest declarations. Both request and reply codecs
must agree with the method catalog.
The certificate category remains Candid. Keep provider Candid registration and
schema export in the same emitter expansion pass. `endpoints::SERVICE_METHODS`
provides the complete method/mode/codec inventory from the same catalog; compare
that inventory with actual Wasm exports and compare only the provider subset
with generated Candid. Native compilation alone is insufficient.

Tenant inspection/update use published IC Wire. Their
bounded projections use explicit field/variant identities, method/direction-bound
frames and finite resource limits while native DTO/policy and persisted schemas
retain their existing owners. See [the adoption state](../../docs/service-wire.md).
The registry requirement targets 0.4. Candidate-source evidence retains its
original scope; it does not qualify later dependencies or other methods.
