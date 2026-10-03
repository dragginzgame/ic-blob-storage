//! Explicit bounded service-wide history; no provider calls, sweep or reconciliation claim.
mod cursor;
use super::{Failure, arguments::Options, query, references::upload_json};
use candid::Principal;
use ic_blob_storage::{
    dto::upload::history::{
        UploadContentState as S, UploadHistoryFailure as F, UploadHistoryFilter,
        UploadHistoryRequest, UploadHistoryScope,
    },
    ops::service::uploads::history::{
        UPLOAD_HISTORY_METHOD,
        reply::{self, UploadHistoryReplyError as R, UploadHistoryReplyLimits},
    },
};
use serde_json::{Value, json};
use std::path::PathBuf;

pub(super) struct Input {
    pub service: Principal,
    pub namespace: u128,
    pub filter: UploadHistoryFilter,
    pub cursor: Option<PathBuf>,
}
pub(super) fn filter(value: &str) -> Result<UploadHistoryFilter, Failure> {
    Ok(match value {
        "all" => UploadHistoryFilter::All,
        "active" => UploadHistoryFilter::Active,
        "deletion-pending" => UploadHistoryFilter::DeletionPending,
        "outstanding" => UploadHistoryFilter::Outstanding,
        _ => return Err(Failure::Arguments),
    })
}
fn filter_text(value: UploadHistoryFilter) -> &'static str {
    match value {
        UploadHistoryFilter::All => "all",
        UploadHistoryFilter::Active => "active",
        UploadHistoryFilter::DeletionPending => "deletion-pending",
        UploadHistoryFilter::Outstanding => "outstanding",
    }
}
fn request(input: &Input) -> Result<UploadHistoryRequest, Failure> {
    let request = UploadHistoryRequest {
        service: input.service,
        namespace: input.namespace,
        scope: UploadHistoryScope::Service,
        filter: input.filter,
        cursor: None,
    };
    let cursor = input
        .cursor
        .as_ref()
        .map(|path| cursor::read(path, request))
        .transpose()?;
    Ok(UploadHistoryRequest { cursor, ..request })
}
pub(super) async fn run(options: &Options, input: &Input) -> Result<Value, Failure> {
    let request = request(input)?;
    let argument = reply::request(request).map_err(failure)?;
    let bytes = query(options, request.service, UPLOAD_HISTORY_METHOD, argument).await?;
    output(options, request, &bytes)
}
fn failure(error: R) -> Failure {
    match error {
        R::Limit => Failure::ReplyLimit,
        R::Invalid => Failure::InvalidReply,
        R::Binding => Failure::Binding,
        R::Remote(error) => match error {
            F::Invalid => Failure::ServiceInvalid,
            F::Denied => Failure::Denied,
            F::Binding => Failure::Binding,
            F::CursorScope => Failure::CursorScope,
            F::Internal => Failure::ServiceInternal,
        },
    }
}
fn output(
    options: &Options,
    request: UploadHistoryRequest,
    bytes: &[u8],
) -> Result<Value, Failure> {
    let page = reply::page(
        request,
        bytes,
        UploadHistoryReplyLimits {
            bytes: (64 * 1024).try_into().expect("positive reply bound"),
            entries: 32.try_into().expect("positive entry bound"),
            scanned: 64.try_into().expect("positive scan bound"),
        },
    )
    .map_err(failure)?;
    let entries: Vec<_> = page
        .entries
        .iter()
        .map(|entry| {
            let state = match entry.state {
                S::Reserved => "reserved",
                S::ExposurePossible => "exposure_possible",
                S::Cancelled => "cancelled",
                S::Live => "live",
                S::DeletionPending => "deletion_pending",
                S::ProviderDeleted => "provider_deleted",
                S::Settled => "settled",
            };
            json!({"upload":upload_json(entry.request),"state":state})
        })
        .collect();
    Ok(
        json!({"schema":1,"observation":"upload_history","operator":options.actor.to_text(),
        "network":options.network,"url":options.url.as_str(),"verification":"query_signatures",
        "scope":{"service":request.service.to_text(),"namespace":request.namespace.to_string()},
        "filter":filter_text(request.filter),"cursor":request.cursor.map(cursor::output),
        "entries":entries,"next":page.next.map(cursor::output),"scanned":page.scanned.to_string(),
        "fenced":page.fenced,"retry_authorized":false,"provider_state":"not_observed"}),
    )
}

#[cfg(test)]
mod tests;
