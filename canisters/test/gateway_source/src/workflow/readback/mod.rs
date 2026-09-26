//! Authenticated controls for the local read-source substitute.
use crate::ops;
use blob_test_protocol::journey::readback::{ReadSourceConfig, ReadSourceObservation};
use candid::Principal;

pub(crate) fn configure(caller: Principal, config: ReadSourceConfig) -> bool {
    ops::read(|state| state.driver == caller && !state.fenced) && ops::readback::configure(config)
}

pub(crate) fn observation(caller: Principal) -> Option<ReadSourceObservation> {
    ops::read(|state| state.driver == caller).then(ops::readback::observation)
}

pub(crate) fn resume(caller: Principal) -> bool {
    ops::read(|state| state.driver == caller && !state.fenced) && ops::readback::resume()
}

pub(crate) async fn reply(caller: Principal, root: &[u8], index: u64) {
    if !ops::read(|state| state.service == caller && !state.fenced) {
        ops::reject();
        return;
    }
    let Some(config) = ops::readback::begin(root, index) else {
        ops::reject();
        return;
    };
    if !ops::readback::wait().await {
        ops::readback::abandon();
        return;
    }
    ops::readback::reply(config);
}
