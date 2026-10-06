//! Bounded v1 journal schemas and transitions; no storage or transport effects.
pub(crate) mod response;
use super::credit::{FundingCreditConfirmation, FundingCreditError, FundingCreditReceipt};
use super::renewal::{FundingBudgetRenewal, FundingRenewalError};
use super::{
    FundingIntent, FundingIntentError, FundingIntentState, FundingIntentView,
    FundingTransportOutcome,
};
use crate::model::{
    billing::{
        allocation::{
            FundingAllocation, FundingAllocationError, FundingAllocationView,
            record::FundingAllocationRecord,
        },
        transfer::FundingTransfer,
    },
    service::configuration::ServiceConfiguration,
};
use candid::{CandidType, DecoderConfig, Deserialize, Principal, decode_one_with_config};
use ic_memory::ic_stable_structures::{Storable, storable::Bound};
use response::FundingResponseRecord;
use std::{borrow::Cow, num::NonZeroU128};

#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) enum FundingPhaseRecord {
    Prepared,
    Uncertain,
    NotEnqueued,
    Callback { refunded: u128 },
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
enum FundingCreditRecord {
    Required,
    Confirmed {
        accepted: u128,
        receipt_digest: [u8; 32],
    },
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
enum FundingMethodRecord {
    AccountTopUpV1,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct FundingIntentRecord {
    version: u8,
    service: Principal,
    cashier: Principal,
    account: Principal,
    namespace: u128,
    operation: u128,
    offered: u128,
    method: FundingMethodRecord,
    target_balance: Option<u128>,
    phase: FundingPhaseRecord,
    response: FundingResponseRecord,
    credit: FundingCreditRecord,
    renewed_allocation: u128,
}
impl FundingIntentRecord {
    pub(crate) fn new(input: FundingIntent) -> Self {
        Self {
            version: 1,
            service: input.service,
            cashier: input.cashier,
            account: input.account,
            namespace: input.namespace.get(),
            operation: input.operation.get(),
            offered: input.offered.get(),
            method: FundingMethodRecord::AccountTopUpV1,
            target_balance: input.target_balance.map(NonZeroU128::get),
            phase: FundingPhaseRecord::Prepared,
            response: FundingResponseRecord::Missing,
            credit: FundingCreditRecord::Required,
            renewed_allocation: 0,
        }
    }
    pub(crate) fn view(self) -> Option<FundingIntentView> {
        if self.version != 1 || !self.response.matches(self.phase) {
            return None;
        }
        let intent = FundingIntent {
            service: self.service,
            cashier: self.cashier,
            account: self.account,
            namespace: NonZeroU128::new(self.namespace)?,
            operation: NonZeroU128::new(self.operation)?,
            offered: NonZeroU128::new(self.offered)?,
            target_balance: match self.target_balance {
                Some(value) => Some(NonZeroU128::new(value)?),
                None => None,
            },
        };
        let state = match self.phase {
            FundingPhaseRecord::Prepared => FundingIntentState::Prepared,
            FundingPhaseRecord::Uncertain => FundingIntentState::Uncertain,
            FundingPhaseRecord::NotEnqueued => FundingIntentState::NotEnqueued,
            FundingPhaseRecord::Callback { refunded } => {
                FundingTransfer::unbounded_callback(intent.offered, refunded).ok()?;
                FundingIntentState::Callback { refunded }
            }
        };
        if let FundingCreditRecord::Confirmed {
            accepted,
            receipt_digest,
        } = self.credit
        {
            let FundingIntentState::Callback { refunded } = state else {
                return None;
            };
            if accepted == 0
                || accepted != intent.offered.get() - refunded
                || receipt_digest == [0; 32]
            {
                return None;
            }
        }
        if self.renewed_allocation != 0
            && self
                .credit()
                .is_none_or(|credit| self.renewed_allocation > credit.accepted_cycles().get())
        {
            return None;
        }
        Some(FundingIntentView { intent, state })
    }
    pub(crate) fn attempted(mut self) -> Result<Self, FundingIntentError> {
        if self.phase != FundingPhaseRecord::Prepared {
            return Err(FundingIntentError::AlreadyAttempted);
        }
        self.phase = FundingPhaseRecord::Uncertain;
        Ok(self)
    }
    pub(crate) fn complete(
        mut self,
        outcome: FundingTransportOutcome,
    ) -> Result<Self, FundingIntentError> {
        let phase = match outcome {
            FundingTransportOutcome::NotEnqueued => FundingPhaseRecord::NotEnqueued,
            FundingTransportOutcome::Callback { refunded } => {
                FundingTransfer::unbounded_callback(
                    NonZeroU128::new(self.offered).expect("validated offer"),
                    refunded,
                )?;
                FundingPhaseRecord::Callback { refunded }
            }
        };
        match self.phase {
            FundingPhaseRecord::Prepared => return Err(FundingIntentError::NotAttempted),
            FundingPhaseRecord::Uncertain => {}
            prior if prior == phase => return Ok(self),
            _ => return Err(FundingIntentError::OutcomeConflict),
        }
        self.phase = phase;
        Ok(self)
    }
    pub(crate) fn transfer(self) -> Option<FundingTransfer> {
        let view = self.view()?;
        Some(match view.state {
            FundingIntentState::Prepared | FundingIntentState::Uncertain => {
                FundingTransfer::unknown(view.intent.offered)
            }
            FundingIntentState::NotEnqueued => FundingTransfer::not_enqueued(view.intent.offered),
            FundingIntentState::Callback { refunded } => {
                FundingTransfer::unbounded_callback(view.intent.offered, refunded).ok()?
            }
        })
    }
    pub(crate) fn credit(self) -> Option<FundingCreditReceipt> {
        match self.credit {
            FundingCreditRecord::Required => None,
            FundingCreditRecord::Confirmed {
                accepted,
                receipt_digest,
            } => Some(FundingCreditReceipt {
                accepted_cycles: NonZeroU128::new(accepted).expect("validated credit receipt"),
                receipt_digest,
            }),
        }
    }
    pub(crate) fn confirm_credit(
        mut self,
        confirmation: FundingCreditConfirmation,
    ) -> Result<Self, FundingCreditError> {
        let accepted = self
            .transfer()
            .and_then(FundingTransfer::accepted)
            .and_then(NonZeroU128::new)
            .ok_or(FundingCreditError::AcceptanceRequired)?;
        if accepted != confirmation.accepted_cycles {
            return Err(FundingCreditError::AmountMismatch);
        }
        if confirmation.receipt_digest == [0; 32] {
            return Err(FundingCreditError::EmptyReceipt);
        }
        let next = FundingCreditRecord::Confirmed {
            accepted: accepted.get(),
            receipt_digest: confirmation.receipt_digest,
        };
        if self.credit != FundingCreditRecord::Required && self.credit != next {
            return Err(FundingCreditError::Conflict);
        }
        self.credit = next;
        Ok(self)
    }
    pub(crate) const fn renewed_allocation(self) -> Option<NonZeroU128> {
        NonZeroU128::new(self.renewed_allocation)
    }
    pub(crate) fn renew(
        mut self,
        input: FundingBudgetRenewal,
    ) -> Result<Self, FundingRenewalError> {
        let credit = self.credit().ok_or(FundingRenewalError::CreditRequired)?;
        if input.additional.get() > credit.accepted_cycles().get() {
            return Err(FundingRenewalError::Amount);
        }
        if self.renewed_allocation != 0 && self.renewed_allocation != input.additional.get() {
            return Err(FundingRenewalError::Conflict);
        }
        self.renewed_allocation = input.additional.get();
        Ok(self)
    }
    pub(crate) const fn response(self) -> FundingResponseRecord {
        self.response
    }
    pub(crate) fn with_response(
        mut self,
        response: FundingResponseRecord,
    ) -> Result<Self, FundingIntentError> {
        if !response.matches(self.phase)
            || (self.response != FundingResponseRecord::Missing && self.response != response)
        {
            return Err(FundingIntentError::OutcomeConflict);
        }
        self.response = response;
        Ok(self)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct FundingJournalRecord {
    version: u8,
    service: Principal,
    operator: Principal,
    cashier: Principal,
    account: Principal,
    namespace: u128,
    allocation: FundingAllocationRecord,
    count: u64,
    last: u128,
}
impl FundingJournalRecord {
    pub(crate) fn new(config: &ServiceConfiguration, allocation: FundingAllocation) -> Self {
        let bindings = config.bindings();
        Self {
            version: 1,
            service: bindings.service,
            operator: bindings.operator,
            cashier: config.billing().cashier(),
            account: bindings.payment_account,
            namespace: bindings.namespace.get(),
            allocation: FundingAllocationRecord::new(allocation),
            count: 0,
            last: 0,
        }
    }
    pub(crate) fn matches(
        self,
        config: &ServiceConfiguration,
        allocation: FundingAllocation,
    ) -> bool {
        let expected = Self::new(config, allocation);
        self.version == 1
            && self.service == expected.service
            && self.operator == expected.operator
            && self.cashier == expected.cashier
            && self.account == expected.account
            && self.namespace == expected.namespace
            && self.allocation.matches(allocation)
    }
    pub(crate) fn scope_matches(self, input: FundingIntent) -> bool {
        input.service == self.service
            && input.cashier == self.cashier
            && input.account == self.account
            && input.namespace.get() == self.namespace
    }
    pub(crate) const fn count(self) -> u64 {
        self.count
    }
    pub(crate) const fn last(self) -> u128 {
        self.last
    }
    pub(crate) const fn limit(self) -> u64 {
        self.allocation.max_attempts()
    }
    pub(crate) const fn view(self) -> FundingAllocationView {
        self.allocation.view()
    }
    pub(crate) fn reserve(mut self, input: FundingIntent) -> Result<Self, FundingAllocationError> {
        if self.count >= self.limit() {
            return Err(FundingAllocationError::Capacity);
        }
        self.allocation = self.allocation.reserve(
            input.offered,
            usize::try_from(self.count).expect("portable lifetime bound"),
        )?;
        self.count += 1;
        self.last = input.operation.get();
        Ok(self)
    }
    pub(crate) fn renew(mut self, additional: NonZeroU128) -> Result<Self, FundingAllocationError> {
        self.allocation = self.allocation.renew(additional)?;
        Ok(self)
    }
    pub(crate) fn confirm_credit(
        mut self,
        accepted: NonZeroU128,
    ) -> Result<Self, FundingAllocationError> {
        self.allocation = self.allocation.confirm_credit(accepted)?;
        Ok(self)
    }
    pub(crate) fn resolve(
        mut self,
        transfer: FundingTransfer,
    ) -> Result<Self, FundingAllocationError> {
        self.allocation = self.allocation.resolve(transfer)?;
        Ok(self)
    }
}
/// The accounting row and its bounded receipt index share one existing memory.
/// Zero digest belongs exclusively to accounting; receipts always have nonzero keys.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
#[expect(
    clippy::large_enum_variant,
    reason = "One bounded persisted union keeps the sole totals row and receipt index together without heap boxing each accounting decode"
)]
pub(crate) enum FundingAccountingRecord {
    Totals(FundingJournalRecord),
    Receipt { operation: u128 },
}
impl FundingAccountingRecord {
    pub(crate) const fn totals(self) -> Option<FundingJournalRecord> {
        match self {
            Self::Totals(totals) => Some(totals),
            Self::Receipt { .. } => None,
        }
    }
}
macro_rules! codec {
    ($record:ty, $max:expr) => {
        impl Storable for $record {
            fn to_bytes(&self) -> Cow<'_, [u8]> {
                Cow::Owned(self.into_bytes())
            }
            fn into_bytes(self) -> Vec<u8> {
                let bytes = candid::encode_one(self).expect("funding record encoding");
                assert!(bytes.len() <= $max, "funding record bound");
                bytes
            }
            fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
                assert!(bytes.len() <= $max, "funding record bound");
                let mut config = DecoderConfig::new();
                config
                    .set_decoding_quota(30_000)
                    .set_skipping_quota(1000)
                    .set_max_type_len(64)
                    .set_max_header_len($max)
                    .set_full_error_message(false);
                decode_one_with_config(&bytes, &config).expect("valid same-release funding record")
            }
            const BOUND: Bound = Bound::Bounded {
                max_size: $max as u32,
                is_fixed_size: false,
            };
        }
    };
}
codec!(FundingIntentRecord, 1024);
codec!(FundingJournalRecord, 1024);

codec!(FundingAccountingRecord, 1024);
