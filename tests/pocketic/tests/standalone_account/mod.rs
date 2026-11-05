//! Actual replicated account queries over a passive source; no provider qualification.
use ic_blob_storage_pocketic_tests::fixture_calls;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
mod native_cli;
use super::*;
use blob_test_protocol::balance::BalanceSourceConfig;
use ic_blob_storage_contracts::dto::account::*;
use ic_blob_storage_contracts::dto::funding::outcome::FundingReportedBalance;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use ic_platform::candid::contract::{CandidType, Int, Nat};

#[derive(CandidType)]
#[candid_path("ic_platform::candid::contract")]
struct Balance {
    total: Int,
    cycles_prepaid: Int,
    cycles_promo: Int,
    cycles_ledger: Int,
    debt_target: DebtTarget,
}
#[derive(CandidType)]
#[candid_path("ic_platform::candid::contract")]
enum DebtTarget {
    Prepaid,
}
#[derive(CandidType)]
#[candid_path("ic_platform::candid::contract")]
struct BalanceReport {
    account: Principal,
    account_cycle_balances: Balance,
}
#[derive(CandidType)]
#[candid_path("ic_platform::candid::contract")]
enum BalanceError {
    AccountNotFound,
    InternalError(String),
}
#[derive(CandidType)]
#[candid_path("ic_platform::candid::contract")]
struct Relationship {
    paid_canister: Principal,
    payment_account: Principal,
    spending_limit_per_day: Int,
    current_period_spent: Int,
    current_period_start: u64,
    added_timestamp: u64,
    expiration_timestamp: Option<u64>,
    bandwidth_baseline_uploaded: Nat,
    bandwidth_baseline_downloaded: Nat,
    bandwidth_baseline_ts_ns: u64,
}
#[derive(CandidType)]
#[candid_path("ic_platform::candid::contract")]
struct RelationshipReport {
    relationship: Option<Relationship>,
}
#[derive(CandidType)]
#[candid_path("ic_platform::candid::contract")]
enum RelationshipError {
    RelationshipNotFound(Principal),
    NotAuthorized(Principal),
    InvalidRequest(String),
    InternalError(String),
}

async fn fixture() -> Fixture {
    fixture_with_operator(Harness::new().await, fake_principal(2)).await
}
async fn fixture_with_operator(harness: Harness, operator: Principal) -> Fixture {
    let cashier = harness.pic.create_canister().await.unwrap();
    let f = Fixture::with_cashier_and_operator(harness, cashier, operator).await;
    f.harness
        .pic
        .install_canister(
            cashier,
            std::fs::read(fixture_path("BLOB_GATEWAY_SOURCE_WASM")).unwrap(),
            blob_test_protocol::fixture_wire::encode_args(
                "fixture_init",
                (f.service, fake_principal(9), f.controller),
            )
            .unwrap(),
            None,
        )
        .await
        .unwrap();
    f
}
fn request(f: &Fixture, kind: AccountInspectionKind) -> AccountInspectionRequest {
    AccountInspectionRequest {
        scope: f.operator_scope(),
        kind,
    }
}
async fn configure(f: &Fixture, kind: AccountInspectionKind, bytes: Vec<u8>) {
    let account = match kind {
        AccountInspectionKind::Balance => f.config.payment_account,
        AccountInspectionKind::PaymentRelationship => f.service.into(),
    };
    assert!(
        fixture_calls::update_fixture::<(bool,), _>(
            &f.harness.pic,
            f.config.billing.cashier.into(),
            f.controller,
            "configure_balance",
            (BalanceSourceConfig {
                account: account.into(),
                bytes,
                reject: false,
                hold: false
            },)
        )
        .await
        .map(|(reply,)| reply)
        .unwrap()
    );
}
async fn inspect(
    f: &Fixture,
    actor: Principal,
    input: AccountInspectionRequest,
) -> Result<AccountInspectionResponse, AccountInspectionFailure> {
    f.harness
        .pic
        .update_blob_as(f.service, actor, "blob_inspect_account", input, REPLY_BYTES)
        .await
        .unwrap()
}
fn balance(account: Principal, total: Int) -> Vec<u8> {
    ic_platform::candid::contract::encode_one(Ok::<_, BalanceError>(BalanceReport {
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
        paid_canister: f.service.into(),
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
    ic_platform::candid::contract::encode_one(Ok::<_, RelationshipError>(RelationshipReport {
        relationship: value.map(|value| Relationship {
            paid_canister: value.paid_canister.into(),
            payment_account: value.payment_account.into(),
            spending_limit_per_day: value.spending_limit_per_day,
            current_period_spent: value.current_period_spent,
            current_period_start: value.current_period_start,
            added_timestamp: value.added_timestamp,
            expiration_timestamp: value.expiration_timestamp,
            bandwidth_baseline_uploaded: value.bandwidth_baseline_uploaded,
            bandwidth_baseline_downloaded: value.bandwidth_baseline_downloaded,
            bandwidth_baseline_ts_ns: value.bandwidth_baseline_ts_ns,
        }),
    }))
    .unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "Keep independent callers and all account observations against the same installed state"
)]
async fn standalone_account_reads_bind_operator_and_scope_and_preserve_all_local_state() {
    let f = fixture().await;
    let input = request(&f, AccountInspectionKind::Balance);
    configure(
        &f,
        input.kind,
        balance(f.config.payment_account.into(), u128::MAX.into()),
    )
    .await;
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let source_before = f
        .harness
        .pic
        .get_stable_memory(input.scope.cashier.into())
        .await
        .unwrap();
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(
            inspect(&f, actor, input).await,
            Err(AccountInspectionFailure::Denied)
        );
    }
    for scope in [
        OperatorScope {
            service: f.tenant.into(),
            ..input.scope
        },
        OperatorScope {
            namespace: 1,
            ..input.scope
        },
        OperatorScope {
            cashier: f.tenant.into(),
            ..input.scope
        },
        OperatorScope {
            payment_account: f.tenant.into(),
            ..input.scope
        },
    ] {
        assert_eq!(
            inspect(&f, f.operator, AccountInspectionRequest { scope, ..input }).await,
            Err(AccountInspectionFailure::Binding)
        );
    }
    assert_eq!(
        inspect(&f, f.operator, input).await,
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
                ic_blob_storage_pocketic_tests::retired_packets::candid_scope()
            )
            .await
            .is_err()
    );
    for bytes in [b"DIDL".to_vec(), vec![0; 4097]] {
        let error = f
            .harness
            .pic
            .update_call(f.service, f.operator, "blob_inspect_account", bytes)
            .await
            .unwrap_err();
        assert_eq!(
            crate::canister_rejection(&error).reject_code,
            RejectCode::CanisterError
        );
    }
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
    unchanged(
        &f.harness
            .pic
            .get_stable_memory(input.scope.cashier.into())
            .await
            .unwrap(),
        &source_before,
    );
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    assert_eq!(
        inspect(&f, f.operator, input).await,
        Err(AccountInspectionFailure::Fenced)
    );
    unchanged(
        &f.harness
            .pic
            .get_stable_memory(input.scope.cashier.into())
            .await
            .unwrap(),
        &source_before,
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn standalone_balance_errors_never_supply_zero_credit_or_trigger_funding() {
    let f = fixture().await;
    let input = request(&f, AccountInspectionKind::Balance);
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    for (bytes, failure) in [
        (
            balance(f.tenant, 1.into()),
            AccountInspectionFailure::Binding,
        ),
        (
            balance(f.config.payment_account.into(), (-1).into()),
            AccountInspectionFailure::InvalidReply,
        ),
        (
            balance(
                f.config.payment_account.into(),
                Int::parse(b"340282366920938463463374607431768211456").unwrap(),
            ),
            AccountInspectionFailure::InvalidReply,
        ),
        (vec![0], AccountInspectionFailure::InvalidReply),
        (vec![0; 4097], AccountInspectionFailure::ReplyTooLarge),
    ] {
        configure(&f, input.kind, bytes).await;
        assert_eq!(inspect(&f, f.operator, input).await, Err(failure));
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
            ic_platform::candid::contract::encode_one(Err::<BalanceReport, _>(error)).unwrap(),
        )
        .await;
        assert_eq!(
            inspect(&f, f.operator, input).await.unwrap().observation,
            expected
        );
    }
    assert!(
        fixture_calls::update_fixture::<(bool,), _>(
            &f.harness.pic,
            input.scope.cashier.into(),
            f.controller,
            "configure_balance",
            (BalanceSourceConfig {
                account: f.config.payment_account.into(),
                bytes: vec![],
                reject: true,
                hold: false
            },)
        )
        .await
        .map(|(reply,)| reply)
        .unwrap()
    );
    assert_eq!(
        inspect(&f, f.operator, input).await,
        Err(AccountInspectionFailure::Rejected(4))
    );
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "One linked-payer journey distinguishes exact signed values, missing links and restore fences"
)]
async fn standalone_relationship_keeps_signed_fields_and_missing_link_distinct_for_linked_payer() {
    let mut f = fixture().await;
    // This fresh local installation has no objects or provider effects.
    f.config.payment_account = fake_principal(8).into();
    f.harness
        .pic
        .reinstall_canister(
            f.service,
            wasm(),
            installation(&f.config),
            Some(f.controller),
        )
        .await
        .unwrap();
    let input = request(&f, AccountInspectionKind::PaymentRelationship);
    let expected = relation(&f);
    configure(&f, input.kind, relationship(Some(expected.clone()))).await;
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(
        inspect(&f, f.operator, input).await,
        Ok(AccountInspectionResponse {
            request: input,
            observation: AccountObservation::Relationship(Box::new(expected.clone()))
        })
    );
    for changed in [
        AccountRelationshipView {
            paid_canister: f.tenant.into(),
            ..expected.clone()
        },
        AccountRelationshipView {
            payment_account: f.service.into(),
            ..expected.clone()
        },
    ] {
        configure(&f, input.kind, relationship(Some(changed))).await;
        assert_eq!(
            inspect(&f, f.operator, input).await,
            Err(AccountInspectionFailure::Binding)
        );
    }
    configure(&f, input.kind, relationship(None)).await;
    assert_eq!(
        inspect(&f, f.operator, input).await.unwrap().observation,
        AccountObservation::NoRelationshipReported
    );
    for (error, expected) in [
        (
            RelationshipError::RelationshipNotFound(f.service),
            AccountObservation::RelationshipNotFound(f.service.into()),
        ),
        (
            RelationshipError::NotAuthorized(f.tenant),
            AccountObservation::NotAuthorized(f.tenant.into()),
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
            ic_platform::candid::contract::encode_one(Err::<RelationshipReport, _>(error)).unwrap(),
        )
        .await;
        assert_eq!(
            inspect(&f, f.operator, input).await.unwrap().observation,
            expected
        );
    }
    for (bytes, expected) in [
        (vec![0], AccountInspectionFailure::InvalidReply),
        (vec![0; 4097], AccountInspectionFailure::ReplyTooLarge),
    ] {
        configure(&f, input.kind, bytes).await;
        assert_eq!(inspect(&f, f.operator, input).await, Err(expected));
    }
    let balance_input = request(&f, AccountInspectionKind::Balance);
    configure(
        &f,
        balance_input.kind,
        balance(f.config.payment_account.into(), 10.into()),
    )
    .await;
    assert!(matches!(
        inspect(&f, f.operator, balance_input)
            .await
            .unwrap()
            .observation,
        AccountObservation::Balance(_)
    ));
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    assert_eq!(
        inspect(&f, f.operator, input).await,
        Err(AccountInspectionFailure::Fenced)
    );
}
