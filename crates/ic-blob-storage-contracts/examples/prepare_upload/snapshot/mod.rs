//! Save the exact hashed buffers, independently of subsequent source changes.
//!
//! The source tree and destination parent must remain caller-controlled. A fresh
//! private directory owns every created file; ordinary failure removes it. A
//! crash can leave incomplete residue, which is never reused or auto-discovered.
//! The report is written last and files are synced, but directory entries are
//! not synced: this is not a portable crash-durable transaction or resume journal.
//! Kept files remain mutable local data; reverify them before future effects.

use super::inventory;
use super::prepare_to;
use serde::Serialize;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use tempfile::Builder;
use tempfile::NamedTempFile;

#[derive(Debug, Serialize)]
pub(super) struct SavedSnapshot {
    directory: PathBuf,
    inventory: inventory::InventoryReport,
}

pub(super) fn run(input: &Path, parent: &Path) -> Result<SavedSnapshot, inventory::InventoryError> {
    let (declarations, base) = inventory::load(input)?;
    let parent = parent.canonicalize()?;
    let mut builder = Builder::new();
    builder.prefix("blob-snapshot-");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        builder.permissions(std::fs::Permissions::from_mode(0o700));
    }
    let stage = builder.tempdir_in(parent)?;
    let bodies = stage.path().join("bodies");
    std::fs::create_dir(&bodies)?;
    let mut stored = BTreeSet::new();
    let inventory = inventory::prepare_with(declarations, &base, |body, declaration, limits| {
        let mut file = NamedTempFile::new_in(stage.path())?;
        let prepared = prepare_to(body, declaration, limits, file.as_file_mut())?;
        if stored.insert(prepared.claim.root.clone()) {
            file.as_file().sync_all()?;
            // Only internally computed roots reach this path, never user strings.
            let name = prepared
                .claim
                .root
                .strip_prefix("sha256:")
                .expect("computed SHA-256 root");
            file.persist_noclobber(bodies.join(name))
                .map_err(|failure| failure.error)?;
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
    Ok(SavedSnapshot {
        directory: stage.keep(),
        inventory,
    })
}

#[cfg(test)]
mod tests;
