# IC Blob Storage Client

Explicit replicated canister clients for Blob admission, manifests, upload status,
references, tenant administration, funding observations and download descriptors.

Import the role module, for example
`ic_blob_storage_client::admission::ReplicatedUploadAdmissionClient`.
Contracts owns exact method-bound Wire codecs and reply validation.
`ic-platform-types` supplies Principal; `ic-platform-runtime` supplies IC calls
and execution APIs directly, with the same identities exposed by Platform. This package does not depend on the service, Memory or Storage.
Client and native CLI share a release group; Contracts and Engine have independent
versions in the same workspace. Service release identity comes from the host,
not the client package version.

The actual executing actor must match the selected binding. Persist exact intent
before polling a mutation. Calls never retry automatically; a rejection or lost
reply can leave an uncertain effect. Recovery and publication remain caller-owned.

## Optional native verified downloads

Enable `ic-blob-storage-client = { version = "0.68.0", features = ["native-download"] }`
for a native publisher. Default clients and Wasm builds gain no HTTP/filesystem
dependencies. Linux and macOS support private create-only file publication. Run the helper
inside a caller-owned Tokio runtime with I/O and time enabled; it starts no runtime.

`native::Verification::new(root, bytes, original_headers, expected_sha256, limits)`
binds trusted metadata before any I/O. Use `Some(ContentDigest)` for an application's
required raw SHA-256 (for Cargo, its archive `cksum`); use `None` when only the
trusted provider root is available. The Blob root and raw checksum are different
identities, computed together in one streaming pass.

```rust,no_run
# #[cfg(all(feature = "native-download", unix, not(target_family = "wasm")))]
# async fn example() -> Result<(), Box<dyn std::error::Error>> {
use ic_blob_storage_client::native::{Verification, download};
use ic_blob_storage_contracts::identity::{ContentDigest, ProviderRootHash};
use ic_blob_storage_contracts::identity::caffeine::{CaffeineHashLimits, CaffeineHeader};
use std::time::Duration;

// Replace these with authenticated application state and an approved provider URL.
let root: ProviderRootHash = "sha256:0000000000000000000000000000000000000000000000000000000000000000".parse()?;
let checksum: ContentDigest = "sha256:0000000000000000000000000000000000000000000000000000000000000000".parse()?;
let headers = [CaffeineHeader { name: "Content-Length", value: "3" }];
let verification = Verification::new(root, 3, &headers, Some(checksum), CaffeineHashLimits {
    max_content_bytes: 1024.try_into()?,
    max_append_bytes: 65536.try_into()?,
    max_headers: 16.try_into()?,
    max_header_bytes: 4096.try_into()?,
})?;
let observed = download(
    &"https://approved-provider.example/v1/blob/?original-binding".parse()?,
    std::path::Path::new("existing-private-parent/fresh-run"),
    verification,
    Duration::from_secs(30),
    false, // Allow loopback HTTP only for an explicitly selected local test.
).await?;
// observed.path contains the fully verified body; observed.hashes contains both digests.
// The application still decides whether to perform its authenticated catalog commit.
# let _ = observed;
# Ok(())
# }
```

Use the provider's canonical `contracts::provider::download::request_target` with
an explicitly selected `CaffeineDownloadScope` to construct real Caffeine targets.
Do not trust a URL, root or headers obtained solely from the downloaded response.
This helper needs no tenant signer: a publisher can verify bytes against its retained
original manifest while its registry canister remains the Blob tenant. The registry
must independently check its own exact permission and current live reference.

Downloads have no redirects, automatic retries or decompression. URL user-info
refuses and no application authentication headers are configured.
Success requires HTTP body EOF, exact length, the original metadata-bound root,
and the supplied checksum. Failures/cancellation retain a private `body.part`;
only success publishes `body.bin`. The fresh run directory cannot be reused.
Keep the returned directory protected against subsequent modification; the result
is a local observation, not a signed attestation, future-retention guarantee or
permission to publish. Preserve application intent and reconcile failures instead
of blindly creating another run. Filesystem sync errors can occur after publication.

Call `Verification::read_response` when an existing native workflow already owns
request policy, a private sink, deadlines and response evidence (as Blob's CLI does).
That lower-level method does not select network policy or publish its sink.
