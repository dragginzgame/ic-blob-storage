//! Explicit standalone host. Paid provider effects and operational recovery remain disabled.
#![expect(
    clippy::needless_pass_by_value,
    clippy::large_types_passed_by_value,
    reason = "Candid endpoint macros own decoded inputs"
)]
use crate::{dto, ops, workflow};
use ic_blob_storage::dto::funding::outcome::{
    FundingOutcomeFailure, FundingOutcomeRequest, FundingOutcomeResponse,
};
use ic_blob_storage::dto::funding::{
    FundingHistoryFailure, FundingHistoryPage, FundingHistoryRequest,
};
use ic_blob_storage::dto::operator::{LocalServiceStatus, LocalStatusFailure, OperatorScope};
use ic_blob_storage::dto::upload::history::{
    UploadHistoryFailure, UploadHistoryPage, UploadHistoryRequest,
};
use ic_blob_storage::{
    dto::{
        reference::{
            ReferenceCommand, ReferenceFailure, ReferenceMutationResponse, ReferenceReceiptLookup,
        },
        tenant::{TenantEnrollmentResponse, TenantFailure, TenantScope, TenantUpdateRequest},
        upload::{
            admission::{
                UploadAdmissionFailure, UploadAdmissionMutation, UploadAdmissionRequest,
                UploadAdmissionResponse, UploadRevocationResponse,
            },
            manifest::{
                UploadManifestFailure, UploadManifestMutation, UploadManifestRequest,
                UploadManifestResponse,
            },
        },
    },
    model::service::upload::UploadContext,
};
fn context() -> UploadContext {
    UploadContext {
        service: ic_cdk::api::canister_self(),
        actor: ic_cdk::api::msg_caller(),
    }
}
#[ic_cdk::init(hidden = true, decode_with = "ops::decode_configuration")]
// Custom decoding bounds raw Candid; explicitly retain its typed wire contract.
#[candid::candid_method(init)]
fn init(input: ic_blob_storage::dto::configuration::ServiceConfigurationInput) {
    workflow::install(&input);
}
#[ic_cdk::post_upgrade]
fn post_upgrade() {
    workflow::restore();
}
#[ic_cdk::query]
fn blob_configuration() -> Result<dto::HostConfigurationView, dto::HostFailure> {
    workflow::configuration(ic_cdk::api::msg_caller())
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_local_status(input: OperatorScope) -> Result<LocalServiceStatus, LocalStatusFailure> {
    workflow::local_status(context(), input)
}
#[ic_cdk::update(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(update)]
fn blob_update_tenant(
    input: TenantUpdateRequest,
) -> Result<TenantEnrollmentResponse, TenantFailure> {
    workflow::update_tenant(context(), input)
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_tenant(input: TenantScope) -> Result<TenantEnrollmentResponse, TenantFailure> {
    workflow::tenant(context(), input)
}
#[ic_cdk::update(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(update)]
fn blob_admit_upload(
    input: UploadAdmissionRequest,
) -> Result<UploadAdmissionMutation, UploadAdmissionFailure> {
    workflow::admit(context(), input, ic_cdk::api::time())
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_upload_admission(
    input: UploadAdmissionRequest,
) -> Result<UploadAdmissionResponse, UploadAdmissionFailure> {
    workflow::admission(context(), input)
}
#[ic_cdk::update(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(update)]
fn blob_revoke_upload(
    input: UploadAdmissionRequest,
) -> Result<UploadRevocationResponse, UploadAdmissionFailure> {
    workflow::revoke(context(), input)
}
#[ic_cdk::update(hidden = true, decode_with = "ops::decode_manifest")]
#[candid::candid_method(update)]
fn blob_prepare_upload(
    input: UploadManifestRequest,
) -> Result<UploadManifestMutation, UploadManifestFailure> {
    workflow::prepare(context(), &input, ic_cdk::api::time())
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_upload_manifest(
    input: UploadAdmissionRequest,
) -> Result<UploadManifestResponse, UploadManifestFailure> {
    workflow::manifest(context(), input)
}
#[ic_cdk::update(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(update)]
fn blob_apply_reference(
    input: ReferenceCommand,
) -> Result<ReferenceMutationResponse, ReferenceFailure> {
    workflow::reference(context(), input)
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_reference_receipt(
    input: ReferenceCommand,
) -> Result<ReferenceReceiptLookup, ReferenceFailure> {
    workflow::receipt(context(), input)
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_upload_history(
    input: UploadHistoryRequest,
) -> Result<UploadHistoryPage, UploadHistoryFailure> {
    workflow::history(context(), input)
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_funding_history(
    input: FundingHistoryRequest,
) -> Result<FundingHistoryPage, FundingHistoryFailure> {
    workflow::funding_history(context(), input)
}
#[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(query)]
fn blob_funding_outcome(
    input: FundingOutcomeRequest,
) -> Result<Option<FundingOutcomeResponse>, FundingOutcomeFailure> {
    workflow::funding_outcome(context(), input)
}
#[ic_cdk::update(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(update)]
fn blob_revoke_gateway(
    input: ic_blob_storage::dto::gateway::GatewayRevocationRequest,
) -> Result<
    ic_blob_storage::dto::gateway::GatewayRevocationResponse,
    ic_blob_storage::dto::gateway::GatewayRevocationFailure,
> {
    workflow::revoke_gateway(context(), input)
}
#[ic_cdk::update(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(update)]
async fn blob_sync_gateways(
    input: OperatorScope,
) -> Result<
    ic_blob_storage::dto::gateway::sync::GatewaySyncResponse,
    ic_blob_storage::dto::gateway::sync::GatewaySyncFailure,
> {
    workflow::sync_gateways(context(), input).await
}
#[ic_cdk::update(hidden = true, decode_with = "ops::decode")]
#[candid::candid_method(update)]
fn blob_cancel_gateway_sync(
    input: ic_blob_storage::dto::gateway::sync::GatewaySyncCancellation,
) -> Result<(), ic_blob_storage::dto::gateway::sync::GatewaySyncFailure> {
    workflow::cancel_gateway_sync(context(), input)
}
ic_cdk::export_candid!();
pub(crate) fn interface() -> String {
    __export_service()
}
#[cfg(test)]
mod tests {
    #[test]
    fn deployment_candid_matches_the_exported_contract() {
        fn schema(text: &str) -> String {
            text.lines()
                .map(str::trim)
                .filter(|line| !line.is_empty() && !line.starts_with("//"))
                .collect::<Vec<_>>()
                .join("\n")
        }
        assert_eq!(
            schema(&super::interface()),
            schema(include_str!("../../service.did"))
        );
    }
}
