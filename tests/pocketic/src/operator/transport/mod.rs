//! Attach to caller-owned local instances without taking lifecycle or retry authority.
use super::{Failure, model::Target};
use candid::Principal;
use ic_testkit::pocket_ic::PocketIc;
use std::{
    net::SocketAddr,
    panic::{AssertUnwindSafe, catch_unwind},
};

/// Explicit transport selection, independent of each fixture's semantic binding.
pub(super) struct QueryTarget {
    pub server: SocketAddr,
    pub instance: usize,
    pub canister: Principal,
    pub caller: Principal,
}

impl From<&Target> for QueryTarget {
    fn from(target: &Target) -> Self {
        Self {
            server: target.server,
            instance: target.instance,
            canister: target.canister,
            caller: target.caller,
        }
    }
}

fn attached<T>(
    target: &QueryTarget,
    call: impl FnOnce(&PocketIc) -> Result<T, Failure>,
) -> Result<T, Failure> {
    // SDK construction/calls may panic on HTTP/API errors. Preserve the fixture's
    // established catch-all boundary without acquiring this instance or a gateway.
    catch_unwind(AssertUnwindSafe(|| {
        let url = format!("http://{}/", target.server)
            .parse()
            .expect("validated socket URL");
        let pic = PocketIc::new_from_existing_instance(url, target.instance, Some(10_000));
        call(&pic)
    }))
    .map_err(|_| Failure::Transport)?
}

pub(super) fn query_target(
    target: &QueryTarget,
    method: &str,
    args: Vec<u8>,
) -> Result<Vec<u8>, Failure> {
    attached(target, |pic| {
        pic.query_call(target.canister, target.caller, method, args)
            .map_err(|_| Failure::QueryRejected)
    })
}

pub(super) fn update_target(
    target: &QueryTarget,
    method: &str,
    args: Vec<u8>,
) -> Result<Vec<u8>, Failure> {
    // Exactly one application dispatch; SDK busy polling never allocates a new identity.
    attached(target, |pic| {
        pic.update_call(target.canister, target.caller, method, args)
            .map_err(|_| Failure::Transport)
    })
}
