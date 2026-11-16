//! Observe proposed targets without creating, installing or changing their controllers.

use clap::Args;
use ic_platform_protocol::management_snapshots::Status as CanisterStatusType;
use ic_platform_types::Principal;
use std::{fmt::Write, time::Duration};

use crate::{
    management::status,
    network::Client,
    overview::{Network, principal},
};

pub(super) async fn admit_fresh(
    agent: &Client,
    canister: Principal,
) -> Result<CanisterStatusType, String> {
    let observed = status::read(agent, canister).await?;
    if observed.module_hash.is_some() {
        return Err(
            "target already has an installed module; no install or reinstall authorized".into(),
        );
    }
    if observed.settings.controllers.as_slice() != [agent.principal()] {
        return Err("target requires the signer as sole controller".into());
    }
    if observed.status == CanisterStatusType::Stopping {
        return Err("target is stopping; disposition remains unresolved".into());
    }
    Ok(observed.status)
}

#[derive(Args)]
#[command(
    mut_arg("identity", |arg| arg.required(true).help("Controller's Ed25519 PKCS#8 PEM; required for both target status calls.")),
    after_help = "Authenticated observations of two targets, not an atomic reservation or installation approval. No module does not prove a canister was never used. Requires custody of freshly created targets; install/reinstall, creation and funding are not performed."
)]
pub(crate) struct Input {
    #[command(flatten)]
    network: Network,
    /// Proposed reference-application canister.
    #[arg(long, value_parser = principal)]
    canister: Principal,
    /// Proposed standalone Blob canister; must differ from the application.
    #[arg(long, value_parser = principal)]
    blob_canister: Principal,
}

pub(crate) async fn run(input: Input) -> Result<String, String> {
    if input.canister == input.blob_canister {
        return Err("application and Blob targets must differ".into());
    }
    let agent = input.network.client()?;
    let signer = agent.principal();
    if signer == Principal::anonymous() {
        return Err("target status requires a controller identity".into());
    }
    tokio::time::timeout(Duration::from_secs(30), async {
        let mut report = format!("Target observations\nController: {signer}\nSource: {}\n", input.network.url);
        for (role, canister) in [("Application", input.canister), ("Blob", input.blob_canister)] {
            let observed = admit_fresh(&agent, canister).await.map_err(|error| format!("{role} {error}"))?;
            writeln!(report, "{role}: {canister} module=absent sole_controller=signer status={observed:?}").unwrap();
        }
        report.push_str("\nSeparate authenticated observations, not a reservation. A missing module does not establish fresh creation, empty history or absence of provider obligations. No install, reinstall, controller change, creation or funding performed; recheck targets before any future installation.");
        Ok(report)
    }).await.map_err(|_| "target status timed out; disposition is unknown".to_owned())?
}
