//! Transport backpressure never causes automatic native query or update resubmission.
use super::{Failure, change, execute, options};
use ic_agent::{Identity, identity::BasicIdentity};
use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::mpsc,
    thread,
    time::Duration,
};

fn server(status: u16) -> (String, mpsc::Sender<()>, thread::JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    listener.set_nonblocking(true).unwrap();
    let (stop, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        let mut requests = Vec::new();
        loop {
            if !matches!(receiver.try_recv(), Err(mpsc::TryRecvError::Empty)) {
                break;
            }
            match listener.accept() {
                Ok((mut stream, _)) => {
                    stream
                        .set_read_timeout(Some(Duration::from_secs(2)))
                        .unwrap();
                    let mut request = [0; 4096];
                    let length = stream.read(&mut request).unwrap();
                    assert!(request[..length].starts_with(b"POST "));
                    let line = String::from_utf8_lossy(&request[..length])
                        .lines()
                        .next()
                        .unwrap()
                        .to_owned();
                    requests.push(line);
                    write!(stream,"HTTP/1.1 {status} Backpressure\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").unwrap();
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
            .filter(|line| line.contains("/query ") || line.contains("/call "))
            .count();
        assert_eq!(
            operations, 1,
            "one service operation despite HTTP backpressure: {requests:?}"
        );
    }
}
