use super::*;
use ic_platform::principal::ExternalPrincipal;
use ic_storage::adapter::Storable;

#[test]
fn private_keys_preserve_tenant_then_numeric_order_and_exact_round_trips() {
    let principals = [
        Principal::management_canister(),
        Principal::anonymous(),
        Principal::from_slice(&[1]),
        Principal::from_slice(&[1, 2]),
    ];
    for left in principals {
        for right in principals {
            let old_left: ExternalPrincipal = left.into();
            let old_right: ExternalPrincipal = right.into();
            assert_eq!(
                TenantKeyRecord::from(left).cmp(&right.into()),
                old_left.cmp(&old_right)
            );
        }
        let tenant = TenantKeyRecord::from(left);
        assert_eq!(
            TenantKeyRecord::from_bytes(tenant.to_bytes().unwrap()).unwrap(),
            tenant
        );
        let first = UploadKeyRecord(tenant, 1);
        let last = UploadKeyRecord(tenant, u128::MAX);
        assert!(first < last);
        assert_eq!(
            UploadKeyRecord::from_bytes(last.to_bytes().unwrap()).unwrap(),
            last
        );
        let manifest = ManifestKeyRecord(last, 1);
        assert_eq!(
            ManifestKeyRecord::from_bytes(manifest.to_bytes().unwrap()).unwrap(),
            manifest
        );
        let reference = ReferenceKeyRecord(last, u128::MAX);
        assert_eq!(
            ReferenceKeyRecord::from_bytes(reference.to_bytes().unwrap()).unwrap(),
            reference
        );
        assert!(ReferenceKeyRecord(first, u128::MAX) < ReferenceKeyRecord(last, 1));
        assert!(UploadKeyRecord::from_bytes(reference.to_bytes().unwrap()).is_err());
    }
}
