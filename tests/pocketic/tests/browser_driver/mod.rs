//! Shared bounded control channel and process ownership for opt-in Chromium cases.
use std::{
    io::{BufRead, BufReader, Read, Write},
    path::PathBuf,
    process::{Child, ChildStdout, Command, Output, Stdio},
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
    child: Option<Child>,
    reader: Option<BufReader<ChildStdout>>,
    _config: tempfile::NamedTempFile,
}
impl BrowserDriver {
    pub(crate) fn start(config: &serde_json::Value) -> Self {
        let saved = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(saved.path(), serde_json::to_vec(config).unwrap()).unwrap();
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let node = std::env::var_os("BLOB_BROWSER_NODE").unwrap_or_else(|| "node".into());
        let mut child = Command::new(node)
            .arg(repo.join("tests/browser/run.mjs"))
            .arg(saved.path())
            .current_dir(&repo)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("run configured browser driver");
        let reader = BufReader::new(child.stdout.take().unwrap());
        Self {
            child: Some(child),
            reader: Some(reader),
            _config: saved,
        }
    }
    pub(crate) fn send(&mut self, value: &impl serde::Serialize) {
        let writer = self.child.as_mut().unwrap().stdin.as_mut().unwrap();
        serde_json::to_writer(&mut *writer, value).unwrap();
        writer.write_all(b"\n").unwrap();
        writer.flush().unwrap();
    }
    pub(crate) fn read<T: serde::de::DeserializeOwned>(&mut self, limit: u64) -> T {
        let mut line = Vec::new();
        self.reader
            .as_mut()
            .unwrap()
            .take(limit + 1)
            .read_until(b'\n', &mut line)
            .unwrap();
        if line.len() as u64 > limit || !line.ends_with(b"\n") {
            let output = self.output(true);
            panic!(
                "invalid bounded browser control: {}\n{}",
                String::from_utf8_lossy(&line),
                String::from_utf8_lossy(&output.stderr)
            );
        }
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
        let mut child = self.child.take().unwrap();
        child.stdout = Some(self.reader.take().unwrap().into_inner());
        if stop {
            let _ = child.kill();
        }
        child.wait_with_output().unwrap()
    }
}
impl Drop for BrowserDriver {
    fn drop(&mut self) {
        if let Some(child) = &mut self.child {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}
