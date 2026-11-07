//! Signed passive absence never invents a restore-fence observation.
use super::*;
use crate::authenticated_cli::{arguments, change, identity_files, live, run};
use ic_host_fs::testing::FixtureDirectory;
use ic_platform::agent::{BasicIdentity, Identity};
use ic_testkit::pocket_ic::PocketIcBuilder;
use serde_json::Value;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn standalone_funding_cli_preserves_absence_and_refusals_after_fenced_upgrade() {
    let mut f = Fixture::with_harness(
        Harness::with_builder(
            PocketIcBuilder::new(crate::support::REQUEST_TIMEOUT)
                .with_nns_subnet()
                .with_application_subnet(),
        )
        .await,
    )
    .await;
    f.operator = BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap();
    f.config.operator = f.operator.into();
    f.harness
        .pic
        .reinstall_canister(
            f.service,
            wasm(),
            installation(&f.config),
            Some(f.controller),
        )
        .await
        .unwrap();
    let dir = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
    let (key, root) = identity_files(&f.harness.pic, dir.path(), "key.pem").await;
    let url = live(&mut f.harness.pic).await;
    let mut args = arguments(
        "funding-outcome",
        f.operator_scope(),
        f.operator,
        &url,
        &key,
        &root,
    );
    args.extend([
        "--operation".into(),
        u128::MAX.to_string(),
        "--offered".into(),
        u128::MAX.to_string(),
        "--target-balance".into(),
        u128::MAX.to_string(),
    ]);
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let absent = run(&args, 0);
    assert_eq!(absent["outcome"], "absent");
    assert_eq!(absent["record"], Value::Null);
    assert_eq!(absent["offered"], u128::MAX.to_string());
    assert_eq!(absent["retry_authorized"], false);
    let mut wrong = args.clone();
    change(&mut wrong, "--namespace", "1");
    assert_eq!(run(&wrong, 3)["error"], "binding");
    wrong = args.clone();
    change(&mut wrong, "--operator", &f.controller.to_text());
    assert_eq!(run(&wrong, 3)["error"], "identity_binding");
    let untrusted = dir.path().join("untrusted.der");
    std::fs::write(&untrusted, [1, 2, 3]).unwrap();
    wrong = args.clone();
    change(&mut wrong, "--root-key", untrusted.to_str().unwrap());
    assert_eq!(run(&wrong, 3)["error"], "transport");
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
    f.harness.pic.stop_live().await.unwrap();
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let url = live(&mut f.harness.pic).await;
    change(&mut args, "--url", &url);
    let mut expected = absent;
    expected["url"] = url.into();
    assert_eq!(run(&args, 0), expected);
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
    f.harness.pic.stop_live().await.unwrap();
    assert!(f.configuration(f.operator).await.unwrap().fenced);
}
