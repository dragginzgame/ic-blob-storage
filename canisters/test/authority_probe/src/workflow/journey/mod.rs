//! A local protocol-shaped journey, not deployed Caffeine behavior or persistence.
use crate::model::content::ContentRequest;
use crate::ops;
use ic_blob_storage_contracts::dto::upload::certificate::CaffeineUploadCertificateResponse;
pub(crate) mod readback;
use blob_test_protocol::journey::{
    JourneyFailure, JourneyProgress, JourneyReservation, JourneyUpload, JourneyUsage,
};
use ic_blob_storage::policy::gateway::GatewayCallbackContext;
use ic_blob_storage::policy::gateway::assess_gateway_callback;
use ic_blob_storage::policy::tenant::TenantAccessContext;
use ic_blob_storage::policy::tenant::assess_tenant_access;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::identity::batch::ProviderRootBatch;
use ic_blob_storage_contracts::identity::batch::RootBatchLimits;
use ic_blob_storage_contracts::upload::history::LifecyclePhase;
use ic_blob_storage_contracts::upload::history::UploadRootState;

fn tenant(context: TenantAccessContext) -> Result<(), JourneyFailure> {
    ops::read(|s| {
        if s.journey.tenants.contains(&context.actor) && s.service == context.service {
            Ok(())
        } else {
            Err(JourneyFailure::Denied)
        }
    })
}

pub(crate) fn reserve(
    context: TenantAccessContext,
    input: &JourneyReservation,
) -> Result<(), JourneyFailure> {
    tenant(context)?;
    let request = ops::journey::request(context.service, context.actor, input.upload)?;
    ops::journey::reserve(context.actor, request, &input.manifest)
}

pub(crate) fn certificate(
    context: TenantAccessContext,
    text: &str,
) -> Result<CaffeineUploadCertificateResponse, JourneyFailure> {
    tenant(context)?;
    let root: ProviderRootHash = text.parse().map_err(|_| JourneyFailure::InvalidInput)?;
    let request = ops::journey::lookup(root)?;
    assess_tenant_access(request.upload.object.first.object(), context)
        .map_err(|_| JourneyFailure::Denied)?;
    ops::journey::expose(context.actor, request)?;
    Ok(CaffeineUploadCertificateResponse {
        method: "upload".to_owned(),
        blob_hash: root.to_string(),
    })
}

pub(crate) fn append(
    context: TenantAccessContext,
    root: &[u8],
    index: u64,
    bytes: &[u8],
) -> Result<(), JourneyFailure> {
    ops::journey::append(owned_request(context, root)?, index, bytes)
}

pub(crate) fn progress(
    context: TenantAccessContext,
    root: &[u8],
) -> Result<JourneyProgress, JourneyFailure> {
    ops::journey::progress(owned_request(context, root)?)
}

fn owned_request(
    context: TenantAccessContext,
    root: &[u8],
) -> Result<ContentRequest, JourneyFailure> {
    tenant(context)?;
    let root = ProviderRootHash::try_from(root).map_err(|_| JourneyFailure::InvalidInput)?;
    let request = ops::journey::lookup(root)?;
    assess_tenant_access(request.upload.object.first.object(), context)
        .map_err(|_| JourneyFailure::Denied)?;
    Ok(request)
}

pub(crate) fn cancel(
    context: TenantAccessContext,
    input: JourneyUpload,
) -> Result<(), JourneyFailure> {
    tenant(context)?;
    ops::journey::cancel(
        context.actor,
        ops::journey::request(context.service, context.actor, input)?,
    )
}

fn gateway(
    context: TenantAccessContext,
    root: ProviderRootHash,
) -> Result<ContentRequest, JourneyFailure> {
    let request = ops::journey::lookup(root)?;
    ops::read(|s| {
        assess_gateway_callback(
            request.upload.object.first.object(),
            &s.registry,
            GatewayCallbackContext {
                service: context.service,
                actor: context.actor,
            },
        )
    })
    .map_err(|_| JourneyFailure::Denied)?;
    Ok(request)
}

pub(crate) fn release(context: TenantAccessContext, root: &[u8]) -> Result<(), JourneyFailure> {
    tenant(context)?;
    let request = ops::journey::lookup(
        ProviderRootHash::try_from(root).map_err(|_| JourneyFailure::InvalidInput)?,
    )?;
    assess_tenant_access(request.upload.object.first.object(), context)
        .map_err(|_| JourneyFailure::Denied)?;
    ops::journey::release(context.actor, request)
}

pub(crate) fn usage(context: TenantAccessContext) -> Result<JourneyUsage, JourneyFailure> {
    tenant(context)?;
    Ok(ops::read(|s| {
        let u = s
            .journey
            .catalog
            .tenant_usage(context.actor)
            .expect("valid durable totals");
        JourneyUsage {
            logical: u.logical_bytes,
            physical: u.physical_bytes,
            liability: u.liability_bytes,
        }
    }))
}

pub(crate) fn live(
    context: TenantAccessContext,
    roots: &[Vec<u8>],
) -> Result<Vec<bool>, JourneyFailure> {
    let batch = ProviderRootBatch::from_bytes(
        roots,
        RootBatchLimits {
            max_entries: ops::bound(8),
            max_bytes: ops::bound(256),
        },
    )
    .map_err(|_| JourneyFailure::InvalidInput)?;
    let statuses = ops::read(|s| {
        ic_blob_storage::policy::gateway::assess_gateway_membership(
            &s.registry,
            GatewayCallbackContext {
                service: context.service,
                actor: context.actor,
            },
        )
        .map_err(|_| JourneyFailure::Denied)?;
        s.journey
            .catalog
            .observe_roots(
                ops::durable::context(s.service, s.operator),
                ops::number(1),
                &batch,
            )
            .map_err(|_| JourneyFailure::InvalidPhase)
    })?;
    statuses
        .into_iter()
        .map(|status| match status {
            ic_blob_storage::ops::service::uploads::read::UploadRootObservation::Malformed(_) => {
                Err(JourneyFailure::InvalidInput)
            }
            ic_blob_storage::ops::service::uploads::read::UploadRootObservation::Known(view)
                if matches!(
                    view.state,
                    UploadRootState::Confirmed(
                        LifecyclePhase::DeletionPending
                            | LifecyclePhase::ProviderDeleted
                            | LifecyclePhase::Settled
                    )
                ) =>
            {
                Ok(false)
            }
            _ => Ok(true),
        })
        .collect()
}

pub(crate) fn pending(context: TenantAccessContext) -> Result<Vec<Vec<u8>>, JourneyFailure> {
    ops::read(|s| {
        ic_blob_storage::policy::gateway::assess_gateway_membership(
            &s.registry,
            GatewayCallbackContext {
                service: context.service,
                actor: context.actor,
            },
        )
        .map_err(|_| JourneyFailure::Denied)?;
        s.journey
            .catalog
            .scan(
                ops::durable::context(s.service, s.operator),
                ic_blob_storage::ops::service::uploads::read::UploadScanScope::Service {
                    namespace: ops::number(1),
                },
                ic_blob_storage_contracts::upload::history::UploadScanFilter::DeletionPending,
                None,
                ic_blob_storage::model::catalog::admission::read::UploadPageLimits {
                    max_scan: ops::bound(8),
                    max_results: ops::bound(8),
                },
            )
            .map(|p| {
                p.entries
                    .iter()
                    .map(|e| e.request.object.root.as_bytes().to_vec())
                    .collect()
            })
            .map_err(|_| JourneyFailure::InvalidPhase)
    })
}

pub(crate) fn deleted(
    context: TenantAccessContext,
    roots: Vec<Vec<u8>>,
) -> Result<(), JourneyFailure> {
    // Check membership even for an empty callback. Endpoint traps on any failure,
    // rolling back earlier mutations in the same real IC message execution.
    live(context, &roots)?;
    for root in roots {
        let root = ProviderRootHash::try_from(root.as_slice())
            .map_err(|_| JourneyFailure::InvalidInput)?;
        ops::journey::deleted(gateway(context, root)?)?;
    }
    Ok(())
}

pub(crate) fn settled(context: TenantAccessContext, root: &[u8]) -> Result<(), JourneyFailure> {
    if !super::is_operator(context) {
        return Err(JourneyFailure::Denied);
    }
    let root = ProviderRootHash::try_from(root).map_err(|_| JourneyFailure::InvalidInput)?;
    ops::journey::settled(ops::journey::lookup(root)?)
}
