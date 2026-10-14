# Permission views, signed intent facts and lifecycle accounting

Date: 2026-10-13. Base: `7ad76317fb1ed2695c16ede4aabee655ded84187`.
Task baseline is the existing dirty 0.23.3 batch retained in
`/tmp/blob-fifth-cleanup-before`; earlier changes are preserved. Shared 0.11.0 at
`a15ff323449f4b6190f647aec0de43cbcfb314ba` remains the adopted snapshot.
Incoming Testkit 0.38.0, Host 0.17.7, Memory 0.36.3 and Metrics 0.9.1 selections
already existed at admission. This task changes no dependency selection or version.

## Ownership and compatibility

Exposure inspection returns the exact view it already authenticated, instead of
reading the same permission again. Completion inspection and confirmed replay
project their validated view through one local lookup. Actor/scope checks happen
before projection. The mutable-owner fence, exact replay statement/verifier checks
and confirmed-record phase/schema/validity checks remain independent. Views never
cross an await, replace mutation-time rechecking or acquire dispatch authority.

The signed-update owner derives request ID, ingress expiry, signed-packet and
root-key digests, request/deadline limits and no-retry facts. A passive serialization
envelope flattens these alongside operation-owned evidence for reference,
attestation, gateway and upload-setup commands. Existing JSON fields and values
are retained; operation identities, roles, source fingerprints and argument hashes
remain local. Retained packets and the permanent directory claim precede intent
serialization and dispatch; failed serialization cannot authorize reuse.

Heap and stable lifecycle accounting use the same private phase-to-bytes function.
Public const getters retain their u64 results; stable aggregates widen individual
contributions to u128 before arithmetic. Logical release, physical deletion and
billing cessation remain separate, including zero-byte objects with obligations.
Persisted schemas, phase tags, wire contracts and public types remain unchanged.
The combined pending release remains compatible 0.23.3.

## Measurements and qualification

Evidence packet: `.tmp/review-validation/fifth-cleanup0233/`, including admitted
input hashes and the rebuilt Wasm identities. Linux uses Rust 1.99.0.

Two isolated source copies, `/tmp/blob-fifth-measure-before` and
`/tmp/blob-fifth-measure-after`, use the same admitted manifest/lock and identical
test-only memory instrumentation. They measure one prepared upload and one exact
confirmed replay through the public workflows, counting actual `Memory::read`
calls and asserting no writes and unchanged receipts. Exposure inspection changes
from 16 reads to 8; replay changes from 30 to 22. The temporary instrumentation
does not enter maintained source. These are native stable-memory call counts,
not deployed provider observations, Canic instruction costs or general throughput.

The 346 service-core and 113 native CLI tests pass. The retained-packet case now
checks the complete flat dispatch facts against the actual signed packet. Strict
all-target/all-feature Clippy passes for the two affected packages. Format,
documentation-link and runtime-free contract graph checks pass. Testkit 0.38's
selected CLI/server is prepared explicitly through the existing shared installer.

All 22 selected PocketIC cases pass against freshly built storage, consumer,
gateway-source and standalone Wasm plus the rebuilt CLI. They cover exposure
rollback and lost acknowledgments, confirmed replay and restored fences, each
lifecycle phase and byte obligation, and signed reference, attestation, upload
setup and gateway commands. The existing opt-in Chromium certificate case remains
ignored; browser qualification was not requested for these native changes.
Strict Rustdoc and final documentation checks pass. Input hashes remain unchanged.

Full Blob CI/release validation, native macOS and deployed provider/Canic acceptance
are outside this focused local qualification. Existing evidence remains bound to
its original source and graph.
