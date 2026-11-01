//! Subprocess evidence for rejecting a blocking input before transport setup.
#![cfg(unix)]
use ic_host_fs::testing::FixtureDirectory;
use std::{
    process::{Command, Stdio},
    time::{Duration, Instant},
};

#[test]
fn fifo_cursor_without_a_writer_returns_typed_file_refusal_promptly() {
    let root = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
    let cursor = root.path().join("cursor");
    assert!(
        Command::new("mkfifo")
            .arg(&cursor)
            .status()
            .unwrap()
            .success()
    );
    let actor = ic_platform::Principal::self_authenticating([42]).to_text();
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

#[test]
fn publication_requires_host_release_before_opening_inputs_or_identity() {
    let root = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
    let actor = ic_platform::Principal::self_authenticating([42]).to_text();
    for operation in ["publish-map", "publish-file-status", "publish-session"] {
        for pin in [None, Some(""), Some("0.77.0")] {
            let mut command = Command::new(env!("CARGO_BIN_EXE_blob-storage"));
            command.current_dir(root.path()).args([
                operation,
                "--network",
                "local",
                "--url",
                "http://127.0.0.1:1",
                "--identity",
                "missing.pem",
                "--actor",
                &actor,
                "--operator-identity",
                "missing-operator.pem",
                "--root-key",
                "missing.der",
                "--service",
                &actor,
                "--namespace",
                "1",
                "--gateway",
                "https://127.0.0.1:65530/",
                "--inputs",
                "missing-inputs",
                "--max-bytes",
                "10",
                "--max-total-bytes",
                "20",
                "--timeout-seconds",
                "1",
                "--run-dir",
                "must-not-exist",
            ]);
            match operation {
                "publish-session" => {
                    command.args(["--uploader-identity", "missing.pem", "--max-steps", "9"]);
                }
                "publish-map" => {
                    command.args(["--max-queries", "4"]);
                }
                _ => {
                    command.args(["--max-queries", "3", "--file-index", "0"]);
                }
            }
            command.env_remove("BLOB_EXPECTED_HOST_RELEASE");
            if let Some(pin) = pin {
                command.env("BLOB_EXPECTED_HOST_RELEASE", pin);
            }
            let output = command.output().unwrap();
            let selected = pin.is_some_and(|p| !p.is_empty());
            assert_eq!(output.status.code(), Some(if selected { 3 } else { 2 }));
            let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(
                result["error"],
                if selected { "file" } else { "arguments" },
                "{operation}: {result}"
            );
            assert!(!root.path().join("must-not-exist").exists());
        }
    }
}
