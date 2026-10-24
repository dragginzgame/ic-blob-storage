//! One explicit passive provider observation, without local account state mutation.
use crate::ops::caffeine::decoding::CaffeineReplyLimits;
use crate::ops::caffeine::query::transport::CashierQueryTransport;
use crate::ops::caffeine::query::transport::replicated::ReplicatedQueryError;
use crate::ops::service::account;
use crate::ops::service::account::AccountInspectionAccess;
use crate::ops::service::operator::OperatorStores;
use ic_memory::Memory;

use ic_blob_storage_contracts::dto::account::AccountInspectionFailure;
use ic_blob_storage_contracts::dto::account::AccountInspectionRequest;
use ic_blob_storage_contracts::dto::account::AccountInspectionResponse;
use ic_blob_storage_contracts::upload::binding::UploadContext;

/// Authenticate the installed scope, send one canonical bounded read, then recheck
/// the same owners before returning. Hosts keep configuration immutable across await.
/// Neither success nor failure updates credit, allocation, membership or readiness.
/// Results are independent observations, never cached funding or retry permits.
/// # Errors
/// Rejects authority, scope, any restore fence, transport and invalid replies.
pub async fn inspect<
    H: AccountInspectionAccess,
    T: CashierQueryTransport<Error = ReplicatedQueryError>,
>(
    host: &H,
    transport: &T,
    context: UploadContext,
    input: AccountInspectionRequest,
    limits: CaffeineReplyLimits,
) -> Result<AccountInspectionResponse, AccountInspectionFailure> {
    host.with_account_stores(|stores| check(stores, context, input))?;
    let request = account::request(input)?;
    let response = transport.query(&request, limits.max_bytes).await;
    host.with_account_stores(|stores| check(stores, context, input))?;
    let observation = account::decode(
        &request,
        &response.map_err(account::transport_error)?,
        limits,
    )?;
    Ok(AccountInspectionResponse {
        request: input,
        observation,
    })
}

fn check<M: Memory>(
    stores: OperatorStores<'_, M>,
    context: UploadContext,
    input: AccountInspectionRequest,
) -> Result<(), AccountInspectionFailure> {
    use ic_blob_storage_contracts::dto::operator::LocalStatusFailure;
    let status =
        crate::workflow::operator::inspect(stores, context, input.scope).map_err(|e| match e {
            LocalStatusFailure::Denied => AccountInspectionFailure::Denied,
            LocalStatusFailure::Binding => AccountInspectionFailure::Binding,
            LocalStatusFailure::Internal => AccountInspectionFailure::Internal,
        })?;
    if status.uploads.fenced
        || status.funding.fenced
        || status.gateways.fenced
        || status.reads.fenced
    {
        return Err(AccountInspectionFailure::Fenced);
    }
    Ok(())
}
