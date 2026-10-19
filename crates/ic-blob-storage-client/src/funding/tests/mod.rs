use super::*;
#[test]
fn client_pins_actor_and_all_scope_fields() {
    let actor = Principal::from_slice(&[1, 1]);
    let scope = OperatorScope {
        service: Principal::from_slice(&[2, 1]),
        namespace: u128::MAX,
        cashier: Principal::from_slice(&[3, 1]),
        payment_account: Principal::from_slice(&[4, 1]),
    };
    let timeout = 300.try_into().unwrap();
    let client = ReplicatedFundingClient::new(actor, scope, timeout).unwrap();
    assert_eq!(client.binding(actor, scope), Ok(()));
    assert_eq!(
        client.binding(scope.service, scope),
        Err(FundingClientError::Binding)
    );
    for changed in [
        OperatorScope {
            service: actor,
            ..scope
        },
        OperatorScope {
            namespace: 1,
            ..scope
        },
        OperatorScope {
            cashier: actor,
            ..scope
        },
        OperatorScope {
            payment_account: actor,
            ..scope
        },
    ] {
        assert_eq!(
            client.binding(actor, changed),
            Err(FundingClientError::Binding)
        );
    }
    assert_eq!(
        ReplicatedFundingClient::new(actor, scope, 301.try_into().unwrap()),
        Err(FundingClientError::Configuration)
    );
    assert_eq!(
        ReplicatedFundingClient::new(
            actor,
            OperatorScope {
                namespace: 0,
                ..scope
            },
            timeout
        ),
        Err(FundingClientError::Configuration)
    );
    for invalid in [Principal::anonymous(), Principal::management_canister()] {
        assert_eq!(
            ReplicatedFundingClient::new(invalid, scope, timeout),
            Err(FundingClientError::Configuration)
        );
        for changed in [
            OperatorScope {
                service: invalid,
                ..scope
            },
            OperatorScope {
                cashier: invalid,
                ..scope
            },
            OperatorScope {
                payment_account: invalid,
                ..scope
            },
        ] {
            assert_eq!(
                ReplicatedFundingClient::new(actor, changed, timeout),
                Err(FundingClientError::Configuration)
            );
        }
    }
}
