//! Exact host-established credit evidence, independent of transport acceptance.
use super::FundingIntent;
use std::num::NonZeroU128;
use thiserror::Error;

/// Host-internal confirmation, never an ingress DTO or self-authenticating proof.
/// The host must verify provider authority and that this unique receipt credits
/// this exact account/intent. Hash the original provider receipt, not a locally
/// renamed operation or a balance snapshot. Preserve that evidence outside this
/// bounded fingerprint. No credential or unbounded response belongs here.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingCreditConfirmation {
    /// Complete original service, namespace, Cashier, account, operation and offer.
    pub intent: FundingIntent,
    /// Full positive transport-accepted attachment reconciled by this evidence.
    /// This is not a provider-balance denomination or a new allocation.
    pub accepted_cycles: NonZeroU128,
    /// SHA-256 of independently authenticated, uniquely attributable credit evidence.
    pub receipt_digest: [u8; 32],
}

/// Retained bounded receipt. Its presence records the trusted host's confirmation,
/// not independent cryptographic verification by the journal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FundingCreditReceipt {
    pub(crate) accepted_cycles: NonZeroU128,
    pub(crate) receipt_digest: [u8; 32],
}
impl FundingCreditReceipt {
    /// Exact accepted attachment confirmed once; remains spent in allocation totals.
    #[must_use]
    pub const fn accepted_cycles(self) -> NonZeroU128 {
        self.accepted_cycles
    }
    /// Fingerprint of the preserved provider evidence; not a credential or proof token.
    #[must_use]
    pub const fn receipt_digest(self) -> [u8; 32] {
        self.receipt_digest
    }
}

/// Credit cannot be invented, moved to another intent or overwritten.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum FundingCreditError {
    /// Missing/uncertain transport, proven unsent or fully refunded attachments.
    #[error("positive exact transport acceptance required")]
    AcceptanceRequired,
    /// Confirmation does not cover the exact full accepted attachment.
    #[error("credit confirmation amount mismatch")]
    AmountMismatch,
    /// A neutral fingerprint is not a recorded receipt.
    #[error("credit receipt fingerprint required")]
    EmptyReceipt,
    /// A retained confirmation has a different amount or fingerprint.
    #[error("credit confirmation conflict")]
    Conflict,
    /// The same receipt was already assigned to another funding intent.
    #[error("credit receipt already assigned")]
    ReceiptReused,
}
