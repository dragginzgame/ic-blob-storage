//! Publish only the file that received the checked bytes, never a reread of input.
//!
//! The destination parent must be a caller-controlled directory, without a
//! concurrent path replacer or temporary-file cleaner. Host's private staging
//! file holds unverified bytes; ordinary failures remove it. Crash residue is not
//! resumable evidence and is never loaded or cleaned up by a later invocation.
//!
//! Host synchronizes the checked file, publishes without replacement and syncs
//! the parent directory. A completion error can leave the verified output visible
//! with unconfirmed directory durability; preserve that evidence and reconcile.
//! No existing target, including a dangling symlink, is replaced. Publication
//! precedes the stdout receipt, so a lost receipt requires inspecting/reverifying
//! the output, not blindly overwriting it.

use super::BodyError;
use super::CaffeineContentHashes;
use super::CaffeineRootVerifier;
use super::Read;
use super::io;
use super::verify_body;
use ic_host_fs::durable::{
    NamedWriteError, PublicationMode, WriteOptions, write_existing_parent_validated_with,
};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub(super) enum OutputError {
    #[error("output must name a file in an existing caller-controlled directory")]
    InvalidDestination,
    #[error("cannot create staging area: {0}")]
    Create(io::Error),
    #[error(transparent)]
    Body(#[from] BodyError),
    #[error(transparent)]
    Publication(NamedWriteError<BodyError>),
}

pub(super) fn save_verified(
    body: impl Read,
    verifier: CaffeineRootVerifier,
    destination: &Path,
) -> Result<CaffeineContentHashes, OutputError> {
    let name = destination
        .file_name()
        .ok_or(OutputError::InvalidDestination)?;
    let parent = destination
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
        .canonicalize()
        .map_err(OutputError::Create)?;
    let destination = parent.join(name);
    write_existing_parent_validated_with(
        &destination,
        WriteOptions {
            mode: PublicationMode::CreateNew,
            permissions: 0o600,
        },
        |file| verify_body(body, verifier, file),
        |_| Ok(()),
    )
    .map_err(|error| match error {
        NamedWriteError::Producer {
            source,
            cleanup_error: None,
        } => OutputError::Body(source),
        other => OutputError::Publication(other),
    })
}

#[cfg(test)]
mod tests;
