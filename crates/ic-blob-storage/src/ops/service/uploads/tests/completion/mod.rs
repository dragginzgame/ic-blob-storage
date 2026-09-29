use super::*;
use crate::{
    dto::upload::{admission::UploadAdmissionFailure as A, completion::*},
    model::service::upload::completion::CompletionAuthority,
    workflow::uploads::{
        admission::admit,
        completion::{attest, inspect},
        manifests::prepare,
    },
};
fn authority() -> CompletionAuthority {
    CompletionAuthority::new(p(1), NonZeroU128::MIN, p(9)).unwrap()
}

#[test]
fn attestation_binds_role_permission_and_observation_without_bypassing_exposure() {
    let mut store = StableUploads::install(memory(), config()).unwrap();
    enroll(&mut store);
    let manifest = manifest_boundary::input();
    let permission = manifest.permission;
    admit(&mut store, context(4), permission, 1).unwrap();
    prepare(&mut store, context(5), &manifest, 2).unwrap();
    let statement = UploadAttestationRequest {
        permission,
        content_digest: [17; 32],
        observed_at_ns: 5,
    };
    for actor in [2, 3, 4, 5] {
        assert_eq!(
            attest(&mut store, authority(), context(actor), &statement, 6),
            Err(UploadAttestationFailure::Denied)
        );
    }
    assert_eq!(
        attest(&mut store, authority(), context(9), &statement, 6),
        Err(UploadAttestationFailure::Phase)
    );
    let model = admission::parse_binding(p(1), permission).unwrap();
    store.expose(context(5), model.request, 3).unwrap();
    let changed = UploadAttestationRequest {
        observed_at_ns: 7,
        ..statement
    };
    assert_eq!(
        attest(&mut store, authority(), context(9), &changed, 6),
        Err(UploadAttestationFailure::Observation)
    );
    let mut wrong = statement;
    wrong.permission.upload.first_reference += 1;
    assert!(matches!(
        attest(&mut store, authority(), context(9), &wrong, 6),
        Err(UploadAttestationFailure::Permission(A::Conflict))
    ));
    let wrong = CompletionAuthority::new(p(1), NonZeroU128::new(2).unwrap(), p(9)).unwrap();
    assert_eq!(
        attest(&mut store, wrong, context(9), &statement, 6),
        Err(UploadAttestationFailure::Permission(A::Binding))
    );
    let accepted = attest(&mut store, authority(), context(9), &statement, 6).unwrap();
    assert!(accepted.changed);
    assert_eq!(accepted.receipt.verifier, p(9));
    assert_eq!(accepted.receipt.accepted_at_ns, 6);
    let usage = store.usage().unwrap();
    assert_eq!(usage.physical_bytes, 10);
    assert_eq!(usage.liability_bytes, 10);
    assert!(
        !attest(&mut store, authority(), context(9), &statement, 99)
            .unwrap()
            .changed
    );
    assert_eq!(store.usage().unwrap(), usage);
    let mut different = statement;
    different.content_digest[0] ^= 1;
    assert_eq!(
        attest(&mut store, authority(), context(9), &different, 99),
        Err(UploadAttestationFailure::Conflict)
    );
}
#[test]
fn late_attestation_survives_revocation_and_fenced_restore_as_immutable_evidence() {
    let m = memory();
    let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
    let enrolled = enroll(&mut store);
    let manifest = manifest_boundary::input();
    let permission = manifest.permission;
    let model = admission::parse_binding(p(1), permission).unwrap();
    admit(&mut store, context(4), permission, 1).unwrap();
    prepare(&mut store, context(5), &manifest, 2).unwrap();
    store.expose(context(5), model.request, 3).unwrap();
    store.revoke(context(4), model.request).unwrap();
    store
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(4),
                expected: Some(enrolled),
                active: false,
            },
        )
        .unwrap();
    let input = UploadAttestationRequest {
        permission,
        content_digest: [18; 32],
        observed_at_ns: 4,
    };
    let accepted = attest(&mut store, authority(), context(9), &input, 101).unwrap();
    let usage = store.usage().unwrap();
    let mut restored = StableUploads::open(clone_memory(&m), config()).unwrap();
    let response = inspect(&restored, authority(), context(4), permission).unwrap();
    assert!(response.fenced);
    assert_eq!(
        response.attestation,
        UploadAttestationLookup::Found(accepted.receipt)
    );
    assert_eq!(
        attest(&mut restored, authority(), context(9), &input, 102),
        Err(UploadAttestationFailure::Permission(A::Fenced))
    );
    assert_eq!(restored.usage().unwrap(), usage);
}
