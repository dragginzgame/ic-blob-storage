//! Actual IC rollback/replay/restore for trusted verifier statements (content source substituted).
use super::*;
use ic_blob_storage::dto::reference::{
    ReferenceAction, ReferenceCommand, ReferenceFailure, ReferenceMutationResponse,
};
use ic_blob_storage::dto::upload::{admission::UploadAdmissionFailure as A, completion::*};

fn inspect(f: &Fixture, statement: &UploadAttestationRequest) -> UploadAttestationResponse {
    f.harness
        .pic
        .query_candid_as::<Result<_, UploadAttestationFailure>, _>(
            f.service,
            f.tenant,
            "blob_upload_attestation",
            (statement.permission,),
        )
        .unwrap()
        .unwrap()
}
fn attest(
    f: &Fixture,
    caller: Principal,
    statement: &UploadAttestationRequest,
) -> Result<UploadAttestationMutation, UploadAttestationFailure> {
    f.harness
        .pic
        .update_candid_as(f.service, caller, "blob_attest_upload", (statement,))
        .unwrap()
}
fn check_write_rollback(f: &Fixture, statement: &UploadAttestationRequest) {
    for fault in [
        WriteFault::Confirmed,
        WriteFault::References,
        WriteFault::Permissions,
        WriteFault::Usage,
    ] {
        let before = f.harness.pic.get_stable_memory(f.service);
        let result = f
            .harness
            .pic
            .update_candid_as::<Result<UploadAttestationMutation, UploadAttestationFailure>, _>(
                f.service,
                f.operator,
                "attest_with_write_trap",
                (statement, fault),
            );
        assert!(result.is_err());
        assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
        assert_eq!(
            inspect(f, statement).attestation,
            UploadAttestationLookup::Absent
        );
    }
}
#[test]
fn verifier_attestation_rolls_back_replays_without_resurrection_and_survives_restore() {
    let f = Fixture::new();
    f.enroll(None, true).unwrap();
    let (permission, manifest) = f.permission(u128::MAX, 4);
    f.admit(f.tenant, permission).unwrap();
    f.prepare(&manifest).unwrap();
    f.expose(permission.request).unwrap();
    let statement = UploadAttestationRequest {
        permission: admission_input(permission),
        content_digest: [17; 32],
        observed_at_ns: f.harness.pic.get_time().as_nanos_since_unix_epoch(),
    };
    for actor in [
        f.controller,
        f.tenant,
        f.uploader,
        f.other,
        Principal::anonymous(),
    ] {
        assert_eq!(
            attest(&f, actor, &statement),
            Err(UploadAttestationFailure::Denied)
        );
    }
    check_write_rollback(&f, &statement);
    let accepted = attest(&f, f.operator, &statement).unwrap();
    assert!(accepted.changed);
    let status = f.status();
    let replay = attest(&f, f.operator, &statement).unwrap();
    assert!(!replay.changed);
    assert_eq!(replay.receipt, accepted.receipt);
    assert_eq!(f.status(), status);
    let mut changed = statement;
    changed.content_digest[0] ^= 1;
    assert_eq!(
        attest(&f, f.operator, &changed),
        Err(UploadAttestationFailure::Conflict)
    );
    let released: Result<ReferenceMutationResponse, ReferenceFailure> = f
        .harness
        .pic
        .update_candid_as(
            f.service,
            f.tenant,
            "blob_apply_reference",
            (ReferenceCommand {
                upload: statement.permission.upload,
                operation: 1,
                reference: statement.permission.upload.first_reference,
                action: ReferenceAction::Release,
            },),
        )
        .unwrap();
    released.unwrap();
    let released = f.status();
    assert!(!attest(&f, f.operator, &statement).unwrap().changed);
    assert_eq!(f.status(), released);
    f.fact(
        permission.request,
        blob_test_protocol::storage::ProviderFact::Deleted,
    )
    .unwrap();
    f.fact(
        permission.request,
        blob_test_protocol::storage::ProviderFact::Settled,
    )
    .unwrap();
    assert_eq!(
        inspect(&f, &statement).attestation,
        UploadAttestationLookup::Found(accepted.receipt)
    );
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            candid::encode_one(f.operator).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    let restored = inspect(&f, &statement);
    assert!(restored.fenced);
    assert_eq!(
        restored.attestation,
        UploadAttestationLookup::Found(accepted.receipt)
    );
    assert_eq!(
        attest(&f, f.operator, &statement),
        Err(UploadAttestationFailure::Permission(A::Fenced))
    );
}
