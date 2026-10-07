//! Actual signed query subprocess; no simulated caller and no provider effects.
use super::*;
use crate::authenticated_cli::{PEM, arguments, run};
use ic_agent::{Identity, identity::BasicIdentity};
use ic_testkit::pocket_ic::PocketIcBuilder;
use serde_json::Value;
fn change(args: &mut [String], flag: &str, value: &str) {
    let index = args.iter().position(|s| s == flag).unwrap();
    args[index + 1] = value.into();
}

fn live(f: &mut Fixture) -> String {
    f.harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .to_string()
}

fn funding_history(args: &[String], fenced: bool) {
    let mut history = args.to_vec();
    history[0] = "funding-history".into();
    let page = run(&history, 0);
    assert_eq!(page["observation"], "funding_history");
    assert_eq!(page["entries"], serde_json::json!([]));
    assert_eq!(page["next"], Value::Null);
    assert_eq!(page["fenced"], fenced);
    let cursor = tempfile::NamedTempFile::new().unwrap();
    let mut value =
        serde_json::json!({"scope":page["scope"],"before_operation":u128::MAX.to_string()});
    std::fs::write(cursor.path(), value.to_string()).unwrap();
    history.extend(["--cursor".into(), cursor.path().to_str().unwrap().into()]);
    assert_eq!(run(&history, 0)["cursor"], value);
    value["scope"]["namespace"] = "1".into();
    std::fs::write(cursor.path(), value.to_string()).unwrap();
    assert_eq!(run(&history, 3)["error"], "cursor_scope");
}

#[test]
fn standalone_authenticated_cli_verifies_trust_identity_scope_and_passive_restore_status() {
    let mut f = Fixture::with_harness(Harness::with_builder(
        PocketIcBuilder::new()
            .with_nns_subnet()
            .with_application_subnet(),
    ));
    f.operator = BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap();
    f.config.operator = f.operator;
    // Fresh local fixture, with no objects, effects or outstanding obligations.
    f.harness
        .pic
        .reinstall_canister(
            f.service,
            wasm(),
            installation(&f.config),
            Some(f.controller),
        )
        .unwrap();
    let directory = tempfile::tempdir().unwrap();
    let key = directory.path().join("identity.pem");
    let root = directory.path().join("root.der");
    std::fs::write(&key, PEM).unwrap();
    std::fs::write(&root, f.harness.pic.root_key().unwrap()).unwrap();
    let url = live(&mut f);
    let args = arguments("status", f.operator_scope(), f.operator, &url, &key, &root);
    let before = f.harness.pic.get_stable_memory(f.service);
    let value = run(&args, 0);
    funding_history(&args, false);
    assert_eq!(value["operator"], f.operator.to_text());
    assert_eq!(
        value["funding"]["available_allocation"],
        f.config.funding.allocated.to_string()
    );
    assert_eq!(value["scope"]["namespace"], u128::MAX.to_string());
    for owner in ["uploads", "funding", "gateways", "reads"] {
        assert_eq!(value[owner]["fenced"], false);
    }
    let mut wrong = args.clone();
    change(&mut wrong, "--operator", &f.controller.to_text());
    assert_eq!(run(&wrong, 3)["error"], "identity_binding");
    for (flag, value) in [
        ("--namespace", "1".into()),
        ("--payer", f.tenant.to_text()),
        ("--cashier", f.tenant.to_text()),
    ] {
        let mut wrong = args.clone();
        change(&mut wrong, flag, &value);
        assert_eq!(run(&wrong, 3)["error"], "binding");
        wrong[0] = "funding-history".into();
        assert_eq!(run(&wrong, 3)["error"], "binding");
    }
    let root_bytes = std::fs::read(&root).unwrap();
    let mut untrusted = root_bytes.clone();
    *untrusted.last_mut().unwrap() ^= 1;
    std::fs::write(&root, untrusted).unwrap();
    assert_eq!(run(&args, 3)["error"], "transport");
    std::fs::write(&root, root_bytes).unwrap();
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let mut args = args;
    change(&mut args, "--url", &live(&mut f));
    let restored = run(&args, 0);
    funding_history(&args, true);
    for owner in ["uploads", "funding", "gateways", "reads"] {
        assert_eq!(restored[owner]["fenced"], true);
    }
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.harness.pic.stop_live();
    // A valid signing identity is still refused when it is not the installed operator.
    f.config.operator = f.controller;
    f.harness
        .pic
        .reinstall_canister(
            f.service,
            wasm(),
            installation(&f.config),
            Some(f.controller),
        )
        .unwrap();
    change(&mut args, "--url", &live(&mut f));
    assert_eq!(run(&args, 3)["error"], "denied");
    args[0] = "funding-history".into();
    assert_eq!(run(&args, 3)["error"], "denied");
}
