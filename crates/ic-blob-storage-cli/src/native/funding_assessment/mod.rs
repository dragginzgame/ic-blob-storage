//! One authenticated passive service assessment, with no payment or balance inference.
use super::{Failure, arguments::Options, query, reply};
use candid::{de::DecoderConfig, decode_one_with_config};
use ic_blob_storage::{
    dto::funding::assessment::{
        FundingPreparationBlocker as B, FundingPreparationFailure as E,
        FundingPreparationRequest as Request, FundingPreparationResponse as Response,
    },
    ops::service::funding::assessment::FUNDING_PREPARATION_METHOD,
};
use serde_json::{Value, json};

fn decode(request: Request, bytes: &[u8]) -> Result<Response, Failure> {
    if bytes.len() > 4096 {
        return Err(Failure::ReplyTooLarge);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(100_000)
        .set_skipping_quota(0)
        .set_max_type_len(128)
        .set_max_header_len(4096)
        .set_full_error_message(false);
    let response: Result<Response, E> =
        decode_one_with_config(bytes, &config).map_err(|_| Failure::InvalidReply)?;
    let response = response.map_err(Failure::FundingAssessmentRefused)?;
    if response.request != request {
        return Err(Failure::Binding);
    }
    validate(&response)?;
    Ok(response)
}
fn validate(response: &Response) -> Result<(), Failure> {
    let journal = response.journal;
    let blockers = &response.blockers;
    if blockers.len() > 10
        || blockers
            .iter()
            .enumerate()
            .any(|(i, b)| blockers[..i].contains(b))
        || journal.intent_capacity == 0
        || journal.retained_intents > journal.intent_capacity
        || (journal.retained_intents == 0) != journal.last_operation.is_none()
        || journal.last_operation == Some(0)
        || journal.attachment_allowance > journal.available_allocation
        || !super::reply::valid_funding_budget(&journal)
        || journal.uncredited_accepted > journal.transport_accepted
    {
        return Err(Failure::InvalidReply);
    }
    for b in [
        B::ProviderUnqualified,
        B::RecoveryUnknown,
        B::FundingUnknown,
        B::SpendabilityUnknown,
    ] {
        if !blockers.contains(&b) {
            return Err(Failure::InvalidReply);
        }
    }
    for (b, needed) in [
        (B::JournalFenced, journal.fenced),
        (
            B::JournalUncredited,
            journal.uncredited_accepted > 0 || journal.reserved_or_uncertain > 0,
        ),
        (
            B::JournalFull,
            journal.retained_intents == journal.intent_capacity,
        ),
    ] {
        if blockers.contains(&b) != needed {
            return Err(Failure::InvalidReply);
        }
    }
    let retained = blockers.contains(&B::IdentityRetained);
    let stale = blockers.contains(&B::IdentityStale);
    if retained && stale
        || (retained || stale)
            != journal
                .last_operation
                .is_some_and(|last| response.request.operation <= last)
        || journal.last_operation == Some(response.request.operation) && !retained
    {
        return Err(Failure::InvalidReply);
    }
    let mut reserve = None;
    for b in blockers {
        if let B::AllocationReserve {
            transferable_cycles,
        } = b
            && reserve.replace(*transferable_cycles).is_some()
        {
            return Err(Failure::InvalidReply);
        }
    }
    if (response.request.offered > journal.attachment_allowance) != reserve.is_some()
        || reserve.is_some_and(|n| n != journal.attachment_allowance)
    {
        return Err(Failure::InvalidReply);
    }
    Ok(())
}
fn blocker(b: B) -> Value {
    match b {
        B::ProviderUnqualified => json!({"kind":"provider_unqualified"}),
        B::JournalFenced => json!({"kind":"journal_fenced"}),
        B::JournalUncredited => json!({"kind":"journal_uncredited"}),
        B::IdentityRetained => json!({"kind":"identity_retained"}),
        B::IdentityStale => json!({"kind":"identity_stale"}),
        B::JournalFull => json!({"kind":"journal_full"}),
        B::AllocationReserve {
            transferable_cycles,
        } => {
            json!({"kind":"allocation_reserve","attachment_allowance":transferable_cycles.to_string()})
        }
        B::RecoveryUnknown => json!({"kind":"recovery_unknown"}),
        B::FundingUnknown => json!({"kind":"funding_unknown"}),
        B::SpendabilityUnknown => json!({"kind":"spendability_unknown"}),
    }
}
pub(super) async fn run(options: &Options, request: Request) -> Result<Value, Failure> {
    let bytes = query(
        options,
        request.scope.service,
        FUNDING_PREPARATION_METHOD,
        candid::encode_one(request).map_err(|_| Failure::Arguments)?,
    )
    .await?;
    let response = decode(request, &bytes)?;
    Ok(output(options, &response))
}
fn output(options: &Options, response: &Response) -> Value {
    let r = response.request;
    json!({"schema":1,"observation":"funding_preparation_assessment",
        "operator":options.actor.to_text(),"network":options.network,"url":options.url.as_str(),
        "verification":"query_signatures",
        "request":{"scope":{"service":r.scope.service.to_text(),"namespace":r.scope.namespace.to_string(),
            "cashier":r.scope.cashier.to_text(),"payment_account":r.scope.payment_account.to_text()},
            "operation":r.operation.to_string(),"offered":r.offered.to_string(),"target_balance":r.target_balance.map(|n|n.to_string())},
        "journal":reply::funding(&response.journal),
        "blockers":response.blockers.iter().copied().map(blocker).collect::<Vec<_>>(),
        "preparation_authorized":false,"dispatch_authorized":false,"retry_authorized":false,
        "provider_credit":"not_established","spendability":"not_established"})
}
pub(super) const fn refusal_code(error: E) -> &'static str {
    match error {
        E::Invalid => "funding_invalid",
        E::Denied => "funding_denied",
        E::Binding => "funding_binding",
        E::Conflict => "funding_conflict",
        E::Internal => "funding_internal",
    }
}

#[cfg(test)]
mod tests;
