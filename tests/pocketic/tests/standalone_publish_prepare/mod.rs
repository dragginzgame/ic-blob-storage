//! Signed indexed setup, distinct actors and lost-reply recovery against real IC state.
use super::*;
use crate::{
    account_native_cli::OUTSIDER_PEM,
    authenticated_cli::{PEM, run},
    standalone_publish_check::{freeze, setup},
    submission_proxy::{Dispatch, Proxy, Reply},
};
use serde_json::{Value, json};
use std::path::Path;

enum Directory {
    Temporary(tempfile::TempDir),
    Retained(std::path::PathBuf),
}
impl Directory {
    fn new(label: &str) -> Self {
        if let Some(root) = std::env::var_os("BLOB_PUBLICATION_REPORT") {
            let root = std::path::PathBuf::from(root);
            std::fs::create_dir_all(&root).unwrap();
            Self::Retained(
                tempfile::Builder::new()
                    .prefix(label)
                    .tempdir_in(root)
                    .unwrap()
                    .keep(),
            )
        } else {
            Self::Temporary(tempfile::tempdir().unwrap())
        }
    }
    fn path(&self) -> &Path {
        match self {
            Self::Temporary(directory) => directory.path(),
            Self::Retained(path) => path,
        }
    }
}

fn args(
    fixture: &Fixture,
    directory: &Path,
    url: &str,
    recovery: bool,
    label: &str,
) -> Vec<String> {
    std::fs::write(directory.join("tenant.pem"), PEM).unwrap();
    std::fs::write(directory.join("uploader.pem"), OUTSIDER_PEM).unwrap();
    std::fs::write(
        directory.join("root.der"),
        fixture.harness.pic.root_key().unwrap(),
    )
    .unwrap();
    let mut args = [
        if recovery {
            "publish-prepare-resume"
        } else {
            "publish-prepare"
        },
        "--network",
        "local",
        "--url",
        url,
        "--identity",
        directory.join("tenant.pem").to_str().unwrap(),
        "--actor",
        &fixture.tenant.to_text(),
        "--uploader-identity",
        directory.join("uploader.pem").to_str().unwrap(),
        "--service",
        &fixture.service.to_text(),
        "--namespace",
        &fixture.config.namespace.to_string(),
        "--root-key",
        directory.join("root.der").to_str().unwrap(),
        "--inputs",
        directory.join("batch").to_str().unwrap(),
        "--file-index",
        "0",
        "--max-bytes",
        "1024",
        "--max-total-bytes",
        "1024",
        "--timeout-seconds",
        "30",
        "--run-dir",
        directory.join(label).to_str().unwrap(),
    ]
    .map(str::to_owned)
    .to_vec();
    if recovery {
        args.extend([
            "--source-run".into(),
            directory.join("original").display().to_string(),
        ]);
    }
    args
}
fn source_report(directory: &Path, stage: &str, name: &str) -> Value {
    serde_json::from_slice(
        &std::fs::read(directory.join("original").join(stage).join(name)).unwrap(),
    )
    .unwrap()
}
fn plan(
    fixture: &Fixture,
    directory: &Path,
    replies: [Reply; 2],
) -> Vec<(std::path::PathBuf, Dispatch, Reply)> {
    [
        ("admission", fixture.tenant, "blob_admit_upload"),
        ("preparation", fixture.uploader, "blob_prepare_upload"),
    ]
    .into_iter()
    .zip(replies)
    .map(|((stage, actor, method), reply)| {
        (
            directory.join("original").join(stage),
            Dispatch {
                service: fixture.service,
                actor,
                method,
                argument_file: "request.candid",
            },
            reply,
        )
    })
    .collect()
}
fn live(fixture: &mut Fixture) -> String {
    fixture
        .harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .to_string()
}
#[test]
fn standalone_indexed_prepare_persists_two_distinct_signed_claims_and_recovery_never_redispatches()
{
    let mut fixture = setup();
    let directory = Directory::new("acknowledged-");
    let permission = freeze(&fixture, directory.path());
    let backend = live(&mut fixture);
    let proxy = Proxy::sequence(backend, plan(&fixture, directory.path(), [Reply::Pass; 2]));
    let report = run(
        &args(&fixture, directory.path(), &proxy.url, false, "original"),
        0,
    );
    assert_eq!(report["state"], "prepared");
    assert_eq!(report["service_updates_this_run"], 2);
    assert_eq!(report["provider_requests"], 0);
    assert_eq!(
        source_report(directory.path(), "admission", "intent.json")["actor"],
        fixture.tenant.to_text()
    );
    assert_eq!(
        source_report(directory.path(), "preparation", "intent.json")["actor"],
        fixture.uploader.to_text()
    );
    let saved = std::fs::read(
        directory
            .path()
            .join("original/preparation/signed-request.cbor"),
    )
    .unwrap();
    let resumed = run(
        &args(&fixture, directory.path(), &proxy.url, true, "recovered"),
        0,
    );
    assert_eq!(resumed["state"], "prepared");
    assert_eq!(resumed["service_updates_this_run"], 0);
    assert_eq!(proxy.calls(), 2);
    assert_eq!(
        fixture.admission(permission).state,
        ic_blob_storage::dto::upload::UploadState::Reserved
    );
    assert_eq!(
        std::fs::read(
            directory
                .path()
                .join("original/preparation/signed-request.cbor")
        )
        .unwrap(),
        saved
    );
    assert_eq!(
        run(
            &args(&fixture, directory.path(), &proxy.url, false, "original"),
            3
        )["error"],
        "new_run_required"
    );
    fixture.harness.pic.stop_live();
}
#[test]
fn standalone_indexed_prepare_recovers_lost_admission_and_preparation_responses_without_resubmission()
 {
    for lost_stage in 0..2 {
        let mut fixture = setup();
        let directory = Directory::new(if lost_stage == 0 {
            "lost-admission-"
        } else {
            "lost-preparation-"
        });
        freeze(&fixture, directory.path());
        let backend = live(&mut fixture);
        let mut replies = [Reply::Pass; 2];
        replies[lost_stage] = Reply::Drop;
        let proxy = Proxy::sequence(backend, plan(&fixture, directory.path(), replies));
        assert_eq!(
            run(
                &args(&fixture, directory.path(), &proxy.url, false, "original"),
                3
            )["error"],
            "transport"
        );
        let stage = if lost_stage == 0 {
            "admission"
        } else {
            "preparation"
        };
        assert_eq!(
            source_report(directory.path(), stage, "outcome.json")["outcome"],
            "uncertain"
        );
        let resumed = run(
            &args(&fixture, directory.path(), &proxy.url, true, "recovered"),
            0,
        );
        assert_eq!(resumed["state"], "prepared");
        assert_eq!(
            resumed["service_updates_this_run"],
            u64::from(lost_stage == 0)
        );
        assert_eq!(proxy.calls(), 2);
        assert_eq!(
            run(
                &args(
                    &fixture,
                    directory.path(),
                    &proxy.url,
                    true,
                    "recovered-again"
                ),
                0
            )["service_updates_this_run"],
            0
        );
        assert_eq!(proxy.calls(), 2);
        fixture.harness.pic.stop_live();
    }
}
#[test]
fn standalone_indexed_prepare_refuses_wrong_role_changed_journal_and_withdrawn_permission() {
    let mut fixture = setup();
    let directory = Directory::new("refusals-");
    let permission = freeze(&fixture, directory.path());
    let url = live(&mut fixture);
    let mut wrong = args(&fixture, directory.path(), &url, false, "wrong");
    let flag = wrong
        .iter()
        .position(|s| s == "--uploader-identity")
        .unwrap();
    wrong[flag + 1] = directory.path().join("tenant.pem").display().to_string();
    assert_eq!(run(&wrong, 3)["error"], "identity_binding");
    assert!(!directory.path().join("wrong").exists());
    assert_eq!(
        run(
            &args(&fixture, directory.path(), &url, false, "original"),
            0
        )["state"],
        "prepared"
    );
    let intent_path = directory.path().join("original/admission/intent.json");
    let original = std::fs::read(&intent_path).unwrap();
    let mut changed: Value = serde_json::from_slice(&original).unwrap();
    changed["actor"] = json!(fixture.uploader.to_text());
    std::fs::write(&intent_path, changed.to_string()).unwrap();
    assert_eq!(
        run(&args(&fixture, directory.path(), &url, true, "tampered"), 3)["error"],
        "binding"
    );
    assert!(!directory.path().join("tampered").exists());
    std::fs::write(intent_path, original).unwrap();
    fixture
        .harness
        .pic
        .update_candid_as::<Result<UploadRevocationResponse, UploadAdmissionFailure>, _>(
            fixture.service,
            fixture.tenant,
            "blob_revoke_upload",
            (permission,),
        )
        .unwrap()
        .unwrap();
    let blocked = run(
        &args(&fixture, directory.path(), &url, true, "withdrawn"),
        0,
    );
    assert_eq!(blocked["state"], "blocked");
    assert_eq!(blocked["service_updates_this_run"], 0);
    fixture.harness.pic.stop_live();
}

#[test]
fn standalone_indexed_prepare_partial_preparation_claim_never_allows_an_update() {
    let mut fixture = setup();
    let directory = Directory::new("partial-claim-");
    freeze(&fixture, directory.path());
    let backend = live(&mut fixture);
    let proxy = Proxy::sequence(
        backend,
        plan(&fixture, directory.path(), [Reply::Drop, Reply::Pass]),
    );
    assert_eq!(
        run(
            &args(&fixture, directory.path(), &proxy.url, false, "original"),
            3
        )["error"],
        "transport"
    );
    std::fs::create_dir(directory.path().join("original/preparation")).unwrap();
    assert_eq!(
        run(
            &args(&fixture, directory.path(), &proxy.url, true, "partial"),
            3
        )["error"],
        "file"
    );
    assert_eq!(proxy.calls(), 1);
    assert!(directory.path().join("original/preparation").is_dir());
    let failure: Value = serde_json::from_slice(
        &std::fs::read(directory.path().join("partial/failure.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(failure["redispatch_authorized"], false);
    fixture.harness.pic.stop_live();
}
