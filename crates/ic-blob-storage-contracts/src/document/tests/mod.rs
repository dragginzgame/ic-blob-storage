use super::*;
use crate::dto::tenant::TenantScope;
use ic_platform::Principal;

fn scope() -> TenantScope {
    TenantScope {
        service: Principal::self_authenticating(b"service"),
        namespace: u128::MAX,
        tenant: Principal::self_authenticating(b"tenant"),
    }
}

#[test]
fn full_width_numbers_and_binary_principals_round_trip() {
    let scope = scope();
    assert_eq!(
        decode::<TenantScope>(&encode(scope).unwrap(), 4096).unwrap(),
        scope
    );
    assert_eq!(
        decode::<u128>(&encode(u128::MAX).unwrap(), 4096).unwrap(),
        u128::MAX
    );
    assert_eq!(decode::<String>(&encode("é").unwrap(), 4096).unwrap(), "é");
}

#[test]
fn old_cbor_candid_and_service_frames_refuse() {
    let value = scope();
    let mut old_cbor = vec![0xc2, 0x50];
    old_cbor.extend_from_slice(&[255; 16]);
    assert_eq!(decode::<u128>(&old_cbor, 4096), Err(Error::Identity));
    assert_eq!(
        decode::<TenantScope>(
            &ic_platform::candid::contract::encode_one((
                ic_platform::principal::ExternalPrincipal::from(value.service),
                value.namespace,
                ic_platform::principal::ExternalPrincipal::from(value.tenant)
            ))
            .unwrap(),
            4096
        ),
        Err(Error::Identity)
    );
    assert_eq!(
        decode::<TenantScope>(
            &crate::wire::encode_request("blob_upload_capacity", value).unwrap(),
            4096
        ),
        Err(Error::Identity)
    );
    assert_eq!(
        decode::<TenantScope>(
            &native::encode_response(IDENTITY, &value, LIMITS).unwrap(),
            4096
        ),
        Err(Error::Identity)
    );
}

#[test]
fn truncation_extra_fields_and_byte_limits_refuse() {
    let value = scope();
    let bytes = encode(value).unwrap();
    assert_eq!(
        decode::<TenantScope>(&bytes, bytes.len() - 1),
        Err(Error::Limit)
    );
    assert!(decode::<TenantScope>(&bytes[..bytes.len() - 1], 4096).is_err());
    let mut trailing = bytes;
    trailing.push(0);
    assert_eq!(decode::<TenantScope>(&trailing, 4096), Err(Error::Trailing));
    assert_eq!(
        decode::<TenantScope>(&encode((value, true)).unwrap(), 4096),
        Err(Error::Trailing)
    );
    assert_eq!(encode("x".repeat(MAX_MESSAGE_BYTES + 1)), Err(Error::Limit));
}

#[derive(serde::Serialize, serde::Deserialize)]
enum Nested {
    Leaf,
    Branch(Box<Self>),
}

#[test]
fn nesting_admission_precedes_value_allocation() {
    let mut value = Nested::Leaf;
    for _ in 0..MAX_DEPTH + 2 {
        value = Nested::Branch(Box::new(value));
    }
    assert_eq!(encode(&value), Err(Error::Limit));
    let bytes = native::encode_request(
        IDENTITY,
        &value,
        Limits {
            depth: 128,
            ..LIMITS
        },
    )
    .unwrap();
    assert!(matches!(decode::<Nested>(&bytes, 4096), Err(Error::Limit)));
}
