use super::*;

fn bound(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).expect("positive bound")
}

fn limits() -> CaffeineHashLimits {
    CaffeineHashLimits {
        max_content_bytes: NonZeroU64::new(ContentVerifier::MAX_BYTES).expect("maximum"),
        max_append_bytes: bound(CAFFEINE_CHUNK_BYTES),
        max_headers: bound(4),
        max_header_bytes: bound(128),
    }
}

fn abc() -> CaffeineContentHashes {
    CaffeineContentHashes {
        content_digest: ContentDigest::compute(b"abc"),
        provider_root: "sha256:b5b435d47a4cce7dfec493b1e020c5308d9c7fe90add1aff510f9c2a9c4ea8e7"
            .parse()
            .expect("independent client vector"),
    }
}

#[test]
fn rejects_input_before_allocation_and_keeps_empty_provider_behavior_unqualified() {
    assert_eq!(
        CaffeineContentHasher::new(0, &[], limits()).err(),
        Some(CaffeineHashError::EmptyContentUnqualified)
    );
    for declared in [ContentVerifier::MAX_BYTES + 1, u64::MAX] {
        assert_eq!(
            CaffeineContentHasher::new(declared, &[], limits()).err(),
            Some(CaffeineHashError::Limit(CaffeineHashLimit::ContentBytes))
        );
    }
    let maximum = CaffeineContentHasher::new(ContentVerifier::MAX_BYTES, &[], limits())
        .expect("constant memory at algorithm maximum");
    assert_eq!(maximum.remaining_bytes(), ContentVerifier::MAX_BYTES);
    assert_eq!(
        maximum.finish(),
        Err(CaffeineHashError::Incomplete {
            remaining: ContentVerifier::MAX_BYTES
        })
    );
    assert_eq!(
        CaffeineContentHasher::new(
            4,
            &[],
            CaffeineHashLimits {
                max_content_bytes: NonZeroU64::new(3).expect("bound"),
                ..limits()
            }
        )
        .err(),
        Some(CaffeineHashError::Limit(CaffeineHashLimit::ContentBytes))
    );
    let header = CaffeineHeader {
        name: "X",
        value: "a",
    };
    assert_eq!(
        CaffeineContentHasher::new(1, &[header; 5], limits()).err(),
        Some(CaffeineHashError::Limit(CaffeineHashLimit::HeaderCount))
    );
    assert_eq!(
        CaffeineContentHasher::new(1, &[header; 2], limits()).err(),
        Some(CaffeineHashError::DuplicateHeader)
    );
    assert_eq!(
        CaffeineContentHasher::new(
            1,
            &[header],
            CaffeineHashLimits {
                max_header_bytes: bound(4),
                ..limits()
            }
        )
        .err(),
        Some(CaffeineHashError::Limit(CaffeineHashLimit::HeaderBytes))
    );
    assert!(
        CaffeineContentHasher::new(
            1,
            &[header],
            CaffeineHashLimits {
                max_header_bytes: bound(5),
                max_headers: bound(1),
                ..limits()
            }
        )
        .is_ok()
    );
    // Trimming must not make oversized input free to process.
    assert_eq!(
        CaffeineContentHasher::new(
            1,
            &[CaffeineHeader {
                name: "    X    ",
                value: " a "
            }],
            CaffeineHashLimits {
                max_header_bytes: bound(5),
                ..limits()
            }
        )
        .err(),
        Some(CaffeineHashError::Limit(CaffeineHashLimit::HeaderBytes))
    );
}

#[test]
fn rejected_appends_preserve_digest_and_offset_and_allow_correct_continuation() {
    let mut state = CaffeineContentHasher::new(
        3,
        &[],
        CaffeineHashLimits {
            max_append_bytes: bound(2),
            ..limits()
        },
    )
    .expect("hash state");
    state.append(0, b"a").expect("first byte");
    for (offset, input, error) in [
        (
            0,
            b"a".as_slice(),
            CaffeineHashError::UnexpectedOffset {
                expected: 1,
                actual: 0,
            },
        ),
        (
            2,
            b"b",
            CaffeineHashError::UnexpectedOffset {
                expected: 1,
                actual: 2,
            },
        ),
        (
            1,
            b"bcd",
            CaffeineHashError::Limit(CaffeineHashLimit::AppendBytes),
        ),
    ] {
        assert_eq!(state.append(offset, input), Err(error));
        assert_eq!(state.received_bytes(), 1);
        assert_eq!(state.remaining_bytes(), 2);
    }
    state.append(1, b"").expect("empty append");
    state.append(1, b"b").expect("next byte");
    assert_eq!(
        state.append(2, b"cd"),
        Err(CaffeineHashError::ExceedsLength)
    );
    state.append(2, b"c").expect("finish valid bytes");
    assert_eq!(state.append(3, b"x"), Err(CaffeineHashError::ExceedsLength));
    assert_eq!(state.verify(abc()), Ok(abc()));
}

#[test]
fn completion_rejects_truncation_corruption_and_metadata_changes_separately() {
    let mut incomplete = CaffeineContentHasher::new(3, &[], limits()).expect("state");
    incomplete.append(0, b"ab").expect("prefix");
    assert_eq!(
        incomplete.verify(abc()),
        Err(CaffeineHashError::Incomplete { remaining: 1 })
    );
    for bytes in [b"abd", b"bac", b"abb"] {
        let mut corrupt = CaffeineContentHasher::new(3, &[], limits()).expect("state");
        corrupt.append(0, bytes).expect("full length");
        assert_eq!(
            corrupt.verify(abc()),
            Err(CaffeineHashError::ContentDigestMismatch)
        );
    }
    let mut metadata = CaffeineContentHasher::new(
        3,
        &[CaffeineHeader {
            name: "Content-Type",
            value: "text/plain",
        }],
        limits(),
    )
    .expect("state");
    metadata.append(0, b"abc").expect("correct bytes");
    assert_eq!(
        metadata.verify(abc()),
        Err(CaffeineHashError::ProviderRootMismatch)
    );
}
