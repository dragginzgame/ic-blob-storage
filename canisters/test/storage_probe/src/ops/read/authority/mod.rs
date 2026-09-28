//! Fixture-only conversion and scheduling call; no production read transport.
use crate::ops::{gateways, read};
use blob_test_protocol::storage::{Failure, gateways::ReadAuthorityInput};
use ic_blob_storage::{
    model::{
        gateway::registry::GatewayScope,
        lifecycle::{
            ReferenceId,
            binding::{ObjectBinding, ObjectIdentity, ReferenceKey},
        },
        service::{read::ReadTarget, upload::UploadContext},
    },
    ops::caffeine::query::{CashierQuery, CashierQueryRequest, transport::CashierQueryTransport},
};
use std::{cell::Cell, num::NonZeroU128};
thread_local! { static BUSY: Cell<bool> = const { Cell::new(false) }; }
// A single ephemeral test call, not a durable production session or recovery path.
pub(crate) fn enter() -> Result<(), Failure> {
    BUSY.with(|busy| {
        if busy.replace(true) {
            Err(Failure::Phase)
        } else {
            Ok(())
        }
    })
}
pub(crate) fn leave() {
    BUSY.set(false);
}
pub(crate) fn parse(
    execution: UploadContext,
    input: ReadAuthorityInput,
) -> Result<(GatewayScope, ReadTarget), Failure> {
    let content = read::lookup(execution, input.target.content)?;
    let number = |n| NonZeroU128::new(n).ok_or(Failure::Invalid);
    let object = ObjectBinding::new(
        execution.service,
        content.tenant,
        ObjectIdentity {
            namespace: content.namespace,
            object: number(input.target.object)?,
            incarnation: number(input.target.incarnation)?,
        },
    )
    .map_err(|_| Failure::Binding)?;
    Ok((
        GatewayScope::new(execution.service, content.namespace, input.cashier)
            .map_err(|_| Failure::Binding)?,
        ReadTarget {
            root: content.root,
            reference: ReferenceKey::new(object, ReferenceId::new(number(input.target.reference)?)),
            gateway: input.gateway,
        },
    ))
}
pub(crate) async fn wait_source(scope: GatewayScope) -> Result<(), Failure> {
    let request = CashierQueryRequest::new(scope.cashier(), CashierQuery::StorageGateways)
        .map_err(|_| Failure::Invalid)?;
    // Reuse the labelled scheduling substitute, discarding its list. No blob
    // bytes are fetched or verified by this authority-only test endpoint.
    gateways::transport::LocalQuery
        .query(&request, 2048.try_into().unwrap())
        .await
        .map(|_| ())
}
