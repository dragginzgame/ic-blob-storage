use super::*;
use ic_blob_storage::dto::{
    reference::ReferenceUpload,
    upload::history::{UploadHistoryCursor, UploadHistoryEntry, UploadHistoryPage},
};

fn input() -> Input {
    Input {
        service: Principal::from_slice(&[1, 1]),
        namespace: u128::MAX,
        filter: UploadHistoryFilter::Active,
        cursor: None,
    }
}
fn position() -> UploadHistoryCursor {
    UploadHistoryCursor {
        service: input().service,
        namespace: u128::MAX,
        scope: UploadHistoryScope::Service,
        filter: input().filter,
        after_tenant: Principal::from_slice(&[2, 1]),
        after_request: u128::MAX,
    }
}
fn arguments() -> Vec<String> {
    [
        "upload-history",
        "--network",
        "ic",
        "--url",
        "https://icp-api.io",
        "--identity",
        "unused.pem",
        "--operator",
        &Principal::from_slice(&[3, 1]).to_text(),
        "--service",
        &input().service.to_text(),
        "--namespace",
        &u128::MAX.to_string(),
        "--filter",
        "active",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

#[test]
fn saved_continuation_round_trips_and_foreign_or_malformed_scope_rejects_before_transport() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("cursor.json");
    let cursor = cursor::output(position());
    std::fs::write(&path, cursor.to_string()).unwrap();
    let input = Input {
        cursor: Some(path.clone()),
        ..input()
    };
    let request = request(&input).unwrap();
    assert_eq!(request.cursor, Some(position()));
    let mut args = arguments();
    args.extend(["--cursor".into(), path.to_str().unwrap().into()]);
    for (field, value, error) in [
        ("namespace", "1", Failure::CursorScope),
        ("service", "2vxsx-fae", Failure::CursorScope),
        ("filter", "all", Failure::CursorScope),
        ("scope", "tenant", Failure::CursorScope),
        ("after_tenant", "2vxsx-fae", Failure::Arguments),
        ("after_request", "01", Failure::Arguments),
        ("after_request", "0", Failure::Arguments),
        ("after_request", "+1", Failure::Arguments),
    ] {
        let mut changed = cursor.clone();
        changed[field] = value.into();
        std::fs::write(&path, changed.to_string()).unwrap();
        assert_eq!(super::super::execute(&args), Err(error));
    }
    std::fs::write(&path, vec![b' '; 2049]).unwrap();
    assert_eq!(super::super::execute(&args), Err(Failure::File));
    let mut changed = cursor;
    changed["unexpected"] = "ignored?".into();
    std::fs::write(&path, changed.to_string()).unwrap();
    assert_eq!(super::super::execute(&args), Err(Failure::Arguments));
}

#[test]
fn json_keeps_empty_progress_full_width_identities_local_states_and_fences() {
    let options = Options::parse(&arguments()).unwrap();
    let input = request(&input()).unwrap();
    let mut page = UploadHistoryPage {
        request: input,
        entries: vec![],
        scanned: 64,
        next: Some(position()),
        fenced: true,
    };
    let encoded = candid::encode_one(Ok::<_, F>(&page)).unwrap();
    let value = output(&options, input, &encoded).unwrap();
    assert_eq!(value["entries"], json!([]));
    assert_eq!(value["scanned"], "64");
    assert_eq!(value["next"]["after_request"], u128::MAX.to_string());
    assert_eq!(value["fenced"], true);
    assert_eq!(value["retry_authorized"], false);
    page.entries.push(UploadHistoryEntry {
        request: ReferenceUpload {
            service: input.service,
            namespace: input.namespace,
            tenant: position().after_tenant,
            upload: u128::MAX,
            object: u128::MAX - 1,
            incarnation: u128::MAX - 2,
            first_reference: u128::MAX - 3,
            root: [42; 32],
            bytes: u64::MAX,
        },
        state: S::ExposurePossible,
    });
    page.next = None;
    page.scanned = 1;
    let value = output(
        &options,
        input,
        &candid::encode_one(Ok::<_, F>(page)).unwrap(),
    )
    .unwrap();
    assert_eq!(value["entries"][0]["state"], "exposure_possible");
    assert_eq!(
        value["entries"][0]["upload"]["object"],
        (u128::MAX - 1).to_string()
    );
    assert_eq!(value["entries"][0]["upload"]["bytes"], u64::MAX.to_string());
    assert_eq!(value["provider_state"], "not_observed");
    let denied: Result<UploadHistoryPage, _> = Err(F::Denied);
    assert_eq!(
        output(&options, input, &candid::encode_one(denied).unwrap()),
        Err(Failure::Denied)
    );
}

#[test]
fn command_requires_an_explicit_filter_and_operator_without_unrelated_billing_scope() {
    let args = arguments();
    assert!(matches!(Options::parse(&args).unwrap().command,
        super::super::arguments::Command::UploadHistory(input) if input.filter == UploadHistoryFilter::Active));
    assert!(matches!(
        Options::parse(&args[..args.len() - 2]),
        Err(Failure::Arguments)
    ));
    let mut changed = args;
    changed.push("--payer".into());
    changed.push(input().service.to_text());
    assert!(matches!(Options::parse(&changed), Err(Failure::Arguments)));
}
