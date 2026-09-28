//! One host query for an existing durable attempt, with callback revalidation.
use super::complete_sync;
use crate::{
    model::service::upload::UploadContext,
    ops::{
        caffeine::{
            gateway::GatewayReplyLimits,
            query::{
                reply::{BoundGatewayReplyError, QueryReplyBindingError},
                transport::CashierQueryTransport,
            },
        },
        service::gateways::{
            access::GatewayRegistryAccess,
            reply::{GatewaySyncReplyError, GatewaySyncRequest},
        },
    },
};
use thiserror::Error;

/// Failed query or application; pending state remains unless another operation
/// invalidated it. The host retains the original attempt for explicit cancellation.
#[derive(Debug, Error)]
pub enum GatewayQueryError<E> {
    /// Authority, fence, correlation or decoder rejection.
    #[error(transparent)]
    Registry(#[from] GatewaySyncReplyError),
    /// Host transport failed; never interpreted as an empty list.
    #[error("gateway query transport failed")]
    Transport(E),
}

/// Execute one query after checking the previously persisted attempt on polling.
/// Release the store borrow before calling transport and revalidate against the
/// current owner before committing. Preserve the original operator and request
/// across the await; callback caller identity is not operator authority.
///
/// No retries, cancellation, timers or lifecycle hooks are installed. This is a
/// read-only query; repeated host invocation is not permission for paid effects.
/// A dropped future, failed query or failed decode leaves pending state available
/// for explicit exact cancellation or operator invalidation. Hosts must establish
/// actual service context and qualified authenticated transport independently.
/// # Errors
/// Rejects stale/unauthorized/fenced attempts before transport; transport and
/// completion failures preserve the current registry without overwriting edits.
/// # Panics
/// Host memory/platform traps must propagate for IC rollback.
pub async fn query_sync<H: GatewayRegistryAccess, T: CashierQueryTransport>(
    host: &H,
    execution: UploadContext,
    attempt: &GatewaySyncRequest,
    transport: &T,
    limits: GatewayReplyLimits,
) -> Result<(), GatewayQueryError<T::Error>> {
    host.with_gateway_registry(|store| store.check_request(execution, attempt))
        .map_err(GatewaySyncReplyError::from)?;
    let response = transport
        .query(attempt.request(), limits.max_bytes)
        .await
        .map_err(GatewayQueryError::Transport)?;
    if response.cashier != attempt.request().cashier() {
        return Err(
            GatewaySyncReplyError::Reply(BoundGatewayReplyError::Binding(
                QueryReplyBindingError::SourceMismatch,
            ))
            .into(),
        );
    }
    host.with_gateway_registry(|store| {
        complete_sync(
            store,
            execution,
            attempt,
            attempt.scope(),
            &response.bytes,
            limits,
        )
    })?;
    Ok(())
}
