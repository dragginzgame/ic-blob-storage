//! Passive typed observations replace the retired fixture inventory application.
use super::*;
use ic_blob_storage_contracts::dto::reference::capacity::{
    ReferenceCapacityFailure, ReferenceCapacityRequest, ReferenceCapacityResponse,
};
use ic_blob_storage_contracts::dto::upload::discovery::{
    UploadDiscoveryFailure, UploadDiscoveryResponse,
};
use ic_blob_storage_contracts::dto::upload::history::UploadContentState;

fn discover(f: &Fixture, tenant: Principal, root: [u8; 32]) -> UploadDiscoveryResponse {
    f.harness
        .pic
        .query_blob_as::<Result<UploadDiscoveryResponse, UploadDiscoveryFailure>, _>(
            f.service,
            tenant,
            "blob_lookup_content",
            UploadDiscoveryRequest {
                scope: TenantScope {
                    tenant: tenant.into(),
                    ..scope(f)
                },
                root,
            },
            REPLY_BYTES,
        )
        .unwrap()
        .unwrap()
}
fn references(f: &Fixture, root: [u8; 32]) -> ReferenceCapacityResponse {
    f.harness
        .pic
        .query_blob_as::<Result<ReferenceCapacityResponse, ReferenceCapacityFailure>, _>(
            f.service,
            f.project,
            "blob_reference_capacity",
            ReferenceCapacityRequest {
                scope: scope(f),
                root,
            },
            REPLY_BYTES,
        )
        .unwrap()
        .unwrap()
}

#[test]
fn observations_preserve_pending_live_and_retired_content_without_leaking_other_tenants() {
    let f = Fixture::new();
    assert_eq!(
        capacity(&f, f.project, scope(&f)),
        Err(UploadCapacityFailure::NotEnrolled)
    );
    f.enroll();
    let vector = vectors::vector("abc-text", 1);
    let root = vector.upload.root;
    assert!(discover(&f, f.project, root).content.is_none());
    assert!(references(&f, root).headroom.is_none());
    let p = f.permission(&vector);
    f.admit(p);
    let before = f.observe(p);
    let pending = discover(&f, f.project, root).content.unwrap();
    assert_eq!(pending.state, UploadContentState::Reserved);
    assert_eq!(f.observe(p), before);
    f.prepare(p, &vector);
    f.call(f.uploader, Command::Expose(root)).unwrap();
    let before = f.observe(p);
    let live = discover(&f, f.project, root);
    assert_eq!(live.content.unwrap().state, UploadContentState::Live);
    assert_eq!(references(&f, root).headroom.unwrap().fresh_retains, 1);
    assert_eq!(discover(&f, f.project, root), live);
    assert_eq!(f.observe(p), before);
    f.call(
        f.project,
        Command::FixtureLifecycle(LifecycleCommand::Reference {
            object: p.request,
            reference: 1,
            operation: 1,
            retain: false,
        }),
    )
    .unwrap();
    for command in [
        LifecycleCommand::SubstituteDeletion(p.request),
        LifecycleCommand::SubstituteSettlement(p.request),
    ] {
        f.call(f.operator, Command::FixtureLifecycle(command))
            .unwrap();
    }
    f.restart();
    assert_eq!(
        discover(&f, f.project, root).content.unwrap().state,
        UploadContentState::Settled
    );
    f.call(
        f.operator,
        Command::Enroll {
            tenant: f.other.into(),
            expected: None,
            active: true,
        },
    )
    .unwrap();
    assert!(discover(&f, f.other, root).content.is_none());
}
