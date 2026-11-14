use std::num::{NonZeroU64, NonZeroU128, NonZeroUsize};

use ic_platform_types::Principal;

use super::{
    Error as E, Limits, LocalModel, ObjectView, ReferenceStatus as R, ReferenceView, Root,
    UploadRequest, UploadState as U, UploadView, Usage,
};
use crate::caffeine::{Header, ManifestLimits, VerificationError};
use crate::references::{ReferenceError, RetentionState};

fn actor(value: u8) -> Principal {
    Principal::from_slice(&[value, 1])
}
fn id(value: u128) -> NonZeroU128 {
    NonZeroU128::new(value).unwrap()
}
fn limit(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).unwrap()
}
fn bytes(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).unwrap()
}
fn hash(hex: &str) -> Root {
    std::array::from_fn(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap())
}
fn abc() -> Root {
    hash("b5b435d47a4cce7dfec493b1e020c5308d9c7fe90add1aff510f9c2a9c4ea8e7")
}
fn other() -> Root {
    hash("82123b33e498318c7b9849ae6aa8fd2b2735d5a478077fce66a9d363cb838941")
}

fn limits() -> Limits {
    Limits {
        manifest: ManifestLimits {
            max_content_bytes: bytes(1024 * 1024),
            max_chunks: limit(1),
            max_headers: limit(2),
            max_header_bytes: limit(100),
        },
        uploads: limit(8),
        references: limit(8),
        logical_bytes: bytes(10),
        exposure_bytes: bytes(10),
    }
}
fn request(n: u128) -> UploadRequest {
    UploadRequest {
        id: id(n),
        uploader: actor(2),
        root: abc(),
        bytes: 3,
        first_reference: id(n),
        expires_at: 100,
    }
}
fn model() -> LocalModel {
    LocalModel::new(actor(1), limits()).unwrap()
}
fn admit(m: &mut LocalModel, request: UploadRequest) -> U {
    m.authorize(actor(1), request, &[request.root], &[], 10)
        .unwrap()
        .state
}
fn usage(m: &LocalModel) -> Usage {
    m.usage(actor(1)).unwrap()
}
fn state(m: &LocalModel, n: u128) -> U {
    m.inspect_upload(actor(1), id(n)).unwrap().state
}

fn reference(root: Root, status: R) -> ReferenceView {
    ReferenceView { root, status }
}

#[test]
fn overlapping_upload_ids_share_one_object_and_never_gain_second_registration() {
    let mut m = model();
    assert_eq!(admit(&mut m, request(1)), U::Reserved);
    let mut expected_object = ObjectView {
        original_upload: UploadView {
            request: request(1),
            state: U::Reserved,
        },
        retention: None,
        live_references: 0,
    };
    assert_eq!(m.inspect_object(actor(1), abc()), Ok(expected_object));
    assert_eq!(
        usage(&m),
        Usage {
            reserved_bytes: 3,
            uploads: 1,
            references: 1,
            ..Usage::default()
        }
    );
    m.register(actor(2), abc(), 11).unwrap();
    let second = UploadRequest {
        uploader: actor(3),
        ..request(2)
    };
    assert_eq!(admit(&mut m, second), U::Attached);
    expected_object.original_upload.state = U::Registered;
    expected_object.retention = Some(RetentionState::Open);
    expected_object.live_references = 2;
    assert_eq!(m.inspect_object(actor(1), abc()), Ok(expected_object));
    // An attached use does not become the root's original uploader.
    assert_eq!(m.register(actor(3), abc(), 12), Err(E::Denied));
    assert_eq!(
        usage(&m),
        Usage {
            reserved_bytes: 0,
            live_bytes: 3,
            unresolved_bytes: 3,
            uploads: 2,
            references: 2
        }
    );
    m.release(actor(1), id(1)).unwrap();
    expected_object.live_references = 1;
    assert_eq!(m.inspect_object(actor(1), abc()), Ok(expected_object));
    assert_eq!(
        m.inspect_reference(actor(1), id(2)),
        Ok(reference(abc(), R::Live))
    );
    // Releasing the original use does not retire the still-Open object.
    m.retain(actor(1), abc(), id(7)).unwrap();
    m.release(actor(1), id(7)).unwrap();
    assert_eq!(usage(&m).live_bytes, 3);
    m.release(actor(1), id(2)).unwrap();
    expected_object.retention = Some(RetentionState::Closing);
    expected_object.live_references = 0;
    assert_eq!(m.inspect_object(actor(1), abc()), Ok(expected_object));
    assert_eq!(usage(&m).live_bytes, 0);
    assert_eq!(usage(&m).unresolved_bytes, 3);
    let before = usage(&m);
    assert_eq!(
        m.authorize(actor(1), request(3), &[abc()], &[], 12),
        Err(E::Reference(ReferenceError::Closing))
    );
    assert_eq!(
        m.retain(actor(1), abc(), id(3)),
        Err(E::Reference(ReferenceError::Closing))
    );
    assert_eq!(m.register(actor(2), abc(), 12), Err(E::NotReserved));
    assert_eq!(usage(&m), before);
}

#[test]
fn root_registration_resolves_the_original_uploader_deadline_and_first_reference() {
    let mut m = model();
    let original = request(u128::MAX);
    let other_grant = UploadRequest {
        root: other(),
        bytes: 1,
        uploader: actor(3),
        expires_at: 12,
        ..request(1)
    };
    admit(&mut m, original);
    admit(&mut m, other_grant);
    let before = usage(&m);
    assert_eq!(m.register(actor(3), original.root, 11), Err(E::Denied));
    assert_eq!(m.register(actor(2), other_grant.root, 11), Err(E::Denied));
    assert_eq!(usage(&m), before);

    // A newer grant for another root cannot replace this root's permission.
    m.register(original.uploader, original.root, 12).unwrap();
    assert_eq!(state(&m, u128::MAX), U::Registered);
    assert_eq!(state(&m, 1), U::Reserved);
    assert_eq!(
        m.inspect_reference(actor(1), original.first_reference),
        Ok(reference(abc(), R::Live))
    );
    assert_eq!(
        m.inspect_reference(actor(1), other_grant.first_reference),
        Ok(reference(other_grant.root, R::Reserved))
    );
    assert_eq!(usage(&m).reserved_bytes, 1);
    assert_eq!(usage(&m).live_bytes, 3);
    assert_eq!(usage(&m).unresolved_bytes, 3);
    let before = usage(&m);
    assert_eq!(
        m.register(other_grant.uploader, other_grant.root, 12),
        Err(E::Expired)
    );
    assert_eq!(usage(&m), before);
    m.expire(actor(1), other_grant.id, 12).unwrap();
    m.release(actor(1), original.first_reference).unwrap();
    assert_eq!(
        m.inspect_reference(actor(1), other_grant.first_reference),
        Ok(reference(other_grant.root, R::Retired))
    );
    assert_eq!(
        m.inspect_reference(actor(1), original.first_reference),
        Ok(reference(original.root, R::Released))
    );
    assert_eq!(usage(&m).reserved_bytes, 0);
    assert_eq!(usage(&m).live_bytes, 0);
    assert_eq!(usage(&m).unresolved_bytes, 3);
}

#[test]
fn captured_actor_not_id_knowledge_controls_every_operation() {
    let mut m = model();
    assert_eq!(
        m.authorize(actor(2), request(1), &[abc()], &[], 10),
        Err(E::Denied)
    );
    assert_eq!(usage(&m), Usage::default());
    admit(&mut m, request(1));
    let before = usage(&m);
    for caller in [actor(1), actor(3), Principal::anonymous()] {
        assert_eq!(m.register(caller, abc(), 11), Err(E::Denied));
    }
    assert_eq!(m.inspect_upload(actor(3), id(1)), Err(E::Denied));
    assert_eq!(
        m.inspect_upload(actor(2), id(1)).unwrap().state,
        U::Reserved
    );
    for caller in [actor(2), actor(3), Principal::anonymous()] {
        assert_eq!(m.retain(caller, abc(), id(2)), Err(E::Denied));
        assert_eq!(m.release(caller, id(1)), Err(E::Denied));
        assert_eq!(m.expire(caller, id(1), 100), Err(E::Denied));
        assert_eq!(m.inspect_reference(caller, id(1)), Err(E::Denied));
        assert_eq!(m.inspect_object(caller, abc()), Err(E::Denied));
        assert_eq!(m.inspect_object(caller, other()), Err(E::Denied));
        assert_eq!(m.usage(caller), Err(E::Denied));
    }
    assert_eq!(usage(&m), before);
    assert_eq!(state(&m, 1), U::Reserved);
    for invalid in [Principal::anonymous(), Principal::management_canister()] {
        assert!(matches!(
            LocalModel::new(invalid, limits()),
            Err(E::InvalidPrincipal)
        ));
        assert_eq!(
            m.authorize(
                actor(1),
                UploadRequest {
                    uploader: invalid,
                    ..request(2)
                },
                &[abc()],
                &[],
                10
            ),
            Err(E::InvalidPrincipal)
        );
    }
}

#[test]
fn retries_keep_the_original_request_and_cannot_renew_or_resurrect_it() {
    let mut m = model();
    let req = request(u128::MAX);
    admit(&mut m, req);
    for changed in [
        UploadRequest {
            uploader: actor(3),
            ..req
        },
        UploadRequest {
            expires_at: 101,
            ..req
        },
        UploadRequest {
            first_reference: id(2),
            ..req
        },
        UploadRequest { bytes: 4, ..req },
        UploadRequest {
            root: other(),
            ..req
        },
    ] {
        assert_eq!(
            m.authorize(actor(1), changed, &[changed.root], &[], 11),
            Err(E::Conflict)
        );
    }
    let before = usage(&m);
    assert_eq!(
        m.authorize(actor(1), req, &[], &[], 200).unwrap().state,
        U::Reserved
    );
    assert_eq!(usage(&m), before);
    assert_eq!(m.register(actor(2), req.root, 200), Err(E::Expired));
    m.release(actor(1), req.first_reference).unwrap();
    assert_eq!(
        m.authorize(actor(1), req, &[], &[], 200).unwrap().state,
        U::Cancelled
    );
    assert_eq!(m.register(actor(2), req.root, 11), Err(E::NotReserved));
    assert_eq!(usage(&m).reserved_bytes, 0);
    assert_eq!(
        m.inspect_reference(actor(1), req.first_reference),
        Ok(reference(abc(), R::Retired))
    );
}

#[test]
fn cancellation_and_expiry_retire_roots_without_blocking_distinct_content() {
    for expired in [false, true] {
        let mut m = model();
        admit(&mut m, request(1));
        assert_eq!(
            m.authorize(actor(1), request(2), &[abc()], &[], 10),
            Err(E::ObjectReserved)
        );
        if expired {
            m.expire(actor(1), id(1), 100).unwrap();
        } else {
            m.release(actor(1), id(1)).unwrap();
        }
        let before = usage(&m);
        assert_eq!(before.reserved_bytes, 0);
        assert_eq!(before.unresolved_bytes, 0);
        assert_eq!(
            m.inspect_object(actor(1), abc()),
            Ok(ObjectView {
                original_upload: UploadView {
                    request: request(1),
                    state: if expired { U::Expired } else { U::Cancelled }
                },
                retention: None,
                live_references: 0,
            })
        );
        // A new ID, later deadline or different uploader cannot reopen this root.
        for uploader in [actor(2), actor(3)] {
            assert_eq!(
                m.authorize(
                    actor(1),
                    UploadRequest {
                        uploader,
                        expires_at: 300,
                        ..request(2)
                    },
                    &[abc()],
                    &[],
                    101
                ),
                Err(E::RootRetired)
            );
        }
        assert_eq!(m.retain(actor(1), abc(), id(2)), Err(E::RootRetired));
        assert_eq!(m.register(actor(2), abc(), 101), Err(E::NotReserved));
        assert_eq!(m.register(actor(2), other(), 101), Err(E::UnknownObject));
        assert_eq!(
            m.inspect_reference(actor(1), id(2)),
            Err(E::UnknownReference)
        );
        assert_eq!(
            m.retain(actor(1), abc(), id(1)),
            Err(E::ReferenceUnavailable)
        );
        assert_eq!(
            m.authorize(actor(1), request(1), &[], &[], 101)
                .unwrap()
                .state,
            if expired { U::Expired } else { U::Cancelled }
        );
        m.release(actor(1), id(1)).unwrap();
        assert_eq!(
            m.inspect_reference(actor(1), id(1)),
            Ok(reference(abc(), R::Retired))
        );
        assert_eq!(usage(&m), before);
        // Refusal consumed no names; freed byte capacity can serve different content.
        let distinct = UploadRequest {
            root: other(),
            bytes: 1,
            expires_at: 300,
            ..request(2)
        };
        assert_eq!(
            m.authorize(actor(1), distinct, &[other()], &[], 102)
                .unwrap()
                .state,
            U::Reserved
        );
        m.register(actor(2), other(), 103).unwrap();
        m.release(actor(1), id(1)).unwrap();
        assert_eq!(
            m.inspect_reference(actor(1), id(2)),
            Ok(reference(other(), R::Live))
        );
        m.release(actor(1), id(2)).unwrap();
        assert_eq!(usage(&m).unresolved_bytes, 1);
        assert_eq!(usage(&m).live_bytes, 0);
    }
}

#[test]
fn successful_retain_and_authorization_replays_never_reopen_a_released_reference() {
    let mut m = model();
    admit(&mut m, request(1));
    m.register(actor(2), abc(), 11).unwrap();
    m.retain(actor(1), abc(), id(2)).unwrap();
    m.release(actor(1), id(2)).unwrap();
    m.retain(actor(1), abc(), id(2)).unwrap();
    assert_eq!(
        m.inspect_reference(actor(1), id(2)),
        Ok(reference(abc(), R::Released))
    );
    m.release(actor(1), id(1)).unwrap();
    let before = usage(&m);
    m.retain(actor(1), abc(), id(2)).unwrap();
    assert_eq!(
        m.authorize(actor(1), request(1), &[], &[], 200)
            .unwrap()
            .state,
        U::Registered
    );
    m.release(actor(1), id(2)).unwrap();
    assert_eq!(usage(&m), before);
    assert_eq!(m.retain(actor(1), other(), id(2)), Err(E::Conflict));
    assert_eq!(
        m.inspect_reference(actor(1), id(2)),
        Ok(reference(abc(), R::Released))
    );
    assert_eq!(
        m.authorize(
            actor(1),
            UploadRequest {
                first_reference: id(2),
                ..request(3)
            },
            &[abc()],
            &[],
            10
        ),
        Err(E::Conflict)
    );
}

#[test]
fn root_conflicts_and_malformed_manifests_do_not_reserve_anything() {
    let mut m = model();
    admit(&mut m, request(1));
    let before = usage(&m);
    for phase in [U::Reserved, U::Registered] {
        let false_length = UploadRequest {
            bytes: 4,
            ..request(2)
        };
        assert_eq!(
            m.authorize(actor(1), false_length, &[abc()], &[], 10),
            Err(E::Conflict)
        );
        if phase == U::Reserved {
            m.register(actor(2), abc(), 11).unwrap();
        }
    }
    assert_eq!(usage(&m).uploads, before.uploads);
    let before = usage(&m);
    assert_eq!(
        m.authorize(actor(1), request(2), &[], &[], 10),
        Err(E::Manifest(VerificationError::ChunkCountMismatch))
    );
    assert_eq!(
        m.authorize(actor(1), request(2), &[[0; 32]], &[], 10),
        Err(E::Manifest(VerificationError::RootMismatch))
    );
    assert_eq!(
        m.authorize(
            actor(1),
            UploadRequest {
                bytes: 0,
                ..request(2)
            },
            &[abc()],
            &[],
            10
        ),
        Err(E::Manifest(VerificationError::EmptyContent))
    );
    assert_eq!(
        m.authorize(
            actor(1),
            request(2),
            &[abc()],
            &[Header {
                name: "X",
                value: "a"
            }; 3],
            10
        ),
        Err(E::Manifest(VerificationError::HeaderCountLimit))
    );
    assert_eq!(m.inspect_upload(actor(1), id(2)), Err(E::UnknownUpload));
    assert_eq!(
        m.inspect_reference(actor(1), id(2)),
        Err(E::UnknownReference)
    );
    assert_eq!(usage(&m), before);
}

#[test]
fn expiry_and_capacity_preserve_inspection_and_cleanup() {
    let mut m = LocalModel::new(
        actor(1),
        Limits {
            uploads: limit(1),
            references: limit(1),
            ..limits()
        },
    )
    .unwrap();
    admit(&mut m, request(1));
    assert_eq!(m.register(actor(2), abc(), 9), Err(E::ClockReversed));
    assert_eq!(m.expire(actor(1), id(1), 99), Err(E::NotExpired));
    assert_eq!(m.register(actor(2), abc(), 100), Err(E::Expired));
    assert_eq!(usage(&m).reserved_bytes, 3);
    m.expire(actor(1), id(1), 100).unwrap();
    m.expire(actor(1), id(1), 101).unwrap();
    m.release(actor(1), id(1)).unwrap();
    assert_eq!(state(&m, 1), U::Expired);
    assert_eq!(
        m.inspect_object(actor(1), abc()),
        Ok(ObjectView {
            original_upload: UploadView {
                request: request(1),
                state: U::Expired
            },
            retention: None,
            live_references: 0,
        })
    );
    assert_eq!(
        m.inspect_reference(actor(1), id(1)),
        Ok(reference(abc(), R::Retired))
    );
    assert_eq!(
        usage(&m),
        Usage {
            uploads: 1,
            references: 1,
            ..Usage::default()
        }
    );
    assert_eq!(
        m.authorize(actor(1), request(2), &[abc()], &[], 10),
        Err(E::Capacity)
    );
    assert_eq!(m.register(actor(2), abc(), 11), Err(E::NotReserved));
    assert_eq!(
        m.authorize(actor(1), request(1), &[], &[], 200)
            .unwrap()
            .state,
        U::Expired
    );

    let mut m = LocalModel::new(
        actor(1),
        Limits {
            references: limit(2),
            ..limits()
        },
    )
    .unwrap();
    admit(&mut m, request(1));
    m.register(actor(2), abc(), 11).unwrap();
    m.retain(actor(1), abc(), id(2)).unwrap();
    assert_eq!(m.retain(actor(1), abc(), id(3)), Err(E::Capacity));
    m.release(actor(1), id(1)).unwrap();
    m.release(actor(1), id(2)).unwrap();
    assert_eq!(usage(&m).live_bytes, 0);
    assert_eq!(usage(&m).unresolved_bytes, 3);
    assert_eq!(usage(&m).references, 2);
}

#[test]
fn reservations_protect_both_byte_limits_and_final_release_cannot_free_exposure() {
    let second = UploadRequest {
        root: other(),
        bytes: 1,
        ..request(2)
    };
    for bounded in [
        Limits {
            logical_bytes: bytes(3),
            ..limits()
        },
        Limits {
            exposure_bytes: bytes(3),
            ..limits()
        },
    ] {
        let mut m = LocalModel::new(actor(1), bounded).unwrap();
        admit(&mut m, request(1));
        assert_eq!(
            m.authorize(actor(1), second, &[other()], &[], 10),
            Err(E::Capacity)
        );
        m.release(actor(1), id(1)).unwrap();
        assert_eq!(admit(&mut m, second), U::Reserved);
        m.register(actor(2), other(), 11).unwrap();
        assert_eq!(usage(&m).reserved_bytes, 0);
        assert_eq!(usage(&m).live_bytes, 1);
        assert_eq!(usage(&m).unresolved_bytes, 1);
    }
    let mut m = LocalModel::new(
        actor(1),
        Limits {
            exposure_bytes: bytes(3),
            ..limits()
        },
    )
    .unwrap();
    admit(&mut m, request(1));
    m.register(actor(2), abc(), 11).unwrap();
    m.release(actor(1), id(1)).unwrap();
    assert_eq!(usage(&m).live_bytes, 0);
    assert_eq!(
        m.authorize(actor(1), second, &[other()], &[], 10),
        Err(E::Capacity)
    );
    assert_eq!(
        m.inspect_reference(actor(1), id(1)),
        Ok(reference(abc(), R::Released))
    );
    assert_eq!(m.expire(actor(1), id(1), u64::MAX), Err(E::NotReserved));
    assert_eq!(usage(&m).unresolved_bytes, 3);
}

#[test]
fn delayed_authorization_must_be_resolved_before_cancelled_draft_cleanup_finishes() {
    for registered_before_cleanup in [false, true] {
        let mut m = model();
        let original = request(1);
        // The application has cancelled its draft, but authorization is unresolved.
        // Absence is not a tombstone: it cannot prevent that request arriving later.
        assert_eq!(
            m.release(actor(1), original.first_reference),
            Err(E::UnknownReference)
        );
        assert_eq!(usage(&m), Usage::default());
        assert_eq!(admit(&mut m, original), U::Reserved);
        if registered_before_cleanup {
            m.register(original.uploader, original.root, 11).unwrap();
        }
        // Resolve the original intent, even if its first successful reply was lost.
        let resolved = m.authorize(actor(1), original, &[], &[], 12).unwrap();
        assert_eq!(resolved.request, original);
        m.release(actor(1), original.first_reference).unwrap();
        let terminal = if registered_before_cleanup {
            R::Released
        } else {
            R::Retired
        };
        assert_eq!(
            m.inspect_reference(actor(1), original.first_reference),
            Ok(reference(original.root, terminal))
        );
        let after = usage(&m);
        assert_eq!(after.reserved_bytes, 0);
        assert_eq!(after.live_bytes, 0);
        assert_eq!(
            after.unresolved_bytes,
            if registered_before_cleanup { 3 } else { 0 }
        );
        assert_eq!((after.uploads, after.references), (1, 1));
        // A late exact authorization and repeated cleanup cannot revive the use.
        assert_eq!(
            m.authorize(actor(1), original, &[], &[], 13).unwrap().state,
            if registered_before_cleanup {
                U::Registered
            } else {
                U::Cancelled
            }
        );
        assert_eq!(
            m.register(original.uploader, original.root, 14),
            Err(E::NotReserved)
        );
        m.release(actor(1), original.first_reference).unwrap();
        assert_eq!(
            m.inspect_reference(actor(1), original.first_reference),
            Ok(reference(original.root, terminal))
        );
        assert_eq!(usage(&m), after);
    }
}

#[test]
fn missing_ids_and_unregistered_objects_cannot_change_usage_or_reference_bindings() {
    let mut m = model();
    admit(&mut m, request(1));
    let before = usage(&m);
    assert_eq!(m.inspect_object(actor(1), other()), Err(E::UnknownObject));
    assert_eq!(m.register(actor(2), other(), 11), Err(E::UnknownObject));
    assert_eq!(m.release(actor(1), id(2)), Err(E::UnknownReference));
    assert_eq!(m.expire(actor(1), id(2), 100), Err(E::UnknownUpload));
    assert_eq!(m.retain(actor(1), other(), id(2)), Err(E::UnknownObject));
    assert_eq!(m.retain(actor(1), abc(), id(2)), Err(E::ObjectReserved));
    assert_eq!(usage(&m), before);
    assert_eq!(
        m.inspect_reference(actor(1), id(2)),
        Err(E::UnknownReference)
    );
}
