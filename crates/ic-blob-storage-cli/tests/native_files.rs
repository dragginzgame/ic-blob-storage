//! Subprocess evidence for rejecting a blocking input before transport setup.
#![cfg(unix)]
use std::{
    process::{Command, Stdio},
    time::{Duration, Instant},
};

#[test]
fn fifo_cursor_without_a_writer_returns_typed_file_refusal_promptly() {
    let root = tempfile::tempdir().unwrap();
    let cursor = root.path().join("cursor");
    assert!(
        Command::new("mkfifo")
            .arg(&cursor)
            .status()
            .unwrap()
            .success()
    );
    let actor = candid::Principal::self_authenticating([42]).to_text();
    let mut child = Command::new(env!("CARGO_BIN_EXE_blob-storage"))
        .args([
            "funding-history",
            "--network",
            "local",
            "--url",
            "http://127.0.0.1:1",
            "--identity",
            "missing.pem",
            "--operator",
            &actor,
            "--service",
            &actor,
            "--namespace",
            "1",
            "--cashier",
            &actor,
            "--payer",
            &actor,
            "--root-key",
            "missing.der",
            "--cursor",
        ])
        .arg(&cursor)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        if child.try_wait().unwrap().is_some() {
            break;
        }
        if Instant::now() >= deadline {
            child.kill().unwrap();
            child.wait().unwrap();
            panic!("non-regular input blocked before typed refusal");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(3));
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap(),
        serde_json::json!({"error":"file"})
    );
    assert_eq!(output.stderr, [] as [u8; 0]);
}
