//! Bounded operator budget authority; never a refund or provider credit claim.
use super::FundingIntent;
use std::num::NonZeroU128;
use thiserror::Error;

/// One host-authorized increase, uniquely bound to an original credited top-up.
/// Each original intent permits at most one immutable increase, no larger than
/// its accepted attachment. This grants planning allowance, not real cycles.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingBudgetRenewal {
    /// Full original scope, identity, offer and target; never a new payment.
    pub intent: FundingIntent,
    /// Additional allowance within the installation's cumulative ceiling.
    pub additional: NonZeroU128,
}

/// Refusal before any durable budget mutation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum FundingRenewalError {
    /// Known accepted transport and exact credit confirmation are required.
    #[error("confirmed original funding credit required")]
    CreditRequired,
    /// A single increase cannot exceed this original acceptance.
    #[error("funding renewal exceeds original accepted cycles")]
    Amount,
    /// This original intent already has a different immutable increase.
    #[error("funding renewal conflict")]
    Conflict,
    /// New increases occur only after the latest complete intent, before the next.
    #[error("funding renewal requires the latest settled intent")]
    NotLatest,
    /// Uncredited or uncertain history is not a completed funding boundary.
    #[error("funding history is not fully reconciled")]
    Unreconciled,
}
