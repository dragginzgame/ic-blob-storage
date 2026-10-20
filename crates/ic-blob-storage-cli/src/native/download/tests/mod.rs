use super::*;
use ic_host_fs::testing::FixtureDirectory;
fn input() -> (FixtureDirectory, Input, DownloadRequest) {
    let temp = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
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
    let path = temp.path().join("request.wire");
    std::fs::write(
        &path,
        ic_blob_storage_contracts::document::encode(request).unwrap(),
    )
    .unwrap();
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
        ic_blob_storage_contracts::document::encode((
            request,
            ic_platform::candid::contract::Reserved,
        ))
        .unwrap(),
    )
    .unwrap();
    assert!(matches!(
        open(&input, request.tenant),
        Err(Failure::Arguments)
    ));
    std::fs::write(
        &input.request,
        ic_blob_storage_contracts::document::encode(request).unwrap(),
    )
    .unwrap();
    assert_eq!(
        open(&input, Principal::self_authenticating([2])).unwrap_err(),
        Failure::Binding
    );
    let response = DownloadResponse {
        request,
        owner: input.service,
        project: input.project.clone(),
        bytes: 3,
        headers: vec![ic_blob_storage_contracts::dto::download::DownloadHeader {
            name: "Content-Length".into(),
            value: "3".into(),
        }],
    };
    let encode = |r: &DownloadResponse| {
        ic_blob_storage_contracts::wire::encode_response("blob_download_descriptor", Ok::<_, E>(r))
            .unwrap()
    };
    let bytes = encode(&response);
    let view = decode(&input, request, &scope, &bytes).unwrap();
    assert_eq!(view.request, response.request);
    assert_eq!(view.owner, response.owner);
    assert_eq!(view.project.as_str(), response.project);
    assert_eq!(view.bytes, response.bytes);
    assert_eq!(view.headers.len(), response.headers.len());
    for (view, owned) in view.headers.iter().zip(&response.headers) {
        assert_eq!(view.name.as_str(), owned.name);
        assert_eq!(view.value.as_str(), owned.value);
    }
    assert_eq!(
        ic_blob_storage_contracts::wire::encode_response(
            "blob_download_descriptor",
            Ok::<_, E>(&view)
        )
        .unwrap(),
        bytes
    );
    let frame = bytes.as_ptr() as usize..bytes.as_ptr() as usize + bytes.len();
    assert!(frame.contains(&(view.project.as_str().as_ptr() as usize)));
    for header in &view.headers {
        assert!(frame.contains(&(header.name.as_str().as_ptr() as usize)));
        assert!(frame.contains(&(header.value.as_str().as_ptr() as usize)));
    }
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
            &ic_blob_storage_contracts::wire::encode_response(
                "blob_download_descriptor",
                Err::<DownloadResponse, _>(E::Fenced)
            )
            .unwrap()
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
    assert!(input.directory.join("body.part").is_file());
    assert!(!input.directory.join("body.bin").exists());
    // Simulate a conflicting output in the caller-controlled directory.
    run.bytes("body.bin", b"original").unwrap();
    assert_eq!(run.publish_body(file), Err(Failure::File));
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
        "request.wire",
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
