//! One claimed install dispatch; original certified outcomes survive later status failures.

#[cfg(test)]
mod tests;

use super::{Input, Selection, load};
use crate::{files, management, network, overview::hex};
use clap::Args;
use ic_host_fs::{Access, Directory, FilePolicy};
use ic_platform::native::{Dispatch, RequestObservation, RequestStatus};
use sha2::{Digest, Sha256};
use std::{path::PathBuf, time::Duration};

const CLAIM: &str = "dispatch.claim";
const OUTCOME: &str = "outcome.cbor";

#[derive(Args)]
#[command(mut_arg("identity", |arg| arg.required(true).help("Controller's Ed25519 PKCS#8 PEM for target admission.")))]
pub(crate) struct DispatchInput {
    #[command(flatten)]
    input: Input,
    /// Trusted Blob checkout with this binary's admitted tooling snapshot.
    #[arg(long)]
    checkout: PathBuf,
    /// Confirm retained creation records identify this target as newly created and never used.
    #[arg(long, required = true)]
    confirm_fresh_target: bool,
}

fn client(input: &Input, selected: &Selection) -> Result<network::Client, String> {
    let client = input.network.client()?;
    if client.principal() != selected.binding.controller
        || <[u8; 32]>::from(Sha256::digest(&client.root)) != selected.binding.root_hash
    {
        return Err("installation controller or root differs from original custody".into());
    }

    Ok(client)
}

fn present(held: &Directory, name: &str) -> Result<bool, String> {
    held.open_file(
        name.as_ref(),
        Access::Read,
        FilePolicy {
            forbidden: 0,
            single_link: false,
        },
    )
    .map(|file| file.is_some())
    .map_err(|_| "cannot inspect installation custody".into())
}

fn unexpired(expiry: u64) -> Result<(), String> {
    if expiry <= network::now()? {
        return Err("original installation expired; preserve custody, never renew".into());
    }

    Ok(())
}

fn retain(held: &Directory, observed: &RequestObservation) -> Result<(), String> {
    if matches!(
        observed.status(),
        RequestStatus::Replied(_) | RequestStatus::Rejected { .. }
    ) {
        // Retain even a rejection or malformed reply before interpreting it.
        // CreateNew preserves any concurrent/partial publication without repair.
        files::publish(held, OUTCOME.as_ref(), observed.certificate_bytes())?;
    }

    Ok(())
}

fn acknowledged(observed: &RequestObservation) -> Result<(), String> {
    match observed.status() {
        RequestStatus::Replied(bytes) => ic_platform_protocol::management::decode_empty_response(
            bytes,
            management::status::LIMITS,
        )
        .map_err(|_| {
            "original installation returned an invalid acknowledgment; preserve custody".into()
        }),
        RequestStatus::Rejected { .. } => Err(
            "original installation was rejected; preserve outcome and claim, no resubmission"
                .into(),
        ),
        RequestStatus::Received | RequestStatus::Processing => {
            Err("original installation is pending; observe it without resubmission".into())
        }
        RequestStatus::Unknown | RequestStatus::Done | RequestStatus::Unavailable => Err(
            "original installation outcome is unavailable; preserve uncertainty, no resubmission"
                .into(),
        ),
    }
}

pub(crate) async fn dispatch(input: DispatchInput) -> Result<String, String> {
    if !input.confirm_fresh_target {
        return Err("fresh-creation custody confirmation is required".into());
    }
    let selected = input.input.selection()?;
    let (held, original) = load(&input.input, &selected)?;
    if present(&held, CLAIM)? || present(&held, OUTCOME)? {
        return Err(
            "installation dispatch already claimed; observe the original, never resubmit".into(),
        );
    }
    unexpired(original.expiry())?;
    let client = client(&input.input, &selected)?;

    tokio::time::timeout(Duration::from_secs(30), async {
        management::admit_tooling(&input.checkout).await?;
        super::super::targets::admit_fresh(&client, selected.target).await?;
        unexpired(original.expiry())?;
        // Atomic cross-process arbitration before exposure. Crashes, timeouts and
        // publication failures keep the claim; absence of a reply never permits retry.
        files::publish(&held, CLAIM.as_ref(), original.request_id())?;
        match client.transport.dispatch(&original, network::policy(false)?).await {
            Ok(Dispatch::Observed(observed)) => {
                retain(&held, &observed)?;
                acknowledged(&observed)?;
                Ok(format!("Original installation acknowledged: request_id={}; outcome retained. Use observe-install for current module/controller readback.", hex(original.request_id())))
            }
            Ok(Dispatch::Accepted) => Err("installation submitted; original outcome pending, use observe-install without resubmission".into()),
            Ok(Dispatch::UncertifiedRejection { .. }) => Err("uncertified installation rejection; preserve claim and observe the original".into()),
            Err(_) => Err("installation dispatch outcome uncertain; preserve claim and observe the original, never resubmit".into()),
        }
    }).await.map_err(|_| "installation admission/dispatch timed out; preserve all custody, no automatic retry".to_owned())?
}

pub(crate) async fn observe(input: &Input) -> Result<String, String> {
    let selected = input.selection_with_format(true)?;
    let (held, original) = load(input, &selected)?;
    if files::read_at(&held, CLAIM.as_ref(), 32)?.as_slice() != original.request_id() {
        return Err("installation dispatch claim does not match the original request".into());
    }
    let client = client(input, &selected)?;

    tokio::time::timeout(Duration::from_secs(30), async {
        let observed = if present(&held, OUTCOME)? {
            let certificate = files::read_at(&held, OUTCOME.as_ref(), network::MESSAGE_BYTES)?;
            client.transport.authenticate_request(&original, &certificate, network::policy(true)?)
                .map_err(|_| "retained installation outcome authentication failed")?
        } else {
            let observed = client.observe(&original).await?;
            retain(&held, &observed)?;
            observed
        };
        acknowledged(&observed)?;
        // Current status is a separate authenticated observation. Failure here
        // cannot erase the original installation result already retained above.
        let current = management::status::read(&client, selected.target).await
            .map_err(|error| format!("original outcome retained; current status unavailable: {error}"))?;
        if current.module_hash.as_deref() != Some(selected.binding.wasm_hash.as_slice())
            || current.settings.controllers.as_slice() != [selected.binding.controller]
        {
            return Err("original installation acknowledged, but current module/controller differs; preserve outcome, no reinstall".into());
        }
        Ok(format!("Installation observed: request_id={} target={} original=acknowledged module_sha256={} sole_controller={} platform={:?}.\nExact init bytes are bound by the acknowledged request; this is not an independent application-configuration read or a continuity/provider guarantee.",
            hex(original.request_id()), selected.target, hex(&selected.binding.wasm_hash),
            selected.binding.controller, current.status))
    }).await.map_err(|_| "installation observation timed out; preserve outcome and claim, never resubmit".to_owned())?
}
