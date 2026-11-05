use super::*;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
mod admission;
mod client;
mod history;
mod summary;
mod transport;
use blob_test_protocol::storage::funding::{Action, Allocation, Command, Intent, Phase};
impl Fixture {
    fn funding_scope(&self) -> ic_blob_storage_contracts::dto::operator::OperatorScope {
        ic_blob_storage_contracts::dto::operator::OperatorScope {
            service: self.service.into(),
            cashier: self.operator.into(),
            payment_account: self.service.into(),
            namespace: 1,
        }
    }
    async fn funding_summary(
        &self,
        actor: Principal,
        scope: ic_blob_storage_contracts::dto::operator::OperatorScope,
    ) -> Result<blob_test_protocol::storage::funding::summary::Summary, Failure> {
        self.harness
            .pic
            .query_control_as(
                self.service,
                actor,
                "funding_summary",
                (scope,),
                REPLY_BYTES,
            )
            .await
            .map(|(reply,)| reply)
            .unwrap()
    }
    pub(super) fn funding_intent(&self, operation: u128, offered: u128) -> Intent {
        Intent {
            service: self.service.into(),
            cashier: self.operator.into(),
            account: self.service.into(),
            namespace: 1,
            operation,
            offered,
            target_balance: None,
        }
    }
    pub(super) async fn funding(
        &self,
        actor: Principal,
        intent: Intent,
        action: Action,
    ) -> Result<bool, Failure> {
        self.harness
            .pic
            .update_control_as(
                self.service,
                actor,
                "fixture_funding",
                (Command {
                    intent,
                    action,
                    fault: None,
                    source: None,
                },),
                REPLY_BYTES,
            )
            .await
            .map(|(reply,)| reply)
            .unwrap()
    }
    async fn funding_lookup(
        &self,
        actor: Principal,
        intent: Intent,
    ) -> Result<Option<Phase>, Failure> {
        self.harness
            .pic
            .query_control_as(
                self.service,
                actor,
                "funding_lookup",
                (intent,),
                REPLY_BYTES,
            )
            .await
            .map(|(reply,)| reply)
            .unwrap()
    }
    pub(super) async fn funding_allocation(&self) -> Allocation {
        self.harness
            .pic
            .query_control_as::<(Result<Allocation, Failure>,), _>(
                self.service,
                self.operator,
                "funding_allocation",
                (),
                REPLY_BYTES,
            )
            .await
            .map(|(reply,)| reply)
            .unwrap()
            .unwrap()
    }
    async fn funding_trap(&self, intent: Intent, action: Action, fault: WriteFault) {
        let error = self
            .harness
            .pic
            .update_call(
                self.service,
                self.operator,
                "fixture_funding",
                ic_blob_storage_pocketic_tests::control_calls::encode_control_one(Command {
                    intent,
                    action,
                    fault: Some(fault),
                    source: None,
                })
                .unwrap(),
            )
            .await
            .unwrap_err();
        assert_eq!(
            crate::canister_rejection(&error).reject_code,
            RejectCode::CanisterError
        );
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn funding_intent_and_outcome_traps_roll_back_rows_and_allocation_together() {
    let f = Fixture::new().await;
    let intent = f.funding_intent(u128::MAX, 900);
    let empty = f.funding_allocation().await;
    f.funding_trap(intent, Action::Prepare, WriteFault::FundingAccounting)
        .await;
    assert_eq!(f.funding_lookup(f.operator, intent).await, Ok(None));
    assert_eq!(f.funding_allocation().await, empty);
    assert_eq!(
        f.funding(f.operator, intent, Action::Prepare).await,
        Ok(true)
    );
    assert_eq!(
        f.funding(f.operator, intent, Action::Prepare).await,
        Ok(false)
    );
    let reserved = f.funding_allocation().await;
    assert_eq!((reserved.available, reserved.uncertain), (100, 900));
    f.funding_trap(intent, Action::Attempt, WriteFault::FundingIntents)
        .await;
    assert_eq!(
        f.funding_lookup(f.operator, intent).await,
        Ok(Some(Phase::Prepared))
    );
    f.funding(f.operator, intent, Action::Attempt)
        .await
        .unwrap();
    assert_eq!(
        f.funding(f.operator, intent, Action::Attempt).await,
        Err(Failure::Phase)
    );
    f.funding_trap(intent, Action::Callback(500), WriteFault::FundingAccounting)
        .await;
    assert_eq!(
        f.funding_lookup(f.operator, intent).await,
        Ok(Some(Phase::Uncertain))
    );
    assert_eq!(f.funding_allocation().await, reserved);
    f.funding(f.operator, intent, Action::Callback(500))
        .await
        .unwrap();
    let finished = f.funding_allocation().await;
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
        f.funding(f.operator, intent, Action::Callback(500)).await,
        Ok(false)
    );
    assert_eq!(
        f.funding(f.operator, intent, Action::NotEnqueued).await,
        Err(Failure::Conflict)
    );
    assert_eq!(f.funding_allocation().await, finished);
    for actor in [f.tenant, f.controller, f.uploader] {
        assert_eq!(
            f.funding(actor, intent, Action::Prepare).await,
            Err(Failure::Denied)
        );
        assert_eq!(f.funding_lookup(actor, intent).await, Err(Failure::Denied));
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn funding_journal_upgrades_preserve_every_phase_and_reject_late_outcomes() {
    for phase in [
        Phase::Prepared,
        Phase::Uncertain,
        Phase::NotEnqueued,
        Phase::Callback(200),
    ] {
        let f = Fixture::new().await;
        let intent = f.funding_intent(1, 900);
        f.funding(f.operator, intent, Action::Prepare)
            .await
            .unwrap();
        if phase != Phase::Prepared {
            f.funding(f.operator, intent, Action::Attempt)
                .await
                .unwrap();
        }
        match phase {
            Phase::NotEnqueued => {
                f.funding(f.operator, intent, Action::NotEnqueued)
                    .await
                    .unwrap();
            }
            Phase::Callback(refunded) => {
                f.funding(f.operator, intent, Action::Callback(refunded))
                    .await
                    .unwrap();
            }
            _ => {}
        }
        let before = f.funding_allocation().await;
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
        assert_eq!(f.funding_lookup(f.operator, intent).await, Ok(Some(phase)));
        assert_eq!(
            f.funding_allocation().await,
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
            assert_eq!(
                f.funding(f.operator, intent, action).await,
                Err(Failure::Fenced)
            );
        }
        assert_eq!(
            f.funding(f.operator, f.funding_intent(2, 1), Action::Prepare)
                .await,
            Err(Failure::Fenced)
        );
        assert_eq!(
            f.funding_lookup(
                f.operator,
                Intent {
                    cashier: f.other.into(),
                    ..intent
                }
            )
            .await,
            Err(Failure::Binding)
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "Keep the same canonical request and charged offer across restore and mismatched outcomes"
)]
async fn canonical_funding_request_survives_upgrade_and_mismatched_outcomes_keep_the_offer_charged()
{
    let f = Fixture::new().await;
    let original = Intent {
        target_balance: Some(u128::MAX),
        ..f.funding_intent(1, 900)
    };
    let inspect = async |actor, intent| {
        f.harness.pic.query_control_as::<
            (Result<blob_test_protocol::storage::funding::Request, Failure>,),
            _,
        >(
            f.service,
            actor,
            "funding_request",
            (intent,),
            REPLY_BYTES,
        )
        .await.map(|(reply,)| reply)
        .unwrap()
    };
    assert_eq!(inspect(f.operator, original).await, Err(Failure::Unknown));
    f.funding(f.operator, original, Action::Prepare)
        .await
        .unwrap();
    let wire = inspect(f.operator, original).await.unwrap();
    let expected = ic_blob_storage::ops::caffeine::funding::request::CashierTopUpRequest::new(
        f.operator.into(),
        f.service.into(),
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
            f.funding(f.operator, changed, Action::Prepare).await,
            Err(Failure::Conflict)
        );
        assert_eq!(inspect(f.operator, changed).await, Err(Failure::Conflict));
    }
    for actor in [f.controller, f.tenant, f.uploader] {
        assert_eq!(inspect(actor, original).await, Err(Failure::Denied));
    }
    f.funding(f.operator, original, Action::Attempt)
        .await
        .unwrap();
    let before = f.funding_allocation().await;
    let wrong_source: Result<bool, Failure> = f
        .harness
        .pic
        .update_control_as(
            f.service,
            f.operator,
            "fixture_funding",
            (Command {
                intent: original,
                action: Action::Callback(900),
                fault: None,
                source: Some(f.other.into()),
            },),
            REPLY_BYTES,
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
    assert_eq!(wrong_source, Err(Failure::Binding));
    assert_eq!(f.funding_allocation().await, before);
    assert_eq!(
        f.funding_lookup(f.operator, original).await,
        Ok(Some(Phase::Uncertain))
    );
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
    assert_eq!(inspect(f.operator, original).await, Ok(wire));
    assert_eq!(
        f.funding(f.operator, original, Action::Callback(900)).await,
        Err(Failure::Fenced)
    );
    assert_eq!(
        f.funding_allocation().await,
        Allocation {
            fenced: true,
            ..before
        }
    );
}
