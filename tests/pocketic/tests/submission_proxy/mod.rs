//! Local HTTP fault transport around real `PocketIC`; no provider protocol substitute here.
use ic_blob_storage_contracts::identity::ContentDigest;
use ic_platform::Principal;
use std::io::Write;
use std::net::TcpListener;
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};
use std::time::Duration;
use std::time::Instant;

pub(crate) struct Proxy {
    pub url: String,
    calls: Arc<AtomicUsize>,
    stop: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum Reply {
    Pass,
    Drop,
    Pending,
}
#[derive(Clone, Copy)]
pub(crate) struct Dispatch {
    pub service: Principal,
    pub actor: Principal,
    pub method: &'static str,
    pub argument_file: &'static str,
}
impl Proxy {
    pub fn start(
        backend: String,
        directory: std::path::PathBuf,
        dispatch: Dispatch,
        reply: Reply,
    ) -> Self {
        Self::sequence(backend, vec![(directory, dispatch, reply)])
    }
    /// Finite ordered updates, each bound to its own durable signed claim.
    pub fn sequence(backend: String, plan: Vec<(std::path::PathBuf, Dispatch, Reply)>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/", listener.local_addr().unwrap());
        listener.set_nonblocking(true).unwrap();
        let calls = Arc::new(AtomicUsize::new(0));
        let stop = Arc::new(AtomicBool::new(false));
        let requests = calls.clone();
        let done = stop.clone();
        let thread = std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let client = reqwest::Client::builder()
                .no_proxy()
                .retry(reqwest::retry::never())
                .redirect(reqwest::redirect::Policy::none())
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap();
            let deadline = Instant::now() + Duration::from_secs(90);
            while !done.load(Ordering::SeqCst) {
                assert!(Instant::now() < deadline);
                let mut stream = match listener.accept() {
                    Ok((s, _)) => s,
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(Duration::from_millis(5));
                        continue;
                    }
                    Err(e) => panic!("local proxy accept: {e}"),
                };
                stream
                    .set_read_timeout(Some(Duration::from_secs(5)))
                    .unwrap();
                stream
                    .set_write_timeout(Some(Duration::from_secs(5)))
                    .unwrap();
                let (path, body) = request(&mut stream);
                let update = path.ends_with("/call");
                let mut reply = Reply::Pass;
                if update {
                    let index = requests.fetch_add(1, Ordering::SeqCst);
                    let (directory, dispatch, selected) =
                        plan.get(index).expect("never resend update");
                    reply = *selected;
                    let intent: serde_json::Value = serde_json::from_slice(
                        &std::fs::read(directory.join("intent.json")).unwrap(),
                    )
                    .unwrap();
                    assert_eq!(
                        std::fs::read(directory.join("signed-request.cbor")).unwrap(),
                        body
                    );
                    assert_eq!(
                        intent["signed_request_sha256"],
                        ContentDigest::compute(&body).to_string()
                    );
                    assert_eq!(intent["method"], dispatch.method);
                    let argument = std::fs::read(directory.join(dispatch.argument_file)).unwrap();
                    ic_platform::agent::signed_update_inspect(
                        dispatch.actor,
                        dispatch.service,
                        dispatch.method,
                        &argument,
                        intent["ingress_expiry_ns"]
                            .as_str()
                            .unwrap()
                            .parse()
                            .unwrap(),
                        body.clone(),
                    )
                    .unwrap();
                }
                let (status, bytes) = runtime.block_on(async {
                    let response = client
                        .post(format!("{}{path}", backend.trim_end_matches('/')))
                        .header("content-type", "application/cbor")
                        .body(body)
                        .send()
                        .await
                        .unwrap();
                    (response.status().as_u16(), response.bytes().await.unwrap())
                });
                if update && reply == Reply::Drop {
                    // The real replica received the signed update. Drop its complete
                    // acknowledgment instead of letting the CLI infer success.
                    drop(stream);
                } else if update && reply == Reply::Pending {
                    // Transport admission is deliberately weaker than the actual
                    // replicated result. The CLI must not infer acceptance here.
                    stream.write_all(b"HTTP/1.1 202 Accepted\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").unwrap();
                } else {
                    write!(stream, "HTTP/1.1 {status} Reply\r\nContent-Type: application/cbor\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", bytes.len()).unwrap();
                    stream.write_all(&bytes).unwrap();
                }
            }
        });
        Self {
            url,
            calls,
            stop,
            thread: Some(thread),
        }
    }
    pub fn calls(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }
}
impl Drop for Proxy {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        if let Some(thread) = self.thread.take() {
            let result = thread.join();
            if !std::thread::panicking() {
                result.unwrap();
            }
        }
    }
}
fn request(stream: &mut std::net::TcpStream) -> (String, Vec<u8>) {
    let request = ic_host_process::testing::http::capture_http_request(
        stream,
        Duration::from_secs(5),
        8192,
        256 * 1024,
        ic_host_process::testing::http::HttpContentLength::Optional,
    )
    .into_result()
    .expect("bounded complete signed request");
    let end = request
        .windows(4)
        .position(|part| part == b"\r\n\r\n")
        .unwrap();
    let header = std::str::from_utf8(&request[..end]).unwrap();
    let path = header
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .to_owned();
    (path, request[end + 4..].to_vec())
}
