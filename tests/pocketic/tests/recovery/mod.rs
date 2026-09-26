//! Authority containment and source-only fenced journal restoration on the IC.
mod authority;
mod source;
use super::*;
use blob_test_protocol::journey::readback::ReadSourceMode;
use ic_testkit::pocket_ic::CanisterInstallMode;
use std::time::Duration;

const CHUNK: usize = 1024 * 1024;

impl Fixture {
    fn controller(&self, canister: Principal) -> Option<Principal> {
        (canister == self.service).then_some(self.operator)
    }

    fn restart(&self, canister: Principal) {
        let pic = &self.harness.pic;
        pic.stop_canister(canister, self.controller(canister))
            .expect("stop current instance");
        pic.start_canister(canister, self.controller(canister))
            .expect("start current instance");
    }

    fn wasm(&self, canister: Principal) -> Vec<u8> {
        std::fs::read(fixture_path(if canister == self.service {
            "BLOB_AUTHORITY_PROBE_WASM"
        } else {
            "BLOB_GATEWAY_SOURCE_WASM"
        }))
        .expect("fixture Wasm")
    }

    fn reject_upgrade(&self, canister: Principal) {
        let rejected = self
            .harness
            .pic
            .upgrade_canister(
                canister,
                self.wasm(canister),
                candid::encode_args(()).expect("no args"),
                self.controller(canister),
            )
            .expect_err("unimplemented restore must reject before discarding state");
        assert_eq!(rejected.reject_code, RejectCode::CanisterError);
    }

    fn upgrade_skipping_outgoing_hook(&self, canister: Principal, succeeds: bool) {
        let pic = &self.harness.pic;
        let wasm = self.wasm(canister);
        let hashes: Vec<_> = wasm
            .chunks(CHUNK)
            .map(|bytes| {
                pic.upload_chunk(canister, self.controller(canister), bytes.to_vec())
                    .expect("local Wasm chunk")
            })
            .collect();
        #[expect(
            clippy::default_trait_access,
            reason = "PocketIC reexports the mode but not UpgradeFlags; infer its own type without another direct dependency"
        )]
        let mut mode = CanisterInstallMode::Upgrade(Some(Default::default()));
        let CanisterInstallMode::Upgrade(Some(flags)) = &mut mode else {
            unreachable!("upgrade flags")
        };
        flags.skip_pre_upgrade = Some(true);
        let result = pic.install_chunked_canister(
            canister,
            self.controller(canister),
            mode,
            canister,
            hashes,
            Sha256::digest(&wasm).to_vec(),
            candid::encode_args(()).expect("no args"),
        );
        if succeeds {
            result.expect("source restores synchronously with outgoing hook skipped");
        } else {
            assert_eq!(
                result.expect_err("unsupported restore").reject_code,
                RejectCode::CanisterError
            );
        }
    }
}

#[test]
fn stop_start_retains_verified_prefix_exposure_and_unchanged_accounting() {
    let f = Fixture::new();
    let partial = chunks::vector("pattern-1048577", 1);
    let exposed = chunks::vector("abc-text", 1);
    assert_eq!(
        f.reserve_manifest(f.first, partial.upload, partial.manifest.clone()),
        Ok(())
    );
    assert_eq!(
        f.append(f.first, partial.upload, 0, &partial.bytes[..CHUNK]),
        Ok(())
    );
    assert_eq!(
        f.reserve_manifest(f.second, exposed.upload, exposed.manifest),
        Ok(())
    );
    assert_eq!(
        f.append(f.second, exposed.upload, 0, &exposed.bytes),
        Ok(())
    );
    f.certificate(f.second, exposed.upload.root)
        .expect("authority issued before stop");
    let prefix = f.progress(f.first, partial.upload);
    let first_usage = f.usage(f.first);
    let second_usage = f.usage(f.second);
    f.restart(f.service);
    f.restart(f.gateway);
    f.harness.pic.advance_time(Duration::from_secs(3600));
    assert_eq!(f.progress(f.first, partial.upload), prefix);
    assert_eq!(f.usage(f.first), first_usage);
    assert_eq!(f.usage(f.second), second_usage);
    assert_eq!(
        f.certificate(f.second, exposed.upload.root),
        Err(RejectCode::CanisterError)
    );
    assert_eq!(
        f.control(f.second, "journey_cancel", exposed.upload),
        Err(JourneyFailure::InvalidPhase)
    );
    assert_eq!(
        f.append(f.first, partial.upload, 0, &partial.bytes[..CHUNK]),
        Ok(())
    );
    assert_eq!(f.progress(f.first, partial.upload), prefix);
    assert_eq!(
        f.append(f.first, partial.upload, 1, &partial.bytes[CHUNK..]),
        Ok(())
    );
    f.certificate(f.first, partial.upload.root)
        .expect("continued prefix");
    assert_eq!(f.complete(f.first, partial.upload), Ok(()));
    assert_eq!(f.complete(f.second, exposed.upload), Ok(()));
    assert_eq!(f.usage(f.first), first_usage);
    assert_eq!(f.usage(f.second), second_usage);
    f.source_config(
        partial.upload,
        1,
        &partial.bytes[CHUNK..],
        ReadSourceMode::Valid,
        false,
    );
    assert_eq!(
        f.read_chunk(f.first, partial.upload, 1)
            .expect("read after restart")
            .bytes,
        partial.bytes[CHUNK..]
    );
}

#[test]
fn rejected_upgrades_keep_uncertain_roots_cancelled_history_and_continuing_billing() {
    let f = Fixture::new();
    let deleted = chunks::vector("abc-binary", 1);
    f.confirm_bytes(&deleted);
    assert_eq!(
        f.root_control(f.first, "journey_release", deleted.upload.root),
        Ok(())
    );
    assert_eq!(f.delete(vec![root(deleted.upload.root)]), Ok(()));
    let exposed = chunks::vector("abc-text", 2);
    assert_eq!(
        f.reserve_manifest(f.first, exposed.upload, exposed.manifest.clone()),
        Ok(())
    );
    assert_eq!(f.append(f.first, exposed.upload, 0, &exposed.bytes), Ok(()));
    f.certificate(f.first, exposed.upload.root)
        .expect("uncertain upload");
    let cancelled = upload(3, b"q");
    assert_eq!(f.reserve(f.first, cancelled), Ok(()));
    assert_eq!(f.control(f.first, "journey_cancel", cancelled), Ok(()));
    assert_eq!(f.usage(f.first), Ok(usage(3, 3, 6)));
    f.reject_upgrade(f.service);
    assert_eq!(f.usage(f.first), Ok(usage(3, 3, 6)));
    assert_eq!(
        f.reserve_manifest(f.first, exposed.upload, exposed.manifest.clone()),
        Ok(())
    );
    assert_eq!(
        f.certificate(f.first, exposed.upload.root),
        Err(RejectCode::CanisterError)
    );
    assert_eq!(
        f.control(f.first, "journey_cancel", exposed.upload),
        Err(JourneyFailure::InvalidPhase)
    );
    assert_eq!(
        f.reserve(f.first, JourneyUpload { id: 4, ..cancelled }),
        Err(JourneyFailure::Conflict)
    );
    assert_eq!(f.control(f.first, "journey_cancel", cancelled), Ok(()));
    assert_eq!(
        f.root_control(f.operator, "journey_settle", deleted.upload.root),
        Ok(())
    );
    assert_eq!(f.usage(f.first), Ok(usage(3, 3, 3)));
    assert_eq!(f.complete(f.first, exposed.upload), Ok(()));
    f.source_config(
        exposed.upload,
        0,
        &exposed.bytes,
        ReadSourceMode::Valid,
        false,
    );
    assert_eq!(
        f.read_chunk(f.first, exposed.upload, 0)
            .expect("old instance still usable")
            .bytes,
        exposed.bytes
    );
}

#[test]
fn skipping_pre_upgrade_still_rejects_and_rolls_back_to_the_usable_instance() {
    let f = Fixture::new();
    let v = chunks::vector("abc-text", 1);
    f.confirm_bytes(&v);
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, false);
    f.upgrade_skipping_outgoing_hook(f.service, false);
    assert_eq!(f.usage(f.first), Ok(usage(3, 3, 3)));
    assert_eq!(
        f.certificate(f.first, v.upload.root),
        Err(RejectCode::CanisterError)
    );
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0)
            .expect("retained object and source bytes")
            .bytes,
        v.bytes
    );
}

#[test]
fn rejected_upgrades_preserve_the_held_read_and_its_release_race() {
    let f = Fixture::new();
    let v = chunks::vector("abc-text", 1);
    f.confirm_bytes(&v);
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, true);
    let id = f.hold_read(v.upload);
    for canister in [f.service, f.gateway] {
        f.reject_upgrade(canister);
        assert_eq!(
            f.read_chunk(f.first, v.upload, 0),
            Err(JourneyFailure::ReadInProgress)
        );
        assert_eq!(f.source_observation().requests, 1);
        assert!(f.source_observation().waiting);
    }
    assert_eq!(
        f.root_control(f.first, "journey_release", v.upload.root),
        Ok(())
    );
    assert_eq!(f.resume_read(id), Err(JourneyFailure::InvalidPhase));
    assert_eq!(f.usage(f.first), Ok(usage(0, 3, 3)));
}

#[test]
fn callback_trap_rolls_back_slot_release_and_stop_start_cannot_clear_it() {
    let f = Fixture::new();
    let v = chunks::vector("abc-text", 1);
    f.confirm_bytes(&v);
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, false);
    for caller in [
        f.first,
        f.second,
        f.gateway,
        Principal::anonymous(),
        f.operator,
    ] {
        let armed: bool = f
            .harness
            .pic
            .update_candid_as(
                f.service,
                caller,
                "journey_arm_read_callback_trap",
                (root(v.upload.root),),
            )
            .expect("fault control");
        assert_eq!(armed, caller == f.operator);
    }
    let error = f
        .harness
        .pic
        .update_call(
            f.service,
            f.first,
            "journey_read",
            candid::encode_args((root(v.upload.root), 0_u64)).expect("read args"),
        )
        .expect_err("actual callback trap");
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    assert_eq!(f.source_observation().requests, 1);
    assert!(!f.source_observation().waiting);
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0),
        Err(JourneyFailure::ReadInProgress)
    );
    f.reject_upgrade(f.service);
    f.restart(f.service);
    f.harness.pic.advance_time(Duration::from_hours(24));
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0),
        Err(JourneyFailure::ReadInProgress)
    );
    assert_eq!(f.source_observation().requests, 1);
    assert_eq!(f.usage(f.first), Ok(usage(3, 3, 3)));
    assert_eq!(
        f.root_control(f.first, "journey_release", v.upload.root),
        Ok(())
    );
    assert_eq!(f.usage(f.first), Ok(usage(0, 3, 3)));
}
