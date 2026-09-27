use super::*;
use std::num::{NonZeroU64, NonZeroUsize};

// Independent abc/no-metadata vector from the pinned published client.
const ROOT: &str = "sha256:b5b435d47a4cce7dfec493b1e020c5308d9c7fe90add1aff510f9c2a9c4ea8e7";
const RAW: &str = "sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

fn limits() -> CaffeineHashLimits {
    CaffeineHashLimits {
        max_content_bytes: NonZeroU64::new(1024).unwrap(),
        max_append_bytes: NonZeroUsize::new(2).unwrap(),
        max_headers: NonZeroUsize::new(4).unwrap(),
        max_header_bytes: NonZeroUsize::new(128).unwrap(),
    }
}

fn verifier() -> CaffeineRootVerifier {
    CaffeineRootVerifier::new(ROOT.parse().unwrap(), 3, &[], limits()).unwrap()
}

#[test]
fn root_only_verification_computes_the_raw_digest_without_trusting_one() {
    let mut v = verifier();
    assert_eq!(v.expected_root(), ROOT.parse().unwrap());
    v.append(0, b"ab").unwrap();
    v.append(2, b"c").unwrap();
    assert_eq!(v.remaining_bytes(), 0);
    assert_eq!(
        v.finish(),
        Ok(CaffeineContentHashes {
            provider_root: ROOT.parse().unwrap(),
            content_digest: RAW.parse().unwrap(),
        })
    );
}

#[test]
fn accepted_length_is_not_integrity_and_corrupt_streams_need_a_fresh_instance() {
    let mut v = verifier();
    v.append(0, b"ab").unwrap();
    v.append(2, b"d").unwrap();
    assert_eq!(v.remaining_bytes(), 0);
    assert_eq!(v.finish(), Err(CaffeineHashError::ProviderRootMismatch));
    let mut retry = verifier();
    assert_eq!(retry.received_bytes(), 0);
    retry.append(0, b"a").unwrap();
    retry.append(1, b"bc").unwrap();
    assert!(retry.finish().is_ok());
}

#[test]
fn bound_metadata_cannot_be_replaced_by_body_consistency() {
    let mut v = CaffeineRootVerifier::new(
        ROOT.parse().unwrap(),
        3,
        &[CaffeineHeader {
            name: "Content-Type",
            value: "text/plain",
        }],
        limits(),
    )
    .unwrap();
    v.append(0, b"ab").unwrap();
    v.append(2, b"c").unwrap();
    assert_eq!(v.finish(), Err(CaffeineHashError::ProviderRootMismatch));
}

#[test]
fn truncation_and_trailing_bytes_cannot_be_counted_as_verified() {
    let mut v = verifier();
    v.append(0, b"ab").unwrap();
    assert_eq!(
        v.finish(),
        Err(CaffeineHashError::Incomplete { remaining: 1 })
    );
    let mut v = verifier();
    v.append(0, b"ab").unwrap();
    assert_eq!(v.append(2, b"cd"), Err(CaffeineHashError::ExceedsLength));
    assert_eq!(v.received_bytes(), 2);
    v.append(2, b"c").unwrap();
    assert_eq!(v.append(3, b"d"), Err(CaffeineHashError::ExceedsLength));
    // The rejected frame changes no hash state, but a transport host must fail
    // the whole response rather than suppressing evidence of trailing bytes.
    assert_eq!(v.received_bytes(), 3);
}

#[test]
fn a_raw_digest_in_the_root_field_does_not_establish_provider_integrity() {
    let mut v = CaffeineRootVerifier::new(RAW.parse().unwrap(), 3, &[], limits()).unwrap();
    v.append(0, b"ab").unwrap();
    v.append(2, b"c").unwrap();
    assert_eq!(v.finish(), Err(CaffeineHashError::ProviderRootMismatch));
}
