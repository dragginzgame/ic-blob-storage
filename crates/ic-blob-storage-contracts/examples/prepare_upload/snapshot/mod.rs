//! Save the exact hashed buffers, independently of subsequent source changes.
//!
//! The source tree and destination parent must remain caller-controlled. A fresh
//! private directory owns every created file; ordinary failure removes it. A
//! crash can leave incomplete residue, which is never reused or auto-discovered.
//! Host publishes each checked body durably. The report is written last and
//! synced, but the snapshot root/parent are not committed together: this is not
//! a crash-durable multi-file transaction or resume journal.
//! Kept files remain mutable local data; reverify them before future effects.

use super::inventory;
use super::prepare_to;
use ic_blob_storage_contracts::upload::inventory::InventoryReport;
use ic_host_fs::directory::create_unique_directory;
#[cfg(unix)]
use ic_host_fs::durable::{create_retained_at, publish_retained_at};
use ic_host_fs::owned_directory::OwnedDirectory;
use serde::Serialize;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub(super) struct SavedSnapshot {
    directory: PathBuf,
    inventory: InventoryReport,
}

pub(super) fn run(input: &Path, parent: &Path) -> Result<SavedSnapshot, inventory::InventoryError> {
    #[cfg(not(unix))]
    {
        let _ = (input, parent);
        Err(std::io::Error::from(std::io::ErrorKind::Unsupported).into())
    }
    #[cfg(unix)]
    {
        use std::os::fd::AsFd as _;
        let (declarations, base) = inventory::load(input)?;
        let parent = parent.canonicalize()?;
        let stage = OwnedDirectory::new(create_unique_directory(
            &parent,
            "blob-snapshot",
            0o700,
            std::num::NonZeroUsize::new(128),
        )?);
        let bodies = stage.path().join("bodies");
        std::fs::create_dir(&bodies)?;
        let bodies_directory = File::open(&bodies)?;
        let pending = std::ffi::OsStr::new("pending");
        let mut stored = BTreeSet::new();
        let inventory =
            inventory::prepare_with(declarations, &base, |body, declaration, limits| {
                let mut file = create_retained_at(bodies_directory.as_fd(), pending, 0o600)?;
                let prepared = prepare_to(body, declaration, limits, &mut file)?;
                if stored.insert(prepared.claim.root.clone()) {
                    // Only internally computed roots reach this path, never user strings.
                    let name = prepared
                        .claim
                        .root
                        .strip_prefix("sha256:")
                        .expect("computed SHA-256 root");
                    publish_retained_at(
                        bodies_directory.as_fd(),
                        pending,
                        std::ffi::OsStr::new(name),
                        file,
                    )
                    .map_err(std::io::Error::other)?;
                } else {
                    drop(file);
                    std::fs::remove_file(bodies.join(pending))?;
                }
                Ok(prepared)
            })?;
        let mut report = File::create_new(stage.path().join("inventory.json"))?;
        serde_json::to_writer(&mut report, &inventory)?;
        writeln!(report)?;
        report.sync_all()?;
        drop(report);
        // Keep precedes the stdout receipt. Lost stdout may leave a complete snapshot;
        // a repeat creates a new directory, never overwrites or resumes the old one.
        let directory = stage.path().to_path_buf();
        stage.preserve();
        Ok(SavedSnapshot {
            directory,
            inventory,
        })
    }
}

#[cfg(test)]
mod tests;
