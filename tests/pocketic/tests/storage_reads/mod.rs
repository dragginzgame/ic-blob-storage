use super::*;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
mod download;
use blob_test_protocol::{
    admission::{
        ContentDescriptor, ContentLookup,
        input::{ReferenceInput, RetainedDescriptor, RetainedDescriptorInput},
    },
    storage::ProviderFact,
};
use ic_blob_storage_contracts::dto::tenant::TenantScope;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryFailure as DiscoveryFailure;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryRequest;
use ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryResponse;
use ic_blob_storage_contracts::dto::upload::history::UploadContentState;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryCursor as Cursor;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryEntry;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryFailure as HistoryFailure;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryFilter as Filter;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryPage as Page;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryRequest as ScanInput;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryScope as Scope;
impl Fixture {
    fn content_input(&self, request: Request) -> ContentLookup {
        ContentLookup {
            service: self.service,
            tenant: request.tenant,
            namespace: request.namespace,
            root: request.root,
        }
    }
    async fn discover(
        &self,
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
        let response: Result<UploadDiscoveryResponse, DiscoveryFailure> = self
            .harness
            .pic
            .query_blob_as(
                self.service,
                actor,
                "blob_lookup_content",
                request,
                REPLY_BYTES,
            )
            .await
            .unwrap();
        let response = response?;
        assert_eq!(response.request, request);
        assert_eq!(response.fenced, self.status().await.fenced);
        Ok(response.content)
    }
    async fn declaration(
        &self,
        actor: Principal,
        input: ContentLookup,
    ) -> Result<Option<ContentDescriptor>, Failure> {
        self.harness
            .pic
            .query_control_as(
                self.service,
                actor,
                "content_descriptor",
                (input,),
                REPLY_BYTES,
            )
            .await
            .map(|(reply,)| reply)
            .unwrap()
    }
    async fn retained(
        &self,
        actor: Principal,
        input: RetainedDescriptorInput,
    ) -> Result<Option<RetainedDescriptor>, Failure> {
        self.harness
            .pic
            .query_control_as(
                self.service,
                actor,
                "retained_content_descriptor",
                (input,),
                REPLY_BYTES,
            )
            .await
            .map(|(reply,)| reply)
            .unwrap()
    }
    async fn scan(&self, actor: Principal, input: ScanInput) -> Result<Page, HistoryFailure> {
        self.harness
            .pic
            .query_blob_as(
                self.service,
                actor,
                "blob_upload_history",
                input,
                REPLY_BYTES,
            )
            .await
            .unwrap()
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn durable_descriptors_require_the_consumers_exact_live_reference_through_upgrade() {
    let f = Fixture::new().await;
    f.enroll(None, true).await.unwrap();
    let (permission, preparation) = f.permission(u128::MAX, 1);
    let lookup = f.content_input(permission.request);
    assert_eq!(f.discover(f.tenant, lookup).await, Ok(None));
    f.admit(f.tenant, permission).await.unwrap();
    assert_eq!(f.declaration(f.tenant, lookup).await, Ok(None));
    f.prepare(&preparation).await.unwrap();
    let first = RetainedDescriptorInput {
        content: lookup,
        object: permission.request.id,
        incarnation: 1,
        reference: 1,
    };
    assert_eq!(f.retained(f.tenant, first).await, Ok(None));
    let mut reordered = preparation.clone();
    reordered.manifest.headers.reverse();
    assert_eq!(f.prepare(&reordered).await, Ok(false));
    f.expose(permission.request).await.unwrap();
    let second = RetainedDescriptorInput {
        reference: 2,
        ..first
    };
    f.reference(ReferenceInput {
        object: permission.request,
        reference: 2,
        operation: 1,
        retain: true,
    })
    .await
    .unwrap();
    f.reference(ReferenceInput {
        object: permission.request,
        reference: 1,
        operation: 2,
        retain: false,
    })
    .await
    .unwrap();
    assert_eq!(f.retained(f.tenant, first).await, Ok(None));
    assert_eq!(
        f.retained(
            f.tenant,
            RetainedDescriptorInput {
                incarnation: 2,
                ..second
            }
        )
        .await,
        Ok(None)
    );
    let view = f.retained(f.tenant, second).await.unwrap().unwrap();
    assert_eq!(view.descriptor.headers, preparation.manifest.headers);
    assert_eq!(view.descriptor.content.request, permission.request);
    for actor in [f.uploader, f.operator, f.controller] {
        assert_eq!(f.retained(actor, second).await, Err(Failure::Denied));
        assert_eq!(
            f.discover(actor, lookup).await,
            Err(DiscoveryFailure::Denied)
        );
    }
    assert_eq!(
        f.discover(
            f.other,
            ContentLookup {
                tenant: f.other,
                ..lookup
            }
        )
        .await,
        Ok(None)
    );
    let active = f.tenant().await.unwrap();
    f.enroll(Some(active), false).await.unwrap();
    let before = f.status().await;
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation("fixture_post_upgrade", f.operator),
            Some(f.controller),
        )
        .await
        .unwrap();
    assert_eq!(f.retained(f.tenant, second).await, Ok(Some(view)));
    assert_eq!(f.retained(f.tenant, first).await, Ok(None));
    assert_eq!(
        f.discover(f.tenant, lookup).await.unwrap().unwrap().request,
        admission_input(permission).upload
    );
    assert_eq!(
        f.status().await,
        Status {
            fenced: true,
            ..before
        }
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "One ordered cursor-isolation, resweep and fenced-upgrade journey"
)]
async fn bounded_cleanup_pages_reject_changed_cursors_and_require_new_sweeps() {
    let f = Fixture::new().await;
    f.enroll(None, true).await.unwrap();
    let (permission, preparation) = f.permission(1, 1);
    f.admit(f.tenant, permission).await.unwrap();
    f.prepare(&preparation).await.unwrap();
    f.expose(permission.request).await.unwrap();
    f.admit(f.tenant, f.permission(u128::MAX, 2).0)
        .await
        .unwrap();
    let query = ScanInput {
        service: f.service.into(),
        namespace: 1,
        scope: Scope::Tenant(f.tenant.into()),
        filter: Filter::DeletionPending,
        cursor: None,
    };
    let first = f.scan(f.tenant, query).await.unwrap();
    assert_eq!(first.entries, []);
    assert_eq!(first.scanned, 1);
    let cursor = first.next.unwrap();
    for changed in [
        Cursor {
            service: f.other.into(),
            ..cursor
        },
        Cursor {
            namespace: 2,
            ..cursor
        },
        Cursor {
            scope: Scope::Service,
            ..cursor
        },
        Cursor {
            filter: Filter::All,
            ..cursor
        },
        Cursor {
            after_tenant: f.other.into(),
            ..cursor
        },
    ] {
        assert_eq!(
            f.scan(
                f.tenant,
                ScanInput {
                    cursor: Some(changed),
                    ..query
                }
            )
            .await,
            Err(HistoryFailure::CursorScope)
        );
    }
    f.reference(ReferenceInput {
        object: permission.request,
        reference: 1,
        operation: 1,
        retain: false,
    })
    .await
    .unwrap();
    let mut end = f
        .scan(
            f.tenant,
            ScanInput {
                cursor: Some(cursor),
                ..query
            },
        )
        .await
        .unwrap();
    assert_eq!(end.scanned, 1);
    assert_eq!(end.entries, []);
    assert!(end.next.is_none());
    let mut restarted = f.scan(f.tenant, query).await.unwrap();
    assert_eq!(
        restarted.entries[0].request,
        admission_input(permission).upload
    );
    assert_eq!(
        restarted.entries[0].state,
        UploadContentState::DeletionPending
    );
    let operator_query = ScanInput {
        scope: Scope::Service,
        ..query
    };
    assert_eq!(
        f.scan(f.controller, operator_query).await,
        Err(HistoryFailure::Denied)
    );
    assert_eq!(
        f.scan(f.tenant, operator_query).await,
        Err(HistoryFailure::Denied)
    );
    assert_eq!(
        f.scan(f.operator, operator_query).await.unwrap().entries,
        restarted.entries
    );
    let foreign = f
        .scan(
            f.other,
            ScanInput {
                scope: Scope::Tenant(f.other.into()),
                ..query
            },
        )
        .await
        .unwrap();
    assert_eq!(foreign.scanned, 0);
    assert_eq!(foreign.entries, []);
    let before = f.status().await;
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation("fixture_post_upgrade", f.operator),
            Some(f.controller),
        )
        .await
        .unwrap();
    restarted.fenced = true;
    end.fenced = true;
    assert_eq!(f.scan(f.tenant, query).await, Ok(restarted));
    assert_eq!(
        f.scan(
            f.tenant,
            ScanInput {
                cursor: Some(cursor),
                ..query
            }
        )
        .await,
        Ok(end)
    );
    assert_eq!(
        f.status().await,
        Status {
            fenced: true,
            ..before
        }
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn shared_history_distinguishes_exposure_deletion_and_billing_cessation() {
    let f = Fixture::new().await;
    f.enroll(None, true).await.unwrap();
    let (permission, preparation) = f.permission(u128::MAX, 1);
    let query = ScanInput {
        service: f.service.into(),
        namespace: 1,
        scope: Scope::Service,
        filter: Filter::Outstanding,
        cursor: None,
    };
    let observe = async |state| {
        let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
        let page = f.scan(f.operator, query).await.unwrap();
        assert_eq!(page.request, query);
        assert_eq!(
            page.entries,
            vec![
                ic_blob_storage_contracts::dto::upload::history::UploadHistoryEntry {
                    request: admission_input(permission).upload,
                    state,
                }
            ]
        );
        assert_eq!(page.scanned, 1);
        assert_eq!(
            f.discover(f.tenant, f.content_input(permission.request))
                .await
                .unwrap(),
            page.entries.first().copied()
        );
        assert_eq!(page.next, None);
        assert!(
            f.harness
                .pic
                .get_stable_memory(f.service)
                .await
                .unwrap()
                .eq(&before),
            "history changed stable bytes"
        );
    };
    f.admit(f.tenant, permission).await.unwrap();
    observe(UploadContentState::Reserved).await;
    f.prepare(&preparation).await.unwrap();
    f.expose(permission.request).await.unwrap();
    observe(UploadContentState::Live).await;
    observe(UploadContentState::Live).await;
    f.reference(ReferenceInput {
        object: permission.request,
        reference: 1,
        operation: 1,
        retain: false,
    })
    .await
    .unwrap();
    observe(UploadContentState::DeletionPending).await;
    f.fact(permission.request, ProviderFact::Deleted)
        .await
        .unwrap();
    observe(UploadContentState::ProviderDeleted).await;
    f.fact(permission.request, ProviderFact::Settled)
        .await
        .unwrap();
    let settled = f.scan(f.operator, query).await.unwrap();
    assert_eq!(settled.entries, []);
    assert_eq!(settled.scanned, 1);
    let history = f
        .scan(
            f.operator,
            ScanInput {
                filter: Filter::All,
                ..query
            },
        )
        .await
        .unwrap();
    assert_eq!(history.entries[0].state, UploadContentState::Settled);
    assert_eq!(
        f.discover(f.tenant, f.content_input(permission.request))
            .await
            .unwrap(),
        history.entries.first().copied()
    );
    assert_eq!(
        history.entries[0].request,
        admission_input(permission).upload
    );
}
