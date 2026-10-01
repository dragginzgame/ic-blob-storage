//! Actual managed admission/preparation and refused issuance through the existing SDK.
mod admission;
use super::{
    Fixture,
    cli::{live, local_subnet_key},
};
use crate::browser_driver::{BrowserDriver, BrowserPreparation};
use admission::Application;
use ic_blob_storage::dto::upload::{
    UploadState,
    certificate::UploadCertificateAssessmentResponse,
    exposure::{UploadExposureBlocker as B, UploadExposureFailure},
};
use ic_blob_storage::workflow::uploads::certificate::UPLOAD_CERTIFICATE_ASSESSMENT_METHOD;
use ic_testkit::pic::CandidCallExt;
use serde_json::{Value, json};

#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-canic-browser"]
fn chromium_managed_upload_setup_refuses_certificate_without_gateway_effects_and_cleans_up() {
    let app = Application::new();
    let trusted = local_subnet_key(&app.f);
    let (mut replica, url) = live(&app.f);
    let mut driver = BrowserDriver::start(&json!({
        "url": url, "service": app.f.app().to_text(), "tenant": app.tenant.to_text(),
        "uploader": app.request.permission.uploader.to_text(),
        "operation": app.request.permission.upload.upload.to_string(),
        "root": app.root, "rootKey": trusted, "content": 42, "certificateBlocked": true,
    }));
    let prepared: BrowserPreparation = driver.read(8192);
    let run = app.handshake(&prepared, &mut driver);
    let before = app.f.pic().get_stable_memory(app.f.app());
    let assessment: Result<UploadCertificateAssessmentResponse, UploadExposureFailure> = app
        .f
        .pic()
        .query_candid_as(
            app.f.app(),
            app.request.permission.uploader,
            UPLOAD_CERTIFICATE_ASSESSMENT_METHOD,
            (&app.root,),
        )
        .unwrap();
    let assessment = assessment.unwrap();
    assert_eq!(assessment.permission, app.request.permission);
    assert_eq!(
        assessment.blockers,
        vec![
            B::PrechargeLimits,
            B::ProviderNamespace,
            B::ReplayCharging,
            B::Recovery
        ]
    );
    driver.send(
        &json!({ "permission": candid::encode_one(app.request.permission).unwrap(),
        "consumer": Application::consumer_commands(&run) }),
    );
    let refused: Value = driver.read(16_384);
    println!("{refused}");
    assert_eq!(refused["outcome"], "refused");
    assert_eq!(refused["calls"], 1);
    assert_eq!(refused["gatewayCalls"], 0);
    assert_eq!(app.f.pic().get_stable_memory(app.f.app()), before);
    app.verify_unexposed(&run);
    driver.send(&json!({ "cleanup": true }));
    let report = driver.finish();
    println!("{report}");
    assert_eq!(report["outcome"], "passed");
    let mut expected = refused["intent"].clone();
    expected["cancelled"] = json!(true);
    assert_eq!(report["intent"], expected);
    app.verify_cleanup(&run, &report["consumer"]);
    replica.stop_progress();
    app.f
        .upgrade_same_release(std::time::Duration::from_secs(5));
    assert_eq!(app.admission().state, UploadState::Cancelled);
    assert!(app.admission().revoked);
    assert_eq!(app.status().uploads.reserved_bytes, 0);
    assert!(app.status().uploads.fenced);
    replica.stop_live();
}
