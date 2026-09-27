//! Test-only transport to the explicitly installed local source canister.

use blob_test_protocol::{GatewaySyncRequest, SyncFailure};
use candid::Principal;
use ic_blob_storage::{
    model::gateway::registry::{GatewayScope, GatewaySyncError, GatewaySyncToken},
    ops::caffeine::{
        gateway::{GatewayReplyError, GatewayReplyLimits},
        query::{
            CashierQuery, CashierQueryError, CashierQueryRequest, reply::BoundGatewayReplyError,
        },
    },
};
use ic_cdk::call::Call;
use std::num::NonZeroU128;

use super::{bound, mutate};

/// Original provider query held across the controlled source's reentrant schedule.
pub(crate) struct GatewaySyncResponse {
    request: CashierQueryRequest,
    bytes: Vec<u8>,
}

pub(crate) fn authorize(service: Principal, actor: Principal) -> Result<(), SyncFailure> {
    let record = super::archive::current(service, actor).ok_or(SyncFailure::Denied)?;
    if record.fenced {
        return Err(SyncFailure::Fenced);
    }
    Ok(())
}

fn expected(input: GatewaySyncRequest) -> Result<GatewayScope, SyncFailure> {
    GatewayScope::new(
        input.service,
        NonZeroU128::new(input.namespace).ok_or(SyncFailure::Binding)?,
        input.source,
    )
    .map_err(|_| SyncFailure::Binding)
}

pub(crate) fn preview(
    service: Principal,
    actor: Principal,
    input: GatewaySyncRequest,
) -> Result<(), SyncFailure> {
    let record = super::archive::current(service, actor).ok_or(SyncFailure::Denied)?;
    let scope = GatewayScope::new(
        record.service,
        super::number(record.namespace),
        record.sync_source,
    )
    .map_err(|_| SyncFailure::Binding)?;
    record.sync_control.check(
        scope,
        ic_blob_storage::model::gateway::registry::GatewaySyncView {
            last_sequence: record.last_sync,
            pending_sequence: record.pending_sync,
        },
        expected(input)?,
        input.revision,
        input.sequence,
    )
}

pub(crate) fn begin(
    input: GatewaySyncRequest,
) -> Result<(GatewaySyncToken, GatewayScope), SyncFailure> {
    let expected = expected(input)?;
    mutate(|state| {
        let registry = &mut state.registry;
        state.sync_control.check(
            registry.scope(),
            registry.sync_view(),
            expected,
            input.revision,
            input.sequence,
        )?;
        let token = registry.begin_sync().map_err(sync_error)?;
        Ok((token, registry.scope()))
    })
}

pub(crate) async fn fetch(
    scope: GatewayScope,
    input: GatewaySyncRequest,
) -> Result<GatewaySyncResponse, SyncFailure> {
    let request = CashierQueryRequest::new(scope.cashier(), CashierQuery::StorageGateways)
        .map_err(|error| match error {
            CashierQueryError::InvalidPrincipal { .. }
            | CashierQueryError::AuditCursorAccountMismatch => SyncFailure::Binding,
            CashierQueryError::EncodingFailed => SyncFailure::Transport,
        })?;
    // This local scheduling endpoint needs fixture-only revision/sequence data.
    // It is not the provider's empty-argument query. Retain the selected provider
    // request for target/method validation when handling the resulting list.
    let response = Call::bounded_wait(request.cashier(), "fixture_gateways")
        .with_arg(input)
        .await
        .map_err(|_| SyncFailure::Transport)?;
    Ok(GatewaySyncResponse {
        request,
        bytes: response.into_bytes(),
    })
}

pub(crate) fn apply(
    token: GatewaySyncToken,
    scope: GatewayScope,
    response: &GatewaySyncResponse,
) -> Result<(), SyncFailure> {
    if super::archive::recovery::is_fenced() {
        return Err(SyncFailure::Fenced);
    }
    mutate(|state| {
        let result = response
            .request
            .apply_gateway_sync_reply(
                &mut state.registry,
                token,
                scope,
                &response.bytes,
                GatewayReplyLimits {
                    max_bytes: bound(4096),
                    decoding_quota: bound(100_000),
                    skipping_quota: bound(1000),
                    max_type_entries: bound(32),
                },
            )
            .map_err(|error| match error {
                BoundGatewayReplyError::Binding(_) => SyncFailure::Binding,
                BoundGatewayReplyError::Reply(GatewayReplyError::ReplyTooLarge) => {
                    SyncFailure::ReplyTooLarge
                }
                BoundGatewayReplyError::Reply(GatewayReplyError::InvalidReply) => {
                    SyncFailure::InvalidReply
                }
                BoundGatewayReplyError::Reply(GatewayReplyError::Sync(error)) => sync_error(error),
            });
        if result.is_ok() {
            state.journey.reads.invalidate();
        }
        result
    })
}

pub(crate) fn cancel(token: GatewaySyncToken) {
    if super::archive::recovery::is_fenced() {
        return;
    }
    mutate(|state| {
        // A stale callback must never cancel a newer attempt. Exact cancellation
        // intentionally does nothing when revocation/replacement consumed this token.
        let _ = state.registry.cancel_sync(token);
    });
}

fn sync_error(error: GatewaySyncError) -> SyncFailure {
    match error {
        GatewaySyncError::SyncInProgress => SyncFailure::InProgress,
        GatewaySyncError::StaleSync => SyncFailure::Stale,
        GatewaySyncError::InvalidList(_) => SyncFailure::InvalidReply,
        GatewaySyncError::WrongScope | GatewaySyncError::SequenceExhausted => {
            SyncFailure::Admission
        }
    }
}
