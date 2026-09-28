use super::*;
#[test]
fn direct_download_target_encodes_each_identity_without_query_injection() {
    let owner = candid::Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    let scope =
        CaffeineDownloadScope::new(owner, std::num::NonZeroU128::MIN, "fixture project/β?&=")
            .unwrap();
    let root = ProviderRootHash::try_from([0xab; 32].as_slice()).unwrap();
    // Same fields/escaping as the reviewed official getDirectURL for this vector.
    assert_eq!(
        request_target(&scope, root),
        concat!(
            "/v1/blob/?blob_hash=sha256%3A",
            "abababababababababababababababababababababababababababababababab",
            "&owner_id=ryjl3-tyaaa-aaaaa-aaaba-cai&project_id=fixture%20project%2F%CE%B2%3F%26%3D"
        )
    );
    let injected =
        CaffeineDownloadScope::new(owner, std::num::NonZeroU128::MIN, "a#b&owner_id=x%2F").unwrap();
    assert!(request_target(&injected, root).ends_with("&project_id=a%23b%26owner_id%3Dx%252F"));
}
