use super::*;
use crate::dto::tenant::TenantScope;
use ic_platform::Principal;
#[test]
fn catalog_identities_are_unique_and_provider_is_external() {
    use crate::endpoints::{MethodCodec, SERVICE_METHODS};
    let mut ids = std::collections::BTreeSet::new();
    for method in SERVICE_METHODS {
        if method.codec == MethodCodec::Candid {
            assert!(identity(method.name).is_err());
        } else {
            assert_eq!(method.codec, MethodCodec::IcWire);
            assert!(ids.insert(identity(method.name).unwrap()));
        }
    }
}
#[test]
fn exact_method_direction_canonicality_and_bounds() {
    let method = "blob_upload_capacity";
    let input = TenantScope {
        service: Principal::self_authenticating(b"service"),
        tenant: Principal::self_authenticating(b"tenant"),
        namespace: u128::MAX,
    };
    let bytes = encode_request(method, input).unwrap();
    assert_eq!(
        decode_request::<TenantScope>(method, &bytes, 4096).unwrap(),
        input
    );
    assert!(decode_request::<TenantScope>("blob_local_status", &bytes, 4096).is_err());
    assert!(decode_response::<TenantScope>(method, &bytes, 4096).is_err());
    let response = encode_response(method, Ok::<_, u8>(input)).unwrap();
    assert_eq!(
        decode_response::<Result<TenantScope, u8>>(method, &response, 4096).unwrap(),
        Ok(input)
    );
    for end in 0..bytes.len() {
        assert!(decode_request::<TenantScope>(method, &bytes[..end], 4096).is_err());
    }
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(decode_request::<TenantScope>(method, &trailing, 4096).is_err());
    assert_eq!(
        decode_request::<TenantScope>(method, &bytes, bytes.len() - 1),
        Err(Error::Limit)
    );
    let old = crate::document::encode(input).unwrap();
    assert!(decode_request::<TenantScope>(method, &old, 4096).is_err());
}

mod manifest;
