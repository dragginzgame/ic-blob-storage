//! Signed passive absence never invents a restore-fence observation.
use super::*;
use crate::authenticated_cli::{PEM, arguments, run};
use ic_agent::{Identity, identity::BasicIdentity};
use ic_testkit::pocket_ic::PocketIcBuilder;
use serde_json::Value;

fn change(args: &mut [String], flag: &str, value: &str) {
    let index = args.iter().position(|s| s == flag).unwrap();
    args[index + 1] = value.into();
}

#[test]
fn standalone_funding_cli_preserves_absence_and_refusals_after_fenced_upgrade() {
    let mut f = Fixture::with_harness(Harness::with_builder(
        PocketIcBuilder::new()
            .with_nns_subnet()
            .with_application_subnet(),
    ));
    f.operator = BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap();
    f.config.operator = f.operator;
    f.harness
        .pic
        .reinstall_canister(
            f.service,
            wasm(),
            installation(&f.config),
            Some(f.controller),
        )
        .unwrap();
    let dir = tempfile::tempdir().unwrap();
    let key = dir.path().join("key.pem");
    let root = dir.path().join("root.der");
    std::fs::write(&key, PEM).unwrap();
    std::fs::write(&root, f.harness.pic.root_key().unwrap()).unwrap();
    let url = f
        .harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .to_string();
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
    let before = f.harness.pic.get_stable_memory(f.service);
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
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let url = f
        .harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .to_string();
    change(&mut args, "--url", &url);
    let mut expected = absent;
    expected["url"] = url.into();
    assert_eq!(run(&args, 0), expected);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
    assert!(f.configuration(f.operator).unwrap().fenced);
}
