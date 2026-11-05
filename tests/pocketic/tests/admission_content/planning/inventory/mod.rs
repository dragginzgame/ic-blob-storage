//! Passive typed observations replace the retired fixture inventory application.
use super::*;
use ic_blob_storage_contracts::dto::reference::capacity::{
    ReferenceCapacityFailure, ReferenceCapacityRequest, ReferenceCapacityResponse,
};
use ic_blob_storage_contracts::dto::upload::discovery::{
    UploadDiscoveryFailure, UploadDiscoveryResponse,
};
use ic_blob_storage_contracts::dto::upload::history::UploadContentState;

async fn discover(f: &Fixture, tenant: Principal, root: [u8; 32]) -> UploadDiscoveryResponse {
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
        .await
        .unwrap()
        .unwrap()
}
async fn references(f: &Fixture, root: [u8; 32]) -> ReferenceCapacityResponse {
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
        .await
        .unwrap()
        .unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn observations_preserve_pending_live_and_retired_content_without_leaking_other_tenants() {
    let f = Fixture::new().await;
    assert_eq!(
        capacity(&f, f.project, scope(&f)).await,
        Err(UploadCapacityFailure::NotEnrolled)
    );
    f.enroll().await;
    let vector = vectors::vector("abc-text", 1);
    let root = vector.upload.root;
    assert!(discover(&f, f.project, root).await.content.is_none());
    assert!(references(&f, root).await.headroom.is_none());
    let p = f.permission(&vector).await;
    f.admit(p).await;
    let before = f.observe(p).await;
    let pending = discover(&f, f.project, root).await.content.unwrap();
    assert_eq!(pending.state, UploadContentState::Reserved);
    assert_eq!(f.observe(p).await, before);
    f.prepare(p, &vector).await;
    f.call(f.uploader, Command::Expose(root)).await.unwrap();
    let before = f.observe(p).await;
    let live = discover(&f, f.project, root).await;
    assert_eq!(live.content.unwrap().state, UploadContentState::Live);
    assert_eq!(
        references(&f, root).await.headroom.unwrap().fresh_retains,
        1
    );
    assert_eq!(discover(&f, f.project, root).await, live);
    assert_eq!(f.observe(p).await, before);
    f.call(
        f.project,
        Command::FixtureLifecycle(LifecycleCommand::Reference {
            object: p.request,
            reference: 1,
            operation: 1,
            retain: false,
        }),
    )
    .await
    .unwrap();
    for command in [
        LifecycleCommand::SubstituteDeletion(p.request),
        LifecycleCommand::SubstituteSettlement(p.request),
    ] {
        f.call(f.operator, Command::FixtureLifecycle(command))
            .await
            .unwrap();
    }
    f.restart().await;
    assert_eq!(
        discover(&f, f.project, root).await.content.unwrap().state,
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
    .await
    .unwrap();
    assert!(discover(&f, f.other, root).await.content.is_none());
}
