//! Owner-authenticated reference-application observations; no publication or cleanup effects.

use clap::{Args, Subcommand};
use ic_blob_storage::protocol::RPC_LIMITS;
use ic_blob_storage_reference_app::protocol::{DraftView, Error, Phase};
use ic_platform::{Principal, agent::Agent};
use serde::{Serialize, de::DeserializeOwned};
use std::{fmt::Write, num::NonZeroU128};

use crate::overview::{Network, hex, principal};

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
    after_help = "Use the owning user's --identity. These queries do not publish, cancel, upload or contact Caffeine."
)]
pub(crate) enum Command {
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
    agent: &Agent,
    canister: Principal,
    method: &str,
    input: &T,
) -> Result<R, String> {
    let request = ic_wire::encode(
        input,
        ic_wire::Encoding::Frame {
            method: ic_wire::method_id(method),
            direction: ic_wire::Direction::Request,
        },
        RPC_LIMITS,
    )
    .map_err(|_| "cannot encode application inspection")?;
    let canister = canister
        .as_slice()
        .try_into()
        .map_err(|_| "invalid canister")?;
    let reply = agent
        .query(&canister, method)
        .with_arg(request)
        .call()
        .await
        .map_err(|error| format!("Application query failed: {:?}", error.to_string()))?;
    let result: Result<R, Error> = ic_wire::decode(
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
        Command::Catalog(target) => (target, None),
        Command::Draft { target, id } => (target, Some(id)),
    };
    let agent = target.network.agent()?;
    let caller = agent
        .get_principal()
        .map_err(|_| "identity has no sender")?;
    if caller.as_slice() == Principal::anonymous().as_slice() {
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
        || views.iter().any(|view| {
            view.uploader.as_slice() != caller.as_slice()
                || (id.is_none() && view.phase != Phase::Published)
        })
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
