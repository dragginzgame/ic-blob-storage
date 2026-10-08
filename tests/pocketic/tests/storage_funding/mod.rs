use super::*;
mod admission;
mod client;
mod history;
mod summary;
mod transport;
use blob_test_protocol::storage::funding::{Action, Allocation, Command, Intent, Phase};
impl Fixture {
    fn funding_scope(&self) -> ic_blob_storage_contracts::dto::operator::OperatorScope {
        ic_blob_storage_contracts::dto::operator::OperatorScope {
            service: self.service,
            cashier: self.operator,
            payment_account: self.service,
            namespace: 1,
        }
    }
    fn funding_summary(
        &self,
        actor: Principal,
        scope: ic_blob_storage_contracts::dto::operator::OperatorScope,
    ) -> Result<blob_test_protocol::storage::funding::summary::Summary, Failure> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, "funding_summary", (scope,))
            .unwrap()
    }
    pub(super) fn funding_intent(&self, operation: u128, offered: u128) -> Intent {
        Intent {
            service: self.service,
            cashier: self.operator,
            account: self.service,
            namespace: 1,
            operation,
            offered,
            target_balance: None,
        }
    }
    pub(super) fn funding(
        &self,
        actor: Principal,
        intent: Intent,
        action: Action,
    ) -> Result<bool, Failure> {
        self.harness
            .pic
            .update_candid_as(
                self.service,
                actor,
                "fixture_funding",
                (Command {
                    intent,
                    action,
                    fault: None,
                    source: None,
                },),
            )
            .unwrap()
    }
    fn funding_lookup(&self, actor: Principal, intent: Intent) -> Result<Option<Phase>, Failure> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, "funding_lookup", (intent,))
            .unwrap()
    }
    pub(super) fn funding_allocation(&self) -> Allocation {
        self.harness
            .pic
            .query_candid_as::<Result<Allocation, Failure>, _>(
                self.service,
                self.operator,
                "funding_allocation",
                (),
            )
            .unwrap()
            .unwrap()
    }
    fn funding_trap(&self, intent: Intent, action: Action, fault: WriteFault) {
        let error = self
            .harness
            .pic
            .update_call(
                self.service,
                self.operator,
                "fixture_funding",
                candid::encode_one(Command {
                    intent,
                    action,
                    fault: Some(fault),
                    source: None,
                })
                .unwrap(),
            )
            .unwrap_err();
        assert_eq!(error.reject_code, RejectCode::CanisterError);
    }
}
#[test]
fn funding_intent_and_outcome_traps_roll_back_rows_and_allocation_together() {
    let f = Fixture::new();
    let intent = f.funding_intent(u128::MAX, 900);
    let empty = f.funding_allocation();
    f.funding_trap(intent, Action::Prepare, WriteFault::FundingAccounting);
    assert_eq!(f.funding_lookup(f.operator, intent), Ok(None));
    assert_eq!(f.funding_allocation(), empty);
    assert_eq!(f.funding(f.operator, intent, Action::Prepare), Ok(true));
    assert_eq!(f.funding(f.operator, intent, Action::Prepare), Ok(false));
    let reserved = f.funding_allocation();
    assert_eq!((reserved.available, reserved.uncertain), (100, 900));
    f.funding_trap(intent, Action::Attempt, WriteFault::FundingIntents);
    assert_eq!(
        f.funding_lookup(f.operator, intent),
        Ok(Some(Phase::Prepared))
    );
    f.funding(f.operator, intent, Action::Attempt).unwrap();
    assert_eq!(
        f.funding(f.operator, intent, Action::Attempt),
        Err(Failure::Phase)
    );
    f.funding_trap(intent, Action::Callback(500), WriteFault::FundingAccounting);
    assert_eq!(
        f.funding_lookup(f.operator, intent),
        Ok(Some(Phase::Uncertain))
    );
    assert_eq!(f.funding_allocation(), reserved);
    f.funding(f.operator, intent, Action::Callback(500))
        .unwrap();
    let finished = f.funding_allocation();
    assert_eq!(
        (
            finished.available,
            finished.accepted,
            finished.refunded,
            finished.uncertain
        ),
        (600, 400, 500, 0)
    );
    assert_eq!(
        f.funding(f.operator, intent, Action::Callback(500)),
        Ok(false)
    );
    assert_eq!(
        f.funding(f.operator, intent, Action::NotEnqueued),
        Err(Failure::Conflict)
    );
    assert_eq!(f.funding_allocation(), finished);
    for actor in [f.tenant, f.controller, f.uploader] {
        assert_eq!(
            f.funding(actor, intent, Action::Prepare),
            Err(Failure::Denied)
        );
        assert_eq!(f.funding_lookup(actor, intent), Err(Failure::Denied));
    }
}
#[test]
fn funding_journal_upgrades_preserve_every_phase_and_reject_late_outcomes() {
    for phase in [
        Phase::Prepared,
        Phase::Uncertain,
        Phase::NotEnqueued,
        Phase::Callback(200),
    ] {
        let f = Fixture::new();
        let intent = f.funding_intent(1, 900);
        f.funding(f.operator, intent, Action::Prepare).unwrap();
        if phase != Phase::Prepared {
            f.funding(f.operator, intent, Action::Attempt).unwrap();
        }
        match phase {
            Phase::NotEnqueued => {
                f.funding(f.operator, intent, Action::NotEnqueued).unwrap();
            }
            Phase::Callback(refunded) => {
                f.funding(f.operator, intent, Action::Callback(refunded))
                    .unwrap();
            }
            _ => {}
        }
        let before = f.funding_allocation();
        f.harness
            .pic
            .upgrade_canister(
                f.service,
                Fixture::wasm(),
                Fixture::installation(f.operator),
                Some(f.controller),
            )
            .unwrap();
        assert_eq!(f.funding_lookup(f.operator, intent), Ok(Some(phase)));
        assert_eq!(
            f.funding_allocation(),
            Allocation {
                fenced: true,
                ..before
            }
        );
        for action in [
            Action::Prepare,
            Action::Attempt,
            Action::NotEnqueued,
            Action::Callback(0),
        ] {
            assert_eq!(f.funding(f.operator, intent, action), Err(Failure::Fenced));
        }
        assert_eq!(
            f.funding(f.operator, f.funding_intent(2, 1), Action::Prepare),
            Err(Failure::Fenced)
        );
        assert_eq!(
            f.funding_lookup(
                f.operator,
                Intent {
                    cashier: f.other,
                    ..intent
                }
            ),
            Err(Failure::Binding)
        );
    }
}

#[test]
fn canonical_funding_request_survives_upgrade_and_mismatched_outcomes_keep_the_offer_charged() {
    let f = Fixture::new();
    let original = Intent {
        target_balance: Some(u128::MAX),
        ..f.funding_intent(1, 900)
    };
    let inspect = |actor, intent| {
        f.harness
            .pic
            .query_candid_as::<Result<blob_test_protocol::storage::funding::Request, Failure>, _>(
                f.service,
                actor,
                "funding_request",
                (intent,),
            )
            .unwrap()
    };
    assert_eq!(inspect(f.operator, original), Err(Failure::Unknown));
    f.funding(f.operator, original, Action::Prepare).unwrap();
    let wire = inspect(f.operator, original).unwrap();
    let expected = ic_blob_storage::ops::caffeine::funding::request::CashierTopUpRequest::new(
        f.operator,
        f.service,
        std::num::NonZeroU128::new(900).unwrap(),
        Some(std::num::NonZeroU128::MAX),
    )
    .unwrap();
    assert_eq!(wire.cashier, expected.cashier());
    assert_eq!(wire.method, expected.method_name());
    assert_eq!(wire.arguments, expected.arguments());
    assert_eq!(wire.offered, expected.offered().get());
    for changed in [
        Intent {
            target_balance: None,
            ..original
        },
        Intent {
            target_balance: Some(100),
            ..original
        },
    ] {
        assert_eq!(
            f.funding(f.operator, changed, Action::Prepare),
            Err(Failure::Conflict)
        );
        assert_eq!(inspect(f.operator, changed), Err(Failure::Conflict));
    }
    for actor in [f.controller, f.tenant, f.uploader] {
        assert_eq!(inspect(actor, original), Err(Failure::Denied));
    }
    f.funding(f.operator, original, Action::Attempt).unwrap();
    let before = f.funding_allocation();
    let wrong_source: Result<bool, Failure> = f
        .harness
        .pic
        .update_candid_as(
            f.service,
            f.operator,
            "fixture_funding",
            (Command {
                intent: original,
                action: Action::Callback(900),
                fault: None,
                source: Some(f.other),
            },),
        )
        .unwrap();
    assert_eq!(wrong_source, Err(Failure::Binding));
    assert_eq!(f.funding_allocation(), before);
    assert_eq!(
        f.funding_lookup(f.operator, original),
        Ok(Some(Phase::Uncertain))
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
    assert_eq!(inspect(f.operator, original), Ok(wire));
    assert_eq!(
        f.funding(f.operator, original, Action::Callback(900)),
        Err(Failure::Fenced)
    );
    assert_eq!(
        f.funding_allocation(),
        Allocation {
            fenced: true,
            ..before
        }
    );
}
