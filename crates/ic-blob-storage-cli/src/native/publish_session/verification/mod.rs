//! Compose the maintained observer and attestation owner; never infer completion
//! from presentation JSON or create a second dispatch/retry journal.
use super::{Failure, Input, Options, Run};
use crate::native::{
    arguments::Command, identity, observe_upload, publish_inputs::FrozenFile,
    references::upload_json, submit_attestation,
};
use ic_blob_storage::{
    dto::configuration::ServiceInstallationInput, model::identity::ContentDigest,
};
use serde_json::{Value, json};
use std::path::PathBuf;

pub(super) fn options(
    tenant: &Options,
    input: &Input,
    installation: &ServiceInstallationInput,
) -> Result<Option<Options>, Failure> {
    let Some(path) = &input.verifier_identity else {
        return Ok(None);
    };
    identity(path, installation.completion_verifier)?;
    Ok(Some(Options {
        command: Command::SubmitAttestation(submit_attestation::Input {
            service: input.prepare.service,
            namespace: input.prepare.namespace,
            directory: input.prepare.directory.clone(),
        }),
        network: tenant.network,
        url: tenant.url.clone(),
        identity: path.clone(),
        actor: installation.completion_verifier,
        root_key: tenant.root_key.clone(),
    }))
}

pub(super) async fn run(
    verifier: &Options,
    input: &Input,
    selected: &FrozenFile,
    directory: PathBuf,
    source: Option<PathBuf>,
) -> Result<(Value, Value), Failure> {
    let claim = Run::create(&directory)?;
    let permission = selected.input.permission;
    let digest: ContentDigest = format!("sha256:{}", selected.body_sha256)
        .parse()
        .map_err(|_| Failure::Binding)?;
    claim.json(
        "intent.json",
        &json!({"schema":1,"operation":"publish_session_verification",
        "upload":upload_json(permission.upload),"uploader":permission.uploader.to_text(),
        "verifier":verifier.actor.to_text(),"body_sha256":selected.body_sha256,
        "gateway":input.gateway.as_str(),"max_provider_requests":1,
        "max_attestation_updates":1,"retry_authorized":false}),
    )?;
    let encoded = candid::encode_one(permission).map_err(|_| Failure::File)?;
    claim.bytes("permission.candid", &encoded)?;
    if let Some(source) = source {
        claim.json(
            "recovery.json",
            &json!({"source_observation":source,
            "max_provider_requests_this_phase":0,"max_attestation_updates_this_phase":0,
            "redispatch_authorized":false}),
        )?;
        let recovered = submit_attestation::recover_for_upload(
            verifier,
            &submit_attestation::Input {
                service: input.prepare.service,
                namespace: input.prepare.namespace,
                directory: source,
            },
            permission,
            digest,
            &input.gateway,
            &claim,
        )
        .await?;
        claim.json("recovery-summary.json", &recovered)?;
        return Ok((Value::Null, recovered));
    }
    let observation = directory.join("observation");
    let observed = observe_upload::run(
        verifier,
        &observe_upload::Input {
            service: input.prepare.service,
            namespace: input.prepare.namespace,
            permission: directory.join("permission.candid"),
            gateway: input.gateway.clone(),
            directory: observation.clone(),
            max_bytes: permission
                .upload
                .bytes
                .try_into()
                .map_err(|_| Failure::Binding)?,
        },
    )
    .await?;
    let submitted = submit_attestation::run_for_upload(
        verifier,
        &submit_attestation::Input {
            service: input.prepare.service,
            namespace: input.prepare.namespace,
            directory: observation,
        },
        permission,
        digest,
        &input.gateway,
    )
    .await?;
    Ok((observed, submitted))
}
