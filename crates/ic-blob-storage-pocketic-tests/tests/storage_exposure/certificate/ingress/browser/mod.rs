//! Explicit browser opt-in: the default Rust suite has no Node/Chromium requirement.
mod admission;
use super::*;
use crate::browser_driver::{BrowserDriver, BrowserPreparation};

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
        let mut driver = BrowserDriver::start(&args, "run.mjs");
        // Bound transport buffering before the shared decoder's own JSON bound.
        let browser: BrowserPreparation = driver.read(8192);
        assert_eq!(browser.hash, expected_root);
        assert_eq!(browser.byte_length, permission.request.bytes);
        let consumer = admission::handshake(
            &h.fixture,
            permission,
            &browser,
            &mut driver,
            scenario == "success",
        );
        let grant = serde_json::json!({ "permission": candid::encode_one(input.permission).unwrap(),
                "consumer": admission::consumer_commands(&consumer) });
        driver.send(&grant);
        let report = driver.finish();
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
