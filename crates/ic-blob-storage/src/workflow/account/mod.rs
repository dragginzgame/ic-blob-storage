//! One explicit passive provider observation, without local account state mutation.
use crate::ops::caffeine::query::transport::CashierQueryTransport;
use crate::ops::caffeine::query::transport::replicated::ReplicatedQueryError;
use crate::ops::service::account;
use crate::ops::service::account::AccountInspectionAccess;
use crate::ops::service::account::AccountInspectionLimits;
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
    limits: AccountInspectionLimits,
) -> Result<AccountInspectionResponse, AccountInspectionFailure> {
    host.with_account_stores(|stores| account::check(stores, context, input))?;
    let request = account::request(input)?;
    let response = transport.query(&request, limits.max_bytes).await;
    host.with_account_stores(|stores| account::check(stores, context, input))?;
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
