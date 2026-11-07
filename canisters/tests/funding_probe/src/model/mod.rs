//! Bounded receiver evidence. Production storage owns outgoing funding and recovery.
use blob_test_protocol::funding::{FundingReceiptRecord, FundingRequest};
use ic_blob_storage_contracts::identity::ContentDigest;
use ic_platform::{Principal, principal::ExternalPrincipal};
use serde::{Deserialize, Serialize};

const MAX_RECEIPTS: usize = 16;
const MAX_OFFERED: u128 = 10_000_000_000_000;

#[derive(Serialize, Deserialize)]
pub(crate) struct FundingJournalRecord {
    service: ExternalPrincipal,
    release: [u8; 32],
    fenced: bool,
    peer: ExternalPrincipal,
    driver: ExternalPrincipal,
    receipts: Vec<FundingReceiptRecord>,
}

impl FundingJournalRecord {
    pub(crate) fn new(service: Principal, peer: Principal, driver: Principal) -> Self {
        let record = Self {
            service: service.into(),
            release: release_binding(),
            fenced: false,
            peer: peer.into(),
            driver: driver.into(),
            receipts: Vec::new(),
        };
        record.validate(service).expect("valid receiver journal");
        record
    }

    pub(crate) fn validate(&self, service: Principal) -> Result<(), &'static str> {
        if self.service != service.into()
            || self.peer == self.service
            || [self.service, self.peer, self.driver].into_iter().any(|p| {
                p == Principal::anonymous().into() || p == Principal::management_canister().into()
            })
            || self.release != release_binding()
        {
            return Err("receiver identity or release mismatch");
        }
        if self.receipts.len() > MAX_RECEIPTS
            || self.receipts.iter().enumerate().any(|(i, receipt)| {
                !valid_receipt(*receipt)
                    || self.receipts[..i]
                        .iter()
                        .any(|earlier| earlier.id == receipt.id)
            })
        {
            return Err("invalid receiver receipts");
        }
        Ok(())
    }

    pub(crate) const fn fenced(&self) -> bool {
        self.fenced
    }

    pub(crate) fn fence(&mut self) {
        self.fenced = true;
    }

    pub(crate) fn receipts(&self, caller: Principal) -> Option<Vec<FundingReceiptRecord>> {
        (caller == self.driver.into()).then(|| self.receipts.clone())
    }

    pub(crate) fn check_receive(
        &self,
        caller: Principal,
        request: FundingRequest,
        available: u128,
    ) {
        assert_eq!(
            caller,
            Principal::from_slice(self.peer.as_slice()),
            "configured peer only"
        );
        assert!(!self.fenced, "restored receiver is inspection-only");
        assert!(
            request.offered > 0
                && request.offered <= MAX_OFFERED
                && request.accept <= request.offered,
            "bounded receiving intent"
        );
        assert_eq!(available, request.offered, "exact attachment");
        assert!(
            !self.receipts.iter().any(|r| r.id == request.id),
            "unique receiving identity"
        );
        assert!(self.receipts.len() < MAX_RECEIPTS, "receiver bound");
    }

    pub(crate) fn record_acceptance(&mut self, receipt: FundingReceiptRecord) {
        assert!(!self.fenced, "restored receiver is inspection-only");
        assert!(valid_receipt(receipt), "bounded acceptance");
        assert!(
            !self.receipts.iter().any(|r| r.id == receipt.id),
            "unique receipt"
        );
        assert!(self.receipts.len() < MAX_RECEIPTS, "receiver bound");
        self.receipts.push(receipt);
    }
}

fn valid_receipt(receipt: FundingReceiptRecord) -> bool {
    receipt.available > 0
        && receipt.available <= MAX_OFFERED
        && receipt.accepted <= receipt.available
}

fn release_binding() -> [u8; 32] {
    *ContentDigest::compute(include_bytes!("../../../../../Cargo.lock")).as_bytes()
}

#[cfg(test)]
mod tests;
