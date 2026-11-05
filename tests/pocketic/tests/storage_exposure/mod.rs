use super::*;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
mod certificate;
mod snapshot;
use blob_test_protocol::storage::exposure::ExposureInput;
use ic_blob_storage_contracts::dto::upload::UploadState;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure as A;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionResponse;
use ic_blob_storage_contracts::dto::upload::exposure::UploadExposureFailure as E;
fn input(f: &Fixture, permission: Permission) -> ExposureInput {
    assert_eq!(permission.uploader, f.uploader);
    ExposureInput {
        permission: admission_input(permission),
        trap_write: false,
        trap_after: false,
    }
}
async fn expose(
    f: &Fixture,
    actor: Principal,
    input: ExposureInput,
) -> Result<UploadAdmissionResponse, E> {
    f.harness
        .pic
        .update_control_as(f.service, actor, "expose", (input,), REPLY_BYTES)
        .await
        .map(|(reply,)| reply)
        .unwrap()
}
async fn inspect(
    f: &Fixture,
    actor: Principal,
    input: ExposureInput,
) -> Result<UploadAdmissionResponse, E> {
    f.harness
        .pic
        .query_control_as(
            f.service,
            actor,
            "exposure_status",
            (input.permission,),
            REPLY_BYTES,
        )
        .await
        .map(|(reply,)| reply)
        .unwrap()
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn exposure_checks_preparation_permission_and_actual_authority() {
    let f = Fixture::new().await;
    let enrolled = f.enroll(None, true).await.unwrap();
    let (permission, declaration) = f.permission(u128::MAX, 8);
    f.admit(f.tenant, permission).await.unwrap();
    let yes = input(&f, permission);
    assert_eq!(expose(&f, f.uploader, yes).await, Err(E::Unprepared));
    f.prepare(&declaration).await.unwrap();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    for actor in [f.tenant, f.controller, f.operator, f.other] {
        assert_eq!(expose(&f, actor, yes).await, Err(E::Permission(A::Denied)));
    }
    let mut changed = yes;
    changed.permission.expires_at_ns -= 1;
    assert_eq!(
        expose(&f, f.uploader, changed).await,
        Err(E::Permission(A::Conflict))
    );
    assert_eq!(
        inspect(&f, f.tenant, changed).await,
        Err(E::Permission(A::Conflict))
    );
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    f.enroll(Some(enrolled), false).await.unwrap();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(
        expose(&f, f.uploader, yes).await,
        Err(E::Permission(A::Inactive))
    );
    assert_eq!(
        inspect(&f, f.uploader, yes).await.unwrap().state,
        UploadState::Reserved
    );
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn exposure_write_and_response_traps_roll_back_but_lost_committed_acknowledgment_never_allows_reissue()
 {
    let f = Fixture::new().await;
    f.enroll(None, true).await.unwrap();
    let (permission, declaration) = f.permission(1, 1);
    f.admit(f.tenant, permission).await.unwrap();
    f.prepare(&declaration).await.unwrap();
    let yes = input(&f, permission);
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
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
        let rejection = f
            .harness
            .pic
            .update_call(
                f.service,
                f.uploader,
                "expose",
                ic_blob_storage_pocketic_tests::control_calls::encode_control_one(fault).unwrap(),
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
        assert_eq!(f.status().await, totals);
        assert_eq!(
            inspect(&f, f.uploader, yes).await.unwrap().state,
            UploadState::Reserved
        );
    }
    // Discard the application's acknowledgment after the IC update committed.
    f.harness
        .pic
        .update_call(
            f.service,
            f.uploader,
            "expose",
            ic_blob_storage_pocketic_tests::control_calls::encode_control_one(yes).unwrap(),
        )
        .await
        .unwrap();
    let original = inspect(&f, f.uploader, yes).await.unwrap();
    assert_eq!(original.state, UploadState::Registered);
    assert_eq!(original.permission, yes.permission);
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(expose(&f, f.uploader, yes).await, Err(E::Phase));
    for actor in [f.operator, f.controller, f.other] {
        assert_eq!(inspect(&f, actor, yes).await, Err(E::Permission(A::Denied)));
    }
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    f.revoke(permission).await.unwrap();
    assert_eq!(
        f.status().await,
        Status {
            active: 0,
            bytes: 0,
            ..totals
        }
    );
    let withdrawn = inspect(&f, f.tenant, yes).await.unwrap();
    assert!(withdrawn.revoked);
    assert_eq!(withdrawn.state, UploadState::Registered);
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
    assert_eq!(inspect(&f, f.uploader, yes).await, Ok(withdrawn));
    assert_eq!(
        expose(&f, f.uploader, yes).await,
        Err(E::Permission(A::Fenced))
    );
}
