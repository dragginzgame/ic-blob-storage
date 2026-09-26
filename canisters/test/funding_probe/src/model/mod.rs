//! Persisted experiment schema and bounded journal invariants, not a service model.

use blob_test_protocol::funding::{
    FundingAttemptRecord, FundingFailure, FundingObservation, FundingOutcome, FundingReceiptRecord,
    FundingRequest,
};
use candid::{CandidType, Principal};
use ic_blob_storage::model::{billing::transfer::FundingTransfer, identity::ContentDigest};
use serde::Deserialize;
use std::{collections::BTreeSet, num::NonZeroU128};

const MAX_ATTEMPTS: usize = 16;
// Permit a local attempt larger than the test sender's balance to exercise an
// actual CDK enqueue failure. This is not a production attachment budget.
const MAX_OFFERED: u128 = 10_000_000_000_000;

#[derive(CandidType, Deserialize)]
pub(crate) struct FundingJournalRecord {
    service: Principal,
    release: [u8; 32],
    fenced: bool,
    peer: Principal,
    driver: Principal,
    attempts: Vec<FundingAttemptRecord>,
    receipts: Vec<FundingReceiptRecord>,
}

impl FundingJournalRecord {
    pub(crate) const fn peer(&self) -> Principal {
        self.peer
    }

    pub(crate) const fn fenced(&self) -> bool {
        self.fenced
    }

    pub(crate) fn fence(&mut self) {
        self.fenced = true;
    }

    pub(crate) fn all_attempts(&self) -> &[FundingAttemptRecord] {
        &self.attempts
    }

    pub(crate) fn new(service: Principal, peer: Principal, driver: Principal) -> Self {
        let record = Self {
            service,
            release: release_binding(),
            fenced: false,
            peer,
            driver,
            attempts: Vec::new(),
            receipts: Vec::new(),
        };
        record
            .validate(service)
            .expect("valid initial funding journal");
        record
    }

    pub(crate) fn validate(&self, service: Principal) -> Result<(), JournalFailure> {
        if self.service != service
            || self.peer == service
            || ![self.service, self.peer, self.driver]
                .into_iter()
                .all(valid_principal)
        {
            return Err(JournalFailure::Binding);
        }
        if self.release != release_binding() {
            return Err(JournalFailure::Release);
        }
        if self.attempts.len() > MAX_ATTEMPTS || self.receipts.len() > MAX_ATTEMPTS {
            return Err(JournalFailure::Capacity);
        }
        let mut ids = BTreeSet::new();
        for (index, entry) in self.attempts.iter().enumerate() {
            if !ids.insert(entry.request.id) {
                return Err(JournalFailure::Duplicate);
            }
            checked_transfer(entry)?;
            if entry.observation.is_none() && index + 1 != self.attempts.len() {
                return Err(JournalFailure::Pending);
            }
            if entry.request.trap_callback && entry.observation.is_some() {
                return Err(JournalFailure::Callback);
            }
        }
        ids.clear();
        for receipt in &self.receipts {
            if !ids.insert(receipt.id) {
                return Err(JournalFailure::Duplicate);
            }
            if !valid_receipt(*receipt) {
                return Err(JournalFailure::Receipt);
            }
        }
        Ok(())
    }

    pub(crate) fn admit(
        &mut self,
        caller: Principal,
        request: FundingRequest,
    ) -> Result<Principal, FundingFailure> {
        if caller != self.driver {
            return Err(FundingFailure::Denied);
        }
        if self.fenced {
            return Err(FundingFailure::Fenced);
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
        if !valid_request(request) || self.attempts.len() == MAX_ATTEMPTS {
            return Err(FundingFailure::Limit);
        }
        self.attempts.push(FundingAttemptRecord {
            request,
            observation: None,
        });
        Ok(self.peer)
    }

    pub(crate) fn complete(&mut self, request: FundingRequest, observation: FundingObservation) {
        assert!(!self.fenced, "restored funding journal is inspection-only");
        let entry = self
            .attempts
            .iter_mut()
            .find(|entry| entry.request == request)
            .expect("exact admitted attempt");
        assert!(entry.observation.is_none(), "one exact completion");
        entry.observation = Some(observation);
        checked_transfer(entry).expect("consistent callback facts");
    }

    pub(crate) fn attempts(&self, caller: Principal) -> Option<Vec<FundingAttemptRecord>> {
        (caller == self.driver).then(|| self.attempts.clone())
    }

    pub(crate) fn receipts(&self, caller: Principal) -> Option<Vec<FundingReceiptRecord>> {
        (caller == self.driver).then(|| self.receipts.clone())
    }

    pub(crate) fn check_receive(
        &self,
        caller: Principal,
        request: FundingRequest,
        available: u128,
    ) {
        assert_eq!(caller, self.peer, "configured peer only");
        assert!(!self.fenced, "restored receiver is inspection-only");
        assert!(valid_request(request), "bounded receiving intent");
        assert_eq!(available, request.offered, "exact attachment");
        assert!(
            !self.receipts.iter().any(|r| r.id == request.id),
            "unique receiving identity"
        );
        assert!(self.receipts.len() < MAX_ATTEMPTS, "receiver bound");
    }

    pub(crate) fn record_acceptance(&mut self, receipt: FundingReceiptRecord) {
        assert!(!self.fenced, "restored receiver is inspection-only");
        assert!(valid_receipt(receipt), "bounded acceptance");
        assert!(
            !self.receipts.iter().any(|r| r.id == receipt.id),
            "unique receipt"
        );
        assert!(self.receipts.len() < MAX_ATTEMPTS, "receiver bound");
        self.receipts.push(receipt);
    }
}

fn valid_principal(value: Principal) -> bool {
    value != Principal::anonymous() && value != Principal::management_canister()
}

fn valid_request(request: FundingRequest) -> bool {
    request.offered > 0 && request.offered <= MAX_OFFERED && request.accept <= request.offered
}

fn valid_receipt(receipt: FundingReceiptRecord) -> bool {
    receipt.available > 0
        && receipt.available <= MAX_OFFERED
        && receipt.accepted <= receipt.available
}

fn release_binding() -> [u8; 32] {
    *ContentDigest::compute(include_bytes!("../../../../../Cargo.lock")).as_bytes()
}

/// Storage arithmetic only; shared policy separately verifies derived reconciliation.
pub(crate) fn checked_transfer(
    entry: &FundingAttemptRecord,
) -> Result<FundingTransfer, JournalFailure> {
    if !valid_request(entry.request) {
        return Err(JournalFailure::Request);
    }
    let offered = NonZeroU128::new(entry.request.offered).expect("checked positive offer");
    let Some(observation) = entry.observation else {
        return Ok(FundingTransfer::unknown(offered));
    };
    let transfer = if observation.outcome == FundingOutcome::NotEnqueued {
        if observation.refunded.is_some() {
            return Err(JournalFailure::Transfer);
        }
        FundingTransfer::not_enqueued(offered)
    } else {
        let refunded = observation.refunded.ok_or(JournalFailure::Transfer)?;
        FundingTransfer::unbounded_callback(offered, refunded)
            .map_err(|_| JournalFailure::Transfer)?
    };
    if transfer.accepted() != observation.transport_accepted
        || transfer
            .accepted()
            .is_none_or(|accepted| accepted > entry.request.accept)
    {
        return Err(JournalFailure::Transfer);
    }
    Ok(transfer)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum JournalFailure {
    Binding,
    Release,
    Capacity,
    Request,
    Duplicate,
    Pending,
    Callback,
    Transfer,
    Receipt,
}

#[cfg(test)]
mod tests;
