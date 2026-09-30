//! Negative recovery evidence. Simulated exposure never contacts a provider.
use super::*;
use crate::snapshots;

#[test]
fn snapshot_rollback_forgets_exposure_and_simulated_freshness_allows_reexposure() {
    let f = Fixture::new();
    let pic = &f.harness.pic;
    f.enroll(None, true).unwrap();
    let (permission, declaration) = f.permission(1, 1);
    f.admit(f.tenant, permission).unwrap();
    f.prepare(&declaration).unwrap();
    let before = snapshots::take(pic, f.service, f.controller);
    let yes = input(&f, permission);
    let first = expose(&f, f.uploader, yes).unwrap();
    assert!(matches!(first, ExposureOutcome::Exposed(_)));
    assert_eq!(
        inspect(&f, f.uploader, yes).unwrap().state,
        UploadState::ExposurePossible
    );
    assert_eq!(expose(&f, f.uploader, yes), Err(E::Phase));
    f.revoke(permission).unwrap();
    assert!(inspect(&f, f.uploader, yes).unwrap().revoked);
    snapshots::load(pic, f.service, f.controller, &before);
    let rolled_back = inspect(&f, f.uploader, yes).unwrap();
    assert!(!rolled_back.revoked);
    assert_eq!(rolled_back.state, UploadState::Reserved);
    assert!(!f.local_status().uploads.fenced);

    // The simulated host claims Recovery=true again. The shared owner cannot
    // recover the first exposure from this backup. This is why production must
    // never derive that fact from a clear fence or locally valid journal.
    assert_eq!(expose(&f, f.uploader, yes).unwrap(), first);
    assert_eq!(expose(&f, f.uploader, yes), Err(E::Phase));
    snapshots::delete(pic, f.service, f.controller, before);
}
