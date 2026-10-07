//! Private create-new artifacts shared by native effect and download commands.
use super::Failure;
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(in crate::native) struct HttpResponseRecord {
    pub status: u16,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(in crate::native) struct DownloadOutcomeRecord<S> {
    pub received_bytes: String,
    pub outcome: S,
    pub error: Option<S>,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(in crate::native) struct FailureRecord<S> {
    pub error: S,
}
pub(in crate::native) struct Run {
    path: PathBuf,
}
impl Run {
    /// Allocate private unverified output without replacing any existing file.
    pub fn open_body(&self) -> Result<File, Failure> {
        let mut options = OpenOptions::new();
        options.create_new(true).write(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let file = options
            .open(self.path.join("body.part"))
            .map_err(|_| Failure::File)?;
        File::open(&self.path)
            .and_then(|f| f.sync_all())
            .map_err(|_| Failure::File)?;
        Ok(file)
    }
    /// Publish only after EOF/root verification and file sync, without replacement.
    pub fn publish_body(&self) -> Result<PathBuf, Failure> {
        let partial = self.path.join("body.part");
        let complete = self.path.join("body.bin");
        std::fs::hard_link(&partial, &complete).map_err(|_| Failure::File)?;
        File::open(&self.path)
            .and_then(|f| f.sync_all())
            .map_err(|_| Failure::File)?;
        std::fs::remove_file(partial).map_err(|_| Failure::File)?;
        File::open(&self.path)
            .and_then(|f| f.sync_all())
            .map_err(|_| Failure::File)?;
        Ok(complete)
    }
    pub fn create(path: &Path) -> Result<Self, Failure> {
        let mut builder = std::fs::DirBuilder::new();
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        builder.create(path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                Failure::ExistingRun
            } else {
                Failure::File
            }
        })?;
        let parent = path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        File::open(parent)
            .and_then(|f| f.sync_all())
            .map_err(|_| Failure::File)?;
        Ok(Self {
            path: path.to_owned(),
        })
    }
    pub fn bytes(&self, name: &str, bytes: &[u8]) -> Result<(), Failure> {
        let path = self.record_path(name)?;
        // Trusted ancestors and exclusion of concurrent directory writers remain
        // caller-owned, as for body.part. Complete records publish without replacement.
        ic_host_fs::durable::create_private_bytes_with_parents(&path, bytes)
            .map_err(|_| Failure::File)
    }
    fn record_path(&self, name: &str) -> Result<PathBuf, Failure> {
        // The shared writer can create parents. A run must already be claimed;
        // never recreate a removed run or follow a substituted directory link.
        if !std::fs::symlink_metadata(&self.path)
            .map_err(|_| Failure::File)?
            .is_dir()
        {
            return Err(Failure::File);
        }
        Ok(self.path.join(name))
    }
    pub fn json(&self, name: &str, value: &impl Serialize) -> Result<(), Failure> {
        use ic_host_fs::durable::{PublicationMode, WriteOptions, write_typed_with};

        write_typed_with(
            &self.record_path(name)?,
            WriteOptions {
                mode: PublicationMode::CreateNew,
                permissions: 0o600,
            },
            |file| serde_json::to_writer_pretty(file, value),
        )
        .map_err(|_| Failure::File)
    }
}

#[cfg(test)]
mod tests;
