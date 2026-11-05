//! Negative recovery evidence. Simulated exposure never contacts a provider.
use super::*;
use crate::snapshots;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn bare_fixture_snapshot_rollback_cannot_supply_installation_continuity() {
    let f = Fixture::new().await;
    let pic = &f.harness.pic;
    f.enroll(None, true).await.unwrap();
    let (permission, declaration) = f.permission(1, 1);
    f.admit(f.tenant, permission).await.unwrap();
    f.prepare(&declaration).await.unwrap();
    let before = snapshots::take(pic, f.service, f.controller).await;
    let yes = input(&f, permission);
    let first = expose(&f, f.uploader, yes).await.unwrap();
    assert_eq!(first.state, UploadState::Registered);
    assert_eq!(
        inspect(&f, f.uploader, yes).await.unwrap().state,
        UploadState::Registered
    );
    assert_eq!(expose(&f, f.uploader, yes).await, Err(E::Phase));
    f.revoke(permission).await.unwrap();
    assert!(inspect(&f, f.uploader, yes).await.unwrap().revoked);
    snapshots::load(pic, f.service, f.controller, &before).await;
    let rolled_back = inspect(&f, f.uploader, yes).await.unwrap();
    assert!(!rolled_back.revoked);
    assert_eq!(rolled_back.state, UploadState::Reserved);
    assert!(!f.local_status().await.uploads.fenced);

    // This bare fixture intentionally bypasses installation continuity observations.
    // The old journal alone cannot reveal the first exposure. Production integrations
    // must use the installed owner, whose actual snapshot fence is tested separately.
    assert_eq!(expose(&f, f.uploader, yes).await.unwrap(), first);
    assert_eq!(expose(&f, f.uploader, yes).await, Err(E::Phase));
    snapshots::delete(pic, f.service, f.controller, before).await;
}
