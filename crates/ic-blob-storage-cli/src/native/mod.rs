mod arguments;
mod attestation;
mod history;
mod observe_upload;
mod reply;
mod submit_attestation;
#[cfg(test)]
mod tests;
mod verify_upload;

use candid::Principal;
use ic_agent::{
    Agent, Identity,
    identity::{BasicIdentity, Secp256k1Identity},
};
use ic_blob_storage::ops::service::operator::LOCAL_STATUS_METHOD;
use serde_json::json;
use std::{fs::File, io::Read, path::Path, process::ExitCode, time::Duration};

const USAGE: &str = concat!(
    "blob-storage status|funding-history --network ic|local --url URL --identity PEM --operator PRINCIPAL --service PRINCIPAL --namespace DECIMAL --cashier PRINCIPAL --payer PRINCIPAL [--root-key DER]\n",
    "verify-upload --network ic|local --url URL --identity PEM --actor PRINCIPAL --service PRINCIPAL --namespace DECIMAL --permission CANDID --body FILE --max-bytes DECIMAL [--root-key DER]\n",
    "upload-attestation --network ic|local --url URL --identity PEM --actor PRINCIPAL --service PRINCIPAL --namespace DECIMAL --verifier PRINCIPAL --statement CANDID [--root-key DER]\n",
    "observe-upload --network ic|local --url URL --identity PEM --actor VERIFIER --service PRINCIPAL --namespace DECIMAL --permission CANDID --gateway ORIGIN --max-bytes DECIMAL --run-dir NEW_DIRECTORY [--root-key DER]\n",
    "submit-attestation --network ic|local --url URL --identity PEM --actor VERIFIER --service PRINCIPAL --namespace DECIMAL --run-dir COMPLETED_OBSERVATION_DIRECTORY [--root-key DER]\n",
    "submit-attestation validates a complete successful observation, saves the exact signed update and intent under run-dir/attestation, then submits once. Existing submission directories refuse; pending or uncertain results require upload-attestation inspection using attestation/statement.candid. No polling, retry, provider read or regenerated statement. Keep these signed artifacts private.\n",
    "observe-upload queries the installed verification plan, performs one bounded gateway GET, checks all bytes against original metadata/root and durably saves statement.candid. Explicit gateway origin and provider read budget are required. Partial runs remain; no resume, redirects, retries or attestation dispatch. Provider reads may incur charges.\n",
    "upload-attestation compares one saved binary Candid UploadAttestationRequest with immutable service history. The expected verifier must come from installation configuration. Outcomes are matched, conflict or absent; none authorizes retry. No mutation, provider fetch or journal write occurs.\n",
    "verify-upload checks a regular local file against the authenticated original manifest. Permission is one binary Candid UploadAdmissionRequest; maximum is at most 1 GiB. No provider availability, completion, or retry authority is established.\n",
    "funding-history additionally accepts --cursor FILE containing the previous non-null next object; reads one page, never auto-paginates. Local mode requires an explicit trusted root-key file and literal loopback origins. IC mode uses the built-in IC root and HTTPS; root-key overrides are rejected. Supports Ed25519 and secp256k1 PEM identities. Only observe-upload calls a provider; only submit-attestation mutates the service. Exit 0: observation (including conflict, empty or fenced), accepted submission or pending request; 2: arguments; 3: failure. Query signatures are verified; observations are not certified state, provider credit or dispatch authority.\n",
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Failure {
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
    ServiceInternal,
    ServiceInvalid,
    CursorScope,
    ReplyLimit,
    Content,
    ManifestRefused,
    Unprepared,
    AttestationRefused,
    ExistingRun,
    VerificationRefused,
    ProviderResponse,
    Clock,
    Observation,
    SubmissionClaimed,
}
impl Failure {
    const fn code(self) -> &'static str {
        match self {
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
            Self::ServiceInternal => "service_internal",
            Self::ServiceInvalid => "service_invalid",
            Self::CursorScope => "cursor_scope",
            Self::ReplyLimit => "reply_limit",
            Self::Content => "content_mismatch",
            Self::ManifestRefused => "manifest_refused",
            Self::Unprepared => "unprepared",
            Self::AttestationRefused => "attestation_refused",
            Self::ExistingRun => "new_run_required",
            Self::VerificationRefused => "verification_refused",
            Self::ProviderResponse => "provider_response",
            Self::Clock => "clock",
            Self::Observation => "invalid_observation",
            Self::SubmissionClaimed => "submission_already_claimed",
        }
    }
}

pub(super) fn run() -> ExitCode {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args == ["--help"] {
        print!("{USAGE}");
        return ExitCode::SUCCESS;
    }
    match execute(&args) {
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
    let mut bytes = Vec::new();
    let file = File::open(path).map_err(|_| Failure::File)?;
    if !file.metadata().map_err(|_| Failure::File)?.is_file() {
        return Err(Failure::File);
    }
    file.take(maximum + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| Failure::File)?;
    if bytes.is_empty() || bytes.len() as u64 > maximum {
        return Err(Failure::File);
    }
    Ok(bytes)
}

fn identity(path: &Path, expected: Principal) -> Result<Box<dyn Identity>, Failure> {
    let bytes = read(path, 16 * 1024)?;
    let identity: Box<dyn Identity> = if let Ok(identity) = Secp256k1Identity::from_pem(&bytes) {
        Box::new(identity)
    } else {
        Box::new(BasicIdentity::from_pem(&bytes).map_err(|_| Failure::Identity)?)
    };
    if identity.sender().map_err(|_| Failure::Identity)? != expected {
        return Err(Failure::IdentityBinding);
    }
    Ok(identity)
}

fn execute(args: &[String]) -> Result<serde_json::Value, Failure> {
    let options = arguments::Options::parse(args)?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| Failure::Transport)?;
    runtime.block_on(observe(&options))
}

async fn observe(options: &arguments::Options) -> Result<serde_json::Value, Failure> {
    match &options.command {
        arguments::Command::SubmitAttestation(input) => {
            submit_attestation::run(options, input).await
        }
        arguments::Command::ObserveUpload(input) => observe_upload::run(options, input).await,
        _ => inspect(options).await,
    }
}

async fn inspect(options: &arguments::Options) -> Result<serde_json::Value, Failure> {
    let recovery = match &options.command {
        arguments::Command::UploadAttestation {
            service,
            namespace,
            verifier,
            statement,
        } => Some(attestation::Recovery::open(
            *service,
            *namespace,
            *verifier,
            options.actor,
            statement,
        )?),
        _ => None,
    };
    let history = match &options.command {
        arguments::Command::FundingHistory { scope, cursor } => {
            Some(history::request(*scope, cursor.as_deref())?)
        }
        _ => None,
    };
    let verification = match &options.command {
        arguments::Command::VerifyUpload {
            service,
            namespace,
            permission,
            body,
            max_bytes,
        } => Some(verify_upload::Verification::open(
            *service,
            *namespace,
            options.actor,
            permission,
            body,
            *max_bytes,
        )?),
        _ => None,
    };
    let (service, method, argument) = if let Some(recovery) = &recovery {
        recovery.query()
    } else if let Some(verification) = &verification {
        (
            verification.permission.upload.service,
            ic_blob_storage::ops::service::uploads::manifests::UPLOAD_MANIFEST_INSPECT_METHOD,
            candid::encode_one(verification.permission).map_err(|_| Failure::Arguments)?,
        )
    } else if let Some(request) = history {
        (
            request.scope.service,
            ic_blob_storage::ops::service::funding::history::boundary::FUNDING_HISTORY_METHOD,
            candid::encode_one(request).map_err(|_| Failure::Arguments)?,
        )
    } else if let arguments::Command::Status { scope } = options.command {
        (
            scope.service,
            LOCAL_STATUS_METHOD,
            candid::encode_one(scope).map_err(|_| Failure::Arguments)?,
        )
    } else {
        return Err(Failure::Arguments);
    };
    let response = query(options, service, method, argument).await?;
    if let Some(recovery) = recovery {
        recovery.output(
            &response,
            options.actor,
            options.network,
            options.url.as_str(),
        )
    } else if let Some(verification) = verification {
        verification.finish(
            &response,
            options.actor,
            options.network,
            options.url.as_str(),
        )
    } else if let Some(request) = history {
        history::output(
            request,
            &response,
            options.actor,
            options.network,
            options.url.as_str(),
        )
    } else if let arguments::Command::Status { scope } = options.command {
        let status = reply::decode(&response, scope)?;
        Ok(reply::output(
            &status,
            options.actor,
            options.network,
            options.url.as_str(),
        ))
    } else {
        Err(Failure::Arguments)
    }
}

fn agent(options: &arguments::Options) -> Result<Agent, Failure> {
    let identity = identity(&options.identity, options.actor)?;
    let root = options
        .root_key
        .as_ref()
        .map(|path| read(path, 1024))
        .transpose()?;
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .retry(reqwest::retry::never())
        .timeout(Duration::from_secs(30));
    // A local replica request must not leave loopback through an environment proxy.
    let client = if options.network == "local" {
        client.no_proxy()
    } else {
        client
    };
    let agent = Agent::builder()
        .with_url(options.url.as_str())
        .with_boxed_identity(identity)
        .with_http_client(client.build().map_err(|_| Failure::Transport)?)
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
    tokio::time::timeout(
        Duration::from_secs(30),
        agent.query(&service, method).with_arg(argument).call(),
    )
    .await
    .map_err(|_| Failure::Timeout)?
    .map_err(|_| Failure::Transport)
}
