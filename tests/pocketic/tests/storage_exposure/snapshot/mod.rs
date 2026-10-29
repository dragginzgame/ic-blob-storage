//! Negative recovery evidence. Simulated exposure never contacts a provider.
use super::*;
use crate::snapshots;

#[test]
fn bare_fixture_snapshot_rollback_cannot_supply_installation_continuity() {
    let f = Fixture::new();
    let pic = &f.harness.pic;
    f.enroll(None, true).unwrap();
    let (permission, declaration) = f.permission(1, 1);
    f.admit(f.tenant, permission).unwrap();
    f.prepare(&declaration).unwrap();
    let before = snapshots::take(pic, f.service, f.controller);
    let yes = input(&f, permission);
    let first = expose(&f, f.uploader, yes).unwrap();
    assert_eq!(first.state, UploadState::Registered);
    assert_eq!(
        inspect(&f, f.uploader, yes).unwrap().state,
        UploadState::Registered
    );
    assert_eq!(expose(&f, f.uploader, yes), Err(E::Phase));
    f.revoke(permission).unwrap();
    assert!(inspect(&f, f.uploader, yes).unwrap().revoked);
    snapshots::load(pic, f.service, f.controller, &before);
    let rolled_back = inspect(&f, f.uploader, yes).unwrap();
    assert!(!rolled_back.revoked);
    assert_eq!(rolled_back.state, UploadState::Reserved);
    assert!(!f.local_status().uploads.fenced);

    // This bare fixture intentionally bypasses installation continuity observations.
    // The old journal alone cannot reveal the first exposure. Production integrations
    // must use the installed owner, whose actual snapshot fence is tested separately.
    assert_eq!(expose(&f, f.uploader, yes).unwrap(), first);
    assert_eq!(expose(&f, f.uploader, yes), Err(E::Phase));
    snapshots::delete(pic, f.service, f.controller, before);
}
