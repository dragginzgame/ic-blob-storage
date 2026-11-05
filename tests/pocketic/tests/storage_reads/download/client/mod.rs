use super::*;
use blob_test_protocol::storage::read::{DownloadClientInput, DownloadProbeFailure};
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
async fn fetch(
    f: &Fixture,
    input: DownloadClientInput,
) -> Result<DownloadResponse, DownloadProbeFailure> {
    f.harness
        .pic
        .update_control_as(
            f.tenant,
            f.operator,
            "fixture_fetch_descriptor",
            (input,),
            REPLY_BYTES,
        )
        .await
        .map(|(reply,)| reply)
        .unwrap()
}
async fn consumer() -> Fixture {
    let mut f = Fixture::new().await;
    let client = f.harness.pic.create_canister().await.unwrap();
    f.harness
        .pic
        .install_canister(
            client,
            Fixture::wasm(),
            Fixture::installation("fixture_init", f.operator),
            None,
        )
        .await
        .unwrap();
    f.tenant = client;
    f
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "One tenant-canister journey compares valid descriptor delivery with independent wrong-context refusals"
)]
async fn actual_tenant_canister_fetches_bound_descriptor_and_refuses_wrong_contexts() {
    let f = consumer().await;
    let client = f.tenant;
    let (permission, _, input) = f.download_input().await;
    f.expose(permission.request).await.unwrap();
    let selection = DownloadClientInput {
        request: wire(input),
        tenant: client.into(),
        project: "fixture project/β?&=".into(),
        max_reply_bytes: 4096,
    };
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let response = fetch(&f, selection.clone()).await.unwrap();
    assert_eq!(response.request.tenant, client.into());
    assert_eq!(response.request, selection.request);
    assert_eq!(response.owner, f.service.into());
    assert_eq!(response.bytes, 10);
    assert_eq!(
        f.download(f.operator, input).await,
        Err(DownloadFailure::Denied)
    );
    assert_eq!(
        fetch(
            &f,
            DownloadClientInput {
                tenant: f.operator.into(),
                ..selection.clone()
            }
        )
        .await,
        Err(DownloadProbeFailure::Binding)
    );
    assert_eq!(
        fetch(
            &f,
            DownloadClientInput {
                project: "wrong project".into(),
                ..selection.clone()
            }
        )
        .await,
        Err(DownloadProbeFailure::Binding)
    );
    assert_eq!(
        fetch(
            &f,
            DownloadClientInput {
                max_reply_bytes: 1,
                ..selection.clone()
            }
        )
        .await,
        Err(DownloadProbeFailure::Limit)
    );
    let denied: Result<DownloadResponse, DownloadProbeFailure> = f
        .harness
        .pic
        .update_control_as(
            client,
            f.other,
            "fixture_fetch_descriptor",
            (selection.clone(),),
            REPLY_BYTES,
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
    assert_eq!(denied, Err(DownloadProbeFailure::Denied));
    let query: Result<DownloadResponse, DownloadProbeFailure> = f
        .harness
        .pic
        .query_control_as(
            client,
            f.operator,
            "fixture_nonreplicated_descriptor",
            (selection.clone(),),
            REPLY_BYTES,
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
    assert_eq!(query, Err(DownloadProbeFailure::Execution));
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    f.enroll(Some(f.tenant().await.unwrap()), false)
        .await
        .unwrap();
    assert_eq!(
        fetch(&f, selection.clone()).await,
        Err(DownloadProbeFailure::Remote(DownloadFailure::Inactive))
    );
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
    assert_eq!(
        fetch(&f, selection.clone()).await,
        Err(DownloadProbeFailure::Remote(DownloadFailure::Fenced))
    );
    f.harness
        .pic
        .stop_canister(f.service, Some(f.controller))
        .await
        .unwrap();
    assert!(matches!(
        fetch(&f, selection).await,
        Err(DownloadProbeFailure::Rejected(_))
    ));
}
