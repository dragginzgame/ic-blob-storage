use super::*;
use crate::operator::reference::record::tests::encoded;
use serde_json::{Value, json};

struct Fixture {
    _dir: tempfile::TempDir,
    input: PathBuf,
    journal: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("input.json");
        let journal = dir.path().join("journal");
        fs::create_dir(&journal).unwrap();
        let fixture = Self {
            _dir: dir,
            input,
            journal,
        };
        fixture.write(&encoded());
        fixture
    }
    fn write(&self, value: &Value) {
        fs::write(&self.input, serde_json::to_vec(value).unwrap()).unwrap();
    }
    fn save(&self) -> Result<SavedIntentView, Failure> {
        save(&self.input, &self.journal)
    }
}

#[test]
fn exact_retry_recovers_same_private_file_and_conflicts_preserve_original() {
    let f = Fixture::new();
    let saved = f.save().unwrap();
    assert!(!saved.existing);
    assert_eq!(saved.record.request().unwrap().operation, u128::MAX);
    let bytes = fs::read(&saved.path).unwrap();
    let retry = f.save().unwrap();
    assert!(retry.existing);
    assert_eq!(retry.path, saved.path);
    assert_eq!(retry.record, saved.record);
    for (field, value) in [
        ("retain", json!(false)),
        ("reference", json!("7")),
        ("bytes", json!(4)),
        ("asset", json!("different")),
        ("root", json!(format!("sha256:{}", "22".repeat(32)))),
    ] {
        let mut changed = encoded();
        changed[field] = value;
        f.write(&changed);
        assert_eq!(f.save().unwrap_err(), Failure::Conflict);
        assert_eq!(fs::read(&saved.path).unwrap(), bytes);
    }
    assert_eq!(fs::read_dir(&f.journal).unwrap().count(), 2);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        for path in [saved.path, f.journal.join(LOCK)] {
            assert_eq!(
                fs::metadata(path).unwrap().permissions().mode() & 0o777,
                0o600
            );
        }
    }
}

#[test]
fn scope_and_operation_are_part_of_the_key() {
    let f = Fixture::new();
    let saved = f.save().unwrap();
    for (field, value) in [
        ("operation", json!("1")),
        ("namespace", json!("2")),
        (
            "tenant",
            json!(candid::Principal::from_slice(&[3, 1]).to_text()),
        ),
        (
            "service",
            json!(candid::Principal::from_slice(&[4, 1]).to_text()),
        ),
    ] {
        let mut changed = encoded();
        changed[field] = value;
        f.write(&changed);
        let next = f.save().unwrap();
        assert_ne!(next.path, saved.path);
        assert!(!next.existing);
    }
    let mut changed = encoded();
    changed["upload"] = json!("2");
    changed["object"] = json!("2");
    f.write(&changed);
    assert_ne!(f.save().unwrap().path, saved.path);
}

#[test]
fn corrupt_final_record_and_interrupted_staging_are_never_replaced() {
    let f = Fixture::new();
    let staging = f.journal.join("interrupted.tmp");
    fs::write(&staging, b"partial").unwrap();
    let saved = f.save().unwrap();
    assert_eq!(fs::read(&staging).unwrap(), b"partial");
    fs::write(&saved.path, b"{").unwrap();
    assert_eq!(f.save().unwrap_err(), Failure::InvalidRequest);
    assert_eq!(fs::read(&saved.path).unwrap(), b"{");
    assert_eq!(fs::read(&staging).unwrap(), b"partial");
}

#[test]
fn capacity_blocks_new_identities_but_preserves_exact_recovery() {
    let f = Fixture::new();
    let saved = f.save().unwrap();
    for i in 1..MAX_ENTRIES {
        fs::write(f.journal.join(format!("residue-{i}")), b"").unwrap();
    }
    assert!(f.save().unwrap().existing);
    let mut next = encoded();
    next["operation"] = json!("1");
    f.write(&next);
    assert_eq!(f.save().unwrap_err(), Failure::JournalFull);
    assert_eq!(load(&saved.path).unwrap(), saved.record);
}

#[test]
fn a_second_writer_is_rejected_until_the_first_handle_closes() {
    let f = Fixture::new();
    let lock = Journal::open(&f.journal).unwrap();
    assert_eq!(f.save().unwrap_err(), Failure::Busy);
    drop(lock);
    assert!(!f.save().unwrap().existing);
}

#[cfg(unix)]
#[test]
fn symlinked_lock_and_record_fail_without_touching_the_target() {
    let f = Fixture::new();
    let lock = f.journal.join(LOCK);
    std::os::unix::fs::symlink("absent", &lock).unwrap();
    assert_eq!(f.save().unwrap_err(), Failure::Storage);
    assert!(!f.journal.join("absent").exists());
    fs::remove_file(lock).unwrap();
    let record = load(&f.input).unwrap();
    let destination = f.journal.join(name(&record).unwrap());
    std::os::unix::fs::symlink(&f.input, &destination).unwrap();
    assert_eq!(f.save().unwrap_err(), Failure::Storage);
    assert_eq!(load(&f.input).unwrap(), record);
    assert_eq!(fs::read_link(destination).unwrap(), f.input);
}
