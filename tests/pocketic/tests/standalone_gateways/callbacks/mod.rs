//! Real mounted Candid callbacks; Cashier membership is a labelled local substitute.
use super::*;
use ic_blob_storage_contracts::dto::gateway::sync::{GatewaySyncFailure, GatewaySyncResponse};
use ic_blob_storage_contracts::dto::reference::ReferenceAction;
use ic_blob_storage_contracts::dto::upload::certificate::CaffeineUploadCertificateResponse;
use ic_blob_storage_pocketic_tests::fixture_calls;

fn live(f: &Fixture, roots: Vec<Vec<u8>>) -> Result<Vec<bool>, Box<dyn std::error::Error>> {
    fixture_calls::query_fixture::<(Vec<bool>,), _>(
        &f.harness.pic,
        f.service,
        Principal::anonymous(),
        "_immutableObjectStorageBlobsAreLive",
        (roots,),
    )
    .map(|r| r.0)
}
fn deletions(f: &Fixture, actor: Principal) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
    fixture_calls::query_fixture::<(Vec<Vec<u8>>,), _>(
        &f.harness.pic,
        f.service,
        actor,
        "_immutableObjectStorageBlobsToDelete",
        (),
    )
    .map(|r| r.0)
}
fn acknowledge(
    f: &Fixture,
    actor: Principal,
    roots: Vec<Vec<u8>>,
) -> Result<(), Box<dyn std::error::Error>> {
    fixture_calls::update_fixture::<(), _>(
        &f.harness.pic,
        f.service,
        actor,
        "_immutableObjectStorageConfirmBlobDeletion",
        (roots,),
    )
}
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One installed original root is carried through reference release, callback replay and restoration"
)]
fn mounted_gc_callbacks_protect_references_and_retain_acknowledgments_without_settlement() {
    let f = sync::fixture_with_operator(Harness::new(), fake_principal(2));
    let gateway = fake_principal(9);
    let synced: Result<GatewaySyncResponse, GatewaySyncFailure> = f
        .harness
        .pic
        .update_blob_as(
            f.service,
            f.operator,
            "blob_sync_gateways",
            f.operator_scope(),
            REPLY_LIMITS.reply_bytes,
        )
        .unwrap();
    synced.unwrap();
    f.enroll(f.operator).unwrap();
    let manifest = f.small_manifest();
    let upload = manifest.permission.upload;
    let roots = vec![upload.root.to_vec()];
    f.harness
        .pic
        .update_blob_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            manifest.permission,
            REPLY_LIMITS.reply_bytes,
        )
        .unwrap()
        .unwrap();
    assert_eq!(live(&f, roots.clone()).unwrap(), vec![true]);
    f.prepare(f.uploader, &manifest).unwrap();
    let root =
        ic_blob_storage_contracts::identity::ProviderRootHash::try_from(upload.root.as_slice())
            .unwrap()
            .to_string();
    fixture_calls::update_fixture::<(CaffeineUploadCertificateResponse,), _>(
        &f.harness.pic,
        f.service,
        f.uploader,
        "_immutableObjectStorageCreateCertificate",
        (root,),
    )
    .unwrap();
    for actor in [f.tenant, f.operator, f.controller, Principal::anonymous()] {
        assert!(deletions(&f, actor).is_err());
        assert!(acknowledge(&f, actor, vec![]).is_err());
    }
    assert!(acknowledge(&f, gateway, roots.clone()).is_err());
    assert_eq!(deletions(&f, gateway).unwrap(), Vec::<Vec<u8>>::new());
    assert_eq!(
        live(
            &f,
            vec![vec![88; 32], upload.root.to_vec(), upload.root.to_vec()]
        )
        .unwrap(),
        vec![true, true, true]
    );
    for malformed in [vec![], vec![0; 31], vec![0; 33]] {
        assert!(live(&f, vec![malformed.clone()]).is_err());
        assert!(acknowledge(&f, gateway, vec![malformed]).is_err());
    }
    let reference =
        |operation, reference, action| -> Result<ReferenceMutationResponse, ReferenceFailure> {
            f.harness
                .pic
                .update_blob_as(
                    f.service,
                    f.tenant,
                    "blob_apply_reference",
                    ReferenceCommand {
                        upload,
                        operation,
                        reference,
                        action,
                    },
                    REPLY_LIMITS.reply_bytes,
                )
                .unwrap()
        };
    reference(1, 99, ReferenceAction::Retain).unwrap();
    reference(2, upload.first_reference, ReferenceAction::Release).unwrap();
    assert_eq!(live(&f, roots.clone()).unwrap(), vec![true]);
    assert_eq!(deletions(&f, gateway).unwrap(), Vec::<Vec<u8>>::new());
    reference(3, 99, ReferenceAction::Release).unwrap();
    assert_eq!(live(&f, roots.clone()).unwrap(), vec![false]);
    assert_eq!(deletions(&f, gateway).unwrap(), roots);
    let obligations = f
        .local_status(f.operator, f.operator_scope())
        .unwrap()
        .uploads;
    assert!(acknowledge(&f, gateway, vec![upload.root.to_vec(), vec![88; 32]]).is_err());
    assert_eq!(deletions(&f, gateway).unwrap(), roots);
    acknowledge(
        &f,
        gateway,
        vec![upload.root.to_vec(), upload.root.to_vec()],
    )
    .unwrap();
    acknowledge(&f, gateway, roots.clone()).unwrap();
    assert_eq!(deletions(&f, gateway).unwrap(), Vec::<Vec<u8>>::new());
    assert_eq!(
        f.local_status(f.operator, f.operator_scope())
            .unwrap()
            .uploads,
        obligations
    );
    assert!(obligations.physical_bytes > 0 && obligations.liability_bytes > 0);
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .unwrap();
    assert!(live(&f, roots.clone()).is_err());
    assert!(deletions(&f, gateway).is_err());
    f.resume(f.operator).unwrap();
    assert_eq!(deletions(&f, gateway).unwrap(), Vec::<Vec<u8>>::new());
    acknowledge(&f, gateway, roots).unwrap();
    assert_eq!(
        f.local_status(f.operator, f.operator_scope())
            .unwrap()
            .uploads
            .physical_bytes,
        obligations.physical_bytes
    );
}
