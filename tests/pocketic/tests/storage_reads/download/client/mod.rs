use super::*;
use blob_test_protocol::storage::read::{DownloadClientInput, DownloadProbeFailure};
fn fetch(
    f: &Fixture,
    input: DownloadClientInput,
) -> Result<DownloadResponse, DownloadProbeFailure> {
    f.harness
        .pic
        .update_candid_as(f.tenant, f.operator, "fixture_fetch_descriptor", (input,))
        .unwrap()
}
fn consumer() -> Fixture {
    let mut f = Fixture::new();
    let client = f.harness.pic.create_canister();
    f.harness.pic.install_canister(
        client,
        Fixture::wasm(),
        candid::encode_one(f.operator).unwrap(),
        None,
    );
    f.tenant = client;
    f
}
#[test]
fn actual_tenant_canister_fetches_bound_descriptor_and_refuses_wrong_contexts() {
    let f = consumer();
    let client = f.tenant;
    let (permission, _, input) = f.download_input();
    f.expose(permission.request).unwrap();
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    let selection = DownloadClientInput {
        request: wire(input),
        tenant: client,
        project: "fixture project/β?&=".into(),
        max_reply_bytes: 4096,
    };
    let before = f.harness.pic.get_stable_memory(f.service);
    let response = fetch(&f, selection.clone()).unwrap();
    assert_eq!(response.request.tenant, client);
    assert_eq!(response.request, selection.request);
    assert_eq!(response.owner, f.service);
    assert_eq!(response.bytes, 10);
    assert_eq!(f.download(f.operator, input), Err(DownloadFailure::Denied));
    assert_eq!(
        fetch(
            &f,
            DownloadClientInput {
                tenant: f.operator,
                ..selection.clone()
            }
        ),
        Err(DownloadProbeFailure::Binding)
    );
    assert_eq!(
        fetch(
            &f,
            DownloadClientInput {
                project: "wrong project".into(),
                ..selection.clone()
            }
        ),
        Err(DownloadProbeFailure::Binding)
    );
    assert_eq!(
        fetch(
            &f,
            DownloadClientInput {
                max_reply_bytes: 1,
                ..selection.clone()
            }
        ),
        Err(DownloadProbeFailure::Limit)
    );
    let denied: Result<DownloadResponse, DownloadProbeFailure> = f
        .harness
        .pic
        .update_candid_as(
            client,
            f.other,
            "fixture_fetch_descriptor",
            (selection.clone(),),
        )
        .unwrap();
    assert_eq!(denied, Err(DownloadProbeFailure::Denied));
    let query: Result<DownloadResponse, DownloadProbeFailure> = f
        .harness
        .pic
        .query_candid_as(
            client,
            f.operator,
            "fixture_nonreplicated_descriptor",
            (selection.clone(),),
        )
        .unwrap();
    assert_eq!(query, Err(DownloadProbeFailure::Execution));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    f.enroll(Some(f.tenant().unwrap()), false).unwrap();
    assert_eq!(
        fetch(&f, selection.clone()),
        Err(DownloadProbeFailure::Remote(DownloadFailure::Inactive))
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
    assert_eq!(
        fetch(&f, selection.clone()),
        Err(DownloadProbeFailure::Remote(DownloadFailure::Fenced))
    );
    f.harness
        .pic
        .stop_canister(f.service, Some(f.controller))
        .unwrap();
    assert!(matches!(
        fetch(&f, selection),
        Err(DownloadProbeFailure::Rejected(_))
    ));
}
