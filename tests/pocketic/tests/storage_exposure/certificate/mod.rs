//! Local ingress update payload/rollback evidence, not gateway certificate acceptance.
use super::*;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::provider_calls;
mod ingress;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::protocol::CAFFEINE_UPLOAD_CERTIFICATE_METHOD as METHOD;
#[derive(
    ic_platform::candid::contract::CandidType, ic_platform::candid::contract::serde::Deserialize,
)]
#[candid_path("ic_platform::candid::contract")]
struct ProviderReply {
    method: String,
    blob_hash: String,
}

fn root(permission: Permission) -> String {
    ProviderRootHash::try_from(permission.request.root.as_slice())
        .unwrap()
        .to_string()
}
async fn configure(f: &Fixture, input: ExposureInput) {
    f.harness
        .pic
        .update_control_as::<(), _>(
            f.service,
            f.operator,
            "configure_certificate_fixture",
            (input,),
            REPLY_BYTES,
        )
        .await
        .unwrap();
}
async fn refused(f: &Fixture, actor: Principal, root: &str) {
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let rejection = f
        .harness
        .pic
        .update_call(
            f.service,
            actor,
            METHOD,
            ic_platform::candid::contract::encode_one(root).unwrap(),
        )
        .await
        .unwrap_err();
    assert_eq!(
        crate::canister_rejection(&rejection).reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn certificate_ingress_derives_authority_without_host_fact_substitution() {
    let f = Fixture::new().await;
    f.enroll(None, true).await.unwrap();
    let (permission, declaration) = f.permission(u128::MAX, 8);
    f.admit(f.tenant, permission).await.unwrap();
    let yes = input(&f, permission);
    let root = root(permission);
    refused(&f, f.uploader, &root).await; // Unprepared before any fault configuration.
    let rejection = f
        .harness
        .pic
        .update_call(
            f.service,
            f.uploader,
            "configure_certificate_fixture",
            ic_blob_storage_pocketic_tests::control_calls::encode_control_one(yes).unwrap(),
        )
        .await
        .unwrap_err();
    assert_eq!(
        crate::canister_rejection(&rejection).reject_code,
        RejectCode::CanisterError
    );
    configure(&f, yes).await;
    refused(&f, f.uploader, &root).await; // Still unprepared.
    f.prepare(&declaration).await.unwrap();
    configure(&f, yes).await;
    for actor in [
        f.tenant,
        f.operator,
        f.controller,
        f.other,
        Principal::anonymous(),
    ] {
        refused(&f, actor, &root).await;
    }
    for root in [
        String::new(),
        "sha256:00".into(),
        format!("sha256:{}", "ff".repeat(32)),
        "x".repeat(4096),
    ] {
        refused(&f, f.uploader, &root).await;
    }
    // The provider contract is an update, never a query that could roll back intent.
    assert!(
        f.harness
            .pic
            .query_call(
                f.service,
                f.uploader,
                METHOD,
                ic_platform::candid::contract::encode_one(&root).unwrap()
            )
            .await
            .is_err()
    );
    let (other, other_declaration) = f.permission(7, 9);
    f.admit(f.tenant, other).await.unwrap();
    f.prepare(&other_declaration).await.unwrap();
    refused(&f, f.uploader, &self::root(other)).await; // Configured evidence belongs to the first root.

    // Independent wire shape mirrors official Mixin.mo; no Result wrapper is accepted.
    let reply: ProviderReply =
        provider_calls::update_provider(&f.harness.pic, f.service, f.uploader, METHOD, (&root,))
            .await
            .map(|(reply,)| reply)
            .unwrap();
    assert_eq!(reply.method, "upload");
    assert_eq!(reply.blob_hash, root);
    assert_eq!(
        inspect(&f, f.uploader, yes).await.unwrap().state,
        UploadState::Registered
    );
    assert_eq!(
        inspect(&f, f.uploader, input(&f, other))
            .await
            .unwrap()
            .state,
        UploadState::Reserved
    );
    refused(&f, f.uploader, &root).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn certificate_response_traps_rollback_and_lost_committed_reply_cannot_reissue() {
    let f = Fixture::new().await;
    f.enroll(None, true).await.unwrap();
    let (permission, declaration) = f.permission(1, 1);
    f.admit(f.tenant, permission).await.unwrap();
    f.prepare(&declaration).await.unwrap();
    let yes = input(&f, permission);
    let root = root(permission);
    let totals = f.status().await;
    for fault in [
        ExposureInput {
            trap_write: true,
            ..yes
        },
        ExposureInput {
            trap_after: true,
            ..yes
        },
    ] {
        configure(&f, fault).await;
        refused(&f, f.uploader, &root).await;
        assert_eq!(
            inspect(&f, f.uploader, yes).await.unwrap().state,
            UploadState::Reserved
        );
        assert_eq!(f.status().await, totals);
    }
    configure(&f, yes).await;
    // Ignore a successful committed response, as if the uploader lost its reply.
    f.harness
        .pic
        .update_call(
            f.service,
            f.uploader,
            METHOD,
            ic_platform::candid::contract::encode_one(&root).unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        inspect(&f, f.uploader, yes).await.unwrap().state,
        UploadState::Registered
    );
    refused(&f, f.uploader, &root).await;
    f.revoke(permission).await.unwrap();
    assert_eq!(
        f.status().await,
        Status {
            active: 0,
            bytes: 0,
            ..totals
        }
    );
    let retained = inspect(&f, f.tenant, yes).await.unwrap();
    assert!(retained.revoked);
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation("fixture_post_upgrade", f.operator),
            Some(f.controller),
        )
        .await
        .unwrap();
    assert_eq!(inspect(&f, f.uploader, yes).await, Ok(retained));
    refused(&f, f.uploader, &root).await; // Fixture configuration is absent after upgrade.
    configure(&f, yes).await;
    refused(&f, f.uploader, &root).await; // Even replaced substitute facts cannot unfreeze the owner.
    assert_eq!(inspect(&f, f.uploader, yes).await, Ok(retained));
}
