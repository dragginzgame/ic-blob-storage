//! Bounded saved continuation bound to the complete original operator scan.
use super::{Failure, filter, filter_text};
use candid::Principal;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryCursor;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryRequest;
use serde::Deserialize;
use serde_json::{Value, json};
use std::path::Path;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CursorInput {
    service: String,
    namespace: String,
    scope: String,
    filter: String,
    after_tenant: String,
    after_request: String,
}
fn decimal(value: &str) -> Result<u128, Failure> {
    let number: u128 = value.parse().map_err(|_| Failure::Arguments)?;
    if number == 0 || number.to_string() != value {
        return Err(Failure::Arguments);
    }
    Ok(number)
}
pub(super) fn read(
    path: &Path,
    request: UploadHistoryRequest,
) -> Result<UploadHistoryCursor, Failure> {
    let input: CursorInput =
        serde_json::from_slice(&super::super::read(path, 2048)?).map_err(|_| Failure::Arguments)?;
    if input.service != request.service.to_text()
        || decimal(&input.namespace)? != request.namespace
        || input.scope != "service"
        || filter(&input.filter)? != request.filter
    {
        return Err(Failure::CursorScope);
    }
    let tenant = Principal::from_text(&input.after_tenant).map_err(|_| Failure::Arguments)?;
    if tenant.to_text() != input.after_tenant
        || [Principal::anonymous(), Principal::management_canister()].contains(&tenant)
    {
        return Err(Failure::Arguments);
    }
    Ok(UploadHistoryCursor {
        service: request.service,
        namespace: request.namespace,
        scope: request.scope,
        filter: request.filter,
        after_tenant: tenant,
        after_request: decimal(&input.after_request)?,
    })
}
pub(super) fn output(c: UploadHistoryCursor) -> Value {
    json!({"service":c.service.to_text(),"namespace":c.namespace.to_string(),"scope":"service",
        "filter":filter_text(c.filter),"after_tenant":c.after_tenant.to_text(),"after_request":c.after_request.to_string()})
}
