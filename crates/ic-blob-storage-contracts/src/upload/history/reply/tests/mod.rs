use crate::dto::reference::ReferenceUpload;
use crate::dto::upload::history::UploadHistoryEntry;
use crate::upload::history::reply::*;

fn tenant() -> Principal {
    Principal::from_slice(&[2, 1])
}
fn input() -> UploadHistoryRequest {
    UploadHistoryRequest {
        service: Principal::from_slice(&[1, 1]),
        namespace: u128::MAX,
        scope: UploadHistoryScope::Service,
        filter: UploadHistoryFilter::All,
        cursor: None,
    }
}
fn entry(id: u128) -> UploadHistoryEntry {
    let mut root = [0; 32];
    root[..16].copy_from_slice(&id.to_be_bytes());
    UploadHistoryEntry {
        request: ReferenceUpload {
            service: input().service,
            tenant: tenant(),
            namespace: u128::MAX,
            upload: id,
            object: if id == u128::MAX {
                u128::MAX - 1
            } else {
                id + 1
            },
            incarnation: u128::MAX - 2,
            first_reference: u128::MAX - 3,
            root,
            bytes: u64::MAX,
        },
        state: UploadContentState::Reserved,
    }
}
fn limits() -> UploadHistoryReplyLimits {
    UploadHistoryReplyLimits {
        bytes: 65536.try_into().unwrap(),
        entries: 32.try_into().unwrap(),
        scanned: 64.try_into().unwrap(),
    }
}
fn position_for(request: UploadHistoryRequest, id: u128) -> UploadHistoryCursor {
    UploadHistoryCursor {
        service: request.service,
        namespace: request.namespace,
        scope: request.scope,
        filter: request.filter,
        after_tenant: tenant(),
        after_request: id,
    }
}
fn encoded(p: &UploadHistoryPage) -> Vec<u8> {
    candid::encode_one(Ok::<_, UploadHistoryFailure>(p)).unwrap()
}
fn observation() -> UploadHistoryPage {
    UploadHistoryPage {
        request: input(),
        entries: vec![entry(1), entry(u128::MAX)],
        scanned: 2,
        next: Some(position_for(input(), u128::MAX)),
        fenced: true,
    }
}

#[test]
fn current_pages_keep_full_width_independent_ids_fences_and_scoped_request_echo() {
    let p = observation();
    assert_eq!(page(input(), &encoded(&p), limits()), Ok(p.clone()));
    assert_eq!(
        candid::decode_one::<UploadHistoryRequest>(&request(input()).unwrap()).unwrap(),
        input()
    );
    let mut changed = p.clone();
    changed.request.filter = UploadHistoryFilter::Active;
    assert_eq!(
        page(input(), &encoded(&changed), limits()),
        Err(UploadHistoryReplyError::Binding)
    );
    for upload in [
        ReferenceUpload {
            namespace: 1,
            ..entry(1).request
        },
        ReferenceUpload {
            service: tenant(),
            ..entry(1).request
        },
    ] {
        let mut changed = p.clone();
        changed.entries[0].request = upload;
        assert_eq!(
            page(input(), &encoded(&changed), limits()),
            Err(UploadHistoryReplyError::Binding)
        );
    }
    let mut tenant_page = p;
    tenant_page.request.scope = UploadHistoryScope::Tenant(tenant());
    tenant_page.next.as_mut().unwrap().scope = tenant_page.request.scope;
    assert_eq!(
        page(tenant_page.request, &encoded(&tenant_page), limits()),
        Ok(tenant_page.clone())
    );
    tenant_page.entries[0].request.tenant = input().service;
    assert_eq!(
        page(tenant_page.request, &encoded(&tenant_page), limits()),
        Err(UploadHistoryReplyError::Binding)
    );
}

#[test]
fn empty_filtered_pages_and_progress_after_unreturned_rows_remain_valid() {
    let input = UploadHistoryRequest {
        filter: UploadHistoryFilter::Active,
        cursor: Some(position_for(
            UploadHistoryRequest {
                filter: UploadHistoryFilter::Active,
                ..input()
            },
            1,
        )),
        ..input()
    };
    let mut p = UploadHistoryPage {
        request: input,
        entries: vec![],
        scanned: 64,
        next: Some(position_for(input, 65)),
        fenced: true,
    };
    assert_eq!(page(input, &encoded(&p), limits()), Ok(p.clone()));
    p.entries.push(entry(2));
    assert_eq!(page(input, &encoded(&p), limits()), Ok(p.clone()));
    p.next = None;
    assert_eq!(page(input, &encoded(&p), limits()), Ok(p));
}

#[test]
fn service_pages_order_tenants_before_request_ids_and_continue_across_the_boundary() {
    let second = Principal::from_slice(&[3, 1]);
    let mut p = observation();
    p.entries = vec![
        entry(u128::MAX),
        UploadHistoryEntry {
            request: ReferenceUpload {
                tenant: second,
                ..entry(1).request
            },
            ..entry(1)
        },
    ];
    p.next = Some(UploadHistoryCursor {
        after_tenant: second,
        after_request: 1,
        ..p.next.unwrap()
    });
    assert_eq!(page(input(), &encoded(&p), limits()), Ok(p.clone()));
    let resumed = UploadHistoryRequest {
        cursor: Some(position_for(input(), u128::MAX)),
        ..input()
    };
    let next = UploadHistoryPage {
        request: resumed,
        entries: vec![p.entries[1]],
        scanned: 1,
        next: None,
        fenced: true,
    };
    assert_eq!(page(resumed, &encoded(&next), limits()), Ok(next));
    p.entries.reverse();
    assert_eq!(
        page(input(), &encoded(&p), limits()),
        Err(UploadHistoryReplyError::Invalid)
    );
}

#[test]
fn malformed_entry_identities_filters_order_and_counts_reject() {
    let original = observation();
    for u in [
        ReferenceUpload {
            upload: 0,
            ..entry(1).request
        },
        ReferenceUpload {
            object: 0,
            ..entry(1).request
        },
        ReferenceUpload {
            incarnation: 0,
            ..entry(1).request
        },
        ReferenceUpload {
            first_reference: 0,
            ..entry(1).request
        },
        ReferenceUpload {
            bytes: 0,
            ..entry(1).request
        },
        ReferenceUpload {
            tenant: Principal::anonymous(),
            ..entry(1).request
        },
    ] {
        let mut p = original.clone();
        p.entries[0].request = u;
        assert_eq!(
            page(input(), &encoded(&p), limits()),
            Err(UploadHistoryReplyError::Invalid)
        );
    }
    let mut duplicate = original.clone();
    duplicate.entries[1] = duplicate.entries[0];
    let mut reversed = original.clone();
    reversed.entries.reverse();
    let mut duplicate_root = original.clone();
    duplicate_root.entries[1].request.root = duplicate_root.entries[0].request.root;
    let mut duplicate_object = original.clone();
    duplicate_object.entries[1].request.object = duplicate_object.entries[0].request.object;
    let mut undercounted = original.clone();
    undercounted.scanned = 1;
    let mut omitted_all = original.clone();
    omitted_all.entries.clear();
    let active = UploadHistoryRequest {
        filter: UploadHistoryFilter::Active,
        ..input()
    };
    let mismatched_filter = UploadHistoryPage {
        request: active,
        entries: vec![UploadHistoryEntry {
            state: UploadContentState::Settled,
            ..entry(1)
        }],
        scanned: 1,
        next: None,
        fenced: false,
    };
    for p in [
        duplicate,
        reversed,
        duplicate_root,
        duplicate_object,
        undercounted,
        omitted_all,
        mismatched_filter,
    ] {
        assert_eq!(
            page(p.request, &encoded(&p), limits()),
            Err(UploadHistoryReplyError::Invalid)
        );
    }
}

#[test]
fn continuation_cannot_reset_regress_or_change_original_scope() {
    let original = observation();
    for c in [
        UploadHistoryCursor {
            namespace: 1,
            ..original.next.unwrap()
        },
        UploadHistoryCursor {
            scope: UploadHistoryScope::Tenant(tenant()),
            ..original.next.unwrap()
        },
        UploadHistoryCursor {
            filter: UploadHistoryFilter::Outstanding,
            ..original.next.unwrap()
        },
    ] {
        let p = UploadHistoryPage {
            next: Some(c),
            ..original.clone()
        };
        assert_eq!(
            page(input(), &encoded(&p), limits()),
            Err(UploadHistoryReplyError::Binding)
        );
        assert_eq!(
            request(UploadHistoryRequest {
                cursor: Some(c),
                ..input()
            }),
            Err(UploadHistoryReplyError::Binding)
        );
    }
    let active = UploadHistoryRequest {
        filter: UploadHistoryFilter::Active,
        ..input()
    };
    let prior = position_for(active, 10);
    let active = UploadHistoryRequest {
        cursor: Some(prior),
        ..active
    };
    for next in [position_for(active, 9), prior, position_for(active, 0)] {
        let p = UploadHistoryPage {
            request: active,
            entries: vec![],
            scanned: 2,
            next: Some(next),
            fenced: false,
        };
        assert_eq!(
            page(active, &encoded(&p), limits()),
            Err(UploadHistoryReplyError::Invalid)
        );
    }
    for (entries, scanned, next) in [
        (vec![], 0, Some(position_for(active, 11))),
        (vec![entry(12)], 1, Some(position_for(active, 11))),
        (vec![entry(10)], 1, None),
    ] {
        let p = UploadHistoryPage {
            request: active,
            entries,
            scanned,
            next,
            fenced: false,
        };
        assert_eq!(
            page(active, &encoded(&p), limits()),
            Err(UploadHistoryReplyError::Invalid)
        );
    }
    let mut all = original;
    all.entries.pop();
    all.scanned = 1;
    assert_eq!(
        page(input(), &encoded(&all), limits()),
        Err(UploadHistoryReplyError::Invalid)
    );
}

#[test]
fn independent_limits_malformed_replies_and_typed_refusals_do_not_become_empty_history() {
    let original = observation();
    let bytes = encoded(&original);
    let budget = limits();
    for limits in [
        UploadHistoryReplyLimits {
            bytes: 1.try_into().unwrap(),
            ..budget
        },
        UploadHistoryReplyLimits {
            entries: 1.try_into().unwrap(),
            ..budget
        },
        UploadHistoryReplyLimits {
            scanned: 1.try_into().unwrap(),
            ..budget
        },
    ] {
        assert_eq!(
            page(input(), &bytes, limits),
            Err(UploadHistoryReplyError::Limit)
        );
    }
    assert_eq!(
        page(input(), &candid::encode_one(true).unwrap(), budget),
        Err(UploadHistoryReplyError::Invalid)
    );
    for error in [
        UploadHistoryFailure::Denied,
        UploadHistoryFailure::Binding,
        UploadHistoryFailure::CursorScope,
        UploadHistoryFailure::Internal,
    ] {
        let response: Result<UploadHistoryPage, _> = Err(error);
        assert_eq!(
            page(input(), &candid::encode_one(response).unwrap(), budget),
            Err(UploadHistoryReplyError::Remote(error))
        );
    }
    for invalid in [
        UploadHistoryRequest {
            namespace: 0,
            ..input()
        },
        UploadHistoryRequest {
            service: Principal::anonymous(),
            ..input()
        },
        UploadHistoryRequest {
            scope: UploadHistoryScope::Tenant(Principal::management_canister()),
            ..input()
        },
    ] {
        assert_eq!(request(invalid), Err(UploadHistoryReplyError::Invalid));
    }
}
