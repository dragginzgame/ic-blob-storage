//! Append-only local intents. File locking is not restored-instance authority.
use super::{ReferenceIntentRecord, load};
use crate::operator::Failure;
use sha2::{Digest, Sha256};
use std::{
    fmt::Write as _,
    fs::{self, File, OpenOptions, TryLockError},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
};

const MAX_ENTRIES: usize = 4096;
const LOCK: &str = ".writer.lock";

#[derive(Debug)]
pub(in crate::operator::reference) struct SavedIntentView {
    pub path: PathBuf,
    pub record: ReferenceIntentRecord,
    pub existing: bool,
}

// Keeping this handle alive holds the OS lock. Never unlink/replace the lock file:
// a second inode would let another process acquire a different lock.
struct Journal {
    path: PathBuf,
    directory: File,
    _writer: File,
}

impl Journal {
    fn open(path: &Path) -> Result<Self, Failure> {
        let path = path.canonicalize().map_err(|_| Failure::Storage)?;
        path.to_str().ok_or(Failure::Storage)?;
        let directory = File::open(&path).map_err(|_| Failure::Storage)?;
        // Verify directory syncing is supported before creating journal contents.
        directory.sync_all().map_err(|_| Failure::Storage)?;
        let lock = path.join(LOCK);
        let mut options = OpenOptions::new();
        options.read(true).write(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let writer = match options.create_new(true).open(&lock) {
            Ok(file) => file,
            Err(error) if error.kind() == ErrorKind::AlreadyExists => {
                regular_file(&lock)?;
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(&lock)
                    .map_err(|_| Failure::Storage)?
            }
            Err(_) => return Err(Failure::Storage),
        };
        writer.try_lock().map_err(|error| match error {
            TryLockError::WouldBlock => Failure::Busy,
            TryLockError::Error(_) => Failure::Storage,
        })?;
        writer.sync_all().map_err(|_| Failure::Storage)?;
        directory.sync_all().map_err(|_| Failure::Storage)?;
        Ok(Self {
            path,
            directory,
            _writer: writer,
        })
    }

    fn check_capacity(&self) -> Result<(), Failure> {
        let entries = fs::read_dir(&self.path).map_err(|_| Failure::Storage)?;
        let mut count = 0;
        for entry in entries {
            if entry.map_err(|_| Failure::Storage)?.file_name() != LOCK {
                count += 1;
                if count >= MAX_ENTRIES {
                    return Err(Failure::JournalFull);
                }
            }
        }
        Ok(())
    }
}

fn regular_file(path: &Path) -> Result<(), Failure> {
    let metadata = fs::symlink_metadata(path).map_err(|_| Failure::Storage)?;
    if !metadata.is_file() {
        return Err(Failure::Storage);
    }
    Ok(())
}

fn name(record: &ReferenceIntentRecord) -> Result<String, Failure> {
    let request = record.request()?;
    // Canonical scope + object lifetime + operation ID. Changed root, reference,
    // action, declared size or asset payload MUST collide and fail comparison.
    let key = serde_json::to_vec(&(
        "reference-intent-v1",
        request.object.service.to_text(),
        request.object.tenant.to_text(),
        request.object.namespace.to_string(),
        request.object.id.to_string(),
        "1",
        request.operation.to_string(),
    ))
    .expect("fixed identity tuple");
    let mut name = String::with_capacity(69);
    for byte in Sha256::digest(key) {
        write!(name, "{byte:02x}").expect("writing to String");
    }
    name.push_str(".json");
    Ok(name)
}

pub(in crate::operator::reference) fn save(
    input: &Path,
    directory: &Path,
) -> Result<SavedIntentView, Failure> {
    let record = load(input)?;
    let journal = Journal::open(directory)?;
    let path = journal.path.join(name(&record)?);
    let existing = match fs::symlink_metadata(&path) {
        Ok(_) => {
            regular_file(&path)?;
            if load(&path)? != record {
                return Err(Failure::Conflict);
            }
            // Recover a completed write whose directory sync or stdout was lost.
            File::open(&path)
                .and_then(|file| file.sync_all())
                .map_err(|_| Failure::Storage)?;
            true
        }
        Err(error) if error.kind() == ErrorKind::NotFound => {
            journal.check_capacity()?;
            let mut file =
                tempfile::NamedTempFile::new_in(&journal.path).map_err(|_| Failure::Storage)?;
            serde_json::to_writer(file.as_file_mut(), &record).map_err(|_| Failure::Storage)?;
            writeln!(file).map_err(|_| Failure::Storage)?;
            file.as_file().sync_all().map_err(|_| Failure::Storage)?;
            file.persist_noclobber(&path)
                .map_err(|_| Failure::Storage)?;
            false
        }
        Err(_) => return Err(Failure::Storage),
    };
    // A failure here leaves uncertain local persistence, not a successful ack.
    // Retrying this exact identity revalidates and syncs the existing record.
    journal.directory.sync_all().map_err(|_| Failure::Storage)?;
    Ok(SavedIntentView {
        path,
        record,
        existing,
    })
}

#[cfg(test)]
mod tests;
