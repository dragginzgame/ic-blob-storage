//! Persist the exact held list request before yielding to the IC scheduler.
use crate::model::SourceJournalRecord;
use candid::Principal;

pub(crate) fn begin(gateway: Principal) -> u64 {
    super::mutate(|state| state.begin_held_sync(gateway))
}

pub(crate) async fn wait(token: u64) -> bool {
    super::scheduling::wait(|| {
        super::read(|state| {
            !state.fenced
                && state
                    .sync
                    .held
                    .as_ref()
                    .is_some_and(|held| held.sequence == token && held.ready)
        })
    })
    .await
}

pub(crate) fn resume() -> bool {
    super::mutate(SourceJournalRecord::resume_held_sync)
}

pub(crate) fn finish(token: u64) {
    super::mutate(|state| state.finish_held_sync(token));
}
