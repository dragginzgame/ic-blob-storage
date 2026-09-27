use super::*;

fn owner() -> Principal {
    Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").expect("owner")
}

fn cashier() -> Principal {
    Principal::from_slice(&[1, 1])
}

fn payer() -> Principal {
    Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").expect("payer")
}

fn binding() -> PaymentRelationshipBinding {
    PaymentRelationshipBinding {
        paid_canister: owner(),
        payment_account: payer(),
    }
}

fn fixture(hex: &str) -> Vec<u8> {
    let (pairs, remainder) = hex.trim().as_bytes().as_chunks::<2>();
    assert!(remainder.is_empty());
    pairs
        .iter()
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).expect("hex"), 16).expect("byte"))
        .collect()
}

#[test]
fn query_methods_and_argument_sequences_match_independent_candid_vectors() {
    for (query, method, hex) in [
        (
            CashierQuery::Balance { account: owner() },
            "account_balance_get_v1",
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/caffeine-query/balance.hex"
            )),
        ),
        (
            CashierQuery::PaymentRelationship(binding()),
            "payment_account_canister_get_v1",
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/caffeine-query/relationship.hex"
            )),
        ),
        (
            CashierQuery::StorageGateways,
            "storage_gateway_list_v1",
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/caffeine-query/gateways.hex"
            )),
        ),
    ] {
        let request = CashierQueryRequest::new(cashier(), query).expect("query");
        assert_eq!(request.cashier(), cashier());
        assert_eq!(request.query(), query);
        assert_eq!(request.method_name(), method);
        assert_eq!(request.arguments(), fixture(hex));
    }
}

#[test]
fn invalid_principals_reject_in_every_role_without_defaults() {
    for invalid in [Principal::anonymous(), Principal::management_canister()] {
        for query in [
            CashierQuery::Balance { account: owner() },
            CashierQuery::PaymentRelationship(binding()),
            CashierQuery::StorageGateways,
        ] {
            assert_eq!(
                CashierQueryRequest::new(invalid, query),
                Err(CashierQueryError::InvalidPrincipal {
                    field: CashierQueryPrincipal::Cashier
                })
            );
        }
        for (query, field) in [
            (
                CashierQuery::Balance { account: invalid },
                CashierQueryPrincipal::BalanceAccount,
            ),
            (
                CashierQuery::PaymentRelationship(PaymentRelationshipBinding {
                    paid_canister: invalid,
                    ..binding()
                }),
                CashierQueryPrincipal::PaidCanister,
            ),
            (
                CashierQuery::PaymentRelationship(PaymentRelationshipBinding {
                    payment_account: invalid,
                    ..binding()
                }),
                CashierQueryPrincipal::PaymentAccount,
            ),
        ] {
            assert_eq!(
                CashierQueryRequest::new(cashier(), query),
                Err(CashierQueryError::InvalidPrincipal { field })
            );
        }
    }
}

#[test]
fn relationship_payer_is_retained_as_an_expectation_not_sent_as_the_owner() {
    let linked = CashierQueryRequest::new(cashier(), CashierQuery::PaymentRelationship(binding()))
        .expect("linked");
    let self_binding = PaymentRelationshipBinding {
        payment_account: owner(),
        ..binding()
    };
    let self_paid =
        CashierQueryRequest::new(cashier(), CashierQuery::PaymentRelationship(self_binding))
            .expect("self");
    assert_eq!(linked.arguments(), self_paid.arguments());
    assert_ne!(linked.query(), self_paid.query());
    let changed_owner = CashierQueryRequest::new(
        cashier(),
        CashierQuery::PaymentRelationship(PaymentRelationshipBinding {
            paid_canister: payer(),
            ..binding()
        }),
    )
    .expect("other owner");
    assert_ne!(linked.arguments(), changed_owner.arguments());
    let changed_cashier = CashierQueryRequest::new(payer(), linked.query()).expect("other target");
    assert_eq!(linked.arguments(), changed_cashier.arguments());
    assert_ne!(linked.cashier(), changed_cashier.cashier());
}

#[test]
fn explicit_balance_account_is_not_replaced_by_the_cashier() {
    let request = CashierQueryRequest::new(cashier(), CashierQuery::Balance { account: owner() })
        .expect("query");
    let other = CashierQueryRequest::new(cashier(), CashierQuery::Balance { account: payer() })
        .expect("other account");
    assert_ne!(request.arguments(), other.arguments());
    assert_eq!(request.method_name(), other.method_name());
}
