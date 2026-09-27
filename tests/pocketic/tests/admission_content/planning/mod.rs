//! Actual caller/scope enforcement and unchanged accounting through passive reads.
use super::*;
mod inventory;
use blob_test_protocol::admission::{
    planning::{AdmissionCapacity, AdmissionCapacityInput},
    release::LifecycleCommand,
};

fn scope(f: &Fixture) -> AdmissionCapacityInput {
    AdmissionCapacityInput {
        service: f.service,
        tenant: f.project,
        namespace: 1,
    }
}

fn capacity(
    f: &Fixture,
    actor: Principal,
    input: AdmissionCapacityInput,
) -> Result<AdmissionCapacity, Failure> {
    f.harness
        .pic
        .query_candid_as(f.service, actor, "admission_capacity", (input,))
        .unwrap()
}

#[test]
fn capacity_queries_enforce_caller_scope_and_input_bound() {
    let f = Fixture::new();
    let input = scope(&f);
    assert_eq!(capacity(&f, f.project, input), Err(Failure::NotEnrolled));
    let enrollment = f.enroll();
    let initial = capacity(&f, f.project, input).unwrap();
    assert_eq!(initial.scope, input);
    assert_eq!(initial.enrollment, enrollment);
    assert_eq!(initial.max_object_bytes, 10 * 1024 * 1024);
    assert_eq!(initial.remaining_objects, 2);
    assert_eq!(initial.remaining_active_uploads, 2);
    assert_eq!(initial.remaining_manifest_chunks, 10);
    assert_eq!(initial.remaining_bytes, 10 * 1024 * 1024);
    for actor in [
        f.operator,
        f.controller,
        f.uploader,
        f.other,
        Principal::anonymous(),
    ] {
        assert_eq!(capacity(&f, actor, input), Err(Failure::NotProject));
    }
    assert_eq!(
        capacity(
            &f,
            f.other,
            AdmissionCapacityInput {
                tenant: f.other,
                ..input
            }
        ),
        Err(Failure::NotEnrolled)
    );
    assert_eq!(
        capacity(
            &f,
            f.project,
            AdmissionCapacityInput {
                service: f.other,
                ..input
            }
        ),
        Err(Failure::WrongService)
    );
    assert_eq!(
        capacity(
            &f,
            f.project,
            AdmissionCapacityInput {
                namespace: 2,
                ..input
            }
        ),
        Err(Failure::WrongNamespace)
    );
    assert_eq!(
        capacity(
            &f,
            f.project,
            AdmissionCapacityInput {
                namespace: 0,
                ..input
            }
        ),
        Err(Failure::InvalidInput)
    );
    let oversized = f
        .harness
        .pic
        .query_call(f.service, f.project, "admission_capacity", vec![0; 4097])
        .unwrap_err();
    assert_eq!(oversized.reject_code, RejectCode::CanisterError);
    assert_eq!(capacity(&f, f.project, input), Ok(initial));
}

#[test]
fn capacity_preserves_cancelled_history_and_suspended_inspection_across_stop_start() {
    let f = Fixture::new();
    let input = scope(&f);
    let enrollment = f.enroll();
    let initial = capacity(&f, f.project, input).unwrap();
    let permission = f.permission(&vectors::vector("abc-text", 1));
    f.admit(permission);
    let before = f.observe(permission);
    let reserved = capacity(&f, f.project, input).unwrap();
    assert_eq!(reserved.remaining_bytes, initial.remaining_bytes - 3);
    assert_eq!(reserved.remaining_objects, 1);
    assert_eq!(reserved.remaining_active_uploads, 1);
    assert_eq!(reserved.remaining_manifest_chunks, 9);
    assert_eq!(f.observe(permission), before);
    f.call(f.project, Command::Revoke(permission.request))
        .unwrap();
    let cancelled = capacity(&f, f.project, input).unwrap();
    assert_eq!(cancelled.remaining_bytes, initial.remaining_bytes);
    assert_eq!(
        cancelled.remaining_active_uploads,
        initial.remaining_active_uploads
    );
    assert_eq!(cancelled.remaining_objects, reserved.remaining_objects);
    assert_eq!(
        cancelled.remaining_manifest_chunks,
        reserved.remaining_manifest_chunks
    );
    f.call(f.operator, f.enrollment(Some(enrollment), false))
        .unwrap();
    f.restart();
    assert_eq!(
        capacity(&f, f.project, input).unwrap(),
        AdmissionCapacity {
            enrollment: Enrollment {
                active: false,
                ..enrollment
            },
            ..cancelled
        }
    );
    assert_eq!(
        f.call(f.project, Command::Admit(permission)),
        Ok(Outcome::Existing(Phase::Cancelled))
    );
    assert_eq!(
        capacity(&f, f.project, input)
            .unwrap()
            .remaining_manifest_chunks,
        9
    );
}

#[test]
fn completion_and_settlement_never_restore_lifetime_manifest_capacity() {
    let f = Fixture::new();
    f.enroll();
    let input = scope(&f);
    let vector = vectors::vector("media-10485760", 1);
    let permission = f.permission(&vector);
    f.admit(permission);
    f.prepare(permission, &vector);
    f.call(f.uploader, Command::Expose(permission.request.root))
        .unwrap();
    let pending = capacity(&f, f.project, input).unwrap();
    assert_eq!(pending.remaining_bytes, 0);
    assert_eq!(pending.remaining_manifest_chunks, 0);
    f.call(
        f.operator,
        Command::FixtureLifecycle(LifecycleCommand::SubstituteCompletion(permission.request)),
    )
    .unwrap();
    assert_eq!(
        capacity(&f, f.project, input)
            .unwrap()
            .remaining_active_uploads,
        2
    );
    f.call(
        f.project,
        Command::FixtureLifecycle(LifecycleCommand::Reference {
            object: permission.request,
            reference: 1,
            operation: 1,
            retain: false,
        }),
    )
    .unwrap();
    for command in [
        LifecycleCommand::SubstituteDeletion(permission.request),
        LifecycleCommand::SubstituteSettlement(permission.request),
    ] {
        f.call(f.operator, Command::FixtureLifecycle(command))
            .unwrap();
    }
    f.restart();
    let settled = capacity(&f, f.project, input).unwrap();
    assert_eq!(settled.remaining_bytes, 10 * 1024 * 1024);
    assert_eq!(settled.remaining_manifest_chunks, 0);
    assert_eq!(settled.remaining_objects, 1);
    let next = f.permission(&vectors::vector("abc-text", 2));
    assert_eq!(
        f.call(f.project, Command::Admit(next)),
        Err(Failure::TenantManifestCapacity)
    );
    assert_eq!(capacity(&f, f.project, input).unwrap(), settled);
}
