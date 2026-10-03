//! Exact local upload reservation/preparation; no certificate or provider dispatch.
mod dispatch;
mod request;
#[cfg(test)]
mod tests;
use super::{Failure, arguments::Options, query, references};
use candid::Principal;
use ic_blob_storage::{
    dto::upload::{
        UploadState,
        admission::{UploadAdmissionFailure as A, UploadAdmissionRequest, UploadAdmissionResponse},
        manifest::{
            UploadManifestDeclaration, UploadManifestFailure as M, UploadManifestInspection,
            UploadManifestRequest,
        },
    },
    ops::service::uploads::{
        admission::{self, reply as admission_reply},
        manifests::{self, reply as manifest_reply},
    },
};
use serde_json::{Value, json};
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Kind {
    Admit,
    Prepare,
    Revoke,
    Permission,
    Manifest,
}
impl Kind {
    pub fn parse(command: &str) -> Option<Self> {
        Some(match command {
            "admit-upload" => Self::Admit,
            "prepare-upload" => Self::Prepare,
            "revoke-upload" => Self::Revoke,
            "upload-permission" => Self::Permission,
            "upload-manifest" => Self::Manifest,
            _ => return None,
        })
    }
    pub const fn mutation(self) -> bool {
        matches!(self, Self::Admit | Self::Prepare | Self::Revoke)
    }
    pub const fn method(self) -> &'static str {
        match self {
            Self::Admit => admission::UPLOAD_ADMISSION_METHOD,
            Self::Revoke => admission::UPLOAD_REVOCATION_METHOD,
            Self::Permission => admission::UPLOAD_ADMISSION_STATUS_METHOD,
            Self::Prepare => manifests::UPLOAD_MANIFEST_PREPARE_METHOD,
            Self::Manifest => manifests::UPLOAD_MANIFEST_INSPECT_METHOD,
        }
    }
}
#[derive(Clone)]
pub(super) struct Input {
    pub kind: Kind,
    pub service: Principal,
    pub namespace: u128,
    pub request: PathBuf,
    pub directory: Option<PathBuf>,
}
pub(super) struct Request {
    permission: UploadAdmissionRequest,
    manifest: Option<UploadManifestRequest>,
    argument: Vec<u8>,
}
pub(super) fn permission_json(p: UploadAdmissionRequest) -> Value {
    json!({"upload":references::upload_json(p.upload),"uploader":p.uploader.to_text(),"expires_at_ns":p.expires_at_ns.to_string()})
}
fn admission_json(response: UploadAdmissionResponse) -> Value {
    let state = match response.state {
        UploadState::Reserved => "reserved",
        UploadState::ExposurePossible => "exposure_possible",
        UploadState::Confirmed => "confirmed",
        UploadState::Cancelled => "cancelled",
    };
    json!({"permission":permission_json(response.permission),"state":state,"revoked":response.revoked})
}
pub(super) fn declaration_json(d: &UploadManifestDeclaration) -> Value {
    json!({"chunks":d.chunks.iter().map(|h| ic_blob_storage::model::identity::caffeine::manifest::CaffeineChunkHash::try_from(h.as_slice()).expect("fixed hash").to_string()).collect::<Vec<_>>(),"headers":d.headers.iter().map(|h|json!({"name":h.name,"value":h.value})).collect::<Vec<_>>()})
}
fn observation(input: &Input, saved: &Request, bytes: &[u8]) -> Result<Value, Failure> {
    let max = 4096.try_into().unwrap();
    Ok(match input.kind {
        Kind::Admit => {
            let r =
                admission_reply::mutation(saved.permission, bytes, max).map_err(admission_error)?;
            json!({"admission":admission_json(r.admission),"replayed":r.replayed})
        }
        Kind::Revoke => {
            let r = admission_reply::revocation(saved.permission, bytes, max)
                .map_err(admission_error)?;
            json!({"admission":admission_json(r.admission),"changed":r.changed})
        }
        Kind::Permission => admission_json(
            admission_reply::inspection(saved.permission, bytes, max).map_err(admission_error)?,
        ),
        Kind::Prepare | Kind::Manifest => {
            let (r, changed) = if let Some(manifest) = &saved.manifest {
                let r = manifest_reply::mutation(manifest, bytes, request::limits())
                    .map_err(manifest_error)?;
                (r.observation, Some(r.changed))
            } else {
                (
                    manifest_reply::inspection(saved.permission, bytes, request::limits())
                        .map_err(manifest_error)?,
                    None,
                )
            };
            let declaration = match r.manifest {
                UploadManifestInspection::Unprepared => Value::Null,
                UploadManifestInspection::Prepared(d) => declaration_json(&d),
            };
            json!({"permission":permission_json(r.permission),"manifest":declaration,"changed":changed})
        }
    })
}
pub(super) async fn run(options: &Options, input: &Input) -> Result<Value, Failure> {
    let saved = request::load(input, options.actor)?;
    if input.kind.mutation() {
        return dispatch::run(options, input, &saved).await;
    }
    let bytes = query(
        options,
        input.service,
        input.kind.method(),
        saved.argument.clone(),
    )
    .await?;
    Ok(
        json!({"schema":1,"operation":input.kind.method(),"authentication":"query_signatures",
        "observation":observation(input, &saved, &bytes)?,"retry_authorized":false,
        "certificate_issued":false,"provider_requests":0,"publication_authorized":false}),
    )
}

/// Retain exact query arguments and raw replies through the same setup decoder.
pub(super) async fn inspect_recorded(
    options: &Options,
    input: &Input,
    run: &crate::native::artifacts::Run,
    label: &str,
) -> Result<Value, Failure> {
    if input.kind.mutation() {
        return Err(Failure::Arguments);
    }
    let saved = request::load(input, options.actor)?;
    run.bytes(&format!("{label}-args.candid"), &saved.argument)?;
    run.json(
        &format!("{label}-intent.json"),
        &json!({"method":input.kind.method(),
        "argument_sha256":crate::native::upload_inputs::digest(&saved.argument),"updates":0}),
    )?;
    let bytes = query(
        options,
        input.service,
        input.kind.method(),
        saved.argument.clone(),
    )
    .await?;
    run.bytes(&format!("{label}-reply.candid"), &bytes)?;
    observation(input, &saved, &bytes)
}
fn admission_error(error: admission_reply::UploadAdmissionReplyError) -> Failure {
    use admission_reply::UploadAdmissionReplyError as R;
    match error {
        R::Limit => Failure::ReplyLimit,
        R::Invalid => Failure::InvalidReply,
        R::Binding => Failure::Binding,
        R::Remote(e) => Failure::UploadAdmissionRefused(e),
    }
}
fn manifest_error(error: manifest_reply::UploadManifestReplyError) -> Failure {
    use manifest_reply::UploadManifestReplyError as R;
    match error {
        R::Limit => Failure::ReplyLimit,
        R::Invalid => Failure::InvalidReply,
        R::Binding => Failure::Binding,
        R::Remote(e) => Failure::UploadManifestRefused(e),
    }
}
pub(super) const fn admission_code(error: A) -> &'static str {
    match error {
        A::Invalid => "upload_invalid",
        A::Denied => "upload_denied",
        A::Binding => "upload_binding",
        A::Unknown => "upload_unknown",
        A::Conflict => "upload_conflict",
        A::Inactive => "upload_inactive",
        A::Expired => "upload_expired",
        A::Capacity => "upload_capacity",
        A::Fenced => "upload_fenced",
        A::Internal => "upload_internal",
    }
}
pub(super) const fn manifest_code(error: M) -> &'static str {
    match error {
        M::Permission(e) => admission_code(e),
        M::Revoked => "upload_revoked",
        M::Phase => "upload_phase",
        M::Declaration => "upload_declaration",
        M::Limit => "upload_limit",
    }
}
