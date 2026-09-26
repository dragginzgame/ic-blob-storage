//! Connected chunk sessions using the retained independent Caffeine vectors.
use super::*;

const CHUNK: usize = 1024 * 1024;

pub(super) struct Vector {
    pub upload: JourneyUpload,
    pub manifest: JourneyManifest,
    pub bytes: Vec<u8>,
}

fn hash(value: &serde_json::Value) -> [u8; 32] {
    let text = value
        .as_str()
        .expect("hash")
        .strip_prefix("sha256:")
        .expect("prefix");
    std::array::from_fn(|i| u8::from_str_radix(&text[2 * i..2 * i + 2], 16).expect("hex"))
}

pub(super) fn vector(name: &str, id: u8) -> Vector {
    let vectors: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../crates/ic-blob-storage/tests/fixtures/caffeine-hashing/vectors.json"
    ))
    .expect("independent JS vectors");
    let v = vectors["vectors"]
        .as_array()
        .expect("vectors")
        .iter()
        .find(|v| v["name"] == name)
        .expect("selected vector");
    let bytes = v["bytes"].as_u64().expect("length");
    Vector {
        upload: JourneyUpload {
            id,
            root: hash(&v["provider_root"]),
            digest: hash(&v["raw_digest"]),
            bytes,
        },
        manifest: JourneyManifest {
            chunks: v["chunk_hashes"]
                .as_array()
                .expect("chunks")
                .iter()
                .map(hash)
                .collect(),
            headers: v["headers"]
                .as_array()
                .expect("headers")
                .iter()
                .map(|h| {
                    (
                        h["name"].as_str().expect("name").to_owned(),
                        h["value"].as_str().expect("value").to_owned(),
                    )
                })
                .collect(),
        },
        bytes: (0..bytes)
            .map(|i| match v["pattern"].as_str().expect("pattern") {
                "abc" => b"abc"[usize::try_from(i).expect("index")],
                "linear_mod251" => u8::try_from((i * 31 + 7) % 251).expect("pattern byte"),
                "zeroes" => 0,
                _ => panic!("unsupported fixture pattern"),
            })
            .collect(),
    }
}

fn progress(
    next_chunk: u64,
    verified_bytes: u64,
    verification: JourneyVerification,
) -> JourneyProgress {
    JourneyProgress {
        next_chunk,
        verified_bytes,
        verification,
    }
}

#[test]
fn metadata_and_multiple_chunks_retain_exact_progress_until_separate_completion() {
    let f = Fixture::new();
    let v = vector("uneven-tree-with-metadata", 1);
    let a = v.upload;
    let amount = u128::from(a.bytes);
    assert_eq!(f.reserve_manifest(f.first, a, v.manifest.clone()), Ok(()));
    assert_eq!(
        f.progress(f.first, a).expect("tenant progress"),
        progress(0, 0, JourneyVerification::Pending)
    );
    for caller in [f.second, f.gateway, f.operator, Principal::anonymous()] {
        assert_eq!(f.progress(caller, a), Err(JourneyFailure::Denied));
        assert_eq!(
            f.append(caller, a, 0, &v.bytes[..CHUNK]),
            Err(JourneyFailure::Denied)
        );
    }
    assert_eq!(
        f.append(f.first, a, 1, &v.bytes[CHUNK..2 * CHUNK]),
        Err(JourneyFailure::OutOfOrder)
    );
    assert_eq!(f.append(f.first, a, 0, &v.bytes[..CHUNK]), Ok(()));
    let prefix = progress(1, CHUNK as u64, JourneyVerification::Pending);
    assert_eq!(f.progress(f.first, a).expect("tenant progress"), prefix);
    // Simulate a lost append reply: exact retry checks bytes without hashing twice.
    assert_eq!(f.append(f.first, a, 0, &v.bytes[..CHUNK]), Ok(()));
    assert_eq!(f.reserve_manifest(f.first, a, v.manifest.clone()), Ok(()));
    assert_eq!(f.progress(f.first, a).expect("tenant progress"), prefix);
    let mut corrupt = v.bytes[..CHUNK].to_vec();
    corrupt[10] ^= 1;
    assert_eq!(
        f.append(f.first, a, 0, &corrupt),
        Err(JourneyFailure::ContentMismatch)
    );
    assert_eq!(
        f.append(f.first, a, 1, &v.bytes[CHUNK..2 * CHUNK - 1]),
        Err(JourneyFailure::ContentMismatch)
    );
    assert_eq!(
        f.append(f.first, a, 1, &v.bytes[CHUNK..=2 * CHUNK]),
        Err(JourneyFailure::ContentMismatch)
    );
    assert_eq!(f.progress(f.first, a).expect("tenant progress"), prefix);
    assert_eq!(
        f.certificate(f.first, a.root),
        Err(RejectCode::CanisterError)
    );
    assert_eq!(f.complete(f.first, a), Err(JourneyFailure::InvalidPhase));
    assert_eq!(f.usage(f.first), Ok(usage(amount, amount, amount)));

    // Another tenant's completed metadata-bound file cannot alter this prefix.
    let other = vector("abc-text", 1);
    assert_eq!(
        f.reserve_manifest(f.second, other.upload, other.manifest),
        Ok(())
    );
    assert_eq!(f.append(f.second, other.upload, 0, &other.bytes), Ok(()));
    f.certificate(f.second, other.upload.root)
        .expect("other certificate");
    assert_eq!(f.complete(f.second, other.upload), Ok(()));
    assert_eq!(f.progress(f.first, a).expect("tenant progress"), prefix);
    for (index, bytes) in v.bytes.chunks(CHUNK).enumerate().skip(1) {
        let index = u64::try_from(index).expect("index");
        assert_eq!(f.append(f.first, a, index, bytes), Ok(()));
        if index < 5 {
            assert_eq!(
                f.progress(f.first, a).expect("tenant progress"),
                progress(
                    index + 1,
                    (index + 1) * CHUNK as u64,
                    JourneyVerification::Pending
                )
            );
            assert_eq!(
                f.certificate(f.first, a.root),
                Err(RejectCode::CanisterError)
            );
        }
    }
    let verified = progress(6, a.bytes, JourneyVerification::Verified);
    assert_eq!(f.progress(f.first, a).expect("tenant progress"), verified);
    assert_eq!(f.append(f.first, a, 5, &v.bytes[5 * CHUNK..]), Ok(()));
    assert_eq!(f.append(f.first, a, 0, &v.bytes[..CHUNK]), Ok(()));
    assert_eq!(
        f.append(f.first, a, 6, b""),
        Err(JourneyFailure::ContentMismatch)
    );
    assert_eq!(f.progress(f.first, a).expect("tenant progress"), verified);
    assert_eq!(f.complete(f.first, a), Err(JourneyFailure::InvalidPhase));
    f.certificate(f.first, a.root)
        .expect("whole content verified");
    assert_eq!(f.complete(f.first, a), Ok(()));
    assert_eq!(f.root_control(f.first, "journey_release", a.root), Ok(()));
    assert_eq!(f.usage(f.first), Ok(usage(0, amount, amount)));
    assert_eq!(f.delete(vec![root(a.root)]), Ok(()));
    assert_eq!(f.usage(f.first), Ok(usage(0, 0, amount)));
    assert_eq!(f.root_control(f.operator, "journey_settle", a.root), Ok(()));
    assert_eq!(f.usage(f.first), Ok(usage(0, 0, 0)));
    assert_eq!(f.usage(f.second), Ok(usage(3, 3, 3)));
}

#[test]
fn invalid_manifests_and_metadata_cannot_consume_or_replace_a_reservation() {
    let f = Fixture::new();
    let v = vector("abc-text", 1);
    let mut invalid = vec![
        JourneyManifest {
            chunks: vec![],
            headers: v.manifest.headers.clone(),
        },
        JourneyManifest {
            chunks: vec![[0; 32]; 7],
            headers: vec![],
        },
        JourneyManifest {
            chunks: v.manifest.chunks.clone(),
            headers: vec![("X".to_owned(), "a".to_owned()); 9],
        },
        JourneyManifest {
            chunks: v.manifest.chunks.clone(),
            headers: vec![("X".to_owned(), "a".repeat(1024))],
        },
    ];
    let mut changed = v.manifest.clone();
    changed.headers[1].1 = "application/json".to_owned();
    invalid.push(changed);
    let mut duplicate = v.manifest.clone();
    duplicate.headers.push(duplicate.headers[0].clone());
    invalid.push(duplicate);
    for manifest in &invalid {
        assert_eq!(
            f.reserve_manifest(f.first, v.upload, manifest.clone()),
            Err(JourneyFailure::InvalidInput)
        );
        assert_eq!(f.usage(f.first), Ok(usage(0, 0, 0)));
    }
    assert_eq!(
        f.reserve_manifest(f.first, v.upload, v.manifest.clone()),
        Ok(())
    );
    for manifest in invalid {
        assert_eq!(
            f.reserve_manifest(f.first, v.upload, manifest),
            Err(JourneyFailure::InvalidInput)
        );
    }
    assert_eq!(
        f.progress(f.first, v.upload).expect("tenant progress"),
        progress(0, 0, JourneyVerification::Pending)
    );
    assert_eq!(f.append(f.first, v.upload, 0, &v.bytes), Ok(()));
    let mut reordered = v.manifest;
    reordered.headers.reverse();
    assert_eq!(f.reserve_manifest(f.first, v.upload, reordered), Ok(()));
    assert_eq!(
        f.progress(f.first, v.upload).expect("tenant progress"),
        progress(1, 3, JourneyVerification::Verified)
    );
    f.certificate(f.first, v.upload.root)
        .expect("unchanged canonical metadata");
}

#[test]
fn unfinished_sessions_remain_charged_until_explicit_unexposed_cancellation() {
    let f = Fixture::new();
    let v = vector("pattern-5242897", 1);
    let other = vector("pattern-2097152", 2);
    let amount = u128::from(v.upload.bytes);
    assert_eq!(
        f.reserve_manifest(f.first, v.upload, v.manifest.clone()),
        Ok(())
    );
    assert_eq!(f.append(f.first, v.upload, 0, &v.bytes[..CHUNK]), Ok(()));
    assert_eq!(
        f.reserve_manifest(f.first, other.upload, other.manifest.clone()),
        Err(JourneyFailure::Limit)
    );
    assert_eq!(f.usage(f.first), Ok(usage(amount, amount, amount)));
    assert_eq!(f.control(f.first, "journey_cancel", v.upload), Ok(()));
    assert_eq!(f.usage(f.first), Ok(usage(0, 0, 0)));
    assert_eq!(
        f.append(f.first, v.upload, 1, &v.bytes[CHUNK..2 * CHUNK]),
        Err(JourneyFailure::InvalidPhase)
    );
    assert_eq!(f.reserve_manifest(f.first, v.upload, v.manifest), Ok(()));
    assert_eq!(
        f.certificate(f.first, v.upload.root),
        Err(RejectCode::CanisterError)
    );
    assert_eq!(
        f.reserve_manifest(f.first, other.upload, other.manifest),
        Ok(())
    );
    assert_eq!(
        f.progress(f.first, other.upload).expect("tenant progress"),
        progress(0, 0, JourneyVerification::Pending)
    );
}

#[test]
fn final_digest_rejection_retains_the_completed_leaf_progress_and_reservation() {
    let f = Fixture::new();
    let v = vector("pattern-1048577", 1);
    let wrong = JourneyUpload {
        digest: [0; 32],
        ..v.upload
    };
    assert_eq!(
        f.reserve_manifest(f.first, wrong, v.manifest.clone()),
        Ok(())
    );
    assert_eq!(f.append(f.first, wrong, 0, &v.bytes[..CHUNK]), Ok(()));
    assert_eq!(
        f.append(f.first, wrong, 1, &v.bytes[CHUNK..]),
        Err(JourneyFailure::ContentMismatch)
    );
    let rejected = progress(2, wrong.bytes, JourneyVerification::Rejected);
    assert_eq!(
        f.progress(f.first, wrong).expect("tenant progress"),
        rejected
    );
    assert_eq!(
        f.append(f.first, wrong, 1, &v.bytes[CHUNK..]),
        Err(JourneyFailure::ContentMismatch)
    );
    assert_eq!(
        f.reserve_manifest(f.first, wrong, v.manifest.clone()),
        Ok(())
    );
    assert_eq!(
        f.reserve_manifest(f.first, v.upload, v.manifest),
        Err(JourneyFailure::Conflict)
    );
    assert_eq!(
        f.progress(f.first, wrong).expect("tenant progress"),
        rejected
    );
    assert_eq!(
        f.certificate(f.first, wrong.root),
        Err(RejectCode::CanisterError)
    );
    let amount = u128::from(wrong.bytes);
    assert_eq!(f.usage(f.first), Ok(usage(amount, amount, amount)));
}
