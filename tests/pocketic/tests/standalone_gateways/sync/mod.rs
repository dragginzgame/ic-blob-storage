//! Real standalone refresh against a labelled query-only Cashier substitute.
use super::*;
use blob_test_protocol::SourceMode;
use ic_blob_storage::dto::gateway::sync::{
    GatewaySyncCancellation, GatewaySyncFailure as SyncError, GatewaySyncResponse,
};
fn fixture() -> Fixture {
    fixture_with_operator(Harness::new(), Fake::principal(2))
}
pub(super) fn fixture_with_operator(harness: Harness, operator: Principal) -> Fixture {
    let driver = Fake::principal(1);
    let cashier = harness.pic.create_canister_with_settings(
        Some(driver),
        Some(CanisterSettings {
            controllers: Some(vec![driver]),
            ..CanisterSettings::default()
        }),
    );
    let f = Fixture::with_cashier_and_operator(harness, cashier, operator);
    f.harness.pic.install_canister(
        cashier,
        std::fs::read(fixture_path("BLOB_GATEWAY_SOURCE_WASM")).unwrap(),
        candid::encode_args((f.service, Fake::principal(9), driver)).unwrap(),
        Some(driver),
    );
    f
}
fn refresh(
    f: &Fixture,
    actor: Principal,
    scope: OperatorScope,
) -> Result<GatewaySyncResponse, SyncError> {
    f.harness
        .pic
        .update_candid_as(f.service, actor, "blob_sync_gateways", (scope,))
        .unwrap()
}
fn cancel(f: &Fixture, actor: Principal, sequence: u64) -> Result<(), SyncError> {
    f.harness
        .pic
        .update_candid_as(
            f.service,
            actor,
            "blob_cancel_gateway_sync",
            (GatewaySyncCancellation {
                scope: f.operator_scope(),
                sequence,
            },),
        )
        .unwrap()
}
pub(super) fn mode(f: &Fixture, selected: SourceMode) {
    assert!(
        f.harness
            .pic
            .update_candid_as::<bool, _>(
                f.config.billing.cashier,
                f.controller,
                "configure",
                (selected,)
            )
            .unwrap()
    );
}
#[test]
fn standalone_refresh_is_scoped_and_failed_queries_require_exact_cancellation() {
    let f = fixture();
    let scope = f.operator_scope();
    let before = f.harness.pic.get_stable_memory(f.service);
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(refresh(&f, actor, scope), Err(SyncError::Denied));
        assert_eq!(cancel(&f, actor, 1), Err(SyncError::Denied));
    }
    for changed in [
        OperatorScope {
            service: f.tenant,
            ..scope
        },
        OperatorScope {
            namespace: 1,
            ..scope
        },
        OperatorScope {
            cashier: f.tenant,
            ..scope
        },
        OperatorScope {
            payment_account: f.tenant,
            ..scope
        },
    ] {
        assert_eq!(refresh(&f, f.operator, changed), Err(SyncError::Binding));
    }
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    let source_before = f.harness.pic.get_stable_memory(scope.cashier);
    assert_eq!(
        refresh(&f, f.operator, scope),
        Ok(GatewaySyncResponse { scope, sequence: 1 })
    );
    let status = f.local_status(f.operator, scope).unwrap();
    assert_eq!(status.gateways.members, vec![Fake::principal(9)]);
    assert_eq!(status.gateways.pending_sequence, None);
    unchanged(
        &f.harness.pic.get_stable_memory(scope.cashier),
        &source_before,
    );
    for (index, (selected, error)) in [
        (SourceMode::Malformed, SyncError::InvalidReply),
        (SourceMode::Empty, SyncError::InvalidReply),
        (SourceMode::Oversized, SyncError::ReplyTooLarge),
        (SourceMode::Reject, SyncError::Rejected(4)),
    ]
    .into_iter()
    .enumerate()
    {
        mode(&f, selected);
        assert_eq!(refresh(&f, f.operator, scope), Err(error));
        let pending = f.local_status(f.operator, scope).unwrap();
        let sequence = pending.gateways.pending_sequence.unwrap();
        assert_eq!(sequence, index as u64 + 2);
        assert_eq!(pending.gateways.members, status.gateways.members);
        assert_eq!(pending.uploads, status.uploads);
        assert_eq!(pending.funding, status.funding);
        assert_eq!(pending.reads, status.reads);
        let bytes = f.harness.pic.get_stable_memory(f.service);
        assert_eq!(refresh(&f, f.operator, scope), Err(SyncError::Busy));
        assert_eq!(
            cancel(&f, f.operator, sequence - 1),
            Err(SyncError::Conflict)
        );
        unchanged(&f.harness.pic.get_stable_memory(f.service), &bytes);
        assert_eq!(cancel(&f, f.operator, sequence), Ok(()));
        assert_eq!(cancel(&f, f.operator, sequence), Err(SyncError::Conflict));
    }
    mode(&f, SourceMode::Valid);
    assert_eq!(refresh(&f, f.operator, scope).unwrap().sequence, 6);
}
#[test]
fn standalone_restore_preserves_failed_sync_and_blocks_refresh_and_cancellation() {
    let f = fixture();
    let scope = f.operator_scope();
    mode(&f, SourceMode::Reject);
    assert_eq!(refresh(&f, f.operator, scope), Err(SyncError::Rejected(4)));
    let pending = f.local_status(f.operator, scope).unwrap().gateways;
    assert_eq!(pending.pending_sequence, Some(1));
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(refresh(&f, f.operator, scope), Err(SyncError::Fenced));
    assert_eq!(cancel(&f, f.operator, 1), Err(SyncError::Fenced));
    let restored = f.local_status(f.operator, scope).unwrap().gateways;
    assert!(restored.fenced);
    assert_eq!(restored.pending_sequence, pending.pending_sequence);
    assert_eq!(restored.last_sequence, pending.last_sequence);
    assert_eq!(restored.members, pending.members);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
}
