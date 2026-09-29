mod arguments;
mod history;
mod reply;
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

const USAGE: &str = "blob-storage status|funding-history --network ic|local --url URL --identity PEM --operator PRINCIPAL --service PRINCIPAL --namespace DECIMAL --cashier PRINCIPAL --payer PRINCIPAL [--root-key DER]\nverify-upload --network ic|local --url URL --identity PEM --actor PRINCIPAL --service PRINCIPAL --namespace DECIMAL --permission CANDID --body FILE --max-bytes DECIMAL [--root-key DER]\nverify-upload checks a regular local file against the authenticated original manifest. Permission is one binary Candid UploadAdmissionRequest; maximum is at most 1 GiB. No provider availability, completion, or retry authority is established.\nfunding-history additionally accepts --cursor FILE containing the previous non-null next object; reads one page, never auto-paginates. Local mode requires an explicit trusted root-key file and a literal loopback URL. IC mode uses the built-in IC root and HTTPS; root-key overrides are rejected. Supports Ed25519 and secp256k1 PEM identities. Queries only shared service observations; no provider calls or mutations. Exit 0: observation (including empty or fenced); 2: arguments; 3: failure. Query signatures are verified; observations are not certified state, provider credit or dispatch authority.\n";

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
    let (service, method, argument) = if let Some(verification) = &verification {
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
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| Failure::Transport)?;
    runtime.block_on(async {
        let response = query(&options, service, method, argument).await?;
        if let Some(verification) = verification {
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
    })
}

async fn query(
    options: &arguments::Options,
    service: Principal,
    method: &str,
    argument: Vec<u8>,
) -> Result<Vec<u8>, Failure> {
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
    tokio::time::timeout(
        Duration::from_secs(30),
        agent.query(&service, method).with_arg(argument).call(),
    )
    .await
    .map_err(|_| Failure::Timeout)?
    .map_err(|_| Failure::Transport)
}
