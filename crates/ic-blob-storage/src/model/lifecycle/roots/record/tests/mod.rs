use super::*;

#[test]
fn largest_identity_round_trips_and_invalid_identity_never_binds() {
    let binding = ObjectBinding::new(
        Principal::from_slice(&[1; 29]),
        Principal::from_slice(&[2; 29]),
        ObjectIdentity {
            namespace: NonZeroU128::MAX,
            object: NonZeroU128::MAX,
            incarnation: NonZeroU128::MAX,
        },
    )
    .unwrap();
    let key = RootObjectKeyRecord::new(binding);
    assert_eq!(RootObjectKeyRecord::from_bytes(key.to_bytes()), key);
    let claim = RootClaimRecord::Claim(key.clone());
    assert_eq!(RootClaimRecord::from_bytes(claim.to_bytes()), claim);
    assert_eq!(key.binding(binding.service()), Some(binding));
    let mut invalid = key;
    invalid.incarnation = 0;
    assert_eq!(invalid.binding(binding.service()), None);
}
