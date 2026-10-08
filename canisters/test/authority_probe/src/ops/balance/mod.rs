//! Explicit local read transport and bounded response conversion; attaches no cycles.
use crate::model::{
    archive::AuthorityArchiveRecord,
    balance::{BalanceJournalRecord, MAX_AGE_NS, OutcomeRecord, ScopeRecord},
};
use blob_test_protocol::balance::{
    BalanceAmountsView, BalanceAttemptView, BalanceFailure as Failure, BalanceOutcomeView,
    BalanceRefreshRequest, BalanceScope, BalanceStatusView,
};
use candid::Principal;
use ic_blob_storage::ops::caffeine::balance::{
    BalanceProviderError, BalanceReply, BalanceReplyError, BalanceReplyLimits,
};
use ic_blob_storage::ops::caffeine::query::reply::BoundBalanceReplyError;
use ic_blob_storage::ops::caffeine::query::{CashierQuery, CashierQueryError, CashierQueryRequest};

/// Original encoded request retained alongside its raw response until completion.
pub(crate) struct BalanceResponse {
    request: CashierQueryRequest,
    bytes: Vec<u8>,
}

pub(crate) fn authorize(service: Principal, actor: Principal) -> Result<(), Failure> {
    let snapshot = super::archive::current(service, actor).ok_or(Failure::Denied)?;
    if snapshot.fenced {
        return Err(Failure::Fenced);
    }
    Ok(())
}

pub(crate) fn configure(scope: BalanceScope) -> Result<(), Failure> {
    super::mutate(|state| {
        if scope.service != state.catalog.service()
            || scope.namespace != state.registry.scope().namespace().get()
        {
            return Err(Failure::Binding);
        }
        let record = ScopeRecord::new(scope.service, scope.namespace, scope.source, scope.account)?;
        state.balance.configure(record)
    })
}

fn expected_scope(input: BalanceRefreshRequest) -> Result<ScopeRecord, Failure> {
    ScopeRecord::new(
        input.scope.service,
        input.scope.namespace,
        input.scope.source,
        input.scope.account,
    )
}

pub(crate) fn preview(
    service: Principal,
    actor: Principal,
    input: BalanceRefreshRequest,
) -> Result<(), Failure> {
    let snapshot = super::archive::current(service, actor).ok_or(Failure::Denied)?;
    snapshot
        .balance
        .check_refresh(expected_scope(input)?, input.revision, input.sequence)
}

pub(crate) fn begin(input: BalanceRefreshRequest) -> Result<(usize, ScopeRecord), Failure> {
    let scope = expected_scope(input)?;
    super::mutate(|state| {
        state
            .balance
            .check_refresh(scope, input.revision, input.sequence)?;
        state.balance.begin(ic_cdk::api::time())
    })
}

pub(crate) async fn fetch(scope: ScopeRecord) -> Result<BalanceResponse, Failure> {
    let request = CashierQueryRequest::new(
        scope.source,
        CashierQuery::Balance {
            account: scope.account,
        },
    )
    .map_err(|error| match error {
        CashierQueryError::InvalidPrincipal { .. }
        | CashierQueryError::AuditCursorAccountMismatch => Failure::Binding,
        CashierQueryError::EncodingFailed => Failure::Transport,
    })?;
    // Deliberately dispatch to a local substitute, not the maintained method name.
    // Exercise the library-owned argument shape and reply decoder across an actual
    // await. The fixture retains its driver controls and does not qualify Cashier.
    let response = ic_cdk::call::Call::bounded_wait(request.cashier(), "fixture_balance")
        .with_raw_args(request.arguments())
        .await
        .map_err(|_| Failure::Transport)?;
    Ok(BalanceResponse {
        request,
        bytes: response.into_bytes(),
    })
}

pub(crate) fn complete(
    id: usize,
    scope: ScopeRecord,
    response: Result<BalanceResponse, Failure>,
) -> Result<(), Failure> {
    // A callback after forced restore must not even reopen/mutate the frozen owner.
    if super::archive::recovery::is_fenced() {
        return Err(Failure::Fenced);
    }
    super::mutate(|state| {
        let decoded = response.and_then(|response| decode(&response, scope.source));
        state
            .balance
            .complete(id, scope, decoded, ic_cdk::api::time())
    })
}

fn decode(response: &BalanceResponse, source: Principal) -> Result<[u128; 4], Failure> {
    let limits = BalanceReplyLimits {
        max_bytes: super::bound(4096),
        decoding_quota: super::bound(100_000),
        skipping_quota: super::bound(1000),
        max_type_entries: super::bound(32),
    };
    match response
        .request
        .decode_balance_reply(source, &response.bytes, limits)
    {
        Ok(BalanceReply::ReportedBalance { balance, .. }) => Ok([
            balance.total(),
            balance.prepaid(),
            balance.promotional(),
            balance.ledger(),
        ]),
        Ok(BalanceReply::ProviderFailure(BalanceProviderError::AccountNotFound)) => {
            Err(Failure::AccountNotFound)
        }
        Ok(BalanceReply::ProviderFailure(BalanceProviderError::InternalError)) => {
            Err(Failure::ProviderInternal)
        }
        Err(BoundBalanceReplyError::Reply(BalanceReplyError::ReplyTooLarge)) => {
            Err(Failure::Oversized)
        }
        Err(BoundBalanceReplyError::Reply(BalanceReplyError::AccountMismatch)) => {
            Err(Failure::AccountMismatch)
        }
        Err(
            BoundBalanceReplyError::Binding(_)
            | BoundBalanceReplyError::Reply(
                BalanceReplyError::InvalidAccount
                | BalanceReplyError::InvalidReply
                | BalanceReplyError::InvalidBalance(_),
            ),
        ) => Err(Failure::Malformed),
    }
}

pub(crate) fn view(record: &AuthorityArchiveRecord, now: u64) -> BalanceStatusView {
    let journal: &BalanceJournalRecord = &record.balance;
    BalanceStatusView {
        configured: journal.configured.map(scope_view),
        revision: journal.revision,
        max_age_ns: MAX_AGE_NS,
        usability: journal.usability(record.fenced, now),
        attempts: journal
            .attempts
            .iter()
            .enumerate()
            .map(|(id, attempt)| BalanceAttemptView {
                sequence: u64::try_from(id + 1).expect("bounded attempts"),
                revision: attempt.revision,
                scope: scope_view(attempt.scope),
                started_at: attempt.started_at,
                outcome: attempt.outcome.map(|outcome| match outcome {
                    OutcomeRecord::Reported {
                        amounts: [total, prepaid, promotional, ledger],
                        received_at,
                    } => BalanceOutcomeView::Reported(
                        BalanceAmountsView {
                            total,
                            prepaid,
                            promotional,
                            ledger,
                        },
                        received_at,
                    ),
                    OutcomeRecord::Failed(error) => BalanceOutcomeView::Failed(error),
                }),
            })
            .collect(),
    }
}

fn scope_view(scope: ScopeRecord) -> BalanceScope {
    BalanceScope {
        service: scope.service,
        namespace: scope.namespace,
        source: scope.source,
        account: scope.account,
    }
}

pub(crate) fn configure_limits(
    input: blob_test_protocol::billing::BillingLimitsInput,
) -> Result<(), Failure> {
    let limits = ic_blob_storage_contracts::configuration::funding::FundingLimits::new(
        input.reserve,
        input.minimum,
        input.target,
    )
    .map_err(|_| Failure::InvalidLimits)?;
    let scope = ScopeRecord::new(
        input.scope.service,
        input.scope.namespace,
        input.scope.source,
        input.scope.account,
    )?;
    super::mutate(|state| {
        state
            .balance
            .configure_limits(scope, input.revision, limits)
    })
}
