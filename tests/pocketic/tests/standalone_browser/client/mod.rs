//! Current native tooling supplies exact files and real signed standalone requests.
use super::super::*;
use super::BrowserPreparation;
use crate::{
    account_native_cli::OUTSIDER_PEM,
    authenticated_cli::{PEM, run_with_tls_roots},
};
use ic_agent::{Identity, identity::BasicIdentity};
use ic_blob_storage::{
    dto::download::DownloadRequest,
    model::identity::{ContentDigest, caffeine::manifest::CaffeineManifestLimits},
    ops::caffeine::preparation::{PreparedManifestLimits, decode_prepared_manifest},
};
use std::{
    num::{NonZeroU64, NonZeroUsize},
    path::PathBuf,
};
pub(super) const PROJECT: &str = "standalone-trial";
// Fixed test-only seed [44;32], distinct from uploader [42;32] and tenant [43;32].
const VERIFIER_PEM: &str = "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEICwsLCwsLCwsLCwsLCwsLCwsLCwsLCwsLCwsLCwsLCws\n-----END PRIVATE KEY-----\n";
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct UploadReport {
    pub gateway: String,
    pub uploaded: bool,
    pub recovered: bool,
    pub root: String,
    pub puts: Vec<serde_json::Value>,
    pub provider_completion: bool,
    pub upload_failure: Option<serde_json::Value>,
    pub final_request_phase: String,
}
pub(super) struct Trial {
    pub f: Fixture,
    pub url: String,
    pub report: PathBuf,
    _temporary: tempfile::TempDir,
}
fn limits() -> CaffeineManifestLimits {
    CaffeineManifestLimits {
        max_content_bytes: NonZeroU64::new(1024).unwrap(),
        max_chunks: NonZeroUsize::MIN,
        max_headers: NonZeroUsize::new(8).unwrap(),
        max_header_bytes: NonZeroUsize::new(1024).unwrap(),
    }
}
impl Trial {
    pub fn new(label: &str) -> Self {
        Self::with_envelope(label, Envelope::Single)
    }
    pub(super) fn with_envelope(label: &str, envelope: Envelope) -> Self {
        use ic_testkit::pocket_ic::PocketIcBuilder;
        let principal = |seed| BasicIdentity::from_raw_key(&[seed; 32]).sender().unwrap();
        let mut f = Fixture::with_profile(
            Harness::with_builder(
                PocketIcBuilder::new()
                    .with_nns_subnet()
                    .with_application_subnet(),
            ),
            Fake::principal(5),
            if matches!(envelope, Envelope::Serial) {
                principal(42)
            } else {
                Fake::principal(2)
            },
            principal(42),
            envelope,
            principal(44),
            PROJECT,
        );
        f.tenant = principal(43);
        f.enroll(f.operator).unwrap();
        let temporary = tempfile::tempdir().unwrap();
        let report = std::env::var_os("BLOB_STANDALONE_BROWSER_REPORT")
            .map_or_else(|| temporary.path().to_path_buf(), PathBuf::from)
            .join(label);
        std::fs::create_dir(&report).unwrap();
        for (name, bytes) in [
            ("uploader.pem", PEM),
            ("tenant.pem", OUTSIDER_PEM),
            ("verifier.pem", VERIFIER_PEM),
        ] {
            std::fs::write(report.join(name), bytes).unwrap();
        }
        std::fs::write(report.join("root.der"), f.harness.pic.root_key().unwrap()).unwrap();
        let url = f
            .harness
            .pic
            .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
            .to_string();
        Self {
            f,
            url,
            report,
            _temporary: temporary,
        }
    }
    pub fn record(&self, name: &str, value: &serde_json::Value) {
        std::fs::write(
            self.report.join(name),
            serde_json::to_vec_pretty(value).unwrap(),
        )
        .unwrap();
    }
    pub(super) fn args(&self, command: &str, identity: &str, actor: Principal) -> Vec<String> {
        [
            command,
            "--network",
            "local",
            "--url",
            &self.url,
            "--identity",
            self.report.join(identity).to_str().unwrap(),
            "--root-key",
            self.report.join("root.der").to_str().unwrap(),
            "--actor",
            &actor.to_text(),
            "--service",
            &self.f.service.to_text(),
            "--namespace",
            &self.f.config.namespace.to_string(),
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    }
    pub(super) fn invoke(&self, label: &str, args: &[String], code: i32) -> serde_json::Value {
        self.record(&format!("{label}-command.json"), &serde_json::json!(args));
        let roots = self.report.join("gateway-ca.pem");
        let result = run_with_tls_roots(args, code, Some(&roots));
        self.record(&format!("{label}-result.json"), &result);
        result
    }
    pub fn inputs(
        &self,
        expected: &UploadManifestRequest,
        browser: &BrowserPreparation,
    ) -> UploadManifestRequest {
        assert_eq!(browser.byte_length, 1024);
        assert_eq!(browser.hash, crate::standalone_certificate::root(expected));
        let declaration = decode_prepared_manifest(
            browser.hash.parse().unwrap(),
            browser.byte_length,
            browser.manifest_json.as_bytes(),
            PreparedManifestLimits {
                max_json_bytes: NonZeroUsize::new(4096).unwrap(),
                manifest: limits(),
            },
        )
        .unwrap();
        assert_eq!(declaration, expected.declaration);
        std::fs::write(self.report.join("manifest.json"), &browser.manifest_json).unwrap();
        std::fs::write(self.report.join("source.bin"), [42; 1024]).unwrap();
        let p = expected.permission;
        let u = p.upload;
        self.record("binding.json", &serde_json::json!({"schema":1,"preparation":{"content_type":"image/png"},"project":PROJECT,"bucket":"local-standalone-trial","service":u.service.to_text(),
            "namespace":u.namespace.to_string(),"tenant":u.tenant.to_text(),"uploader":p.uploader.to_text(),
            "upload":u.upload.to_string(),"object":u.object.to_string(),"incarnation":u.incarnation.to_string(),
            "first_reference":u.first_reference.to_string(),"root":browser.hash,"bytes":u.bytes.to_string(),
            "expires_at_ns":p.expires_at_ns.to_string()}));
        let host = self.f.configuration(self.f.operator).unwrap();
        let carrier = ServiceInstallationInput {
            configuration: host.configuration,
            project: host.project,
            completion_verifier: host.completion_verifier,
            trusted_uploader: host.trusted_uploader,
        };
        std::fs::write(
            self.report.join("installation.candid"),
            candid::encode_one(carrier).unwrap(),
        )
        .unwrap();
        self.freeze();
        assert_eq!(
            std::fs::read(self.report.join("inputs/installation.candid")).unwrap(),
            std::fs::read(self.report.join("installation.candid")).unwrap()
        );
        let generated = candid::decode_one(
            &std::fs::read(self.report.join("inputs/file-0000/manifest.candid")).unwrap(),
        )
        .unwrap();
        assert_eq!(expected, &generated);
        generated
    }
    fn freeze(&self) {
        let hash = |name: &str| {
            ContentDigest::compute(&std::fs::read(self.report.join(name)).unwrap())
                .to_string()
                .trim_start_matches("sha256:")
                .to_owned()
        };
        self.record(
            "inventory.json",
            &serde_json::json!({"schema":1,"files":[{
            "binding":"binding.json","binding_sha256":hash("binding.json"),
            "manifest":"manifest.json","manifest_sha256":hash("manifest.json"),
            "body":"source.bin","body_sha256":hash("source.bin")}]}),
        );
        let args = [
            "publish-inputs",
            "--inventory",
            self.report.join("inventory.json").to_str().unwrap(),
            "--root",
            self.report.to_str().unwrap(),
            "--installation",
            self.report.join("installation.candid").to_str().unwrap(),
            "--max-bytes",
            "1024",
            "--max-total-bytes",
            "1024",
            "--run-dir",
            self.report.join("inputs").to_str().unwrap(),
        ]
        .map(str::to_owned);
        assert_eq!(self.invoke("inputs", &args, 0)["all_bodies_verified"], true);
    }
    pub fn setup(&self, input: &UploadManifestRequest) {
        let mut args = self.args("publish-prepare", "tenant.pem", self.f.tenant);
        args.extend([
            "--inputs".into(),
            self.report.join("inputs").display().to_string(),
            "--uploader-identity".into(),
            self.report.join("uploader.pem").display().to_string(),
            "--file-index".into(),
            "0".into(),
            "--max-bytes".into(),
            "1024".into(),
            "--max-total-bytes".into(),
            "1024".into(),
            "--timeout-seconds".into(),
            "30".into(),
            "--run-dir".into(),
            self.report.join("setup").display().to_string(),
        ]);
        assert_eq!(self.invoke("setup", &args, 0)["state"], "prepared");
        assert_eq!(
            self.f.admission(input.permission).state,
            ic_blob_storage::dto::upload::UploadState::Reserved
        );
        assert_eq!(
            crate::standalone_certificate::inspect(
                &self.f,
                self.f.uploader,
                &crate::standalone_certificate::root(input)
            )
            .unwrap()
            .blockers,
            []
        );
    }
    pub fn transfer_inputs(&self) -> serde_json::Value {
        let root = self.report.join("inputs/file-0000");
        let body = std::fs::read(root.join("body.bin")).unwrap();
        let binding: serde_json::Value =
            serde_json::from_slice(&std::fs::read(root.join("certificate-binding.json")).unwrap())
                .unwrap();
        serde_json::json!({"binding":binding,"snapshot":{"body":body,
            "bodySha256":ContentDigest::compute(&body).to_string().trim_start_matches("sha256:"),
            "manifestJSON":std::fs::read_to_string(root.join("manifest.json")).unwrap()}})
    }
    pub fn observe(
        &self,
        input: &UploadManifestRequest,
        gateway: &str,
        code: i32,
    ) -> serde_json::Value {
        let args = self.observation_args(input, gateway, "observation");
        self.invoke("observation", &args, code)
    }
    pub fn observe_with_untrusted_root(
        &self,
        input: &UploadManifestRequest,
        gateway: &str,
    ) -> serde_json::Value {
        let label = "untrusted-observation";
        let args = self.observation_args(input, gateway, label);
        self.record(&format!("{label}-command.json"), &serde_json::json!(args));
        let result = run_with_tls_roots(&args, 3, Some(&self.report.join("untrusted-ca.pem")));
        self.record(&format!("{label}-result.json"), &result);
        result
    }
    pub fn observe_refused_stream(
        &self,
        input: &UploadManifestRequest,
        gateway: &str,
    ) -> serde_json::Value {
        let label = "refused-observation";
        let args = self.observation_args(input, gateway, label);
        self.invoke(label, &args, 3)
    }
    fn observation_args(
        &self,
        input: &UploadManifestRequest,
        gateway: &str,
        label: &str,
    ) -> Vec<String> {
        let verifier = self
            .f
            .configuration(self.f.operator)
            .unwrap()
            .completion_verifier;
        let mut args = self.args("observe-upload", "verifier.pem", verifier);
        args.extend([
            "--permission".into(),
            self.report
                .join("inputs/file-0000/permission.candid")
                .display()
                .to_string(),
            "--gateway".into(),
            gateway.into(),
            "--max-bytes".into(),
            input.permission.upload.bytes.to_string(),
            "--run-dir".into(),
            self.report.join(label).display().to_string(),
        ]);
        args
    }
    pub fn attest(&self, expected: i32, label: &str) -> serde_json::Value {
        let verifier = self
            .f
            .configuration(self.f.operator)
            .unwrap()
            .completion_verifier;
        let mut args = self.args("submit-attestation", "verifier.pem", verifier);
        args.extend([
            "--run-dir".into(),
            self.report.join("observation").display().to_string(),
        ]);
        self.invoke(label, &args, expected)
    }
    pub fn download(
        &self,
        input: &UploadManifestRequest,
        gateway: &str,
        label: &str,
        code: i32,
    ) -> serde_json::Value {
        let u = input.permission.upload;
        let request = DownloadRequest {
            service: u.service,
            tenant: u.tenant,
            namespace: u.namespace,
            root: u.root,
            object: u.object,
            incarnation: u.incarnation,
            reference: u.first_reference,
        };
        let file = self.report.join(format!("{label}.candid"));
        std::fs::write(&file, candid::encode_one(request).unwrap()).unwrap();
        let mut args = self.args("download", "tenant.pem", self.f.tenant);
        args.extend([
            "--request".into(),
            file.display().to_string(),
            "--gateway".into(),
            gateway.into(),
            "--project".into(),
            PROJECT.into(),
            "--max-bytes".into(),
            input.permission.upload.bytes.to_string(),
            "--run-dir".into(),
            self.report.join(label).display().to_string(),
        ]);
        let report = self.invoke(label, &args, code);
        if code == 3 {
            assert_eq!(report["error"], "download_unavailable");
        }
        report
    }
    pub fn release(&self, input: &UploadManifestRequest) {
        let command = ReferenceCommand {
            upload: input.permission.upload,
            reference: input.permission.upload.first_reference,
            operation: 7,
            action: ReferenceAction::Release,
        };
        std::fs::write(
            self.report.join("release-intent.candid"),
            candid::encode_one(command).unwrap(),
        )
        .unwrap();
        let result: Result<ReferenceMutationResponse, ReferenceFailure> = self
            .f
            .harness
            .pic
            .update_candid_as(
                self.f.service,
                self.f.tenant,
                "blob_apply_reference",
                (command,),
            )
            .unwrap();
        assert!(result.unwrap().receipt.result.is_ok());
    }
}
