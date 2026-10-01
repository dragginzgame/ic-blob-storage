//! Private create-new artifacts shared by native effect and download commands.
use super::Failure;
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::Write,
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
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(self.path.join(name))
            .map_err(|_| Failure::File)?;
        file.write_all(bytes)
            .and_then(|()| file.sync_all())
            .map_err(|_| Failure::File)?;
        File::open(&self.path)
            .and_then(|f| f.sync_all())
            .map_err(|_| Failure::File)
    }
    pub fn json(&self, name: &str, value: &impl Serialize) -> Result<(), Failure> {
        self.bytes(
            name,
            &serde_json::to_vec_pretty(value).map_err(|_| Failure::File)?,
        )
    }
}
