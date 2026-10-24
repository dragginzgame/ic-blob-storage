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
wire/hash identities. The current service uses bounded IC Wire for all owned methods and standalone lifecycle; see [the protocol](../../docs/service-wire.md).
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

Use the bounded native IC Wire tenant contracts for their two categories and the
method-bound native Wire helpers for ordinary/manifest declarations. Both request and reply codecs
must agree with the method catalog.
The certificate and provider GC categories remain Candid. Mount the endpoint catalogue before exporting the provider Candid service. `endpoints::SERVICE_METHODS`
provides the complete method/mode/codec inventory from the same catalog; compare
that inventory with actual Wasm exports and compare only the provider subset
with generated Candid. Native compilation alone is insufficient.

All owned methods use published IC Wire; passive offline Wire values have their separate
`document` owner. The tenant methods'
bounded projections use Serde declaration order, method/direction-bound
frames and finite resource limits while native DTO/policy and persisted schemas
retain their existing owners. See [the adoption state](../../docs/service-wire.md).
The registry requirement targets Wire 0.6.10. Service/document generic decoding
requires explicit `Deserialize<'de>` and can return bounded `BytesRef`/`TextRef`
views. Retain the input for their lifetime; use owned values across async results,
persistence and retained retries. Borrowed tenant wire projections now take a
lifetime and convert from native references; their native helper functions retain
their signatures. `download::reply::decode_borrowed` shares the owned descriptor
validator over `DownloadResponse<Text = String>` without copying metadata text.
See [measured applicability and tradeoffs](../../docs/evidence/borrowed-wire0390.md).
 Literal vectors in
[`tests/wire_golden`](tests/wire_golden/README.md) freeze all owned method and
lifecycle contracts independently of round trips. Nested DTOs are exercised
through these boundary values and focused unit tests rather than a separate
exhaustive DTO registry. Application-only DTOs no
longer implement `CandidType`; the external provider certificate retains it.
Frozen historical test schemas and stable record codecs have their own owners.

Application DTO principals use Platform 0.4's owned `ic_platform::Principal`.
Provider Candid projections use `ic_platform::principal::ExternalPrincipal`;
convert with `.into()` when crossing that boundary. Binary Wire bytes remain
unchanged; the public Rust type identity requires aligned host dependencies.

## Host-only Wire schemas

Enable `wire-schema` on a native build to use `wire::schema::methods()` and the
`ic_wire::typescript::Wire` implementations on service DTOs. The catalog is
expanded from the same declarations as `mount_service_endpoints!`; it contains
all owned callable methods, their dispatch modes and stable IDs, actual request
and response schemas, and complete byte/allocation/node/depth limits. Tenant
entries use their bounded serialized projections rather than the native DTOs.
`endpoints::SERVICE_METHODS` remains the complete Wire/external-Candid inventory.
Lifecycle inputs are host-owned and are not callable methods in this catalog.

For generated clients, use `wire::schema::typescript("./blob-runtime.ts")`,
`wire::schema::runtime_adapter("./wire-runtime.ts")` and the unchanged published
`ic_wire::typescript::RUNTIME`. The adapter supplies Blob's custom decimal
validators. Account integers remain **strings** in canonical underscore-grouped
form, including arbitrary-width values; do not substitute JavaScript numbers or
omit the adapter. The raw metadata names these validators as `blobInt`/`blobNat`.
Wire's TypeScript conventions use bigint for wide integers, Uint8Array for
principals, `{some: value}`/null for options and `{ok: value}`/`{err: value}` for
results. Metadata does not authenticate calls or establish provider guarantees.

The `wire_schema` example emits `metadata`, `client`, `adapter` or `runtime` to
stdout. `make test-wire-schema` uses explicitly prepared browser tools to bundle
and test generated TypeScript against the frozen Rust frames, including rejection
cases. It installs nothing. The `Wire schema and generated clients` CI job runs this
check on pull requests and main pushes after explicit pinned dependency setup,
and retains generated files and qualification logs for 30 days. The regular
all-feature native test gate covers the
catalog, projections and custom scalars. The deployment dependency check refuses
schema features in the normal standalone graph; the metadata API/derives are
absent on Wasm. Explicit all-feature tooling builds are a separate selection.
Exported allocation limits describe Rust admission, **not** a JavaScript heap
bound. Wire's shared TypeScript runtime owns its own traversal accounting.

This additive export preserves endpoint identities, bytes and persisted formats.
Consumers own transport, final-Wasm inventory comparison and release sidecar
binding. In particular Canic owns embedded-host qualification and adoption after
this API is published; native catalog checks cannot replace those artifact checks.


## Compile-time mounted method policy

The existing `wire::{identity, request_limit, request_limits, response_limits}`
lookups are const functions. `endpoints::MethodSpec::wire()` returns a
`WireContract` containing the ID and complete request/reply `ic_wire::Limits`.
External Candid descriptors refuse with `Error::Identity`. This API works without
`wire-schema`, including on Wasm, and uses the same owner as runtime lookup and
native schema generation. Lifecycle lookup names `blob_install` and
`blob_post_upgrade` remain separate from `SERVICE_METHODS` and endpoint mounting.

Supply `describe = describe` immediately after `emit = endpoint` in
`mount_service_endpoints!` to receive each descriptor before endpoint emission.
The callback receives `(IcWire | Candid, Rust_function_identifier, MethodSpec_expression)`.
For example, a host can create constants for its own artifact format:

```rust
macro_rules! describe {
    (IcWire, $name:ident, $spec:expr) => {
        mod $name {
            pub const SPEC: ic_blob_storage_contracts::endpoints::MethodSpec = $spec;
            pub const WIRE: ic_blob_storage_contracts::endpoints::WireContract =
                match SPEC.wire() {
                    Ok(contract) => contract,
                    Err(_) => panic!("owned Wire method"),
                };
            pub const ID: u64 = WIRE.id;
        }
    };
    (Candid, $name:ident, $spec:expr) => {
        mod $name {
            pub const SPEC: ic_blob_storage_contracts::endpoints::MethodSpec = $spec;
        }
    };
}
```

The host's unchanged endpoint emitter can use `$name::ID`, `$name::WIRE.request`
and `$name::WIRE.response` for owned methods. Provider branches use their external
classification and never receive a fabricated Wire ID. Hosts own section encoding,
transform preservation and final-artifact/schema comparison; Blob supplies no
Canic-specific format or framework dependency. Omitting `describe` retains the
existing mounting invocation. Endpoint IDs, bytes, limits and lifecycle behavior
are unchanged.

## Native tooling versus deployed contracts

The default `provider-json` feature preserves the SDK prepared-manifest and
chunk-reply parsing APIs. Canister consumers that do not parse those HTTP/SDK
payloads should select `default-features = false`; native upload tooling selects
`provider-json` explicitly. The native `wire-schema` feature independently enables
its JSON dependency. Other dependencies can still introduce JSON transitively;
inspect the selected deployment graph rather than inferring Wasm size savings.

Native schema hosts can call `wire::schema::projections()` to obtain `MethodSchema`
values with catalog `spec`, `contract` budgets and request/response schemas. This
avoids parsing names and limits back out of JSON. `methods()` remains the canonical
serialized registry used by TypeScript generation and retained schema artifacts;
its bytes/layout and method identities are unchanged.
