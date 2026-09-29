//! Account observation composition reuses maintained requests and decoders.
use crate::{
    dto::{
        account::{
            AccountInspectionFailure, AccountInspectionKind, AccountInspectionRequest,
            AccountObservation, AccountRelationshipView,
        },
        funding::outcome::FundingReportedBalance,
    },
    model::service::upload::UploadContext,
    ops::{
        caffeine::{
            balance::{BalanceProviderError, BalanceReply, BalanceReplyError, BalanceReplyLimits},
            query::{
                CashierQuery, CashierQueryRequest,
                reply::{BoundBalanceReplyError, BoundRelationshipReplyError},
                transport::{CashierQueryResponse, replicated::ReplicatedQueryError},
            },
            relationship::{
                PaymentRelationshipBinding, PaymentRelationshipProviderError,
                PaymentRelationshipReply, PaymentRelationshipReplyError,
                PaymentRelationshipReplyLimits,
            },
        },
        service::operator::{self, OperatorStores},
    },
};
use ic_memory::ic_stable_structures::Memory;
use std::num::NonZeroUsize;

/// Synchronous host borrowing across independent stages; never retain borrows over await.
/// Host configuration must remain immutable during the invocation. No cached reports
/// or ingress-provided account evidence can satisfy this interface.
pub trait AccountInspectionAccess {
    /// Host-granted stable-memory implementation.
    type Memory: Memory;
    /// Borrow the same installed owners on every call, without performing transport.
    fn with_account_stores<R>(&self, f: impl FnOnce(OperatorStores<'_, Self::Memory>) -> R) -> R;
}
/// Explicit byte/work/type limits for both canonical account reply decoders.
#[derive(Clone, Copy, Debug)]
pub struct AccountInspectionLimits {
    /// Bytes checked at transport and decoder boundaries.
    pub max_bytes: NonZeroUsize,
    /// Candid decoding work.
    pub decoding_quota: NonZeroUsize,
    /// Candid skipped-value work.
    pub skipping_quota: NonZeroUsize,
    /// Type-table entries.
    pub max_type_entries: NonZeroUsize,
}
pub(crate) fn check<M: Memory>(
    stores: OperatorStores<'_, M>,
    context: UploadContext,
    input: AccountInspectionRequest,
) -> Result<(), AccountInspectionFailure> {
    use crate::dto::operator::LocalStatusFailure;
    let status = operator::inspect(stores, context, input.scope).map_err(|e| match e {
        LocalStatusFailure::Denied => AccountInspectionFailure::Denied,
        LocalStatusFailure::Binding => AccountInspectionFailure::Binding,
        LocalStatusFailure::Internal => AccountInspectionFailure::Internal,
    })?;
    if status.uploads.fenced
        || status.funding.fenced
        || status.gateways.fenced
        || status.reads.fenced
    {
        return Err(AccountInspectionFailure::Fenced);
    }
    Ok(())
}
pub(crate) fn request(
    input: AccountInspectionRequest,
) -> Result<CashierQueryRequest, AccountInspectionFailure> {
    let query = match input.kind {
        AccountInspectionKind::Balance => CashierQuery::Balance {
            account: input.scope.payment_account,
        },
        AccountInspectionKind::PaymentRelationship => {
            CashierQuery::PaymentRelationship(PaymentRelationshipBinding {
                paid_canister: input.scope.service,
                payment_account: input.scope.payment_account,
            })
        }
    };
    CashierQueryRequest::new(input.scope.cashier, query)
        .map_err(|_| AccountInspectionFailure::Invalid)
}
pub(crate) fn transport_error(error: ReplicatedQueryError) -> AccountInspectionFailure {
    use AccountInspectionFailure as F;
    match error {
        ReplicatedQueryError::InvalidPrincipal | ReplicatedQueryError::TimeoutOutOfRange => {
            F::Invalid
        }
        ReplicatedQueryError::Binding | ReplicatedQueryError::Method => F::Binding,
        ReplicatedQueryError::Execution => F::Denied,
        ReplicatedQueryError::NotEnqueued => F::NotEnqueued,
        ReplicatedQueryError::Rejected(code) => F::Rejected(code),
        ReplicatedQueryError::ReplyTooLarge => F::ReplyTooLarge,
    }
}
pub(crate) fn decode(
    request: &CashierQueryRequest,
    response: &CashierQueryResponse,
    limits: AccountInspectionLimits,
) -> Result<AccountObservation, AccountInspectionFailure> {
    use AccountInspectionFailure as F;
    use AccountObservation as O;
    match request.query() {
        CashierQuery::Balance { .. } => {
            let reply = request
                .decode_balance_reply(
                    response.cashier,
                    &response.bytes,
                    BalanceReplyLimits {
                        max_bytes: limits.max_bytes,
                        decoding_quota: limits.decoding_quota,
                        skipping_quota: limits.skipping_quota,
                        max_type_entries: limits.max_type_entries,
                    },
                )
                .map_err(|e| match e {
                    BoundBalanceReplyError::Binding(_)
                    | BoundBalanceReplyError::Reply(BalanceReplyError::AccountMismatch) => {
                        F::Binding
                    }
                    BoundBalanceReplyError::Reply(BalanceReplyError::ReplyTooLarge) => {
                        F::ReplyTooLarge
                    }
                    BoundBalanceReplyError::Reply(_) => F::InvalidReply,
                })?;
            Ok(match reply {
                BalanceReply::ReportedBalance { balance, .. } => {
                    O::Balance(FundingReportedBalance {
                        total: balance.total(),
                        prepaid: balance.prepaid(),
                        promotional: balance.promotional(),
                        ledger: balance.ledger(),
                    })
                }
                BalanceReply::ProviderFailure(BalanceProviderError::AccountNotFound) => {
                    O::AccountNotFound
                }
                BalanceReply::ProviderFailure(BalanceProviderError::InternalError) => {
                    O::ProviderInternalError
                }
            })
        }
        CashierQuery::PaymentRelationship(_) => {
            let reply = request
                .decode_relationship_reply(
                    response.cashier,
                    &response.bytes,
                    PaymentRelationshipReplyLimits {
                        max_bytes: limits.max_bytes,
                        decoding_quota: limits.decoding_quota,
                        skipping_quota: limits.skipping_quota,
                        max_type_entries: limits.max_type_entries,
                    },
                )
                .map_err(|e| match e {
                    BoundRelationshipReplyError::Binding(_)
                    | BoundRelationshipReplyError::Reply(
                        PaymentRelationshipReplyError::PaidCanisterMismatch
                        | PaymentRelationshipReplyError::PaymentAccountMismatch,
                    ) => F::Binding,
                    BoundRelationshipReplyError::Reply(
                        PaymentRelationshipReplyError::ReplyTooLarge,
                    ) => F::ReplyTooLarge,
                    BoundRelationshipReplyError::Reply(_) => F::InvalidReply,
                })?;
            Ok(match reply {
                PaymentRelationshipReply::ReportedRelationship(v) => {
                    O::Relationship(Box::new(AccountRelationshipView {
                        paid_canister: v.binding().paid_canister,
                        payment_account: v.binding().payment_account,
                        spending_limit_per_day: v.spending_limit_per_day().clone(),
                        current_period_spent: v.current_period_spent().clone(),
                        current_period_start: v.current_period_start(),
                        added_timestamp: v.added_timestamp(),
                        expiration_timestamp: v.expiration_timestamp(),
                        bandwidth_baseline_uploaded: v.bandwidth_baseline_uploaded().clone(),
                        bandwidth_baseline_downloaded: v.bandwidth_baseline_downloaded().clone(),
                        bandwidth_baseline_ts_ns: v.bandwidth_baseline_ts_ns(),
                    }))
                }
                PaymentRelationshipReply::NoRelationshipReported => O::NoRelationshipReported,
                PaymentRelationshipReply::ProviderFailure(e) => match e {
                    PaymentRelationshipProviderError::RelationshipNotFound(p) => {
                        O::RelationshipNotFound(p)
                    }
                    PaymentRelationshipProviderError::NotAuthorized(p) => O::NotAuthorized(p),
                    PaymentRelationshipProviderError::InvalidRequest => O::InvalidRequest,
                    PaymentRelationshipProviderError::InternalError => O::ProviderInternalError,
                },
            })
        }
        CashierQuery::StorageGateways | CashierQuery::AuditLog(_) => Err(F::Binding),
    }
}
