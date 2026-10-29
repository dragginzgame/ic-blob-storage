//! Real standalone heap reporting; query-only Cashier substitutes are not provider evidence.
use super::*;
use ic_blob_storage_contracts::standalone::{
    METRICS_METHOD, MetricOperation as Op, MetricsReport, OperationMetrics,
};

pub(super) fn report(f: &Fixture, actor: Principal) -> Result<MetricsReport, HostFailure> {
    f.harness
        .pic
        .query_blob_as(f.service, actor, METRICS_METHOD.name, (), 4096)
        .unwrap()
}
fn row(report: &MetricsReport, operation: Op) -> &OperationMetrics {
    report
        .operations
        .iter()
        .find(|row| row.operation == operation)
        .unwrap()
}
fn admit(f: &Fixture, actor: Principal) -> Result<UploadAdmissionMutation, UploadAdmissionFailure> {
    f.harness
        .pic
        .update_blob_as(
            f.service,
            actor,
            "blob_admit_upload",
            f.small_manifest().permission,
            REPLY_BYTES,
        )
        .unwrap()
}
fn register_and_reference(f: &Fixture) {
    use ic_blob_storage_contracts::dto::upload::certificate::CaffeineUploadCertificateResponse;
    use ic_blob_storage_pocketic_tests::provider_calls;
    let manifest = f.small_manifest();
    let root = super::standalone_certificate::root(&manifest);
    super::standalone_certificate::refuses(f, f.controller, &root);
    provider_calls::update_provider::<(CaffeineUploadCertificateResponse,), _>(
        &f.harness.pic,
        f.service,
        f.uploader,
        "_immutableObjectStorageCreateCertificate",
        (root,),
    )
    .unwrap();
    for (operation, reference) in [(1, 999), (2, manifest.permission.upload.first_reference)] {
        let command = ReferenceCommand {
            upload: manifest.permission.upload,
            reference,
            operation,
            action: ReferenceAction::Release,
        };
        for replayed in [false, true] {
            let reply: Result<ReferenceMutationResponse, ReferenceFailure> = f
                .harness
                .pic
                .update_blob_as(
                    f.service,
                    f.tenant,
                    "blob_apply_reference",
                    command,
                    REPLY_BYTES,
                )
                .unwrap();
            let reply = reply.unwrap();
            assert_eq!(reply.replayed, replayed);
            assert_eq!(reply.receipt.result.is_ok(), operation == 2);
        }
    }
}

#[test]
fn standalone_metrics_authenticate_replays_and_heap_reset_without_stable_writes() {
    let f = Fixture::small(Harness::new(), fake_principal(4));
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(report(&f, actor), Err(HostFailure::Denied));
    }
    let initial = report(&f, f.operator).unwrap();
    assert_eq!(row(&initial, Op::Admission).instructions.maximum(), None);
    f.enroll(f.operator).unwrap();
    assert_eq!(admit(&f, f.uploader), Err(UploadAdmissionFailure::Denied));
    assert!(!admit(&f, f.tenant).unwrap().replayed);
    assert!(admit(&f, f.tenant).unwrap().replayed);
    let manifest = f.small_manifest();
    assert!(f.prepare(f.uploader, &manifest).unwrap().changed);
    assert!(!f.prepare(f.uploader, &manifest).unwrap().changed);
    register_and_reference(&f);
    let measured = report(&f, f.operator).unwrap();
    assert_eq!(measured.window_version, initial.window_version);
    assert!(measured.observed_version > initial.observed_version);
    let admission = row(&measured, Op::Admission);
    assert_eq!(
        (
            admission.attempts.total(),
            admission.applied.total(),
            admission.replayed.total(),
            admission.refused.total()
        ),
        (3, 1, 1, 1)
    );
    assert_eq!(admission.instructions.samples(), 3);
    assert!(admission.instructions.total() > 0);
    assert!(admission.request_bytes.total() > 0);
    assert_eq!(row(&measured, Op::Manifest).applied.total(), 1);
    assert_eq!(row(&measured, Op::Manifest).replayed.total(), 1);
    assert_eq!(row(&measured, Op::Registration).applied.total(), 1);
    let references = row(&measured, Op::Reference);
    assert_eq!(
        (
            references.applied.total(),
            references.replayed.total(),
            references.refused.total()
        ),
        (1, 2, 1)
    );
    let stable = f.harness.pic.get_stable_memory(f.service);
    assert!(
        f.harness
            .pic
            .update_call(f.service, f.tenant, "blob_admit_upload", vec![0])
            .is_err()
    );
    assert!(
        f.harness
            .pic
            .query_call(f.service, f.operator, METRICS_METHOD.name, vec![0; 65])
            .is_err()
    );
    assert_eq!(
        report(&f, f.operator).unwrap().operations,
        measured.operations
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &stable);
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .unwrap();
    let reset = report(&f, f.operator).unwrap();
    assert_ne!(reset.window_version, measured.window_version);
    assert!(reset.window_version <= reset.observed_version);
    assert!(
        reset
            .operations
            .iter()
            .all(|row| row.attempts.total() == 0 && row.instructions.maximum().is_none())
    );
    assert!(f.configuration(f.operator).unwrap().fenced);
}

#[test]
fn standalone_metrics_native_command_uses_signed_query_and_exact_scope() {
    use crate::authenticated_cli::{PEM, arguments, run};
    use ic_host_fs::testing::FixtureDirectory;
    let mut f = Fixture::with_cashier_and_operator(
        Harness::with_builder(
            ic_platform::pocket_ic::PocketIcBuilder::new()
                .with_nns_subnet()
                .with_application_subnet(),
        ),
        fake_principal(5),
        crate::account_native_cli::signer(),
    );
    let files = FixtureDirectory::new(&std::env::temp_dir(), "blob-metrics").unwrap();
    let identity = files.path().join("identity.pem");
    let root = files.path().join("root.der");
    std::fs::write(&identity, PEM).unwrap();
    std::fs::write(&root, f.harness.pic.root_key().unwrap()).unwrap();
    let url = f
        .harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .to_string();
    let mut args = arguments(
        "metrics",
        f.operator_scope(),
        f.operator,
        &url,
        &identity,
        &root,
    );
    for flag in ["--cashier", "--payer"] {
        let index = args.iter().position(|value| value == flag).unwrap();
        args.drain(index..index + 2);
    }
    let observed = run(&args, 0);
    assert_eq!(observed["observation"], "standalone_heap_metrics");
    assert_eq!(observed["operations"][0]["attempts"]["total"], "0");
    let namespace = args
        .iter()
        .position(|value| value == "--namespace")
        .unwrap();
    args[namespace + 1] = "1".into();
    assert_eq!(run(&args, 3)["error"], "binding");
    f.harness.pic.stop_live();
}
