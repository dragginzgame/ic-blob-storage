//! Explicit local server ownership shared by integration cases.

use std::{
    io::BufReader,
    process::{ChildStdout, Output},
    time::Duration,
};

use ic_host_process::{
    child::OwnedChild,
    tool::{OutputLimit, OutputLimits, drain_child_output},
};
use ic_testkit::pic::{PocketIcManagedServer, PocketIcStartupConfig};
use ic_testkit::pocket_ic::{PocketIc, PocketIcBuilder};

/// Drain caller-owned pipes while Host waits and cleans the child group.
/// Keep buffered protocol bytes: converting a reader back to its pipe loses them.
pub(super) fn wait_with_output(
    child: &mut OwnedChild,
    stdout: Option<BufReader<ChildStdout>>,
) -> Output {
    let output = drain_child_output(
        child,
        stdout,
        OutputLimits {
            stdout: OutputLimit::Terminate(16 * 1024 * 1024),
            stderr: OutputLimit::Terminate(4 * 1024 * 1024),
            // Session/scenario owners retain their original deadlines.
            timeout: None,
        },
        || false,
    )
    .expect("drain bounded child output and clean child group");
    Output {
        status: output.status,
        stdout: output.stdout,
        stderr: output.stderr,
    }
}

#[cfg(test)]
mod tests;

pub(super) use ic_blob_storage_pocketic_tests::installation::fixture_path;

/// Blob-owned per-exchange request budget, preserving the prior five-minute limit.
/// This is separate from server startup, signed CLI and scenario deadlines.
pub(super) const REQUEST_TIMEOUT: Duration = Duration::from_secs(300);

pub(super) struct Harness {
    // Every harness owns an isolated server. Its process cleanup stops all
    // instances, including on panic; async clients issue no requests from Drop.
    pub pic: PocketIc,
    _server: PocketIcManagedServer,
}

impl Harness {
    pub async fn new() -> Self {
        Self::with_builder(PocketIcBuilder::new(REQUEST_TIMEOUT).with_application_subnet()).await
    }

    pub async fn with_builder(builder: PocketIcBuilder) -> Self {
        let (instance, server) = PocketIcStartupConfig::Spawn {
            server_binary: fixture_path("POCKET_IC_BIN"),
            timeout: Duration::from_secs(30),
            server_hard_ttl: None,
            server_idle_ttl: None,
            server_output_files: None,
        }
        .build_instance(builder)
        .await
        .expect("start explicitly selected local server and instance")
        .into_parts();
        Self {
            pic: instance,
            _server: server.expect("spawn configuration owns the isolated server"),
        }
    }
}
