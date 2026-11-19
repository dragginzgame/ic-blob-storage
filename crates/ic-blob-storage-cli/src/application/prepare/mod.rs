//! Encode the two existing init contracts before publishing an offline input directory.

#[cfg(test)]
mod tests;

use clap::Args;
use ic_blob_storage::{
    caffeine::ManifestLimits,
    local::Limits,
    protocol::{Installation as BlobInstallation, RPC_LIMITS},
};
use ic_blob_storage_reference_app::protocol::Installation;
use ic_platform_types::Principal;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    num::{NonZeroU32, NonZeroU64},
    path::PathBuf,
};

use crate::{
    files,
    overview::{hex, is_special_principal, principal},
};

#[derive(Args)]
#[command(
    after_help = "All limits are immutable local admission limits, not provider-enforced storage or cost caps. Both canister IDs must already be selected. The output is not a signed install request or proof of an empty target; installation and controller checks remain separate."
)]
pub(crate) struct Input {
    /// Proposed reference-application canister; becomes Blob's application authority.
    #[arg(long, value_parser = principal)]
    canister: Principal,
    /// Proposed standalone Blob canister; becomes the application's Blob target.
    #[arg(long, value_parser = principal)]
    blob_canister: Principal,
    /// Blob maintenance authority, separate from both canisters and all users.
    #[arg(long, value_parser = principal)]
    manager: Principal,
    /// Allowed uploader; repeat for one to eight distinct users.
    #[arg(long, required = true, value_parser = principal)]
    user: Vec<Principal>,
    /// Maximum declared bytes in one upload.
    #[arg(long)]
    max_content_bytes: NonZeroU64,
    /// Maximum supplied manifest chunk hashes.
    #[arg(long)]
    max_chunks: NonZeroU32,
    /// Maximum raw metadata entries.
    #[arg(long)]
    max_headers: NonZeroU32,
    /// Maximum raw UTF-8 metadata bytes.
    #[arg(long)]
    max_header_bytes: NonZeroU32,
    /// Historical upload slots, including cancelled uploads (1–64).
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=64))]
    uploads: u8,
    /// Historical reference slots (1–64).
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=64))]
    references: u8,
    /// Maximum reserved plus live declared bytes.
    #[arg(long)]
    logical_bytes: NonZeroU64,
    /// Maximum reserved plus unresolved declared bytes.
    #[arg(long)]
    exposure_bytes: NonZeroU64,
    /// New output directory under an existing parent; never replaced or resumed.
    #[arg(long)]
    output: PathBuf,
}

fn frame<T: Serialize>(method: &str, input: &T) -> Result<Vec<u8>, String> {
    ic_wire::encode(
        input,
        ic_wire::Encoding::Frame {
            method: ic_wire::method_id(method),
            direction: ic_wire::Direction::Request,
        },
        RPC_LIMITS,
    )
    .map_err(|_| "cannot encode installation input".into())
}

pub(super) fn validate_roles(
    canister: Principal,
    blob_canister: Principal,
    manager: Principal,
    users: &[Principal],
) -> Result<(), String> {
    let authorities = [canister, blob_canister, manager];
    // This is a proposed deployment arrangement; neither IDs nor output authenticate a host.
    if authorities
        .iter()
        .enumerate()
        .any(|(i, p)| is_special_principal(*p) || authorities[..i].contains(p))
        || users.is_empty()
        || users.len() > 8
        || users.iter().enumerate().any(|(i, p)| {
            is_special_principal(*p) || authorities.contains(p) || users[..i].contains(p)
        })
    {
        return Err("canisters, manager and one to eight users must be distinct".into());
    }
    Ok(())
}

pub(crate) fn run(input: Input) -> Result<String, String> {
    validate_roles(
        input.canister,
        input.blob_canister,
        input.manager,
        &input.user,
    )?;
    let blob = BlobInstallation {
        application: input.canister,
        manager: input.manager,
        limits: Limits {
            manifest: ManifestLimits {
                max_content_bytes: input.max_content_bytes,
                max_chunks: input.max_chunks.try_into().unwrap(),
                max_headers: input.max_headers.try_into().unwrap(),
                max_header_bytes: input.max_header_bytes.try_into().unwrap(),
            },
            uploads: usize::from(input.uploads).try_into().unwrap(),
            references: usize::from(input.references).try_into().unwrap(),
            logical_bytes: input.logical_bytes,
            exposure_bytes: input.exposure_bytes,
        },
    };
    let app = Installation {
        blob: input.blob_canister,
        users: input.user,
    };
    let blob_bytes = frame("blob_init", &blob)?;
    let app_bytes = frame("app_init", &app)?;
    let report = format!(
        "Offline fresh-install inputs (Blob {})\nApplication canister: {}\nBlob canister: {}\nManager: {}\nUsers: {:?}\nBlob limits: {:?}\nblob-init.wire SHA-256: {}\napp-init.wire SHA-256: {}\n\nNo network or signing. Targets, controllers, emptiness, Wasm and deployment are not authenticated. These are init arguments, not signed requests or replay protection. Never use reinstall to apply them to existing state. Provider acceptance and charge limits remain unqualified.\n",
        env!("CARGO_PKG_VERSION"),
        input.canister,
        input.blob_canister,
        input.manager,
        app.users.iter().map(Principal::to_text).collect::<Vec<_>>(),
        blob.limits,
        hex(&Sha256::digest(&blob_bytes)),
        hex(&Sha256::digest(&app_bytes)),
    );
    // Encode everything before claiming the directory. A partial publication stays occupied.
    let output = files::claim(&input.output)?;
    files::publish(&output, "blob-init.wire".as_ref(), &blob_bytes)?;
    files::publish(&output, "app-init.wire".as_ref(), &app_bytes)?;
    files::publish(&output, "inputs.txt".as_ref(), report.as_bytes())?;
    Ok(report)
}
