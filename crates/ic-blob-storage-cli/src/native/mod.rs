mod account;
mod arguments;
mod artifacts;
mod attestation;
mod certificate_assessment;
mod download;
mod funding_assessment;
mod funding_outcome;
mod gateway_controls;
mod history;
mod local_body;
mod observe_upload;
mod provider_download;
mod reference_inputs;
mod references;
mod reply;
mod submit_attestation;
mod submit_reference;
#[cfg(test)]
mod tests;
mod upload_history;
mod upload_inputs;
mod upload_setup;
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
    "blob-storage upload-inputs --binding JSON --manifest UPSTREAM_MANIFEST_JSON --body FILE --max-bytes DECIMAL --run-dir NEW_DIRECTORY\n",
    "Offline preparation of permission.candid, manifest.candid, first-reference download.candid/reference-status.candid, certificate-binding.json for the existing browser client and a complete root-verified body.bin snapshot. Explicit original identities and Caffeine's prepared declaration; no signer, network, ID allocation or certificate. Failed snapshots remain private body.part; existing or partial directories refuse.\n",
    "blob-storage reference-inputs --permission PERMISSION_CANDID --action retain|release --reference DECIMAL --operation DECIMAL --run-dir NEW_DIRECTORY\n",
    "Offline exact reference.candid, reference-status.candid and download.candid generation from the original saved permission. Canonical positive identities are caller-supplied, never allocated. No signer, network, mutation, liveness, expiry renewal or retry authority; existing or partial directories refuse.\n",
    "blob-storage admit-upload|prepare-upload|revoke-upload --network ic|local --url URL --identity PEM --actor PRINCIPAL --service PRINCIPAL --namespace DECIMAL --request CANDID --run-dir NEW_DIRECTORY [--root-key DER]\n",
    "Upload setup persists exact signed intent before one local service update, with no polling or retry. Admit/revoke require the tenant; prepare requires the uploader and a complete UploadManifestRequest. No certificate, provider transfer or publication. Existing or partial runs refuse.\n",
    "blob-storage upload-permission|upload-manifest --network ic|local --url URL --identity PEM --actor PRINCIPAL --service PRINCIPAL --namespace DECIMAL --request PERMISSION_CANDID [--root-key DER]\n",
    "Exact historical inspection uses a saved UploadAdmissionRequest (permission.candid). Permission requires the tenant; manifest permits tenant or uploader. A retained record or unprepared/unknown result never authorizes redispatch or renews an expiry.\n",
    "blob-storage download --network ic|local --url URL --identity PEM --actor TENANT --service PRINCIPAL --namespace DECIMAL --request CANDID --project PROJECT --gateway ORIGIN --max-bytes DECIMAL --run-dir NEW_DIRECTORY [--root-key DER]\n",
    "download authenticates one exact live-reference descriptor through a replicated update, then performs one bounded provider GET. Original metadata/root and complete EOF verification precede body.bin publication; failed bodies remain body.part. No redirects, decoding, retry, publication lease or funding. Provider reads require an approved origin and budget. Existing or partial runs refuse.\n",
    "blob-storage funding-assessment --network ic|local --url URL --identity PEM --operator PRINCIPAL --service PRINCIPAL --namespace DECIMAL --cashier PRINCIPAL --payer PRINCIPAL --operation DECIMAL --offered DECIMAL [--target-balance DECIMAL] [--root-key DER]\n",
    "funding-assessment signs one passive preparation-policy query. It reports exact local limits/history/fence and missing provider/recovery/account/spendability evidence, without reserving funds, allocating an operation, querying a provider or paying. Observations never authorize preparation, dispatch or retry.\n",
    "blob-storage sync-gateways|cancel-gateway-sync|revoke-gateway --network ic|local --url URL --identity PEM --operator PRINCIPAL --service PRINCIPAL --namespace DECIMAL --cashier PRINCIPAL --payer PRINCIPAL --run-dir NEW_DIRECTORY [--sequence DECIMAL | --gateway PRINCIPAL] [--root-key DER]\n",
    "Gateway controls persist exact signed intent before one update. Cancel requires the observed pending sequence; revoke requires an explicit gateway. Existing/partial runs refuse. No polling, retry, provider deletion or billing cessation. After a lost reply inspect status; current membership is not an exact historical receipt or permission to repeat.\n",
    "blob-storage inspect-account --network ic|local --url URL --identity PEM --operator PRINCIPAL --service PRINCIPAL --namespace DECIMAL --cashier PRINCIPAL --payer PRINCIPAL --kind balance|relationship [--root-key DER]\n",
    "inspect-account submits one scoped service read update, then waits for that exact IC request within thirty seconds. It reports one provider balance or relationship, including absence/errors; no payment, gateway change, credit inference, automatic refresh or redispatch. A lost result remains unobserved.\n",
    "blob-storage submit-reference --network ic|local --url URL --identity PEM --actor TENANT --service PRINCIPAL --namespace DECIMAL --request CANDID --run-dir NEW_DIRECTORY [--root-key DER]\n",
    "submit-reference validates one exact ReferenceCommand, claims a new private directory and saves request.candid, the signed update and intent before one dispatch. Pending or uncertain results require reference-receipt inspection with saved request.candid; no polling, retry or identity allocation. A recorded inner failure is not a successful retain/release. Existing or partial runs refuse.\n",
    "blob-storage reference-receipt|reference-status --network ic|local --url URL --identity PEM --actor TENANT --service PRINCIPAL --namespace DECIMAL --request CANDID [--root-key DER]\n",
    "reference-receipt inspects one saved binary Candid ReferenceCommand; original success/failure is historical, with no fence or liveness observation. reference-status reads one ReferenceStatusRequest and returns current local liveness and fence. Each signs one tenant query; no mutation, provider call or retry authority. Canister tenants use ReplicatedReferenceClient.\n",
    "blob-storage funding-outcome --network ic|local --url URL --identity PEM --operator PRINCIPAL --service PRINCIPAL --namespace DECIMAL --cashier PRINCIPAL --payer PRINCIPAL --operation DECIMAL --offered DECIMAL [--target-balance DECIMAL] [--root-key DER]\n",
    "funding-outcome inspects one exact original funding intent. Omit target-balance only when the original intent had none. Absence, unknown transfer and reported balance never authorize another payment; no mutation or provider call occurs.\n",
    "blob-storage upload-history --network ic|local --url URL --identity PEM --operator PRINCIPAL --service PRINCIPAL --namespace DECIMAL --filter all|active|deletion-pending|outstanding [--cursor FILE] [--root-key DER]\n",
    "upload-history reads one service-wide page of retained upload identities and local states. Save a non-null next object for explicit continuation; empty filtered pages can still continue. No auto-pagination, provider call, retry or mutation.\n",
    "blob-storage certificate-assessment --network ic|local --url URL --identity PEM --actor UPLOADER --service PRINCIPAL --namespace DECIMAL --permission CANDID [--root-key DER]\n",
    "certificate-assessment reads one exact permission assessment and its missing prerequisites. It never issues a certificate, reserves issuance or authorizes retry, even with no blockers.\n",
    "blob-storage status|funding-history --network ic|local --url URL --identity PEM --operator PRINCIPAL --service PRINCIPAL --namespace DECIMAL --cashier PRINCIPAL --payer PRINCIPAL [--root-key DER]\n",
    "verify-upload --network ic|local --url URL --identity PEM --actor PRINCIPAL --service PRINCIPAL --namespace DECIMAL --permission CANDID --body FILE --max-bytes DECIMAL [--root-key DER]\n",
    "upload-attestation --network ic|local --url URL --identity PEM --actor PRINCIPAL --service PRINCIPAL --namespace DECIMAL --verifier PRINCIPAL --statement CANDID [--root-key DER]\n",
    "observe-upload --network ic|local --url URL --identity PEM --actor VERIFIER --service PRINCIPAL --namespace DECIMAL --permission CANDID --gateway ORIGIN --max-bytes DECIMAL --run-dir NEW_DIRECTORY [--root-key DER]\n",
    "submit-attestation --network ic|local --url URL --identity PEM --actor VERIFIER --service PRINCIPAL --namespace DECIMAL --run-dir COMPLETED_OBSERVATION_DIRECTORY [--root-key DER]\n",
    "submit-attestation validates a complete successful observation, saves the exact signed update and intent under run-dir/attestation, then submits once. Existing submission directories refuse; pending or uncertain results require upload-attestation inspection using attestation/statement.candid. No polling, retry, provider read or regenerated statement. Keep these signed artifacts private.\n",
    "observe-upload queries the installed verification plan, performs one bounded gateway GET, checks all bytes against original metadata/root and durably saves statement.candid. Explicit gateway origin and provider read budget are required. Partial runs remain; no resume, redirects, retries or attestation dispatch. Provider reads may incur charges.\n",
    "upload-attestation compares one saved binary Candid UploadAttestationRequest with immutable service history. The expected verifier must come from installation configuration. Outcomes are matched, conflict or absent; none authorizes retry. No mutation, provider fetch or journal write occurs.\n",
    "verify-upload checks a regular local file against the authenticated original manifest. Permission is one binary Candid UploadAdmissionRequest; maximum is at most 1 GiB. No provider availability, completion, or retry authority is established.\n",
    "funding-history additionally accepts --cursor FILE containing the previous non-null next object; reads one page, never auto-paginates. Local mode requires an explicit trusted root-key file and literal loopback origins. IC mode uses the built-in IC root and HTTPS; root-key overrides are rejected. Supports Ed25519 and secp256k1 PEM identities. Observe-upload fetches provider bytes; inspect-account asks the service for a provider observation; submit-attestation and submit-reference mutate the service. Exit 0: observation (including conflict, empty or fenced), recorded submission (inspect its result) or pending request; 2: arguments; 3: failure. Query signatures and update certificates are verified; observations are not provider credit or dispatch authority.\n",
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Failure {
    PreparedManifest,
    UploadAdmissionRefused(ic_blob_storage::dto::upload::admission::UploadAdmissionFailure),
    UploadManifestRefused(ic_blob_storage::dto::upload::manifest::UploadManifestFailure),
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
    FundingConflict,
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
    AssessmentRefused(ic_blob_storage::dto::upload::exposure::UploadExposureFailure),
    ReferenceRefused(ic_blob_storage::dto::reference::ReferenceFailure),
    AccountRefused(ic_blob_storage::dto::account::AccountInspectionFailure),
    FundingAssessmentRefused(ic_blob_storage::dto::funding::assessment::FundingPreparationFailure),
    GatewaySyncRefused(ic_blob_storage::dto::gateway::sync::GatewaySyncFailure),
    GatewayRevocationRefused(ic_blob_storage::dto::gateway::GatewayRevocationFailure),
    DownloadRefused(ic_blob_storage::dto::download::DownloadFailure),
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
            Self::ServiceInternal => "service_internal",
            Self::ServiceInvalid => "service_invalid",
            Self::FundingConflict => "funding_conflict",
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
    if args
        .first()
        .is_some_and(|command| command == "reference-inputs")
    {
        return reference_inputs::run(args);
    }
    if args
        .first()
        .is_some_and(|command| command == "upload-inputs")
    {
        return upload_inputs::run(args);
    }
    let options = arguments::Options::parse(args)?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| Failure::Transport)?;
    runtime.block_on(observe(&options))
}

async fn observe(options: &arguments::Options) -> Result<serde_json::Value, Failure> {
    match &options.command {
        arguments::Command::UploadSetup(input) => upload_setup::run(options, input).await,
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
        arguments::Command::SubmitAttestation(input) => {
            submit_attestation::run(options, input).await
        }
        arguments::Command::SubmitReference(input) => submit_reference::run(options, input).await,
        arguments::Command::ObserveUpload(input) => observe_upload::run(options, input).await,
        arguments::Command::Download(input) => download::run(options, input).await,
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
    tokio::time::timeout(
        Duration::from_secs(30),
        agent.query(&service, method).with_arg(argument).call(),
    )
    .await
    .map_err(|_| Failure::Timeout)?
    .map_err(|_| Failure::Transport)
}
