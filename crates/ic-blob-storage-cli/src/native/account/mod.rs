//! One scoped service read update; reports never become credit or dispatch authority.
use super::{Failure, agent, arguments::Options};
use ic_blob_storage_contracts::dto::account::AccountInspectionFailure as E;
use ic_blob_storage_contracts::dto::account::AccountInspectionKind as Kind;
use ic_blob_storage_contracts::dto::account::AccountInspectionRequest;
use ic_blob_storage_contracts::dto::account::AccountInspectionResponse;
use ic_blob_storage_contracts::dto::account::AccountObservation as Observation;
use serde_json::{Value, json};

pub(super) fn kind(value: &str) -> Result<Kind, Failure> {
    match value {
        "balance" => Ok(Kind::Balance),
        "relationship" => Ok(Kind::PaymentRelationship),
        _ => Err(Failure::Arguments),
    }
}
fn decode(
    request: AccountInspectionRequest,
    bytes: &[u8],
) -> Result<AccountInspectionResponse, Failure> {
    let reply: Result<AccountInspectionResponse, E> =
        super::reply::decode_small(bytes, Failure::ReplyTooLarge)?;
    let response = reply.map_err(Failure::AccountRefused)?;
    if response.request != request {
        return Err(Failure::Binding);
    }
    let compatible = matches!(
        (&response.observation, request.kind),
        (
            Observation::Balance(_) | Observation::AccountNotFound,
            Kind::Balance
        ) | (
            Observation::Relationship(_)
                | Observation::NoRelationshipReported
                | Observation::RelationshipNotFound(_)
                | Observation::NotAuthorized(_)
                | Observation::InvalidRequest,
            Kind::PaymentRelationship,
        ) | (Observation::ProviderInternalError, _)
    );
    if !compatible {
        return Err(Failure::InvalidReply);
    }
    if let Observation::Relationship(value) = &response.observation
        && (value.paid_canister != request.scope.service
            || value.payment_account != request.scope.payment_account)
    {
        return Err(Failure::Binding);
    }
    Ok(response)
}

pub(super) async fn run(
    options: &Options,
    request: AccountInspectionRequest,
) -> Result<Value, Failure> {
    let agent = agent(options)?;
    let argument = candid::encode_one(request).map_err(|_| Failure::Arguments)?;
    let bytes = super::inspection_update(
        &agent,
        request.scope.service,
        "blob_inspect_account",
        argument,
    )
    .await?;
    let response = decode(request, &bytes)?;
    Ok(output(&response, options))
}

fn report(observation: &Observation) -> Value {
    match observation {
        Observation::Balance(b) => json!({"kind":"reported_balance",
            "total":b.total.to_string(),"prepaid":b.prepaid.to_string(),
            "promotional":b.promotional.to_string(),"ledger":b.ledger.to_string()}),
        Observation::Relationship(r) => json!({"kind":"reported_relationship",
            "paid_canister":r.paid_canister.to_text(),"payment_account":r.payment_account.to_text(),
            "spending_limit_per_day":r.spending_limit_per_day.0.to_str_radix(10),
            "current_period_spent":r.current_period_spent.0.to_str_radix(10),
            "current_period_start":r.current_period_start.to_string(),
            "added_timestamp":r.added_timestamp.to_string(),
            "expiration_timestamp":r.expiration_timestamp.map(|v|v.to_string()),
            "bandwidth_baseline_uploaded":r.bandwidth_baseline_uploaded.0.to_str_radix(10),
            "bandwidth_baseline_downloaded":r.bandwidth_baseline_downloaded.0.to_str_radix(10),
            "bandwidth_baseline_ts_ns":r.bandwidth_baseline_ts_ns.to_string()}),
        Observation::AccountNotFound => json!({"kind":"account_not_found"}),
        Observation::NoRelationshipReported => json!({"kind":"no_relationship_reported"}),
        Observation::RelationshipNotFound(p) => {
            json!({"kind":"relationship_not_found","reported_principal":p.to_text()})
        }
        Observation::NotAuthorized(p) => {
            json!({"kind":"not_authorized","reported_principal":p.to_text()})
        }
        Observation::InvalidRequest => json!({"kind":"invalid_request"}),
        Observation::ProviderInternalError => json!({"kind":"provider_internal_error"}),
    }
}
fn output(response: &AccountInspectionResponse, options: &Options) -> Value {
    let scope = response.request.scope;
    json!({"schema":1,"observation":"account_inspection",
        "kind":match response.request.kind { Kind::Balance=>"balance",Kind::PaymentRelationship=>"relationship" },
        "operator":options.actor.to_text(),"network":options.network,"url":options.url.as_str(),
        "verification":"ic_update_certificate",
        "scope":super::reply::scope_json(scope),
        "provider_report":report(&response.observation),
        "provider_credit":"not_established","spendability":"not_established",
        "retry_authorized":false,
    })
}
pub(super) const fn refusal_code(error: E) -> &'static str {
    match error {
        E::Invalid => "account_invalid",
        E::Denied => "account_denied",
        E::Binding => "account_binding",
        E::Fenced => "account_fenced",
        E::Internal => "account_internal",
        E::NotEnqueued => "account_not_enqueued",
        E::Rejected(_) => "account_rejected",
        E::ReplyTooLarge => "account_reply_too_large",
        E::InvalidReply => "account_invalid_reply",
    }
}

#[cfg(test)]
mod tests;
