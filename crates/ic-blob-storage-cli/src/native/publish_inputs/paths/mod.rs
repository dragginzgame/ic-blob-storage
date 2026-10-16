//! One admitted input-root capability; Blob owns names, budgets and artifact bindings.
use super::Failure;
use crate::native::local_body::BodySource;
use std::{
    fs::File,
    path::{Path, PathBuf},
    sync::Arc,
};

pub(super) struct Root {
    path: PathBuf,
    directory: Arc<File>,
}
impl Root {
    pub fn new(path: &Path) -> Result<Self, Failure> {
        let path = path.canonicalize().map_err(|_| Failure::File)?;
        #[cfg(unix)]
        let directory =
            Arc::new(ic_host_fs::read::open_directory_no_follow(&path).map_err(|_| Failure::File)?);
        #[cfg(not(unix))]
        let directory: Arc<File> = {
            let _ = path;
            return Err(Failure::File);
        };
        Ok(Self { path, directory })
    }
    pub fn source(&self, name: &str) -> Result<BodySource, Failure> {
        if name.is_empty() || name.len() > 1024 {
            return Err(Failure::Arguments);
        }
        Ok(BodySource::Beneath {
            root: Arc::clone(&self.directory),
            relative: name.into(),
            display: self.path.join(name),
        })
    }
    pub fn read(&self, name: &str, maximum: u64) -> Result<Vec<u8>, Failure> {
        let file = self.source(name)?.file()?;
        let maximum = usize::try_from(maximum).map_err(|_| Failure::File)?;
        let bytes = ic_host_fs::read::read_opened_file(file, maximum).map_err(|_| Failure::File)?;
        if bytes.is_empty() {
            return Err(Failure::File);
        }
        Ok(bytes)
    }
}

#[cfg(all(test, unix))]
mod tests;
