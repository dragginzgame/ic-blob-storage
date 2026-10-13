//! Explicit local server ownership shared by integration cases.

use std::{io::Read, path::PathBuf, process::Output, time::Duration};

use ic_testkit::{
    ic_host_process::child::OwnedChild,
    pic::{PocketIcInstance, PocketIcManagedServer, PocketIcStartupConfig},
    pocket_ic::{PocketIc, PocketIcBuilder},
};

/// Drain caller-owned pipes while Host waits and cleans the child group.
/// Keep buffered protocol bytes: converting a reader back to its pipe loses them.
pub(super) fn wait_with_output(child: &mut OwnedChild, mut stdout: impl Read + Send) -> Output {
    child.take_stdin();
    let mut stderr = child.take_stderr().expect("configured stderr pipe");
    let mut out = Vec::new();
    let mut err = Vec::new();
    let status = std::thread::scope(|scope| {
        scope.spawn(|| stdout.read_to_end(&mut out).expect("read child stdout"));
        scope.spawn(|| stderr.read_to_end(&mut err).expect("read child stderr"));
        child.wait().unwrap_or_else(|error| {
            // Scoped readers must be released before unwinding joins them.
            if let Err(cleanup) = child.terminate() {
                eprintln!("child cleanup after wait failure: {cleanup}");
            }
            panic!("wait and clean child group: {error}");
        })
    });
    Output {
        status,
        stdout: out,
        stderr: err,
    }
}

#[cfg(test)]
mod tests;

pub(super) fn fixture_path(variable: &str) -> PathBuf {
    let path = PathBuf::from(
        std::env::var_os(variable).expect("run make test-pocketic to provision fixture paths"),
    );
    assert!(
        path.is_file(),
        "fixture input does not exist: {}",
        path.display()
    );
    path
}

pub(super) struct Harness {
    // Rust drops fields in declaration order: instance before its owned server.
    pub pic: PocketIc,
    _server: Option<PocketIcManagedServer>,
}

impl Harness {
    pub fn new() -> Self {
        Self::with_builder(PocketIcBuilder::new().with_application_subnet())
    }

    pub fn with_builder(builder: PocketIcBuilder) -> Self {
        let PocketIcInstance { instance, server } =
            PocketIcStartupConfig::spawn(fixture_path("POCKET_IC_BIN"), Duration::from_secs(30))
                .build_instance(builder)
                .expect("start explicitly selected local server and instance");
        Self {
            pic: instance,
            _server: server,
        }
    }
}
