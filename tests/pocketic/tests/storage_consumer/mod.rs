//! Real consumer-owned transactions through canonical storage clients.
use super::*;
use blob_test_protocol::consumer::{
    AssetView, Failure as ConsumerFailure, Fault, Recovery, Registration, RegistrationSource,
    Release, Run, Use,
};
use blob_test_protocol::storage::ProviderFact;
use ic_blob_storage::dto::reference::{
    ReferenceAction, ReferenceChange, ReferenceCommand, ReferenceFailure, ReferenceReceiptLookup,
    ReferenceUpload,
};
fn consumer_wasm() -> Vec<u8> {
    std::fs::read(fixture_path("BLOB_CONSUMER_PROBE_WASM")).unwrap()
}
fn fixture() -> Fixture {
    let mut f = Fixture::new();
    let tenant = f.harness.pic.create_canister_with_settings(
        Some(f.controller),
        Some(CanisterSettings {
            controllers: Some(vec![f.controller]),
            ..CanisterSettings::default()
        }),
    );
    f.harness.pic.install_canister(
        tenant,
        consumer_wasm(),
        candid::encode_args((f.operator, f.service)).unwrap(),
        Some(f.controller),
    );
    f.tenant = tenant;
    f.enroll(None, true).unwrap();
    f
}
fn input(f: &Fixture, id: u8) -> Run {
    let (permission, manifest) = f.permission(u128::from(id), id);
    f.admit(f.tenant, permission).unwrap();
    f.prepare(&manifest).unwrap();
    f.expose(permission.request).unwrap();
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    Run {
        registration: Registration {
            asset: u128::from(id),
            payload: vec![id; 8],
            release_operation: 2,
            source: RegistrationSource::Existing(ReferenceCommand {
                upload: ReferenceUpload {
                    service: f.service,
                    tenant: f.tenant,
                    namespace: 1,
                    upload: permission.request.id,
                    object: permission.request.id,
                    incarnation: 1,
                    first_reference: 1,
                    root: permission.request.root,
                    bytes: 10,
                },
                operation: 1,
                reference: 2,
                action: ReferenceAction::Retain,
            }),
        },
        fault: Fault::None,
        hold: false,
        max_reply_bytes: 4096,
    }
}
fn register(f: &Fixture, input: &Run) -> Result<AssetView, ConsumerFailure> {
    f.harness
        .pic
        .update_candid_as(f.tenant, f.operator, "register", (input,))
        .unwrap()
}
fn asset(f: &Fixture, id: u128) -> Result<AssetView, ConsumerFailure> {
    f.harness
        .pic
        .query_candid_as(f.tenant, f.operator, "asset", (id,))
        .unwrap()
}
fn cancel(f: &Fixture, id: u128) -> Result<AssetView, ConsumerFailure> {
    f.harness
        .pic
        .update_candid_as(f.tenant, f.operator, "cancel", (id,))
        .unwrap()
}
fn release(f: &Fixture, id: u128) -> Result<AssetView, ConsumerFailure> {
    f.harness
        .pic
        .update_candid_as(
            f.tenant,
            f.operator,
            "release",
            (Release {
                asset: id,
                fault: Fault::None,
            },),
        )
        .unwrap()
}
fn recover(f: &Fixture, id: u128, release: bool) -> Result<AssetView, ConsumerFailure> {
    f.harness
        .pic
        .update_candid_as(
            f.tenant,
            f.operator,
            "recover",
            (Recovery { asset: id, release },),
        )
        .unwrap()
}
fn use_asset(f: &Fixture, attach: bool) -> Result<AssetView, ConsumerFailure> {
    f.harness
        .pic
        .update_candid_as(
            f.tenant,
            f.operator,
            "use_asset",
            (Use { asset: 1, attach },),
        )
        .unwrap()
}
fn receipt(f: &Fixture, command: ReferenceCommand) -> ReferenceReceiptLookup {
    let value: Result<ReferenceReceiptLookup, ReferenceFailure> = f
        .harness
        .pic
        .query_candid_as(f.service, f.tenant, "blob_reference_receipt", (command,))
        .unwrap();
    value.unwrap()
}
#[test]
fn cancellation_and_cleanup_prevent_late_registration_and_new_uses() {
    let f = fixture();
    let mut input = input(&f, 1);
    input.hold = true;
    let call = f
        .harness
        .pic
        .submit_call(
            f.tenant,
            f.operator,
            "register",
            candid::encode_one(&input).unwrap(),
        )
        .unwrap();
    let mut held = false;
    for _ in 0..40 {
        f.harness.pic.tick();
        let waiting: Result<bool, ConsumerFailure> = f
            .harness
            .pic
            .query_candid_as(f.tenant, f.operator, "waiting", ())
            .unwrap();
        if waiting.unwrap() {
            held = true;
            break;
        }
    }
    assert!(
        held,
        "consumer must reach the post-descriptor publication hold"
    );
    assert!(!asset(&f, 1).unwrap().published);
    assert!(cancel(&f, 1).unwrap().cancelled);
    assert_eq!(use_asset(&f, true), Err(ConsumerFailure::State));
    assert_eq!(
        release(&f, 1).unwrap().release_result,
        Some(Ok(ReferenceChange::Changed))
    );
    let resumed: Result<(), ConsumerFailure> = f
        .harness
        .pic
        .update_candid_as(f.tenant, f.operator, "resume", ())
        .unwrap();
    resumed.unwrap();
    let result: Result<AssetView, ConsumerFailure> =
        candid::decode_one(&f.harness.pic.await_call(call).unwrap()).unwrap();
    assert_eq!(result, Err(ConsumerFailure::State));
    let state = register(&f, &input).unwrap();
    assert!(state.cancelled);
    assert!(!state.published_once);
    assert!(!state.published);
    assert_eq!(
        f.status().usage,
        JourneyUsage {
            logical: 10,
            physical: 10,
            liability: 10
        }
    );
}
#[test]
fn consumer_traps_preserve_intent_and_recover_without_retain_redispatch() {
    for fault in [Fault::AfterIntent, Fault::AfterRetain, Fault::AfterPublish] {
        let f = fixture();
        let mut input = input(&f, 1);
        input.fault = fault;
        let error = f
            .harness
            .pic
            .update_call(
                f.tenant,
                f.operator,
                "register",
                candid::encode_one(&input).unwrap(),
            )
            .unwrap_err();
        assert_eq!(error.reject_code, RejectCode::CanisterError);
        input.fault = Fault::None;
        if fault == Fault::AfterIntent {
            assert_eq!(asset(&f, 1), Err(ConsumerFailure::Unknown));
            assert_eq!(
                receipt(&f, retained(&input.registration)),
                ReferenceReceiptLookup::Absent
            );
            assert!(register(&f, &input).unwrap().published);
        } else if fault == Fault::AfterRetain {
            let pending = asset(&f, 1).unwrap();
            assert!(pending.retain_started);
            assert_eq!(pending.retain_result, None);
            cancel(&f, 1).unwrap();
            assert_eq!(release(&f, 1), Err(ConsumerFailure::State));
            let before = f.harness.pic.get_stable_memory(f.service);
            let recovered = recover(&f, 1, false).unwrap();
            assert!(recovered.cancelled);
            assert_eq!(recovered.retain_result, Some(Ok(ReferenceChange::Changed)));
            assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
            assert_eq!(
                release(&f, 1).unwrap().release_result,
                Some(Ok(ReferenceChange::Changed))
            );
            assert!(!register(&f, &input).unwrap().published);
        } else {
            assert!(!asset(&f, 1).unwrap().published);
            let before = f.harness.pic.get_stable_memory(f.service);
            let published = register(&f, &input).unwrap();
            assert!(published.published);
            assert_eq!(register(&f, &input), Ok(published));
            assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
        }
    }
}
#[test]
fn release_acknowledgment_trap_preserves_tombstone_and_recovery_evidence_through_upgrade() {
    let f = fixture();
    let input = input(&f, 1);
    assert!(register(&f, &input).unwrap().published);
    assert_eq!(use_asset(&f, true).unwrap().uses, 1);
    assert_eq!(cancel(&f, 1), Err(ConsumerFailure::State));
    use_asset(&f, false).unwrap();
    cancel(&f, 1).unwrap();
    let error = f
        .harness
        .pic
        .update_call(
            f.tenant,
            f.operator,
            "release",
            candid::encode_one(Release {
                asset: 1,
                fault: Fault::AfterRelease,
            })
            .unwrap(),
        )
        .unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    let pending = asset(&f, 1).unwrap();
    assert!(pending.release_started);
    assert_eq!(pending.release_result, None);
    assert!(pending.cancelled && pending.published_once && !pending.published);
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(
        recover(&f, 1, true).unwrap().release_result,
        Some(Ok(ReferenceChange::Changed))
    );
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    let recovered = asset(&f, 1).unwrap();
    f.harness
        .pic
        .upgrade_canister(
            f.tenant,
            consumer_wasm(),
            candid::encode_args(()).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(asset(&f, 1), Ok(recovered));
    assert_eq!(register(&f, &input), Err(ConsumerFailure::Fenced));
    assert_eq!(release(&f, 1), Err(ConsumerFailure::Fenced));
    assert_eq!(recover(&f, 1, true), Err(ConsumerFailure::Fenced));
    assert_eq!(use_asset(&f, true), Err(ConsumerFailure::Fenced));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
}
#[test]
fn bounded_consumer_history_rejects_changed_payload_and_preserves_cleanup_at_capacity() {
    let f = fixture();
    let first = input(&f, 1);
    let second = input(&f, 2);
    register(&f, &first).unwrap();
    register(&f, &second).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let mut changed = first.clone();
    changed.registration.payload.push(9);
    assert_eq!(register(&f, &changed), Err(ConsumerFailure::Conflict));
    let mut third = first.clone();
    third.registration.asset = 3;
    let mut command = retained(&third.registration);
    command.reference = 3;
    command.operation = 3;
    third.registration.source = RegistrationSource::Existing(command);
    third.registration.release_operation = 4;
    assert_eq!(register(&f, &third), Err(ConsumerFailure::Capacity));
    let denied: Result<AssetView, ConsumerFailure> = f
        .harness
        .pic
        .update_candid_as(f.tenant, f.other, "register", (&first,))
        .unwrap();
    assert_eq!(denied, Err(ConsumerFailure::Denied));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    for id in [1, 2] {
        cancel(&f, id).unwrap();
        assert_eq!(
            release(&f, id).unwrap().release_result,
            Some(Ok(ReferenceChange::Changed))
        );
    }
    assert_eq!(register(&f, &third), Err(ConsumerFailure::Capacity));
    assert!(register(&f, &first).unwrap().cancelled);
}

#[test]
fn upgrade_keeps_uncertain_retain_and_tombstone_without_authorizing_recovery_or_resend() {
    let f = fixture();
    let mut input = input(&f, 1);
    input.fault = Fault::AfterRetain;
    let error = f
        .harness
        .pic
        .update_call(
            f.tenant,
            f.operator,
            "register",
            candid::encode_one(&input).unwrap(),
        )
        .unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    let pending = cancel(&f, 1).unwrap();
    assert!(pending.retain_started && pending.cancelled);
    assert_eq!(pending.retain_result, None);
    assert!(matches!(
        receipt(&f, retained(&input.registration)),
        ReferenceReceiptLookup::Found(_)
    ));
    let before = f.harness.pic.get_stable_memory(f.service);
    f.harness
        .pic
        .upgrade_canister(
            f.tenant,
            consumer_wasm(),
            candid::encode_args(()).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(asset(&f, 1), Ok(pending));
    assert_eq!(recover(&f, 1, false), Err(ConsumerFailure::Fenced));
    assert_eq!(register(&f, &input), Err(ConsumerFailure::Fenced));
    assert_eq!(release(&f, 1), Err(ConsumerFailure::Fenced));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
}

fn retained(intent: &Registration) -> ReferenceCommand {
    match intent.source {
        RegistrationSource::Existing(command) => command,
        RegistrationSource::Fresh(_) => panic!("existing-content test"),
    }
}
mod fresh;
