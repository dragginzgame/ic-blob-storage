use super::*;
use crate::native::{execute, upload_inputs::tests as fixture};
use candid::Principal;
use ic_blob_storage::dto::{
    tenant::{TenantEnrollment, TenantScope},
    upload::{
        capacity::{UploadCapacityFailure, UploadCapacityResponse},
        discovery::{UploadDiscoveryFailure, UploadDiscoveryResponse},
        history::{UploadContentState, UploadHistoryEntry},
    },
};
use std::fs;

pub(in crate::native) fn frozen() -> tempfile::TempDir {
    let d = tempfile::tempdir().unwrap();
    let binding = serde_json::to_vec(&fixture::binding()).unwrap();
    let manifest = serde_json::to_vec(&fixture::manifest()).unwrap();
    let hash = crate::native::upload_inputs::digest;
    fs::write(d.path().join("binding.json"), &binding).unwrap();
    fs::write(d.path().join("manifest.json"), &manifest).unwrap();
    fs::write(d.path().join("body.bin"), b"abc").unwrap();
    fs::write(
        d.path().join("installation.candid"),
        candid::encode_one(fixture::installation(&fixture::binding())).unwrap(),
    )
    .unwrap();
    fs::write(d.path().join("inventory.json"),json!({"schema":1,"files":[{"binding":"binding.json","binding_sha256":hash(&binding),"manifest":"manifest.json","manifest_sha256":hash(&manifest),"body":"body.bin","body_sha256":hash(b"abc")}]}).to_string()).unwrap();
    let args = [
        "publish-inputs",
        "--inventory",
        d.path().join("inventory.json").to_str().unwrap(),
        "--root",
        d.path().to_str().unwrap(),
        "--installation",
        d.path().join("installation.candid").to_str().unwrap(),
        "--max-bytes",
        "10",
        "--max-total-bytes",
        "20",
        "--run-dir",
        d.path().join("batch").to_str().unwrap(),
    ]
    .map(str::to_owned);
    execute(&args).unwrap();
    d
}
pub(in crate::native) fn batch(d: &tempfile::TempDir) -> publish_inputs::PreparedBatch {
    publish_inputs::PreparedBatch::open_frozen(
        &d.path().join("batch"),
        NonZeroU64::new(10).unwrap(),
        NonZeroU64::new(20).unwrap(),
    )
    .unwrap()
}
pub(in crate::native) fn capacity(scope: TenantScope) -> UploadCapacityResponse {
    UploadCapacityResponse {
        scope,
        enrollment: TenantEnrollment {
            generation: 1,
            active: true,
        },
        max_object_bytes: 10,
        max_headers: 4,
        max_header_bytes: 1024,
        remaining_objects: 2,
        remaining_active_uploads: 1,
        remaining_manifest_chunks: 2,
        remaining_bytes: 20,
        fenced: false,
    }
}
fn inspect(
    d: &tempfile::TempDir,
    state: Option<UploadContentState>,
    mut c: UploadCapacityResponse,
) -> Value {
    let b = batch(d);
    let scope = c.scope;
    let run = Run::create(&d.path().join("checks")).unwrap();
    c.scope = scope;
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(observation::inspect(
            scope,
            &b.files,
            &run,
            |method, arg| {
                let bytes = if method
                    == ic_blob_storage::ops::service::uploads::capacity::UPLOAD_CAPACITY_METHOD
                {
                    assert_eq!(candid::decode_one::<TenantScope>(&arg).unwrap(), scope);
                    candid::encode_one(Ok::<_, UploadCapacityFailure>(c)).unwrap()
                } else {
                    let request = candid::decode_one(&arg).unwrap();
                    candid::encode_one(Ok::<_, UploadDiscoveryFailure>(UploadDiscoveryResponse {
                        request,
                        content: state.map(|state| UploadHistoryEntry {
                            request: b.files[0].input.permission.upload,
                            state,
                        }),
                        fenced: c.fenced,
                    }))
                    .unwrap()
                };
                std::future::ready(Ok(bytes))
            },
        ))
        .unwrap()
        .report
}
#[test]
fn snapshots_and_usable_requests_are_reverified_before_identity_or_network() {
    for file in [
        "body.bin",
        "binding.json",
        "manifest.json",
        "permission.candid",
        "manifest.candid",
        "certificate-binding.json",
        "installation.candid",
    ] {
        let d = frozen();
        let file = d.path().join("batch/file-0000").join(file);
        let mut bytes = fs::read(&file).unwrap();
        bytes[0] ^= 1;
        fs::write(&file, bytes).unwrap();
        assert!(
            publish_inputs::PreparedBatch::open_frozen(
                &d.path().join("batch"),
                NonZeroU64::new(10).unwrap(),
                NonZeroU64::new(20).unwrap()
            )
            .is_err()
        );
    }
    let d = frozen();
    fs::remove_file(d.path().join("batch/summary.json")).unwrap();
    assert!(
        publish_inputs::PreparedBatch::open_frozen(
            &d.path().join("batch"),
            NonZeroU64::new(10).unwrap(),
            NonZeroU64::new(20).unwrap()
        )
        .is_err()
    );
}
#[test]
fn absent_existing_and_retired_roots_never_infer_upload_or_retry_authority() {
    for (state, status, blocked) in [
        (None, "not_visible", false),
        (
            Some(UploadContentState::Reserved),
            "recover_existing_operation",
            true,
        ),
        (
            Some(UploadContentState::ExposurePossible),
            "recover_existing_operation",
            true,
        ),
        (Some(UploadContentState::Live), "live_requires_retain", true),
        (Some(UploadContentState::Cancelled), "retired_root", true),
        (
            Some(UploadContentState::DeletionPending),
            "retired_root",
            true,
        ),
        (
            Some(UploadContentState::ProviderDeleted),
            "retired_root",
            true,
        ),
        (Some(UploadContentState::Settled), "retired_root", true),
    ] {
        let d = frozen();
        let b = batch(&d);
        let u = b.files[0].input.permission.upload;
        let scope = TenantScope {
            service: u.service,
            namespace: u.namespace,
            tenant: u.tenant,
        };
        let report = inspect(&d, state, capacity(scope));
        assert_eq!(report["files"][0]["status"], status);
        assert_eq!(report["blocked"], blocked);
        assert_eq!(report["retry_authorized"], false);
        assert_eq!(report["admission_proven"], false);
        assert_eq!(
            report["not_visible_demand"]["objects"],
            u64::from(state.is_none())
        );
    }
}
#[test]
fn independent_headroom_fences_and_suspension_are_visible() {
    let d = frozen();
    let b = batch(&d);
    let u = b.files[0].input.permission.upload;
    let scope = TenantScope {
        service: u.service,
        namespace: u.namespace,
        tenant: u.tenant,
    };
    let mut c = capacity(scope);
    c.remaining_objects = 0;
    c.remaining_active_uploads = 0;
    c.remaining_manifest_chunks = 0;
    c.remaining_bytes = 0;
    c.fenced = true;
    c.enrollment.active = false;
    let report = inspect(&d, None, c);
    for blocker in [
        "object_history_capacity",
        "no_active_upload_slot",
        "manifest_capacity",
        "byte_capacity",
        "service_fenced",
        "tenant_suspended",
    ] {
        assert!(
            report["blockers"]
                .as_array()
                .unwrap()
                .contains(&json!(blocker))
        );
    }
    assert_eq!(
        observation::scope(
            &b.files,
            u.service,
            u.namespace,
            Principal::self_authenticating([9])
        ),
        Err(Failure::Denied)
    );
}
#[test]
fn wrong_reply_scope_remote_errors_and_oversize_refuse() {
    let d = frozen();
    let b = batch(&d);
    let u = b.files[0].input.permission.upload;
    let scope = TenantScope {
        service: u.service,
        namespace: u.namespace,
        tenant: u.tenant,
    };
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    for (n, bytes, expected) in [
        (
            0,
            candid::encode_one(Err::<UploadCapacityResponse, _>(
                UploadCapacityFailure::NotEnrolled,
            ))
            .unwrap(),
            Failure::NotEnrolled,
        ),
        (
            1,
            {
                let mut c = capacity(scope);
                c.scope.namespace -= 1;
                candid::encode_one(Ok::<_, UploadCapacityFailure>(c)).unwrap()
            },
            Failure::Binding,
        ),
        (2, vec![0; 4097], Failure::ReplyTooLarge),
    ] {
        let run = Run::create(&d.path().join(format!("bad-{n}"))).unwrap();
        assert_eq!(
            runtime.block_on(observation::inspect(scope, &b.files, &run, |_, _| {
                std::future::ready(Ok(bytes.clone()))
            })),
            Err(expected)
        );
        assert!(
            d.path()
                .join(format!("bad-{n}/query-0000-reply.candid"))
                .exists()
        );
    }
}

#[test]
fn duplicate_roots_are_blocked_and_query_failures_retain_partial_evidence() {
    let d = frozen();
    let mut b = batch(&d);
    let mut second = batch(&d).files.pop().unwrap();
    second.input.permission.upload.upload -= 1;
    second.input.permission.upload.object += 1;
    second.input.permission.upload.first_reference += 1;
    b.files.push(second);
    let u = b.files[0].input.permission.upload;
    let scope = TenantScope {
        service: u.service,
        namespace: u.namespace,
        tenant: u.tenant,
    };
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let run = Run::create(&d.path().join("duplicates")).unwrap();
    let report = runtime
        .block_on(observation::inspect(
            scope,
            &b.files,
            &run,
            |method, argument| {
                let reply = if method
                    == ic_blob_storage::ops::service::uploads::capacity::UPLOAD_CAPACITY_METHOD
                {
                    candid::encode_one(Ok::<_, UploadCapacityFailure>(capacity(scope))).unwrap()
                } else {
                    candid::encode_one(Ok::<_, UploadDiscoveryFailure>(UploadDiscoveryResponse {
                        request: candid::decode_one(&argument).unwrap(),
                        content: None,
                        fenced: false,
                    }))
                    .unwrap()
                };
                std::future::ready(Ok(reply))
            },
        ))
        .unwrap()
        .report;
    assert!(
        report["blockers"]
            .as_array()
            .unwrap()
            .contains(&json!("duplicate_planned_root"))
    );
    let run = Run::create(&d.path().join("partial")).unwrap();
    let mut calls = 0;
    let failed = runtime.block_on(observation::inspect(scope, &b.files, &run, |_, _| {
        calls += 1;
        std::future::ready(if calls == 1 {
            Ok(candid::encode_one(Ok::<_, UploadCapacityFailure>(capacity(scope))).unwrap())
        } else {
            Err(Failure::Transport)
        })
    }));
    assert_eq!(failed, Err(Failure::Transport));
    assert!(d.path().join("partial/query-0000-reply.candid").exists());
    assert!(d.path().join("partial/query-0001-args.candid").exists());
    assert!(!d.path().join("partial/query-0001-reply.candid").exists());
}

#[test]
fn query_budget_refuses_before_loading_identity_or_claiming_output() {
    let d = frozen();
    let b = batch(&d);
    let p = b.files[0].input.permission;
    let options = crate::native::arguments::Options {
        command: crate::native::arguments::Command::Status {
            scope: ic_blob_storage::dto::operator::OperatorScope {
                service: p.upload.service,
                namespace: p.upload.namespace,
                cashier: Principal::self_authenticating([8]),
                payment_account: p.upload.tenant,
            },
        },
        network: "ic",
        url: url::Url::parse("https://icp-api.io").unwrap(),
        identity: d.path().join("missing.pem"),
        actor: p.upload.tenant,
        root_key: None,
    };
    let input = Input {
        service: p.upload.service,
        namespace: p.upload.namespace,
        inputs: d.path().join("batch"),
        directory: d.path().join("refused"),
        max_bytes: NonZeroU64::new(10).unwrap(),
        max_total_bytes: NonZeroU64::new(20).unwrap(),
        max_queries: 1,
        timeout_seconds: 30,
    };
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    assert_eq!(
        runtime.block_on(run(&options, &input)),
        Err(Failure::ReplyLimit)
    );
    assert!(!input.directory.exists());
}
