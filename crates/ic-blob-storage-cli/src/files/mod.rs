//! Blob's native custody policy over held Host directories; no cleanup or retry.

use ic_host_artifacts::Length;
use ic_host_fs::{Access, Commit, Destination, Directory, FilePolicy, Policy, Retention};
use std::{
    ffi::OsStr,
    path::{Component, Path},
};

// Existing ancestors remain caller-owned; newly prepared directories are private.
// Reject traversal before creating anything, then resolve each child without links.
pub(crate) fn directory(path: &Path, prepare: bool) -> Result<Directory, String> {
    let mut names = Vec::new();
    for component in path.components() {
        match component {
            Component::RootDir | Component::CurDir => {}
            Component::Normal(name)
                if !name
                    .as_encoded_bytes()
                    .iter()
                    .any(|b| matches!(b, 0 | 10 | 13)) =>
            {
                names.push(name)
            }
            _ => return Err("invalid custody directory component".into()),
        }
    }
    let root = if path.is_absolute() {
        Path::new("/")
    } else {
        Path::new(".")
    };
    let mut held = Directory::open(root, 0)
        .map_err(|e| e.to_string())?
        .ok_or("missing directory root")?;
    if prepare {
        return held
            .prepare_directories(&names, 0o700, 0)
            .map_err(|e| e.to_string());
    }
    for name in names {
        held = held
            .open_directory(name, 0)
            .map_err(|e| e.to_string())?
            .ok_or("missing custody directory")?;
    }
    Ok(held)
}

pub(crate) fn parent(path: &Path) -> Result<(Directory, &OsStr), String> {
    let name = path.file_name().ok_or("missing file name")?;
    let parent = directory(path.parent().ok_or("missing parent")?, false)?;
    Ok((parent, name))
}

pub(crate) fn claim(path: &Path) -> Result<Directory, String> {
    let (parent, name) = parent(path)?;
    parent
        .create_directory(name, 0o700, 0o077)
        .map_err(|e| e.to_string())
}

pub(crate) fn read_at(parent: &Directory, name: &OsStr, limit: usize) -> Result<Vec<u8>, String> {
    let file = parent
        .open_file(
            name,
            Access::Read,
            FilePolicy {
                forbidden: 0,
                single_link: false,
            },
        )
        .map_err(|e| e.to_string())?
        .ok_or("missing file")?;
    let (bytes, _) =
        ic_host_artifacts::read(file, Length::AtMost(limit as u64)).map_err(|e| e.to_string())?;
    if bytes.is_empty() {
        return Err("empty input".into());
    }
    Ok(bytes)
}

/// A deterministic retained stage also fences a failed/uncertain publication.
/// Existing output and stage names are never replaced or removed on refusal.
pub(crate) fn publish(parent: &Directory, name: &OsStr, bytes: &[u8]) -> Result<(), String> {
    // Refuse known occupied destinations without creating more recovery evidence.
    // CreateNew at commit still arbitrates a competing publisher after this check.
    if parent
        .open_file(
            name,
            Access::Read,
            FilePolicy {
                forbidden: 0,
                single_link: false,
            },
        )
        .map_err(|e| format!("cannot admit publication destination; preserve custody: {e}"))?
        .is_some()
    {
        return Err("publication destination already exists; preserve custody".into());
    }
    let mut staging = std::ffi::OsString::from(".");
    staging.push(name);
    staging.push(".pending");
    let stage = parent
        .stage(
            &staging,
            name,
            Policy {
                destination: Destination::CreateNew,
                permissions: 0o600,
                retention: Retention::Retain,
            },
        )
        .map_err(|e| format!("publication staging failed; preserve custody: {e:?}"))?;
    let sealed = stage
        .write_from(bytes, Length::Exactly(bytes.len() as u64))
        .and_then(ic_host_fs::Written::seal)
        .map_err(|e| format!("publication incomplete; preserve custody: {e:?}"))?;
    match sealed.commit() {
        Commit::Committed(_) => Ok(()),
        Commit::NotPublished { cause, .. } => {
            Err(format!("not published; preserve retained stage: {cause}"))
        }
        Commit::PublishedUnconfirmed { phase, cause, .. } => Err(format!(
            "publication visible but unconfirmed at {phase:?}; preserve custody, never retry: {cause}"
        )),
    }
}

#[cfg(test)]
mod tests;
