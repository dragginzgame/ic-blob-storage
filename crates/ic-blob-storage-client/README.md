# IC Blob Storage Client

Explicit replicated canister clients for Blob admission, manifests, upload status,
references, tenant administration, funding observations and download descriptors.

Import the role module, for example
`ic_blob_storage_client::admission::ReplicatedUploadAdmissionClient`.
Contracts owns exact method-bound Wire codecs and reply validation; Platform supplies IC
calls. This package does not depend on the service, Memory or Storage.

The actual executing actor must match the selected binding. Persist exact intent
before polling a mutation. Calls never retry automatically; a rejection or lost
reply can leave an uncertain effect. Recovery and publication remain caller-owned.
