use super::*;
use ic_blob_storage_contracts::identity::caffeine::CaffeineContentHasher;
use ic_host_fs::testing::FixtureDirectory;
use std::os::unix::fs::PermissionsExt;
use std::{net::TcpListener, time::Instant};

fn verification(checksum: Option<ContentDigest>) -> Verification {
    let limits = CaffeineHashLimits {
        max_content_bytes: 3.try_into().unwrap(),
        max_append_bytes: 2.try_into().unwrap(),
        max_headers: 2.try_into().unwrap(),
        max_header_bytes: 128.try_into().unwrap(),
    };
    let headers = [CaffeineHeader {
        name: "Content-Length",
        value: "3",
    }];
    let mut hash = CaffeineContentHasher::new(3, &headers, limits).unwrap();
    hash.append(0, b"ab").unwrap();
    hash.append(2, b"c").unwrap();
    Verification::new(
        hash.finish().unwrap().provider_root,
        3,
        &headers,
        checksum,
        limits,
    )
    .unwrap()
}
fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}
fn case(headers: &str, bytes: &[u8], checksum: Option<ContentDigest>, expected: &str) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = Url::parse(&format!("http://{}/blob", listener.local_addr().unwrap())).unwrap();
    let dir = FixtureDirectory::new(&std::env::temp_dir(), "blob-native-download").unwrap();
    let output = dir.path().join("read");
    std::thread::scope(|scope| {
        let server = scope.spawn(|| {
            let (mut socket, _) = ic_host_process::testing::net::accept_tcp_until(
                &listener,
                Instant::now() + Duration::from_secs(5),
            )
            .unwrap();
            let request = ic_host_process::testing::http::capture_http_request(
                &mut socket,
                Duration::from_secs(5),
                8192,
                0,
                ic_host_process::testing::http::HttpContentLength::Optional,
            )
            .into_result()
            .unwrap();
            let request = String::from_utf8(request).unwrap().to_ascii_lowercase();
            assert!(request.contains("accept-encoding: identity"));
            assert!(!request.contains("authorization:"));
            let _ = socket
                .write_all(headers.as_bytes())
                .and_then(|()| socket.write_all(bytes));
        });
        let result = runtime().block_on(download(
            &url,
            &output,
            verification(checksum),
            Duration::from_secs(3),
            true,
        ));
        match (expected, result) {
            ("ok", Ok(found)) => {
                assert_eq!(found.bytes, 3);
                assert_eq!(found.hashes.content_digest, ContentDigest::compute(b"abc"));
                assert_eq!(std::fs::read(found.path).unwrap(), b"abc");
                assert!(!output.join("body.part").exists());
                assert_eq!(
                    std::fs::metadata(&output).unwrap().permissions().mode() & 0o077,
                    0
                );
            }
            ("response", Err(DownloadError::Response))
            | ("length", Err(DownloadError::Length))
            | ("integrity", Err(DownloadError::Integrity(_)))
            | ("transport", Err(DownloadError::Transport(_)))
            | ("checksum", Err(DownloadError::Checksum)) => {
                assert!(!output.join("body.bin").exists());
                assert!(output.join("body.part").exists());
            }
            (_, other) => panic!("expected {expected}, got {other:?}"),
        }
        server.join().unwrap();
    });
}
#[test]
fn verified_output_requires_complete_identity_body_and_both_expected_hashes() {
    let valid = "HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n";
    case(valid, b"abc", Some(ContentDigest::compute(b"abc")), "ok");
    case(valid, b"abc", None, "ok");
    case(
        valid,
        b"abc",
        Some(ContentDigest::compute(b"wrong")),
        "checksum",
    );
    case(valid, b"abd", None, "integrity");
    case(valid, b"ab", None, "integrity");
    case(valid, b"abcd", None, "length");
    for response in [
        "HTTP/1.1 302 Found\r\nLocation: http://127.0.0.1:9/\r\nContent-Length: 0\r\n\r\n",
        "HTTP/1.1 206 Partial Content\r\nContent-Length: 3\r\n\r\n",
        "HTTP/1.1 200 OK\r\nContent-Encoding: gzip\r\nContent-Length: 3\r\n\r\n",
        "HTTP/1.1 200 OK\r\nContent-Range: bytes 0-2/3\r\nContent-Length: 3\r\n\r\n",
    ] {
        case(response, b"abc", None, "response");
    }
    case(
        "HTTP/1.1 200 OK\r\nContent-Length: 4\r\n\r\n",
        b"abcd",
        None,
        "length",
    );
    case(
        "HTTP/1.1 200 OK\r\nContent-Length: 3\r\n\r\n",
        b"ab",
        None,
        "transport",
    );
}
#[test]
fn invalid_targets_and_existing_claims_refuse_without_network_or_replacement() {
    let dir = FixtureDirectory::new(&std::env::temp_dir(), "blob-native-download").unwrap();
    let output = dir.path().join("run");
    for url in [
        "http://example.com/blob",
        "https://name:secret@example.com/blob",
        "https://example.com/blob#fragment",
    ] {
        assert!(matches!(
            runtime().block_on(download(
                &Url::parse(url).unwrap(),
                &output,
                verification(None),
                Duration::from_secs(1),
                true
            )),
            Err(DownloadError::Input)
        ));
        assert!(!output.exists());
    }
    std::fs::create_dir(&output).unwrap();
    std::fs::write(output.join("original"), b"keep").unwrap();
    assert!(matches!(
        runtime().block_on(download(
            &Url::parse("http://127.0.0.1:9/blob").unwrap(),
            &output,
            verification(None),
            Duration::from_secs(1),
            true
        )),
        Err(DownloadError::Directory(_))
    ));
    assert_eq!(std::fs::read(output.join("original")).unwrap(), b"keep");
    assert!(!output.join("body.part").exists());
}
#[test]
fn timeout_and_future_cancellation_never_publish_partial_output() {
    for cancel in [false, true] {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = Url::parse(&format!("http://{}/", listener.local_addr().unwrap())).unwrap();
        let dir = FixtureDirectory::new(&std::env::temp_dir(), "blob-native-download").unwrap();
        let output = dir.path().join("run");
        std::thread::scope(|scope| {
            let server = scope.spawn(|| {
                let (mut stream, _) = ic_host_process::testing::net::accept_tcp_until(
                    &listener,
                    Instant::now() + Duration::from_secs(5),
                )
                .unwrap();
                let _ = ic_host_process::testing::http::capture_http_request(
                    &mut stream,
                    Duration::from_secs(5),
                    8192,
                    0,
                    ic_host_process::testing::http::HttpContentLength::Optional,
                )
                .into_result()
                .unwrap();
                stream
                    .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 3\r\n\r\na")
                    .unwrap();
                std::thread::sleep(Duration::from_secs(1));
            });
            let rt = runtime();
            if cancel {
                assert!(
                    rt.block_on(async {
                        tokio::time::timeout(Duration::from_millis(500), async {
                            download(
                                &url,
                                &output,
                                verification(None),
                                Duration::from_secs(2),
                                true,
                            )
                            .await
                        })
                        .await
                    })
                    .is_err()
                );
            } else {
                assert!(matches!(
                    rt.block_on(download(
                        &url,
                        &output,
                        verification(None),
                        Duration::from_millis(500),
                        true
                    )),
                    Err(DownloadError::Timeout)
                ));
            }
            assert!(!output.join("body.bin").exists());
            assert!(output.join("body.part").exists());
            server.join().unwrap();
        });
    }
}

#[test]
fn publication_race_never_overwrites_an_existing_destination() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = Url::parse(&format!("http://{}/", listener.local_addr().unwrap())).unwrap();
    let dir = FixtureDirectory::new(&std::env::temp_dir(), "blob-native-download").unwrap();
    let output = dir.path().join("run");
    std::thread::scope(|scope| {
        let server = scope.spawn(|| {
            let (mut socket, _) = ic_host_process::testing::net::accept_tcp_until(
                &listener,
                Instant::now() + Duration::from_secs(5),
            )
            .unwrap();
            ic_host_process::testing::http::capture_http_request(
                &mut socket,
                Duration::from_secs(5),
                8192,
                0,
                ic_host_process::testing::http::HttpContentLength::Optional,
            )
            .into_result()
            .unwrap();
            std::fs::write(output.join("body.bin"), b"existing").unwrap();
            socket
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 3\r\n\r\nabc")
                .unwrap();
        });
        assert!(matches!(
            runtime().block_on(download(
                &url,
                &output,
                verification(None),
                Duration::from_secs(3),
                true
            )),
            Err(DownloadError::Publication(_))
        ));
        server.join().unwrap();
    });
    assert_eq!(std::fs::read(output.join("body.bin")).unwrap(), b"existing");
    assert_eq!(std::fs::read(output.join("body.part")).unwrap(), b"abc");
}
