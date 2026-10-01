//! Usable tenant output after signed verification/completion; provider exposure/content are local.
mod sharing;
use super::*;
use crate::observation_provider::{incoming, serve};
use ic_blob_storage::{
    model::{identity::ContentDigest, service::read::download::CaffeineDownloadScope},
    ops::caffeine::download::request_target,
};
use std::{cell::Cell, io::Write};

struct Client<'a> {
    f: &'a Fixture,
    args: Vec<String>,
    report: PathBuf,
    sequence: Cell<u32>,
    permission: UploadAdmissionRequest,
}
impl Client<'_> {
    fn call(&self, args: &[String], code: i32) -> Value {
        let n = self.sequence.get() + 1;
        assert!(n <= 40, "bounded tenant download journey");
        self.sequence.set(n);
        save(&self.report, &format!("{n:02}-command.json"), &json!(args));
        let value = run(args, code);
        save(&self.report, &format!("{n:02}-result.json"), &value);
        value
    }
    fn args(&self, label: &str) -> Vec<String> {
        let mut args = self.args.clone();
        args.extend([
            "--run-dir".into(),
            self.report.join(label).to_str().unwrap().into(),
        ]);
        args
    }
    fn reference_args(&self, command: &str, file: &str, label: Option<&str>) -> Vec<String> {
        let mut args = vec![command.into()];
        let (pairs, remainder) = self.args[1..].as_chunks::<2>();
        assert!(remainder.is_empty());
        for pair in pairs {
            if !["--project", "--gateway", "--max-bytes"].contains(&pair[0].as_str()) {
                args.extend_from_slice(pair);
            }
        }
        change(
            &mut args,
            "--request",
            self.report.join(file).to_str().unwrap(),
        );
        if let Some(label) = label {
            args.extend([
                "--run-dir".into(),
                self.report.join(label).display().to_string(),
            ]);
        }
        args
    }
    fn prepare_reference(&self, request: ReferenceCommand, directory: &str) {
        assert_eq!(request.upload, self.permission.upload);
        let args = vec![
            "reference-inputs".into(),
            "--permission".into(),
            self.report.join("permission.candid").display().to_string(),
            "--action".into(),
            match request.action {
                ReferenceAction::Retain => "retain",
                ReferenceAction::Release => "release",
            }
            .into(),
            "--reference".into(),
            request.reference.to_string(),
            "--operation".into(),
            request.operation.to_string(),
            "--run-dir".into(),
            self.report.join(directory).display().to_string(),
        ];
        let result = self.call(&args, 0);
        assert_eq!(result["service_dispatched"], false);
        assert_eq!(result["identities_allocated"], false);
    }
    fn refuse(&self, label: &str, error: &str) {
        let value = self.call(&self.args(label), 3);
        assert_eq!(value["error"], error);
        let path = self.report.join(label);
        assert!(!path.join("download-request.json").exists());
        assert!(!path.join("body.bin").exists());
    }
    fn fetch(
        &self,
        listener: &TcpListener,
        request_file: &str,
        label: &str,
        header: &str,
        body: &[u8],
        error: Option<&str>,
    ) {
        let path = self.report.join(label);
        let scope =
            CaffeineDownloadScope::new(self.f.app(), u128::MAX.try_into().unwrap(), "fixture/β?&=")
                .unwrap();
        let target = request_target(
            &scope,
            self.permission.upload.root.as_slice().try_into().unwrap(),
        );
        let value = std::thread::scope(|threads| {
            let server = threads.spawn(|| {
                let (mut stream, request) = incoming(listener);
                assert_eq!(
                    request.lines().next().unwrap(),
                    format!("GET {target} HTTP/1.1")
                );
                assert!(!request.to_lowercase().contains("authorization:"));
                for file in [
                    "plan.json",
                    "request.candid",
                    "service-response.candid",
                    "download-request.json",
                ] {
                    assert!(path.join(file).is_file());
                }
                assert!(!path.join("body.bin").exists());
                std::fs::write(path.join("fixture-http-request.txt"), request).unwrap();
                let mut reply = header.as_bytes().to_vec();
                reply.extend_from_slice(body);
                std::fs::write(path.join("fixture-http-response.bin"), &reply).unwrap();
                let _ = stream.write_all(&reply);
            });
            let mut args = self.args(label);
            change(
                &mut args,
                "--request",
                self.report.join(request_file).to_str().unwrap(),
            );
            let value = self.call(&args, if error.is_some() { 3 } else { 0 });
            server.join().unwrap();
            value
        });
        if let Some(error) = error {
            assert_eq!(value["error"], error);
            assert!(!path.join("body.bin").exists());
            assert!(!path.join("summary.json").exists());
            assert!(std::fs::metadata(path.join("body.part")).unwrap().len() <= 10);
        } else {
            assert_eq!(value["verified"], true);
            assert_eq!(value["descriptor_authentication"], "ic_update_certificate");
            let request: DownloadRequest =
                candid::decode_one(&std::fs::read(self.report.join(request_file)).unwrap())
                    .unwrap();
            assert_eq!(value["request"]["reference"], request.reference.to_string());
            assert_eq!(value["project"], scope.project());
            assert_eq!(value["bytes"], "10");
            assert_eq!(
                value["content_digest"],
                ContentDigest::compute(&[42; 10]).to_string()
            );
            assert_eq!(std::fs::read(path.join("body.bin")).unwrap(), [42; 10]);
            assert!(!path.join("body.part").exists());
            assert_eq!(value["publication_authorized"], false);
            assert_eq!(value["retry_authorized"], false);
        }
    }
}
fn no_get(listener: &TcpListener) {
    listener.set_nonblocking(true).unwrap();
    assert_eq!(
        listener.accept().unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
}
fn confirm(
    f: &Fixture,
    keys: &Path,
    url: &str,
    report: &Path,
    listener: &TcpListener,
    permission: UploadAdmissionRequest,
    gateway: &str,
) {
    let directory = keys.join("verifier");
    std::fs::create_dir(&directory).unwrap();
    std::fs::write(directory.join("identity.pem"), OUTSIDER_PEM).unwrap();
    std::fs::copy(keys.join("root.der"), directory.join("root.der")).unwrap();
    let verifier = BasicIdentity::from_raw_key(&[43; 32]).sender().unwrap();
    let base = verifier::arguments(f, verifier, url, &directory);
    let observed = report.join("observed");
    let mut args = verifier::command(&base, "observe-upload", &observed);
    args.extend([
        "--permission".into(),
        report.join("permission.candid").to_str().unwrap().into(),
        "--gateway".into(),
        gateway.into(),
        "--max-bytes".into(),
        "10".into(),
    ]);
    let scope =
        CaffeineDownloadScope::new(f.app(), u128::MAX.try_into().unwrap(), "fixture/β?&=").unwrap();
    let target = request_target(
        &scope,
        permission.upload.root.as_slice().try_into().unwrap(),
    );
    std::thread::scope(|threads| {
        let server = threads.spawn(|| serve(listener, &observed, &target, &[42; 10]));
        save(report, "verifier-observe-command.json", &json!(args));
        let output = run(&args, 0);
        save(report, "verifier-observe-result.json", &output);
        server.join().unwrap();
    });
    let args = verifier::command(&base, "submit-attestation", &observed);
    save(report, "verifier-submit-command.json", &json!(args));
    let output = run(&args, 0);
    save(report, "verifier-submit-result.json", &output);
    assert_eq!(output["outcome"], "accepted");
}
fn refusals(t: &Client<'_>, keys: &Path, listener: &TcpListener) {
    let mut args = t.args("wrong-identity");
    change(
        &mut args,
        "--identity",
        keys.join("verifier/identity.pem").to_str().unwrap(),
    );
    assert_eq!(t.call(&args, 3)["error"], "identity_binding");
    assert!(!t.report.join("wrong-identity").exists());
    let mut args = t.args("wrong-project");
    change(&mut args, "--project", "different");
    assert_eq!(t.call(&args, 3)["error"], "binding");
    assert!(
        !t.report
            .join("wrong-project/download-request.json")
            .exists()
    );
    let mut request: DownloadRequest = candid::decode_one(
        &std::fs::read(t.report.join("reference-inputs/download.candid")).unwrap(),
    )
    .unwrap();
    request.reference -= 1;
    std::fs::write(
        t.report.join("wrong-reference.candid"),
        candid::encode_one(request).unwrap(),
    )
    .unwrap();
    let mut args = t.args("wrong-reference");
    change(
        &mut args,
        "--request",
        t.report.join("wrong-reference.candid").to_str().unwrap(),
    );
    assert_eq!(t.call(&args, 3)["error"], "download_unavailable");
    assert!(
        !t.report
            .join("wrong-reference/download-request.json")
            .exists()
    );
    no_get(listener);
}
fn release_and_restore(t: &Client<'_>, listener: &TcpListener) {
    let retained = sharing::retain(t);
    let status = t.reference_args(
        "reference-status",
        "reference-inputs/reference-status.candid",
        None,
    );
    assert_eq!(t.call(&status, 0)["live"], true);
    let submit = t.reference_args(
        "submit-reference",
        "reference-inputs/reference.candid",
        Some("release"),
    );
    let result = t.call(&submit, 0);
    assert_eq!(result["outcome"], "recorded");
    assert_eq!(result["result"]["state"], "success");
    assert_eq!(result["provider_deletion"], "not_established");
    assert_eq!(result["billing_cessation"], "not_established");
    let receipt = t.reference_args(
        "reference-receipt",
        "reference-inputs/reference.candid",
        None,
    );
    assert_eq!(t.call(&receipt, 0)["outcome"], "found");
    assert_eq!(t.call(&status, 0)["live"], false);
    let before = t.f.pic().get_stable_memory(t.f.app());
    t.refuse("released", "download_unavailable");
    no_get(listener);
    assert_eq!(t.f.pic().get_stable_memory(t.f.app()), before);
    sharing::release_last(t, listener, &retained);
    t.f.pic().stop_progress();
    t.f.upgrade_same_release(Duration::from_secs(5));
    t.f.pic().auto_progress();
    let before = t.f.pic().get_stable_memory(t.f.app());
    t.refuse("restored", "download_fenced");
    let restored_status = t.call(&status, 0);
    assert_eq!(restored_status["live"], false);
    assert_eq!(restored_status["fenced"], true);
    assert_eq!(t.call(&receipt, 0)["outcome"], "found");
    sharing::restored(t, listener, &retained);
    no_get(listener);
    assert_eq!(t.f.pic().get_stable_memory(t.f.app()), before);
    assert_eq!(
        std::fs::read(t.report.join("good/body.bin")).unwrap(),
        [42; 10]
    );
}
#[test]
fn managed_native_tenant_download_returns_verified_file_after_completion_and_refuses_released_or_restored_references()
 {
    let temp = tempfile::tempdir().unwrap();
    let report = std::env::var_os("BLOB_TENANT_DOWNLOAD_REPORT").map_or_else(
        || temp.path().join("managed"),
        |p| PathBuf::from(p).join("managed"),
    );
    std::fs::create_dir(&report).unwrap();
    let tenant = BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap();
    let mut input = blob_canic_probe::configuration::input();
    input.completion_verifier = BasicIdentity::from_raw_key(&[43; 32]).sender().unwrap();
    let f = Fixture::with_input(&input);
    let scope = TenantScope {
        service: f.app(),
        tenant,
        namespace: u128::MAX,
    };
    update_tenant(&f, scope, None, true);
    let permission = prepare(&f, scope);
    std::fs::write(
        report.join("permission.candid"),
        candid::encode_one(permission).unwrap(),
    )
    .unwrap();
    std::fs::write(temp.path().join("identity.pem"), PEM).unwrap();
    let root = local_subnet_key(&f);
    std::fs::write(temp.path().join("root.der"), &root).unwrap();
    std::fs::write(report.join("fixture-root.der"), root).unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let gateway = format!("http://{}/", listener.local_addr().unwrap());
    save(
        &report,
        "fixture-plan.json",
        &json!({"evidence":"local_exposure_and_ten_byte_source_substitute","service":f.app().to_text(),"tenant":tenant.to_text(),"verifier":input.completion_verifier.to_text(),"max_cli_invocations":40,"max_source_gets":7,"max_content_bytes":"10","deployed_provider_requests":0,"attached_provider_cycles":"0"}),
    );
    let (mut replica, url) = live(&f);
    let mut args = verifier::arguments(&f, tenant, &url, temp.path());
    args[0] = "download".into();
    args.extend([
        "--request".into(),
        report
            .join("reference-inputs/download.candid")
            .to_str()
            .unwrap()
            .into(),
        "--project".into(),
        input.project.clone(),
        "--gateway".into(),
        gateway.clone(),
        "--max-bytes".into(),
        "10".into(),
    ]);
    let t = Client {
        f: &f,
        args,
        report: report.clone(),
        sequence: Cell::new(0),
        permission,
    };
    t.prepare_reference(
        ReferenceCommand {
            upload: permission.upload,
            reference: permission.upload.first_reference,
            operation: u128::MAX,
            action: ReferenceAction::Release,
        },
        "reference-inputs",
    );
    t.refuse("unconfirmed", "download_unavailable");
    no_get(&listener);
    confirm(
        &f,
        temp.path(),
        &url,
        &report,
        &listener,
        permission,
        &gateway,
    );
    let before = f.pic().get_stable_memory(f.app());
    t.fetch(
        &listener,
        "reference-inputs/download.candid",
        "good",
        "HTTP/1.1 200 OK\r\nContent-Type: misleading/type\r\nConnection: close\r\n\r\n",
        &[42; 10],
        None,
    );
    assert_eq!(t.call(&t.args("good"), 3)["error"], "new_run_required");
    no_get(&listener);
    refusals(&t, temp.path(), &listener);
    corrupt_responses(&t, &listener);
    no_get(&listener);
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
    release_and_restore(&t, &listener);
    save(
        &report,
        "fixture-summary.json",
        &json!({"cli_invocations":t.sequence.get()+2,"source_gets":7,"verified_bytes":"10","deployed_provider_requests":0,"attached_provider_cycles":"0"}),
    );
    replica.stop_live();
}

fn corrupt_responses(t: &Client<'_>, listener: &TcpListener) {
    for (label, header, body, error) in [
        (
            "corrupt",
            "HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n",
            vec![41; 10],
            "content_mismatch",
        ),
        (
            "short",
            "HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n",
            vec![42; 9],
            "content_mismatch",
        ),
        (
            "long",
            "HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n",
            vec![42; 11],
            "content_mismatch",
        ),
        (
            "redirect",
            "HTTP/1.1 302 Found\r\nLocation: http://127.0.0.1:9\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            vec![],
            "provider_response",
        ),
    ] {
        t.fetch(
            listener,
            "reference-inputs/download.candid",
            label,
            header,
            &body,
            Some(error),
        );
    }
}
