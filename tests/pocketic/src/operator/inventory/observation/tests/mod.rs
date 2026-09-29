use super::*;
use crate::operator::inventory::input::tests::prepared;
use blob_test_protocol::admission::Request;
use candid::Principal;
use ic_blob_storage::dto::reference::capacity::ReferenceHeadroom;
use ic_blob_storage::dto::tenant::TenantEnrollment;

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
            "lookup_content" => reply(Some(content(&s, &inventory, ContentState::Live))),
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
        "lookup_content" => reply(None::<ContentObservation>),
        _ => panic!("unexpected query"),
    })
    .unwrap();
    assert!(report.blocked);
    assert_eq!(report.value["blockers"], json!(["service_fenced"]));
    assert_eq!(report.value["capacity"]["fenced"], true);
}

fn content(s: &Selection, inventory: &Inventory, state: ContentState) -> ContentObservation {
    ContentObservation {
        state,
        request: Request {
            service: s.scope.service,
            tenant: s.scope.tenant,
            namespace: s.scope.namespace,
            id: 1,
            bytes: 3,
            root: *inventory.blobs[0].root.as_bytes(),
        },
    }
}

fn reply<T: CandidType>(value: T) -> Result<Vec<u8>, Failure> {
    candid::encode_one(Ok::<_, RemoteFailure>(value)).map_err(|_| Failure::InvalidReply)
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
            "lookup_content" => reply(None::<ContentObservation>),
            _ => panic!("unexpected call"),
        }
    })
    .unwrap();
    assert!(!report.blocked);
    assert_eq!(methods, ["blob_upload_capacity", "lookup_content"]);
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
}

#[test]
fn pending_and_retired_content_never_become_new_upload_candidates() {
    let s = selection();
    let inventory = prepared();
    for state in [
        ContentState::Reserved,
        ContentState::ExposurePossible,
        ContentState::Cancelled,
        ContentState::DeletionPending,
        ContentState::ProviderDeleted,
        ContentState::Settled,
    ] {
        let report = inspect(&s, &inventory, |method, _| match method {
            "blob_upload_capacity" => Ok(capacity_reply(capacity(&s))),
            "lookup_content" => reply(Some(content(&s, &inventory, state))),
            _ => panic!("unexpected call"),
        })
        .unwrap();
        assert!(report.blocked);
        assert_eq!(report.value["not_visible_demand"]["objects"], 0);
        assert_eq!(
            report.value["contents"][0]["observation"]["original_operation"],
            "1"
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
        "lookup_content" => reply(Some(content(&s, &inventory, ContentState::Live))),
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
        "lookup_content" => reply(Some(content(&s, &inventory, ContentState::Live))),
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
    let mut changed = content(&s, &inventory, ContentState::Live);
    changed.request.bytes = 4;
    assert_eq!(
        inspect(&s, &inventory, |method, _| {
            if method == "blob_upload_capacity" {
                Ok(capacity_reply(capacity(&s)))
            } else {
                reply(Some(changed))
            }
        }),
        Err(Failure::Binding)
    );
    assert!(matches!(
        decode_capacity(&vec![0; 4097]),
        Err(Failure::ReplyTooLarge)
    ));
    assert!(matches!(
        decode_capacity(&[0; 10]),
        Err(Failure::InvalidReply)
    ));
    let denied = candid::encode_one(Err::<UploadCapacityResponse, _>(
        UploadCapacityFailure::Denied,
    ))
    .unwrap();
    assert_eq!(decode_capacity(&denied), Err(Failure::Denied));
}

#[test]
fn reports_known_byte_history_metadata_and_suspension_blockers() {
    let s = selection();
    let inventory = prepared();
    let mut c = capacity(&s);
    c.enrollment.active = false;
    c.remaining_bytes = 2;
    c.remaining_objects = 0;
    c.remaining_manifest_chunks = 0;
    c.remaining_active_uploads = 0;
    c.max_headers = 1;
    let report = inspect(&s, &inventory, |method, _| {
        if method == "blob_upload_capacity" {
            Ok(capacity_reply(c))
        } else {
            reply(None::<ContentObservation>)
        }
    })
    .unwrap();
    assert!(report.blocked);
    assert_eq!(
        report.value["blockers"],
        json!([
            "byte_capacity",
            "manifest_capacity",
            "new_object_limits",
            "no_active_upload_slot",
            "object_history_capacity",
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
    let report = inspect(&s, &inventory, |method, _| {
        if method == "blob_upload_capacity" {
            Ok(capacity_reply(capacity(&s)))
        } else {
            reply(None::<ContentObservation>)
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
            "lookup_content" => reply(Some(content(&s, &inventory, ContentState::Live))),
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
