use super::*;
#[test]
fn replicated_gateway_configuration_requires_explicit_valid_principals_and_timeout() {
    let service = Principal::from_slice(&[1, 1]);
    let cashier = Principal::from_slice(&[2, 1]);
    for invalid in [Principal::anonymous(), Principal::management_canister()] {
        assert_eq!(
            ReplicatedGatewayQuery::new(invalid, cashier, NonZeroU32::MIN),
            Err(ReplicatedQueryError::InvalidPrincipal)
        );
        assert_eq!(
            ReplicatedGatewayQuery::new(service, invalid, NonZeroU32::MIN),
            Err(ReplicatedQueryError::InvalidPrincipal)
        );
    }
    for timeout in [301, u32::MAX] {
        assert_eq!(
            ReplicatedGatewayQuery::new(service, cashier, timeout.try_into().unwrap()),
            Err(ReplicatedQueryError::TimeoutOutOfRange)
        );
    }
    for timeout in [1, 300] {
        assert!(ReplicatedGatewayQuery::new(service, cashier, timeout.try_into().unwrap()).is_ok());
    }
}
