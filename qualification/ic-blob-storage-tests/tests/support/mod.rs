//! Blob's local simulator deadlines and explicit teardown assertions.
use ic_testkit::{
    pocket_ic::PocketIcBuilder,
    server::{Deletion, Instance, Limits, PreparedServer, Server},
};
use std::{path::Path, time::Duration};

pub async fn instance(builder: PocketIcBuilder, lifetime_seconds: u64) -> (Server, Instance) {
    let directory =
        std::env::var_os("BLOB_TEST_SERVER").expect("run make install-test-server first");
    let prepared = PreparedServer::check(Path::new(&directory)).unwrap();
    let server = Server::start(
        prepared,
        Limits {
            startup: Duration::from_secs(60),
            lifetime: Duration::from_secs(lifetime_seconds),
            stdout_bytes: 4 * 1024 * 1024,
            stderr_bytes: 4 * 1024 * 1024,
        },
    )
    .unwrap();
    let instance = server
        .instance(builder, Duration::from_secs(60))
        .await
        .unwrap();
    (server, instance)
}

pub async fn close(mut server: Server, instance: Instance) {
    assert_eq!(
        instance.close(Duration::from_secs(30)).await,
        Deletion::Attempted
    );
    let stopped = server.stop().unwrap();
    assert_eq!(stopped.abandoned, 0);
    assert!(stopped.process.cleanup.is_empty(), "{stopped:?}");
    assert!(
        matches!(
            stopped.process.failure,
            Some(ic_host_process::Failure::Cancelled)
        ),
        "{stopped:?}"
    );
}
