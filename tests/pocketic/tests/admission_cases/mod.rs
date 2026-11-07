//! Direct-upload manifest, retained accounting and lifecycle evidence for the actual shared owner.
use super::*;

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
async fn workload_reopen_retains_the_original_manifest_and_fences_fresh_admission() {
    use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
    let f = Fixture::new().await;
    f.enroll().await;
    let v = vectors::vector("media-1048577", 1);
    let p = f.permission(&v).await;
    f.admit(p).await;
    f.prepare(p, &v).await;
    let before = f.observe(p).await;
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            blob_test_protocol::fixture_wire::encode_one("fixture_post_upgrade", f.installation)
                .unwrap(),
            Some(f.controller),
        )
        .await
        .unwrap();
    assert_eq!(f.observe(p).await, before);
    assert_eq!(
        f.call(f.project, Command::Admit(p)).await,
        Err(Failure::Fenced)
    );
    let capacity: Result<
        ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityResponse,
        ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityFailure,
    > = f
        .harness
        .pic
        .query_blob_as(
            f.service,
            f.project,
            "blob_upload_capacity",
            ic_blob_storage_contracts::dto::tenant::TenantScope {
                service: f.service.into(),
                tenant: f.project.into(),
                namespace: 1,
            },
            4096,
        )
        .await
        .unwrap();
    assert!(capacity.unwrap().fenced);
}
