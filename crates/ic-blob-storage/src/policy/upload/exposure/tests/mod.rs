use super::*;

fn envelope() -> RestrictedUploadEnvelope {
    RestrictedUploadEnvelope {
        tenants: 1,
        objects: 1,
        tenant_objects: 1,
        object_bytes: 1024,
        physical_bytes: 1024,
        liability_bytes: 1024,
        tenant_bytes: 1024,
        references: 1,
        receipts: 2,
        active: 1,
        tenant_active: 1,
    }
}

#[test]
fn restricted_bounds_preserve_cleanup_and_refuse_zero_or_larger_lifetime_exposure() {
    let accepted = envelope();
    assert!(accepted.permits(1));
    assert!(accepted.permits(1024));
    assert!(!accepted.permits(0));
    assert!(!accepted.permits(1025));
    let changes: [fn(&mut RestrictedUploadEnvelope); 12] = [
        |e| e.tenants = 2,
        |e| e.objects = 2,
        |e| e.tenant_objects = 2,
        |e| e.references = 2,
        |e| e.receipts = 1,
        |e| e.active = 2,
        |e| e.tenant_active = 2,
        |e| e.object_bytes = 1025,
        |e| e.physical_bytes = 1025,
        |e| e.liability_bytes = 1025,
        |e| e.tenant_bytes = 1025,
        |e| e.object_bytes = 0,
    ];
    for change in changes {
        let mut e = accepted;
        change(&mut e);
        assert!(!e.permits(1));
    }
    assert!(
        !RestrictedUploadEnvelope {
            object_bytes: 10,
            ..accepted
        }
        .permits(11)
    );
}
