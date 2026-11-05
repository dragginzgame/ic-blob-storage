//! Actual caller isolation and passive discovery through the shared owner.
use super::*;
use blob_test_protocol::admission::{ContentLookup, ContentState};
use ic_blob_storage_contracts::dto::reference::ReferenceUpload;
use ic_blob_storage_contracts::dto::tenant::TenantScope;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryFailure as DiscoveryFailure;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryRequest;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryResponse;
use ic_blob_storage_contracts::dto::upload::history::UploadContentState;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryEntry;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;

mod descriptor;
mod planning;

async fn query(
    f: &Fixture,
    actor: Principal,
    input: ContentLookup,
) -> Result<Option<UploadHistoryEntry>, DiscoveryFailure> {
    let request = UploadDiscoveryRequest {
        scope: TenantScope {
            service: input.service.into(),
            tenant: input.tenant.into(),
            namespace: input.namespace,
        },
        root: input.root,
    };
    let response: Result<UploadDiscoveryResponse, DiscoveryFailure> = f
        .harness
        .pic
        .query_blob_as(
            f.service,
            actor,
            "blob_lookup_content",
            request,
            REPLY_BYTES,
        )
        .await
        .expect("typed content query");
    response.map(|response| {
        assert_eq!(response.request, request);
        assert!(!response.fenced);
        response.content
    })
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn content_discovery_keeps_exact_identity_isolation_and_uncertain_history() {
    let f = Fixture::new().await;
    let enrollment = f.enroll().await;
    let v = vectors::vector("abc-text", 1);
    let p = f.permission(&v).await;
    let input = ContentLookup {
        service: f.service,
        tenant: f.project,
        namespace: 1,
        root: p.request.root,
    };
    assert_eq!(query(&f, f.project, input).await, Ok(None));
    f.admit(p).await;
    let reserved = f.observe(p).await;
    for actor in [
        f.operator,
        f.controller,
        f.uploader,
        f.other,
        Principal::anonymous(),
    ] {
        assert_eq!(query(&f, actor, input).await, Err(DiscoveryFailure::Denied));
    }
    assert_eq!(
        query(
            &f,
            f.other,
            ContentLookup {
                tenant: f.other,
                ..input
            }
        )
        .await,
        Ok(None)
    );
    assert_eq!(
        query(
            &f,
            f.project,
            ContentLookup {
                service: f.other,
                ..input
            }
        )
        .await,
        Err(DiscoveryFailure::Binding)
    );
    assert_eq!(
        query(
            &f,
            f.project,
            ContentLookup {
                namespace: 2,
                ..input
            }
        )
        .await,
        Err(DiscoveryFailure::Binding)
    );
    assert_eq!(
        query(&f, f.project, input).await,
        Ok(Some(UploadHistoryEntry {
            request: identity(p.request),
            state: UploadContentState::Reserved
        }))
    );
    assert_eq!(f.observe(p).await, reserved);
    f.prepare(p, &v).await;
    assert_eq!(
        f.call(f.uploader, Command::Expose(p.request.root)).await,
        Ok(Outcome::Exposed)
    );
    let exposed = f.observe(p).await;
    f.call(f.operator, f.enrollment(Some(enrollment), false))
        .await
        .unwrap();
    f.restart().await;
    assert_eq!(
        query(&f, f.project, input).await,
        Ok(Some(UploadHistoryEntry {
            request: identity(p.request),
            state: UploadContentState::Live
        }))
    );
    assert_eq!(f.observe(p).await, exposed);
    assert_eq!(
        f.call(f.project, Command::Revoke(p.request)).await,
        Ok(Outcome::Changed(true))
    );
    assert_eq!(
        query(&f, f.project, input).await,
        Ok(Some(UploadHistoryEntry {
            request: identity(p.request),
            state: UploadContentState::Live
        }))
    );
    assert_eq!(f.observe(p).await.usage, exposed.usage);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn content_discovery_keeps_cancelled_history_after_stop_start() {
    let f = Fixture::new().await;
    f.enroll().await;
    let p = f.permission(&vectors::vector("abc-text", 1)).await;
    f.admit(p).await;
    f.call(f.project, Command::Revoke(p.request)).await.unwrap();
    let original = f.observe(p).await;
    f.restart().await;
    let input = ContentLookup {
        service: f.service,
        tenant: f.project,
        namespace: 1,
        root: p.request.root,
    };
    assert_eq!(
        query(&f, f.project, input).await,
        Ok(Some(UploadHistoryEntry {
            request: identity(p.request),
            state: UploadContentState::Cancelled
        }))
    );
    assert_eq!(f.observe(p).await, original);
}

fn identity(request: Request) -> ReferenceUpload {
    ReferenceUpload {
        service: request.service.into(),
        tenant: request.tenant.into(),
        namespace: request.namespace,
        upload: request.id,
        object: request.id,
        incarnation: 1,
        first_reference: 1,
        root: request.root,
        bytes: request.bytes,
    }
}
