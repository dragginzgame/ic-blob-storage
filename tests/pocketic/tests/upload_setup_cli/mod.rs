//! Signed local reservation/preparation/cancellation journey for the standalone host.
mod inputs;
mod phases;
use crate::{
    account_native_cli::OUTSIDER_PEM,
    authenticated_cli::{PEM, run},
    submission_proxy::{Dispatch, Proxy, Reply},
};
use candid::Principal;
use ic_blob_storage::dto::{
    configuration::ServiceInstallationInput, upload::manifest::UploadManifestRequest,
};
use serde_json::{Value, json};
use std::{
    cell::Cell,
    path::{Path, PathBuf},
};
pub(crate) struct Client {
    base: Vec<String>,
    report: PathBuf,
    temporary: tempfile::TempDir,
    count: Cell<u32>,
    permission: UploadManifestRequest,
    installation: ServiceInstallationInput,
}
pub(crate) fn uploader() -> Principal {
    use ic_agent::Identity;
    ic_agent::identity::BasicIdentity::from_raw_key(&[43; 32])
        .sender()
        .unwrap()
}
pub(crate) fn cancelled_totals(
    pic: &ic_testkit::pocket_ic::PocketIc,
    scope: ic_blob_storage::dto::operator::OperatorScope,
    operator: Principal,
) {
    use ic_blob_storage::dto::operator::{LocalServiceStatus, LocalStatusFailure};
    use ic_testkit::pic::CandidCallExt;
    let status = pic
        .query_candid_as::<Result<LocalServiceStatus, LocalStatusFailure>, _>(
            scope.service,
            operator,
            "blob_local_status",
            (scope,),
        )
        .unwrap()
        .unwrap();
    assert_eq!(status.uploads.operations, 1);
    assert_eq!(status.uploads.active_reservations, 0);
    assert_eq!(status.uploads.reserved_bytes, 0);
    assert_eq!(status.uploads.logical_bytes, 0);
    assert_eq!(status.uploads.physical_bytes, 0);
    assert_eq!(status.uploads.liability_bytes, 0);
}
fn save(path: &Path, value: &[u8]) {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap();
    file.write_all(value).unwrap();
}
fn change(args: &mut [String], flag: &str, value: &str) {
    let n = args.iter().position(|a| a == flag).unwrap();
    args[n + 1] = value.into();
}
impl Client {
    pub fn new(
        permission: UploadManifestRequest,
        body: &[u8],
        url: &str,
        trusted: &[u8],
        label: &str,
        installation: ServiceInstallationInput,
    ) -> Self {
        let temporary = tempfile::tempdir().unwrap();
        let report = std::env::var_os("BLOB_UPLOAD_SETUP_REPORT").map_or_else(
            || temporary.path().join(label),
            |p| PathBuf::from(p).join(label),
        );
        std::fs::create_dir(&report).unwrap();
        save(&temporary.path().join("tenant.pem"), PEM.as_bytes());
        save(
            &temporary.path().join("uploader.pem"),
            OUTSIDER_PEM.as_bytes(),
        );
        save(&temporary.path().join("root.der"), trusted);
        save(&report.join("root.der"), trusted);
        save(&report.join("plan.json"),&serde_json::to_vec_pretty(&json!({"evidence":"signed_local_upload_setup",
            "service":permission.permission.upload.service.to_text(),"tenant":permission.permission.upload.tenant.to_text(),
            "uploader":permission.permission.uploader.to_text(),"max_cli_invocations":32,"max_local_updates":8,
            "max_service_traffic_bytes":16*1024*1024,"provider_requests":0,"attached_provider_cycles":"0",
            "snapshot_bytes":permission.permission.upload.bytes.to_string(),"local_manifest_source":"maintained_rust_fixture_upstream_format",
            "client_deadline_seconds":30,"fixture_exposure_or_completion":false})).unwrap());
        let base: Vec<String> = [
            "--network",
            "local",
            "--url",
            url,
            "--service",
            &permission.permission.upload.service.to_text(),
            "--namespace",
            &permission.permission.upload.namespace.to_string(),
            "--root-key",
            temporary.path().join("root.der").to_str().unwrap(),
        ]
        .into_iter()
        .map(str::to_owned)
        .collect();
        let client = Self {
            base,
            report,
            temporary,
            count: Cell::new(0),
            permission,
            installation,
        };
        inputs::prepare(&client, body);
        client
    }
    fn args(&self, command: &str, label: Option<&str>, as_uploader: bool) -> Vec<String> {
        let mut args = vec![command.into()];
        args.extend(self.base.clone());
        let p = if as_uploader {
            self.permission.permission.uploader
        } else {
            self.permission.permission.upload.tenant
        };
        args.extend([
            "--identity".into(),
            self.temporary
                .path()
                .join(if as_uploader {
                    "uploader.pem"
                } else {
                    "tenant.pem"
                })
                .to_str()
                .unwrap()
                .into(),
            "--actor".into(),
            p.to_text(),
            "--request".into(),
            self.report
                .join("inputs")
                .join(if command == "prepare-upload" {
                    "manifest.candid"
                } else {
                    "permission.candid"
                })
                .to_str()
                .unwrap()
                .into(),
        ]);
        if let Some(label) = label {
            args.extend([
                "--run-dir".into(),
                self.report.join(label).to_str().unwrap().into(),
            ]);
        }
        args
    }
    fn call(&self, args: &[String], exit: i32) -> Value {
        let n = self.count.get() + 1;
        assert!(n <= 32);
        self.count.set(n);
        save(
            &self.report.join(format!("{n:02}-command.json")),
            &serde_json::to_vec(args).unwrap(),
        );
        let value = run(args, exit);
        save(
            &self.report.join(format!("{n:02}-result.json")),
            &serde_json::to_vec_pretty(&value).unwrap(),
        );
        value
    }
    fn dispatch(&self, command: &str, label: &str, reply: Reply) -> Value {
        let uploader = command == "prepare-upload";
        let method = match command {
            "admit-upload" => "blob_admit_upload",
            "prepare-upload" => "blob_prepare_upload",
            "revoke-upload" => "blob_revoke_upload",
            _ => unreachable!(),
        };
        let actual = if uploader {
            self.permission.permission.uploader
        } else {
            self.permission.permission.upload.tenant
        };
        let directory = self.report.join(label);
        let proxy = Proxy::start(
            self.base[3].clone(),
            directory.clone(),
            Dispatch {
                service: self.permission.permission.upload.service,
                actor: actual,
                method,
                argument_file: "request.candid",
            },
            reply,
        );
        let mut args = self.args(command, Some(label), uploader);
        change(&mut args, "--url", &proxy.url);
        let result = self.call(&args, if reply == Reply::Drop { 3 } else { 0 });
        assert_eq!(proxy.calls(), 1);
        let recorded: Value =
            serde_json::from_slice(&std::fs::read(directory.join("outcome.json")).unwrap())
                .unwrap();
        assert_eq!(
            recorded["outcome"],
            match reply {
                Reply::Drop => "uncertain",
                Reply::Pending => "pending",
                Reply::Pass => "acknowledged",
            }
        );
        assert_eq!(recorded["certificate_issued"], false);
        assert_eq!(recorded["provider_requests"], 0);
        assert_eq!(recorded["retry_authorized"], false);
        let original = std::fs::read(directory.join("outcome.json")).unwrap();
        assert_eq!(self.call(&args, 3)["error"], "submission_already_claimed");
        let mut recovery = self.args(
            if uploader {
                "upload-manifest"
            } else {
                "upload-permission"
            },
            None,
            uploader,
        );
        change(&mut recovery, "--url", &proxy.url);
        change(
            &mut recovery,
            "--request",
            directory.join("permission.candid").to_str().unwrap(),
        );
        let observed = self.call(&recovery, 0);
        assert_eq!(observed["retry_authorized"], false);
        assert_eq!(
            std::fs::read(directory.join("outcome.json")).unwrap(),
            original
        );
        assert_eq!(proxy.calls(), 1);
        if uploader {
            assert!(observed["observation"]["manifest"].is_object());
        } else {
            assert_eq!(
                observed["observation"]["revoked"],
                command == "revoke-upload"
            );
        }
        result
    }
    pub fn before_restore(&self) {
        phases::before_restore(self);
    }
    pub fn restored(&self) {
        phases::restored(self);
    }
}
