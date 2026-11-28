//! Reference-application observations and explicitly selected installation operations.

mod install;
mod prepare;
mod targets;

use crate::{
    network::Client,
    overview::{Network, hex, principal},
};
use clap::{Args, Subcommand};
use ic_blob_storage_reference_app::protocol::{DraftView, Error, Phase};
use ic_blob_storage_wire::RPC_LIMITS;
use ic_platform_types::Principal;
use serde::{Serialize, de::DeserializeOwned};
use std::{fmt::Write, num::NonZeroU128};

#[derive(Args)]
#[command(mut_arg("identity", |arg| arg.required(true).help("Owning user's Ed25519 PKCS#8 PEM; required for application inspection.")))]
pub(crate) struct Target {
    #[command(flatten)]
    network: Network,
    /// Reference application canister, not its separate Blob host.
    #[arg(long, value_parser = principal)]
    canister: Principal,
}

#[derive(Subcommand)]
#[command(
    after_help = "Prepare-installation writes unsigned inputs. Prepare-install signs offline; inspect-install checks saved bytes without a private key. Only dispatch-install submits an installation. Observe-install recovers its original outcome and checks current status without resubmission. Remote operations require the appropriate user's or controller's --identity."
)]
pub(crate) enum Command {
    /// Prepare paired fresh-install arguments offline; does not create or install canisters.
    PrepareInstallation(prepare::Input),
    /// Sign and retain one fresh install offline; never submits or replaces a claim.
    #[command(mut_arg("identity", |arg| arg.required(true).help("Controller's Ed25519 PKCS#8 PEM for offline signing.")))]
    PrepareInstall(install::Input),
    /// Check retained install bytes against the selected artifacts and controller, offline.
    #[command(mut_arg("identity", |arg| arg.help("Unused by offline inspection; no private key is read.")))]
    InspectInstall(install::Input),
    /// Admit a fresh target and submit the retained installation once.
    DispatchInstall(install::execution::DispatchInput),
    /// Retain the original certified outcome and check current module/controller.
    #[command(mut_arg("identity", |arg| arg.required(true).help("Controller's Ed25519 PKCS#8 PEM for authenticated reads.")))]
    ObserveInstall(install::Input),
    /// Check two proposed targets have no module and the signer as sole controller.
    CheckTargets(targets::Input),
    /// List published drafts in your reference-application catalog.
    Catalog(Target),
    /// Inspect one original draft, including cancelled or cleaned records.
    Draft {
        #[command(flatten)]
        target: Target,
        /// Original nonzero draft ID; knowledge of it grants no permission.
        #[arg(long)]
        id: NonZeroU128,
    },
}

async fn query<T: Serialize, R: DeserializeOwned>(
    agent: &Client,
    canister: Principal,
    method: &str,
    input: &T,
) -> Result<R, String> {
    let request = ic_blob_storage_wire::frame::encode(
        input,
        ic_wire::Encoding::Frame {
            method: ic_wire::method_id(method),
            direction: ic_wire::Direction::Request,
        },
        RPC_LIMITS,
    )
    .map_err(|_| "cannot encode application inspection")?;
    let reply = agent
        .query(canister, method, &request)
        .await
        .map_err(|error| format!("Application query failed: {:?}", error))?;
    let result: Result<R, Error> = ic_blob_storage_wire::frame::decode(
        &reply,
        ic_wire::Encoding::Frame {
            method: ic_wire::method_id(method),
            direction: ic_wire::Direction::Response,
        },
        RPC_LIMITS,
    )
    .map_err(|_| "invalid reference-application reply (target may not support this command)")?;

    result.map_err(|error| format!("Application inspection refused: {error:?}"))
}

pub(crate) async fn run(command: Command) -> Result<String, String> {
    let (target, id) = match command {
        Command::PrepareInstallation(input) => return prepare::run(input),
        Command::PrepareInstall(input) => return install::prepare(&input),
        Command::InspectInstall(input) => return install::inspect(&input),
        Command::DispatchInstall(input) => return install::execution::dispatch(input).await,
        Command::ObserveInstall(input) => return install::execution::observe(&input).await,
        Command::CheckTargets(input) => return targets::run(input).await,
        Command::Catalog(target) => (target, None),
        Command::Draft { target, id } => (target, Some(id)),
    };
    let agent = target.network.client()?;
    let caller = agent.principal();
    if caller == Principal::anonymous() {
        return Err(
            "reference-application inspection requires the owning user's --identity".into(),
        );
    }
    let views: Vec<DraftView> = if let Some(id) = id {
        let view: DraftView = query(&agent, target.canister, "app_draft", &id).await?;
        if view.input.id != id {
            return Err("application draft ID mismatch".into());
        }
        vec![view]
    } else {
        query(&agent, target.canister, "app_catalog", &()).await?
    };
    if views.len() > 16
        || views
            .iter()
            .any(|view| view.uploader != caller || (id.is_none() && view.phase != Phase::Published))
    {
        return Err("application reply differs from the requested owner or catalog".into());
    }
    let mut output = format!(
        "Reference application: {}\nUser: {}\nSource: {}\n{}: {}\n",
        target.canister,
        caller,
        target.network.url,
        if id.is_some() {
            "Draft records"
        } else {
            "Published catalog entries"
        },
        views.len()
    );
    for view in views {
        let disposition = match view.phase {
            Phase::Draft => "Draft (unpublished)",
            Phase::Published => "Published (in application catalog)",
            Phase::Cancelled => "Cancelled (cleanup pending)",
            Phase::Cleaned => "Cleaned (application cleanup finished)",
        };
        writeln!(output,
            "\nDraft {}\n  Asset: {:?}\n  State: {}\n  Root: sha256:{}\n  Declared bytes: {}\n  First reference: {}",
            view.input.id, view.input.asset, disposition, hex(&view.input.root), view.input.bytes,
            view.input.id).unwrap();
    }
    output.push_str("\nQuery observation, not consensus-certified evidence. Catalog publication does not prove provider availability; Cleaned does not prove deletion or settlement.");

    Ok(output)
}
