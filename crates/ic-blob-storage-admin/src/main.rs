//! Native management inspection and guarded dispatch of an immutable drain request.

mod drain;

use clap::{Args, Parser, Subcommand};
use ic_blob_storage::{
    local::Error,
    protocol::{Installation, ManagementView, RPC_LIMITS},
};
use ic_platform::{
    Principal,
    agent::{Agent, BasicIdentity, Identity},
};
use sha2::{Digest, Sha256};
use std::{
    path::{Path, PathBuf},
    process::ExitCode,
    time::Duration,
};
use url::Url;

#[derive(Parser)]
#[command(
    version,
    about = "Inspect a Blob host and retain original management request identity"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Authenticate and compare installed configuration and Wasm with admitted inputs.
    Inspect(Target),
    /// Retain a signed drain request in a new private directory, WITHOUT submitting it.
    PrepareDrain {
        #[command(flatten)]
        target: Target,
        /// New immutable claim directory under a trusted, existing parent.
        #[arg(long)]
        record: PathBuf,
        /// Exact next maintenance attempt. Never inferred or advanced on retry.
        #[arg(long)]
        attempt: std::num::NonZeroU64,
    },
    /// Authenticate the original request status; never submit or resubmit it.
    ObserveDrain {
        #[command(flatten)]
        target: Target,
        /// Original complete claim directory; missing/partial records refuse.
        #[arg(long)]
        record: PathBuf,
    },
    /// Dispatch the retained drain once under target custody; never renew or retry it.
    DispatchDrain {
        #[command(flatten)]
        target: Target,
        /// Original complete claim directory.
        #[arg(long)]
        record: PathBuf,
        /// Single trusted custody root used by all managers of this target.
        #[arg(long)]
        custody: PathBuf,
        /// Trusted Blob checkout with this binary's admitted tooling snapshot.
        #[arg(long)]
        checkout: PathBuf,
    },
}

#[derive(Args)]
struct Target {
    /// IC API endpoint: HTTPS, or loopback HTTP for a local replica.
    #[arg(long, value_parser = endpoint)]
    url: Url,
    /// Target canister principal.
    #[arg(long)]
    canister: Principal,
    /// Manager's Ed25519 PKCS#8 PEM; never printed or uploaded.
    #[arg(long)]
    identity: PathBuf,
    /// Independently trusted network BLS root key (DER). Never fetched automatically.
    #[arg(long)]
    root_key: PathBuf,
    /// Exact Wire blob_init request containing the expected immutable configuration.
    #[arg(long)]
    installation: PathBuf,
    /// Independently reviewed, uncompressed installed Wasm artifact.
    #[arg(long)]
    wasm: PathBuf,
    /// Expected durable record format, currently 1; not the release version.
    #[arg(long, value_parser = clap::value_parser!(u32).range(1..))]
    state_version: u32,
}

struct Context {
    agent: Agent,
    url: String,
    canister: Principal,
    configuration: Installation,
    wasm_hash: [u8; 32],
    root_hash: [u8; 32],
    state_version: u32,
}

fn endpoint(value: &str) -> Result<Url, String> {
    let url = Url::parse(value).map_err(|_| "invalid endpoint URL")?;
    let loopback = url.host_str().is_some_and(|host| {
        host == "localhost"
            || host
                .trim_matches(['[', ']'])
                .parse::<std::net::IpAddr>()
                .is_ok_and(|ip| ip.is_loopback())
    });
    if (url.scheme() != "https" && !(url.scheme() == "http" && loopback))
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err("use HTTPS or loopback HTTP without credentials, query or fragment".into());
    }
    Ok(url)
}

fn read(path: &Path, limit: usize) -> Result<Vec<u8>, String> {
    let bytes = ic_host_fs::read::read_file(path, limit)
        .map_err(|_| format!("cannot read bounded regular file: {}", path.display()))?;
    if bytes.is_empty() {
        return Err(format!("empty input: {}", path.display()));
    }
    Ok(bytes)
}

impl Context {
    fn new(target: Target) -> Result<Self, String> {
        let Target {
            url,
            canister,
            identity,
            root_key,
            installation,
            wasm,
            state_version,
        } = target;
        if canister == Principal::anonymous() || canister == Principal::management_canister() {
            return Err("expected an installed Blob canister".into());
        }
        let configuration: Installation = ic_wire::native::decode_request(
            ic_wire::native::method_id("blob_init"),
            &read(&installation, RPC_LIMITS.bytes)?,
            RPC_LIMITS,
        )
        .map_err(|_| "invalid expected installation frame")?;
        let signer = BasicIdentity::from_pem(read(&identity, 16 * 1024)?)
            .map_err(|_| "invalid Ed25519 identity PEM")?;
        let caller = signer.sender().map_err(|_| "identity has no sender")?;
        if caller.as_slice() != configuration.manager.as_slice() {
            return Err("identity does not match the expected manager".into());
        }
        let wasm_hash = Sha256::digest(read(&wasm, 16 * 1024 * 1024)?).into();
        let key = read(&root_key, 1024)?;
        let root_hash = Sha256::digest(&key).into();
        let agent = Agent::builder()
            .with_url(url.as_str())
            .with_identity(signer)
            .with_max_response_body_size(2 * 1024 * 1024)
            .with_max_polling_time(Duration::from_secs(30))
            .with_max_tcp_error_retries(0)
            .build()
            .map_err(|_| "cannot configure IC transport")?;
        agent.set_root_key(key);
        Ok(Self {
            agent,
            url: url.into(),
            canister,
            configuration,
            wasm_hash,
            root_hash,
            state_version,
        })
    }

    async fn inspect(&self) -> Result<ManagementView, String> {
        let target = self
            .canister
            .as_slice()
            .try_into()
            .map_err(|_| "invalid target")?;
        let request = ic_wire::native::encode_request(
            ic_wire::native::method_id("blob_management"),
            &(),
            RPC_LIMITS,
        )
        .map_err(|_| "cannot encode inspection")?;
        let observed = self
            .agent
            .read_state_canister_module_hash(target)
            .await
            .map_err(|_| "module hash authentication failed")?;
        if observed != self.wasm_hash {
            return Err("installed Wasm does not match the admitted artifact".into());
        }
        let bytes = self
            .agent
            .update(&target, "blob_management")
            .with_arg(request)
            .call_and_wait()
            .await
            .map_err(|_| "management inspection failed; no state-changing retry was attempted")?;
        let result: Result<ManagementView, Error> = ic_wire::native::decode_response(
            ic_wire::native::method_id("blob_management"),
            &bytes,
            RPC_LIMITS,
        )
        .map_err(|_| "invalid management reply")?;
        let view = result.map_err(|_| "host refused management inspection")?;
        if view.installation != self.canister
            || view.configuration != self.configuration
            || view.state_version != self.state_version
        {
            return Err("installed identity, configuration or state format mismatch".into());
        }
        // Authenticated observations are not atomic and do not establish continuity.
        let observed = self
            .agent
            .read_state_canister_module_hash(target)
            .await
            .map_err(|_| "module hash authentication failed")?;
        if observed != self.wasm_hash {
            return Err("installed Wasm changed during inspection".into());
        }
        Ok(view)
    }
}

async fn run(command: Command) -> Result<String, String> {
    match command {
        Command::Inspect(target) => {
            let view = Context::new(target)?.inspect().await?;
            Ok(format!(
                "Inspection matched: installation={} state_version={} attempt={} active={}\nAuthenticated observations only; no continuity or activation approval.",
                view.installation,
                view.state_version,
                view.maintenance.attempt,
                view.maintenance.active
            ))
        }
        Command::PrepareDrain {
            target,
            record,
            attempt,
        } => drain::prepare(&Context::new(target)?, &record, attempt.get()).await,
        Command::ObserveDrain { target, record } => {
            drain::observe(&Context::new(target)?, &record).await
        }
        Command::DispatchDrain {
            target,
            record,
            custody,
            checkout,
        } => {
            drain::admit_tooling(&checkout).await?;
            drain::dispatch(&Context::new(target)?, &record, &custody).await
        }
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let command = Cli::parse().command;
    match tokio::time::timeout(Duration::from_secs(45), run(command)).await {
        Ok(Ok(message)) => {
            println!("{message}");
            ExitCode::SUCCESS
        }
        result => {
            let message = match result {
                Ok(Err(message)) => message,
                Err(_) => "management timed out; dispatch may have occurred. Preserve custody and observe the original request; do not resubmit".into(),
                Ok(Ok(_)) => unreachable!(),
            };
            eprintln!("Management refused: {message}");
            ExitCode::FAILURE
        }
    }
}
