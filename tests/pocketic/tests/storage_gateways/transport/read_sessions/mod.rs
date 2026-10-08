//! Real local chunks validate durable sessions, verification and callback authority.
mod resources;
use super::*;
use blob_test_protocol::{
    admission::{
        ContentLookup,
        input::{ReferenceInput, RetainedDescriptorInput},
    },
    journey::readback::{
        JourneyReadChunk, ReadSourceConfig, ReadSourceMode, ReadSourceObservation,
    },
    storage::{ProviderFact, gateways::ReadSessionInput},
};
impl Fixture {
    fn chunk_mode(&self, mode: ReadSourceMode, hold: bool) {
        let config = ReadSourceConfig {
            root: self.permission(1, 1).0.request.root,
            index: 0,
            bytes: vec![1; 10],
            mode,
            hold,
        };
        assert!(
            self.harness
                .pic
                .update_candid_as::<bool, _>(
                    self.operator,
                    self.controller,
                    "configure_read",
                    (config,)
                )
                .unwrap()
        );
    }
    fn chunk_observation(&self) -> ReadSourceObservation {
        self.harness
            .pic
            .query_candid_as::<Option<ReadSourceObservation>, _>(
                self.operator,
                self.controller,
                "read_observation",
                (),
            )
            .unwrap()
            .unwrap()
    }
    fn wait_chunk(&self) {
        for _ in 0..30 {
            self.harness.pic.tick();
            if self.chunk_observation().waiting {
                return;
            }
        }
        panic!("source did not hold its chunk");
    }
    fn resume_chunk(&self) {
        assert!(
            self.harness
                .pic
                .update_candid_as::<bool, _>(self.operator, self.controller, "resume_read", ())
                .unwrap()
        );
    }
    fn read_input(&self) -> ReadSessionInput {
        self.enroll(None, true).unwrap();
        let (permission, preparation) = self.permission(1, 1);
        self.admit(self.tenant, permission).unwrap();
        self.prepare(&preparation).unwrap();
        self.expose(permission.request).unwrap();
        self.fact(permission.request, ProviderFact::Uploaded)
            .unwrap();
        self.gateways(self.operator, Action::Add(self.operator))
            .unwrap();
        self.chunk_mode(ReadSourceMode::Valid, false);
        ReadSessionInput {
            index: 0,
            admit_fault: None,
            callback_fault: None,
            cashier: self.operator,
            gateway: self.operator,
            target: RetainedDescriptorInput {
                content: ContentLookup {
                    service: self.service,
                    tenant: self.tenant,
                    namespace: 1,
                    root: permission.request.root,
                },
                object: 1,
                incarnation: 1,
                reference: 1,
            },
        }
    }
    fn read_chunk(&self, actor: Principal, input: ReadSessionInput) -> Result<(), Failure> {
        self.harness
            .pic
            .update_candid_as::<Result<JourneyReadChunk, Failure>, _>(
                self.service,
                actor,
                "fixture_read_chunk",
                (input,),
            )
            .unwrap()
            .map(|chunk| {
                assert_eq!(
                    chunk,
                    JourneyReadChunk {
                        index: input.index,
                        offset: 0,
                        bytes: vec![1; 10]
                    }
                );
            })
    }
    fn session_view(&self) -> blob_test_protocol::storage::gateways::ReadSessionsView {
        self.harness.pic.query_candid_as::<Result<blob_test_protocol::storage::gateways::ReadSessionsView, Failure>, _>(self.service, self.operator, "read_sessions", ()).unwrap().unwrap()
    }
}
#[test]
fn read_authority_rejects_restored_instances_and_wrong_callers_before_sending() {
    let f = Fixture::with_gateway_source();
    let input = f.read_input();
    for actor in [
        f.operator,
        f.controller,
        f.uploader,
        f.other,
        Principal::anonymous(),
    ] {
        assert_eq!(f.read_chunk(actor, input), Err(Failure::Denied));
    }
    let mut wrong = input;
    wrong.target.incarnation = 2;
    assert_eq!(f.read_chunk(f.tenant, wrong), Err(Failure::Unknown));
    assert_eq!(f.chunk_observation().requests, 0);
    let before = f.status();
    assert_eq!(f.session_view().last_sequence, 0);
    assert_eq!(f.read_chunk(f.tenant, input), Ok(()));
    assert_eq!(f.status(), before);
    assert_eq!(f.session_view().sessions, 0);
    assert_eq!(f.session_view().reserved_bytes, 0);
    assert_eq!(f.session_view().last_sequence, 1);
    f.chunk_mode(ReadSourceMode::Reject, false);
    assert_eq!(f.read_chunk(f.tenant, input), Err(Failure::Transport));
    assert_eq!(f.session_view().sessions, 0);
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation(f.operator),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(f.read_chunk(f.tenant, input), Err(Failure::Fenced));
    assert_eq!(f.chunk_observation().requests, 2);
}
#[test]
fn held_read_authority_rejects_remove_readd_and_tenant_reactivation() {
    let f = Fixture::with_gateway_source();
    let input = f.read_input();
    for reactivate_tenant in [false, true] {
        f.chunk_mode(ReadSourceMode::Valid, true);
        let call = f
            .harness
            .pic
            .submit_call(
                f.service,
                f.tenant,
                "fixture_read_chunk",
                candid::encode_one(input).unwrap(),
            )
            .unwrap();
        f.wait_chunk();
        assert_eq!(f.read_chunk(f.tenant, input), Err(Failure::Capacity));
        assert_eq!(f.session_view().sessions, 1);
        assert_eq!(f.session_view().reserved_bytes, 2048);
        if reactivate_tenant {
            f.enroll(Some(f.tenant().unwrap()), false).unwrap();
            f.enroll(Some(f.tenant().unwrap()), true).unwrap();
        } else {
            let occupancy = f.session_view();
            let accounting = f.local_status();
            assert!(f.revoke_gateway(f.operator, f.operator).unwrap().removed);
            assert_eq!(f.session_view(), occupancy);
            let after = f.local_status();
            assert_eq!(after.uploads, accounting.uploads);
            assert_eq!(after.funding, accounting.funding);
            assert_eq!(after.reads, accounting.reads);
            f.gateways(f.operator, Action::Add(f.operator)).unwrap();
        }
        f.resume_chunk();
        let result: Result<JourneyReadChunk, Failure> =
            candid::decode_one(&f.harness.pic.await_call(call).unwrap()).unwrap();
        assert_eq!(result.map(|_| ()), Err(Failure::Conflict));
        f.chunk_mode(ReadSourceMode::Valid, false);
        assert_eq!(f.read_chunk(f.tenant, input), Ok(()));
    }
}
#[test]
fn held_read_authority_rejects_exact_reference_release_despite_another_live_reference() {
    let f = Fixture::with_gateway_source();
    let input = f.read_input();
    let request = f.permission(1, 1).0.request;
    f.reference(ReferenceInput {
        object: request,
        reference: 2,
        operation: 1,
        retain: true,
    })
    .unwrap();
    f.chunk_mode(ReadSourceMode::Valid, true);
    let call = f
        .harness
        .pic
        .submit_call(
            f.service,
            f.tenant,
            "fixture_read_chunk",
            candid::encode_one(input).unwrap(),
        )
        .unwrap();
    f.wait_chunk();
    f.reference(ReferenceInput {
        object: request,
        reference: 1,
        operation: 2,
        retain: false,
    })
    .unwrap();
    f.resume_chunk();
    let result: Result<JourneyReadChunk, Failure> =
        candid::decode_one(&f.harness.pic.await_call(call).unwrap()).unwrap();
    assert_eq!(result.map(|_| ()), Err(Failure::Unknown));
    f.chunk_mode(ReadSourceMode::Valid, false);
    let mut other = input;
    other.target.reference = 2;
    assert_eq!(f.read_chunk(f.tenant, other), Ok(()));
}
#[test]
fn gateway_write_rollback_preserves_read_authority_but_same_list_sync_invalidates_it() {
    let f = Fixture::with_gateway_source();
    let input = f.read_input();
    for commit in [false, true] {
        f.chunk_mode(ReadSourceMode::Valid, true);
        let call = f
            .harness
            .pic
            .submit_call(
                f.service,
                f.tenant,
                "fixture_read_chunk",
                candid::encode_one(input).unwrap(),
            )
            .unwrap();
        f.wait_chunk();
        if commit {
            let token = f.gateway_begin();
            f.gateways(
                f.operator,
                Action::Apply {
                    token,
                    source: f.gateway_scope(),
                    reply: gateway_reply(&[f.operator]),
                },
            )
            .unwrap();
        } else {
            f.revoke_gateway_trap(f.operator);
        }
        f.resume_chunk();
        let result: Result<JourneyReadChunk, Failure> =
            candid::decode_one(&f.harness.pic.await_call(call).unwrap()).unwrap();
        assert_eq!(
            result.map(|_| ()),
            if commit {
                Err(Failure::Conflict)
            } else {
                Ok(())
            }
        );
    }
}

#[test]
fn interrupted_read_admission_rolls_back_all_counters_and_sends_nothing() {
    use blob_test_protocol::storage::WriteFault;
    let f = Fixture::with_gateway_source();
    let input = f.read_input();
    let before = f.harness.pic.get_stable_memory(f.service);
    for fault in [
        WriteFault::ReadSessions,
        WriteFault::ReadTenants,
        WriteFault::ReadJournal,
    ] {
        let failure = f
            .harness
            .pic
            .update_call(
                f.service,
                f.tenant,
                "fixture_read_chunk",
                candid::encode_one(ReadSessionInput {
                    admit_fault: Some(fault),
                    ..input
                })
                .unwrap(),
            )
            .unwrap_err();
        assert_eq!(failure.reject_code, RejectCode::CanisterError);
        assert_eq!(f.chunk_observation().requests, 0);
        assert_eq!(f.session_view().last_sequence, 0);
        assert_eq!(f.session_view().sessions, 0);
        assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    }
    assert_eq!(f.read_chunk(f.tenant, input), Ok(()));
}
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One restore preserves four owners and checks their shared operator snapshot"
)]
fn upgrade_preserves_upload_funding_gateway_and_interrupted_read_obligations() {
    use blob_test_protocol::storage::funding::{Action as FundingAction, Allocation};
    let f = Fixture::with_gateway_source();
    let input = f.read_input();
    let failure = f
        .harness
        .pic
        .update_call(
            f.service,
            f.tenant,
            "fixture_read_chunk",
            candid::encode_one(ReadSessionInput {
                callback_fault: Some(WriteFault::ReadJournal),
                ..input
            })
            .unwrap(),
        )
        .unwrap_err();
    assert_eq!(failure.reject_code, RejectCode::CanisterError);
    assert_eq!(f.chunk_observation().requests, 1);
    let pending = f.session_view();
    assert_eq!(pending.sessions, 1);
    assert_eq!(pending.reserved_bytes, 2048);
    assert_eq!(pending.last_sequence, 1);
    assert_eq!(f.read_chunk(f.tenant, input), Err(Failure::Capacity));

    // One restore must preserve every owner's obligations together. Funding here
    // is a labelled local journal substitute; it sends no provider payment.
    let intent = f.funding_intent(u128::MAX, 900);
    f.funding(f.operator, intent, FundingAction::Prepare)
        .unwrap();
    f.funding(f.operator, intent, FundingAction::Attempt)
        .unwrap();
    let funding_before = f.funding_allocation();
    let token = f.gateway_begin();
    let gateways_before = f.gateway_view();
    let uploads_before = f.status();
    let stable_before = f.harness.pic.get_stable_memory(f.service);
    let local_before = f.local_status();
    assert_eq!(local_before.funding.reserved_or_uncertain, 900);
    assert_eq!(local_before.funding.available_allocation, 100);
    assert_eq!(local_before.funding.attachment_allowance, 0);
    assert_eq!(local_before.funding.last_operation, Some(u128::MAX));
    assert_eq!(local_before.funding.retained_intents, 1);
    assert_eq!(local_before.reads.sessions, 1);
    assert_eq!(local_before.reads.reserved_bytes, 2048);
    assert_eq!(local_before.gateways.members, gateways_before.members);
    assert_eq!(
        local_before.gateways.pending_sequence,
        gateways_before.pending_sequence
    );
    assert_eq!(
        local_before.uploads.liability_bytes,
        uploads_before.usage.liability
    );
    assert!(
        f.harness
            .pic
            .get_stable_memory(f.service)
            .eq(&stable_before),
        "inspection changed stable bytes"
    );
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation(f.operator),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(
        f.session_view(),
        blob_test_protocol::storage::gateways::ReadSessionsView {
            fenced: true,
            ..pending
        }
    );
    assert_eq!(f.read_chunk(f.tenant, input), Err(Failure::Fenced));
    assert_eq!(f.chunk_observation().requests, 1);
    assert_eq!(
        f.funding_allocation(),
        Allocation {
            fenced: true,
            ..funding_before
        }
    );
    assert_eq!(
        f.funding(f.operator, intent, FundingAction::Callback(900)),
        Err(Failure::Fenced)
    );
    assert_eq!(
        f.gateway_view(),
        View {
            fenced: true,
            ..gateways_before
        }
    );
    assert_eq!(
        f.cancel_gateway_sync(f.operator, token),
        Err(GatewaySyncFailure::Fenced)
    );
    assert_eq!(
        f.status(),
        Status {
            fenced: true,
            ..uploads_before
        }
    );
    let stable_restored = f.harness.pic.get_stable_memory(f.service);
    let mut expected = local_before;
    expected.uploads.fenced = true;
    expected.funding.fenced = true;
    expected.gateways.fenced = true;
    expected.reads.fenced = true;
    assert_eq!(f.local_status(), expected);
    assert!(
        f.harness
            .pic
            .get_stable_memory(f.service)
            .eq(&stable_restored),
        "inspection changed stable bytes"
    );
    assert_eq!(f.chunk_observation().requests, 1);
}

#[test]
fn invalid_chunks_never_disclose_and_each_settled_callback_releases_capacity() {
    let f = Fixture::with_gateway_source();
    let input = f.read_input();
    let before = f.status();
    for (mode, error) in [
        (ReadSourceMode::Corrupt, Failure::ContentMismatch),
        (ReadSourceMode::Truncated, Failure::ContentMismatch),
        (ReadSourceMode::Oversized, Failure::Capacity),
        (ReadSourceMode::Malformed, Failure::Invalid),
        (ReadSourceMode::WrongType, Failure::Invalid),
        (ReadSourceMode::TruncatedEncoding, Failure::Invalid),
        (ReadSourceMode::Reject, Failure::Transport),
    ] {
        f.chunk_mode(mode, false);
        assert_eq!(f.read_chunk(f.tenant, input), Err(error));
        assert_eq!(f.session_view().sessions, 0);
        assert_eq!(f.session_view().reserved_bytes, 0);
        assert_eq!(f.status(), before);
        f.chunk_mode(ReadSourceMode::Valid, false);
        assert_eq!(f.read_chunk(f.tenant, input), Ok(()));
    }
}
