//! Offline qualification frames only; signing and external custody stay with the operator.

use clap::{Parser, ValueEnum};
use ic_blob_storage_enrollment_probe::protocol::{Error, Installation, LIMITS, View};
use ic_platform_types::Principal;
use ic_wire::native;
use serde::Serialize;
use serde_json::{Value, json};

mod cashier;

#[derive(Parser)]
#[command(about = "Encode/decode enrollment qualification frames offline; never signs or sends")]
enum Command {
    /// Prepare immutable installation and unit method frames as JSON hexadecimal strings.
    Prepare {
        #[arg(long, value_parser = principal)]
        operator: Principal,
        #[arg(long, value_parser = principal)]
        cashier: Principal,
        #[arg(long, value_parser = clap::value_parser!(u64).range(1..=9_900_000_000))]
        target_balance: u64,
    },
    /// Decode a retained raw Wire response; provider reply bytes remain uninterpreted.
    Decode {
        #[arg(long, value_enum)]
        method: Method,
        #[arg(long, value_parser = |value: &str| unhex(value).map(Reply))]
        reply_hex: Reply,
    },
    /// Classify a retained account_top_up_v1 Candid reply offline; does not establish credit.
    DecodeTopUp {
        #[arg(long, value_parser = |value: &str| unhex(value).map(Reply))]
        reply_hex: Reply,
    },
}

#[derive(Clone)]
struct Reply(Vec<u8>);

#[derive(Clone, Copy, ValueEnum)]
enum Method {
    Inspect,
    Attempt,
}

impl Method {
    fn name(self) -> &'static str {
        match self {
            Self::Inspect => "enrollment_inspect",
            Self::Attempt => "enrollment_attempt",
        }
    }
}

fn main() -> Result<(), String> {
    let output = run(Command::parse())?;
    println!(
        "{}",
        serde_json::to_string_pretty(&output).map_err(|error| error.to_string())?
    );
    Ok(())
}

fn run(command: Command) -> Result<Value, String> {
    match command {
        Command::DecodeTopUp { reply_hex } => cashier::classify(&reply_hex.0),
        Command::Prepare {
            operator,
            cashier,
            target_balance,
        } => {
            let config = Installation {
                operator,
                cashier,
                target_balance,
            };
            Ok(json!({
                "configuration": config,
                "init_hex": frame("enrollment_init", &config)?,
                "inspect_hex": frame("enrollment_inspect", &())?,
                "attempt_hex": frame("enrollment_attempt", &())?,
                "upgrade_hex": frame("enrollment_upgrade", &())?,
            }))
        }
        Command::Decode { method, reply_hex } => {
            let result: Result<View, Error> =
                native::decode_response(native::method_id(method.name()), &reply_hex.0, LIMITS)
                    .map_err(|error| error.to_string())?;
            serde_json::to_value(result).map_err(|error| error.to_string())
        }
    }
}

fn frame<T: Serialize>(method: &str, value: &T) -> Result<String, String> {
    native::encode_request(native::method_id(method), value, LIMITS)
        .map(|bytes| bytes.iter().map(|byte| format!("{byte:02x}")).collect())
        .map_err(|error| error.to_string())
}

fn principal(value: &str) -> Result<Principal, String> {
    let value = Principal::from_text(value).map_err(|error| error.to_string())?;
    if value == Principal::anonymous() || value == Principal::management_canister() {
        return Err("an explicit non-anonymous principal is required".into());
    }
    Ok(value)
}

fn unhex(value: &str) -> Result<Vec<u8>, String> {
    if value.len() > LIMITS.bytes * 2 || !value.len().is_multiple_of(2) || !value.is_ascii() {
        return Err("expected bounded, even-length hexadecimal".into());
    }
    (0..value.len())
        .step_by(2)
        .map(|offset| {
            u8::from_str_radix(&value[offset..offset + 2], 16)
                .map_err(|_| "invalid hexadecimal".into())
        })
        .collect()
}

#[cfg(test)]
mod tests;
