//! Native storage inspection, offline enrollment proposals and maintenance custody.

#![forbid(unsafe_code)]

mod application;
mod certificate;
mod files;
mod management;
mod network;
mod overview;
mod provider;
#[cfg(test)]
mod tests;

use clap::{Args, CommandFactory, FromArgMatches, Parser, Subcommand};
use ic_blob_storage::{
    local::Error,
    protocol::{Installation, ManagementView, RPC_LIMITS},
};
use ic_platform_types::Principal;
use network::Client;
use sha2::{Digest, Sha256};
use std::{
    path::{Path, PathBuf},
    process::ExitCode,
    time::Duration,
};
use url::Url;

#[derive(Parser)]
#[command(
    name = "blob",
    version,
    about = "Inspect storage and manage Blob uploads",
    after_help = "Start here: blob overview --help\nExplore a group: blob <group> --help"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn help_command() -> clap::Command {
    fn banner(command: clap::Command) -> clap::Command {
        command
            .help_template(concat!(
                "Blob ",
                env!("CARGO_PKG_VERSION"),
                " : {about-with-newline}\n{usage-heading} {usage}\n\n{all-args}{after-help}"
            ))
            .mut_subcommands(banner)
    }
    banner(Cli::command())
}

#[derive(Subcommand)]
enum Command {
    /// List one uploader's local grants, distinct roots and application references.
    Overview(overview::Input),
    /// Inspect your application catalog or prepare fresh installation inputs.
    #[command(subcommand, name = "app")]
    Application(application::Command),
    /// Inspect Caffeine accounts and prepare enrollment proposals.
    #[command(subcommand)]
    Provider(ProviderCommand),
    /// Prepare, dispatch or recover an upload certificate request.
    #[command(subcommand)]
    Certificate(CertificateCommand),
    /// Claim a transfer using an existing certificate.
    #[command(subcommand)]
    Transfer(TransferCommand),
    /// Advanced: inspect a host and manage its upgrade lifecycle.
    #[command(subcommand)]
    Maintenance(MaintenanceCommand),
}

#[derive(Subcommand)]
enum ProviderCommand {
    /// Query storage and balance for one exact Caffeine account principal.
    Inspect(provider::Inspect),
    /// Print an offline enrollment proposal; does not sign, submit or spend.
    PrepareEnrollment(provider::Enrollment),
}

#[derive(Subcommand)]
#[command(
    after_help = "Workflow: prepare -> dispatch -> recover. Recover observes the original request; it never resends it."
)]
enum CertificateCommand {
    /// Retain an original signed request offline in permanent custody.
    Prepare(certificate::custody::Input),
    /// Dispatch the retained request once.
    Dispatch(certificate::custody::Dispatch),
    /// Recover the original certified reply without issuing or resending.
    Recover(certificate::Input),
}

#[derive(Subcommand)]
enum TransferCommand {
    /// Authenticate saved proof and permanently claim one transfer offline.
    Claim(certificate::transfer::Input),
}

#[derive(Subcommand)]
#[command(
    after_help = "Lifecycle: drain -> stop -> upgrade -> start -> activate.\nFor each stage: prepare, dispatch once, then observe the original request."
)]
enum MaintenanceCommand {
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
    /// Retain a signed stop request for the exact current drained attempt.
    PrepareStop {
        #[command(flatten)]
        target: Target,
        /// New immutable claim directory under a trusted, existing parent.
        #[arg(long)]
        record: PathBuf,
        /// Exact current closed maintenance attempt.
        #[arg(long)]
        attempt: std::num::NonZeroU64,
    },
    /// Submit the original stop under the same target lock used for drain.
    DispatchStop {
        #[command(flatten)]
        target: Target,
        #[arg(long)]
        record: PathBuf,
        /// Single trusted custody root used by all managers of this target.
        #[arg(long)]
        custody: PathBuf,
        /// Trusted Blob checkout with this binary's admitted tooling snapshot.
        #[arg(long)]
        checkout: PathBuf,
    },
    /// Observe original stop status and authenticated platform state; never restart.
    ObserveStop {
        #[command(flatten)]
        target: Target,
        #[arg(long)]
        record: PathBuf,
    },
    /// Retain an ordinary upgrade request chained to an acknowledged stop.
    PrepareUpgrade {
        #[command(flatten)]
        input: UpgradeInput,
        /// Exact stopped maintenance attempt.
        #[arg(long)]
        attempt: std::num::NonZeroU64,
    },
    /// Submit the original upgrade once; never reinstall or fall back after failure.
    DispatchUpgrade {
        #[command(flatten)]
        input: UpgradeInput,
        /// Same trusted custody root used for the preceding stop.
        #[arg(long)]
        custody: PathBuf,
        #[arg(long)]
        checkout: PathBuf,
    },
    /// Authenticate the original upgrade outcome and replacement artifact while stopped.
    ObserveUpgrade(UpgradeInput),
    /// Retain a start request chained to acknowledged stop and upgrade requests.
    PrepareStart {
        #[command(flatten)]
        input: StartInput,
        #[arg(long)]
        attempt: std::num::NonZeroU64,
    },
    /// Start the admitted upgraded host once and verify it remains drained.
    DispatchStart {
        #[command(flatten)]
        input: StartInput,
        #[arg(long)]
        custody: PathBuf,
        #[arg(long)]
        checkout: PathBuf,
    },
    /// Recover the original start outcome and verify the current reopened host.
    ObserveStart(StartInput),
    /// Retain explicit activation for an acknowledged start and current drained attempt.
    PrepareActivate {
        #[command(flatten)]
        input: ActivationInput,
        #[arg(long)]
        attempt: std::num::NonZeroU64,
    },
    /// Recheck verified reopening and submit the original activation once.
    DispatchActivate {
        #[command(flatten)]
        input: ActivationInput,
        #[arg(long)]
        custody: PathBuf,
        #[arg(long)]
        checkout: PathBuf,
    },
    /// Observe original activation and current admission without retrying.
    ObserveActivate(ActivationInput),
}

#[derive(Args)]
struct ActivationInput {
    #[command(flatten)]
    start: StartInput,
    /// Original acknowledged start request directory.
    #[arg(long)]
    start_record: PathBuf,
}

#[derive(Args)]
struct StartInput {
    #[command(flatten)]
    upgrade: UpgradeInput,
    /// Original acknowledged upgrade request directory.
    #[arg(long)]
    upgrade_record: PathBuf,
}

#[derive(Args)]
struct UpgradeInput {
    #[command(flatten)]
    target: Target,
    /// Operation claim directory: new for preparation, original for dispatch/observation.
    #[arg(long)]
    record: PathBuf,
    /// Original acknowledged stop request directory for this same target/attempt.
    #[arg(long)]
    stop_record: PathBuf,
    /// Independently reviewed replacement Wasm, uncompressed and at most 1 MiB.
    #[arg(long)]
    next_wasm: PathBuf,
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
    /// Independently trusted subnet corresponding to the root key.
    #[arg(long)]
    root_subnet: Principal,
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
    client: Client,
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
    // Ordinary operator inputs historically follow links and accept relative
    // paths. Resolve once, then read a held regular file. Custody records instead
    // use files::read_at on their already admitted directory without this step.
    let resolved = path
        .canonicalize()
        .map_err(|_| format!("cannot resolve input file: {}", path.display()))?;
    let (parent, name) = files::parent(&resolved)?;
    files::read_at(&parent, name, limit)
        .map_err(|e| format!("cannot read bounded regular file {}: {e}", path.display()))
}

impl Context {
    fn new(target: Target) -> Result<Self, String> {
        let Target {
            url,
            canister,
            identity,
            root_key,
            root_subnet,
            installation,
            wasm,
            state_version,
        } = target;
        if canister == Principal::anonymous() || canister == Principal::management_canister() {
            return Err("expected an installed Blob canister".into());
        }
        let configuration: Installation = ic_wire::decode(
            &read(&installation, RPC_LIMITS.bytes)?,
            ic_wire::Encoding::Frame {
                method: ic_wire::method_id("blob_init"),
                direction: ic_wire::Direction::Request,
            },
            RPC_LIMITS,
        )
        .map_err(|_| "invalid expected installation frame")?;
        let pem = read(&identity, 16 * 1024)?;
        let wasm_hash = Sha256::digest(read(&wasm, 16 * 1024 * 1024)?).into();
        let key = read(&root_key, 1024)?;
        let root_hash = Sha256::digest(&key).into();
        let client = Client::new(url.as_str(), key, root_subnet, Some(&pem))?;
        if client.principal() != configuration.manager {
            return Err("identity does not match the expected manager".into());
        }
        Ok(Self {
            client,
            url: url.into(),
            canister,
            configuration,
            wasm_hash,
            root_hash,
            state_version,
        })
    }

    async fn inspect(&self) -> Result<ManagementView, String> {
        self.inspect_artifact(&self.wasm_hash).await
    }

    async fn inspect_artifact(&self, expected: &[u8; 32]) -> Result<ManagementView, String> {
        let target = self.canister;
        let request = ic_wire::encode(
            &(),
            ic_wire::Encoding::Frame {
                method: ic_wire::method_id("blob_management"),
                direction: ic_wire::Direction::Request,
            },
            RPC_LIMITS,
        )
        .map_err(|_| "cannot encode inspection")?;
        let observed = self
            .client
            .transport
            .canister_module_hash(
                target,
                network::now()?.saturating_add(120_000_000_000),
                network::policy(false)?,
            )
            .await
            .map_err(|_| "module hash authentication failed")?;
        if observed != *expected {
            return Err("installed Wasm does not match the admitted artifact".into());
        }
        let bytes = self
            .client
            .update(target, target, "blob_management", &request)
            .await
            .map_err(|_| "management inspection failed; no state-changing retry was attempted")?;
        let result: Result<ManagementView, Error> = ic_wire::decode(
            &bytes,
            ic_wire::Encoding::Frame {
                method: ic_wire::method_id("blob_management"),
                direction: ic_wire::Direction::Response,
            },
            RPC_LIMITS,
        )
        .map_err(|_| "invalid management reply")?;
        let view = result.map_err(|_| "host refused management inspection")?;
        if view.installation.as_slice() != self.canister.as_slice()
            || view.configuration != self.configuration
            || view.state_version != self.state_version
        {
            return Err("installed identity, configuration or state format mismatch".into());
        }
        // Authenticated observations are not atomic and do not establish continuity.
        let observed = self
            .client
            .transport
            .canister_module_hash(
                target,
                network::now()?.saturating_add(120_000_000_000),
                network::policy(false)?,
            )
            .await
            .map_err(|_| "module hash authentication failed")?;
        if observed != *expected {
            return Err("installed Wasm changed during inspection".into());
        }
        Ok(view)
    }
}

async fn run(command: Command) -> Result<String, String> {
    match command {
        Command::Overview(input) => overview::run(input).await,
        Command::Application(command) => application::run(command).await,
        Command::Provider(ProviderCommand::Inspect(input)) => provider::inspect(input).await,
        Command::Provider(ProviderCommand::PrepareEnrollment(input)) => provider::prepare(&input),
        Command::Certificate(CertificateCommand::Recover(input)) => certificate::run(input).await,
        Command::Certificate(CertificateCommand::Prepare(input)) => {
            certificate::custody::prepare(&input)
        }
        Command::Certificate(CertificateCommand::Dispatch(input)) => {
            certificate::custody::dispatch(input).await
        }
        Command::Transfer(TransferCommand::Claim(input)) => certificate::transfer::claim(&input),
        Command::Maintenance(command) => run_maintenance(command).await,
    }
}

async fn run_maintenance(command: MaintenanceCommand) -> Result<String, String> {
    match command {
        MaintenanceCommand::Inspect(target) => {
            let view = Context::new(target)?.inspect().await?;
            Ok(format!(
                "Inspection matched: installation={} state_version={} attempt={} active={}\nAuthenticated observations only; no continuity or activation approval.",
                view.installation,
                view.state_version,
                view.maintenance.attempt,
                view.maintenance.active
            ))
        }
        MaintenanceCommand::PrepareDrain {
            target,
            record,
            attempt,
        } => {
            management::prepare(
                &Context::new(target)?,
                &record,
                attempt.get(),
                management::Operation::Drain,
            )
            .await
        }
        MaintenanceCommand::ObserveDrain { target, record } => {
            management::observe(
                &Context::new(target)?,
                &record,
                management::Operation::Drain,
            )
            .await
        }
        MaintenanceCommand::DispatchDrain {
            target,
            record,
            custody,
            checkout,
        } => {
            management::admit_tooling(&checkout).await?;
            management::dispatch(
                &Context::new(target)?,
                &record,
                &custody,
                management::Operation::Drain,
            )
            .await
        }
        MaintenanceCommand::PrepareStop {
            target,
            record,
            attempt,
        } => {
            management::prepare(
                &Context::new(target)?,
                &record,
                attempt.get(),
                management::Operation::Stop,
            )
            .await
        }
        MaintenanceCommand::DispatchStop {
            target,
            record,
            custody,
            checkout,
        } => {
            management::admit_tooling(&checkout).await?;
            management::dispatch(
                &Context::new(target)?,
                &record,
                &custody,
                management::Operation::Stop,
            )
            .await
        }
        MaintenanceCommand::ObserveStop { target, record } => {
            management::observe(&Context::new(target)?, &record, management::Operation::Stop).await
        }
        MaintenanceCommand::PrepareUpgrade { input, attempt } => {
            let wasm = read(&input.next_wasm, 1024 * 1024)?;
            management::prepare(
                &Context::new(input.target)?,
                &input.record,
                attempt.get(),
                management::Operation::Upgrade {
                    wasm: &wasm,
                    stop_record: &input.stop_record,
                },
            )
            .await
        }
        MaintenanceCommand::DispatchUpgrade {
            input,
            custody,
            checkout,
        } => {
            management::admit_tooling(&checkout).await?;
            let wasm = read(&input.next_wasm, 1024 * 1024)?;
            management::dispatch(
                &Context::new(input.target)?,
                &input.record,
                &custody,
                management::Operation::Upgrade {
                    wasm: &wasm,
                    stop_record: &input.stop_record,
                },
            )
            .await
        }
        MaintenanceCommand::ObserveUpgrade(input) => {
            let wasm = read(&input.next_wasm, 1024 * 1024)?;
            management::observe(
                &Context::new(input.target)?,
                &input.record,
                management::Operation::Upgrade {
                    wasm: &wasm,
                    stop_record: &input.stop_record,
                },
            )
            .await
        }
        MaintenanceCommand::PrepareStart { input, attempt } => {
            let wasm = read(&input.upgrade.next_wasm, 1024 * 1024)?;
            management::prepare(
                &Context::new(input.upgrade.target)?,
                &input.upgrade.record,
                attempt.get(),
                management::Operation::Start {
                    wasm: &wasm,
                    stop_record: &input.upgrade.stop_record,
                    upgrade_record: &input.upgrade_record,
                },
            )
            .await
        }
        MaintenanceCommand::DispatchStart {
            input,
            custody,
            checkout,
        } => {
            management::admit_tooling(&checkout).await?;
            let wasm = read(&input.upgrade.next_wasm, 1024 * 1024)?;
            management::dispatch(
                &Context::new(input.upgrade.target)?,
                &input.upgrade.record,
                &custody,
                management::Operation::Start {
                    wasm: &wasm,
                    stop_record: &input.upgrade.stop_record,
                    upgrade_record: &input.upgrade_record,
                },
            )
            .await
        }
        MaintenanceCommand::ObserveStart(input) => {
            let wasm = read(&input.upgrade.next_wasm, 1024 * 1024)?;
            management::observe(
                &Context::new(input.upgrade.target)?,
                &input.upgrade.record,
                management::Operation::Start {
                    wasm: &wasm,
                    stop_record: &input.upgrade.stop_record,
                    upgrade_record: &input.upgrade_record,
                },
            )
            .await
        }
        MaintenanceCommand::PrepareActivate { input, attempt } => {
            let wasm = read(&input.start.upgrade.next_wasm, 1024 * 1024)?;
            management::prepare(
                &Context::new(input.start.upgrade.target)?,
                &input.start.upgrade.record,
                attempt.get(),
                management::Operation::Activate {
                    wasm: &wasm,
                    stop_record: &input.start.upgrade.stop_record,
                    upgrade_record: &input.start.upgrade_record,
                    start_record: &input.start_record,
                },
            )
            .await
        }
        MaintenanceCommand::DispatchActivate {
            input,
            custody,
            checkout,
        } => {
            management::admit_tooling(&checkout).await?;
            let wasm = read(&input.start.upgrade.next_wasm, 1024 * 1024)?;
            management::dispatch(
                &Context::new(input.start.upgrade.target)?,
                &input.start.upgrade.record,
                &custody,
                management::Operation::Activate {
                    wasm: &wasm,
                    stop_record: &input.start.upgrade.stop_record,
                    upgrade_record: &input.start.upgrade_record,
                    start_record: &input.start_record,
                },
            )
            .await
        }
        MaintenanceCommand::ObserveActivate(input) => {
            let wasm = read(&input.start.upgrade.next_wasm, 1024 * 1024)?;
            management::observe(
                &Context::new(input.start.upgrade.target)?,
                &input.start.upgrade.record,
                management::Operation::Activate {
                    wasm: &wasm,
                    stop_record: &input.start.upgrade.stop_record,
                    upgrade_record: &input.start.upgrade_record,
                    start_record: &input.start_record,
                },
            )
            .await
        }
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let command = Cli::from_arg_matches(&help_command().get_matches())
        .unwrap_or_else(|error| error.exit())
        .command;
    let read_only = matches!(
        command,
        Command::Overview(_)
            | Command::Application(_)
            | Command::Provider(_)
            | Command::Certificate(CertificateCommand::Recover(_) | CertificateCommand::Prepare(_))
            | Command::Transfer(_)
    );
    match tokio::time::timeout(Duration::from_secs(45), run(command)).await {
        Ok(Ok(message)) => {
            println!("{message}");
            ExitCode::SUCCESS
        }
        result => {
            let message = match result {
                Ok(Err(message)) => message,
                Err(_) if read_only => "inspection timed out; storage is unknown; no mutation was submitted".into(),
                Err(_) => "operation timed out; dispatch may have occurred. Preserve custody and observe the original request; do not resubmit".into(),
                Ok(Ok(_)) => unreachable!(),
            };
            eprintln!("Command failed: {message}");
            ExitCode::FAILURE
        }
    }
}
