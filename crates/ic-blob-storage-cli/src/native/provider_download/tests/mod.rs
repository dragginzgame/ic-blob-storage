use super::*;
use candid::Principal;
use ic_blob_storage::{
    dto::{
        reference::ReferenceUpload,
        upload::{
            admission::UploadAdmissionRequest,
            completion::UploadVerificationPlan,
            manifest::{UploadManifestDeclaration, UploadManifestHeader},
        },
    },
    model::identity::caffeine::manifest::builder::CaffeineManifestBuilder,
};
use std::{
    io::{Read, Write},
    net::TcpListener,
    time::Instant,
};

fn plan(bytes: &[u8]) -> UploadVerificationPlan {
    let length = bytes.len().to_string();
    let headers = [
        CaffeineHeader {
            name: "Content-Length",
            value: &length,
        },
        CaffeineHeader {
            name: "Content-Type",
            value: "application/octet-stream",
        },
    ];
    let mut builder = CaffeineManifestBuilder::new(
        bytes.len() as u64,
        &headers,
        CaffeineHashLimits {
            max_content_bytes: (bytes.len() as u64).try_into().unwrap(),
            max_append_bytes: bytes.len().try_into().unwrap(),
            max_headers: 16.try_into().unwrap(),
            max_header_bytes: 4096.try_into().unwrap(),
        },
        8.try_into().unwrap(),
    )
    .unwrap();
    builder.append(0, bytes).unwrap();
    let prepared = builder.finish().unwrap();
    let principal = Principal::self_authenticating([1]);
    UploadVerificationPlan {
        permission: UploadAdmissionRequest {
            upload: ReferenceUpload {
                service: principal,
                namespace: 1,
                tenant: principal,
                upload: 1,
                object: 2,
                incarnation: 3,
                first_reference: 4,
                root: *prepared.hashes().provider_root.as_bytes(),
                bytes: bytes.len() as u64,
            },
            uploader: principal,
            expires_at_ns: u64::MAX,
        },
        verifier: principal,
        owner: principal,
        project: "project".into(),
        admitted_at_ns: 1,
        declaration: UploadManifestDeclaration {
            chunks: prepared
                .manifest()
                .chunks()
                .iter()
                .map(|c| *c.as_bytes())
                .collect(),
            headers: headers
                .iter()
                .map(|h| UploadManifestHeader {
                    name: h.name.into(),
                    value: h.value.into(),
                })
                .collect(),
        },
    }
}
fn serve(listener: &TcpListener, headers: &str, bytes: &[u8]) {
    listener.set_nonblocking(true).unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut stream = loop {
        match listener.accept() {
            Ok((stream, _)) => break stream,
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock && Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(5));
            }
            other => panic!("expected local HTTP request: {other:?}"),
        }
    };
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    stream
        .set_write_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    let mut request = Vec::new();
    while !request.ends_with(b"\r\n\r\n") {
        assert!(request.len() < 8192);
        let mut byte = [0];
        stream.read_exact(&mut byte).unwrap();
        request.push(byte[0]);
    }
    let request = String::from_utf8(request).unwrap().to_ascii_lowercase();
    assert!(request.contains("accept-encoding: identity"));
    assert!(!request.contains("authorization:"));
    // The verifier may reject headers/length and close without accepting the supplied body.
    let _ = stream
        .write_all(headers.as_bytes())
        .and_then(|()| stream.write_all(bytes));
}
fn run_case(
    plan: &UploadVerificationPlan,
    headers: &str,
    bytes: &[u8],
    expected: Result<ContentDigest, Failure>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = Url::parse(&format!(
        "http://{}/v1/blob/",
        listener.local_addr().unwrap()
    ))
    .unwrap();
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("run");
    let run = Run::create(&path).unwrap();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let original_headers: Vec<_> = plan
        .declaration
        .headers
        .iter()
        .map(|h| CaffeineHeader {
            name: &h.name,
            value: &h.value,
        })
        .collect();
    let expected_body = ExpectedBody {
        root: plan.permission.upload.root.as_slice().try_into().unwrap(),
        bytes: plan.permission.upload.bytes,
        headers: &original_headers,
        maximum: plan.permission.upload.bytes.try_into().unwrap(),
    };
    std::thread::scope(|scope| {
        let server = scope.spawn(|| serve(&listener, headers, bytes));
        assert_eq!(
            runtime.block_on(fetch(
                &url,
                "local",
                &expected_body,
                &run,
                &mut std::io::sink()
            )),
            expected
        );
        server.join().unwrap();
    });
    let outcome: serde_json::Value =
        serde_json::from_slice(&std::fs::read(path.join("download-outcome.json")).unwrap())
            .unwrap();
    assert_eq!(
        outcome["outcome"],
        if expected.is_ok() {
            "verified"
        } else {
            "failed"
        }
    );
    assert!(!path.join("statement.candid").exists());
}
#[test]
fn download_stream_requires_eof_and_original_metadata_with_no_redirect_or_decoding() {
    let bytes = vec![42; 2 * 1024 * 1024 + 7];
    let plan = plan(&bytes);
    // Response Content-Type is never substituted for the original hash metadata.
    run_case(
        &plan,
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\n",
        &bytes,
        Ok(ContentDigest::compute(&bytes)),
    );
    let small = plan_for_failures();
    for (headers, body, failure) in [
        (
            "HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n",
            b"abd".as_slice(),
            Failure::Content,
        ),
        (
            "HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n",
            b"ab",
            Failure::Content,
        ),
        (
            "HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n",
            b"abcd",
            Failure::Content,
        ),
        (
            "HTTP/1.1 200 OK\r\nContent-Length: 3\r\nConnection: close\r\n\r\n",
            b"ab",
            Failure::Transport,
        ),
        (
            "HTTP/1.1 200 OK\r\nContent-Length: 4\r\nConnection: close\r\n\r\n",
            b"abcd",
            Failure::Content,
        ),
        (
            "HTTP/1.1 302 Found\r\nLocation: http://127.0.0.1:9/\r\nContent-Length: 0\r\n\r\n",
            b"",
            Failure::ProviderResponse,
        ),
        (
            "HTTP/1.1 206 Partial Content\r\nContent-Length: 3\r\n\r\n",
            b"abc",
            Failure::ProviderResponse,
        ),
        (
            "HTTP/1.1 200 OK\r\nContent-Encoding: gzip\r\nContent-Length: 3\r\n\r\n",
            b"abc",
            Failure::ProviderResponse,
        ),
        (
            "HTTP/1.1 200 OK\r\nContent-Range: bytes 0-2/3\r\nContent-Length: 3\r\n\r\n",
            b"abc",
            Failure::ProviderResponse,
        ),
    ] {
        run_case(&small, headers, body, Err(failure));
    }
}
fn plan_for_failures() -> UploadVerificationPlan {
    plan(b"abc")
}
