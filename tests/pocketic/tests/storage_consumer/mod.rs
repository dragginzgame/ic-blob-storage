//! Real consumer-owned transactions through canonical storage clients.
use super::*;
use blob_test_protocol::consumer::{
    AssetView, Failure as ConsumerFailure, Fault, Recovery, Registration, RegistrationSource,
    Release, Run, Use,
};
use ic_blob_storage_contracts::dto::reference::ReferenceAction;
use ic_blob_storage_contracts::dto::reference::ReferenceChange;
use ic_blob_storage_contracts::dto::reference::ReferenceCommand;
use ic_blob_storage_contracts::dto::reference::ReferenceFailure;
use ic_blob_storage_contracts::dto::reference::ReferenceReceiptLookup;
use ic_blob_storage_contracts::dto::reference::ReferenceUpload;
use ic_blob_storage_pocketic_tests::control_calls::ControlCallExt;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
fn consumer_wasm() -> Vec<u8> {
    std::fs::read(fixture_path("BLOB_CONSUMER_PROBE_WASM")).unwrap()
}
async fn fixture() -> Fixture {
    let f = Fixture::new().await;
    let arguments =
        blob_test_protocol::fixture_wire::encode_args("fixture_init", (f.operator, f.service))
            .unwrap();
    fixture_with_wasm(f, consumer_wasm(), arguments).await
}
async fn fixture_with_wasm(mut f: Fixture, wasm: Vec<u8>, arguments: Vec<u8>) -> Fixture {
    let tenant = f
        .harness
        .pic
        .create_canister_with_settings(
            Some(f.controller),
            Some(CanisterSettings {
                controllers: Some(vec![f.controller]),
                ..CanisterSettings::default()
            }),
        )
        .await
        .unwrap();
    f.harness
        .pic
        .install_canister(tenant, wasm, arguments, Some(f.controller))
        .await
        .unwrap();
    f.tenant = tenant;
    f.enroll(None, true).await.unwrap();
    f
}
async fn input(f: &Fixture, id: u8) -> Run {
    let (permission, manifest) = f.permission(u128::from(id), id);
    f.admit(f.tenant, permission).await.unwrap();
    f.prepare(&manifest).await.unwrap();
    f.expose(permission.request).await.unwrap();
    Run {
        registration: Registration {
            asset: u128::from(id),
            payload: vec![id; 8],
            release_operation: 2,
            source: RegistrationSource::Existing(ReferenceCommand {
                upload: ReferenceUpload {
                    service: f.service.into(),
                    tenant: f.tenant.into(),
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
async fn register(f: &Fixture, input: &Run) -> Result<AssetView, ConsumerFailure> {
    f.harness
        .pic
        .update_control_as(f.tenant, f.operator, "register", input, REPLY_BYTES)
        .await
        .unwrap()
}
async fn asset(f: &Fixture, id: u128) -> Result<AssetView, ConsumerFailure> {
    f.harness
        .pic
        .query_control_as(f.tenant, f.operator, "asset", id, REPLY_BYTES)
        .await
        .unwrap()
}
async fn cancel(f: &Fixture, id: u128) -> Result<AssetView, ConsumerFailure> {
    f.harness
        .pic
        .update_control_as(f.tenant, f.operator, "cancel", id, REPLY_BYTES)
        .await
        .unwrap()
}
async fn release(f: &Fixture, id: u128) -> Result<AssetView, ConsumerFailure> {
    f.harness
        .pic
        .update_control_as(
            f.tenant,
            f.operator,
            "release",
            Release {
                asset: id,
                fault: Fault::None,
            },
            REPLY_BYTES,
        )
        .await
        .unwrap()
}
async fn recover(f: &Fixture, id: u128, release: bool) -> Result<AssetView, ConsumerFailure> {
    f.harness
        .pic
        .update_control_as(
            f.tenant,
            f.operator,
            "recover",
            Recovery { asset: id, release },
            REPLY_BYTES,
        )
        .await
        .unwrap()
}
async fn use_asset(f: &Fixture, attach: bool) -> Result<AssetView, ConsumerFailure> {
    f.harness
        .pic
        .update_control_as(
            f.tenant,
            f.operator,
            "use_asset",
            Use { asset: 1, attach },
            REPLY_BYTES,
        )
        .await
        .unwrap()
}
async fn receipt(f: &Fixture, command: ReferenceCommand) -> ReferenceReceiptLookup {
    let value: Result<ReferenceReceiptLookup, ReferenceFailure> = f
        .harness
        .pic
        .query_blob_as(
            f.service,
            f.tenant,
            "blob_reference_receipt",
            command,
            REPLY_BYTES,
        )
        .await
        .unwrap();
    value.unwrap()
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn consumer_control_ingress_refuses_unusable_packets_before_asset_intent() {
    let f = fixture().await;
    let input = input(&f, 1).await;
    let original = ic_blob_storage_contracts::document::encode(&input).unwrap();
    let mut truncated = original.clone();
    truncated.pop();
    let mut trailing = original.clone();
    trailing.push(0);
    let unknown = ic_blob_storage_contracts::document::encode((&input, true)).unwrap();
    let consumer_before = f.harness.pic.get_stable_memory(f.tenant).await.unwrap();
    let service_before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    for (method, bytes) in [
        ("register", Vec::new()),
        ("resume", vec![0xf6]),
        ("register", truncated),
        ("register", trailing),
        ("register", vec![0; 4097]),
        ("register", unknown),
        (
            "prepare",
            ic_blob_storage_pocketic_tests::retired_packets::candid_scope(),
        ),
    ] {
        let error = f
            .harness
            .pic
            .update_call(f.tenant, f.operator, method, bytes)
            .await
            .unwrap_err();
        assert_eq!(
            crate::canister_rejection(&error).reject_code,
            RejectCode::CanisterError
        );
        assert_eq!(
            f.harness.pic.get_stable_memory(f.tenant).await.unwrap(),
            consumer_before
        );
        assert_eq!(
            f.harness.pic.get_stable_memory(f.service).await.unwrap(),
            service_before
        );
        assert_eq!(asset(&f, 1).await, Err(ConsumerFailure::Unknown));
    }
    assert!(register(&f, &input).await.unwrap().published);
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancellation_and_cleanup_prevent_late_registration_and_new_uses() {
    let f = fixture().await;
    let mut input = input(&f, 1).await;
    input.hold = true;
    let call = f
        .harness
        .pic
        .submit_call(
            f.tenant,
            f.operator,
            "register",
            ic_blob_storage_contracts::document::encode(&input).unwrap(),
        )
        .await
        .unwrap();
    let mut held = false;
    for _ in 0..40 {
        f.harness.pic.tick().await.unwrap();
        let waiting: Result<bool, ConsumerFailure> = f
            .harness
            .pic
            .query_control_as(f.tenant, f.operator, "waiting", (), REPLY_BYTES)
            .await
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
    assert!(!asset(&f, 1).await.unwrap().published);
    assert!(cancel(&f, 1).await.unwrap().cancelled);
    assert_eq!(use_asset(&f, true).await, Err(ConsumerFailure::State));
    assert_eq!(
        release(&f, 1).await.unwrap().release_result,
        Some(Ok(ReferenceChange::Changed))
    );
    let resumed: Result<(), ConsumerFailure> = f
        .harness
        .pic
        .update_control_as(f.tenant, f.operator, "resume", (), REPLY_BYTES)
        .await
        .unwrap();
    resumed.unwrap();
    let result: Result<AssetView, ConsumerFailure> = ic_blob_storage_contracts::document::decode(
        &f.harness.pic.await_call(call).await.unwrap(),
        REPLY_BYTES,
    )
    .unwrap();
    assert_eq!(result, Err(ConsumerFailure::State));
    let state = register(&f, &input).await.unwrap();
    assert!(state.cancelled);
    assert!(!state.published_once);
    assert!(!state.published);
    assert_eq!(
        f.status().await.usage,
        JourneyUsage {
            logical: 10,
            physical: 10,
            liability: 10
        }
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn consumer_traps_preserve_intent_and_recover_without_retain_redispatch() {
    for fault in [Fault::AfterIntent, Fault::AfterRetain, Fault::AfterPublish] {
        let f = fixture().await;
        let mut input = input(&f, 1).await;
        input.fault = fault;
        let error = f
            .harness
            .pic
            .update_call(
                f.tenant,
                f.operator,
                "register",
                ic_blob_storage_contracts::document::encode(&input).unwrap(),
            )
            .await
            .unwrap_err();
        assert_eq!(
            crate::canister_rejection(&error).reject_code,
            RejectCode::CanisterError
        );
        input.fault = Fault::None;
        if fault == Fault::AfterIntent {
            assert_eq!(asset(&f, 1).await, Err(ConsumerFailure::Unknown));
            assert_eq!(
                receipt(&f, retained(&input.registration)).await,
                ReferenceReceiptLookup::Absent
            );
            assert!(register(&f, &input).await.unwrap().published);
        } else if fault == Fault::AfterRetain {
            let pending = asset(&f, 1).await.unwrap();
            assert!(pending.retain_started);
            assert_eq!(pending.retain_result, None);
            cancel(&f, 1).await.unwrap();
            assert_eq!(release(&f, 1).await, Err(ConsumerFailure::State));
            let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
            let recovered = recover(&f, 1, false).await.unwrap();
            assert!(recovered.cancelled);
            assert_eq!(recovered.retain_result, Some(Ok(ReferenceChange::Changed)));
            assert_eq!(
                f.harness.pic.get_stable_memory(f.service).await.unwrap(),
                before
            );
            assert_eq!(
                release(&f, 1).await.unwrap().release_result,
                Some(Ok(ReferenceChange::Changed))
            );
            assert!(!register(&f, &input).await.unwrap().published);
        } else {
            assert!(!asset(&f, 1).await.unwrap().published);
            let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
            let published = register(&f, &input).await.unwrap();
            assert!(published.published);
            assert_eq!(register(&f, &input).await, Ok(published));
            assert_eq!(
                f.harness.pic.get_stable_memory(f.service).await.unwrap(),
                before
            );
        }
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn release_acknowledgment_trap_preserves_tombstone_and_recovery_evidence_through_upgrade() {
    let f = fixture().await;
    let input = input(&f, 1).await;
    assert!(register(&f, &input).await.unwrap().published);
    assert_eq!(use_asset(&f, true).await.unwrap().uses, 1);
    assert_eq!(cancel(&f, 1).await, Err(ConsumerFailure::State));
    use_asset(&f, false).await.unwrap();
    cancel(&f, 1).await.unwrap();
    let error = f
        .harness
        .pic
        .update_call(
            f.tenant,
            f.operator,
            "release",
            ic_blob_storage_contracts::document::encode(Release {
                asset: 1,
                fault: Fault::AfterRelease,
            })
            .unwrap(),
        )
        .await
        .unwrap_err();
    assert_eq!(
        crate::canister_rejection(&error).reject_code,
        RejectCode::CanisterError
    );
    let pending = asset(&f, 1).await.unwrap();
    assert!(pending.release_started);
    assert_eq!(pending.release_result, None);
    assert!(pending.cancelled && pending.published_once && !pending.published);
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(
        recover(&f, 1, true).await.unwrap().release_result,
        Some(Ok(ReferenceChange::Changed))
    );
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    let recovered = asset(&f, 1).await.unwrap();
    f.harness
        .pic
        .upgrade_canister(
            f.tenant,
            consumer_wasm(),
            blob_test_protocol::fixture_wire::encode_args("fixture_post_upgrade", ()).unwrap(),
            Some(f.controller),
        )
        .await
        .unwrap();
    assert_eq!(asset(&f, 1).await, Ok(recovered));
    assert_eq!(register(&f, &input).await, Err(ConsumerFailure::Fenced));
    assert_eq!(release(&f, 1).await, Err(ConsumerFailure::Fenced));
    assert_eq!(recover(&f, 1, true).await, Err(ConsumerFailure::Fenced));
    assert_eq!(use_asset(&f, true).await, Err(ConsumerFailure::Fenced));
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn bounded_consumer_history_rejects_changed_payload_and_preserves_cleanup_at_capacity() {
    let f = fixture().await;
    let first = input(&f, 1).await;
    let second = input(&f, 2).await;
    register(&f, &first).await.unwrap();
    register(&f, &second).await.unwrap();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let mut changed = first.clone();
    changed.registration.payload.push(9);
    assert_eq!(register(&f, &changed).await, Err(ConsumerFailure::Conflict));
    let mut third = first.clone();
    third.registration.asset = 3;
    let mut command = retained(&third.registration);
    command.reference = 3;
    command.operation = 3;
    third.registration.source = RegistrationSource::Existing(command);
    third.registration.release_operation = 4;
    assert_eq!(register(&f, &third).await, Err(ConsumerFailure::Capacity));
    let denied: Result<AssetView, ConsumerFailure> = f
        .harness
        .pic
        .update_control_as(f.tenant, f.other, "register", &first, REPLY_BYTES)
        .await
        .unwrap();
    assert_eq!(denied, Err(ConsumerFailure::Denied));
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    for id in [1, 2] {
        cancel(&f, id).await.unwrap();
        assert_eq!(
            release(&f, id).await.unwrap().release_result,
            Some(Ok(ReferenceChange::Changed))
        );
    }
    assert_eq!(register(&f, &third).await, Err(ConsumerFailure::Capacity));
    assert!(register(&f, &first).await.unwrap().cancelled);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn upgrade_keeps_uncertain_retain_and_tombstone_without_authorizing_recovery_or_resend() {
    let f = fixture().await;
    let mut input = input(&f, 1).await;
    input.fault = Fault::AfterRetain;
    let error = f
        .harness
        .pic
        .update_call(
            f.tenant,
            f.operator,
            "register",
            ic_blob_storage_contracts::document::encode(&input).unwrap(),
        )
        .await
        .unwrap_err();
    assert_eq!(
        crate::canister_rejection(&error).reject_code,
        RejectCode::CanisterError
    );
    let pending = cancel(&f, 1).await.unwrap();
    assert!(pending.retain_started && pending.cancelled);
    assert_eq!(pending.retain_result, None);
    assert!(matches!(
        receipt(&f, retained(&input.registration)).await,
        ReferenceReceiptLookup::Found(_)
    ));
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    f.harness
        .pic
        .upgrade_canister(
            f.tenant,
            consumer_wasm(),
            blob_test_protocol::fixture_wire::encode_args("fixture_post_upgrade", ()).unwrap(),
            Some(f.controller),
        )
        .await
        .unwrap();
    assert_eq!(asset(&f, 1).await, Ok(pending));
    assert_eq!(recover(&f, 1, false).await, Err(ConsumerFailure::Fenced));
    assert_eq!(register(&f, &input).await, Err(ConsumerFailure::Fenced));
    assert_eq!(release(&f, 1).await, Err(ConsumerFailure::Fenced));
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
}

fn retained(intent: &Registration) -> ReferenceCommand {
    match intent.source {
        RegistrationSource::Existing(command) => command,
        RegistrationSource::Fresh(_) => panic!("existing-content test"),
    }
}
mod fresh;

fn pre_trait_consumer_wasm() -> Vec<u8> {
    let old = std::fs::read(fixture_path("BLOB_PRE_TRAIT_CONSUMER_WASM")).unwrap();
    let expected =
        std::env::var("BLOB_PRE_TRAIT_CONSUMER_SHA256").expect("explicit pre-cut artifact hash");
    assert_eq!(
        ic_blob_storage_contracts::identity::ContentDigest::compute(&old).to_string(),
        expected,
    );
    assert_ne!(old, consumer_wasm(), "must cross the actual projection cut");
    old
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "Requires retained pre-trait consumer Wasm and its explicit SHA-256"]
async fn retired_consumer_upgrade_refuses_without_losing_published_or_cancelled_history() {
    let f = Fixture::new().await;
    // The pinned old image still owns its original Candid initializer.
    let arguments = ic_platform::candid::contract::encode_args((f.operator, f.service)).unwrap();
    let f = fixture_with_wasm(f, pre_trait_consumer_wasm(), arguments).await;
    let mut first = input(&f, 1).await;
    first.registration.asset = u128::MAX;
    first.registration.release_operation = u128::MAX;
    let second = input(&f, 2).await;
    assert!(register(&f, &first).await.unwrap().published);
    assert!(register(&f, &second).await.unwrap().published);
    cancel(&f, 2).await.unwrap();
    let before = [
        asset(&f, u128::MAX).await.unwrap(),
        asset(&f, 2).await.unwrap(),
    ];
    let service_before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let consumer_before = f.harness.pic.get_stable_memory(f.tenant).await.unwrap();
    crate::install_code::settle_install_code_debt(&f.harness.pic).await;
    let error = f
        .harness
        .pic
        .upgrade_canister(
            f.tenant,
            consumer_wasm(),
            blob_test_protocol::fixture_wire::encode_args("fixture_post_upgrade", ()).unwrap(),
            Some(f.controller),
        )
        .await
        .unwrap_err();
    assert_eq!(
        crate::canister_rejection(&error).reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(
        f.harness.pic.get_stable_memory(f.tenant).await.unwrap(),
        consumer_before
    );
    assert_eq!(
        [
            asset(&f, u128::MAX).await.unwrap(),
            asset(&f, 2).await.unwrap()
        ],
        before
    );
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        service_before
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "Requires retained pre-trait consumer Wasm and its explicit SHA-256"]
async fn retired_consumer_upgrade_refuses_without_losing_uncertain_retains() {
    let f = Fixture::new().await;
    let arguments = ic_platform::candid::contract::encode_args((f.operator, f.service)).unwrap();
    let f = fixture_with_wasm(f, pre_trait_consumer_wasm(), arguments).await;
    let mut command = input(&f, 1).await;
    command.fault = Fault::AfterRetain;
    assert!(
        f.harness
            .pic
            .update_call(
                f.tenant,
                f.operator,
                "register",
                ic_blob_storage_contracts::document::encode(&command).unwrap(),
            )
            .await
            .is_err()
    );
    let pending = cancel(&f, 1).await.unwrap();
    assert!(pending.retain_started && pending.cancelled);
    assert_eq!(pending.retain_result, None);
    assert!(matches!(
        receipt(&f, retained(&command.registration)).await,
        ReferenceReceiptLookup::Found(_)
    ));
    let service_before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let consumer_before = f.harness.pic.get_stable_memory(f.tenant).await.unwrap();
    crate::install_code::settle_install_code_debt(&f.harness.pic).await;
    let error = f
        .harness
        .pic
        .upgrade_canister(
            f.tenant,
            consumer_wasm(),
            blob_test_protocol::fixture_wire::encode_args("fixture_post_upgrade", ()).unwrap(),
            Some(f.controller),
        )
        .await
        .unwrap_err();
    assert_eq!(
        crate::canister_rejection(&error).reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(
        f.harness.pic.get_stable_memory(f.tenant).await.unwrap(),
        consumer_before
    );
    assert_eq!(asset(&f, 1).await, Ok(pending));
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        service_before
    );
}
