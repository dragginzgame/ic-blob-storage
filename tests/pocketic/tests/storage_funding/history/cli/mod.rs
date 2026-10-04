//! Paid outcomes are explicit fixture substitutes; CLI uses genuine signed queries.
use super::*;
use crate::authenticated_cli::{PEM, arguments, run};
use ic_agent::{Identity, identity::BasicIdentity};
use ic_testkit::pocket_ic::PocketIcBuilder;
use serde_json::Value;

fn exact(args: &[String], intent: Intent) -> Vec<String> {
    let mut args = args.to_vec();
    args[0] = "funding-outcome".into();
    args.extend([
        "--operation".into(),
        intent.operation.to_string(),
        "--offered".into(),
        intent.offered.to_string(),
    ]);
    if let Some(target) = intent.target_balance {
        args.extend(["--target-balance".into(), target.to_string()]);
    }
    args
}

fn unchanged(f: &Fixture, before: &[u8]) {
    assert!(
        f.harness
            .pic
            .get_stable_memory(f.service)
            .iter()
            .eq(before.iter())
    );
}

fn outcomes(f: &Fixture, args: &[String], uncertain: Intent, first: &Value) -> Value {
    let observed = run(&exact(args, uncertain), 0);
    assert_eq!(observed["record"]["phase"]["state"], "uncertain");
    assert_eq!(observed["target_balance"], u128::MAX.to_string());
    assert_eq!(observed["record"]["response"], Value::Null);
    assert_eq!(observed["record"]["reconciliation"]["offered"], "30");
    assert_eq!(observed["record"]["fenced"], false);
    assert_eq!(observed["retry_authorized"], false);
    assert_eq!(observed["provider_credit"], "not_established");
    // Recover the exact original amounts directly from the history entry.
    let callback = Intent {
        operation: first["entries"][1]["operation"]
            .as_str()
            .unwrap()
            .parse()
            .unwrap(),
        offered: first["entries"][1]["offered"]
            .as_str()
            .unwrap()
            .parse()
            .unwrap(),
        target_balance: None,
        ..uncertain
    };
    let refunded = run(&exact(args, callback), 0);
    assert_eq!(refunded["record"]["phase"]["refunded"], "10");
    assert_eq!(refunded["record"]["reconciliation"]["accepted"], "10");
    assert_eq!(refunded["record"]["response"], Value::Null);
    let not_enqueued = run(&exact(args, f.funding_intent(1, 20)), 0);
    assert_eq!(
        not_enqueued["record"]["reconciliation"]["state"],
        "no_transfer"
    );
    let absent = run(&exact(args, f.funding_intent(2, 20)), 0);
    assert_eq!(absent["outcome"], "absent");
    assert_eq!(absent["record"], Value::Null);
    for changed in [
        Intent {
            offered: 31,
            ..uncertain
        },
        Intent {
            target_balance: None,
            ..uncertain
        },
    ] {
        assert_eq!(run(&exact(args, changed), 3)["error"], "funding_conflict");
    }
    observed
}

#[test]
fn authenticated_funding_history_cli_recovers_cursor_and_exact_outcomes_after_restore() {
    let mut f = Fixture::with_operator(
        Harness::with_builder(
            PocketIcBuilder::new()
                .with_nns_subnet()
                .with_application_subnet(),
        ),
        BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap(),
    );
    for (id, action) in [
        (1, Action::NotEnqueued),
        (u128::MAX - 1, Action::Callback(10)),
    ] {
        let intent = f.funding_intent(id, 20);
        f.funding(f.operator, intent, Action::Prepare).unwrap();
        f.funding(f.operator, intent, Action::Attempt).unwrap();
        f.funding(f.operator, intent, action).unwrap();
    }
    let uncertain = Intent {
        target_balance: Some(u128::MAX),
        ..f.funding_intent(u128::MAX, 30)
    };
    f.funding(f.operator, uncertain, Action::Prepare).unwrap();
    f.funding(f.operator, uncertain, Action::Attempt).unwrap();
    let dir = tempfile::tempdir().unwrap();
    let key = dir.path().join("key.pem");
    let root = dir.path().join("root.der");
    let cursor = dir.path().join("cursor.json");
    std::fs::write(&key, PEM).unwrap();
    std::fs::write(&root, f.harness.pic.root_key().unwrap()).unwrap();
    let url = f
        .harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .to_string();
    let args = arguments(
        "funding-history",
        f.funding_scope(),
        f.operator,
        &url,
        &key,
        &root,
    );
    let before = f.harness.pic.get_stable_memory(f.service);
    let first = run(&args, 0);
    assert_eq!(first["entries"][0]["operation"], u128::MAX.to_string());
    assert_eq!(first["entries"][0]["phase"]["state"], "uncertain");
    assert_eq!(first["entries"][0]["target_balance"], u128::MAX.to_string());
    assert_eq!(first["entries"][1]["phase"]["refunded"], "10");
    assert_eq!(
        first["next"]["before_operation"],
        (u128::MAX - 1).to_string()
    );
    std::fs::write(&cursor, first["next"].to_string()).unwrap();
    let mut tail = args.clone();
    tail.extend(["--cursor".into(), cursor.to_str().unwrap().into()]);
    let last = run(&tail, 0);
    assert_eq!(last["entries"][0]["operation"], "1");
    assert_eq!(last["entries"][0]["phase"]["state"], "not_enqueued");
    assert_eq!(last["next"], Value::Null);
    let observed = outcomes(&f, &args, uncertain, &first);
    unchanged(&f, &before);
    f.harness.pic.stop_live();
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation(f.operator),
            Some(f.controller),
        )
        .unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let url = f
        .harness
        .pic
        .make_live_with_params(None, None, Some(vec!["127.0.0.1".into()]), None)
        .to_string();
    let index = tail.iter().position(|s| s == "--url").unwrap();
    tail[index + 1] = url.clone();
    let restored = run(&tail, 0);
    assert_eq!(restored["entries"], last["entries"]);
    assert_eq!(restored["cursor"], first["next"]);
    assert_eq!(restored["fenced"], true);
    let mut exact_args = exact(&args, uncertain);
    let index = exact_args.iter().position(|s| s == "--url").unwrap();
    exact_args[index + 1] = url;
    let mut expected = observed;
    expected["url"] = exact_args[index + 1].clone().into();
    expected["record"]["fenced"] = true.into();
    assert_eq!(run(&exact_args, 0), expected);
    unchanged(&f, &before);
    assert_eq!(f.funding_allocation().uncertain, 30);
    f.harness.pic.stop_live();
}
