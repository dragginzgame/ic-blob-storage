use super::*;
#[test]
fn passive_scope_refusals_preserve_authority_precedence() {
    let service = Principal::from_slice(&[1]);
    let tenant = Principal::from_slice(&[2]);
    let stranger = Principal::from_slice(&[3]);
    // Invalid namespace must not mask foreign service or caller admission.
    assert_eq!(
        assess_scope(service, stranger, stranger, tenant, 0),
        Err(TenantScopeError::Binding)
    );
    assert_eq!(
        assess_scope(service, stranger, service, tenant, 0),
        Err(TenantScopeError::Denied)
    );
    for invalid in [Principal::anonymous(), Principal::management_canister()] {
        assert_eq!(
            assess_scope(service, invalid, service, invalid, 1),
            Err(TenantScopeError::Invalid)
        );
    }
    assert_eq!(
        assess_scope(service, tenant, service, tenant, 0),
        Err(TenantScopeError::Invalid)
    );
    assert_eq!(
        assess_scope(service, tenant, service, tenant, u128::MAX)
            .unwrap()
            .get(),
        u128::MAX
    );
}
