//! Blob's local simulator deadlines and explicit teardown assertions.

use ic_testkit::{
    pocket_ic::PocketIcBuilder,
    server::{Deletion, Instance, Limits, PreparedServer, Server},
};
use std::{path::Path, process::Output, time::Duration};

/// Read the complete text emitted by a fixture command.
///
/// # Panics
/// Panics if stdout contains invalid UTF-8.
#[must_use]
pub fn stdout(output: &Output) -> &str {
    std::str::from_utf8(&output.stdout).expect("UTF-8 command output")
}

/// Start the prepared local simulator and create one owned test instance.
/// # Panics
/// Panics if preparation, startup or instance creation fails within its bounds.
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

/// Delete the instance and assert complete cleanup of the owned simulator.
/// # Panics
/// Panics if deletion, process teardown or cleanup assertions fail.
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

/// Observe declared accounting and all associations of the supplied fixture uploaders.
/// This compares domain state, not allocator bytes or unspecified heap contents.
/// # Panics
/// Panics if an authorized query or its method-bound response fails.
pub async fn blob_state(
    pic: &ic_testkit::pocket_ic::nonblocking::PocketIc,
    target: ic_testkit::pocket_ic::CanisterId,
    application: ic_testkit::pocket_ic::CanisterId,
    uploaders: &[ic_platform_types::Principal],
) -> Vec<ic_blob_storage::protocol::Observation> {
    use ic_blob_storage::{
        local::Error,
        protocol::{Inspection, Observation, RPC_LIMITS},
    };
    let mut state = Vec::new();
    for inspection in std::iter::once(Inspection::Usage)
        .chain(uploaders.iter().copied().map(Inspection::Overview))
    {
        let request = ic_wire::encode(
            &inspection,
            ic_wire::Encoding::Frame {
                method: ic_wire::method_id("blob_query"),
                direction: ic_wire::Direction::Request,
            },
            RPC_LIMITS,
        )
        .unwrap();
        let reply = pic
            .query_call(target, application, "blob_query", request)
            .await
            .unwrap();
        let value: Result<Observation, Error> = ic_wire::decode(
            &reply,
            ic_wire::Encoding::Frame {
                method: ic_wire::method_id("blob_query"),
                direction: ic_wire::Direction::Response,
            },
            RPC_LIMITS,
        )
        .unwrap();
        state.push(value.unwrap());
    }

    state
}
