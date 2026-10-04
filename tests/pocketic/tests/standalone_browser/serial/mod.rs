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
    VerifiedWorker,
    CoordinatedWorker,
}
impl Driver {
    fn script(self) -> &'static str {
        match self {
            Self::VerifiedWorker | Self::CoordinatedWorker => {
                "../../.tmp/browser/launcher-serial.mjs"
            }
            _ => "serial.mjs",
        }
    }
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
    fn validate_plan(&self, plans: &BrowserPublicationPlan) {
        assert_eq!(plans.files.len(), self.files.len());
        for (plan, file) in plans.files.iter().zip(&self.files) {
            assert_eq!(plan.byte_length, file.permission.upload.bytes);
            assert_eq!(plan.hash, crate::standalone_certificate::root(file));
        }
    }
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
        *control = Control::Persistent(self.session(gateway, "resumed-session", false));
        assert_eq!(
            control.phase(&json!({"phase":"status","index":0}))["file_live"],
            false
        );
        let recovered = control.phase(&json!({"phase":"prepare","index":0}));
        assert_eq!(recovered["original_run"], *original);
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
    fn grant(
        &self,
        control: &mut Control,
        browser: &mut BrowserDriver,
        driver: Driver,
        index: usize,
        prepared: &Value,
    ) {
        if !matches!(driver, Driver::VerifiedWorker) {
            browser.send(&self.transfer(index, prepared));
            return;
        }
        assert!(prepared["transfer"].is_null());
        let handoff = control.phase(&json!({"phase":"transfer","index":index}));
        assert_eq!(handoff["recovery"], false);
        self.trial
            .record(&format!("transfer-{index}-phase.json"), &handoff);
        let recovery = control.phase(&json!({"phase":"transfer","index":index}));
        assert_eq!(recovery["recovery"], true);
        assert_eq!(recovery["service_updates"], 0);
        self.trial
            .record(&format!("transfer-{index}-recovery.json"), &recovery);
        browser.send(
            &json!({"index":index,"native_phase":handoff["native_phase"],
            "recovery_phase":recovery["native_phase"]}),
        );
    }
    fn session_args(&self, gateway: &str, label: &str, verify: bool) -> Vec<String> {
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
        if verify {
            args.extend([
                "--verifier-identity".into(),
                self.trial.report.join("verifier.pem").display().to_string(),
            ]);
        }
        let browser_selection = self.trial.report.join("browser-selection.json");
        let selected_browser =
            browser_selection.is_file() && matches!(label, "session" | "resumed-session");
        if selected_browser {
            args.extend([
                "--browser-selection".into(),
                browser_selection.display().to_string(),
            ]);
        }
        if label == "resumed-session" {
            args.extend([
                "--source-session".into(),
                self.trial.report.join("session").display().to_string(),
            ]);
        }
        self.trial
            .record(&format!("{label}-command.json"), &json!(args));
        args
    }
    fn session(&self, gateway: &str, label: &str, verify: bool) -> NativeSession {
        let args = self.session_args(gateway, label, verify);
        let mut session = NativeSession::start_with_roots(
            &args,
            verify
                .then(|| self.trial.report.join("gateway-ca.pem"))
                .as_deref(),
        );
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
    fn unconfirmed(&self, control: &mut Control, index: usize, gateway: &str) {
        let before = self.status_using(
            control,
            Some(index),
            gateway,
            &format!("unconfirmed-{index}"),
        );
        assert_eq!(before["file_live"], false);
        assert_eq!(before["blockers"][0]["code"], "completion_unmatched");
        // A successful or lost SDK response occupies the same reservation.
        if index == 0 {
            assert_eq!(
                self.prepare_using(control, 1, "blocked-setup-1")["state"],
                "blocked"
            );
            assert!(!self.trial.report.join("blocked-setup-1/admission").exists());
        }
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
        let live = self.status_using(control, Some(index), gateway, &format!("confirmed-{index}"));
        self.confirmed(index, gateway, observation, &live);
    }
    fn confirmed(&self, index: usize, gateway: &str, observation: &Value, live: &Value) {
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
    fn verify_in_session(
        &self,
        control: &mut Control,
        index: usize,
        gateway: &str,
        scenario: Scenario,
        browser: &mut BrowserDriver,
    ) -> bool {
        if matches!(scenario, Scenario::Corrupt) {
            let Control::Persistent(mut session) = std::mem::replace(control, Control::Indexed)
            else {
                panic!("verifying session")
            };
            session.send(&json!({"phase":"verify","index":index}));
            let failure = session.finish(3);
            self.trial.record("verification-failure.json", &failure);
            self.refuse_corrupt_map(control, gateway, &failure);
            assert!(
                !self
                    .trial
                    .report
                    .join("session/verification-0000/observation/attestation")
                    .exists()
            );
            browser.send(&json!({"finish":true}));
            return true;
        }
        let verified = control.phase(&json!({"phase":"verify","index":index}));
        self.trial
            .record(&format!("verified-{index}.json"), &verified);
        assert_eq!(verified["attestation"]["outcome"], "accepted");
        assert_eq!(verified["file_live"], true);
        self.confirmed(
            index,
            gateway,
            &verified["observation"],
            &verified["publication"],
        );
        if index == 0 && matches!(scenario, Scenario::LostReply) {
            let Control::Persistent(session) = std::mem::replace(control, Control::Indexed) else {
                panic!("verifying session")
            };
            assert_eq!(session.finish(3)["error"], "transport");
            *control = Control::Persistent(self.session(gateway, "resumed-session", true));
            let recovered = control.phase(&json!({"phase":"verify","index":0}));
            self.trial.record("verification-recovery.json", &recovered);
            assert_eq!(recovered["attestation"]["outcome"], "matched");
            assert_eq!(recovered["max_provider_requests_this_phase"], 0);
            assert_eq!(recovered["file_live"], true);
            assert!(
                !self
                    .trial
                    .report
                    .join("resumed-session/verification-0000/observation")
                    .exists()
            );
        }
        if index == 1 {
            self.complete_map(control, gateway);
            self.reject_foreign_observation(gateway, scenario);
            self.reject_conflicting_observation(gateway);
            browser.send(&json!({"finish":true}));
        }
        false
    }
    fn reject_foreign_observation(&self, gateway: &str, scenario: Scenario) {
        let mut session = self.session(gateway, "foreign-observation", true);
        let original = if matches!(scenario, Scenario::LostReply) {
            "resumed-session"
        } else {
            "session"
        };
        session.send(&json!({"phase":"verify","index":0,
            "source_observation":self.trial.report.join(original).join("verification-0001/observation")}));
        let failure = session.finish(3);
        self.trial
            .record("foreign-observation-failure.json", &failure);
        assert_eq!(failure["error"], "binding");
        assert!(
            !self
                .trial
                .report
                .join("foreign-observation/verification-0000/attestation-query.candid")
                .exists()
        );
    }
    fn reject_conflicting_observation(&self, gateway: &str) {
        let source = self
            .trial
            .report
            .join("session/verification-0000/observation");
        let copy = self.trial.report.join("conflicting-observation-input");
        std::fs::create_dir(&copy).unwrap();
        for name in [
            "plan.json",
            "summary.json",
            "permission.candid",
            "service-response.candid",
            "download-request.json",
            "download-outcome.json",
            "http-response.json",
            "statement.candid",
        ] {
            std::fs::copy(source.join(name), copy.join(name)).unwrap();
        }
        // Same upload, verified digest and current live reference, but a distinct
        // observation time: content equality cannot reconcile another statement.
        let mut statement: ic_blob_storage::dto::upload::completion::UploadAttestationRequest =
            candid::decode_one(&std::fs::read(copy.join("statement.candid")).unwrap()).unwrap();
        statement.observed_at_ns += 1;
        let encoded = candid::encode_one(statement).unwrap();
        std::fs::write(copy.join("statement.candid"), &encoded).unwrap();
        let mut summary: Value =
            serde_json::from_slice(&std::fs::read(copy.join("summary.json")).unwrap()).unwrap();
        summary["observed_at_ns"] = json!(statement.observed_at_ns.to_string());
        summary["statement_sha256"] = json!(ContentDigest::compute(&encoded).to_string());
        std::fs::write(
            copy.join("summary.json"),
            serde_json::to_vec_pretty(&summary).unwrap(),
        )
        .unwrap();
        let mut session = self.session(gateway, "conflicting-observation", true);
        session.send(&json!({"phase":"verify","index":0,"source_observation":copy}));
        let failure = session.finish(3);
        self.trial
            .record("conflicting-observation-failure.json", &failure);
        assert_eq!(failure["error"], "verification_refused");
        let inspected: Value = serde_json::from_slice(
            &std::fs::read(
                self.trial
                    .report
                    .join("conflicting-observation/verification-0000/attestation-inspection.json"),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(inspected["outcome"], "conflict");
        assert!(
            !self
                .trial
                .report
                .join("conflicting-observation/media-map.json")
                .exists()
        );
        assert!(
            !self
                .trial
                .report
                .join("conflicting-observation/verification-0000/observation")
                .exists()
        );
    }
    fn coordinate(
        &self,
        control: &mut Control,
        browser: &mut BrowserDriver,
        gateway: &str,
        scenario: Scenario,
    ) {
        let repo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        browser.send(&json!({"native": {
            "binary":std::env::var("BLOB_CLI_BIN").unwrap(),
            "cwd":repo.canonicalize().unwrap(),
            "timeoutSeconds":120,
            "args":self.session_args(gateway,"session",true),
            "env":{"SSL_CERT_FILE":self.trial.report.join("gateway-ca.pem"),
                "SSL_CERT_DIR":null,"HTTP_PROXY":"http://127.0.0.1:9",
                "HTTPS_PROXY":"http://127.0.0.1:9","ALL_PROXY":"http://127.0.0.1:9","NO_PROXY":""}
        },"resumedArgs":self.session_args(gateway,"resumed-session",true)}));
        for ordinal in 0..34 {
            let request: Value = browser.read(8192);
            if !request["ready"].is_null() {
                assert_eq!(request["ready"]["event"], "ready");
                self.trial.record(
                    &format!("{}-ready.json", request["label"].as_str().unwrap()),
                    &request["ready"],
                );
                browser.send(&json!({"accepted":true}));
                continue;
            }
            if !request["restart"].is_null() {
                assert!(matches!(scenario, Scenario::LostReply));
                assert_eq!(request["restart"]["report"]["error"], "transport");
                assert_eq!(request["restart"]["exit_code"], 3);
                self.trial
                    .record("driver-interruption.json", &request["restart"]);
                browser.send(&json!({"accepted":true}));
                continue;
            }
            if !request["finished"].is_null() {
                let final_result = &request["finished"];
                self.trial.record("driver-native-result.json", final_result);
                assert_eq!(
                    final_result["exit_code"],
                    if matches!(scenario, Scenario::Corrupt) {
                        3
                    } else {
                        0
                    }
                );
                browser.send(&json!({"accepted":true}));
                if !matches!(scenario, Scenario::Corrupt) {
                    assert_eq!(final_result["report"]["all_references_live"], true);
                    return;
                }
                let failure = &final_result["report"];
                assert_eq!(failure["error"], "content_mismatch");
                self.trial
                    .record("driver-verification-failure.json", failure);
                self.refuse_corrupt_map(control, gateway, failure);
                return;
            }
            let frame = &request["frame"];
            let event = &request["event"];
            assert_eq!(event["event"], "phase");
            self.trial
                .record(&format!("driver-phase-{ordinal:04}.json"), event);
            if frame["phase"] == "verify" {
                let index = usize::try_from(frame["index"].as_u64().unwrap()).unwrap();
                assert_eq!(event["report"]["file_live"], true);
                self.confirmed(
                    index,
                    gateway,
                    &event["report"]["observation"],
                    &event["report"]["publication"],
                );
            }
            browser.send(&json!({"accepted":true}));
        }
        panic!("driver must finish within the two selected session budgets");
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
        "tenant":serial.trial.f.tenant.to_text(),"bucket":"fixture-bucket","worker":matches!(driver,Driver::Worker | Driver::VerifiedWorker | Driver::CoordinatedWorker),
        "coordinated":matches!(driver,Driver::CoordinatedWorker),
        "rootKey":serial.trial.f.harness.pic.root_key().unwrap(),"project":super::client::PROJECT,
        "lostFinalReply":matches!(scenario,Scenario::LostReply),"corruptRead":matches!(scenario,Scenario::Corrupt),
        "providerRootCertificate":serial.trial.report.join("gateway-ca.pem"),
        "profile":serial.trial.report.join("browser-profile"),
        "session":serial.trial.report.join("session"),
        "browserSelection":serial.trial.report.join("browser-selection.json")});
    let mut browser = BrowserDriver::start(&config, driver.script());
    let plans: BrowserPublicationPlan = browser.read(16384);
    let mut control = match driver {
        Driver::Indexed | Driver::CoordinatedWorker => Control::Indexed,
        Driver::Persistent | Driver::Worker | Driver::VerifiedWorker => {
            Control::Persistent(serial.session(
                &plans.gateway,
                "session",
                matches!(driver, Driver::VerifiedWorker),
            ))
        }
    };
    serial.validate_plan(&plans);
    if matches!(driver, Driver::CoordinatedWorker) {
        serial.coordinate(&mut control, &mut browser, &plans.gateway, scenario);
        serial.finish(browser, scenario);
        return;
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
        serial.grant(&mut control, &mut browser, driver, index, &prepared);
        let uploaded: Value = browser.read(8192);
        serial
            .trial
            .record(&format!("browser-upload-{index}.json"), &uploaded);
        let gateway = uploaded["gateway"].as_str().unwrap();
        serial.unconfirmed(&mut control, index, gateway);
        if index == 0
            && matches!(scenario, Scenario::LostReply)
            && matches!(driver, Driver::Persistent | Driver::Worker)
        {
            serial.restart_after_exposure(&mut control, gateway, &prepared["original_run"]);
        }
        let corrupt = matches!(scenario, Scenario::Corrupt);
        if matches!(driver, Driver::VerifiedWorker) {
            if serial.verify_in_session(&mut control, index, gateway, scenario, &mut browser) {
                break;
            }
            continue;
        }
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

#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser-standalone"]
fn chromium_standalone_trial_native_verifier_completes_exact_files_before_advancing() {
    run(
        "native-verifier-complete",
        Scenario::Complete,
        Driver::VerifiedWorker,
    );
}

#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser-standalone"]
fn chromium_standalone_trial_native_verifier_recovers_original_observation_without_another_get_or_update()
 {
    run(
        "native-verifier-lost",
        Scenario::LostReply,
        Driver::VerifiedWorker,
    );
}

#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser-standalone"]
fn chromium_standalone_trial_native_verifier_corruption_stops_before_attestation_and_next_file() {
    run(
        "native-verifier-corrupt",
        Scenario::Corrupt,
        Driver::VerifiedWorker,
    );
}

#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser-standalone"]
fn chromium_native_phase_driver_restarts_with_original_handoff_without_another_upload() {
    run(
        "native-driver-restart",
        Scenario::LostReply,
        Driver::CoordinatedWorker,
    );
}

#[test]
#[ignore = "Requires pinned browser packages and Chromium; run make test-browser-standalone"]
fn chromium_native_phase_driver_stops_on_corruption_before_attestation_and_next_file() {
    run(
        "native-driver-corrupt",
        Scenario::Corrupt,
        Driver::CoordinatedWorker,
    );
}
