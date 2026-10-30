//! Offline explicit Cashier account-link inputs; no credentials or dispatch.
use super::{Failure, artifacts::Run};
use ic_blob_storage_contracts::identity::ContentDigest;
use ic_blob_storage_contracts::provider::onboarding::AccountLinkBinding;
use ic_blob_storage_contracts::provider::onboarding::AccountLinkRequest;
use ic_blob_storage_contracts::provider::onboarding::AccountLinkTerms;
use serde_json::{Value, json};

use super::arguments::value;
pub(super) fn run(flags: &clap::ArgMatches) -> Result<Value, Failure> {
    let binding = AccountLinkBinding {
        cashier: value(flags, "--cashier"),
        caller: value(flags, "--caller"),
        paid_canister: value(flags, "--owner"),
        payment_account: value(flags, "--payer"),
    };
    let terms = AccountLinkTerms {
        daily_limit: value(flags, "--daily-limit"),
        expiration_timestamp: value(flags, "--expiry"),
    };
    let directory = value::<std::path::PathBuf>(flags, "--run-dir");
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
