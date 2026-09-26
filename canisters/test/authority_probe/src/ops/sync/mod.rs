//! Test-only transport to the explicitly installed local source canister.

use blob_test_protocol::SyncFailure;
use ic_blob_storage::{
    model::gateway::registry::{GatewayScope, GatewaySyncError, GatewaySyncToken},
    ops::caffeine::gateway::{GatewayReplyError, GatewayReplyLimits, apply_gateway_sync_reply},
};
use ic_cdk::call::Call;

use super::{STATE, bound};

pub(crate) fn begin() -> Result<(GatewaySyncToken, GatewayScope), SyncFailure> {
    STATE.with_borrow_mut(|state| {
        let registry = &mut state.as_mut().expect("initialized fixture").registry;
        let token = registry.begin_sync().map_err(sync_error)?;
        Ok((token, registry.scope()))
    })
}

pub(crate) async fn fetch(scope: GatewayScope) -> Result<Vec<u8>, SyncFailure> {
    Call::bounded_wait(scope.cashier(), "fixture_gateways")
        .await
        .map(ic_cdk::call::Response::into_bytes)
        .map_err(|_| SyncFailure::Transport)
}

pub(crate) fn apply(
    token: GatewaySyncToken,
    scope: GatewayScope,
    bytes: &[u8],
) -> Result<(), SyncFailure> {
    STATE.with_borrow_mut(|state| {
        apply_gateway_sync_reply(
            &mut state.as_mut().expect("initialized fixture").registry,
            token,
            scope,
            bytes,
            GatewayReplyLimits {
                max_bytes: bound(4096),
                decoding_quota: bound(100_000),
                skipping_quota: bound(1000),
                max_type_entries: bound(32),
            },
        )
        .map_err(|error| match error {
            GatewayReplyError::ReplyTooLarge => SyncFailure::ReplyTooLarge,
            GatewayReplyError::InvalidReply => SyncFailure::InvalidReply,
            GatewayReplyError::Sync(error) => sync_error(error),
        })
    })
}

pub(crate) fn cancel(token: GatewaySyncToken) {
    STATE.with_borrow_mut(|state| {
        // A stale callback must never cancel a newer attempt. Exact cancellation
        // intentionally does nothing when revocation/replacement consumed this token.
        let _ = state
            .as_mut()
            .expect("initialized fixture")
            .registry
            .cancel_sync(token);
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
