use std::num::{NonZeroU64, NonZeroU128, NonZeroUsize};

use ic_platform_types::Principal;

use super::{
    Error as E, Limits, LocalModel, ReferenceStatus as R, Root, UploadRequest, UploadState as U,
    Usage,
};
use crate::caffeine::{Header, ManifestLimits, VerificationError};
use crate::references::ReferenceError;

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

#[test]
fn overlapping_upload_ids_share_one_object_and_never_gain_second_registration() {
    let mut m = model();
    assert_eq!(admit(&mut m, request(1)), U::Reserved);
    assert_eq!(
        usage(&m),
        Usage {
            reserved_bytes: 3,
            uploads: 1,
            references: 1,
            ..Usage::default()
        }
    );
    m.register(actor(2), id(1), 11).unwrap();
    let second = UploadRequest {
        uploader: actor(3),
        ..request(2)
    };
    assert_eq!(admit(&mut m, second), U::Attached);
    assert_eq!(m.register(actor(3), id(2), 12), Err(E::NotReserved));
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
    assert_eq!(m.inspect_reference(actor(1), id(2)), Ok(R::Live));
    assert_eq!(usage(&m).live_bytes, 3);
    m.release(actor(1), id(2)).unwrap();
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
    assert_eq!(m.register(actor(2), id(1), 12), Err(E::NotReserved));
    assert_eq!(usage(&m), before);
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
        assert_eq!(m.register(caller, id(1), 11), Err(E::Denied));
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
    assert_eq!(m.register(actor(2), req.id, 200), Err(E::Expired));
    m.release(actor(1), req.first_reference).unwrap();
    assert_eq!(
        m.authorize(actor(1), req, &[], &[], 200).unwrap().state,
        U::Cancelled
    );
    assert_eq!(m.register(actor(2), req.id, 11), Err(E::NotReserved));
    assert_eq!(usage(&m).reserved_bytes, 0);
    assert_eq!(
        m.inspect_reference(actor(1), req.first_reference),
        Ok(R::Retired)
    );
}

#[test]
fn first_reference_release_serializes_with_registration_and_old_releases_are_harmless() {
    let mut m = model();
    admit(&mut m, request(1));
    assert_eq!(
        m.authorize(actor(1), request(2), &[abc()], &[], 10),
        Err(E::ObjectReserved)
    );
    assert_eq!(
        m.retain(actor(1), abc(), id(1)),
        Err(E::ReferenceUnavailable)
    );
    m.release(actor(1), id(1)).unwrap();
    assert_eq!(m.register(actor(2), id(1), 11), Err(E::NotReserved));
    // A provably unexposed cancellation frees this root, but not its old IDs.
    admit(&mut m, request(2));
    let before = usage(&m);
    m.release(actor(1), id(1)).unwrap();
    assert_eq!(usage(&m), before);
    assert_eq!(state(&m, 2), U::Reserved);
    assert_eq!(
        m.retain(actor(1), abc(), id(1)),
        Err(E::ReferenceUnavailable)
    );
    m.register(actor(2), id(2), 11).unwrap();
    m.release(actor(1), id(1)).unwrap();
    assert_eq!(m.inspect_reference(actor(1), id(2)), Ok(R::Live));
    m.release(actor(1), id(2)).unwrap();
    assert_eq!(usage(&m).unresolved_bytes, 3);
    assert_eq!(usage(&m).live_bytes, 0);
}

#[test]
fn successful_retain_and_authorization_replays_never_reopen_a_released_reference() {
    let mut m = model();
    admit(&mut m, request(1));
    m.register(actor(2), id(1), 11).unwrap();
    m.retain(actor(1), abc(), id(2)).unwrap();
    m.release(actor(1), id(2)).unwrap();
    m.retain(actor(1), abc(), id(2)).unwrap();
    assert_eq!(m.inspect_reference(actor(1), id(2)), Ok(R::Released));
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
            m.register(actor(2), id(1), 11).unwrap();
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
    assert_eq!(m.register(actor(2), id(1), 9), Err(E::ClockReversed));
    assert_eq!(m.expire(actor(1), id(1), 99), Err(E::NotExpired));
    assert_eq!(m.register(actor(2), id(1), 100), Err(E::Expired));
    assert_eq!(usage(&m).reserved_bytes, 3);
    m.expire(actor(1), id(1), 100).unwrap();
    m.expire(actor(1), id(1), 101).unwrap();
    m.release(actor(1), id(1)).unwrap();
    assert_eq!(state(&m, 1), U::Expired);
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
    assert_eq!(m.register(actor(2), id(1), 11), Err(E::NotReserved));
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
    m.register(actor(2), id(1), 11).unwrap();
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
        m.register(actor(2), id(2), 11).unwrap();
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
    m.register(actor(2), id(1), 11).unwrap();
    m.release(actor(1), id(1)).unwrap();
    assert_eq!(usage(&m).live_bytes, 0);
    assert_eq!(
        m.authorize(actor(1), second, &[other()], &[], 10),
        Err(E::Capacity)
    );
    assert_eq!(m.inspect_reference(actor(1), id(1)), Ok(R::Released));
    assert_eq!(m.expire(actor(1), id(1), u64::MAX), Err(E::NotReserved));
    assert_eq!(usage(&m).unresolved_bytes, 3);
}

#[test]
fn missing_ids_and_unregistered_objects_cannot_change_usage_or_reference_bindings() {
    let mut m = model();
    admit(&mut m, request(1));
    let before = usage(&m);
    assert_eq!(m.register(actor(2), id(2), 11), Err(E::UnknownUpload));
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
