use super::{
    CHUNK_BYTES, Header, Limits, ManifestLimits, RootVerifier, VerificationError as E,
    check_manifest,
};
use std::num::{NonZeroU64, NonZeroUsize};

// Frozen first-party observations of the unmodified SDK 1.1.2 algorithm.
// Provenance and input recipe: docs/design/caffeine-contract-review.md.
const PATTERN_VECTORS: &[(usize, &str)] = &[
    (
        1,
        "82123b33e498318c7b9849ae6aa8fd2b2735d5a478077fce66a9d363cb838941",
    ),
    (
        1048575,
        "e3797b4fba6b7b71d5a7325f5b76d929f4fd691030ad641b5d4a05382efc8fe7",
    ),
    (
        1048576,
        "f6ecd22dc439c229d43a9abf28813e3ab5775ddea33dd7cc22e59a4a0bfed925",
    ),
    (
        1048577,
        "3cbf73d9219a8e76f060cb29b51781b75477eaea3c14b8c98e57c93220b27b0e",
    ),
    (
        2097152,
        "c1b9014b5e1e97e0d013f52f1e5f980cc91a8890c94d41f7079fa0d0e6ac7dc5",
    ),
    (
        3145728,
        "37e92b74d726771d3e9b1ae5a99cdf1be6d455a60a8d8433a31a8891b4d2d336",
    ),
    (
        5242897,
        "741203810d8d36fa96546085041011ed6b14e667af5aa6a86cf48645bbb46ac1",
    ),
    (
        7340032,
        "9eebf78adde7ff07365e635c05b7977d4e904aac543a0cebedb92aa06727c2a8",
    ),
    (
        8388608,
        "eb2f64325c9e4de5f16994f649172269d2eccac13b302781274aa07852689651",
    ),
    (
        9437215,
        "46b478349ce1f306a5f59bf9df3c64373b6a02c7f78a7c702521c2b300180b99",
    ),
    (
        15728640,
        "f2ca95a594df4f215ad80137f1d36ae26bf43c7511a1a33daef10da09b6e5c08",
    ),
    (
        16777216,
        "2adfb28b1da815649a0fbded761ebdbd7443d708914f22b84ef7ac58a5f7ab9a",
    ),
    (
        17825793,
        "b0b3698fc6708635bfdca263b3c5947df63992f2ad5df5f08f5d58a407e6fa13",
    ),
    (
        10485760,
        "73f61c0e75523270a1f2e196c7211752a391c69232c4b84290fbab9dd3666f85",
    ),
    (
        10485761,
        "13cb5a56f922791a59aca4c79564bf69237aaa3b3cd4cceb09298fc49c2edfaa",
    ),
];
const ABC: &str = "b5b435d47a4cce7dfec493b1e020c5308d9c7fe90add1aff510f9c2a9c4ea8e7";

fn root(hex: &str) -> [u8; 32] {
    assert_eq!(hex.len(), 64);

    std::array::from_fn(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap())
}

fn limits() -> Limits {
    Limits {
        max_content_bytes: NonZeroU64::new(u64::MAX).unwrap(),
        max_append_bytes: NonZeroUsize::new(CHUNK_BYTES + 19).unwrap(),
        max_headers: NonZeroUsize::new(5).unwrap(),
        max_header_bytes: NonZeroUsize::new(512).unwrap(),
    }
}

fn verifier(bytes: u64) -> RootVerifier {
    RootVerifier::new(root(ABC), bytes, &[], limits()).unwrap()
}

fn feed(v: &mut RootVerifier, bytes: &[u8], frame: usize) {
    for chunk in bytes.chunks(frame) {
        v.append(v.received_bytes(), chunk).unwrap();
    }
}

fn pattern(bytes: usize) -> Vec<u8> {
    (0..bytes).map(|i| ((i * 31 + 7) % 251) as u8).collect()
}

#[test]
fn sdk_vectors_cover_leaf_boundaries_and_uneven_trees() {
    for &(bytes, expected) in PATTERN_VECTORS {
        let content = pattern(bytes);
        // Frames cross both full and partial leaf boundaries.
        let mut v = RootVerifier::new(root(expected), bytes as u64, &[], limits()).unwrap();
        feed(&mut v, &content, 73_301);
        assert_eq!(v.finish(), Ok(()), "{bytes} bytes");
    }
    let mut v = verifier(3);
    feed(&mut v, b"abc", 1);
    assert_eq!(v.finish(), Ok(()));
}

#[test]
fn repeated_chunks_keep_their_positions() {
    let content = vec![0; 3 * CHUNK_BYTES + 7];
    let expected = root("35c283bc6bcd38f9ae0ea84d9e08f94d2b3b2cf1152e008cd1288d355abd379e");
    let mut v = RootVerifier::new(expected, content.len() as u64, &[], limits()).unwrap();
    feed(&mut v, &content, CHUNK_BYTES + 19);
    assert_eq!(v.finish(), Ok(()));
}

#[test]
fn multichunk_metadata_verification_rejects_corruption_and_wrong_order() {
    let content = pattern(5 * CHUNK_BYTES + 17);
    let headers = [
        Header {
            name: "Content-Length",
            value: "5242897",
        },
        Header {
            name: "Content-Type",
            value: "application/octet-stream",
        },
    ];
    let expected = root("bc608ea2567f6809b872afc8e9ec44fc0addf3fd58a00e7f7aa7757b35927a03");
    let create = || RootVerifier::new(expected, content.len() as u64, &headers, limits()).unwrap();
    let mut v = create();
    feed(&mut v, &content, CHUNK_BYTES);
    assert_eq!(v.finish(), Ok(()));

    let mut corrupt = content.clone();
    corrupt[CHUNK_BYTES + 1] ^= 1;
    let mut v = create();
    feed(&mut v, &corrupt, 73_301);
    assert_eq!(v.finish(), Err(E::RootMismatch));

    let mut reordered = content.clone();
    let (first, rest) = reordered.split_at_mut(CHUNK_BYTES);
    first.swap_with_slice(&mut rest[..CHUNK_BYTES]);
    let mut v = create();
    feed(&mut v, &reordered, 73_301);
    assert_eq!(v.finish(), Err(E::RootMismatch));

    let mut v = create();
    feed(&mut v, &content[..content.len() - 1], 73_301);
    assert_eq!(v.finish(), Err(E::Incomplete));
}

#[test]
fn original_metadata_and_tree_root_cannot_be_substituted() {
    let headers = [
        Header {
            name: "Content-Type",
            value: "text/plain",
        },
        Header {
            name: "Content-Length",
            value: "3",
        },
    ];
    let expected = root("0e9afaf413b048e40834d5b0e737d80fbf304af2045c7564d96ad8aebaf74dfd");
    let mut v = RootVerifier::new(expected, 3, &headers, limits()).unwrap();
    feed(&mut v, b"abc", 2);
    assert_eq!(v.finish(), Ok(()));
    for supplied in [&[][..], &headers[..1]] {
        let mut v = RootVerifier::new(expected, 3, supplied, limits()).unwrap();
        feed(&mut v, b"abc", 2);
        assert_eq!(v.finish(), Err(E::RootMismatch));
    }
    let raw_digest = root("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    let mut v = RootVerifier::new(raw_digest, 3, &[], limits()).unwrap();
    feed(&mut v, b"abc", 2);
    assert_eq!(v.finish(), Err(E::RootMismatch));
}

#[test]
fn metadata_matches_ecmascript_trimming_and_utf16_order() {
    let headers = [
        Header {
            name: "\u{feff}X\u{a0}",
            value: "\u{2028} a \u{feff}",
        },
        Header {
            name: "\u{85}Y",
            value: "\u{85}b\u{85}",
        },
        Header {
            name: "\u{10000}",
            value: "astral",
        },
        Header {
            name: "\u{e000}",
            value: "bmp",
        },
        Header {
            name: " X ",
            value: "second trimmed name",
        },
    ];
    let expected = root("20011bf00d8a4f7e08d5786760c8b0068c24014f04fd7aa1b7d19a38f6f396d5");
    let mut v = RootVerifier::new(expected, 3, &headers, limits()).unwrap();
    feed(&mut v, b"abc", 2);
    assert_eq!(v.finish(), Ok(()));
}

#[test]
fn input_limits_refuse_before_processing() {
    assert_eq!(
        RootVerifier::new(root(ABC), 0, &[], limits()).err(),
        Some(E::EmptyContent)
    );
    let bounded = Limits {
        max_content_bytes: NonZeroU64::new(2).unwrap(),
        ..limits()
    };
    assert_eq!(
        RootVerifier::new(root(ABC), 3, &[], bounded).err(),
        Some(E::ContentLimit)
    );
    let h = Header {
        name: "X",
        value: "a",
    };
    assert_eq!(
        RootVerifier::new(root(ABC), 3, &[h; 6], limits()).err(),
        Some(E::HeaderCountLimit)
    );
    assert_eq!(
        RootVerifier::new(root(ABC), 3, &[h; 2], limits()).err(),
        Some(E::DuplicateHeader)
    );
    let bounded = Limits {
        max_header_bytes: NonZeroUsize::new(5).unwrap(),
        ..limits()
    };
    assert!(RootVerifier::new(root(ABC), 3, &[h], bounded).is_ok());
    let padded = Header {
        name: " X ",
        value: "a",
    };
    assert_eq!(
        RootVerifier::new(root(ABC), 3, &[padded], bounded).err(),
        Some(E::HeaderBytesLimit)
    );
    // Construction of a huge declared file retains no body-sized allocation.
    let v = RootVerifier::new(root(ABC), u64::MAX, &[], limits()).unwrap();
    assert_eq!(v.received_bytes(), 0);
    assert_eq!(v.finish(), Err(E::Incomplete));
}

#[test]
fn rejected_appends_preserve_the_stream_but_do_not_validate_transport() {
    let bounded = Limits {
        max_append_bytes: NonZeroUsize::new(2).unwrap(),
        ..limits()
    };
    let mut v = RootVerifier::new(root(ABC), 3, &[], bounded).unwrap();
    v.append(0, b"a").unwrap();
    for (offset, bytes, error) in [
        (0, b"a".as_slice(), E::UnexpectedOffset),
        (2, b"b".as_slice(), E::UnexpectedOffset),
        (1, b"bcd".as_slice(), E::AppendLimit),
    ] {
        assert_eq!(v.append(offset, bytes), Err(error));
        assert_eq!(v.received_bytes(), 1);
    }
    v.append(1, b"").unwrap();
    v.append(1, b"b").unwrap();
    assert_eq!(v.append(2, b"cd"), Err(E::ExceedsLength));
    v.append(2, b"c").unwrap();
    assert_eq!(v.append(3, b"d"), Err(E::ExceedsLength));
    assert_eq!(v.received_bytes(), 3);
    // This proves rejection preserved hash state. A real transport must still
    // fail a response containing trailing bytes, not suppress that error.
    assert_eq!(v.finish(), Ok(()));
}

#[test]
fn trusted_length_is_checked_independently_of_the_root() {
    let mut v = verifier(4);
    feed(&mut v, b"abc", 2);
    assert_eq!(v.finish(), Err(E::Incomplete));
    let mut v = verifier(2);
    assert_eq!(v.append(0, b"abc"), Err(E::ExceedsLength));
    let mut v = verifier(4);
    feed(&mut v, b"abcd", 2);
    assert_eq!(v.finish(), Err(E::RootMismatch));
    let mut v = verifier(3);
    feed(&mut v, b"abd", 2);
    assert_eq!(v.finish(), Err(E::RootMismatch));
}

fn manifest_limits() -> ManifestLimits {
    ManifestLimits {
        max_content_bytes: NonZeroU64::new(u64::MAX).unwrap(),
        max_chunks: NonZeroUsize::new(6).unwrap(),
        max_headers: NonZeroUsize::new(2).unwrap(),
        max_header_bytes: NonZeroUsize::new(128).unwrap(),
    }
}

#[test]
fn manifest_matches_frozen_sdk_leaves_and_rejects_substitution() {
    // Frozen leaves from the same SDK observations as the streaming vectors.
    let chunks = [
        "f6ecd22dc439c229d43a9abf28813e3ab5775ddea33dd7cc22e59a4a0bfed925",
        "bca62763243aeeae8e10de85e7161e5932e30c99c01beaa1dbdd844158c3ba87",
        "d8d64a80aa7b2ed2b37e9d4c9e268c8f3f00888fae73f8dd559c43b7853cf703",
        "e94b04a2cd33555c8152c334616a0628fceccd89910d90df2730691023d57eec",
        "55a2198e0c31902dc19f1deced6fa8bab331c6445c7dcd1dfdcbe6ff4b0cfeb8",
        "7840422043ff9041ab2fcd0bff33f6d245e077f5a6a5616aa44263d17234aeb6",
    ]
    .map(root);
    for &(bytes, expected) in PATTERN_VECTORS.iter().filter(|(bytes, _)| {
        [
            CHUNK_BYTES,
            2 * CHUNK_BYTES,
            3 * CHUNK_BYTES,
            5 * CHUNK_BYTES + 17,
        ]
        .contains(bytes)
    }) {
        let leaves = &chunks[..bytes.div_ceil(CHUNK_BYTES)];
        assert_eq!(
            check_manifest(root(expected), bytes as u64, leaves, &[], manifest_limits()),
            Ok(())
        );
    }
    let headers = [
        Header {
            name: "Content-Length",
            value: "5242897",
        },
        Header {
            name: "Content-Type",
            value: "application/octet-stream",
        },
    ];
    let expected = root("bc608ea2567f6809b872afc8e9ec44fc0addf3fd58a00e7f7aa7757b35927a03");
    let check = |leaves: &[[u8; 32]], metadata: &[Header<'_>]| {
        check_manifest(expected, 5242897, leaves, metadata, manifest_limits())
    };
    assert_eq!(check(&chunks, &headers), Ok(()));
    assert_eq!(check(&chunks[..5], &headers), Err(E::ChunkCountMismatch));
    assert_eq!(check(&chunks, &[]), Err(E::RootMismatch));
    assert_eq!(check(&chunks, &headers[..1]), Err(E::RootMismatch));
    let mut wrong = chunks;
    wrong.swap(0, 1);
    assert_eq!(check(&wrong, &headers), Err(E::RootMismatch));
    wrong = chunks;
    wrong[5][0] ^= 1;
    assert_eq!(check(&wrong, &headers), Err(E::RootMismatch));
    wrong = chunks;
    wrong[1] = wrong[0];
    assert_eq!(check(&wrong, &headers), Err(E::RootMismatch));
}

#[test]
fn manifest_limits_and_count_precede_metadata_and_tree_work() {
    let chunk = root(ABC);
    // Invalid metadata must not take precedence over length/count admission.
    let headers = [Header {
        name: "X",
        value: "a",
    }; 3];
    let check =
        |bytes, chunks: &[[u8; 32]], limits| check_manifest(chunk, bytes, chunks, &headers, limits);
    assert_eq!(check(0, &[], manifest_limits()), Err(E::EmptyContent));
    let bounded = ManifestLimits {
        max_content_bytes: NonZeroU64::new(2).unwrap(),
        ..manifest_limits()
    };
    assert_eq!(check(3, &[chunk], bounded), Err(E::ContentLimit));
    assert_eq!(check(3, &[chunk; 7], manifest_limits()), Err(E::ChunkLimit));
    for (bytes, chunks) in [(3, &[][..]), (3, &[chunk; 2][..]), (u64::MAX, &[chunk][..])] {
        assert_eq!(
            check(bytes, chunks, manifest_limits()),
            Err(E::ChunkCountMismatch)
        );
    }
    assert_eq!(
        check(3, &[chunk], manifest_limits()),
        Err(E::HeaderCountLimit)
    );
}

#[test]
fn manifest_metadata_uses_the_same_bounded_original_input_rules() {
    let chunk = root(ABC);
    let h = Header {
        name: "X",
        value: "a",
    };
    let check =
        |headers: &[Header<'_>], limits| check_manifest(chunk, 3, &[chunk], headers, limits);
    assert_eq!(check(&[h; 2], manifest_limits()), Err(E::DuplicateHeader));
    let bounded = ManifestLimits {
        max_header_bytes: NonZeroUsize::new(5).unwrap(),
        ..manifest_limits()
    };
    assert_eq!(
        check(
            &[Header {
                name: " X ",
                value: "a"
            }],
            bounded
        ),
        Err(E::HeaderBytesLimit)
    );
    // Fits exactly, but adding metadata to the original bare root is a mismatch.
    assert_eq!(check(&[h], bounded), Err(E::RootMismatch));
    assert_eq!(check(&[], bounded), Ok(()));
}

#[test]
fn a_consistent_manifest_does_not_authenticate_declared_length() {
    let chunk = root(ABC);
    // All these lengths imply one leaf. Its hash alone reveals none of them.
    for bytes in [1, 3, 4, CHUNK_BYTES as u64] {
        assert_eq!(
            check_manifest(chunk, bytes, &[chunk], &[], manifest_limits()),
            Ok(())
        );
    }
    assert_eq!(
        check_manifest(
            chunk,
            CHUNK_BYTES as u64 + 1,
            &[chunk],
            &[],
            manifest_limits()
        ),
        Err(E::ChunkCountMismatch)
    );
    let mut v = verifier(4);
    feed(&mut v, b"abc", 2);
    assert_eq!(v.finish(), Err(E::Incomplete));
    let mut v = verifier(4);
    feed(&mut v, b"abcd", 2);
    assert_eq!(v.finish(), Err(E::RootMismatch));
}
