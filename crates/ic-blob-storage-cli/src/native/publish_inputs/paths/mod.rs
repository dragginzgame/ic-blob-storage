//! Inventory paths must select regular files beneath the explicit input root.
use super::Failure;
use std::path::{Component, Path, PathBuf};

pub(super) fn resolve(root: &Path, name: &str) -> Result<PathBuf, Failure> {
    if name.is_empty() || name.len() > 1024 {
        return Err(Failure::Arguments);
    }
    let mut path = root.to_owned();
    for component in Path::new(name).components() {
        let Component::Normal(part) = component else {
            return Err(Failure::Arguments);
        };
        path.push(part);
        if std::fs::symlink_metadata(&path)
            .map_err(|_| Failure::File)?
            .file_type()
            .is_symlink()
        {
            return Err(Failure::Arguments);
        }
    }
    let canonical = path.canonicalize().map_err(|_| Failure::File)?;
    if !canonical.starts_with(root) || !canonical.is_file() {
        return Err(Failure::Arguments);
    }
    Ok(canonical)
}
