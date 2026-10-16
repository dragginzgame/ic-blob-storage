//! File selection retains its original following or root-relative opening authority.
use super::{Failure, LocalBody};
#[cfg(unix)]
use ic_host_artifacts::artifact::ArtifactError;
use std::{
    fs::File,
    path::{Path, PathBuf},
    sync::Arc,
};

pub(in crate::native) enum BodySource {
    Path(PathBuf),
    Beneath {
        root: Arc<File>,
        relative: PathBuf,
        display: PathBuf,
    },
}

impl From<PathBuf> for BodySource {
    fn from(path: PathBuf) -> Self {
        Self::Path(path)
    }
}

impl BodySource {
    pub fn path(&self) -> &Path {
        match self {
            Self::Path(path) => path,
            Self::Beneath { display, .. } => display,
        }
    }

    pub fn file(&self) -> Result<File, Failure> {
        match self {
            Self::Path(path) => {
                ic_host_fs::read::open_regular_file(path).map_err(|_| Failure::File)
            }
            Self::Beneath { root, relative, .. } => {
                #[cfg(unix)]
                {
                    use std::os::fd::AsFd;
                    // Reads and exact-length verifiers enforce their own byte budgets
                    // after descriptor admission, before any bytes are consumed.
                    ic_host_fs::read::open_regular_file_beneath(root.as_fd(), relative, u64::MAX)
                        .map_err(|error| match error {
                            ArtifactError::NotRegularFile => Failure::Arguments,
                            ArtifactError::Io(error)
                                if error.kind() == std::io::ErrorKind::InvalidInput =>
                            {
                                Failure::Arguments
                            }
                            _ => Failure::File,
                        })
                }
                #[cfg(not(unix))]
                {
                    let _ = (root, relative);
                    Err(Failure::File)
                }
            }
        }
    }

    pub fn open(&self, bytes: u64) -> Result<LocalBody, Failure> {
        LocalBody::from_file(self.file()?, bytes)
    }
}
