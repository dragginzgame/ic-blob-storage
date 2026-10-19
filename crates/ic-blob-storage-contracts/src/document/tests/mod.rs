use super::*;
use crate::dto::tenant::TenantScope;
use ic_platform::Principal;

#[test]
fn full_width_numbers_and_binary_principals_round_trip() {
    let scope = TenantScope {
        service: Principal::self_authenticating(b"service"),
        namespace: u128::MAX,
        tenant: Principal::self_authenticating(b"tenant"),
    };
    let bytes = encode(scope).unwrap();
    assert_eq!(decode::<TenantScope>(&bytes, 4096).unwrap(), scope);
    assert!(!bytes.starts_with(b"DIDL"));
    let mut expected = vec![0xc2, 0x50];
    expected.extend_from_slice(&[255; 16]);
    assert_eq!(encode(u128::MAX).unwrap(), expected);
    assert_eq!(encode("é").unwrap(), [0x62, 0xc3, 0xa9]);
}

#[test]
fn old_candid_trailing_and_oversized_inputs_refuse() {
    let value = TenantScope {
        service: Principal::self_authenticating(b"service"),
        namespace: 1,
        tenant: Principal::self_authenticating(b"tenant"),
    };
    assert_eq!(
        decode::<TenantScope>(
            &ic_platform::candid::contract::encode_one(value).unwrap(),
            4096
        ),
        Err(Error::Invalid)
    );
    let mut bytes = encode(value).unwrap();
    assert_eq!(
        decode::<TenantScope>(&bytes, bytes.len() - 1),
        Err(Error::Limit)
    );
    bytes.push(0xf6);
    assert_eq!(decode::<TenantScope>(&bytes, 4096), Err(Error::Trailing));
    assert!(decode::<TenantScope>(&bytes[..3], 4096).is_err());
}

#[test]
fn duplicate_and_unknown_authority_fields_refuse() {
    let value = TenantScope {
        service: Principal::self_authenticating(b"service"),
        namespace: 1,
        tenant: Principal::self_authenticating(b"tenant"),
    };
    let mut map: ciborium::Value = decode(&encode(value).unwrap(), 4096).unwrap();
    let ciborium::Value::Map(fields) = &mut map else {
        panic!("record map")
    };
    fields.push(("retired_authority".into(), true.into()));
    assert_eq!(
        decode::<TenantScope>(&encode(&map).unwrap(), 4096),
        Err(Error::Invalid)
    );
    let ciborium::Value::Map(fields) = &mut map else {
        unreachable!()
    };
    fields.pop();
    fields.push(fields[0].clone());
    assert_eq!(
        decode::<TenantScope>(&encode(&map).unwrap(), 4096),
        Err(Error::Invalid)
    );
}

#[test]
fn nesting_and_output_limits_are_enforced() {
    let mut value = serde_json::Value::Null;
    for _ in 0..MAX_DEPTH + 2 {
        value = serde_json::Value::Array(vec![value]);
    }
    assert_eq!(
        decode::<serde_json::Value>(&encode(value).unwrap(), 4096),
        Err(Error::Invalid)
    );
    assert_eq!(encode("x".repeat(MAX_MESSAGE_BYTES + 1)), Err(Error::Limit));
}
