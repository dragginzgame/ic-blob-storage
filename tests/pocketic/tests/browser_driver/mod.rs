//! Shared bounded control channel and process ownership for opt-in Chromium cases.
use ic_host_fs::testing::FixtureDirectory;
use ic_host_process::child::OwnedChild;
use std::{
    io::{BufReader, Write},
    path::PathBuf,
    process::{ChildStdin, ChildStdout, Command, Output, Stdio},
};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct BrowserPreparation {
    pub(crate) hash: String,
    pub(crate) byte_length: u64,
    #[serde(rename = "manifestJSON")]
    pub(crate) manifest_json: String,
}

pub(crate) struct BrowserDriver {
    child: OwnedChild,
    stdin: Option<ChildStdin>,
    reader: Option<BufReader<ChildStdout>>,
    _config: FixtureDirectory,
}
impl BrowserDriver {
    pub(crate) fn start(config: &serde_json::Value, script: &str) -> Self {
        let saved = FixtureDirectory::new(&std::env::temp_dir(), "blob-browser").unwrap();
        let config_path = ic_host_fs::scratch::create_private_bytes(
            saved.path(),
            "config",
            &serde_json::to_vec(config).unwrap(),
            128,
        )
        .unwrap();
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let node = std::env::var_os("BLOB_BROWSER_NODE").unwrap_or_else(|| "node".into());
        let mut command = Command::new(node);
        command
            .arg(repo.join("tests/browser").join(script))
            .arg(config_path)
            .current_dir(&repo)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = OwnedChild::spawn(
            &mut command,
            ic_host_process::child::CleanupPolicy::KillAndWait,
        )
        .expect("run configured browser driver");
        let reader = BufReader::new(child.take_stdout().unwrap());
        let stdin = child.take_stdin();
        Self {
            child,
            stdin,
            reader: Some(reader),
            _config: saved,
        }
    }
    pub(crate) fn send(&mut self, value: &impl serde::Serialize) {
        let writer = self.stdin.as_mut().unwrap();
        serde_json::to_writer(&mut *writer, value).unwrap();
        writer.write_all(b"\n").unwrap();
        writer.flush().unwrap();
    }
    pub(crate) fn read<T: serde::de::DeserializeOwned>(&mut self, limit: u64) -> T {
        let line = ic_host_process::framing::read_line(self.reader.as_mut().unwrap(), limit)
            .unwrap_or_else(|error| {
                let output = self.output(true);
                panic!(
                    "invalid bounded browser control: {error}: {}\n{}",
                    String::from_utf8_lossy(error.rejected_bytes().unwrap_or_default()),
                    String::from_utf8_lossy(&output.stderr)
                );
            });
        serde_json::from_slice(&line).unwrap()
    }
    pub(crate) fn finish(mut self) -> serde_json::Value {
        let output = self.output(false);
        assert!(
            output.status.success(),
            "browser failed: {}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        serde_json::from_slice(&output.stdout).unwrap()
    }
    fn output(&mut self, stop: bool) -> Output {
        self.stdin.take();
        if stop && let Err(error) = self.child.terminate() {
            eprintln!("browser cleanup: {error}");
        }
        super::support::wait_with_output(&mut self.child, self.reader.take())
    }
}
