use super::*;
fn input() -> (tempfile::TempDir, Input, DownloadRequest) {
    let temp = tempfile::tempdir().unwrap();
    let principal = Principal::self_authenticating([1]);
    let request = DownloadRequest {
        service: principal,
        tenant: principal,
        namespace: u128::MAX,
        root: [1; 32],
        object: u128::MAX,
        incarnation: u128::MAX,
        reference: u128::MAX,
    };
    let path = temp.path().join("request.candid");
    std::fs::write(&path, candid::encode_one(request).unwrap()).unwrap();
    let input = Input {
        service: principal,
        namespace: request.namespace,
        request: path,
        project: "project/β?&=".into(),
        gateway: Url::parse("https://gateway.example").unwrap(),
        directory: temp.path().join("run"),
        max_bytes: 3.try_into().unwrap(),
    };
    (temp, input, request)
}
#[test]
fn download_binds_exact_reference_and_project_before_any_output_or_provider_request() {
    let (_temp, input, request) = input();
    let (_, scope) = open(&input, request.tenant).unwrap();
    std::fs::write(
        &input.request,
        candid::encode_args((request, candid::Reserved)).unwrap(),
    )
    .unwrap();
    assert!(matches!(
        open(&input, request.tenant),
        Err(Failure::Arguments)
    ));
    std::fs::write(&input.request, candid::encode_one(request).unwrap()).unwrap();
    assert_eq!(
        open(&input, Principal::self_authenticating([2])).unwrap_err(),
        Failure::Binding
    );
    let response = DownloadResponse {
        request,
        owner: input.service,
        project: input.project.clone(),
        bytes: 3,
        headers: vec![ic_blob_storage::dto::download::DownloadHeader {
            name: "Content-Length".into(),
            value: "3".into(),
        }],
    };
    let encode = |r: &DownloadResponse| candid::encode_one(Ok::<_, E>(r)).unwrap();
    assert_eq!(
        decode(&input, request, &scope, &encode(&response)).unwrap(),
        response
    );
    let mut changed = response.clone();
    changed.request.reference -= 1;
    assert!(matches!(
        decode(&input, request, &scope, &encode(&changed)),
        Err(Failure::Binding)
    ));
    changed = response.clone();
    changed.project = "other".into();
    assert!(matches!(
        decode(&input, request, &scope, &encode(&changed)),
        Err(Failure::Binding)
    ));
    changed = response;
    changed.bytes = 4;
    assert!(matches!(
        decode(&input, request, &scope, &encode(&changed)),
        Err(Failure::ReplyLimit)
    ));
    assert!(matches!(
        decode(
            &input,
            request,
            &scope,
            &candid::encode_one(Err::<DownloadResponse, _>(E::Fenced)).unwrap()
        ),
        Err(Failure::DownloadRefused(E::Fenced))
    ));
    assert!(!input.directory.exists());
}
#[test]
fn output_stays_partial_until_verified_and_existing_verified_output_is_never_replaced() {
    use std::io::Write;
    let (_temp, input, _) = input();
    let run = Run::create(&input.directory).unwrap();
    let mut file = run.open_body().unwrap();
    file.write_all(b"abc").unwrap();
    file.sync_all().unwrap();
    drop(file);
    assert!(input.directory.join("body.part").is_file());
    assert!(!input.directory.join("body.bin").exists());
    // Simulate a conflicting output in the caller-controlled directory.
    run.bytes("body.bin", b"original").unwrap();
    assert_eq!(run.publish_body(), Err(Failure::File));
    assert_eq!(
        std::fs::read(input.directory.join("body.bin")).unwrap(),
        b"original"
    );
    assert_eq!(
        std::fs::read(input.directory.join("body.part")).unwrap(),
        b"abc"
    );
}

#[test]
fn tenant_download_arguments_require_explicit_origin_project_budget_and_request() {
    use crate::native::arguments::{Command, Options};
    let p = Principal::self_authenticating([1]).to_text();
    let args: Vec<String> = [
        "download",
        "--network",
        "ic",
        "--url",
        "https://icp-api.io",
        "--identity",
        "key.pem",
        "--actor",
        &p,
        "--service",
        &p,
        "--namespace",
        "1",
        "--request",
        "request.candid",
        "--project",
        "project",
        "--gateway",
        "https://gateway.example",
        "--max-bytes",
        "10",
        "--run-dir",
        "new-run",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    assert!(matches!(
        Options::parse(&args).unwrap().command,
        Command::Download(_)
    ));
    for (flag, value) in [
        ("--gateway", "http://gateway.example"),
        ("--gateway", "https://user@gateway.example"),
        ("--gateway", "https://gateway.example/path"),
        ("--max-bytes", "0"),
    ] {
        let mut invalid = args.clone();
        let i = invalid.iter().position(|v| v == flag).unwrap();
        invalid[i + 1] = value.into();
        assert!(matches!(Options::parse(&invalid), Err(Failure::Arguments)));
    }
}
