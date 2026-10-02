use super::*;
mod download;
use blob_test_protocol::{
    admission::{
        ContentDescriptor, ContentLookup,
        input::{ReferenceInput, RetainedDescriptor, RetainedDescriptorInput},
    },
    storage::ProviderFact,
};
use ic_blob_storage::dto::upload::history::{
    UploadContentState, UploadHistoryCursor as Cursor, UploadHistoryFailure as HistoryFailure,
    UploadHistoryFilter as Filter, UploadHistoryPage as Page, UploadHistoryRequest as ScanInput,
    UploadHistoryScope as Scope,
};
use ic_blob_storage::dto::{
    tenant::TenantScope,
    upload::{
        discovery::{
            UploadDiscoveryFailure as DiscoveryFailure, UploadDiscoveryRequest,
            UploadDiscoveryResponse,
        },
        history::UploadHistoryEntry,
    },
};
impl Fixture {
    fn content_input(&self, request: Request) -> ContentLookup {
        ContentLookup {
            service: self.service,
            tenant: request.tenant,
            namespace: request.namespace,
            root: request.root,
        }
    }
    fn discover(
        &self,
        actor: Principal,
        input: ContentLookup,
    ) -> Result<Option<UploadHistoryEntry>, DiscoveryFailure> {
        let request = UploadDiscoveryRequest {
            scope: TenantScope {
                service: input.service,
                tenant: input.tenant,
                namespace: input.namespace,
            },
            root: input.root,
        };
        let response: Result<UploadDiscoveryResponse, DiscoveryFailure> = self
            .harness
            .pic
            .query_candid_as(self.service, actor, "blob_lookup_content", (request,))
            .unwrap();
        response.map(|response| {
            assert_eq!(response.request, request);
            assert_eq!(response.fenced, self.status().fenced);
            response.content
        })
    }
    fn declaration(
        &self,
        actor: Principal,
        input: ContentLookup,
    ) -> Result<Option<ContentDescriptor>, Failure> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, "content_descriptor", (input,))
            .unwrap()
    }
    fn retained(
        &self,
        actor: Principal,
        input: RetainedDescriptorInput,
    ) -> Result<Option<RetainedDescriptor>, Failure> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, "retained_content_descriptor", (input,))
            .unwrap()
    }
    fn scan(&self, actor: Principal, input: ScanInput) -> Result<Page, HistoryFailure> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, "blob_upload_history", (input,))
            .unwrap()
    }
}
#[test]
fn durable_descriptors_require_the_consumers_exact_live_reference_through_upgrade() {
    let f = Fixture::new();
    f.enroll(None, true).unwrap();
    let (permission, preparation) = f.permission(u128::MAX, 1);
    let lookup = f.content_input(permission.request);
    assert_eq!(f.discover(f.tenant, lookup), Ok(None));
    f.admit(f.tenant, permission).unwrap();
    assert_eq!(f.declaration(f.tenant, lookup), Ok(None));
    f.prepare(&preparation).unwrap();
    let first = RetainedDescriptorInput {
        content: lookup,
        object: permission.request.id,
        incarnation: 1,
        reference: 1,
    };
    assert_eq!(f.retained(f.tenant, first), Ok(None));
    let mut reordered = preparation.clone();
    reordered.manifest.headers.reverse();
    assert_eq!(f.prepare(&reordered), Ok(false));
    f.expose(permission.request).unwrap();
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
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
    .unwrap();
    f.reference(ReferenceInput {
        object: permission.request,
        reference: 1,
        operation: 2,
        retain: false,
    })
    .unwrap();
    assert_eq!(f.retained(f.tenant, first), Ok(None));
    assert_eq!(
        f.retained(
            f.tenant,
            RetainedDescriptorInput {
                incarnation: 2,
                ..second
            }
        ),
        Ok(None)
    );
    let view = f.retained(f.tenant, second).unwrap().unwrap();
    assert_eq!(view.descriptor.headers, preparation.manifest.headers);
    assert_eq!(view.descriptor.content.request, permission.request);
    for actor in [f.uploader, f.operator, f.controller] {
        assert_eq!(f.retained(actor, second), Err(Failure::Denied));
        assert_eq!(f.discover(actor, lookup), Err(DiscoveryFailure::Denied));
    }
    assert_eq!(
        f.discover(
            f.other,
            ContentLookup {
                tenant: f.other,
                ..lookup
            }
        ),
        Ok(None)
    );
    let active = f.tenant().unwrap();
    f.enroll(Some(active), false).unwrap();
    let before = f.status();
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            candid::encode_one(f.operator).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(f.retained(f.tenant, second), Ok(Some(view)));
    assert_eq!(f.retained(f.tenant, first), Ok(None));
    assert_eq!(
        f.discover(f.tenant, lookup).unwrap().unwrap().request,
        admission_input(permission).upload
    );
    assert_eq!(
        f.status(),
        Status {
            fenced: true,
            ..before
        }
    );
}
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One ordered cursor-isolation, resweep and fenced-upgrade journey"
)]
fn bounded_cleanup_pages_reject_changed_cursors_and_require_new_sweeps() {
    let f = Fixture::new();
    f.enroll(None, true).unwrap();
    let (permission, preparation) = f.permission(1, 1);
    f.admit(f.tenant, permission).unwrap();
    f.prepare(&preparation).unwrap();
    f.expose(permission.request).unwrap();
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    f.admit(f.tenant, f.permission(u128::MAX, 2).0).unwrap();
    let query = ScanInput {
        service: f.service,
        namespace: 1,
        scope: Scope::Tenant(f.tenant),
        filter: Filter::DeletionPending,
        cursor: None,
    };
    let first = f.scan(f.tenant, query).unwrap();
    assert_eq!(first.entries, []);
    assert_eq!(first.scanned, 1);
    let cursor = first.next.unwrap();
    for changed in [
        Cursor {
            service: f.other,
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
            after_tenant: f.other,
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
            ),
            Err(HistoryFailure::CursorScope)
        );
    }
    f.reference(ReferenceInput {
        object: permission.request,
        reference: 1,
        operation: 1,
        retain: false,
    })
    .unwrap();
    let mut end = f
        .scan(
            f.tenant,
            ScanInput {
                cursor: Some(cursor),
                ..query
            },
        )
        .unwrap();
    assert_eq!(end.scanned, 1);
    assert_eq!(end.entries, []);
    assert!(end.next.is_none());
    let mut restarted = f.scan(f.tenant, query).unwrap();
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
        f.scan(f.controller, operator_query),
        Err(HistoryFailure::Denied)
    );
    assert_eq!(
        f.scan(f.tenant, operator_query),
        Err(HistoryFailure::Denied)
    );
    assert_eq!(
        f.scan(f.operator, operator_query).unwrap().entries,
        restarted.entries
    );
    let foreign = f
        .scan(
            f.other,
            ScanInput {
                scope: Scope::Tenant(f.other),
                ..query
            },
        )
        .unwrap();
    assert_eq!(foreign.scanned, 0);
    assert_eq!(foreign.entries, []);
    let before = f.status();
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            candid::encode_one(f.operator).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    restarted.fenced = true;
    end.fenced = true;
    assert_eq!(f.scan(f.tenant, query), Ok(restarted));
    assert_eq!(
        f.scan(
            f.tenant,
            ScanInput {
                cursor: Some(cursor),
                ..query
            }
        ),
        Ok(end)
    );
    assert_eq!(
        f.status(),
        Status {
            fenced: true,
            ..before
        }
    );
}

#[test]
fn shared_history_distinguishes_exposure_deletion_and_billing_cessation() {
    let f = Fixture::new();
    f.enroll(None, true).unwrap();
    let (permission, preparation) = f.permission(u128::MAX, 1);
    let query = ScanInput {
        service: f.service,
        namespace: 1,
        scope: Scope::Service,
        filter: Filter::Outstanding,
        cursor: None,
    };
    let observe = |state| {
        let before = f.harness.pic.get_stable_memory(f.service);
        let page = f.scan(f.operator, query).unwrap();
        assert_eq!(page.request, query);
        assert_eq!(
            page.entries,
            vec![ic_blob_storage::dto::upload::history::UploadHistoryEntry {
                request: admission_input(permission).upload,
                state,
            }]
        );
        assert_eq!(page.scanned, 1);
        assert_eq!(
            f.discover(f.tenant, f.content_input(permission.request))
                .unwrap(),
            page.entries.first().copied()
        );
        assert_eq!(page.next, None);
        assert!(
            f.harness.pic.get_stable_memory(f.service).eq(&before),
            "history changed stable bytes"
        );
    };
    f.admit(f.tenant, permission).unwrap();
    observe(UploadContentState::Reserved);
    f.prepare(&preparation).unwrap();
    f.expose(permission.request).unwrap();
    observe(UploadContentState::ExposurePossible);
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    observe(UploadContentState::Live);
    f.reference(ReferenceInput {
        object: permission.request,
        reference: 1,
        operation: 1,
        retain: false,
    })
    .unwrap();
    observe(UploadContentState::DeletionPending);
    f.fact(permission.request, ProviderFact::Deleted).unwrap();
    observe(UploadContentState::ProviderDeleted);
    f.fact(permission.request, ProviderFact::Settled).unwrap();
    let settled = f.scan(f.operator, query).unwrap();
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
        .unwrap();
    assert_eq!(history.entries[0].state, UploadContentState::Settled);
    assert_eq!(
        f.discover(f.tenant, f.content_input(permission.request))
            .unwrap(),
        history.entries.first().copied()
    );
    assert_eq!(
        history.entries[0].request,
        admission_input(permission).upload
    );
}
