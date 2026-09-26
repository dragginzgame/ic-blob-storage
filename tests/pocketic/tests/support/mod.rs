//! Explicit local server ownership shared by integration cases.

use std::{path::PathBuf, time::Duration};

use ic_testkit::{
    pic::{PocketIcBuilderExt, PocketIcManagedServer, PocketIcStartupConfig},
    pocket_ic::{PocketIc, PocketIcBuilder},
};

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
    _server: PocketIcManagedServer,
}

impl Harness {
    pub fn new() -> Self {
        let server =
            PocketIcStartupConfig::spawn(fixture_path("POCKET_IC_BIN"), Duration::from_secs(30))
                .start_managed_server()
                .expect("start explicitly selected local server");
        let pic = PocketIcBuilder::new()
            .with_application_subnet()
            .try_build(PocketIcStartupConfig::connect(
                server.url(),
                Duration::from_secs(30),
            ))
            .expect("PocketIC instance");
        Self {
            pic,
            _server: server,
        }
    }
}
