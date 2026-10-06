//! Actual IC rollback, repeated dispatch and same-image restore of bounded grants.
use super::*;
use blob_test_protocol::storage::funding::RenewalCommand;
fn increase(f: &Fixture, actor: Principal, input: RenewalCommand) -> Result<bool, Failure> {
    let packet = candid::encode_one(input).unwrap();
    retain_packet("renewal-request", &packet);
    let reply = f
        .harness
        .pic
        .update_call(f.service, actor, "fixture_renew_funding_budget", packet)
        .unwrap();
    retain_packet("renewal-reply", &reply);
    candid::decode_one(&reply).unwrap()
}
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One actual IC journey binds grant traps, immutable replay, later dispatch and fenced restoration"
)]
fn bounded_budget_grant_rolls_back_replays_and_survives_fenced_restoration() {
    let f = Fixture::with_cashier();
    let first = f.funding_intent(1, 400);
    f.configure_cashier(first, 300, FundingReplyMode::Success);
    f.funding(f.operator, first, Action::Prepare).unwrap();
    settled(f.guarded_dispatch(f.operator, request(first)));
    f.confirm_credit(f.operator, credit(first, 300, 1)).unwrap();
    let grant = RenewalCommand {
        intent: first,
        additional: 200,
        fault: None,
    };
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(increase(&f, f.other, grant), Err(Failure::Denied));
    assert_eq!(
        increase(
            &f,
            f.operator,
            RenewalCommand {
                additional: 301,
                ..grant
            }
        ),
        Err(Failure::Invalid)
    );
    assert!(
        f.harness.pic.get_stable_memory(f.service) == before,
        "refused grant changed stable bytes"
    );
    for fault in [WriteFault::FundingAccounting, WriteFault::FundingCommit] {
        let packet = candid::encode_one(RenewalCommand {
            fault: Some(fault),
            ..grant
        })
        .unwrap();
        retain_packet("renewal-trap-request", &packet);
        let result = f.harness.pic.update_call(
            f.service,
            f.operator,
            "fixture_renew_funding_budget",
            packet,
        );
        retain_packet("renewal-trap-result", format!("{result:?}").as_bytes());
        assert_eq!(result.unwrap_err().reject_code, RejectCode::CanisterError);
        assert!(
            f.harness.pic.get_stable_memory(f.service) == before,
            "grant trap changed stable bytes"
        );
    }
    assert_eq!(increase(&f, f.operator, grant), Ok(true));
    let granted = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(increase(&f, f.operator, grant), Ok(false));
    assert_eq!(
        increase(
            &f,
            f.operator,
            RenewalCommand {
                additional: 201,
                ..grant
            }
        ),
        Err(Failure::Conflict)
    );
    assert!(
        f.harness.pic.get_stable_memory(f.service) == granted,
        "grant replay/conflict changed stable bytes"
    );
    let status = f.local_status().funding;
    assert_eq!(
        (
            status.cumulative_allocation,
            status.renewal_ceiling,
            status.available_allocation,
            status.transport_accepted
        ),
        (1200, 2000, 900, 300)
    );
    assert_eq!(
        f.retained_outcome(f.operator, first)
            .unwrap()
            .unwrap()
            .renewed_allocation,
        200
    );
    let second = f.funding_intent(2, 400);
    f.configure_cashier(second, 200, FundingReplyMode::Success);
    f.funding(f.operator, second, Action::Prepare).unwrap();
    assert_eq!(
        settled(f.guarded_dispatch(f.operator, request(second))).accepted,
        200
    );
    f.confirm_credit(f.operator, credit(second, 200, 2))
        .unwrap();
    assert_eq!(increase(&f, f.operator, grant), Ok(false));
    assert_eq!(f.incoming().len(), 2);
    let before = f.local_status().funding;
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation(f.operator),
            Some(f.controller),
        )
        .unwrap();
    let restored = f.local_status().funding;
    assert_eq!(restored.cumulative_allocation, before.cumulative_allocation);
    assert_eq!(restored.available_allocation, before.available_allocation);
    assert_eq!(restored.transport_accepted, 500);
    assert_eq!(restored.uncredited_accepted, 0);
    assert!(restored.fenced);
    let reopened = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(increase(&f, f.operator, grant), Err(Failure::Fenced));
    assert!(
        f.harness.pic.get_stable_memory(f.service) == reopened,
        "fenced grant changed stable bytes"
    );
    retain(&f, "renewal");
}
#[test]
fn post_commit_credit_trap_rolls_back_intent_index_and_accounting_without_another_payment() {
    let f = Fixture::with_cashier();
    let first = f.funding_intent(1, 400);
    f.configure_cashier(first, 300, FundingReplyMode::Success);
    f.funding(f.operator, first, Action::Prepare).unwrap();
    settled(f.guarded_dispatch(f.operator, request(first)));
    let before = f.harness.pic.get_stable_memory(f.service);
    let packet = candid::encode_one(CreditCommand {
        fault: Some(WriteFault::FundingCommit),
        ..credit(first, 300, 1)
    })
    .unwrap();
    retain_packet("post-credit-trap-request", &packet);
    let result = f.harness.pic.update_call(
        f.service,
        f.operator,
        "fixture_confirm_funding_credit",
        packet,
    );
    retain_packet("post-credit-trap-result", format!("{result:?}").as_bytes());
    assert_eq!(result.unwrap_err().reject_code, RejectCode::CanisterError);
    assert!(
        f.harness.pic.get_stable_memory(f.service) == before,
        "post-credit trap changed stable bytes"
    );
    assert_eq!(f.local_status().funding.uncredited_accepted, 300);
    assert_eq!(
        f.confirm_credit(f.operator, credit(first, 300, 1)),
        Ok(Some(true))
    );
    assert_eq!(
        f.confirm_credit(f.operator, credit(first, 300, 1)),
        Ok(Some(false))
    );
    assert_eq!(f.incoming().len(), 1);
    retain(&f, "post-credit-rollback");
}
