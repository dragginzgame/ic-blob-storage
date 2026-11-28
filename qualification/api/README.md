# Callable Wire methods

Run `make check-callable-api` to rebuild the standalone and reference-app release
Wasms and compare them with [the reviewed baseline](callable/). The two text
files record callable names and modes from the Wasms. Each callable must also
match exactly one mounted Wire contract by name and mode. The per-method JSON
files record Wire's routing ID, request and reply roots, reachable schemas and
payload budgets. A shape change can therefore fail CI even when the Wasm export
inventory stays identical.

The text inventory dates from v0.107.1. The first schema baseline uses the
v0.108.0 product declarations with Wire 0.27.1. For an intended API change,
inspect the candidate directory printed by the shared comparison runner, review
the affected manifests against the mount and copy the accepted files into the
baseline. The installed endpoint suite also checks request and reply bytes
captured with the 0.107.1 codecs.

## How qualification obtains the schemas

The [qualification build script](../ic-blob-storage-tests/build/contracts/mod.rs)
reads the Platform mounts, their Wire codec IDs and budgets, and the Serde
declarations used by their handlers. It preserves field order, variant order
and Serde attributes for Wire's derive and marks projected structs and tuples
as fixed. The [exporter](../ic-blob-storage-tests/examples/callable-api.rs)
uses Wire's validated registry and format-1 manifest writer. Wire budgets in
these files exclude the frame header. The native allocation budget is outside
Wire's manifest format.

The Wire 0.31.1 baseline adds the `fixed: true` annotations to the existing
ordinary Serde layouts. Removing those annotations reproduces the previous
eight manifests byte for byte.

Wire 0.32.0 reproduces all eight reviewed manifests byte for byte. It supplies the ordinary scalar, container and Platform principal
projections. Four integer types used by these mounts still lack a native
`Wire` implementation. The [supplier projections](../../crates/ic-blob-storage/src/projection.rs)
cover their existing binary Serde representations.

| Rust type | Encoded unsigned width | Range accepted on the release Wasm |
|---|---:|---|
| `NonZeroU128` | 128 | 1 through `u128::MAX` |
| `NonZeroU64` | 64 | 1 through `u64::MAX` |
| `usize` | 64 | 0 through `u32::MAX` |
| `NonZeroUsize` | 64 | 1 through `u32::MAX` |

Native consumers can enable the supplier’s `inspection` feature to register the actual protocol types. The ordinary product Wasm build omits this feature.
The two product Wasms have eight callable Wire methods. IC lifecycle hooks have
no callable names. The Caffeine callback and enrollment probes are private
qualification canisters.
