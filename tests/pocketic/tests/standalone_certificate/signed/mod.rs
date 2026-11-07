//! Two separately signed browser identities against the actual standalone boundary.
use super::*;
use ic_platform::agent::{Agent, BasicIdentity, Identity};
use ic_platform::candid::decoder;
use ic_testkit::pocket_ic::PocketIcBuilder;
use std::time::Duration;

fn agent(url: &str, seed: u8, root_key: &[u8]) -> Agent {
    let agent = Agent::builder()
        .with_url(url)
        .with_identity(BasicIdentity::from_raw_key(&[seed; 32]))
        .with_max_tcp_error_retries(0)
        .with_max_polling_time(Duration::from_secs(20))
        .build()
        .unwrap();
    agent.set_root_key(root_key.to_vec());
    agent
}

async fn update(
    agent: &Agent,
    service: Principal,
    method: &str,
    argument: Vec<u8>,
) -> Result<Vec<u8>, ic_platform::agent::AgentError> {
    (agent
        .update(&service, method)
        .with_arg(argument)
        .call_and_wait())
    .await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "One signed two-user journey preserves identities through certificate refusal and restore"
)]
async fn standalone_project_grants_two_signed_uploaders_without_cross_user_certificate_authority() {
    let mut f = Fixture::with_harness(
        Harness::with_builder(
            PocketIcBuilder::new(crate::support::REQUEST_TIMEOUT)
                .with_nns_subnet()
                .with_application_subnet(),
        )
        .await,
    )
    .await;
    f.tenant = BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap();
    f.enroll(f.operator).await.unwrap();
    let identities =
        [43, 44].map(|seed| BasicIdentity::from_raw_key(&[seed; 32]).sender().unwrap());
    let manifests = identities.map(|uploader| {
        f.uploader = uploader;
        // Distinct content roots and original object/operation identities.
        let mut manifest = f.manifest_bytes(if uploader == identities[0] { 1 } else { 2 });
        let id = if uploader == identities[0] { 1 } else { 2 };
        manifest.permission.upload.upload = id;
        manifest.permission.upload.object = id;
        manifest
    });
    let root_key = f.harness.pic.root_key().await.unwrap().unwrap();
    let url = f.harness.pic.make_live(None).await.unwrap().to_string();
    let tenant = agent(&url, 42, &root_key);
    let uploaders = [agent(&url, 43, &root_key), agent(&url, 44, &root_key)];
    for (manifest, uploader) in manifests.iter().zip(&uploaders) {
        let admission = update(
            &tenant,
            f.service,
            "blob_admit_upload",
            ic_blob_storage_contracts::wire::encode_request(
                "blob_admit_upload",
                manifest.permission,
            )
            .unwrap(),
        )
        .await
        .unwrap();
        ic_blob_storage_contracts::wire::decode_response::<
            Result<UploadAdmissionMutation, UploadAdmissionFailure>,
        >("blob_admit_upload", &admission, 4096)
        .unwrap()
        .unwrap();
        let prepared = update(
            uploader,
            f.service,
            "blob_prepare_upload",
            ic_blob_storage_contracts::wire::encode_request("blob_prepare_upload", manifest)
                .unwrap(),
        )
        .await
        .unwrap();
        ic_blob_storage_contracts::wire::decode_response::<
            Result<UploadManifestMutation, UploadManifestFailure>,
        >("blob_prepare_upload", &prepared, 4096)
        .unwrap()
        .unwrap();
    }
    // Actual signed callers cannot borrow another user's root or grant themselves a tenant role.
    for (index, uploader) in uploaders.iter().enumerate() {
        assert!(
            update(
                uploader,
                f.service,
                ISSUE,
                ic_platform::candid::contract::encode_one(root(&manifests[1 - index])).unwrap()
            )
            .await
            .is_err()
        );
        let denied = update(
            uploader,
            f.service,
            "blob_admit_upload",
            ic_blob_storage_contracts::wire::encode_request(
                "blob_admit_upload",
                manifests[index].permission,
            )
            .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(
            ic_blob_storage_contracts::wire::decode_response::<
                Result<UploadAdmissionMutation, UploadAdmissionFailure>,
            >("blob_admit_upload", &denied, 4096)
            .unwrap(),
            Err(UploadAdmissionFailure::Denied)
        );
    }
    for (manifest, uploader) in manifests.iter().zip(&uploaders) {
        let root = root(manifest);
        let bytes = update(
            uploader,
            f.service,
            ISSUE,
            ic_platform::candid::contract::encode_one(&root).unwrap(),
        )
        .await
        .unwrap();
        let reply: CaffeineUploadCertificateResponse =
            decoder::decode_one_with_config(&bytes, &decoder::DecoderConfig::new()).unwrap();
        assert_eq!(reply.method, "upload");
        assert_eq!(reply.blob_hash, root);
        // IC-Agent verifies the local IC certificate against the explicit emulator root.
        assert!(
            update(
                uploader,
                f.service,
                ISSUE,
                ic_platform::candid::contract::encode_one(&root).unwrap()
            )
            .await
            .is_err()
        );
    }
    f.harness.pic.stop_live().await.unwrap();
    for manifest in &manifests {
        assert_eq!(
            f.admission(manifest.permission).await.state,
            UploadState::Registered
        );
    }
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    for manifest in &manifests {
        assert_eq!(
            inspect(&f, manifest.permission.uploader.into(), &root(manifest)).await,
            Err(E::Permission(UploadAdmissionFailure::Fenced))
        );
        refuses(&f, manifest.permission.uploader.into(), &root(manifest)).await;
    }
}
