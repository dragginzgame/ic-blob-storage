//! Persist the exact signed local update before dispatch; never resubmit or poll it.
use super::{Failure, Input, Request, SetupOutcome, observation, permission_json};
use crate::native::{
    arguments::Options,
    signed_update::{MANIFEST_REPLY_BYTES, PreparedUpdate, UpdateInput},
};
use ic_blob_storage_contracts::identity::ContentDigest;
use serde_json::json;

pub(super) async fn run(
    options: &Options,
    input: &Input,
    saved: &Request,
    agent: &ic_platform::agent::Agent,
    directory: &std::path::Path,
) -> Result<SetupOutcome, Failure> {
    let actor = agent.get_principal().map_err(|_| Failure::Identity)?;
    let prepared = PreparedUpdate::claim(
        agent,
        UpdateInput {
            service: input.service,
            method: saved.payload.kind().method(),
            argument: &saved.argument,
            argument_file: "request.wire",
            directory,
            reply_limit: MANIFEST_REPLY_BYTES,
        },
        || {
            let binding = super::record::SetupIntentRecord {
                schema: 1,
                network: options.network.to_owned(),
                service_url: options.url.as_str().to_owned(),
                actor: actor.to_text(),
                service: input.service.to_text(),
                namespace: input.namespace.to_string(),
                permission: permission_json(saved.payload.permission()),
                method: saved.payload.kind().method().to_owned(),
                request_sha256: ContentDigest::compute(&saved.argument).to_string(),
            };
            let mut intent = serde_json::to_value(binding).expect("passive JSON intent fields");
            intent["runner_version"] = env!("CARGO_PKG_VERSION").into();
            intent["runner_source_sha256"] = crate::native::provenance::runner_source().into();
            intent["provider_requests"] = 0.into();
            intent
        },
    )?;
    prepared.run().bytes(
        "permission.wire",
        &ic_blob_storage_contracts::document::encode(saved.payload.permission())
            .map_err(|_| Failure::Arguments)?,
    )?;
    let (observation, report) = prepared.dispatch(agent).await.finish(
        |bytes| observation(saved, bytes),
        |request_id, result| {
            let state = crate::native::signed_update::outcome(result, "acknowledged", |e| {
                matches!(
                    e,
                    Failure::UploadAdmissionRefused(_) | Failure::UploadManifestRefused(_)
                )
            });
            let mut report = json!({"schema":1,"operation":saved.payload.kind().method(),"request_id":request_id,
                "outcome":state,"observation":null,"authentication":null,
                "retry_authorized":false,"certificate_issued":false,"provider_requests":0,"publication_authorized":false});
            match result {
                Ok(Some(observed)) => {
                    report["authentication"] = "ic_update_certificate".into();
                    report["observation"] = observed.json();
                }
                Ok(None) => {}
                Err(e) => {
                    report["error"] = e.code().into();
                    if state == "refused" {
                        report["authentication"] = "ic_update_certificate".into();
                    }
                }
            }
            report
        },
    )?;
    Ok(SetupOutcome {
        observation,
        report,
    })
}
