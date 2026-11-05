//! Fixture orchestration over the production durable owner and explicit caller policy.
pub(crate) mod balance;
pub(crate) mod journey;
use crate::ops;
use blob_test_protocol::{
    SyncFailure,
    content::{ContentProbeCase, ContentProbeFailure, ContentProbeReport},
    obligations::{ObligationProbeFact, ObligationProbeView},
    uploads::UploadProbeState,
};
use ic_blob_storage::{
    model::catalog::admission::read::UploadPageLimits,
    ops::service::uploads::read::{UploadRootObservation, UploadScanScope},
    policy::{
        gateway::{GatewayCallbackContext, assess_gateway_membership},
        tenant::{TenantAccessContext, assess_tenant_access},
    },
};
use ic_blob_storage_contracts::upload::history::UploadScanFilter;
use ic_platform::Principal;

pub(crate) fn unsettled_objects(context: TenantAccessContext) -> Option<Vec<ObligationProbeView>> {
    if !concrete_tenant(context.actor) {
        return None;
    }
    ops::obligations::views(context)
}
pub(crate) fn confirm_obligation(
    context: TenantAccessContext,
    root: u8,
    fact: ObligationProbeFact,
) -> bool {
    is_operator(context) && ops::obligations::confirm(root, fact)
}
pub(crate) fn upload_usage(context: TenantAccessContext) -> Option<u128> {
    if !concrete_tenant(context.actor) {
        return None;
    }
    ops::read(|state| {
        state
            .uploads
            .catalog
            .scan(
                ops::durable::context(context.service, context.actor),
                UploadScanScope::Tenant {
                    tenant: context.actor,
                    namespace: ops::number(1),
                },
                UploadScanFilter::All,
                None,
                UploadPageLimits {
                    max_scan: ops::bound(4),
                    max_results: ops::bound(4),
                },
            )
            .ok()?;
        Some(
            state
                .uploads
                .catalog
                .tenant_usage(context.actor)
                .ok()?
                .logical_bytes,
        )
    })
}
pub(crate) fn active_uploads(context: TenantAccessContext) -> Option<Vec<u8>> {
    if !concrete_tenant(context.actor) {
        return None;
    }
    ops::read(|state| {
        state
            .uploads
            .catalog
            .scan(
                ops::durable::context(context.service, context.actor),
                UploadScanScope::Tenant {
                    tenant: context.actor,
                    namespace: ops::number(1),
                },
                UploadScanFilter::Active,
                None,
                UploadPageLimits {
                    max_scan: ops::bound(4),
                    max_results: ops::bound(4),
                },
            )
            .ok()
            .map(|p| {
                p.entries
                    .iter()
                    .map(|e| u8::try_from(e.request.id.get().get()).expect("fixture ID"))
                    .collect()
            })
    })
}
pub(crate) fn upload_roots(context: TenantAccessContext) -> Option<Vec<UploadProbeState>> {
    ops::read(|state| {
        assess_gateway_membership(
            &state.registry,
            GatewayCallbackContext {
                service: context.service,
                actor: context.actor,
            },
        )
        .ok()?;
        state
            .uploads
            .catalog
            .observe_roots(
                ops::durable::context(state.service, state.operator),
                ops::number(1),
                &ops::uploads::roots(),
            )
            .ok()?
            .into_iter()
            .map(|e| match e {
                UploadRootObservation::Known(view) => ops::uploads::status(
                    ic_blob_storage::policy::catalog::upload::UploadRootStatus::Known(view.state),
                ),
                _ => None,
            })
            .collect()
    })
}
pub(crate) fn cancel_upload(context: TenantAccessContext, root: u8) -> bool {
    ops::read(|state| state.service == context.service) && ops::uploads::cancel(context.actor, root)
}
pub(crate) fn initialize(
    service: Principal,
    first: Principal,
    second: Principal,
    gateway: Principal,
    operator: Principal,
) {
    ops::initialize(service, first, second, gateway, operator);
}
pub(crate) fn usage(context: TenantAccessContext) -> Option<u128> {
    if !concrete_tenant(context.actor) {
        return None;
    }
    ops::read(|state| {
        state
            .catalog
            .scan(
                ops::durable::context(context.service, context.actor),
                UploadScanScope::Tenant {
                    tenant: context.actor,
                    namespace: ops::number(1),
                },
                UploadScanFilter::All,
                None,
                UploadPageLimits {
                    max_scan: ops::bound(2),
                    max_results: ops::bound(2),
                },
            )
            .ok()?;
        Some(
            state
                .catalog
                .tenant_usage(context.actor)
                .ok()?
                .logical_bytes,
        )
    })
}
pub(crate) fn reference_live(
    context: TenantAccessContext,
    root: u8,
    service: Principal,
    tenant: Principal,
) -> Option<bool> {
    let reference = ops::reference(root, service, tenant)?.reference;
    ops::read(|state| {
        state
            .catalog
            .reference_is_live(
                ops::durable::context(context.service, context.actor),
                ops::sample(state, root)?,
                reference,
            )
            .ok()
    })
}
pub(crate) fn release(context: TenantAccessContext, root: u8) -> bool {
    let Some(binding) = ops::binding(root) else {
        return false;
    };
    assess_tenant_access(binding, context).is_ok() && ops::release(root, context.actor)
}
pub(crate) fn pending(context: TenantAccessContext) -> Option<Vec<u8>> {
    ops::read(|state| {
        assess_gateway_membership(
            &state.registry,
            GatewayCallbackContext {
                service: context.service,
                actor: context.actor,
            },
        )
        .ok()?;
        state
            .catalog
            .scan(
                ops::durable::context(state.service, state.operator),
                UploadScanScope::Service {
                    namespace: ops::number(1),
                },
                UploadScanFilter::DeletionPending,
                None,
                UploadPageLimits {
                    max_scan: ops::bound(2),
                    max_results: ops::bound(2),
                },
            )
            .ok()
            .map(|p| {
                p.entries
                    .iter()
                    .map(|e| u8::try_from(e.request.id.get().get()).expect("fixture ID"))
                    .collect()
            })
    })
}
pub(crate) fn revoke_gateway(context: TenantAccessContext) -> bool {
    // Explicit fixture operator authority does not confer tenant access.
    if !is_operator(context) {
        return false;
    }
    ops::revoke_gateway();
    true
}

fn is_operator(context: TenantAccessContext) -> bool {
    ops::read(|state| context.actor == state.operator && context.service == state.service)
}

pub(crate) fn probe_content(
    context: TenantAccessContext,
    case: ContentProbeCase,
) -> Result<ContentProbeReport, ContentProbeFailure> {
    if !is_operator(context) {
        return Err(ContentProbeFailure::Denied);
    }
    ops::content::run(case)
}

pub(crate) fn preview_gateway_sync(
    context: TenantAccessContext,
    input: blob_test_protocol::GatewaySyncRequest,
) -> Result<(), SyncFailure> {
    ops::sync::authorize(context.service, context.actor)?;
    ops::sync::preview(context.service, context.actor, input)
}

pub(crate) async fn sync_gateway(
    context: TenantAccessContext,
    input: blob_test_protocol::GatewaySyncRequest,
) -> Result<(), SyncFailure> {
    ops::sync::authorize(context.service, context.actor)?;
    let (token, scope) = ops::sync::begin(input)?;
    let outcome = match ops::sync::fetch(scope, input).await {
        Ok(response) => ops::sync::apply(token, scope, &response),
        Err(error) => Err(error),
    };
    if outcome.is_err() {
        // This local read-only attempt is abandoned explicitly. No automatic
        // retry, paid effect or provider completion rule is implied.
        ops::sync::cancel(token);
    }
    outcome
}

pub(crate) fn archive(
    context: TenantAccessContext,
) -> Option<blob_test_protocol::authority::AuthorityArchiveView> {
    ops::archive::inspect(context.service, context.actor)
}

pub(crate) fn operator_status(
    context: TenantAccessContext,
) -> Option<blob_test_protocol::status::OperatorStatusView> {
    let snapshot = ops::archive::current(context.service, context.actor)?;
    let input = ops::status::billing::snapshot(&snapshot);
    let billing =
        ic_blob_storage::policy::billing::balance::assess_balance(input.limits, input.context);
    let diagnosis = ic_blob_storage::policy::diagnostics::assess_operator(
        ops::status::observation(&snapshot, &billing),
    );
    Some(ops::status::view(
        &snapshot, &diagnosis, &billing, input.now,
    ))
}

pub(crate) fn probe_checkpoint(
    context: TenantAccessContext,
    input: &blob_test_protocol::authority::CheckpointProbeInput,
) -> Result<
    blob_test_protocol::journey::JourneyProgress,
    blob_test_protocol::authority::CheckpointProbeFailure,
> {
    ops::archive::probe(context.service, context.actor, input)
}

// These fixture reads accept an empty catalog, so object lookup cannot authenticate
// a concrete tenant for them. Store scope checks still run on each read.
fn concrete_tenant(actor: Principal) -> bool {
    actor != Principal::anonymous() && actor != Principal::management_canister()
}
