//! Owned local byte source shared by signed verifier journeys; never deployed Caffeine.
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    time::{Duration, Instant},
};

pub(crate) fn incoming(listener: &TcpListener) -> (TcpStream, String) {
    listener.set_nonblocking(true).unwrap();
    let deadline = Instant::now() + Duration::from_secs(15);
    let mut stream = loop {
        match listener.accept() {
            Ok((stream, _)) => break stream,
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock && Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(5));
            }
            other => panic!("expected one local byte request: {other:?}"),
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
    (stream, String::from_utf8(request).unwrap())
}

pub(crate) fn serve(listener: &TcpListener, path: &Path, expected: &str, body: &[u8]) {
    let (mut stream, request) = incoming(listener);
    assert_eq!(
        request.lines().next().unwrap(),
        format!("GET {expected} HTTP/1.1")
    );
    assert!(!request.to_ascii_lowercase().contains("authorization:"));
    for file in [
        "plan.json",
        "permission.candid",
        "service-response.candid",
        "download-request.json",
    ] {
        assert!(
            path.join(file).is_file(),
            "intent/declaration must precede source GET"
        );
    }
    assert!(!path.join("statement.candid").exists());
    // Preserve the exact observed request beside the maintained client evidence.
    std::fs::write(path.join("fixture-http-request.txt"), request).unwrap();
    let header = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let mut response = header.into_bytes();
    response.extend_from_slice(body);
    std::fs::write(path.join("fixture-http-response.bin"), &response).unwrap();
    stream.write_all(&response).unwrap();
}
