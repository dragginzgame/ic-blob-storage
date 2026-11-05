//! Journal access for the explicitly labelled local dispatcher experiment.
use super::{ProbeMemory, STATE, TRAP_WRITE, UploadContext};
use blob_test_protocol::storage::WriteFault;
use ic_blob_storage::{
    model::billing::journal::{FundingIntent, FundingIntentState},
    ops::service::funding::{StableFundingJournal, access::FundingJournalAccess},
};

pub(crate) struct FixtureDispatchJournal {
    pub execution: UploadContext,
    pub intent: FundingIntent,
    pub attempt_fault: Option<WriteFault>,
    pub callback_fault: Option<WriteFault>,
}
impl FundingJournalAccess for FixtureDispatchJournal {
    type Memory = ProbeMemory;
    fn with_funding_journal<R>(
        &self,
        operation: impl FnOnce(&mut StableFundingJournal<ProbeMemory>) -> R,
    ) -> R {
        STATE.with_borrow_mut(|state| {
            let journal = &mut state.as_mut().unwrap().funding;
            // Select the fault by retained phase, without counting access calls.
            let attempted = journal
                .lookup(self.execution, self.intent)
                .ok()
                .flatten()
                .is_some_and(|v| v.state == FundingIntentState::Uncertain);
            TRAP_WRITE.set(if attempted {
                self.callback_fault
            } else {
                self.attempt_fault
            });
            let result = operation(journal);
            TRAP_WRITE.set(None);
            result
        })
    }
}
