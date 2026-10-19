# Blob codec qualification, 2026-10-19

This local experiment measures actual UploadManifestRequest values with 1, 32 and
1,024 leaves, plus a separately labelled 1 MiB opaque-byte control. Blob bodies
still use HTTP. Fixture roots and wide identities exercise codec shapes, not
provider-valid upload declarations. Every native/Wasm frame hash agrees.

[Inputs](inputs.json) retain compiler, targets, profile, source/lock/fixture hashes,
limits and linked Wasm identities. Raw reports retain seven samples of four
iterations, measured after construction, roundtrip checks and charge searches.
Each sample includes output allocation and drop, and decode includes canonical
verification. Native time is diagnostic on a shared Linux host; no native speedup
is claimed. No provider request, deployment, size comparison or macOS run occurred.

| Workload | Frame bytes | Native encode / decode ns | IC baseline decode | IC candidate decode |
| --- | ---: | ---: | ---: | ---: |
| Manifest 1 | 255 | 571 / 961 | 44,372.5 | 44,574.5 |
| Manifest 32 | 1,249 | 4,544 / 6,901 | 359,459.5 | 359,685.5 |
| Manifest 1,024 | 32,997 | 133,033 / 200,129 | 10,405,691.5 | 10,405,917.5 |
| Opaque 1 MiB control | 1,048,590 | 17,499 / 34,102 | 4,195,809.5 | 2,885,112.5 |

Values are sample medians divided by four. The [candidate](candidate.patch)
changes only Wire's Wasm canonical word comparison from short-circuit equality
to exact XOR/OR accumulation. It reduces opaque decode instructions by 31.24%.
Manifest cost increases by 202–226 instructions (0.46% at most). Encoding,
canonical frames and minimum accepted allocation allowances remain identical.
This supports the narrow Wire change, not a Blob transfer or manifest speed claim.
The comparison uses isolated copies of Wire 0.4.5 from `9e336f4`; the consumer
continues selecting published 0.4.4 until a separately authorized adoption.

Allocation figures are minimum accepted **admission charges**, found by binary
search under the unchanged byte/node/depth limits. They are not allocator counts,
peak heap or IC memory usage. Native and Wasm charges differ with pointer width.
For the 1 MiB control they are 10,486,144 and 10,485,952 respectively. Native/Wasm
manifest decode charges are 151,064/149,080; 182,176/179,744; and
1,198,000/1,195,568. These conservative charges belong to Wire admission.

## Reproduction

Run the native driver with a fresh output path:

```sh
cargo run --locked --release -p blob-test-protocol --example wire-benchmark -- NEW.json
```

Build the private storage probe for release wasm32, select the pinned PocketIC
server, and run the ignored `measure_installed_wire_codecs` storage test with
BLOB_STORAGE_PROBE_WASM and a fresh BLOB_WIRE_REPORT. The private query measures
only the shared workload using performance_counter(0); transport is outside the
measured interval. Both writers refuse an existing report. For A/B experiments,
use separate source copies and verify Cargo actually selects each local patch;
retain the resolved graph, source patch and Wasm hash. Do not change a production
lockfile for a measurement.

Raw reports: [native baseline](native-baseline.json),
[native candidate](native-candidate.json), [IC baseline](ic-baseline.json),
[IC candidate](ic-candidate.json). Initial unused-patch attempts and all failed
validation attempts remain under `/tmp/blob-*`; they are not A/B evidence.

## DTO and retained-record evidence

The frozen contract registry has 505 literal vectors. Application DTOs no longer
implement CandidType; the external provider certificate response retains it.
Current storage-probe controls use bounded CBOR tuples. Production storage and
external provider schemas are unchanged. Local frozen test schemas preserve
historical image calls and consumer-probe records, without migration or fallback.

The pre-trait-removal consumer Wasm from PR #90 (`2967298`) is pinned by hash in
the two ignored `frozen_*` storage tests. Both pass against the new consumer:
published/cancelled assets and prepared manifests retain their exact views,
service stable bytes remain unchanged, and restored instances remain fenced.
Run them with BLOB_PRE_TRAIT_CONSUMER_WASM pointing to that retained image;
the image is an artifact, not a second supported production contract.

Canic `4caeec25` still selects Blob 0.35 and its wrapper invokes the retired
wire::encode/decode CBOR helpers. Its separate adoption remains
[Canic #603](https://github.com/dragginzgame/canic/issues/603); no Canic source or
suite was changed. This does not claim downstream deployment readiness.
