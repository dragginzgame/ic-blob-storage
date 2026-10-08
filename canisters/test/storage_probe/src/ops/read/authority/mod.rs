//! Fixture-only boundary conversion for the shared read workflow.
use crate::ops::read;
use blob_test_protocol::storage::{Failure, gateways::ReadSessionInput};
use ic_blob_storage::model::gateway::registry::GatewayScope;
use ic_blob_storage::model::service::read::ReadTarget;
use ic_blob_storage_contracts::binding::ObjectBinding;
use ic_blob_storage_contracts::binding::ObjectIdentity;
use ic_blob_storage_contracts::binding::ReferenceId;
use ic_blob_storage_contracts::binding::ReferenceKey;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use std::num::NonZeroU128;
pub(crate) fn parse(
    execution: UploadContext,
    input: ReadSessionInput,
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
