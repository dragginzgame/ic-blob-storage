//! Canonical service/provider method names. Linking exports no endpoints.
/// Reviewed Caffeine ingress update name. Its wire reply is a plain record, not Result.
pub const CAFFEINE_UPLOAD_CERTIFICATE_METHOD: &str = "_immutableObjectStorageCreateCertificate";
/// Uploader-only read-only assessment; its reply is never a certificate or permit.
pub const UPLOAD_CERTIFICATE_ASSESSMENT_METHOD: &str = "blob_upload_certificate_assessment";
/// Canonical query method, also callable through replicated IC execution.
/// Linking the library does not export it.
pub const REFERENCE_RECEIPT_METHOD: &str = "blob_reference_receipt";
/// Canonical update method. The caller must durably retain the exact intent first.
pub const REFERENCE_APPLY_METHOD: &str = "blob_apply_reference";
/// Canonical passive local query; linking exports no endpoint.
pub const LOCAL_STATUS_METHOD: &str = "blob_local_status";
/// Canonical explicit operator update. Linking exports no endpoint.
pub const GATEWAY_REVOCATION_METHOD: &str = "blob_revoke_gateway";
/// Explicit operator refresh update; linking exports no endpoint.
pub const GATEWAY_SYNC_METHOD: &str = "blob_sync_gateways";
/// Exact local read-only cancellation update; linking exports no endpoint.
pub const GATEWAY_SYNC_CANCEL_METHOD: &str = "blob_cancel_gateway_sync";
/// Canonical passive reference query; linking exports no endpoint.
pub const REFERENCE_STATUS_METHOD: &str = "blob_reference_status";
/// Canonical tenant query; linking the library exports no endpoint.
pub const REFERENCE_CAPACITY_METHOD: &str = "blob_reference_capacity";
/// Canonical service update method. No endpoint is exported by this constant.
pub const DOWNLOAD_METHOD: &str = "blob_download_descriptor";
/// Canonical passive service query; linking exports no endpoint.
pub const FUNDING_PREPARATION_METHOD: &str = "blob_funding_preparation_assessment";
/// Canonical passive funding history query. Linking exports no endpoint.
pub const FUNDING_HISTORY_METHOD: &str = "blob_funding_history";
/// Canonical passive exact-outcome query. Linking exports no endpoint.
pub const FUNDING_OUTCOME_METHOD: &str = "blob_funding_outcome";
/// Canonical admission update. Linking the library exports no endpoint.
pub const UPLOAD_ADMISSION_METHOD: &str = "blob_admit_upload";
/// Read-only exact permission query, also callable through replicated execution.
pub const UPLOAD_ADMISSION_STATUS_METHOD: &str = "blob_upload_admission";
/// Canonical tenant permission withdrawal; this is not a provider delete call.
pub const UPLOAD_REVOCATION_METHOD: &str = "blob_revoke_upload";
/// Canonical query, also called through replicated execution. Linking exports nothing.
pub const UPLOAD_STATUS_METHOD: &str = "blob_upload_status";
/// Canonical passive tenant query; linking exports no endpoint.
pub const UPLOAD_CAPACITY_METHOD: &str = "blob_upload_capacity";
/// Canonical bounded history query. Linking exports no endpoint.
pub const UPLOAD_HISTORY_METHOD: &str = "blob_upload_history";
/// Canonical tenant query; linking the library exports no endpoint.
pub const UPLOAD_DISCOVERY_METHOD: &str = "blob_lookup_content";
/// Explicit operator compare-and-set update; linking exports no endpoint.
pub const TENANT_UPDATE_METHOD: &str = "blob_update_tenant";
/// Scoped enrollment inspection for the operator or original tenant.
pub const TENANT_INSPECTION_METHOD: &str = "blob_tenant";
/// Canonical verifier update; linking the library exports no endpoint.
pub const UPLOAD_ATTEST_METHOD: &str = "blob_attest_upload";
/// Canonical historical receipt query; absence never authorizes a new upload.
pub const UPLOAD_ATTESTATION_METHOD: &str = "blob_upload_attestation";
/// Canonical verifier-only historical manifest query; no fetch or attestation is implied.
pub const UPLOAD_VERIFICATION_MANIFEST_METHOD: &str = "blob_verification_manifest";
/// Canonical preparation update. Linking exports no endpoint.
pub const UPLOAD_MANIFEST_PREPARE_METHOD: &str = "blob_prepare_upload";
/// Canonical exact permission manifest query, including after fencing/revocation.
pub const UPLOAD_MANIFEST_INSPECT_METHOD: &str = "blob_upload_manifest";
/// Canonical verifier-only query. Linking the library exports no endpoint.
pub const UPLOAD_VERIFICATION_PLAN_METHOD: &str = "blob_verification_plan";
