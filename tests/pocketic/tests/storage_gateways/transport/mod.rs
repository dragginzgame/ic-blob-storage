//! Real IC scheduling against an explicitly different local source method.
use ic_blob_storage_pocketic_tests::fixture_calls;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;

use super::*;
use blob_test_protocol::{
    SourceMode, SourceObservation, source::SourceRecoveryView, storage::gateways::TransportInput,
};
impl Fixture {
    async fn with_gateway_source() -> Self {
        let harness = Harness::new().await;
        let source = harness.pic.create_canister().await.unwrap();
        // Existing fixture binds operator and Cashier to the same explicit input.
        // PocketIC acts for that operator; source controls remain driver-only.
        let f = Self::with_operator(harness, source).await;
        f.harness
            .pic
            .install_canister(
                source,
                std::fs::read(fixture_path("BLOB_GATEWAY_SOURCE_WASM")).unwrap(),
                blob_test_protocol::fixture_wire::encode_args(
                    "fixture_init",
                    (f.service, f.other, f.controller),
                )
                .unwrap(),
                None,
            )
            .await
            .unwrap();
        f
    }
    async fn source_mode(&self, mode: SourceMode) {
        assert!(
            fixture_calls::update_fixture::<(bool,), _>(
                &self.harness.pic,
                self.operator,
                self.controller,
                "configure",
                (mode,)
            )
            .await
            .map(|(reply,)| reply)
            .unwrap()
        );
    }
    async fn source_requests(&self) -> u64 {
        fixture_calls::query_fixture::<(Option<SourceObservation>,), _>(
            &self.harness.pic,
            self.operator,
            self.controller,
            "observation",
            (),
        )
        .await
        .map(|(reply,)| reply)
        .unwrap()
        .unwrap()
        .requests
    }
    fn transport_input(&self, token: u64) -> TransportInput {
        TransportInput {
            scope: self.gateway_scope(),
            token,
            callback_fault: false,
        }
    }
    async fn query_gateways(&self, actor: Principal, token: u64) -> Result<(), Failure> {
        self.harness
            .pic
            .update_control_as(
                self.service,
                actor,
                "fixture_gateway_transport",
                (self.transport_input(token),),
                REPLY_BYTES,
            )
            .await
            .map(|(reply,)| reply)
            .unwrap()
    }
    async fn wait_source(&self) {
        for _ in 0..30 {
            self.harness.pic.tick().await.unwrap();
            let view: Option<SourceRecoveryView> = fixture_calls::query_fixture(
                &self.harness.pic,
                self.operator,
                self.controller,
                "recovery_observation",
                (),
            )
            .await
            .map(|(reply,)| reply)
            .unwrap();
            if view.unwrap().held_sync.is_some() {
                return;
            }
        }
        panic!("source did not hold its reply");
    }
    async fn resume_source(&self) {
        assert!(
            fixture_calls::update_fixture::<(bool,), _>(
                &self.harness.pic,
                self.operator,
                self.controller,
                "resume_sync",
                ()
            )
            .await
            .map(|(reply,)| reply)
            .unwrap()
        );
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn durable_gateway_transport_preserves_failed_attempts_and_refuses_unsendable_requests() {
    let f = Fixture::with_gateway_source().await;
    let old = f.gateway_begin().await;
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    for actor in [f.controller, f.tenant, Principal::anonymous()] {
        assert_eq!(f.query_gateways(actor, old).await, Err(Failure::Denied));
    }
    assert_eq!(f.source_requests().await, 0);
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    for (mode, expected) in [
        (SourceMode::Reject, Failure::Transport),
        (SourceMode::Malformed, Failure::Invalid),
        (SourceMode::Oversized, Failure::Capacity),
        (SourceMode::Empty, Failure::Invalid),
    ] {
        f.source_mode(mode).await;
        assert_eq!(f.query_gateways(f.operator, old).await, Err(expected));
        assert_eq!(
            f.harness.pic.get_stable_memory(f.service).await.unwrap(),
            before
        );
    }
    f.cancel_gateway_sync(f.operator, old).await.unwrap();
    let requests = f.source_requests().await;
    assert_eq!(
        f.query_gateways(f.operator, old).await,
        Err(Failure::Conflict)
    );
    assert_eq!(f.source_requests().await, requests);
    let current = f.gateway_begin().await;
    f.source_mode(SourceMode::Valid).await;
    f.query_gateways(f.operator, current).await.unwrap();
    assert_eq!(
        f.gateway_view().await.members,
        vec![ic_platform::Principal::from(f.other)]
    );
    assert_eq!(f.gateway_view().await.pending_sequence, None);
    assert_eq!(
        f.query_gateways(f.operator, current).await,
        Err(Failure::Conflict)
    );
    assert_eq!(f.source_requests().await, requests + 1);
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn delayed_gateway_reply_cannot_overwrite_edits_or_a_newer_pending_attempt() {
    for cancel in [false, true] {
        let f = Fixture::with_gateway_source().await;
        f.gateways(f.operator, Action::Add(f.other.into()))
            .await
            .unwrap();
        let old = f.gateway_begin().await;
        f.source_mode(SourceMode::Hold).await;
        let call = f
            .harness
            .pic
            .submit_call(
                f.service,
                f.operator,
                "fixture_gateway_transport",
                ic_blob_storage_pocketic_tests::control_calls::encode_control_one(
                    f.transport_input(old),
                )
                .unwrap(),
            )
            .await
            .unwrap();
        f.wait_source().await;
        let replacement = f.gateway_begin().await;
        assert_ne!(replacement, old);
        assert_eq!(f.gateway_view().await.pending_sequence, Some(replacement));
        assert_eq!(
            f.cancel_gateway_sync(f.operator, old).await,
            Err(GatewaySyncFailure::Conflict)
        );
        if cancel {
            f.cancel_gateway_sync(f.operator, replacement)
                .await
                .unwrap();
        } else {
            f.revoke_gateway(f.operator, f.other).await.unwrap();
        }
        let current = f.gateway_begin().await;
        let pending = f.gateway_view().await;
        f.resume_source().await;
        let result: Result<(), Failure> =
            ic_blob_storage_pocketic_tests::control_calls::decode_control_one(
                &f.harness.pic.await_call(call).await.unwrap(),
                4 * 1024 * 1024,
            )
            .unwrap();
        assert_eq!(result, Err(Failure::Conflict));
        assert_eq!(f.gateway_view().await, pending);
        assert_eq!(
            pending.members,
            if cancel {
                vec![ic_platform::Principal::from(f.other)]
            } else {
                vec![]
            }
        );
        f.source_mode(SourceMode::Valid).await;
        f.query_gateways(f.operator, current).await.unwrap();
        assert_eq!(
            f.gateway_view().await.members,
            vec![ic_platform::Principal::from(f.other)]
        );
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn delayed_gateway_reply_cannot_overwrite_a_completed_replacement() {
    let f = Fixture::with_gateway_source().await;
    let old = f.gateway_begin().await;
    f.source_mode(SourceMode::Hold).await;
    let call = f
        .harness
        .pic
        .submit_call(
            f.service,
            f.operator,
            "fixture_gateway_transport",
            ic_blob_storage_pocketic_tests::control_calls::encode_control_one(
                f.transport_input(old),
            )
            .unwrap(),
        )
        .await
        .unwrap();
    f.wait_source().await;
    f.revoke_gateway(f.operator, f.other).await.unwrap();
    let current = f.gateway_begin().await;
    f.gateways(
        f.operator,
        Action::Apply {
            token: current,
            source: f.gateway_scope(),
            reply: gateway_reply(&[f.tenant]),
        },
    )
    .await
    .unwrap();
    let completed = f.gateway_view().await;
    f.resume_source().await;
    let result: Result<(), Failure> =
        ic_blob_storage_pocketic_tests::control_calls::decode_control_one(
            &f.harness.pic.await_call(call).await.unwrap(),
            4 * 1024 * 1024,
        )
        .unwrap();
    assert_eq!(result, Err(Failure::Conflict));
    assert_eq!(f.gateway_view().await, completed);
    assert_eq!(
        completed.members,
        vec![ic_platform::Principal::from(f.tenant)]
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn gateway_callback_write_trap_keeps_pending_state_and_restoration_blocks_resending() {
    let f = Fixture::with_gateway_source().await;
    f.gateways(f.operator, Action::Add(f.tenant.into()))
        .await
        .unwrap();
    let token = f.gateway_begin().await;
    let before = f.gateway_view().await;
    let error = f
        .harness
        .pic
        .update_call(
            f.service,
            f.operator,
            "fixture_gateway_transport",
            ic_blob_storage_pocketic_tests::control_calls::encode_control_one(TransportInput {
                callback_fault: true,
                ..f.transport_input(token)
            })
            .unwrap(),
        )
        .await
        .unwrap_err();
    assert_eq!(
        crate::canister_rejection(&error).reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(f.source_requests().await, 1);
    assert_eq!(f.gateway_view().await, before);
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation("fixture_post_upgrade", f.operator),
            Some(f.controller),
        )
        .await
        .unwrap();
    assert_eq!(
        f.query_gateways(f.operator, token).await,
        Err(Failure::Fenced)
    );
    assert_eq!(f.source_requests().await, 1);
    assert_eq!(
        f.gateway_view().await,
        View {
            fenced: true,
            ..before
        }
    );
}

mod replicated;
