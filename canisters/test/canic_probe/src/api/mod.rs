//! Explicit managed blob endpoints over the shared service workflows.
//! This controlled artifact has no provider effects or operational recovery authority.
#![expect(
    clippy::needless_pass_by_value,
    reason = "Candid adapters own decoded manifest input"
)]
#![expect(
    clippy::large_types_passed_by_value,
    reason = "Candid adapters own the decoded verifier statement"
)]
use super::{context, dto::TransportFailure, ops};
pub(crate) use ic_blob_storage::dto::{
    account::{AccountInspectionFailure, AccountInspectionRequest, AccountInspectionResponse},
    configuration::{HostConfigurationView, HostFailure},
    download::{DownloadFailure, DownloadRequest, DownloadResponse},
    funding::{
        FundingHistoryFailure, FundingHistoryPage, FundingHistoryRequest,
        assessment::{
            FundingPreparationFailure, FundingPreparationRequest, FundingPreparationResponse,
        },
        outcome::{FundingOutcomeFailure, FundingOutcomeRequest, FundingOutcomeResponse},
    },
    gateway::{
        GatewayRevocationFailure, GatewayRevocationRequest, GatewayRevocationResponse,
        sync::{GatewaySyncCancellation, GatewaySyncFailure, GatewaySyncResponse},
    },
    operator::{LocalServiceStatus, LocalStatusFailure, OperatorScope},
    reference::{
        ReferenceCommand, ReferenceFailure, ReferenceMutationResponse, ReferenceReceiptLookup,
        ReferenceUpload,
        capacity::{ReferenceCapacityFailure, ReferenceCapacityRequest, ReferenceCapacityResponse},
        status::{ReferenceStatusRequest, ReferenceStatusResponse},
    },
    tenant::{TenantEnrollmentResponse, TenantFailure, TenantScope, TenantUpdateRequest},
    upload::{
        UploadStatusFailure, UploadStatusResponse,
        admission::{
            UploadAdmissionFailure, UploadAdmissionMutation, UploadAdmissionRequest,
            UploadAdmissionResponse, UploadRevocationResponse,
        },
        capacity::{UploadCapacityFailure, UploadCapacityResponse},
        certificate::{CaffeineUploadCertificateResponse, UploadCertificateAssessmentResponse},
        completion::{
            UploadAttestationFailure, UploadAttestationMutation, UploadAttestationRequest,
            UploadAttestationResponse, UploadVerificationPlan,
        },
        discovery::{UploadDiscoveryFailure, UploadDiscoveryRequest, UploadDiscoveryResponse},
        exposure::UploadExposureFailure,
        history::{UploadHistoryFailure, UploadHistoryPage, UploadHistoryRequest},
        manifest::{
            UploadManifestFailure, UploadManifestMutation, UploadManifestRequest,
            UploadManifestResponse,
        },
    },
};
use ic_blob_storage::workflow;

fn actual_context() -> ic_blob_storage::model::service::upload::UploadContext {
    context::context().unwrap_or_else(|_| ic_cdk::trap("managed service is inactive"))
}

#[canic::canic_query(public, decode = crate::limits::REQUEST)]
fn blob_configuration() -> Result<HostConfigurationView, TransportFailure<HostFailure>> {
    let context = actual_context();
    ops::read(|owner| workflow::installation::inspect(owner, context).map_err(TransportFailure))
}

#[canic::canic_update(public, decode = crate::limits::REQUEST)]
fn blob_admit_upload(
    input: UploadAdmissionRequest,
) -> Result<UploadAdmissionMutation, TransportFailure<UploadAdmissionFailure>> {
    let context = actual_context();
    ops::mutate(|owner| {
        workflow::uploads::admission::admit(
            &mut owner.stores_mut().uploads,
            context,
            input,
            ic_cdk::api::time(),
        )
        .map_err(TransportFailure)
    })
}

#[canic::canic_query(public, decode = crate::limits::REQUEST)]
fn blob_upload_admission(
    input: UploadAdmissionRequest,
) -> Result<UploadAdmissionResponse, TransportFailure<UploadAdmissionFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::uploads::admission::inspect(&owner.stores().uploads, context, input)
            .map_err(TransportFailure)
    })
}

#[canic::canic_update(public, decode = crate::limits::MANIFEST)]
fn blob_prepare_upload(
    input: UploadManifestRequest,
) -> Result<UploadManifestMutation, TransportFailure<UploadManifestFailure>> {
    let context = actual_context();
    ops::mutate(|owner| {
        workflow::uploads::manifests::prepare(
            &mut owner.stores_mut().uploads,
            context,
            &input,
            ic_cdk::api::time(),
        )
        .map_err(TransportFailure)
    })
}

#[canic::canic_query(public, decode = crate::limits::REQUEST)]
fn blob_upload_manifest(
    input: UploadAdmissionRequest,
) -> Result<UploadManifestResponse, TransportFailure<UploadManifestFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::uploads::manifests::inspect(&owner.stores().uploads, context, input)
            .map_err(TransportFailure)
    })
}

#[canic::canic_query(public, decode = crate::limits::REQUEST)]
fn blob_upload_capacity(
    input: TenantScope,
) -> Result<UploadCapacityResponse, TransportFailure<UploadCapacityFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::uploads::capacity::inspect(&owner.stores().uploads, context, input)
            .map_err(TransportFailure)
    })
}

#[canic::canic_update(public, decode = crate::limits::REQUEST)]
fn blob_update_tenant(
    input: TenantUpdateRequest,
) -> Result<TenantEnrollmentResponse, TransportFailure<TenantFailure>> {
    let context = actual_context();
    ops::mutate(|owner| {
        workflow::tenants::update(&mut owner.stores_mut().uploads, context, input)
            .map_err(TransportFailure)
    })
}
#[canic::canic_query(public, decode = crate::limits::REQUEST)]
fn blob_tenant(
    input: TenantScope,
) -> Result<TenantEnrollmentResponse, TransportFailure<TenantFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::tenants::inspect(&owner.stores().uploads, context, input)
            .map_err(TransportFailure)
    })
}
#[canic::canic_query(public, decode = crate::limits::REQUEST)]
fn blob_upload_status(
    input: ReferenceUpload,
) -> Result<UploadStatusResponse, TransportFailure<UploadStatusFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::uploads::inspect(&owner.stores().uploads, context, input)
            .map_err(TransportFailure)
    })
}
#[canic::canic_update(public, decode = crate::limits::REQUEST)]
fn blob_revoke_upload(
    input: UploadAdmissionRequest,
) -> Result<UploadRevocationResponse, TransportFailure<UploadAdmissionFailure>> {
    let context = actual_context();
    ops::mutate(|owner| {
        workflow::uploads::admission::revoke(&mut owner.stores_mut().uploads, context, input)
            .map_err(TransportFailure)
    })
}
#[canic::canic_query(public, decode = crate::limits::REQUEST)]
fn blob_upload_history(
    input: UploadHistoryRequest,
) -> Result<UploadHistoryPage, TransportFailure<UploadHistoryFailure>> {
    let context = actual_context();
    let limits = ic_blob_storage::model::catalog::admission::read::UploadPageLimits {
        max_scan: 64.try_into().expect("fixed scan bound"),
        max_results: 32.try_into().expect("fixed reply bound"),
    };
    ops::read(|owner| {
        workflow::uploads::history::inspect(&owner.stores().uploads, context, input, limits)
            .map_err(TransportFailure)
    })
}
#[canic::canic_query(public, decode = crate::limits::REQUEST)]
fn blob_lookup_content(
    input: UploadDiscoveryRequest,
) -> Result<UploadDiscoveryResponse, TransportFailure<UploadDiscoveryFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::uploads::discovery::inspect(&owner.stores().uploads, context, input)
            .map_err(TransportFailure)
    })
}
#[canic::canic_update(public, decode = crate::limits::REQUEST)]
fn blob_apply_reference(
    input: ReferenceCommand,
) -> Result<ReferenceMutationResponse, TransportFailure<ReferenceFailure>> {
    let context = actual_context();
    ops::mutate(|owner| {
        workflow::references::apply(&mut owner.stores_mut().uploads, context, input)
            .map_err(TransportFailure)
    })
}
#[canic::canic_query(public, decode = crate::limits::REQUEST)]
fn blob_reference_receipt(
    input: ReferenceCommand,
) -> Result<ReferenceReceiptLookup, TransportFailure<ReferenceFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::references::receipt(&owner.stores().uploads, context, input)
            .map_err(TransportFailure)
    })
}
#[canic::canic_query(public, decode = crate::limits::REQUEST)]
fn blob_reference_status(
    input: ReferenceStatusRequest,
) -> Result<ReferenceStatusResponse, TransportFailure<ReferenceFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::references::status::inspect(&owner.stores().uploads, context, input)
            .map_err(TransportFailure)
    })
}
#[canic::canic_query(public, decode = crate::limits::REQUEST)]
fn blob_reference_capacity(
    input: ReferenceCapacityRequest,
) -> Result<ReferenceCapacityResponse, TransportFailure<ReferenceCapacityFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::references::capacity::inspect(&owner.stores().uploads, context, input)
            .map_err(TransportFailure)
    })
}
#[canic::canic_query(public, decode = crate::limits::REQUEST)]
fn blob_local_status(
    input: OperatorScope,
) -> Result<LocalServiceStatus, TransportFailure<LocalStatusFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::operator::inspect(owner.stores().into(), context, input).map_err(TransportFailure)
    })
}
#[canic::canic_query(public, decode = crate::limits::REQUEST)]
fn blob_host_memory_status(
    input: OperatorScope,
) -> Result<
    ic_blob_storage::dto::operator::memory::HostMemoryStatus,
    TransportFailure<LocalStatusFailure>,
> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::operator::memory_status(
            owner.stores().into(),
            context,
            input,
            ic_blob_storage::ic_memory::default_memory_manager_memory_allocation_summary,
        )
        .map_err(TransportFailure)
    })
}
#[canic::canic_query(public, decode = crate::limits::REQUEST)]
fn blob_funding_history(
    input: FundingHistoryRequest,
) -> Result<FundingHistoryPage, TransportFailure<FundingHistoryFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::funding::history::inspect(
            &owner.stores().funding,
            context,
            input,
            32.try_into().expect("fixed reply bound"),
        )
        .map_err(TransportFailure)
    })
}
#[canic::canic_query(public, decode = crate::limits::REQUEST)]
fn blob_funding_outcome(
    input: FundingOutcomeRequest,
) -> Result<Option<FundingOutcomeResponse>, TransportFailure<FundingOutcomeFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::funding::outcome::inspect(&owner.stores().funding, context, input)
            .map_err(TransportFailure)
    })
}
#[canic::canic_query(public, decode = crate::limits::REQUEST)]
fn blob_funding_preparation_assessment(
    input: FundingPreparationRequest,
) -> Result<FundingPreparationResponse, TransportFailure<FundingPreparationFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::funding::assessment::inspect(&owner.stores().funding, context, input)
            .map_err(TransportFailure)
    })
}

#[canic::canic_update(public, decode = crate::limits::REQUEST)]
fn blob_attest_upload(
    input: UploadAttestationRequest,
) -> Result<UploadAttestationMutation, TransportFailure<UploadAttestationFailure>> {
    let context = actual_context();
    ops::mutate(|owner| {
        let authority = owner.completion_authority();
        workflow::uploads::completion::attest(
            &mut owner.stores_mut().uploads,
            authority,
            context,
            &input,
            ic_cdk::api::time(),
        )
        .map_err(TransportFailure)
    })
}

#[canic::canic_query(public, decode = crate::limits::REQUEST)]
fn blob_upload_attestation(
    input: UploadAdmissionRequest,
) -> Result<UploadAttestationResponse, TransportFailure<UploadAttestationFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::uploads::completion::inspect(
            &owner.stores().uploads,
            owner.completion_authority(),
            context,
            input,
        )
        .map_err(TransportFailure)
    })
}

#[canic::canic_query(public, decode = crate::limits::REQUEST)]
fn blob_verification_manifest(
    input: UploadAdmissionRequest,
) -> Result<UploadManifestResponse, TransportFailure<UploadManifestFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::uploads::completion::manifest(
            &owner.stores().uploads,
            owner.completion_authority(),
            context,
            input,
        )
        .map_err(TransportFailure)
    })
}

#[canic::canic_query(public, decode = crate::limits::REQUEST)]
fn blob_verification_plan(
    input: UploadAdmissionRequest,
) -> Result<UploadVerificationPlan, TransportFailure<UploadAttestationFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::uploads::completion::verification_plan(
            &owner.stores().uploads,
            owner.completion_authority(),
            owner.download_scope(),
            context,
            input,
        )
        .map_err(TransportFailure)
    })
}

#[canic::canic_update(public, decode = crate::limits::REQUEST)]
fn blob_download_descriptor(
    input: DownloadRequest,
) -> Result<DownloadResponse, TransportFailure<DownloadFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::reads::download::handle(
            &owner.stores().uploads,
            context,
            owner.download_scope(),
            input,
        )
        .map_err(TransportFailure)
    })
}

#[canic::canic_query(public, decode = crate::limits::REQUEST)]
fn blob_upload_certificate_assessment(
    root: String,
) -> Result<UploadCertificateAssessmentResponse, TransportFailure<UploadExposureFailure>> {
    let context = actual_context();
    let now = ic_cdk::api::time();
    ops::read(|owner| {
        let store = &owner.stores().uploads;
        let permission = workflow::uploads::certificate::resolve(store, context, &root, now)
            .map_err(TransportFailure)?;
        workflow::uploads::certificate::inspect(
            store,
            context,
            &root,
            ops::certificate::evidence(permission, now),
            now,
        )
        .map_err(TransportFailure)
    })
}

#[canic::canic_update(
    public,
    name = "_immutableObjectStorageCreateCertificate",
    on_access_denied = "reject",
    decode = crate::limits::REQUEST
)]
fn caffeine_upload_certificate(root: String) -> CaffeineUploadCertificateResponse {
    let context = actual_context();
    let now = ic_cdk::api::time();
    ops::mutate(|owner| {
        let store = &mut owner.stores_mut().uploads;
        let permission = workflow::uploads::certificate::resolve(store, context, &root, now)
            .unwrap_or_else(|_| ic_cdk::trap("certificate issuance refused"));
        workflow::uploads::certificate::issue(
            store,
            context,
            &root,
            ops::certificate::evidence(permission, now),
            now,
        )
        .unwrap_or_else(|_| ic_cdk::trap("certificate issuance refused"))
    })
}

#[canic::canic_update(public, decode = crate::limits::REQUEST)]
fn blob_revoke_gateway(
    input: GatewayRevocationRequest,
) -> Result<GatewayRevocationResponse, TransportFailure<GatewayRevocationFailure>> {
    let context = actual_context();
    ops::mutate(|owner| {
        workflow::gateways::revocation::revoke(&mut owner.stores_mut().gateways, context, input)
            .map_err(TransportFailure)
    })
}

#[canic::canic_update(public, decode = crate::limits::REQUEST)]
fn blob_cancel_gateway_sync(
    input: GatewaySyncCancellation,
) -> Result<(), TransportFailure<GatewaySyncFailure>> {
    let context = actual_context();
    ops::mutate(|owner| {
        workflow::gateways::sync::cancel(&mut owner.stores_mut().gateways, context, input)
            .map_err(TransportFailure)
    })
}

#[canic::canic_update(public, decode = crate::limits::REQUEST)]
async fn blob_sync_gateways(
    input: OperatorScope,
) -> Result<GatewaySyncResponse, TransportFailure<GatewaySyncFailure>> {
    workflow::gateways::sync::refresh(
        &ops::gateways::GatewayHost,
        actual_context(),
        input,
        30.try_into().expect("fixed timeout"),
        ops::gateways::limits(),
    )
    .await
    .map_err(TransportFailure)
}

#[canic::canic_update(public, decode = crate::limits::REQUEST)]
async fn blob_inspect_account(
    input: AccountInspectionRequest,
) -> Result<AccountInspectionResponse, TransportFailure<AccountInspectionFailure>> {
    use ic_blob_storage::ops::caffeine::query::transport::replicated::account::ReplicatedAccountQuery;
    let context = actual_context();
    let transport = ReplicatedAccountQuery::new(
        input.scope.service,
        input.scope.cashier,
        input.scope.payment_account,
        30.try_into().expect("fixed timeout"),
    )
    .map_err(|_| TransportFailure(AccountInspectionFailure::Invalid))?;
    workflow::account::inspect(
        &ops::account::AccountHost,
        &transport,
        context,
        input,
        ops::account::limits(),
    )
    .await
    .map_err(TransportFailure)
}
