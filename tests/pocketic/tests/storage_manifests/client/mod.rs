//! The uploader is a distinct canister, separate from the admitted tenant.
use super::*;
mod cancellation;
use blob_test_protocol::consumer::{
    Failure as C,
    manifests::{ManifestDispatch, ManifestFault, ManifestIntent, ManifestIntentView},
};
fn wasm() -> Vec<u8> {
    std::fs::read(fixture_path("BLOB_CONSUMER_PROBE_WASM")).unwrap()
}
fn fixture() -> Fixture {
    let mut f = Fixture::new();
    f.uploader = f.harness.pic.create_canister_with_settings(
        Some(f.controller),
        Some(CanisterSettings {
            controllers: Some(vec![f.controller]),
            ..CanisterSettings::default()
        }),
    );
    f.harness.pic.install_canister(
        f.uploader,
        wasm(),
        candid::encode_args((f.operator, f.service)).unwrap(),
        Some(f.controller),
    );
    f.enroll(None, true).unwrap();
    f
}
fn save(f: &Fixture, input: &ManifestIntent) -> Result<ManifestIntentView, C> {
    f.harness
        .pic
        .update_candid_as(f.uploader, f.operator, "save_manifest", (input,))
        .unwrap()
}
fn view(f: &Fixture, id: u128) -> Result<ManifestIntentView, C> {
    f.harness
        .pic
        .query_candid_as(f.uploader, f.operator, "manifest_intent", (id,))
        .unwrap()
}
fn dispatch(f: &Fixture, input: ManifestDispatch) -> Result<ManifestIntentView, C> {
    f.harness
        .pic
        .update_candid_as(f.uploader, f.operator, "dispatch_manifest", (input,))
        .unwrap()
}
fn recover(f: &Fixture, id: u128) -> Result<ManifestIntentView, C> {
    f.harness
        .pic
        .update_candid_as(f.uploader, f.operator, "recover_manifest", (id,))
        .unwrap()
}
fn run(id: u128) -> ManifestDispatch {
    ManifestDispatch {
        hold: false,
        id,
        max_reply_bytes: 4096,
        fault: ManifestFault::None,
    }
}
fn intent(f: &Fixture, id: u128, content: u8) -> (Permission, ManifestIntent) {
    let (p, declaration) = f.permission(id, content);
    (
        p,
        ManifestIntent {
            id,
            request: f.preparation_input(&declaration),
        },
    )
}
#[test]
fn manifest_client_recovers_committed_preparation_without_redispatch_after_reply_loss_or_callback_trap()
 {
    for trap in [false, true] {
        let f = fixture();
        let (permission, input) = intent(&f, 1, 7);
        f.admit(f.tenant, permission).unwrap();
        let before = f.harness.pic.get_stable_memory(f.service);
        assert!(!save(&f, &input).unwrap().started);
        assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
        if trap {
            let rejected = f
                .harness
                .pic
                .update_call(
                    f.uploader,
                    f.operator,
                    "dispatch_manifest",
                    candid::encode_one(ManifestDispatch {
                        fault: ManifestFault::AfterPreparation,
                        ..run(1)
                    })
                    .unwrap(),
                )
                .unwrap_err();
            assert_eq!(rejected.reject_code, RejectCode::CanisterError);
        } else {
            assert_eq!(
                dispatch(
                    &f,
                    ManifestDispatch {
                        max_reply_bytes: 1,
                        ..run(1)
                    }
                ),
                Err(C::Transport)
            );
        }
        let pending = view(&f, 1).unwrap();
        assert!(pending.started);
        assert_eq!(pending.result, None);
        assert_eq!(dispatch(&f, run(1)), Err(C::Pending));
        f.expose(permission.request).unwrap();
        f.revoke(permission).unwrap();
        f.harness
            .pic
            .upgrade_canister(
                f.service,
                Fixture::wasm(),
                candid::encode_one(f.operator).unwrap(),
                Some(f.controller),
            )
            .unwrap();
        let before = f.harness.pic.get_stable_memory(f.service);
        let recovered = recover(&f, 1).unwrap();
        assert_eq!(
            recovered.result,
            Some(Ok(input.request.declaration.clone()))
        );
        assert_eq!(dispatch(&f, run(1)), Ok(recovered));
        assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    }
}
#[test]
fn manifest_start_write_rolls_back_before_dispatch_and_queries_cannot_send() {
    let f = fixture();
    let (permission, input) = intent(&f, 1, 1);
    f.admit(f.tenant, permission).unwrap();
    save(&f, &input).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let local = f.harness.pic.get_stable_memory(f.uploader);
    let rejected = f
        .harness
        .pic
        .update_call(
            f.uploader,
            f.operator,
            "dispatch_manifest",
            candid::encode_one(ManifestDispatch {
                fault: ManifestFault::BeforeDispatch,
                ..run(1)
            })
            .unwrap(),
        )
        .unwrap_err();
    assert_eq!(rejected.reject_code, RejectCode::CanisterError);
    assert!(!view(&f, 1).unwrap().started);
    let queried: Result<UploadManifestResponse, C> = f
        .harness
        .pic
        .query_candid_as(f.uploader, f.operator, "query_manifest", (1u128,))
        .unwrap();
    assert_eq!(queried, Err(C::Transport));
    let denied: Result<ManifestIntentView, C> = f
        .harness
        .pic
        .update_candid_as(f.uploader, f.other, "dispatch_manifest", (run(1),))
        .unwrap();
    assert_eq!(denied, Err(C::Denied));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    assert_eq!(f.harness.pic.get_stable_memory(f.uploader), local);
    assert_eq!(
        dispatch(&f, run(1)).unwrap().result,
        Some(Ok(input.request.declaration))
    );
}
#[test]
fn manifest_refusal_is_retained_and_unprepared_inspection_cannot_clear_uncertainty() {
    for lose_reply in [false, true] {
        let f = fixture();
        let (permission, input) = intent(&f, 1, 4);
        f.admit(f.tenant, permission).unwrap();
        save(&f, &input).unwrap();
        let suspended = f.enroll(Some(f.tenant().unwrap()), false).unwrap();
        let outcome = dispatch(
            &f,
            ManifestDispatch {
                max_reply_bytes: if lose_reply { 1 } else { 4096 },
                ..run(1)
            },
        );
        if lose_reply {
            assert_eq!(outcome, Err(C::Transport));
            assert_eq!(recover(&f, 1), Err(C::Pending));
            assert_eq!(view(&f, 1).unwrap().result, None);
        } else {
            assert_eq!(
                outcome.unwrap().result,
                Some(Err(F::Permission(A::Inactive)))
            );
        }
        f.enroll(Some(suspended), true).unwrap();
        let before = f.harness.pic.get_stable_memory(f.service);
        if lose_reply {
            assert_eq!(dispatch(&f, run(1)), Err(C::Pending));
        } else {
            assert_eq!(
                dispatch(&f, run(1)).unwrap().result,
                Some(Err(F::Permission(A::Inactive)))
            );
        }
        assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    }
}
#[test]
fn manifest_intent_capacity_conflicts_and_upgrade_preserve_uncertain_originals() {
    let f = fixture();
    let (permission, input) = intent(&f, 1, 3);
    f.admit(f.tenant, permission).unwrap();
    save(&f, &input).unwrap();
    let mut changed = input.clone();
    changed.request.permission.expires_at_ns -= 1;
    assert_eq!(save(&f, &changed), Err(C::Conflict));
    changed = input.clone();
    changed.id = 2;
    assert_eq!(save(&f, &changed), Err(C::Conflict));
    let (_, second) = intent(&f, 2, 5);
    save(&f, &second).unwrap();
    let (_, third) = intent(&f, 3, 6);
    assert_eq!(save(&f, &third), Err(C::Capacity));
    assert_eq!(save(&f, &input).unwrap().intent, input);
    assert_eq!(
        dispatch(
            &f,
            ManifestDispatch {
                max_reply_bytes: 1,
                ..run(1)
            }
        ),
        Err(C::Transport)
    );
    let pending = view(&f, 1).unwrap();
    f.harness
        .pic
        .upgrade_canister(
            f.uploader,
            wasm(),
            candid::encode_args(()).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(view(&f, 1), Ok(pending));
    assert_eq!(recover(&f, 1), Err(C::Fenced));
    assert_eq!(dispatch(&f, run(1)), Err(C::Fenced));
    assert_eq!(save(&f, &input), Err(C::Fenced));
}
