//! Explicit tenant queries keep historical results separate from current liveness.
use super::{Failure, arguments::Options, query, read};
use candid::Principal;
use ic_blob_storage::{
    dto::reference::{
        ReferenceAction, ReferenceChange, ReferenceCommand, ReferenceFailure,
        ReferenceReceiptLookup, ReferenceTransitionFailure, ReferenceUpload,
        status::ReferenceStatusRequest,
    },
    model::identity::ProviderRootHash,
    ops::service::references::{REFERENCE_RECEIPT_METHOD, reply, status::REFERENCE_STATUS_METHOD},
};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

pub(super) enum Kind {
    Receipt,
    Status,
}
pub(super) struct Input {
    pub service: Principal,
    pub namespace: u128,
    pub request: PathBuf,
    pub kind: Kind,
}
enum Request {
    Receipt(ReferenceCommand),
    Status(ReferenceStatusRequest),
}
struct Inspection {
    request: Request,
    argument: Vec<u8>,
}

fn open(input: &Input, actor: Principal) -> Result<Inspection, Failure> {
    let request: ReferenceStatusRequest = match input.kind {
        Kind::Receipt => {
            let (request, argument) =
                command(input.service, input.namespace, &input.request, actor)?;
            return Ok(Inspection {
                request: Request::Receipt(request),
                argument,
            });
        }
        Kind::Status => {
            crate::native::exact_candid::decode(&read(&input.request, 4096)?, 4096, 64, 100_000)?
        }
    };
    let argument = reply::status_request(request).map_err(|_| Failure::Arguments)?;
    let upload = request.upload;
    if upload.service != input.service || upload.namespace != input.namespace {
        return Err(Failure::Binding);
    }
    if upload.tenant != actor {
        return Err(Failure::Denied);
    }
    Ok(Inspection {
        request: Request::Status(request),
        argument,
    })
}
pub(super) fn command(
    service: Principal,
    namespace: u128,
    path: &Path,
    actor: Principal,
) -> Result<(ReferenceCommand, Vec<u8>), Failure> {
    let request: ReferenceCommand =
        crate::native::exact_candid::decode(&read(path, 4096)?, 4096, 64, 100_000)?;
    let argument = reply::receipt_request(request).map_err(|_| Failure::Arguments)?;
    if request.upload.service != service || request.upload.namespace != namespace {
        return Err(Failure::Binding);
    }
    if request.upload.tenant != actor {
        return Err(Failure::Denied);
    }
    Ok((request, argument))
}
pub(super) async fn run(options: &Options, input: &Input) -> Result<Value, Failure> {
    let inspection = open(input, options.actor)?;
    let method = match inspection.request {
        Request::Receipt(_) => REFERENCE_RECEIPT_METHOD,
        Request::Status(_) => REFERENCE_STATUS_METHOD,
    };
    let bytes = query(options, input.service, method, inspection.argument).await?;
    output(&inspection.request, &bytes, options)
}
pub(super) fn failure(error: reply::ReferenceReplyError) -> Failure {
    match error {
        reply::ReferenceReplyError::Binding => Failure::Binding,
        reply::ReferenceReplyError::Limit => Failure::ReplyLimit,
        reply::ReferenceReplyError::Invalid => Failure::InvalidReply,
        reply::ReferenceReplyError::Remote(error) => Failure::ReferenceRefused(error),
    }
}
pub(super) const fn refusal_code(error: ReferenceFailure) -> &'static str {
    match error {
        ReferenceFailure::Invalid => "reference_invalid",
        ReferenceFailure::Denied => "reference_denied",
        ReferenceFailure::Binding => "reference_binding",
        ReferenceFailure::Unknown => "reference_unknown",
        ReferenceFailure::Unconfirmed => "reference_unconfirmed",
        ReferenceFailure::Conflict => "reference_conflict",
        ReferenceFailure::Inactive => "reference_inactive",
        ReferenceFailure::Fenced => "reference_fenced",
        ReferenceFailure::Capacity => "reference_capacity",
        ReferenceFailure::Internal => "reference_internal",
    }
}
pub(super) fn upload_json(u: ReferenceUpload) -> Value {
    json!({"service":u.service.to_text(),"tenant":u.tenant.to_text(),"namespace":u.namespace.to_string(),
        "upload":u.upload.to_string(),"object":u.object.to_string(),"incarnation":u.incarnation.to_string(),
        "first_reference":u.first_reference.to_string(),"root":ProviderRootHash::try_from(u.root.as_slice()).expect("fixed root").to_string(),"bytes":u.bytes.to_string()})
}
pub(super) fn result_json(result: Result<ReferenceChange, ReferenceTransitionFailure>) -> Value {
    match result {
        Ok(change) => json!({"state":"success","change":match change {
            ReferenceChange::Changed => "changed", ReferenceChange::Unchanged => "unchanged",
        }}),
        Err(error) => json!({"state":"failure","failure":match error {
            ReferenceTransitionFailure::UnknownReference => "unknown_reference",
            ReferenceTransitionFailure::Released => "released",
            ReferenceTransitionFailure::Limit => "limit",
            ReferenceTransitionFailure::DeletionQueued => "deletion_queued",
        }}),
    }
}
fn output(request: &Request, bytes: &[u8], options: &Options) -> Result<Value, Failure> {
    let max = 4096.try_into().expect("positive reply limit");
    let mut value = json!({"schema":1,"actor":options.actor.to_text(),"network":options.network,
        "url":options.url.as_str(),"verification":"query_signatures","retry_authorized":false,
        "publication_authorized":false,"provider_availability":"not_observed"});
    match *request {
        Request::Receipt(request) => {
            let lookup = reply::decode(request, bytes, max).map_err(failure)?;
            let result = match lookup {
                ReferenceReceiptLookup::Absent => None,
                ReferenceReceiptLookup::Found(receipt) => Some(result_json(receipt.result)),
            };
            value["observation"] = "reference_receipt".into();
            value["upload"] = upload_json(request.upload);
            value["reference"] = request.reference.to_string().into();
            value["operation"] = request.operation.to_string().into();
            value["action"] = match request.action {
                ReferenceAction::Retain => "retain",
                ReferenceAction::Release => "release",
            }
            .into();
            value["outcome"] = if result.is_some() { "found" } else { "absent" }.into();
            value["result"] = result.unwrap_or(Value::Null);
            value["reference_liveness"] = "not_observed".into();
            value["fence"] = "not_observed".into();
        }
        Request::Status(request) => {
            let response = reply::decode_status(request, bytes, max).map_err(failure)?;
            value["observation"] = "reference_status".into();
            value["upload"] = upload_json(request.upload);
            value["reference"] = request.reference.to_string().into();
            value["live"] = response.live.into();
            value["fenced"] = response.fenced.into();
        }
    }
    Ok(value)
}

#[cfg(test)]
mod tests;
