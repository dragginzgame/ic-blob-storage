//! Real mounted Candid callbacks; Cashier membership is a labelled local substitute.
use super::*;
use ic_blob_storage_contracts::dto::gateway::sync::{GatewaySyncFailure, GatewaySyncResponse};
use ic_blob_storage_contracts::dto::reference::ReferenceAction;
use ic_blob_storage_contracts::dto::upload::certificate::CaffeineUploadCertificateResponse;
use ic_blob_storage_pocketic_tests::provider_calls;

async fn live(f: &Fixture, roots: Vec<Vec<u8>>) -> Result<Vec<bool>, Box<dyn std::error::Error>> {
    provider_calls::query_provider::<(Vec<bool>,), _>(
        &f.harness.pic,
        f.service,
        Principal::anonymous(),
        "_immutableObjectStorageBlobsAreLive",
        (roots,),
    )
    .await
    .map(|r| r.0)
}
async fn deletions(
    f: &Fixture,
    actor: Principal,
) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
    provider_calls::query_provider::<(Vec<Vec<u8>>,), _>(
        &f.harness.pic,
        f.service,
        actor,
        "_immutableObjectStorageBlobsToDelete",
        (),
    )
    .await
    .map(|r| r.0)
}
async fn acknowledge(
    f: &Fixture,
    actor: Principal,
    roots: Vec<Vec<u8>>,
) -> Result<(), Box<dyn std::error::Error>> {
    provider_calls::update_provider::<(), _>(
        &f.harness.pic,
        f.service,
        actor,
        "_immutableObjectStorageConfirmBlobDeletion",
        (roots,),
    )
    .await
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "One installed original root is carried through reference release, callback replay and restoration"
)]
async fn mounted_gc_callbacks_protect_references_and_retain_acknowledgments_without_settlement() {
    let f = sync::fixture_with_operator(Harness::new().await, fake_principal(2)).await;
    let gateway = fake_principal(9);
    let synced: Result<GatewaySyncResponse, GatewaySyncFailure> = f
        .harness
        .pic
        .update_blob_as(
            f.service,
            f.operator,
            "blob_sync_gateways",
            f.operator_scope(),
            REPLY_BYTES,
        )
        .await
        .unwrap();
    synced.unwrap();
    f.enroll(f.operator).await.unwrap();
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
            REPLY_BYTES,
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(live(&f, roots.clone()).await.unwrap(), vec![true]);
    f.prepare(f.uploader, &manifest).await.unwrap();
    let root =
        ic_blob_storage_contracts::identity::ProviderRootHash::try_from(upload.root.as_slice())
            .unwrap()
            .to_string();
    provider_calls::update_provider::<(CaffeineUploadCertificateResponse,), _>(
        &f.harness.pic,
        f.service,
        f.uploader,
        "_immutableObjectStorageCreateCertificate",
        (root,),
    )
    .await
    .unwrap();
    for actor in [f.tenant, f.operator, f.controller, Principal::anonymous()] {
        assert!(deletions(&f, actor).await.is_err());
        assert!(acknowledge(&f, actor, vec![]).await.is_err());
    }
    assert!(acknowledge(&f, gateway, roots.clone()).await.is_err());
    assert_eq!(deletions(&f, gateway).await.unwrap(), Vec::<Vec<u8>>::new());
    assert_eq!(
        live(
            &f,
            vec![vec![88; 32], upload.root.to_vec(), upload.root.to_vec()]
        )
        .await
        .unwrap(),
        vec![true, true, true]
    );
    for malformed in [vec![], vec![0; 31], vec![0; 33]] {
        assert!(live(&f, vec![malformed.clone()]).await.is_err());
        assert!(acknowledge(&f, gateway, vec![malformed]).await.is_err());
    }
    let reference = async |operation,
                           reference,
                           action|
           -> Result<ReferenceMutationResponse, ReferenceFailure> {
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
                REPLY_BYTES,
            )
            .await
            .unwrap()
    };
    reference(1, 99, ReferenceAction::Retain).await.unwrap();
    reference(2, upload.first_reference, ReferenceAction::Release)
        .await
        .unwrap();
    assert_eq!(live(&f, roots.clone()).await.unwrap(), vec![true]);
    assert_eq!(deletions(&f, gateway).await.unwrap(), Vec::<Vec<u8>>::new());
    reference(3, 99, ReferenceAction::Release).await.unwrap();
    assert_eq!(live(&f, roots.clone()).await.unwrap(), vec![false]);
    assert_eq!(deletions(&f, gateway).await.unwrap(), roots);
    let obligations = f
        .local_status(f.operator, f.operator_scope())
        .await
        .unwrap()
        .uploads;
    assert!(
        acknowledge(&f, gateway, vec![upload.root.to_vec(), vec![88; 32]])
            .await
            .is_err()
    );
    assert_eq!(deletions(&f, gateway).await.unwrap(), roots);
    acknowledge(
        &f,
        gateway,
        vec![upload.root.to_vec(), upload.root.to_vec()],
    )
    .await
    .unwrap();
    acknowledge(&f, gateway, roots.clone()).await.unwrap();
    assert_eq!(deletions(&f, gateway).await.unwrap(), Vec::<Vec<u8>>::new());
    assert_eq!(
        f.local_status(f.operator, f.operator_scope())
            .await
            .unwrap()
            .uploads,
        obligations
    );
    assert!(obligations.physical_bytes > 0 && obligations.liability_bytes > 0);
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    assert!(live(&f, roots.clone()).await.is_err());
    assert!(deletions(&f, gateway).await.is_err());
    f.resume(f.operator).await.unwrap();
    assert_eq!(deletions(&f, gateway).await.unwrap(), Vec::<Vec<u8>>::new());
    acknowledge(&f, gateway, roots).await.unwrap();
    assert_eq!(
        f.local_status(f.operator, f.operator_scope())
            .await
            .unwrap()
            .uploads
            .physical_bytes,
        obligations.physical_bytes
    );
}
