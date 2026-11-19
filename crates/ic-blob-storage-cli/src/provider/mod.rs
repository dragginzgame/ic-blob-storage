//! Private Caffeine query adapter and offline enrollment proposal. No paid effects.

#[cfg(test)]
mod tests;

use crate::overview::{Network, hex, principal};
use clap::Args;
use ic_platform_protocol::{
    caffeine::{self as c, Account, AccountRequest, AccountResult},
    candid::Limits,
};
use ic_platform_types::Principal;
use std::num::NonZeroU64;

#[derive(Args)]
pub(crate) struct Inspect {
    #[command(flatten)]
    network: Network,
    /// Explicit Caffeine Cashier canister; no default provider discovery.
    #[arg(long, value_parser = principal)]
    cashier: Principal,
    /// Exact account/owner to query; payer accounts need not own stored blobs.
    #[arg(long, value_parser = principal)]
    principal: Principal,
}

fn decode(reply: &[u8], expected: Principal) -> Result<Account, String> {
    let result = c::decode_account(
        reply,
        Limits {
            bytes: 65_536,
            header: 8_192,
            types: 128,
            work: 262_144,
            allocation: 262_144,
            vector: 65_536,
            skipping: 262_144,
        },
    )
    .map_err(|_| "invalid or oversized Cashier account reply")?;
    let account = match result {
        AccountResult::Ok(account) => account,
        AccountResult::Err(error) => {
            return Err(format!(
                "Cashier account inspection refused: {error:?}; storage is unknown"
            ));
        }
    };
    if account.account_owner_id != expected {
        return Err("Cashier returned a different account principal".into());
    }
    Ok(account)
}

pub(crate) async fn inspect(input: Inspect) -> Result<String, String> {
    let request = AccountRequest {
        account: input.principal,
    }
    .encode(1024)
    .map_err(|_| "cannot encode Cashier query")?;
    let target = input.cashier;
    let reply = tokio::time::timeout(
        std::time::Duration::from_secs(30),
        input
            .network
            .client()?
            .query(target, "account_info_get_v1", &request),
    )
    .await
    .map_err(|_| "Cashier query timed out; storage is unknown")?
    .map_err(|error| format!("Cashier query failed: {:?}", error))?;
    let account = decode(&reply, input.principal)?;
    let usage = account.storage_usage;
    Ok(format!(
        "Caffeine Cashier: {}\nAccount principal: {}\nSource: {}\n\nProvider-reported blobs: {}\nProvider-reported blob bytes: {}\nProvider-reported chunks: {}\nProvider-reported chunk bytes: {}\nProvider-reported cycle balance: {}\nLast charge timestamp (ns): {}\n\nOne account only; payer-funded owner accounts are not aggregated.\nNo file names, roots or individual file inventory are provided by this query.\nQuery observation, not consensus-certified evidence, verified availability or charge closure.",
        input.cashier,
        account.account_owner_id,
        input.network.url,
        usage.blob_count,
        usage.blob_size_bytes,
        usage.chunk_count,
        usage.chunk_size_bytes,
        account.cycle_balance,
        account.last_charge_ts_ns
    ))
}

#[derive(Args)]
pub(crate) struct Enrollment {
    /// Intended IC API endpoint. This command never connects to it.
    #[arg(long, value_parser = crate::endpoint)]
    url: url::Url,
    #[arg(long, value_parser = principal)]
    cashier: Principal,
    #[arg(long, value_parser = principal)]
    ledger: Principal,
    #[arg(long, value_parser = principal)]
    payer: Principal,
    /// Existing fresh owner canister; creation is outside this proposal.
    #[arg(long, value_parser = principal)]
    owner: Principal,
    /// Exact 32-byte Cashier-returned subaccount, 64 hexadecimal characters.
    #[arg(long, value_parser = subaccount)]
    deposit_subaccount: [u8; 32],
    #[arg(long)]
    deposit_cycles: NonZeroU64,
    #[arg(long)]
    ledger_fee_cycles: NonZeroU64,
    /// Maximum source debit for the proposed deposit, including the ledger fee.
    #[arg(long)]
    max_debit_cycles: NonZeroU64,
    /// Proposed provider daily limit. Units/enforcement still require qualification.
    #[arg(long)]
    daily_limit: NonZeroU64,
    /// Explicit proposed relationship expiry; not evidence of enforcement.
    #[arg(long)]
    expires_at_ns: NonZeroU64,
}

fn subaccount(value: &str) -> Result<[u8; 32], String> {
    if value.len() != 64 || !value.is_ascii() {
        return Err("expected exactly 64 hexadecimal characters".into());
    }
    let mut bytes = [0; 32];
    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&value[i * 2..i * 2 + 2], 16)
            .map_err(|_| "invalid deposit subaccount hex")?;
    }
    Ok(bytes)
}

pub(crate) fn prepare(input: &Enrollment) -> Result<String, String> {
    if input.payer == input.owner {
        return Err("this proposal requires a separate payer and owner".into());
    }
    let debit = input
        .deposit_cycles
        .get()
        .checked_add(input.ledger_fee_cycles.get())
        .ok_or("deposit plus fee overflows")?;
    if debit > input.max_debit_cycles.get() {
        return Err("deposit plus ledger fee exceeds maximum debit".into());
    }
    Ok(format!(
        "OFFLINE ENROLLMENT PROPOSAL — not signed or executable\nEndpoint: {}\nCashier: {}\nCycles Ledger: {}\nPayer: {}\nExisting owner: {}\n\n1. Inspect this exact payer, owner and existing payment relationship.\n   Recheck ledger fee and Cashier deposit address for this payer; do not infer them.\n2. Proposed deposit: {} cycles; ledger fee: {}; source debit: {}; maximum: {}.\n   Destination: Cashier {}, subaccount {}.\n   Retain exact intent and signed request before any later submission.\n   Existing or uncertain deposits must be reconciled; this proposal is no retry authority.\n3. After a separately executed deposit, retain its acknowledgment and notify Cashier once.\n   Credit amount remains unknown until acknowledged and read back.\n4. Proposed relationship: owner {} paid by {}, daily limit {}, expiry {} ns.\n   Qualify units, expiry enforcement and existing settings before signing.\n5. Inspect owner allocation and gateway credit before considering upload readiness.\n\nOwner creation, relationship effects, continuing storage/download charges and other fees\nare outside the deposit debit bound. No spending cap or provider closure is established.\nNo identity was loaded, no network call was made and no cycles were spent.",
        input.url,
        input.cashier,
        input.ledger,
        input.payer,
        input.owner,
        input.deposit_cycles,
        input.ledger_fee_cycles,
        debit,
        input.max_debit_cycles,
        input.cashier,
        hex(&input.deposit_subaccount),
        input.owner,
        input.payer,
        input.daily_limit,
        input.expires_at_ns
    ))
}
