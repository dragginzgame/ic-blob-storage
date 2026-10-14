//! Private create-new artifacts shared by native effect and download commands.
use super::Failure;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
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
    directory: File,
}
impl Run {
    /// Allocate private unverified output without replacing any existing file.
    pub fn open_body(&self) -> Result<File, Failure> {
        self.admit_directory()?;
        #[cfg(unix)]
        {
            use std::{ffi::OsStr, os::fd::AsFd as _};
            ic_host_fs::durable::create_retained_at(
                self.directory.as_fd(),
                OsStr::new("body.part"),
                0o600,
            )
            .map_err(|_| Failure::File)
        }
        #[cfg(not(unix))]
        {
            Err(Failure::File)
        }
    }
    /// Publish the verified held body without replacing any existing output.
    pub fn publish_body(&self, body: File) -> Result<PathBuf, Failure> {
        self.admit_directory()?;
        #[cfg(unix)]
        {
            use std::{ffi::OsStr, os::fd::AsFd as _};
            ic_host_fs::durable::publish_retained_at(
                self.directory.as_fd(),
                OsStr::new("body.part"),
                OsStr::new("body.bin"),
                body,
            )
            .map_err(|_| Failure::File)?;
            Ok(self.path.join("body.bin"))
        }
        #[cfg(not(unix))]
        {
            let _ = body;
            Err(Failure::File)
        }
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
        let directory = File::open(path).map_err(|_| Failure::File)?;
        Ok(Self {
            path: path.to_owned(),
            directory,
        })
    }
    pub fn bytes(&self, name: &str, bytes: &[u8]) -> Result<(), Failure> {
        use std::io::Write as _;
        self.write_record(name, |file| file.write_all(bytes))
    }

    fn admit_directory(&self) -> Result<(), Failure> {
        // Preserve the claimed pathname contract before any new output. The
        // held descriptor then owns publication, without parent recreation.
        // Trusted ancestors and exclusion of concurrent writers stay caller-owned.
        let current = std::fs::symlink_metadata(&self.path).map_err(|_| Failure::File)?;
        if !current.is_dir() {
            return Err(Failure::File);
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt as _;
            let held = self.directory.metadata().map_err(|_| Failure::File)?;
            if (current.dev(), current.ino()) != (held.dev(), held.ino()) {
                return Err(Failure::File);
            }
        }
        Ok(())
    }

    fn write_record<T, E>(
        &self,
        name: &str,
        produce: impl FnOnce(&mut File) -> Result<T, E>,
    ) -> Result<T, Failure> {
        self.admit_directory()?;
        #[cfg(unix)]
        {
            use ic_host_fs::durable::{PublicationMode, WriteOptions, write_at_with};
            use std::{ffi::OsStr, os::fd::AsFd as _};
            write_at_with(
                self.directory.as_fd(),
                OsStr::new(name),
                WriteOptions {
                    mode: PublicationMode::CreateNew,
                    permissions: 0o600,
                },
                produce,
            )
            .map_err(|_| Failure::File)
        }
        #[cfg(not(unix))]
        {
            // Host's durable engine does not support this platform.
            let _ = (name, produce);
            Err(Failure::File)
        }
    }

    pub fn json(&self, name: &str, value: &impl Serialize) -> Result<(), Failure> {
        self.write_record(name, |file| serde_json::to_writer_pretty(file, value))
    }
}

#[cfg(test)]
mod tests;
