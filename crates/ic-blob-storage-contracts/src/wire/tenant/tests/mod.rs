use super::*;
use crate::tenant::reply;
use ic_platform::Principal;

fn scope() -> native::TenantScope {
    native::TenantScope {
        service: Principal::from_slice(&[1; 29]),
        tenant: Principal::from_slice(&[2; 29]),
        namespace: u128::MAX,
    }
}
#[test]
fn full_width_native_values_and_every_application_failure_round_trip() {
    let scope = scope();
    let query = inspection_request(scope).unwrap();
    assert_eq!(decode_inspection_request(&query).unwrap(), scope);
    assert_eq!(&query[..2], b"IW");
    assert_eq!(&query[2..10], &Inspect::ID.to_le_bytes());
    assert_eq!(query[10], 0);
    for expected in [
        None,
        Some(native::TenantEnrollment {
            generation: u64::MAX,
            active: true,
        }),
    ] {
        let command = native::TenantUpdateRequest {
            scope,
            expected,
            active: false,
        };
        assert_eq!(
            decode_update_request(&update_request(command).unwrap()).unwrap(),
            command
        );
        let observation = native::TenantEnrollmentResponse {
            scope,
            enrollment: expected,
            fenced: true,
        };
        let response = Ok(observation);
        let bytes = inspection_response(response).unwrap();
        assert_eq!(decode_inspection_response(&bytes).unwrap(), response);
        assert_eq!(
            reply::inspection(scope, &bytes, 512.try_into().unwrap()).unwrap(),
            observation
        );
    }
    for failure in [
        native::TenantFailure::Binding,
        native::TenantFailure::Denied,
        native::TenantFailure::Invalid,
        native::TenantFailure::Conflict,
        native::TenantFailure::Capacity,
        native::TenantFailure::GenerationExhausted,
        native::TenantFailure::Fenced,
        native::TenantFailure::Internal,
    ] {
        let value = Err(failure);
        assert_eq!(
            decode_inspection_response(&inspection_response(value).unwrap()).unwrap(),
            value
        );
        assert_eq!(
            decode_update_response(&update_response(value).unwrap()).unwrap(),
            value
        );
    }
}
#[test]
fn method_direction_and_previous_encodings_never_become_current_authority() {
    let scope = scope();
    let request = inspection_request(scope).unwrap();
    let update = update_request(native::TenantUpdateRequest {
        scope,
        expected: None,
        active: true,
    })
    .unwrap();
    let response = inspection_response(Ok(native::TenantEnrollmentResponse {
        scope,
        enrollment: None,
        fenced: false,
    }))
    .unwrap();
    assert!(decode_update_request(&request).is_err());
    assert!(decode_inspection_request(&update).is_err());
    assert!(decode_inspection_request(&response).is_err());
    assert!(decode_inspection_response(&request).is_err());
    assert!(decode_update_response(&response).is_err());
    for old in [
        crate::document::encode(scope).unwrap(),
        ic_platform::candid::contract::encode_one((
            ic_platform::principal::ExternalPrincipal::from(scope.service),
            scope.namespace,
            ic_platform::principal::ExternalPrincipal::from(scope.tenant),
        ))
        .unwrap(),
    ] {
        assert!(decode_inspection_request(&old).is_err());
    }
    let mut trailing = request.clone();
    trailing.push(0);
    assert!(decode_inspection_request(&trailing).is_err());
    assert!(decode_inspection_request(&vec![0; 513]).is_err());
    for end in 0..request.len() {
        assert!(decode_inspection_request(&request[..end]).is_err());
    }
}
#[test]
fn bounded_principals_and_canonical_integers_refuse_malformed_payloads() {
    let mut native = scope();
    native.namespace = 1;
    let request = inspection_request(native).unwrap();
    let namespace = 11 + 1 + native.service.as_slice().len();
    let mut nonminimal = request.clone();
    nonminimal.splice(namespace..=namespace, [0x81, 0x00]);
    assert!(decode_inspection_request(&nonminimal).is_err());
    let mut principal = request.clone();
    principal[11] = 30;
    assert!(decode_inspection_request(&principal).is_err());
    assert!(BytesRef::<29>::new(&[0; 30]).is_err());
    assert_ne!(Inspect::ID, Update::ID);
    assert_eq!(Inspect::NAME, crate::protocol::TENANT_INSPECTION_METHOD);
    assert_eq!(Update::NAME, crate::protocol::TENANT_UPDATE_METHOD);
}
