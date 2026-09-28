use super::*;
#[test]
fn download_scope_requires_explicit_bounded_project_and_owner() {
    let owner = Principal::from_slice(&[1, 1]);
    for invalid in [Principal::anonymous(), Principal::management_canister()] {
        assert_eq!(
            CaffeineDownloadScope::new(invalid, NonZeroU128::MIN, "project"),
            Err(DownloadScopeError::Owner)
        );
    }
    for invalid in [
        "",
        " project",
        "project\n",
        "a\0b",
        "a\nb",
        &"é".repeat(129),
    ] {
        assert_eq!(
            CaffeineDownloadScope::new(owner, NonZeroU128::MIN, invalid),
            Err(DownloadScopeError::Project)
        );
    }
    let scope = CaffeineDownloadScope::new(owner, NonZeroU128::MAX, &"é".repeat(128)).unwrap();
    assert_eq!(scope.project().len(), 256);
    assert_eq!(scope.namespace(), NonZeroU128::MAX);
    assert_eq!(scope.owner(), owner);
}
