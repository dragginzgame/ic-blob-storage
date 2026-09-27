//! Historical receipt recovery uses actual callers and the saved-intent executable.
use super::*;
use blob_test_protocol::admission::{
    input::{ReferenceInput, ReferenceReceipt},
    release::{LifecycleCommand, ReferenceFailure},
};
use serde_json::{Value, json};
use std::{fs, path::Path, process::Command as Process};

fn confirmed(f: &Fixture) -> Permission {
    f.enroll();
    let vector = vectors::vector("abc-text", 1);
    let p = f.permission(&vector);
    f.admit(p);
    f.prepare(p, &vector);
    f.call(f.uploader, Command::Expose(p.request.root)).unwrap();
    f.call(
        f.operator,
        Command::FixtureLifecycle(LifecycleCommand::SubstituteCompletion(p.request)),
    )
    .unwrap();
    p
}

fn intent(path: &Path, input: ReferenceInput) {
    let root =
        ic_blob_storage::model::identity::ProviderRootHash::try_from(input.object.root.as_slice())
            .unwrap();
    fs::write(path,serde_json::to_vec(&json!({
        "schema":1,"scope":"pocketic_fixture","asset":"release-image", "service":input.object.service.to_text(),
        "tenant":input.object.tenant.to_text(),"namespace":input.object.namespace.to_string(),
        "upload":input.object.id.to_string(),"object":input.object.id.to_string(),"incarnation":"1",
        "root":root.to_string(),
        "bytes":input.object.bytes,"reference":input.reference.to_string(),"operation":input.operation.to_string(),"retain":input.retain,
    })).unwrap()).unwrap();
}

fn command(args: &[String], code: i32) -> Value {
    let output = Process::new(env!("CARGO_BIN_EXE_blob-fixture-reference"))
        .args(args)
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(code),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

fn inspect(f: &Fixture, path: &Path, code: i32) -> Value {
    let url = f.harness.pic.get_server_url();
    command(
        &[
            "inspect".into(),
            "--intent".into(),
            path.to_str().unwrap().into(),
            "--server".into(),
            format!("{}:{}", url.host_str().unwrap(), url.port().unwrap()),
            "--instance".into(),
            f.harness.pic.instance_id().to_string(),
            "--canister".into(),
            f.service.to_text(),
            "--caller".into(),
            f.project.to_text(),
        ],
        code,
    )
}

fn query(
    f: &Fixture,
    actor: Principal,
    input: ReferenceInput,
) -> Result<Option<ReferenceReceipt>, Failure> {
    f.harness
        .pic
        .query_candid_as(f.service, actor, "reference_receipt", (input,))
        .unwrap()
}

fn apply(f: &Fixture, input: ReferenceInput) {
    f.call(
        f.project,
        Command::FixtureLifecycle(LifecycleCommand::Reference {
            object: input.object,
            reference: input.reference,
            operation: input.operation,
            retain: input.retain,
        }),
    )
    .unwrap();
}

fn assert_tenant_only(f: &Fixture, input: ReferenceInput) {
    for actor in [
        f.uploader,
        f.operator,
        f.controller,
        f.other,
        Principal::anonymous(),
    ] {
        assert_eq!(query(f, actor, input), Err(Failure::NotProject));
    }
}

#[test]
fn saved_intent_recovers_historical_success_without_resurrecting_a_released_reference() {
    let f = Fixture::new();
    let p = confirmed(&f);
    let input = ReferenceInput {
        object: p.request,
        reference: 2,
        operation: 1,
        retain: true,
    };
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("source.json");
    let journal = dir.path().join("journal");
    fs::create_dir(&journal).unwrap();
    intent(&source, input);
    let stored = command(
        &[
            "save".into(),
            "--intent".into(),
            source.to_str().unwrap().into(),
            "--journal".into(),
            journal.to_str().unwrap().into(),
        ],
        0,
    );
    let saved = Path::new(stored["saved"].as_str().unwrap());
    fs::remove_file(source).unwrap();
    let before = f.observe(p);
    assert_eq!(inspect(&f, saved, 4)["observation"]["status"], "absent");
    assert_eq!(f.observe(p), before);
    // The client has only its saved intent, not the outcome returned to this harness.
    apply(&f, input);
    let before = f.observe(p);
    let recovered = inspect(&f, saved, 0);
    assert_eq!(recovered["observation"]["status"], "recorded_success");
    assert_eq!(recovered["current_liveness"], "not_assessed");
    assert_eq!(f.observe(p), before);
    f.call(
        f.operator,
        f.enrollment(
            Some(Enrollment {
                generation: 1,
                active: true,
            }),
            false,
        ),
    )
    .unwrap();
    apply(
        &f,
        ReferenceInput {
            operation: 2,
            retain: false,
            ..input
        },
    );
    apply(
        &f,
        ReferenceInput {
            operation: 3,
            reference: 1,
            retain: false,
            ..input
        },
    );
    f.call(
        f.operator,
        Command::FixtureLifecycle(LifecycleCommand::SubstituteDeletion(p.request)),
    )
    .unwrap();
    f.call(
        f.operator,
        Command::FixtureLifecycle(LifecycleCommand::SubstituteSettlement(p.request)),
    )
    .unwrap();
    f.restart();
    let settled = f.observe(p);
    assert_eq!(inspect(&f, saved, 0), recovered);
    assert_eq!(
        query(
            &f,
            f.project,
            ReferenceInput {
                operation: 4,
                ..input
            }
        ),
        Ok(None)
    );
    assert_eq!(f.observe(p), settled);
}

#[test]
fn receipt_query_checks_every_binding_and_preserves_recorded_failure() {
    let f = Fixture::new();
    let p = confirmed(&f);
    let input = ReferenceInput {
        object: p.request,
        reference: 9,
        operation: 1,
        retain: false,
    };
    apply(&f, input);
    let before = f.observe(p);
    assert_eq!(
        query(&f, f.project, input),
        Ok(Some(ReferenceReceipt {
            request: input,
            result: Err(ReferenceFailure::Unknown)
        }))
    );
    assert_tenant_only(&f, input);
    assert_eq!(
        query(
            &f,
            f.project,
            ReferenceInput {
                retain: true,
                ..input
            }
        ),
        Err(Failure::Conflict)
    );
    assert_eq!(
        query(
            &f,
            f.project,
            ReferenceInput {
                reference: 8,
                ..input
            }
        ),
        Err(Failure::Conflict)
    );
    assert_eq!(
        query(
            &f,
            f.project,
            ReferenceInput {
                operation: 0,
                ..input
            }
        ),
        Err(Failure::InvalidInput)
    );
    for (object, error) in [
        (
            Request {
                service: f.other,
                ..p.request
            },
            Failure::WrongService,
        ),
        (
            Request {
                namespace: 2,
                ..p.request
            },
            Failure::WrongNamespace,
        ),
        (
            Request {
                bytes: 4,
                ..p.request
            },
            Failure::Conflict,
        ),
        (
            Request {
                root: [9; 32],
                ..p.request
            },
            Failure::Catalog,
        ),
    ] {
        assert_eq!(
            query(&f, f.project, ReferenceInput { object, ..input }),
            Err(error)
        );
    }
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("failed.json");
    intent(&path, input);
    let observed = inspect(&f, &path, 4);
    assert_eq!(observed["observation"]["status"], "recorded_failure");
    assert_eq!(observed["observation"]["failure"], "Unknown");
    assert_eq!(f.observe(p), before);
}

#[test]
fn oversized_receipt_input_rejects_before_inspection() {
    let f = Fixture::new();
    let oversized = f
        .harness
        .pic
        .query_call(f.service, f.project, "reference_receipt", vec![0; 4097])
        .unwrap_err();
    assert_eq!(oversized.reject_code, RejectCode::CanisterError);
}
