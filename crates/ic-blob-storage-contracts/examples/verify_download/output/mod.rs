//! Publish only the file that received the checked bytes, never a reread of input.
//!
//! The destination parent must be a caller-controlled directory, without a
//! concurrent path replacer or temporary-file cleaner. A private subdirectory
//! holds unverified bytes; normal failures drop it. Crash residue is not resumable
//! evidence and is never loaded or cleaned up by a later invocation.
//!
//! File contents are synced before no-clobber persistence. This is not a portable
//! crash-durable filesystem transaction: the parent directory is not synced,
//! and tempfile's platform fallback can leave an extra link. No existing target,
//! including a dangling symlink, is replaced. Publication precedes the stdout
//! receipt, so a lost receipt requires inspecting/reverifying the output, not
//! blindly overwriting it.

use super::BodyError;
use super::CaffeineContentHashes;
use super::CaffeineRootVerifier;
use super::Read;
use super::io;
use super::verify_body;
use std::path::Path;
use tempfile::Builder;
use tempfile::NamedTempFile;
use thiserror::Error;

#[derive(Debug, Error)]
pub(super) enum OutputError {
    #[error("output must name a file in an existing caller-controlled directory")]
    InvalidDestination,
    #[error("cannot create staging area: {0}")]
    Create(io::Error),
    #[error(transparent)]
    Body(#[from] BodyError),
    #[error("cannot sync verified file: {0}")]
    Sync(io::Error),
    #[error("cannot publish verified file without replacing output: {0}")]
    Publish(io::Error),
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
    let mut staging = Builder::new();
    staging.prefix(".blob-download-");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        staging.permissions(std::fs::Permissions::from_mode(0o700));
    }
    let stage = staging.tempdir_in(parent).map_err(OutputError::Create)?;
    let mut file = NamedTempFile::new_in(stage.path()).map_err(OutputError::Create)?;
    let hashes = verify_body(body, verifier, file.as_file_mut())?;
    file.as_file().sync_all().map_err(OutputError::Sync)?;
    // persist_noclobber owns the final existence check, including racing creators.
    file.persist_noclobber(destination)
        .map_err(|failure| OutputError::Publish(failure.error))?;
    Ok(hashes)
}

#[cfg(test)]
mod tests;
