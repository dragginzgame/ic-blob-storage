//! Frozen consumer-probe persistence schema. These local projections preserve Candid
//! record bytes independently of application DTO trait implementations.
use ic_platform::{Principal, candid::contract::CandidType};
use serde::{Deserialize, Serialize};
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
#[expect(
    clippy::struct_excessive_bools,
    reason = "Frozen saved record schema preserves independent obligations"
)]
struct AssetRecord {
    intent: Registration,
    cancelled: bool,
    published: bool,
    published_once: bool,
    admission_started: bool,
    admission_result: Option<Result<(), UploadAdmissionFailure>>,
    revocation_started: bool,
    revocation_result: Option<Result<(), UploadAdmissionFailure>>,
    retain_started: bool,
    retain_result: Option<Result<ReferenceChange, ReferenceTransitionFailure>>,
    upload_state: Option<UploadState>,
    release_started: bool,
    release_result: Option<Result<ReferenceChange, ReferenceTransitionFailure>>,
    uses: u32,
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
struct ConsumerRecord {
    operator: Principal,
    service: Principal,
    tenant: Principal,
    fenced: bool,
    assets: Vec<AssetRecord>,
    manifests: Vec<ManifestIntentRecord>,
    tenant_command: Option<TenantCommandRecord>,
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
struct ManifestIntent {
    id: u128,
    request: UploadManifestRequest,
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
struct ManifestIntentRecord {
    intent: ManifestIntent,
    cancelled: bool,
    started: bool,
    result: Option<Result<UploadManifestDeclaration, UploadManifestFailure>>,
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
enum ReferenceAction {
    Retain,
    Release,
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
enum ReferenceChange {
    Changed,
    Unchanged,
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
struct ReferenceCommand {
    upload: ReferenceUpload,
    reference: u128,
    operation: u128,
    action: ReferenceAction,
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
enum ReferenceTransitionFailure {
    UnknownReference,
    Released,
    Limit,
    DeletionQueued,
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
struct ReferenceUpload {
    service: Principal,
    tenant: Principal,
    namespace: u128,
    upload: u128,
    object: u128,
    incarnation: u128,
    first_reference: u128,
    root: [u8; 32],
    bytes: u64,
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
struct Registration {
    asset: u128,
    payload: Vec<u8>,
    source: RegistrationSource,
    release_operation: u128,
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
enum RegistrationSource {
    Existing(ReferenceCommand),
    Fresh(UploadAdmissionRequest),
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
struct TenantCommandRecord {
    service: Principal,
    namespace: u128,
    tenant: Principal,
    expected: Option<(u64, bool)>,
    active: bool,
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
enum UploadAdmissionFailure {
    Invalid,
    Denied,
    Binding,
    Unknown,
    Conflict,
    Inactive,
    Expired,
    Capacity,
    Fenced,
    Internal,
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
struct UploadAdmissionRequest {
    upload: ReferenceUpload,
    uploader: Principal,
    expires_at_ns: u64,
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
struct UploadManifestDeclaration {
    chunks: Vec<[u8; 32]>,
    headers: Vec<UploadManifestHeader>,
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
enum UploadManifestFailure {
    Permission(UploadAdmissionFailure),
    Revoked,
    Phase,
    Declaration,
    Limit,
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
struct UploadManifestHeader {
    name: String,
    value: String,
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
struct UploadManifestRequest {
    permission: UploadAdmissionRequest,
    declaration: UploadManifestDeclaration,
}
#[derive(CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
enum UploadState {
    Reserved,
    ExposurePossible,
    Confirmed,
    Cancelled,
}

pub(crate) fn encode(value: &super::ConsumerRecord) -> Vec<u8> {
    let bytes = ic_blob_storage_contracts::document::encode(value).expect("record projection");
    let record: ConsumerRecord =
        ic_blob_storage_contracts::document::decode(&bytes, 4 * 1024 * 1024)
            .expect("record projection");
    ic_platform::candid::contract::encode_one(record).expect("frozen record encoding")
}
pub(crate) fn decode(
    bytes: &[u8],
    config: &ic_platform::candid::decoder::DecoderConfig,
) -> super::ConsumerRecord {
    let record: ConsumerRecord =
        ic_platform::candid::decoder::decode_one_with_config(bytes, config)
            .expect("frozen record decoding");
    let projection =
        ic_blob_storage_contracts::document::encode(record).expect("record projection");
    ic_blob_storage_contracts::document::decode(&projection, 4 * 1024 * 1024)
        .expect("record projection")
}
