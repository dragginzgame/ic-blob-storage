use super::*;

#[test]
fn raw_content_matches_sha256_vectors() {
    for (content, expected) in [
        (
            b"".as_slice(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        ),
        (
            b"abc".as_slice(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        ),
        (
            b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq".as_slice(),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
        ),
    ] {
        let digest = ContentDigest::compute(content);
        assert_eq!(digest.to_string(), format!("sha256:{expected}"));
        assert_eq!(digest.to_string().parse(), Ok(digest));
        assert_eq!(
            ContentDigest::try_from(digest.as_bytes().as_slice()),
            Ok(digest)
        );
    }
    assert_ne!(
        ContentDigest::compute(b"abc"),
        ContentDigest::compute(b"abd")
    );
}

#[test]
fn provider_root_roundtrips_every_byte_and_normalizes_hex() {
    for byte in u8::MIN..=u8::MAX {
        let bytes = [byte; DIGEST_BYTES];
        let root = ProviderRootHash::try_from(bytes.as_slice()).expect("32 bytes");
        let upper = format!("sha256:{}", format!("{byte:02X}").repeat(DIGEST_BYTES));
        assert_eq!(upper.parse(), Ok(root));
        assert_eq!(
            root.to_string(),
            format!("sha256:{}", format!("{byte:02x}").repeat(DIGEST_BYTES))
        );
        assert_eq!(root.as_bytes(), &bytes);
        assert_eq!(root.to_string().parse(), Ok(root));
    }
}

#[test]
fn rejects_malformed_text_and_binary_with_typed_errors() {
    let cases = [
        (String::new(), HashParseError::Empty),
        (
            format!("SHA256:{}", "0".repeat(64)),
            HashParseError::InvalidPrefix,
        ),
        ("0".repeat(64), HashParseError::InvalidPrefix),
        (
            "sha256:00".into(),
            HashParseError::InvalidTextLength { actual: 9 },
        ),
        (
            format!("sha256:{}0", "0".repeat(64)),
            HashParseError::InvalidTextLength { actual: 72 },
        ),
        (
            format!("sha256:{}g", "0".repeat(63)),
            HashParseError::InvalidHex { index: 70 },
        ),
        (
            format!("sha256:{}\n", "0".repeat(63)),
            HashParseError::InvalidHex { index: 70 },
        ),
        (
            format!("sha256: {}", "0".repeat(63)),
            HashParseError::InvalidHex { index: 7 },
        ),
        (
            format!("sha256:é{}", "0".repeat(62)),
            HashParseError::InvalidHex { index: 7 },
        ),
    ];
    for (input, error) in cases {
        assert_eq!(input.parse::<ProviderRootHash>(), Err(error));
        assert_eq!(input.parse::<ContentDigest>(), Err(error));
    }
    for len in [0, 1, 31, 33, 64] {
        assert_eq!(
            ContentDigest::try_from(vec![0; len].as_slice()),
            Err(HashParseError::InvalidByteLength { actual: len })
        );
        assert_eq!(
            ProviderRootHash::try_from(vec![0; len].as_slice()),
            Err(HashParseError::InvalidByteLength { actual: len })
        );
    }
}

#[test]
fn pinned_caffeine_root_is_distinct_from_raw_content_digest() {
    // docs/evidence/caffeine-client-observations.json: upstream client 1.1.2,
    // abc without metadata. Parsing this vector does not implement its tree.
    let root: ProviderRootHash =
        "sha256:b5b435d47a4cce7dfec493b1e020c5308d9c7fe90add1aff510f9c2a9c4ea8e7"
            .parse()
            .expect("upstream vector");
    assert_ne!(root.as_bytes(), ContentDigest::compute(b"abc").as_bytes());
    assert_eq!(
        ProviderRootHash::try_from(root.as_bytes().as_slice()),
        Ok(root)
    );
}
