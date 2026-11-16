//! Bounded loopback-only fault injection; forwards real IC bytes, never signs replies.

use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    task::{JoinHandle, JoinSet},
};

pub(super) struct Relay {
    pub url: String,
    packet: Arc<Mutex<Vec<u8>>>,
    dropped: Arc<AtomicUsize>,
    task: JoinHandle<()>,
}

impl Relay {
    pub async fn start(upstream_port: u16) -> Self {
        let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0))
            .await
            .unwrap();
        let url = format!(
            "http://127.0.0.1:{}/",
            listener.local_addr().unwrap().port()
        );
        let packet = Arc::new(Mutex::new(Vec::new()));
        let dropped = Arc::new(AtomicUsize::new(0));
        let selected = Arc::clone(&packet);
        let count = Arc::clone(&dropped);
        let task = tokio::spawn(async move {
            // Dropping this owner aborts every active relay connection.
            let mut connections = JoinSet::new();
            loop {
                tokio::select! {
                    accepted = listener.accept() => {
                        let (client, _) = accepted.unwrap();
                        let selected = Arc::clone(&selected);
                        let count = Arc::clone(&count);
                        assert!(connections.len() < 16, "bounded fixture connections");
                        connections.spawn(async move {
                            tokio::time::timeout(Duration::from_secs(30), forward(client, upstream_port, selected, count)).await.unwrap();
                        });
                    }
                    completed = connections.join_next(), if !connections.is_empty() => {
                        completed.unwrap().unwrap();
                    }
                }
            }
        });
        Self {
            url,
            packet,
            dropped,
            task,
        }
    }

    pub fn drop_reply_for(&self, packet: &[u8]) {
        assert!(!packet.is_empty() && packet.len() <= 2 * 1024 * 1024);
        *self.packet.lock().unwrap() = packet.to_vec();
    }

    pub fn dropped(&self) -> usize {
        self.dropped.load(Ordering::SeqCst)
    }

    pub async fn close(mut self) {
        self.task.abort();
        assert!((&mut self.task).await.unwrap_err().is_cancelled());
    }
}

impl Drop for Relay {
    fn drop(&mut self) {
        self.task.abort();
    }
}

async fn forward(
    mut client: TcpStream,
    port: u16,
    selected: Arc<Mutex<Vec<u8>>>,
    count: Arc<AtomicUsize>,
) {
    // The fixture accepts only the native client's bounded Content-Length HTTP/1
    // requests. Closing each response prevents a pooled connection bypassing the trap.
    let mut header = Vec::new();
    while !header.ends_with(b"\r\n\r\n") {
        assert!(header.len() < 16 * 1024);
        header.push(client.read_u8().await.unwrap());
    }
    let header = std::str::from_utf8(&header).unwrap();
    let mut lines = header.split("\r\n");
    let request = lines.next().unwrap();
    assert!(request.starts_with("POST /api/") && request.ends_with(" HTTP/1.1"));
    let mut length = None;
    let mut forwarded = format!("{request}\r\n");
    for line in lines.filter(|line| !line.is_empty()) {
        let (name, value) = line.split_once(':').unwrap();
        assert!(!name.eq_ignore_ascii_case("transfer-encoding"));
        if name.eq_ignore_ascii_case("content-length") {
            assert!(length.is_none());
            length = Some(value.trim().parse::<usize>().unwrap());
        }
        if !name.eq_ignore_ascii_case("connection") {
            forwarded.push_str(line);
            forwarded.push_str("\r\n");
        }
    }
    forwarded.push_str("Connection: close\r\n\r\n");
    let length = length.unwrap();
    assert!(length <= 2 * 1024 * 1024);
    let mut body = vec![0; length];
    client.read_exact(&mut body).await.unwrap();
    let drop_reply = *selected.lock().unwrap() == body;
    let mut upstream = TcpStream::connect((std::net::Ipv4Addr::LOCALHOST, port))
        .await
        .unwrap();
    upstream.write_all(forwarded.as_bytes()).await.unwrap();
    upstream.write_all(&body).await.unwrap();
    if drop_reply {
        // Wait for the real gateway to respond, then close without forwarding a byte.
        upstream.read_u8().await.unwrap();
        count.fetch_add(1, Ordering::SeqCst);
    } else {
        let copied = tokio::io::copy(&mut upstream.take(4 * 1024 * 1024 + 1), &mut client)
            .await
            .unwrap();
        assert!(copied <= 4 * 1024 * 1024);
    }
    client.shutdown().await.unwrap();
}
