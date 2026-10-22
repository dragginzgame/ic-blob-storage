// Frozen named inputs and literal bytes, captured before removing application Candid traits.
use contracts::{dto, wire};
use ic_blob_storage_contracts as contracts;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

#[derive(serde::Deserialize)]
struct Case {
    input: Value,
    hex: String,
}

fn check<T: Serialize + DeserializeOwned>(label: &str, method: &str, source: &str) {
    let cases: Vec<Case> = serde_json::from_str(source).unwrap();
    for case in &cases {
        let value: T =
            serde_json::from_value(case.input.clone()).unwrap_or_else(|e| panic!("{label}: {e}"));
        let request = label.ends_with("-request");
        let bytes = match (method, request) {
            ("blob_tenant", true) => wire::tenant::inspection_request(
                serde_json::from_value(case.input.clone()).unwrap(),
            )
            .unwrap(),
            ("blob_update_tenant", true) => {
                wire::tenant::update_request(serde_json::from_value(case.input.clone()).unwrap())
                    .unwrap()
            }
            ("blob_tenant", false) => wire::tenant::inspection_response(
                serde_json::from_value(case.input.clone()).unwrap(),
            )
            .unwrap(),
            ("blob_update_tenant", false) => {
                wire::tenant::update_response(serde_json::from_value(case.input.clone()).unwrap())
                    .unwrap()
            }
            (_, true) => wire::encode_request(method, &value).unwrap(),
            (_, false) => wire::encode_response(method, &value).unwrap(),
        };
        let frozen = hex::decode(&case.hex).unwrap();
        assert_eq!(bytes, frozen, "{label}: changed frozen bytes");
        let decoded = match (method, request) {
            ("blob_tenant", true) => {
                serde_json::to_value(wire::tenant::decode_inspection_request(&frozen).unwrap())
                    .unwrap()
            }
            ("blob_update_tenant", true) => {
                serde_json::to_value(wire::tenant::decode_update_request(&frozen).unwrap()).unwrap()
            }
            ("blob_tenant", false) => {
                serde_json::to_value(wire::tenant::decode_inspection_response(&frozen).unwrap())
                    .unwrap()
            }
            ("blob_update_tenant", false) => {
                serde_json::to_value(wire::tenant::decode_update_response(&frozen).unwrap())
                    .unwrap()
            }
            _ => {
                let decoded: T = if request {
                    wire::decode_request(method, &frozen, 4 * 1024 * 1024).unwrap()
                } else {
                    wire::decode_response(method, &frozen, 4 * 1024 * 1024).unwrap()
                };
                serde_json::to_value(decoded).unwrap()
            }
        };
        assert_eq!(
            decoded, case.input,
            "{label}: changed frozen interpretation"
        );
    }
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One typed registration per owned method protects the frozen contract"
)]
fn frozen_method_and_lifecycle_surface() {
    let mut expected = std::collections::BTreeSet::new();
    macro_rules! method {
        ($method:literal, $request:ty, $response:ty) => {{
            assert!(
                expected.insert($method),
                "duplicate frozen method: {}",
                $method
            );
            check::<$request>(
                concat!($method, "-request"),
                $method,
                include_str!(concat!("../vectors/", $method, "-request.json")),
            );
            check::<$response>(
                concat!($method, "-response"),
                $method,
                include_str!(concat!("../vectors/", $method, "-response.json")),
            );
        }};
    }
    method!(
        "blob_resume_current_instance",
        (),
        Result<
            (),
            dto::recovery::CurrentInstanceRecoveryFailure,
        >
    );
    method!(
        "blob_configuration",
        (),
        Result<
            dto::configuration::HostConfigurationView,
            dto::configuration::HostFailure,
        >
    );
    method!(
        "blob_local_status",
        dto::operator::OperatorScope,
        Result<
            dto::operator::LocalServiceStatus,
            dto::operator::LocalStatusFailure,
        >
    );
    method!(
        "blob_update_tenant",
        dto::tenant::TenantUpdateRequest,
        Result<
            dto::tenant::TenantEnrollmentResponse,
            dto::tenant::TenantFailure,
        >
    );
    method!(
        "blob_tenant",
        dto::tenant::TenantScope,
        Result<
            dto::tenant::TenantEnrollmentResponse,
            dto::tenant::TenantFailure,
        >
    );
    method!(
        "blob_admit_upload",
        dto::upload::admission::UploadAdmissionRequest,
        Result<
            dto::upload::admission::UploadAdmissionMutation,
            dto::upload::admission::UploadAdmissionFailure,
        >
    );
    method!(
        "blob_upload_admission",
        dto::upload::admission::UploadAdmissionRequest,
        Result<
            dto::upload::admission::UploadAdmissionResponse,
            dto::upload::admission::UploadAdmissionFailure,
        >
    );
    method!(
        "blob_upload_status",
        dto::reference::ReferenceUpload,
        Result<
            dto::upload::UploadStatusResponse,
            dto::upload::UploadStatusFailure,
        >
    );
    method!(
        "blob_revoke_upload",
        dto::upload::admission::UploadAdmissionRequest,
        Result<
            dto::upload::admission::UploadRevocationResponse,
            dto::upload::admission::UploadAdmissionFailure,
        >
    );
    method!(
        "blob_prepare_upload",
        dto::upload::manifest::UploadManifestRequest,
        Result<
            dto::upload::manifest::UploadManifestMutation,
            dto::upload::manifest::UploadManifestFailure,
        >
    );
    method!(
        "blob_upload_certificate_assessment",
        String,
        Result<
            dto::upload::certificate::UploadCertificateAssessmentResponse,
            dto::upload::exposure::UploadExposureFailure,
        >
    );
    method!(
        "blob_upload_manifest",
        dto::upload::admission::UploadAdmissionRequest,
        Result<
            dto::upload::manifest::UploadManifestResponse,
            dto::upload::manifest::UploadManifestFailure,
        >
    );
    method!(
        "blob_apply_reference",
        dto::reference::ReferenceCommand,
        Result<
            dto::reference::ReferenceMutationResponse,
            dto::reference::ReferenceFailure,
        >
    );
    method!(
        "blob_reference_receipt",
        dto::reference::ReferenceCommand,
        Result<
            dto::reference::ReferenceReceiptLookup,
            dto::reference::ReferenceFailure,
        >
    );
    method!(
        "blob_upload_history",
        dto::upload::history::UploadHistoryRequest,
        Result<
            dto::upload::history::UploadHistoryPage,
            dto::upload::history::UploadHistoryFailure,
        >
    );
    method!(
        "blob_funding_history",
        dto::funding::FundingHistoryRequest,
        Result<
            dto::funding::FundingHistoryPage,
            dto::funding::FundingHistoryFailure,
        >
    );
    method!(
        "blob_funding_outcome",
        dto::funding::outcome::FundingOutcomeRequest,
        Result<
            Option<dto::funding::outcome::FundingOutcomeResponse>,
            dto::funding::outcome::FundingOutcomeFailure,
        >
    );
    method!(
        "blob_funding_preparation_assessment",
        dto::funding::assessment::FundingPreparationRequest,
        Result<
            dto::funding::assessment::FundingPreparationResponse,
            dto::funding::assessment::FundingPreparationFailure,
        >
    );
    method!(
        "blob_revoke_gateway",
        dto::gateway::GatewayRevocationRequest,
        Result<
            dto::gateway::GatewayRevocationResponse,
            dto::gateway::GatewayRevocationFailure,
        >
    );
    method!(
        "blob_sync_gateways",
        dto::operator::OperatorScope,
        Result<
            dto::gateway::sync::GatewaySyncResponse,
            dto::gateway::sync::GatewaySyncFailure,
        >
    );
    method!(
        "blob_cancel_gateway_sync",
        dto::gateway::sync::GatewaySyncCancellation,
        Result<
            (),
            dto::gateway::sync::GatewaySyncFailure,
        >
    );
    method!(
        "blob_upload_capacity",
        dto::tenant::TenantScope,
        Result<
            dto::upload::capacity::UploadCapacityResponse,
            dto::upload::capacity::UploadCapacityFailure,
        >
    );
    method!(
        "blob_reference_capacity",
        dto::reference::capacity::ReferenceCapacityRequest,
        Result<
            dto::reference::capacity::ReferenceCapacityResponse,
            dto::reference::capacity::ReferenceCapacityFailure,
        >
    );
    method!(
        "blob_lookup_content",
        dto::upload::discovery::UploadDiscoveryRequest,
        Result<
            dto::upload::discovery::UploadDiscoveryResponse,
            dto::upload::discovery::UploadDiscoveryFailure,
        >
    );
    method!(
        "blob_download_descriptor",
        dto::download::DownloadRequest,
        Result<
            dto::download::DownloadResponse,
            dto::download::DownloadFailure,
        >
    );
    method!(
        "blob_reference_status",
        dto::reference::status::ReferenceStatusRequest,
        Result<
            dto::reference::status::ReferenceStatusResponse,
            dto::reference::ReferenceFailure,
        >
    );
    method!(
        "blob_inspect_account",
        dto::account::AccountInspectionRequest,
        Result<
            dto::account::AccountInspectionResponse,
            dto::account::AccountInspectionFailure,
        >
    );
    check::<dto::configuration::ServiceInstallationInput>(
        "blob_install-request",
        "blob_install",
        include_str!("../vectors/blob_install-request.json"),
    );
    check::<()>(
        "blob_post_upgrade-request",
        "blob_post_upgrade",
        include_str!("../vectors/blob_post_upgrade-request.json"),
    );
    let actual: std::collections::BTreeSet<_> = contracts::endpoints::SERVICE_METHODS
        .iter()
        .filter(|m| m.codec == contracts::endpoints::MethodCodec::IcWire)
        .map(|m| m.name)
        .collect();
    assert_eq!(
        expected, actual,
        "update frozen vectors when changing the owned method surface"
    );
}
