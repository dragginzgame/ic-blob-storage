//! Frozen consumer-probe persistence schema. These local projections preserve Candid
//! record bytes independently of application DTO trait implementations.
use ic_platform::candid::contract::CandidType;
use ic_platform::principal::ExternalPrincipal;
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
    operator: ExternalPrincipal,
    service: ExternalPrincipal,
    tenant: ExternalPrincipal,
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
    service: ExternalPrincipal,
    tenant: ExternalPrincipal,
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
    service: ExternalPrincipal,
    namespace: u128,
    tenant: ExternalPrincipal,
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
    uploader: ExternalPrincipal,
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
    Registered,
    Cancelled,
}

pub(crate) fn encode(value: &super::ConsumerRecord) -> Vec<u8> {
    // Named fields isolate this frozen persistence schema from application Wire order.
    let bytes = serde_json::to_vec(value).expect("record projection");
    assert!(bytes.len() <= 4 * 1024 * 1024, "record projection bound");
    let record: ConsumerRecord = serde_json::from_slice(&bytes).expect("record projection");
    ic_platform::candid::contract::encode_one(record).expect("frozen record encoding")
}
pub(crate) fn decode(
    bytes: &[u8],
    config: &ic_platform::candid::decoder::DecoderConfig,
) -> super::ConsumerRecord {
    let record: ConsumerRecord =
        ic_platform::candid::decoder::decode_one_with_config(bytes, config)
            .expect("frozen record decoding");
    let projection = serde_json::to_vec(&record).expect("record projection");
    assert!(
        projection.len() <= 4 * 1024 * 1024,
        "record projection bound"
    );
    serde_json::from_slice(&projection).expect("record projection")
}
