//! One authenticated read-only assessment; no certificate update or provider effect.
use super::{Failure, arguments::Options, query, read, references::upload_json};
use candid::Principal;
use ic_blob_storage::{
    dto::upload::{
        admission::{UploadAdmissionFailure as A, UploadAdmissionRequest},
        exposure::{UploadExposureBlocker as B, UploadExposureFailure as E},
    },
    ops::service::uploads::certificate::reply::{self, UploadCertificateAssessmentReplyError as R},
    workflow::uploads::certificate::UPLOAD_CERTIFICATE_ASSESSMENT_METHOD,
};
use serde_json::{Value, json};
use std::path::PathBuf;

pub(super) struct Input {
    pub service: Principal,
    pub namespace: u128,
    pub permission: PathBuf,
}

fn open(input: &Input, actor: Principal) -> Result<(UploadAdmissionRequest, Vec<u8>), Failure> {
    let bytes = read(&input.permission, 4096)?;
    let permission: UploadAdmissionRequest =
        crate::native::exact_candid::decode(&bytes, 4096, 64, 100_000)?;
    let argument = reply::assessment_request(permission).map_err(|_| Failure::Arguments)?;
    if permission.upload.service != input.service || permission.upload.namespace != input.namespace
    {
        return Err(Failure::Binding);
    }
    if permission.uploader != actor {
        return Err(Failure::Denied);
    }
    Ok((permission, argument))
}

pub(super) async fn run(options: &Options, input: &Input) -> Result<Value, Failure> {
    let (permission, argument) = open(input, options.actor)?;
    let bytes = query(
        options,
        input.service,
        UPLOAD_CERTIFICATE_ASSESSMENT_METHOD,
        argument,
    )
    .await?;
    output(permission, &bytes, options)
}

fn output(
    permission: UploadAdmissionRequest,
    bytes: &[u8],
    options: &Options,
) -> Result<Value, Failure> {
    let response = reply::assessment(permission, bytes, 4096.try_into().expect("positive bound"))
        .map_err(|error| match error {
        R::Limit => Failure::ReplyLimit,
        R::Invalid => Failure::InvalidReply,
        R::Binding => Failure::Binding,
        R::Remote(error) => Failure::AssessmentRefused(error),
    })?;
    let blockers: Vec<_> = response
        .blockers
        .into_iter()
        .map(|blocker| match blocker {
            B::StaleObservation => "stale_observation",
            B::NamespaceBinding => "namespace_binding",
            B::TrustedUploader => "trusted_uploader",
            B::CurrentOwner => "current_owner",
            B::Durability => "durability",
        })
        .collect();
    Ok(json!({
        "schema":1,"observation":"certificate_assessment","actor":options.actor.to_text(),
        "network":options.network,"url":options.url.as_str(),"authentication":"query_signatures",
        "upload":upload_json(permission.upload),
        "uploader":permission.uploader.to_text(),"expires_at_ns":permission.expires_at_ns.to_string(),
        "assessed_at_ns":response.assessed_at_ns.to_string(),"blockers":blockers,
        "issuance_authorized":false,"retry_authorized":false,
    }))
}

pub(super) const fn refusal_code(error: E) -> &'static str {
    match error {
        E::Unprepared => "unprepared",
        E::Revoked => "assessment_revoked",
        E::Phase => "assessment_phase",
        E::EvidenceBinding => "assessment_evidence_binding",
        E::Permission(error) => match error {
            A::Invalid => "assessment_permission_invalid",
            A::Denied => "assessment_permission_denied",
            A::Binding => "assessment_permission_binding",
            A::Unknown => "assessment_permission_unknown",
            A::Conflict => "assessment_permission_conflict",
            A::Inactive => "assessment_permission_inactive",
            A::Expired => "assessment_permission_expired",
            A::Capacity => "assessment_permission_capacity",
            A::Fenced => "assessment_permission_fenced",
            A::Internal => "assessment_permission_internal",
        },
    }
}

#[cfg(test)]
mod tests;
