//! Transport backpressure never causes automatic native query or update resubmission.
use super::{Failure, change, execute, options};
use ic_agent::{Identity, identity::BasicIdentity};
use std::io;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const MAX_HEADERS: usize = 16 * 1024;
const MAX_BODY: usize = 256 * 1024;

fn service_operation(line: &str) -> bool {
    line.contains("/query ") || line.contains("/call ")
}

fn server(status: u16) -> (String, mpsc::Sender<()>, thread::JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    listener.set_nonblocking(true).unwrap();
    let (stop, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        let mut requests = Vec::new();
        let mut lookups = Vec::new();
        let mut responded = false;
        let mut connections = 0;
        loop {
            if !matches!(receiver.try_recv(), Err(mpsc::TryRecvError::Empty)) {
                break;
            }
            match listener.accept() {
                Ok((mut stream, _)) => {
                    connections += 1;
                    assert!(connections <= 16, "bounded fixture connection count");
                    let deadline = std::time::Instant::now() + Duration::from_secs(2);
                    stream
                        .set_write_timeout(Some(Duration::from_secs(2)))
                        .unwrap();
                    let line = match ic_host_process::testing::http::read_http_request_line(
                        &mut stream,
                        deadline,
                        MAX_HEADERS,
                    ) {
                        Ok(line) => line,
                        // The SDK can cancel a trust lookup without sending a request.
                        Err(error)
                            if responded
                                && matches!(
                                    error.kind(),
                                    io::ErrorKind::UnexpectedEof | io::ErrorKind::ConnectionReset
                                ) =>
                        {
                            continue;
                        }
                        Err(error) => panic!("owned transport request: {error}"),
                    };
                    let request_line = String::from_utf8(line).unwrap();
                    assert!(request_line.starts_with("POST "));
                    let operation = service_operation(&request_line);
                    if operation {
                        let request = ic_host_process::testing::http::read_http_request_with_line(
                            &mut stream,
                            request_line.as_bytes(),
                            deadline,
                            MAX_HEADERS,
                            MAX_BODY,
                        )
                        .expect("complete bounded service request");
                        // Service POSTs require explicit framing; GET-like fixture readers may omit it.
                        let header_end = request
                            .windows(4)
                            .position(|part| part == b"\r\n\r\n")
                            .unwrap();
                        let headers = std::str::from_utf8(&request[..header_end]).unwrap();
                        assert!(
                            headers
                                .lines()
                                .skip(1)
                                .filter_map(|line| line.split_once(':'))
                                .any(|(name, _)| name.eq_ignore_ascii_case("content-length"))
                        );
                        let response = format!(
                            "HTTP/1.1 {status} Backpressure\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                        );
                        stream
                            .write_all(response.as_bytes())
                            .expect("service backpressure response");
                        responded = true;
                    } else {
                        assert!(request_line.contains("/read_state "));
                        // ic-agent try_join! cancels this concurrent lookup when
                        // the query fails. Hold its socket until shutdown: only
                        // the service's HTTP backpressure may decide this test.
                        lookups.push(stream);
                    }
                    requests.push(request_line);
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(5));
                }
                Err(error) => panic!("owned transport fixture: {error}"),
            }
        }
        requests
    });
    (url, stop, handle)
}

#[test]
fn backpressure_returns_failure_after_one_request_for_both_native_call_kinds() {
    let directory = tempfile::tempdir().unwrap();
    let key = directory.path().join("identity.pem");
    let root = directory.path().join("root.der");
    std::fs::write(&key,"-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEICoqKioqKioqKioqKioqKioqKioqKioqKioqKioqKioq\n-----END PRIVATE KEY-----\n").unwrap();
    // HTTP failure precedes certificate validation; no fake trust is accepted.
    std::fs::write(&root, [0]).unwrap();
    let signer = BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap();
    for (command, status) in [
        ("status", 429),
        ("status", 503),
        ("inspect-account", 429),
        ("inspect-account", 503),
    ] {
        let (url, stop, handle) = server(status);
        let mut args = options();
        args[0] = command.into();
        change(&mut args, "--url", &url);
        change(&mut args, "--identity", key.to_str().unwrap());
        change(&mut args, "--root-key", root.to_str().unwrap());
        change(&mut args, "--operator", &signer.to_text());
        if command == "inspect-account" {
            args.extend(["--kind".into(), "balance".into()]);
        }
        let result = execute(&args);
        stop.send(()).unwrap();
        let requests = handle.join().unwrap();
        assert_eq!(result, Err(Failure::Transport));
        // Verified queries may independently read subnet trust keys. Count the
        // service operation itself rather than treating that lookup as a retry.
        let operations = requests
            .iter()
            .filter(|line| service_operation(line))
            .count();
        assert_eq!(
            operations, 1,
            "one service operation despite HTTP backpressure: {requests:?}"
        );
    }
}

#[test]
fn fixture_drains_fragmented_service_body_while_lookup_is_cancelled() {
    use std::net::{Shutdown, TcpStream};

    let (url, stop, handle) = server(503);
    let address = url.strip_prefix("http://").unwrap();
    let mut lookup = TcpStream::connect(address).unwrap();
    lookup
        .write_all(b"POST /api/v3/canister/test/read_state HTTP/1.1\r\n")
        .unwrap();
    let mut operation = TcpStream::connect(address).unwrap();
    operation
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();
    for fragment in [
        b"POST /api/v3/canister/test/qu".as_slice(),
        b"ery HTTP/1.1\r\nContent-Length: 8192\r\n\r\n".as_slice(),
        &[42; 8192],
    ] {
        operation.write_all(fragment).unwrap();
    }
    let mut response = String::new();
    operation.read_to_string(&mut response).unwrap();
    assert_eq!(
        response,
        "HTTP/1.1 503 Backpressure\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
    );
    lookup.shutdown(Shutdown::Both).unwrap();
    stop.send(()).unwrap();
    let requests = handle.join().unwrap();
    assert!(requests.iter().any(|line| line.contains("/read_state ")));
    assert_eq!(
        requests
            .iter()
            .filter(|line| service_operation(line))
            .count(),
        1
    );
}
