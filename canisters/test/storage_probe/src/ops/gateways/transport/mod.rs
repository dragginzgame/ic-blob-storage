//! Labelled local update substitute, never a Cashier query/update fallback.
use super::{failure, scope};
use crate::ops::{ProbeMemory, STATE, TRAP_WRITE};
use blob_test_protocol::storage::{Failure, WriteFault, gateways::TransportInput};
use ic_blob_storage::{
    model::service::upload::UploadContext,
    ops::{
        caffeine::query::{
            CashierQuery, CashierQueryRequest,
            transport::{CashierQueryResponse, CashierQueryTransport},
        },
        service::gateways::{
            StableGatewayRegistry, access::GatewayRegistryAccess, reply::GatewaySyncRequest,
        },
    },
};
use std::num::NonZeroUsize;

pub(crate) struct FixtureRegistry {
    pub callback_fault: bool,
}
impl GatewayRegistryAccess for FixtureRegistry {
    type Memory = ProbeMemory;
    fn with_gateway_registry<R>(
        &self,
        operation: impl FnOnce(&mut StableGatewayRegistry<ProbeMemory>) -> R,
    ) -> R {
        STATE.with_borrow_mut(|state| {
            TRAP_WRITE.set(self.callback_fault.then_some(WriteFault::Gateways));
            let result = operation(&mut state.as_mut().unwrap().gateways);
            TRAP_WRITE.set(None);
            result
        })
    }
}
pub(crate) fn attempt(
    context: UploadContext,
    input: TransportInput,
) -> Result<GatewaySyncRequest, Failure> {
    if context.service != ic_cdk::api::canister_self() {
        return Err(Failure::Binding);
    }
    let scope = scope(input.scope)?;
    STATE.with_borrow(|state| {
        let state = state.as_ref().unwrap();
        if state
            .gateways
            .inspect(context, scope)
            .map_err(failure)?
            .fenced
        {
            return Err(Failure::Fenced);
        }
        input
            .token
            .checked_sub(1)
            .and_then(|v| usize::try_from(v).ok())
            .and_then(|i| state.gateway_attempts.get(i))
            .cloned()
            .ok_or(Failure::Unknown)
    })
}
// Dedicated test transport targets a different, visibly fixture-only method.
// The exact canonical empty arguments still travel over an actual IC call.
pub(crate) struct LocalQuery;
impl CashierQueryTransport for LocalQuery {
    type Error = Failure;
    async fn query(
        &self,
        request: &CashierQueryRequest,
        max_bytes: NonZeroUsize,
    ) -> Result<CashierQueryResponse, Failure> {
        if request.query() != CashierQuery::StorageGateways {
            return Err(Failure::Binding);
        }
        let response = ic_cdk::call::Call::bounded_wait(request.cashier(), "fixture_gateway_query")
            .with_raw_args(request.arguments())
            .await
            .map_err(|_| Failure::Transport)?;
        let bytes = response.as_ref();
        if bytes.len() > max_bytes.get() {
            return Err(Failure::Capacity);
        }
        Ok(CashierQueryResponse {
            cashier: request.cashier(),
            bytes: bytes.to_vec(),
        })
    }
}
