//! Explicit browser opt-in: the default Rust suite has no Node/Chromium requirement.
mod admission;
use super::*;
use std::{
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    process::{Child, Command, Stdio},
};

// Ensure failed assertions also terminate and reap the owned browser driver.
struct BrowserProcess(Option<Child>);
impl Drop for BrowserProcess {
    fn drop(&mut self) {
        if let Some(child) = &mut self.0 {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct BrowserPreparation {
    hash: String,
    byte_length: u64,
    #[serde(rename = "manifestJSON")]
    manifest_json: String,
}

fn read_control<T: serde::de::DeserializeOwned>(reader: &mut impl BufRead, limit: u64) -> T {
    use std::io::Read;
    let mut line = Vec::new();
    reader.take(limit + 1).read_until(b'\n', &mut line).unwrap();
    assert!(line.len() as u64 <= limit && line.ends_with(b"\n"));
    serde_json::from_slice(&line).unwrap()
}

#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser"]
fn chromium_certificate_intent_survives_reload_competing_tabs_and_cancellation() {
    for (id, content, scenario) in [
        (u128::MAX, 1, "success"),
        (u128::MAX - 1, 2, "lost-response"),
        (u128::MAX - 2, 3, "gateway-failure"),
        (u128::MAX - 3, 4, "abort-after-tree"),
        (u128::MAX - 4, 5, "gateway-write-abort"),
        (u128::MAX - 5, 6, "gateway-lost-response"),
        (u128::MAX - 6, 7, "gateway-observe-abort"),
        (u128::MAX - 7, 8, "gateway-late-cancel"),
        (u128::MAX - 8, 9, "gateway-oversize"),
        (u128::MAX - 9, 10, "gateway-cancel-before-claim"),
    ] {
        let mut h = Headless::new();
        admission::install(&mut h.fixture);
        let held = scenario == "lost-response";
        let cancelled = held
            || matches!(
                scenario,
                "gateway-lost-response" | "gateway-late-cancel" | "gateway-cancel-before-claim"
            );
        let (permission, _) = h.fixture.permission(id, content);
        let input = input(&h.fixture, permission);
        let expected_root = root(permission);
        let args = serde_json::json!({
            "url": h.url,
            "service": h.fixture.service.to_text(),
            "uploader": h.fixture.uploader.to_text(),
            "tenant": input.permission.upload.tenant.to_text(),
            "operation": input.permission.upload.upload.to_string(),
            "root": expected_root,
            "rootKey": h.agent.read_root_key(),
            "holdResponse": held,
            "content": content,
            "gatewayFailure": scenario == "gateway-failure",
            "abortAfterTree": scenario == "abort-after-tree",
            "gatewayWriteAbort": scenario == "gateway-write-abort",
            "gatewayObserveAbort": scenario == "gateway-observe-abort",
            "holdGateway": matches!(scenario, "gateway-lost-response" | "gateway-late-cancel"),
            "lateGateway": scenario == "gateway-late-cancel",
            "oversizeGateway": scenario == "gateway-oversize",
            "cancelAtGatewayClaim": scenario == "gateway-cancel-before-claim",
        });
        let config = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(config.path(), serde_json::to_vec(&args).unwrap()).unwrap();
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let node = std::env::var_os("BLOB_BROWSER_NODE").unwrap_or_else(|| "node".into());
        let child = Command::new(node)
            .arg(repo.join("tests/browser/run.mjs"))
            .arg(config.path())
            .current_dir(&repo)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("run configured browser driver");
        let mut process = BrowserProcess(Some(child));
        let child = process.0.as_mut().unwrap();
        // Bound transport buffering before the shared decoder's own JSON bound.
        let mut reader = BufReader::new(child.stdout.take().unwrap());
        let browser: BrowserPreparation = read_control(&mut reader, 8192);
        assert_eq!(browser.hash, expected_root);
        assert_eq!(browser.byte_length, permission.request.bytes);
        let consumer = admission::handshake(
            &h.fixture,
            permission,
            &browser,
            &mut reader,
            child.stdin.as_mut().unwrap(),
            scenario == "success",
        );
        child.stdout = Some(reader.into_inner());
        let grant = serde_json::json!({ "permission": candid::encode_one(input.permission).unwrap(),
                "consumer": admission::consumer_commands(&consumer) });
        writeln!(child.stdin.take().unwrap(), "{grant}").unwrap();
        let output = process.0.take().unwrap().wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "browser failed: {}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(report["outcome"], "passed");
        assert_eq!(report["cancelled"], cancelled);
        admission::verify_consumer(&h.fixture, &consumer, cancelled, report["consumer"].clone());
        assert_eq!(
            inspect(&h.fixture, h.fixture.uploader, input)
                .unwrap()
                .state,
            UploadState::ExposurePossible
        );
        // The explicit consumer follow-through withdraws; browser cancellation alone does not.
        assert_eq!(
            inspect(&h.fixture, h.fixture.tenant, input)
                .unwrap()
                .revoked,
            cancelled
        );
    }
}
