//! Two separately signed browser identities against the actual standalone boundary.
use super::*;
use ic_agent::{Agent, Identity, identity::BasicIdentity};
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

fn update(
    runtime: &tokio::runtime::Runtime,
    agent: &Agent,
    service: Principal,
    method: &str,
    argument: impl candid::CandidType,
) -> Result<Vec<u8>, ic_agent::AgentError> {
    runtime.block_on(
        agent
            .update(&service, method)
            .with_arg(candid::encode_one(argument).unwrap())
            .call_and_wait(),
    )
}

#[test]
fn standalone_project_grants_two_signed_uploaders_without_cross_user_certificate_authority() {
    let mut f = Fixture::with_harness(Harness::with_builder(
        PocketIcBuilder::new()
            .with_nns_subnet()
            .with_application_subnet(),
    ));
    f.tenant = BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap();
    f.enroll(f.operator).unwrap();
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
    let root_key = f.harness.pic.root_key().unwrap();
    let url = f.harness.pic.make_live(None).to_string();
    let tenant = agent(&url, 42, &root_key);
    let uploaders = [agent(&url, 43, &root_key), agent(&url, 44, &root_key)];
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    for (manifest, uploader) in manifests.iter().zip(&uploaders) {
        let admission = update(
            &runtime,
            &tenant,
            f.service,
            "blob_admit_upload",
            manifest.permission,
        )
        .unwrap();
        candid::decode_one::<Result<UploadAdmissionMutation, UploadAdmissionFailure>>(&admission)
            .unwrap()
            .unwrap();
        let prepared = update(
            &runtime,
            uploader,
            f.service,
            "blob_prepare_upload",
            manifest,
        )
        .unwrap();
        candid::decode_one::<Result<UploadManifestMutation, UploadManifestFailure>>(&prepared)
            .unwrap()
            .unwrap();
    }
    // Actual signed callers cannot borrow another user's root or grant themselves a tenant role.
    for (index, uploader) in uploaders.iter().enumerate() {
        assert!(
            update(
                &runtime,
                uploader,
                f.service,
                ISSUE,
                root(&manifests[1 - index])
            )
            .is_err()
        );
        let denied = update(
            &runtime,
            uploader,
            f.service,
            "blob_admit_upload",
            manifests[index].permission,
        )
        .unwrap();
        assert_eq!(
            candid::decode_one::<Result<UploadAdmissionMutation, UploadAdmissionFailure>>(&denied)
                .unwrap(),
            Err(UploadAdmissionFailure::Denied)
        );
    }
    for (manifest, uploader) in manifests.iter().zip(&uploaders) {
        let root = root(manifest);
        let bytes = update(&runtime, uploader, f.service, ISSUE, &root).unwrap();
        let reply: CaffeineUploadCertificateResponse = candid::decode_one(&bytes).unwrap();
        assert_eq!(reply.method, "upload");
        assert_eq!(reply.blob_hash, root);
        // IC-Agent verifies the local IC certificate against the explicit emulator root.
        assert!(update(&runtime, uploader, f.service, ISSUE, &root).is_err());
    }
    f.harness.pic.stop_live();
    for manifest in &manifests {
        assert_eq!(
            f.admission(manifest.permission).state,
            UploadState::ExposurePossible
        );
    }
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    for manifest in &manifests {
        assert_eq!(
            inspect(&f, manifest.permission.uploader, &root(manifest)),
            Err(E::Permission(UploadAdmissionFailure::Fenced))
        );
        refuses(&f, manifest.permission.uploader, &root(manifest));
    }
}
