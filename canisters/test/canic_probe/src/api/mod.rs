//! Explicit managed blob endpoints over the shared service workflows.
//! This controlled artifact has no provider effects or operational recovery authority.
#![expect(
    clippy::needless_pass_by_value,
    reason = "Candid adapters own decoded manifest input"
)]
use super::{context, dto::TransportFailure, ops};
pub(crate) use ic_blob_storage::dto::{
    configuration::{HostConfigurationView, HostFailure},
    funding::{
        FundingHistoryFailure, FundingHistoryPage, FundingHistoryRequest,
        outcome::{FundingOutcomeFailure, FundingOutcomeRequest, FundingOutcomeResponse},
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
        discovery::{UploadDiscoveryFailure, UploadDiscoveryRequest, UploadDiscoveryResponse},
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

#[canic::canic_query(public)]
fn blob_configuration() -> Result<HostConfigurationView, TransportFailure<HostFailure>> {
    let context = actual_context();
    ops::read(|owner| workflow::installation::inspect(owner, context).map_err(TransportFailure))
}

#[canic::canic_update(public, payload(max_bytes = 4096))]
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

#[canic::canic_query(public)]
fn blob_upload_admission(
    input: UploadAdmissionRequest,
) -> Result<UploadAdmissionResponse, TransportFailure<UploadAdmissionFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::uploads::admission::inspect(&owner.stores().uploads, context, input)
            .map_err(TransportFailure)
    })
}

#[canic::canic_update(public, payload(max_bytes = 131_072))]
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

#[canic::canic_query(public)]
fn blob_upload_manifest(
    input: UploadAdmissionRequest,
) -> Result<UploadManifestResponse, TransportFailure<UploadManifestFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::uploads::manifests::inspect(&owner.stores().uploads, context, input)
            .map_err(TransportFailure)
    })
}

#[canic::canic_query(public)]
fn blob_upload_capacity(
    input: TenantScope,
) -> Result<UploadCapacityResponse, TransportFailure<UploadCapacityFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::uploads::capacity::inspect(&owner.stores().uploads, context, input)
            .map_err(TransportFailure)
    })
}

#[canic::canic_update(public, payload(max_bytes = 4096))]
fn blob_update_tenant(
    input: TenantUpdateRequest,
) -> Result<TenantEnrollmentResponse, TransportFailure<TenantFailure>> {
    let context = actual_context();
    ops::mutate(|owner| {
        workflow::tenants::update(&mut owner.stores_mut().uploads, context, input)
            .map_err(TransportFailure)
    })
}
#[canic::canic_query(public)]
fn blob_tenant(
    input: TenantScope,
) -> Result<TenantEnrollmentResponse, TransportFailure<TenantFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::tenants::inspect(&owner.stores().uploads, context, input)
            .map_err(TransportFailure)
    })
}
#[canic::canic_query(public)]
fn blob_upload_status(
    input: ReferenceUpload,
) -> Result<UploadStatusResponse, TransportFailure<UploadStatusFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::uploads::inspect(&owner.stores().uploads, context, input)
            .map_err(TransportFailure)
    })
}
#[canic::canic_update(public, payload(max_bytes = 4096))]
fn blob_revoke_upload(
    input: UploadAdmissionRequest,
) -> Result<UploadRevocationResponse, TransportFailure<UploadAdmissionFailure>> {
    let context = actual_context();
    ops::mutate(|owner| {
        workflow::uploads::admission::revoke(&mut owner.stores_mut().uploads, context, input)
            .map_err(TransportFailure)
    })
}
#[canic::canic_query(public)]
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
#[canic::canic_query(public)]
fn blob_lookup_content(
    input: UploadDiscoveryRequest,
) -> Result<UploadDiscoveryResponse, TransportFailure<UploadDiscoveryFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::uploads::discovery::inspect(&owner.stores().uploads, context, input)
            .map_err(TransportFailure)
    })
}
#[canic::canic_update(public, payload(max_bytes = 4096))]
fn blob_apply_reference(
    input: ReferenceCommand,
) -> Result<ReferenceMutationResponse, TransportFailure<ReferenceFailure>> {
    let context = actual_context();
    ops::mutate(|owner| {
        workflow::references::apply(&mut owner.stores_mut().uploads, context, input)
            .map_err(TransportFailure)
    })
}
#[canic::canic_query(public)]
fn blob_reference_receipt(
    input: ReferenceCommand,
) -> Result<ReferenceReceiptLookup, TransportFailure<ReferenceFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::references::receipt(&owner.stores().uploads, context, input)
            .map_err(TransportFailure)
    })
}
#[canic::canic_query(public)]
fn blob_reference_status(
    input: ReferenceStatusRequest,
) -> Result<ReferenceStatusResponse, TransportFailure<ReferenceFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::references::status::inspect(&owner.stores().uploads, context, input)
            .map_err(TransportFailure)
    })
}
#[canic::canic_query(public)]
fn blob_reference_capacity(
    input: ReferenceCapacityRequest,
) -> Result<ReferenceCapacityResponse, TransportFailure<ReferenceCapacityFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::references::capacity::inspect(&owner.stores().uploads, context, input)
            .map_err(TransportFailure)
    })
}
#[canic::canic_query(public)]
fn blob_local_status(
    input: OperatorScope,
) -> Result<LocalServiceStatus, TransportFailure<LocalStatusFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::operator::inspect(owner.stores().into(), context, input).map_err(TransportFailure)
    })
}
#[canic::canic_query(public)]
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
#[canic::canic_query(public)]
fn blob_funding_outcome(
    input: FundingOutcomeRequest,
) -> Result<Option<FundingOutcomeResponse>, TransportFailure<FundingOutcomeFailure>> {
    let context = actual_context();
    ops::read(|owner| {
        workflow::funding::outcome::inspect(&owner.stores().funding, context, input)
            .map_err(TransportFailure)
    })
}
