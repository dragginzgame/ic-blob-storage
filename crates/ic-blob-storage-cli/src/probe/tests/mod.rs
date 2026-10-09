use super::*;
use std::{
    io::{Read, Write},
    net::TcpListener,
};

#[test]
fn shared_artifact_reads_keep_record_limits_and_raw_hashes() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("selected");
    std::fs::write(&path, b"abc").unwrap();
    assert_eq!(
        record::read(directory.path(), "selected", 3).unwrap(),
        b"abc"
    );
    assert_eq!(
        record::read(directory.path(), "selected", 2),
        Err("record_limit".into())
    );
    assert_eq!(
        record::hash(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    std::fs::write(&path, b"").unwrap();
    assert_eq!(record::read(directory.path(), "selected", 0).unwrap(), b"");
}

#[cfg(unix)]
#[test]
fn probe_records_reject_links_and_special_files_with_existing_codes() {
    let directory = tempfile::tempdir().unwrap();
    let target = directory.path().join("target");
    std::fs::write(&target, b"ok").unwrap();
    std::os::unix::fs::symlink(&target, directory.path().join("link")).unwrap();
    assert_eq!(
        record::read(directory.path(), "link", 2),
        Err("record_not_file".into())
    );
    let fifo = directory.path().join("fifo");
    assert!(
        std::process::Command::new("mkfifo")
            .arg(&fifo)
            .status()
            .unwrap()
            .success()
    );
    assert_eq!(
        record::read(directory.path(), "fifo", 2),
        Err("record_not_file".into())
    );
}

#[test]
fn immutable_records_detect_tampering_and_incomplete_runs() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path();
    record::json(p, "plan.json", &plan().unwrap()).unwrap();
    assert_eq!(record::verify(p).unwrap()["outcome"], "incomplete");
    record::json(
        p,
        "request-0.json",
        &RequestRecord {
            index: 0,
            started_unix_seconds: 1,
            method: "GET".into(),
            url: COMMIT_URL.into(),
        },
    )
    .unwrap();
    assert_eq!(record::verify(p).unwrap()["requests_attempted"], 1);
    record::save(p, "response-0.body", b"failure").unwrap();
    record::json(
        p,
        "response-0.json",
        &ResponseRecord {
            index: 0,
            finished_unix_seconds: 2,
            status: Some(503),
            outcome: "http_status".into(),
            bytes: 7,
            sha256: record::hash(b"failure"),
        },
    )
    .unwrap();
    record::json(
        p,
        "summary.json",
        &SummaryRecord {
            finished_unix_seconds: 2,
            outcome: "failed".into(),
            requests_attempted: 1,
            source_commit: None,
            limitation: LIMITATION.into(),
        },
    )
    .unwrap();
    assert_eq!(record::verify(p).unwrap()["outcome"], "failed");
    assert!(record::save(p, "response-0.body", b"replacement").is_err());
    // Retained-body verification admits empty/exact-limit bodies and preserves
    // the existing typed size refusal independently of recorded length/digest.
    for bytes in [Vec::new(), vec![b'x'; MAX_BODY], vec![b'x'; MAX_BODY + 1]] {
        std::fs::write(p.join("response-0.body"), &bytes).unwrap();
        let mut response: ResponseRecord = record::decode(p, "response-0.json").unwrap();
        response.bytes = bytes.len();
        response.sha256 = record::hash(&bytes);
        std::fs::write(
            p.join("response-0.json"),
            serde_json::to_vec(&response).unwrap(),
        )
        .unwrap();
        if bytes.len() > MAX_BODY {
            assert_eq!(record::verify(p), Err("record_limit".into()));
        } else {
            assert_eq!(record::verify(p).unwrap()["outcome"], "failed");
        }
    }
    #[cfg(unix)]
    {
        std::fs::remove_file(p.join("response-0.body")).unwrap();
        std::os::unix::fs::symlink(p.join("response-0.json"), p.join("response-0.body")).unwrap();
        assert_eq!(record::verify(p), Err("record_not_file".into()));
        std::fs::remove_file(p.join("response-0.body")).unwrap();
    }
    assert!(record::save(p, "response-0.json", b"replacement").is_err());
    std::fs::write(p.join("response-0.body"), b"changed").unwrap();
    assert!(record::verify(p).is_err());
    assert!(capture(p).is_err());
    assert_eq!(
        std::fs::read(p.join("response-0.body")).unwrap(),
        b"changed"
    );
}

#[test]
fn only_exact_commit_ids_can_select_source_paths() {
    assert!(commit(br#"{"sha":"78781961e52b8c9c874becd473402950429d4818"}"#).is_ok());
    for bytes in [
        br#"{"sha":"../../main"}"#.as_slice(),
        br#"{"sha":"main"}"#,
        b"{}",
        b"not json",
    ] {
        assert!(commit(bytes).is_err());
    }
}

#[test]
fn public_http_capture_records_intent_before_dispatch_and_retains_failed_bounded_bodies() {
    for (status, body_size, outcome) in [
        (200, 8, "captured"),
        (503, 16, "http_status"),
        (302, 0, "http_status"),
        (200, MAX_BODY + 1, "body_limit"),
    ] {
        let dir = tempfile::tempdir().unwrap();
        record::json(dir.path(), "plan.json", &plan().unwrap()).unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/", listener.local_addr().unwrap());
        let path = dir.path().to_path_buf();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut buffer = [0; 4096];
            let size = stream.read(&mut buffer).unwrap();
            assert!(size > 0);
            let intent: RequestRecord = record::decode(&path, "request-0.json").unwrap();
            assert_eq!(intent.method, "GET");
            write!(stream,"HTTP/1.1 {status} Test\r\nContent-Length: {body_size}\r\nLocation: http://127.0.0.1:9/\r\nConnection: close\r\n\r\n").unwrap();
            let _ = stream.write_all(&vec![b'x'; body_size]);
        });
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let mut attempted = 0;
        let result = runtime.block_on(async {
            let client = client_builder().no_proxy().build().unwrap();
            fetch(dir.path(), &client, &url, &mut attempted).await
        });
        server.join().unwrap();
        assert_eq!(attempted, 1);
        assert_eq!(result.is_ok(), outcome == "captured");
        let recorded: ResponseRecord = record::decode(dir.path(), "response-0.json").unwrap();
        assert_eq!(recorded.outcome, outcome);
        assert_eq!(recorded.bytes, body_size.min(MAX_BODY));
        assert_eq!(recorded.status, Some(status));
        assert_eq!(record::verify(dir.path()).unwrap()["responses_recorded"], 1);
    }
}
