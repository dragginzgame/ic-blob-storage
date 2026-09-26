//! A local protocol-shaped journey, not deployed Caffeine behavior or persistence.
use crate::ops;
pub(crate) mod readback;
use blob_test_protocol::journey::{
    JourneyCertificate, JourneyFailure, JourneyProgress, JourneyReservation, JourneyUpload,
    JourneyUsage,
};
use candid::Principal;
use ic_blob_storage::{
    model::{
        catalog::{admission::read::UploadRootState, pending::PendingPageLimits},
        identity::{
            ProviderRootHash,
            batch::{ProviderRootBatch, RootBatchLimits},
        },
        lifecycle::LifecyclePhase,
    },
    policy::{
        catalog::{
            assess_gateway_pending,
            upload::{UploadRootStatus, assess_gateway_upload_roots},
        },
        gateway::{GatewayCallbackContext, assess_gateway_callback},
        tenant::{TenantAccessContext, assess_tenant_access},
    },
};

fn tenant(context: TenantAccessContext) -> Result<(), JourneyFailure> {
    ops::read(|s| {
        if s.journey.tenants.contains(&context.actor)
            && s.journey.catalog.confirmed().service() == context.service
        {
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
) -> Result<JourneyCertificate, JourneyFailure> {
    tenant(context)?;
    let root: ProviderRootHash = text.parse().map_err(|_| JourneyFailure::InvalidInput)?;
    let request = ops::journey::lookup(root)?;
    assess_tenant_access(request.object.first.object(), context)
        .map_err(|_| JourneyFailure::Denied)?;
    ops::journey::expose(context.actor, request)?;
    Ok(JourneyCertificate {
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
) -> Result<ic_blob_storage::model::catalog::admission::UploadRequest, JourneyFailure> {
    tenant(context)?;
    let root = ProviderRootHash::try_from(root).map_err(|_| JourneyFailure::InvalidInput)?;
    let request = ops::journey::lookup(root)?;
    assess_tenant_access(request.object.first.object(), context)
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
) -> Result<ic_blob_storage::model::catalog::admission::UploadRequest, JourneyFailure> {
    let request = ops::journey::lookup(root)?;
    ops::read(|s| {
        assess_gateway_callback(
            request.object.first.object(),
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

pub(crate) fn complete(
    context: TenantAccessContext,
    owner: Principal,
    input: JourneyUpload,
) -> Result<(), JourneyFailure> {
    let request = ops::journey::request(context.service, owner, input)?;
    gateway(context, request.object.root)?;
    // Explicitly substituted exact completion fact, not a client progress report.
    ops::journey::complete(request)
}

pub(crate) fn release(context: TenantAccessContext, root: &[u8]) -> Result<(), JourneyFailure> {
    tenant(context)?;
    let request = ops::journey::lookup(
        ProviderRootHash::try_from(root).map_err(|_| JourneyFailure::InvalidInput)?,
    )?;
    assess_tenant_access(request.object.first.object(), context)
        .map_err(|_| JourneyFailure::Denied)?;
    ops::journey::release(context.actor, request)
}

pub(crate) fn usage(context: TenantAccessContext) -> Result<JourneyUsage, JourneyFailure> {
    tenant(context)?;
    Ok(ops::read(|s| {
        let u = s.journey.catalog.tenant_usage(context.actor);
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
        assess_gateway_upload_roots(
            &s.journey.catalog,
            &s.registry,
            GatewayCallbackContext {
                service: context.service,
                actor: context.actor,
            },
            &batch,
        )
    })
    .map_err(|_| JourneyFailure::Denied)?;
    statuses
        .into_iter()
        .map(|status| match status {
            UploadRootStatus::Malformed(_) => Err(JourneyFailure::InvalidInput),
            UploadRootStatus::Known(UploadRootState::Confirmed(
                LifecyclePhase::DeletionPending
                | LifecyclePhase::ProviderDeleted
                | LifecyclePhase::Settled,
            )) => Ok(false),
            // Conservative fixture mapping: protect uncertainty and unknown roots.
            // This does not prove that bytes were uploaded or qualify provider behavior.
            _ => Ok(true),
        })
        .collect()
}

pub(crate) fn pending(context: TenantAccessContext) -> Result<Vec<Vec<u8>>, JourneyFailure> {
    ops::read(|s| {
        assess_gateway_pending(
            s.journey.catalog.confirmed(),
            &s.registry,
            GatewayCallbackContext {
                service: context.service,
                actor: context.actor,
            },
            None,
            PendingPageLimits {
                max_scan: ops::bound(8),
                max_results: ops::bound(8),
            },
        )
    })
    .map_err(|_| JourneyFailure::Denied)
    .map(|page| {
        page.entries
            .into_iter()
            .map(|entry| entry.root.as_bytes().to_vec())
            .collect()
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
