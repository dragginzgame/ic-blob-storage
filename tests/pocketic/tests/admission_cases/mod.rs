//! Direct-upload manifest, retained accounting and lifecycle evidence for the actual shared owner.
use super::*;
use ic_testkit::pocket_ic::CanisterInstallMode;
use sha2::{Digest, Sha256};

const CHUNK: usize = 1024 * 1024;

#[test]
fn metadata_rejection_preserves_reservation_and_reordered_retry_preserves_binding() {
    let f = Fixture::new();
    f.enroll();
    let v = vectors::vector("abc-text", 1);
    let p = f.permission(&v);
    f.admit(p);
    for bound in [false, true] {
        if bound {
            f.prepare(p, &v);
        }
        let original = f.observe(p);
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
                f.call(f.uploader, Command::Prepare(p.request, invalid)),
                Err(Failure::Metadata)
            );
            assert_eq!(f.observe(p), original);
        }
    }
    let original = f.observe(p);
    let mut reordered = v.manifest.clone();
    reordered.headers.reverse();
    assert_eq!(
        f.call(f.uploader, Command::Prepare(p.request, reordered)),
        Ok(Outcome::Changed(false))
    );
    assert_eq!(f.observe(p), original);
    let mut changed = v.manifest.clone();
    changed
        .headers
        .iter_mut()
        .find(|(name, _)| name == "Content-Type")
        .unwrap()
        .1 = "application/octet-stream".into();
    assert_eq!(
        f.call(f.uploader, Command::Prepare(p.request, changed)),
        Err(Failure::Manifest)
    );
    assert_eq!(f.observe(p), original);
    assert_eq!(
        f.call(f.uploader, Command::Expose(p.request.root)),
        Ok(Outcome::Exposed)
    );
    let exposed = f.observe(p);
    assert_eq!(exposed.phase, Phase::ExposurePossible);
    assert_eq!(exposed.usage, original.usage);
}

#[test]
fn retained_manifest_budget_survives_cancel_restart_and_other_tenant_admission() {
    let f = Fixture::new();
    f.enroll();
    let large = vectors::vector("media-10485760", 1);
    let p = f.permission(&large);
    f.admit(p);
    f.prepare(p, &large);
    assert_eq!(
        f.call(f.project, Command::Revoke(p.request)),
        Ok(Outcome::Changed(true))
    );
    let retained = f.observe(p);
    assert_eq!(retained.manifest, ManifestState::Bound);
    assert_eq!(retained.usage.unwrap().logical, 0);
    let small = f.permission(&vectors::vector("abc-text", 2));
    assert_eq!(
        f.call(f.project, Command::Admit(small)),
        Err(Failure::TenantManifestCapacity)
    );
    assert_eq!(f.inspect(f.project, small.request), Err(Failure::Unknown));
    f.restart();
    assert_eq!(f.observe(p), retained);
    assert_eq!(
        f.call(f.project, Command::Admit(p)),
        Ok(Outcome::Existing(Phase::Cancelled))
    );
    assert_eq!(
        f.call(f.project, Command::Admit(small)),
        Err(Failure::TenantManifestCapacity)
    );
    assert_eq!(
        f.call(
            f.operator,
            Command::Enroll {
                tenant: f.other,
                expected: None,
                active: true
            }
        ),
        Ok(Outcome::Enrolled(Enrollment {
            generation: 1,
            active: true
        }))
    );

    // Only five global slots remain. A rejection must not reserve an ID/root or
    // consume those slots; the other project can still use all five afterward.
    let rejected = for_tenant(
        f.permission(&vectors::vector("pattern-8388608", 1)),
        f.other,
    );
    assert_eq!(
        f.call(f.other, Command::Admit(rejected)),
        Err(Failure::ManifestCapacity)
    );
    assert_eq!(f.inspect(f.other, rejected.request), Err(Failure::Unknown));
    for (name, id) in [("pattern-3145728", 1), ("pattern-2097152", 2)] {
        let input = for_tenant(f.permission(&vectors::vector(name, id)), f.other);
        assert_eq!(
            f.call(f.other, Command::Admit(input)),
            Ok(Outcome::Admitted)
        );
        assert_eq!(
            f.call(f.other, Command::Revoke(input.request)),
            Ok(Outcome::Changed(true))
        );
        assert_eq!(
            f.inspect(f.other, input.request)
                .unwrap()
                .usage
                .unwrap()
                .logical,
            0
        );
    }
    let final_input = for_tenant(f.permission(&vectors::vector("abc-binary", 3)), f.other);
    assert_eq!(
        f.call(f.other, Command::Admit(final_input)),
        Err(Failure::ManifestCapacity)
    );
    assert_eq!(f.observe(p), retained);
}

fn for_tenant(mut permission: Permission, tenant: Principal) -> Permission {
    permission.request.tenant = tenant;
    permission
}

#[test]
fn ten_mib_manifest_authorizes_local_exposure_without_claiming_completion() {
    let f = Fixture::new();
    f.enroll();
    let v = vectors::vector("media-10485760", 1);
    let p = f.permission(&v);
    f.admit(p);
    let initial = f.observe(p);
    let mut invalid = v.manifest.clone();
    invalid.chunks[0][0] ^= 1;
    assert_eq!(
        f.call(f.uploader, Command::Prepare(p.request, invalid)),
        Err(Failure::Manifest)
    );
    assert_eq!(f.observe(p), initial);
    f.prepare(p, &v);
    let prepared = f.observe(p);
    assert_eq!(prepared.manifest, ManifestState::Bound);
    assert_eq!(prepared.phase, Phase::Reserved);
    assert_eq!(
        f.call(f.uploader, Command::Prepare(p.request, v.manifest)),
        Ok(Outcome::Changed(false))
    );
    assert_eq!(f.observe(p), prepared);
    f.restart();
    assert_eq!(f.observe(p), prepared);
    assert_eq!(
        f.call(f.uploader, Command::Expose(p.request.root)),
        Ok(Outcome::Exposed)
    );
    let exposed = f.observe(p);
    assert_eq!(exposed.phase, Phase::ExposurePossible);
    assert_eq!(exposed.usage, initial.usage);
    f.restart();
    assert_eq!(f.observe(p), exposed);
    assert_eq!(
        f.call(f.uploader, Command::Expose(p.request.root)),
        Err(Failure::Phase)
    );
    assert_eq!(
        f.call(f.project, Command::Revoke(p.request)),
        Ok(Outcome::Changed(true))
    );
    let retained = f.observe(p);
    let usage = retained.usage.unwrap();
    let amount = u128::from(p.request.bytes);
    assert_eq!(
        (usage.logical, usage.physical, usage.liability),
        (amount, amount, amount)
    );
    assert_eq!(retained.phase, Phase::ExposurePossible);
}

#[test]
fn unsupported_upgrade_preserves_manifest_even_when_outgoing_hook_is_skipped() {
    let f = Fixture::new();
    f.enroll();
    let v = vectors::vector("media-1048577", 1);
    let p = f.permission(&v);
    f.admit(p);
    f.prepare(p, &v);
    let prefix = f.observe(p);
    let pic = &f.harness.pic;
    let error = pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            candid::encode_args(()).unwrap(),
            Some(f.controller),
        )
        .unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    assert_eq!(f.observe(p), prefix);

    let wasm = Fixture::wasm();
    let hashes = wasm
        .chunks(CHUNK)
        .map(|bytes| {
            pic.upload_chunk(f.service, Some(f.controller), bytes.to_vec())
                .unwrap()
        })
        .collect();
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
            candid::encode_args(()).unwrap(),
        )
        .unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    assert_eq!(f.observe(p), prefix);
    assert_eq!(f.observe(p).manifest, ManifestState::Bound);
}
