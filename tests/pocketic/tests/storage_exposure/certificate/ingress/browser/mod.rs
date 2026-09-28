//! Explicit browser opt-in: the default Rust suite has no Node/Chromium requirement.
use super::*;
use std::{path::PathBuf, process::Command};

#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser"]
fn chromium_certificate_intent_survives_reload_competing_tabs_and_cancellation() {
    let h = Headless::new();
    for (id, content, held) in [(u128::MAX, 1, false), (u128::MAX - 1, 2, true)] {
        let (input, expected) = h.prepare(id, content);
        let args = serde_json::json!({
            "url": h.url,
            "service": expected.service.to_text(),
            "uploader": expected.uploader.to_text(),
            "tenant": input.permission.upload.tenant.to_text(),
            "operation": input.permission.upload.upload.to_string(),
            "permission": candid::encode_one(input.permission).unwrap(),
            "root": expected.root,
            "rootKey": h.agent.read_root_key(),
            "holdResponse": held,
        });
        let config = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(config.path(), serde_json::to_vec(&args).unwrap()).unwrap();
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let node = std::env::var_os("BLOB_BROWSER_NODE").unwrap_or_else(|| "node".into());
        let output = Command::new(node)
            .arg(repo.join("tests/browser/run.mjs"))
            .arg(config.path())
            .current_dir(&repo)
            .output()
            .expect("run configured browser driver");
        assert!(
            output.status.success(),
            "browser failed: {}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(report["outcome"], "passed");
        assert_eq!(report["cancelled"], held);
        assert_eq!(
            inspect(&h.fixture, h.fixture.uploader, input)
                .unwrap()
                .state,
            UploadState::ExposurePossible
        );
        // Browser cancellation is not tenant permission withdrawal or quota release.
        assert!(
            !inspect(&h.fixture, h.fixture.tenant, input)
                .unwrap()
                .revoked
        );
    }
}
