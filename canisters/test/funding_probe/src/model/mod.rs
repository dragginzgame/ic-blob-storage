//! Persisted experiment schema and bounded journal invariants, not a service model.

use blob_test_protocol::funding::{
    FundingAttemptRecord, FundingFailure, FundingObservation, FundingReceiptRecord, FundingRequest,
};
use candid::{CandidType, Principal};
use serde::Deserialize;

const MAX_ATTEMPTS: usize = 16;
// Permit a local attempt larger than the test sender's balance to exercise an
// actual CDK enqueue failure. This is not a production attachment budget.
const MAX_OFFERED: u128 = 10_000_000_000_000;

#[derive(CandidType, Deserialize)]
pub(crate) struct FundingJournalRecord {
    peer: Principal,
    driver: Principal,
    attempts: Vec<FundingAttemptRecord>,
    receipts: Vec<FundingReceiptRecord>,
}

impl FundingJournalRecord {
    pub(crate) fn new(peer: Principal, driver: Principal) -> Self {
        let record = Self {
            peer,
            driver,
            attempts: Vec::new(),
            receipts: Vec::new(),
        };
        record.validate();
        record
    }

    pub(crate) fn validate(&self) {
        assert_ne!(
            self.driver,
            Principal::anonymous(),
            "retained fixture driver"
        );
        assert_ne!(self.peer, Principal::anonymous(), "retained fixture peer");
        assert!(self.attempts.len() <= MAX_ATTEMPTS, "bounded attempts");
        assert!(self.receipts.len() <= MAX_ATTEMPTS, "bounded receipts");
    }

    pub(crate) fn admit(
        &mut self,
        caller: Principal,
        request: FundingRequest,
    ) -> Result<Principal, FundingFailure> {
        if caller != self.driver {
            return Err(FundingFailure::Denied);
        }
        if self
            .attempts
            .iter()
            .any(|entry| entry.request.id == request.id)
        {
            return Err(FundingFailure::AlreadyAdmitted);
        }
        if self
            .attempts
            .iter()
            .any(|entry| entry.observation.is_none())
        {
            return Err(FundingFailure::InProgress);
        }
        if request.offered == 0
            || request.offered > MAX_OFFERED
            || request.accept > request.offered
            || self.attempts.len() == MAX_ATTEMPTS
        {
            return Err(FundingFailure::Limit);
        }
        self.attempts.push(FundingAttemptRecord {
            request,
            observation: None,
        });
        Ok(self.peer)
    }

    pub(crate) fn complete(&mut self, id: u64, observation: FundingObservation) {
        self.attempts
            .iter_mut()
            .find(|entry| entry.request.id == id)
            .expect("admitted attempt")
            .observation = Some(observation);
    }

    pub(crate) fn attempts(&self, caller: Principal) -> Option<Vec<FundingAttemptRecord>> {
        (caller == self.driver).then(|| self.attempts.clone())
    }

    pub(crate) fn receipts(&self, caller: Principal) -> Option<Vec<FundingReceiptRecord>> {
        (caller == self.driver).then(|| self.receipts.clone())
    }

    pub(crate) fn check_receive(&self, caller: Principal) {
        assert_eq!(caller, self.peer, "configured peer only");
        assert!(self.receipts.len() < MAX_ATTEMPTS, "receiver bound");
    }

    pub(crate) fn record_acceptance(&mut self, receipt: FundingReceiptRecord) {
        assert!(self.receipts.len() < MAX_ATTEMPTS, "receiver bound");
        self.receipts.push(receipt);
    }
}
