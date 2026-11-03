//! Offline explicit Cashier account-link inputs; no credentials or dispatch.
use super::{Failure, artifacts::Run};
use ic_blob_storage_contracts::identity::ContentDigest;
use ic_blob_storage_contracts::provider::onboarding::AccountLinkBinding;
use ic_blob_storage_contracts::provider::onboarding::AccountLinkRequest;
use ic_blob_storage_contracts::provider::onboarding::AccountLinkTerms;
use serde_json::{Value, json};
pub(super) fn run(input: super::arguments::schema::AccountLink) -> Result<Value, Failure> {
    let binding = AccountLinkBinding {
        cashier: input.cashier,
        caller: input.caller,
        paid_canister: input.owner,
        payment_account: input.payer,
    };
    let terms = AccountLinkTerms {
        daily_limit: input.daily_limit,
        expiration_timestamp: input.expiry,
    };
    let directory = input.run_dir;
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
    let output = Run::create(&directory)?;
    output.bytes("account-link.candid", request.arguments())?;
    output.json("summary.json", &report)?;
    Ok(report)
}

#[cfg(test)]
mod tests;
