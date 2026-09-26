use super::super::tests::abc;
use super::*;

fn limits() -> MissingChunkPageLimits {
    MissingChunkPageLimits {
        max_scan: NonZeroUsize::new(1).expect("bound"),
        max_results: NonZeroUsize::new(1).expect("bound"),
    }
}

#[test]
fn ranges_and_end_positions_do_not_grant_verification_or_change_coverage() {
    let mut verifier = CaffeineChunkVerifier::new(abc());
    let range = CaffeineChunkRange {
        index: 0,
        offset: 0,
        bytes: 3,
    };
    let before = verifier.progress();
    assert_eq!(verifier.manifest().chunk_range(0), Ok(range));
    let page = verifier.missing_chunks(0, limits()).expect("first page");
    assert_eq!(
        page,
        MissingChunkPage {
            chunks: vec![range],
            next_index: None,
            scanned: 1
        }
    );
    assert_eq!(verifier.missing_chunks(0, limits()), Ok(page));
    assert_eq!(verifier.progress(), before);
    // Exhausting the scan is not successful verification.
    assert!(!verifier.progress().all_chunks_verified());
    assert_eq!(
        verifier.missing_chunks(1, limits()),
        Ok(MissingChunkPage {
            chunks: vec![],
            next_index: None,
            scanned: 0,
        })
    );
    for index in [2, u64::MAX] {
        let error = CaffeineManifestError::ChunkIndexOutOfRange { index };
        assert_eq!(verifier.missing_chunks(index, limits()), Err(error));
        assert_eq!(verifier.manifest().chunk_range(index), Err(error));
        assert_eq!(verifier.progress(), before);
    }
    verifier.verify_chunk(0, b"abc").expect("valid bytes");
    assert_eq!(
        verifier.missing_chunks(0, limits()),
        Ok(MissingChunkPage {
            chunks: vec![],
            next_index: None,
            scanned: 1,
        })
    );
}
