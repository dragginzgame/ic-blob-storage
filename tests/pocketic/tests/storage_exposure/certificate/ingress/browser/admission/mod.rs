//! Actual browser identity -> consumer intent -> tenant admission -> uploader preparation.
use super::*;
use blob_test_protocol::consumer::{
    AssetView, Failure as ConsumerFailure, Fault, Registration, RegistrationSource, Revocation, Run,
};
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionMutation;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestFailure;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestRequest;
use ic_blob_storage_contracts::identity::caffeine::manifest::CaffeineManifestLimits;
use ic_blob_storage_contracts::provider::preparation::PreparedManifestLimits;
use ic_blob_storage_contracts::provider::preparation::decode_prepared_manifest;
use ic_blob_storage_contracts::upload::manifests::reply;
use ic_blob_storage_contracts::upload::manifests::reply::UploadManifestReplyLimits;

pub(super) fn install(f: &mut Fixture) {
    let consumer = f.harness.pic.create_canister_with_settings(
        Some(f.controller),
        Some(CanisterSettings {
            controllers: Some(vec![f.controller]),
            ..CanisterSettings::default()
        }),
    );
    f.harness.pic.install_canister(
        consumer,
        std::fs::read(fixture_path("BLOB_CONSUMER_PROBE_WASM")).unwrap(),
        candid::encode_args((f.uploader, f.service)).unwrap(),
        Some(f.controller),
    );
    f.tenant = consumer;
    f.enroll(None, true).unwrap();
}

fn limits() -> CaffeineManifestLimits {
    CaffeineManifestLimits {
        max_content_bytes: NonZeroU64::new(10).unwrap(),
        max_chunks: NonZeroUsize::MIN,
        max_headers: NonZeroUsize::new(8).unwrap(),
        max_header_bytes: NonZeroUsize::new(1024).unwrap(),
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct AuthorityReplies {
    foreign_admission: Vec<u8>,
    direct_admission: Vec<u8>,
    foreign_preparation: Vec<u8>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct BrowserReplies {
    authority: Option<AuthorityReplies>,
    admission: Vec<u8>,
    preparation: Vec<u8>,
}

pub(super) fn handshake(
    f: &Fixture,
    permission: Permission,
    browser: &BrowserPreparation,
    driver: &mut BrowserDriver,
    verify_authority: bool,
) -> Run {
    let declaration = decode_prepared_manifest(
        browser.hash.parse().unwrap(),
        browser.byte_length,
        browser.manifest_json.as_bytes(),
        PreparedManifestLimits {
            max_json_bytes: NonZeroUsize::new(4096).unwrap(),
            manifest: limits(),
        },
    )
    .unwrap();
    let request = UploadManifestRequest {
        permission: admission_input(permission),
        declaration,
    };
    let run = Run {
        registration: Registration {
            asset: permission.request.id,
            payload: vec![1; 8],
            source: RegistrationSource::Fresh(request.permission),
            release_operation: 2,
        },
        fault: Fault::None,
        hold: false,
        max_reply_bytes: 4096,
    };
    let commands = serde_json::json!({
        "verifyAuthority": verify_authority,
        "admission": candid::encode_one(&run).unwrap(),
        "permission": candid::encode_one(request.permission).unwrap(),
        "preparation": candid::encode_one(&request).unwrap(),
    });
    driver.send(&commands);
    let replies: BrowserReplies = driver.read(32768);
    assert_eq!(replies.authority.is_some(), verify_authority);
    if let Some(authority) = replies.authority {
        verify_refusals(&request, &authority);
    }
    verify_admission(f, &run, &replies.admission);
    let prepared = reply::mutation(
        &request,
        &replies.preparation,
        UploadManifestReplyLimits {
            max_reply_bytes: NonZeroUsize::new(4096).unwrap(),
            declaration: limits(),
        },
    )
    .unwrap();
    assert!(prepared.changed);
    let input = input(f, permission);
    assert_eq!(
        inspect(f, f.uploader, input).unwrap().state,
        UploadState::Reserved
    );
    configure(f, input);
    run
}

fn verify_refusals(request: &UploadManifestRequest, replies: &AuthorityReplies) {
    let denied: Result<AssetView, ConsumerFailure> =
        candid::decode_one(&replies.foreign_admission).unwrap();
    assert_eq!(denied, Err(ConsumerFailure::Denied));
    let denied: Result<UploadAdmissionMutation, A> =
        candid::decode_one(&replies.direct_admission).unwrap();
    assert_eq!(denied, Err(A::Denied));
    assert_eq!(
        reply::mutation(
            request,
            &replies.foreign_preparation,
            UploadManifestReplyLimits {
                max_reply_bytes: NonZeroUsize::new(4096).unwrap(),
                declaration: limits(),
            }
        ),
        Err(reply::UploadManifestReplyError::Remote(
            UploadManifestFailure::Permission(A::Denied)
        ))
    );
}

fn verify_admission(f: &Fixture, run: &Run, bytes: &[u8]) {
    let admitted: Result<AssetView, ConsumerFailure> = candid::decode_one(bytes).unwrap();
    let admitted = admitted.unwrap();
    assert_eq!(admitted.registration, run.registration);
    assert!(admitted.admission_started);
    assert_eq!(admitted.admission_result, Some(Ok(())));
    assert!(!admitted.published);
    let stored: Result<AssetView, ConsumerFailure> = f
        .harness
        .pic
        .query_candid_as(f.tenant, f.uploader, "asset", (run.registration.asset,))
        .unwrap();
    assert_eq!(stored, Ok(admitted));
}

pub(super) fn consumer_commands(run: &Run) -> serde_json::Value {
    serde_json::json!({
        "registration": candid::encode_one(run).unwrap(),
        "asset": candid::encode_one(run.registration.asset).unwrap(),
        "revocation": candid::encode_one(Revocation {
            asset: run.registration.asset, fault: Fault::None, max_reply_bytes: 4096,
        }).unwrap(),
    })
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ConsumerReplies {
    pending: Vec<u8>,
    cancelled: Option<Vec<u8>>,
    withdrawn: Option<Vec<u8>>,
}

pub(super) fn verify_consumer(f: &Fixture, run: &Run, cancelled: bool, value: serde_json::Value) {
    let replies: ConsumerReplies = serde_json::from_value(value).unwrap();
    let view = |bytes: &[u8]| {
        assert!(bytes.len() <= 4096);
        let result: Result<AssetView, ConsumerFailure> = candid::decode_one(bytes).unwrap();
        let view = result.unwrap();
        assert_eq!(view.registration, run.registration);
        assert!(!view.published);
        assert!(!view.retain_started);
        assert_eq!(view.upload_state, Some(UploadState::ExposurePossible));
        view
    };
    let pending = view(&replies.pending);
    assert!(!pending.cancelled);
    assert_eq!(replies.cancelled.is_some(), cancelled);
    assert_eq!(replies.withdrawn.is_some(), cancelled);
    let expected = if cancelled {
        let tombstone = view(replies.cancelled.as_ref().unwrap());
        assert!(tombstone.cancelled);
        assert!(!tombstone.revocation_started);
        let withdrawn = view(replies.withdrawn.as_ref().unwrap());
        assert!(withdrawn.cancelled && withdrawn.revocation_started);
        assert_eq!(withdrawn.revocation_result, Some(Ok(())));
        withdrawn
    } else {
        pending
    };
    let stored: Result<AssetView, ConsumerFailure> = f
        .harness
        .pic
        .query_candid_as(f.tenant, f.uploader, "asset", (run.registration.asset,))
        .unwrap();
    assert_eq!(stored, Ok(expected));
    // Neither an HTTP success nor withdrawal erases the exposed reservation.
    assert_eq!(f.status().bytes, 10);
    assert_eq!(f.status().active, 1);
}
