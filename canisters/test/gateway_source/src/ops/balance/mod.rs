//! Source state access and real IC scheduling, not provider semantics.
use crate::model::balance::ReplyRecord;
use blob_test_protocol::balance::{BalanceSourceConfig, BalanceSourceView};
use candid::Principal;

pub(crate) fn configure(config: BalanceSourceConfig) -> bool {
    super::mutate(|state| {
        state.balance.configure(ReplyRecord {
            account: config.account,
            bytes: config.bytes,
            reject: config.reject,
            hold: config.hold,
        })
    })
}
pub(crate) fn begin(account: Principal) -> Option<ReplyRecord> {
    super::mutate(|state| state.balance.begin(account))
}
pub(crate) async fn wait() -> bool {
    super::scheduling::wait(|| super::read(|s| s.fenced || s.balance.ready)).await
        && !super::read(|s| s.fenced)
}
pub(crate) fn finish() {
    if !super::read(|s| s.fenced) {
        super::mutate(|s| s.balance.finish());
    }
}
pub(crate) fn resume() -> bool {
    super::mutate(|s| s.balance.resume())
}
pub(crate) fn view() -> BalanceSourceView {
    super::read(|s| BalanceSourceView {
        fenced: s.fenced,
        requests: s.balance.requests,
        pending: s
            .balance
            .config
            .as_ref()
            .filter(|_| s.balance.pending)
            .map(|c| c.account),
    })
}

pub(crate) fn inspection_reply(account: Principal) -> Option<Vec<u8>> {
    super::read(|state| state.balance.inspection_reply(account))
}
