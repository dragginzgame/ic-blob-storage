//! Serial native/browser composition; provider and public-serving facts remain local.
use super::{BrowserDriver, Trial, UploadState};
use crate::{Envelope, UploadManifestRequest, standalone_publish_check::freeze_files};
use crate::{browser_driver::BrowserPreparation, native_session::NativeSession};
use ic_blob_storage::model::identity::ContentDigest;
use serde_json::{Value, json};

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct BrowserPublicationPlan {
    gateway: String,
    files: Vec<BrowserPreparation>,
}

#[derive(Clone, Copy)]
enum Scenario {
    Complete,
    LostReply,
    Corrupt,
}

#[derive(Clone, Copy)]
enum Driver {
    Indexed,
    Persistent,
    Worker,
}

enum Control {
    Indexed,
    Persistent(NativeSession),
}
impl Control {
    fn phase(&mut self, frame: &Value) -> Value {
        let Self::Persistent(session) = self else {
            panic!("persistent phase owner");
        };
        session.send(frame);
        let event = session.read();
        assert_eq!(event["event"], "phase");
        event["report"].clone()
    }
}
struct Serial {
    trial: Trial,
    files: Vec<UploadManifestRequest>,
}
impl Serial {
    fn refuse_corrupt_map(&self, control: &mut Control, gateway: &str, observation: &Value) {
        assert_eq!(observation["error"], "content_mismatch");
        assert!(
            !self
                .trial
                .report
                .join("observation-0/statement.candid")
                .exists()
        );
        let map = self.status_using(control, None, gateway, "blocked-map");
        match control {
            Control::Persistent(_) => assert_eq!(map["code"], "files_incomplete"),
            Control::Indexed => assert_eq!(map["all_references_live"], false),
        }
        assert!(
            !self
                .trial
                .report
                .join("blocked-map/media-map.json")
                .exists()
        );
        assert!(!self.trial.report.join("session/media-map.json").exists());
    }

    fn complete_map(&self, control: &mut Control, gateway: &str) {
        let map = self.status_using(control, None, gateway, "complete-map");
        assert_eq!(map["all_references_live"], true);
        for (index, file) in map["files"].as_array().unwrap().iter().enumerate() {
            assert_eq!(file["index"], index);
            assert_eq!(
                file["body_sha256"],
                self.snapshot(index)["snapshot"]["bodySha256"]
            );
        }
    }

    fn restart_after_exposure(&self, control: &mut Control, gateway: &str, original: &Value) {
        let Control::Persistent(session) = std::mem::replace(control, Control::Indexed) else {
            panic!("persistent owner");
        };
        let closed = session.finish(3);
        assert_eq!(closed["error"], "transport");
        self.trial.record("interrupted-session.json", &closed);
        *control = Control::Persistent(self.session(gateway, "resumed-session"));
        assert_eq!(
            control.phase(&json!({"phase":"status","index":0}))["file_live"],
            false
        );
        let recovered = control.phase(&json!({"phase":"prepare","index":0,"source_run":original}));
        assert_eq!(recovered["state"], "permission_inactive");
        assert_eq!(recovered["service_updates_this_run"], 0);
        self.trial.record("exposed-setup-recovery.json", &recovered);
    }
    fn transfer(&self, index: usize, prepared: &Value) -> Value {
        let transfer = &prepared["transfer"];
        if transfer.is_null() {
            return self.snapshot(index);
        }
        let body = std::fs::read(transfer["body"].as_str().unwrap()).unwrap();
        json!({"index":index,"binding":transfer["binding"],"snapshot":{"body":body,
            "bodySha256":transfer["body_sha256"],"manifestJSON":transfer["manifest_json"]}})
    }
    fn session(&self, gateway: &str, label: &str) -> NativeSession {
        let mut args = self
            .trial
            .args("publish-session", "tenant.pem", self.trial.f.tenant);
        self.limits(&mut args, label);
        args.extend([
            "--operator-identity".into(),
            self.trial.report.join("uploader.pem").display().to_string(),
            "--uploader-identity".into(),
            self.trial.report.join("uploader.pem").display().to_string(),
            "--gateway".into(),
            gateway.into(),
            "--max-steps".into(),
            "17".into(),
        ]);
        self.trial
            .record(&format!("{label}-command.json"), &json!(args));
        let mut session = NativeSession::start(&args);
        let ready = session.read();
        assert_eq!(ready["event"], "ready");
        self.trial.record(&format!("{label}-ready.json"), &ready);
        session
    }
    fn prepare_using(&self, control: &mut Control, index: usize, label: &str) -> Value {
        let report = match control {
            Control::Indexed => self.prepare(index, label),
            Control::Persistent(_) => control.phase(&json!({"phase":"prepare","index":index})),
        };
        self.trial.record(&format!("{label}-phase.json"), &report);
        report
    }
    fn status_using(
        &self,
        control: &mut Control,
        index: Option<usize>,
        gateway: &str,
        label: &str,
    ) -> Value {
        let report = match control {
            Control::Indexed => self.status(index, gateway, label),
            Control::Persistent(_) => control.phase(&index.map_or_else(
                || json!({"phase":"map"}),
                |index| json!({"phase":"status","index":index}),
            )),
        };
        self.trial.record(&format!("{label}-phase.json"), &report);
        report
    }
    fn new(label: &str) -> Self {
        let trial = Trial::with_envelope(label, Envelope::Serial);
        // This local operator also signs as the uploader; tenant/verifier differ.
        freeze_files(&trial.f, &trial.report, &[1024, 2048]);
        let files = (0..2)
            .map(|index| {
                candid::decode_one(
                    &std::fs::read(
                        trial
                            .report
                            .join(format!("batch/file-{index:04}/manifest.candid")),
                    )
                    .unwrap(),
                )
                .unwrap()
            })
            .collect();
        Self { trial, files }
    }
    fn limits(&self, args: &mut Vec<String>, label: &str) {
        args.extend([
            "--inputs".into(),
            self.trial.report.join("batch").display().to_string(),
            "--max-bytes".into(),
            "2048".into(),
            "--max-total-bytes".into(),
            "3072".into(),
            "--timeout-seconds".into(),
            "30".into(),
            "--run-dir".into(),
            self.trial.report.join(label).display().to_string(),
        ]);
    }
    fn prepare(&self, index: usize, label: &str) -> Value {
        let mut args = self
            .trial
            .args("publish-prepare", "tenant.pem", self.trial.f.tenant);
        self.limits(&mut args, label);
        args.extend([
            "--uploader-identity".into(),
            self.trial.report.join("uploader.pem").display().to_string(),
            "--file-index".into(),
            index.to_string(),
        ]);
        self.trial.invoke(label, &args, 0)
    }
    fn snapshot(&self, index: usize) -> Value {
        let directory = self.trial.report.join(format!("batch/file-{index:04}"));
        let body = std::fs::read(directory.join("body.bin")).unwrap();
        let binding: Value = serde_json::from_slice(
            &std::fs::read(directory.join("certificate-binding.json")).unwrap(),
        )
        .unwrap();
        json!({"index":index,"binding":binding,"snapshot":{"body":body,
            "bodySha256":ContentDigest::compute(&body).to_string().trim_start_matches("sha256:"),
            "manifestJSON":std::fs::read_to_string(directory.join("manifest.json")).unwrap()}})
    }
    fn status(&self, index: Option<usize>, gateway: &str, label: &str) -> Value {
        let command = if index.is_some() {
            "publish-file-status"
        } else {
            "publish-map"
        };
        let mut args = self.trial.args(command, "tenant.pem", self.trial.f.tenant);
        self.limits(&mut args, label);
        args.extend([
            "--operator-identity".into(),
            self.trial.report.join("uploader.pem").display().to_string(),
            "--gateway".into(),
            gateway.into(),
            "--max-queries".into(),
            if index.is_some() { "4" } else { "6" }.into(),
        ]);
        if let Some(index) = index {
            args.extend(["--file-index".into(), index.to_string()]);
        }
        self.trial.invoke(label, &args, 0)
    }
    fn observe(&self, index: usize, gateway: &str, code: i32) -> Value {
        let verifier = self
            .trial
            .f
            .configuration(self.trial.f.operator)
            .unwrap()
            .completion_verifier;
        let mut args = self.trial.args("observe-upload", "verifier.pem", verifier);
        let label = format!("observation-{index}");
        args.extend([
            "--permission".into(),
            self.trial
                .report
                .join(format!("batch/file-{index:04}/permission.candid"))
                .display()
                .to_string(),
            "--gateway".into(),
            gateway.into(),
            "--max-bytes".into(),
            self.files[index].permission.upload.bytes.to_string(),
            "--run-dir".into(),
            self.trial.report.join(&label).display().to_string(),
        ]);
        self.trial.invoke(&label, &args, code)
    }
    fn attest(&self, index: usize) {
        let verifier = self
            .trial
            .f
            .configuration(self.trial.f.operator)
            .unwrap()
            .completion_verifier;
        let mut args = self
            .trial
            .args("submit-attestation", "verifier.pem", verifier);
        args.extend([
            "--run-dir".into(),
            self.trial
                .report
                .join(format!("observation-{index}"))
                .display()
                .to_string(),
        ]);
        assert_eq!(
            self.trial.invoke(&format!("attestation-{index}"), &args, 0)["outcome"],
            "accepted"
        );
    }
    fn confirm(&self, control: &mut Control, index: usize, gateway: &str, observation: &Value) {
        assert_eq!(
            observation["bytes"],
            self.files[index].permission.upload.bytes.to_string()
        );
        self.attest(index);
        assert_eq!(
            self.trial.f.admission(self.files[index].permission).state,
            UploadState::Confirmed
        );
        assert_eq!(
            self.trial
                .f
                .capacity(self.trial.f.tenant, self.trial.f.scope())
                .unwrap()
                .remaining_active_uploads,
            1
        );
        let live = self.status_using(control, Some(index), gateway, &format!("confirmed-{index}"));
        assert_eq!(live["file_live"], true);
        assert_eq!(live["file_index"], index);
        assert_eq!(live["batch_complete"], false);
        assert!(
            !self
                .trial
                .report
                .join(format!("confirmed-{index}/media-map.json"))
                .exists()
        );
        assert_eq!(
            live["files"][0]["body_sha256"],
            self.snapshot(index)["snapshot"]["bodySha256"]
        );
        let delivered =
            self.trial
                .download(&self.files[index], gateway, &format!("download-{index}"), 0);
        assert_eq!(delivered["content_digest"], observation["content_digest"]);
    }
    fn finish(&mut self, driver: BrowserDriver, scenario: Scenario) {
        let browser = driver.finish();
        assert_eq!(browser["outcome"], "passed");
        self.trial.record("browser-result.json", &browser);
        let retained = self
            .trial
            .f
            .local_status(self.trial.f.operator, self.trial.f.operator_scope())
            .unwrap();
        let bytes = if matches!(scenario, Scenario::Corrupt) {
            1024
        } else {
            3072
        };
        assert_eq!(retained.uploads.physical_bytes, bytes);
        assert_eq!(retained.uploads.liability_bytes, bytes);
        self.trial.record("summary.json", &json!({"schema":1,
        "evidence":"serial_actual_standalone_native_chromium_local_provider_substitute",
        "max_active_uploads":1,"confirmed_map_written":!matches!(scenario,Scenario::Corrupt),
        "retained_physical_bytes":bytes,"retained_liability_bytes":bytes,
        "provider_puts":browser["puts"],"provider_gets":browser["gets"],
        "browser_restart_preserved":true,"automatic_upload_retries":0,
        "live_provider_requests":0,"paid_effects":0,"production_headless_publisher_complete":false}));
        self.trial.f.harness.pic.stop_live();
    }
}

fn run(label: &str, scenario: Scenario, driver: Driver) {
    let mut serial = Serial::new(label);
    let config = json!({"url":serial.trial.url,"service":serial.trial.f.service.to_text(),
        "tenant":serial.trial.f.tenant.to_text(),"bucket":"fixture-bucket","worker":matches!(driver,Driver::Worker),
        "rootKey":serial.trial.f.harness.pic.root_key().unwrap(),"project":super::client::PROJECT,
        "lostFinalReply":matches!(scenario,Scenario::LostReply),"corruptRead":matches!(scenario,Scenario::Corrupt),
        "providerRootCertificate":serial.trial.report.join("gateway-ca.pem"),
        "profile":serial.trial.report.join("browser-profile")});
    let mut browser = BrowserDriver::start(&config, "serial.mjs");
    let plans: BrowserPublicationPlan = browser.read(16384);
    let mut control = match driver {
        Driver::Indexed => Control::Indexed,
        Driver::Persistent | Driver::Worker => {
            Control::Persistent(serial.session(&plans.gateway, "session"))
        }
    };
    for (index, plan) in plans.files.iter().enumerate() {
        assert_eq!(
            plan.byte_length,
            serial.files[index].permission.upload.bytes
        );
        assert_eq!(
            plan.hash,
            crate::standalone_certificate::root(&serial.files[index])
        );
    }
    for index in 0..2 {
        let prepared = serial.prepare_using(&mut control, index, &format!("setup-{index}"));
        assert_eq!(prepared["prepared"], true);
        assert_eq!(
            serial
                .trial
                .f
                .capacity(serial.trial.f.tenant, serial.trial.f.scope())
                .unwrap()
                .remaining_active_uploads,
            0
        );
        browser.send(&serial.transfer(index, &prepared));
        let uploaded: Value = browser.read(8192);
        serial
            .trial
            .record(&format!("browser-upload-{index}.json"), &uploaded);
        let gateway = uploaded["gateway"].as_str().unwrap();
        let before = serial.status_using(
            &mut control,
            Some(index),
            gateway,
            &format!("unconfirmed-{index}"),
        );
        assert_eq!(before["file_live"], false);
        assert_eq!(before["blockers"][0]["code"], "completion_unmatched");
        // A successful or lost SDK response occupies the same single reservation.
        if index == 0 {
            assert_eq!(
                serial.prepare_using(&mut control, 1, "blocked-setup-1")["state"],
                "blocked"
            );
            assert!(
                !serial
                    .trial
                    .report
                    .join("blocked-setup-1/admission")
                    .exists()
            );
        }
        if index == 0
            && matches!(scenario, Scenario::LostReply)
            && matches!(driver, Driver::Persistent | Driver::Worker)
        {
            serial.restart_after_exposure(&mut control, gateway, &prepared["original_run"]);
        }
        let corrupt = matches!(scenario, Scenario::Corrupt);
        let observation = serial.observe(index, gateway, if corrupt { 3 } else { 0 });
        if corrupt {
            serial.refuse_corrupt_map(&mut control, gateway, &observation);
            browser.send(&json!({"finish":true}));
            break;
        }
        serial.confirm(&mut control, index, gateway, &observation);
        if index == 1 {
            serial.complete_map(&mut control, gateway);
            browser.send(&json!({"finish":true}));
        }
    }
    if let Control::Persistent(session) = control {
        let final_report = session.finish(if matches!(scenario, Scenario::Corrupt) {
            3
        } else {
            0
        });
        serial.trial.record("session-final.json", &final_report);
    }
    serial.finish(browser, scenario);
}

#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser-standalone"]
fn chromium_standalone_trial_serial_completion_frees_one_reservation_and_maps_both_files() {
    run("serial-complete", Scenario::Complete, Driver::Indexed);
}
#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser-standalone"]
fn chromium_standalone_trial_serial_lost_reply_recovers_original_browser_history_without_redispatch()
 {
    run("serial-lost", Scenario::LostReply, Driver::Indexed);
}
#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser-standalone"]
fn chromium_standalone_trial_serial_corrupt_observation_blocks_next_transfer_and_complete_map() {
    run("serial-corrupt", Scenario::Corrupt, Driver::Indexed);
}

#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser-standalone"]
fn chromium_standalone_trial_persistent_session_completes_one_active_upload_at_a_time() {
    run(
        "persistent-complete",
        Scenario::Complete,
        Driver::Persistent,
    );
}
#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser-standalone"]
fn chromium_standalone_trial_persistent_session_keeps_lost_reply_history_without_redispatch() {
    run("persistent-lost", Scenario::LostReply, Driver::Persistent);
}
#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser-standalone"]
fn chromium_standalone_trial_persistent_session_corrupt_observation_cannot_advance_or_emit_map() {
    run("persistent-corrupt", Scenario::Corrupt, Driver::Persistent);
}

#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser-standalone"]
fn chromium_standalone_trial_dedicated_worker_completes_and_preserves_redacted_port_results() {
    run("worker-complete", Scenario::Complete, Driver::Worker);
}
#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser-standalone"]
fn chromium_standalone_trial_dedicated_worker_lost_reply_recovers_without_another_upload() {
    run("worker-lost", Scenario::LostReply, Driver::Worker);
}
#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser-standalone"]
fn chromium_standalone_trial_dedicated_worker_cancellation_and_corruption_keep_exposure() {
    run("worker-corrupt", Scenario::Corrupt, Driver::Worker);
}
