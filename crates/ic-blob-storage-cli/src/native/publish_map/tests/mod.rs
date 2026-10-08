//! Decoder and map correlation checks; platform state is qualified in `PocketIC`.
use super::*;
use crate::native::publish_check::tests as fixture;
use ic_blob_storage_contracts::dto::reference::ReferenceFailure;
use ic_blob_storage_contracts::dto::reference::status::ReferenceStatusResponse;
use ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityFailure;
use ic_blob_storage_contracts::dto::upload::completion::UploadAttestationFailure;
use ic_blob_storage_contracts::dto::upload::completion::UploadAttestationReceipt;
use ic_blob_storage_contracts::dto::upload::completion::UploadAttestationRequest;
use ic_blob_storage_contracts::dto::upload::completion::UploadAttestationResponse;

enum Replies {
    Valid,
    WrongProject,
    WrongDigest,
    Inactive,
    Fenced,
    Transport,
}

fn observe(replies: &Replies) -> Result<Value, Failure> {
    let directory = fixture::frozen();
    let batch = fixture::batch(&directory);
    let permission = batch.files[0].input.permission;
    let scope = publish_check::observation::scope(
        &batch.files,
        permission.upload.service,
        permission.upload.namespace,
        permission.upload.tenant,
    )
    .unwrap();
    let input: ServiceInstallationInput = exact_candid::decode(
        &batch.installation,
        exact_candid::INSTALLATION_BYTES,
        64,
        100_000,
    )
    .unwrap();
    let mut host = HostConfigurationView {
        configuration: input.configuration,
        project: input.project,
        completion_verifier: input.completion_verifier,
        trusted_uploader: input.trusted_uploader,
        release: ic_blob_storage_contracts::CONTRACT_VERSION.into(),
        fenced: matches!(replies, Replies::Fenced),
    };
    if matches!(replies, Replies::WrongProject) {
        host.project.push_str("-wrong");
    }
    let mut digest =
        *ic_blob_storage_contracts::identity::ContentDigest::compute(b"abc").as_bytes();
    if matches!(replies, Replies::WrongDigest) {
        digest[0] ^= 1;
    }
    let run = Run::create(&directory.path().join("map")).unwrap();
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(inspect(
            &batch,
            permission.upload.tenant,
            &Url::parse("https://gateway.example/").unwrap(),
            Selection::Batch,
            &run,
            |method, args| {
                let value = match method {
                    "blob_configuration" => {
                        candid::encode_one(Ok::<_, HostFailure>(&host)).unwrap()
                    }
                    UPLOAD_CAPACITY_METHOD => {
                        candid::encode_one(Ok::<_, UploadCapacityFailure>(fixture::capacity(scope)))
                            .unwrap()
                    }
                    UPLOAD_ATTESTATION_METHOD => candid::encode_one(Ok::<
                        _,
                        UploadAttestationFailure,
                    >(
                        UploadAttestationResponse {
                            permission,
                            fenced: false,
                            attestation: UploadAttestationLookup::Found(UploadAttestationReceipt {
                                request: UploadAttestationRequest {
                                    permission,
                                    content_digest: digest,
                                    observed_at_ns: 10,
                                },
                                verifier: input.completion_verifier,
                                accepted_at_ns: 11,
                            }),
                        },
                    ))
                    .unwrap(),
                    REFERENCE_STATUS_METHOD => {
                        if matches!(replies, Replies::Transport) {
                            return std::future::ready(Err(Failure::Transport));
                        }
                        let request = candid::decode_one(&args).unwrap();
                        candid::encode_one(Ok::<_, ReferenceFailure>(ReferenceStatusResponse {
                            request,
                            live: !matches!(replies, Replies::Inactive),
                            fenced: false,
                        }))
                        .unwrap()
                    }
                    _ => panic!("unexpected query method"),
                };
                std::future::ready(Ok(value))
            },
        ))
        .map(|observation| observation.report)
}

#[test]
fn map_requires_exact_installed_project_matching_digest_live_reference_and_no_fence() {
    let passed = observe(&Replies::Valid).unwrap();
    assert_eq!(passed["all_references_live"], true);
    assert_eq!(
        passed["files"][0]["body_sha256"],
        crate::native::upload_inputs::digest(b"abc")
    );
    let url = Url::parse(passed["files"][0]["url"].as_str().unwrap()).unwrap();
    assert_eq!(
        url.query_pairs()
            .find(|(k, _)| k == "project_id")
            .unwrap()
            .1,
        "fixture-project"
    );
    assert_eq!(passed["publication_lease"], false);
    assert_eq!(observe(&Replies::WrongProject), Err(Failure::Binding));
    for (replies, code) in [
        (Replies::WrongDigest, "completion_unmatched"),
        (Replies::Inactive, "reference_inactive"),
        (Replies::Fenced, "fenced"),
    ] {
        let blocked = observe(&replies).unwrap();
        assert_eq!(blocked["all_references_live"], false);
        assert_eq!(blocked["blockers"][0]["code"], code);
    }
    assert_eq!(observe(&Replies::Transport), Err(Failure::Transport));
}
