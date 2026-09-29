//! Thin host composition; all tenant and blob transitions use shared workflows.
use crate::ops;
pub(crate) fn revoke_gateway(
    context: UploadContext,
    input: ic_blob_storage::dto::gateway::GatewayRevocationRequest,
) -> Result<
    ic_blob_storage::dto::gateway::GatewayRevocationResponse,
    ic_blob_storage::dto::gateway::GatewayRevocationFailure,
> {
    ops::mutate(|stores| {
        ic_blob_storage::workflow::gateways::revocation::revoke(
            &mut stores.gateways,
            context,
            input,
        )
    })
}
use ic_blob_storage::dto::funding::outcome::{
    FundingOutcomeFailure, FundingOutcomeRequest, FundingOutcomeResponse,
};
use ic_blob_storage::dto::funding::{
    FundingHistoryFailure, FundingHistoryPage, FundingHistoryRequest,
};
use ic_blob_storage::dto::upload::history::{
    UploadHistoryFailure, UploadHistoryPage, UploadHistoryRequest,
};
use ic_blob_storage::{
    dto::operator::{LocalServiceStatus, LocalStatusFailure, OperatorScope},
    ops::service::operator::OperatorStores,
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
pub(crate) fn history(
    context: UploadContext,
    input: UploadHistoryRequest,
) -> Result<UploadHistoryPage, UploadHistoryFailure> {
    use ic_blob_storage::model::catalog::admission::read::UploadPageLimits;
    use std::num::NonZeroUsize;
    ops::read(|stores| {
        ic_blob_storage::workflow::uploads::history::inspect(
            &stores.uploads,
            context,
            input,
            UploadPageLimits {
                max_scan: NonZeroUsize::new(64).expect("fixed scan bound"),
                max_results: NonZeroUsize::new(32).expect("fixed reply bound"),
            },
        )
    })
}
pub(crate) fn install(input: &ic_blob_storage::dto::configuration::ServiceConfigurationInput) {
    ops::install(input);
}
pub(crate) fn funding_history(
    context: UploadContext,
    input: FundingHistoryRequest,
) -> Result<FundingHistoryPage, FundingHistoryFailure> {
    ops::read(|stores| {
        ic_blob_storage::workflow::funding::history::inspect(
            &stores.funding,
            context,
            input,
            std::num::NonZeroUsize::new(32).expect("fixed funding page bound"),
        )
    })
}
pub(crate) fn restore() {
    ops::restore();
}
pub(crate) fn funding_outcome(
    context: UploadContext,
    input: FundingOutcomeRequest,
) -> Result<Option<FundingOutcomeResponse>, FundingOutcomeFailure> {
    ops::read(|stores| {
        ic_blob_storage::workflow::funding::outcome::inspect(&stores.funding, context, input)
    })
}
pub(crate) fn local_status(
    context: UploadContext,
    input: OperatorScope,
) -> Result<LocalServiceStatus, LocalStatusFailure> {
    ops::read(|stores| {
        ic_blob_storage::workflow::operator::inspect(OperatorStores::from(stores), context, input)
    })
}
pub(crate) fn configuration(
    actor: candid::Principal,
) -> Result<crate::dto::HostConfigurationView, crate::dto::HostFailure> {
    ops::configuration(actor)
}
pub(crate) fn update_tenant(
    context: UploadContext,
    input: TenantUpdateRequest,
) -> Result<TenantEnrollmentResponse, TenantFailure> {
    ops::mutate(|stores| {
        ic_blob_storage::workflow::tenants::update(&mut stores.uploads, context, input)
    })
}
pub(crate) fn tenant(
    context: UploadContext,
    input: TenantScope,
) -> Result<TenantEnrollmentResponse, TenantFailure> {
    ops::read(|stores| ic_blob_storage::workflow::tenants::inspect(&stores.uploads, context, input))
}
pub(crate) fn admit(
    context: UploadContext,
    input: UploadAdmissionRequest,
    now: u64,
) -> Result<UploadAdmissionMutation, UploadAdmissionFailure> {
    ops::mutate(|stores| {
        ic_blob_storage::workflow::uploads::admission::admit(
            &mut stores.uploads,
            context,
            input,
            now,
        )
    })
}
pub(crate) fn admission(
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<UploadAdmissionResponse, UploadAdmissionFailure> {
    ops::read(|stores| {
        ic_blob_storage::workflow::uploads::admission::inspect(&stores.uploads, context, input)
    })
}
pub(crate) fn revoke(
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<UploadRevocationResponse, UploadAdmissionFailure> {
    ops::mutate(|stores| {
        ic_blob_storage::workflow::uploads::admission::revoke(&mut stores.uploads, context, input)
    })
}
pub(crate) fn prepare(
    context: UploadContext,
    input: &UploadManifestRequest,
    now: u64,
) -> Result<UploadManifestMutation, UploadManifestFailure> {
    ops::mutate(|stores| {
        ic_blob_storage::workflow::uploads::manifests::prepare(
            &mut stores.uploads,
            context,
            input,
            now,
        )
    })
}
pub(crate) fn manifest(
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<UploadManifestResponse, UploadManifestFailure> {
    ops::read(|stores| {
        ic_blob_storage::workflow::uploads::manifests::inspect(&stores.uploads, context, input)
    })
}
pub(crate) fn reference(
    context: UploadContext,
    input: ReferenceCommand,
) -> Result<ReferenceMutationResponse, ReferenceFailure> {
    ops::mutate(|stores| {
        ic_blob_storage::workflow::references::apply(&mut stores.uploads, context, input)
    })
}
pub(crate) fn receipt(
    context: UploadContext,
    input: ReferenceCommand,
) -> Result<ReferenceReceiptLookup, ReferenceFailure> {
    ops::read(|stores| {
        ic_blob_storage::workflow::references::receipt(&stores.uploads, context, input)
    })
}

pub(crate) async fn sync_gateways(
    context: UploadContext,
    input: OperatorScope,
) -> Result<
    ic_blob_storage::dto::gateway::sync::GatewaySyncResponse,
    ic_blob_storage::dto::gateway::sync::GatewaySyncFailure,
> {
    ic_blob_storage::workflow::gateways::sync::refresh(
        &ops::gateways::GatewayHost,
        context,
        input,
        30.try_into().unwrap(),
        ops::gateways::limits(),
    )
    .await
}
pub(crate) fn cancel_gateway_sync(
    context: UploadContext,
    input: ic_blob_storage::dto::gateway::sync::GatewaySyncCancellation,
) -> Result<(), ic_blob_storage::dto::gateway::sync::GatewaySyncFailure> {
    ops::mutate(|stores| {
        ic_blob_storage::workflow::gateways::sync::cancel(&mut stores.gateways, context, input)
    })
}
