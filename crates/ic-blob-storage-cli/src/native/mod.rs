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
mod http;
mod installation_check;
mod local_body;
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

const SAFETY_HELP: &str = concat!(
    "Offline JSON tool/version identity; not installed-service or provider qualification.\n",
    "New run directories use owner-only creation and reject CR/LF in their path or physical parent. Existing and partial runs refuse.\n",
    "account-link-inputs is offline. It encodes explicit positive raw provider terms with an explicit payer/expiry, never selects units, signs, links or funds an account. Existing/partial output refuses; generated input is not an effect journal, financial cap proof or retry permit.\n",
    "installation-check is offline. It reads a strict JSON configuration proposal and validates the complete proposed installation, preserving configuration.json and encoded configuration.wire/installation.wire with hashes and a passive summary. It does not authenticate platform/release identity, grant memory, deploy, provision, fund or qualify provider facts. Existing/partial output refuses.\n",
    "Offline preparation checks the complete installation candidate against the original service/namespace/project/trusted uploader and installed resource bounds. It saves installation.wire, permission.wire, manifest.wire, first-reference requests, certificate-binding.json and a root-verified body.bin snapshot. No authentication, actual installed-state/provisioning proof, signer, network, ID allocation or certificate. Failed snapshots remain private body.part; existing or partial directories refuse.\n",
    "Offline batch preparation verifies frozen input hashes, unique identities, one tenant/provider scope and aggregate fresh-installation capacity before serial root-verified snapshots. Final summary appears only on complete success; failures retain partial evidence and refuse overwrite. No remaining-live-capacity proof, allocation, certificate, network, publication or retry authority.\n",
    "Reverify one complete frozen batch and authenticate current enrollment, fence and exact first-reference liveness. Write media-map.json only when every file passes; blocked or partial runs never emit a map. Sequential observations are not an atomic publication/retention lease or public serving qualification. No update, provider request, reference mutation or retry.\n",
    "publish-check reverifies complete frozen snapshots and request packets before signed sequential capacity/content queries. Retain query intents/replies, blockers and original discovered IDs. Existing roots require recovery, retain or retirement handling; no allocation, reservation, updates, provider calls or retry/publication authority. A blocked summary is an observation, not command failure. Partial runs retain failure evidence and refuse overwrite.\n",
    "Prepare one indexed file serially. Reverify the entire batch, authenticate both roles and retain finite capacity/discovery observations before at most two local updates. Resume requires --source-run, binds the original signed admission, and never resubmits a claimed update; it may send only an unclaimed preparation. Four queries maximum per invocation. No certificate, provider transfer, expiry renewal, allocation or publication. Preserve the original journal; partial claims and unknown state never license redispatch.\n",
    "A bounded JSON-lines session retains one validated batch. Prepare/transfer/status accept the current original index; status advances on exact registration and a live reference, not provider availability. Transfer owns the original browser handoff; keep its journal/profile for lost replies. Map finishes only after every index passes. No verifier, automatic retry or publication lease.
",
    "Verify the frozen batch once and serially prepare its files through the same signed setup workflow. Each file has an independent file-NNNN journal, at most two updates and four queries. Stop at the first blocker, uncertainty or failure; never repeat a claimed step or resume the whole batch. The overall deadline covers all setup calls. Recovery uses publish-prepare-resume with the original file directory. No certificate, provider transfer or publication; active reservation capacity still limits progress.\n",
    "Offline exact reference.wire, reference-status.wire and download.wire generation from the original saved permission. Canonical positive identities are caller-supplied, never allocated. No signer, network, mutation, liveness, expiry renewal or retry authority; existing or partial directories refuse.\n",
    "Upload setup persists exact signed intent before one local service update, with no polling or retry. Admit/revoke require the tenant; prepare requires the uploader and a complete UploadManifestRequest. No certificate, provider transfer or publication. Existing or partial runs refuse.\n",
    "Exact historical inspection uses a saved UploadAdmissionRequest (permission.wire). Permission requires the tenant; manifest permits tenant or uploader. A retained record or unprepared/unknown result never authorizes redispatch or renews an expiry.\n",
    "download authenticates one exact live-reference descriptor through a replicated update, then performs one bounded provider GET. Original metadata/root and complete EOF verification precede body.bin publication; failed bodies remain body.part. No redirects, decoding, retry, publication lease or funding. Provider reads require an approved origin and budget. Existing or partial runs refuse.\n",
    "funding-assessment signs one passive preparation-policy query. It reports exact local limits/history/fence and missing provider/recovery/account/spendability evidence, without reserving funds, allocating an operation, querying a provider or paying. Observations never authorize preparation, dispatch or retry.\n",
    "Gateway controls persist exact signed intent before one update. Cancel requires the observed pending sequence; revoke requires an explicit gateway. Existing/partial runs refuse. No polling, retry, provider deletion or billing cessation. After a lost reply inspect status; current membership is not an exact historical receipt or permission to repeat.\n",
    "inspect-account submits one scoped service read update, then waits for that exact IC request within thirty seconds. It reports one provider balance or relationship, including absence/errors; no payment, gateway change, credit inference, automatic refresh or redispatch. A lost result remains unobserved.\n",
    "submit-reference validates one exact ReferenceCommand, claims a new private directory and saves request.wire, reference.wire, the signed update and intent before one dispatch. Pending or uncertain results require reference-receipt inspection with saved reference.wire; no polling, retry or identity allocation. A recorded inner failure is not a successful retain/release. Existing or partial runs refuse.\n",
    "reference-receipt inspects one saved Wire document ReferenceCommand; original success/failure is historical, with no fence or liveness observation. reference-status reads one ReferenceStatusRequest and returns current local liveness and fence. Each signs one tenant query; no mutation, provider call or retry authority. Canister tenants use ReplicatedReferenceClient.\n",
    "funding-outcome inspects one exact original funding intent. Omit target-balance only when the original intent had none. Absence, unknown transfer and reported balance never authorize another payment; no mutation or provider call occurs.\n",
    "upload-history reads one service-wide page of retained upload identities and local states. Save a non-null next object for explicit continuation; empty filtered pages can still continue. No auto-pagination, provider call, retry or mutation.\n",
    "certificate-assessment reads one exact permission assessment and its missing prerequisites. It never issues a certificate, reserves issuance or authorizes retry, even with no blockers.\n",
);

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
    let args: Vec<_> = std::env::args().skip(1).collect();
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
