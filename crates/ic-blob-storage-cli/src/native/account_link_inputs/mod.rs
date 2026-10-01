//! Offline explicit Cashier account-link inputs; no credentials or dispatch.
use super::{Failure, artifacts::Run};
use candid::Principal;
use ic_blob_storage::{
    model::identity::ContentDigest,
    ops::caffeine::onboarding::{AccountLinkBinding, AccountLinkRequest, AccountLinkTerms},
};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fmt::Display,
    num::{NonZeroU64, NonZeroU128},
    path::Path,
    str::FromStr,
};

fn positive<T: FromStr + Display>(value: &str) -> Result<T, Failure> {
    let result = value.parse::<T>().map_err(|_| Failure::Arguments)?;
    if result.to_string() != value {
        return Err(Failure::Arguments);
    }
    Ok(result)
}
fn principal(value: &str) -> Result<Principal, Failure> {
    let principal = Principal::from_text(value).map_err(|_| Failure::Arguments)?;
    if principal.to_text() != value {
        return Err(Failure::Arguments);
    }
    Ok(principal)
}
pub(super) fn run(args: &[String]) -> Result<Value, Failure> {
    let mut flags = BTreeMap::new();
    for pair in args[1..].chunks(2) {
        if pair.len() != 2 || flags.insert(pair[0].as_str(), pair[1].as_str()).is_some() {
            return Err(Failure::Arguments);
        }
    }
    let mut take = |key| flags.remove(key).ok_or(Failure::Arguments);
    let binding = AccountLinkBinding {
        cashier: principal(take("--cashier")?)?,
        caller: principal(take("--caller")?)?,
        paid_canister: principal(take("--owner")?)?,
        payment_account: principal(take("--payer")?)?,
    };
    let terms = AccountLinkTerms {
        daily_limit: positive::<NonZeroU128>(take("--daily-limit")?)?,
        expiration_timestamp: positive::<NonZeroU64>(take("--expiry")?)?,
    };
    let directory = Path::new(take("--run-dir")?);
    if !flags.is_empty() {
        return Err(Failure::Arguments);
    }
    let request = AccountLinkRequest::new(binding, terms).map_err(|_| Failure::Arguments)?;
    let report = json!({"schema":1,"observation":"local_account_link_inputs",
        "cashier":binding.cashier.to_text(),"caller":binding.caller.to_text(),
        "paid_canister":binding.paid_canister.to_text(),"payment_account":binding.payment_account.to_text(),
        "daily_limit":terms.daily_limit.to_string(),"expiration_timestamp":terms.expiration_timestamp.to_string(),
        "method":request.method_name(),"request_file":"account-link.candid",
        "request_sha256":ContentDigest::compute(request.arguments()).to_string(),
        "authenticated":false,"identities_allocated":false,"provider_dispatched":false,
        "funding_dispatched":false,"retry_authorized":false,"units_interpreted":false,
        "spending_guarantee":"not_established"});
    let output = Run::create(directory)?;
    output.bytes("account-link.candid", request.arguments())?;
    output.json("summary.json", &report)?;
    Ok(report)
}

#[cfg(test)]
mod tests;
