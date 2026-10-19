use super::*;
#[test]
fn client_pins_actual_caller_and_every_scope_field_without_platform_calls() {
    let actor = Principal::from_slice(&[1, 1]);
    let scope = TenantScope {
        service: Principal::from_slice(&[2, 1]),
        namespace: u128::MAX,
        tenant: Principal::from_slice(&[3, 1]),
    };
    let timeout = NonZeroU32::new(300).unwrap();
    let client = ReplicatedTenantClient::new(actor, scope, timeout).unwrap();
    assert_eq!(client.binding(actor, scope), Ok(()));
    assert_eq!(
        client.binding(scope.tenant, scope),
        Err(TenantClientError::Binding)
    );
    for wrong in [
        TenantScope {
            service: actor,
            ..scope
        },
        TenantScope {
            namespace: 1,
            ..scope
        },
        TenantScope {
            tenant: actor,
            ..scope
        },
    ] {
        assert_eq!(
            client.binding(actor, wrong),
            Err(TenantClientError::Binding)
        );
    }
    assert_eq!(
        ReplicatedTenantClient::new(actor, scope, 301.try_into().unwrap()),
        Err(TenantClientError::Configuration)
    );
    assert_eq!(
        ReplicatedTenantClient::new(
            actor,
            TenantScope {
                namespace: 0,
                ..scope
            },
            timeout
        ),
        Err(TenantClientError::Configuration)
    );
    for invalid in [Principal::anonymous(), Principal::management_canister()] {
        assert_eq!(
            ReplicatedTenantClient::new(invalid, scope, timeout),
            Err(TenantClientError::Configuration)
        );
        assert_eq!(
            ReplicatedTenantClient::new(
                actor,
                TenantScope {
                    service: invalid,
                    ..scope
                },
                timeout
            ),
            Err(TenantClientError::Configuration)
        );
        assert_eq!(
            ReplicatedTenantClient::new(
                actor,
                TenantScope {
                    tenant: invalid,
                    ..scope
                },
                timeout
            ),
            Err(TenantClientError::Configuration)
        );
    }
}
