use super::*;
#[test]
fn account_transport_binds_payer_owner_source_and_method() {
    let service = Principal::from_slice(&[1]);
    let cashier = Principal::from_slice(&[2]);
    let payer = Principal::from_slice(&[3]);
    let transport =
        ReplicatedAccountQuery::new(service, cashier, payer, 30.try_into().unwrap()).unwrap();
    for (source, query, expected) in [
        (cashier, CashierQuery::Balance { account: payer }, Ok(())),
        (
            cashier,
            CashierQuery::PaymentRelationship(transport.binding),
            Ok(()),
        ),
        (
            cashier,
            CashierQuery::Balance { account: service },
            Err(ReplicatedQueryError::Binding),
        ),
        (
            cashier,
            CashierQuery::PaymentRelationship(PaymentRelationshipBinding {
                paid_canister: payer,
                payment_account: payer,
            }),
            Err(ReplicatedQueryError::Binding),
        ),
        (
            payer,
            CashierQuery::Balance { account: payer },
            Err(ReplicatedQueryError::Binding),
        ),
        (
            cashier,
            CashierQuery::StorageGateways,
            Err(ReplicatedQueryError::Method),
        ),
    ] {
        assert_eq!(
            transport.check(&CashierQueryRequest::new(source, query).unwrap()),
            expected
        );
    }
    assert_eq!(
        ReplicatedAccountQuery::new(
            service,
            cashier,
            Principal::anonymous(),
            30.try_into().unwrap()
        ),
        Err(ReplicatedQueryError::InvalidPrincipal)
    );
}
