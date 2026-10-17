//! Explicit local server ownership shared by integration cases.

use std::{
    io::BufReader,
    path::PathBuf,
    process::{ChildStdout, Output},
    time::Duration,
};

use ic_host_process::{
    child::OwnedChild,
    tool::{OutputLimit, OutputLimits, drain_child_output},
};
use ic_testkit::{
    pic::{PocketIcInstance, PocketIcManagedServer, PocketIcStartupConfig},
    pocket_ic::{PocketIc, PocketIcBuilder},
};

/// Finite decoding budgets for replies from Blob's installed local fixtures.
/// These cover bounded status/history pages and byte-bearing probe replies;
/// production provider and operator clients retain their own narrower policies.
pub(super) const REPLY_LIMITS: ic_testkit_candid::DecodeLimits = ic_testkit_candid::DecodeLimits {
    reply_bytes: 4 * 1024 * 1024,
    decoding_quota: 20_000_000,
    skipping_quota: 1_000_000,
    type_entries: 512,
    header_bytes: 64 * 1024,
};

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
        let PocketIcInstance { instance, server } = PocketIcStartupConfig::Spawn {
            server_binary: fixture_path("POCKET_IC_BIN"),
            timeout: Duration::from_secs(30),
            server_hard_ttl: None,
            server_idle_ttl: None,
            server_output_files: None,
        }
        .build_instance(builder)
        .expect("start explicitly selected local server and instance");
        Self {
            pic: instance,
            _server: server,
        }
    }
}
