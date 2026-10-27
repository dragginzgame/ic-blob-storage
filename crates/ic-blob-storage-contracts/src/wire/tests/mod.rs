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
    for retired in [
        "blob_attest_upload",
        "blob_upload_attestation",
        "blob_verification_manifest",
        "blob_verification_plan",
    ] {
        assert!(identity(retired).is_err());
        assert!(!SERVICE_METHODS.iter().any(|method| method.name == retired));
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
    let document = crate::document::encode(input).unwrap();
    assert!(decode_request::<TenantScope>(method, &document, 4096).is_err());
    assert_eq!(
        decode_request::<()>("blob_resume_current_instance", &[0xf6], 4096),
        Err(Error::Malformed)
    );
}

mod manifest;

#[test]
fn lifecycle_constants_remain_separate_from_callable_methods() {
    const INSTALL: u64 = match identity("blob_install") {
        Ok(id) => id,
        Err(_) => panic!("install identity"),
    };
    const POST_UPGRADE: u64 = match identity("blob_post_upgrade") {
        Ok(id) => id,
        Err(_) => panic!("recovery identity"),
    };
    const INSTALL_LIMITS: Limits = match request_limits("blob_install") {
        Ok(limits) => limits,
        Err(_) => panic!("install limits"),
    };
    const RECOVERY_LIMITS: Limits = match request_limits("blob_post_upgrade") {
        Ok(limits) => limits,
        Err(_) => panic!("recovery limits"),
    };
    const UNKNOWN: Result<u64, Error> = identity("unknown");
    assert_eq!(
        (INSTALL, POST_UPGRADE),
        (0x424c_4f42_0052_ff01, 0x424c_4f42_0042_ff02)
    );
    assert_eq!(
        (INSTALL_LIMITS.bytes, RECOVERY_LIMITS.bytes),
        (16 * 1024, 4096)
    );
    assert_eq!(UNKNOWN, Err(Error::Identity));
    for name in ["blob_install", "blob_post_upgrade"] {
        assert!(
            !crate::endpoints::SERVICE_METHODS
                .iter()
                .any(|m| m.name == name)
        );
    }
}

#[test]
fn generic_helpers_enforce_declared_tenant_response_and_traversal_limits() {
    let method = crate::protocol::TENANT_INSPECTION_METHOD;
    let oversized = vec![0u8; 1024];
    assert!(super::encode_response(method, &oversized).is_err());
    let permissive = super::limits(super::MAX_MESSAGE_BYTES);
    let frame =
        ic_wire::native::encode_response(super::identity(method).unwrap(), &oversized, permissive)
            .unwrap();
    assert!(super::decode_response::<Vec<u8>>(method, &frame, super::MAX_MESSAGE_BYTES).is_err());
    // Small bytes still exceed the tenant traversal budget; the generic path must
    // not silently substitute the ordinary service's larger node allowance.
    let many_nodes = vec![0u8; 100];
    let frame =
        ic_wire::native::encode_request(super::identity(method).unwrap(), &many_nodes, permissive)
            .unwrap();
    assert!(frame.len() < super::tenant::MAX_REQUEST_BYTES);
    assert!(super::decode_request::<Vec<u8>>(method, &frame, 512).is_err());
    assert!(super::encode_request(method, &many_nodes).is_err());
}
