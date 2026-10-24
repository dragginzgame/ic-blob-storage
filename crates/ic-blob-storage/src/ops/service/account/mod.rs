//! Account observation composition reuses maintained requests and decoders.
use crate::ops::caffeine::balance::BalanceProviderError;
use crate::ops::caffeine::balance::BalanceReply;
use crate::ops::caffeine::balance::BalanceReplyError;
use crate::ops::caffeine::decoding::CaffeineReplyLimits;

use crate::ops::caffeine::query::CashierQuery;
use crate::ops::caffeine::query::CashierQueryRequest;
use crate::ops::caffeine::query::reply::BoundBalanceReplyError;
use crate::ops::caffeine::query::reply::BoundRelationshipReplyError;
use crate::ops::caffeine::query::transport::CashierQueryResponse;
use crate::ops::caffeine::query::transport::replicated::ReplicatedQueryError;
use crate::ops::caffeine::relationship::PaymentRelationshipBinding;
use crate::ops::caffeine::relationship::PaymentRelationshipProviderError;
use crate::ops::caffeine::relationship::PaymentRelationshipReply;
use crate::ops::caffeine::relationship::PaymentRelationshipReplyError;

use crate::ops::service::operator::OperatorStores;
use ic_blob_storage_contracts::dto::account::AccountInspectionFailure;
use ic_blob_storage_contracts::dto::account::AccountInspectionKind;
use ic_blob_storage_contracts::dto::account::AccountInspectionRequest;
use ic_blob_storage_contracts::dto::account::AccountObservation;
use ic_blob_storage_contracts::dto::account::AccountRelationshipView;
use ic_blob_storage_contracts::dto::funding::outcome::FundingReportedBalance;
use ic_memory::Memory;

/// Synchronous host borrowing across independent stages; never retain borrows over await.
/// Host configuration must remain immutable during the invocation. No cached reports
/// or ingress-provided account evidence can satisfy this interface.
pub trait AccountInspectionAccess {
    /// Backing memory of the host runtime; store owners retain exclusive leases.
    type Memory: Memory;
    /// Borrow the same installed owners on every call, without performing transport.
    fn with_account_stores<R>(&self, f: impl FnOnce(OperatorStores<'_, Self::Memory>) -> R) -> R;
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
    limits: CaffeineReplyLimits,
) -> Result<AccountObservation, AccountInspectionFailure> {
    use AccountInspectionFailure as F;
    use AccountObservation as O;
    match request.query() {
        CashierQuery::Balance { .. } => {
            let reply = request
                .decode_balance_reply(response.cashier, &response.bytes, limits)
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
                .decode_relationship_reply(response.cashier, &response.bytes, limits)
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
