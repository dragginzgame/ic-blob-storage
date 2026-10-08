//! Actual replicated account queries over a passive source; no provider qualification.
mod native_cli;
use super::*;
use blob_test_protocol::balance::BalanceSourceConfig;
use candid::{CandidType, Int, Nat};
use ic_blob_storage_contracts::dto::account::*;
use ic_blob_storage_contracts::dto::funding::outcome::FundingReportedBalance;
use ic_blob_storage_contracts::dto::operator::OperatorScope;

#[derive(CandidType)]
struct Balance {
    total: Int,
    cycles_prepaid: Int,
    cycles_promo: Int,
    cycles_ledger: Int,
    debt_target: DebtTarget,
}
#[derive(CandidType)]
enum DebtTarget {
    Prepaid,
}
#[derive(CandidType)]
struct BalanceReport {
    account: Principal,
    account_cycle_balances: Balance,
}
#[derive(CandidType)]
enum BalanceError {
    AccountNotFound,
    InternalError(String),
}
#[derive(CandidType)]
struct RelationshipReport {
    relationship: Option<AccountRelationshipView>,
}
#[derive(CandidType)]
enum RelationshipError {
    RelationshipNotFound(Principal),
    NotAuthorized(Principal),
    InvalidRequest(String),
    InternalError(String),
}

fn fixture() -> Fixture {
    fixture_with_operator(Harness::new(), Fake::principal(2))
}
fn fixture_with_operator(harness: Harness, operator: Principal) -> Fixture {
    let cashier = harness.pic.create_canister();
    let f = Fixture::with_cashier_and_operator(harness, cashier, operator);
    f.harness.pic.install_canister(
        cashier,
        std::fs::read(fixture_path("BLOB_GATEWAY_SOURCE_WASM")).unwrap(),
        candid::encode_args((f.service, Fake::principal(9), f.controller)).unwrap(),
        None,
    );
    f
}
fn request(f: &Fixture, kind: AccountInspectionKind) -> AccountInspectionRequest {
    AccountInspectionRequest {
        scope: f.operator_scope(),
        kind,
    }
}
fn configure(f: &Fixture, kind: AccountInspectionKind, bytes: Vec<u8>) {
    let account = match kind {
        AccountInspectionKind::Balance => f.config.payment_account,
        AccountInspectionKind::PaymentRelationship => f.service,
    };
    assert!(
        f.harness
            .pic
            .update_candid_as::<bool, _>(
                f.config.billing.cashier,
                f.controller,
                "configure_balance",
                (BalanceSourceConfig {
                    account,
                    bytes,
                    reject: false,
                    hold: false
                },)
            )
            .unwrap()
    );
}
fn inspect(
    f: &Fixture,
    actor: Principal,
    input: AccountInspectionRequest,
) -> Result<AccountInspectionResponse, AccountInspectionFailure> {
    f.harness
        .pic
        .update_candid_as(f.service, actor, "blob_inspect_account", (input,))
        .unwrap()
}
fn balance(account: Principal, total: Int) -> Vec<u8> {
    candid::encode_one(Ok::<_, BalanceError>(BalanceReport {
        account,
        account_cycle_balances: Balance {
            total,
            cycles_prepaid: 2.into(),
            cycles_promo: 3.into(),
            cycles_ledger: 4.into(),
            debt_target: DebtTarget::Prepaid,
        },
    }))
    .unwrap()
}
fn relation(f: &Fixture) -> AccountRelationshipView {
    AccountRelationshipView {
        paid_canister: f.service,
        payment_account: f.config.payment_account,
        spending_limit_per_day: Int::parse(b"-340282366920938463463374607431768211456").unwrap(),
        current_period_spent: Int::parse(b"340282366920938463463374607431768211456").unwrap(),
        current_period_start: u64::MAX,
        added_timestamp: 1,
        expiration_timestamp: None,
        bandwidth_baseline_uploaded: Nat::parse(b"340282366920938463463374607431768211456")
            .unwrap(),
        bandwidth_baseline_downloaded: 2u64.into(),
        bandwidth_baseline_ts_ns: 3,
    }
}
fn relationship(value: Option<AccountRelationshipView>) -> Vec<u8> {
    candid::encode_one(Ok::<_, RelationshipError>(RelationshipReport {
        relationship: value,
    }))
    .unwrap()
}

#[test]
fn standalone_account_reads_bind_operator_and_scope_and_preserve_all_local_state() {
    let f = fixture();
    let input = request(&f, AccountInspectionKind::Balance);
    configure(
        &f,
        input.kind,
        balance(f.config.payment_account, u128::MAX.into()),
    );
    let before = f.harness.pic.get_stable_memory(f.service);
    let source_before = f.harness.pic.get_stable_memory(input.scope.cashier);
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(
            inspect(&f, actor, input),
            Err(AccountInspectionFailure::Denied)
        );
    }
    for scope in [
        OperatorScope {
            service: f.tenant,
            ..input.scope
        },
        OperatorScope {
            namespace: 1,
            ..input.scope
        },
        OperatorScope {
            cashier: f.tenant,
            ..input.scope
        },
        OperatorScope {
            payment_account: f.tenant,
            ..input.scope
        },
    ] {
        assert_eq!(
            inspect(&f, f.operator, AccountInspectionRequest { scope, ..input }),
            Err(AccountInspectionFailure::Binding)
        );
    }
    assert_eq!(
        inspect(&f, f.operator, input),
        Ok(AccountInspectionResponse {
            request: input,
            observation: AccountObservation::Balance(FundingReportedBalance {
                total: u128::MAX,
                prepaid: 2,
                promotional: 3,
                ledger: 4
            })
        })
    );
    assert!(
        f.harness
            .pic
            .query_call(
                f.service,
                f.operator,
                "blob_inspect_account",
                candid::encode_one(input).unwrap()
            )
            .is_err()
    );
    for bytes in [b"DIDL".to_vec(), vec![0; 4097]] {
        let error = f
            .harness
            .pic
            .update_call(f.service, f.operator, "blob_inspect_account", bytes)
            .unwrap_err();
        assert_eq!(error.reject_code, RejectCode::CanisterError);
    }
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    unchanged(
        &f.harness.pic.get_stable_memory(input.scope.cashier),
        &source_before,
    );
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    assert_eq!(
        inspect(&f, f.operator, input),
        Err(AccountInspectionFailure::Fenced)
    );
    unchanged(
        &f.harness.pic.get_stable_memory(input.scope.cashier),
        &source_before,
    );
}

#[test]
fn standalone_balance_errors_never_supply_zero_credit_or_trigger_funding() {
    let f = fixture();
    let input = request(&f, AccountInspectionKind::Balance);
    let before = f.harness.pic.get_stable_memory(f.service);
    for (bytes, failure) in [
        (
            balance(f.tenant, 1.into()),
            AccountInspectionFailure::Binding,
        ),
        (
            balance(f.config.payment_account, (-1).into()),
            AccountInspectionFailure::InvalidReply,
        ),
        (
            balance(
                f.config.payment_account,
                Int::parse(b"340282366920938463463374607431768211456").unwrap(),
            ),
            AccountInspectionFailure::InvalidReply,
        ),
        (vec![0], AccountInspectionFailure::InvalidReply),
        (vec![0; 4097], AccountInspectionFailure::ReplyTooLarge),
    ] {
        configure(&f, input.kind, bytes);
        assert_eq!(inspect(&f, f.operator, input), Err(failure));
    }
    for (error, expected) in [
        (
            BalanceError::AccountNotFound,
            AccountObservation::AccountNotFound,
        ),
        (
            BalanceError::InternalError("private diagnostic".into()),
            AccountObservation::ProviderInternalError,
        ),
    ] {
        configure(
            &f,
            input.kind,
            candid::encode_one(Err::<BalanceReport, _>(error)).unwrap(),
        );
        assert_eq!(
            inspect(&f, f.operator, input).unwrap().observation,
            expected
        );
    }
    assert!(
        f.harness
            .pic
            .update_candid_as::<bool, _>(
                input.scope.cashier,
                f.controller,
                "configure_balance",
                (BalanceSourceConfig {
                    account: f.config.payment_account,
                    bytes: vec![],
                    reject: true,
                    hold: false
                },)
            )
            .unwrap()
    );
    assert_eq!(
        inspect(&f, f.operator, input),
        Err(AccountInspectionFailure::Rejected(4))
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
}

#[test]
fn standalone_relationship_keeps_signed_fields_and_missing_link_distinct_for_linked_payer() {
    let mut f = fixture();
    // This fresh local installation has no objects or provider effects.
    f.config.payment_account = Fake::principal(8);
    f.harness
        .pic
        .reinstall_canister(
            f.service,
            wasm(),
            installation(&f.config),
            Some(f.controller),
        )
        .unwrap();
    let input = request(&f, AccountInspectionKind::PaymentRelationship);
    let expected = relation(&f);
    configure(&f, input.kind, relationship(Some(expected.clone())));
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(
        inspect(&f, f.operator, input),
        Ok(AccountInspectionResponse {
            request: input,
            observation: AccountObservation::Relationship(Box::new(expected.clone()))
        })
    );
    for changed in [
        AccountRelationshipView {
            paid_canister: f.tenant,
            ..expected.clone()
        },
        AccountRelationshipView {
            payment_account: f.service,
            ..expected.clone()
        },
    ] {
        configure(&f, input.kind, relationship(Some(changed)));
        assert_eq!(
            inspect(&f, f.operator, input),
            Err(AccountInspectionFailure::Binding)
        );
    }
    configure(&f, input.kind, relationship(None));
    assert_eq!(
        inspect(&f, f.operator, input).unwrap().observation,
        AccountObservation::NoRelationshipReported
    );
    for (error, expected) in [
        (
            RelationshipError::RelationshipNotFound(f.service),
            AccountObservation::RelationshipNotFound(f.service),
        ),
        (
            RelationshipError::NotAuthorized(f.tenant),
            AccountObservation::NotAuthorized(f.tenant),
        ),
        (
            RelationshipError::InvalidRequest("private".into()),
            AccountObservation::InvalidRequest,
        ),
        (
            RelationshipError::InternalError("private".into()),
            AccountObservation::ProviderInternalError,
        ),
    ] {
        configure(
            &f,
            input.kind,
            candid::encode_one(Err::<RelationshipReport, _>(error)).unwrap(),
        );
        assert_eq!(
            inspect(&f, f.operator, input).unwrap().observation,
            expected
        );
    }
    for (bytes, expected) in [
        (vec![0], AccountInspectionFailure::InvalidReply),
        (vec![0; 4097], AccountInspectionFailure::ReplyTooLarge),
    ] {
        configure(&f, input.kind, bytes);
        assert_eq!(inspect(&f, f.operator, input), Err(expected));
    }
    let balance_input = request(&f, AccountInspectionKind::Balance);
    configure(
        &f,
        balance_input.kind,
        balance(f.config.payment_account, 10.into()),
    );
    assert!(matches!(
        inspect(&f, f.operator, balance_input).unwrap().observation,
        AccountObservation::Balance(_)
    ));
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    assert_eq!(
        inspect(&f, f.operator, input),
        Err(AccountInspectionFailure::Fenced)
    );
}
