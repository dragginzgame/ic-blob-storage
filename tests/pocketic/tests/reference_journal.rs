//! Real subprocess journal locking and recovery; no service effects are sent.
use candid::Principal;
use serde_json::{Value, json};
use std::{
    fs::{self, OpenOptions},
    path::Path,
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};

fn save(input: &Path, journal: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_blob-fixture-reference"));
    command
        .arg("save")
        .arg("--intent")
        .arg(input)
        .arg("--journal")
        .arg(journal);
    command
}

fn result(mut command: Command, code: i32) -> Value {
    let output = command.output().unwrap();
    assert_eq!(
        output.status.code(),
        Some(code),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

struct ChildGuard(Child);
impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[test]
fn interrupted_writer_releases_lock_and_lost_ack_recovers_exact_intent() {
    let dir = tempfile::tempdir().unwrap();
    let journal = dir.path().join("journal");
    fs::create_dir(&journal).unwrap();
    let input = dir.path().join("input.json");
    let value = json!({"schema":1,"scope":"pocketic_fixture","asset":"image-a",
        "service":Principal::from_slice(&[1,1]).to_text(),"tenant":Principal::from_slice(&[2,1]).to_text(),
        "namespace":"1","upload":"1","object":"1","incarnation":"1",
        "root":format!("sha256:{}", "11".repeat(32)),"bytes":3,"reference":"2","operation":"1","retain":true});
    fs::write(&input, serde_json::to_vec(&value).unwrap()).unwrap();
    let ready = dir.path().join("ready");
    let mut holder = ChildGuard(
        Command::new(std::env::current_exe().unwrap())
            .args(["--ignored", "--exact", "lock_holder_child"])
            .env("BLOB_JOURNAL_TEST_DIRECTORY", &journal)
            .env("BLOB_JOURNAL_TEST_READY", &ready)
            .stdout(Stdio::null())
            .spawn()
            .unwrap(),
    );
    let deadline = Instant::now() + Duration::from_secs(10);
    while !ready.exists() {
        assert!(Instant::now() < deadline, "lock holder did not start");
        assert!(holder.0.try_wait().unwrap().is_none(), "lock holder exited");
        thread::sleep(Duration::from_millis(10));
    }
    assert_eq!(result(save(&input, &journal), 3)["error"], "journal_busy");
    holder.0.kill().unwrap();
    holder.0.wait().unwrap();
    // No receipt reaches this client. The next process must recover by exact identity.
    assert!(
        save(&input, &journal)
            .stdout(Stdio::null())
            .status()
            .unwrap()
            .success()
    );
    let recovered = result(save(&input, &journal), 0);
    assert_eq!(recovered["existing"], true);
    assert_eq!(recovered["intent"], value);
    assert_eq!(recovered["persistence"], "file_and_directory_synced");
    assert_eq!(recovered["restore_authority"], "not_established");
    assert_eq!(recovered["dispatch"], "not_performed");
    let record = Path::new(recovered["saved"].as_str().unwrap());
    let original = fs::read(record).unwrap();
    let mut changed = value;
    changed["retain"] = json!(false);
    fs::write(&input, serde_json::to_vec(&changed).unwrap()).unwrap();
    assert_eq!(
        result(save(&input, &journal), 3)["error"],
        "request_conflict"
    );
    assert_eq!(fs::read(record).unwrap(), original);
    assert_eq!(fs::read_dir(&journal).unwrap().count(), 2);
}

#[test]
#[ignore = "subprocess helper holds the actual filesystem lock until killed"]
fn lock_holder_child() {
    let directory = std::env::var_os("BLOB_JOURNAL_TEST_DIRECTORY").unwrap();
    let ready = std::env::var_os("BLOB_JOURNAL_TEST_READY").unwrap();
    let lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(Path::new(&directory).join(".writer.lock"))
        .unwrap();
    lock.try_lock().unwrap();
    fs::write(ready, b"locked").unwrap();
    thread::sleep(Duration::from_secs(30));
    panic!("parent did not terminate the lock holder");
}
