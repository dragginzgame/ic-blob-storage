//! Attach to caller-owned local instances without taking lifecycle or retry authority.
use super::{Failure, model::QueryTarget};
use ic_testkit::pocket_ic::PocketIc;
use std::panic::{AssertUnwindSafe, catch_unwind};

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
