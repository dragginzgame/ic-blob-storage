use super::*;

#[test]
fn special_principals_are_rejected_in_each_role() {
    let valid = Principal::from_slice(&[1, 1]);
    let one = NonZeroU128::new(1).expect("positive ID");
    let identity = ObjectIdentity {
        namespace: one,
        object: one,
        incarnation: one,
    };
    for special in [Principal::anonymous(), Principal::management_canister()] {
        assert_eq!(
            ObjectBinding::new(special, valid, identity),
            Err(ObjectBindingError::InvalidService)
        );
        assert_eq!(
            ObjectBinding::new(valid, special, identity),
            Err(ObjectBindingError::InvalidTenant)
        );
    }
    assert!(ObjectBinding::new(valid, valid, identity).is_ok());
}
