//! Real IC scheduling against an explicitly different local source method.
use super::*;
use blob_test_protocol::{
    SourceMode, SourceObservation, source::SourceRecoveryView, storage::gateways::TransportInput,
};
impl Fixture {
    fn with_gateway_source() -> Self {
        let harness = Harness::new();
        let source = harness.pic.create_canister();
        // Existing fixture binds operator and Cashier to the same explicit input.
        // PocketIC acts for that operator; source controls remain driver-only.
        let f = Self::with_operator(harness, source);
        f.harness.pic.install_canister(
            source,
            std::fs::read(fixture_path("BLOB_GATEWAY_SOURCE_WASM")).unwrap(),
            candid::encode_args((f.service, f.other, f.controller)).unwrap(),
            None,
        );
        f
    }
    fn source_mode(&self, mode: SourceMode) {
        assert!(
            self.harness
                .pic
                .update_candid_as::<bool, _>(self.operator, self.controller, "configure", (mode,))
                .unwrap()
        );
    }
    fn source_requests(&self) -> u64 {
        self.harness
            .pic
            .query_candid_as::<Option<SourceObservation>, _>(
                self.operator,
                self.controller,
                "observation",
                (),
            )
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
    fn query_gateways(&self, actor: Principal, token: u64) -> Result<(), Failure> {
        self.harness
            .pic
            .update_candid_as(
                self.service,
                actor,
                "fixture_gateway_transport",
                (self.transport_input(token),),
            )
            .unwrap()
    }
    fn wait_source(&self) {
        for _ in 0..30 {
            self.harness.pic.tick();
            let view: Option<SourceRecoveryView> = self
                .harness
                .pic
                .query_candid_as(self.operator, self.controller, "recovery_observation", ())
                .unwrap();
            if view.unwrap().held_sync.is_some() {
                return;
            }
        }
        panic!("source did not hold its reply");
    }
    fn resume_source(&self) {
        assert!(
            self.harness
                .pic
                .update_candid_as::<bool, _>(self.operator, self.controller, "resume_sync", ())
                .unwrap()
        );
    }
}
#[test]
fn durable_gateway_transport_preserves_failed_attempts_and_refuses_unsendable_requests() {
    let f = Fixture::with_gateway_source();
    let old = f.gateway_begin();
    let before = f.harness.pic.get_stable_memory(f.service);
    for actor in [f.controller, f.tenant, Principal::anonymous()] {
        assert_eq!(f.query_gateways(actor, old), Err(Failure::Denied));
    }
    assert_eq!(f.source_requests(), 0);
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    for (mode, expected) in [
        (SourceMode::Reject, Failure::Transport),
        (SourceMode::Malformed, Failure::Invalid),
        (SourceMode::Oversized, Failure::Capacity),
        (SourceMode::Empty, Failure::Invalid),
    ] {
        f.source_mode(mode);
        assert_eq!(f.query_gateways(f.operator, old), Err(expected));
        assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    }
    f.gateways(f.operator, Action::Cancel(old)).unwrap();
    let requests = f.source_requests();
    assert_eq!(f.query_gateways(f.operator, old), Err(Failure::Conflict));
    assert_eq!(f.source_requests(), requests);
    let current = f.gateway_begin();
    f.source_mode(SourceMode::Valid);
    f.query_gateways(f.operator, current).unwrap();
    assert_eq!(f.gateway_view().members, vec![f.other]);
    assert_eq!(f.gateway_view().pending_sequence, None);
    assert_eq!(
        f.query_gateways(f.operator, current),
        Err(Failure::Conflict)
    );
    assert_eq!(f.source_requests(), requests + 1);
}
#[test]
fn delayed_gateway_reply_cannot_overwrite_edits_or_a_newer_pending_attempt() {
    let f = Fixture::with_gateway_source();
    f.gateways(f.operator, Action::Add(f.other)).unwrap();
    let old = f.gateway_begin();
    f.source_mode(SourceMode::Hold);
    let call = f
        .harness
        .pic
        .submit_call(
            f.service,
            f.operator,
            "fixture_gateway_transport",
            candid::encode_one(f.transport_input(old)).unwrap(),
        )
        .unwrap();
    f.wait_source();
    assert_eq!(f.gateways(f.operator, Action::Begin), Err(Failure::Phase));
    assert_eq!(f.gateway_view().pending_sequence, Some(1));
    f.gateways(f.operator, Action::Remove(f.other)).unwrap();
    let current = f.gateway_begin();
    let pending = f.gateway_view();
    f.resume_source();
    let result: Result<(), Failure> =
        candid::decode_one(&f.harness.pic.await_call(call).unwrap()).unwrap();
    assert_eq!(result, Err(Failure::Conflict));
    assert_eq!(f.gateway_view(), pending);
    assert!(pending.members.is_empty());
    f.source_mode(SourceMode::Valid);
    f.query_gateways(f.operator, current).unwrap();
    assert_eq!(f.gateway_view().members, vec![f.other]);
}
#[test]
fn delayed_gateway_reply_cannot_overwrite_a_completed_replacement() {
    let f = Fixture::with_gateway_source();
    let old = f.gateway_begin();
    f.source_mode(SourceMode::Hold);
    let call = f
        .harness
        .pic
        .submit_call(
            f.service,
            f.operator,
            "fixture_gateway_transport",
            candid::encode_one(f.transport_input(old)).unwrap(),
        )
        .unwrap();
    f.wait_source();
    f.gateways(f.operator, Action::Remove(f.other)).unwrap();
    let current = f.gateway_begin();
    f.gateways(
        f.operator,
        Action::Apply {
            token: current,
            source: f.gateway_scope(),
            reply: gateway_reply(&[f.tenant]),
        },
    )
    .unwrap();
    let completed = f.gateway_view();
    f.resume_source();
    let result: Result<(), Failure> =
        candid::decode_one(&f.harness.pic.await_call(call).unwrap()).unwrap();
    assert_eq!(result, Err(Failure::Conflict));
    assert_eq!(f.gateway_view(), completed);
    assert_eq!(completed.members, vec![f.tenant]);
}
#[test]
fn gateway_callback_write_trap_keeps_pending_state_and_restoration_blocks_resending() {
    let f = Fixture::with_gateway_source();
    f.gateways(f.operator, Action::Add(f.tenant)).unwrap();
    let token = f.gateway_begin();
    let before = f.gateway_view();
    let error = f
        .harness
        .pic
        .update_call(
            f.service,
            f.operator,
            "fixture_gateway_transport",
            candid::encode_one(TransportInput {
                callback_fault: true,
                ..f.transport_input(token)
            })
            .unwrap(),
        )
        .unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    assert_eq!(f.source_requests(), 1);
    assert_eq!(f.gateway_view(), before);
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            candid::encode_one(f.operator).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(f.query_gateways(f.operator, token), Err(Failure::Fenced));
    assert_eq!(f.source_requests(), 1);
    assert_eq!(
        f.gateway_view(),
        View {
            fenced: true,
            ..before
        }
    );
}
