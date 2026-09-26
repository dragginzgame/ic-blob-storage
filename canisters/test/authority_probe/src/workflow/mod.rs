//! Test-only orchestration using shared library policy and fixture state access.

use blob_test_protocol::SyncFailure;
use blob_test_protocol::content::{ContentProbeCase, ContentProbeFailure, ContentProbeReport};
use candid::Principal;
use ic_blob_storage::{
    model::catalog::pending::PendingPageLimits,
    policy::{
        catalog::{assess_gateway_pending, assess_tenant_usage, tenant::assess_tenant_references},
        gateway::GatewayCallbackContext,
        tenant::{TenantAccessContext, assess_tenant_access},
    },
};

use crate::ops;

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
    ops::read(|state| {
        assess_tenant_usage(&state.catalog, context)
            .ok()
            .map(|usage| usage.logical_bytes)
    })
}

pub(crate) fn reference_live(
    context: TenantAccessContext,
    root: u8,
    service: Principal,
    tenant: Principal,
) -> Option<bool> {
    let key = ops::reference(root, service, tenant)?;
    ops::read(|state| {
        assess_tenant_references(&state.catalog, &[key], context, ops::bound(1))
            .ok()?
            .first()
            .copied()
    })
}

pub(crate) fn release(context: TenantAccessContext, root: u8) -> bool {
    let Some(binding) = ops::binding(root) else {
        return false;
    };
    if assess_tenant_access(binding, context).is_err() {
        return false;
    }
    // Synchronous local admission and mutation; no await or provider effect.
    ops::release(root, context.actor)
}

pub(crate) fn pending(context: TenantAccessContext) -> Option<Vec<u8>> {
    ops::read(|state| {
        assess_gateway_pending(
            &state.catalog,
            &state.registry,
            GatewayCallbackContext {
                service: context.service,
                actor: context.actor,
            },
            None,
            PendingPageLimits {
                max_scan: ops::bound(2),
                max_results: ops::bound(2),
            },
        )
        .ok()
        .map(|page| {
            page.entries
                .iter()
                .map(|item| item.root.as_bytes()[0])
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
    ops::read(|state| context.actor == state.operator && context.service == state.catalog.service())
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

pub(crate) async fn sync_gateway(context: TenantAccessContext) -> Result<(), SyncFailure> {
    if !is_operator(context) {
        return Err(SyncFailure::Denied);
    }
    let (token, scope) = ops::sync::begin()?;
    let outcome = match ops::sync::fetch(scope).await {
        Ok(bytes) => ops::sync::apply(token, scope, &bytes),
        Err(error) => Err(error),
    };
    if outcome.is_err() {
        // This local read-only attempt is abandoned explicitly. No automatic
        // retry, paid effect, persistence or provider completion rule is implied.
        ops::sync::cancel(token);
    }
    outcome
}
