//! Direct-upload manifest, retained accounting and lifecycle evidence for the actual shared owner.
use super::*;
use ic_testkit::pocket_ic::CanisterInstallMode;
use sha2::{Digest, Sha256};

const CHUNK: usize = 1024 * 1024;

#[tokio::test(flavor = "multi_thread")]
async fn metadata_rejection_preserves_reservation_and_reordered_retry_preserves_binding() {
    let f = Fixture::new().await;
    f.enroll().await;
    let v = vectors::vector("abc-text", 1);
    let p = f.permission(&v).await;
    f.admit(p).await;
    for bound in [false, true] {
        if bound {
            f.prepare(p, &v).await;
        }
        let original = f.observe(p).await;
        for headers in [
            vec![],
            vec![("Content-Length".into(), "4".into())],
            vec![
                ("Content-Length".into(), "3".into()),
                ("content-length".into(), "3".into()),
            ],
            vec![
                ("Content-Length".into(), "3".into()),
                ("Content-Type".into(), "text/plain\r\nInjected: yes".into()),
            ],
        ] {
            let mut invalid = v.manifest.clone();
            invalid.headers = headers;
            assert_eq!(
                f.call(f.uploader, Command::Prepare(p.request, invalid))
                    .await,
                Err(Failure::Metadata)
            );
            assert_eq!(f.observe(p).await, original);
        }
    }
    let original = f.observe(p).await;
    let mut reordered = v.manifest.clone();
    reordered.headers.reverse();
    assert_eq!(
        f.call(f.uploader, Command::Prepare(p.request, reordered))
            .await,
        Ok(Outcome::Changed(false))
    );
    assert_eq!(f.observe(p).await, original);
    let mut changed = v.manifest.clone();
    changed
        .headers
        .iter_mut()
        .find(|(name, _)| name == "Content-Type")
        .unwrap()
        .1 = "application/octet-stream".into();
    assert_eq!(
        f.call(f.uploader, Command::Prepare(p.request, changed))
            .await,
        Err(Failure::Manifest)
    );
    assert_eq!(f.observe(p).await, original);
    assert_eq!(
        f.call(f.uploader, Command::Expose(p.request.root)).await,
        Ok(Outcome::Exposed)
    );
    let exposed = f.observe(p).await;
    assert_eq!(exposed.phase, Phase::Registered);
    assert_eq!(exposed.usage, original.usage);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn retained_manifest_budget_survives_cancel_restart_and_other_tenant_admission() {
    let f = Fixture::new().await;
    f.enroll().await;
    let large = vectors::vector("media-10485760", 1);
    let p = f.permission(&large).await;
    f.admit(p).await;
    f.prepare(p, &large).await;
    assert_eq!(
        f.call(f.project, Command::Revoke(p.request)).await,
        Ok(Outcome::Changed(true))
    );
    let retained = f.observe(p).await;
    assert_eq!(retained.manifest, ManifestState::Bound);
    assert_eq!(retained.usage.unwrap().logical, 0);
    let small = f.permission(&vectors::vector("abc-text", 2)).await;
    assert_eq!(
        f.call(f.project, Command::Admit(small)).await,
        Err(Failure::TenantManifestCapacity)
    );
    assert_eq!(
        f.inspect(f.project, small.request).await,
        Err(Failure::Unknown)
    );
    f.restart().await;
    assert_eq!(f.observe(p).await, retained);
    assert_eq!(
        f.call(f.project, Command::Admit(p)).await,
        Ok(Outcome::Existing(Phase::Cancelled))
    );
    assert_eq!(
        f.call(f.project, Command::Admit(small)).await,
        Err(Failure::TenantManifestCapacity)
    );
    assert_eq!(
        f.call(
            f.operator,
            Command::Enroll {
                tenant: f.other.into(),
                expected: None,
                active: true
            }
        )
        .await,
        Ok(Outcome::Enrolled(Enrollment {
            generation: 1,
            active: true
        }))
    );

    // Only five global slots remain. A rejection must not reserve an ID/root or
    // consume those slots; the other project can still use all five afterward.
    let rejected = for_tenant(
        f.permission(&vectors::vector("pattern-8388608", 1)).await,
        f.other,
    );
    assert_eq!(
        f.call(f.other, Command::Admit(rejected)).await,
        Err(Failure::ManifestCapacity)
    );
    assert_eq!(
        f.inspect(f.other, rejected.request).await,
        Err(Failure::Unknown)
    );
    for (name, id) in [("pattern-3145728", 1), ("pattern-2097152", 2)] {
        let input = for_tenant(f.permission(&vectors::vector(name, id)).await, f.other);
        assert_eq!(
            f.call(f.other, Command::Admit(input)).await,
            Ok(Outcome::Admitted)
        );
        assert_eq!(
            f.call(f.other, Command::Revoke(input.request)).await,
            Ok(Outcome::Changed(true))
        );
        assert_eq!(
            f.inspect(f.other, input.request)
                .await
                .unwrap()
                .usage
                .unwrap()
                .logical,
            0
        );
    }
    let final_input = for_tenant(
        f.permission(&vectors::vector("abc-binary", 3)).await,
        f.other,
    );
    assert_eq!(
        f.call(f.other, Command::Admit(final_input)).await,
        Err(Failure::ManifestCapacity)
    );
    assert_eq!(f.observe(p).await, retained);
}

fn for_tenant(mut permission: Permission, tenant: Principal) -> Permission {
    permission.request.tenant = tenant;
    permission
}

#[tokio::test(flavor = "multi_thread")]
async fn ten_mib_manifest_authorizes_local_exposure_without_claiming_completion() {
    let f = Fixture::new().await;
    f.enroll().await;
    let v = vectors::vector("media-10485760", 1);
    let p = f.permission(&v).await;
    f.admit(p).await;
    let initial = f.observe(p).await;
    let mut invalid = v.manifest.clone();
    invalid.chunks[0][0] ^= 1;
    assert_eq!(
        f.call(f.uploader, Command::Prepare(p.request, invalid))
            .await,
        Err(Failure::Manifest)
    );
    assert_eq!(f.observe(p).await, initial);
    f.prepare(p, &v).await;
    let prepared = f.observe(p).await;
    assert_eq!(prepared.manifest, ManifestState::Bound);
    assert_eq!(prepared.phase, Phase::Reserved);
    assert_eq!(
        f.call(f.uploader, Command::Prepare(p.request, v.manifest))
            .await,
        Ok(Outcome::Changed(false))
    );
    assert_eq!(f.observe(p).await, prepared);
    f.restart().await;
    assert_eq!(f.observe(p).await, prepared);
    assert_eq!(
        f.call(f.uploader, Command::Expose(p.request.root)).await,
        Ok(Outcome::Exposed)
    );
    let exposed = f.observe(p).await;
    assert_eq!(exposed.phase, Phase::Registered);
    assert_eq!(exposed.usage, initial.usage);
    f.restart().await;
    assert_eq!(f.observe(p).await, exposed);
    assert_eq!(
        f.call(f.uploader, Command::Expose(p.request.root)).await,
        Err(Failure::Exposure(ExposureFailure::Phase))
    );
    assert_eq!(
        f.call(f.project, Command::Revoke(p.request)).await,
        Ok(Outcome::Changed(true))
    );
    let retained = f.observe(p).await;
    let usage = retained.usage.unwrap();
    let amount = u128::from(p.request.bytes);
    assert_eq!(
        (usage.logical, usage.physical, usage.liability),
        (amount, amount, amount)
    );
    assert_eq!(retained.phase, Phase::Registered);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn unsupported_upgrade_preserves_manifest_even_when_outgoing_hook_is_skipped() {
    let f = Fixture::new().await;
    f.enroll().await;
    let v = vectors::vector("media-1048577", 1);
    let p = f.permission(&v).await;
    f.admit(p).await;
    f.prepare(p, &v).await;
    let prefix = f.observe(p).await;
    let pic = &f.harness.pic;
    let error = pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            blob_test_protocol::fixture_wire::encode_args("fixture_post_upgrade", ()).unwrap(),
            Some(f.controller),
        )
        .await
        .unwrap_err();
    assert_eq!(
        crate::canister_rejection(&error).reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(f.observe(p).await, prefix);

    let wasm = Fixture::wasm();
    let hashes = {
        let mut values = Vec::new();
        for bytes in wasm.chunks(CHUNK) {
            values.push({
                pic.upload_chunk(f.service, Some(f.controller), bytes.to_vec())
                    .await
                    .unwrap()
            });
        }
        values
    };
    #[expect(
        clippy::default_trait_access,
        reason = "PocketIC reexports the install mode but not UpgradeFlags"
    )]
    let mut mode = CanisterInstallMode::Upgrade(Some(Default::default()));
    let CanisterInstallMode::Upgrade(Some(flags)) = &mut mode else {
        unreachable!()
    };
    flags.skip_pre_upgrade = Some(true);
    let error = pic
        .install_chunked_canister(
            f.service,
            Some(f.controller),
            mode,
            f.service,
            hashes,
            Sha256::digest(&wasm).to_vec(),
            blob_test_protocol::fixture_wire::encode_args("fixture_post_upgrade", ()).unwrap(),
        )
        .await
        .unwrap_err();
    assert_eq!(
        crate::canister_rejection(&error).reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(f.observe(p).await, prefix);
    assert_eq!(f.observe(p).await.manifest, ManifestState::Bound);
}
