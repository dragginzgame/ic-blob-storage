//! Complete authenticated reference observations; never a serving or publication lease.
use super::{
    Failure, arguments::Options, artifacts::Run, exact_candid, publish_check, publish_inputs,
};
use candid::Principal;
use ic_blob_storage_contracts::download::scope::CaffeineDownloadScope;
use ic_blob_storage_contracts::dto::configuration::HostConfigurationView;
use ic_blob_storage_contracts::dto::configuration::HostFailure;
use ic_blob_storage_contracts::dto::configuration::ServiceInstallationInput;
use ic_blob_storage_contracts::dto::reference::status::ReferenceStatusRequest;
use ic_blob_storage_contracts::dto::upload::completion::UploadAttestationLookup;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::protocol::REFERENCE_STATUS_METHOD;
use ic_blob_storage_contracts::protocol::UPLOAD_ATTESTATION_METHOD;
use ic_blob_storage_contracts::protocol::UPLOAD_CAPACITY_METHOD;
use ic_blob_storage_contracts::provider::download::request_target;
use ic_blob_storage_contracts::reference::reply;
use ic_blob_storage_contracts::upload::completion::CompletionAuthority;
use ic_blob_storage_contracts::upload::completion::reply as completion_reply;
use serde_json::{Value, json};
use std::{future::Future, path::PathBuf, time::Duration};
use url::Url;
#[cfg(test)]
mod tests;

pub(super) struct Input {
    pub checks: publish_check::Input,
    pub gateway: Url,
    pub operator_identity: PathBuf,
    pub selection: Selection,
}

#[derive(Clone, Copy)]
pub(super) enum Selection {
    Batch,
    File(usize),
}

/// The checked result owns completion; JSON is only its boundary presentation.
#[derive(Debug, PartialEq)]
pub(in crate::native) struct PublicationObservation {
    pub complete: bool,
    pub report: Value,
}

impl Selection {
    const fn operation(self) -> &'static str {
        match self {
            Self::Batch => "publish_map",
            Self::File(_) => "publish_file_status",
        }
    }
}

pub(super) async fn run(options: &Options, input: &Input) -> Result<Value, Failure> {
    let limits = &input.checks;
    let batch = publish_inputs::PreparedBatch::open_frozen(
        &limits.inputs,
        limits.max_bytes,
        limits.max_total_bytes,
    )?;
    let scope = publish_check::observation::scope(
        &batch.files,
        limits.service,
        limits.namespace,
        options.actor,
    )?;
    let required_queries = match input.selection {
        Selection::Batch => 2 * batch.files.len() as u64 + 2,
        Selection::File(index) => {
            batch.files.get(index).ok_or(Failure::Arguments)?;
            4
        }
    };
    if limits.max_queries < required_queries {
        return Err(Failure::ReplyLimit);
    }
    let installation: ServiceInstallationInput = exact_candid::decode(
        &batch.installation,
        exact_candid::INSTALLATION_BYTES,
        64,
        100_000,
    )?;
    let agent = super::agent(options)?;
    let operator = super::agent_for(
        options,
        &input.operator_identity,
        installation.configuration.operator,
    )?;
    let run = Run::create(&limits.directory)?;
    run.bytes("inventory.json", &batch.inventory)?;
    run.bytes("installation.candid", &batch.installation)?;
    run.json("intent.json", &json!({"schema":1,"operation":input.selection.operation(),
        "file_index":match input.selection { Selection::Batch => None, Selection::File(index) => Some(index) },
        "service":scope.service.to_text(),"namespace":scope.namespace.to_string(),"tenant":scope.tenant.to_text(),
        "network":options.network,"url":options.url.as_str(),"gateway":input.gateway.as_str(),
        "root_key_sha256":super::upload_inputs::digest(&agent.read_root_key()),
        "inventory_sha256":super::upload_inputs::digest(&batch.inventory),
        "installation_sha256":super::upload_inputs::digest(&batch.installation),
        "max_queries":limits.max_queries,"timeout_seconds":limits.timeout_seconds,
        "updates":0,"provider_requests":0,"automatic_retries":0}))?;
    let result = tokio::time::timeout(
        Duration::from_secs(limits.timeout_seconds),
        inspect(
            &batch,
            options.actor,
            &input.gateway,
            input.selection,
            &run,
            |method, args| {
                let agent = if method == "blob_configuration" {
                    &operator
                } else {
                    &agent
                };
                async move {
                    tokio::time::timeout(
                        Duration::from_secs(30),
                        agent.query(&scope.service, method).with_arg(args).call(),
                    )
                    .await
                    .map_err(|_| Failure::Timeout)?
                    .map_err(|_| Failure::Transport)
                }
            },
        ),
    )
    .await
    .map_err(|_| Failure::Timeout)
    .and_then(|v| v);
    match result {
        Ok(observation) => {
            if matches!(input.selection, Selection::Batch) && observation.complete {
                run.json("media-map.json", &observation.report)?;
            }
            run.json("summary.json", &observation.report)?;
            Ok(observation.report)
        }
        Err(error) => {
            run.json("failure.json", &json!({"error":error.code(),"complete":false,"map_written":false,"retry_authorized":false}))?;
            Err(error)
        }
    }
}

pub(in crate::native) async fn inspect<Q, F>(
    batch: &publish_inputs::PreparedBatch,
    actor: Principal,
    gateway: &Url,
    selection: Selection,
    run: &Run,
    mut query: Q,
) -> Result<PublicationObservation, Failure>
where
    Q: FnMut(&'static str, Vec<u8>) -> F,
    F: Future<Output = Result<Vec<u8>, Failure>>,
{
    let first = batch
        .files
        .first()
        .ok_or(Failure::Arguments)?
        .input
        .permission
        .upload;
    let scope =
        publish_check::observation::scope(&batch.files, first.service, first.namespace, actor)?;
    let host = configuration(batch, run, &mut query).await?;
    let authority = CompletionAuthority::new(
        first.service,
        first.namespace.try_into().map_err(|_| Failure::Binding)?,
        host.completion_verifier,
    )
    .map_err(|_| Failure::Binding)?;
    let capacity = publish_check::observation::capacity(
        scope,
        &publish_check::observation::observe(
            run,
            1,
            UPLOAD_CAPACITY_METHOD,
            candid::encode_one(scope).map_err(|_| Failure::Arguments)?,
            &mut query,
        )
        .await?,
    )?;
    let mut blockers = Vec::new();
    if capacity.fenced || host.fenced {
        blockers.push(json!({"code":"fenced"}));
    }
    if !capacity.enrollment.active {
        blockers.push(json!({"code":"tenant_inactive"}));
    }
    let selected = match selection {
        Selection::Batch => 0..batch.files.len(),
        Selection::File(index) => {
            batch.files.get(index).ok_or(Failure::Arguments)?;
            index..index + 1
        }
    };
    let expected_files = selected.len();
    let mut files = Vec::with_capacity(expected_files);
    if blockers.is_empty() {
        for index in selected {
            let file = &batch.files[index].input;
            let digest = &batch.files[index].body_sha256;
            let blocked = match completion(file, index, authority, digest, run, &mut query).await? {
                Some(code) => Some(code),
                None => reference(file, index, run, &mut query).await?,
            };
            if let Some(code) = blocked {
                blockers.push(json!({"index":index,"code":code}));
                break;
            }
            files.push(mapping(file, index, digest, gateway)?);
        }
    }
    let complete = blockers.is_empty() && files.len() == expected_files;
    let mut report = json!({"schema":1,"operation":selection.operation(),"authentication":"query_signatures",
        "inventory_sha256":super::upload_inputs::digest(&batch.inventory),
        "installation_sha256":super::upload_inputs::digest(&batch.installation),
        "files":files,"blockers":blockers,"atomic_snapshot":false,"publication_lease":false,
        "provider_requests":0,"public_serving_qualified":false,"retry_authorized":false});
    match selection {
        Selection::Batch => report["all_references_live"] = json!(complete),
        Selection::File(index) => {
            report["file_index"] = json!(index);
            report["file_live"] = json!(complete);
            report["batch_complete"] = json!(false);
        }
    }
    Ok(PublicationObservation { complete, report })
}

pub(in crate::native) async fn configuration<Q, F>(
    batch: &publish_inputs::PreparedBatch,
    run: &Run,
    query: &mut Q,
) -> Result<HostConfigurationView, Failure>
where
    Q: FnMut(&'static str, Vec<u8>) -> F,
    F: Future<Output = Result<Vec<u8>, Failure>>,
{
    let bytes = publish_check::observation::observe(
        run,
        0,
        "blob_configuration",
        candid::encode_args(()).map_err(|_| Failure::Arguments)?,
        query,
    )
    .await?;
    decode_configuration(batch, &bytes)
}

/// Shared immutable installation/release check for fresh and retained host replies.
pub(in crate::native) fn decode_configuration(
    batch: &publish_inputs::PreparedBatch,
    bytes: &[u8],
) -> Result<HostConfigurationView, Failure> {
    let installation: ServiceInstallationInput = exact_candid::decode(
        &batch.installation,
        exact_candid::INSTALLATION_BYTES,
        64,
        100_000,
    )?;
    if bytes.len() > exact_candid::INSTALLATION_BYTES {
        return Err(Failure::ReplyLimit);
    }
    let host: Result<HostConfigurationView, HostFailure> =
        exact_candid::decode(bytes, exact_candid::INSTALLATION_BYTES, 64, 100_000)
            .map_err(|_| Failure::InvalidReply)?;
    let host = host.map_err(|_| Failure::Denied)?;
    if host.configuration != installation.configuration
        || host.project != installation.project
        || host.completion_verifier != installation.completion_verifier
        || host.trusted_uploader != installation.trusted_uploader
        || host.release != ic_blob_storage_contracts::CONTRACT_VERSION
    {
        return Err(Failure::Binding);
    }
    Ok(host)
}

async fn completion<Q, F>(
    file: &super::upload_inputs::PreparedInput,
    index: usize,
    authority: CompletionAuthority,
    digest: &str,
    run: &Run,
    query: &mut Q,
) -> Result<Option<&'static str>, Failure>
where
    Q: FnMut(&'static str, Vec<u8>) -> F,
    F: Future<Output = Result<Vec<u8>, Failure>>,
{
    let args = completion_reply::inspection_request(authority, file.permission)
        .map_err(super::attestation::failure)?;
    let bytes = publish_check::observation::observe(
        run,
        2 * index + 2,
        UPLOAD_ATTESTATION_METHOD,
        args,
        query,
    )
    .await?;
    let confirmation = match completion_reply::inspection(
        authority,
        file.permission,
        &bytes,
        4096.try_into().expect("positive reply bound"),
    ) {
        Ok(confirmation) => confirmation,
        Err(completion_reply::UploadAttestationReplyError::Remote(
            ic_blob_storage_contracts::dto::upload::completion::UploadAttestationFailure::Permission(
                ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure::Unknown,
            ),
        )) => return Ok(Some("completion_unmatched")),
        Err(error) => return Err(super::attestation::failure(error)),
    };
    if confirmation.fenced {
        return Ok(Some("fenced"));
    }
    let digest: ic_blob_storage_contracts::identity::ContentDigest = format!("sha256:{digest}")
        .parse()
        .map_err(|_| Failure::Binding)?;
    match confirmation.attestation {
        UploadAttestationLookup::Found(receipt)
            if receipt.request.content_digest == *digest.as_bytes() =>
        {
            Ok(None)
        }
        _ => Ok(Some("completion_unmatched")),
    }
}

async fn reference<Q, F>(
    file: &super::upload_inputs::PreparedInput,
    index: usize,
    run: &Run,
    query: &mut Q,
) -> Result<Option<&'static str>, Failure>
where
    Q: FnMut(&'static str, Vec<u8>) -> F,
    F: Future<Output = Result<Vec<u8>, Failure>>,
{
    let request = ReferenceStatusRequest {
        upload: file.permission.upload,
        reference: file.permission.upload.first_reference,
    };
    let args = reply::status_request(request).map_err(super::references::failure)?;
    let bytes = publish_check::observation::observe(
        run,
        2 * index + 3,
        REFERENCE_STATUS_METHOD,
        args,
        query,
    )
    .await?;
    let status = reply::decode_status(
        request,
        &bytes,
        4096.try_into().expect("positive reply bound"),
    )
    .map_err(super::references::failure)?;
    Ok(if status.fenced {
        Some("fenced")
    } else if !status.live {
        Some("reference_inactive")
    } else {
        None
    })
}

fn mapping(
    file: &super::upload_inputs::PreparedInput,
    index: usize,
    digest: &str,
    gateway: &Url,
) -> Result<Value, Failure> {
    let u = file.permission.upload;
    let serving = CaffeineDownloadScope::new(
        u.service,
        u.namespace.try_into().map_err(|_| Failure::Binding)?,
        file.project(),
    )
    .map_err(|_| Failure::Binding)?;
    let root = ProviderRootHash::try_from(u.root.as_slice()).map_err(|_| Failure::Binding)?;
    let url = gateway
        .join(&request_target(&serving, root))
        .map_err(|_| Failure::Arguments)?;
    Ok(
        json!({"index":index,"upload":super::references::upload_json(u),
        "reference":u.first_reference.to_string(),"project":file.project(),"url":url.as_str(),
        "body_sha256":digest,"headers":file.declaration().headers.iter()
            .map(|h|json!({"name":h.name,"value":h.value})).collect::<Vec<_>>()}),
    )
}
