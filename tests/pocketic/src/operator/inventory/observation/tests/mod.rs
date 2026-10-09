use super::*;
use crate::operator::inventory::input::tests::prepared;
use candid::Principal;
use ic_blob_storage_contracts::dto::reference::ReferenceUpload;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceHeadroom;
use ic_blob_storage_contracts::dto::tenant::TenantEnrollment;

fn selection() -> Selection {
    let service = Principal::from_slice(&[1, 1]);
    let tenant = Principal::from_slice(&[2, 1]);
    Selection::parse(
        &[
            "inspect",
            "--server",
            "127.0.0.1:12345",
            "--instance",
            "0",
            "--canister",
            &service.to_text(),
            "--caller",
            &tenant.to_text(),
            "--tenant",
            &tenant.to_text(),
            "--namespace",
            "1",
            "--inventory",
            "report.json",
        ]
        .map(str::to_owned),
    )
    .unwrap()
}

fn capacity(s: &Selection) -> UploadCapacityResponse {
    UploadCapacityResponse {
        scope: s.scope,
        enrollment: TenantEnrollment {
            generation: 1,
            active: true,
        },
        max_object_bytes: 10,
        max_headers: 16,
        max_header_bytes: 4096,
        remaining_objects: 10,
        remaining_active_uploads: 1,
        remaining_manifest_chunks: 10,
        remaining_logical_bytes: u128::from(u64::MAX) + 100,
        remaining_physical_bytes: u128::from(u64::MAX) + 100,
        remaining_liability_bytes: u128::from(u64::MAX) + 100,
        remaining_bytes: u128::from(u64::MAX) + 100,
        fenced: false,
    }
}
fn capacity_reply(value: UploadCapacityResponse) -> Vec<u8> {
    candid::encode_one(Ok::<_, UploadCapacityFailure>(value)).unwrap()
}
fn reference_reply(
    s: &Selection,
    inventory: &Inventory,
    headroom: Option<ReferenceHeadroom>,
) -> Vec<u8> {
    candid::encode_one(Ok::<_, ReferenceCapacityFailure>(
        ReferenceCapacityResponse {
            request: ReferenceCapacityRequest {
                scope: s.scope,
                root: *inventory.blobs[0].root.as_bytes(),
            },
            headroom,
            fenced: false,
        },
    ))
    .unwrap()
}

#[test]
fn reference_observation_rejects_wrong_echoes_and_keeps_later_restore_fences() {
    let s = selection();
    let inventory = prepared();
    let initial = ReferenceCapacityResponse {
        request: ReferenceCapacityRequest {
            scope: s.scope,
            root: *inventory.blobs[0].root.as_bytes(),
        },
        headroom: Some(ReferenceHeadroom {
            reference_slots: 1,
            unreserved_receipts: 2,
            release_reserved_receipts: 1,
            fresh_retains: 1,
        }),
        fenced: false,
    };
    let mut wrong_root = initial;
    wrong_root.request.root = [7; 32];
    let mut wrong_tenant = initial;
    wrong_tenant.request.scope.tenant = Principal::from_slice(&[9, 1]);
    let mut fenced = initial;
    fenced.fenced = true;
    for response in [wrong_root, wrong_tenant, fenced] {
        let result = inspect(&s, &inventory, |method, _| match method {
            "blob_upload_capacity" => Ok(capacity_reply(capacity(&s))),
            "blob_lookup_content" => discovery_reply(
                &s,
                &inventory,
                Some(content(&s, &inventory, UploadContentState::Live)),
            ),
            "blob_reference_capacity" => {
                Ok(candid::encode_one(Ok::<_, ReferenceCapacityFailure>(response)).unwrap())
            }
            _ => panic!("unexpected query"),
        });
        if response.fenced {
            let report = result.unwrap();
            assert!(report.blocked);
            assert_eq!(report.value["blockers"], json!(["service_fenced"]));
            assert_eq!(report.value["contents"][0]["observation"]["fenced"], true);
        } else {
            assert_eq!(result, Err(Failure::Binding));
        }
    }
}

#[test]
fn restored_capacity_is_a_reported_blocker_even_with_spare_quota() {
    let s = selection();
    let inventory = prepared();
    let mut capacity = capacity(&s);
    capacity.fenced = true;
    let report = inspect(&s, &inventory, |method, _| match method {
        "blob_upload_capacity" => Ok(capacity_reply(capacity)),
        "blob_lookup_content" => discovery_reply(&s, &inventory, None),
        _ => panic!("unexpected query"),
    })
    .unwrap();
    assert!(report.blocked);
    assert_eq!(report.value["blockers"], json!(["service_fenced"]));
    assert_eq!(report.value["capacity"]["fenced"], true);
}

fn content(s: &Selection, inventory: &Inventory, state: UploadContentState) -> UploadHistoryEntry {
    UploadHistoryEntry {
        state,
        request: ReferenceUpload {
            service: s.scope.service,
            tenant: s.scope.tenant,
            namespace: s.scope.namespace,
            upload: u128::MAX,
            object: u128::MAX - 1,
            incarnation: u128::MAX - 2,
            first_reference: u128::MAX - 3,
            bytes: 3,
            root: *inventory.blobs[0].root.as_bytes(),
        },
    }
}

fn discovery_reply(
    s: &Selection,
    inventory: &Inventory,
    content: Option<UploadHistoryEntry>,
) -> Result<Vec<u8>, Failure> {
    candid::encode_one(Ok::<_, UploadDiscoveryFailure>(UploadDiscoveryResponse {
        request: UploadDiscoveryRequest {
            scope: s.scope,
            root: *inventory.blobs[0].root.as_bytes(),
        },
        content,
        fenced: false,
    }))
    .map_err(|_| Failure::InvalidReply)
}

#[test]
fn absent_content_is_unproven_and_wide_capacity_is_rendered_exactly() {
    let s = selection();
    let inventory = prepared();
    let mut methods = vec![];
    let report = inspect(&s, &inventory, |method, _| {
        methods.push(method.to_owned());
        match method {
            "blob_upload_capacity" => Ok(capacity_reply(capacity(&s))),
            "blob_lookup_content" => discovery_reply(&s, &inventory, None),
            _ => panic!("unexpected call"),
        }
    })
    .unwrap();
    assert!(!report.blocked);
    assert_eq!(methods, ["blob_upload_capacity", "blob_lookup_content"]);
    assert_eq!(report.value["admission"], "not_proven");
    assert_eq!(
        report.value["contents"][0]["observation"]["status"],
        "not_visible"
    );
    assert_eq!(
        report.value["capacity"]["remaining_bytes"],
        capacity(&s).remaining_bytes.to_string()
    );
    assert_eq!(report.value["not_visible_demand"]["bytes"], "3");
    for field in [
        "remaining_logical_bytes",
        "remaining_physical_bytes",
        "remaining_liability_bytes",
    ] {
        assert_eq!(
            report.value["capacity"][field],
            (u128::from(u64::MAX) + 100).to_string()
        );
    }
}

#[test]
fn pending_and_retired_content_never_become_new_upload_candidates() {
    let s = selection();
    let inventory = prepared();
    for state in [
        UploadContentState::Reserved,
        UploadContentState::ExposurePossible,
        UploadContentState::Cancelled,
        UploadContentState::DeletionPending,
        UploadContentState::ProviderDeleted,
        UploadContentState::Settled,
    ] {
        let report = inspect(&s, &inventory, |method, _| match method {
            "blob_upload_capacity" => Ok(capacity_reply(capacity(&s))),
            "blob_lookup_content" => {
                discovery_reply(&s, &inventory, Some(content(&s, &inventory, state)))
            }
            _ => panic!("unexpected call"),
        })
        .unwrap();
        assert!(report.blocked);
        assert_eq!(
            report.value["contents"][0]["observation"]["original_object"],
            (u128::MAX - 1).to_string()
        );
        assert_eq!(
            report.value["contents"][0]["observation"]["original_incarnation"],
            (u128::MAX - 2).to_string()
        );
        assert_eq!(
            report.value["contents"][0]["observation"]["original_first_reference"],
            (u128::MAX - 3).to_string()
        );
        assert_eq!(report.value["not_visible_demand"]["objects"], 0);
        assert_eq!(
            report.value["contents"][0]["observation"]["original_operation"],
            u128::MAX.to_string()
        );
    }
}

#[test]
fn live_content_checks_each_assets_new_reference_and_stops_on_lost_observations() {
    let s = selection();
    let mut inventory = prepared();
    inventory.blobs[0].assets.push("alias".into());
    let report = inspect(&s, &inventory, |method, _| match method {
        "blob_upload_capacity" => Ok(capacity_reply(capacity(&s))),
        "blob_lookup_content" => discovery_reply(
            &s,
            &inventory,
            Some(content(&s, &inventory, UploadContentState::Live)),
        ),
        "blob_reference_capacity" => Ok(reference_reply(
            &s,
            &inventory,
            Some(ReferenceHeadroom {
                reference_slots: 1,
                unreserved_receipts: 2,
                release_reserved_receipts: 1,
                fresh_retains: 1,
            }),
        )),
        _ => panic!("unexpected call"),
    })
    .unwrap();
    assert!(report.blocked);
    assert_eq!(report.value["blockers"], json!(["reference_capacity"]));
    let failed = inspect(&s, &inventory, |method, _| match method {
        "blob_upload_capacity" => Ok(capacity_reply(capacity(&s))),
        "blob_lookup_content" => discovery_reply(
            &s,
            &inventory,
            Some(content(&s, &inventory, UploadContentState::Live)),
        ),
        "blob_reference_capacity" => Err(Failure::Transport),
        _ => panic!("unexpected call"),
    });
    assert_eq!(failed, Err(Failure::Transport));
}

#[test]
fn rejects_wrong_scope_wrong_content_and_malformed_or_denied_replies() {
    let s = selection();
    let inventory = prepared();
    let mut wrong = capacity(&s);
    wrong.scope.namespace = 2;
    assert_eq!(
        inspect(&s, &inventory, |_, _| Ok(capacity_reply(wrong))),
        Err(Failure::Binding)
    );
    let mut changed = content(&s, &inventory, UploadContentState::Live);
    changed.request.bytes = 4;
    assert_eq!(
        inspect(&s, &inventory, |method, _| {
            if method == "blob_upload_capacity" {
                Ok(capacity_reply(capacity(&s)))
            } else {
                discovery_reply(&s, &inventory, Some(changed))
            }
        }),
        Err(Failure::Binding)
    );
    assert!(matches!(
        decode_capacity(s.scope, &vec![0; 4097]),
        Err(Failure::ReplyTooLarge)
    ));
    assert!(matches!(
        decode_capacity(s.scope, &[0; 10]),
        Err(Failure::InvalidReply)
    ));
    let denied = candid::encode_one(Err::<UploadCapacityResponse, _>(
        UploadCapacityFailure::Denied,
    ))
    .unwrap();
    assert_eq!(decode_capacity(s.scope, &denied), Err(Failure::Denied));
}

#[test]
fn byte_dimensions_are_exact_and_inconsistent_minima_stop_before_discovery() {
    let s = selection();
    let inventory = prepared();
    for reason in [
        "logical_byte_capacity",
        "physical_byte_capacity",
        "liability_byte_capacity",
    ] {
        let mut c = capacity(&s);
        match reason {
            "logical_byte_capacity" => c.remaining_logical_bytes = 0,
            "physical_byte_capacity" => c.remaining_physical_bytes = 0,
            "liability_byte_capacity" => c.remaining_liability_bytes = 0,
            _ => unreachable!(),
        }
        c.remaining_bytes = 0;
        let report = inspect(&s, &inventory, |method, _| {
            if method == "blob_upload_capacity" {
                Ok(capacity_reply(c))
            } else {
                discovery_reply(&s, &inventory, None)
            }
        })
        .unwrap();
        assert_eq!(report.value["blockers"], json!([reason]));
        for (field, available) in [
            ("remaining_logical_bytes", c.remaining_logical_bytes),
            ("remaining_physical_bytes", c.remaining_physical_bytes),
            ("remaining_liability_bytes", c.remaining_liability_bytes),
        ] {
            assert_eq!(report.value["capacity"][field], available.to_string());
        }
        for bad_minimum in [1, u128::MAX] {
            c.remaining_bytes = bad_minimum;
            assert!(matches!(
                inspect(&s, &inventory, |method, _| {
                    assert_eq!(method, "blob_upload_capacity");
                    Ok(capacity_reply(c))
                }),
                Err(Failure::InvalidReply)
            ));
        }
        let mut understated = capacity(&s);
        understated.remaining_bytes = 0;
        assert!(matches!(
            inspect(&s, &inventory, |method, _| {
                assert_eq!(method, "blob_upload_capacity");
                Ok(capacity_reply(understated))
            }),
            Err(Failure::InvalidReply)
        ));
    }
}

#[test]
fn reports_known_byte_history_metadata_and_suspension_blockers() {
    let s = selection();
    let inventory = prepared();
    let mut c = capacity(&s);
    c.enrollment.active = false;
    c.remaining_physical_bytes = 2;
    c.remaining_bytes = 2;
    c.remaining_objects = 0;
    c.remaining_manifest_chunks = 0;
    c.remaining_active_uploads = 0;
    c.max_headers = 1;
    let report = inspect(&s, &inventory, |method, _| {
        if method == "blob_upload_capacity" {
            Ok(capacity_reply(c))
        } else {
            discovery_reply(&s, &inventory, None)
        }
    })
    .unwrap();
    assert!(report.blocked);
    assert_eq!(
        report.value["blockers"],
        json!([
            "manifest_capacity",
            "new_object_limits",
            "no_active_upload_slot",
            "object_history_capacity",
            "physical_byte_capacity",
            "tenant_suspended"
        ])
    );
}

#[test]
fn batch_size_does_not_require_every_new_upload_to_fit_concurrently() {
    let s = selection();
    let mut inventory = prepared();
    let mut second = prepared().blobs.pop().unwrap();
    second.root = "sha256:1111111111111111111111111111111111111111111111111111111111111111"
        .parse()
        .unwrap();
    second.assets = vec!["second".into()];
    inventory.blobs.push(second);
    let report = inspect(&s, &inventory, |method, args| {
        if method == "blob_upload_capacity" {
            Ok(capacity_reply(capacity(&s)))
        } else {
            candid::encode_one(Ok::<_, UploadDiscoveryFailure>(UploadDiscoveryResponse {
                request: candid::decode_one(&args).unwrap(),
                content: None,
                fenced: false,
            }))
            .map_err(|_| Failure::InvalidReply)
        }
    })
    .unwrap();
    assert!(!report.blocked);
    assert_eq!(report.value["not_visible_demand"]["objects"], 2);
    assert_eq!(report.value["capacity"]["remaining_active_uploads"], 1);
}

#[test]
fn inconsistent_capacity_and_disappeared_confirmed_content_fail_closed() {
    let s = selection();
    let inventory = prepared();
    let mut invalid = capacity(&s);
    invalid.enrollment.generation = 0;
    assert_eq!(
        inspect(&s, &inventory, |_, _| Ok(capacity_reply(invalid))),
        Err(Failure::InvalidReply)
    );
    for available in [
        None,
        Some(ReferenceHeadroom {
            reference_slots: 1,
            unreserved_receipts: 1,
            release_reserved_receipts: 1,
            fresh_retains: 1,
        }),
    ] {
        let result = inspect(&s, &inventory, |method, _| match method {
            "blob_upload_capacity" => Ok(capacity_reply(capacity(&s))),
            "blob_lookup_content" => discovery_reply(
                &s,
                &inventory,
                Some(content(&s, &inventory, UploadContentState::Live)),
            ),
            "blob_reference_capacity" => Ok(reference_reply(&s, &inventory, available)),
            _ => panic!("unexpected call"),
        });
        assert_eq!(
            result,
            Err(if available.is_none() {
                Failure::Conflict
            } else {
                Failure::InvalidReply
            })
        );
    }
}

#[test]
fn discovery_rejects_wrong_echoes_even_for_absence_and_keeps_a_later_fence() {
    let s = selection();
    let inventory = prepared();
    let response = UploadDiscoveryResponse {
        request: UploadDiscoveryRequest {
            scope: s.scope,
            root: *inventory.blobs[0].root.as_bytes(),
        },
        content: None,
        fenced: false,
    };
    let mut wrong_root = response;
    wrong_root.request.root = [7; 32];
    let mut wrong_tenant = response;
    wrong_tenant.request.scope.tenant = Principal::from_slice(&[9, 1]);
    let mut fenced = response;
    fenced.fenced = true;
    for response in [wrong_root, wrong_tenant, fenced] {
        let observed = inspect(&s, &inventory, |method, _| match method {
            "blob_upload_capacity" => Ok(capacity_reply(capacity(&s))),
            "blob_lookup_content" => {
                Ok(candid::encode_one(Ok::<_, UploadDiscoveryFailure>(response)).unwrap())
            }
            _ => panic!("unexpected query"),
        });
        if response.fenced {
            let report = observed.unwrap();
            assert!(report.blocked);
            assert_eq!(report.value["blockers"], json!(["service_fenced"]));
            assert_eq!(
                report.value["contents"][0]["observation"]["discovery_fenced"],
                true
            );
        } else {
            assert_eq!(observed, Err(Failure::Binding));
        }
    }
}

#[test]
fn discovery_rejects_zero_independent_identities_and_remote_refusals() {
    let s = selection();
    let inventory = prepared();
    let original = content(&s, &inventory, UploadContentState::Reserved);
    for request in [
        ReferenceUpload {
            upload: 0,
            ..original.request
        },
        ReferenceUpload {
            object: 0,
            ..original.request
        },
        ReferenceUpload {
            incarnation: 0,
            ..original.request
        },
        ReferenceUpload {
            first_reference: 0,
            ..original.request
        },
    ] {
        let result = inspect(&s, &inventory, |method, _| match method {
            "blob_upload_capacity" => Ok(capacity_reply(capacity(&s))),
            "blob_lookup_content" => discovery_reply(
                &s,
                &inventory,
                Some(UploadHistoryEntry {
                    request,
                    ..original
                }),
            ),
            _ => panic!("unexpected query"),
        });
        assert_eq!(result, Err(Failure::Binding));
    }
    for (remote, expected) in [
        (UploadDiscoveryFailure::Denied, Failure::Denied),
        (UploadDiscoveryFailure::Binding, Failure::Binding),
        (UploadDiscoveryFailure::Invalid, Failure::InvalidReply),
        (UploadDiscoveryFailure::Internal, Failure::InvalidReply),
    ] {
        let result = inspect(&s, &inventory, |method, _| match method {
            "blob_upload_capacity" => Ok(capacity_reply(capacity(&s))),
            "blob_lookup_content" => {
                Ok(candid::encode_one(Err::<UploadDiscoveryResponse, _>(remote)).unwrap())
            }
            _ => panic!("unexpected query"),
        });
        assert_eq!(result, Err(expected));
    }
}
