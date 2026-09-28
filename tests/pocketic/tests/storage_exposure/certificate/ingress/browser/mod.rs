//! Explicit browser opt-in: the default Rust suite has no Node/Chromium requirement.
use super::*;
use ic_blob_storage::{
    dto::upload::manifest::{UploadManifestFailure, UploadManifestMutation, UploadManifestRequest},
    model::identity::caffeine::manifest::CaffeineManifestLimits,
    ops::caffeine::preparation::{PreparedManifestLimits, decode_prepared_manifest},
};
use std::{
    io::{BufRead, BufReader, Read, Write},
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

fn admit_browser_manifest(f: &Fixture, permission: Permission, browser: &BrowserPreparation) {
    let input = input(f, permission);
    let declaration = decode_prepared_manifest(
        browser.hash.parse().unwrap(),
        browser.byte_length,
        browser.manifest_json.as_bytes(),
        PreparedManifestLimits {
            max_json_bytes: NonZeroUsize::new(4096).unwrap(),
            manifest: CaffeineManifestLimits {
                max_content_bytes: NonZeroU64::new(10).unwrap(),
                max_chunks: NonZeroUsize::MIN,
                max_headers: NonZeroUsize::new(8).unwrap(),
                max_header_bytes: NonZeroUsize::new(1024).unwrap(),
            },
        },
    )
    .unwrap();
    f.admit(f.tenant, permission).unwrap();
    let request = UploadManifestRequest {
        permission: input.permission,
        declaration,
    };
    // The browser's data is not uploader authority: the actual tenant cannot prepare it.
    let denied: Result<UploadManifestMutation, UploadManifestFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.tenant, "blob_prepare_upload", (&request,))
        .unwrap();
    assert_eq!(denied, Err(UploadManifestFailure::Permission(A::Denied)));
    let accepted: Result<UploadManifestMutation, UploadManifestFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.uploader, "blob_prepare_upload", (&request,))
        .unwrap();
    assert!(accepted.unwrap().changed);
    assert_eq!(
        inspect(f, f.uploader, input).unwrap().state,
        UploadState::Reserved
    );
    configure(f, input);
}

#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser"]
fn chromium_certificate_intent_survives_reload_competing_tabs_and_cancellation() {
    for (id, content, scenario) in [
        (u128::MAX, 1, "success"),
        (u128::MAX - 1, 2, "lost-response"),
        (u128::MAX - 2, 3, "gateway-failure"),
        (u128::MAX - 3, 4, "abort-after-tree"),
    ] {
        let h = Headless::new();
        let held = scenario == "lost-response";
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
        let mut reader = BufReader::new(child.stdout.take().unwrap()).take(8193);
        let mut line = Vec::new();
        reader.read_until(b'\n', &mut line).unwrap();
        assert!(line.len() <= 8192 && line.ends_with(b"\n"));
        let browser: BrowserPreparation = serde_json::from_slice(&line).unwrap();
        assert_eq!(browser.hash, expected_root);
        assert_eq!(browser.byte_length, permission.request.bytes);
        admit_browser_manifest(&h.fixture, permission, &browser);
        child.stdout = Some(reader.into_inner().into_inner());
        let grant =
            serde_json::json!({ "permission": candid::encode_one(input.permission).unwrap() });
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
