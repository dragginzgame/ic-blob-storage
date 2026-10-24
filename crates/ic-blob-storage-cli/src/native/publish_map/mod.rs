//! Complete authenticated reference observations; never a serving or publication lease.
use super::{
    Failure, arguments::Options, artifacts::Run, exact_wire, publish_check, publish_inputs,
};
use ic_blob_storage_contracts::download::scope::CaffeineDownloadScope;
use ic_blob_storage_contracts::dto::configuration::HostConfigurationView;
use ic_blob_storage_contracts::dto::configuration::HostFailure;
use ic_blob_storage_contracts::dto::reference::status::ReferenceStatusRequest;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::protocol;
use ic_blob_storage_contracts::protocol::REFERENCE_STATUS_METHOD;
use ic_blob_storage_contracts::protocol::UPLOAD_CAPACITY_METHOD;
use ic_blob_storage_contracts::provider::download::request_target;
use ic_blob_storage_contracts::reference::reply;
use ic_platform::Principal;
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

/// Publication reads authenticate configuration as operator and other reads as tenant.
/// Each call has its own deadline; the caller retains the overall query budget.
pub(super) async fn query(
    tenant: &ic_platform::agent::Agent,
    operator: &ic_platform::agent::Agent,
    service: Principal,
    method: &'static str,
    args: Vec<u8>,
) -> Result<Vec<u8>, Failure> {
    let agent = if method == protocol::BLOB_CONFIGURATION_METHOD {
        operator
    } else {
        tenant
    };
    super::query::with_agent(agent, service, method, args).await
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
        Selection::Batch => batch.files.len() as u64 + 2,
        Selection::File(index) => {
            batch.files.get(index).ok_or(Failure::Arguments)?;
            3
        }
    };
    if limits.max_queries < required_queries {
        return Err(Failure::ReplyLimit);
    }
    let installation = batch.installation();
    let agent = super::agent(options)?;
    let operator = super::agent_for(
        options,
        &input.operator_identity,
        installation.configuration.operator,
    )?;
    let run = Run::create(&limits.directory)?;
    run.bytes("inventory.json", &batch.inventory)?;
    run.bytes("installation.wire", &batch.installation.bytes)?;
    run.json("intent.json", &json!({"schema":1,"operation":input.selection.operation(),
        "file_index":match input.selection { Selection::Batch => None, Selection::File(index) => Some(index) },
        "service":scope.service.to_text(),"namespace":scope.namespace.to_string(),"tenant":scope.tenant.to_text(),
        "network":options.network,"url":options.url.as_str(),"gateway":input.gateway.as_str(),
        "root_key_sha256":super::upload_inputs::digest(&agent.read_root_key()),
        "inventory_sha256":super::upload_inputs::digest(&batch.inventory),
        "installation_sha256":super::upload_inputs::digest(&batch.installation.bytes),
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
            |method, args| query(&agent, &operator, scope.service, method, args),
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
    let capacity = publish_check::observation::capacity(
        scope,
        &super::query::observe(
            run,
            1,
            UPLOAD_CAPACITY_METHOD,
            ic_blob_storage_contracts::wire::encode_request(UPLOAD_CAPACITY_METHOD, scope)
                .map_err(|_| Failure::Arguments)?,
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
            let blocked = reference(file, index, run, &mut query).await?;
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
        "installation_sha256":super::upload_inputs::digest(&batch.installation.bytes),
        "files":files,"blockers":blockers,"atomic_snapshot":false,"publication_lease":false,
        "provider_requests":0,"provider_upload_verified":false,"public_serving_qualified":false,"retry_authorized":false});
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
    let bytes = super::query::observe(
        run,
        0,
        protocol::BLOB_CONFIGURATION_METHOD,
        ic_blob_storage_contracts::wire::encode_request(protocol::BLOB_CONFIGURATION_METHOD, ())
            .map_err(|_| Failure::Arguments)?,
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
    let installation = batch.installation();
    if bytes.len() > exact_wire::INSTALLATION_BYTES {
        return Err(Failure::ReplyLimit);
    }
    let host: Result<HostConfigurationView, HostFailure> =
        ic_blob_storage_contracts::wire::decode_response(
            protocol::BLOB_CONFIGURATION_METHOD,
            bytes,
            exact_wire::INSTALLATION_BYTES,
        )
        .map_err(|_| Failure::InvalidReply)?;
    let host = host.map_err(|_| Failure::Denied)?;
    if host.configuration != installation.configuration
        || host.project != installation.project
        || host.release != ic_blob_storage_contracts::CONTRACT_VERSION
    {
        return Err(Failure::Binding);
    }
    Ok(host)
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
    let bytes = super::query::observe(run, index + 2, REFERENCE_STATUS_METHOD, args, query).await?;
    let status = match reply::decode_status(
        request,
        &bytes,
        4096.try_into().expect("positive reply bound"),
    ) {
        Ok(status) => status,
        Err(reply::ReferenceReplyError::Remote(
            ic_blob_storage_contracts::dto::reference::ReferenceFailure::Unknown
            | ic_blob_storage_contracts::dto::reference::ReferenceFailure::Unconfirmed,
        )) => return Ok(Some("registration_unmatched")),
        Err(error) => return Err(super::references::failure(error)),
    };
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
    digest: &ic_host_artifacts::artifact::Sha256Digest,
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
        "body_sha256":digest.to_string(),"headers":file.declaration().headers.iter()
            .map(|h|json!({"name":h.name,"value":h.value})).collect::<Vec<_>>()}),
    )
}
