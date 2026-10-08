//! Transport backpressure never causes automatic native query or update resubmission.
use super::{Failure, change, execute, options};
use ic_agent::{Identity, identity::BasicIdentity};
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const MAX_HEADERS: u64 = 16 * 1024;
const MAX_BODY: u64 = 256 * 1024;

fn line(reader: &mut impl BufRead, remaining: u64) -> io::Result<String> {
    let mut line = String::new();
    let length = match reader.take(remaining + 1).read_line(&mut line) {
        Ok(length) => length,
        Err(error) if line.is_empty() => return Err(error),
        Err(_) => return Err(io::ErrorKind::InvalidData.into()),
    };
    if length == 0 {
        return Err(io::ErrorKind::UnexpectedEof.into());
    }
    if length as u64 > remaining || !line.ends_with("\r\n") {
        return Err(io::ErrorKind::InvalidData.into());
    }
    Ok(line)
}

// This fixture accepts only the SDK's bounded, Content-Length-framed requests.
// Drain the complete service body before responding or closing the connection.
fn body(reader: &mut impl BufRead, request_line: &str) -> io::Result<()> {
    let mut remaining = MAX_HEADERS - request_line.len() as u64;
    let mut length = None;
    loop {
        let header = line(reader, remaining)?;
        remaining -= header.len() as u64;
        if header == "\r\n" {
            break;
        }
        let (name, value) = header.split_once(':').ok_or(io::ErrorKind::InvalidData)?;
        if name.eq_ignore_ascii_case("content-length") {
            if length.is_some() {
                return Err(io::ErrorKind::InvalidData.into());
            }
            length = Some(
                value
                    .trim()
                    .parse::<u64>()
                    .map_err(|_| io::ErrorKind::InvalidData)?,
            );
        }
        if name.eq_ignore_ascii_case("transfer-encoding") {
            return Err(io::ErrorKind::InvalidData.into());
        }
    }
    let length = length
        .filter(|value| *value <= MAX_BODY)
        .ok_or(io::ErrorKind::InvalidData)?;
    if io::copy(&mut reader.take(length), &mut io::sink())? != length {
        return Err(io::ErrorKind::UnexpectedEof.into());
    }
    Ok(())
}

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
                Ok((stream, _)) => {
                    connections += 1;
                    assert!(connections <= 16, "bounded fixture connection count");
                    // Darwin inherits the listener's nonblocking state at accept.
                    stream.set_nonblocking(false).unwrap();
                    stream
                        .set_read_timeout(Some(Duration::from_secs(2)))
                        .unwrap();
                    stream
                        .set_write_timeout(Some(Duration::from_secs(2)))
                        .unwrap();
                    let mut reader = BufReader::new(stream);
                    let request_line = match line(&mut reader, MAX_HEADERS) {
                        Ok(line) => line,
                        // Once the operation fails, the SDK can cancel a lookup
                        // before sending its first byte. It is not an HTTP request.
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
                    assert!(request_line.starts_with("POST "));
                    let operation = service_operation(&request_line);
                    if operation {
                        body(&mut reader, &request_line).expect("complete bounded service request");
                        let response = format!(
                            "HTTP/1.1 {status} Backpressure\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                        );
                        reader
                            .get_mut()
                            .write_all(response.as_bytes())
                            .expect("service backpressure response");
                        responded = true;
                    } else {
                        assert!(request_line.contains("/read_state "));
                        // ic-agent try_join! cancels this concurrent lookup when
                        // the query fails. Hold its socket until shutdown: only
                        // the service's HTTP backpressure may decide this test.
                        lookups.push(reader);
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

#[test]
fn fixture_refuses_oversized_or_truncated_service_frames() {
    for (frame, expected) in [
        ("Content-Length: 262145\r\n\r\n", io::ErrorKind::InvalidData),
        ("Content-Length: 3\r\n\r\nab", io::ErrorKind::UnexpectedEof),
    ] {
        assert_eq!(
            body(&mut frame.as_bytes(), "POST /query HTTP/1.1\r\n")
                .unwrap_err()
                .kind(),
            expected
        );
    }
    assert_eq!(
        line(&mut "POST /query HTTP/1.1\r\n".as_bytes(), 4)
            .unwrap_err()
            .kind(),
        io::ErrorKind::InvalidData
    );
}
