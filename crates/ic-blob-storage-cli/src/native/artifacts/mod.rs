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
        use ic_host_fs::durable::{DirectoryCreationError, create_private_directory};
        let directory = create_private_directory(path).map_err(|error| match error {
            DirectoryCreationError::BeforeCreation(source)
                if source.kind() == std::io::ErrorKind::AlreadyExists =>
            {
                Failure::ExistingRun
            }
            _ => Failure::File,
        })?;
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
        ic_host_fs::durable::admit_directory(&self.path, &self.directory).map_err(|_| Failure::File)
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
