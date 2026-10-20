//! Explicit account-scoped audit inputs; cursor semantics remain unqualified.
use std::num::NonZeroU64;

use ic_platform::Principal;
use ic_platform::candid::contract::CandidType;

use super::{CashierQueryError, CashierQueryPrincipal, validate};

/// One diagnostic page request. No defaults, all-account mode or automatic follow-up.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuditLogQuery {
    /// Explicit account retained alongside the encoded request.
    pub account: Principal,
    /// Positive requested page bound, also enforced on reported reply counts.
    pub max_entries: NonZeroU64,
    /// Optional provider event filter; CSV rows are not interpreted or verified.
    pub event_type: Option<AuditLogEvent>,
    /// Explicit opaque cursor. It cannot replace the requested account or filter.
    pub continuation: Option<AuditLogCursor>,
}

/// Caller-supplied cursor fields, not proof of progress, ownership or completeness.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuditLogCursor {
    /// Preserve absence. A present account must equal the request account.
    pub account: Option<Principal>,
    /// Opaque provider sequence; zero and the full nat64 range are preserved.
    pub sequence: u64,
}

/// Advertised event filters, without interpreting their accounting effects.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuditLogEvent {
    /// Provider spend event.
    Spend,
    /// Payment relationship addition.
    RelationshipAdd,
    /// Payment relationship change.
    RelationshipUpdate,
    /// Ledger deposit event.
    LedgerDeposit,
    /// Direct top-up event.
    TopUp,
    /// Payment relationship removal.
    RelationshipRemove,
    /// Provider unit charge.
    UnitCharge,
}

pub(super) fn encode(input: AuditLogQuery) -> Result<Vec<u8>, CashierQueryError> {
    validate(input.account, CashierQueryPrincipal::AuditAccount)?;
    if input
        .continuation
        .and_then(|c| c.account)
        .is_some_and(|account| account != input.account)
    {
        return Err(CashierQueryError::AuditCursorAccountMismatch);
    }
    ic_platform::candid::contract::encode_args((wire::AuditLogDownloadRequest {
        account: Some(input.account.into()),
        max_entries: Some(input.max_entries.get()),
        event_type: input.event_type.map(wire::AuditEventType::from),
        continuation_token: input.continuation.map(|c| wire::AuditLogContinuationToken {
            account: c.account.map(Into::into),
            sequence: c.sequence,
        }),
    },))
    .map_err(|_| CashierQueryError::EncodingFailed)
}

mod wire {
    use super::AuditLogEvent;
    use super::CandidType;
    use ic_platform::candid::contract::Principal;

    #[derive(CandidType)]
    #[candid_path("ic_platform::candid::contract")]
    pub(super) struct AuditLogDownloadRequest {
        pub continuation_token: Option<AuditLogContinuationToken>,
        pub max_entries: Option<u64>,
        pub account: Option<Principal>,
        pub event_type: Option<AuditEventType>,
    }

    #[derive(CandidType)]
    #[candid_path("ic_platform::candid::contract")]
    pub(super) struct AuditLogContinuationToken {
        pub account: Option<Principal>,
        pub sequence: u64,
    }

    #[derive(CandidType)]
    #[candid_path("ic_platform::candid::contract")]
    pub(super) enum AuditEventType {
        Spend,
        RelationshipAdd,
        RelationshipUpdate,
        LedgerDeposit,
        TopUp,
        RelationshipRemove,
        UnitCharge,
    }

    impl From<AuditLogEvent> for AuditEventType {
        fn from(value: AuditLogEvent) -> Self {
            match value {
                AuditLogEvent::Spend => Self::Spend,
                AuditLogEvent::RelationshipAdd => Self::RelationshipAdd,
                AuditLogEvent::RelationshipUpdate => Self::RelationshipUpdate,
                AuditLogEvent::LedgerDeposit => Self::LedgerDeposit,
                AuditLogEvent::TopUp => Self::TopUp,
                AuditLogEvent::RelationshipRemove => Self::RelationshipRemove,
                AuditLogEvent::UnitCharge => Self::UnitCharge,
            }
        }
    }
}

#[cfg(test)]
mod tests;
