//! Owned native subprocess for the maintained publication session control protocol.
use ic_host_process::child::OwnedChild;
use serde_json::Value;
use std::{
    io::{BufReader, Write},
    process::{ChildStdin, ChildStdout, Stdio},
    time::{Duration, Instant},
};

pub(crate) struct NativeSession {
    child: OwnedChild,
    stdin: Option<ChildStdin>,
    reader: BufReader<ChildStdout>,
}
impl NativeSession {
    pub(crate) fn start(args: &[String]) -> Self {
        Self::start_with_roots(args, None)
    }
    pub(crate) fn start_with_roots(args: &[String], roots: Option<&std::path::Path>) -> Self {
        let mut command = super::authenticated_cli::command(args, roots);
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = OwnedChild::spawn(
            &mut command,
            ic_host_process::child::CleanupPolicy::KillAndWait,
        )
        .unwrap();
        let reader = BufReader::new(child.take_stdout().unwrap());
        let stdin = child.take_stdin();
        Self {
            child,
            stdin,
            reader,
        }
    }
    pub(crate) fn send(&mut self, frame: &Value) {
        let writer = self.stdin.as_mut().unwrap();
        serde_json::to_writer(&mut *writer, frame).unwrap();
        writer.write_all(b"\n").unwrap();
        writer.flush().unwrap();
    }
    pub(crate) fn read(&mut self) -> Value {
        let bytes = ic_host_process::framing::read_line(&mut self.reader, 1024 * 1024)
            .expect("bounded complete session response");
        serde_json::from_slice(&bytes).unwrap()
    }
    /// Wait without closing stdin: the session's deadline must end an idle read.
    pub(crate) fn wait_for_deadline(&mut self) {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if self.child.try_wait().unwrap().is_some() {
                return;
            }
            assert!(
                Instant::now() < deadline,
                "session exits while its control pipe remains open"
            );
            std::thread::sleep(Duration::from_millis(10));
        }
    }
    pub(crate) fn finish(mut self, expected: i32) -> Value {
        self.stdin.take();
        let result = self.read();
        let output = super::support::wait_with_output(&mut self.child, Some(self.reader));
        assert_eq!(
            output.status.code(),
            Some(expected),
            "native session status"
        );
        assert!(
            output.stderr.is_empty(),
            "native errors stay redacted on stdout"
        );
        result
    }
}
