use ic_blob_storage_contracts::{protocol, wire};
mod account;
mod account_link_inputs;
mod arguments;
mod artifacts;
mod certificate_assessment;
mod download;
mod exact_wire;
mod funding_assessment;
mod funding_outcome;
mod gateway_controls;
mod history;
use crate::http;
mod installation_check;
mod local_body;
mod metrics;
mod parsing;
mod provenance;
mod provider_download;
mod publish_check;
mod publish_inputs;
mod publish_map;
mod publish_prepare;
mod publish_session;
mod query;
mod reference_inputs;
mod references;
mod reply;
mod signed_update;
mod submit_reference;
use ic_blob_storage_contracts::protocol::LOCAL_STATUS_METHOD;
use ic_platform::Principal;
use ic_platform::agent::{Agent, BasicIdentity, Identity, Secp256k1Identity};
use serde_json::json;
use std::{path::Path, process::ExitCode, time::Duration};

#[cfg(test)]
mod tests;
mod upload_history;
mod upload_inputs;
mod upload_setup;
mod verify_upload;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Failure {
    PreparedManifest,
    UploadAdmissionRefused(
        ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure,
    ),
    UploadManifestRefused(ic_blob_storage_contracts::dto::upload::manifest::UploadManifestFailure),
    Arguments,
    File,
    Identity,
    IdentityBinding,
    Transport,
    Timeout,
    ReplyTooLarge,
    InvalidReply,
    Binding,
    Denied,
    NotEnrolled,
    ServiceInternal,
    ServiceInvalid,
    FundingConflict,
    CursorScope,
    ReplyLimit,
    Content,
    ManifestRefused,
    Unprepared,
    ExistingRun,
    ProviderResponse,
    SubmissionClaimed,
    AssessmentRefused(ic_blob_storage_contracts::dto::upload::exposure::UploadExposureFailure),
    ReferenceRefused(ic_blob_storage_contracts::dto::reference::ReferenceFailure),
    AccountRefused(ic_blob_storage_contracts::dto::account::AccountInspectionFailure),
    FundingAssessmentRefused(
        ic_blob_storage_contracts::dto::funding::assessment::FundingPreparationFailure,
    ),
    GatewaySyncRefused(ic_blob_storage_contracts::dto::gateway::sync::GatewaySyncFailure),
    GatewayRevocationRefused(ic_blob_storage_contracts::dto::gateway::GatewayRevocationFailure),
    DownloadRefused(ic_blob_storage_contracts::dto::download::DownloadFailure),
}
impl Failure {
    const fn code(self) -> &'static str {
        match self {
            Self::PreparedManifest => "prepared_manifest",
            Self::UploadAdmissionRefused(e) => upload_setup::admission_code(e),
            Self::UploadManifestRefused(e) => upload_setup::manifest_code(e),
            Self::Arguments => "arguments",
            Self::File => "file",
            Self::Identity => "identity",
            Self::IdentityBinding => "identity_binding",
            Self::Transport => "transport",
            Self::Timeout => "timeout",
            Self::ReplyTooLarge => "reply_too_large",
            Self::InvalidReply => "invalid_reply",
            Self::Binding => "binding",
            Self::Denied => "denied",
            Self::NotEnrolled => "not_enrolled",
            Self::ServiceInternal => "service_internal",
            Self::ServiceInvalid => "service_invalid",
            Self::FundingConflict => "funding_conflict",
            Self::CursorScope => "cursor_scope",
            Self::ReplyLimit => "reply_limit",
            Self::Content => "content_mismatch",
            Self::ManifestRefused => "manifest_refused",
            Self::Unprepared => "unprepared",
            Self::ExistingRun => "new_run_required",
            Self::ProviderResponse => "provider_response",
            Self::SubmissionClaimed => "submission_already_claimed",
            Self::AssessmentRefused(error) => certificate_assessment::refusal_code(error),
            Self::ReferenceRefused(error) => references::refusal_code(error),
            Self::AccountRefused(error) => account::refusal_code(error),
            Self::FundingAssessmentRefused(error) => funding_assessment::refusal_code(error),
            Self::GatewaySyncRefused(error) => gateway_controls::sync_code(error),
            Self::GatewayRevocationRefused(error) => gateway_controls::revocation_code(error),
            Self::DownloadRefused(error) => download::refusal_code(error),
        }
    }
}

pub(super) fn run() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let result = match arguments::invocation(&args) {
        Ok(None) => return ExitCode::SUCCESS,
        Ok(Some(matches)) => execute_matches(&matches),
        Err(error) => Err(error),
    };
    match result {
        Ok(value) => {
            println!("{value}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            // Never print key material, paths, remote bodies or SDK diagnostics.
            println!("{}", json!({"error":error.code()}));
            ExitCode::from(if error == Failure::Arguments { 2 } else { 3 })
        }
    }
}

fn read(path: &Path, maximum: u64) -> Result<Vec<u8>, Failure> {
    let file = ic_host_fs::read::open_regular_file(path).map_err(|_| Failure::File)?;
    let maximum = usize::try_from(maximum).map_err(|_| Failure::File)?;
    let bytes = ic_host_fs::read::read_opened_file(file, maximum).map_err(|_| Failure::File)?;
    if bytes.is_empty() {
        return Err(Failure::File);
    }
    Ok(bytes)
}

/// Read recovery evidence through a no-follow descriptor; ancestors stay caller-owned.
fn read_no_follow(path: &Path, maximum: u64) -> Result<Vec<u8>, Failure> {
    #[cfg(unix)]
    {
        let maximum = usize::try_from(maximum).map_err(|_| Failure::File)?;
        let bytes =
            ic_host_fs::read::read_file_no_follow(path, maximum).map_err(|error| match error {
                ic_host_artifacts::artifact::ArtifactError::Io(error)
                    if error.raw_os_error() == Some(libc::ELOOP) =>
                {
                    Failure::Binding
                }
                _ => Failure::File,
            })?;
        if bytes.is_empty() {
            return Err(Failure::File);
        }
        Ok(bytes)
    }
    #[cfg(not(unix))]
    {
        let _ = (path, maximum);
        Err(Failure::File)
    }
}

fn identity(path: &Path, expected: Principal) -> Result<Box<dyn Identity>, Failure> {
    let bytes = read(path, 16 * 1024)?;
    let identity: Box<dyn Identity> = if let Ok(identity) = Secp256k1Identity::from_pem(&bytes) {
        Box::new(identity)
    } else {
        Box::new(BasicIdentity::from_pem(&bytes).map_err(|_| Failure::Identity)?)
    };
    if identity.sender().map_err(|_| Failure::Identity)? != expected.into() {
        return Err(Failure::IdentityBinding);
    }
    Ok(identity)
}

#[cfg(test)]
fn execute(args: &[String]) -> Result<serde_json::Value, Failure> {
    execute_matches(&arguments::parse(args)?)
}

fn execute_matches(matches: &clap::ArgMatches) -> Result<serde_json::Value, Failure> {
    if matches.get_flag("version") {
        return Ok(
            json!({"tool":"blob-storage","version":ic_blob_storage_contracts::CONTRACT_VERSION}),
        );
    }
    let (name, flags) = matches.subcommand().ok_or(Failure::Arguments)?;
    match name {
        "publish-inputs" => return publish_inputs::run(flags),
        "installation-check" => return installation_check::run(flags),
        "account-link-inputs" => return account_link_inputs::run(flags),
        "reference-inputs" => return reference_inputs::run(flags),
        "upload-inputs" => return upload_inputs::run(flags),
        _ => {}
    }
    let (command, options) = arguments::Options::from_matches(name, flags)?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| Failure::Transport)?;
    runtime.block_on(observe(&command, &options))
}

async fn observe(
    command: &arguments::Command,
    options: &arguments::Options,
) -> Result<serde_json::Value, Failure> {
    match command {
        arguments::Command::Metrics { service, namespace } => {
            metrics::run(options, *service, *namespace).await
        }
        arguments::Command::PublishSession(input) => publish_session::run(options, input).await,
        arguments::Command::PublishMap(input) => publish_map::run(options, input).await,
        arguments::Command::UploadSetup(input) => upload_setup::run(options, input)
            .await
            .map(|outcome| outcome.report),
        arguments::Command::PublishCheck(input) => publish_check::run(options, input).await,
        arguments::Command::PublishPrepare(input) => publish_prepare::run(options, input).await,
        arguments::Command::PublishPrepareBatch(input) => {
            publish_prepare::batch::run(options, input).await
        }
        arguments::Command::InspectAccount(input) => account::run(options, *input).await,
        arguments::Command::FundingAssessment(input) => {
            funding_assessment::run(options, *input).await
        }
        arguments::Command::GatewayControl(input) => gateway_controls::run(options, input).await,
        arguments::Command::Reference(input) => references::run(options, input).await,
        arguments::Command::FundingOutcome(input) => funding_outcome::run(options, *input).await,
        arguments::Command::UploadHistory(input) => upload_history::run(options, input).await,
        arguments::Command::CertificateAssessment(input) => {
            certificate_assessment::run(options, input).await
        }
        arguments::Command::SubmitReference(input) => submit_reference::run(options, input).await,
        arguments::Command::Download(input) => download::run(options, input).await,
        arguments::Command::FundingHistory { scope, cursor } => {
            let request = history::request(*scope, cursor.as_deref())?;
            let response = query(
                options,
                request.scope.service,
                protocol::FUNDING_HISTORY_METHOD,
                wire::encode_request(protocol::FUNDING_HISTORY_METHOD, request)
                    .map_err(|_| Failure::Arguments)?,
            )
            .await?;
            history::output(
                request,
                &response,
                options.actor,
                options.network,
                options.url.as_str(),
            )
        }
        arguments::Command::VerifyUpload {
            service,
            namespace,
            permission,
            body,
            max_bytes,
        } => {
            let verification = verify_upload::Verification::open(
                *service,
                *namespace,
                options.actor,
                permission,
                body,
                *max_bytes,
            )?;
            let response = query(
                options,
                verification.permission.upload.service,
                protocol::UPLOAD_MANIFEST_INSPECT_METHOD,
                wire::encode_request(
                    protocol::UPLOAD_MANIFEST_INSPECT_METHOD,
                    verification.permission,
                )
                .map_err(|_| Failure::Arguments)?,
            )
            .await?;
            verification.finish(
                &response,
                options.actor,
                options.network,
                options.url.as_str(),
            )
        }
        arguments::Command::Status { scope } => {
            let response = query(
                options,
                scope.service,
                LOCAL_STATUS_METHOD,
                wire::encode_request(protocol::LOCAL_STATUS_METHOD, scope)
                    .map_err(|_| Failure::Arguments)?,
            )
            .await?;
            let status = reply::decode(&response, *scope)?;
            Ok(reply::output(
                &status,
                options.actor,
                options.network,
                options.url.as_str(),
            ))
        }
    }
}

fn agent(options: &arguments::Options) -> Result<Agent, Failure> {
    agent_for(options, &options.identity, options.actor)
}

fn agent_for(
    options: &arguments::Options,
    identity_path: &Path,
    actor: Principal,
) -> Result<Agent, Failure> {
    agent_with_identity(options, identity(identity_path, actor)?)
}

/// Configure transport from the already checked signer without reopening its file.
fn agent_with_identity(
    options: &arguments::Options,
    identity: Box<dyn Identity>,
) -> Result<Agent, Failure> {
    let root = options
        .root_key
        .as_ref()
        .map(|path| read(path, 1024))
        .transpose()?;
    let client = http::client_builder(options.network);
    let agent = Agent::builder()
        .with_url(options.url.as_str())
        .with_boxed_identity(identity)
        // The SDK's with_http_client adds automatic 429/503 retries even when
        // TCP retries are zero. Its public middleware hook uses our client
        // directly, retaining byte limits without an automatic redispatch layer.
        .with_arc_http_middleware(std::sync::Arc::new(
            client.build().map_err(|_| Failure::Transport)?,
        ))
        .with_max_tcp_error_retries(0)
        .with_max_response_body_size(256 * 1024)
        .with_verify_query_signatures(true)
        .build()
        .map_err(|_| Failure::Transport)?;
    if let Some(root) = root {
        agent.set_root_key(root);
    }
    Ok(agent)
}

async fn query(
    options: &arguments::Options,
    service: Principal,
    method: &str,
    argument: Vec<u8>,
) -> Result<Vec<u8>, Failure> {
    let agent = agent(options)?;
    query::with_agent(&agent, service, method, argument).await
}

/// Submit one inspection update and let the SDK poll only that request identity.
/// Callers retain agent configuration, request validation and artifact ordering.
async fn inspection_update(
    agent: &Agent,
    service: Principal,
    method: &str,
    argument: Vec<u8>,
) -> Result<Vec<u8>, Failure> {
    tokio::time::timeout(
        Duration::from_secs(30),
        agent
            .update(&service.into(), method)
            .with_arg(argument)
            .call_and_wait(),
    )
    .await
    .map_err(|_| Failure::Timeout)?
    .map_err(|_| Failure::Transport)
}
