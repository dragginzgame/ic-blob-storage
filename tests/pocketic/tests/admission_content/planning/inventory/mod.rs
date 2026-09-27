//! Exercise the actual query-only executable against the existing service owner.
use super::*;
use serde_json::{Value, json};
use std::{fs, path::Path, process::Command as Process};

fn write_inventory(path: &Path, aliases: bool) {
    let vector = vectors::source("abc-text");
    let assets: Vec<_> = if aliases { vec!["a", "alias"] } else { vec!["a"] }.into_iter()
        .map(|asset| json!({"asset":asset,"source":"unopened-source.bin","root":vector["provider_root"]})).collect();
    let report = json!({
        "totals":{"assets":assets.len(),"source_bytes":assets.len()*3,"source_chunks":assets.len(),
            "distinct_blobs":1,"distinct_bytes":3,"distinct_chunks":1},
        "assets":assets,
        "blobs":[{"claim":{"root":vector["provider_root"],"bytes":3,"headers":vector["headers"]},
            "chunk_hashes":vector["chunk_hashes"],"computed_content_digest":vector["raw_digest"]}],
    });
    fs::write(path, serde_json::to_vec(&report).unwrap()).unwrap();
}

fn inspect(f: &Fixture, tenant: Principal, input: &Path, exit_code: i32) -> Value {
    let url = f.harness.pic.get_server_url();
    let output = Process::new(env!("CARGO_BIN_EXE_blob-fixture-inventory"))
        .args([
            "inspect".into(),
            "--server".into(),
            format!("{}:{}", url.host_str().unwrap(), url.port().unwrap()),
            "--instance".into(),
            f.harness.pic.instance_id().to_string(),
            "--canister".into(),
            f.service.to_text(),
            "--caller".into(),
            tenant.to_text(),
            "--tenant".into(),
            tenant.to_text(),
            "--namespace".into(),
            "1".into(),
            "--inventory".into(),
            input.to_str().unwrap().into(),
        ])
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(exit_code),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn inventory_cli_distinguishes_absence_pending_live_reuse_and_retired_roots() {
    let f = Fixture::new();
    f.enroll();
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("inventory.json");
    write_inventory(&input, true);
    let absent = inspect(&f, f.project, &input, 0);
    assert_eq!(
        absent["contents"][0]["observation"]["status"],
        "not_visible"
    );
    assert_eq!(absent["admission"], "not_proven");
    let vector = vectors::vector("abc-text", 1);
    let p = f.permission(&vector);
    f.admit(p);
    let before = f.observe(p);
    let pending = inspect(&f, f.project, &input, 4);
    assert_eq!(
        pending["contents"][0]["observation"]["status"],
        "recover_existing_operation"
    );
    assert_eq!(f.observe(p), before);
    f.prepare(p, &vector);
    f.call(f.uploader, Command::Expose(p.request.root)).unwrap();
    f.call(
        f.operator,
        Command::FixtureLifecycle(LifecycleCommand::SubstituteCompletion(p.request)),
    )
    .unwrap();
    let live = inspect(&f, f.project, &input, 4);
    assert_eq!(live["blockers"], json!(["reference_capacity"]));
    assert_eq!(live["contents"][0]["fresh_reference_demand"], 2);
    write_inventory(&input, false);
    let before = f.observe(p);
    let reusable = inspect(&f, f.project, &input, 0);
    assert_eq!(
        reusable["contents"][0]["observation"]["status"],
        "live_requires_retain"
    );
    assert_eq!(reusable["contents"][0]["observation"]["fresh_retains"], 1);
    assert_eq!(inspect(&f, f.project, &input, 0), reusable);
    assert_eq!(f.observe(p), before);
    f.call(
        f.project,
        Command::FixtureLifecycle(LifecycleCommand::Reference {
            object: p.request,
            reference: 1,
            operation: 1,
            retain: false,
        }),
    )
    .unwrap();
    for command in [
        LifecycleCommand::SubstituteDeletion(p.request),
        LifecycleCommand::SubstituteSettlement(p.request),
    ] {
        f.call(f.operator, Command::FixtureLifecycle(command))
            .unwrap();
    }
    f.restart();
    let retired = inspect(&f, f.project, &input, 4);
    assert_eq!(
        retired["contents"][0]["observation"]["status"],
        "retired_root"
    );
    assert_eq!(retired["not_visible_demand"]["objects"], 0);
    // A different enrolled tenant cannot discover the historical operation.
    f.call(
        f.operator,
        Command::Enroll {
            tenant: f.other,
            expected: None,
            active: true,
        },
    )
    .unwrap();
    let foreign = inspect(&f, f.other, &input, 0);
    assert_eq!(
        foreign["contents"][0]["observation"]["status"],
        "not_visible"
    );
    assert!(
        foreign["contents"][0]["observation"]
            .get("original_operation")
            .is_none()
    );
}

#[test]
fn inventory_cli_rejects_invalid_reports_and_denied_queries_without_partial_results() {
    let f = Fixture::new();
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("inventory.json");
    write_inventory(&input, false);
    let denied = inspect(&f, f.project, &input, 3);
    assert_eq!(denied["error"], "denied");
    assert!(denied.get("contents").is_none());
    fs::write(&input, b"{\"totals\":{}}").unwrap();
    let rejected = inspect(&f, f.project, &input, 3);
    assert_eq!(rejected["error"], "invalid_request");
    assert!(rejected.get("contents").is_none());
}
