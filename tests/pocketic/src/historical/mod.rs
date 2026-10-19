//! Frozen projections used only by tests of retired Candid images and external replies.
//! Captured from the pre-trait-removal schema; never used by current owned endpoints.
use ic_platform::{
    Principal,
    candid::contract::{CandidType, Int, Nat},
};
use serde::{Deserialize, Serialize};
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub enum AccountInspectionKind {
    /// Frozen variant.
    Balance,
    /// Frozen variant.
    PaymentRelationship,
}
impl Projection for ic_blob_storage_contracts::dto::account::AccountInspectionKind {
    type Stored = AccountInspectionKind;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct AccountInspectionRequest {
    /// Frozen field.
    pub scope: OperatorScope,
    /// Frozen field.
    pub kind: AccountInspectionKind,
}
impl Projection for ic_blob_storage_contracts::dto::account::AccountInspectionRequest {
    type Stored = AccountInspectionRequest;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct AccountRelationshipView {
    /// Frozen field.
    pub paid_canister: Principal,
    /// Frozen field.
    pub payment_account: Principal,
    /// Frozen field.
    #[serde(with = "decimal")]
    pub spending_limit_per_day: Int,
    /// Frozen field.
    #[serde(with = "decimal")]
    pub current_period_spent: Int,
    /// Frozen field.
    pub current_period_start: u64,
    /// Frozen field.
    pub added_timestamp: u64,
    /// Frozen field.
    pub expiration_timestamp: Option<u64>,
    /// Frozen field.
    #[serde(with = "decimal")]
    pub bandwidth_baseline_uploaded: Nat,
    /// Frozen field.
    #[serde(with = "decimal")]
    pub bandwidth_baseline_downloaded: Nat,
    /// Frozen field.
    pub bandwidth_baseline_ts_ns: u64,
}
impl Projection for ic_blob_storage_contracts::dto::account::AccountRelationshipView {
    type Stored = AccountRelationshipView;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub enum HostFailure {
    /// Frozen variant.
    Denied,
}
impl Projection for ic_blob_storage_contracts::dto::configuration::HostFailure {
    type Stored = HostFailure;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub enum LocalStatusFailure {
    /// Frozen variant.
    Binding,
    /// Frozen variant.
    Denied,
    /// Frozen variant.
    Internal,
}
impl Projection for ic_blob_storage_contracts::dto::operator::LocalStatusFailure {
    type Stored = LocalStatusFailure;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct LocalUploadStatus {
    /// Frozen field.
    pub operations: u64,
    /// Frozen field.
    pub active_reservations: u64,
    /// Frozen field.
    pub reserved_bytes: u128,
    /// Frozen field.
    pub logical_bytes: u128,
    /// Frozen field.
    pub physical_bytes: u128,
    /// Frozen field.
    pub liability_bytes: u128,
    /// Frozen field.
    pub fenced: bool,
}
impl Projection for ic_blob_storage_contracts::dto::operator::LocalUploadStatus {
    type Stored = LocalUploadStatus;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct OperatorScope {
    /// Frozen field.
    pub service: Principal,
    /// Frozen field.
    pub namespace: u128,
    /// Frozen field.
    pub cashier: Principal,
    /// Frozen field.
    pub payment_account: Principal,
}
impl Projection for ic_blob_storage_contracts::dto::operator::OperatorScope {
    type Stored = OperatorScope;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub enum ReferenceAction {
    /// Frozen variant.
    Retain,
    /// Frozen variant.
    Release,
}
impl Projection for ic_blob_storage_contracts::dto::reference::ReferenceAction {
    type Stored = ReferenceAction;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub enum ReferenceChange {
    /// Frozen variant.
    Changed,
    /// Frozen variant.
    Unchanged,
}
impl Projection for ic_blob_storage_contracts::dto::reference::ReferenceChange {
    type Stored = ReferenceChange;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct ReferenceCommand {
    /// Frozen field.
    pub upload: ReferenceUpload,
    /// Frozen field.
    pub reference: u128,
    /// Frozen field.
    pub operation: u128,
    /// Frozen field.
    pub action: ReferenceAction,
}
impl Projection for ic_blob_storage_contracts::dto::reference::ReferenceCommand {
    type Stored = ReferenceCommand;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub enum ReferenceFailure {
    /// Frozen variant.
    Invalid,
    /// Frozen variant.
    Denied,
    /// Frozen variant.
    Binding,
    /// Frozen variant.
    Unknown,
    /// Frozen variant.
    Unconfirmed,
    /// Frozen variant.
    Conflict,
    /// Frozen variant.
    Inactive,
    /// Frozen variant.
    Fenced,
    /// Frozen variant.
    Capacity,
    /// Frozen variant.
    Internal,
}
impl Projection for ic_blob_storage_contracts::dto::reference::ReferenceFailure {
    type Stored = ReferenceFailure;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct ReferenceMutationResponse {
    /// Frozen field.
    pub receipt: ReferenceReceiptResponse,
    /// Frozen field.
    pub replayed: bool,
}
impl Projection for ic_blob_storage_contracts::dto::reference::ReferenceMutationResponse {
    type Stored = ReferenceMutationResponse;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct ReferenceReceiptResponse {
    /// Frozen field.
    pub request: ReferenceCommand,
    /// Frozen field.
    pub result: Result<ReferenceChange, ReferenceTransitionFailure>,
}
impl Projection for ic_blob_storage_contracts::dto::reference::ReferenceReceiptResponse {
    type Stored = ReferenceReceiptResponse;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub enum ReferenceTransitionFailure {
    /// Frozen variant.
    UnknownReference,
    /// Frozen variant.
    Released,
    /// Frozen variant.
    Limit,
    /// Frozen variant.
    DeletionQueued,
}
impl Projection for ic_blob_storage_contracts::dto::reference::ReferenceTransitionFailure {
    type Stored = ReferenceTransitionFailure;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct ReferenceUpload {
    /// Frozen field.
    pub service: Principal,
    /// Frozen field.
    pub tenant: Principal,
    /// Frozen field.
    pub namespace: u128,
    /// Frozen field.
    pub upload: u128,
    /// Frozen field.
    pub object: u128,
    /// Frozen field.
    pub incarnation: u128,
    /// Frozen field.
    pub first_reference: u128,
    /// Frozen field.
    pub root: [u8; 32],
    /// Frozen field.
    pub bytes: u64,
}
impl Projection for ic_blob_storage_contracts::dto::reference::ReferenceUpload {
    type Stored = ReferenceUpload;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct Registration {
    /// Frozen field.
    pub asset: u128,
    /// Frozen field.
    pub payload: Vec<u8>,
    /// Frozen field.
    pub source: RegistrationSource,
    /// Frozen field.
    pub release_operation: u128,
}
impl Projection for blob_test_protocol::consumer::Registration {
    type Stored = Registration;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub enum RegistrationSource {
    /// Frozen variant.
    Existing(ReferenceCommand),
    /// Frozen variant.
    Fresh(UploadAdmissionRequest),
}
impl Projection for blob_test_protocol::consumer::RegistrationSource {
    type Stored = RegistrationSource;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct ServiceBillingInput {
    /// Frozen field.
    pub cashier: Principal,
    /// Frozen field.
    pub reserve: u128,
    /// Frozen field.
    pub minimum_balance: u128,
    /// Frozen field.
    pub target_balance: u128,
    /// Frozen field.
    pub max_gateway_entries: u32,
    /// Frozen field.
    pub max_gateway_unique: u32,
}
impl Projection for ic_blob_storage_contracts::dto::configuration::ServiceBillingInput {
    type Stored = ServiceBillingInput;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct ServiceConfigurationInput {
    /// Frozen field.
    pub service: Principal,
    /// Frozen field.
    pub operator: Principal,
    /// Frozen field.
    pub payment_account: Principal,
    /// Frozen field.
    pub namespace: u128,
    /// Frozen field.
    pub resources: ServiceResourceInput,
    /// Frozen field.
    pub billing: ServiceBillingInput,
    /// Frozen field.
    pub funding: ServiceFundingInput,
    /// Frozen field.
    pub reads: ServiceReadInput,
}
impl Projection for ic_blob_storage_contracts::dto::configuration::ServiceConfigurationInput {
    type Stored = ServiceConfigurationInput;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct ServiceFundingInput {
    /// Frozen field.
    pub allocated: u128,
    /// Frozen field.
    pub renewal_ceiling: u128,
    /// Frozen field.
    pub reserve: u128,
    /// Frozen field.
    pub max_attempts: u32,
}
impl Projection for ic_blob_storage_contracts::dto::configuration::ServiceFundingInput {
    type Stored = ServiceFundingInput;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct ServiceInstallationInput {
    /// Frozen field.
    pub configuration: ServiceConfigurationInput,
    /// Frozen field.
    pub project: String,
    /// Frozen field.
    pub completion_verifier: Principal,
}
impl Projection for ic_blob_storage_contracts::dto::configuration::ServiceInstallationInput {
    type Stored = ServiceInstallationInput;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct ServiceReadInput {
    /// Frozen field.
    pub sessions: u32,
    /// Frozen field.
    pub tenant_sessions: u32,
    /// Frozen field.
    pub reply_bytes: u32,
    /// Frozen field.
    pub bytes: u64,
    /// Frozen field.
    pub tenant_bytes: u64,
}
impl Projection for ic_blob_storage_contracts::dto::configuration::ServiceReadInput {
    type Stored = ServiceReadInput;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct ServiceResourceInput {
    /// Frozen field.
    pub max_tenants: u32,
    /// Frozen field.
    pub max_object_bytes: u64,
    /// Frozen field.
    pub max_headers: u32,
    /// Frozen field.
    pub max_header_bytes: u32,
    /// Frozen field.
    pub max_chunks: u32,
    /// Frozen field.
    pub max_tenant_chunks: u32,
    /// Frozen field.
    pub max_objects: u32,
    /// Frozen field.
    pub max_tenant_objects: u32,
    /// Frozen field.
    pub max_physical_bytes: u128,
    /// Frozen field.
    pub max_liability_bytes: u128,
    /// Frozen field.
    pub max_tenant_logical_bytes: u128,
    /// Frozen field.
    pub max_references_per_object: u32,
    /// Frozen field.
    pub max_receipts_per_object: u32,
    /// Frozen field.
    pub max_active: u32,
    /// Frozen field.
    pub max_tenant_active: u32,
}
impl Projection for ic_blob_storage_contracts::dto::configuration::ServiceResourceInput {
    type Stored = ServiceResourceInput;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct TenantEnrollment {
    /// Frozen field.
    pub generation: u64,
    /// Frozen field.
    pub active: bool,
}
impl Projection for ic_blob_storage_contracts::dto::tenant::TenantEnrollment {
    type Stored = TenantEnrollment;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct TenantEnrollmentResponse {
    /// Frozen field.
    pub scope: TenantScope,
    /// Frozen field.
    pub enrollment: Option<TenantEnrollment>,
    /// Frozen field.
    pub fenced: bool,
}
impl Projection for ic_blob_storage_contracts::dto::tenant::TenantEnrollmentResponse {
    type Stored = TenantEnrollmentResponse;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub enum TenantFailure {
    /// Frozen variant.
    Binding,
    /// Frozen variant.
    Denied,
    /// Frozen variant.
    Invalid,
    /// Frozen variant.
    Conflict,
    /// Frozen variant.
    Capacity,
    /// Frozen variant.
    GenerationExhausted,
    /// Frozen variant.
    Fenced,
    /// Frozen variant.
    Internal,
}
impl Projection for ic_blob_storage_contracts::dto::tenant::TenantFailure {
    type Stored = TenantFailure;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct TenantScope {
    /// Frozen field.
    pub service: Principal,
    /// Frozen field.
    pub namespace: u128,
    /// Frozen field.
    pub tenant: Principal,
}
impl Projection for ic_blob_storage_contracts::dto::tenant::TenantScope {
    type Stored = TenantScope;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct TenantUpdateRequest {
    /// Frozen field.
    pub scope: TenantScope,
    /// Frozen field.
    pub expected: Option<TenantEnrollment>,
    /// Frozen field.
    pub active: bool,
}
impl Projection for ic_blob_storage_contracts::dto::tenant::TenantUpdateRequest {
    type Stored = TenantUpdateRequest;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub enum UploadAdmissionFailure {
    /// Frozen variant.
    Invalid,
    /// Frozen variant.
    Denied,
    /// Frozen variant.
    Binding,
    /// Frozen variant.
    Unknown,
    /// Frozen variant.
    Conflict,
    /// Frozen variant.
    Inactive,
    /// Frozen variant.
    Expired,
    /// Frozen variant.
    Capacity,
    /// Frozen variant.
    Fenced,
    /// Frozen variant.
    Internal,
}
impl Projection for ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure {
    type Stored = UploadAdmissionFailure;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct UploadAdmissionMutation {
    /// Frozen field.
    pub admission: UploadAdmissionResponse,
    /// Frozen field.
    pub replayed: bool,
}
impl Projection for ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionMutation {
    type Stored = UploadAdmissionMutation;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct UploadAdmissionRequest {
    /// Frozen field.
    pub upload: ReferenceUpload,
    /// Frozen field.
    pub uploader: Principal,
    /// Frozen field.
    pub expires_at_ns: u64,
}
impl Projection for ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest {
    type Stored = UploadAdmissionRequest;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct UploadAdmissionResponse {
    /// Frozen field.
    pub permission: UploadAdmissionRequest,
    /// Frozen field.
    pub state: UploadState,
    /// Frozen field.
    pub revoked: bool,
}
impl Projection for ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionResponse {
    type Stored = UploadAdmissionResponse;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub enum UploadAttestationFailure {
    /// Frozen variant.
    Permission(UploadAdmissionFailure),
    /// Frozen variant.
    Denied,
    /// Frozen variant.
    Phase,
    /// Frozen variant.
    Conflict,
    /// Frozen variant.
    Observation,
}
impl Projection for ic_blob_storage_contracts::dto::upload::completion::UploadAttestationFailure {
    type Stored = UploadAttestationFailure;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct UploadAttestationMutation {
    /// Frozen field.
    pub receipt: UploadAttestationReceipt,
    /// Frozen field.
    pub changed: bool,
}
impl Projection for ic_blob_storage_contracts::dto::upload::completion::UploadAttestationMutation {
    type Stored = UploadAttestationMutation;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct UploadAttestationReceipt {
    /// Frozen field.
    pub request: UploadAttestationRequest,
    /// Frozen field.
    pub verifier: Principal,
    /// Frozen field.
    pub accepted_at_ns: u64,
}
impl Projection for ic_blob_storage_contracts::dto::upload::completion::UploadAttestationReceipt {
    type Stored = UploadAttestationReceipt;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct UploadAttestationRequest {
    /// Frozen field.
    pub permission: UploadAdmissionRequest,
    /// Frozen field.
    pub content_digest: [u8; 32],
    /// Frozen field.
    pub observed_at_ns: u64,
}
impl Projection for ic_blob_storage_contracts::dto::upload::completion::UploadAttestationRequest {
    type Stored = UploadAttestationRequest;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct UploadManifestDeclaration {
    /// Frozen field.
    pub chunks: Vec<[u8; 32]>,
    /// Frozen field.
    pub headers: Vec<UploadManifestHeader>,
}
impl Projection for ic_blob_storage_contracts::dto::upload::manifest::UploadManifestDeclaration {
    type Stored = UploadManifestDeclaration;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub enum UploadManifestFailure {
    /// Frozen variant.
    Permission(UploadAdmissionFailure),
    /// Frozen variant.
    Revoked,
    /// Frozen variant.
    Phase,
    /// Frozen variant.
    Declaration,
    /// Frozen variant.
    Limit,
}
impl Projection for ic_blob_storage_contracts::dto::upload::manifest::UploadManifestFailure {
    type Stored = UploadManifestFailure;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct UploadManifestHeader {
    /// Frozen field.
    pub name: String,
    /// Frozen field.
    pub value: String,
}
impl Projection for ic_blob_storage_contracts::dto::upload::manifest::UploadManifestHeader {
    type Stored = UploadManifestHeader;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub enum UploadManifestInspection {
    /// Frozen variant.
    Unprepared,
    /// Frozen variant.
    Prepared(UploadManifestDeclaration),
}
impl Projection for ic_blob_storage_contracts::dto::upload::manifest::UploadManifestInspection {
    type Stored = UploadManifestInspection;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct UploadManifestMutation {
    /// Frozen field.
    pub observation: UploadManifestResponse,
    /// Frozen field.
    pub changed: bool,
}
impl Projection for ic_blob_storage_contracts::dto::upload::manifest::UploadManifestMutation {
    type Stored = UploadManifestMutation;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct UploadManifestRequest {
    /// Frozen field.
    pub permission: UploadAdmissionRequest,
    /// Frozen field.
    pub declaration: UploadManifestDeclaration,
}
impl Projection for ic_blob_storage_contracts::dto::upload::manifest::UploadManifestRequest {
    type Stored = UploadManifestRequest;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct UploadManifestResponse {
    /// Frozen field.
    pub permission: UploadAdmissionRequest,
    /// Frozen field.
    pub manifest: UploadManifestInspection,
}
impl Projection for ic_blob_storage_contracts::dto::upload::manifest::UploadManifestResponse {
    type Stored = UploadManifestResponse;
}
/// Frozen historical schema.
#[derive(Clone, Debug, PartialEq, CandidType, Deserialize, Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub enum UploadState {
    /// Frozen variant.
    Reserved,
    /// Frozen variant.
    ExposurePossible,
    /// Frozen variant.
    Confirmed,
    /// Frozen variant.
    Cancelled,
}
impl Projection for ic_blob_storage_contracts::dto::upload::UploadState {
    type Stored = UploadState;
}

/// Explicit historical counterpart; application DTOs themselves have no Candid trait.
pub trait Projection {
    /// Frozen representation.
    type Stored: CandidType + Serialize + serde::de::DeserializeOwned;
}
impl<T: Projection> Projection for (T,) {
    type Stored = (T::Stored,);
}
impl<T: Projection, E: Projection> Projection for Result<T, E> {
    type Stored = Result<T::Stored, E::Stored>;
}
impl<T: Projection> Projection for Option<T> {
    type Stored = Option<T::Stored>;
}
/// Convert exact named fields through bounded passive Serde without changing the saved codec.
///
/// # Panics
/// Panics when fixture fields differ from their frozen projection or exceed its bound.
pub fn project<T: Serialize, R: serde::de::DeserializeOwned>(value: T) -> R {
    let bytes = ic_blob_storage_contracts::document::encode(value).expect("historical projection");
    ic_blob_storage_contracts::document::decode(&bytes, 4 * 1024 * 1024)
        .expect("historical projection")
}
/// Encode a historical one-argument packet through its frozen local schema.
///
/// # Errors
/// Returns codec or historical canister call failures.
///
/// # Panics
/// Panics if fixture fields cannot project into the frozen schema.
pub fn encode<T: Projection + Serialize>(value: T) -> Result<Vec<u8>, String> {
    ic_platform::candid::contract::encode_one(project::<_, T::Stored>(value))
        .map_err(|e| e.to_string())
}
/// Decode a historical one-value reply through its frozen local schema.
///
/// # Errors
/// Returns codec or historical canister call failures.
///
/// # Panics
/// Panics if fixture fields cannot project into the frozen schema.
pub fn decode<T: Projection + serde::de::DeserializeOwned>(
    bytes: &[u8],
    config: &ic_platform::candid::decoder::DecoderConfig,
) -> Result<T, String> {
    ic_platform::candid::decoder::decode_one_with_config::<T::Stored>(bytes, config)
        .map(project)
        .map_err(|e| e.to_string())
}
/// Read an explicitly selected retired image. Never a current-service fallback.
///
/// # Errors
/// Returns codec or historical canister call failures.
///
/// # Panics
/// Panics if fixture fields cannot project into the frozen schema.
pub fn query_args<R, A>(
    pic: &ic_testkit::pocket_ic::PocketIc,
    service: Principal,
    caller: Principal,
    method: &str,
    args: A,
    limits: ic_platform::candid::DecodeLimits,
) -> Result<R, String>
where
    R: Projection + serde::de::DeserializeOwned,
    A: Projection + Serialize,
    A::Stored: ic_platform::candid::contract::ArgumentEncoder,
    R::Stored: for<'de> ic_platform::candid::contract::ArgumentDecoder<'de>,
{
    ic_testkit::pic::query_candid_args::<R::Stored, A::Stored>(
        pic,
        service,
        caller,
        method,
        project(args),
        limits,
    )
    .map(project)
    .map_err(|e| e.to_string())
}
/// Update an explicitly selected retired image; saved packets remain its Candid contract.
///
/// # Errors
/// Returns codec or historical canister call failures.
///
/// # Panics
/// Panics if fixture fields cannot project into the frozen schema.
pub fn update_args<R, A>(
    pic: &ic_testkit::pocket_ic::PocketIc,
    service: Principal,
    caller: Principal,
    method: &str,
    args: A,
    limits: ic_platform::candid::DecodeLimits,
) -> Result<R, String>
where
    R: Projection + serde::de::DeserializeOwned,
    A: Projection + Serialize,
    A::Stored: ic_platform::candid::contract::ArgumentEncoder,
    R::Stored: for<'de> ic_platform::candid::contract::ArgumentDecoder<'de>,
{
    ic_testkit::pic::update_candid_args::<R::Stored, A::Stored>(
        pic,
        service,
        caller,
        method,
        project(args),
        limits,
    )
    .map(project)
    .map_err(|e| e.to_string())
}
mod decimal {
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn serialize<T: std::fmt::Display, S: Serializer>(v: &T, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&v.to_string())
    }
    pub fn deserialize<'de, T: std::str::FromStr, D: Deserializer<'de>>(
        d: D,
    ) -> Result<T, D::Error> {
        String::deserialize(d)?
            .parse()
            .map_err(|_| serde::de::Error::custom("historical integer"))
    }
}

impl Projection
    for ic_blob_storage_contracts::dto::upload::certificate::CaffeineUploadCertificateResponse
{
    type Stored = Self;
}
impl Projection for String {
    type Stored = Self;
}

impl<T: Projection> Projection for &T {
    type Stored = T::Stored;
}
