use super::*;
fn scope() -> TenantScope {
    TenantScope {
        service: Principal::from_slice(&[1, 1]),
        tenant: Principal::from_slice(&[2, 1]),
        namespace: u128::MAX,
    }
}
fn bytes(enrollment: Option<TenantEnrollment>, fenced: bool) -> Vec<u8> {
    candid::encode_one(Ok::<_, TenantFailure>(TenantEnrollmentResponse {
        scope: scope(),
        enrollment,
        fenced,
    }))
    .unwrap()
}
fn max() -> NonZeroUsize {
    4096.try_into().unwrap()
}

#[test]
fn acknowledgments_reuse_the_exact_model_transition_including_generation_exhaustion() {
    for (before, active, generation) in [
        (None, true, 1),
        (None, false, 1),
        (
            Some(TenantEnrollment {
                generation: 8,
                active: true,
            }),
            false,
            8,
        ),
        (
            Some(TenantEnrollment {
                generation: 8,
                active: false,
            }),
            true,
            9,
        ),
        (
            Some(TenantEnrollment {
                generation: u64::MAX,
                active: true,
            }),
            false,
            u64::MAX,
        ),
        (
            Some(TenantEnrollment {
                generation: u64::MAX,
                active: false,
            }),
            false,
            u64::MAX,
        ),
    ] {
        let input = TenantUpdateRequest {
            scope: scope(),
            expected: before,
            active,
        };
        let value = TenantEnrollment { generation, active };
        assert_eq!(
            mutation(input, &bytes(Some(value), false), max())
                .unwrap()
                .enrollment,
            Some(value)
        );
        for bad in [
            bytes(None, false),
            bytes(Some(value), true),
            bytes(
                Some(TenantEnrollment {
                    active: !active,
                    ..value
                }),
                false,
            ),
        ] {
            assert_eq!(mutation(input, &bad, max()), Err(TenantReplyError::Invalid));
        }
    }
    let input = TenantUpdateRequest {
        scope: scope(),
        expected: Some(TenantEnrollment {
            generation: u64::MAX,
            active: false,
        }),
        active: true,
    };
    assert_eq!(validate_update(input), Err(TenantReplyError::Invalid));
    assert_eq!(
        validate_update(TenantUpdateRequest {
            expected: Some(TenantEnrollment {
                generation: 0,
                active: false
            }),
            ..input
        }),
        Err(TenantReplyError::Invalid)
    );
    let input = TenantUpdateRequest {
        expected: None,
        ..input
    };
    assert_eq!(
        mutation(
            input,
            &bytes(
                Some(TenantEnrollment {
                    generation: 2,
                    active: true
                }),
                false
            ),
            max()
        ),
        Err(TenantReplyError::Invalid)
    );
}

#[test]
fn observation_preserves_fence_and_absence_but_rejects_foreign_scope_and_zero_generation() {
    for enrollment in [
        None,
        Some(TenantEnrollment {
            generation: u64::MAX,
            active: false,
        }),
    ] {
        for fenced in [true, false] {
            let view = inspection(scope(), &bytes(enrollment, fenced), max()).unwrap();
            assert_eq!(
                view,
                TenantEnrollmentResponse {
                    scope: scope(),
                    enrollment,
                    fenced
                }
            );
        }
    }
    for foreign in [
        TenantScope {
            namespace: 1,
            ..scope()
        },
        TenantScope {
            service: scope().tenant,
            ..scope()
        },
        TenantScope {
            tenant: scope().service,
            ..scope()
        },
    ] {
        assert_eq!(
            inspection(foreign, &bytes(None, false), max()),
            Err(TenantReplyError::Binding)
        );
    }
    assert_eq!(
        inspection(
            scope(),
            &bytes(
                Some(TenantEnrollment {
                    generation: 0,
                    active: true
                }),
                false
            ),
            max()
        ),
        Err(TenantReplyError::Invalid)
    );
}

#[test]
fn byte_bounds_malformed_encoding_and_remote_refusals_remain_distinct() {
    let good = bytes(None, false);
    assert!(inspection(scope(), &good, good.len().try_into().unwrap()).is_ok());
    assert_eq!(
        inspection(scope(), &good, (good.len() - 1).try_into().unwrap()),
        Err(TenantReplyError::Limit)
    );
    for malformed in [
        Vec::new(),
        b"DIDL".to_vec(),
        candid::encode_one(1u8).unwrap(),
    ] {
        assert_eq!(
            inspection(scope(), &malformed, max()),
            Err(TenantReplyError::Invalid)
        );
    }
    let input = TenantUpdateRequest {
        scope: scope(),
        expected: None,
        active: true,
    };
    let refused =
        candid::encode_one(Err::<TenantEnrollmentResponse, _>(TenantFailure::Conflict)).unwrap();
    assert_eq!(
        mutation(input, &refused, max()),
        Err(TenantReplyError::Remote(TenantFailure::Conflict))
    );
}
