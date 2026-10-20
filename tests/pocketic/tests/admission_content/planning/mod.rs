//! Actual caller/scope enforcement and unchanged accounting through passive reads.
use super::*;
use crate::support::REPLY_LIMITS;
use ic_blob_storage_contracts::dto::tenant::TenantScope;
use ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityFailure;
use ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityResponse;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
mod inventory;
use blob_test_protocol::admission::release::LifecycleCommand;

fn scope(f: &Fixture) -> TenantScope {
    TenantScope {
        service: f.service.into(),
        tenant: f.project.into(),
        namespace: 1,
    }
}

fn capacity(
    f: &Fixture,
    actor: Principal,
    input: TenantScope,
) -> Result<UploadCapacityResponse, UploadCapacityFailure> {
    f.harness
        .pic
        .query_blob_as(
            f.service,
            actor,
            "blob_upload_capacity",
            input,
            REPLY_LIMITS.reply_bytes,
        )
        .unwrap()
}

#[test]
fn discovery_and_capacity_queries_refuse_foreign_frames_without_changing_observations() {
    use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityRequest;
    use ic_blob_storage_contracts::{document, wire};

    let f = Fixture::new();
    f.enroll();
    let scope = scope(&f);
    let root = [1; 32];
    let requests = [
        (
            "blob_lookup_content",
            wire::encode_request(
                "blob_lookup_content",
                UploadDiscoveryRequest { scope, root },
            )
            .unwrap(),
        ),
        (
            "blob_upload_capacity",
            wire::encode_request("blob_upload_capacity", scope).unwrap(),
        ),
        (
            "blob_reference_capacity",
            wire::encode_request(
                "blob_reference_capacity",
                ReferenceCapacityRequest { scope, root },
            )
            .unwrap(),
        ),
    ];
    for (method, request) in requests {
        let before = f
            .harness
            .pic
            .query_call(f.service, f.project, method, request.clone())
            .expect("current method-bound request succeeds");
        let mut trailing = request.clone();
        trailing.push(0);
        for bytes in [
            document::encode(scope).unwrap(),
            wire::encode_request("blob_configuration", ()).unwrap(),
            wire::encode_response(method, ()).unwrap(),
            ic_blob_storage_pocketic_tests::historical::encode(scope).unwrap(),
            vec![0xf6],
            request[..request.len() - 1].to_vec(),
            trailing,
            vec![0; 4097],
        ] {
            let error = f
                .harness
                .pic
                .query_call(f.service, f.project, method, bytes)
                .unwrap_err();
            assert_eq!(error.reject_code, RejectCode::CanisterError);
        }
        assert_eq!(
            f.harness
                .pic
                .query_call(f.service, f.project, method, request)
                .unwrap(),
            before
        );
    }
}

#[test]
fn capacity_queries_enforce_caller_scope_and_input_bound() {
    let f = Fixture::new();
    let input = scope(&f);
    assert_eq!(
        capacity(&f, f.project, input),
        Err(UploadCapacityFailure::NotEnrolled)
    );
    let enrollment = f.enroll();
    let initial = capacity(&f, f.project, input).unwrap();
    assert_eq!(initial.scope, input);
    assert_eq!(initial.enrollment.generation, enrollment.generation);
    assert_eq!(initial.enrollment.active, enrollment.active);
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
        assert_eq!(
            capacity(&f, actor, input),
            Err(UploadCapacityFailure::Denied)
        );
    }
    assert_eq!(
        capacity(
            &f,
            f.other,
            TenantScope {
                tenant: f.other.into(),
                ..input
            }
        ),
        Err(UploadCapacityFailure::NotEnrolled)
    );
    assert_eq!(
        capacity(
            &f,
            f.project,
            TenantScope {
                service: f.other.into(),
                ..input
            }
        ),
        Err(UploadCapacityFailure::Binding)
    );
    assert_eq!(
        capacity(
            &f,
            f.project,
            TenantScope {
                namespace: 2,
                ..input
            }
        ),
        Err(UploadCapacityFailure::Binding)
    );
    assert_eq!(
        capacity(
            &f,
            f.project,
            TenantScope {
                namespace: 0,
                ..input
            }
        ),
        Err(UploadCapacityFailure::Invalid)
    );
    let oversized = f
        .harness
        .pic
        .query_call(f.service, f.project, "blob_upload_capacity", vec![0; 4097])
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
        UploadCapacityResponse {
            enrollment: ic_blob_storage_contracts::dto::tenant::TenantEnrollment {
                active: false,
                generation: enrollment.generation
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
